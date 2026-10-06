#!/usr/bin/env python3
"""Run X_eTaL's Rosetta idioms in the real runtimes and compare the results.

  scripts/idiom.py check             # every idiom with an input and an expected
                                     # output, in every column: agree or differ
  scripts/idiom.py run IDIOM COLUMN  # one cell: print what the runtime prints
  scripts/idiom.py tests             # a reg-rs test for every cell that agrees
  scripts/idiom.py attributions      # rewrite "Idioms by source" in ATTRIBUTIONS.md

A stopgap in Python so that tested idioms exist before the Rust harness does
(docs/plan.md, M2); the harness replaces it. X_eTaL's data is read in place
(XETAL, default ../X_eTaL), never copied here. The comparison is lenient: it
compares the items printed, not types or shapes, and accepts a result that is
the expected indices counted from 0. A cell that "agrees" has passed that
check on one input, no more.

Columns: xetal, gnu-apl (X_eTaL's APL2 cells), j, bqn, k (ngn/k),
k3 (Kona, trying X_eTaL's ngn/k cell unchanged), uiua."""
import os
import re
import subprocess
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
XETAL = Path(os.environ.get("XETAL", ROOT.parent / "X_eTaL"))
D = tomllib.load(open(XETAL / "demos/rosetta/data.toml", "rb"))
NAMES = dict(zip(D["idioms"], D["idiom_names"]))
CREDIT = "X_eTaL Rosetta data (Michael A Wright, MIT)"

# column -> (runtime name, X_eTaL column the candidate comes from)
COLS = {"xetal": ("xetal", "xetal"), "gnu-apl": ("gnu-apl", "apl2"), "j": ("j", "j"), "bqn": ("cbqn", "bqn"),
        "k": ("ngn-k", "k"), "k3": ("kona", "k"), "uiua": ("uiua", "uiua")}

def parse(binding):
    out = []
    for part in binding.split(";"):
        name, val = [s.strip() for s in part.split(":=")]
        if val.startswith('"'):
            out.append((name, ("str", val.strip('"'))))
        elif "r_eshape" in val:
            shape, items = val.split("r_eshape")
            out.append((name, ("mat", shape.split(), items.split())))
        elif len(val.split()) == 1:
            out.append((name, ("int", val)))
        else:
            out.append((name, ("vec", val.split())))
    return out

def lit(col, v):
    kind = v[0]
    if col == "gnu-apl":
        return {"str": lambda: f"'{v[1]}'", "int": lambda: v[1], "vec": lambda: " ".join(v[1]),
                "mat": lambda: f"{' '.join(v[1])}⍴{' '.join(v[2])}"}[kind]()
    if col == "j":
        return {"str": lambda: f"'{v[1]}'", "int": lambda: v[1], "vec": lambda: " ".join(v[1]),
                "mat": lambda: f"{' '.join(v[1])} $ {' '.join(v[2])}"}[kind]()
    if col == "bqn":
        return {"str": lambda: f'"{v[1]}"', "int": lambda: v[1], "vec": lambda: "⟨" + ",".join(v[1]) + "⟩",
                "mat": lambda: "‿".join(v[1]) + "⥊⟨" + ",".join(v[2]) + "⟩"}[kind]()
    if col in ("k", "k3"):
        return {"str": lambda: f'"{v[1]}"', "int": lambda: v[1], "vec": lambda: " ".join(v[1]),
                "mat": lambda: f"{' '.join(v[1])}#{' '.join(v[2])}"}[kind]()
    if col == "uiua":
        return {"str": lambda: f'"{v[1]}"', "int": lambda: v[1], "vec": lambda: "[" + " ".join(v[1]) + "]",
                "mat": lambda: "↯" + "_".join(v[1]) + " [" + " ".join(v[2]) + "]"}[kind]()

def program(col, idiom):
    src = D["source"][idiom].get(COLS[col][1])
    binding = D["input"][idiom]["xetal"]
    if src is None:
        return None
    if col == "xetal":
        return f"{binding}; {src}"
    arrow = {"gnu-apl": "←", "j": " =: ", "bqn": " ← ", "k": ":", "k3": ":", "uiua": " ← "}[col]
    lines = [(n.upper() if col == "gnu-apl" else n) + arrow + lit(col, v) for n, v in parse(binding)]
    return "\n".join(lines + [src])

