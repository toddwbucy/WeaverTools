# Pre-registration record: runs 5 and 6 deferred

Runs 5 (the guest CPU cell) and 6 (the driver-branch cell) of
`2026-11-15-preregistration.md` were not run on 2026-11-16. No turn of either ran.

Runs 1 to 4 ran first, in the pre-registration's order. Run 5 was next. The guest of
runs 3 and 4 was shut down and its virtual CPU was set from host-passthrough to the named
model EPYC-Milan, with `match='exact'`, `fallback='forbid'` and `check='full'`. The
hypervisor refused to start the guest at 2026-11-16T03:51:08Z:

```text
error: operation failed: guest CPU doesn't match specification: extra features:
x2apic,hypervisor,topoext, missing features: svm,npt,nrip-save,svme-addr-chk
```

`check='full'` compares the CPU the emulator builds with the named model, feature by
feature. The hypervisor always adds `x2apic` and `hypervisor` and the emulator adds
`topoext`, so no guest on that model can pass this check on this host. The defect is in
the measurement's own script, which a cardless start had not tested. The guest and the
card did not cause it. Run 6 was to follow run 5 in a second guest and was not reached.

On 2026-11-16 the operator ruled that the next measurement, the first agent's model on
the host and in a guest, goes first. Runs 5 and 6 are deferred, not dropped. They run
later under the same pre-registration. First, a cardless guest must start on EPYC-Milan
with `check='partial'`, which checks before the start that the host can supply the model,
with the model and `fallback='forbid'` kept. Nothing else in the pre-registration
changes.
