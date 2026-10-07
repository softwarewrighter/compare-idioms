#!/usr/bin/env bash
# Install, build and check the language runtimes (docs/plan.md, M1).
#   scripts/runtimes.sh install [name...]   # default: all of NAMES
#   scripts/runtimes.sh doctor              # what is present, with a 1+1 check
#   scripts/runtimes.sh run NAME            # run the program on stdin in NAME
#   scripts/runtimes.sh tests               # a reg-rs test (1+1) per runtime
# Package-manager runtimes live outside the repository. Runtimes built from
# source are cloned into work/runtimes/ (gitignored): nothing third-party is
# ever committed here (CLAUDE.md).
set -uo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
RT="$root/work/runtimes"
BREW="$(brew --prefix 2>/dev/null || echo /opt/homebrew)"
NAMES="kona gnu-apl j uiua cbqn ngn-k kbm"
KBM="${KBM:-$root/../kbm-fork}"
UIUA_VERSION="${UIUA_VERSION:-0.19.1}"
# Pinned commits of the runtimes built from source (checked 2026-10-06).
CBQN_COMMIT="${CBQN_COMMIT:-c893d3e7828899a50150df03ad33c9052fa51d3c}"
NGNK_COMMIT="${NGNK_COMMIT:-b9eeb91e0343ef6029f59d55dd98f82667b9bd59}"
BAREMETAL_COMMIT="${BAREMETAL_COMMIT:-0edc835}"

# Where each runtime's binary is. J is called by this path, never as a bare
# `jconsole`: /usr/bin/jconsole on macOS is Java's JConsole.
bin_of() {
    case "$1" in
        kona)    echo "$BREW/bin/k" ;;
        gnu-apl) echo "$BREW/bin/apl" ;;
        j)       echo "$BREW/bin/jcon" ;;
        uiua)    echo "${CARGO_HOME:-$HOME/.cargo}/bin/uiua" ;;
        cbqn)    echo "$RT/CBQN/BQN" ;;
        ngn-k)   echo "$RT/ngn-k/k" ;;
        xetal)   echo "${XETAL:-$root/../X_eTaL}/target/debug/xetal" ;;
        kbm)     echo "$RT/kbm/k-head.img" ;;
    esac
}

clone() { # url dir commit
    [ -d "$2/.git" ] || git clone --quiet "$1" "$2"
    git -C "$2" checkout --quiet "$3"
}

# kbm (../kbm-fork) is k on BareMetal-OS, run under QEMU: the fork's own Mac
# path, test/qemu-portable.sh, builds a disk image that boots straight into k.
# Needs nasm, mtools, qemu and x86_64-elf-binutils (Homebrew), BareMetal-OS
# cloned and set up under work/, and GNU ld/objcopy/objdump first on PATH
# (work/bin). The image is copied to work/runtimes/kbm/.
install_kbm() {
    brew install nasm mtools qemu x86_64-elf-binutils
    clone https://github.com/ReturnInfinity/BareMetal-OS "$RT/BareMetal-OS" "$BAREMETAL_COMMIT"
    [ -f "$RT/BareMetal-OS/sys/bmfs" ] || (cd "$RT/BareMetal-OS" && ./baremetal.sh setup)
    mkdir -p "$root/work/bin" "$RT/kbm"
    local t; for t in ld objcopy objdump; do ln -sf "$BREW/bin/x86_64-elf-$t" "$root/work/bin/$t"; done
    PATH="$root/work/bin:$PATH" B="$RT/BareMetal-OS" "$KBM/test/qemu-portable.sh" --test
    cp "$KBM/test/out/k-head.img" "$RT/kbm/k-head.img"
}

# Run a k program in kbm: boot QEMU, type each line, print everything k
# printed (an error on a binding line included), without the echoed input.
# One boot per program, about 8 seconds.
run_kbm() {
    local f="$root/work/kbm-$$.k"
    # drive_qemu.py skips lines that start with # as comments: indent them.
    sed 's/^#/ #/' > "$f"
    perl -e 'alarm shift; exec @ARGV' 120 python3 "$KBM/test/drive_qemu.py" "$f" \
        qemu-system-x86_64 -machine q35 -cpu Westmere -smp 1 -m 2048 -display none \
        -monitor none -serial stdio -no-reboot \
        -drive "id=disk0,file=$(bin_of kbm),if=none,format=raw,snapshot=on" \
        -device virtio-blk-pci,drive=disk0 | grep -v '^ <<'
    rm -f "$f"
}

