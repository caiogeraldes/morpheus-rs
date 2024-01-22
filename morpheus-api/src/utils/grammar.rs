use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Gender {
    Masculine,
    Feminine,
    Neuter,
    #[serde(rename = "masculine/feminine")]
    MasculineFeminine,
}

impl Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Gender::Masculine => "masc",
            Gender::Feminine => "fem",
            Gender::Neuter => "neut",
            Gender::MasculineFeminine => "masc/fem",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Number {
    Singular,
    Dual,
    Plural,
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Number::Singular => "sg",
            Number::Dual => "du",
            Number::Plural => "pl",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mood {
    Indicative,
    Imperative,
    Subjunctive,
    Optative,
    Participle,
    Gerundive,
}

impl Display for Mood {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Mood::Indicative => "ind",
            Mood::Imperative => "imp",
            Mood::Subjunctive => "subj",
            Mood::Optative => "opt",
            Mood::Participle => "part",
            Mood::Gerundive => "ger",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Tense {
    Present,
    Imperfect,
    Future,
    Aorist,
    Perfect,
    Pluperfect,
    #[serde(rename = "future perfect")]
    FuturePerfect,
}

impl Display for Tense {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Tense::Present => "pres",
            Tense::Imperfect => "imperf",
            Tense::Future => "fut",
            Tense::Aorist => "aor",
            Tense::Perfect => "perf",
            Tense::Pluperfect => "pluperf",
            Tense::FuturePerfect => "futperf",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Voice {
    Active,
    Middle,
    Passive,
    #[serde(rename = "mediopassive")]
    MiddlePassive,
}

impl Display for Voice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Voice::Active => "actv",
            Voice::Middle => "mid",
            Voice::Passive => "pass",
            Voice::MiddlePassive => "mp",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Person {
    #[serde(rename = "1st")]
    First,
    #[serde(rename = "2nd")]
    Second,
    #[serde(rename = "3rd")]
    Third,
}

impl Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Person::First => "1st",
            Person::Second => "2nd",
            Person::Third => "3rd",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Declension {
    #[serde(rename = "1st")]
    First,
    #[serde(rename = "2nd")]
    Second,
    #[serde(rename = "3rd")]
    Third,
    #[serde(rename = "4th")]
    Fourth,
    #[serde(rename = "5th")]
    Fifth,
}

impl Display for Declension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Declension::First => "1st",
            Declension::Second => "2nd",
            Declension::Third => "3rd",
            Declension::Fourth => "4th",
            Declension::Fifth => "5th",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Case {
    Nominative,
    Accusative,
    Genitive,
    Dative,
    Ablative,
    Vocative,
}

impl Display for Case {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
