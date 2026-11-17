#!/usr/bin/env bash
# The offline reads of a btrfs root (release 2.0), lib.sh's own: image_path_regex, the
# reads through btrfs restore with their guards (the Planner's rulings of 2026-11-16: each
# path asked for must come out or be refused by name, an expected absence is its own call,
# a misspelled path refuses, a path with regex metacharacters matches only itself, a
# directory request returns its tree), image_open's filesystem type and the dispatch by it,
# and image_late_writes on btrfs. The disks are built here: a GPT with one "Linux root
# (x86-64)" partition, holding a btrfs that mkfs.btrfs fills from a directory (zstd, as the
# image's root is), or ext4, or nothing, each in a qcow2 that qemu-storage-daemon exports
# over FUSE as the user. Pure logic: no root, no guest.
source "$(dirname "$0")/lib-test.sh"
need qemu-img qemu-storage-daemon fusermount3 sfdisk blkid mkfs.btrfs btrfs mkfs.ext4 debugfs
export VM_ROOT=$T/vmroot; mkdir -p "$VM_ROOT"; source "$CODE/lib.sh"

# ---- image_path_regex, alone.
re() { image_path_regex "$@" 2>"$T/re.err"; }
m() { grep -qxE -- "$1" <<<"$2"; }   # m REGEX PATH: the regex matches PATH whole
r=$(re /etc/a.b)
check "a name with a dot matches itself, its directories and the root" eval 'm "$r" /etc/a.b && m "$r" /etc && m "$r" /'
check "and nothing else: not a name the dot would match as a wildcard, nor below it" \
    eval '! m "$r" /etc/aXb && ! m "$r" /etc/a.b/x && ! m "$r" /etcx'
r=$(re '/etc/x+y' '/etc/(p)' '/etc/[q]' '/etc/a|b' '/etc/s^t$' '/etc/b\c' '/etc/{2}')
for n in 'x+y' '(p)' '[q]' 'a|b' 's^t$' 'b\c' '{2}'; do
    check "the metacharacters of '$n' match only that name" m "$r" "/etc/$n"
done
check "and none of the strings they would match as a regex" \
    eval '! m "$r" /etc/xxy && ! m "$r" /etc/p && ! m "$r" /etc/q && ! m "$r" /etc/a && ! m "$r" /etc/b && ! m "$r" /etc/st && ! m "$r" /etc/bc'
r=$(re '/etc/d/**')
check "a directory request matches the directory and everything under it" \
    eval 'm "$r" /etc/d && m "$r" /etc/d/e && m "$r" /etc/d/e/f && m "$r" "/etc/d/a b"'
check "but not a sibling that begins with its name" eval '! m "$r" /etc/dd && ! m "$r" /etc/dd/e'
for bad in etc/a / '' /etc//a /etc/./a /etc/../a '/etc/a/'; do
    re "$bad" >/dev/null; rc=$?
    check "'$bad' is refused, status 2, and named" test "$rc" = 2 -a "$(grep -c "'$bad' is not an absolute path" "$T/re.err")" = 1
done
re >/dev/null; check "no path at all is refused" test $? = 2

# ---- The stand-in root tree, and the disks.
S=$T/tree
mkdir -p "$S/etc/d/e" "$S/etc/empty" "$S/etc/weaver-appliance" "$S/var/log/journal/abc" \
    "$S/var/lib/systemd" "$S/var/lib/wtmpdb" "$S/opt/weaver/lib"
