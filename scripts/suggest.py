#!/usr/bin/env python3
"""Ask a Gemini model to check X_eTaL's idioms and suggest the same idiom in
each target language (docs/plan.md, "Filling a language column").

  scripts/suggest.py --list-models
  scripts/suggest.py --model MODEL [--idioms a,b,...] [--dry-run]

Reads X_eTaL's idioms in place (XETAL, default ../X_eTaL): the Rosetta data
(demos/rosetta/data.toml) and the mainstream table of docs/idioms.md.
Writes one JSON file per idiom under work/suggestions/MODEL/ (gitignored).
A suggestion is a candidate only: nothing here enters the catalog until the
harness has run it (CLAUDE.md, idiom rule 6).

The key comes from GEMINI_API_KEY, or the first line of work/gemini.key.
It is never printed."""

import argparse
import html
import json
import os
import re
import sys
import time
import tomllib
import urllib.error
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
XETAL = Path(os.environ.get("XETAL", ROOT.parent / "X_eTaL"))
API = "https://generativelanguage.googleapis.com/v1beta"
# Target columns (Dyalog is deferred, docs/plan.md) and what to tell the model.
TARGETS = {
    "gnu-apl": "GNU APL (an APL2 implementation: index origin 1; no dfns, no trains)",
    "j": "J 9",
    "bqn": "BQN (CBQN)",
    "k": "ngn/k (a K6-style dialect)",
    "k3": "K3 as run by Kona (the dialect of Eugene McDonnell's k idiom list)",
    "kbm": "k edu (Arthur Whitney's shakti k, a small subset: where, sort and "
           "float printing are not implemented)",
    "uiua": "Uiua",
}


def key():
    k = os.environ.get("GEMINI_API_KEY", "").strip()
    f = ROOT / "work/gemini.key"
    if not k and f.exists():
        k = f.read_text().splitlines()[0].strip()
    if not k:
        sys.exit("no key: set GEMINI_API_KEY or put it in work/gemini.key")
    return k


def call(path, body=None):
    req = urllib.request.Request(f"{API}/{path}", json.dumps(body).encode() if body else None,
                                 {"x-goog-api-key": key(), "Content-Type": "application/json"})
    for attempt in range(1, 7):  # busy or rate-limited: wait and try again
        try:
            return json.load(urllib.request.urlopen(req, timeout=180))
        except urllib.error.HTTPError as e:
            if e.code not in (429, 500, 503) or attempt == 6:
                sys.exit(f"Gemini API: HTTP {e.code}: {e.read().decode()[:300]}")
            time.sleep(15 * attempt)


def mainstream():
    """The first table of X_eTaL's docs/idioms.md: idiom name -> X_eTaL cell."""
    text = (XETAL / "docs/idioms.md").read_text()
    table = text.split("## Mainstream languages")[1].split("## Array languages")[0]
    rows, row = {}, ""
    for line in table.splitlines():
        row += line + "\n"
        if not line.rstrip().endswith("|"):
            continue
        cells = [c.strip() for c in row.strip().strip("|").split(" | ")]
        row = ""
        if len(cells) < 3 or cells[0] in ("Idiom", "-----"):
            continue
        rows[cells[0]] = html.unescape(re.sub(r"</?code>", "", cells[-1]))
    return rows


def idioms():
    """Every idiom X_eTaL documents, as id -> record."""
    d = tomllib.load((XETAL / "demos/rosetta/data.toml").open("rb"))
    out = {}
    for i, name in zip(d["idioms"], d["idiom_names"]):
        src = dict(d["source"].get(i, {}))
        src.pop("dyalog", None)
        if "apl2" in src:  # X_eTaL's APL2 cells are the candidates for GNU APL
            src["gnu-apl"] = src.pop("apl2")
        out[i] = {"name": name, "table": "rosetta", "xetal": src.pop("xetal", None),
                  "input": d.get("input", {}).get(i, {}).get("xetal"),
                  "output": d.get("output", {}).get(i, {}).get("xetal"),
                  "existing": src, "notes": d.get("notes", {}).get(i, {})}
    known = {r["name"].lower() for r in out.values()}
    for name, cell in mainstream().items():
        if name.lower() in known:
            continue
        slug = re.sub(r"[^a-z0-9]+", "-", name.lower()).strip("-")
        out[slug] = {"name": name, "table": "mainstream", "xetal": cell,
                     "input": None, "output": None, "existing": {}, "notes": {}}
    return out


