use std::path::Path;
use std::process::Command;

use ci_lang::Column;

use crate::cell::Verdict;
use crate::check::Cell;

const CREDIT: &str = "X_eTaL Rosetta data (Michael A Wright, MIT)";

/// A reg-rs test for every cell that agrees and has none yet. The test runs
/// `compare-idioms run IDIOM COLUMN`; its description carries the credit.
pub fn create_tests(
    root: &Path,
    names: &[(String, String)],
    cells: &[Cell],
) -> Result<usize, String> {
    let mut made = 0;
    for c in cells
        .iter()
        .filter(|c| matches!(c.verdict, Verdict::Agree(_)))
    {
        let test = format!("idiom-{}-{}", c.idiom, c.col.name());
        if root.join("reg").join(format!("{test}.rgt")).exists() {
            continue;
        }
        let name = names
            .iter()
            .find(|(id, _)| *id == c.idiom)
            .map_or(c.idiom.as_str(), |n| &n.1);
        reg_create(
            root,
            &test,
            &command(&c.idiom, c.col),
            &description(name, c.col),
        )?;
        made += 1;
    }
    Ok(made)
}

fn command(idiom: &str, col: Column) -> String {
    format!("target/release/compare-idioms run {idiom} {}", col.name())
}

fn reg_create(root: &Path, test: &str, command: &str, desc: &str) -> Result<(), String> {
    let out = Command::new(root.join("scripts/reg.sh"))
        .current_dir(root)
        .args([
            "create",
            "-t",
            test,
            "--timeout",
            "180",
            "-c",
            command,
            "--desc",
            desc,
        ])
        .output()
        .map_err(|e| e.to_string())?;
    if out.status.success() {
        Ok(())
    } else {
        Err(format!(
            "reg-rs create {test}: {}",
            String::from_utf8_lossy(&out.stderr)
        ))
    }
}

fn description(name: &str, col: Column) -> String {
    let what = match col {
        Column::K3 => "ngn/k expression run unchanged in Kona",
        Column::Kbm => "ngn/k expression run unchanged in kbm",
        _ => "expression",
    };
    format!(
        "{name} in {}: agrees with X_eTaL. Credit: {what} from {CREDIT}.",
        col.name()
    )
}
