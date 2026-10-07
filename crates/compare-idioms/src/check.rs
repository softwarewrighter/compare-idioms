use std::path::Path;
use std::thread;

use ci_catalog::Idiom;
use ci_lang::{Column, Fault};

use crate::cell::{Verdict, expected, run_cell};

/// One finished cell.
pub struct Cell {
    pub idiom: String,
    pub col: Column,
    pub verdict: Verdict,
    pub output: String,
}

/// Run every idiom in every column. Columns run in parallel; within a column
/// the idioms run one after another.
pub fn check_all(root: &Path, idioms: &[Idiom]) -> Vec<Cell> {
    let per_column: Vec<Vec<Cell>> = thread::scope(|s| {
        let handles: Vec<_> = Column::ALL
            .into_iter()
            .map(|col| s.spawn(move || idioms.iter().map(|i| one(root, i, col)).collect()))
            .collect();
        handles
            .into_iter()
            .map(|h| h.join().unwrap_or_default())
            .collect()
    });
    let mut cells: Vec<Cell> = per_column.into_iter().flatten().collect();
    let order = |id: &str| idioms.iter().position(|i| i.id == id);
    cells.sort_by_key(|c| {
        (
            order(&c.idiom),
            Column::ALL.iter().position(|k| *k == c.col),
        )
    });
    cells
}

fn one(root: &Path, idiom: &Idiom, col: Column) -> Cell {
    let (verdict, output) = run_cell(root, idiom, col)
        .unwrap_or_else(|e| (Verdict::Fault(Fault::Unreadable(e.clone())), e));
    Cell {
        idiom: idiom.id.clone(),
        col,
        verdict,
        output,
    }
}

/// The table, the cells that did not agree, and a count. Returns the number
/// of cells that differ.
pub fn report(idioms: &[Idiom], cells: &[Cell]) -> usize {
    print!("{:<12}", "idiom");
    Column::ALL.iter().for_each(|c| print!("{:>9}", c.name()));
    println!();
    for chunk in cells.chunks(Column::ALL.len()) {
        print!("{:<12}", chunk[0].idiom);
        chunk
            .iter()
            .for_each(|c| print!("{:>9}", short(&c.verdict)));
        println!();
    }
    for c in cells
        .iter()
        .filter(|c| !matches!(c.verdict, Verdict::Agree(_) | Verdict::NoCell))
    {
        let idiom = idioms
            .iter()
            .find(|i| i.id == c.idiom)
            .expect("cell of a known idiom");
        println!("\n## {}/{}: {}", c.idiom, c.col.name(), c.verdict.line());
        println!("   want: {}", expected(idiom, c.col));
        println!(
            "   output: {:?}",
            c.output.trim().chars().take(160).collect::<String>()
        );
    }
    summary(cells)
}

fn short(v: &Verdict) -> &'static str {
    match v {
        Verdict::Agree(_) => "ok",
        Verdict::Differs(_) => "DIFF",
        Verdict::Fault(Fault::NotImplemented) => "nyi",
        Verdict::Fault(Fault::Error(_)) => "err",
        Verdict::Fault(Fault::Unreadable(_)) => "??",
        Verdict::NoCell => "-",
    }
}

fn summary(cells: &[Cell]) -> usize {
    let count = |k: &str| cells.iter().filter(|c| short(&c.verdict) == k).count();
    let (ok, diff) = (count("ok"), count("DIFF"));
    println!(
        "\n{ok} of {} cells agree; {diff} differ; {} errors; {} not implemented (kbm); {} unreadable",
        cells.len(),
        count("err"),
        count("nyi"),
        count("??")
    );
    diff
}
