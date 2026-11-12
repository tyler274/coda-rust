---
name: swarm-coordinator
description: Coordinate a swarm of worker agents over the coda-swarm board MCP server. Use when work splits into independent parts that separate agents can do at once, when the user asks for a swarm or parallel workers, or when you are the coordinator on an existing swarm board.
---

# Swarm coordinator

The `coda-swarm` MCP server is a shared message board. In Cursor you join it as
`coordinator`, the member that adds, re-models and kills workers. Every worker
is another agent with its own connection to the same board. The board lives in
`.swarm/` at the repository root, which git ignores. It survives Cursor
restarts, so a swarm you started earlier is still there.

## Decide whether to swarm

Start a swarm only when the work has parts that do not need each other's
results to begin: separate crates, separate test failures, a survey across
many files, or competing approaches to one design. Do the work yourself when it
is one sequential change, when it is small enough to finish in a few edits, or
when every part would touch the same files. Each worker costs a full agent
session, and a brief you could not write in a few paragraphs is a sign the
split is wrong.

## Start and inspect

1. Call `swarm` with `action: "status"` to see the roster and the board head.
   If workers from an earlier run are still live, decide whether to reuse or
   kill them before adding more.
2. Call `swarm` with `action: "title"` to name the swarm after the goal.
3. Call `swarm` with `action: "models"` to see which models `add` accepts.
   `any: true` means the server does not restrict them.

## Add workers

Call `swarm` with `action: "add"`, a `name`, a `brief`, and usually a `model`.
The server posts the brief to the worker as you, so the brief is the worker's
entire starting context.

- One worker: `{"action": "add", "name": "parser", "brief": "...", "model": "..."}`.
- A group sharing a brief: add `n` or `tasks`. With `tasks`, member `k` gets
  the shared brief followed by task `k`, and the group is named `name-1` to
  `name-n`. Mention the whole group as `@name`.
- `workspace` picks the checkout: `inherit` shares yours, `share` gives each
  worker its own worktree, and `branch` gives each its own clone. Give workers
  their own worktree whenever two of them might edit the same files.
- `channels` subscribes the worker to channels; the first one is where its
  posts go by default. Create a channel first with
  `{"action": "channel", "op": "create", "channel": "...", "topic": "..."}`.

Pick the model by the job. Use a strong model for design judgment or subtle
debugging, and a fast one for mechanical edits, searches and test runs. Switch
a worker later with `{"action": "model", "name": "...", "model": "..."}`.

## Write a brief that stands alone

A worker cannot see your chat, your open files or what the user told you. Each
brief says:

- The goal and why it matters, in the user's terms.
- The exact scope: paths, crates, branch or worktree, and what is out of bounds.
- Facts you already established, so the worker does not rediscover them.
- How to verify: the commands to run and what passing looks like.
- What to post back and when: a summary with `@coordinator`, the commit or
  file list, and any blocker the moment it appears.

Do not write "as discussed" or "the bug above". Paste the error, the path or
the decision itself.

## Read and post

- `post_message` with `text` posts to your home channel, or to `channel` when
  given. Mention `@name`, `@group`, `@all` or `@#channel` to reach members.
- `read_board` without `since` returns the newest messages. With `since`, it
  pages forward from that seq. Both advance your read cursor.
- Keep the highest seq you have read. Every wait and read after it starts there.

## Wait instead of polling

After you hand out briefs, call `swarm_wait` with `since` set to the highest
seq you have read. It parks until a post mentions you and then returns every
message you can see after `since`. Set `mentions_only: false` to wake for any
post. It returns `woken: false` after `timeout` seconds, 120 by default and at
most 3600; call it again with the same `since`.

Never loop on `read_board` or `swarm status` to watch for progress. Each call
spends context and a turn, and `swarm_wait` already returns the moment a
worker reports. Workers only wake you when they mention `@coordinator`, so say
so in the brief.

## Steer, stop and finish

- Answer a worker's question by posting with its `@name`. Redirect one by
  posting the new instruction. The brief rule applies: make the post stand on
  its own.
- Kill a worker that is done, stuck or no longer needed with
  `{"action": "kill", "name": "..."}`. A group base kills the whole group. Kill
  is final for that name; add a new name to restart the work.
- Before you report to the user, check each worker's result yourself: read the
  diff, rerun its verification, and reconcile workers that disagree. A worker's
  "done" is a claim, not evidence.
- Kill every worker you added once the work is reconciled.

## Show the board

Call `board_view` when the user asks what the swarm is doing, after adding
workers, and when you report. In Cursor it renders as a live board with
members, their state and model, channels and recent messages, and it refreshes
itself while it is open. Hosts without MCP Apps get a plain-text summary of the
same snapshot. `board_view` does not move your read cursor, so it never hides
messages from your next `swarm_wait`.
