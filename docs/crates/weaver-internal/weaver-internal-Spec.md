# weaver-internal - Spec

**Status:** DRAFT, in review. Cut 2026-08-18 against the ratified charter of the
same date. Ratification is the operator's act on the review's close, and code is
written against this document only after it.

**Date filed:** 2026-08-18
**Document ID:** `weaver-internal-Spec`
**Parent:** `weaver-internal-PRD`
**Editorial:** Per the Working Rules.

---

## 0. What this document is

The representation and enforcement decisions for `weaver-internal`, read beside its
charter. The charter says what the crate is - the operator's promotion space for
loop-reachable callables dispatched inward - and this document says what shape the
first member takes, what a member may and may not reach, and which instrument holds
each claim. What an operator later promotes into the space is specified by the
operator at promotion, against the charter's terms, and this document binds the
framework's own members alone.

## 1. The crate

One library target and nothing else: no binary, because a member holds no process,
and no socket bound anywhere in the crate, because a member that listened would
have an inbound seam the charter forbids. **The dependency set is empty.** The
first member computes over its arguments with the standard library alone, and an
empty set is the manifest form of the charter's pure bar: a crate that cannot name
a filesystem, network, or clock crate cannot reach one by dependency. A member
that needs a dependency is arguing for a promotion-space entry with operator-owned
risk, and that argument happens in an act, not in a manifest edit.

**Edition and toolchain.** Edition 2024 on the pinned nightly, no nightly feature
gates, per the workspace's standing terms.

```graph
node: internal-no-dependencies
kind: assertion
tag: manifest

edge: asserts
from: weaver-internal
to: internal-no-dependencies

node: internal-one-library-target
kind: assertion
tag: manifest

edge: asserts
from: weaver-internal
to: internal-one-library-target
```

`internal-one-library-target` is the target-shape half of the same fact: the
manifest declares exactly one library target and no target of any other kind,
which is the shape the first sentence of this section states and the
instrument `weaver-spu`'s one-binary claim already uses.

## 2. The member surface, and what the agent never sees

**The agent's innate tool surface is the shell, and this crate is not on it.** Per
the operator's ruling of 2026-08-18: the shell is the source of every future tool
and the only tool the agent innately holds, so no registry distinguishes internal
from external tooling and no member of this crate is advertised to the model. A
member carries no schema, no name the model calls, and no entry in any prompt
assembly. The elected inward corner of `weaver-tools-vision` section 6 is served
by the loop recognizing substitutable work in the stream and answering
deterministically, never by the model addressing a member as a tool it knows.

Until the charter's calling-surface cell settles, a member's surface is the
interim minimum the relocation needs: one public pure function per member, value
in, answer out, on the crate's library surface. For the calculator that is a
function from an expression string to either a rendered value or a refusal in the
member's own words. The exact Rust signature is the builder's within that shape,
and the surface cell's settlement may wrap this function without changing it.

```graph
node: internal-no-advertisement
kind: assertion
tag: review

edge: asserts
from: weaver-internal
to: internal-no-advertisement

node: internal-member-initiates-nothing
kind: assertion
tag: review

edge: asserts
from: weaver-internal
to: internal-member-initiates-nothing
```

`internal-member-initiates-nothing` reads as an absence review holds: no thread
spawned, no process forked, no socket dialed or bound, anywhere in the crate. A
member's whole surface is call in, answer out, per charter section 4.

## 3. The calculator's grammar

A scientific expression grammar, stated here so the member's answers are a
specified behavior rather than an implementation's habit:

- The operators `+ - * / ^` over floating point, with parentheses, and whitespace
  carrying nothing.
- `^` binds tighter than `*` and `/` and associates right, so `2^3^2` is
  `2^(3^2)`.
- Unary minus binds looser than `^`, so `-2^2` is `-(2^2)`, and an exponent may
  carry its own sign, so `2^-3` holds. These are the conventions a scientific
  reader expects and they are asserted, not assumed.
- The one-argument functions `sin cos tan asin acos atan sqrt ln log exp abs`,
  angles in radians, `log` base ten, and the constants `pi` and `e`. A call is
  the function's name followed by a parenthesized argument, names are lowercase
  and case-sensitive, and there is no implicit multiplication: `2pi` refuses at
  the `p` rather than guessing a product.
