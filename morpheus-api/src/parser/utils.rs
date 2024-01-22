use serde::{de::Deserializer, Deserialize};

use crate::parser::Term;

pub(super) fn parse_ds_option<'de, D, T: Deserialize<'de>>(
    deserializer: D,
) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
{
    type Outer<T> = Option<Inner<T>>;
    #[derive(Deserialize)]
    struct Inner<T> {
        #[serde(rename = "$")]
        value: T,
    }

    match Outer::deserialize(deserializer)? {
        Some(a) => Ok(Some(a.value)),
        None => Ok(None),
    }
}

pub(super) fn parse_ds_value<'de, D, T: Deserialize<'de>>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Inner<T> {
        #[serde(rename = "$")]
        value: T,
    }

    Ok(Inner::deserialize(deserializer)?.value)
}

pub(super) fn parse_stem_from_term<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let outer = Term::deserialize(deserializer)?;
    dbg!(&outer);
    Ok(outer.stem.clone())
}
