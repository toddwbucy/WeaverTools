"""The probe's pure readings, per issue #511."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from weaver_probe import first_divergence, aligned_agreement_after, truncated_kl, reading_one, reading_two

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
    one=reading_one(a,b); assert one["first_token_divergence"]==2 and one["first_char_divergence"]==2 and not one["identical"]
    two=reading_two(a,b); assert two["entropy_positions_exact"]==2 and two["entropy_first_difference"]==2 and two["field_kl_bits_max"]==0

if __name__=="__main__":
    for n,f in sorted(globals().items()):
        if n.startswith("test_"): f(); print("ok", n)
