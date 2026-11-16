#!/usr/bin/env bash
# proof's sealed-file block, as build-template.sh holds it, and compact_check's bound, on
# small qcow2 files: the bound grows with the disk's virtual size (the Planner's review
# of 2026-11-15, F6). Pure logic: no root, no guest.
source "$(dirname "$0")/lib-test.sh"
need qemu-img qemu-io python3
BT=$CODE/build-template.sh
export VM_ROOT=$T/vmroot; mkdir -p "$VM_ROOT"; source "$CODE/lib.sh"
BLOCK=$(sed -n '/^    local fsum ssum$/,/^    } 2>&1 | tee "\$out\/sealed-file.txt"$/p' "$BT")
check "proof's sealed-file block is in the file" test "$(wc -l <<<"$BLOCK")" -gt 5
eval "blk() { $BLOCK
}"
cd "$T" || exit 1
out=$T/out; LOG=$T/log; mkdir -p "$out" "$LOG"
qemu-img create -q -f qcow2 b.qcow2 1G
qemu-io -c 'write -P 1 0 100M' b.qcow2 >/dev/null; qemu-io -c 'discard 0 96M' b.qcow2 >/dev/null
qemu-img convert -O qcow2 b.qcow2 c.qcow2
DISK=$T/c.qcow2; sha256sum "$DISK" >"$LOG/appliance-sha256.txt"; blk >/dev/null
check "a compact file the seal recorded passes" \
    test "$(grep -c '^PROOF-FAIL' "$out/sealed-file.txt")" = 0 -a "$(grep -c '^the seal recorded the same sha256' "$out/sealed-file.txt")" = 1
DISK=$T/b.qcow2; sha256sum "$DISK" >"$LOG/appliance-sha256.txt"; blk >/dev/null
check "a file that is not compact fails" test "$(grep -c '^PROOF-FAIL: the file is not compact' "$out/sealed-file.txt")" = 1
DISK=$T/c.qcow2; sha256sum b.qcow2 >"$LOG/appliance-sha256.txt"; blk >/dev/null
check "a file whose sha256 the seal did not record fails" test "$(grep -c '^PROOF-FAIL: the seal recorded sha256' "$out/sealed-file.txt")" = 1
rm -f -- "$LOG/appliance-sha256.txt"; blk >/dev/null
check "no record of the seal's sha256 fails" test "$(grep -c '^PROOF-FAIL: no sha256 recorded by the seal' "$out/sealed-file.txt")" = 1

# compact_check alone: its status, and a bound that grows with the virtual size. A 1 TiB
# disk with one cluster written in every 512 MiB needs 2048 second-level tables, 128 MiB,
# more than COMPACT_SLACK alone, and its compacted file is correct.
compact_check c.qcow2 >/dev/null; rc=$?
check "compact_check returns 0 on a compact file" test "$rc" = 0
compact_check b.qcow2 >/dev/null; rc=$?
check "and 1 on a file that is not compact" test "$rc" = 1
compact_check absent.qcow2 >/dev/null 2>&1; rc=$?
check "and 1 on a file it cannot check" test "$rc" = 1
qemu-img create -q -f qcow2 big.qcow2 1T
cmds=(); for i in $(seq 0 2047); do cmds+=(-c "write -P 5 $((i * 512))M 64k"); done
qemu-io "${cmds[@]}" big.qcow2 >/dev/null
qemu-img convert -O qcow2 big.qcow2 bigc.qcow2
r=$(compact_check bigc.qcow2); rc=$?
beyond=$(sed -n 's/.*; \([0-9]*\) bytes beyond them.*/\1/p' <<<"$r")
check "the compacted 1 TiB file lies more than COMPACT_SLACK beyond its clusters ($beyond bytes)" test "${beyond:-0}" -gt "$COMPACT_SLACK"
check "and passes, its bound counting the second-level tables" test "$rc" = 0 -a "$(grep -c PROOF-FAIL <<<"$r")" = 0
finish
