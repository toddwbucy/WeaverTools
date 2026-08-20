# The context-injection loop, in Python: the same behavior the compiled
# dev_loop carries, here to iterate at conversation speed. Edit this file
# and the NEXT TURN runs the edit - the connector reads it per crossing.
#
# The seat offers exactly seven calls:
#   seat.assembled_empty() -> bool        first-turn test
#   seat.session_shape()   -> list|None   [{"run": str, "kinds": {k: n}}]
#   seat.fullness()        -> tuple|None  (resident, capacity), plain counts
#   seat.flush(keep=0)     -> tuple|None  (resident_before, resident_after);
#                                         keep is the resident length the
#                                         session returns to, the cleanup
#                                         line, bounded by the seam at the
#                                         identity prefix and the resident
#                                         count
#   seat.classify(text)    -> list|None   [(label, score)] from the classify
#                                         artifact's head; None covers every
#                                         absence alike - no classifier
#                                         declared, the ask refused typed,
#                                         the content malformed, or the
#                                         channel lost - the record holding
#                                         which, never this return
#   seat.recall(n)         -> list|None   message events, newest n turns:
#                                         {"kind", "turn", "sequence",
#                                          "pairs": {key: json_text}}
#   seat.turn(delta)       -> dict        runs one turn, delta is a list of
#                                         {"role": "system"|"user"|
#                                          "assistant", "text": str}
#
# drive(seat, text) is the one crossing. Run at least one turn: a crossing
# that runs none falls back to a plain unshaped turn.
#
# Every judgment below is the loop's alone: the trigger, the recall depth,
# the quote budget, the memory conventions, and what every injected line
# says. The framework holds no threshold and no convention anywhere.
#
# THE MEMORY CONVENTIONS, series one. The model queries and saves its own
# state through its outputs: a line "RECALL: <subject>" asks memory, a line
# "REMEMBER: <fact>" saves one. Both are loop-detected in the emission and
# dispatched inward against the state seam - internal tools, never the
# gate's. A REMEMBER needs no write call at all: the line is already in the
# recorded emission, distilled into custody through the record's one
# ingress, so the save IS the utterance and the loop's job is the
# acknowledgment that keeps a generic model using the convention. A RECALL
# runs the premade query: recall custody, match the subject, answer labeled
# or report the miss honestly. Every contribution this loop authors - the
# opening, the re-entry, the feedback - rides the system role, the loop's
# own voice per the system role act, which is what lets the search exclude
# it by kind and the operator exclude it from custody at the election, no
# text ever matched to decide whose voice a message is.

import json

SYSTEM_PROMPT = (
    "You are a careful assistant. Answer from what you know, say plainly "
    "when you do not know, and keep answers as short as the question allows."
)

# The teaching, series one's framing: memory as the model's own faculty,
# one rigid line per verb, and the promise of feedback. Re-taught after a
# flush because the reset retires the resident copy of this paragraph too.
MEMORY_PROMPT = (
    "You have a private memory for this session.\n"
    "To save a fact you will need later, write a line: REMEMBER: <the fact>\n"
    "To ask your memory about something, write a line: RECALL: <subject> "
    "and stop your answer there.\n"
    "Memory results arrive labeled 'From your memory:'. A save is "
    "confirmed with 'Saved.'"
)

# The flush trigger: four fifths of capacity, checked between turns. Set
# this to what your setup wants - it is policy, not physics.
PRESSURE = 4 / 5

# How many recent turns the re-entry recalls, and how much recalled text
# it quotes in total, one budget across every message: a re-entry that
# quoted everything would rebuild the pressure the flush just relieved.
RECALL_TURNS = 4
QUOTE_BOUND = 600

# The memory conventions' own judgments. Rounds cap the detect-and-refeed
# cycle the way the tool rounds are capped. Asks per round and hits per
# ask keep an answer a working set. The quote bound is per hit, centered
# on the match. MISS_REDIRECT is the experiment's second arm: None tells
# the truth and stops, a string is appended to the miss to point the model
# somewhere else.
MEMORY_ROUNDS = 3
MEMORY_ASKS = 3
MEMORY_HITS = 4
MEMORY_QUOTE = 300
MISS_REDIRECT = None

