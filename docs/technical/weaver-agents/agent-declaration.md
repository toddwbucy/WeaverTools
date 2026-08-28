---
title: the agent declaration
summary: agent.yaml, field by field - what the operator declares, what refuses, and what this build leaves tunable
version: v0.1
date: 2026-08-25
commit: unreleased
parent: WeaverTools Technical Documentation
---

# The agent declaration

**Status:** technical documentation. Describes, decides nothing.

**Rough draft, first pass**, written the day the first from-scratch agent on a
second box was declared, refused twice by name, corrected from the refusals
alone, and loaded.

One file per agent, YAML, kebab-case keys, owned by the operator and read by
admin at `validate` and `load` from the directory admin's own config names
(`agent-config-directory`). **Nothing defaults.** An absent required field
refuses the parse at any depth, because a default would be the program
finishing a declaration the operator did not, and an unknown key refuses too,
because a misspelled field that silently vanishes is a declaration that lies.
The named exceptions - fields whose absence a document rules the meaning of -
are called out below where they occur.

## The surface

```yaml
session: s-karl-1
spu-instruction:
  decoder:
    model-binding:
      artifact: /opt/weaver/models/qwen2.5-0.5b-instruct-q6_k.gguf
      devices: [0]
    residual-readout-election: false
    identity:
      - role: system
        content:
          - type: text
            text: |-
              You are Karl, a small local agent running on this laptop.
    tunable-values:
      seed: 451234785645
      context-capacity: 8192
      max-tokens-per-turn: 1024
tool-set: []
permission-mode: ask
gate-instruction:
  access-rule:
    allowed-uids: [1000]
    allowed-gids: []
    denied-uids: []
trace-sink:
  kind: file
  path: /home/todd/.weaveragents/karl/trace.ndjson
  create: true
```

**`session`** - the session's identifier, carried on every trace event the run
emits.

**`spu-instruction.decoder.model-binding`** - the artifact path and the
ordered device list, the order being the shard order. An empty device list
refuses the parse: a binding assigning no device is a declaration the operator
did not finish, and defaulting it to device zero would be a placement decision
the program refuses to make. Whether the devices exist and can shard is judged
at admission, not at parse. The SPU refuses a conflicted device and never
evicts, so two declarations naming one device serve one loaded agent at a
time.

**`spu-instruction.decoder.residual-readout-election`** - whether the load
raises the per-layer residual readout. A diagnostic election, `false` for a
plain serving agent.

**`spu-instruction.decoder.identity`** - the canonical messages the identity
prefix renders from. Configuration rather than history: an empty list is a
declaration the operator made, where an absent field is a file unfinished.
Worth writing with care - an identity that advertises tools the `tool-set`
does not grant is a lie told to the model every turn.

**Every message here carries `role: system`, and any other role refuses the
load** with `bad value: identity.<n>.role`. The identity door writes
`message.system` and refuses the rest, so a prefix under another role would
be seated into the model and left out of the record. Declarations written
before 2026-08-28 carry `role: user` and need the one-word change. A family
whose own template names no system turn, gemma and mistral among them, folds
the prefix into its first user turn when rendering, so the role is what the
operator writes everywhere and the shape is the family's.

**`spu-instruction.decoder.tunable-values`** - the knob map, its own section
below.

**`spu-instruction.decoder.field-election`** and **`surprisal-election`** -
further diagnostic elections, each standing alone by design. The first two
elect an observation into existence and are absent by ordinary posture. The
surprisal election runs the other way: `false`, the default, means the
per-position vector is not produced and the generation's perplexity stands in
its place.

**`spu-instruction.classify`** - optional. Present, it binds the classifier
role's model at the smaller size. Absent, the operator declares the agent
runs no classifier.

**`tool-set`** - the granted tools by name. Empty is toolless, the current
deliverable's shape.

**`permission-mode`** - `ask`, `allow`, or `deny`.

