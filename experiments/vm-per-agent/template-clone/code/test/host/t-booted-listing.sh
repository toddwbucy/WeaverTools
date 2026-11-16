#!/usr/bin/env bash
# The booted guest's file listing in proof_first_boot: the command text build-template.sh
# sends, expanded as the host expands it, then run as the guest's login shell runs a
# command over SSH (bash -c). A find that fails part way fails the listing, though sort
# and sha256sum succeed (CodeRabbit's full review of pull request 31, 2026-11-15). The
# scope is a stand-in, and an unreadable directory makes find fail, so this test needs an
# account other than root. Pure logic: no root, no guest.
source "$(dirname "$0")/lib-test.sh"
[ "$(id -u)" != 0 ] || { echo "SKIP: $(basename "$0") needs an account other than root"; exit 77; }
BT=$CODE/build-template.sh
# The line that lists the files, less any privileged prefix (no test runs one).
P='s''udo '
line=$(grep -F -- "-xdev -type f -not -path" "$BT" | grep -F 'weaver-appliance/MANIFEST')
check "one listing line in build-template.sh" [ "$(grep -c . <<<"$line")" = 1 ]
line=$(sed -e 's/^[[:space:]]*//' -e 's/[[:space:]]*\\$//' <<<"${line//$P/}")
mkdir -p "$T/scope/etc/ssh" "$T/scope/opt/new"
echo a >"$T/scope/etc/a"; echo n >"$T/scope/opt/new/n"
echo k >"$T/scope/etc/ssh/ssh_host_ed25519_key"; echo p >"$T/scope/etc/ssh/ssh_host_ed25519_key.pub"
MANIFEST_SCOPE="$T/scope/etc $T/scope/opt"
remote=""
eval "remote=$line"
check "the host expands the scope into the command" grep -qF "find $T/scope/etc $T/scope/opt -xdev" <<<"$remote"

chmod 000 "$T/scope/opt/new"
rc=0; bash -c "$remote" >"$T/fail.out" 2>"$T/fail.err" || rc=$?
chmod 755 "$T/scope/opt/new"
check "a find that fails part way: the listing exits non-zero" [ "$rc" != 0 ]
check "find's error is on stderr" grep -q 'Permission denied' "$T/fail.err"

rc=0; bash -c "$remote" >"$T/ok.out" 2>"$T/ok.err" || rc=$?
check "a find that succeeds: exit 0" [ "$rc" = 0 ]
check "every readable file listed, as sha256sum lines" bash -c '
    [ "$(wc -l <"$1")" = 3 ] && ! grep -qvE "^[0-9a-f]{64}  " "$1"' _ "$T/ok.out"
check "the new subtree's file is listed" grep -q " $T/scope/opt/new/n\$" "$T/ok.out"
check "a host key's private half is not listed" bash -c '! grep -q "ssh_host_ed25519_key$" "$1"' _ "$T/ok.out"
check "its public half is" grep -q "ssh_host_ed25519_key.pub\$" "$T/ok.out"
finish
