#!/usr/bin/env bash
# proof_inbound's grading: gssh stubbed with what a guest would print, and loopback servers
# on PORT and DPORT standing in for what the forwards relay. The listener serves a new
# empty directory (the Planner's review of 2026-11-15). The table's own grading runs in the
# guest and is not reached here. Pure logic: no root, no guest.
source "$(dirname "$0")/lib-test.sh"
need python3
export VM_ROOT=$T/vmroot; mkdir -p "$VM_ROOT"; source "$CODE/lib.sh"
eval "$(fn "$CODE/build-template.sh" proof_inbound)"
TABLE='table inet weaver_appliance {
	chain input {
		type filter hook input priority filter; policy drop;
		tcp dport 22 accept
	}
}'
gssh() { shift
    case $* in
        *"nft list table inet weaver_appliance"*) [ -e "$T/no-table" ] && { echo "Error: No such file or directory"; echo "PROOF-FAIL: the table inet weaver_appliance is not loaded"; return 0; }
            echo "-- nftables.service: active"; echo "table inet weaver_appliance"; printf '%s\n' "$TABLE" ;;
        *"http.server 8080"*) echo 4242 ;;
        *"urllib.request"*) echo "LISTEN 0 5 0.0.0.0:8080 0.0.0.0:*"; [ -e "$T/no-loopback" ] || echo "loopback: 200" ;;
        *"kill 4242"*) echo "the listener is stopped" ;;
        *) echo "unexpected: $*" ;;
    esac; }
serve() { # serve KIND: a loopback server, its port in T/KIND
    python3 -I - "$T" "$1" <<'PY' &
import socket, sys, threading, time, os
d, kind = sys.argv[1], sys.argv[2]
l = socket.socket(); l.bind(("127.0.0.1", 0)); l.listen(8)
open(os.path.join(d, kind), "w").write(str(l.getsockname()[1]))
while True:
    c, _ = l.accept()
    if kind == "ssh": c.sendall(b"SSH-2.0-OpenSSH_10.0p2\r\n"); c.close()
    elif kind == "http": c.recv(100); c.sendall(b"HTTP/1.0 200 OK\r\n\r\n"); c.close()
    elif kind == "silent": threading.Thread(target=lambda c=c: (time.sleep(30), c.close()), daemon=True).start()
PY
}
for k in ssh http silent; do serve "$k"; done
wait_for "$(jobs -p | tr '\n' ' ')" 10 eval '[ -s "$T/ssh" ] && [ -s "$T/http" ] && [ -s "$T/silent" ]' \
    || { no "the three loopback servers came up within 10 seconds"; kill $(jobs -p) 2>/dev/null; finish; exit 1; }
P=$(cat "$T/ssh"); H=$(cat "$T/http"); Q=$(cat "$T/silent")
proof_inbound "$P" "$Q" x >"$T/good.out" 2>&1
check "a dropped listener and SSH answering pass" test "$(grep -c PROOF-FAIL "$T/good.out")" = 0 -a "$(grep -c '^-- TCP 8080 from the host, through the forward: silent$' "$T/good.out")" = 1
proof_inbound "$P" "$H" x >"$T/open.out" 2>&1
check "a listener that answers from the host fails" grep -q 'PROOF-FAIL: inbound to TCP 8080 was not dropped (answered: HTTP/1.0 200 OK)' "$T/open.out"
proof_inbound "$Q" "$Q" x >"$T/nossh.out" 2>&1
check "SSH silent fails" grep -q 'PROOF-FAIL: SSH did not answer through the forward (silent)' "$T/nossh.out"
touch "$T/no-table"; proof_inbound "$P" "$Q" x >"$T/notable.out" 2>&1; rm "$T/no-table"
check "no table fails" grep -q 'PROOF-FAIL: the table inet weaver_appliance is not loaded' "$T/notable.out"
touch "$T/no-loopback"; proof_inbound "$P" "$Q" x >"$T/nolb.out" 2>&1; rm "$T/no-loopback"
check "a listener that does not answer on loopback fails" grep -q "PROOF-FAIL: the listener on TCP 8080 did not answer on the guest's loopback" "$T/nolb.out"
# The table's own grading (policy drop, the SSH accept) runs in the guest and is not
# reached here: a host test stubs no privileged command.
kill $(jobs -p) 2>/dev/null
# The listener's start, run as sshd runs a command: bash -c, its output a pipe that closes
# only when every holder exits. The rehearsal of 2026-11-16 hung here for 49 minutes: the
# whole && list went to the background, so a subshell held the channel open while the
# server ran, and $! named that subshell, not the server. Here the server is a stand-in
# (sleep, then a listener on a port of its own), and ss is stubbed to see that listener.
start=$(fn "$CODE/build-template.sh" proof_inbound | awk '/lpid=\$\(KNOWN_HOSTS=/ { f = 1; next } f && /^ *echo "\$p"/ { sub(/'"'"' 2>\/dev\/null\) \|\| lpid=""$/, ""); print; exit } f { print }')
check "the start's text is found" grep -qF 'http.server 8080' <<<"$start"
check "the listener serves a new empty directory, not /tmp" eval 'grep -qF "d=\$(mktemp -d) && cd \"\$d\"" <<<"$start" && ! grep -qF "cd /tmp" <<<"$start"'
W=$(mktemp -d "$T/start.XXXXXX"); mkdir -p "$W/bin"
cat >"$W/bin/python3" <<'STUB'
#!/usr/bin/env bash
# python3 -m http.server 8080 ...: the stand-in server marks itself up, then runs.
echo "$$" >"$STARTDIR/up"; exec -a stand-in-server sleep 30
STUB
cat >"$W/bin/ss" <<'STUB'
#!/usr/bin/env bash
[ -s "$STARTDIR/up" ] && echo "LISTEN 0 5 0.0.0.0:8080 0.0.0.0:*"
STUB
chmod 755 "$W/bin/python3" "$W/bin/ss"
t0=$(date +%s)
STARTDIR=$W PATH="$W/bin:$PATH" timeout 10 bash -c "$start" | cat >"$W/out"; rc=${PIPESTATUS[0]}; got=$(cat "$W/out")
t1=$(date +%s)
check "the start returns once the server is up, its pipe closed (no hang)" test "$rc" = 0 -a $((t1 - t0)) -lt 8
check "and the PID it prints is the server's own" eval '[[ "$got" =~ ^[0-9]+$ ]] && [ "$got" = "$(cat "$W/up" 2>/dev/null)" ]'
kill "$(cat "$W/up" 2>/dev/null)" 2>/dev/null
finish
