use crate::serial::parse_number;
use crate::value::Value;

/// Box-drawing characters Uiua and BQN draw around a matrix.
const BOX: &[char] = &['╭', '╮', '╯', '╰', '─', '│', '╷', '╵', '┌', '┐', '└', '┘'];

/// Read a language's own display of a value: a scalar (`10`), a vector
/// (`1 2 3`, `[1 2 3]`, `,5`), a string (`"EDCBA"`, `@A`), or a matrix of
/// numbers, one row per line (bare, boxed, or in k's parentheses).
///
/// `want_char` says the result should be text. APL, J and X_eTaL print a
/// string without quotes, so `42` may be the number or the text; with
/// `want_char` an unquoted line is read as characters.
pub fn parse_native(text: &str, want_char: bool) -> Option<Value> {
    let rows = rows(text);
    match rows.len() {
        0 => None,
        1 => line(&rows[0], want_char),
        _ => matrix(&rows),
    }
}

fn rows(text: &str) -> Vec<String> {
    text.lines()
        .map(|l| l.chars().filter(|c| !BOX.contains(c)).collect::<String>())
        .map(|l| {
            l.trim()
                .trim_start_matches('(')
                .trim_end_matches(')')
                .trim()
                .to_string()
        })
        .filter(|l| !l.is_empty())
        .collect()
}

fn line(l: &str, want_char: bool) -> Option<Value> {
    if let Some(s) = l.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
        return Some(Value::chars(vec![s.chars().count()], s));
    }
    if let Some(c) = l.strip_prefix('@').filter(|c| c.chars().count() == 1) {
        return Some(Value::chars(vec![], c));
    }
    let listed = l.starts_with('[') || l.starts_with(',');
    let body = l.trim_start_matches(['[', ',']).trim_end_matches(']');
    let nums: Option<Vec<f64>> = body.split_whitespace().map(parse_number).collect();
    match nums {
        Some(v) if !want_char && !v.is_empty() => {
            let shape = if v.len() == 1 && !listed {
                vec![]
            } else {
                vec![v.len()]
            };
            Some(Value::nums(shape, v))
        }
        _ => Some(Value::chars(vec![l.chars().count()], l)),
    }
}

fn matrix(rows: &[String]) -> Option<Value> {
    let parsed: Vec<Vec<f64>> = rows
        .iter()
        .map(|r| {
            r.split_whitespace()
                .map(parse_number)
                .collect::<Option<_>>()
        })
        .collect::<Option<_>>()?;
    let cols = parsed[0].len();
    if cols == 0 || parsed.iter().any(|r| r.len() != cols) {
        return None;
    }
    Some(Value::nums(vec![parsed.len(), cols], parsed.concat()))
}
