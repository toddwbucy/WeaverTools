#!/usr/bin/env bash
# The ready layer as the image lays it (weaver-appliance-build-Spec section 2, step 10 and
# the probes): shape_check, ready_check and private_keys from guest/template-steps' own
# text, against a stand-in root and stand-in account files, with their root prefix
# dropped, their paths pointed at the tree, and root's ownership read as this account's
# (the stand-in tree is this account's). getent, id, systemctl and findmnt are stood in;
# they read files and print. Then where it is wired: the step, the probes in both passes,
# the stamp's lines, the copy into the build guest, and the seal's removal of that copy.
# Pure logic: no root, no guest.
source "$(dirname "$0")/lib-test.sh"
TS=$CODE/guest/template-steps
BT=$CODE/build-template.sh
P='s''udo '
ME=$(id -un); MG=$(id -gn)
eval "$(grep -E '^READY_[A-Z_]+=' "$TS")"
check "the three accounts, defined once" test "$READY_ACCOUNTS" = "weaver-gatecon weaver-admincon weaver-pair" -a "$(grep -c '^READY_ACCOUNTS=' "$TS")" = 1
check "the six folders, parents first, with their owners and modes" test "$READY_DIRS" = \
    "/etc/weaver-web:root:755 /etc/weaver-web/pairing:root:755 /etc/weaver-web/pairing/gate:weaver-gatecon:700 /etc/weaver-web/pairing/admin:weaver-admincon:700 /etc/weaver-web/pairing/exchange:weaver-pair:755 /etc/weaver-web/connectors:root:755"
check "the survey, the request script and the unit at their paths" test \
    "$READY_SURVEY $READY_REQUESTS $READY_UNIT_FILE" = "/opt/weaver-appliance/bin/weaver-survey /opt/weaver-appliance/libexec/requests /etc/systemd/system/weaver-appliance-requests.service"

