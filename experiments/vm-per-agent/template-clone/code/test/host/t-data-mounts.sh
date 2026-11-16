#!/usr/bin/env bash
# The two data mount points the profile fixes (weaver-appliance-profile section 3.4), as
# the image lays them: mounts_check from guest/template-steps' own text against a stand-in
# tree, with its root prefix dropped, its paths pointed at the tree, and root's ownership
# read as this account's (the stand-in tree is this account's). Then where it is wired: the
# cleanup lays them and checks them, the probes run the check in both passes, and lib.sh
# reads the one definition. Whether a load finds a separate filesystem on each is admin's
# test, not the image's, and is not here. Pure logic: no root, no guest.
source "$(dirname "$0")/lib-test.sh"
TS=$CODE/guest/template-steps
P='s''udo '
me="$(id -u):$(id -g)"
eval "$(grep -m1 "^DATA_MOUNTS=" "$TS")"
check "the two mount points the profile fixes, defined once" test "$DATA_MOUNTS" = "/mnt/weaver/models /mnt/weaver/state"
M=$T/root
mc=$(fn "$TS" mounts_check | sed -e "s|$P||g" -e "s|/mnt/weaver|$M/mnt/weaver|g" -e "s|directory 0:0 755|directory $me 755|" -e "s|stat -c %d -- / |stat -c %d -- $M |")
DATA_MOUNTS="$M/mnt/weaver/models $M/mnt/weaver/state"
eval "$mc"
laid() { rm -rf -- "${M:?}"; mkdir -p "$M/mnt/weaver/models" "$M/mnt/weaver/state"; chmod 0755 "$M/mnt/weaver" "$M/mnt/weaver/models" "$M/mnt/weaver/state"; }
laid
out=$(mounts_check 2>&1); rc=$?
check "as laid: two empty directories, 0755, on the root's filesystem; no finding" test "$rc" = 0 -a -z "$out"
laid; : >"$M/mnt/weaver/models/weights.gguf"
out=$(mounts_check 2>&1); rc=$?
check "a file in the models mount point: a finding (the image carries no model)" test "$rc" = 1 -a "$(grep -c "$M/mnt/weaver/models holds 1 entries" <<<"$out")" = 1
laid; chmod 0700 "$M/mnt/weaver/state"
out=$(mounts_check 2>&1)
check "a mode other than 0755: a finding" test "$(grep -c "$M/mnt/weaver/state is 'directory $me 700', not a directory root's with mode 755" <<<"$out")" = 1
laid; rmdir "$M/mnt/weaver/state"
out=$(mounts_check 2>&1)
check "a mount point absent: named, and /mnt/weaver no longer holds the two" \
    test "$(grep -c "$M/mnt/weaver/state is 'absent'" <<<"$out")" = 1 -a "$(grep -c '/mnt/weaver does not hold the two mount points alone' <<<"$out")" = 1
laid; mkdir "$M/mnt/weaver/cache"
out=$(mounts_check 2>&1)
check "a third directory beside them: a finding" test "$(grep -c '/mnt/weaver does not hold the two mount points alone' <<<"$out")" = 1
laid; rmdir "$M/mnt/weaver/models"; mkdir "$M/elsewhere"; ln -s "$M/elsewhere" "$M/mnt/weaver/models"
out=$(mounts_check 2>&1)
check "a symbolic link in place of a mount point: a finding" test "$(grep -c "$M/mnt/weaver/models is 'symbolic link" <<<"$out")" = 1
# A mount point on another filesystem than the root's: the comparison pointed at /proc,
# which no stand-in tree shares a device with.
laid
eval "$(sed "s|stat -c %d -- $M |stat -c %d -- /proc |" <<<"$mc")"
out=$(mounts_check 2>&1)
check "a mount point on another filesystem than the root's: a finding for each" \
    test "$(grep -o "is not on the image's root filesystem" <<<"$out" | wc -l)" = 2
eval "$mc"

# Where it is wired.
cleanup=$(fn "$TS" cleanup); probes=$(fn "$TS" probes)
check "the cleanup lays /mnt/weaver and the two, root's, mode 0755, and checks them" \
    bash -c 'grep -qF "sudo install -d -o root -g root -m 0755 /mnt/weaver \$DATA_MOUNTS || return 1" <<<"$1" && grep -qxF "    mounts_check || return 1" <<<"$1"' _ "$cleanup"
final_at=$(grep -n -F 'if [ "$mode" = final ]; then' <<<"$probes" | cut -d: -f1)
at=$(grep -n -F 'check "the data mount points not as laid" "$(mounts_check 2>&1)"' <<<"$probes" | cut -d: -f1)
check "the probes run it in both passes" eval '[ -n "$at" ] && [ -n "$final_at" ] && [ "$at" -lt "$final_at" ]'
( export VM_ROOT=$T/vmroot; mkdir -p "$VM_ROOT"; source "$CODE/lib.sh"; [ "$DATA_MOUNTS" = "/mnt/weaver/models /mnt/weaver/state" ] )
check "lib.sh reads the same two for the proof" test $? = 0
finish
