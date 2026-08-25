# Adding an agent to this box (verified 2026-08-19, bravo)

The whole process, in order. Admin's `validate` refuses with typed
reasons until every step is done, so run it after each step if lost.

1. **Declaration**: `/etc/weaver/agents/<name>.yaml`. Copy an existing
   one; change at least `devices` (GPU index) and `trace-sink.path`
   (`/home/todd/.weaveragents/weaver-<name>/trace.out`).
2. **Allow-list**: add the name to `/etc/weaver/config/allow-list`,
   one name per line. Without this: `{"kind":"no_such_agent"}`.
3. **System user**: `useradd --system --user-group --home-dir
   /home/weaver-<name> --shell /usr/sbin/nologin weaver-<name>`, and
   `mkdir /home/weaver-<name>` owned `weaver-<name>:weaver-<name>`
   mode 2770. Admin verifies the home exists. Without it:
   `{"kind":"boundary_unverified"}`.
4. **Sink directory**: `mkdir /home/todd/.weaveragents/weaver-<name>`,
   owned `root:todd`, mode 2750. Admin verifies both halves of the
   sink boundary: the agent uid cannot traverse to it (denial), and
   root or admin owns it (custody). Wrong ownership or mode:
   `{"kind":"boundary_unverified"}`.
5. **Operator group membership**: `gpasswd -a todd weaver-<name>`.
   The gate socket lands `srwxrwxr-x weaver-<name>:weaver-<name>`, so
   connecting needs group write. NOTE: group membership is evaluated
   at login - weaver-web-connector (the process that dials the
   socket) must be (re)started from a fresh login context
   (`sudo -u todd ...`) after this, or it gets
   "Permission denied (os error 13)" dialing the socket.
6. **Validate, then load**:
   `sudo WEAVER_ADMIN_CONFIG=/etc/weaver/config \
    /usr/local/libexec/weaver/weaver-admin {validate|load} <name>`
7. **connector config**: add an `[[agents]]` block (name, gate socket
   path, trace path) to `/etc/weaver-web/connector.toml` and restart
   weaver-web-connector. The roster reaches the server in the
   connector's hello, and the registry, queue worker, trace view,
   lifecycle row, and channel-create checkbox all follow from it. The
   server needs no change and no restart.

GPU placement is the declaration's `devices` field. The SPU refuses a
conflicted device at admission and never evicts.
