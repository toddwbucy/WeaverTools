#!/usr/bin/env bash
# The offline mock of template-clone-Spec.md section 8: the real put, fetch_round,
# driver-install, uris_sha256, ci_judge, cloud_init_verdict, netdev_ok, ensure_key, harness
# preflight and relaxed, and the host scripts' step and seal, with no virtual machine, no
# network beyond loopback and no sudo.
#
#   code/test/offline-mock.sh        exit 0 when every check passes; the first failure stops
#   KEEP=1 code/test/offline-mock.sh keeps its temporary directory for reading
#
# The mock's guest is a temporary home directory. gssh runs each guest command there, with
# a PATH of the host tools the steps need (and no rustc) behind stubs of sudo, apt-get and
# install that record what they are asked. A loopback HTTP server stands in for the
# archive. Each of the 2026-11-12 slice's three script defects (D1, D2, D3) is reproduced
# against the behaviour that had it, and shown caught by this one.
set -uo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
CODE=$(cd "$HERE/.." && pwd)
T=$(mktemp -d "${TMPDIR:-/tmp}/template-mock.XXXXXX")
SRV=
cleanup() { [ -z "$SRV" ] || kill "$SRV" 2>/dev/null; [ -n "${KEEP:-}" ] || rm -rf -- "${T:?}"; }
trap cleanup EXIT

pass=0
ok() { printf 'ok   %s\n' "$*"; pass=$((pass + 1)); }
no() { printf 'FAIL %s\n' "$*"; exit 1; }
check() { local what=$1; shift; if "$@"; then ok "$what"; else no "$what"; fi; }
refused() { ! ( "$@" ) >/dev/null 2>&1; }   # refused CMD...: CMD fails, or stops its subshell

# ---- the host side: lib.sh as the scripts source it, gssh pointed at the mock guest
export VM_ROOT=$T/vm
mkdir -p "$VM_ROOT"
# shellcheck source=../lib.sh
source "$CODE/lib.sh"
KNOWN_HOSTS=$T/known_hosts
: >"$KNOWN_HOSTS"
G=$T/guest      # the guest's home
B=$T/bin        # stubs, first on the guest's PATH
S=$T/sys        # the host tools the guest steps need, and nothing else
LOG=$T/log
CACHE=$T/cache  # the host's checked .debs (DEB_CACHE)
mkdir -p "$G" "$B" "$S" "$LOG" "$CACHE" "$T/lists" "$T/archives" "$T/www/pool"
for t in bash sh env cat cp rm mkdir find tar sed awk grep cut tail sort id basename \
         chmod python3 sha256sum stat sleep timeout; do
    p=$(command -v "$t") || no "the host has no $t"
    ln -s "$p" "$S/$t"
done
gssh() {
    shift
    (cd "$G" && env -i HOME="$G" PATH="$B:$S" LANG=C.UTF-8 APT_LISTS="$T/lists" \
        APT_CACHE="$T/archives" MOCK="$T" bash -c "$*")
}
guest_reset() { rm -rf -- "${G:?}" "${T:?}/archives" "${T:?}/calls"; mkdir -p "$G" "$T/archives"; : >"$T/calls"; }

# ---- the guest's stubs
cat >"$B/sudo" <<'EOF'
#!/bin/bash
# runs the command as the caller: leading VAR=value words become its environment
while [[ "${1:-}" == [A-Z]*=* ]]; do export "${1?}"; shift; done
[ "${1:-}" != -n ] || shift
exec "$@"
EOF
cat >"$B/install" <<'EOF'
#!/bin/bash
# install [-o U] [-g G] [-m MODE] FILE... DIR: a copy, owner ignored, recorded
m=0644; a=()
while [ $# -gt 0 ]; do case $1 in -o|-g) shift 2 ;; -m) m=$2; shift 2 ;; *) a+=("$1"); shift ;; esac; done
d=${a[-1]}; unset 'a[-1]'
for f in "${a[@]}"; do cp -- "$f" "$d/" && chmod "$m" "$d/${f##*/}" || exit 1; done
echo "install ${#a[@]} into ${d##*/}" >>"$MOCK/calls"
EOF
cat >"$B/apt-get" <<'EOF'
#!/bin/bash
# --print-uris answers from MOCK/uris-ROUND.txt (ROUND by the package asked for); an
# install with --no-download is recorded and succeeds
echo "apt-get $*" >>"$MOCK/calls"
case " $* " in
*" --print-uris "*) case " $* " in *" pinpkg"*) cat "$MOCK/uris-pin.txt" ;; *) cat "$MOCK/uris-open.txt" ;; esac ;;
esac
exit 0
EOF
chmod 0755 "$B"/*
ln -s "$CODE/guest/gpu-steps" "$B/gpu-steps"

# ---- the archive: two rounds' .debs, an index naming them, a loopback server
deb() { head -c "$2" /dev/urandom >"$T/www/pool/$1"; }
deb pinpkg_1.0_all.deb 3000
deb openpkg_1.0_amd64.deb 5000
deb openlib_1.0_amd64.deb 4000
deb badpkg_1.0_all.deb 1000
stanza() { # stanza FILE [SHA256]: a Packages stanza for www/pool/FILE
    local f=$T/www/pool/$1
    printf 'Package: %s\nVersion: 1.0\nFilename: pool/%s\nSize: %s\nMD5sum: %s\nSHA256: %s\n\n' \
        "${1%%_*}" "$1" "$(stat -c %s "$f")" "$(md5sum "$f" | cut -d' ' -f1)" "${2:-$(sha256sum "$f" | cut -d' ' -f1)}"
}
{
    stanza pinpkg_1.0_all.deb
    stanza openpkg_1.0_amd64.deb
    stanza openlib_1.0_amd64.deb
    stanza badpkg_1.0_all.deb "$(printf '%064d' 0)"
} >"$T/lists/mock_Packages"
python3 -I -u -m http.server --bind 127.0.0.1 --directory "$T/www" 0 >"$T/srv.log" 2>&1 &
SRV=$!
for _ in $(seq 50); do PORT=$(sed -n 's/.* port \([0-9]*\) .*/\1/p' "$T/srv.log"); [ -n "$PORT" ] && break; sleep 0.1; done
[ -n "${PORT:-}" ] || no "the loopback server did not start"
uri() { # uri FILE: apt's --print-uris line for www/pool/FILE, with apt's MD5Sum
    local f=$T/www/pool/$1
    printf "'http://127.0.0.1:%s/pool/%s' %s %s MD5Sum:%s\n" "$PORT" "$1" "$1" "$(stat -c %s "$f")" "$(md5sum "$f" | cut -d' ' -f1)"
}
uri pinpkg_1.0_all.deb >"$T/uris-pin.txt"
{ uri openpkg_1.0_amd64.deb; uri openlib_1.0_amd64.deb; } >"$T/uris-open.txt"
export DRIVER_PIN=pinpkg=1.0 DRIVER_OPEN="openpkg=1.0 openlib=1.0"
DEB_ORIGINS="http://127.0.0.1:$PORT/pool/"

