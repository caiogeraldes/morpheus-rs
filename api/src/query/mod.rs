use crate::utils::Language;
use unicode_normalization::UnicodeNormalization;
pub(crate) const API_URL: &str = "https://morph.perseids.org/analysis/word";

#[derive(Debug, PartialEq, Eq)]
pub struct Query {
    word: String,
    lang: Language,
}

impl Query {
    pub fn into_url(self) -> String {
        let (engine, lang) = match self.lang {
            Language::Latin => ("morpheuslat", "lat"),
            Language::Greek => ("morpheusgrc", "grc"),
        };

        format!(
            "{}?lang={}&engine={}&word={}",
            API_URL, lang, engine, self.word
        )
    }

    pub fn cache_name(&self) -> String {
        let engine = match self.lang {
            Language::Latin => "morpheuslat",
            Language::Greek => "morpheusgrc",
        };
        format!("{}.{}", self.word, engine)
    }
}

#[derive(Debug)]
pub struct QueryBuilder {
    word: Option<String>,
    lang: Option<Language>,
}

impl QueryBuilder {
    pub fn new() -> QueryBuilder {
        QueryBuilder {
            word: None,
            lang: None,
        }
    }

    pub fn build(self) -> anyhow::Result<Query> {
        let lang = self.lang.unwrap();
        let word = self.word.unwrap();
        Ok(Query { word, lang })
    }

    pub fn set_word<T: Into<String>>(mut self, word: T) -> anyhow::Result<QueryBuilder> {
        let word = normalize_unicode(word);
        self.word = Some(word.try_into()?);
        Ok(self)
    }

    pub fn set_lang_from_str(mut self, lang: &str) -> anyhow::Result<QueryBuilder> {
        self.lang = Some(Language::try_from(lang)?);
        Ok(self)
    }

    pub fn set_lang(mut self, lang: Language) -> anyhow::Result<QueryBuilder> {
        self.lang = Some(lang);
        Ok(self)
    }
}

fn normalize_unicode<T: Into<String>>(input: T) -> String {
    let input: &str = &input.into();
    input.nfkc().collect::<String>()
}
