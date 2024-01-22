use super::*;
use lazy_static::lazy_static;
use serde_json;
use std::path::Path;
use std::{env, fs};

lazy_static! {
    static ref DATA_A: String = {
        let cmd: String = env::var("CARGO_MANIFEST_DIR").unwrap();
        let data_path = Path::new(&cmd).join("resources/").join("leleipsetai.json");
        fs::read_to_string(data_path).unwrap()
    };
}
lazy_static! {
    static ref DATA_B: String = {
        let cmd: String = env::var("CARGO_MANIFEST_DIR").unwrap();
        let data_path = Path::new(&cmd).join("resources/").join("andra.json");
        fs::read_to_string(data_path).unwrap()
    };
}
lazy_static! {
    static ref DATA_C: String = {
        let cmd: String = env::var("CARGO_MANIFEST_DIR").unwrap();
        let data_path = Path::new(&cmd).join("resources/").join("epepaideukh.json");
        fs::read_to_string(data_path).unwrap()
    };
}
lazy_static! {
    static ref DATA_D: String = {
        let cmd: String = env::var("CARGO_MANIFEST_DIR").unwrap();
        let data_path = Path::new(&cmd).join("resources/").join("facienda.json");
        fs::read_to_string(data_path).unwrap()
    };
}
lazy_static! {
    static ref DATA_E: String = {
        let cmd: String = env::var("CARGO_MANIFEST_DIR").unwrap();
        let data_path = Path::new(&cmd).join("resources/").join("luomenos.json");
        fs::read_to_string(data_path).unwrap()
    };
}

#[test]
fn h() {
    let mut data: Vec<&str> = Vec::new();
    data.push(&DATA_A);
    data.push(&DATA_B);
    data.push(&DATA_C);
    data.push(&DATA_D);
    data.push(&DATA_E);
    for d in data {
        let output = serde_json::from_str::<Response>(d).unwrap();
        let bodies = output.get_body();

        for body in bodies {
            let mut wtr = csv::Writer::from_writer(vec![]);

            let entries = body.entry.build_flat_entries();

            for dict in entries {
                wtr.serialize(dict).unwrap()
            }

            let strings = String::from_utf8(wtr.into_inner().unwrap()).unwrap();
            println!("{}", strings);
        }
    }
}
