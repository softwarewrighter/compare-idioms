# compare-idioms: plan

Status: draft, 2026-10-06. Nothing below is built yet.

## Goal

The starting brief:

> I will now try to combine FinnAPL idiom library, J Phrases, BQN Crate, Gene
> McDonnell's K Idiom List, and
> https://github.com/codereport/array-language-comparisons to see if I can
> build a tool that tests idioms across languages, by running them with the
> same inputs, checking for the same outputs. The tricky part is when an idiom
> for one language cannot easily be applied to another language (e.g., lambdas
> in APL2)

So the project has three deliverables:

1. A **catalog** of idioms, each with one implementation per language and a
   pointer back to where it came from.
2. A **harness**, written in Rust, that runs every implementation of an idiom
   on the same inputs and checks that the outputs agree.
3. A **regression library** in reg-rs: every tested idiom is a golden test, so
   a runtime upgrade or a catalog edit that changes an answer is caught.

Languages: BQN, J, K in three dialects (Kona, ngn/k and kbm), Uiua, GNU APL
and X_eTaL. Dyalog APL is deferred.

## Third-party software and data

Rules set by the user on 2026-10-06:

- Anything cloned or downloaded (language runtimes, tools, idiom sources)
  goes under `work/`, which is gitignored.
- No third-party software is redistributed from this repository.
- Nothing is pushed that was not created here.

What follows from them:

- Runtimes installed by a package manager (Homebrew, cargo) live outside the
  repository. Runtimes built from source are cloned and built in
  `work/runtimes/`. The repository holds only the scripts that do this and a
  file of pinned versions and commits.
- Idiom sources are downloaded to `work/sources/`. The repository holds the
  script that fetches them and the record of where each came from.
- `../X_eTaL` and `../kbm-fork` are read and run in place, by a configurable
  path. Their files are not copied here.

### Idioms from a source

Decided 2026-10-06: an idiom expression taken from a source may be committed
and used in tests, with attribution. The user's reasoning: a single idiom is
not copyrightable, but a source's list might be, as a compilation. So the aim
is to give credit without redistributing anyone's list.

How that is done:

- **Credit travels with the idiom.** Each catalog entry names its source and
  the source's own identifier. Each reg-rs test repeats the credit in its
  `desc` field, which is part of the committed `.rgt` file.
- **`ATTRIBUTIONS.md` groups idioms by source**, by this project's idiom ids
  only. It carries no expressions and none of a source's order, numbering,
  headings or descriptions. The per-source lists are generated from the
  catalog.
- **The catalog is this project's own arrangement**: its own ids, its own
  categories, its own descriptions. An idiom is added because it is tested
  across languages, never as part of importing a source.
- **Whole sources stay in `work/sources/`.** They are downloaded for
  cross-referencing and never committed.

One thing this does not solve: if the catalog one day covers nearly every
entry of one source, the selection has been reproduced whatever the layout.
That is far off, and it matters most for the sources that grant no license
(see the table below). It is worth another look before M4 goes deep into any
single source.

## What is known and what is not

Checked on this machine (arm64 macOS) on 2026-10-06:

- Six runtimes are installed and answer `1+1` (`scripts/runtimes.sh doctor`):

  | Column | Runtime | Version | Where |
  |---|---|---|---|
  | `k3` | Kona | 20211225 | Homebrew |
  | `gnu-apl` | GNU APL | 2.0 | Homebrew |
  | `j` | J | 9.7.1 | Homebrew cask, called as `jcon` |
  | `uiua` | Uiua | 0.19.1 | `cargo install` |
  | `bqn` | CBQN | commit c893d3e7 | `work/runtimes/CBQN`, built with `FFI=0` |
  | `k` | ngn/k | commit b9eeb91e | `work/runtimes/ngn-k`, a native arm64 build |

- kbm is not built yet. It needs `nasm`, which is not installed.
- `../X_eTaL` has a built `target/debug/xetal`.
- `/usr/bin/jconsole` is **Java's JConsole, not J**. The J cask links J as
  both `jconsole` and `jcon` in Homebrew's bin; the scripts use `jcon`.
