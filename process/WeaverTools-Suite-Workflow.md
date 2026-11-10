# WeaverTools Suite Workflow

**Version:** v0.1, 2026-11-10.

This document says how work moves through the suite's repositories: which seats exist,
how an act becomes a pull request, how it is reviewed and merged, and how an act that
touches more than one repository is ordered. It is the common pattern. Each repository's
`CLAUDE.md` keeps what is its own: its gates, its commands, its boundaries and its
documents.

## 1. Standing

The pattern was run in WeaverAgent and in WeaverWeb as two separate loops until
2026-11-10. On that date the operator moved planning for both to one seat above the
repositories, because the work ahead needs the two ordered together. This document is the
one statement of the loop both then follow.

**It binds a repository from the act that makes that repository's `CLAUDE.md` cite it.**
Until that act merges, the repository's own text governs its tree. An act already in
flight finishes under the form it started in. Nothing here interrupts a stack that is
open when this document lands.

Where this document and a repository's `CLAUDE.md` disagree about seats or the pull
request path, this document governs once cited. On gates, commands and boundaries the
repository's file governs always.

## 2. Seats

Three seats carry the work. The operator closes every loop.

- **The Planner** sits above the repositories, in the suite directory. It plans the acts,
  writes the briefs, grades every draft, handles third-party review, decides when an
  Executor pushes, and reports to the operator. It writes no code in any repository.
- **An Executor** sits inside one repository and works only there. A repository may have
  more than one, each with its own lane and its own worktree. An Executor works on a
  branch, runs the repository's gates, checks its own diff, opens draft pull requests,
  and pushes only on the Planner's word. It reports each round to the Planner before any
  push, and quotes git's own output for every hash.
- **The operator** rules on every Spec gap and every design change, runs every privileged
  step on a box, takes every pull request to `main` out of draft, and merges every pull
  request. Nobody else merges, and nobody pushes to `main`, with the one exception
  below.

**The instruction sets go straight to `main`**, on the operator's word of 2026-11-10.
This document, the other process documents and each `CLAUDE.md` are the seats' own
instructions, and a third-party review of them spends the allowance on nothing a reviewer
can check. The Planner commits a change to one of them to `main` on the operator's word
for that change. Where a repository's branch protection refuses the push, the change is a
pull request the operator merges without a review pass.

A seat that cannot do a thing its lane names hands it to the operator with the exact
command. It does not route the thing through another seat.

## 3. An act is a stack

An act reaches `main` through one pull request, built from smaller ones.

1. **The brief.** The Planner sends the Executor a brief: the goal, what is already ruled
   out, the files and Spec sections to read, the sub-acts and their order, the acceptance,
   and the scope boundary. The brief lands in the repository as the stack's first commit,
   so the record of what was asked stands beside what was built.
2. **The stack.** The Executor branches the stack from `main` as `act/<name>` and opens
   the stack pull request as a draft against `main`.
3. **The sub-acts.** Each sub-act is a branch from the stack's head, opened as a draft
   pull request against the stack branch when its work is done. A sub-act that depends on
   another branches from it and is retargeted when the first merges.
4. **Merging in.** The operator merges each cleared sub-act into the stack, innermost
   first. The Executor rebases the next sub-act on the stack's new head.
5. **The close.** When every sub-act is in, the stack's body is rewritten as one account
   of the whole: what the stack does, every issue it closes, every pull request it holds.
   The operator takes it out of draft, the last review runs, and the operator merges it.

A stack stays well under 200 files touched, since a larger diff gets a shallower review.
An act that would pass that is two stacks. When `main` moves under a stack, the stack
branch merges `main` in on the Planner's word. A design change that surfaces during
review is parked on its own branch and lands as its own act.

Commit subjects carry `code:`, `docs:` or `process:`. A commit subject never carries a
closing keyword for an issue. Closing keywords live in the pull request body.

## 4. The pull request body