echo weaver-appliance >"$S/etc/hostname"
for n in a.b aXb 'x+y' '(p)'; do printf '%s\n' "$n" >"$S/etc/$n"; done
echo deep >"$S/etc/d/e/f"; ln -s ../a.b "$S/etc/d/l"; ln -s d "$S/etc/link-dir"
echo "appliance.version = 1.1" >"$S/etc/weaver-appliance/stamp"
head -c 100 /dev/zero >"$S/var/log/journal/abc/system.journal"; : >"$S/var/log/empty.log"
head -c 32 /dev/zero >"$S/var/lib/systemd/random-seed"; echo w >"$S/var/lib/wtmpdb/wtmp.db"
head -c 300000 /dev/urandom >"$S/opt/weaver/lib/libggml-cuda.so.0.1"
ln -s libggml-cuda.so.0.1 "$S/opt/weaver/lib/libggml-cuda.so.0"
# Absolute links in the image: one to /etc, and one to a stand-in for a host directory.
# Restored, each keeps its target as stored, so on the host it leads out of the restore.
HOSTSIDE=$T/host-side; mkdir -p "$HOSTSIDE/logs"
echo "the host's own" >"$HOSTSIDE/hostname"; echo x >"$HOSTSIDE/logs/h.log"
ln -s /etc "$S/etc/up"; ln -s "$HOSTSIDE" "$S/etc/out"
# A link whose target is over 60 bytes: ext4 keeps such a target in a block, and debugfs's
# stat prints no "Fast link dest:" line for it.
LONG=/etc/a-target-file-whose-name-runs-long-enough-that-ext4-keeps-it-in-a-block
echo long >"$S$LONG"; ln -s "$LONG" "$S/etc/long-link"
# disk NAME KIND: T/NAME.qcow2, read-only, its root partition KIND (btrfs, ext4, none),
# filled from DIR where given.
disk() {
    local n=$1 kind=$2 dir=${3:-$S} sectors=270336
    truncate -s $(( (sectors + 4096) * 512 )) "$T/$n.raw"
    printf 'label: gpt\nstart=2048, size=%s, type=4F68BCE3-E8CD-4DB1-96E7-FBCAF984B709\n' "$sectors" | sfdisk -q "$T/$n.raw" || return 1
    truncate -s $((sectors * 512)) "$T/$n.part"
    case $kind in
        btrfs) mkfs.btrfs -q --compress zstd --rootdir "$dir" "$T/$n.part" >/dev/null || return 1 ;;
        ext4) mkfs.ext4 -q -F -d "$dir" "$T/$n.part" || return 1 ;;
        none) ;;
    esac
    dd if="$T/$n.part" of="$T/$n.raw" bs=512 seek=2048 conv=notrunc status=none
    qemu-img convert -f raw -O qcow2 "$T/$n.raw" "$T/$n.qcow2" && chmod 0444 "$T/$n.qcow2"
}
disk b btrfs && disk e ext4 && disk z none || exit 1
mkdir -p "$T/noetc/var"; echo x >"$T/noetc/var/x"; disk n btrfs "$T/noetc" || exit 1
qsd_count() { pgrep -u "$(id -u)" -f "qemu-storage-daemon.*$T/" | wc -l; }
mounted() { findmnt -rn -o TARGET | grep -F "$T/"; }
before=$(sha256sum "$T/b.qcow2" | cut -d' ' -f1)
# Each export is named in OPENED before it is opened, and the EXIT trap closes every one,
# a half-opened one included, so a stop anywhere (an exit, a failed open, a signal) leaves
# no FUSE export on the host (CodeRabbit on pull request 35, 2026-11-17: a failure between
# two opens exited past the closes). image_close is quiet and never fails, so the trap
# names each export it finds still mounted, and a failed open is a FAIL line with
# image_open's own cause above it.
OPENED=""
open_export() { OPENED+=" $2"; ( image_open "$1" "$2" ); }
close_all() {
    local d
    for d in $OPENED; do
        ! mounted | grep -qF "$d/" || echo "closing the export left under $d" >&2
        image_close "$d"
    done
}
trap close_all EXIT
trap 'exit 130' INT; trap 'exit 143' TERM; trap 'exit 129' HUP
open_or_stop() { open_export "$1" "$2" || { no "image_open of $1 (the cause above)"; finish; exit 1; }; }

