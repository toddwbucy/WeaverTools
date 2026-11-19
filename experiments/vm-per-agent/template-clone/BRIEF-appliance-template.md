# Brief: the appliance template

**Date:** 2026-11-19. **From:** the Planner. **To:** the image Executor.
**Authority:** the operator's rulings of 2026-11-19, made while walking a virtual machine
from appliance 4.0 through the console's setup pages by hand. The work is two phases.
Phase 1, this act, is the appliance template alone: a virtual machine made from it holds
everything it needs to operate as a Weaver agent appliance, with nothing typed. Phase 2,
after phase 1 is reliable, is what an operator does with the template: pairing it with
the console, the drives, the reports, and creating the agent. The operator asked for a
separate script whose only job is creating the template. The act is one stack of three
sub-acts from `main`.

## Goal

At appliance 4.0 a person at the console of a new virtual machine typed four things
before the machine could pair: three accounts, a folder layout, and one request from
each connector. The image also lacked a survey command, which came on a separate disk.
This act puts all of that in the template.

- Laid at build, in the sealed file: the two connector accounts and the pairing
  account, the folder layout under `/etc/weaver-web`, and the survey command.
- Made at each box's first boot, by a unit the image carries: each connector's key and
  its request. A key is per box and is never in the image.
- One script in the tree builds the template from end to end and stops at the first
  refusal.

The template stops there. It does not pair, name the box, move or adopt a config, start
a connector, report, touch a drive, pull a model or lay an agent.

## The sub-acts, in order

1. **The Spec.**
   - The build Spec: the new guest step and where it sits, what the cleanup keeps, the
     seal's probes, the stamp's lines, the proof's two new rows, and the one script.
   - The profile: that the image carries the survey and the ready layer, with the
     setup script still to come. The evidence the proof gathers. The reliability bar,
     stated as a proposal for the operator's ruling.
   - The survey format: that the image carries the survey at its path.
   - Each change cites the operator's rulings of 2026-11-19 by date.
2. **The layer.**
   - The guest step that lays the accounts in the shape the agent-creation tool
     accepts, the folders with their owners and modes, the survey and the request unit.
   - The request unit: it needs no network, it runs at every boot and remakes nothing
     that stands, it writes each file whole or not at all, it refuses a config that
     stands without its request, and it prints one ready line on the console.
   - The seal's probes: the accounts, the folders exact, the pairing folders empty with
     no key at the seal, and the unit's and the survey's digests equal to the stamp's.
   - Host tests for the layer and the requests, with stand-ins that write line by line.
     A check that no producer is piped to an early-exiting reader where the pipeline's
     status decides something.
3. **The proof and the script.**
   - A ready row, with nothing typed: the ready line before the login prompt, the
     accounts, folders and configs read by their attributes alone so that no key is
     read, each request verified, the survey run as an unprivileged account and its
     output parsed, the network as the credential laid it, the unit finished before login.
   - A reboot row: after a second boot nothing is remade, shown by digest.
   - The one script: the host tests, the build, every step, the seal, the proof and the
     tag's message.

## Shown running

A sub-act leaves draft only when it is shown where it runs. For the Spec that is a
read. For the code it is a build from the branch that seals, passes the proof with both
new rows, and boots ready with nothing typed.

**Reliable, as a proposal for the operator's ruling.** Three runs of the one script
from clean, one after another, unattended. Each seals and passes the proof. Every
binary is byte-equal across the three. The requests differ, as they must. The stamps
and the manifests differ only where each difference is explained. A run counts only if
no code changed after it. The stack's body states this bar and its result, and says the
bar is proposed.

## Ruled out

- Nothing of phase 2: pairing, the box's name, the moves and adopts, the connectors'
  units, the reports, the drives, the model, the laying, the agent.
- No site condition of any one machine in the image or its scripts.
- Nothing installed after the seal, and no script that patches a sealed image.
- No change to the first login.

## Other repositories

A gap in another repository's interface is an issue there, one per question. The
pairing account's name colliding with an agent's name is such a question for the agent
repository. What the image lays for the connectors is confirmed against the console's
Spec before the code leaves draft.

## Review

Each sub-act is a draft child of the stack. A child goes to review when it is shown
running, and merges into the stack on a clean read. The stack then goes to both
reviewers, and the operator takes it to `main`. The tag is the operator's.