install_one() {
    mkdir -p "$RT"
    case "$1" in
        kona)    brew install kona ;;
        gnu-apl) brew install gnu-apl ;;
        j)       brew install --cask j ;;
        uiua)    cargo install uiua --locked --version "$UIUA_VERSION" ;;
        cbqn)    clone https://github.com/dzaima/CBQN "$RT/CBQN" "$CBQN_COMMIT" && make -C "$RT/CBQN" FFI=0 ;;   # no libffi needed: idioms do not call C
        ngn-k)   clone https://codeberg.org/ngn/k "$RT/ngn-k" "$NGNK_COMMIT" && make -C "$RT/ngn-k" CC=clang k ;;
        kbm)     install_kbm ;;
        *)       echo "unknown runtime: $1" >&2; return 2 ;;
    esac
}

# Run a command for at most 20 seconds (macOS has no timeout(1)).
limit() { perl -e 'alarm shift; exec @ARGV' 20 "$@"; }

# One line of 1+1 in each language; every one should print 2.
# GNU APL never exits at end of input: its input must end with )OFF. It also
# starts a background APserver that holds the output pipe open: --noSV stops that.
smoke() {
    local b; b="$(bin_of "$1")"
    case "$1" in
        kona)    echo '1+1' | limit "$b" 2>&1 ;;
        gnu-apl) printf '1+1\n)OFF\n' | limit "$b" --script --noSV 2>&1 ;;
        j)       echo '1+1' | limit "$b" 2>&1 ;;
        uiua)    limit "$b" eval '+1 1' 2>&1 ;;
        cbqn)    limit "$b" -p '1+1' 2>&1 ;;
        ngn-k)   echo '1+1' | limit "$b" 2>&1 ;;
        kbm)     echo '1+1' | run_kbm 2>&1 ;;
    esac | tr -d ' \r' | grep -v '^$' | tail -1
}

# Run the program on stdin in one runtime and print what it prints. This is
# the one place that knows each interpreter's flags and habits.
run_one() {
    local b prog; b="$(bin_of "$1")"; prog="$(cat)"
    [ -e "$b" ] || { echo "runtime not installed: $1" >&2; return 127; }
    [ "$1" = kbm ] && { printf '%s\n' "$prog" | run_kbm; return; }
    case "$1" in
        kona|ngn-k|j) printf '%s\n' "$prog" | limit "$b" ;;
        gnu-apl)      printf '%s\n)OFF\n' "$prog" | limit "$b" --script --noSV ;;
        uiua)         limit "$b" eval "$prog" ;;
        cbqn)         limit "$b" -p "$prog" ;;
        xetal)        limit "$b" eval -e "$prog" ;;
    esac 2>&1
}

where_from() {
    case "$1" in
        kbm)        echo "kbm-fork $(git -C "$KBM" rev-parse --short HEAD 2>/dev/null), BareMetal-OS $BAREMETAL_COMMIT" ;;
        cbqn|ngn-k) local d="$RT/$([ "$1" = cbqn ] && echo CBQN || echo ngn-k)"
                    echo "commit $(git -C "$d" rev-parse --short HEAD 2>/dev/null)" ;;
        uiua)       "$(bin_of uiua)" --version 2>/dev/null | head -1 ;;
        j)          brew list --cask --versions j 2>/dev/null ;;
        *)          brew list --versions "$1" 2>/dev/null ;;
    esac
}

# reg-rs smoke tests: one per runtime, so a missing or changed runtime fails
# on its own, apart from the idiom tests.
tests() {
    local n prog
    for n in $NAMES xetal; do
        [ -e "$root/reg/runtime-$n.rgt" ] && continue
        case "$n" in uiua) prog='+1 1' ;; xetal) prog='1 + 1' ;; *) prog='1+1' ;; esac
        "$root/scripts/reg.sh" create -t "runtime-$n" --timeout 60 --desc "$n answers 1+1" \
            -c "echo '$prog' | scripts/runtimes.sh run $n" >/dev/null
    done
}

doctor() {
    local bad=0 n b got
    for n in $NAMES; do
        b="$(bin_of "$n")"
        if [ ! -e "$b" ]; then printf '%-8s MISSING  %s\n' "$n" "$b"; bad=1; continue; fi
        got="$(smoke "$n")"
        if [ "$got" = 2 ]; then printf '%-8s ok       %s  (%s)\n' "$n" "$b" "$(where_from "$n")"
        else printf '%-8s BROKEN   %s  1+1 gave: %s\n' "$n" "$b" "$got"; bad=1; fi
    done
    return $bad
}

case "${1:-}" in
    install) shift; for n in ${*:-$NAMES}; do echo "== $n"; install_one "$n" || echo "FAILED: $n" >&2; done ;;
    doctor)  doctor ;;
    run)     run_one "${2:?runtime name}" ;;
    tests)   tests ;;
    *)       sed -n '2,6p' "$0"; exit 2 ;;
esac
