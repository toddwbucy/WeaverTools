#!/usr/bin/env python3
"""Difference two diagnostic records' residual columns, position for position.

The measurement certification step 3 rests on: the apex names a GPU float
tolerance for the reader pass's vector comparison, and no number for it has
been taken on this hardware. Two column-elected replays of one source under
one declaration are differenced here - if the engine's determinism extends
to the tap's copies, the tolerance is zero within a device and the vector
comparison gets exact match like the token path; if not, this prints the
actual figure.

Run: python3 compare_columns.py <record-a.ndjson[.zst]> <record-b.ndjson[.zst]>
"""
import io, json, subprocess, sys


def columns_of(path):
    if path.endswith(".zst"):
        out = subprocess.run(["zstd", "-dc", path], capture_output=True, check=True)
        lines = io.StringIO(out.stdout.decode())
    else:
        lines = open(path)
    columns = {}
    tokens = {}
    for line in lines:
        e = json.loads(line)
        if e["kind"] == "residual.column":
            p = e["payload"]
            columns[(e.get("turn"), p["position"])] = p["values"]
        if e["kind"] == "model.measurement":
            tokens[e.get("turn")] = e["payload"]["output_tokens"]
    return columns, tokens


def main():
    a_path, b_path = sys.argv[1], sys.argv[2]
    a, a_tokens = columns_of(a_path)
    b, b_tokens = columns_of(b_path)
    if a_tokens != b_tokens:
        print(json.dumps({"refusal": "the two records' token paths differ: "
                          "these are not two replays of one run"}))
        return 1
    if set(a) != set(b):
        print(json.dumps({"refusal": "the sampled positions differ",
                          "only_a": len(set(a) - set(b)),
                          "only_b": len(set(b) - set(a))}))
        return 1
    positions = 0
    values = 0
    exact = 0
    max_abs = 0.0
    worst = None
    for key in sorted(a):
        va, vb = a[key], b[key]
        positions += 1
        for layer, (la, lb) in enumerate(zip(va, vb)):
            for x, y in zip(la, lb):
                values += 1
                if x == y:
                    exact += 1
                else:
                    d = abs(x - y)
                    if d > max_abs:
                        max_abs = d
                        worst = {"turn": key[0], "position": key[1],
                                 "layer": layer, "a": x, "b": y}
    print(json.dumps({
        "positions": positions,
        "values": values,
        "exact": exact,
        "exact_rate": round(exact / values, 8) if values else None,
        "max_abs_diff": max_abs,
        "worst": worst,
        "verdict": "byte-identical" if exact == values else "within-tolerance-question",
    }, indent=1))
    return 0


if __name__ == "__main__":
    sys.exit(main())
