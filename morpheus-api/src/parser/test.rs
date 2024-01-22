use serde_json;

use crate::parser::{Dictionary, FlatInfl};

use super::Inflection;
#[test]
fn a() {
    let source = r#"{"RDF":{"Annotation":{"about":"urn:TuftsMorphologyService:homo:morpheuslat","creator":{"Agent":{"about":"org.perseus:tools:morpheus.v1"}},"created":{"$":"2020-01-01T00:00:00.000000"},"hasTarget":{"Description":{"about":"urn:word:homo"}},"title":{},"hasBody":{"resource":"urn:uuid:idm1f6495b96a1082a031f512e601dd2f8277cb96aa6b3da261d9a7a4df18316e72"},"Body":{"about":"urn:uuid:idm1f6495b96a1082a031f512e601dd2f8277cb96aa6b3da261d9a7a4df18316e72","type":{"resource":"cnt:ContentAsXML"},"rest":{"entry":{"uri":null,"dict":{"hdwd":{"lang":"lat","$":"homo"},"pofs":{"order":3,"$":"noun"},"decl":{"$":"3rd"},"gend":{"$":"masculine/feminine"}},"infl":[{"term":{"lang":"lat","stem":{"$":"hom"},"suff":{"$":"o_"}},"pofs":{"order":3,"$":"noun"},"decl":{"$":"3rd"},"case":{"order":7,"$":"nominative"},"gend":{"$":"masculine"},"num":{"$":"singular"},"stemtype":{"$":"o_inis"}},{"term":{"lang":"lat","stem":{"$":"hom"},"suff":{"$":"o_"}},"pofs":{"order":3,"$":"noun"},"decl":{"$":"3rd"},"case":{"order":7,"$":"nominative"},"gend":{"$":"feminine"},"num":{"$":"singular"},"stemtype":{"$":"o_inis"}}]}}}}}}"#;
    let output: super::Response = serde_json::from_str(source).unwrap();
    // dbg!(output);
    let dicts = output.get_dicts();
    for dict in dicts {
        println!("{}", dict);
    }
}
#[test]
fn b() {
    let source = r#"{"RDF":{"Annotation":{"about":"urn:TuftsMorphologyService:andra:morpheuslat","creator":{"Agent":{"about":"org.perseus:tools:morpheus.v1"}},"created":{"$":"2020-01-01T00:00:00.000000"},"hasTarget":{"Description":{"about":"urn:word:andra"}},"title":{}}}}"#;
    let output: super::Response = serde_json::from_str(source).unwrap();
    // dbg!(output);
    let dicts = output.get_dicts();
    for dict in dicts {
        println!("{}", dict);
    }
}
#[test]
fn c() {
    let source = r#"{"RDF":{"Annotation":{"about":"urn:TuftsMorphologyService:lu/w:morpheusgrc","creator":{"Agent":{"about":"org.perseus:tools:morpheus.v1"}},"created":{"$":"2020-01-01T00:00:00.000000"},"hasTarget":{"Description":{"about":"urn:word:lu/w"}},"title":{},"hasBody":{"resource":"urn:uuid:idmf75d121b0a6342a933b207e40856fca4379d3a4357ff8d07e6411dccd864024f"},"Body":{"about":"urn:uuid:idmf75d121b0a6342a933b207e40856fca4379d3a4357ff8d07e6411dccd864024f","type":{"resource":"cnt:ContentAsXML"},"rest":{"entry":{"uri":null,"dict":{"hdwd":{"lang":"grc","$":"λύω"},"pofs":{"order":1,"$":"verb"}},"infl":[{"term":{"lang":"grc","stem":{"$":"λυ"},"suff":{"$":"ω"}},"pofs":{"order":1,"$":"verb"},"mood":{"$":"subjunctive"},"num":{"$":"singular"},"pers":{"$":"1st"},"tense":{"$":"present"},"voice":{"$":"active"},"dial":{"$":"epic"},"stemtype":{"$":"w_stem"},"derivtype":{"$":"reg_conj"}},{"term":{"lang":"grc","stem":{"$":"λυ"},"suff":{"$":"ω"}},"pofs":{"order":1,"$":"verb"},"mood":{"$":"indicative"},"num":{"$":"singular"},"pers":{"$":"1st"},"tense":{"$":"present"},"voice":{"$":"active"},"dial":{"$":"epic"},"stemtype":{"$":"w_stem"},"derivtype":{"$":"reg_conj"}},{"term":{"lang":"grc","stem":{"$":"λῡ"},"suff":{"$":"ω"}},"pofs":{"order":1,"$":"verb"},"mood":{"$":"subjunctive"},"num":{"$":"singular"},"pers":{"$":"1st"},"tense":{"$":"present"},"voice":{"$":"active"},"stemtype":{"$":"w_stem"},"derivtype":{"$":"reg_conj"}},{"term":{"lang":"grc","stem":{"$":"λῡ"},"suff":{"$":"ω"}},"pofs":{"order":1,"$":"verb"},"mood":{"$":"indicative"},"num":{"$":"singular"},"pers":{"$":"1st"},"tense":{"$":"present"},"voice":{"$":"active"},"stemtype":{"$":"w_stem"},"derivtype":{"$":"reg_conj"}}]}}}}}}"#;
    let output: super::Response = serde_json::from_str(source).unwrap();
    // dbg!(output);
    let dicts = output.get_dicts();
    for dict in dicts {
        println!("{}", dict);
    }
}
#[test]
fn d() {
    let source = r#"{"RDF":{"Annotation":{"about":"urn:TuftsMorphologyService:a)/ndra:morpheusgrc","creator":{"Agent":{"about":"org.perseus:tools:morpheus.v1"}},"created":{"$":"2020-01-01T00:00:00.000000"},"hasTarget":{"Description":{"about":"urn:word:a)/ndra"}},"title":{},"hasBody":{"resource":"urn:uuid:idm781f5aa444b019ffcfa75e1e4c71a4cba20b1b998fdd8536a1b56bf6c51f298d"},"Body":{"about":"urn:uuid:idm781f5aa444b019ffcfa75e1e4c71a4cba20b1b998fdd8536a1b56bf6c51f298d","type":{"resource":"cnt:ContentAsXML"},"rest":{"entry":{"uri":null,"dict":{"hdwd":{"lang":"grc","$":"ἀνήρ"},"pofs":{"order":3,"$":"noun"},"decl":{"$":"3rd"},"gend":{"$":"masculine"}},"infl":{"term":{"lang":"grc","stem":{"$":"ἄνδρα"}},"pofs":{"order":3,"$":"noun"},"decl":{"$":"3rd"},"case":{"order":4,"$":"accusative"},"gend":{"$":"masculine"},"num":{"$":"singular"},"stemtype":{"$":"irreg_decl3"},"morph":{"$":"indeclform"}}}}}}}}"#;
    let output: super::Response = serde_json::from_str(source).unwrap();
    // dbg!(output);
    let dicts = output.get_dicts();
    for dict in dicts {
        println!("{}", dict);
    }
}

