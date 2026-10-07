# compare-idioms

A tool that tests idioms across array languages: the same idiom, run in
each language on the same inputs, checked for the same outputs. The
tested idioms form a regression library in
[reg-rs](https://github.com/sw-vibe-coding/reg-rs), so a runtime upgrade
or an edit that changes an answer is caught.

The idea is to combine the classic idiom collections (the FinnAPL idiom
library, J Phrases, BQNcrate, Eugene McDonnell's K idiom list,
codereport's array-language-comparisons) and find out, by running them,
which idioms carry over between languages and which do not. The hard
part is the idiom that has no counterpart in another language (a lambda
in APL2, a rank-3 array in K): those are recorded as not applicable, with
a reason, rather than forced.

## Languages

| Column | Language | Runtime |
| ------ | -------- | ------- |
| `gnu-apl` | APL (APL2 style) | GNU APL |
| `j` | J | J 9 |
| `bqn` | BQN | CBQN |
| `k` | K | ngn/k |
| `k3` | K3 | Kona |
| `kbm` | k edu, a subset of K | kbm, on BareMetal-OS under QEMU |
| `uiua` | Uiua | uiua |
| `xetal` | X_eTaL | [X_eTaL](https://github.com/softwarewrighter/X_eTaL) |

Dyalog APL is deferred until its license terms are settled.

## Status

The first idiom set is X_eTaL's own: the 31 idioms its Rosetta stone
compares across APL2, Dyalog, J, BQN, ngn/k, Uiua and X_eTaL, plus 18
from its idiom table. Of the 16 that have an input and an expected
output, 112 of 128 cells agree across eight runtimes (11 after allowing
for counting from 0), and each agreeing cell is a reg-rs test: 120
tests with the runtime checks, all passing. Four cells turned out to be
wrong in X_eTaL's data, and kbm, a deliberately small k, handles 4 of
the 16. Details, the design and what comes next are in
[`docs/plan.md`](docs/plan.md).

## Quick start

Needs `reg-rs`, Homebrew, cargo, clang and make; X_eTaL built next door
in `../X_eTaL`, and kbm's fork in `../kbm-fork`.

```bash
scripts/runtimes.sh install     # Kona, GNU APL, J, Uiua, CBQN, ngn/k, kbm
scripts/runtimes.sh doctor      # each runtime answers 1+1
scripts/idiom.py check          # X_eTaL's idioms in every runtime
scripts/reg.sh run              # the regression tests in reg/
echo '|"ABCDE"' | scripts/runtimes.sh run kbm   # one line of k, booted in QEMU
```

Runtimes built from source live under `work/`, which is not committed;
nothing third-party is redistributed from this repository.

## Attribution

Every tested idiom credits its source in its catalog entry and in its
reg-rs test. [`ATTRIBUTIONS.md`](ATTRIBUTIONS.md) lists each source,
its license terms, and the idioms taken from it. No source's list is
reproduced.

## Links

- Blog: [Software Wrighter Lab](https://software-wrighter-lab.github.io/)
- Discord: [Join the community](https://discord.com/invite/Ctzk5uHggZ)
- YouTube: [Software Wrighter](https://www.youtube.com/@SoftwareWrighter)

## Copyright

Copyright (c) 2026 Michael A Wright

## License

MIT. See [`LICENSE`](LICENSE) and [`COPYRIGHT`](COPYRIGHT).
