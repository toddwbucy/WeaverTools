-- Spec 2.9: the plan, and Spec 2.10: the refs.
--
-- A run row says what a run did and a staged experiment says what was
-- registered. The plan is the row for what an operator is still composing,
-- so that a column trimmed before registration is still a fact about what
-- was considered rather than nothing at all.

-- Spec 2.9: one parent run, which every column branches from, and the
-- author and version every authored row carries per section 3.2. The author
-- has no default: a null is passed rather than fallen into, so an author
-- nobody could name stays distinguishable from one nobody asked for.
CREATE TABLE plan (
  plan_id        BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  parent_run_id  TEXT NOT NULL REFERENCES run(run_id),
  author         TEXT,
  version        BIGINT NOT NULL DEFAULT 1
);

-- Spec 2.9: a column becomes at most one staged experiment and holds a
-- nullable reference to the one it became. Spec 5.1: the column has no
-- state of its own - its status is that row's state, and a null reference
-- records that it has not been registered.
--
-- **The reference is unique**, which is what holds the claim at most once:
-- two registrations racing on one column leave one staged experiment and
-- the loser finds the column already registered. Without it one column
-- would carry two frozen experiments and the matrix would read two arms
-- where the operator authored one.
CREATE TABLE plan_column (
  plan_id        BIGINT NOT NULL REFERENCES plan(plan_id) ON DELETE CASCADE,
  column_key     TEXT NOT NULL,
  experiment_id  BIGINT UNIQUE REFERENCES staged_experiment(experiment_id),

  PRIMARY KEY (plan_id, column_key)
);

-- Spec 2.9: an entry names a member of the tuple, carries its disposition,
-- and carries the value where it is held or the value set where it is
-- freed.
--
-- **The disposition is the entry's own and the status is the column's.** An
-- entry that carried both could not say what a freed entry in a queued
-- column is, which is both at once.
CREATE TABLE plan_entry (
  plan_id      BIGINT NOT NULL,
  column_key   TEXT NOT NULL,
  member       TEXT NOT NULL,

  disposition  TEXT NOT NULL,
  held_value   JSONB,
  freed_values JSONB,

  PRIMARY KEY (plan_id, column_key, member),
  FOREIGN KEY (plan_id, column_key)
    REFERENCES plan_column(plan_id, column_key) ON DELETE CASCADE,

  CONSTRAINT plan_entry_disposition_is_held_or_freed
    CHECK (disposition IN ('held', 'freed')),

  -- The disposition and the value are one fact stated once: a held entry
  -- carries the value it holds, a freed entry carries the set it frees, and
  -- neither carries the other's member.
  CONSTRAINT plan_entry_states_the_value_its_disposition_names
    CHECK (
      (disposition = 'held'
        AND held_value IS NOT NULL AND freed_values IS NULL)
      OR
      (disposition = 'freed'
        AND freed_values IS NOT NULL AND held_value IS NULL)
    ),

  CONSTRAINT plan_entry_freed_values_is_an_array
    CHECK (freed_values IS NULL OR jsonb_typeof(freed_values) = 'array')
);

-- Spec 2.9: a column frees at most one member and holds the rest, section
-- 5.4 having a sweep name one member and its value set. A column that frees
-- none is the ordinary point experiment rather than a sweep, so the bound
-- is one and not exactly one.
CREATE UNIQUE INDEX plan_column_frees_at_most_one_member
  ON plan_entry (plan_id, column_key) WHERE disposition = 'freed';

-- Spec 2.10: a named reference from a person to a run, and the one thing
-- this document takes from a commit graph. A run reachable from no root is
-- scaffolding; what must survive is what a bound cites, and a ref is how a
-- run says so.
--
-- Nothing in this document deletes a run. This row exists so that a later
-- act which does can tell what it is deleting.
CREATE TABLE ref (
  ref_id   BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  name     TEXT NOT NULL,
  run_id   TEXT NOT NULL REFERENCES run(run_id),
  author   TEXT,
  version  BIGINT NOT NULL DEFAULT 1,
  made_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Spec 2.7. The roots of section 2.10's reachability are three - a ref, a
-- plan's parent run, and the parent run of a staged experiment that has not
-- returned - and two of the three are read by these. The third is the
-- lineage index 0007 already carries.
CREATE INDEX ref_by_run ON ref (run_id);
CREATE INDEX plan_by_parent ON plan (parent_run_id);
