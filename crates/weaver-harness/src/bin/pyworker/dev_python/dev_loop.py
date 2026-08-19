# The context-injection loop, in Python: the same behavior the compiled
# dev_loop carries, here to iterate at conversation speed. Edit this file
# and the NEXT TURN runs the edit - the connector reads it per crossing.
#
# The seat offers exactly three calls:
#   seat.assembled_empty() -> bool        first-turn test
#   seat.session_shape()   -> list|None   [{"run": str, "kinds": {k: n}}]
#   seat.turn(delta)       -> dict        runs one turn, delta is a list of
#                                         {"role": "user"|"assistant",
#                                          "text": str}
#
# drive(seat, text) is the one crossing. Run at least one turn: a crossing
# that runs none falls back to a plain unshaped turn.

SYSTEM_PROMPT = (
    "You are a careful assistant. Answer from what you know, say plainly "
    "when you do not know, and keep answers as short as the question allows."
)


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


def drive(seat, text):
    delta = []
    if seat.assembled_empty():
        opening = SYSTEM_PROMPT
        line = continuity(seat.session_shape())
        if line:
            opening += "\n\n" + line
        delta.append({"role": "user", "text": opening})
    delta.append({"role": "user", "text": text})
    seat.turn(delta)
