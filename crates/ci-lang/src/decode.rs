use std::fmt;

use ci_value::{Value, parse_native, parse_serialized};

use crate::column::Column;

/// Why a run produced no value.
#[derive(Clone, Debug, PartialEq)]
pub enum Fault {
    /// kbm answered `nyi`: the primitive is outside its subset.
    NotImplemented,
    /// The language reported an error; the first line of the report.
    Error(String),
    /// The output could not be read as a value.
    Unreadable(String),
}

impl fmt::Display for Fault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Fault::NotImplemented => write!(f, "nyi"),
            Fault::Error(m) => write!(f, "error {m}"),
            Fault::Unreadable(m) => write!(f, "unreadable {m}"),
        }
    }
}

/// k's error classes, which kbm prints after the failing token (`#rank`).
const K_CLASSES: &[&str] = &[
    "rank", "type", "length", "domain", "value", "index", "parse",
];

/// Read a run's output as a value. `want_char` says the expected result is
/// text, for displays that print text without quotes.
pub fn decode(col: Column, output: &str, want_char: bool) -> Result<Value, Fault> {
    if let Some(fault) = fault(output) {
        return Err(fault);
    }
    let found = if col.self_serializes() {
        output
            .lines()
            .map(str::trim_start)
            .find(|l| l.starts_with("n|") || l.starts_with("c|"))
            .and_then(parse_serialized)
    } else {
        parse_native(output, want_char)
    };
    found.ok_or_else(|| Fault::Unreadable(first_line(output)))
}

fn fault(output: &str) -> Option<Fault> {
    if output.split_whitespace().any(|w| w.ends_with("nyi")) {
        return Some(Fault::NotImplemented);
    }
    output
        .lines()
        .map(str::trim)
        .find(|l| is_error(l))
        .map(|l| Fault::Error(l.to_string()))
}

/// An error report in any of the languages: ngn/k `'type`, J `|domain error`,
/// BQN and Uiua `Error: ...`, GNU APL `DOMAIN ERROR`, Kona `type error`,
/// kbm `#rank`.
fn is_error(l: &str) -> bool {
    let k_class = K_CLASSES
        .iter()
        .any(|c| l.ends_with(c) && l.len() <= c.len() + 2);
    k_class || l.starts_with('\'') || l.to_lowercase().contains("error")
}

fn first_line(output: &str) -> String {
    output
        .lines()
        .map(str::trim)
        .find(|l| !l.is_empty())
        .unwrap_or("(no output)")
        .to_string()
}
