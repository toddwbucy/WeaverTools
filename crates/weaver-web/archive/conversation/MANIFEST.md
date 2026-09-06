# Archive: the conversation half of weaver-web

**Status:** ARCHIVE, frozen 2026-09-06 on the operator's ruling. **These files
are a copy and not the build.** The crate still compiles the originals at
`crates/weaver-web/src/`, and removing them from the build is its own act,
held for the reason section 3 gives.

**Why they are here.** `weaver-web-PRD` and `weaver-web-Spec` were rewritten
whole on 2026-09-04 for the instrument, retiring the charter these files were
written to, per `inventory-weaver-web-code`. The operator ruled on 2026-09-06
that the conversation half is archived rather than deleted or kept, so that
whoever writes the chat interface the vision still names can reach it without
reading a diff.

**Frozen from `main` at `78aa739`.** Git is the archive and this is a
convenience copy, so **the commit is the authority and these bytes are the
reading aid.** Verify before trusting:

```text
cd crates/weaver-web/archive/conversation && sha256sum -c MANIFEST.sha256
```

## 1. What is here

| file | lines | what it was |
|---|---|---|
| `src/user.rs` | 429 | the gate surface: channels, messages, members, the session open |
| `src/router.rs` | 212 | mention parsing and multi-agent invocation routing |
| `src/channel.rs` | 130 | channel reads, log pages, and the projections the templates render |
| `src/registry.rs` | 111 | participants and providers, reconciled from the link's hello |
| `templates/` | 5 files | `channel`, `channels`, `sidebar`, `name`, `event` |

The schema they ran on is `crates/weaver-web/migrations/0001_init.sql` and
`0002_roles.sql`, which are not copied here because they are still the
crate's own and their drop is the same held act.

## 2. What they were written to

**The Spec at `13b8a6a`, 2026-08-25**, the last commit before the rewrite,
where section 12 is the trace view, 13 the HTTP surface, 14 sessions and
roles, 15 the open elections, 16 the link and 17 the confirm view. Every
module's citations resolve there and nowhere in the working tree.

## 3. Why the build still holds them

**The two halves share an identity model**, which the inventory did not say
and trying to lift them out is what found it.

`registry.rs` carries `Participant` with its `role` field and its
`is_admin()`, and `web/admin.rs` gates on it. **That is the role model the
rewritten charter's section 6 keeps as structural**, where identity and
authentication are deferred with a named trigger. So `registry.rs` is not
retiring whole: the participant as a conversation member goes and the role
as a standing fact stays, and separating them is the section 6 act rather
than this one.

`channel.rs` is reached by `queue.rs` and `web/mod.rs`, both of which the
inventory holds open, so it cannot leave the build before they are ruled.

**Nine call sites cross the boundary** from modules that do not retire:
two in `bin/weaver-web.rs` to `registry`, three in `queue.rs` to `router`
and `channel`, one in `web/admin.rs` to `registry::Participant`, and three
in `web/mod.rs` to both. Each is small. None is safe to cut before section
6 is answered.
