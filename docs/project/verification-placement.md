# Verification placement

**Status:** v0.1, 2026-08-11. Architecture-seat material, outside the document set,
drafted by the authoring seat from the operator's ruling of this date and landed in
the same act as its companions. `reasoning-loop-boundary` section 7 cites this
document for the admin face, and the IS NOT entry on verification points here.

**Document ID:** `verification-placement`
**Companions:** `reasoning-loop-boundary`, `reasoning-loop-is-and-is-not`.
**Editorial:** Per the Working Rules.

## Verification is external to the loop, on latency grounds alone

Verification is one of the tasks that would interfere with the latency of the
reasoning loop, and that is the whole reason it is removed from the loop. The
placement is not a statement about verification's importance. It is the criterion's
cost model applied: a check on the hot path is paid on every event, and the loop's
named purpose is processing meaning, not auditing it.

What holds instead is split by when it runs. At author time, the vocabulary guard:
the harness authors every event and enforces the kind-to-payload pairing and the
payload's conformance at the one submit call, which is where the splice's opacity
is paid for. After emission, the observability consumer: the record leaves through
admin's egress and faithfulness is checked where the record lands, outside the
process whose account it is.

## Corrective mechanisms are allowed for, and none is built

This placement does not preclude admin-level sockets in `weaver-admin` for the
express purpose of performing verification, injecting remedial context, or
establishing other corrective mechanisms. What it rules is that those mechanisms
and the monitoring for them are external and secondary to the reasoning loop's
named purpose and do not belong with it.

One consistency fact binds any future corrective mechanism. The coordination seam's
merged contract guarantees that no directive carries work of any kind, so a
corrective-context path cannot ride the existing admin seam. It arrives as a new
socket under a new contract, which is the feature-add pattern the corpus already
mandates, a schema extension plus a new socket and contract, never a retrofit.
Nothing is reserved for it meanwhile: no slot, no field, no dormant party.

## The security model, stated once

The security model this placement rests on is role-scoped socket handoff. Each
Unix socket descriptor handed across a boundary, from the gate and from admin
alike, exists for an express purpose with allowances based on role, authenticated
by kernel credential where the channel has a name and by descriptor possession
where it does not, per the apex's invariant. Possession authenticates only
because the handoffs are controlled, and the precondition is stated rather than
assumed: every pair is created with close-on-exec in the act that makes it, each
party receives exactly its own ends, and an organ counts its descriptors at
entry and refuses to serve when the count is wrong, so an unrelated descriptor
cannot stand in for a handoff. The contracts bind this per seam, the gate
receiving only its own end and the unnamed channels carrying the same hygiene.
Trust above a network protocol is not
part of the model because the network is not part of the agent: the agent's edge
is the network boundary, and what crosses that line is outside the agent entirely.
