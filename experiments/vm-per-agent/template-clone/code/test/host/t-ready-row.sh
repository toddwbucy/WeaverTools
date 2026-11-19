#!/usr/bin/env bash
# The proof's ready row and second boot (weaver-appliance-build-Spec section 2, step 13; the
# operator's rulings of 2026-11-19), with no guest. Real: the console judge, the requests'
# judge with the host's openssl, the survey judge with the format's own reader and WeaverWeb's
# worked example, the two boots' comparison, the guest read run against stand-in commands,
# and proof_ready and proof_second_boot from build-template.sh against stand-in SSH, a
# stand-in hypervisor and a stand-in console. Pure logic: no root, no guest.
source "$(dirname "$0")/lib-test.sh"
need python3 openssl
BT=$CODE/build-template.sh
export VM_ROOT=$T/vmroot; mkdir -p "$VM_ROOT"; source "$CODE/lib.sh"
eval "$(fn "$BT" proof_ready proof_second_boot)"
fails() { grep -c '^PROOF-FAIL' <<<"$1" || true; }
has() { grep -qxF -- "$2" <<<"$1"; }
P='\r\n\r\nArch Linux 7.3.4-arch1-2 (ttyS0)\r\n\r\nweaver-appliance login: '
MADE='weaver-appliance: ready (gate-con made, admin-con made)'
FOUND='weaver-appliance: ready (gate-con found, admin-con found)'

# ---- The console's ready line.
con() { printf '%b' "$1" >"$T/con.log"; ready_console_judge "$T/con.log" "$2"; }
o=$(con "[  OK  ] Started Network Time Synchronization.\r\n$MADE\r\n$P" made)
check "a line naming both made, before the prompt: read, and no finding" \
    eval '[ "$(fails "$o")" = 0 ] && has "$o" "on the console: $MADE" && has "$o" "the line comes before the login prompt"'
o=$(con "$MADE\r\n$P" found)
check "the same line, where the second boot wants found: a finding naming the line wanted" \
    eval '[ "$(fails "$o")" = 1 ] && has "$o" "PROOF-FAIL: the console'"'"'s weaver-appliance line is not '"'"'$FOUND'"'"'"'
o=$(con "$FOUND\r\n$P" found)
check "a line naming both found, at the second boot: no finding" test "$(fails "$o")" = 0
o=$(con "$P\r\n$MADE\r\n" made)
check "a line after the login prompt is a finding" \
    eval '[ "$(fails "$o")" = 1 ] && has "$o" "PROOF-FAIL: the weaver-appliance line comes after the login prompt"'
o=$(con "[  OK  ] Started x.\r\n$P" made)
check "no line on the console is a finding" \
    eval '[ "$(fails "$o")" = 1 ] && has "$o" "PROOF-FAIL: no weaver-appliance line on the console"'
R='weaver-appliance: refused: gate-con: its config stands without its request; nothing is remade or removed; to clear it, remove /etc/weaver-web/pairing/gate/gate-con.toml as root and boot again, or make the virtual machine again from the template (admin-con made)'
o=$(con "$R\r\n$P" made)
check "a refusal is shown whole, with what clears it, and is a finding" \
    eval '[ "$(fails "$o")" = 1 ] && has "$o" "on the console: $R"'
o=$(con "$MADE\r\n$MADE\r\n$P" made)
check "two lines are a finding" eval 'has "$o" "PROOF-FAIL: 2 weaver-appliance lines on the console, not one"'
o=$(con "\r\033[K[  ***  ] A start job is running for x (3s / 2min)\r$MADE\r\n\033[0m$P" made)
check "a line written over systemd's passing status, with its escapes, is read" test "$(fails "$o")" = 0
printf '\033]3008;start=8d1e;type=service\033\\%s\033]3008;end=8d1e\033\\\r\n%b' "$MADE" "$P" >"$T/con.log"
o=$(ready_console_judge "$T/con.log" made)
check "a line between systemd's OSC 3008 context marks is read" eval '[ "$(fails "$o")" = 0 ] && has "$o" "on the console: $MADE"'
o=$(con "$MADE\r\n" made)
check "a console with no login prompt: the line's place is a finding" \
    eval 'has "$o" "PROOF-FAIL: no login prompt on the console, so the line'"'"'s place is not known"'
o=$(ready_console_judge "$T/absent.log" made)
check "a console log that cannot be read is a finding" eval '[ "$(fails "$o")" = 1 ] && grep -q "^PROOF-FAIL: the console log could not be read" <<<"$o"'

