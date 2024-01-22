use std::str::FromStr;

use anyhow::anyhow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Copy)]
pub enum Language {
    #[serde(rename = "lat")]
    Latin,
    #[serde(rename = "grc")]
    Greek,
}

impl TryFrom<&str> for Language {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "lat" | "latin" | "Latin" => Ok(Self::Latin),
            "grc" | "Greek" | "greek" => Ok(Self::Greek),
            _ => Err(anyhow!("Unknown language: ${value}")),
        }
    }
}

impl FromStr for Language {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match s.as_str() {
            "grc" | "g" | "greek" => Ok(Language::Greek),
            "lat" | "l" | "latin" => Ok(Language::Latin),
            _ => Err(anyhow!("Unknown language")),
        }
    }

    type Err = anyhow::Error;
}

pub mod grammar;
