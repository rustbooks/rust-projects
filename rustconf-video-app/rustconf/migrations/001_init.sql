-- Migration 001: initial schema

CREATE TABLE IF NOT EXISTS meetings (
    id          TEXT PRIMARY KEY,           -- UUID v4
    passkey     TEXT NOT NULL,              -- SHA-256 hex of the passkey
    host_token  TEXT NOT NULL,              -- JWT for the host
    title       TEXT NOT NULL DEFAULT '',
    created_at  INTEGER NOT NULL,           -- Unix timestamp
    closed_at   INTEGER                     -- NULL = still open
);

CREATE TABLE IF NOT EXISTS participants (
    id          TEXT PRIMARY KEY,           -- UUID v4
    meeting_id  TEXT NOT NULL REFERENCES meetings(id) ON DELETE CASCADE,
    display_name TEXT NOT NULL,
    role        TEXT NOT NULL DEFAULT 'participant',  -- 'host' | 'participant'
    joined_at   INTEGER NOT NULL,
    left_at     INTEGER
);

CREATE INDEX IF NOT EXISTS idx_participants_meeting ON participants(meeting_id);