# ---- The requests, judged on the host. Two real requests, made here by openssl.
mkcsr() { openssl req -new -newkey ec -pkeyopt ec_paramgen_curve:P-256 -nodes -subj "/CN=$1" \
    -keyout "$T/$1.key" -out "$T/$1.csr" >/dev/null 2>&1; }
if ! mkcsr gate-con || ! mkcsr admin-con; then no "openssl could not make the stand-in requests"; finish; exit; fi
reqline() { python3 -I -c 'import json, sys; sys.stdout.write(json.dumps({"event": "request", "request": open(sys.argv[1]).read()}, separators=(",", ":")) + "\n")' "$1"; }
reqs() { mkdir -p "$T/rq"; rm -f -- "$T/rq/gate.req" "$T/rq/admin.req"; reqline "$T/gate-con.csr" >"$T/rq/gate.req"; reqline "$T/admin-con.csr" >"$T/rq/admin.req"; }
reqs; o=$(ready_requests_judge "$T/rq")
check "two requests of the form, each verified by openssl: no finding" \
    eval '[ "$(fails "$o")" = 0 ] && [ "$(grep -cE "^(gate|admin): one line, [0-9]+ bytes, of the form pair reads; openssl: .*verify OK" <<<"$o")" = 2 ]'
# The signature's last byte turned: the request still parses, and its signature fails.
python3 -I - "$T/gate-con.csr" "$T/bad.csr" <<'PY'
import base64, sys
t = open(sys.argv[1]).read().split("\n")
der = bytearray(base64.b64decode("".join(t[1:-2])))
der[-1] ^= 1
b = base64.b64encode(bytes(der)).decode()
open(sys.argv[2], "w").write(t[0] + "\n" + "\n".join(b[i:i + 64] for i in range(0, len(b), 64)) + "\n" + t[-2] + "\n")
PY
reqline "$T/bad.csr" >"$T/rq/gate.req"; o=$(ready_requests_judge "$T/rq")
check "a request whose signature does not hold is a finding naming its plane" \
    eval '[ "$(fails "$o")" = 1 ] && grep -q "^PROOF-FAIL: the gate request.s PEM does not verify as a certificate request" <<<"$o"'
# openssl's verdict, read as well as its status: a stand-in openssl first on PATH for one
# judge each. One prints the failure and exits 0, as OpenSSL 3.0 does (its review of
# 2026-11-19); its mirror prints "verify OK" and exits 1; one prints both and exits 0; one
# prints 1.1.1's short "verify OK" and exits 0, which passes.
stand_ssl() { local n=$1 st=$2 l; shift 2; mkdir -p "$T/ssl-$n"
    { echo '#!/usr/bin/env bash'; echo 'if [ "$1" = version ]; then echo "OpenSSL stand-in '"$n"'"; exit 0; fi'
      echo 'cat >/dev/null'; for l; do printf 'echo %q >&2\n' "$l"; done; echo "exit $st"; } >"$T/ssl-$n/openssl"
    chmod 0755 "$T/ssl-$n/openssl"; }
stand_ssl fail0 0 "Certificate request self-signature verify failure"
stand_ssl ok1 1 "Certificate request self-signature verify OK"
stand_ssl both0 0 "Certificate request self-signature verify OK" "Certificate request self-signature verify failure"
stand_ssl short0 0 "verify OK"
reqs; o=$(PATH="$T/ssl-fail0:$PATH" ready_requests_judge "$T/rq")
check "openssl printing the failure and exiting 0, as 3.0 does: a finding for each request, with its status" \
    eval '[ "$(fails "$o")" = 2 ] && grep -qx "PROOF-FAIL: the gate request.s PEM does not verify as a certificate request (openssl exit 0): Certificate request self-signature verify failure" <<<"$o"'
o=$(PATH="$T/ssl-ok1:$PATH" ready_requests_judge "$T/rq")
check "openssl printing verify OK and exiting 1: a finding for each request" \
    eval '[ "$(fails "$o")" = 2 ] && grep -qx "PROOF-FAIL: the admin request.s PEM does not verify as a certificate request (openssl exit 1): Certificate request self-signature verify OK" <<<"$o"'
o=$(PATH="$T/ssl-both0:$PATH" ready_requests_judge "$T/rq")
check "openssl printing both verdicts and exiting 0: a finding for each request" test "$(fails "$o")" = 2
o=$(PATH="$T/ssl-short0:$PATH" ready_requests_judge "$T/rq")
check "openssl 1.1.1's short verify OK with exit 0 passes, its version printed once, first" \
    eval '[ "$(fails "$o")" = 0 ] && [ "$(grep -cE "^(gate|admin): one line" <<<"$o")" = 2 ] && [ "$(sed -n 1p <<<"$o")" = "the host'"'"'s openssl: OpenSSL stand-in short0" ] && [ "$(grep -c "^the host.s openssl: " <<<"$o")" = 1 ]'