echo "== uris_sha256 (Spec section 6, step 1)"
eval "$(sed -n '/^uris_sha256()/,/^}/p' "$CODE/guest/gpu-steps")"
declare -F uris_sha256 >/dev/null || no "no uris_sha256 in gpu-steps"
L=$T/lists/mock_Packages
U=$T/u; mkdir -p "$U"
sha() { sha256sum "$T/www/pool/$1" | cut -d' ' -f1; }
o=$(uri pinpkg_1.0_all.deb | uris_sha256 "$L"); rc=$?
check "a stanza found by exact Filename gives its SHA256" \
    test "$rc" = 0 -a "$(grep -c "SHA256:$(sha pinpkg_1.0_all.deb)\$" <<<"$o")" = 1
cp "$L" "$U/two_Packages"
o=$(uri pinpkg_1.0_all.deb | uris_sha256 "$L" "$U/two_Packages"); rc=$?
check "two stanzas that agree pass" test "$rc" = 0 -a "$(grep -c '2 stanzas agree' <<<"$o")" = 1
sed "s/^SHA256: $(sha pinpkg_1.0_all.deb)\$/SHA256: $(printf '%064d' 1)/" "$L" >"$U/dis_Packages"
o=$(uri pinpkg_1.0_all.deb | uris_sha256 "$L" "$U/dis_Packages"); rc=$?
check "disagreeing stanzas stop" test "$rc" = 1 -a "$(grep -c '^STOP: .*disagree' <<<"$o")" = 1
sed "s/^Size: 3000\$/Size: 3001/" "$L" >"$U/size_Packages"
o=$(uri pinpkg_1.0_all.deb | uris_sha256 "$U/size_Packages"); rc=$?
check "a size that is not apt's stops" test "$rc" = 1 -a "$(grep -c '^STOP: .*bytes' <<<"$o")" = 1
grep -v '^SHA256:' "$L" >"$U/nosha_Packages"
o=$(uri pinpkg_1.0_all.deb | uris_sha256 "$U/nosha_Packages"); rc=$?
check "a stanza with no SHA256 leaves apt's line" \
    test "$rc" = 0 -a "$(grep -c "^'.* MD5Sum:" <<<"$o")" = 1 -a "$(grep -c 'carries no SHA256' <<<"$o")" = 1
o=$(printf "'http://127.0.0.1:%s/pool/none_1.0_all.deb' none_1.0_all.deb 10 MD5Sum:%032d\n" "$PORT" 0 | uris_sha256 "$L"); rc=$?
check "a file no stanza names leaves apt's line" test "$rc" = 0 -a "$(grep -c 'no stanza names it' <<<"$o")" = 1
o=$(uri pinpkg_1.0_all.deb | sed "s/MD5Sum:.*/MD5Sum:$(printf '%032d' 7)/" | uris_sha256 "$L"); rc=$?
check "apt's own hash differing from the stanza's stops" test "$rc" = 1 -a "$(grep -c '^STOP: .*apt printed md5sum' <<<"$o")" = 1
sed "s|^Filename: pool/pinpkg_1.0_all.deb\$|Filename: other/pinpkg_1.0_all.deb|" "$L" >"$U/name_Packages"
o=$(uri pinpkg_1.0_all.deb | uris_sha256 "$U/name_Packages"); rc=$?
check "a stanza is found by Filename, never by package name" test "$rc" = 0 -a "$(grep -c 'no stanza names it' <<<"$o")" = 1

