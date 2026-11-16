#!/usr/bin/env bash
# The seal's end, seal_verdict and compact_disk (decision 18 of 2026-11-15, and the
# Planner's review of 2026-11-15, F3, F4, F5 and F10): the functions as build-template.sh
# defines them, under its own set -euo pipefail. virsh_ and the image reads are stubbed,
# qemu-img is real on small files, or a stand-in for each failure. Each run has its own
# build directory and log. Pure logic: no root, no guest.
source "$(dirname "$0")/lib-test.sh"
need qemu-img qemu-io python3
BT=$CODE/build-template.sh
export VM_ROOT=$T/vmroot; mkdir -p "$VM_ROOT"
A1=1111111111111111111111111111111111111111 A2=2222222222222222222222222222222222222222
IMG=$T/img; mkdir -p "$IMG/etc/weaver-appliance"
printf 'appliance.version = 1.0\nweaveragent.commit = %s\nweaverweb.commit = %s\n' "$A1" "$A2" >"$IMG/etc/weaver-appliance/stamp"
echo "x  /etc/hostname" >"$IMG/etc/weaver-appliance/MANIFEST"
# fresh: a new build directory, a 1 GiB disk with 200 MiB written then discarded and 8 MiB
# kept, so a copy compacts, and a new log.
fresh() {
    TD=$(mktemp -d "$T/appliance.XXXXXX"); mkdir -p "$TD/hostkey"; : >"$TD/known_hosts"; : >"$TD/seed.iso"
    qemu-img create -q -f qcow2 "$TD/appliance.qcow2" 1G
    qemu-io -c 'write -P 0x33 0 200M' -c 'write -P 0x44 512M 8M' "$TD/appliance.qcow2" >/dev/null
    qemu-io -c 'discard 0 200M' "$TD/appliance.qcow2" >/dev/null
    chmod 0644 "$TD/appliance.qcow2"
    LOGD=$(mktemp -d "$T/log.XXXXXX"); CALLS=$LOGD/calls; : >"$CALLS"
}
# run [STUB]: seal_verdict in a subshell, as build-template.sh runs it.
run() {
    ( set -euo pipefail
      echo "$BASHPID" >"$LOGD/pid"
      source "$CODE/lib.sh"
      eval "$(fn "$BT" inputs_ok seal_verdict compact_disk)"
      NAME=t-appliance DIR=$TD DISK=$TD/appliance.qcow2 LOG=$LOGD WA_COMMIT=$A1 WW_COMMIT=$A2 VERSION=1.0 OS_GIVEN=26.04
      virsh_() { echo "virsh_ $*" >>"$CALLS"
                 case $1 in
                     domstate) echo "shut off" ;;
                     dumpxml) printf "<domain><name>%s</name><devices><disk type='file' device='disk'><source file='%s'/></disk></devices></domain>\n" "$2" "$DISK" ;;
                 esac; }
      image_open() { :; }; image_close() { :; }
      image_get() { [ -e "$IMG$2" ] || return 1; cat "$IMG$2" >"$3"; }
      sha256sum() { echo "sha256sum $*" >>"$CALLS"; command sha256sum "$@"; }
      chmod() { echo "chmod $*" >>"$CALLS"; command chmod "$@"; }
      sync() { echo "sync $*" >>"$CALLS"; command sync "$@"; }
      mv() { echo "mv $*" >>"$CALLS"; command mv "$@"; }
      q() { echo "qemu-img $*" >>"$CALLS"; }
      case ${1:-} in
          convert-fails) qemu-img() { q "$@"; case $1 in convert) return 1 ;; *) command qemu-img "$@" ;; esac; } ;;
          compare-differs) qemu-img() { q "$@"; case $1 in compare) echo "Content mismatch"; return 1 ;; *) command qemu-img "$@" ;; esac; } ;;
          # A copy that compares equal but is not compact: the disk copied as it stands.
          not-compact) qemu-img() { q "$@"; case $1 in convert) cp -- "${@: -2:1}" "${@: -1}" ;; *) command qemu-img "$@" ;; esac; } ;;
          slow-convert) qemu-img() { q "$@"; case $1 in convert) command qemu-img "$@"; touch "$LOGD/converted"; sleep 3 ;; *) command qemu-img "$@" ;; esac; } ;;
          undefine-fails) qemu-img() { q "$@"; command qemu-img "$@"; }
                          virsh_() { echo "virsh_ $*" >>"$CALLS"
                                     case $1 in
                                         domstate) echo "shut off" ;;
                                         dumpxml) printf "<domain><name>%s</name><devices><disk type='file' device='disk'><source file='%s'/></disk></devices></domain>\n" "$2" "$DISK" ;;
                                         undefine) return 1 ;;
                                     esac; } ;;
          *) qemu-img() { q "$@"; command qemu-img "$@"; } ;;
      esac
      seal_verdict; echo "after the verdict" )
}
not_sealed() { [ "$(grep -c '^virsh_ undefine' "$CALLS")" = 0 ] && [ "$(stat -c %a "$TD/appliance.qcow2")" = 644 ] \
    && [ -e "$TD/hostkey" ] && [ ! -e "$LOGD/appliance-sha256.txt" ] && [ ! -e "$TD/appliance.compact.part" ]; }
