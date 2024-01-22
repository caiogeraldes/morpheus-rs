use serde::{Deserialize, Serialize};
use serde_with::{formats::PreferOne, serde_as, OneOrMany};

mod utils;
use crate::utils::{grammar::*, Language};
use utils::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
    #[serde(rename = "RDF")]
    rdf: RDF,
}

impl Response {
    pub fn get_body(&self) -> Vec<Body> {
        self.rdf.annot.body.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RDF {
    #[serde(rename = "Annotation")]
    annot: Annotation,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Annotation {
    // about: String,
    // creator: Creator,
    // created: Created,
    #[serde(rename = "Body", default)]
    #[serde_as(deserialize_as = "OneOrMany<_, PreferOne>")]
    body: Vec<Body>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Body {
    // about: String,
    // #[serde(rename = "type")]
    // body_type: BType,
    #[serde(deserialize_with = "parse_inner_entry", rename(deserialize = "rest"))]
    entry: Entry,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry {
    // uri: Option<String>,
    dict: Dict,
    #[serde_as(deserialize_as = "OneOrMany<_, PreferOne>")]
    infl: Vec<Infl>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Dict {
    #[serde(rename = "hdwd", deserialize_with = "parse_ds_value")]
    headword: String,
    pofs: OrderedValue,
    #[serde(deserialize_with = "parse_ds_option", default)]
    decl: Option<Declension>,
    #[serde(deserialize_with = "parse_ds_option", default)]
    gend: Option<Gender>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Infl {
    term: Term,
    #[serde(deserialize_with = "parse_ds_value")]
    pofs: String,
    #[serde(deserialize_with = "parse_ds_option", default)]
    decl: Option<Declension>,
    // Analysis
    #[serde(deserialize_with = "parse_ds_option", default)]
    gend: Option<Gender>,
    #[serde(deserialize_with = "parse_ds_option", default)]
    num: Option<Number>,
    #[serde(deserialize_with = "parse_ds_option", default)]
    mood: Option<Mood>,
    #[serde(deserialize_with = "parse_ds_option", default)]
    tense: Option<Tense>,
    #[serde(deserialize_with = "parse_ds_option", default)]
    voice: Option<Voice>,
    #[serde(deserialize_with = "parse_ds_option", default)]
    pers: Option<Person>,
    #[serde(deserialize_with = "parse_ds_option", default)]
    case: Option<Case>,
    // Dialect
    #[serde(deserialize_with = "parse_ds_option", default)]
    dial: Option<String>,
    // Word class
    #[serde(deserialize_with = "parse_ds_option", default)]
    stemtype: Option<String>,
    #[serde(deserialize_with = "parse_ds_option", default)]
    morph: Option<String>,
    #[serde(deserialize_with = "parse_ds_option", default)]
    derivtype: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Term {
    // lang: Language,
    #[serde(deserialize_with = "parse_ds_value", default)]
    stem: String,
    #[serde(deserialize_with = "parse_ds_option", default)]
    suff: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderedValue {
    order: u8,
    #[serde(rename = "$")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Headword {
    lang: Language,
    #[serde(rename = "$")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BType {
    resource: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Creator {
    #[serde(rename = "Agent")]
    agent: Agent,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Created {
    #[serde(rename = "$")]
    date: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Agent {
    about: String,
}

#[derive(Serialize)]
pub struct FlatEntry {
    headword: String,
    headword_decl: Option<Declension>,
    headword_gend: Option<Gender>,
    stem: String,
    suff: Option<String>,
    decl: Option<Declension>,
    pofs: String,
    // Analysis
    gend: Option<Gender>,
    num: Option<Number>,
    mood: Option<Mood>,
    tense: Option<Tense>,
    voice: Option<Voice>,
    pers: Option<Person>,
    case: Option<Case>,
    // Dialect
    dial: Option<String>,
    // Word class
    stemtype: Option<String>,
    morph: Option<String>,
    derivtype: Option<String>,
}

impl Entry {
    pub fn build_flat_entries(&self) -> Vec<FlatEntry> {
        self.infl
            .iter()
            .map(|value| FlatEntry {
                headword: self.dict.headword.clone(),
                headword_gend: self.dict.gend.clone(),
                headword_decl: self.dict.decl.clone(),
                stem: value.term.stem.clone(),
                suff: value.term.suff.clone(),
                pofs: value.pofs.clone(),
                decl: value.decl.clone(),
                gend: value.gend.clone(),
                num: value.num.clone(),
                mood: value.mood.clone(),
                tense: value.tense.clone(),
                voice: value.voice.clone(),
                pers: value.pers.clone(),
                case: value.case.clone(),
                dial: value.dial.clone(),
                stemtype: value.stemtype.clone(),
                morph: value.morph.clone(),
                derivtype: value.derivtype.clone(),
            })
            .collect()
    }
}

#[cfg(test)]
mod test;