echo "== ci_judge (Spec section 3)"
nw="Failed to wait for network: Unexpected error while running command.
Command: ['systemctl', 'start', 'systemd-networkd-wait-online.service']
Exit code: 1"
judge() { # judge JSON: ci_judge's exit on that status
    printf '%s' "$1" >"$T/ci.json"
    ci_judge "$T/ci.json" >/dev/null
}
j() { python3 -I -c 'import json, sys; print(json.dumps(json.loads(sys.argv[1])))' "$1"; }
nwj=$(python3 -I -c 'import json, sys; print(json.dumps(sys.argv[1]))' "$nw")
check "exit 2 with network-wait alone passes" judge "$(j "{\"status\":\"done\",\"errors\":[],\"recoverable_errors\":{\"ERROR\":[$nwj]}}")"
check "done with no item passes" judge '{"status":"done","errors":[],"recoverable_errors":{}}'
check "a schema warning stops" refused judge "$(j "{\"status\":\"done\",\"errors\":[],\"recoverable_errors\":{\"ERROR\":[$nwj],\"WARNING\":[\"cloud-config failed schema validation!\"]}}")"
check "a status not done stops" refused judge '{"status":"running","errors":[]}'
check "an unreadable status stops" refused judge '{not json'

echo "== cloud-init's wait has a deadline (Spec section 3)"
cat >"$B/cloud-init" <<'EOF'
#!/bin/bash
# status --wait sleeps MOCK/ci-wait seconds; --long and --format json answer MOCK/ci-status,
# exit 0, as cloud-init does while it is still running
case "$*" in
"status --wait") sleep "$(cat "$MOCK/ci-wait")" ;;
"status --long") echo "status: $(cat "$MOCK/ci-status")" ;;
"status --format json") printf '{"status": "%s", "errors": []}\n' "$(cat "$MOCK/ci-status")" ;;
*) exit 9 ;;
esac
EOF
chmod 0755 "$B/cloud-init"
echo 3 >"$T/ci-wait"; echo running >"$T/ci-status"
( CI_WAIT=1 cloud_init_verdict 1 "$LOG" deadline ) >"$T/ci-deadline.out" 2>&1
check "a wait past CI_WAIT is recorded as exit 124 and stops, though status still answers 0" \
    test $? != 0 -a "$(grep -c '^the wait passed its deadline, 1 s$' "$LOG/cloud-init-deadline.txt")" = 1 \
         -a "$(tail -1 "$LOG/cloud-init-deadline.txt")" = "exit 124"
echo 0 >"$T/ci-wait"; echo done >"$T/ci-status"
( CI_WAIT=1 cloud_init_verdict 1 "$LOG" in-time ) >"$T/ci-in-time.out" 2>&1
check "a wait that ends in time, status done, passes" test $? = 0
check "a CI_WAIT that is not a number of seconds is refused" refused env CI_WAIT=1s bash -c \
    "source $(printf '%q' "$CODE/lib.sh"); gssh() { :; }; cloud_init_verdict 1 $(printf '%q' "$LOG") bad"
check "a CI_WAIT of 0, which would set no deadline, is refused" refused env CI_WAIT=0 bash -c \
    "source $(printf '%q' "$CODE/lib.sh"); gssh() { :; }; cloud_init_verdict 1 $(printf '%q' "$LOG") zero"

echo "== clone.sh netdev keeps every forward on the host's loopback (Spec section 4)"
nd() { netdev_ok "$1" 5022; }
check "the SSH forward alone passes" nd 'user,id=un0,hostfwd=tcp:127.0.0.1:5022-:22'
check "restrict=on with a guestfwd passes" \
    nd 'user,id=un0,ipv6=off,restrict=on,hostfwd=tcp:127.0.0.1:5022-:22,guestfwd=tcp:10.0.2.100:8080-tcp:127.0.0.1:18080'
check "a second forward on loopback passes" \
    nd 'user,id=un0,hostfwd=tcp:127.0.0.1:5022-:22,hostfwd=udp:127.0.0.1:5053-10.0.2.15:53'
check "the SSH forward bound to every address is refused" refused nd 'user,id=un0,hostfwd=tcp::5022-:22'
check "a second forward bound to every address is refused" \
    refused nd 'user,id=un0,hostfwd=tcp:127.0.0.1:5022-:22,hostfwd=tcp:0.0.0.0:8080-:80'
check "no SSH forward on PORT is refused" refused nd 'user,id=un0,hostfwd=tcp:127.0.0.1:5023-:22'
check "an argument not starting user,id=un0 is refused" refused nd 'user,id=un1,hostfwd=tcp:127.0.0.1:5022-:22'

echo "== ensure_key (Spec section 1)"
( KEY=$T/keys/sub/measure_ed25519; ensure_key ) >/dev/null 2>&1
check "ensure_key makes the key's directory, 0700, and the key in it" \
    test -f "$T/keys/sub/measure_ed25519" -a "$(stat -c %a "$T/keys/sub")" = 700

echo "== D1: two driver rounds back to back (Spec sections 5 and 6)"
guest_reset
( fetch_round 1 "$LOG" "$CACHE" pin ) >"$T/pin.out" 2>&1
check "the pin round passes" test $? = 0
check "after the pin round's install, no ~/debs stands" test ! -e "$G/debs"
( fetch_round 1 "$LOG" "$CACHE" open ) >"$T/open.out" 2>&1
check "the open round's put lands in a new ~/debs and its install passes" test $? = 0
check "both rounds' files reached apt's cache, and nothing else" \
    test "$(ls "$T/archives" | tr '\n' ' ')" = "openlib_1.0_amd64.deb openpkg_1.0_amd64.deb pinpkg_1.0_all.deb "