# Stand-in account files, and the commands that read them.
getent() {
    [ $# -ge 2 ] || { cat "$T/$1"; return 0; }
    awk -F: -v k="$2" '$1 == k || $3 == k { print; f = 1 } END { exit !f }' "$T/$1" || return 2
}
id() { [ "$1" = -Gn ] || return 9; awk -v n="$3" '$1 == n { $1 = ""; sub(/^ /, ""); print; f = 1 } END { exit !f }' "$T/idgroups"; }
systemctl() { [ "$1" = is-enabled ] && echo "${UNIT_STATE:-enabled}"; }
findmnt() { printf '%s\n' / /proc ${EXTRA_MOUNT:-}; }
accounts() {
    printf 'UID_MIN 1000\n' >"$T/login.defs"
    : >"$T/passwd"; : >"$T/group"; : >"$T/idgroups"
    local a u=970
    for a in $READY_ACCOUNTS; do
        printf '%s:x:%s:%s::%s:/usr/sbin/nologin\n' "$a" "$u" "$u" "$T/nohome/$a" >>"$T/passwd"
        printf '%s:x:%s:\n' "$a" "$u" >>"$T/group"
        printf '%s %s\n' "$a" "$a" >>"$T/idgroups"
        u=$((u - 1))
    done
}
eval "$(fn "$TS" shape_check | sed -e "s|/etc/login.defs|$T/login.defs|")"
accounts
for a in $READY_ACCOUNTS; do
    out=$(shape_check "$a"); rc=$?
    check "$a in the box's shape: no finding" test "$rc" = 0 -a -z "$out"
done
shape() { # shape SED-ON-PASSWD|- SED-ON-GROUP|- SED-ON-IDGROUPS|- TEXT: a finding naming TEXT for weaver-gatecon
    accounts
    [ "$1" = - ] || sed -i "$1" "$T/passwd"; [ "$2" = - ] || sed -i "$2" "$T/group"; [ "$3" = - ] || sed -i "$3" "$T/idgroups"
    out=$(shape_check weaver-gatecon); rc=$?
    [ "$rc" = 1 ] && grep -qF -- "$4" <<<"$out"
}
check "uid 1000, not a system account: named" shape 's/^weaver-gatecon:x:970:/weaver-gatecon:x:1000:/' - - "has uid 1000, not above 0 and below UID_MIN 1000"
check "uid 0, root's: named" shape 's/^weaver-gatecon:x:970:/weaver-gatecon:x:0:/' - - "has uid 0"
check "a login shell: named" shape '/^weaver-gatecon:/s|/usr/sbin/nologin|/bin/bash|' - - "has the shell '/bin/bash', not nologin"
mkdir -p "$T/nohome/weaver-gatecon"
check "a home standing: named" shape - - - "has a home standing"
rmdir "$T/nohome/weaver-gatecon"
check "a primary group of another name: named" shape - 's/^weaver-gatecon:x:970:/gatecon:x:970:/' - "primary group is gatecon, not its own"
check "a member in its group: named" shape - 's/^weaver-gatecon:x:970:$/weaver-gatecon:x:970:op/' - "group has the members op"
check "its group another account's primary group too: named" shape 's/^weaver-pair:x:968:968:/weaver-pair:x:968:970:/' - - "is the primary group of weaver-pair too"
check "a second group: named" shape - - 's/^weaver-gatecon weaver-gatecon$/weaver-gatecon weaver-gatecon wheel/' "holds the groups 'weaver-gatecon wheel'"
check "an account that does not stand: named" shape '/^weaver-gatecon:/d' - - "weaver-gatecon does not stand"
accounts

# ready_check against a stand-in root.
R=$T/root
BUILD=$T/build; RECORD=$T/record
READY_WEB=$R/etc/weaver-web
dirs=""
for d in $READY_DIRS; do IFS=: read -r path owner mode <<<"$d"; dirs+="${dirs:+ }$R$path:$ME:$mode"; done
READY_DIRS=$dirs
READY_SURVEY=$R$READY_SURVEY; READY_REQUESTS=$R$READY_REQUESTS; READY_UNIT_FILE=$R$READY_UNIT_FILE
rc_text=$(fn "$TS" ready_recorded ready_check | sed -e "s|$P||g" -e "s|directory \$owner:\$owner|directory \$owner:$MG|g" \
    -e "s|regular file root:root|regular file $ME:$MG|g" -e "s|stat -c %d -- / |stat -c %d -- $R |")
check "the stand-in tree's substitutions took" bash -c 'grep -qF "directory \$owner:$2" <<<"$1" && grep -qF "regular file $3:$2" <<<"$1" && grep -qF "stat -c %d -- $4 " <<<"$1" && ! grep -qF "s""udo " <<<"$1"' _ "$rc_text" "$MG" "$ME" "$R"
eval "$rc_text"
laid() {
    rm -rf -- "${R:?}" "${BUILD:?}" "${RECORD:?}"; mkdir -p "$BUILD/stamp.d" "$RECORD"
    local d path owner mode
    for d in $READY_DIRS; do IFS=: read -r path owner mode <<<"$d"; mkdir -p "$path"; chmod "0$mode" "$path"; done
    mkdir -p "${READY_SURVEY%/*}" "${READY_REQUESTS%/*}" "${READY_UNIT_FILE%/*}"
    cp "$CODE/guest/ready/weaver-survey" "$READY_SURVEY"; cp "$CODE/guest/ready/requests" "$READY_REQUESTS"
    cp "$CODE/guest/ready/$READY_UNIT" "$READY_UNIT_FILE"; chmod 0755 "$READY_SURVEY" "$READY_REQUESTS"; chmod 0644 "$READY_UNIT_FILE"
    local k f
    for k in survey:$READY_SURVEY requests:$READY_REQUESTS unit:$READY_UNIT_FILE; do
        f=${k#*:}; sha256sum "$f" | cut -d' ' -f1 >"$BUILD/stamp.d/ready.${k%%:*}"
    done
    accounts; unset UNIT_STATE EXTRA_MOUNT
}
finding() { out=$(ready_check 2>&1); rc=$?; [ "$rc" = 1 ] && grep -qF -- "$1" <<<"$out"; }
laid; out=$(ready_check 2>&1); rc=$?
check "as laid: the accounts, the six folders, the unit, the three files; no finding" test "$rc" = 0 -a -z "$out"
laid; ( umask 077; echo 'key = "-----BEGIN PRIVATE KEY-----"' >"$READY_WEB/pairing/gate/gate-con.toml" )
check "a config in pairing/gate: named, so no key stands at the seal" finding "holds other than the six folders: $READY_WEB/pairing/gate/gate-con.toml"
laid; echo '{"event":"request"}' >"$READY_WEB/pairing/exchange/gate.req"
check "a request in pairing/exchange: named" finding "holds other than the six folders: $READY_WEB/pairing/exchange/gate.req"
laid; mkdir "$READY_WEB/connectors/agent"
check "an agent's folder under connectors: named" finding "holds other than the six folders: $READY_WEB/connectors/agent"
laid; chmod 0755 "$READY_WEB/pairing/gate"
check "pairing/gate at 0755: named" finding "$READY_WEB/pairing/gate is 'directory $ME:$MG 755"
laid; rmdir "$READY_WEB/connectors"; ln -s "$T" "$READY_WEB/connectors"
check "a link in place of connectors: named" finding "$READY_WEB/connectors is 'symbolic link"
laid; rmdir "$READY_WEB/pairing/exchange"
check "exchange absent: named" finding "$READY_WEB/pairing/exchange is 'absent'"
laid; EXTRA_MOUNT=$READY_WEB/pairing
check "a filesystem mounted under /etc/weaver-web: named" finding "mounted at or under $READY_WEB: $READY_WEB/pairing"
laid; EXTRA_MOUNT=${READY_WEB}x
out=$(ready_check 2>&1); rc=$?
check "a mount beside it, of a longer name, is not under it" test "$rc" = 0
laid; eval "$(sed "s|stat -c %d -- $R |stat -c %d -- /proc |" <<<"$rc_text")"
out=$(ready_check 2>&1)
check "the folders on another filesystem than the root's: each of the six named" test "$(grep -o 'on the root filesystem' <<<"$out" | wc -l)" = 6
eval "$rc_text"
laid; UNIT_STATE=masked
check "the unit masked: named" finding "$READY_UNIT is 'masked', not enabled"
laid; echo '# changed' >>"$READY_SURVEY"
check "the survey changed after its record: named" finding "$READY_SURVEY is '"
laid; chmod 0644 "$READY_REQUESTS"
check "the request script at 0644: named" finding "$READY_REQUESTS is 'regular file $ME:$MG 644', not a regular file $ME:$MG 755"
laid; sed -i '/^weaver-pair:/s|:/usr/sbin/nologin$|:/bin/sh|' "$T/passwd"
check "an account out of shape: named by ready_check" finding "weaver-pair has the shell '/bin/sh', not nologin"
# The final pass: the build's records are gone, and the stamp being assembled holds the lines.
laid; { for k in survey requests unit; do printf 'ready.%s = %s\n' "$k" "$(cat "$BUILD/stamp.d/ready.$k")"; done; } >"$RECORD/stamp.part"
rm -rf -- "${BUILD:?}/stamp.d"
out=$(ready_check 2>&1); rc=$?
check "the final pass reads the digests from the stamp being assembled: no finding" test "$rc" = 0 -a -z "$out"
sed -i '/^ready\.unit = /d' "$RECORD/stamp.part"
check "the final pass, a line missing from the stamp: named" finding "not the stamp's ready.unit ''"
laid; rm -f "$BUILD/stamp.d/ready.unit"; rm -f "$RECORD/stamp.part"
check "no record of the unit's digest at all: named" finding "not the stamp's ready.unit ''"

# private_keys reads /etc/weaver-web for a key, whatever the file's name.
pk=$(fn "$TS" private_keys | sed -e "s|$P||g" -e "s|/etc /var /home /root /opt /usr/local|$T/empty|g")
mkdir -p "$T/empty"; SEAL_KEYS_DIR=$T/nokeys
eval "$pk"
laid; out=$(private_keys)
check "private_keys: none in the layer as laid" test -z "$out"
( umask 077; printf -- '-----BEGIN PRIVATE KEY-----\nAAAA\n-----END PRIVATE KEY-----\n' >"$READY_WEB/pairing/admin/admin-con.toml" )
out=$(private_keys)
check "private_keys: a connector's config holding a key, named" test "$out" = "$READY_WEB/pairing/admin/admin-con.toml"

# Where it is wired.
step=$(fn "$TS" ready); probes=$(fn "$TS" probes); cleanup=$(fn "$TS" cleanup); seal_final=$(fn "$TS" seal_final)
check "the step lays each account by create-agent.sh's own line" \
    grep -qF 'sudo useradd --system --shell /usr/sbin/nologin --no-create-home --user-group "$a"' <<<"$step"
check "the step enables the unit and never starts it" \
    bash -c 'grep -qF "sudo systemctl enable \"\$READY_UNIT\"" <<<"$1" && ! grep -qE "systemctl (start|restart)|--now" <<<"$1"' _ "$step"
check "the step records the four ready lines and ends on ready_check" \
    bash -c 'for k in ready.accounts ready.survey ready.requests ready.unit; do grep -qF "$k" <<<"$1" || exit 1; done; grep -qF "v=\$(ready_check 2>&1)" <<<"$1"' _ "$step"
final_at=$(grep -n -F 'if [ "$mode" = final ]; then' <<<"$probes" | cut -d: -f1)
at=$(grep -n -F 'check "the ready layer not as laid" "$(ready_check 2>&1)"' <<<"$probes" | cut -d: -f1)
check "the probes run ready_check in both passes" eval '[ -n "$at" ] && [ -n "$final_at" ] && [ "$at" -lt "$final_at" ]'
check "the connector probe reads the units alone, no longer /etc/weaver-web" \
    bash -c 'grep -qF "check \"a connector'"'"'s unit\" \"\$(ls -d /etc/systemd/system/weaver-web-* 2>/dev/null)\"" <<<"$1" && ! grep -qF "ls -d /etc/weaver-web" <<<"$1"' _ "$probes"
check "the cleanup names nothing of the layer" bash -c '! grep -qE "weaver-appliance/|/etc/weaver-web|READY_" <<<"$1"' _ "$cleanup"
check "the dispatch runs the step" grep -qxF 'ready) ready ;;' "$TS"
check "the build copies the three files in, where the step reads them" \
    bash -c 'grep -qF "\"\$HERE/guest/ready/weaver-survey\" \"\$HERE/guest/ready/requests\"" "$1" && grep -qF "\"\$HERE/guest/ready/weaver-appliance-requests.service\" /tmp/" "$1" && grep -qF "sudo install -D -m 0644 -t $2 /tmp/weaver-survey /tmp/requests /tmp/weaver-appliance-requests.service" "$1"' _ "$BT" "$READY_SRC"
check "the copy sits under the harness folder the seal removes" \
    bash -c '[ "${2#/usr/local/share/weaver-build/}" != "$2" ] && grep -qF "rm -rf -- \"\$BUILD\" /usr/local/sbin/template-steps /usr/local/share/weaver-build" <<<"$1"' _ "$seal_final" "$READY_SRC"
finish