# ---- image_open: the filesystem's type, and a root it cannot read refused.
open_or_stop "$T/b.qcow2" "$T/rb"
open_or_stop "$T/e.qcow2" "$T/re"
check "image_open records btrfs and ext4 by what blkid reads" test "$(cat "$T/rb/fs")" = btrfs -a "$(cat "$T/re/fs")" = ext4
open_export "$T/z.qcow2" "$T/rz" >"$T/z.out" 2>&1; rc=$?
check "a root partition with no filesystem is refused, named, and its export stopped" \
    test "$rc" = 1 -a "$(grep -c "holds 'no filesystem blkid knows', not ext4 or btrfs" "$T/z.out")" = 1 -a ! -e "$T/rz/root.raw"
check "the reads dispatch: the same image_get reads either filesystem" \
    eval 'image_get "$T/rb" /etc/hostname "$T/h1" && image_get "$T/re" /etc/hostname "$T/h2" && cmp -s "$T/h1" "$T/h2"'
image_state "$T/vmroot" /etc 2>"$T/nofs.err"; rc=$?
check "a directory image_open did not make is refused, status 2" test "$rc" = 2 -a -s "$T/nofs.err"

R=$T/rb
# ---- What comes out: the path asked for, its directories, /etc, and nothing beside.
# image_restore's caller removes its directory; the reads below do so themselves.
o=$(image_restore "$R" /etc/a.b)
check "a restore brings out the file asked for, and not the one its dot would match" \
    test -f "$o/etc/a.b" -a ! -e "$o/etc/aXb" -a ! -e "$o/etc/hostname"
rm -rf -- "${o:?}"; o=$(image_restore "$R" '/etc/x+y')
check "a name with + brings out itself alone" test -f "$o/etc/x+y" -a "$(ls "$o/etc" | wc -l)" = 1
rm -rf -- "${o:?}"; o=$(image_restore "$R" '/etc/d/**')
check "a directory request brings out its tree, links as links" \
    test -f "$o/etc/d/e/f" -a -L "$o/etc/d/l" -a "$(readlink "$o/etc/d/l")" = ../a.b
rm -rf -- "${o:?}"

# ---- image_get, image_cat: a misspelled path refuses by name.
image_get "$R" /etc/hostnme "$T/g" 2>"$T/g.err"; rc=$?
check "image_get of a misspelled path refuses, status 2, naming it, and writes nothing" \
    test "$rc" = 2 -a ! -e "$T/g" -a "$(grep -c '^image_get: /etc/hostnme did not come out of the image' "$T/g.err")" = 1
image_get "$R" '/etc/(p)' "$T/g"
check "image_get of a name with parentheses copies that file" test "$(cat "$T/g")" = '(p)'
image_get "$R" /opt/weaver/lib/libggml-cuda.so.0.1 "$T/g"
check "a larger file, compressed on disk, comes out whole" cmp -s "$T/g" "$S/opt/weaver/lib/libggml-cuda.so.0.1"
image_get "$R" /etc/d "$T/g" 2>"$T/g.err"; rc=$?
check "image_get of a directory refuses" test "$rc" = 2 -a "$(grep -c 'is not a regular file' "$T/g.err")" = 1
image_get "$R" /etc/d/l "$T/g" 2>"$T/g.err"; rc=$?
check "image_get of a link refuses: no read follows a link" test "$rc" = 2 -a "$(grep -c 'is not a regular file' "$T/g.err")" = 1
check "image_cat prints a file" test "$(image_cat "$R" /etc/hostname)" = weaver-appliance
out=$(image_cat "$R" /etc/hostnme 2>"$T/c.err"); rc=$?
check "image_cat of a misspelled path refuses by name, and prints nothing" \
    test "$rc" = 2 -a -z "$out" -a "$(grep -c '/etc/hostnme did not come out' "$T/c.err")" = 1

