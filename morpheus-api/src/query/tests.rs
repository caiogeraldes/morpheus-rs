use super::*;

#[test]
fn lat_query() {
    let q = QueryBuilder::new()
        .set_word("unus")
        .unwrap()
        .set_lang_from_str("lat")
        .unwrap()
        .build()
        .unwrap();
    let expected = "https://morph.perseids.org/analysis/word?lang=lat&engine=morpheuslat&word=unus";
    let result = q.into_url();
    assert_eq!(expected, &result);
}

#[test]
#[should_panic]
fn invalid_lang() {
    let _ = QueryBuilder::new()
        .set_word("unus")
        .unwrap()
        .set_lang_from_str("lit")
        .unwrap();
}

#[test]
fn grc_query() {
    let q = QueryBuilder::new()
        .set_word("ἱστορίης")
        .unwrap()
        .set_lang_from_str("grc")
        .unwrap()
        .build()
        .unwrap();
    let expected =
        "https://morph.perseids.org/analysis/word?lang=grc&engine=morpheusgrc&word=ἱστορίης";
    let result = q.into_url();
    assert_eq!(expected, &result);
}
