# Sourced by each host logic test in this directory.
#
# These tests are pure logic. They need no root and no guest, start no virtual machine,
# and touch no network beyond loopback. Each takes the real functions out of
# build-template.sh, lib.sh and guest/template-steps, by their own text, and runs them on
# the host against stand-ins: small files, stand-in images, loopback servers, and stubs for
# the commands a guest or a hypervisor would answer. No test stubs a privileged command:
# where a guest function calls one, the test drops that prefix from the function's text.
#
# Each run works in a fresh directory of its own, made here with mktemp. /tmp is often
# memory, so the directory goes at exit when every check passed, by a removal that
# matches its pattern and stays on one filesystem; a skip removes it where it is empty.
# On a failed check, an exit before finish or a stop by INT, TERM or HUP it is kept and
# named on the last line. The removal runs from the EXIT trap, after a test's own
# closes: a test that sets its own EXIT trap calls lib_exit last in it, handing it the
# status the trap was entered with, and sets no INT, TERM or HUP trap of its own
# (t-test-folders.sh checks both). Every host test run through run-all.sh leaving its
# directory behind, pass or fail, is what this replaced (the operator's rule on /tmp of
# 2026-11-18). The offline mock (../offline-mock.sh) is not one of these: it runs only
# inside a build guest.
#
# The tools are the guest's: GNU grep, sed and awk, bash, coreutils and python3. A test
# that needs more (qemu-img, debugfs, nvcc) says so and is skipped where a tool is absent,
# with exit status 77.
set -uo pipefail
# The build's inputs and knobs, cleared here before any test reads them: the seven inputs,
# VM_ROOT, NAME and PORT, and every other name build-template.sh, lib.sh and
# guest/template-steps read from the environment. So a test passes or fails the same in a
# clean shell and in one that has exported a build's inputs, as COMMANDS.md's recipe block
# does (on 2026-11-16, with WA_COMMIT exported t-tag failed 13 checks, and with VERSION
# exported t-version failed one). Kept: KEPT_IMAGE, NVCC and CUOBJDUMP, which name what a
# test may read and the tools it uses, and TMPDIR. t-harness-env.sh checks this list
# against the scripts.
BUILD_ENV_NAMES='OS VERSION DRIVER DRIVER_ROUTE COMPUTE_CAP WA_COMMIT WW_COMMIT VM_ROOT NAME PORT
 PROOF_PORT PROOF_BOOT PROOF_FIRST_BOOT PROOF_ACPI LOG KEY KNOWN_HOSTS QEMU_USER VCPUS MEMORY SIZE
 CPUSET GUEST_ENV BASE_IMAGE DEB_ORIGINS SEAL_WAIT CI_WAIT FIRST_BOOT_CHANGES FIRST_BOOT_ADDS'
# shellcheck disable=SC2086
unset $BUILD_ENV_NAMES
# The names the scripts set and export themselves, bar PATH (cuda_env, stack_build and the
# toolchain's target directory), with CUDA_HOME, and every CUDA_ name the host exports. A
# test that stubs the function setting one must not read the host's value in its place:
# on 2026-11-17 t-tag stubbed cuda_env with nothing, and passed on a host that exports
# CUDA_PATH and failed 5 checks on one that does not (Codex on pull request 35). Cleared
# here, a test that leans on them fails on every host. t-harness-env.sh checks this list
# against the scripts' exports.
SET_ENV_NAMES='CUDA_HOME CUDA_PATH CARGO_TARGET_DIR NVCC_CCBIN CUDA_LIB_DIR CUDA_COMPUTE_CAP
 WEAVER_AGENT_DIR'
# shellcheck disable=SC2046,SC2086
unset $SET_ENV_NAMES $(compgen -e CUDA_)
CODE=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
T=$(mktemp -d "${TMPDIR:-/tmp}/weaver-host-test.XXXXXX") || exit 1
pass=0; fail=0; lib_passed=""; lib_named=""
# lib_exit [STATUS]: at exit, the directory removed where finish found every check passed,
# removed where a skip (STATUS 77, by default the status the trap was entered with) left it
# empty, and otherwise named, once.
lib_exit() {
    local rc=${1:-$?}
    if [ -n "$lib_passed" ]; then
        case $T in
            */weaver-host-test.??????) rm -rf --one-file-system -- "$T" || echo "== its folder could not be removed whole: $T" ;;
            *) echo "== its folder is not of the pattern, so it is kept: $T" ;;
        esac
    elif [ "$rc" = 77 ] && rmdir -- "$T" 2>/dev/null; then
        :
    elif [ -z "$lib_named" ]; then
        echo "== exit $rc before every check passed; its folder kept: $T"
    fi
}
# lib_stopped SIGNAL STATUS: a stop by a signal names the directory and exits, so the EXIT
# trap's closes still run.
lib_stopped() { lib_named=1; echo "== stopped by SIG$1; its folder kept: $T"; exit "$2"; }
trap lib_exit EXIT
trap 'lib_stopped INT 130' INT; trap 'lib_stopped TERM 143' TERM; trap 'lib_stopped HUP 129' HUP
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
# wait_for PIDS SECONDS CMD...: run CMD each tenth of a second until it succeeds. Status 1
# once any of PIDS (one or more, space-separated) has exited, or SECONDS have passed, with
# CMD still failing, so a fixture that never comes up is a failed check, not a hang. The
# offline mock carries the same text, which t-wait-for.sh checks and runs.
wait_for() {
    local pids=$1 n=$(($2 * 10)) q
    shift 2
    until "$@"; do
        [ "$n" -gt 0 ] || return 1
        for q in $pids; do kill -0 "$q" 2>/dev/null || return 1; done
        n=$((n - 1)); sleep 0.1
    done
}
# finish: the totals on the last line; where a check failed, the directory named and kept.
finish() {
    if [ "$fail" = 0 ]; then
        lib_passed=1; echo "== $pass passed, 0 failed"
    else
        lib_named=1; echo "== $pass passed, $fail failed (its folder kept: $T)"; return 1
    fi
}
