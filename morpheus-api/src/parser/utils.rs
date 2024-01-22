use serde::{de::Deserializer, Deserialize};

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

pub(super) fn parse_inner_entry<'de, D, T: Deserialize<'de>>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Inner<T> {
        entry: T,
    }

    Ok(Inner::deserialize(deserializer)?.entry)
}