- `Implements: <Spec> <sections>`, with one verdict per section named. **Conforms**: the
  code does what the section says, and a claim about behaviour has a test. **Drifted**:
  the code is fixed. **Better way**: the Spec changes in the same act, and a design-level
  change goes to the operator. **Spec gap**: the Spec is extended, and a new capability
  is the operator's ruling.
- Every issue the act closes, or one line saying it closes nothing.
- Every small fix the act picked up on its way, in a table.

A merged Spec authorizes code. Where code and a Spec disagree, the same act fixes
whichever is wrong.

**Every repository is public.** Before a body, a brief or a diff is sent, it is searched
for box paths, host names, account and group names, and credentials. The Planner holds
facts about a box that one repository records and another forbids. A brief to a
repository carries only what that repository's boundaries admit.

## 5. Review

Every act passes four reviews, each a different lens.

1. **The Executor's self-check**, on its own diff, before the draft. The repository's
   gates run first, and the suite runs before the commit that claims it.
2. **The Planner's grade**, against a clean extract of the head, with a verdict per Spec
   section. Findings go to the Executor as one brief per round.
3. **Third-party review.** The Planner takes a cleared sub-act out of draft, which fires
   Codex. CodeRabbit reads only a pull request whose base is `main`, so a stack costs one
   CodeRabbit pass, taken when the operator takes the stack out of draft.
4. **The operator's merge.**

The rules of a round:

- **A severity line is agreed before review and held.** A finding that would bite in
  normal use, lose state, restore stale state silently or cross a privilege boundary is
  fixed. Documentation and consistency findings are batched. Crash windows and rare races
  are filed as issues with what was measured.
- **A finding is verified against the code before it is routed.** A finding that does not
  hold is answered on the pull request with the reason.
- **A finding names one site of its class.** Every site of the same shape in the act is
  found and tabled before the next pass.
- **One push per round.** A push to a pull request out of draft is a paid pass. At most
  one paid event happens in an hour.
- **A seam that draws a second finding gets a full read** of the governing passage
  against the code, not another patch.
- **Every thread is answered and resolved**, fixed or declined with the reason.

A clean automated gate is evidence the gate did not fire, not evidence of correctness.

## 6. Merges

The operator merges with the head pinned:

    gh pr merge <N> --merge --match-head-commit <40-hex sha>

A hash handed to the operator is copied from git or from the API, never typed from a
short form and never from memory. After each merge the next pull request's head is read
again before its pin is given. After a stack merges, the Executor ticks the items it
closed on their epic with the merge commit.

## 7. An act across repositories

The repositories meet only at contracts, across a process or network boundary. An act
that moves a contract is ordered by that fact.

- **A gap in another repository's interface is an issue on that repository**, one issue
  per interface question. It says what was measured, what is asked, and which document
  would move. This holds although one Planner now sees both sides, because the issue is
  the record a later reader finds.
- **The party that owns the contract moves first.** Its act amends the contract and lands.
  Each other party's act then builds against the merged text and names it. No repository
  builds against an unmerged contract.
- **One stack per repository.** A cross-repository act is one stack in each repository it
  touches. The stacks cite each other. No stack waits on another's review, only on its
  merge where the order above requires it.
- **A measurement on a box belongs to the repository that owns the box's layout.** Another
  repository cites the result in its own vocabulary and records no box fact.

## 8. The record

- A finding that needs a decision is an issue that carries what was measured, filed by the
  seat that measured it.
- The Planner reports to the operator with what changed, what was measured, what is
  pending, and the exact commands the operator runs next. The outcome comes first.
- Every act ends with an after-action review. Each improvement goes into this document or
  into a repository's `CLAUDE.md`, so the next plan carries it.

## 9. What this document does not do

It does not set a repository's gates, commands or boundaries. It does not say what shape a
document takes or how prose is written. Those belong to the repository's `CLAUDE.md` and
to whatever documents that file names as binding the repository. The Document Format and
the Working Rules bind WeaverAgent and do not bind WeaverWeb. It does not replace the
Working Process, which stands in WeaverAgent until it is revised and moved here.
