# WeaverTools

A local-first agent framework whose primary artifact is the trace: one agent, one
machine, every organ behind its own socket, and one record of everything that
crossed between them.

## Purpose

We built WeaverTools to answer a practical frustration. Our agents do not behave
the way we need them to, and when they fail we cannot see why. The failure is not
the model's alone and it is not the scaffolding's alone. It happens somewhere in
the traffic between them, and that traffic is exactly what conventional setups
leave unobservable. So we built a framework whose first commitment is visibility
into that traffic, on one machine, where nothing crosses a boundary we cannot
watch and nothing varies that we did not set.

The unit of concern is the agent, not the model. The model is one organ among
several, and how an agent meets the world is a property of the whole assembly,
the memory it carries, the tools it reaches for, the loop that sequences its
work, not of the decoder in isolation. WeaverTools makes the agent central by
giving every organ its own place and connecting them through a single
content-neutral switchboard, the harness, which holds the sockets, routes
between organs, and holds no opinion about the content it routes. That is the
whole architecture, a 1920s operator's board, and every organ, including the
memory we build ourselves, plugs into it as an equal caller. If you have a
better idea for how memory should work, connect it. The board does not care
whose voice is on the line.

Making the agent central does not mean giving up sight of the model. It means
the opposite. To get the agent behavior we want, we have to understand why the
model emits what it does and why it fails, and that demands real insight into
the model's internals, captured locally, at conditions fixed before the run. The
architecture keeps that instrument where physics already puts it, inside the
organ that holds the model, while keeping the agent as the thing under study.
Local, legible, and honest about the difference between the model and the agent
it serves.

## Why not the network?

Every seam that crosses a process line in WeaverTools is a Unix domain socket on
a single machine. The first question any engineer asks is why not localhost, and
the answer has three parts, each of which stands on its own.

Latency. We cop to one piece of theory here: latency is the enemy of agency. An
agent routes through the harness on every exchange, and any per-hop cost
compounds directly into the loop, per token, at batch one. Loopback still pays
the TCP stack, the kernel network path, and serialization on every one of those
hops. Unix domain sockets collapse that cost to nearly nothing while keeping the
same topology. The agent that pauses is the agent that behaves worse, so the
transport with the least overhead is not an optimization, it is a behavioral
requirement.

Security. A Unix socket inherits the operating system's trust model:
SO_PEERCRED, filesystem permissions, kernel-enforced identity. Localhost
inherits the network's, which means adding authentication above a surface that
is, by construction, reachable as a network port. We went OS-level precisely so
that trust belongs to the kernel and no organ exposes a network surface to
defend. Loopback would reintroduce the exact attack surface the architecture was
shaped to eliminate.

Measurement. We wanted network variability out of the equation completely, and
this stands by itself on engineering grounds. Even on localhost you are adding
compute and process that must be accounted for when you are trying to isolate
one slice of behavior. When the object of study is a stochastic engine, every
component between you and it is noise in your instrument. The simpler the
substrate, the cleaner the reading.

We give things up for this. No fungibility across machines, no distributed
uptime, no direct network applicability of the deployed whole. We take that
trade deliberately, because what remains applicable to the network is not the
whole system, it is the individual organs and their interactions, observed with
the network out of the way. The topology is network-shaped on purpose: a hub and
spoke of content-neutral duplex channels is the same architecture a network
uses, moved onto a quieter substrate. Work proven at this level carries upward
because nothing was ever coded to the substrate, only to the topology. The seam
that runs over a socket today can run over a wire tomorrow, and moving one seam
while pinning everything else is exactly the controlled comparison this
framework exists to make possible.

## The trace is the artifact

The deliverable is a deployable proto-stateful agent that completes a turn end
to end against a real local model and emits a clean, turn-bracketed,
correctly-custodied trace. The trace is not a log kept beside the work. It is
the substrate the organs coordinate over: every component reports what it did,
the harness authors those reports into one record, and the record leaves the
program as a single ordered stream, one event per line, in the same canonical
form the agent itself reasons over. Correctly custodied means the agent cannot
reach its own record. The stream lands behind a boundary the kernel enforces,
so the account of what an agent did is never something that agent could edit.

The record is built to be replayed. Because it holds the sampler's actual
tokens rather than a seed, a recorded scenario can be fed back through the
forward pass with nothing re-sampled, and when deeper visibility is needed an
agent can be reloaded with residual readout enabled by a change to its
configuration alone, per-layer activations reduced in place at production time.
Observation is a deployment decision, not a rebuild.

## What is here today

This repository runs a strict order of work: documents first, then a knowledge
graph built from those documents, then code. It is in the first of those phases
now, so what `main` holds is the document corpus: a charter per crate, a
contract for every seam, the Specs arriving crate by crate, the workflow
documents that read the set back as one motion, and the process documents the
corpus governs itself by. No code
is written until the document set is ratified, and ratification is mechanical:
the documents carry their nodes and edges in a fixed notation, and the graph is
generated from them rather than maintained beside them.

## Reading the corpus

- `process/WeaverTools-Working-Process.md` is the boot prompt: who is primary,
  in what order the work moves, and where it currently sits.
- `docs/project/WeaverTools-PRD.md` is the apex: the deliverable, the four
  invariants, the lifecycle, and the enforcement posture every other document
  answers to.
- `docs/project/weaver-tools-vision.md` is the longer line the program is on.
- `docs/crates/` holds one directory per crate, each carrying that crate's PRD
  and Spec, with the contracts under `docs/crates/contracts/` and the workflow
  documents under `docs/crates/weaver-harness/Loops/`.
