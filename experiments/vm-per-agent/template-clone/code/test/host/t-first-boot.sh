#!/usr/bin/env bash
# proof_first_boot's grading, with no guest: virt-install, xorriso, ssh-keyscan, the
# guest's answers (gssh), the placement and the domain's removal stood in. Real: the seed
# the image must ignore (the cloud-init proposal of 2026-11-15), the prompt check,
# proof_keyscan's merge (the Planner's review of 2026-11-15, F11), the host-key match, the
# GAP line, and manifest_compare with lib.sh's lists. Each run has its own VM_ROOT. Pure
# logic: no root, no guest.
source "$(dirname "$0")/lib-test.sh"
need ssh-keygen python3
BT=$CODE/build-template.sh
export VM_ROOT=$T/vmroot; mkdir -p "$VM_ROOT"; source "$CODE/lib.sh"
eval "$(fn "$BT" proof_first_boot proof_keyscan)"
NAME=mock-appliance PORT=2230 PROOF_OWN="" PROOF_FIRST_BOOT=4 PROOF_NORELABEL=stand-in
for t in rsa ecdsa ed25519; do ssh-keygen -q -t $t -N '' -C x -f "$T/hk_$t" </dev/null; done
ssh-keygen -q -t ed25519 -N '' -C x -f "$T/other_ed25519" </dev/null
ssh-keygen -q -t ed25519 -N '' -C x -f "$T/proofkey" </dev/null; KEY=$T/proofkey
proof_overlay() { : >"$1"; }
qemu_file() { :; }
proof_place() { echo "placed (stand-in)"; }
proof_sleep() { :; }
proof_inbound() { echo "== the base inbound policy (stand-in)"; }
dom_drop() { return 0; }
pool_drop() { return 0; }
virt-install() { printf '%s\n' "$@" >"$T/vi-args"; sed 's/$/\r/' <"$T/console" >"$VM_ROOT/first/console.log"; }
# xorriso: writes the image it is asked for and records its path in T/iso-path, or with
# T/no-iso fails. The proof removes its directory, the image with it, when it ends.
xorriso() { local o=""; [ -e "$T/no-iso" ] && return 1
    while [ $# -gt 0 ]; do [ "$1" = -o ] && { o=$2; shift; }; shift; done; : >"$o"; printf '%s\n' "$o" >"$T/iso-path"; }
# ssh-keyscan: the three keys, or with T/scan-other a foreign ed25519 key, or with
# T/scan-split the first two keys on the first scan and the third alone on the retry.
ssh-keyscan() { local p n=0
    [ -e "$T/scan-split" ] && { n=$(cat "$T/scan-count" 2>/dev/null || echo 0); echo $((n + 1)) >"$T/scan-count"; }
    for p in "$T"/hk_*.pub; do
        [ -e "$T/scan-other" ] && [ "${p##*/}" = hk_ed25519.pub ] && p=$T/other_ed25519.pub
        if [ -e "$T/scan-split" ]; then
            if [ "$n" = 0 ]; then [ "${p##*/}" = hk_ed25519.pub ] && continue; else [ "${p##*/}" = hk_ed25519.pub ] || continue; fi
        fi
        printf '[127.0.0.1]:2330 %s\n' "$(cut -d' ' -f1,2 "$p")"
    done; }
# The sealed MANIFEST: two files and one link. The booted listing changes machine-id and
# adds the host keys' public halves and the proof's netplan file, or (T/booted-extra) also
# changes /etc/passwd, which is not on the expected list.
D=0000000000000000000000000000000000000000000000000000000000000000
E=1111111111111111111111111111111111111111111111111111111111111111
mkdir -p "$T/out"
printf '%s  /etc/machine-id\n%s  /etc/passwd\n%s\n/etc/x\t/dev/null\n' "$D" "$D" "$LINKS_MARK" >"$T/out/MANIFEST"
gssh() { shift
    case $* in
        *"ssh_host_*_key.pub"*) [ -e "$T/no-login" ] && { echo "root@127.0.0.1: Permission denied (publickey)."; return 255; }
            for p in "$T"/hk_*.pub; do ssh-keygen -lf "$p"; done ;;
        *"cloud-init status --long"*) printf '%s\n' "$*" >"$T/reads.sh"; echo "== cloud-init, with a seed attached"; echo "status: disabled"
            echo "== the seed's asks"; echo "not run: bootcmd"; echo "not run: runcmd"; echo "not made: the default account"
            echo "== the hostname"; echo "$IMAGE_HOSTNAME" ;;
        *"-type f"*) printf '%s  /etc/machine-id\n' "$E"; printf '%s  %s\n' "$([ -e "$T/booted-extra" ] && echo "$E" || echo "$D")" /etc/passwd
            printf '%s  /etc/ssh/ssh_host_ed25519_key.pub\n%s  %s\n' "$E" "$E" "$PROOF_NETPLAN" ;;
        *"-type l"*) printf '/etc/x\0/dev/null\0' ;;
        *) echo "unexpected: $*" ;;
    esac; }
run() { VM_ROOT=$(mktemp -d "$T/vmroot.XXXXXX"); rm -f -- "$T/out/first-boot.txt" "$T/scan-count" "$T/vi-args"; ( proof_first_boot "$T/out" ) >"$T/run.out" 2>&1; }
fb() { cat "$T/out/first-boot.txt"; }
match3='^the host keys: MATCH, the guest holds each key SSH presents, and SSH presents each the guest holds (3 keys)$'

