use ci_value::{Value, parse_number};

/// X_eTaL input bindings, `v := 1 2 3; t := "abc"`, as names and values.
/// None when a binding is not a plain array literal (a function operand such
/// as `u:f_ := { x -> x }`): those idioms are not run yet.
pub fn parse_bindings(text: &str) -> Option<Vec<(String, Value)>> {
    text.split(';')
        .map(|part| {
            let (name, value) = part.split_once(":=")?;
            let name = name.trim();
            let plain = !name.is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric());
            plain.then_some(())?;
            Some((name.to_string(), parse_literal(value.trim())?))
        })
        .collect()
}

/// An X_eTaL array literal: a number, numbers, a string, or
/// `2 3 r_eshape 1 2 3 4 5 6`.
pub fn parse_literal(text: &str) -> Option<Value> {
    if let Some(s) = text.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
        return Some(Value::chars(vec![s.chars().count()], s));
    }
    if let Some((shape, items)) = text.split_once("r_eshape") {
        let shape: Vec<usize> = shape
            .split_whitespace()
            .map(|t| t.parse().ok())
            .collect::<Option<_>>()?;
        let v = Value::nums(shape, numbers(items)?);
        return v.is_consistent().then_some(v);
    }
    let v = numbers(text)?;
    let shape = if v.len() == 1 { vec![] } else { vec![v.len()] };
    Some(Value::nums(shape, v))
}

fn numbers(text: &str) -> Option<Vec<f64>> {
    let v: Vec<f64> = text
        .split_whitespace()
        .map(parse_number)
        .collect::<Option<_>>()?;
    (!v.is_empty()).then_some(v)
}