#[test]
fn e() {
    let source = r#"{"RDF":{"Annotation":{"about":"urn:TuftsMorphologyService:luo/menos:morpheusgrc","creator":{"Agent":{"about":"org.perseus:tools:morpheus.v1"}},"created":{"$":"2020-01-01T00:00:00.000000"},"hasTarget":{"Description":{"about":"urn:word:luo/menos"}},"title":{},"hasBody":{"resource":"urn:uuid:idmf059395acea257aaf75a3037bd983785f50350a2a05dfaa8cae13c31f74890a7"},"Body":{"about":"urn:uuid:idmf059395acea257aaf75a3037bd983785f50350a2a05dfaa8cae13c31f74890a7","type":{"resource":"cnt:ContentAsXML"},"rest":{"entry":{"uri":null,"dict":{"hdwd":{"lang":"grc","$":"λύω"},"pofs":{"order":1,"$":"verb"}},"infl":[{"term":{"lang":"grc","stem":{"$":"λυ"},"suff":{"$":"όμενος"}},"pofs":{"order":0,"$":"verb participle"},"case":{"order":7,"$":"nominative"},"gend":{"$":"masculine"},"mood":{"$":"participle"},"num":{"$":"singular"},"tense":{"$":"present"},"voice":{"$":"mediopassive"},"dial":{"$":"epic"},"stemtype":{"$":"w_stem"},"derivtype":{"$":"reg_conj"}},{"term":{"lang":"grc","stem":{"$":"λῡ"},"suff":{"$":"όμενος"}},"pofs":{"order":0,"$":"verb participle"},"case":{"order":7,"$":"nominative"},"gend":{"$":"masculine"},"mood":{"$":"participle"},"num":{"$":"singular"},"tense":{"$":"present"},"voice":{"$":"mediopassive"},"stemtype":{"$":"w_stem"},"derivtype":{"$":"reg_conj"}}]}}}}}}"#;
    let output: super::Response = serde_json::from_str(source).unwrap();
    // dbg!(output);
    let dicts = output.get_dicts();
    for dict in dicts {
        println!("{}", dict);
    }
}