reqs; o=$(ready_requests_judge "$T/rq")
check "with the host's own openssl, its version is the judge's first line" eval 'grep -q "^the host.s openssl: OpenSSL [0-9]" <<<"$(sed -n 1p <<<"$o")"'
reqs; printf 'more\n' >>"$T/rq/admin.req"; o=$(ready_requests_judge "$T/rq")
check "a request of two lines is a finding" eval 'has "$o" "PROOF-FAIL: the admin request is not one line ending in its newline"'
reqs; printf '%s' "$(cat "$T/rq/admin.req")" >"$T/rq/admin.req"; o=$(ready_requests_judge "$T/rq")
check "a request with no newline at its end is a finding" eval 'has "$o" "PROOF-FAIL: the admin request is not one line ending in its newline"'
reqs; python3 -I -c 'import sys; sys.stdout.write("{\"event\":\"request\",\"request\":\"" + "x" * 16400 + "\"}\n")' >"$T/rq/gate.req"; o=$(ready_requests_judge "$T/rq")
check "a request over 16384 bytes is a finding" eval 'grep -q "^PROOF-FAIL: the gate request is 16433 bytes, over the 16384 pair reads$" <<<"$o"'
reqs; python3 -I -c 'import json, sys; sys.stdout.write(json.dumps({"event": "request", "request": open(sys.argv[1]).read(), "key": "x"}) + "\n")' "$T/gate-con.csr" >"$T/rq/gate.req"; o=$(ready_requests_judge "$T/rq")
check "a request with a member beyond the two is a finding" eval 'grep -q "^PROOF-FAIL: the gate request is not of the form" <<<"$o"'
reqs; printf 'request: x\n' >"$T/rq/gate.req"; o=$(ready_requests_judge "$T/rq")
check "a request that is not JSON is a finding" eval 'grep -q "^PROOF-FAIL: the gate request is not a line of ASCII JSON" <<<"$o"'
reqs; printf '{"event":"request","request":"\xc3\xa9"}\n' >"$T/rq/gate.req"; o=$(ready_requests_judge "$T/rq")
check "a request not of ASCII is a finding" eval 'grep -q "^PROOF-FAIL: the gate request is not a line of ASCII JSON" <<<"$o"'
reqs; rm -f -- "$T/rq/admin.req"; o=$(ready_requests_judge "$T/rq")
check "a request not read off the guest is a finding, the other still judged" \
    eval 'grep -q "^PROOF-FAIL: the admin request could not be read off the guest" <<<"$o" && grep -q "^gate: one line" <<<"$o"'
reqs; cp -- "$T/rq/gate.req" "$T/rq/admin.req"; o=$(ready_requests_judge "$T/rq")
check "the same request for both connectors is a finding" eval 'has "$o" "PROOF-FAIL: the two connectors'"'"' requests are the same request"'

# ---- The survey, as nobody: the format's own reader, against WeaverWeb's worked example.
EX=$CODE/guest/ready/test/fixtures/weaver-survey-1-worked-example.json
o=$(ready_survey_judge "$EX" 0)
check "the worked example, exit 0: accepted by the reader, no finding" \
    eval '[ "$(fails "$o")" = 0 ] && grep -q "^the survey: exit 0, 1643 bytes, one object the reader accepts" <<<"$o"'
o=$(ready_survey_judge "$EX" 1)
check "an exit other than 0 is a finding" eval '[ "$(fails "$o")" = 1 ] && has "$o" "PROOF-FAIL: the survey exited 1 as nobody, not 0"'
printf '{"format": "weaver-survey/1"} trailing\n' >"$T/survey-bad.json"; o=$(ready_survey_judge "$T/survey-bad.json" 0)
check "an output the reader refuses is a finding naming its reason" eval 'grep -q "^PROOF-FAIL: the reader refuses the survey.s output: " <<<"$o"'
o=$(ready_survey_judge "$T/absent.json" 0)
check "an output that cannot be read is a finding" eval 'grep -q "^PROOF-FAIL: the survey.s output could not be read" <<<"$o"'