check "each round installed with no download, the open round without recommends" \
    test "$(grep -c -- '--no-download -y pinpkg=1.0$' "$T/calls")" = 1 -a \
         "$(grep -c -- '--no-download -y --no-install-recommends openpkg=1.0 openlib=1.0$' "$T/calls")" = 1
check "each round's checked files are listed with their SHA256" \
    test "$(grep -c " SHA256 $(sha openpkg_1.0_amd64.deb)\$" "$LOG/debs-open.txt")" = 1
guest_reset
( fetch_round 1 "$LOG" "$CACHE" pin ) >"$T/pin2.out" 2>&1
check "a second pin round takes the cached copy" test "$(grep -c 'the cached copy matches' "$T/pin2.out")" = 1
guest_reset
mkdir -p "$G/debs/nested"
cp "$CACHE/pinpkg_1.0_all.deb" "$G/debs/"
gssh 1 "$(guest_env DRIVER_PIN DRIVER_OPEN)gpu-steps driver-install pin" >"$T/di.out" 2>&1
check "driver-install refuses a ~/debs holding a directory" \
    test $? != 0 -a "$(grep -c 'holds more than .deb files' "$T/di.out")" = 1
guest_reset
mkdir -p "$G/debs"; cp "$CACHE/pinpkg_1.0_all.deb" "$G/debs/"
rm -rf -- "${T:?}/archives"; : >"$T/archives"      # apt's cache not a directory: the copy fails
gssh 1 "$(guest_env DRIVER_PIN DRIVER_OPEN)gpu-steps driver-install pin" >"$T/di2.out" 2>&1
check "a cache copy that fails removes ~/debs and says so, so the round can be put again" \
    test $? != 0 -a ! -e "$G/debs" -a "$(grep -c 'is removed, so the round can be put again' "$T/di2.out")" = 1 \
         -a "$(grep -c -- '--no-download' "$T/calls")" = 0