- GNU APL has two habits a harness must allow for. It does not exit at end
  of input, so its input must end with `)OFF`. And it starts a background
  `APserver` that keeps the output pipe open, so it is run with `--noSV`.
  It also writes `.apl.history` where it runs (gitignored).
- Installing GNU APL brought in GTK 3 and its dependencies and upgraded
  Homebrew's readline. The J cask put three apps in `/Applications`.

## Results so far

2026-10-06, `scripts/idiom.py check`: the 16 Rosetta idioms that have a plain
input and an expected output, run in all seven columns (112 cells) with
X_eTaL's own cells as the candidates. 108 agree with X_eTaL's expected
output, 10 of those after allowing for counting from 0 (`where` and
`index-of` in J, BQN, both Ks and Uiua). Each agreeing cell is a reg-rs test,
`reg/idiom-<idiom>-<column>.rgt`, plus one `runtime-<name>` test per
runtime: 115 tests, all passing.

The comparison is lenient (items printed, not types or shapes), so "agrees"
means "passed one input". Four cells differ, and all four are wrong in
X_eTaL's data, which its own checks never ran:

| Cell | Expression | What happened |
|---|---|---|
| rotate, ngn/k | `1!v` | gives `0 0 0 0 0`: in ngn/k `!` is not rotate (it is in K3, where the same cell works) |
| numbers, BQN | `•ParseFloat t` | "Malformed input": parses one number, not a list |
| numbers, Uiua | `⋕ t` | "Cannot parse into number": same problem |
| numbers, Kona | `.t` | the ngn/k cell tried unchanged; prints a garbage float |

The `k3` column has no cells of its own: it runs X_eTaL's ngn/k cells
unchanged in Kona, and 15 of 16 happen to work.

Not yet done:

- The other 15 Rosetta idioms (function operands, I/O, no expected output)
  and the 18 mainstream-only idioms have not been run anywhere.
- No idiom source has been downloaded or parsed. Counts and numbering
  schemes are from web pages, not from the data.
- Gemini has not answered: every request on 2026-10-06 got "high demand"
  (503), and `gemini-pro-latest` is over this key's quota (429).

A Linux machine is available if needed (the user has an Arch Linux system).
The six runtimes above do not need it. kbm might: the fork's Linux builds run
as ordinary programs there, with no emulator. QEMU on the Mac stays the plan
unless it proves too slow or fragile.

## What X_eTaL already has

Read on 2026-10-06 from `../X_eTaL`.

- `demos/rosetta/data.toml` holds 31 idioms across 7 languages: APL2, Dyalog
  APL, J, BQN, ngn/k, Uiua and X_eTaL. It has four tables keyed by idiom and
  then language: `input`, `source`, `output`, `notes`.
- Only the X_eTaL cells are ever run. `scripts/rosetta-check.py` evaluates
  each X_eTaL cell with `xetal eval -e "<input>; <source>"` and compares the
  printed result. Inputs and outputs exist only for X_eTaL, and only for
  about half the idioms.
- The file's own header says the K and Uiua cells "were added from memory and
  are reviewed by the user". No other language's cell has been executed.
- `docs/idioms.md` is generated from that data.
- X_eTaL counts from 1, like APL2. J, BQN, ngn/k and Uiua count from 0.

This changes the plan in two ways:

- **The first idiom set is X_eTaL's** (decided 2026-10-06): the 31 Rosetta
  idioms plus the 18 idioms that appear only in the mainstream table of
  `docs/idioms.md`, 49 in all. For each, the matching idiom is found in every
  target language except Dyalog.
- **The first useful result is to run X_eTaL's own table.** Those 31 idioms
  are already lined up across six of this project's languages and none of the
  non-X_eTaL cells has been checked. The harness spike uses them as its pilot
  set, read in place from `../X_eTaL`.
- **The catalog uses the same shape**: TOML, keyed by idiom and then
  language, with the same four tables, extended with provenance and status.
  A result can then be handed back to X_eTaL without translation.

## Idiom sources

