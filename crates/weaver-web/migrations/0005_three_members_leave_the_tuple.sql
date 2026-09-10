-- Two members of the run's tuple had no producer anywhere in the corpus and
-- leave it, per weaver-web-Spec section 2.2 as revised 2026-09-09 on the
-- operator's rulings at issue #532. The third, the declared boundary set,
-- stays and is written empty, which is a fact about every run this ingest
-- can meet rather than a default.

-- The batching election records a choice the program does not offer:
-- weaver-spu-Spec has one forward per prompt and never a batch, and every
-- use of the word elsewhere in the corpus is the serving technique the
-- architecture refuses. A member nobody can fill is not a member.
ALTER TABLE run DROP COLUMN batching;

-- Precision is the artifact's and not the run's. No organ reports a dtype
-- and the declaration excludes it, quantization being a deployment's
-- knowledge per weaver-spu-Spec, while section 2.2's record identity is
-- already at a grain fine enough to catch a quantization difference. So the
-- tuple loses no distinction, and what a person reads is a label on the
-- artifact, authored by Models at import.
ALTER TABLE run DROP COLUMN compute_precision;
ALTER TABLE artifact ADD COLUMN precision_label TEXT;

-- The device model and the engine libraries come from a deposit the caller
-- names, per weaver-analysis-web-contract section 2.2, and a caller may name
-- none: the sink may be a pipe, and a record written before deposits were
-- kept has one nowhere. Absent rather than defaulted, per Spec section 6, a
-- run whose silicon nobody recorded being a run this store holds rather than
-- refuses.
ALTER TABLE run ALTER COLUMN device DROP NOT NULL;
ALTER TABLE run ALTER COLUMN engine DROP NOT NULL;

-- Spec 2.2: the task's verdict, the predicate the task answered and the
-- ratio over its denominator. The seam carries it as of this act and the
-- trace kind that produces it is owed at issue #523, so the column reads
-- absent on every row until that kind lands.
ALTER TABLE run ADD COLUMN task_verdict JSONB;

-- Spec 2.3's own sentinel clause says a hash the SPU could not compute
-- crosses as the empty string and joins to nothing, which is a run recorded
-- as one whose identity failed. The check on this column refused exactly
-- that, making such a run unrecordable rather than recorded, where the
-- catalog's own check is what keeps the sentinel out of the catalog.
ALTER TABLE run DROP CONSTRAINT run_record_identity_is_not_the_sentinel;
