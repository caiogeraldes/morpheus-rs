use morpheus_api::parser::Response;
use morpheus_api::utils::grammar::*;
use serde::Serialize;

type DetailedFlatEntries = Vec<DetailedFlatEntry>;

#[derive(Serialize)]
pub(crate) struct DetailedFlatEntry {
    queryword: String,
    headword: String,
    headword_decl: Option<Declension>,
    headword_gend: Option<Gender>,
    stem: String,
    suff: Option<String>,
    decl: Option<Declension>,
    pofs: String,
    gend: Option<Gender>,
    num: Option<Number>,
    mood: Option<Mood>,
    tense: Option<Tense>,
    voice: Option<Voice>,
    pers: Option<Person>,
    case: Option<Case>,
    dial: Option<String>,
    stemtype: Option<String>,
    morph: Option<String>,
    derivtype: Option<String>,
}

pub(crate) fn from_query_and_entries(query: &str, response: Response) -> DetailedFlatEntries {
    let mut v = vec![];
    for entry in response.build_flat_entries() {
        v.push(DetailedFlatEntry {
            queryword: query.into(),
            headword: entry.headword.clone(),
            headword_gend: entry.gend.clone(),
            headword_decl: entry.decl.clone(),
            stem: entry.stem.clone(),
            suff: entry.suff.clone(),
            pofs: entry.pofs.clone(),
            decl: entry.decl.clone(),
            gend: entry.gend.clone(),
            num: entry.num.clone(),
            mood: entry.mood.clone(),
            tense: entry.tense.clone(),
            voice: entry.voice.clone(),
            pers: entry.pers.clone(),
            case: entry.case.clone(),
            dial: entry.dial.clone(),
            stemtype: entry.stemtype.clone(),
            morph: entry.morph.clone(),
            derivtype: entry.derivtype.clone(),
        });
    }
    v
}
