# Tool Egress Boundary

**Status:** DRAFT v0.3, 2026-08-07. Phase-one architecture-seat material, outside the
document set and never ratified. Argument and framing, no code. It decides nothing the
tool workflow is there to decide.

**Date filed:** 2026-08-07
**Document ID:** `tool-egress-boundary-frame`
**Parent:** `WeaverTools-PRD`
**Editorial:** Per the Working Rules.

---

## 0. What this document is

It states where the agent's boundary to the world sits, what authorizes a crossing, and
what the boundary claim does not reach. It answers `weaver-gate-PRD` section 7, the
PENDING cell staged awaiting the tool workflow chartering plus a stated boundary
rationale, and it is the frame that workflow opens against.

It bears on apex section 3 step 1 and step 7, which it contradicts. A gate edit may not
revise a merged apex clause, so the change is apex-level and section 9 names it as such.

**The first draft priced this against the wrong thing, and the record is worth
keeping.** It stated the threat as a compromise of the agent's process family reaching
for the network, and laid two arms against that threat. A compromise wall has to be
kernel-enforced to be real, and the enforcement belongs to the network-facing layer this
program is not building yet, so the threat framing pulled application-tier work into a
primitives charter. Section 5 places that work where it belongs. What survives is the
narrower claim that was the real content underneath, and it needs no threat to stand,
because the structure it rests on is already in force.

## 1. The claim

The gate is the agent's only boundary to the world, and a tool call is world traffic
that crosses it like any other. Everything the agent does as an agent, a prompt
perceived or an outbound call, passes the gate. Nothing the agent initiates toward the
world runs around it.

The gate never touches the network. The gate speaks AF_UNIX, and the network is some
application's problem on the far side of that socket, outside the agent. This is not a
property the frame adds. The primitives were brought down to Unix sockets so that no
primitive is ever exposed to the network, and the gate is one of those primitives. The
claim is therefore about where the boundary sits and what authorizes a crossing, and it
is not a claim about network capability at all, because the primitive that would hold
network capability is not among these components.

This retires the merged tool model for any tool that reaches the world. Apex step 7
makes a tool a subprocess the harness forks and runs as the agent's own Linux user, and
step 1 states that the outbound connection such a tool opens is not ingress and does not
pass the gate. A world-reaching tool built that way puts a network-touching process
inside the component set, which is the one thing the socket floor exists to prevent.
Under this proposal a world-reaching tool is a registered application the agent
addresses over an egress socket, and the network lives with that application, outside
the agent, past the socket.

## 2. What the component set is

The claim above turns on which processes are inside the set, and the line is drawn by
what creates a process rather than by what that process does.

**What this program forks is inside the set, and what the operator provisions is outside
it.** The worker and every process it forks are the program's own, and no process in
that family holds a network descriptor. A registered tool is started by the operator,
under its own identity and its own lifecycle, and it holds whatever network capability
its job requires. The two classes are told apart by what created them, which is a fact
about the running system rather than a declaration a process makes about itself.

The definition is what makes section 4's enumeration bounded and section 6's argument
structural. Without it the set is whatever a reader takes it to be, and the claim that
no component holds a network descriptor would rest on every forked child being honest
about what it opens.

**Bash is the case that forces the line.** Apex section 4 makes bash the reference tool
and a real capability of the deliverable, and bash reaches the network as ordinary user
capability. A forked process may hold no network descriptor, and bash cannot promise
that, so a world-capable bash is a registered application outside the set rather than a
subprocess inside it. Section 9 carries what that costs the definition of done.

## 3. Two sockets, split by which party opens the exchange

The gate carries two sockets, and the axis that separates them is which party may open
an exchange, the property `weaver-organ-channel` section 1 names, per the initiation
ruling of 2026-08-07. Each of the two is single-initiator. The world opens exchanges on
one, and the agent perceives and answers. The agent opens exchanges on the other, and
the world's registered tool answers. Both carry a request and its answer, since a prompt
is answered and a tool call returns a result, and an answer riding back on the socket
that carried the request is the same exchange closing rather than a crossing to the
other channel.

**Single-initiator is what the interior's channels are not, and the difference is where
the fact lives.** On an organ channel either party may open an exchange, so which one
did is carried per message and the exchange is identified by its opening party and that
party's ordinal. On a single-initiator socket one answer is legal, so the fact lives in
the object: the socket says which party opens on it, established before any content is
read. What that buys is a check that reads envelope headers alone and never a payload,
which is the parsing the gate must not do if it is to stay an opaque pass-through.

