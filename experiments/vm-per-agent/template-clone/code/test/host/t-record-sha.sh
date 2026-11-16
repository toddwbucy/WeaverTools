#!/usr/bin/env bash
# record_sha and record_read, as guest/template-steps defines them with record and sha: a file's digest is
# read before it is recorded, so a digest that cannot be read fails the step and makes no
# record (CodeRabbit's full review of pull request 31, 2026-11-15). Run under the file's
# own set -uo pipefail. Pure logic: no root, no guest.
source "$(dirname "$0")/lib-test.sh"
TS=$CODE/guest/template-steps
P='s''udo '
f=$(fn "$TS" record sha record_sha record_read) || exit 1
eval "${f//$P/}"
BUILD=$T/build; mkdir -p "$BUILD/stamp.d" "$T/files"
echo one >"$T/files/one"; mkdir "$T/files/dir"
want=$(sha256sum "$T/files/one" | cut -d' ' -f1)
stamp() { cat "$BUILD/stamp.d/$1" 2>/dev/null; }

out=$(record_sha t.plain "$T/files/one" 2>&1); rc=$?
check "a readable file: status 0" [ "$rc" = 0 ]
check "a readable file: its digest alone" [ "$(stamp t.plain)" = "$want" ]
check "a readable file: the stamp line is printed" grep -qx "stamp: t.plain = $want" <<<"$out"

record_sha t.note "$T/files/one" '' ' (not laid)' >/dev/null 2>&1
check "text after the digest" [ "$(stamp t.note)" = "$want (not laid)" ]
record_sha t.pre "$T/files/one" 'sh.rustup.rs sha256 ' >/dev/null 2>&1
check "text before the digest" [ "$(stamp t.pre)" = "sh.rustup.rs sha256 $want" ]
record_sha t.apos "$T/files/one" '' " (as written at image build; see the build's note)" >/dev/null 2>&1
check "text with an apostrophe" [ "$(stamp t.apos)" = "$want (as written at image build; see the build's note)" ]

out=$(record_sha t.absent "$T/files/absent" '' ' (not laid)' 2>&1); rc=$?
check "an absent file: the step fails" [ "$rc" != 0 ]
check "an absent file: no record" [ ! -e "$BUILD/stamp.d/t.absent" ]
check "an absent file: named" grep -qx "record: no sha256 of $T/files/absent for t.absent" <<<"$out"

out=$(record_sha t.dir "$T/files/dir" 2>&1); rc=$?
check "a directory: the step fails, no record" [ "$rc" != 0 ] && [ ! -e "$BUILD/stamp.d/t.dir" ]

out=$(record_sha 'Bad/Key' "$T/files/one" 2>&1); rc=$?
check "a bad key is still refused by record" [ "$rc" != 0 ] && grep -q '^record: bad key' <<<"$out"

# record_read: a read that failed, or gave nothing, makes no record.
v=$(printf '4\n'); record_read t.ok $? "$v" >/dev/null 2>&1; rc=$?
check "record_read: a read that worked is recorded" [ "$rc" = 0 ] && [ "$(stamp t.ok)" = 4 ]
v=$(id -un); record_read t.more $? "$v" "/home/x" >/dev/null 2>&1
check "record_read: MORE follows READ" [ "$(stamp t.more)" = "$(id -un) /home/x" ]
v=$(false); out=$(record_read t.failed $? "$v" 2>&1); rc=$?
check "record_read: a failed read fails, no record" [ "$rc" != 0 ] && [ ! -e "$BUILD/stamp.d/t.failed" ]
check "record_read: named with its status" grep -qx "record: t.failed could not be read (status 1)" <<<"$out"
v=$(printf '' | sort -n | tail -1); out=$(record_read t.empty $? "$v" "/home/x" 2>&1); rc=$?
check "record_read: an empty read fails though MORE is given" [ "$rc" != 0 ] && [ ! -e "$BUILD/stamp.d/t.empty" ]
v=$(sort -n "$T/files/no-samples" | tail -1); record_read t.pipe $? "$v" >/dev/null 2>&1; rc=$?
check "record_read: a pipeline's first command failing fails, under the file's pipefail" [ "$rc" != 0 ]

# The old form, for the record: inside record's own argument a failed sha is lost.
record t.old "$(sha "$T/files/absent" 2>/dev/null) (not laid)" >/dev/null 2>&1; rc=$?
check "the old form recorded ' (not laid)' with no digest, status 0" [ "$rc" = 0 ] && [ "$(stamp t.old)" = " (not laid)" ]

# Every site in guest/template-steps: no record takes a command substitution in its own
# arguments, where a read that fails is lost, and each size is read through record_read.
check "no record in template-steps reads inside its own argument" \
    bash -c '! grep -nE "(^|[^_[:alnum:]])record [^|]*[$][(]" "$1"' _ "$TS"
check "each size in template-steps is recorded through record_read" \
    bash -c '! grep -nE "(^|[^_[:alnum:]])record size[.]" "$1"' _ "$TS"
finish
