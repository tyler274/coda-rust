# Session sources

Use the invoking developer's available history. Start with the host's session
listing and reading tools, paging as needed to cover the requested period and
including accessible remote-host sessions. A sidebar listing may omit archived
work; an updated timestamp alone does not establish substantive activity.

When native history is unavailable or incomplete, use permitted local files or
user-provided exports. Resolve paths from that developer's configuration and home
directory. Do not assume another developer's installation, OS or account layout.
If a source is unavailable, describe the coverage gap; do not claim to have read it.

## Codex

When provided by the host, use `list_threads` and `read_thread`, with archive
listing when needed. Prefer bounded pages over dumping complete transcripts.

For local discovery, inspect the current session index or read-only thread
metadata under the configured Codex home (commonly `~/.codex`). Session files may
be in `sessions/` and `archived_sessions/`. Inspect the actual database schema
before querying it; database names and layouts vary by version. In JSONL records,
conversational messages commonly appear as `response_item` payloads; use message
timestamps and roles, not file modification times. Inspect representative records
before choosing fields, and avoid counting mirrored event records twice.

## Claude Code

Use available session metadata or exports. Local metadata may be under
`~/.claude/sessions/`; CLI transcripts commonly live under
`~/.claude/projects/`. On macOS, Claude Desktop may also keep session metadata in
`~/Library/Application Support/Claude/claude-code-sessions/`. Resolve the session's
CLI ID to its transcript rather than assuming that metadata is the conversation.

Read `user` and `assistant` messages, handling string and typed-block content.
Use message timestamps to include the day's work from older sessions. Treat
compaction summaries and tool notifications as context, not separate progress.

## Other sources

Use the editor's supported history, supplied notes, and GitHub or tracker evidence
that the developer can access. A commit or PR can establish a change and its state;
it may not establish the original intent, authorship of every contribution or
all work done that day. Read configured team notes or a vault for priorities when
available, while checking current status with its actual owner.
