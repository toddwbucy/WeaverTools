#!/usr/bin/env bash
# The proof's offline placement (decision 16 of 2026-11-15): image_open's writable mode on
# a scratch overlay, image_put, image_got, image_close, and proof_place whole. The disk is
# built here: a GPT with one "Linux root (x86-64)" partition holding ext4, in a qcow2 made
# read-only like a sealed image, and an overlay on it. qemu-storage-daemon exports it over
# FUSE as the user. Pure logic: no root, no guest.
source "$(dirname "$0")/lib-test.sh"
need qemu-img qemu-storage-daemon fusermount3 sfdisk mkfs.ext4 debugfs e2fsck ssh-keygen
export VM_ROOT=$T/vmroot; mkdir -p "$VM_ROOT"; source "$CODE/lib.sh"
eval "$(fn "$CODE/build-template.sh" proof_place)"
# The stand-in root tree: /root/.ssh with an empty authorized_keys, /etc/netplan empty.
mkdir -p "$T/tree/root/.ssh" "$T/tree/etc/netplan" "$T/tree/etc/ssh"
: >"$T/tree/root/.ssh/authorized_keys"; chmod 0700 "$T/tree/root/.ssh"; chmod 0600 "$T/tree/root/.ssh/authorized_keys"
echo base >"$T/tree/etc/hostname"
truncate -s 64M "$T/disk.raw"
printf 'label: gpt\nstart=2048, size=110592, type=4F68BCE3-E8CD-4DB1-96E7-FBCAF984B709\n' | sfdisk -q "$T/disk.raw" || exit 1
truncate -s $((110592 * 512)) "$T/part.raw"
mkfs.ext4 -q -F -d "$T/tree" "$T/part.raw" || exit 1
dd if="$T/part.raw" of="$T/disk.raw" bs=512 seek=2048 conv=notrunc status=none
qemu-img convert -f raw -O qcow2 "$T/disk.raw" "$T/sealed.qcow2"; chmod 0444 "$T/sealed.qcow2"
before=$(sha256sum "$T/sealed.qcow2" | cut -d' ' -f1)
newov() { rm -f -- "$T/ov.qcow2"; qemu-img create -q -f qcow2 -b "$T/sealed.qcow2" -F qcow2 "$T/ov.qcow2"; }
qsd_count() { pgrep -u "$(id -u)" -f "qemu-storage-daemon.*$T/" | wc -l; }

# 1. The sealed image (no backing file) is refused writable, and nothing is exported.
( image_open "$T/sealed.qcow2" "$T/r1" rw ) >"$T/1.out" 2>&1
check "image_open refuses rw on a file that names no backing file" \
    test $? = 1 -a "$(grep -c 'REFUSED: image_open writes only to an overlay' "$T/1.out")" = 1 -a "$(qsd_count)" = 0
# 2. An overlay is opened writable and both files written, then closed, opened read-only
# and read back.
newov
printf 'ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIStandInKeyStandInKeyStandInKeyStandInKey\n' >"$T/ak"
printf 'network:\n  version: 2\n' >"$T/np"
( image_open "$T/ov.qcow2" "$T/r2" rw && image_put "$T/r2" "$T/ak" /root/.ssh/authorized_keys 0100600 \
    && image_put "$T/r2" "$T/np" /etc/netplan/90-weaver-proof.yaml 0100600 ) >"$T/2.out" 2>&1; rc=$?
image_close "$T/r2"
check "an overlay opens writable and both writes run" test "$rc" = 0 -a "$(qsd_count)" = 0
( image_open "$T/ov.qcow2" "$T/r2"; image_got "$T/r2" "$T/ak" /root/.ssh/authorized_keys 0100600; \
  image_got "$T/r2" "$T/np" /etc/netplan/90-weaver-proof.yaml 0100600; e2fsck -fn "$T/r2/root.raw"; echo "fsck $?"; \
  image_close "$T/r2" ) >"$T/2b.out" 2>&1
check "both read back from the overlay opened again, root's, mode 0600" \
    test "$(grep -c ': read back, .* mode 0600, owner 0:0$' "$T/2b.out")" = 2
check "and e2fsck -n finds the filesystem clean" test "$(grep -c '^fsck 0$' "$T/2b.out")" = 1
check "the sealed file is unchanged" test "$(sha256sum "$T/sealed.qcow2" | cut -d' ' -f1)" = "$before"
( image_open "$T/sealed.qcow2" "$T/r3"; image_got "$T/r3" "$T/ak" /root/.ssh/authorized_keys 0100600; image_close "$T/r3" ) >"$T/3.out" 2>&1
check "and still holds its own empty authorized_keys (image_got finds other bytes)" \
    test "$(grep -c 'read back with other bytes than written' "$T/3.out")" = 1