order() { grep -oE '^(qemu-img convert|sync -- [^ ]*compact.part|qemu-img compare|qemu-img check [^ ]* [^ ]*compact.part|mv -f -- [^ ]*compact.part|sync -- [^ ]*appliance\.[A-Za-z0-9]+$|sha256sum|chmod 0444|virsh_ undefine)' "$CALLS" \
          | sed -E 's|^sync .*compact.part$|sync copy|; s|^qemu-img check .*|check copy|; s|^mv .*|rename|; s|^sync -- .*|sync dir|' | tr '\n' ,; }

fresh; before=$(stat -c %s "$TD/appliance.qcow2")
run >"$LOGD/pass.out" 2>&1; rc=$?
check "a verdict that passes exits 0" test "$rc" = 0 -a "$(grep -c '^after the verdict' "$LOGD/pass.out")" = 1
check "the order: convert, sync, compare, the copy checked, rename, sync, sha256, read-only, undefine ($(order))" \
    test "$(order)" = "qemu-img convert,sync copy,qemu-img compare,check copy,rename,sync dir,sha256sum,chmod 0444,virsh_ undefine,"
check "the disk is shorter after ($before before, $(stat -c %s "$TD/appliance.qcow2") after), 0444, and no copy is left" \
    test "$(stat -c %s "$TD/appliance.qcow2")" -lt "$before" -a "$(stat -c %a "$TD/appliance.qcow2")" = 444 -a ! -e "$TD/appliance.compact.part"
check "the recorded sha256 is of the compacted file" test "$(cut -d' ' -f1 "$LOGD/appliance-sha256.txt")" = "$(command sha256sum "$TD/appliance.qcow2" | cut -d' ' -f1)"
check "the seal's compactness record passes" test "$(grep -c '^PROOF-FAIL' "$LOGD/appliance-compact.txt")" = 0 -a "$(grep -c '^end offset' "$LOGD/appliance-compact.txt")" = 1
check "the compaction record names both lengths, the comparison and the copy's figures" \
    test "$(grep -c "^before compaction: $before bytes long" "$LOGD/appliance-compaction.txt")" = 1 -a "$(grep -c '^Images are identical' "$LOGD/appliance-compaction.txt")" = 1 \
         -a "$(grep -c '^end offset' "$LOGD/appliance-compaction.txt")" = 1 -a "$(grep -c '^after compaction: .*compared equal$' "$LOGD/appliance-compaction.txt")" = 1