printf 'Ubuntu 26.04.1 LTS %s ttyS0\n\n%s login: ' "$IMAGE_HOSTNAME" "$IMAGE_HOSTNAME" >"$T/console"
run
check "a first boot that greets as the image, whose keys match, passes with no PROOF-FAIL" \
    test "$(fb | grep -c '^PROOF-FAIL')" = 0 -a "$(fb | grep -c "$match3")" = 1
check "and names the console-login gap once, as a GAP line, not a failure" \
    test "$(fb | grep -c '^GAP: the console login, and the account it lands in, wait for the operator')" = 1
check "and its MANIFEST comparison marks machine-id, the host key and the netplan file expected, with 0 findings" \
    test "$(fb | grep -c '(expected)$')" = 3 -a "$(fb | grep -c ', 0 finding(s)$')" = 1
check "and reads the inbound policy" test "$(fb | grep -c 'base inbound policy (stand-in)')" = 1
check "the in-guest reads parse as bash" bash -n "$T/reads.sh"
check "a seed is attached, as a SATA cdrom beside the overlay" \
    test "$(cat "$T/iso-path")" = "$VM_ROOT/first/seed.iso" -a "$(grep -cxF -- "path=$VM_ROOT/first/seed.iso,device=cdrom,bus=sata" "$T/vi-args")" = 1
check "its meta-data names a hostname of its own and the proof key" \
    test "$(grep -cx 'local-hostname: weaver-proof-seed' "$T/out/first-boot-seed-meta-data.txt")" = 1 \
         -a "$(grep -cxF -- "  - $(cut -d' ' -f1,2 "$KEY.pub")" "$T/out/first-boot-seed-meta-data.txt")" = 1
check "its user-data asks for a bootcmd, a runcmd and the default account" \
    test "$(grep -c 'weaver-proof-bootcmd' "$T/out/first-boot-seed-user-data.txt")" = 1 -a "$(grep -c 'weaver-proof-runcmd' "$T/out/first-boot-seed-user-data.txt")" = 1 \
         -a "$(grep -cx '  - default' "$T/out/first-boot-seed-user-data.txt")" = 1
check "the in-guest reads require cloud-init disabled, the marker, and none of the asks carried out" \
    test "$(grep -c 'not disabled, with a seed attached' "$T/reads.sh")" = 1 -a "$(grep -c '/etc/cloud/cloud-init.disabled' "$T/reads.sh")" = 1 \
         -a "$(grep -c '/var/tmp/weaver-proof-bootcmd /var/tmp/weaver-proof-runcmd' "$T/reads.sh")" = 1 -a "$(grep -c "default account was made" "$T/reads.sh")" = 1
touch "$T/no-iso"; run; rc=$?; rm -f -- "$T/no-iso"
check "a seed image that cannot be made refuses, and nothing boots" \
    test "$rc" != 0 -a ! -e "$T/vi-args" -a "$(grep -c "REFUSED: the first boot's seed image could not be made; nothing booted" "$T/run.out")" = 1
touch "$T/scan-split"; run; rm -f -- "$T/scan-split"
check "a first scan that misses a key and a retry that finds it: all three kept, and they match" \
    test "$(fb | grep -c "$match3")" = 1 -a "$(fb | grep -c '^PROOF-FAIL')" = 0 -a "$(fb | grep -c '1 more scans$')" = 1
touch "$T/booted-extra"; run; rm -f -- "$T/booted-extra"
check "a first boot that changes /etc/passwd fails: the account files are off the expected list" \
    test "$(fb | grep -c '^PROOF-FAIL: the first boot changed a MANIFEST file outside the expected list: /etc/passwd$')" = 1
printf 'Ubuntu 26.04.1 LTS x ttyS0\n\nweaver-appliance-build5 login: ' >"$T/console"
run
check "a console that greets with the build guest's name fails" \
    test "$(fb | grep -c "^PROOF-FAIL: the console's login prompt is 'weaver-appliance-build5 login:', not the image's hostname")" = 1
{ printf 'Ubuntu 26.04.1 LTS %s ttyS0\n\n%s login: \n' "$IMAGE_HOSTNAME" "$IMAGE_HOSTNAME"; head -c 400000 /dev/zero | tr '\0' x | fold -w 100; } >"$T/console"
run
check "a console with 400 kB after the prompt is read at once, not at the bound (the wait under pipefail)" \
    test "$(fb | grep -c '^== first boot, 0 s to the console.s login prompt')" = 1 -a "$(fb | grep -c '^PROOF-FAIL')" = 0
: >"$T/console"; run
check "a console with no login prompt fails, after the bound" \
    test "$(fb | grep -c "^PROOF-FAIL: the console's login prompt is 'none'")" = 1 -a "$(fb | grep -c '^== first boot, 4 s to')" = 1
printf '%s login: ' "$IMAGE_HOSTNAME" >"$T/console"
touch "$T/scan-other"; run; rm -f -- "$T/scan-other"
check "a key SSH presents that the guest does not hold fails, and nothing more is read" \
    test "$(fb | grep -c '^PROOF-FAIL: the host keys: the guest.s fingerprints do not name the keys SSH presents$')" = 1 \
         -a "$(fb | grep -c 'cloud-init, with a seed attached')" = 0 -a "$(fb | grep -c 'MANIFEST against')" = 0
touch "$T/no-login"; run; rm -f -- "$T/no-login"
check "a login as root that is refused fails the host-key match" \
    test "$(fb | grep -c '^PROOF-FAIL: the host keys:')" = 1 -a "$(fb | grep -c 'exited non-zero')" = 1
finish
