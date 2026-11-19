#!/usr/bin/env bash
# Run every host logic test in this directory: pure logic, no root, no guest. Prints each
# test's last line, then the totals, and exits non-zero where any test failed. A test
# skipped for a missing tool (exit 77) is named and does not fail the run.
#
# Each test runs with a TMPDIR of its own, made new here under the caller's TMPDIR (or
# /tmp), holding the test's output and the work folder lib-test.sh makes. /tmp is often
# memory, so on a pass the folder is removed by a removal that matches its pattern and
# stays on one filesystem; on a fail or a skip it is kept and named. INT, TERM or HUP
# stops the run: the test in hand is sent TERM, so it names its own work folder, and the
# run names the folder it kept and exits 130, 143 or 129. Nothing here tees its output,
# so no reader of it can be killed out from under the cleanup. t-test-folders.sh checks
# all of it with stand-in tests.
set -uo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
BASE=${TMPDIR:-/tmp}
passed=0 failed=0 skipped=0 t="" HT="" pid=""
# stopped SIGNAL STATUS: the test in hand stopped, its folder named, the run ended.
stopped() {
    if [ -n "$pid" ]; then
        kill -TERM "$pid" 2>/dev/null
        wait "$pid" 2>/dev/null
    fi
    if [ -n "$HT" ]; then
        sed 's/^/      /' "$HT/out" 2>/dev/null | tail -n 3
        echo "STOPPED by SIG$1 in ${t##*/}; its folder kept: $HT"
    else
        echo "STOPPED by SIG$1 between tests; no folder kept"
    fi
    echo "== $passed passed, $failed failed, $skipped skipped, stopped"
    exit "$2"
}
trap 'stopped INT 130' INT; trap 'stopped TERM 143' TERM; trap 'stopped HUP 129' HUP
for t in "$HERE"/t-*.sh; do
    HT=$(mktemp -d "$BASE/weaver-host-run.XXXXXX") || { echo "no folder could be made for ${t##*/} under $BASE"; exit 1; }
    # In the background, so a signal to the run is taken at once and the test then stopped.
    TMPDIR=$HT bash "$t" >"$HT/out" 2>&1 &
    pid=$!
    wait "$pid"; rc=$?; pid=""
    last=$(tail -n 1 "$HT/out")
    case $rc in
        0) passed=$((passed + 1)); printf 'pass  %-22s %s\n' "${t##*/}" "$last"
           # let go of it first, so a signal from here on names no folder already removed
           d=$HT; HT=""
           case $d in
               "$BASE"/weaver-host-run.??????) rm -rf --one-file-system -- "$d" || echo "      its folder could not be removed whole: $d" ;;
               *) echo "      its folder is not of the pattern, so it is kept: $d" ;;
           esac ;;
        77) skipped=$((skipped + 1)); printf 'skip  %-22s %s\n' "${t##*/}" "$(grep -m1 '^SKIP' "$HT/out")"
            echo "      its folder kept: $HT" ;;
        *) failed=$((failed + 1)); printf 'FAIL  %-22s %s\n' "${t##*/}" "$last"
           grep -E '^FAIL ' "$HT/out" | sed 's/^/      /'
           echo "      its folder kept: $HT" ;;
    esac
    HT=""
done
echo "== $passed passed, $failed failed, $skipped skipped"
[ "$failed" = 0 ]
