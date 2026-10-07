use std::collections::BTreeMap;
use std::path::Path;

use ci_value::{Value, parse_native};
use serde::Deserialize;

use crate::binding::parse_bindings;

/// Idioms whose result is an index, so it depends on the language's index
/// origin. X_eTaL's data does not say which these are; decided here.
const INDEX_RESULT: &[&str] = &["where", "index-of"];

/// Idioms whose result is text although X_eTaL prints it bare (`42`).
const TEXT_RESULT: &[&str] = &["format"];

type Table = BTreeMap<String, BTreeMap<String, String>>;

#[derive(Deserialize)]
struct Data {
    idioms: Vec<String>,
    idiom_names: Vec<String>,
    source: Table,
    #[serde(default)]
    input: Table,
    #[serde(default)]
    output: Table,
}

/// One idiom that can be run: plain array inputs and an expected result.
#[derive(Clone, Debug)]
pub struct Idiom {
    pub id: String,
    pub name: String,
    /// The bindings as X_eTaL writes them, for the X_eTaL column.
    pub binding_text: String,
    pub binding: Vec<(String, Value)>,
    /// The result, counted from 1 where it is an index.
    pub expected: Value,
    /// The result is an index (see [`INDEX_RESULT`]).
    pub index_result: bool,
    /// Expression per X_eTaL data column (`apl2`, `j`, `bqn`, `k`, ...).
    pub cells: BTreeMap<String, String>,
}

/// Every idiom in `data.toml` that has plain inputs and an expected output,
/// in the data's order.
pub fn load(path: &Path) -> Result<Vec<Idiom>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let data: Data = toml::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
    let names: BTreeMap<_, _> = data.idioms.iter().zip(&data.idiom_names).collect();
    Ok(data
        .idioms
        .iter()
        .filter_map(|id| idiom(&data, id, names.get(id).map_or(id.as_str(), |n| n.as_str())))
        .collect())
}

fn idiom(data: &Data, id: &str, name: &str) -> Option<Idiom> {
    let binding_text = data.input.get(id)?.get("xetal")?.clone();
    let shown = data.output.get(id)?.get("xetal")?;
    Some(Idiom {
        id: id.to_string(),
        name: name.to_string(),
        binding: parse_bindings(&binding_text)?,
        binding_text,
        expected: parse_native(shown, TEXT_RESULT.contains(&id))?,
        index_result: INDEX_RESULT.contains(&id),
        cells: data.source.get(id).cloned().unwrap_or_default(),
    })
}