# 3. A directory that is not there is refused, with nothing written.
newov
( image_open "$T/ov.qcow2" "$T/r4" rw && image_put "$T/r4" "$T/np" /etc/absent/x.yaml 0100600 ) >"$T/4.out" 2>&1; rc=$?
image_close "$T/r4"
check "image_put refuses a path whose directory is not in the image" \
    test "$rc" = 1 -a "$(grep -c 'REFUSED: image_put: /etc/absent is not in the image' "$T/4.out")" = 1
# 4. A filesystem not marked clean is refused.
newov
( image_open "$T/ov.qcow2" "$T/r5" rw && debugfs -w -R "ssv state 0" "$T/r5/root.raw" >/dev/null 2>&1 \
    && image_put "$T/r5" "$T/np" /etc/netplan/x.yaml 0100600 ) >"$T/5.out" 2>&1; rc=$?
image_close "$T/r5"
check "image_put refuses a filesystem that is not clean" \
    test "$rc" = 1 -a "$(grep -c 'REFUSED: image_put: the filesystem is not clean' "$T/5.out")" = 1
# 5. A mode or owner other than asked is caught by image_got.
newov
( image_open "$T/ov.qcow2" "$T/r6" rw && image_put "$T/r6" "$T/np" /etc/netplan/90-weaver-proof.yaml 0100644 ) >/dev/null 2>&1
image_close "$T/r6"
( image_open "$T/ov.qcow2" "$T/r6"; image_got "$T/r6" "$T/np" /etc/netplan/90-weaver-proof.yaml 0100600; echo "rc $?"; image_close "$T/r6" ) >"$T/6.out" 2>&1
check "image_got fails a file whose mode is not the one asked" \
    test "$(grep -c "a regular file with mode '0644'" "$T/6.out")" = 1 -a "$(grep -c '^rc 1$' "$T/6.out")" = 1
# 6. proof_place whole: a stand-in key, the overlay in PDIR, the records in OUT.
newov; mkdir -p "$T/pd" "$T/out"; cp "$T/ov.qcow2" "$T/pd/overlay.qcow2"
ssh-keygen -q -t ed25519 -N '' -C stand-in -f "$T/key" </dev/null
exec {PROOF_LOCK_FD}>>"$T/lock"
( KEY=$T/key; proof_place "$T/out" "$T/pd" ) >"$T/7.out" 2>&1; rc=$?
check "proof_place passes, and says both files read back and the filesystem is clean" \
    test "$rc" = 0 -a "$(tail -1 "$T/out/first-boot-place.txt")" = "placed: both files read back as written; the filesystem is clean"
check "and records the key, the network file and debugfs's output, and leaves no export or place directory" \
    test -s "$T/out/first-boot-authorized_keys.txt" -a -s "$T/out/first-boot-netplan.txt" -a -s "$T/out/first-boot-place-debugfs.txt" \
         -a ! -e "$T/pd/place" -a "$(qsd_count)" = 0
check "the authorized key is the stand-in's public line, and the network file names the MAC address" \
    test "$(cat "$T/out/first-boot-authorized_keys.txt")" = "$(cut -d' ' -f1,2 "$T/key.pub")" \
         -a "$(grep -c "macaddress: \"$PROOF_MAC\"" "$T/out/first-boot-netplan.txt")" = 1
# 7. proof_place on the sealed file itself (a PDIR whose overlay is a standalone image)
# refuses.
mkdir -p "$T/pd2" "$T/out2"; cp "$T/sealed.qcow2" "$T/pd2/overlay.qcow2"; chmod 0644 "$T/pd2/overlay.qcow2"
( KEY=$T/key; proof_place "$T/out2" "$T/pd2" ) >"$T/8.out" 2>&1; rc=$?
check "proof_place refuses an overlay that is a standalone image, nothing written" \
    test "$rc" = 1 -a "$(grep -c 'REFUSED: image_open writes only to an overlay' "$T/8.out")" = 1 \
         -a "$(sha256sum "$T/pd2/overlay.qcow2" | cut -d' ' -f1)" = "$before"
# 8. A writable export left open (a run cut short) is closed by image_close, its daemon gone.
newov
( image_open "$T/ov.qcow2" "$T/r9" rw ) >/dev/null 2>&1
n_open=$(qsd_count); image_close "$T/r9"
check "image_close stops a writable export left open and unmounts it" \
    test "$n_open" = 1 -a "$(qsd_count)" = 0 -a ! -e "$T/r9/root.raw"
finish
