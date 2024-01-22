use std::fmt::Display;

use serde::{Deserialize, Serialize};
use serde_with::{formats::PreferOne, serde_as, OneOrMany};

mod utils;
use crate::utils::{grammar::*, Language};
use utils::*;

pub trait Dictionary {
    fn get_dicts(&self) -> Vec<Dict>;
}

pub trait Inflection {
    fn get_infl(&self) -> Vec<Infl>;
}

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

impl Dictionary for Response {
    fn get_dicts(&self) -> Vec<Dict> {
        self.rdf.get_dicts()
    }
}

impl Inflection for Response {
    fn get_infl(&self) -> Vec<Infl> {
        self.rdf.get_infl()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RDF {
    #[serde(rename = "Annotation")]
    annot: Annotation,
}

impl Dictionary for RDF {
    fn get_dicts(&self) -> Vec<Dict> {
        self.annot.get_dicts()
    }
}

impl Inflection for RDF {
    fn get_infl(&self) -> Vec<Infl> {
        self.annot.get_infl()
    }
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

impl Dictionary for Annotation {
    fn get_dicts(&self) -> Vec<Dict> {
        let mut v = vec![];
        for body in &self.body {
            v.push(body.get_dicts()[0].clone());
        }
        v
    }
}

impl Inflection for Annotation {
    fn get_infl(&self) -> Vec<Infl> {
        let mut v = vec![];
        for body in &self.body {
            let mut infls = body.get_infl().clone();
            v.append(&mut infls);
        }
        v
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Body {
    // about: String,
    // #[serde(rename = "type")]
    // body_type: BType,
    rest: Rest,
}

impl Dictionary for Body {
    fn get_dicts(&self) -> Vec<Dict> {
        self.rest.get_dicts()
    }
}

impl Inflection for Body {
    fn get_infl(&self) -> Vec<Infl> {
        self.rest.get_infl()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Rest {
    entry: Entry,
}

impl Dictionary for Rest {
    fn get_dicts(&self) -> Vec<Dict> {
        self.entry.get_dicts()
    }
}

impl Inflection for Rest {
    fn get_infl(&self) -> Vec<Infl> {
        self.entry.get_infl()
    }
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry {
    // uri: Option<String>,
    dict: Dict,
    #[serde_as(deserialize_as = "OneOrMany<_, PreferOne>")]
    infl: Vec<Infl>,
}

impl Dictionary for super::parser::Entry {
    fn get_dicts(&self) -> Vec<Dict> {
        Vec::from([self.dict.clone()])
    }
}

impl Inflection for Entry {
    fn get_infl(&self) -> Vec<Infl> {
        self.infl.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Dict {
    #[serde(rename = "hdwd")]
    headword: Headword,
    pofs: OrderedValue,
    #[serde(deserialize_with = "parse_ds_option", default)]
    decl: Option<Declension>,
    #[serde(deserialize_with = "parse_ds_option", default)]
    gend: Option<Gender>,
}

impl Display for Dict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.decl, &self.gend) {
            (None, None) => write!(f, "{}, {}", self.headword, self.pofs),
            (Some(decl), Some(gend)) => {
                write!(f, "{}, {} ({} {})", self.headword, self.pofs, decl, gend)
            }
            _ => todo!(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Infl {
    #[serde(skip_serializing)]
    term: Term,
    #[serde(deserialize_with = "parse_stem_from_term", default)]
    stem: String,
    #[serde(skip_serializing)]
    pofs: OrderedValue,
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

impl Display for Infl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut analysis = String::new();
        if let Some(mood) = &self.mood {
            analysis.push_str(&format!("{} ", mood));
        }
        if let Some(tense) = &self.tense {
            analysis.push_str(&format!("{} ", tense));
        }
        if let Some(voice) = &self.voice {
            analysis.push_str(&format!("{} ", voice));
        }
        if let Some(pers) = &self.pers {
            analysis.push_str(&format!("{} ", pers));
        }
        if let Some(num) = &self.num {
            analysis.push_str(&format!("{} ", num));
        }
        if let Some(gend) = &self.gend {
            analysis.push_str(&format!("{} ", gend));
        }
        if let Some(case) = &self.case {
            analysis.push_str(&format!("{} ", case));
        }

        let mut wc = String::new();
        if let Some(stemtype) = &self.stemtype {
            wc.push_str(stemtype);
        }
        if let Some(derivtype) = &self.derivtype {
            wc.push(',');
            wc.push(' ');
            wc.push_str(derivtype);
        }
        if let Some(morph) = &self.morph {
            wc.push(',');
            wc.push(' ');
            wc.push_str(morph);
        }

        let dial: String = match &self.dial {
            Some(d) => d.clone(),
            None => "attic".into(),
        };

        write!(
            f,
            "{}, {}, {}, {}, {}",
            self.term,
            self.pofs,
            analysis.trim(),
            dial,
            wc.trim()
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Term {
    // lang: Language,
    #[serde(deserialize_with = "parse_ds_value", default)]
    stem: String,
    #[serde(deserialize_with = "parse_ds_option", default)]
    suff: Option<String>,
}

impl Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.suff {
            Some(suff) => write!(f, "{} (-{})", self.stem, suff),
            None => write!(f, "{}", self.stem),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Value {
    #[serde(rename = "$")]
    value: String,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderedValue {
    order: u8,
    #[serde(rename = "$")]
    value: String,
}

impl Display for OrderedValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Headword {
    lang: Language,
    #[serde(rename = "$")]
    value: String,
}

impl Display for Headword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
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

#[derive(Serialize, Debug, Clone)]
pub struct FlatInfl {
    stem: String,
    suff: Option<String>,
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

impl From<Infl> for FlatInfl {
    fn from(value: Infl) -> Self {
        Self {
            stem: value.term.stem,
            suff: value.term.suff,
            decl: value.decl,
            gend: value.gend,
            num: value.num,
            mood: value.mood,
            tense: value.tense,
            voice: value.voice,
            pers: value.pers,
            case: value.case,
            dial: value.dial,
            stemtype: value.stemtype,
            morph: value.morph,
            derivtype: value.derivtype,
        }
    }
}

#[cfg(test)]
mod test;
