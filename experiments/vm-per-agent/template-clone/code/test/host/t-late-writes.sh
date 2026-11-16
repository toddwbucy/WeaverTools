#!/usr/bin/env bash
# The offline read of what the power-off wrote, image_late_writes and image_files, as
# lib.sh defines them, on stand-in ext4 images that mke2fs builds from a directory. Two
# have a log directory damaged with debugfs's own writer, so that debugfs cannot list it.
# A listing that does not complete is a finding, not an empty /var/log (CodeRabbit's full
# review of pull request 31, 2026-11-15). Pure logic: no root, no guest.
source "$(dirname "$0")/lib-test.sh"
need mke2fs debugfs
eval "$(fn "$CODE/lib.sh" image_state image_files image_late_writes)" || exit 1
# mk NAME: the image directory NAME, its root.raw built from $T/src-NAME.
mk() { mkdir -p "$T/$1" && mke2fs -q -t ext4 -d "$T/src-$1" "$T/$1/root.raw" 8M; }
# late NAME: image_late_writes on image NAME, as the proof records it (both streams).
late() { image_late_writes "$T/$1" >"$T/$1.out" 2>&1; }
has() { grep -qE -- "$2" "$T/$1.out"; }

# log: one 100-byte journal three levels down and one empty log.
mkdir -p "$T/src-log/var/log/journal/abc" "$T/src-log/var/lib/systemd"
head -c 100 /dev/zero >"$T/src-log/var/log/journal/abc/system.journal"
: >"$T/src-log/var/log/empty.log"
mk log || exit 1
# short, csum: the same tree, the journal directory's extent pointed past the end of the
# filesystem, or at block 1, which is no directory block.
for x in short csum; do cp -a "$T/src-log" "$T/src-$x" && mk $x || exit 1; done
debugfs -w -R 'sif /var/log/journal/abc block[5] 999999' "$T/short/root.raw" 2>/dev/null
debugfs -w -R 'sif /var/log/journal/abc block[5] 1' "$T/csum/root.raw" 2>/dev/null
# deep: a non-empty log 13 directories down.
mkdir -p "$T/src-deep/var/log/1/2/3/4/5/6/7/8/9/10/11/12/13" "$T/src-deep/var/lib/systemd"
echo deep >"$T/src-deep/var/log/1/2/3/4/5/6/7/8/9/10/11/12/13/x.log"
mk deep || exit 1
# empty: an empty /var/log. quiet: only empty logs, a login record under /var/lib/wtmpdb,
# and systemd's random seed.
mkdir -p "$T/src-empty/var/log" "$T/src-empty/var/lib/systemd"
mk empty || exit 1
mkdir -p "$T/src-quiet/var/log/apt" "$T/src-quiet/var/lib/wtmpdb" "$T/src-quiet/var/lib/systemd"
: >"$T/src-quiet/var/log/apt/history.log"; echo w >"$T/src-quiet/var/lib/wtmpdb/wtmp.db"
head -c 32 /dev/zero >"$T/src-quiet/var/lib/systemd/random-seed"
mk quiet || exit 1
# nolog: no /var/log at all.
mkdir -p "$T/src-nolog/var/lib/systemd"
mk nolog || exit 1

late log
check "a non-empty log is a finding, with its size" \
    has log '^PROOF-FAIL: a log written after the final probe: /var/log/journal/abc/system\.journal \(100 bytes\)$'
check "an empty log is not" bash -c '! grep -q empty.log "$1"' _ "$T/log.out"
check "the listing completed" bash -c '! grep -qE "could not list|image_files:" "$1"' _ "$T/log.out"

for x in short csum; do
    late $x
    check "$x: debugfs's error fails the listing" \
        has $x '^PROOF-FAIL: the offline read could not list every directory under /var/log in the sealed image$'
    check "$x: the record names the directory" has $x '^image_files: /var/log/journal/abc: '
    check "$x: not read as an empty /var/log" bash -c '! grep -q "^no non-empty file" "$1"' _ "$T/$x.out"
done
check "short: debugfs's own words are kept" has short 'short read'
check "csum: debugfs's own words are kept" has csum 'checksum'

late deep
check "a directory past the bound fails the listing" \
    has deep '^PROOF-FAIL: the offline read could not list every directory under /var/log'
check "the bound is named" has deep 'past the depth bound of 12'

late empty
check "an empty /var/log has no non-empty file" has empty '^no non-empty file under /var/log$'
check "an empty /var/log is no finding" bash -c '! grep -q PROOF-FAIL "$1"' _ "$T/empty.out"

late quiet
check "only empty logs: no non-empty file" has quiet '^no non-empty file under /var/log$'
check "the random seed is a finding" \
    has quiet '^PROOF-FAIL: /var/lib/systemd/random-seed is in the sealed image, written after the final probe$'
check "the credential secret reads absent" has quiet '^absent: /var/lib/systemd/credential\.secret$'
check "the login record is listed" has quiet '^  2 /var/lib/wtmpdb/wtmp\.db$'
check "an absent lastlog lists nothing, and is not a failed listing" \
    bash -c 'grep -qx "== /var/lib/lastlog, listed and not judged" "$1" && ! grep -q "did not complete" "$1"' _ "$T/quiet.out"

late nolog
check "no /var/log keeps its own finding" has nolog '^PROOF-FAIL: the offline read could not list /var/log in the sealed image$'

# image_files alone: status 2 on a listing that did not complete, 0 and nothing on an
# absent path.
rc=0; image_files "$T/short" /var/log >/dev/null 2>&1 || rc=$?
check "image_files returns 2 on a damaged directory" [ "$rc" = 2 ]
rc=0; out=$(image_files "$T/log" /var/nothing 2>&1) || rc=$?
check "image_files on an absent path: status 0, nothing printed" eval '[ "$rc" = 0 ] && [ -z "$out" ]'
finish