rm -f -- "${T:?}/archives"
guest_reset
mkdir -p "$T/round1" "$T/round2"
cp "$CACHE/pinpkg_1.0_all.deb" "$T/round1/"
cp "$CACHE/openpkg_1.0_amd64.deb" "$T/round2/"
( put 1 "$T/round1" debs ) >/dev/null 2>&1
rm -f "$G"/debs/*.deb            # the slice's old install: the .debs went, ~/debs stayed
( put 1 "$T/round2" debs ) >"$T/put2.out" 2>&1
check "under the old install, the second put is refused, and nothing nests" \
    test $? != 0 -a -z "$(find "$G/debs" -mindepth 1 -print -quit)"
cp -r "$T/round2" "$G/debs"      # what scp -r did into the ~/debs that stood
check "the old copy (scp -r, as cp -r) nests, as run 3 found" test -f "$G/debs/round2/openpkg_1.0_amd64.deb"

echo "== the fetch refuses what it cannot check (Spec section 6, step 2)"
guest_reset
uri badpkg_1.0_all.deb >"$T/uris-pin.txt"
( fetch_round 1 "$LOG" "$CACHE" pin ) >"$T/bad.out" 2>&1
check "a file whose SHA256 is not the index's stops the round" \
    test $? != 0 -a "$(grep -c 'hashes .* the index says' "$T/bad.out")" = 1
check "and leaves no file and no partial file in the cache" \
    test -z "$(find "$CACHE" -name 'badpkg*' -o -name '.badpkg*' | head -1)"
printf "'http://127.0.0.1:%s/pool/none_1.0_all.deb' none_1.0_all.deb 10 MD5Sum:%032d\n" "$PORT" 0 >"$T/uris-pin.txt"
( fetch_round 1 "$LOG" "$CACHE" pin ) >"$T/md5.out" 2>&1
check "a line left with apt's MD5Sum is refused" \
    test $? != 0 -a "$(grep -c 'a hash this script does not check: MD5Sum' "$T/md5.out")" = 1
: >"$T/uris-pin.txt"
( fetch_round 1 "$LOG" "$CACHE" pin ) >"$T/none.out" 2>&1
check "a round that names no file stops" test $? != 0 -a "$(grep -c 'named no .deb' "$T/none.out")" = 1
gets() { grep -c '"GET ' "$T/srv.log"; }
mkdir -p "$T/www/other"
cp "$T/www/pool/pinpkg_1.0_all.deb" "$T/www/other/"
n0=$(gets)
uri pinpkg_1.0_all.deb | sed 's|//127\.0\.0\.1:|//localhost:|' >"$T/uris-pin.txt"
( fetch_round 1 "$LOG" "$CACHE" pin ) >"$T/origin.out" 2>&1
check "a URL from an origin not in DEB_ORIGINS is refused, though its file and hash are good" \
    test $? != 0 -a "$(grep -c 'is not under DEB_ORIGINS' "$T/origin.out")" = 1
printf "'http://127.0.0.1:%s/pool/../other/pinpkg_1.0_all.deb' pinpkg_1.0_all.deb 3000 SHA256:%s\n" \
    "$PORT" "$(sha pinpkg_1.0_all.deb)" >"$T/uris-pin.txt"
( fetch_round 1 "$LOG" "$CACHE" pin ) >"$T/dotdot.out" 2>&1
check "a URL climbing out of an origin with .. is refused" \
    test $? != 0 -a "$(grep -c 'is not under DEB_ORIGINS' "$T/dotdot.out")" = 1
check "and neither was fetched" test "$(gets)" = "$n0"
uri pinpkg_1.0_all.deb >"$T/uris-pin.txt"
( DEB_ORIGINS="http://127.0.0.1:$PORT/pool"; fetch_round 1 "$LOG" "$CACHE" pin ) >"$T/badorigin.out" 2>&1
check "an origin not ending in / is refused" \
    test $? != 0 -a "$(grep -c 'not an http(s) URL prefix ending in /' "$T/badorigin.out")" = 1
guest_reset
mkdir -p "$T/www/pool/redir_1.0_all.deb"   # a directory: the server answers 301 to the same path with "/"
printf "'http://127.0.0.1:%s/pool/redir_1.0_all.deb' redir_1.0_all.deb 3000 SHA256:%s\n" \
    "$PORT" "$(sha pinpkg_1.0_all.deb)" >"$T/uris-pin.txt"
( fetch_round 1 "$LOG" "$CACHE" pin ) >"$T/redir.out" 2>&1
check "a URL answered with a redirect is refused, naming its status" \
    test $? != 0 -a "$(grep -c 'answered HTTP 301; redirects are not followed' "$T/redir.out")" = 1
check "and the redirect is never requested, and nothing of it stays in the cache" \
    test "$(grep -c '"GET /pool/redir_1.0_all.deb/ ' "$T/srv.log")" = 0 \
         -a -z "$(find "$CACHE" -name '*redir*' -print -quit)"
uri pinpkg_1.0_all.deb >"$T/uris-pin.txt"

echo "== apt's lists, uncompressed only (Spec section 6, step 1)"
uris() { gssh 1 "$(guest_env DRIVER_PIN DRIVER_OPEN)gpu-steps driver-uris pin"; }
mv "$T/lists/mock_Packages" "$T/mock_Packages.kept"
gzip -c "$T/mock_Packages.kept" >"$T/lists/mock_Packages.gz"
uris >"$T/lz-only.out" 2>&1
check "lists kept only compressed stop the step, named" \
    test $? != 0 -a "$(grep -c 'compressed Packages lists .*(mock_Packages.gz)' "$T/lz-only.out")" = 1
rm "$T/lists/mock_Packages.gz"
cp "$T/mock_Packages.kept" "$T/lists/mock_Packages"
printf 'x' >"$T/lists/other_Packages.lz4"
uris >"$T/lz-mixed.out" 2>&1
check "a compressed list among uncompressed ones stops the step, naming it" \
    test $? != 0 -a "$(grep -c 'compressed Packages lists .*(other_Packages.lz4)' "$T/lz-mixed.out")" = 1
rm "$T/lists/other_Packages.lz4" "$T/lists/mock_Packages"
uris >"$T/nolist.out" 2>&1
check "no list at all stops the step" test $? != 0 -a "$(grep -c 'holds no Packages list' "$T/nolist.out")" = 1
mv "$T/mock_Packages.kept" "$T/lists/mock_Packages"
uris >"$T/lists-ok.out" 2>&1
check "the uncompressed list alone passes" test $? = 0 -a "$(grep -c " SHA256:$(sha pinpkg_1.0_all.deb)\$" "$T/lists-ok.out")" = 1

echo "== a round's own files, in a cache two rounds share (Spec section 6, step 2)"
mkdir -p "$T/tmp"   # the rounds' TMPDIR, where each makes its stage
round() { ( export TMPDIR=$T/tmp; fetch_round 1 "$LOG" "$CACHE" pin ); }
guest_reset
rm -f "$CACHE/pinpkg_1.0_all.deb"
printf 'other' >"$CACHE/.pinpkg_1.0_all.deb.part"         # another round's download, by the old fixed name
printf 'other' >"$CACHE/.pinpkg_1.0_all.deb.Zq81xT.part"  # and by a name of its own
round >"$T/share.out" 2>&1
check "a round that fetches leaves another round's download in progress alone" \
    test $? = 0 -a "$(cat "$CACHE/.pinpkg_1.0_all.deb.part")" = other -a "$(cat "$CACHE/.pinpkg_1.0_all.deb.Zq81xT.part")" = other
check "and leaves no temporary file or stage of its own" \
    test "$(find "$CACHE" -name '.pinpkg*.part' | wc -l)" = 2 -a -z "$(ls -A "$T/tmp")"
rm -f "$CACHE"/.pinpkg_1.0_all.deb*.part
guest_reset
head -c 3000 /dev/zero >"$CACHE/pinpkg_1.0_all.deb"     # a cached copy of the right size, not the file
round >"$T/corrupt.out" 2>&1
check "a cached copy that does not match is fetched again, and the checked file replaces it" \
    test $? = 0 -a "$(grep -c 'the cached copy does not match; fetching it again' "$T/corrupt.out")" = 1 \
         -a "$(sha256sum <"$CACHE/pinpkg_1.0_all.deb" | cut -d' ' -f1)" = "$(sha pinpkg_1.0_all.deb)"
check "and what reached the guest's apt cache is the checked file" \
    test "$(sha256sum <"$T/archives/pinpkg_1.0_all.deb" | cut -d' ' -f1)" = "$(sha pinpkg_1.0_all.deb)"
guest_reset
mkdir -p "$G/debs"                                       # a ~/debs that stands: put refuses
round >"$T/putrefused.out" 2>&1
check "a refusal in put leaves no stage behind" test $? != 0 -a -z "$(ls -A "$T/tmp")"
guest_reset
rm -f "$CACHE/pinpkg_1.0_all.deb"
mkdir -p "$T/slow"
cat >"$T/slow/curl" <<EOF
#!/bin/bash
sleep 2
exec $(command -v curl) "\$@"
EOF
chmod 0755 "$T/slow/curl"
( export TMPDIR=$T/tmp PATH="$T/slow:$PATH"; fetch_round 1 "$LOG" "$CACHE" pin ) >"$T/term.out" 2>&1 &
pid=$!
for _ in $(seq 50); do [ -n "$(find "$CACHE" -name '.pinpkg*.part' -print -quit)" ] && break; sleep 0.1; done
kill -TERM "$pid"
wait "$pid"
rc=$?
check "a TERM during a fetch removes the round's temporary file and stage" \
    test "$rc" = 143 -a -z "$(find "$CACHE" -name '.pinpkg*.part' -print -quit)" -a -z "$(ls -A "$T/tmp")"

echo "== D2: links (Spec section 5)"
guest_reset
mkdir -p "$T/lib"
printf 'x' >"$T/lib/libx.so.0.1"
ln -s libx.so.0.1 "$T/lib/libx.so.0"
ln -s libx.so.0 "$T/lib/libx.so"
( put 1 "$T/lib" stage/lib ) >/dev/null 2>&1
check "put lands the source's links as links" \
    test -L "$G/stage/lib/libx.so" -a "$(readlink "$G/stage/lib/libx.so.0")" = libx.so.0.1 -a -f "$G/stage/lib/libx.so.0.1"
cp -rL "$T/lib" "$G/stage-old"   # what scp -r did: it follows links
check "the old copy (scp -r, as cp -rL) turns them into files, and the check tells them apart" \
    test ! -L "$G/stage-old/libx.so" -a -f "$G/stage-old/libx.so"
check "put refuses a destination that stands" refused put 1 "$T/lib" stage/lib
check "put refuses a path holding .." refused put 1 "$T/lib" a/../b

echo "== D3: the toolchain reading (Spec section 7)"
guest_reset
REPO=$G/wa
HD=$G/harness
mkdir -p "$REPO" "$HD" "$G/.cargo/bin"
: >"$HD/determinism_matrix.py"
READING='rustc 1.95.0-nightly (47611e160 2026-02-12)'
# The stub writes to the mock's own path: the preflight reads in an emptied environment,
# which has no MOCK. Under an inherited RUSTUP_TOOLCHAIN it reads that toolchain's line.
cat >"$G/.cargo/bin/rustc" <<EOF
#!/bin/bash
echo "rustc \$* in \$PWD, RUSTUP_AUTO_INSTALL=\${RUSTUP_AUTO_INSTALL-unset}" >>"$T/calls"
if [ -n "\${RUSTUP_TOOLCHAIN:-}" ]; then echo "rustc for \$RUSTUP_TOOLCHAIN"; else echo '$READING'; fi
EOF
cat >"$G/.cargo/bin/rustup" <<'EOF'
#!/bin/bash
echo "nightly-2026-02-13-x86_64-unknown-linux-gnu (overridden by '$PWD/rust-toolchain.toml')"
EOF
chmod 0755 "$G/.cargo/bin/rustc" "$G/.cargo/bin/rustup"
# pre READING: the preflight as clone.sh step runs it; XENV, where set, is more of the
# step's environment (as GUEST_ENV gives it)
pre() { gssh 1 "env ${XENV:-} TOOLCHAIN_READING=$(printf '%q' "$1") HARNESS_DIR=$(printf '%q' "$HD") gpu-steps harness-preflight $(printf '%q' "$REPO")"; }
gssh 1 'rustc --version' >/dev/null 2>&1
check "the inherited PATH alone answers 127, as the rehearsal read" test $? = 127
pre "$READING" >"$T/pre.out" 2>&1
check "the preflight puts ~/.cargo/bin first and reads the template's rustc" \
    test $? = 0 -a "$(grep -c "rustc: $G/.cargo/bin/rustc" "$T/pre.out")" = 1
check "it reads in the repository, with RUSTUP_AUTO_INSTALL=0" \
    test "$(grep -c "^rustc --version in $REPO, RUSTUP_AUTO_INSTALL=0\$" "$T/calls")" = 1
pre 'rustc 1.96.0-nightly (0000000 2026-03-01)' >"$T/pre2.out" 2>&1
check "a rustc reading another line is refused" test $? != 0 -a "$(grep -c 'not .rustc 1.96' "$T/pre2.out")" = 1
XENV=RUSTUP_TOOLCHAIN=nightly-other pre "$READING" >"$T/pre-tc.out" 2>&1
check "an inherited RUSTUP_TOOLCHAIN does not reach the readings, so the matrix's line passes" test $? = 0
XENV=RUSTUP_TOOLCHAIN=nightly-other pre 'rustc for nightly-other' >"$T/pre-tc2.out" 2>&1
check "a line only that inherited setting gives is refused, since the matrix runs without it" \
    test $? != 0 -a "$(grep -c "not 'rustc for nightly-other'" "$T/pre-tc2.out")" = 1
rm "$G/.cargo/bin/rustc"
pre "$READING" >"$T/pre3.out" 2>&1
check "no rustc in ~/.cargo/bin is refused" test $? != 0 -a "$(grep -c 'no rustc on PATH' "$T/pre3.out")" = 1

echo "== a guest step that fails stops the host script (Spec section 0)"
# host_fn SCRIPT FN: FN as the host script defines it. The copy-in lines (gscp, and the
# guest's sudo install of the steps) are stubbed; each run is a subshell under the
# scripts' own set -euo pipefail.
host_fn() { eval "$(sed -n "/^$2() {/,/^}/p" "$CODE/$1")"; declare -F "$2" >/dev/null || no "no $2 in $1"; }
copy_in_stubbed() {
    gscp() { :; }
    install_steps() { :; }
    eval "mock_gssh() $(declare -f gssh | tail -n +2)"
    gssh() { case $2 in 'sudo install '*) return 0 ;; esac; mock_gssh "$@"; }
}
cat >"$B/template-steps" <<'EOF'
#!/bin/bash
echo "template-steps $*" >>"$MOCK/calls"
[ ! -e "$MOCK/steps-fail" ] || { echo "the step failed"; exit 3; }
echo "the step passed"
EOF
chmod 0755 "$B/template-steps"
guest_reset
touch "$T/steps-fail"
( set -euo pipefail; host_fn build-template.sh step; copy_in_stubbed; step packages; echo "after the step" ) >"$T/bstep.out" 2>&1
check "build-template.sh step: a failed guest step exits non-zero, and nothing after it runs" \
    test $? = 1 -a "$(grep -c '^after the step' "$T/bstep.out")" = 0 \
         -a "$(grep -c 'REFUSED: step packages did not pass' "$T/bstep.out")" = 1
rm "$T/steps-fail"
( set -euo pipefail; host_fn build-template.sh step; copy_in_stubbed; step packages; echo "after the step" ) >"$T/bstep2.out" 2>&1
check "and a step that passes goes on" test $? = 0 -a "$(grep -c '^after the step' "$T/bstep2.out")" = 1
( set -euo pipefail; host_fn clone.sh step; copy_in_stubbed; step harness-preflight "$G"; echo "after the step" ) >"$T/cstep.out" 2>&1
check "clone.sh step: a failed guest step exits non-zero, and nothing after it runs" \
    test $? = 1 -a "$(grep -c '^after the step' "$T/cstep.out")" = 0 \
         -a "$(grep -c 'REFUSED: step harness-preflight did not pass' "$T/cstep.out")" = 1
( set -euo pipefail; host_fn clone.sh step; copy_in_stubbed; step driver-uris pin; echo "after the step" ) >"$T/cstep2.out" 2>&1
check "and a step that passes goes on" test $? = 0 -a "$(grep -c '^after the step' "$T/cstep2.out")" = 1
# seal: virsh_ and qemu-img recorded, the template's directory a stand-in
TD=$T/template
sealrun() {
    rm -rf -- "${TD:?}"; mkdir -p "$TD/hostkey"; : >"$TD/known_hosts"; : >"$TD/seed.iso"
    head -c 4096 /dev/zero >"$TD/template.qcow2"; chmod 0644 "$TD/template.qcow2"
    : >"$T/host-calls"; rm -f "$LOG/template-sha256.txt"
    ( set -euo pipefail; host_fn build-template.sh seal; copy_in_stubbed
      NAME=mock-template DIR=$TD DISK=$TD/template.qcow2
      virsh_() { echo "virsh_ $*" >>"$T/host-calls"; [ "$1" != domstate ] || echo "shut off"; }
      qemu-img() { echo "qemu-img $*" >>"$T/host-calls"; }
      seal; echo "after the seal" )
}
touch "$T/steps-fail"
sealrun >"$T/seal.out" 2>&1
check "a seal that fails stops before the undefine, the read-only disk and the checksum" \
    test $? = 1 -a ! -s "$T/host-calls" -a "$(stat -c %a "$TD/template.qcow2")" = 644 -a -e "$TD/hostkey" \
         -a ! -e "$LOG/template-sha256.txt" -a "$(grep -c 'REFUSED: the seal did not pass' "$T/seal.out")" = 1
rm "$T/steps-fail"
sealrun >"$T/seal2.out" 2>&1
check "a seal that passes goes on to all three" \
    test $? = 0 -a "$(grep -c '^virsh_ undefine mock-template --nvram$' "$T/host-calls")" = 1 \
         -a "$(stat -c %a "$TD/template.qcow2")" = 444 -a ! -e "$TD/hostkey" -a -s "$LOG/template-sha256.txt"
chmod 0644 "$TD/template.qcow2"

echo "== relaxed puts both settings back (template-steps and clone-steps)"
# relaxed as each guest file defines it, with sysctl, readlink, ln and sudo as functions
# over two files: R/userns, the sysctl's value, and R/sh, where /bin/sh points (empty: not
# a link). R/ln-fails makes the change to bash fail.
R=$T/relaxed
mkdir -p "$R"
relaxed_in() { # relaxed_in FILE COMMAND...: relaxed COMMAND in a subshell; prints the state after
    local f=$1; shift
    (
        # The subshell's own trap table, empty: until a subshell sets a trap, `trap -p`
        # still prints the parent's (the mock's cleanup).
        trap - EXIT INT HUP TERM PIPE
        eval "$(sed -n '/^relaxed() {/,/^}/p' "$CODE/guest/$f")"
        declare -F relaxed >/dev/null || exit 99
        sudo() { "$@"; }
        sysctl() { case $1 in -n) cat "$R/userns" ;; -q) printf '%s\n' "${3#*=}" >"$R/userns" ;; esac; }
        readlink() { [ "$1" = /bin/sh ] && cat "$R/sh"; }
        ln() { [ "$3" = /bin/sh ] || return 9; [ ! -e "$R/ln-fails" ] || [ "$2" != bash ] || return 1
               [ ! -e "$R/restore-fails" ] || [ "$2" = bash ] || return 1
               [ ! -e "$R/restore-silent" ] || [ "$2" = bash ] || return 0; printf '%s\n' "$2" >"$R/sh"; }
        during() { echo "$(cat "$R/userns") $(cat "$R/sh")" >"$R/during"; }
        quits() { exit "$1"; }
        unset_var() { echo "$RELAXED_MOCK_UNSET"; }
        writes() { sleep 0.3; printf 'after the client went away\n'; printf 'more\n'; }
        relaxed "$@"
        rc=$?
        [ -z "$(trap -p EXIT INT HUP TERM PIPE)" ] || { echo "a trap outlived relaxed"; exit 98; }
        exit "$rc"
    )
}
relaxed_state() { echo "$(cat "$R/userns") $(cat "$R/sh")"; }
for f in template-steps clone-steps; do
    rr() { echo 1 >"$R/userns"; echo dash >"$R/sh"; rm -f "$R/during" "$R/ln-fails" "$R/restore-fails" "$R/restore-silent"; }
    rr; relaxed_in "$f" during >/dev/null 2>&1
    check "$f: relaxed runs the command relaxed and restores both, its traps gone" \
        test $? = 0 -a "$(cat "$R/during")" = "0 bash" -a "$(relaxed_state)" = "1 dash"
    rr; relaxed_in "$f" false >/dev/null 2>&1
    check "$f: a command that fails is answered, both restored" test $? = 1 -a "$(relaxed_state)" = "1 dash"
    rr; touch "$R/ln-fails"; relaxed_in "$f" during >/dev/null 2>&1
    check "$f: a change to /bin/sh that fails restores the sysctl, and the command never runs" \
        test $? = 1 -a "$(relaxed_state)" = "1 dash" -a ! -e "$R/during"
    rr; relaxed_in "$f" bash -c 'kill -TERM $PPID; sleep 0.2' >/dev/null 2>&1
    check "$f: a TERM during the command restores both, exit 143" test $? = 143 -a "$(relaxed_state)" = "1 dash"
    rr; relaxed_in "$f" bash -c 'kill -HUP $PPID; sleep 0.2' >/dev/null 2>&1
    check "$f: a HUP during the command restores both, exit 129" test $? = 129 -a "$(relaxed_state)" = "1 dash"
    rr; touch "$R/restore-fails"; relaxed_in "$f" during >"$T/relaxed-nores.out" 2>&1
    check "$f: a restore that fails after a command that passed fails the step, and says so" \
        test $? = 1 -a "$(cat "$R/sh")" = bash -a "$(grep -c '^NOT RESTORED: ' "$T/relaxed-nores.out")" = 1
    rr; touch "$R/restore-silent"; relaxed_in "$f" during >"$T/relaxed-silent.out" 2>&1
    check "$f: a restore whose command answers 0 but does not read back fails the step" \
        test $? = 1 -a "$(grep -c '^NOT RESTORED: .*/bin/sh -> bash (was dash)' "$T/relaxed-silent.out")" = 1
    rr; relaxed_in "$f" quits 3 >/dev/null 2>&1
    check "$f: an exit inside the command restores both, its status kept" test $? = 3 -a "$(relaxed_state)" = "1 dash"
    rr; touch "$R/restore-fails"; relaxed_in "$f" quits 0 >/dev/null 2>&1
    check "$f: an exit 0 inside the command with a restore that fails exits 1" test $? = 1
    rr; ( relaxed_in "$f" unset_var ) >/dev/null 2>&1   # the fatal error unwinds the call's own redirection
    check "$f: an unset variable under set -u inside the command restores both" \
        test $? != 0 -a "$(relaxed_state)" = "1 dash"
    rr; ( set -u; relaxed_in "$f" writes 2>/dev/null | true; exit "${PIPESTATUS[0]}" )
    check "$f: a write to a closed pipe (the dropped SSH session) restores both, exit 141" \
        test $? = 141 -a "$(relaxed_state)" = "1 dash"
    rr; : >"$R/sh"; relaxed_in "$f" during >"$T/relaxed-nolink.out" 2>&1
    check "$f: a /bin/sh that is not a link is refused, nothing changed" \
        test $? = 1 -a "$(cat "$R/userns")" = 1 -a ! -e "$R/during" \
             -a "$(grep -c 'not a link; nothing relaxed' "$T/relaxed-nolink.out")" = 1
done

echo "== $pass checks passed"
