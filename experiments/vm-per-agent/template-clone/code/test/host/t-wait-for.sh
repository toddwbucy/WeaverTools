#!/usr/bin/env bash
# wait_for, the bounded wait for a fixture: the host tests' copy here and the offline mock's
# own, taken out of offline-mock.sh by its text and run without running the mock. A fixture
# that comes up passes; one that exits first, or lives on without coming up, fails within
# its bound instead of hanging (CodeRabbit's review of #33, 2026-11-16). Pure logic: no
# root, no guest.
source "$(dirname "$0")/lib-test.sh"
MOCK=$CODE/test/offline-mock.sh
check "the mock's wait_for is this harness's, by its text" \
    test "$(fn "$MOCK" wait_for)" = "$(fn "$CODE/test/host/lib-test.sh" wait_for)"
eval "$(fn "$MOCK" wait_for)"
secs() { echo $(( $(date +%s%N) / 100000000 )); }   # tenths of a second
( sleep 0.5; : >"$T/up"; sleep 10 ) & p=$!
t0=$(secs); wait_for "$p" 5 test -e "$T/up"; rc=$?; t1=$(secs)
check "a condition that comes true while the fixture lives: status 0, at once" test "$rc" = 0 -a $((t1 - t0)) -lt 30
kill "$p" 2>/dev/null; wait "$p" 2>/dev/null
( sleep 0.3 ) & p=$!
t0=$(secs); wait_for "$p" 5 test -e "$T/never"; rc=$?; t1=$(secs)
check "a fixture that exits first: status 1, before the bound" test "$rc" = 1 -a $((t1 - t0)) -lt 30
sleep 30 & p=$!
t0=$(secs); wait_for "$p" 1 test -e "$T/never"; rc=$?; t1=$(secs)
check "a fixture that lives on without coming up: status 1, at the bound" test "$rc" = 1 -a $((t1 - t0)) -ge 10 -a $((t1 - t0)) -lt 40
kill "$p" 2>/dev/null; wait "$p" 2>/dev/null
sleep 30 & q=$!; ( sleep 0.3 ) & p=$!
t0=$(secs); wait_for "$q $p" 5 test -e "$T/never"; rc=$?; t1=$(secs)
check "two fixtures, one of which exits: status 1, before the bound" test "$rc" = 1 -a $((t1 - t0)) -lt 30
kill "$q" 2>/dev/null; wait 2>/dev/null
finish