| Source | Where | Form | License |
|---|---|---|---|
| X_eTaL Rosetta data (31 idioms, 7 languages) | `../X_eTaL/demos/rosetta/data.toml` | TOML | MIT, the user's own |
| FinnAPL idiom library (1984, 700+ one-liners) | [APL Wiki copy](https://old.aplwiki.com/FinnAplIdiomLibrary) ([raw](https://old.aplwiki.com/FinnAplIdiomLibrary?action=raw)), original at `finnapl.fi/idilib.htm`, [Dyalog idiom search](https://miserver.dyalog.com/Examples/Applications/Idiom_Search.mipage) | wiki text, numbered | not found |
| APL2 idioms | [IBM APL2IDIOMS.pdf](https://public.dhe.ibm.com/ps/products/apl2/info/APL2IDIOMS.pdf) | PDF | not found |
| The APL Idiom List (Perlis and Rugaber) | [softwarepreservation.org](https://www.softwarepreservation.org/projects/apl/Papers/THEAPLIDIOMLIST) | scanned paper | not found |
| APLcart (2000+ phrases, includes FinnAPL and APL2 entries) | [aplcart.info](https://aplcart.info), `github.com/abrudz/aplcart`, `table.tsv` | TSV | MIT; table content may be copied without the notice |
| J Phrases (Burke, Hui, Iverson, McDonnell, McIntyre) | [jsoftware.com](https://www.jsoftware.com/help/phrases/title.htm), [J wiki](https://code.jsoftware.com/wiki/Help/Phrases/1._A._Conventions) | HTML chapters | all rights reserved, Jsoftware; no license granted |
| APL to J phrase book | [J wiki APL2JPhraseBook](https://code.jsoftware.com/wiki/APL2JPhraseBook) | wiki | not found |
| BQNcrate | [mlochbaum.github.io/bqncrate](https://mlochbaum.github.io/bqncrate/), [repo](https://github.com/mlochbaum/bqncrate), `table.tsv` | TSV | MIT |
| K idiom list (Eugene McDonnell, k2) | [Kona wiki Idioms](https://github.com/kevinlawler/kona/wiki/Idioms), ["Learning K programming: idiom by idiom"](https://nsl.com/papers/idioms_K3.pdf) (K2/K3, 199 pages) | wiki, PDF | not found |
| Q Phrasebook (port of McDonnell's list to q) | [code.kx.com/phrases](https://code.kx.com/phrases/intro/), [repo](https://github.com/kxcontrib/phrases) | markdown | CC BY 4.0 per the site: attribution required |
| array-language-comparisons (codereport) | [repo](https://github.com/codereport/array-language-comparisons); a local fork is at `../array-language-comparisons-fork` (not yet read) | markdown tables, `comparisons/`, `code/` | MIT |
| ngnkcart (ngn/k snippets, named in ngn/k's readme) | `github.com/secwang/ngnkcart` | not yet looked at | not found |
| Uiua | [uiua.org](https://www.uiua.org) docs | no idiom list found | n/a |

License findings, 2026-10-06 (details and requirements per source are in
`ATTRIBUTIONS.md`):

- No source found so far restricts commercial use.
- One Creative Commons source: the Q Phrasebook, CC BY 4.0. It requires
  attribution, a link to the license and a note of changes.
- J Phrases is "All Rights Reserved" with no license granted.
- No terms were found for FinnAPL, McDonnell's K list, IBM's APL2 list or the
  Perlis and Rugaber paper. The APL Wiki and J Wiki pages refused the fetch,
  so FinnAPL's terms are unread, not absent.

Three things about these sources shape the design:

- **FinnAPL is the spine.** McDonnell's K list is a port of FinnAPL, the Q
  Phrasebook is a port of McDonnell's list, and APLcart includes the FinnAPL
  entries. If McDonnell's list kept FinnAPL's numbering, the FinnAPL number
  is the join key across APL and K. That needs confirming from the data. The
  Q Phrasebook says it renumbered, and publishes an index from old numbers
  to new.
- **BQNcrate was started as a clone of APLcart**, so its rows may line up with
  APLcart rows. Also to confirm.
- **Uiua has no idiom collection** that the search turned up, so its column
  will mostly be written here.

## Runtimes

| Language | Runtime | How it gets here | Notes |
|---|---|---|---|
| BQN | CBQN, `github.com/dzaima/CBQN` | cloned and built in `work/runtimes/` | no Homebrew formula found |
| J | J 9.7 | `brew install --cask j` | name clash with Java's `jconsole`; use a full path |
| K (historical) | Kona, a K3 implementation | `brew install kona` | closest to the dialect McDonnell's list was written in |
| K (modern) | ngn/k, `codeberg.org/ngn/k` | cloned and built in `work/runtimes/` | AGPL-3.0; upstream says it is no longer maintained and points to `codeberg.org/growler/k` |
| K (subset) | kbm, `../kbm-fork` | built and run in place | see [kbm](#kbm) |
| Uiua | `uiua` | `cargo install uiua` | pre-1.0, syntax changes between releases; pin the version |
| GNU APL | GNU APL 2.0 | `brew install gnu-apl` | the column is named `gnu-apl`, not APL2 (decided 2026-10-06); no dfns, which is the "lambdas in APL2" case from the brief; X_eTaL's APL2 cells are its candidates |
| X_eTaL | `../X_eTaL/target/debug/xetal` | built in that repository | `xetal eval -e "<input>; <source>"` |
| APL (modern) | Dyalog 20.0 | deferred | see [Dyalog](#dyalog-apl) |

Each runtime gets a pinned version or commit recorded in the repository, and
one reg-rs smoke test that evaluates a trivial expression, so a missing or
changed runtime fails loudly and separately from the idiom tests.

### K: three dialects

Decided 2026-10-06: K is carried as three separate columns.

- `k3`, Kona. McDonnell's list is k2 and Kona runs K3, so the list should run
  nearly as written. This is the reference column for the historical list.
- `k`, ngn/k. A k6-style dialect in which many of those expressions need
  rewriting. It is the K column of X_eTaL's Rosetta data.
- `kbm`. See below.

### kbm

Read on 2026-10-06 from `../kbm-fork` (branch `feat/apple-clang-portable-qemu`).

kbm is Arthur Whitney's k edu (shakti) ported to BareMetal-OS by Jack
Andrews. The fork adds portable builds for other CPUs and operating systems.

How it differs from the other runtimes:

- **It is not a host program on a Mac.** Decided 2026-10-06: kbm runs under
  QEMU. That is the fork's documented Mac path, `test/qemu-portable.sh`:
  build with `make PORTABLE=1`, boot BareMetal under `qemu-system-x86_64`,
  and talk to k over the serial console. It needs `../BareMetal-OS`
  (present) and `nasm` (not installed). It has not been tried on this
  machine.
- **It is driven as a transcript.** Lines go in, echoed results come out, as
  in the fork's own `test/golden/basic.expected`. Booting once per idiom
  would be slow, so the kbm adapter sends every case in one session.
- **It is a real subset.** The fork's goldens record that `&` (where) and
  `^` (sort) answer `nyi`, `?` on a list is a type error, and every float
  prints as `?.?` because float formatting is stubbed out. Idioms that need
  those are not applicable in kbm, with the reason "outside the dialect's
  subset". They are not rewritten into something else.

Two consequences for the harness:

- Adapters must support a batch mode (many cases, one process). This also
  helps the slower-starting runtimes.
- kbm cannot serialize its own results into the canonical form if a float is
  involved, so its adapter parses k's native display on the Rust side.

### Dyalog APL

Decided 2026-10-06: the Dyalog column is deferred. Nothing is installed and
no milestone depends on it. The notes below are kept for when the question is
taken up again.

- Dyalog's free basic license covers "personal or non-commercial use and
  experimentation". Dyalog's own summary is that you are fine as long as you
  are not making or trying to make money from the use.
- Whether work under softwarewrighter counts as non-commercial is the user's
  call, and the
  [terms and conditions](https://www.dyalog.com/uploads/documents/Terms_and_Conditions.pdf)
  should be read first. This plan has only read Dyalog's summary page.
- X_eTaL's Rosetta data has a Dyalog column. It stays unrun until this is
  decided.

## Design

### The catalog

TOML in the shape of X_eTaL's Rosetta data: keyed by idiom and then language.
One record per idiom:

- an id, a name and a one-line description;
- source references (for example FinnAPL number, BQNcrate row, J Phrases
  section, X_eTaL Rosetta id); these are the per-idiom credits that
  `ATTRIBUTIONS.md` is generated from;
- the argument shape: how many arguments and what kind;
- test cases: inputs and the expected result, in a language-neutral form;
- per language: the expression, where it came from (copied, translated,
  suggested by an LLM, written here), and a status.

Status is one of: passes, differs, not applicable, untested. "Not applicable"
carries a reason from a fixed list, so the gaps can be counted and reported.

### Same inputs, same outputs

Each language prints arrays its own way, so comparing printed output directly
would fail on formatting alone. Each language gets an adapter that does two
things: turns a neutral input value into a literal in that language, and
turns the result into one canonical text form (type, shape, elements). The
harness compares canonical text. Where the language can serialize its own
result, it does; where it cannot (kbm), the adapter parses the native display.

The differences that the canonical form or the status has to account for:

- **Index origin.** FinnAPL, APL2 and X_eTaL count from 1; BQN, J, K and Uiua
  count from 0. Idioms that return indices need a declared adjustment.
- **Arrays against lists.** K has nested lists, not rank-N arrays. A matrix
  result is a list of lists there.
- **Booleans and numbers.** Some languages have a boolean type, some use 0 and 1.
- **Scalars and one-element vectors, and typed empty arrays.**
- **Boxes.** J and Uiua box nested data; APL2 nests directly.
- **Floating point.** Compare with a tolerance, stated per idiom.

### Filling a language column

An expression for an idiom in a language can come from four places, and the
catalog records which:

- **copied** from a named source that has it in that language;
- **translated** here from a source's expression in another language;
- **suggested by an LLM**;
- **written here**.

The LLM route was proposed by the user on 2026-10-06: give a model other than
the one writing this project an idiom, ask it to check the idiom and to
suggest the equivalent in each target language. It helps in two ways. It
fills the columns no source covers, Uiua above all. And it means an
expression is produced for this project instead of being lifted from a list,
which eases the question of redistributing lists.

How it fits:

- **The harness is the judge, not the model.** A suggestion is a candidate.
  It enters the catalog only after it has run on the idiom's inputs and
  matched the expected output. X_eTaL's Rosetta data is the cautionary case:
  its K and Uiua cells were written from memory and still await review.
- **The model's check is a second opinion**, useful for catching an idiom
  whose description and expression disagree, or two expressions that agree
  on the test inputs but differ in general. It prompts more test cases; it
  does not pass a test.
- **The model is recorded.** Provenance names the model and the date, so a
  cell can be traced and redone.
- **Credit is still due.** When a suggested expression is the same idiom a
  source documents, the catalog entry still credits that source. A model may
  well be repeating what it read there.

Which model (decided 2026-10-06): Google's Gemini, through its API.
`scripts/suggest.py` sends each X_eTaL idiom with the cells already proposed
and asks for a verdict and an expression per target column; answers go to
`work/suggestions/`. It has not been run: it needs the user's API key.

Local models were tried first, through Ollama, on five Rosetta idioms (20
cells), scored against X_eTaL's table:

| Model | Cells matching | What went wrong |
|---|---|---|
| qwen2.5-coder 7b | 1 of 20 | invented words (`where b`, `iota n`) |
| devstral-small-2 24b | 4 of 20 | APL glyphs in BQN, K and Uiua; forgot the 1-origin |
| gemma4 31b | 0 of 20 | APL glyphs everywhere; ignored the JSON-only instruction |

A match with X_eTaL's table is not proof of correctness, but the misses were
plainly wrong. The local models are not good enough for this job.

### When an idiom does not carry over

This is the part the brief calls tricky. The plan is to classify, not to
force a translation. Starting reasons:

- needs first-class or anonymous functions (GNU APL, APL2);
- depends on rank-N arrays (K);
- depends on a system variable or setting such as index origin or comparison
  tolerance;
- depends on a primitive with no counterpart;
- is outside the dialect's subset (kbm);
- is about the language's own notation (for example tacit or stack
  manipulation), not a computation.

X_eTaL's Rosetta data already does this in prose: a missing cell is shown as
"not directly expressible" or "no concise built-in idiom". The reason list
here should cover those two.

An idiom marked not applicable in one language still counts as tested in the
others.

### reg-rs layout

Following X_eTaL's conventions:

- Baselines live in `reg/`. Every reg-rs command goes through
  `scripts/reg.sh`, which sets `REG_RS_DATA_DIR`.
- `reg/*.rgt`, `reg/*.out` and `reg/*.err` are committed. `reg/*.tdb*` are
  caches and are gitignored.
- A baseline is created only after its output has been reviewed. `reg-rs
  rebase` is used only when a change in output is intended, and the commit
  message says so.

Tests:

- One per idiom and language, `idiom-<id>-<lang>`. Its command runs that
  implementation on every test case and prints the canonical results. The
  golden catches a change in that one language. Its `desc` credits the
  source of that expression.
- One per idiom, `idiom-<id>-agree`, prints which languages agree with the
  expected result. This is the cross-language check.
- One smoke test per runtime, `runtime-<lang>`.

Per-language tests mean a missing runtime fails only its own tests. Because
kbm and some others are run in batch, the harness caches one batch run per
language and the per-idiom tests read from it; how that fits reg-rs's
one-command-per-test model is a question for the spike.

## Milestones

**M0. Bootstrap.** Done: CLAUDE.md, AGENTS.md, agentrail saga, `.gitignore`,
`scripts/reg.sh`, `ATTRIBUTIONS.md`, this plan.

**M1. Runtimes.** Partly done: `scripts/runtimes.sh` installs or builds six
runtimes with pinned versions and commits, and its `doctor` checks them.
Left: kbm under QEMU (needs `nasm` and a first working build on this
machine), X_eTaL in the doctor check, and one reg-rs smoke test per runtime.

**M2. Harness spike on X_eTaL's table.** The Rust crate, the canonical value
format and one adapter per language. Run the 31 Rosetta idioms read in place
from `../X_eTaL`. The outcome is a report of which cells pass, which differ
and which never ran, goldens in `reg/`, and the generator for the "Idioms
by source" section of `ATTRIBUTIONS.md`. The spike exists to find out
whether the canonical form survives contact with all the runtimes before any
bulk import.

**M3. Source ingestion.** Fetch each source into `work/sources/` with its
provenance and license. Read the terms still marked "not found", FinnAPL's
first. Parse the machine-readable ones (APLcart, BQNcrate,
FinnAPL raw). Confirm or refute the FinnAPL-number join and the APLcart to
BQNcrate row alignment. Read `../array-language-comparisons-fork` if the user
adds it.

**M4. Grow the catalog.** Work through FinnAPL category by category, filling
each language column (from sources, by translation, or from an LLM's
suggestions run through the harness) and classifying the idioms that do not
carry over.

**M5. Reports.** A coverage table of idiom by language (passes, differs, not
applicable, untested), generated from the catalog and the test results, in a
form X_eTaL's Rosetta data can take back.

## Decisions

Made 2026-10-06:

1. **K dialects.** Three columns: Kona, ngn/k and kbm.
2. **Dyalog.** Deferred.
3. **Harness language.** Rust.
4. **Third-party material.** Clones and downloads under `work/`, nothing
   third-party redistributed, nothing pushed that was not created here.
5. **Idioms from a source.** Committed and tested with per-idiom attribution;
   `ATTRIBUTIONS.md` groups them by source; no source's list is reproduced.
6. **kbm.** Under QEMU on the Mac.
7. **The APL column** is GNU APL, labeled `gnu-apl`, not APL2.
8. **The LLM** is Gemini, through its API. Local models were tried and are
   not good enough.
9. **The first idiom set** is X_eTaL's 49 documented idioms.

Open:

1. The terms of FinnAPL, McDonnell's K list, IBM's APL2 list and the Perlis
   and Rugaber paper are unread.
2. How far to go into a single source that grants no license, before M4.
3. The Gemini API key, and which Gemini model to use.
