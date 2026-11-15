# Sourced by each host logic test in this directory.
#
# These tests are pure logic. They need no root and no guest, start no virtual machine,
# and touch no network beyond loopback. Each takes the real functions out of
# build-template.sh, lib.sh and guest/template-steps, by their own text, and runs them on
# the host against stand-ins: small files, stand-in images, loopback servers, and stubs for
# the commands a guest or a hypervisor would answer. No test stubs a privileged command:
# where a guest function calls one, the test drops that prefix from the function's text.
#
# Each run works in a fresh directory of its own, made here with mktemp and named on the
# last line. Nothing is removed afterwards. The offline mock (../offline-mock.sh) is not
# one of these: it runs only inside a build guest.
#
# The tools are the guest's: GNU grep, sed and awk, bash, coreutils and python3. A test
# that needs more (qemu-img, debugfs, nvcc) says so and is skipped where a tool is absent,
# with exit status 77.
set -uo pipefail
CODE=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
T=$(mktemp -d "${TMPDIR:-/tmp}/weaver-host-test.XXXXXX") || exit 1
pass=0; fail=0
ok() { echo "ok   $*"; pass=$((pass + 1)); }
no() { echo "FAIL $*"; fail=$((fail + 1)); }
check() { local what=$1; shift; if "$@"; then ok "$what"; else no "$what"; fi; }
# need TOOL...: skip the whole test where a tool it needs is absent.
need() {
    local t
    for t; do command -v "$t" >/dev/null 2>&1 || { echo "SKIP: $(basename "$0") needs $t"; exit 77; }; done
}
# fn FILE NAME...: the named functions' text, as FILE defines them, for eval. The text is
# read whole before it is tested: under pipefail, sed piped into grep -q can be cut by
# SIGPIPE once grep has its match, and a long function then reads as absent.
fn() {
    local f=$1 n s; shift
    for n; do
        s=$(sed -n "/^$n() {/,/^}/p" "$f")
        [ -n "$s" ] || { echo "FAIL no function $n in $f" >&2; return 1; }
        printf '%s\n' "$s"
    done
}
finish() { echo "== $pass passed, $fail failed (in $T)"; [ "$fail" = 0 ]; }
