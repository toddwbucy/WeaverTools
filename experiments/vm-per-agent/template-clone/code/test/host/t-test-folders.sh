#!/usr/bin/env bash
# The host tests leave no folder in the temporary directory on a pass (the operator's
# rule on /tmp of 2026-11-18; weaver-appliance-build-Spec section 2, the host tests).
# run-all.sh and lib-test.sh, copied into stand-in directories, run stand-in tests: one
# that passes, one that fails, one skipped, one that exits before finish, and one that
# waits until it is stopped. Checked: a run leaves no folder for the passing test and
# exactly the named folders for the failing and the skipped; each test run alone leaves
# nothing on a pass and the folder its last line names otherwise; a run stopped by TERM,
# and a test stopped by HUP, name what they left and leave nothing running. Then every
# host test's traps: one that sets its own EXIT trap calls lib_exit last in it, and none
# sets a trap on INT, TERM or HUP of its own. Pure logic: no root, no guest.
source "$(dirname "$0")/lib-test.sh"
need pgrep mkfifo
H=$T/h
mkdir -p "$H" "$T/tmp" "$T/alone"
cp "$CODE/test/host/run-all.sh" "$CODE/test/host/lib-test.sh" "$H/"
stand_in() { # stand_in DIR NAME BODY: a stand-in test that sources lib-test.sh, then BODY
    printf '#!/usr/bin/env bash\nsource "$(dirname "$0")/lib-test.sh"\n%s\n' "$3" >"$1/$2"
}
stand_in "$H" t-a-pass.sh 'echo work >"$T/work"; check "a stand-in check that holds" true; finish'
stand_in "$H" t-b-fail.sh 'echo work >"$T/work"; check "a stand-in check that fails" false; finish'
stand_in "$H" t-c-skip.sh 'need a-tool-no-host-has-for-this-test; finish'
stand_in "$H" t-d-exit.sh 'echo work >"$T/work"; exit 3'
stand_in "$H" t-e-zero.sh 'echo work >"$T/work"; check "a stand-in check before an early exit 0" true; exit 0'

