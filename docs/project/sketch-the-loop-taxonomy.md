# Sketch: the loop taxonomy

**Status:** SKETCH, opened 2026-08-24. Decides nothing and authors no Spec. It
records architecture settled in conversation on 2026-08-23 so that the corpus stops
pointing at a ruling it does not hold.

**Why it exists at all.** `reasoning-loop-boundary` was archived carrying a status
line saying its two-level model is superseded by this taxonomy. That was true and the
taxonomy was written down nowhere, so the archive pointed at nothing. **A sketch that
records a settled thing badly is better than a pointer to an unrecorded one**, and it
is a sketch rather than a charter because where it lands is still open.

**Document ID:** `sketch-the-loop-taxonomy`
**Editorial:** Per the Working Rules.

---

## 1. Two substrates and one overlay

The corpus described one loop and implied a second. What is settled is **two
substrates, distinguished by what they refuse, and one overlay that composes with
either, distinguished by what it produces.**

|  | no intervention | with intervention |
|---|---|---|
| **production** | the ordinary agent | **the calculator**, a detector choosing the cut |
| **diagnostic** | the replay class: residual tap, logit lens, attention capture, entropy recompute | **counterfactual replay**, activation patching |

**This corrects the first framing of it.** That framing had three loops and said the
distinguishing property in every case is what the loop refuses. **The substrates are
distinguished by refusals. The overlay is not** - intervention adds rather than
refuses, and what names it is that its product is a counterfactual token path rather
than an observation keyed to the original run.

## 2. The production substrate

Work enters through Gate. The in-RAM working structure accumulates and is queried at
decode speed. State is written.

## 3. The diagnostic substrate, which is three refusals

**Gate never starts**, so nothing enters from outside.

**The working structure is preloaded from a finished trace and read positionally as
the source of prompts** rather than accumulating.

**Nothing writes back**, so the substrate under examination is immutable for the
loop's duration.

State management is still in the path and is not doing the job it does in production.
**Same organ, opposite direction.**

## 4. The intervention overlay

It runs a token path and perturbs it. Its product is **a counterfactual token path
rather than an observation keyed to the original**, and that is what names it.

**It composes with either substrate**, which is why it is an overlay rather than a
third loop. Two consequences follow that the three-loop framing obscured.

**Writing to the working structure does not distinguish it.** Production writes too.
Writing distinguishes intervention against the diagnostic substrate only, which is a
fact about that one pairing rather than a defining property.

**Activation patching does not leave the diagnostic substrate.** It fails the passive
reader test, which is the diagnostic class's membership test, and it fails it by
applying intervention. It is not a third thing.

## 5. The diagnostic loop is a class, and the readout is interchangeable

The residual tap is one passive reader on a faithful re-execution. **Swap the reader
and the loop is unchanged**, which is what makes this a class rather than one
instrument.

**The membership test: the reader needs only a forward pass over a fixed token path,
and writes nothing.**

- **Attention-pattern capture** passes.
- **Per-layer logit-lens decoding** passes and is the cheapest member.
- **Recomputation of per-position entropy and surprisal** passes and is close to free,
  since the measurement payload already carries the production figures to compare
  against.
- **Activation patching fails**, because it changes the input and therefore produces a
  different run.

**What is authored once is the loop, the custody rule, the artifact identity, and the
certification. Every instrument after the first is a tap plus a Spec clause.**

## 6. Diagnostic mode is a declaration, not an operation

This is what makes the rest cost almost nothing.

**Diagnostic mode is a field in the agent's declaration**, beside a field naming the
trace to be replayed. Admin gains no mode concept and no new contract. It gains a
field, read at load, refused at admit if malformed, which is a path the load already
has.

**Nobody puts an agent into diagnostic mode.** A diagnostic agent is loaded, pointed
at a trace, and it replays. **Its production twin is a different binding of the same
weights.** There is no state transition anywhere for anything to get wrong.

Three things follow without further authoring.

**Gate is a binding property rather than an ingress carve-out.** Load starts Gate last
and unload stops Gate first, so a diagnostic binding is one that never reaches that
step. Gate ingress and replay ingress cannot coexist in any configuration, and **the
exception is unrepresentable rather than permitted**. Narrowing the scope criteria by
arguing that replay input is not work would invite relitigation on every reading,
because a replay does present prompts.

**Custody is unchanged and needs no exception.** The trace path is declared material,
so the driver reads the operator-held stream as an operator principal. `weaver-state`
never opens a trace file and `weaver-trace` keeps its write-only pin. The trace parser
belongs to the driver, outside the agent.

**The working structure inverts**, per section 3.

## 7. Certification splits by substrate

**The null replay certifies the diagnostic substrate.** The mechanic's correctness is
not established by whether a readout is interpretable. It is established by
exact-match comparison of a replay against the original, and again with the readout
elected, to show the read is passive. **A readout from an uncertified replay is a
picture of an unknown run.**

**An output comparison means nothing unless the input was the same one, so the
certification checks the input first.** Apex section 8 names what a deterministic
re-feed requires, exactly: input token ids, output token ids, model identity with its
weights hash, sampling parameters, and the prompt-block partition. **The template's
identity travels with them**, which is what closes the gap canonical messages leave:
the same conversation rendered under a later template is a different prompt, so a
replay that re-rendered rather than re-fed would be comparing two different runs and
finding them different. **Certification is therefore two claims and not one** - that
the input was identical, established from the record, and that the output matched.

**No second instrument lands before the first replay is certified**, because a second
tap sharing an uncertified replay inherits the uncertainty rather than dividing it.

**The production column needs no replay certification**, because nothing is being
replayed. What it needs instead is for the mechanic to work, and **that is not yet
shown.** The calculator crate stands, but the cut-and-recompute wiring it would use
waits on the harness-SPU splice amendment, and `weaver-internal`'s own cell says so.

**So the inheritance is conditional and the condition is unmet.** The intended
ordering is that the mechanic earns its correctness on the production side and the
diagnostic side inherits it. Until the splice lands and is shown working there, the
diagnostic side would be inheriting a mechanic nothing has exercised, which is the
same defect as a readout over an uncertified replay one level up. **A plan, not a
record.**

## 8. The mechanic and the motive

This is the region where the two intersect, which is why the line needs stating rather
than assuming.

**In scope for the mechanics**, and this program's to answer: whether replay
re-executes the same forward passes, whether the readout perturbs the pass, whether
the capture joins back to the forward pass that produced it, whether the artifact is
identified by what determines it, and whether the vectors reach disk uncorrupted.

**Out of scope and the operator's**: whether a readout means anything, how it compares
against a cheaper instrument, how many fitting sequences suffice, what a trajectory
across layers indicates, and whether one binding's readouts agree with another's.

**The framework's claim is that it delivers a faithful readout with replayability. It
is not a claim about what the readout is worth.** Two bindings serving the same
weights at different precisions are, to the mechanic, two bindings it must serve
identically. Whether their readouts agree is a finding.

## 9. What this sketch does not settle

- **Where it lands.** A domain ruling belongs in `weaver-agents-PRD`. A statement
  about what the program is for belongs at the suite level, which has no document.
  This sits in `docs/project/` until that is answered, and it wants answering
  alongside the two use cases, which are recorded nowhere either.
- **The instrument suite** it would eventually carry is named in the ruling as
  `sketch-diagnostic-instrument-suite`, which does not exist in this tree.
- **Nothing here is chartered.** No wire format, no field list, no type. The
  declaration field of section 6 is described and not specified.
