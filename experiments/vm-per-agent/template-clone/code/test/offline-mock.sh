#!/usr/bin/env bash
# The offline mock of template-clone-Spec.md section 8: the real put, fetch_round,
# driver-install, uris_sha256, ci_judge and harness preflight, with no virtual machine, no
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
         chmod python3 sha256sum stat; do
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
cat >"$G/.cargo/bin/rustc" <<EOF
#!/bin/bash
echo "rustc \$* in \$PWD, RUSTUP_AUTO_INSTALL=\${RUSTUP_AUTO_INSTALL-unset}" >>"\$MOCK/calls"
echo '$READING'
EOF
cat >"$G/.cargo/bin/rustup" <<'EOF'
#!/bin/bash
echo "nightly-2026-02-13-x86_64-unknown-linux-gnu (overridden by '$PWD/rust-toolchain.toml')"
EOF
chmod 0755 "$G/.cargo/bin/rustc" "$G/.cargo/bin/rustup"
pre() { gssh 1 "env TOOLCHAIN_READING=$(printf '%q' "$1") HARNESS_DIR=$(printf '%q' "$HD") gpu-steps harness-preflight $(printf '%q' "$REPO")"; }
gssh 1 'rustc --version' >/dev/null 2>&1
check "the inherited PATH alone answers 127, as the rehearsal read" test $? = 127
pre "$READING" >"$T/pre.out" 2>&1
check "the preflight puts ~/.cargo/bin first and reads the template's rustc" \
    test $? = 0 -a "$(grep -c "rustc: $G/.cargo/bin/rustc" "$T/pre.out")" = 1
check "it reads in the repository, with RUSTUP_AUTO_INSTALL=0" \
    test "$(grep -c "^rustc --version in $REPO, RUSTUP_AUTO_INSTALL=0\$" "$T/calls")" = 1
pre 'rustc 1.96.0-nightly (0000000 2026-03-01)' >"$T/pre2.out" 2>&1
check "a rustc reading another line is refused" test $? != 0 -a "$(grep -c 'not .rustc 1.96' "$T/pre2.out")" = 1
rm "$G/.cargo/bin/rustc"
pre "$READING" >"$T/pre3.out" 2>&1
check "no rustc in ~/.cargo/bin is refused" test $? != 0 -a "$(grep -c 'no rustc on PATH' "$T/pre3.out")" = 1

echo "== $pass checks passed"
