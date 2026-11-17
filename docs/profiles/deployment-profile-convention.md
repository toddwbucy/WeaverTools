# Deployment profiles - convention

**Status:** Merged in pull request 32 (2026-11-16); revised for format documents, not
merged.
**Date filed:** 2026-11-15
**Document ID:** `deployment-profile-convention`
**Authority:** the operator's ruling of 2026-11-15 that the agent appliance is one
deployment profile for a Weaver agent, and that the class is defined once, in a short
convention beside its first instance.
**Editorial:** ASCII, absolute dates, prose at 88 characters.

---

## 0. What this document is

It defines the document class "deployment profile" for the WeaverTools suite. It states
what a profile is, what a profile holds, and what a profile may not do. Each profile in
this directory follows it.

## 1. What a deployment profile is

**A deployment profile describes one way to stand up a Weaver agent on a machine.** The
WeaverAgent system is the same in every profile. The machine under it is not. A profile
names one kind of machine, states what that kind of machine holds fixed, and states how
an agent deployed that way is recorded.

**A profile is not a PRD and not a Spec.** A PRD states what a component needs and why.
A Spec states how a requirement is represented. A profile does neither. It collects, for
one kind of machine, what the charters already require and what the operator has ruled.

Three profiles are named. One is written.

| Profile | Machine | Document |
|---|---|---|
| The agent appliance | a virtual machine made from one sealed image | `weaver-appliance-profile.md` |
| The host install | a machine the operator installs the WeaverAgent system onto | not written |
| The Spark-class machine | a single-use machine of the Spark and GB10 class | not written, and out of scope until such a machine exists (ruling of 2026-11-15) |

## 2. What a profile holds

A profile has six parts, in this order.

1. **The substrate and the entry condition.** The kind of machine, and what the operator
   must already have before the profile applies.
2. **What the profile fixes.** What is the same on every machine of this profile.
3. **What the declaration still chooses.** What the operator still sets for each agent.
4. **How a deployment is recorded on the tuple.** Which members of the deployment tuple
   the profile supplies, and how each is named.
5. **The asks of the charters.** Each requirement the profile needs from a component,
   the charter that holds or would hold it, and where that ask stands.
6. **The evidence.** What has been measured about the profile, with its counts and its
   conditions, and what is owed.

## 3. What a profile may not do

**A profile decides nothing about any component.** A requirement on a component belongs
in that component's charter. Where a profile needs a requirement that no charter states,
the profile lists it as an ask, and the ask becomes an issue on the repository that owns
the charter.

**A profile binds no Spec.** A Spec takes its grounds from its own charter and from the
contracts. A Spec does not cite a profile as authority.

**One exception has a narrow scope.** The build of a profile's own artifact, such as an
image, has a Spec in this repository under the profile. That Spec represents the build
and nothing else.

**A profile may also carry a format document.** Beside its build Spec, a profile may
carry a document for a wire format that two repositories meet on, such as a report that
its image writes and the console reads. The profile cites it, and it decides that format
and nothing else.

**A profile states no claim that is not measured or ruled.** A sentence in a profile is
one of three things: a ruling of the operator with its date, a requirement with the
charter that holds it, or a measurement with its record. A sentence that is none of the
three is marked as open and names who settles it.

## 4. Open

- The node kind of a profile in the document graph. The graph is suspended until release
  by the ruling of 2026-09-27. Settled by the operator when the graph resumes.
