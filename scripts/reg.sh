#!/usr/bin/env bash
# Wrap reg-rs so baselines live in the repo under reg/.
#   scripts/reg.sh run            # run every baseline
#   scripts/reg.sh <reg-rs args>  # any other reg-rs command
# Baseline commands run from the repo root.
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"
export REG_RS_DATA_DIR="$root/reg"
mkdir -p "$REG_RS_DATA_DIR"
command -v reg-rs >/dev/null || { echo "reg-rs not found on PATH" >&2; exit 127; }
if [ "${1:-}" = "run" ] && [ "$#" -eq 1 ]; then
    exec reg-rs run -p .rgt --parallel
fi
exec reg-rs "$@"
