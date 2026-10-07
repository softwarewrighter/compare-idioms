use std::path::Path;

use ci_catalog::{load, parse_bindings, parse_literal};

#[test]
fn literals() {
    let cases = [
        ("5", "num [] 5"),
        ("1 2 3", "num [3] 1 2 3"),
        ("\"ABCDE\"", "char [5] \"ABCDE\""),
        ("2 3 r_eshape 1 2 3 4 5 6", "num [2 3] 1 2 3 4 5 6"),
    ];
    for (text, want) in cases {
        assert_eq!(parse_literal(text).unwrap().to_string(), want, "{text}");
    }
    assert!(parse_literal("2 3 r_eshape 1 2").is_none(), "too few items");
}

#[test]
fn bindings() {
    let b = parse_bindings("v := 3 1 4; x := 4").unwrap();
    assert_eq!(b.len(), 2);
    assert_eq!(
        (b[1].0.as_str(), b[1].1.to_string().as_str()),
        ("x", "num [] 4")
    );
    assert!(parse_bindings("u:f_ := { x -> n_eg x }; x := 3").is_none());
}

const DATA: &str = r#"
idioms = ["sum", "where", "format", "each"]
idiom_names = ["Sum", "Where", "Number to text", "Each"]
[input.sum]
xetal = 'v := 1 2 3 4'
[input.where]
xetal = 'b := 0 1 1 0'
[input.format]
xetal = 'v := 42'
[input.each]
xetal = 'u:f_ := { x -> n_eg x }; v := 1 2 3'
[source.sum]
j = '+/ v'
k = '+/v'
[source.where]
j = 'I. b'
[source.format]
j = '": v'
[output.sum]
xetal = "10"
[output.where]
xetal = "2 3"
[output.format]
xetal = "42"
[notes.sum]
k = "ignored"
"#;

#[test]
fn loads_only_runnable_idioms() {
    let dir = std::env::temp_dir().join(format!("ci-catalog-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("data.toml");
    std::fs::write(&path, DATA).unwrap();
    let idioms = load(&path).unwrap();
    let ids: Vec<&str> = idioms.iter().map(|i| i.id.as_str()).collect();
    assert_eq!(
        ids,
        ["sum", "where", "format"],
        "each has a function operand"
    );
    assert_eq!(idioms[0].expected.to_string(), "num [] 10");
    assert_eq!(idioms[0].cells["k"], "+/v");
    assert!(idioms[1].index_result && !idioms[0].index_result);
    assert_eq!(idioms[2].expected.to_string(), "char [2] \"42\"");
    std::fs::remove_dir_all(&dir).unwrap();
    assert!(load(Path::new("/no/such/data.toml")).is_err());
}
