use crate::value::{Items, Value};

/// Strict agreement: the same shape, the same kind of item, and the same
/// items (numbers within a relative tolerance of 1e-9).
pub fn agree(got: &Value, want: &Value) -> bool {
    got.shape == want.shape
        && match (&got.items, &want.items) {
            (Items::Num(a), Items::Num(b)) => {
                a.len() == b.len() && a.iter().zip(b).all(|(x, y)| close(*x, *y))
            }
            (Items::Char(a), Items::Char(b)) => a == b,
            _ => false,
        }
}

fn close(x: f64, y: f64) -> bool {
    (x - y).abs() <= 1e-9 * x.abs().max(y.abs()).max(1.0)
}

/// The value with `by` added to every number: an index result counted from 1
/// becomes the same result counted from 0 with `by = -1.0`.
pub fn shift_origin(value: &Value, by: f64) -> Value {
    match &value.items {
        Items::Num(v) => Value::nums(value.shape.clone(), v.iter().map(|x| x + by).collect()),
        Items::Char(_) => value.clone(),
    }
}