# ---- The two boots' attributes and digests.
S1=1111111111111111111111111111111111111111111111111111111111111111
S2=2222222222222222222222222222222222222222222222222222222222222222
attrs() { printf 'attr /etc/weaver-web/pairing/gate/gate-con.toml 1201 412 weaver-gatecon:weaver-gatecon 600 %s\n' "${1:-1763540000}"
    printf 'attr /etc/weaver-web/pairing/admin/admin-con.toml 1202 415 weaver-admincon:weaver-admincon 600 1763540001\n'
    printf 'sum /etc/weaver-web/pairing/exchange/gate.req %s\nsum /etc/weaver-web/pairing/exchange/admin.req %s\n' "${2:-$S1}" "$S2"; }
attrs >"$T/a1"; attrs >"$T/a2"; o=$(ready_attrs_compare "$T/a1" "$T/a2")
check "the same four lines: four unchanged, no finding" eval '[ "$(fails "$o")" = 0 ] && [ "$(grep -c "^unchanged: " <<<"$o")" = 4 ]'
attrs 1763549999 >"$T/a2"; o=$(ready_attrs_compare "$T/a1" "$T/a2")
check "a config modified again is a finding: remade" \
    eval '[ "$(fails "$o")" = 1 ] && has "$o" "PROOF-FAIL: remade: the config'"'"'s attributes at /etc/weaver-web/pairing/gate/gate-con.toml changed between the boots"'
attrs "" "$S2" >"$T/a2"; o=$(ready_attrs_compare "$T/a1" "$T/a2")
check "a request of another digest is a finding: remade" \
    eval 'has "$o" "PROOF-FAIL: remade: the request'"'"'s digest at /etc/weaver-web/pairing/exchange/gate.req changed between the boots"'
attrs | sed -n 1,3p >"$T/a2"; o=$(ready_attrs_compare "$T/a1" "$T/a2")
check "a line the second boot did not read is a finding" \
    eval 'has "$o" "PROOF-FAIL: the second boot did not read the request'"'"'s digest at /etc/weaver-web/pairing/exchange/admin.req"'
attrs | sed -n 1,3p >"$T/a1"; attrs >"$T/a2"; o=$(ready_attrs_compare "$T/a1" "$T/a2")
check "a first boot that read three of the four lines is a finding" eval 'has "$o" "PROOF-FAIL: the first boot read 3 of the four attribute and digest lines"'
o=$(ready_attrs_compare "$T/absent1" "$T/absent2")
check "nothing read at either boot is a finding" eval 'has "$o" "PROOF-FAIL: the first boot read 0 of the four attribute and digest lines"'

