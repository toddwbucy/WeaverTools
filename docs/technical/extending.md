---
title: Extending the program
summary: the operator surfaces above the loop - the floor, the contracts, and a new organ - and the ritual that governs all three
version: v0.1
date: 2026-08-23
commit: unreleased
parent: WeaverTools Technical Documentation
---

# Extending the program

**Status:** technical documentation. Describes, decides nothing.

**Rough draft, first pass.**

The code described here is unreleased and is scheduled for release in the first
quarter of 2027.

[The loop](loop.md) is the surface an operator writes against every turn. It is not
the only one. When what you need is a shape the program does not yet have - a new
vocabulary on a seam, a tool the trait set does not describe, an organ that is not
here - you are past the springs and changing what is on the board.

**This page is about that, and about the price the program charges for it.** The
price is deliberate and it is not high, but it is exact.

## The four surfaces, cheapest first

| surface | what it changes | when it takes effect | what it costs |
|---|---|---|---|
| the agent's declaration | the model binding, the elections, the operator-tunable knobs | the next load | edit a file |
| [the loop](loop.md) | what the agent does with a turn | the next crossing, under the Python surface | edit a file |
| the composition root | which knobs are frozen and which are the operator's | the next build | a rebuild |
| **the floor and its contracts** | what shapes exist to be sent at all | the next build | **the ritual, below** |

The first three are yours alone and cost nothing but your own time. The fourth
touches everything, and that is the one this page is about.

## The floor is two crates and it is deliberately thin

`weaver-types` holds the agent's declaration, peer identity, and the wire
vocabulary the socket contracts draw. `weaver-traits` holds the message model,
roles, permission modes, and the tool surface.

Every domain draws from both and no domain contains either. They are linked rather
than dialed, because **a type definition cannot be sent over a socket** - it has to
be present at compile time on both sides of every seam that names it.

**Thin is the point rather than an accident.** The value of the floor is that it is
small enough to audit. A floor that accumulates becomes a place to put things
rather than a place that means something.

## The ritual

The floor does not grow when someone thinks a shape might be useful. **It grows
when a contract draws it, and it grows in one act.**

> A crate PRD added or changed updates the floor in the same act.
>
> A floor change updates every affected PRD and contract in one act, and **a change
> that cannot be carried in one act is a change that has not been thought
> through.**

That last clause is the whole test. If you cannot see every consumer your new shape
touches well enough to update them together, you do not yet understand the change
you are making, and the program would rather you found that out before the shape
exists than after every crate has linked it.

## Four terms bind anyone who links the floor

**Declare what you use.** Every contract names what it draws from the floor in a
vocabulary clause, and **the clause is present even when the answer is nothing.**
The union of those clauses is the floor's required surface. A definition no clause
names is unused. A definition a clause names and the floor lacks is a gap. The
clause is mandatory because that check is what it buys.

**The declared vocabulary is the only vocabulary.** A consumer does not invent a
parallel definition of something the floor defines, and does not reach around the
contract to a shape of its own. The previous tree is the cautionary case: its
attribute vocabulary was a convention emitters could ignore rather than a boundary
they had to cross, and the declared names ended up a strict subset of the emitted
ones.

**A change here is loud by design.** Everything links the floor, so a change ripples
to every consumer. **That is correct rather than unfortunate.** A quiet floor change
is one whose blast radius nobody measured.

**Nothing enters in advance.** Everything in the wire vocabulary arrived because a
written contract drew it, never before. A definition arriving with no contract
behind it is a reserved slot, which is **a shape carried for a reader that does not
exist**, and the program forbids it in schema form as anywhere else. A definition no
contract draws leaves the floor, and the vocabulary gate tests exactly that.

## Naming, if you are adding wire vocabulary

**Wire vocabulary is named for the loop whose traffic it carries**, not for a sender
and not for a seam. The channels have two initiators, so direction is a fact about a
loop's walk rather than a property a name can carry, and loops are unique where
senders are not. An earlier sender convention was retired when it collided on the
harness.

## Adding an organ

An organ is a crate that governs a domain and holds a two-initiator channel with
the harness. Adding one is smaller than it sounds, because of how the hub is
arranged.

Every organ presents its contracts to the harness and nothing to any other organ,
so a conflict between two organs is settled in the contracts each holds with the
harness rather than between them. **There are no lateral edges.** So **adding an
organ is a socket and a contract onto the hub rather than surgery on everything
already standing.**

What that costs in practice: a socket, a contract naming what crosses it, whatever
vocabulary that contract draws from the floor, and the ritual to carry the floor
change. What it does not cost is any change to the organs already there.

## What this page does not yet carry

- **A worked example.** The most useful version of this page walks one real
  extension end to end - a contract drawing a new shape, the floor edit beside it,
  and the consumers updated in the same act. That is owed.
- **The extension seam and the working-list socket** are the two builder-facing
  surfaces for loops rather than for the floor, and they are named in
  [the loop](loop.md) rather than described in either place.
- **The graph's part in this.** Documents carry their nodes and edges in a fixed
  notation and the knowledge graph is generated from them, so a floor change is a
  graph change too. What a builder owes the graph is not written here.
