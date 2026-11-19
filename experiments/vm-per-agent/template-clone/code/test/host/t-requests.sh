#!/usr/bin/env bash
# The request unit's script, guest/ready/requests (weaver-appliance-build-Spec section 2,
# the ready layer), run from its own text against a stand-in tree: its paths pointed at
# the tree, its setpriv prefix dropped (no test stubs a privileged command), and two
# stand-in connectors that make their config new and print their request line in pieces,
# as a real producer writes. Checked: both made where neither stands; nothing remade where
# both stand; a refusal naming the connector, with nothing removed, where one stands
# without the other, with what clears it, and both made once the half-made one is cleared;
# no request file where a connector exits other than 0, prints two lines, a line not of the
# form or one over the bound; no temporary file left; words that follow what stands where
# the request's folder could not be synced after its rename; a link at a request's path
# never written through; the console's two lines. Then the unit file.
# Pure logic: no root, no guest.
source "$(dirname "$0")/lib-test.sh"
RQ=$CODE/guest/ready/requests
UNIT=$CODE/guest/ready/weaver-appliance-requests.service
TS=$CODE/guest/template-steps
SP='setpriv --reuid="$acct" --regid="$acct" --init-groups --reset-env --no-new-privs -- '
check "the script drops to each connector's account by setpriv, the one privileged prefix" \
    test "$(grep -cF -- "$SP" "$RQ")" = 1
R=$T/root
B=$T/bin
mkdir -p "$B"
# A stand-in connector: request --config PATH makes PATH new at 0600 and prints its line
# in pieces. MODE_<plane> picks a fault: fail, two, form, long.
for con in gate-con admin-con; do
    cat >"$B/$con" <<'EOF'
#!/usr/bin/env bash
set -uo pipefail
[ "$1" = request ] && [ "$2" = --config ] || exit 2
cfg=$3; plane=${cfg%/*}; plane=${plane##*/}
echo "$(basename "$0") $*" >>"$CALLS"
mode=MODE_$plane; mode=${!mode:-ok}
[ "$mode" = fail ] && { echo "a stand-in fault" >&2; exit 3; }
( set -C; umask 077; printf 'key = "-----BEGIN PRIVATE KEY-----stand-in-----END PRIVATE KEY-----"\n' >"$cfg" ) || exit 4
case $mode in
    form) printf '{"event":"refused","reason":"no"}\n' ;;
    two) printf '{"event":"request","request":"-----BEGIN CERTIFICATE REQUEST-----\\nAAAA\\n-----END CERTIFICATE REQUEST-----\\n"}\nmore\n' ;;
    long) printf '{"event":"request","request":"-----BEGIN CERTIFICATE REQUEST-----\\n'; head -c 16400 /dev/zero | tr '\0' A; printf '\\n-----END CERTIFICATE REQUEST-----\\n"}\n' ;;
    *) printf '{"event":"request",'; printf '"request":"-----BEGIN CERTIFICATE REQUEST-----\\n'
       printf 'MIG6MGICAQAw%s\\n' "$plane"; printf -- '-----END CERTIFICATE REQUEST-----\\n"}'; printf '\n' ;;
esac
EOF
    chmod 0755 "$B/$con"
done
export CALLS=$T/calls
script=$(sed -e "s|$SP||" -e "s|^PAIRING=/etc/weaver-web/pairing\$|PAIRING=$R/pairing|" \
    -e "s|^BIN=/opt/weaver-web/bin\$|BIN=$B|" -e 's|^export LC_ALL=C PATH=/usr/bin:/usr/sbin$|export LC_ALL=C|' "$RQ")
check "the stand-in tree's paths took" bash -c 'grep -qxF "PAIRING=$1/pairing" <<<"$2" && grep -qxF "BIN=$3" <<<"$2" && ! grep -qF setpriv <<<"$2"' _ "$R" "$script" "$B"
printf '%s\n' "$script" >"$T/requests"
laid() {
    rm -rf -- "${R:?}"; mkdir -p "$R/pairing/gate" "$R/pairing/admin" "$R/pairing/exchange"
    : >"$CALLS"; unset MODE_gate MODE_admin
}
run() { out=$(bash "$T/requests" 2>"$T/err"); rc=$?; }
temps() { find "$R/pairing" -maxdepth 1 -name '.*.req.*' | wc -l; }

