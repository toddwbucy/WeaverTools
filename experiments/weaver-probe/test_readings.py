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
    k=truncated_kl(r, r); assert k["kl_bits"]==0 and k["shared_candidates"]==2 and abs(k["retained_mass_p"]-0.9)<1e-9
    q=[{"token":1,"probability":0.3},{"token":3,"probability":0.6}]
    k=truncated_kl(r, q); assert k["kl_bits"]>0 and k["shared_candidates"]==1 and abs(k["excluded_mass_p"]-0.3)<1e-9

def test_readings_shape():
    a={"output_tokens":[1,2,3],"emission":"abc","entropies":[0.5,0.5,0.5],"field":{0:{"ranked":[{"token":1,"probability":1.0}],"realized":0}}}
    b={"output_tokens":[1,2,4],"emission":"abd","entropies":[0.5,0.5,0.7],"field":{0:{"ranked":[{"token":1,"probability":1.0}],"realized":0}}}
    one=reading_one(a,b); assert one["first_token_divergence"]==2 and one["first_char_divergence"]==2 and not one["identical"]
    two=reading_two(a,b); assert two["entropy_positions_exact"]==2 and two["entropy_first_difference"]==2 and two["field_kl_bits_max"]==0

if __name__=="__main__":
    for n,f in sorted(globals().items()):
        if n.startswith("test_"): f(); print("ok", n)