out=$(TMPDIR=$T/tmp bash "$H/run-all.sh" 2>&1); rc=$?
printf '%s\n' "$out" | sed 's/^/    /'
check "the run exits 1: a test failed" test "$rc" = 1
check "its totals: 1 passed, 3 failed, 1 skipped" grep -qxF '== 1 passed, 3 failed, 1 skipped' <<<"$out"
left=$(find "$T/tmp" -mindepth 1 -maxdepth 1 | LC_ALL=C sort)
named=$(sed -n 's/^      its folder kept: //p' <<<"$out" | LC_ALL=C sort)
check "four folders left, for the failing, the two exiting and the skipped test" test "$(grep -c . <<<"$left")" = 4
check "each folder left is named by the run, and each named folder is left" test "$left" = "$named"
holds() { grep -lF -- "$1" "$T"/tmp/*/out 2>/dev/null; }
check "nothing left holds the passing test's output" eval '[ -z "$(holds "a stand-in check that holds")" ]'
fb=$(holds "a stand-in check that fails"); fb=${fb%/out}
inner=$(find "$fb" -mindepth 1 -maxdepth 1 -type d -name 'weaver-host-test.*')
check "the failing test's folder holds its work folder, which its last line names" \
    eval '[ -f "$inner/work" ] && [ "$(tail -n 1 "$fb/out")" = "== 0 passed, 1 failed (its folder kept: $inner)" ]'
fd=$(holds "== exit 3 before every check passed"); fd=${fd%/out}
inner=$(find "$fd" -mindepth 1 -maxdepth 1 -type d -name 'weaver-host-test.*')
check "the exiting test's folder holds its work folder, which its last line names" \
    eval '[ -f "$inner/work" ] && [ "$(tail -n 1 "$fd/out")" = "== exit 3 before every check passed; its folder kept: $inner" ]'
fz=$(holds "== exit 0 before every check passed"); fz=${fz%/out}
inner=$(find "$fz" -mindepth 1 -maxdepth 1 -type d -name 'weaver-host-test.*')
check "a test that exits 0 before finish is counted failed, and keeps the work folder its last line names" \
    eval 'grep -q "^FAIL  t-e-zero.sh" <<<"$out" && [ -f "$inner/work" ] && [ "$(tail -n 1 "$fz/out")" = "== exit 0 before every check passed; its folder kept: $inner" ]'
fc=$(holds "SKIP: t-c-skip.sh needs"); fc=${fc%/out}
check "the skipped test's empty work folder is removed; its output is kept for its reason" \
    eval '[ -n "$fc" ] && [ -z "$(find "$fc" -mindepth 1 -maxdepth 1 -type d)" ]'

# Each test run alone.
TMPDIR=$T/alone bash "$H/t-a-pass.sh" >/dev/null 2>&1; rc=$?
check "a passing test run alone exits 0 and leaves nothing" eval '[ "$rc" = 0 ] && [ -z "$(ls -A "$T/alone")" ]'
o=$(TMPDIR=$T/alone bash "$H/t-b-fail.sh" 2>&1); rc=$?
a=$(find "$T/alone" -mindepth 1 -maxdepth 1)
check "a failing test run alone leaves exactly the folder its last line names" \
    eval '[ "$rc" = 1 ] && [ "$(grep -c . <<<"$a")" = 1 ] && [ "$(tail -n 1 <<<"$o")" = "== 0 passed, 1 failed (its folder kept: $a)" ]'
case $a in "$T"/alone/weaver-host-test.??????) rm -rf -- "$a" ;; esac
o=$(TMPDIR=$T/alone bash "$H/t-e-zero.sh" 2>&1); rc=$?
a=$(find "$T/alone" -mindepth 1 -maxdepth 1)
check "a test that exits 0 before finish, run alone, exits 1 and leaves the folder its last line names" \
    eval '[ "$rc" = 1 ] && [ "$(grep -c . <<<"$a")" = 1 ] && [ "$(tail -n 1 <<<"$o")" = "== exit 0 before every check passed; its folder kept: $a" ]'
case $a in "$T"/alone/weaver-host-test.??????) rm -rf -- "$a" ;; esac
TMPDIR=$T/alone bash "$H/t-c-skip.sh" >/dev/null 2>&1; rc=$?
check "a skipped test run alone exits 77 and leaves nothing" eval '[ "$rc" = 77 ] && [ -z "$(ls -A "$T/alone")" ]'

# Stopped by a signal. The waiting test writes its work folder's path, then waits on a
# fifo opened both ways, which never ends of itself, so only the signal ends it.
S=$T/s
mkdir -p "$S" "$T/tmp2"
cp "$H/run-all.sh" "$H/lib-test.sh" "$S/"
stand_in "$S" t-w-wait.sh 'mkfifo "$T/f"; printf "%s\n" "$T" >"$READY.part" && mv "$READY.part" "$READY"
read -r -t 30 _ <>"$T/f"; echo "the wait ended by itself, status $?"; finish'
# The same, with an EXIT trap of its own that closes something, as t-btrfs-reads.sh does;
# in a directory of its own, so the run above has one test.
S2=$T/s2
mkdir -p "$S2"; cp "$H/lib-test.sh" "$S2/"
stand_in "$S2" t-w-close.sh 'mkfifo "$T/f"; trap '"'"'lib_rc=$?; echo "closing what it opened"; lib_exit "$lib_rc"'"'"' EXIT
printf "%s\n" "$T" >"$READY.part" && mv "$READY.part" "$READY"; read -r -t 30 _ <>"$T/f"; finish'
export READY=$T/ready
TMPDIR=$T/tmp2 bash "$S/run-all.sh" >"$T/s.out" 2>&1 &
rp=$!
if wait_for "$rp" 10 test -s "$READY"; then
    kill -TERM "$rp"; wait "$rp"; rc=$?
else
    rc=none; kill -TERM "$rp" 2>/dev/null; wait "$rp" 2>/dev/null
fi
so=$(cat "$T/s.out"); w=$(cat "$READY" 2>/dev/null)
kept=$(find "$T/tmp2" -mindepth 1 -maxdepth 1)
printf '%s\n' "$so" | sed 's/^/    /'
check "a run sent TERM exits 143" test "$rc" = 143
check "it names the folder it kept, the one folder left" \
    eval '[ "$(grep -c . <<<"$kept")" = 1 ] && grep -qxF "STOPPED by SIGTERM in t-w-wait.sh; its folder kept: $kept" <<<"$so"'
check "the stopped test named its own work folder, which stands inside the run's" \
    eval '[ -n "$w" ] && [ -d "$w" ] && [ "${w%/*}" = "$kept" ] && grep -qxF "      == stopped by SIGTERM; its folder kept: $w" <<<"$so"'
check "the test was stopped, not left to end by itself" eval '! grep -q "the wait ended by itself" "$kept/out"'
check "no stand-in test still runs" test -z "$(pgrep -f "$S/t-w-wait.sh")"
rm -f -- "$READY"
TMPDIR=$T/alone bash "$S2/t-w-close.sh" >"$T/w.out" 2>&1 &
wp=$!
if wait_for "$wp" 10 test -s "$READY"; then
    kill -HUP "$wp"; wait "$wp"; rc=$?
else
    rc=none; kill -TERM "$wp" 2>/dev/null; wait "$wp" 2>/dev/null
fi
w=$(cat "$READY" 2>/dev/null)
check "a test with its own EXIT trap, run alone and sent HUP, exits 129 and names its folder" \
    eval '[ "$rc" = 129 ] && [ -d "$w" ] && [ "$(tail -n 1 "$T/w.out")" = "== stopped by SIGHUP; its folder kept: $w" ]'
check "the naming is its last line, after its own trap closed what it opened" \
    test "$(tail -n 2 "$T/w.out" | sed -n 1p)" = "closing what it opened"

# Every host test's traps, read in the test's own code: less its comments and the text of
# its heredocs, since a stand-in script a test writes (t-make-template.sh's has a TERM trap
# of its own) is not the test.
own_code() { awk '
    skip != "" { if ($0 == skip || $0 ~ ("^\t*" skip "$")) skip = ""; next }
    /^[[:space:]]*#/ { next }
    { print
      if (match($0, /<<-?[[:space:]]*["\047]?[A-Za-z_][A-Za-z0-9_]*["\047]?/)) {
          t = substr($0, RSTART, RLENGTH); sub(/^<<-?[[:space:]]*/, "", t); gsub(/["\047]/, "", t); skip = t } }' "$1"; }
printf '%s\n' 'cat >"$T/s.sh" <<'"'"'SH'"'"'' "trap 'x' TERM" 'SH' 'echo "$(wc -l <<<"$x")"' "trap 'y' HUP" >"$T/own.sh"
check "a test's own code keeps its lines and a trap after a heredoc, and drops the heredoc's text" \
    test "$(own_code "$T/own.sh")" = "cat >\"\$T/s.sh\" <<'SH'
echo \"\$(wc -l <<<\"\$x\")\"
trap 'y' HUP"
own_exit="" own_signal=""
re="lib_exit[^;]*'[[:space:]]+EXIT\$"
for f in "$CODE"/test/host/t-*.sh; do
    code=$(own_code "$f")
    while IFS= read -r l; do
        [[ $l =~ $re ]] || own_exit+="${f##*/}: $l; "
    done < <(grep -E "^[[:space:]]*trap .*[[:space:]]EXIT$" <<<"$code")
    grep -qE "^[[:space:]]*trap .*[[:space:]](INT|TERM|HUP)([[:space:]]|$|;)" <<<"$code" && own_signal+="${f##*/} "
done
check "every host test's own EXIT trap calls lib_exit last${own_exit:+: $own_exit}" test -z "$own_exit"
check "no host test sets its own INT, TERM or HUP trap${own_signal:+: $own_signal}" test -z "$own_signal"
check "t-btrfs-reads.sh's EXIT trap closes its exports, then calls lib_exit" \
    grep -qxF "trap 'lib_rc=\$?; close_all; lib_exit \"\$lib_rc\"' EXIT" "$CODE/test/host/t-btrfs-reads.sh"
code=$(grep -vE '^[[:space:]]*#' "$CODE/test/host/run-all.sh")
check "run-all.sh tees nothing, so no reader of its output can be killed out from under it" \
    eval '! grep -qE "(^|[|[:space:]])tee([[:space:]]|$)" <<<"$code"'
check "the removals are guarded by their patterns and stay on one filesystem" \
    eval 'grep -qF "*/weaver-host-test.??????) rm -rf --one-file-system -- \"\$T\"" "$CODE/test/host/lib-test.sh" && grep -qF "\"\$BASE\"/weaver-host-run.??????) rm -rf --one-file-system -- \"\$d\"" "$CODE/test/host/run-all.sh"'
finish
