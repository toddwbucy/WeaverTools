"""The probe's pure readings, per issue #511."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from weaver_probe import first_divergence, aligned_agreement_after, truncated_kl, reading_one, reading_two, extract_run, measured_events, divergence_ordinal

def test_divergence_and_agreement():
    assert first_divergence([1,2,3],[1,2,3]) is None
    assert first_divergence([1,2,3],[1,9,3]) == 1
    assert first_divergence([1,2],[1,2,3]) == 2
    assert aligned_agreement_after([1,2,3,4],[1,9,3,5], 1) == 1/3
    assert aligned_agreement_after([1,2],[1,2], None) is None

def test_truncated_kl_identical_is_zero_and_reports_coverage():
    r=[{"token":1,"probability":0.6},{"token":2,"probability":0.3}]
    k=truncated_kl(r, r); assert k["kl_bits"]==0 and k["shared_candidates"]==2 and abs(k["ranked_mass_p"]-0.9)<1e-9 and abs(k["shared_mass_p"]-0.9)<1e-9
    q=[{"token":1,"probability":0.3},{"token":3,"probability":0.6}]
    k=truncated_kl(r, q); assert k["kl_bits"]==0 and k["shared_candidates"]==1 and abs(k["excluded_mass_p"]-0.3)<1e-9
    # one shared candidate carries all the conditional mass on both sides, so the KL over the shared support is zero

def test_truncated_kl_is_never_negative_and_names_what_it_saw():
    # the review's case: an unrenormalized partial sum lands at -0.42 bits here
    p=[{"token":1,"probability":0.50},{"token":2,"probability":0.30}]
    q=[{"token":1,"probability":0.90},{"token":9,"probability":0.05}]
    k=truncated_kl(p, q)
    assert k["kl_bits"]>=0, k
    assert abs(k["shared_mass_p"]-0.5)<1e-9 and abs(k["shared_mass_q"]-0.9)<1e-9, k
    assert abs(k["ranked_mass_p"]-0.8)<1e-9 and abs(k["excluded_mass_p"]-0.3)<1e-9, k
    p2=[{"token":1,"probability":0.5},{"token":2,"probability":0.3}]
    q2=[{"token":1,"probability":0.3},{"token":2,"probability":0.5}]
    assert truncated_kl(p2, q2)["kl_bits"]>0
    assert truncated_kl(p2, [{"token":9,"probability":1.0}])["kl_bits"] is None

def test_thin_support_is_counted_beside_the_kl():
    # one shared candidate out of two: the conditional KL is zero by construction
    # and the reading says the support was thin and single
    a={"output_tokens":[1],"emission":"a","entropies":[0.5],"field":{0:{"ranked":[{"token":1,"probability":0.5},{"token":2,"probability":0.3}],"realized":0}}}
    b={"output_tokens":[1],"emission":"a","entropies":[0.5],"field":{0:{"ranked":[{"token":1,"probability":0.9},{"token":9,"probability":0.05}],"realized":0}}}
    two=reading_two(a,b)
    assert two["field_kl_bits_max"]==0 and two["field_positions_single_shared_candidate"]==1
    assert two["field_positions_thin_support"]==0  # shared 0.5 of ranked 0.8 is not below half
    b["field"][0]["ranked"]=[{"token":1,"probability":0.1},{"token":9,"probability":0.8}]
    a["field"][0]["ranked"]=[{"token":1,"probability":0.1},{"token":2,"probability":0.8}]
    two=reading_two(a,b)
    assert two["field_positions_thin_support"]==1, two


def test_a_shorter_run_diverges_at_its_end():
    assert first_divergence([1,2],[1,2,3]) == 2
    assert first_divergence([1,2,3],[1,2,3]) is None

def test_readings_shape():
    a={"output_tokens":[1,2,3],"emission":"abc","entropies":[0.5,0.5,0.5],"field":{0:{"ranked":[{"token":1,"probability":1.0}],"realized":0}}}
    b={"output_tokens":[1,2,4],"emission":"abd","entropies":[0.5,0.5,0.7],"field":{0:{"ranked":[{"token":1,"probability":1.0}],"realized":0}}}
    one=reading_one(a,b)
    assert one["first_token_divergence"]==2
    assert one["first_char_divergence"]==2
    assert not one["identical"]
    two=reading_two(a,b)
    assert two["entropy_positions_exact"]==2
    assert two["entropy_first_difference_ordinal"]==2
    assert two["field_kl_bits_max"]==0

if __name__=="__main__":
    for n,f in sorted(globals().items()):
        if n.startswith("test_"): f(); print("ok", n)


def test_the_field_reports_its_floor_and_both_coordinates():
    # The field is keyed by resident position and starts after the prompt,
    # here at 154, while the entropies are indexed by output ordinal.
    same=[{"token":1,"probability":0.9},{"token":2,"probability":0.1}]
    moved=[{"token":1,"probability":0.6},{"token":2,"probability":0.4}]
    a={"entropies":[0.5,0.5,0.5],"output_tokens":[1,1,1],"field":{"154":{"ranked":same},"155":{"ranked":same},"156":{"ranked":same}}}
    b={"entropies":[0.5,0.5,0.7],"output_tokens":[1,1,1],"field":{"154":{"ranked":same},"155":{"ranked":same},"156":{"ranked":moved}}}
    two=reading_two(a,b)
    assert two["field_first_position"]==154
    assert two["field_first_nonzero_kl_position"]==156 and two["field_first_nonzero_kl_ordinal"]==2
    assert two["entropy_first_difference_ordinal"]==2
    empty=reading_two({"entropies":[],"output_tokens":[],"field":{}},{"entropies":[],"output_tokens":[],"field":{}})
    assert empty["field_first_position"] is None and empty["field_first_nonzero_kl_ordinal"] is None


def test_extract_run_keeps_the_input_count_the_divergence_is_read_against():
    events=[{"kind":"model.measurement","payload":{"input_tokens":[1]*127,"output_tokens":[5,6],"entropies":[0.1,0.2]}},
            {"kind":"model.field","payload":{"position":154,"ranked":[],"realized":0}}]
    ex=extract_run(events)
    assert ex["input_tokens"]==127 and ex["output_tokens"]==[5,6] and list(ex["field"])==[154]
    assert extract_run([])["input_tokens"] is None


def test_a_run_of_only_its_close_is_not_measured():
    # The close carries the run id, so filtering by run alone is never empty.
    for kind in ("certified", "diverged"):
        close={"kind":"replay.closed","run":"r1","payload":{"outcome":{"kind":kind}}}
        assert measured_events([close], "r1") is None
        assert measured_events([close, {"kind":"model.field","run":"r1","payload":{}}], "r1") is None
        measured={"kind":"model.measurement","run":"r1","payload":{"input_tokens":[1],"output_tokens":[2],"entropies":[0.1]}}
        got=measured_events([close, measured, {"kind":"model.measurement","run":"other","payload":{}}], "r1")
        assert got==[close, measured]
        assert extract_run(got)["output_tokens"]==[2]


def test_the_divergence_ordinal_is_the_position_less_the_fields_floor():
    # The position is the resident length at the draw, the field's key, per
    # weaver-diagnostic-Spec section 3.3 on the ruling of 2026-09-09.
    ex={"input_tokens":127,"output_tokens":[5,6],"field":{154:{},155:{}}}
    assert divergence_ordinal({"kind":"token_path","position":176}, ex)==22
    assert divergence_ordinal({"kind":"token_path","position":"176"}, ex)==22
    assert divergence_ordinal({"kind":"token_path","position":150}, ex)==-4
    assert divergence_ordinal({"kind":"readout","position":176,"layer":3}, ex) is None
    assert divergence_ordinal(None, ex) is None
    assert divergence_ordinal({"kind":"token_path","position":176}, {"field":{}}) is None
