use crate::value::Value;

/// A number as the array languages print it: the minus sign may be `-`, the
/// APL and BQN high minus `¯`, or J's `_`.
pub fn parse_number(token: &str) -> Option<f64> {
    let body = token.trim_start_matches(['-', '¯', '_']);
    let x: f64 = body.parse().ok()?;
    Some(if body.len() < token.len() { -x } else { x })
}

/// The line a language's serializer prints: `kind|shape|items`, where kind is
/// `n` (numbers) or `c` (characters), shape is the axis lengths separated by
/// spaces (empty for a scalar), and items are the numbers separated by
/// spaces, or the characters as they are.
pub fn parse_serialized(line: &str) -> Option<Value> {
    let mut parts = line.trim_start().splitn(3, '|');
    let (kind, shape, items) = (parts.next()?, parts.next()?, parts.next()?);
    let shape: Vec<usize> = shape
        .split_whitespace()
        .map(|t| t.parse().ok())
        .collect::<Option<_>>()?;
    let value = match kind {
        "c" => Value::chars(shape, items.trim_end_matches(['\n', '\r'])),
        "n" => Value::nums(
            shape,
            items
                .split_whitespace()
                .map(parse_number)
                .collect::<Option<_>>()?,
        ),
        _ => return None,
    };
    value.is_consistent().then_some(value)
}