# ---- image_state, image_get_present: the explicit absence.
st() { image_state "$R" "$1" 2>"$T/st.err"; echo $?; }
check "image_state: a file present is 0" test "$(st /etc/hostname)" = 0
check "a directory present is 0, and a link present is 0" test "$(st /etc/empty)" = 0 -a "$(st /etc/d/l)" = 0
check "a path absent is 1" test "$(st /etc/hostnme)" = 1
check "a path under a directory that is absent is 1" test "$(st /etc/nothing/deeper)" = 1
check "a path through a link is 2, named: the read does not follow it" \
    test "$(st /etc/link-dir/e)" = 2 -a "$(grep -c 'lies under /etc/link-dir, a link' "$T/st.err")" = 1
image_get_present "$R" /etc/weaver-appliance/SEAL-REFUSED "$T/p"; rc=$?
check "image_get_present of an absent file is 1, and writes nothing" test "$rc" = 1 -a ! -e "$T/p"
image_get_present "$R" /etc/weaver-appliance/stamp "$T/p"; rc=$?
check "and of a present one is 0, the file copied" test "$rc" = 0 -a "$(cat "$T/p")" = "appliance.version = 1.1"

# ---- A directory above the path that is a link in the image: never followed on the host.
image_get "$R" /etc/up/hostname "$T/g" 2>"$T/g.err"; rc=$?
check "image_get under an absolute link to /etc refuses, status 2, naming the link, and writes nothing" \
    test "$rc" = 2 -a ! -e "$T/g" -a "$(grep -c 'lies under /etc/up, a link in the image' "$T/g.err")" = 1
image_get "$R" /etc/out/hostname "$T/g" 2>"$T/g.err"; rc=$?
check "under a link to a host directory, image_get refuses and copies nothing of the host's" \
    test "$rc" = 2 -a ! -e "$T/g" -a "$(grep -c 'lies under /etc/out, a link in the image' "$T/g.err")" = 1
check "image_state of that path is 2, not present" test "$(st /etc/out/hostname)" = 2
image_files "$R" /etc/out/logs >"$T/f.out" 2>"$T/f.err"; rc=$?
check "image_files under it is 2 and lists nothing of the host's" test "$rc" = 2 -a ! -s "$T/f.out"
image_ls "$R" /etc/out/logs >"$T/l.out" 2>/dev/null; rc=$?
check "image_ls under it is 2 and lists nothing" test "$rc" = 2 -a ! -s "$T/l.out"
image_resolve "$R" /etc/out/hostname >"$T/r.out" 2>"$T/r.err"; rc=$?
check "image_resolve under it is 2, refused, naming the link" \
    test "$rc" = 2 -a ! -s "$T/r.out" -a "$(grep -c 'lies under /etc/out' "$T/r.err")" = 1
# The perturbation: image_came_out as it stood until 2026-11-17, testing the whole path
# with -e, which follows each link on the way, makes the same reads take the host's files.
( image_came_out() { [ -e "$1$2" ] || [ -L "$1$2" ]; }
  image_get "$R" /etc/out/hostname "$T/g-old"; image_files "$R" /etc/out/logs >"$T/f-old" ) 2>/dev/null
check "the perturbation: with the old test, image_get copies the host's file and image_files lists the host's" \
    test "$(cat "$T/g-old" 2>/dev/null)" = "the host's own" -a "$(cat "$T/f-old")" = "2 /etc/out/logs/h.log"

# ---- image_ls, image_files, image_resolve.
check "image_ls lists a directory's entries, a link with its target" \
    test "$(image_ls "$R" /etc/d)" = "$(printf 'd e\nl l -> ../a.b')"
image_ls "$R" /etc/nothing >/dev/null 2>&1; rc=$?
check "image_ls of an absent directory is 1" test "$rc" = 1
check "image_files lists every regular file under a directory, size and path" \
    test "$(image_files "$R" /var/log)" = "$(printf '0 /var/log/empty.log\n100 /var/log/journal/abc/system.journal')"
