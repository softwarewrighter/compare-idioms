//! compare-idioms: run the same idiom in several array languages on the same
//! inputs and check the outputs agree.

mod attrib;
mod cell;
mod check;
mod regtests;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use ci_catalog::{Idiom, load};
use ci_lang::Column;
use clap::{Parser, Subcommand};

const LONG_ABOUT: &str = "\
Run the same idiom in several array languages on the same inputs and check
that the outputs agree.

The idioms are read in place from X_eTaL's Rosetta data
(XETAL, default ../X_eTaL). Each language's result is read back as a value
and printed in one canonical form (num [2 3] 1 2 3 4 5 6), then compared
strictly: shape, kind and items. Index results are shifted to the
language's index origin.

Columns: xetal, gnu-apl, j, bqn, k (ngn/k), k3 (Kona), kbm (k edu under
QEMU), uiua. Runtimes are run through scripts/runtimes.sh.

AI CODING AGENT INSTRUCTIONS:
  1. Run `compare-idioms check` to see every idiom in every column.
  2. `compare-idioms run IDIOM COLUMN` prints one cell's verdict and value;
     the reg-rs tests in reg/ run exactly this.
  3. `compare-idioms tests` adds a reg-rs test for each new agreeing cell;
     review the new reg/*.out files before committing them.
  4. A cell that differs is never edited to pass: record why in docs/plan.md.";

const VERSION: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    "\nCopyright (c) 2026 Michael A Wright\nLicense: MIT\nRepository: ",
    env!("CARGO_PKG_REPOSITORY"),
    "\n\nBuild Information:\n  Host: ",
    env!("BUILD_HOST"),
    "\n  Commit: ",
    env!("BUILD_COMMIT"),
    "\n  Build Time: ",
    env!("BUILD_TIME"),
);

#[derive(Parser)]
#[command(name = "compare-idioms", version = VERSION, about, long_about = LONG_ABOUT)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Run every idiom in every column and report which agree
    Check,
    /// Run one idiom in one column; print the verdict and the value
    Run { idiom: String, column: String },
    /// Add a reg-rs test for each agreeing cell that has none
    Tests,
    /// Regenerate the "Idioms by source" section of ATTRIBUTIONS.md
    Attributions,
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("compare-idioms: {e}");
            ExitCode::from(2)
        }
    }
}

fn run(cli: Cli) -> Result<ExitCode, String> {
    let root = cell::find_root()?;
    if let Cmd::Attributions = cli.cmd {
        println!("{} idioms listed", attrib::write_attributions(&root)?);
        return Ok(ExitCode::SUCCESS);
    }
    let idioms = load(&data_path(&root))?;
    match cli.cmd {
        Cmd::Run { idiom, column } => run_one(&root, &idioms, &idiom, &column),
        Cmd::Check => {
            let differ = check::report(&idioms, &check::check_all(&root, &idioms));
            Ok(if differ == 0 {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            })
        }
        Cmd::Tests => {
            let names: Vec<_> = idioms
                .iter()
                .map(|i| (i.id.clone(), i.name.clone()))
                .collect();
            let made = regtests::create_tests(&root, &names, &check::check_all(&root, &idioms))?;
            println!("created {made} tests");
            Ok(ExitCode::SUCCESS)
        }
        Cmd::Attributions => unreachable!("handled above"),
    }
}

fn run_one(root: &Path, idioms: &[Idiom], id: &str, column: &str) -> Result<ExitCode, String> {
    let col = Column::from_name(column).ok_or(format!("unknown column {column}"))?;
    let idiom = idioms
        .iter()
        .find(|i| i.id == id)
        .ok_or(format!("unknown idiom {id}"))?;
    let (verdict, _) = cell::run_cell(root, idiom, col)?;
    println!("{}", verdict.line());
    Ok(ExitCode::SUCCESS)
}

fn data_path(root: &Path) -> PathBuf {
    let xetal = std::env::var_os("XETAL").map_or_else(|| root.join("../X_eTaL"), PathBuf::from);
    xetal.join("demos/rosetta/data.toml")
}