laid; run
check "neither stands: both made, exit 0" test "$rc" = 0
check "the ready line names both made" test "$out" = "weaver-appliance: ready (gate-con made, admin-con made)"
check "each request is the connector's one line, mode 0644 (root's in the image, the script's account here)" \
    bash -c 'for p in gate admin; do f=$1/pairing/exchange/$p.req; [ "$(stat -c %a "$f")" = 644 ] && [ "$(wc -l <"$f")" = 1 ] && grep -q "^{\"event\":\"request\",\"request\":\"-----BEGIN CERTIFICATE REQUEST-----\\\\nMIG6MGICAQAw$p" "$f" || exit 1; done' _ "$R"
check "each config made by its connector, mode 0600" bash -c '[ "$(stat -c %a "$1/pairing/gate/gate-con.toml")" = 600 ] && [ "$(stat -c %a "$1/pairing/admin/admin-con.toml")" = 600 ]' _ "$R"
check "no temporary file left" test "$(temps)" = 0
check "each connector asked once, gate first" test "$(cat "$CALLS")" = "gate-con request --config $R/pairing/gate/gate-con.toml
admin-con request --config $R/pairing/admin/admin-con.toml"
before=$(cd "$R" && find . -type f -exec sha256sum {} + | sort)

: >"$CALLS"; run
check "both stand: found, exit 0, no connector asked" test "$rc" = 0 -a "$out" = "weaver-appliance: ready (gate-con found, admin-con found)" -a ! -s "$CALLS"
check "nothing remade: every file as it was" test "$(cd "$R" && find . -type f -exec sha256sum {} + | sort)" = "$before"

laid; ( umask 077; : >"$R/pairing/gate/gate-con.toml" ); run
check "a config without its request: refused naming gate-con, exit 1" \
    bash -c '[ "$1" = 1 ] && grep -qF "weaver-appliance: refused: gate-con: its config stands without its request; nothing is remade or removed;" <<<"$2" && grep -qF "(admin-con made)" <<<"$2"' _ "$rc" "$out"
# A first boot cut between the config and its request: every later boot refuses, and the
# line says what clears it, naming the config's own path.
check "the refusal says what clears it: the config removed as root and a boot again, or the machine made again" \
    grep -qF "gate-con: its config stands without its request; nothing is remade or removed; to clear it, remove $R/pairing/gate/gate-con.toml as root and boot again, or make the virtual machine again from the template" <<<"$out"
check "the config left in place, and gate-con not asked" test -f "$R/pairing/gate/gate-con.toml" -a "$(grep -c '^gate-con' "$CALLS")" = 0
rm -f -- "$R/pairing/gate/gate-con.toml"; : >"$CALLS"; run
check "with that config removed, the next boot makes both: gate-con made, admin-con found" \
    test "$rc" = 0 -a "$out" = "weaver-appliance: ready (gate-con made, admin-con found)"

laid; echo '{"event":"request","request":"x"}' >"$R/pairing/exchange/admin.req"; run
check "a request without its config: refused naming admin-con, the request left" \
    bash -c '[ "$1" = 1 ] && grep -qF "admin-con: its request stands without its config" <<<"$2" && [ -s "$3" ]' _ "$rc" "$out" "$R/pairing/exchange/admin.req"
check "and says what clears it, naming the request's path" \
    grep -qF "to clear it, remove $R/pairing/exchange/admin.req as root and boot again, or make the virtual machine again from the template" <<<"$out"

laid; export MODE_gate=fail; run
check "a connector that exits 3: refused, no request file, no temporary file" \
    bash -c '[ "$1" = 1 ] && grep -qF "gate-con: request exited 3" <<<"$2" && [ ! -e "$3" ] && [ "$4" = 0 ]' _ "$rc" "$out" "$R/pairing/exchange/gate.req" "$(temps)"
check "its reason reaches standard error, the journal's" grep -qF "a stand-in fault" "$T/err"
for m in two form long; do
    laid; export MODE_admin=$m; run
    check "a connector that prints $m: refused, no request file, its config standing" \
        bash -c '[ "$1" = 1 ] && grep -qF "admin-con: request printed no request line of the form pair reads; its config stands, so nothing is remade; to clear it, remove $4 as root" <<<"$2" && [ ! -e "$3" ] && [ -f "$4" ] && [ "$5" = 0 ]' \
        _ "$rc" "$out" "$R/pairing/exchange/admin.req" "$R/pairing/admin/admin-con.toml" "$(temps)"