# ---- The guest read, run against stand-in commands. A stand-in guest as the request unit
# leaves it: the accounts in the box's shape, the six folders, the configs and requests.
cat >"$T/standins.sh" <<'SH'
getent() {
    [ $# -ge 2 ] || { cat "$T/$1"; return 0; }
    awk -F: -v k="$2" '$1 == k || $3 == k { print; f = 1 } END { exit !f }' "$T/$1" || return 2
}
id() { [ "$1" = -Gn ] || return 9; awk -v n="$3" '$1 == n { $1 = ""; sub(/^ /, ""); print; f = 1 } END { exit !f }' "$T/idgroups"; }
systemctl() { [ "$1" = show ] && cat "$T/unit-show"; }
journalctl() { cat "$T/journal"; }
stat() { local f=$2 p=${!#}; awk -F'|' -v k="$f|$p" '$1 "|" $2 == k { print $3; x = 1 } END { exit !x }' "$T/stat"; }
find() { cat "$T/find"; }
findmnt() { cat "$T/mounts"; }
sha256sum() { local p=${!#}; printf '%s  %s\n' "$(awk -v k="$p" '$1 == k { print $2 }' "$T/sums")" "$p"; }
SH
guest() { printf 'T=%q\n' "$T"; cat "$T/standins.sh"; ready_guest_read | sed "s|/etc/login.defs|$T/login.defs|"; }
laid() {
    printf 'UID_MIN 1000\n' >"$T/login.defs"; : >"$T/passwd"; : >"$T/group"; : >"$T/idgroups"
    local a u=970 d path owner mode
    for a in $READY_ACCOUNTS; do
        printf '%s:x:%s:%s::%s:/usr/sbin/nologin\n' "$a" "$u" "$u" "$T/nohome/$a" >>"$T/passwd"
        printf '%s:x:%s:\n' "$a" "$u" >>"$T/group"; printf '%s %s\n' "$a" "$a" >>"$T/idgroups"; u=$((u + 1))
    done
    printf 'Result=success\nExecMainStatus=0\nExecMainExitTimestampMonotonic=9100000\n' >"$T/unit-show"
    { printf '{"MESSAGE": "Watching system buttons on /dev/input/event0", "__MONOTONIC_TIMESTAMP": "8000000"}\n'
      printf '{"MESSAGE": "New session 1 of user root.", "__MONOTONIC_TIMESTAMP": "%s"}\n' "${LOGIN_AT:-61000000}"
      printf '{"MESSAGE": "New session 2 of user root.", "__MONOTONIC_TIMESTAMP": "90000000"}\n'; } >"$T/journal"
    : >"$T/stat"; : >"$T/find"; printf '/\n/proc\n/sys\n/run\n/efi\n' >"$T/mounts"; : >"$T/sums"
    printf '%%d|/|66\n' >>"$T/stat"
    for d in $READY_DIRS; do
        IFS=: read -r path owner mode <<<"$d"
        printf '%%F %%U:%%G %%a %%d|%s|directory %s:%s %s 66\n' "$path" "$owner" "$owner" "$mode" >>"$T/stat"; echo "$path" >>"$T/find"
    done
    for c in gate-con:weaver-gatecon:gate:1201 admin-con:weaver-admincon:admin:1202; do
        IFS=: read -r con acct plane ino <<<"$c"
        printf '%%F %%U:%%G %%a|/etc/weaver-web/pairing/%s/%s.toml|regular file %s:%s 600\n' "$plane" "$con" "$acct" "$acct" >>"$T/stat"
        printf '%%i %%s %%U:%%G %%a %%Y|/etc/weaver-web/pairing/%s/%s.toml|%s 412 %s:%s 600 1763540000\n' "$plane" "$con" "$ino" "$acct" "$acct" >>"$T/stat"
        printf '%%F %%U:%%G %%a|/etc/weaver-web/pairing/exchange/%s.req|regular file root:root 644\n' "$plane" >>"$T/stat"
        printf '/etc/weaver-web/pairing/%s/%s.toml\n/etc/weaver-web/pairing/exchange/%s.req\n' "$plane" "$con" "$plane" >>"$T/find"
        printf '/etc/weaver-web/pairing/exchange/%s.req %s\n' "$plane" "$S1" >>"$T/sums"
    done
}
gr() { guest >"$T/guest.sh"; bash "$T/guest.sh" 2>&1; }
laid; o=$(gr)
check "the guest as the unit leaves it: no finding" eval '[ "$(fails "$o")" = 0 ]' || printf '%s\n' "$o" | grep '^PROOF-FAIL' | sed 's/^/    /'
check "it reads the unit's run, its exit before the first session, the accounts and the six folders" \
    eval 'grep -q "^the request unit: Result=success ExecMainStatus=0" <<<"$o" && has "$o" "the unit exited at 9100000 us; the first login session began at 61000000 us (this boot'"'"'s monotonic clock, by the journal)" && [ "$(grep -c ": in the box.s shape$" <<<"$o")" = 3 ] && [ "$(grep -c "^/etc/weaver-web[a-z/]*: directory " <<<"$o")" = 6 ]'
check "and prints the configs' attributes and the requests' digests for the second boot, four lines" \
    eval '[ "$(grep -cE "^(attr|sum) /etc/weaver-web/" <<<"$o")" = 4 ] && has "$o" "attr /etc/weaver-web/pairing/gate/gate-con.toml 1201 412 weaver-gatecon:weaver-gatecon 600 1763540000" && has "$o" "sum /etc/weaver-web/pairing/exchange/admin.req $S1"'
laid; printf 'Result=exit-code\nExecMainStatus=1\nExecMainExitTimestampMonotonic=9100000\n' >"$T/unit-show"; o=$(gr)
check "the unit's run not a success is a finding" eval 'has "$o" "PROOF-FAIL: the request unit'"'"'s run was not a success"'
LOGIN_AT=5000000 laid; o=$(gr)
check "the unit's exit after the first session began is a finding" \
    eval '[ "$(fails "$o")" = 1 ] && has "$o" "PROOF-FAIL: the request unit'"'"'s exit is not before the first login by the journal"'
laid; grep -v 'New session' "$T/journal" >"$T/j2"; mv -- "$T/j2" "$T/journal"; o=$(gr)
check "no session in the journal: the order unread is a finding" \
    eval 'grep -q "the first login session began at unread us" <<<"$o" && has "$o" "PROOF-FAIL: the request unit'"'"'s exit is not before the first login by the journal"'
laid; sed -i 's|^weaver-pair:x:972:972::\(.*\):/usr/sbin/nologin$|weaver-pair:x:972:972::\1:/bin/bash|' "$T/passwd"; o=$(gr)
check "an account with a login shell is a finding naming it" eval 'has "$o" "PROOF-FAIL: weaver-pair has the shell '"'"'/bin/bash'"'"', not nologin. "'
laid; sed -i 's/^\(%F %U:%G %a %d|\/etc\/weaver-web\/pairing\/gate|directory weaver-gatecon:weaver-gatecon\) 700 66$/\1 755 66/' "$T/stat"; o=$(gr)
check "a connector's folder of mode 0755 is a finding" \
    eval '[ "$(fails "$o")" = 1 ] && grep -q "^PROOF-FAIL: /etc/weaver-web/pairing/gate is .directory weaver-gatecon:weaver-gatecon 755 66., not a directory weaver-gatecon:weaver-gatecon 700" <<<"$o"'
laid; sed -i 's/^\(%F %U:%G %a %d|\/etc\/weaver-web\/connectors|directory root:root 755\) 66$/\1 67/' "$T/stat"; o=$(gr)
check "a folder on another filesystem is a finding" eval 'grep -q "^PROOF-FAIL: /etc/weaver-web/connectors is .directory root:root 755 67., not .* on the root filesystem" <<<"$o"'
laid; sed -i 's/^\(%F %U:%G %a|\/etc\/weaver-web\/pairing\/admin\/admin-con.toml|regular file weaver-admincon:weaver-admincon\) 600$/\1 644/' "$T/stat"; o=$(gr)
check "a config readable by others is a finding" \
    eval '[ "$(fails "$o")" = 1 ] && grep -q "^PROOF-FAIL: /etc/weaver-web/pairing/admin/admin-con.toml is .regular file weaver-admincon:weaver-admincon 644., not a regular file weaver-admincon:weaver-admincon 600$" <<<"$o"'
laid; sed -i 's/^\(%F %U:%G %a|\/etc\/weaver-web\/pairing\/exchange\/gate.req|regular file\) root:root 644$/\1 weaver-pair:weaver-pair 644/' "$T/stat"; o=$(gr)
check "a request that is not root's is a finding" eval 'grep -q "^PROOF-FAIL: /etc/weaver-web/pairing/exchange/gate.req is .regular file weaver-pair:weaver-pair 644., not a regular file root:root 644$" <<<"$o"'
laid; sed -i '/|\/etc\/weaver-web\/pairing\/gate\/gate-con.toml|/d' "$T/stat"; sed -i '/gate-con.toml$/d' "$T/find"; o=$(gr)
check "a config absent is a finding, and so is the folder's listing" \
    eval 'has "$o" "PROOF-FAIL: /etc/weaver-web/pairing/gate/gate-con.toml is '"'"'absent'"'"', not a regular file weaver-gatecon:weaver-gatecon 600" && grep -q "^PROOF-FAIL: /etc/weaver-web holds other than .*: /etc/weaver-web/pairing/gate/gate-con.toml $" <<<"$o"'
laid; echo /etc/weaver-web/pairing/gate/gate-con.toml.bak >>"$T/find"; o=$(gr)
check "a file beyond the configs and requests is a finding naming it" \
    eval '[ "$(fails "$o")" = 1 ] && grep -q "^PROOF-FAIL: /etc/weaver-web holds other than the six folders, the two configs and the two requests: /etc/weaver-web/pairing/gate/gate-con.toml.bak $" <<<"$o"'
laid; echo /etc/weaver-web/connectors >>"$T/mounts"; o=$(gr)
check "a filesystem mounted under /etc/weaver-web is a finding" eval 'has "$o" "PROOF-FAIL: mounted at or under /etc/weaver-web: /etc/weaver-web/connectors "'
laid; echo /etc/weaver-webx >>"$T/mounts"; o=$(gr)
check "a mount beside it with a longer name is not" test "$(fails "$o")" = 0
check "the guest read carries template-steps' own shape_check, not a copy" \
    eval 'diff <(sed -n "/^shape_check() {/,/^}/p" "$CODE/guest/template-steps") <(sed -n "/^shape_check() {/,/^}/p" <(ready_guest_read)) >/dev/null'
check "the guest read never opens a config: no read of a .toml but stat's" \
    eval '! grep -E "\\b(cat|head|tail|sha256sum|grep|cp|scp|less|od|xxd)\\b[^|;]*\.toml" <(ready_guest_read)'

# ---- proof_ready, against stand-in SSH. The guest's answers are read from T: the read as
# root (the stand-in guest above, run here), each request, and the survey's output.
OUT=$T/out; PD=$T/pd; mkdir -p "$OUT" "$PD/con"
gssh() { local port=$1; shift; printf '%s\n' "$port $*" >>"$T/gssh.log"; printf '%s' "$*" >"$T/gssh-last"; echo "$port" >>"$T/gssh-ports"
    case $* in
        *"shape_check() {"*) bash "$T/guest.sh" ;;
        "cat /etc/weaver-web/pairing/exchange/gate.req") [ ! -e "$T/no-gate" ] || { echo "ssh: closed" >&2; return 255; }; cat "$T/rq/gate.req" ;;
        "cat /etc/weaver-web/pairing/exchange/admin.req") cat "$T/rq/admin.req" ;;
        *"--stage pairing") cat "$T/survey-out"; [ ! -e "$T/survey-err" ] || { echo "weaver-survey: no drives" >&2; return 3; } ;;
        *"U=weaver-appliance-requests.service"*) cat "$T/second-read" ;;
        *) echo "unexpected: $*"; return 9 ;;
    esac; }
