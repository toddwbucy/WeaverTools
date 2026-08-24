-- Roles, per the operator's ruling of 2026-08-19: the user surface
-- (gate boundary) and the admin surface (operator boundary) separate
-- now, so IAM later attaches authentication to existing roles rather
-- than rearchitecting. v1 assignment is operator declaration in config.

ALTER TABLE participants
  ADD COLUMN role TEXT NOT NULL DEFAULT 'user';