**This axis is not apex section 4's, and neither may be read onto the other.** Apex
section 4 reserves initiation for who moved first inside the agent, elected when the
model emitted the call as ordinary output and autonomic when the loop fired it, and that
judgment is a fact the trace records. The axis here is which party opens an exchange on
a seam. The agent is the opening party on its own socket whether the call was elected or
autonomic, so the two cross and neither answers the other.

**The party that opens an exchange on the agent's socket is the loop the operator
compiles into the harness.** That loop drives the agent's logic and holds the tool
bindings, and a binding is not a connection to the internet handed over with
instructions to go at it. A binding is the address of one registered tool reaching one
endpoint and nothing else. The loop opens the exchange and the gate holds the door, so
the loop reaches out only where the door was provisioned to allow the traffic. The same
door admits inbound traffic, so the only processes that enter are the ones already named
as allowed, and they enter this one way. The gate is the configured aperture in both
directions, and the loop never holds a world descriptor. It holds the ability to speak
through a door another component configured.

This is the ears-eyes-mouth socket and the hands socket, drawn by which party opens
rather than by data direction (Bucy, 2026). The first draft named an asymmetry, a
membrane on the perception side and a fence on the action side. This proposal makes both
sides the same kind of thing, a contracted socket with kernel-supplied identity, so the
agent meets the world through one crate in both directions.

**The world-opened socket is the agent's whole perception surface, and the reading stays
in the vision's register.** A prompt is what the agent perceives of the world, and a
modality added later, audio or video, is a new content kind crossing the same door
rather than a new door. The sockets are named by opening party rather than by function
because a tool's answer is world-authored content arriving on the agent-opened socket,
so a name drawn from perception would be false of one of the two crossings carrying it.
`weaver-tools-vision` section 6 is where the perception reading belongs, and this frame
carries it no further.

Admin is not on this axis and must not be read onto it. The administrator side gives an
operator a view into the agent's components and the interior, and it does not act as the
agent toward the world. It is operator-into-the-parts, a separate door in a separate
crate, and the agent does not experience it as communication with the world at all. The
trace stream belongs on this same operator side rather than on the agent's world axis.
It is the highest-volume outbound flow in the system, and the emitted record is the
operator's durability responsibility per `weaver-admin-operator-contract` section 3, so
counting it as the agent acting toward the world would leak the single-point claim
against the busiest line in the machine. The line to hold is the agent-as-a-unit through
the gate against the operator-into-the-parts through admin, and the separation is
load-bearing rather than incidental.

## 4. Why the boundary holds

The gate charter is right that a boundary owes a rationale or it is an upgrade priced
without a model, a motive wearing a mechanic's clothes. The rationale here is structural
and it is already in force. Apex invariant 5.1 makes every seam where one component asks
another process to do something a Unix socket under a named contract, with no exceptions
including for crates that arrive later. No primitive holds a network descriptor, so no
primitive is reachable from the network or able to reach it directly. A tool call is
made to fit this same floor rather than to punch through it. The agent addresses a
socket, and whatever network work the far side does is done outside the agent by a
process the agent never becomes.

This is a checkable claim and a small one. Enumerate the sockets every component in the
set holds and confirm they are all AF_UNIX, and confirm no component in the set holds an
AF_INET descriptor. The boundary is that enumeration coming back clean, which is one
pass over the running component set rather than an argument, and section 2 is what makes
the set the enumeration walks a bounded thing. The claim is deliberately narrower than
any statement about what a tool can be made to do, and section 5 holds that line.

## 5. What this frame does not claim

Three things sit next to this boundary and are not it, and naming them keeps the
boundary claim from being asked to carry weight it was not built for.

**It is not a compromise answer.** The gate authorizes the processes on its list and
refuses the rest, and it holds the door for the traffic it was configured to allow.
Whether a process that was authorized is later turned is a separate problem at a
separate layer, and loading it onto the gate is how network paranoia creeps back into
the one place the socket floor cleared it out of. This frame states the boundary and
parks the compromise question with its own layer, unaddressed here on purpose.

**It is not the network-facing security layer.** A registered tool that reaches the
world will eventually want a firewall, application support, and a hardened surface, and
that layer is real and is coming. It is the body built around this brain, and it is not
what this program is building yet. Treating the network as this frame's responsibility
was the scope leak the first draft made. The core inner layer is the work in front of
us, and the body waits for the brain to stand.

