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
   A `note` saying no model registry is configured means added workers are
   roster entries only and nothing runs them.

Your server runs workers only with a model registry. Copy
`crates/workspace-tools/swarm/models.example.json` to `.swarm/models.json`
(gitignored), keep the models you can run, and restart the `coda-swarm` MCP
server; `.cursor/mcp.json` passes that file with `--models`. CLI models carry
no `key`: in the sandbox container the `claude`, `codex` and `gemini`
wrappers fetch their own. HTTP models name a broker item in `key`, which the
server fetches with `coda-sandbox token <item>`. Workers run as
children of your server: closing the session or stopping the server stops
every worker and marks it stopped on the board.

## Add workers

Call `swarm` with `action: "add"`, a `name`, a `brief`, and usually a `model`.
The server posts the brief to the worker as you, so the brief is the worker's
entire starting context.

- One worker: `{"action": "add", "name": "parser", "brief": "...", "model": "..."}`.
- A group sharing a brief: add `n` or `tasks`. With `tasks`, member `k` gets
  the shared brief followed by task `k`, and the group is named `name-1` to
  `name-n`. Mention the whole group as `@name`.
- `workspace` picks the checkout: `inherit` shares yours, `share` gives the
  whole group one worktree, and `branch` gives each worker its own worktree.
  Use `branch` whenever two workers might edit the same files.
- `fork` names a member whose session each new worker continues, when both
  models run on the same CLI or API.
- `channels` subscribes the worker to channels; the first one is where its
  posts go by default. Create a channel first with
  `{"action": "channel", "op": "create", "channel": "...", "topic": "..."}`.

Pick the model by the job. Use a strong model for design judgment or subtle
debugging, and a fast one for mechanical edits, searches and test runs. Switch
a worker later with `{"action": "model", "name": "...", "model": "..."}`.

## Add a narrator or monitor

For a long run, add an observer with `role` on `add`. Only you can add one.
Each also takes a turn on its own timer, so a cheap fast model is enough.

- `"role": "narrator"` posts a short progress summary for the user about every
  5 minutes while others post. Its posts are summaries that wake no one, so
  read them when you report rather than waiting on them. Relay user feedback
  to it with `@narrator`.
- `"role": "monitor"` checks about every 3 minutes while a worker is live and
  posts a `[stall]`, `[loop]` or `[quota]` line that mentions you when a worker
  goes quiet, repeats itself or is paused for quota. Treat an alert as a lead:
  read the cited posts before you act.

The brief for an observer only needs the goal and what the user cares about;
the role instructions come with it.

When a worker's turn ends with unread posts that mention it, the supervisor
gives it one more turn to read and answer them, once per new post. You do not
need to repeat a question because a worker ended its turn early.

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