def norm(text):
    text = re.sub(r"[⟨⟩\[\]()┌─╵┘╭╷╯│·\"',;]", " ", text)
    toks = []
    for t in text.split():
        t = re.sub(r"^[¯_](?=[\d.])", "-", t)
        try:
            f = float(t); t = str(int(f)) if f == int(f) else str(f)
        except ValueError:
            pass
        toks.append(t)
    return toks

def verdict(got, want):
    g, w = norm(got), norm(want)
    if g == w:
        return "match"
    try:
        if len(g) == len(w) and all(int(a) + 1 == int(b) for a, b in zip(g, w)):
            return "match-0-origin"
    except ValueError:
        pass
    return "differs"


def testable():
    """Idioms with a plain input (no function operands) and an expected output."""
    for idiom in D["idioms"]:
        binding = D.get("input", {}).get(idiom, {}).get("xetal")
        if binding and "u:" not in binding and D.get("output", {}).get(idiom, {}).get("xetal"):
            yield idiom


def run(idiom, col):
    prog = program(col, idiom)
    if prog is None:
        return None, None
    p = subprocess.run([str(ROOT / "scripts/runtimes.sh"), "run", COLS[col][0]], input=prog,
                       capture_output=True, text=True, timeout=60)
    return prog, p.stdout.rstrip()


def check():
    res = {}
    for idiom in testable():
        for col in COLS:
            prog, got = run(idiom, col)
            want = D["output"][idiom]["xetal"]
            res[idiom, col] = ("no-cell", prog, got, want) if prog is None else (verdict(got, want), prog, got, want)
    return res


def report(res):
    short = {"match": "ok", "match-0-origin": "ok(0)", "differs": "DIFF", "no-cell": "-"}
    print(f"{'idiom':12}" + "".join(f"{c:>9}" for c in COLS))
    for idiom in dict.fromkeys(i for i, _ in res):
        print(f"{idiom:12}" + "".join(f"{short[res[idiom, c][0]]:>9}" for c in COLS))
    bad = [(k, r) for k, r in res.items() if r[0] == "differs"]
    for (idiom, col), (_, prog, got, want) in bad:
        print(f"\n## {idiom}/{col}\n{prog}\n-> got:  {got[:200]!r}\n   want: {want!r}")
    n = sum(r[0].startswith("match") for r in res.values())
    print(f"\n{n} of {len(res)} cells agree; {len(bad)} differ")
    return len(bad)


def tests(res):
    """One reg-rs test per agreeing cell; the description carries the credit."""
    made = 0
    for (idiom, col), (v, _, _, _) in res.items():
        name = f"idiom-{idiom}-{col}"
        if not v.startswith("match") or (ROOT / "reg" / f"{name}.rgt").exists():
            continue
        how = "agrees with X_eTaL" + (" counting from 0" if v == "match-0-origin" else "")
        what = "ngn/k expression run unchanged in Kona" if col == "k3" else "expression"
        desc = f"{NAMES[idiom]} in {col}: {how}. Credit: {what} from {CREDIT}."
        subprocess.run([str(ROOT / "scripts/reg.sh"), "create", "-t", name, "--timeout", "60",
                        "-c", f"scripts/idiom.py run {idiom} {col}", "--desc", desc], check=True,
                       stdout=subprocess.DEVNULL)
        made += 1
    print(f"created {made} tests")


def attributions():
    """The idioms that have a reg-rs test, listed under their source."""
    ids = sorted({p.stem.split("-", 1)[1].rsplit("-", 1)[0].removesuffix("-gnu")
                  for p in (ROOT / "reg").glob("idiom-*.rgt")})
    f = ROOT / "ATTRIBUTIONS.md"
    text = f.read_text()
    start = text.index("## Idioms by source")
    f.write_text(text[:start] + "## Idioms by source\n\n"
                 "Generated by `scripts/idiom.py attributions` from the reg-rs tests in\n"
                 "`reg/`. Do not edit by hand. Ids are this project's.\n\n"
                 f"### X_eTaL Rosetta data ({len(ids)})\n\n" + "".join(f"- {i}\n" for i in ids))
    print(f"{len(ids)} idioms listed")


def main():
    cmd = sys.argv[1] if len(sys.argv) > 1 else ""
    if cmd == "run" and len(sys.argv) == 4:
        prog, got = run(sys.argv[2], sys.argv[3])
        print(got if prog else "no cell")
    elif cmd == "check":
        sys.exit(1 if report(check()) else 0)
    elif cmd == "tests":
        tests(check())
    elif cmd == "attributions":
        attributions()
    else:
        sys.exit(__doc__)


if __name__ == "__main__":
    main()
