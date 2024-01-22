use anyhow::anyhow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

pub mod grammar;