out=$(image_files "$R" /etc/empty); rc=$?
check "an empty directory lists nothing, status 0" test "$rc" = 0 -a -z "$out"
image_files "$R" /var/lgo >/dev/null 2>"$T/f.err"; rc=$?
check "image_files of a misspelled directory refuses, status 2, naming it" \
    test "$rc" = 2 -a "$(grep -c '^image_files: /var/lgo did not come out of the image' "$T/f.err")" = 1
image_files "$R" /etc/hostname >/dev/null 2>"$T/f.err"; rc=$?
check "and of a file is refused, not listed" test "$rc" = 2 -a "$(grep -c 'is not a directory' "$T/f.err")" = 1
check "image_resolve follows a link in the image to its file" \
    test "$(image_resolve "$R" /opt/weaver/lib/libggml-cuda.so.0)" = /opt/weaver/lib/libggml-cuda.so.0.1
image_resolve "$R" /opt/weaver/lib/libnothing.so >/dev/null 2>&1; rc=$?
check "and fails on a path that did not come out" test "$rc" = 1
# A link whose target climbs with "..": /etc/d/l -> ../a.b, though /etc/a.b stands. It is
# refused, 2, never resolved and never reported missing (the Planner's ruling of
# 2026-11-17, on CodeRabbit's review of pull request 35), and no read is made through it:
# each restore the resolve asks for is logged, and only the link itself is asked for.
resolve_logged() {
    ( eval "orig_$(declare -f image_restore)"
      image_restore() { printf '%s\n' "${*:2}" >>"$T/restores"; orig_image_restore "$@"; }
      image_resolve "$1" "$2" ) >"$T/r.out" 2>"$T/r.err"
}
: >"$T/restores"; resolve_logged "$R" /etc/d/l; rc=$?
check "image_resolve of a link that climbs with '..' is 2, naming the link and its target, and prints no path" \
    test "$rc" = 2 -a ! -s "$T/r.out" -a "$(grep -cF "/etc/d/l is a link to ../a.b, which climbs with '..'" "$T/r.err")" = 1
check "and reads only the link: no restore is asked for through it" test "$(cat "$T/restores")" = /etc/d/l
image_resolve "$T/re" /etc/d/l >"$T/r.out" 2>"$T/r.err"; rc=$?
check "the ext4 reader refuses it the same way, 2, naming it" \
    test "$rc" = 2 -a ! -s "$T/r.out" -a "$(grep -cF "/etc/d/l is a link to ../a.b, which climbs with '..'" "$T/r.err")" = 1
# The perturbation: the climb's refusal answering 1, as image_resolve answered before,
# fails the check above.
pert=$(fn "$CODE/lib.sh" image_resolve_btrfs | sed '/image_link_climbs/s/return 2$/return 1/')
( eval "$pert"; image_resolve "$R" /etc/d/l ) >/dev/null 2>&1; rc=$?
check "the perturbation: with the refusal answering 1, the status is 1, which the check above fails" \
    test "$pert" != "$(fn "$CODE/lib.sh" image_resolve_btrfs)" -a "$rc" = 1
# A long-target link (/etc/long-link, its target 76 bytes): btrfs resolves it; on ext4,
# whose stat prints no fast line for it, the target cannot be read, and the link is
# refused, 2, never reported missing.
check "the long target is over 60 bytes, and debugfs prints no fast line for it" \
    eval 'test "${#LONG}" -gt 60 && ! debugfs -c -R "stat /etc/long-link" "$T/re/root.raw" 2>/dev/null | grep -q "^Fast link dest:"'
check "btrfs resolves the long-target link to its file" test "$(image_resolve "$R" /etc/long-link)" = "$LONG"
image_resolve "$T/re" /etc/long-link >"$T/r.out" 2>"$T/r.err"; rc=$?
check "on ext4 it is 2, naming the link whose target cannot be read, and prints no path" \
    test "$rc" = 2 -a ! -s "$T/r.out" -a "$(grep -cF '/etc/long-link is a link whose target cannot be read' "$T/r.err")" = 1