#[test]
fn f() {
    let source = r#"{"RDF":{"Annotation":{"about":"urn:TuftsMorphologyService:e)/rxomai:morpheusgrc","creator":{"Agent":{"about":"org.perseus:tools:morpheus.v1"}},"created":{"$":"2020-01-01T00:00:00.000000"},"hasTarget":{"Description":{"about":"urn:word:e)/rxomai"}},"title":{},"hasBody":{"resource":"urn:uuid:idm55a92bdc367db5e3d6f0171ac9ce8e6299ee159a0ea99399eb31ec3b578675f4"},"Body":{"about":"urn:uuid:idm55a92bdc367db5e3d6f0171ac9ce8e6299ee159a0ea99399eb31ec3b578675f4","type":{"resource":"cnt:ContentAsXML"},"rest":{"entry":{"uri":null,"dict":{"hdwd":{"lang":"grc","$":"ἔρχομαι"},"pofs":{"order":1,"$":"verb"}},"infl":{"term":{"lang":"grc","stem":{"$":"ἐρχ"},"suff":{"$":"ομαι"}},"pofs":{"order":1,"$":"verb"},"mood":{"$":"indicative"},"num":{"$":"singular"},"pers":{"$":"1st"},"tense":{"$":"present"},"voice":{"$":"mediopassive"},"stemtype":{"$":"w_stem"}}}}}}}}"#;
    let output: super::Response = serde_json::from_str(source).unwrap();
    // dbg!(output);
    let dicts = output.get_dicts();
    for dict in dicts {
        println!("{}", dict);
    }
}

#[test]
fn g() {
    let source = r#"{"RDF":{"Annotation":{"about":"urn:TuftsMorphologyService:e)pepaideu/kh:morpheusgrc","creator":{"Agent":{"about":"org.perseus:tools:morpheus.v1"}},"created":{"$":"2020-01-01T00:00:00.000000"},"hasTarget":{"Description":{"about":"urn:word:e)pepaideu/kh"}},"title":{},"hasBody":{"resource":"urn:uuid:idm134223e4e7d1b1617ec51469f1b4ec9e1b790f74453bea98dd7a37e20e6d1cc3"},"Body":{"about":"urn:uuid:idm134223e4e7d1b1617ec51469f1b4ec9e1b790f74453bea98dd7a37e20e6d1cc3","type":{"resource":"cnt:ContentAsXML"},"rest":{"entry":{"uri":null,"dict":{"hdwd":{"lang":"grc","$":"παιδεύω"},"pofs":{"order":1,"$":"verb"}},"infl":[{"term":{"lang":"grc","stem":{"$":"ἐ:πεπαιδευκ"},"suff":{"$":"η"}},"pofs":{"order":1,"$":"verb"},"mood":{"$":"indicative"},"num":{"$":"singular"},"pers":{"$":"3rd"},"tense":{"$":"pluperfect"},"voice":{"$":"active"},"dial":{"$":"Doric Aeolic"},"stemtype":{"$":"perf_act"},"derivtype":{"$":"euw"},"morph":{"$":"contr redupl"}},{"term":{"lang":"grc","stem":{"$":"ἐ:πεπαιδευκ"},"suff":{"$":"η"}},"pofs":{"order":1,"$":"verb"},"mood":{"$":"indicative"},"num":{"$":"singular"},"pers":{"$":"1st"},"tense":{"$":"pluperfect"},"voice":{"$":"active"},"stemtype":{"$":"perf_act"},"derivtype":{"$":"euw"},"morph":{"$":"redupl"}}]}}}}}}"#;
    let output: super::Response = serde_json::from_str(source).unwrap();
    // dbg!(output);
    let dicts = output.get_dicts();
    for dict in dicts {
        println!("{}", dict);
    }
}

#[test]
fn h() {
    let source = r#"{"RDF":{"Annotation":{"about":"urn:TuftsMorphologyService:lelei/yetai:morpheusgrc","creator":{"Agent":{"about":"org.perseus:tools:morpheus.v1"}},"created":{"$":"2020-01-01T00:00:00.000000"},"hasTarget":{"Description":{"about":"urn:word:lelei/yetai"}},"title":{},"hasBody":{"resource":"urn:uuid:idm94e7bf9b530efa4d3e8c9ccac4a1755d7c3f52b46054478d1432dd80d6b50ccc"},"Body":{"about":"urn:uuid:idm94e7bf9b530efa4d3e8c9ccac4a1755d7c3f52b46054478d1432dd80d6b50ccc","type":{"resource":"cnt:ContentAsXML"},"rest":{"entry":{"uri":null,"dict":{"hdwd":{"lang":"grc","$":"λείπω"},"pofs":{"order":1,"$":"verb"}},"infl":{"term":{"lang":"grc","stem":{"$":"λελειψ"},"suff":{"$":"εται"}},"pofs":{"order":1,"$":"verb"},"mood":{"$":"indicative"},"num":{"$":"singular"},"pers":{"$":"3rd"},"tense":{"$":"future perfect"},"voice":{"$":"middle"},"stemtype":{"$":"fut_perf"},"derivtype":{"$":"reg_conj"}}}}}}}}"#;
    let output: super::Response = serde_json::from_str(source).unwrap();

    let bodies = output.get_body();

    for body in bodies {
        let mut wtr = csv::Writer::from_writer(vec![]);

        let dicts = body.get_dicts();
        for dict in dicts {
            println!("{}", dict);
        }

        let dicts = body.get_infl();
        for dict in dicts {
            wtr.serialize(FlatInfl::from(dict)).unwrap()
        }

        let strings = String::from_utf8(wtr.into_inner().unwrap()).unwrap();
        println!("{}", strings);
    }
}
