use ci_value::{Value, agree, parse_native, parse_number, parse_serialized, shift_origin};

#[test]
fn canonical_forms() {
    assert_eq!(Value::nums(vec![], vec![10.0]).to_string(), "num [] 10");
    assert_eq!(
        Value::nums(vec![2], vec![-1.5, 2.0]).to_string(),
        "num [2] -1.5 2"
    );
    assert_eq!(
        Value::chars(vec![5], "EDCBA").to_string(),
        "char [5] \"EDCBA\""
    );
}

#[test]
fn minus_signs() {
    for t in ["-1.5", "¯1.5", "_1.5"] {
        assert_eq!(parse_number(t), Some(-1.5), "{t}");
    }
    assert_eq!(parse_number("abc"), None);
}

#[test]
fn serialized_lines() {
    let m = parse_serialized("n|2 3|1 2 3 4 5 6").unwrap();
    assert_eq!(m.to_string(), "num [2 3] 1 2 3 4 5 6");
    assert_eq!(parse_serialized("n||10").unwrap().to_string(), "num [] 10");
    assert_eq!(
        parse_serialized("c|5|EDCBA").unwrap().to_string(),
        "char [5] \"EDCBA\""
    );
    assert_eq!(
        parse_serialized("n|2|1 2 3"),
        None,
        "count must match the shape"
    );
    assert_eq!(parse_serialized("x|1|1"), None);
}

#[test]
fn native_displays() {
    let cases = [
        ("10", "num [] 10"),
        ("1 2 3", "num [3] 1 2 3"),
        ("[1 2 3]  ", "num [3] 1 2 3"),
        ("[5]", "num [1] 5"),
        (",5", "num [1] 5"),
        ("\"EDCBA\"", "char [5] \"EDCBA\""),
        ("@A", "char [] \"A\""),
        ("EDCBA", "char [5] \"EDCBA\""),
        ("1 4\n2 5\n3 6", "num [3 2] 1 4 2 5 3 6"),
        ("(1 4\n 2 5\n 3 6)", "num [3 2] 1 4 2 5 3 6"),
        ("╭─\n╷ 1 2 3\n  4 5 6\n      ╯", "num [2 3] 1 2 3 4 5 6"),
    ];
    for (text, want) in cases {
        assert_eq!(
            parse_native(text, false).unwrap().to_string(),
            want,
            "{text:?}"
        );
    }
}

#[test]
fn unquoted_text_when_text_is_wanted() {
    assert_eq!(
        parse_native("42", true).unwrap().to_string(),
        "char [2] \"42\""
    );
    assert_eq!(parse_native("42", false).unwrap().to_string(), "num [] 42");
}

#[test]
fn strict_agreement() {
    let v = Value::nums(vec![3], vec![1.0, 2.0, 3.0]);
    assert!(agree(&v, &parse_native("1 2 3", false).unwrap()));
    assert!(
        !agree(&v, &Value::nums(vec![1, 3], vec![1.0, 2.0, 3.0])),
        "shape"
    );
    assert!(!agree(
        &Value::nums(vec![], vec![5.0]),
        &Value::nums(vec![1], vec![5.0])
    ));
    assert!(!agree(
        &Value::chars(vec![2], "42"),
        &Value::nums(vec![], vec![42.0])
    ));
    assert!(agree(
        &Value::nums(vec![], vec![0.1 + 0.2]),
        &Value::nums(vec![], vec![0.3])
    ));
}

#[test]
fn origin_shift() {
    let v = Value::nums(vec![2], vec![2.0, 3.0]);
    assert_eq!(shift_origin(&v, -1.0).to_string(), "num [2] 1 2");
}