pr() { rm -f -- "$T/gssh.log" "$T/gssh-ports" "$OUT/ready-attributes.txt"; laid; reqs; guest >"$T/guest.sh"; cp -- "$EX" "$T/survey-out"
    printf '%b' "[  OK  ] Started x.\r\n$MADE\r\n$P" >"$PD/con/console.log"; }
pr; o=$(proof_ready "$OUT" "$PD" 2330)
check "proof_ready on a guest as laid: no finding" eval '[ "$(fails "$o")" = 0 ]' || printf '%s\n' "$o" | grep '^PROOF-FAIL' | sed 's/^/    /'
check "it reads the console, the guest, both requests and the survey, in that order" \
    eval 'a=$(grep -n "^on the console: " <<<"$o" | cut -d: -f1); b=$(grep -n "^the request unit: " <<<"$o" | cut -d: -f1); c=$(grep -n "^gate: one line" <<<"$o" | cut -d: -f1); d=$(grep -n "^the survey: exit 0" <<<"$o" | cut -d: -f1); [ -n "$a" ] && [ "$a" -lt "$b" ] && [ "$b" -lt "$c" ] && [ "$c" -lt "$d" ]'
check "the requests are kept in the proof's log, as read off the guest" \
    eval 'cmp -s "$OUT/ready-requests/gate.req" "$T/rq/gate.req" && cmp -s "$OUT/ready-requests/admin.req" "$T/rq/admin.req"'