# THE EXTERNAL TOOL. This is bravo's loop, the taught arm of the
# ablation, declared by bravo's `loop-file` per issue #243: the loop is
# a member of this agent's harness and unique to it, so the teaching
# below is unconditional here and absent from the untaught arm's file
# rather than gated by any conditional a shared file would need. The
# shell is the gate's one held tool, merged and standing, and the
# calculator is a script provisioned in this agent's own home - so the
# whole grant is this advertisement and nothing else.

# The advertisement speaks the family's trained shape - the tools block
# and the tool_call envelope are the forms the artifact's tuning expects
# - and then binds it to the one provisioned script. The last line is
# the experiment's instruction: arithmetic goes through the tool, not
# through the weights.
TOOL_PROMPT = (
    "# Tools\n"
    "\n"
    "You may call one or more functions to assist with the user query.\n"
    "\n"
    "You are provided with function signatures within <tools></tools> "
    "XML tags:\n"
    "<tools>\n"
    '{"type": "function", "function": {"name": "bash", "description": '
    '"Runs one shell command in your home directory. Your calculator '
    "lives there: ./calc \\\"EXPRESSION\\\" prints the value of an "
    "arithmetic expression. Numbers, + - * / // % ** and parentheses "
    'only.", "parameters": {"type": "object", "properties": {"command": '
    '{"type": "string", "description": "the shell command to run"}}, '
    '"required": ["command"]}}}\n'
    "</tools>\n"
    "\n"
    "For each function call, return a json object with function name "
    "and arguments within <tool_call></tool_call> XML tags:\n"
    "<tool_call>\n"
    '{"name": <function-name>, "arguments": <args-json-object>}\n'
    "</tool_call>\n"
    "Use your calculator for every arithmetic operation rather than "
    "computing in your head."
)


def teachings():
    """The opening's and the re-entry's shared curriculum: the system
    prompt, the memory conventions, and the tool advertisement. One
    builder so the flush cannot retire a lesson the opening taught.
    """
    return "\n\n".join([SYSTEM_PROMPT, MEMORY_PROMPT, TOOL_PROMPT])


def pressured(fullness):
    if not fullness:
        return False
    resident, capacity = fullness
    return capacity > 0 and resident >= capacity * PRESSURE


def continuity(shape):
    if not shape or len(shape) < 2:
        return None
    prior = shape[:-1]
    turns = sum(r["kinds"].get("turn.closed", 0) for r in prior)
    runs_word = "run" if len(prior) == 1 else "runs"
    turns_word = "turn" if turns == 1 else "turns"
    return (
        f"Continuity: this session held {len(prior)} earlier {runs_word} "
        f"with {turns} completed {turns_word} before this one. The "
        "conversation content of those runs is not in your context."
    )


def event_texts(events):
    """Every text block out of recalled message events, in landing order."""
    texts = []
    speakers = {
        "message.system": "system",
        "message.user": "user",
        "message.assistant": "assistant",
        "message.tool_result": "tool",
    }
    for event in events or []:
        speaker = speakers.get(event.get("kind"))
        raw = event.get("pairs", {}).get("content")
        if not speaker or raw is None:
            continue
        try:
            blocks = json.loads(raw)
        except ValueError:
            continue
        if not isinstance(blocks, list):
            continue
        for block in blocks:
            piece = block.get("text") if isinstance(block, dict) else None
            if isinstance(piece, str) and piece:
                texts.append((speaker, piece))
    return texts


def reentry(events):
    text = teachings() + (
        "\n\nThe working context was reset to stay within its limit. "
        "Recent conversation, restored from the session's record:"
    )
    if events is None:
        return text + (
            "\n(The record could not be reached: continue from the "
            "request alone.)"
        )
    budget = QUOTE_BOUND
    for speaker, piece in event_texts(events):
        if speaker not in ("user", "assistant") or budget <= 0:
            continue
        kept = []
        for word in piece.split():
            if len(word) > budget:
                kept.append("...")
                budget = 0
                break
            kept.append(word)
            budget -= len(word)
        if kept:
            text += "\n" + speaker + ": " + " ".join(kept)
    return text


