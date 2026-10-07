use ci_lang::{Column, Fault, decode, literal, program};
use ci_value::Value;

fn matrix() -> Value {
    Value::nums(vec![2, 3], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0])
}

#[test]
fn names_round_trip() {
    for c in Column::ALL {
        assert_eq!(Column::from_name(c.name()), Some(c));
    }
    assert_eq!(Column::K3.runtime(), "kona");
    assert_eq!(Column::Kbm.source_key(), "k");
    assert!(!Column::GnuApl.zero_origin() && Column::J.zero_origin());
}

#[test]
fn literals_per_language() {
    let v = Value::nums(vec![2], vec![-1.0, 2.0]);
    let want = [
        (Column::GnuApl, "¯1 2"),
        (Column::J, "_1 2"),
        (Column::Bqn, "⟨¯1,2⟩"),
        (Column::K, "-1 2"),
        (Column::Uiua, "[¯1 2]"),
    ];
    for (col, text) in want {
        assert_eq!(literal(col, &v), text, "{col:?}");
    }
    assert_eq!(literal(Column::Bqn, &matrix()), "2‿3⥊⟨1,2,3,4,5,6⟩");
    assert_eq!(literal(Column::K3, &matrix()), "2 3#1 2 3 4 5 6");
    assert_eq!(
        literal(Column::J, &Value::chars(vec![3], "it's")),
        "'it''s'"
    );
}

#[test]
fn programs() {
    let b = vec![("v".to_string(), Value::nums(vec![3], vec![1.0, 2.0, 3.0]))];
    assert_eq!(
        program(Column::Xetal, &b, "v := 1 2 3", "'+ r_/ v"),
        "v := 1 2 3; '+ r_/ v"
    );
    assert_eq!(program(Column::Kbm, &b, "", "+/v"), "v:1 2 3\n+/v");
    assert!(program(Column::GnuApl, &b, "", "+/V").starts_with("V←1 2 3\nR←+/V\n"));
    assert!(program(Column::Bqn, &b, "", "+´ v").starts_with("F←"));
}

#[test]
fn decoding() {
    let got = decode(Column::Bqn, "n|3|1 2 3\n\"n|3|1 2 3\"\n", false).unwrap();
    assert_eq!(got.to_string(), "num [3] 1 2 3");
    let got = decode(Column::Uiua, "[1 3 6 10]  \n", false).unwrap();
    assert_eq!(got.to_string(), "num [4] 1 3 6 10");
    assert_eq!(
        decode(Column::Kbm, "/nyi \n", false),
        Err(Fault::NotImplemented)
    );
    assert_eq!(
        decode(Column::Kbm, "#rank\n", false),
        Err(Fault::Error("#rank".into()))
    );
    assert!(matches!(
        decode(Column::J, "|domain error\n", false),
        Err(Fault::Error(_))
    ));
    assert!(matches!(
        decode(Column::K, "", false),
        Err(Fault::Unreadable(_))
    ));
}