check "the survey's output is kept, and the four attribute lines for the second boot" \
    eval 'cmp -s "$OUT/ready-survey.json" "$EX" && [ "$(wc -l <"$OUT/ready-attributes.txt")" = 4 ]'
check "every read is on the proof's port, the survey as nobody from /" \
    eval '[ "$(sort -u "$T/gssh-ports")" = 2330 ] && grep -qx "2330 cd / && sudo -n -u nobody /opt/weaver-appliance/bin/weaver-survey --stage pairing" "$T/gssh.log"'
pr; touch "$T/no-gate"; o=$(proof_ready "$OUT" "$PD" 2330); rm -f -- "$T/no-gate"
check "a request that cannot be read off the guest is a finding with its cause" \
    eval 'has "$o" "PROOF-FAIL: the gate request could not be read off the guest (exit 255): ssh: closed"'
pr; touch "$T/survey-err"; o=$(proof_ready "$OUT" "$PD" 2330); rm -f -- "$T/survey-err"
check "a survey exiting 3 is a finding, its standard error shown" \
    eval 'has "$o" "PROOF-FAIL: the survey exited 3 as nobody, not 0" && has "$o" "  its standard error: weaver-survey: no drives"'
pr; printf '%b' "$P\r\n$MADE\r\n" >"$PD/con/console.log"; o=$(proof_ready "$OUT" "$PD" 2330)
check "a ready line after the prompt fails proof_ready" eval 'has "$o" "PROOF-FAIL: the weaver-appliance line comes after the login prompt"'

# ---- proof_second_boot, against a stand-in hypervisor whose start writes the second boot's
# console where QEMU's logfile is.
qemu_file() { : >"$1"; }
proof_sleep() { :; }
PROOF_FIRST_BOOT=4
virsh_() { printf '%s\n' "$*" >>"$T/virsh.log"
    case $1 in
        domstate) cat "$T/domstate" ;;
        start) [ ! -e "$T/no-start" ] || { echo "error: Failed to start domain"; return 1; }
            printf '%b' "$(cat "$T/second-console")" >>"$PD/con/console.log"; echo "Domain '$2' started" ;;
        *) return 9 ;;
    esac; }
