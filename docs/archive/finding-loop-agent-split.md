# Finding: the loop and agent split as a result of the stripping method

**Status:** ARCHIVED 2026-08-23. Its forcing argument - that enforcement by
construction requires a tool result constructible from exactly one source - is stated
in `weaver-agents-PRD` and `weaver-harness-Spec` in their own words, and their
citations to this document were dropped on that date so the governance documents carry
it rather than lean on it. Kept because this is where the argument was made.

Previously: finding, v0.1, 2026-08-11. Architecture-seat material, outside the
document set, landed by the authoring seat as it arrived. This records how the
reasoning-loop boundary was arrived at and what the corpus reversal it forces does
and does not claim. It is a companion to `reasoning-loop-boundary` and it does not
state the boundary. The boundary document says where the line is. This says how the
line was found, and it exists because the finding will otherwise be read as a design
intent held all along.

**Document ID:** `finding-loop-agent-split`
**Editorial:** Per the Working Rules.

## What was planned

One thing was planned and pre-registered. WeaverTools would be reimplemented by
removal, stripping out everything not required until what remained was the bare
minimum. That is a method, and it was stated before it was run.

The method makes no prediction about what is left when it finishes. It says only
that whatever survives survives because it could not be removed. Nothing in it
anticipates the shape of the residue.

## What was not planned

Where the removal bottomed out was not planned and could not have been. The
provisional code is out, the minimum stands, the SPU has been tested successfully,
and the work has circled back into the harness to look at the rest of the agent's
internals. At that point a seam appeared that nobody drew. What remains divides,
and the division is between the reasoning loop and everything else the agent is.

This is a natural split point in the development of the architecture rather than a
milestone that was on a plan. The methodology did not predict it. What the
methodology did was leave room for it, because a method whose entire job is
removing the unrequired does not need to know the shape of what remains, it needs
to not flinch when the shape turns out to be a division rather than a smaller copy
of the same thing. Unplanned, and accounted for.

## The claim being made, and the one being refused

The claim is that a boundary found by a pre-registered removal method is
established convergence. The stripping kept cutting down to the same seam without
being aimed at it, and a seam that survives that is worth more than a seam that was
designed and then found reasons to like.

The claim being refused is any retroactive design intent. Reading a two-level
architecture back into the earlier work as though it had been the plan would be
recruited convergence (Bucy, 2026), which is the named failure mode this project
watches for, and it would poison the result it pretends to support. The two-level
description is post hoc analysis and is labeled as such here. The corpus should
carry it in the tense it happened in.

## The working stance that was in force during the stripping

Through the removal, the agent was treated as though it were the reasoning loop and
nothing else. That was not a mechanism and it did not produce the boundary. The
stripping produced the boundary. The stance was a discipline held during the
stripping, and it was useful for a specific reason. It denied any candidate the
category of part of the agent, just elsewhere, which is a place to file things that
have not justified themselves. With no outside available, every candidate had to
prove it belonged in the loop or leave the program entirely. That pressure is what
drove the removal down far enough to expose the seam.

The stance stops being useful at the moment the seam appears, because from then on
there is a real outside and the denial is denying something true. It comes down as
scaffolding rather than being corrected as an error.

## What this forces in the corpus

The bash shell is the first case and it lands as a named reversal rather than as a
clarification. The standing rule sorted tools by port as discriminator, holding
that a shell provisioned onto the agent's machine is invoked rather than listened
to, so it is internal. That rule was answering the question that existed at the
time, which is whether a thing is internal to the agent. The loop boundary asks a
different question, which is whether a thing is internal to the reasoning loop.
Both answers are correct about the shell. It is internal to the agent and external
to the loop. Under a one-level model those two could not be told apart, and now
they can.

So the move is a refiling rather than an eviction. Nothing is being removed from
the agent. What changes is which boundary the shell is filed against, and the
change became available only when the second boundary appeared.

One thing makes the reversal necessary now rather than optional. Compile-time
enforcement of the loop boundary requires that a tool result be constructible from
exactly one source. A model in which some tools implement the trait directly inside
the harness leaves a legitimate in-loop construction path, and a path that must
stay open cannot be closed by the type system. Enforcement by construction and the
two-category model are incompatible, so choosing the first retires the second. That
is the rationale the reversal note should carry, because it is what makes this a
forced consequence rather than a change of taste.

## The discipline that replaces the denial

Part of the agent, outside the loop is now a legitimate category, and it inherits
the risk the old denial was suppressing. It will be tempting to file unresolved
questions there. The replacement discipline is that outside the loop still has to
earn its place in the agent, it just earns it against a different bar than loop
membership, and durable substrate-resident identity state written through admin is
the first clear tenant. The outer region is not a junk drawer and center-first
means built-first, never matters-more.

## What this does not decide

Whether the gate spawns and supervises tool processes or whether tools are
provisioned as peers that connect to it, the shape of the tool trait after the
reversal, and the audit of `weaver-types` and `weaver-traits` all belong to the
seat with codebase context. This finding records the arrival and the reversal it
forces. It rules on neither mechanism.
