use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use ci_catalog::Idiom;
use ci_lang::{Column, Fault, decode, program};
use ci_value::{Value, agree, shift_origin};

/// What running one idiom in one column came to.
#[derive(Clone, Debug)]
pub enum Verdict {
    Agree(Value),
    Differs(Value),
    Fault(Fault),
    /// X_eTaL's data has no expression for this column.
    NoCell,
}

impl Verdict {
    /// One line: the verdict, then the canonical value or the fault.
    pub fn line(&self) -> String {
        match self {
            Verdict::Agree(v) => format!("agree {v}"),
            Verdict::Differs(v) => format!("differs {v}"),
            Verdict::Fault(f) => f.to_string(),
            Verdict::NoCell => "no cell".to_string(),
        }
    }
}

/// The result an idiom should give in a column: an index result is shifted
/// to the column's index origin.
pub fn expected(idiom: &Idiom, col: Column) -> Value {
    if idiom.index_result && col.zero_origin() {
        shift_origin(&idiom.expected, -1.0)
    } else {
        idiom.expected.clone()
    }
}

/// The program for one cell, or None when there is no expression.
pub fn cell_program(idiom: &Idiom, col: Column) -> Option<String> {
    let expr = idiom.cells.get(col.source_key())?;
    Some(program(col, &idiom.binding, &idiom.binding_text, expr))
}

/// Run one cell in its runtime and judge the result.
pub fn run_cell(root: &Path, idiom: &Idiom, col: Column) -> Result<(Verdict, String), String> {
    let Some(prog) = cell_program(idiom, col) else {
        return Ok((Verdict::NoCell, String::new()));
    };
    let output = run_runtime(root, col.runtime(), &prog)?;
    let want = expected(idiom, col);
    let verdict = match decode(col, &output, want.is_char()) {
        Ok(v) if agree(&v, &want) => Verdict::Agree(v),
        Ok(v) => Verdict::Differs(v),
        Err(f) => Verdict::Fault(f),
    };
    Ok((verdict, output))
}

/// `scripts/runtimes.sh run NAME` with the program on stdin: the one place
/// that knows each interpreter's flags and habits.
fn run_runtime(root: &Path, runtime: &str, prog: &str) -> Result<String, String> {
    let script = root.join("scripts/runtimes.sh");
    let mut child = Command::new(&script)
        .args(["run", runtime])
        .current_dir(root)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("{}: {e}", script.display()))?;
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(prog.as_bytes())
            .map_err(|e| e.to_string())?;
    }
    let out = child.wait_with_output().map_err(|e| e.to_string())?;
    Ok(String::from_utf8_lossy(&out.stdout).into_owned() + &String::from_utf8_lossy(&out.stderr))
}

/// The repository root: the nearest directory, from here up, that has
/// `scripts/runtimes.sh`.
pub fn find_root() -> Result<PathBuf, String> {
    let here = std::env::current_dir().map_err(|e| e.to_string())?;
    here.ancestors()
        .find(|d| d.join("scripts/runtimes.sh").is_file())
        .map(Path::to_path_buf)
        .ok_or_else(|| "not inside the compare-idioms repository".to_string())
}