sb() { rm -f -- "$T/virsh.log" "$PD/con/first-console.log"; echo "shut off" >"$T/domstate"
    printf '%b' "$MADE\r\n$P" >"$PD/con/console.log"; printf '%s' "[  OK  ] Started x.\r\n$FOUND\r\n$P" >"$T/second-console"
    printf 'Result=success\nExecMainStatus=0\nExecMainExitTimestampMonotonic=8800000\n' >"$T/second-unit"
    attrs >"$OUT/ready-attributes.txt"; { echo "the request unit: $(tr '\n' ' ' <"$T/second-unit")"; attrs; } >"$T/second-read"; }
sb; o=$(proof_second_boot "$OUT" "$PD" mock-appliance-first 2330); rc=$?
check "a second boot that finds both and remade nothing: no finding, status 0" \
    eval '[ "$rc" = 0 ] && [ "$(fails "$o")" = 0 ] && [ "$(grep -c "^unchanged: " <<<"$o")" = 4 ] && has "$o" "on the console: $FOUND"'
check "the first boot's console is set aside whole, and the second's written new" \
    eval 'grep -qaF "$MADE" "$PD/con/first-console.log" && ! grep -qaF "$MADE" "$PD/con/console.log" && grep -qaF "$FOUND" "$PD/con/console.log"'
check "the same domain is started after it was read shut off" test "$(cat "$T/virsh.log")" = "domstate mock-appliance-first
start mock-appliance-first"
sb; echo running >"$T/domstate"; o=$(proof_second_boot "$OUT" "$PD" mock-appliance-first 2330); rc=$?
check "a guest not shut off by its own button: no second boot, a finding, status 1" \
    eval '[ "$rc" = 1 ] && has "$o" "PROOF-FAIL: no second boot: mock-appliance-first is '"'"'running'"'"', not shut off by its own power button" && [ "$(grep -c "^start" "$T/virsh.log")" = 0 ]'
sb; touch "$T/no-start"; o=$(proof_second_boot "$OUT" "$PD" mock-appliance-first 2330); rc=$?; rm -f -- "$T/no-start"
check "a domain that will not start: a finding naming the hypervisor's answer, status 1" \
    eval '[ "$rc" = 1 ] && has "$o" "PROOF-FAIL: no second boot: mock-appliance-first could not be started: error: Failed to start domain "'
sb; printf '%s' "[  OK  ] Started x.\r\n$FOUND\r\n" >"$T/second-console"; o=$(proof_second_boot "$OUT" "$PD" mock-appliance-first 2330); rc=$?
check "no login prompt within the bound is a finding" eval '[ "$rc" = 0 ] && has "$o" "PROOF-FAIL: the second boot gave no login prompt within 4 s"'
sb; printf '%s' "$MADE\r\n$P" >"$T/second-console"; o=$(proof_second_boot "$OUT" "$PD" mock-appliance-first 2330)
check "a second boot that made the connectors again is a finding" \
    eval 'has "$o" "PROOF-FAIL: the console'"'"'s weaver-appliance line is not '"'"'$FOUND'"'"'"'
sb; { echo "the request unit: Result=exit-code ExecMainStatus=1"; echo "PROOF-FAIL: the request unit's run was not a success"; attrs; } >"$T/second-read"
o=$(proof_second_boot "$OUT" "$PD" mock-appliance-first 2330)
check "the unit failed at the second boot is a finding" eval 'has "$o" "PROOF-FAIL: the request unit'"'"'s run was not a success"'
sb; { echo "the request unit: Result=success"; attrs "" "$S2"; } >"$T/second-read"; o=$(proof_second_boot "$OUT" "$PD" mock-appliance-first 2330)
check "a request rewritten at the second boot is a finding" \
    eval '[ "$(fails "$o")" = 1 ] && grep -q "^PROOF-FAIL: remade: the request.s digest at /etc/weaver-web/pairing/exchange/gate.req" <<<"$o"'
# The second boot's read runs the same unit and attribute text the first boot's does.
sb; o=$(proof_second_boot "$OUT" "$PD" mock-appliance-first 2330)
check "the second boot reads with the first boot's own unit and attribute text, on the proof's port" \
    eval '[ "$(cat "$T/gssh-last")" = "$(printf "U=%s\n%s\n%s" "$READY_UNIT" "$READY_UNIT_READ" "$READY_ATTRS_READ")" ] && [ "$(sort -u "$T/gssh-ports")" = 2330 ]'
finish