# The perturbation: that refusal answering 1, as it did, gives status 1, which fails it.
pert=$(fn "$CODE/lib.sh" image_resolve_ext4 | sed '/target cannot be read/s/return 2; }$/return 1; }/')
( eval "$pert"; image_resolve "$T/re" /etc/long-link ) >/dev/null 2>&1; rc=$?
check "the perturbation: with the refusal answering 1, the status is 1, which the check above fails" \
    test "$pert" != "$(fn "$CODE/lib.sh" image_resolve_ext4)" -a "$rc" = 1

# ---- image_late_writes on btrfs, as the proof records it.
image_late_writes "$R" >"$T/late.out" 2>&1
check "the late writes: the random seed and the non-empty log are findings" \
    test "$(grep -c '^PROOF-FAIL: /var/lib/systemd/random-seed is in the sealed image' "$T/late.out")" = 1 \
         -a "$(grep -c '^PROOF-FAIL: a log written after the final probe: /var/log/journal/abc/system.journal (100 bytes)$' "$T/late.out")" = 1
check "the credential secret reads absent, the login record is listed, lastlog absent" \
    test "$(grep -cx 'absent: /var/lib/systemd/credential.secret' "$T/late.out")" = 1 \
         -a "$(grep -cx '  2 /var/lib/wtmpdb/wtmp.db' "$T/late.out")" = 1 -a "$(grep -cx '  (absent)' "$T/late.out")" = 1

# ---- A read that does not work is told apart from an absence.
open_or_stop "$T/n.qcow2" "$T/rn"
image_state "$T/rn" /etc/hostname 2>"$T/n.err"; rc=$?
check "a root with no /etc: image_state is 2, not an absence, and says /etc did not come out" \
    test "$rc" = 2 -a "$(grep -c '/etc did not come out' "$T/n.err")" = 1
mkdir -p "$T/rx"; cp "$T/e.part" "$T/rx/root.raw"; echo btrfs >"$T/rx/fs"
image_get "$T/rx" /etc/hostname "$T/x" 2>"$T/x.err"; rc=$?
check "an ext4 root read as btrfs: image_get is 2, naming btrfs restore's failure" \
    test "$rc" = 2 -a "$(grep -c '^image reads: btrfs restore exited' "$T/x.err")" = 1
check "no read leaves a restore directory behind" \
    test -z "$(find "$T/rb" "$T/rn" "$T/rx" -maxdepth 1 \( -name 'restore.*' -o -name 'cat.*' \) | head -1)"

for d in rb re rn; do image_close "$T/$d"; done
check "image_close stops every export, and no FUSE export is left mounted under the test's directory" \
    test "$(qsd_count)" = 0 -a ! -e "$T/rb/root.raw" -a -z "$(mounted)"
check "and the image read is unchanged: the reads export it read-only and write nothing into it" \
    test "$(sha256sum "$T/b.qcow2" | cut -d' ' -f1)" = "$before" -a -z "$(grep -n 'writable=on\|read-only=off' "$CODE/lib.sh")"
# The offline mock reads the seal's verdict through stand-in readers, by its text alone
# (the mock runs only in a build guest): image_get_present reads image_state before
# image_get, so each place it stands in image_get stands in image_state too (the 2.0
# build's third attempt of 2026-11-17 stopped at the mock on that).
MOCK=$CODE/test/offline-mock.sh
check "image_get_present reads image_state, then image_get" \
    eval '[ "$(fn "$CODE/lib.sh" image_get_present | grep -oE "image_(state|get) " | tr -d " " | tr "\n" " ")" = "image_state image_get " ]'
check "the mock stands in image_state wherever it stands in image_get" \
    eval 'n=$(grep -cE "(^|[;[:space:]])image_get\(\) \{" "$MOCK"); [ "$n" -ge 2 ] && [ "$(grep -cE "(^|[;[:space:]])image_state\(\) \{" "$MOCK")" = "$n" ]'
finish
