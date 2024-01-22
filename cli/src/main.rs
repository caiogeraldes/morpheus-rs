use anyhow::bail;
use anyhow::ensure;
use clap::Parser;
use log::info;
use log::warn;
use morpheus_api::parser::*;
use morpheus_api::query;
use morpheus_api::utils::Language;
use regex::Regex;
use serde_json;
use std::env;
use std::fs;
use std::path;
use tv_lib::print::print_from_csv_str;

mod util;
use util::from_query_and_entries;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    word: String,
    /// Output file
    #[arg(long, short, value_name = "FILE")]
    output: Option<String>,
    /// Keep query word in CSV output
    #[arg(long, short, default_value_t = false)]
    query: bool,
    /// Pretty print CSV output
    #[arg(long, short, default_value_t = false)]
    pretty: bool,
    /// Force betacode conversion of <WORD> (forces set language to Greek)
    #[arg(long, short, default_value_t = false)]
    betacode: bool,
    /// Language for the query, can be grc or lat.
    #[arg(long, short)]
    language: Option<Language>,
    /// Disable use of cached responses
    #[arg(long, short, default_value_t = false)]
    uncached: bool,
    /// Prints full JSON response from Perseids Morpheus API
    #[arg(long, short, default_value_t = false)]
    json: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();
    let mut args = Args::parse();

    if args.language.is_none() {
        let re = Regex::new(r"[\p{Greek})(=\/\\]")?;
        if re.is_match(&args.word) {
            args.language = Some(Language::Greek);
        } else {
            args.language = Some(Language::Latin);
        }
    }

    if args.betacode {
        if args.language == Some(Language::Latin) {
            warn!(
                "Command called with betacode but language set as Latin, setting language to Greek"
            );
        }
        args.language = Some(Language::Greek);
        args.word = betacode::converter::convert(args.word);
    } else {
        let re = Regex::new(r"[)(=\/\\]")?;
        if re.is_match(&args.word) {
            args.word = betacode::converter::convert(args.word);
        }
    }

    let query = query::QueryBuilder::new()
        .set_word(&args.word)?
        .set_lang(args.language.unwrap())?
        .build()?;

    let response: String;
    if !args.uncached {
        let cache_dir = match env::var("MORPHEUS_CACHE_DIR") {
            Ok(value) => path::PathBuf::from(value),
            Err(_) => {
                let home = home::home_dir().unwrap();
                warn!(
                    "Unknown env variable MORPHEUS_CACHE_DIR, assuming {:?}",
                    home.join(".cache/morpheus_rs")
                );
                home.join(".cache/morpheus_rs")
            }
        };
        if !cache_dir.is_dir() {
            info!("Creating path {:?}", &cache_dir);
            fs::create_dir(&cache_dir)?;
        }
        let cache_file_name = query.cache_name();
        let cache_file_path = cache_dir.join(cache_file_name);
        if cache_file_path.is_file() {
            info!("Loading response from {:?}", &cache_file_path);
            response = fs::read_to_string(cache_file_path)?;
        } else {
            response = reqwest::get(query.into_url()).await?.text().await?;
            info!("Saving response to {:?}", &cache_file_path);
            fs::write(cache_file_path, &response)?;
        }
    } else {
        response = reqwest::get(query.into_url()).await?.text().await?;
    }

    if args.json {
        if let Some(output) = args.output {
            info!("Saving JSON to {}", output);
            match fs::write(&output, response) {
                Ok(_) => info!("JSON saved to {}", output),
                Err(e) => {
                    log::error!("{}", e);
                    bail!("{}", e)
                }
            }
        } else {
            println!("{}", json::stringify_pretty(response, 2));
        }
    } else {
        let response: Response = serde_json::from_str(&response)?;
        ensure!(
            !response.is_empty(),
            "Unable to find analysis for {} in {:?}",
            args.word,
            &args.language.unwrap()
        );

        let mut wtr = csv::Writer::from_writer(vec![]);

        if args.query {
            let entries = from_query_and_entries(&args.word, response);
            for entry in entries {
                wtr.serialize(entry).unwrap()
            }
        } else {
            let entries = response.build_flat_entries();
            for entry in entries {
                wtr.serialize(entry).unwrap()
            }
        }

        let csv = String::from_utf8(wtr.into_inner().unwrap()).unwrap();
        if let Some(output) = args.output {
            info!("Saving CSV to {}", output);
            match fs::write(&output, csv) {
                Ok(_) => info!("CSV saved to {}", output),
                Err(e) => {
                    log::error!("{}", e);
                    bail!("{}", e)
                }
            }
        } else {
            if args.pretty {
                print_from_csv_str(&csv);
            } else {
                println!("{}", &csv);
            }
        }
    }

    Ok(())
}
