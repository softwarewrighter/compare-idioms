# compare-idioms saga

Build a Rust tool that runs the same idiom in several array languages on the
same inputs and checks for the same outputs, with a reg-rs regression library
of tested idioms. The full plan, sources, runtimes and open decisions are in
`docs/plan.md`; read it before any step.

Languages: BQN (CBQN), J, K in three dialects (Kona, ngn/k, and kbm from
`../kbm-fork`, a subset of K), Uiua, GNU APL and X_eTaL (`../X_eTaL`). Dyalog
APL is deferred.

## Milestones

- M1 Runtimes: scripts that install or build each runtime (clones under
  `work/runtimes/`), pinned versions, a `doctor` command, one reg-rs smoke
  test per runtime. kbm last: it runs under QEMU on the Mac.
- M2 Harness spike on X_eTaL's table: the Rust crate, canonical value format,
  one adapter per language, the 31 idioms of
  `../X_eTaL/demos/rosetta/data.toml` read in place, goldens in `reg/`.
- M3 Source ingestion into `work/sources/`: FinnAPL, APLcart, BQNcrate,
  J Phrases, McDonnell's K idiom list, Q Phrasebook,
  array-language-comparisons; provenance and license recorded;
  cross-reference by FinnAPL number.
- M4 Grow the catalog by FinnAPL category; classify idioms that do not carry
  over to a language.
- M5 Reports: coverage table of idiom by language, in a form X_eTaL's Rosetta
  data can take back.

## Rules

- Clones and downloads go under `work/` (gitignored). No third-party
  software is redistributed from this repository. Nothing is pushed that was
  not created here.
- `../X_eTaL` and `../kbm-fork` are read and run in place, never copied.
- Idioms from a source are committed with per-idiom attribution (catalog
  entry and reg-rs test `desc`); `ATTRIBUTIONS.md` groups them by source; no
  source's list is reproduced.
- reg-rs baselines live in `reg/`, always through `scripts/reg.sh`.
- Call J by full path: `/usr/bin/jconsole` on macOS is Java's, not J.
- American spellings; ASCII-only markdown docs.
