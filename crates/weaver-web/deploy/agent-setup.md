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
   The gate socket lands `srwxrwx--- weaver-<name>:weaver-<name>` and its
   runtime directory `/run/weaver-<name>` lands `0750`, so reaching the
   socket needs group execute on the directory and connecting needs group
   write on the socket. Both are the agent's group, so this step is what
   grants them.

   **This is no longer only a convenience.** Since 2026-08-28 the mode is
   the boundary's own election rather than whatever umask the process
   inherited, and `weaver-admin validate` refuses a declaration whose
   `allowed-uids` name a uid outside this group - naming the field, before
   any unit starts. So a missed membership shows up as a typed refusal at
   load rather than as `Permission denied` at the dial.

   NOTE: group membership is evaluated
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

## The posture before IAM, named where a deployer meets it

The sudoers rule is the one privilege widening this application asks of
the box, and the path to it is shorter than the notes above imply: v1
sessions are anonymous, so anyone who can reach the listener and types
a name from the config's `admins` list holds the admin role, and the
admin role drives these verbs. The trust boundary is the LAN and the
box until the IAM act lands (weaver-web-PRD section 8, item 2), and a
deployer widening `listen` beyond a trusted LAN before that act is
undoing the deployment's one safety assumption.

Two shapes for the rule itself, the wrapper being the exact one:

- **Wrapper (recommended):** install `deploy/weaver-admin-verb`
  root-owned at a root-owned path, edit its two box-fact lines, point
  the sudoers rule at the wrapper with no SETENV, and set the
  connector's `admin_bin` to the wrapper with `admin_env = false`. The
  widening is then exactly three verbs on validated agent names with a
  fixed config.
- **Direct binary:** the rule names the admin binary with wildcarded
  arguments and permits the config variable through a
  command-specific `env_keep` (the shipped fragment's commented
  alternative shows the shape - never a blanket SETENV, which would
  permit every variable), and it is still wider than "narrow"
  suggests. It works, and the wrapper is what makes the description
  exact.
