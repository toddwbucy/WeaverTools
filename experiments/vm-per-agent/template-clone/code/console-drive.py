#!/usr/bin/env python3
"""Drive a guest's serial console through QEMU's unix socket, as an operator types at it.

Usage: console-drive.py SOCKET LOG STEPS

STEPS is a file of steps, one per line:
  expect <seconds> <python regex>   wait until the output read since the last match matches
  send <text>                       send text; \\r, \\n, \\t and \\\\ are read as escapes
  sendfile <path>                   send the first line of a file, then \\r. A password is
                                    sent this way, so it is never written into STEPS or LOG
  sleep <seconds>                   read for that long
  # comment

Every byte the console sends is appended to LOG. Each send is logged on a line of its own
as [sent: TEXT], a sendfile as [sent: <file NAME>], with NAME the file's base name. The
socket is connected for up to 30 s, since QEMU makes it as the guest starts. Exit 0 when
every step held; 1 at the first expect that timed out, with the output since the last match
printed; 2 on a usage error or a socket that never answered. Standard library only.
"""
import os, re, select, socket, sys, time


def main():
    if len(sys.argv) != 4:
        sys.stdout.write(__doc__)
        return 2
    path, logp, stepsp = sys.argv[1:4]
    steps = [l.rstrip("\n") for l in open(stepsp) if l.strip() and not l.lstrip().startswith("#")]
    sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    end = time.time() + 30
    while True:
        try:
            sock.connect(path)
            break
        except OSError as e:
            if time.time() > end:
                sys.stdout.write("the console socket %s did not answer: %s\n" % (path, e))
                return 2
            time.sleep(0.5)
    fd = sock.fileno()
    log = open(logp, "ab", buffering=0)
    buf = b""
    closed = False

    def read_for(t):
        nonlocal buf, closed
        end = time.time() + t
        while not closed:
            left = end - time.time()
            if left <= 0:
                return
            r, _, _ = select.select([fd], [], [], min(left, 0.5))
            if r:
                try:
                    d = os.read(fd, 4096)
                except OSError:
                    d = b""
                if not d:
                    closed = True
                    return
                log.write(d)
                buf += d

    def send(b, shown):
        os.write(fd, b)
        log.write(("\n[sent: %s]\n" % shown).encode())

    rc = 0
    for s in steps:
        op, _, arg = s.partition(" ")
        if op == "expect":
            t, _, rx = arg.partition(" ")
            pat = re.compile(rx.encode(), re.S)
            end = time.time() + float(t)
            m = pat.search(buf)
            while not m and time.time() < end and not closed:
                read_for(min(0.5, max(end - time.time(), 0)))
                m = pat.search(buf)
            if not m:
                sys.stdout.write("TIMEOUT waiting %ss for %r%s; output since the last match:\n%s\n"
                                 % (t, rx, " (the console closed)" if closed else "",
                                    buf.decode("utf-8", "replace")[-2000:]))
                rc = 1
                break
            buf = buf[m.end():]
        elif op == "send":
            txt = (arg.replace("\\\\", "\0").replace("\\r", "\r").replace("\\n", "\n")
                   .replace("\\t", "\t").replace("\0", "\\"))
            send(txt.encode(), arg)
        elif op == "sendfile":
            with open(arg) as f:
                line = f.readline().rstrip("\n")
            send((line + "\r").encode(), "<file %s>" % os.path.basename(arg))
        elif op == "sleep":
            read_for(float(arg))
        else:
            sys.stdout.write("unknown step: %s\n" % s)
            rc = 2
            break
    read_for(1.0)
    sock.close()
    return rc


sys.exit(main())
