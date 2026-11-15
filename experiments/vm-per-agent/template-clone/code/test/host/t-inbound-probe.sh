#!/usr/bin/env bash
# inbound_probe against loopback servers that stand in for what a forward relays: one that
# speaks first (SSH), one that answers a request (HTTP), one that accepts and never speaks
# (a drop, as QEMU's forward relays it), one that closes, and no listener. Pure logic: no
# root, no guest.
source "$(dirname "$0")/lib-test.sh"
need python3
export VM_ROOT=$T/vmroot; mkdir -p "$VM_ROOT"; source "$CODE/lib.sh"
python3 -I - "$T" <<'PY' &
import socket, sys, threading, time, os
d = sys.argv[1]
def serve(kind):
    l = socket.socket(); l.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    l.bind(("127.0.0.1", 0)); l.listen(8)
    open(os.path.join(d, kind), "w").write(str(l.getsockname()[1]))
    while True:
        c, _ = l.accept()
        if kind == "ssh": c.sendall(b"SSH-2.0-OpenSSH_10.0p2 Ubuntu-5\r\n"); time.sleep(5); c.close()
        elif kind == "http": c.recv(100); c.sendall(b"HTTP/1.0 200 OK\r\n\r\nhi"); c.close()
        elif kind == "silent": threading.Thread(target=lambda c=c: (time.sleep(30), c.close()), daemon=True).start()
        elif kind == "close": c.close()
for k in ("ssh", "http", "silent", "close"):
    threading.Thread(target=serve, args=(k,), daemon=True).start()
time.sleep(60)
PY
srv=$!
until [ -s "$T/ssh" ] && [ -s "$T/http" ] && [ -s "$T/silent" ] && [ -s "$T/close" ]; do sleep 0.1; done
r=$(inbound_probe "$(cat "$T/ssh")" 5); echo "  ssh: $r"; check "a server that speaks first is answered" test "${r#answered: SSH-2.0-}" != "$r"
r=$(inbound_probe "$(cat "$T/http")" 5); echo "  http: $r"; check "a server that answers a request is answered" test "$r" = "answered: HTTP/1.0 200 OK"
s0=$(date +%s); r=$(inbound_probe "$(cat "$T/silent")" 4); s1=$(date +%s); echo "  silent: $r ($((s1 - s0)) s)"
check "an accepted connection with nothing sent is silent, after SECONDS" test "$r" = silent -a $((s1 - s0)) -ge 4
r=$(inbound_probe "$(cat "$T/close")" 5); echo "  close: $r"; check "a server that closes is closed" test "${r%% *}" = closed
r=$(inbound_probe 1 3); echo "  none: $r"; check "no listener is closed (refused)" test "${r%% *}" = closed
kill "$srv" 2>/dev/null
finish
