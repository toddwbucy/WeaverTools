-- weaver-web store schema, per docs/SPEC.md section 4.

CREATE TABLE participants (
  id          BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  name        TEXT NOT NULL UNIQUE,
  display     TEXT NOT NULL,
  kind        TEXT NOT NULL,
  adapter     TEXT,
  respond     TEXT NOT NULL DEFAULT 'mention',
  credential  BYTEA
);

CREATE TABLE channels (
  id          BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  name        TEXT NOT NULL UNIQUE,
  topic       TEXT,
  created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE members (
  channel_id     BIGINT NOT NULL REFERENCES channels(id),
  participant_id BIGINT NOT NULL REFERENCES participants(id),
  joined_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
  PRIMARY KEY (channel_id, participant_id)
);

CREATE TABLE channel_events (
  id             BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  channel_id     BIGINT NOT NULL REFERENCES channels(id),
  ts             TIMESTAMPTZ NOT NULL DEFAULT now(),
  participant_id BIGINT REFERENCES participants(id),
  kind           TEXT NOT NULL,
  body           TEXT,
  run_label      TEXT,
  turn_label     TEXT,
  close_kind     TEXT
);
CREATE INDEX idx_events_channel ON channel_events(channel_id, id);

CREATE TABLE sessions (
  id             BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  token          TEXT NOT NULL UNIQUE,
  participant_id BIGINT REFERENCES participants(id),
  opened_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
  closed_at      TIMESTAMPTZ
);