- A numeric literal is decimal digits with an optional fractional part. No
  scientific notation, no separators, and a sign is the unary minus of the
  grammar rather than part of the literal.
- **A position is a character offset into the expression with whitespace
  removed**, zero-based, because the scan runs over that form. Stated so two
  implementations cannot count the same defect differently.
- **The rendered value is the shortest decimal that round-trips the answer**,
  an integer-valued answer rendering with no fractional part, so `2^10`
  answers `1024` and never `1024.0`.

**Every refusal is the member's own words and names its position.** An unknown
function or name refuses naming itself and where it sits. A square root of a
negative number, a logarithm of a non-positive number, a division by zero, and a
result that is not finite each refuse by naming the defect. The words are content
a model reasons over at whatever seat the answer lands in, so a refusal that only
an implementer could read is a defect against this section.

**The recursion is depth-bounded at 64.** The expression arrives from outside
the member's control, so its nesting is nobody's promise, and the descent
refuses past the bound in the member's own words, naming the bound, rather than
overflowing the caller's stack. The root opens at depth zero and every
construct that recurses increments it: a parenthesized subexpression, a
function's argument, and the power level's two self-recursions, the unary
minus and the exponent - a minus chain or an exponent chain reaches no other
level on the way down, so the bound holds at every level that recurses into
itself, the power level's entry included.

```graph
node: internal-calculator-power-conventions
kind: assertion
tag: perturbation

edge: asserts
from: weaver-internal
to: internal-calculator-power-conventions

node: internal-calculator-refuses-in-its-own-words
kind: assertion
tag: perturbation

edge: asserts
from: weaver-internal
to: internal-calculator-refuses-in-its-own-words

node: internal-calculator-depth-bounded
kind: assertion
tag: perturbation

edge: asserts
from: weaver-internal
to: internal-calculator-depth-bounded
```

## 4. Purity, and what it buys

A framework member is a function of its arguments alone: no filesystem, no
network, no clock, no randomness, and no state held between calls, so one
expression answers one value on the deployment that ran it - the same binary
on the same host always answers the same. **The claim is deterministic per
deployment and not bit-identical per universe**: the four arithmetic
operations follow IEEE 754 and carry across platforms, while `^` and the
function set follow the platform's math library, whose last digit may differ
between hosts. That is the honest form of what the harness-SPU splice
amendment prices: the trace re-derives what the model saw on the deployment
that recorded it, which is the deployment the splice runs on, so per-binary
determinism is the property the mechanism needs and the property claimed.
Purity is why the framework holds its own members to a bar it does not impose
on the operator's promotions. The manifest assertion of section 1 is the mechanical
half, and the review half is that no ambient authority of the standard library -
environment, time, entropy, paths - is reached either.

```graph
node: internal-member-pure-function
kind: assertion
tag: review

edge: asserts
from: weaver-internal
to: internal-member-pure-function
```

## 5. What is enforced, and by which instrument

Eight assertions. Two are the manifest instrument's: the resolved dependency
set of the crate is empty, checked by a test reading the lockfile's view of
this package, and the manifest declares exactly one library target and no
other kind. The three perturbation claims are bought by
tests that fail when the property is removed: the power conventions fail when the
minus level or the associativity is moved, the own-words refusals fail when a
refusal is replaced by a bare error, and the depth bound fails when the entry
check at any self-recursing level is dropped. `internal-no-advertisement`,
`internal-member-initiates-nothing`, and `internal-member-pure-function` are
review's, each an absence a reviewer confirms against the whole crate, and each
worded so the inverse overclaim is refused: review here means the instrument is
the reviewer, not that no instrument could exist.

## 6. Open elections

- **The calling surface**, the charter's PENDING cell. Direct call for pure
  members and the Unix socket handoff for promoted ones are the named arms, and
  the cell settles when the harness-SPU splice amendment lands with the first
  caller. Nothing here builds against either arm beyond the interim function of
  section 2.
- **The refusal's Rust shape.** Whether the member's refusal is a bare string or
  a small struct carrying the position as a field is the builder's until the
  splice amendment says what the clerked event wants to hold.
- **Member naming at the surface.** How a control loop names a member when the
  surface cell settles - a function path at link time or a name over the socket -
  rides that cell.