def prompt(rec):
    lines = [
        "You are reviewing idioms across array programming languages.",
        "X_eTaL is a new array language; its expressions are read right to left "
        "with no precedence, it counts from 1, and a name with an underscore is a function.",
        "",
        f"Idiom: {rec['name']}",
        f"X_eTaL expression: {rec['xetal']}",
    ]
    if rec["input"]:
        lines.append(f"X_eTaL input bindings: {rec['input']}")
    if rec["output"]:
        lines.append(f"Expected result (X_eTaL display): {rec['output']}")
    if rec["existing"]:
        lines.append("Expressions already proposed for other languages (unverified):")
        lines += [f"  {lang}: {expr}" for lang, expr in rec["existing"].items()]
    lines += [
        "",
        "For each target language below, give the idiomatic single expression for this "
        "idiom, using the same variable names as the input bindings. If one was already "
        "proposed, say whether it is correct, and correct it if not. If the idiom cannot "
        "be written in that language, set expression to null and say why. Do not invent "
        "primitives; say so when you are unsure.",
    ]
    lines += [f"  {lang}: {desc}" for lang, desc in TARGETS.items()]
    lines += [
        "",
        "Answer with JSON only, one key per target language:",
        '{"<lang>": {"expression": "..." or null, "proposed_ok": true|false|null, '
        '"confidence": "high"|"medium"|"low", "note": "..."}, ...}',
    ]
    return "\n".join(lines)


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("--list-models", action="store_true")
    ap.add_argument("--model")
    ap.add_argument("--idioms", help="comma-separated ids (default: all)")
    ap.add_argument("--dry-run", action="store_true", help="print the prompts, call nothing")
    ap.add_argument("--force", action="store_true", help="ask again for idioms already answered")
    a = ap.parse_args()
    if a.list_models:
        for m in call("models?pageSize=200").get("models", []):
            if "generateContent" in m.get("supportedGenerationMethods", []):
                print(m["name"].removeprefix("models/"), "-", m.get("displayName", ""))
        return
    recs = idioms()
    ids = a.idioms.split(",") if a.idioms else list(recs)
    if a.dry_run:
        for i in ids:
            print(f"=== {i} ({recs[i]['table']})\n{prompt(recs[i])}\n")
        print(f"{len(ids)} of {len(recs)} idioms")
        return
    if not a.model:
        sys.exit("--model is required (see --list-models)")
    out = ROOT / "work/suggestions" / a.model
    out.mkdir(parents=True, exist_ok=True)
    for i in ids:
        if (out / f"{i}.json").exists() and not a.force:
            continue  # already answered: a stopped run resumes where it left off
        body = {"contents": [{"parts": [{"text": prompt(recs[i])}]}],
                "generationConfig": {"temperature": 0, "responseMimeType": "application/json"}}
        r = call(f"models/{a.model}:generateContent", body)
        text = r["candidates"][0]["content"]["parts"][0]["text"]
        try:
            answer = json.loads(text)
        except json.JSONDecodeError:
            answer = {"unparsed": text}
        (out / f"{i}.json").write_text(json.dumps(
            {"idiom": i, "model": a.model, "date": time.strftime("%Y-%m-%d"),
             "record": recs[i], "answer": answer}, indent=1, ensure_ascii=False) + "\n")
        print(f"{i}: {'ok' if 'unparsed' not in answer else 'unparsed'}")
        time.sleep(float(os.environ.get("SUGGEST_PAUSE", "4")))


if __name__ == "__main__":
    main()