done

# The rename made and the exchange folder's sync failed: a stand-in sync first on PATH,
# failing for that folder alone. Both files stand, so the words say the request is in
# place and advise a boot again, removing nothing; that boot finds both.
mkdir -p "$T/sb"
printf '#!/usr/bin/env bash\nfor a; do case $a in */pairing/exchange) echo "sync: a stand-in fault" >&2; exit 1 ;; esac; done\nexec /usr/bin/sync "$@"\n' >"$T/sb/sync"
chmod 0755 "$T/sb/sync"
laid; out=$(PATH="$T/sb:$PATH" bash "$T/requests" 2>"$T/err"); rc=$?
check "the exchange folder's sync failing after the rename: refused for that boot, the request said to be in place" \
    bash -c '[ "$1" = 1 ] && grep -qF "gate-con: its request is in place at $3, but its folder could not be synced to disk; nothing is removed; boot again, and the next boot finds both" <<<"$2"' _ "$rc" "$out" "$R/pairing/exchange/gate.req"
check "and no advice to remove anything, no word that the request was not written, both files standing" \
    bash -c '! grep -qE "could not be written|to clear it, remove" <<<"$1" && [ -s "$2" ] && [ -f "$3" ] && [ "$4" = 0 ]' _ "$out" "$R/pairing/exchange/gate.req" "$R/pairing/gate/gate-con.toml" "$(temps)"
: >"$CALLS"; run
check "the next boot finds both, asking no connector" test "$rc" = 0 -a "$out" = "weaver-appliance: ready (gate-con found, admin-con found)" -a ! -s "$CALLS"

laid; echo kept >"$T/target"; ln -s "$T/target" "$R/pairing/exchange/gate.req"; run
check "a link at a request's path, its config absent: refused, never written through" \
    bash -c '[ "$1" = 1 ] && grep -qF "gate-con: its request stands without its config" <<<"$2" && [ "$(cat "$3")" = kept ]' _ "$rc" "$out" "$T/target"
laid; ( umask 077; : >"$R/pairing/gate/gate-con.toml" ); ln -s "$T/target" "$R/pairing/exchange/gate.req"; run
check "a link at a request's path beside its config: refused as no regular file" \
    bash -c '[ "$1" = 1 ] && grep -qF "gate-con: $3 is not a regular file" <<<"$2"' _ "$rc" "$out" "$R/pairing/exchange/gate.req"

# request_line's bound: a line of exactly 16383 bytes plus its newline passes, one more fails.
eval "$(fn "$T/requests" request_line)"
eval "$(grep -E '^(BOUND|HEAD|TAIL)=' "$T/requests")"
mk() { local n=$1 fill; fill=$(head -c $((n - ${#HEAD} - ${#TAIL})) /dev/zero | tr '\0' A); printf '%s%s%s' "$HEAD" "$fill" "$TAIL"; }
check "a line of 16383 bytes and its newline is within the bound" request_line "$(mk 16383)"
check "a line of 16384 bytes and its newline is over it" eval '! request_line "$(mk 16384)"'

# The unit.
u=$(cat "$UNIT")
eval "$(grep -E '^READY_(REQUESTS|UNIT)=' "$TS")"
check "the unit runs the script the ready step lays" grep -qxF "ExecStart=$READY_REQUESTS" <<<"$u"
check "the unit's name is the one the ready step enables" test "$(basename "$UNIT")" = "$READY_UNIT"
check "a oneshot, before logins are allowed, after the local filesystems" \
    bash -c 'grep -qx "Type=oneshot" <<<"$1" && grep -qx "Before=systemd-user-sessions.service" <<<"$1" && grep -qx "After=local-fs.target" <<<"$1"' _ "$u"
check "no network: none wanted or waited for, and none given" \
    bash -c '! grep -qiE "network|nss-lookup" <<<"$(grep -vE "^(#|PrivateNetwork=)" <<<"$1")" && grep -qx "PrivateNetwork=yes" <<<"$1"' _ "$u"
check "its line on the console and in the journal; enabled in multi-user.target" \
    bash -c 'grep -qx "StandardOutput=journal+console" <<<"$1" && grep -qx "WantedBy=multi-user.target" <<<"$1"' _ "$u"
finish
