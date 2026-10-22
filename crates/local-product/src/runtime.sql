-- Physician-owned operational persistence; explicit migration job only.
CREATE TABLE IF NOT EXISTS physician_runtime_conversations (
    practice TEXT NOT NULL,
    physician TEXT NOT NULL,
    workspace TEXT NOT NULL,
    chat TEXT NOT NULL,
    state JSONB NOT NULL,
    PRIMARY KEY (practice, physician, workspace, chat)
);

CREATE TABLE IF NOT EXISTS physician_runtime_commands (
    practice TEXT NOT NULL,
    physician TEXT NOT NULL,
    key TEXT NOT NULL,
    command JSONB NOT NULL,
    PRIMARY KEY (practice, physician, key)
);
