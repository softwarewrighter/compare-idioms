use ci_value::{Items, Value};

use crate::column::Column;

/// An input value written as a literal in the column's language.
pub fn literal(col: Column, value: &Value) -> String {
    let body = match &value.items {
        Items::Char(c) => text(col, &c.iter().collect::<String>()),
        Items::Num(v) => numbers(col, v),
    };
    match value.shape.as_slice() {
        [_, _, ..] => reshape(col, &value.shape, &body),
        _ => body,
    }
}

fn text(col: Column, s: &str) -> String {
    match col {
        Column::GnuApl | Column::J => format!("'{}'", s.replace('\'', "''")),
        _ => format!("{s:?}"),
    }
}

fn numbers(col: Column, v: &[f64]) -> String {
    let minus = match col {
        Column::GnuApl | Column::Bqn | Column::Uiua => "¯",
        Column::J => "_",
        _ => "-",
    };
    let items: Vec<String> = v
        .iter()
        .map(|x| x.to_string().replacen('-', minus, 1))
        .collect();
    match (col, items.len()) {
        (Column::Bqn, n) if n > 1 => format!("⟨{}⟩", items.join(",")),
        (Column::Uiua, n) if n > 1 => format!("[{}]", items.join(" ")),
        _ => items.join(" "),
    }
}

fn reshape(col: Column, shape: &[usize], body: &str) -> String {
    let dims: Vec<String> = shape.iter().map(|n| n.to_string()).collect();
    match col {
        Column::GnuApl => format!("{}⍴{body}", dims.join(" ")),
        Column::J => format!("{} $ {body}", dims.join(" ")),
        Column::Bqn => format!("{}⥊{body}", dims.join("‿")),
        Column::Uiua => format!("↯{} {body}", dims.join("_")),
        Column::Xetal => format!("{} r_eshape {body}", dims.join(" ")),
        Column::K | Column::K3 | Column::Kbm => format!("{}#{body}", dims.join(" ")),
    }
}