**It is not an injection answer.** Prompt injection is handled where it lives, in small
activation networks and rerankers and categorizers inside the reasoning process that are
not built yet, and in the network-facing application behind the contract that belongs to
the later layer above. A tool's answer is world-authored content entering the next
prompt, so the answer path has an injection surface, and its owner is that later
network-facing wrapper rather than the gate, a registered tool being part of that skin
rather than part of this set. The frame names injection to place it and demotes it, so
the boundary claim is not asked to contain it.

## 6. The tool is reached, not forked

The first draft laid two arms, a tool as a subprocess under its own uid against a tool
as a standing registered application, and deferred the election. The axis those arms
differed on was network-capability containment, which section 5 places outside this
frame, so the arms collapse for the case this frame is about. Under section 2's
definition a world-reaching tool cannot be a forked subprocess inside the component set
without putting a network-touching process where the socket floor forbids one. For a
world-reaching tool the registered-application form is not one option among two. It is
what the floor already requires.

Tools that touch no network are a different matter and not this frame's subject. This
frame is the egress boundary, the tools that reach out, and for those the shape follows
from the floor rather than from a measurement.

## 7. The cost the egress seam carries

A world-reaching tool becomes a standing registered application with its own identity,
socket, and lifecycle, provisioned and supervised outside the run rather than forked and
reaped within it. That is real operational weight and the frame carries it in the open.

The seam contract has one design point worth flagging, because it turns the trust
direction around. On the world-opened socket the world dials the agent and the gate
reads the peer by `SO_PEERCRED`. On the agent-opened socket the loop reaches through the
gate toward the tool, so the tool reads the agent by the same kernel-supplied
credential, and the agent in turn confirms it reached the registered tool and not a
process squatting the socket path. Both ends can read peer credentials on AF_UNIX, so
mutual identification is available, and who binds the path and with what permissions
becomes a provisioning clause the contract states rather than leaves implied.

## 8. Scope, and the line it holds

The seam and its contract are primitive. A tool-world contract takes the shape the
gate-world contract already holds, a wire contract between two nodes with
kernel-supplied identity and no dependence on a specific neighbor. That is in the
program.

The tools themselves are not. A tool is body and world, addressed through the gate and
living outside it, application-tier and out of the OS-primitives program by the same
line drawn between the brain inside the skull and the instruments outside it. The
charter frames the seam and never the tools. This keeps the mechanic with the primitive
and the motive with the consumer, which is the boundary the Working Rules exist to hold.

## 9. Where this lands

The change is apex, not gate. It contradicts apex step 1, which rules outbound tool
connections out of the gate, and apex step 7, which makes a tool a forked subprocess
running as the agent uid. A gate charter may not contradict a merged document, so this
cannot be a gate edit. It needs apex step 1 and step 7 revised, a tool-seam contract
authored, and a party to that contract that does not exist yet.

**The sweep is wider than those two clauses, and it is owed as apex-edit work rather
than performed here**, since this frame decides nothing and the retirements are the apex
batch's own manifest.

Apex section 9's grip paragraphs answer talks-to through the world-opened socket's
answer path, which is client-held execution and a different mechanism than the
agent-opened socket, so the sweep retires or re-points that wording. The vision's
section 6 has a tool's answer re-entering through the gate as part of the next prompt,
which is the world-opened path and now names the wrong socket, though its initiation
axis survives untouched and the two-socket split is that axis turned into hardware,
worth one line where the sweep lands. Definition-of-done item 5 requires a real tool
executed under kernel-enforced OS constraint, and under section 2 the reference bash is
a registered application the program does not ship, so the item lands as a conformance
fixture the way the gate suite ships fake clients. `weaver-admin-PRD` section 7 names
the agent-uid tool case as its own assumption and files the cell to the gate, so the
assumption sentence is edited when the cell resolves. The follow-on gate-charter edits
are the no-second-listener surface clause of section 3, the relay of section 13.1
gaining a second leg, and the section 7 cell itself, all after the apex, since
apex-first is the only legal order.

**The apex edit will want a grounding the rationale does not need.** The egress seam
grounds in apex invariant 5.1, that every seam where one crate asks another process to
do something is a Unix socket under a named contract, no exceptions including for crates
that arrive later. The program has already contracted three parties that are not crates,
the client, the operator, and the init system, so a tool is the fourth instance of a
standing pattern rather than a novelty. One paragraph now spares the workflow the
rediscovery.

`weaver-gate-PRD` section 7 holds the PENDING cell open for exactly this. This document
is the boundary rationale and the opening frame the workflow charters against. It
decides nothing the workflow is there to decide. It states where the boundary sits, why
it holds structurally, what it does not claim, and where the apex sweep runs, so the
tool workflow opens with the question already framed rather than rediscovering it from
the code.
