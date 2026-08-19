# The context-injection loop, in Python: the same behavior the compiled
# dev_loop carries, here to iterate at conversation speed. Edit this file
# and the NEXT TURN runs the edit - the connector reads it per crossing.
#
# The seat offers exactly six calls:
#   seat.assembled_empty() -> bool        first-turn test
#   seat.session_shape()   -> list|None   [{"run": str, "kinds": {k: n}}]
#   seat.fullness()        -> tuple|None  (resident, capacity), plain counts
#   seat.flush(keep=0)     -> tuple|None  (resident_before, resident_after);
#                                         keep is the resident length the
#                                         session returns to, the cleanup
#                                         line, bounded by the seam at the
#                                         identity prefix and the resident
#                                         count
#   seat.recall(n)         -> list|None   message events, newest n turns:
#                                         {"kind", "turn", "sequence",
#                                          "pairs": {key: json_text}}
#   seat.turn(delta)       -> dict        runs one turn, delta is a list of
#                                         {"role": "user"|"assistant",
#                                          "text": str}
#
# drive(seat, text) is the one crossing. Run at least one turn: a crossing
# that runs none falls back to a plain unshaped turn.
#
# Every judgment below is the loop's alone: the trigger, the recall depth,
# the quote budget, and what the re-entry says. The framework holds no
# threshold anywhere - if this loop never checks pressure, nothing flushes
# on its behalf and the wall answers with the honest backstops.

import json

SYSTEM_PROMPT = (
    "You are a careful assistant. Answer from what you know, say plainly "
    "when you do not know, and keep answers as short as the question allows."
)

# The flush trigger: four fifths of capacity, checked between turns. Set
# this to what your setup wants - it is policy, not physics.
PRESSURE = 4 / 5

# How many recent turns the re-entry recalls, and how much recalled text
# it quotes in total, one budget across every message: a re-entry that
# quoted everything would rebuild the pressure the flush just relieved.
RECALL_TURNS = 4
QUOTE_BOUND = 600


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


def reentry(events):
    text = SYSTEM_PROMPT + (
        "\n\nThe working context was reset to stay within its limit. "
        "Recent conversation, restored from the session's record:"
    )
    if events is None:
        return text + (
            "\n(The record could not be reached: continue from the "
            "request alone.)"
        )
    budget = QUOTE_BOUND
    speakers = {"message.user": "user", "message.assistant": "assistant"}
    for event in events:
        speaker = speakers.get(event["kind"])
        raw = event["pairs"].get("content")
        if not speaker or raw is None or budget <= 0:
            continue
        try:
            blocks = json.loads(raw)
        except ValueError:
            continue
        if not isinstance(blocks, list):
            continue
        for block in blocks:
            piece = block.get("text") if isinstance(block, dict) else None
            if piece is None or budget <= 0:
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


def drive(seat, text):
    delta = []
    first = seat.assembled_empty()
    # The context management: at pressure the loop elects the flush and
    # rebuilds from custody's recall. The flush confirmation gates nothing
    # - a missing answer cannot prove the flush did not land.
    if not first and pressured(seat.fullness()):
        seat.flush()
        delta.append({"role": "user", "text": reentry(seat.recall(RECALL_TURNS))})
    if first:
        opening = SYSTEM_PROMPT
        line = continuity(seat.session_shape())
        if line:
            opening += "\n\n" + line
        delta.append({"role": "user", "text": opening})
    delta.append({"role": "user", "text": text})
    seat.turn(delta)
