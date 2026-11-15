#!/usr/bin/env bash
# Run every host logic test in this directory, each in a fresh directory of its own (see
# lib-test.sh): pure logic, no root, no guest. Prints each test's last line, then the
# totals, and exits non-zero where any test failed. A test skipped for a missing tool
# (exit 77) is named and does not fail the run.
set -uo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
passed=0 failed=0 skipped=0
for t in "$HERE"/t-*.sh; do
    out=$(bash "$t" 2>&1); rc=$?
    case $rc in
        0) passed=$((passed + 1)); printf 'pass  %-22s %s\n' "${t##*/}" "$(tail -1 <<<"$out")" ;;
        77) skipped=$((skipped + 1)); printf 'skip  %-22s %s\n' "${t##*/}" "$(grep -m1 '^SKIP' <<<"$out")" ;;
        *) failed=$((failed + 1)); printf 'FAIL  %-22s %s\n' "${t##*/}" "$(tail -1 <<<"$out")"
           grep -E '^FAIL ' <<<"$out" | sed 's/^/      /' ;;
    esac
done
echo "== $passed passed, $failed failed, $skipped skipped"
[ "$failed" = 0 ]