check "the build guest's host key, pin and seed are gone" test ! -e "$TD/hostkey" -a ! -e "$TD/known_hosts" -a ! -e "$TD/seed.iso"
for c in convert-fails compare-differs not-compact; do
    fresh; s0=$(sha256sum "$TD/appliance.qcow2" | cut -d' ' -f1)
    run "$c" >"$LOGD/$c.out" 2>&1; rc=$?
    check "$c: refused, nothing marked sealed, the disk unchanged and no copy left" \
        test "$rc" = 1 -a "$(not_sealed && echo y)" = y -a "$(sha256sum "$TD/appliance.qcow2" | cut -d' ' -f1)" = "$s0" \
             -a "$(grep -c 'REFUSED: the compact.*the disk is unchanged' "$LOGD/$c.out")" = 1
    run >"$LOGD/$c-again.out" 2>&1; rc=$?
    check "$c: verdict run again afterwards passes" test "$rc" = 0 -a "$(stat -c %a "$TD/appliance.qcow2")" = 444
done
check "not-compact: the refusal names the copy as not compact" grep -q 'REFUSED: the compacted copy is not compact' "$LOGD/not-compact.out"
fresh; echo left >"$TD/appliance.compact.part"
run >"$LOGD/leftover.out" 2>&1; rc=$?
check "a copy left by a run cut short is refused and kept, with no convert run, and may be removed alone" \
    test "$rc" = 1 -a "$(cat "$TD/appliance.compact.part")" = left -a "$(grep -c '^qemu-img convert' "$CALLS")" = 0 \
         -a "$(grep -c 'compact.part stands, left by a compaction cut short. It may be removed by hand, alone, and verdict run again' "$LOGD/leftover.out")" = 1
fresh; s0=$(sha256sum "$TD/appliance.qcow2" | cut -d' ' -f1)
run slow-convert >"$LOGD/term.out" 2>&1 & p=$!
if wait_for "$p" 60 test -e "$LOGD/converted"; then
    kill -TERM "$(cat "$LOGD/pid")"; wait "$p"; rc=$?
else
    no "slow-convert: the convert's marker within 60 seconds (the run ended or hung first)"
    kill "$p" 2>/dev/null; wait "$p" 2>/dev/null; rc=none
fi
check "a TERM during the compaction exits 143, removes the copy and leaves the disk unchanged and unsealed" \
    test "$rc" = 143 -a ! -e "$TD/appliance.compact.part" -a "$(sha256sum "$TD/appliance.qcow2" | cut -d' ' -f1)" = "$s0" \
         -a "$(grep -c '^virsh_ undefine' "$CALLS")" = 0 -a "$(stat -c %a "$TD/appliance.qcow2")" = 644
fresh
run undefine-fails >"$LOGD/undef.out" 2>&1; rc=$?
check "an undefine that fails leaves the digest recorded and the disk read-only, for proof" \
    test "$rc" != 0 -a "$(cut -d' ' -f1 "$LOGD/appliance-sha256.txt")" = "$(command sha256sum "$TD/appliance.qcow2" | cut -d' ' -f1)" \
         -a "$(stat -c %a "$TD/appliance.qcow2")" = 444
for v in none 2.0; do
    fresh; cp "$IMG/etc/weaver-appliance/stamp" "$T/stamp.keep"
    if [ "$v" = none ]; then sed -i '/^appliance.version/d' "$IMG/etc/weaver-appliance/stamp"
    else sed -i "s/^appliance.version = .*/appliance.version = $v/" "$IMG/etc/weaver-appliance/stamp"; fi
    run >"$LOGD/ver-$v.out" 2>&1; rc=$?
    check "a stamp with version $v is refused before the compaction, nothing marked sealed" \
        test "$rc" = 1 -a "$(not_sealed && echo y)" = y -a "$(grep -c '^qemu-img convert' "$CALLS")" = 0 \
             -a "$(grep -cE "REFUSED: the image's stamp names (no appliance.version|appliance.version 2.0, not VERSION 1.0)" "$LOGD/ver-$v.out")" = 1
    cp "$T/stamp.keep" "$IMG/etc/weaver-appliance/stamp"
done
finish