**`binding-kind`** - optional, and absence means `serving`, per the ruling
that a declaration written before the diagnostic member existed declares what
it always meant. A `serving` binding requires `gate-instruction` and a
`diagnostic` binding excludes it - the one cross-field rule, checked by admin
at inventory rather than by the parse.

**`gate-instruction.access-rule`** - who may pass the gate, judged by
`SO_PEERCRED` at connect: `allowed-uids`, `allowed-gids`, `denied-uids`.
**The socket's path is deliberately not here.** Where the door stands is the
program's (inside the unit's runtime directory, `/run/weaver-<agent>/`), and
only who may pass is the operator's.

**`trace-sink`** - where the record leaves the program. Three kinds. `file`
opens append-only, creating when `create` is true. `pipe` creates the FIFO
when asked, then opens nonblocking so a reader-less pipe refuses the load
loudly instead of hanging it. `socket` connects to a listener of the
operator's that must already stand. The sink's directory must deny the agent
uid traversal and be owned by root or the admin principal - denial and
custody, two halves, neither implying the other - because the record's
custody is admin's by charter and the agent must not be able to reach the
file that testifies about it.

**`state-election`** - optional, the tee's election, absence meaning the
default election per the state charter.

**`loop-file`** - optional, the agent's own loop, absence meaning the
worker's default loop.

## Tunable values, and the disposition mechanism

Which parameters a declaration may set is **not a property of the format. It
is a property of the binary**: every knob in the SPU carries a compiled
disposition, `Frozen(value)` or `OperatorTunable`, and the set moves only
with a recompile. The map in the declaration is keyed by name so that no
floor type has to enumerate a set that is some deployment's to elect.

The rules of the mechanism, each the source of a refusal met in practice:

- A name the binary froze is **ignored where it appears** - frozen means
  compiled in and never carried, so a declaration cannot move what the
  deployment locked.
- A name the binary left tunable **must be supplied**. The load refuses by
  name - `organ_refused`, `config_invalid`, `tunable-values.<name>` - which
  makes the refusal the discovery protocol: load, read the name, supply it,
  load again. Karl's first two loads were exactly this loop.
- A non-finite value refuses at parse, before any organ runs.

This deployment's build (2026-08-25) leaves three names tunable:

| name | meaning |
|---|---|
| `seed` | The base the per-generation seed derives from. Each generation's stream is fixed by the declared seed, the turn's reference, and the generation's index, so no two draws share a stream and a rerun of a turn reproduces one. |
| `context-capacity` | The session's context size in tokens, sized by the operator against the KV cache's device footprint. Unfroze on the ruling of 2026-08-19 (#221), 32768 the ruled starting point on the primary deployment. |
| `max-tokens-per-turn` | The per-turn generation ceiling, the stop condition's backstop for a model that never emits a stop token. Unfroze the same day (#218) after the frozen 512 met its first real code answer. |

The sampling surface stands frozen beside them: temperature 0.7, top-k 40,
top-p 0.95, repetition-penalty 1.1, repetition-window 64. Frozen on purpose -
a distribution gathered while the sampler varied would report the sampler
rather than the task. A deployment iterating on an agent flips the knobs it
is moving and freezes them back for production.

**The table above is a fact about one build.** The authoritative list for
the binary in front of you is the one its refusals name, and reading a
refusal costs one failed load.

## What refuses where

Three layers judge a declaration, in order, and each refusal is typed:

1. **Parse** (weaver-types): unknown key, missing required field, empty
   device list, non-finite tunable value, malformed trace-sink surface.
2. **Inventory** (admin, at validate and load): the agent resolves in the
   account database and its home exists, the artifact is readable, the sink
   exists or is creatable, the sink directory's denial and custody both
   hold, the allow-list carries the name, the cross-field binding rule.
3. **The organs** (at load): each may refuse its own instruction, the SPU's
   unsupplied-knob refusal being the one an operator meets first.

A refused load leaves a record: the run's trace carries `load`, the typed
`refusal`, and `unload`, so the box's history of almost-agents is readable
after the fact from the sink alone.