def memory_lines(emission):
    """The sigil lines of one emission: (remembered facts, recall subjects).

    Column-zero-anchored on purpose: a sentence mentioning the convention
    does not fire it, and neither does an indented occurrence - a quoted
    line or a code block carrying the sigil is content, not an ask. Only a
    line that begins the sigil at its first column fires.
    """
    remembers, recalls = [], []
    for line in (emission or "").splitlines():
        if line.startswith("REMEMBER:"):
            fact = line[len("REMEMBER:"):].strip()
            if fact:
                remembers.append(fact)
        elif line.startswith("RECALL:"):
            subject = line[len("RECALL:"):].strip()
            if subject:
                recalls.append(subject)
    return remembers, recalls


def memory_search(events, subject):
    """The premade query: a case-insensitive match of the subject against
    custody's texts, newest hits last, each quoted in a window around the
    match. RECALL lines are excluded - they are asks, not facts - the
    loop's own voice is excluded by its role - the system kind is the
    loop's and the operator's framing, and feedback quoting a hit would
    match the next ask for the same subject, an echo chamber - and
    REMEMBER lines are exactly what should surface, so they stay. The
    role is the marker: no text is matched to decide whose voice a
    message is, so a genuine record beginning with any feedback phrase
    stays searchable.
    """
    needle = subject.lower()
    hits = []
    for speaker, piece in event_texts(events):
        if speaker == "system":
            continue
        for line in piece.splitlines():
            stripped = line.strip()
            if line.startswith("RECALL:"):
                continue
            at = stripped.lower().find(needle)
            if at < 0:
                continue
            start = max(0, at - MEMORY_QUOTE // 3)
            window = stripped[start : start + MEMORY_QUOTE]
            if start > 0:
                window = "..." + window
            if start + MEMORY_QUOTE < len(stripped):
                window = window + "..."
            if window not in hits:
                hits.append(window)
    return hits[-MEMORY_HITS:]


def memory_followup(seat, emission):
    """The inward dispatch: None where the emission holds no sigil, else
    the one contribution answering every sigil it held - saves confirmed,
    recalls answered labeled or missed honestly.
    """
    remembers, recalls = memory_lines(emission)
    if not remembers and not recalls:
        return None
    parts = []
    if remembers:
        parts.append("Saved.")
    if recalls:
        events = seat.recall(None)
        for subject in recalls[:MEMORY_ASKS]:
            if events is None:
                parts.append("(Your memory could not be reached.)")
                break
            found = memory_search(events, subject)
            if found:
                parts.append(
                    "From your memory:\n" + "\n".join("- " + hit for hit in found)
                )
            else:
                miss = f"Your memory holds nothing about {subject}."
                if MISS_REDIRECT:
                    miss += " " + MISS_REDIRECT
                parts.append(miss)
    return "\n".join(parts)


def drive(seat, text):
    delta = []
    first = seat.assembled_empty()
    # The context management: at pressure the loop elects the flush and
    # rebuilds from custody's recall. The flush confirmation gates nothing
    # - a missing answer cannot prove the flush did not land.
    if not first and pressured(seat.fullness()):
        seat.flush()
        delta.append({"role": "system", "text": reentry(seat.recall(RECALL_TURNS))})
    if first:
        opening = teachings()
        line = continuity(seat.session_shape())
        if line:
            opening += "\n\n" + line
        delta.append({"role": "system", "text": opening})
    delta.append({"role": "user", "text": text})
    outcome = seat.turn(delta)
    # The memory rounds: detect the sigils, dispatch inward, refeed, and
    # stop when an emission holds none or the cap lands. A follow-up turn
    # that refuses fails the request whole, which is the crossing's
    # standing economics.
    for _ in range(MEMORY_ROUNDS):
        follow = memory_followup(seat, (outcome or {}).get("emission", ""))
        if follow is None:
            break
        outcome = seat.turn([{"role": "system", "text": follow}])
