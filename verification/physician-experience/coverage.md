# Physician app coverage

What the physician app shows today for each state of [the graph](physician.mmd).
The graph says what the product should do; this file says how much of it the
Rust app shows, so built, partial and missing UI stay explicit. It is the
starting list for the UI program.

- **built:** the app shows this state as the graph means it.
- **partial:** something shows, but not everything the graph promises; the note
  says what's missing.
- **missing:** the app shows nothing for it yet.
- **not shown:** the graph state has nothing a physician sees, and the app
  correctly shows nothing.

**Server data** says whether the wire and the sample server already carry what
the state needs (yes), need a server or wire change first (no), or don't apply
(n/a). **Where in the app** names the owner by component, not line number.

Sign-in's page states are in [sign-in.mmd](sign-in.mmd), which is connected to
the graph by its own map; this file covers the graph's states.

Check it from the repository root. It fails when the graph gains or drops a
state this file doesn't match:

```bash
node .agents/skills/coda-statechart/scripts/statechart.mjs coverage verification/physician-experience/physician.mmd verification/physician-experience/coverage.md
```

When the app or the graph changes a state, update its row in the same PR. The
check prints the current counts.

# States by region

## Practice membership

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| A member of the practice | built | PhysicianRoot, PhysicianShell sidebar footer (physician-app lib.rs) | yes | A signed-in member gets the practice and the sidebar names the physician; the graph needs no membership marker. |
| Not a member of the practice | partial | SignInScreen NoAccess stage, SessionState::refused (physician-app lib.rs) | yes | A 403 on a link shows "No access to this practice", but access removed mid-session (401 access_removed) drops to the plain sign-in page with nothing saying access ended. |

## Physician session

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Signed out.Sign-in page | built | SignInScreen (physician-app lib.rs) | yes | Its page states (asking for the email, sending, check your email, couldn't send) are in sign-in.mmd, which follows this graph in both directions. |
| Signed out.No access to this practice | built | SignInScreen, SignInStage::NoAccess (physician-app lib.rs) | yes | Shown when a sign-in link's verification refuses access, with a way to try another account. Losing access mid-session is the Practice membership region's row. |
| Signed in | built | PhysicianRoot, SessionState (physician-app lib.rs) | yes | Everything shown while signed in sits under it and closes with the session. Other tabs follow sign-in and sign-out through BroadcastChannel news (tabs.rs). |

## Physician session/Signed in/Sign-in state

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Signed in to the practice | built | PhysicianRoot, SessionState (physician-app lib.rs) | yes |  |
| Session expired, sign in to continue | built | PhysicianShell expired-session prompt, SignInScreen when resuming (physician-app lib.rs) | yes | The prompt hides patient content, and signing in again returns to the same place (#95). |

## Physician session/Signed in/Screen

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Today | built | TodaySurface, TodayRows (physician-app lib.rs) | yes | Heading with need-you count, rows with a Needs you now mark for urgency now, and the quiet copy when empty. |
| Patient page | built | PatientSurface, ReadNotice (physician-app lib.rs); answer_to (physician-app shell.rs) | yes | Opening the patient… shows whenever the patient being opened hasn't answered yet; the last patient's page never stands in for the next. |
| Conversation.Source pane in the conversation.No source open in the conversation | partial | PatientSurface Conversation section, FormattedMessage (physician-app lib.rs) | no | The conversation is a section of the patient page, not its own screen, and its answers carry no citations, so no source can open. |
| Conversation.Source pane in the conversation.Source open in the conversation.Citation preview in the conversation | missing |  | no | Conversation turns render through FormattedMessage with no citation chips; wire ChatTurn carries only text/authored_text. |
| Conversation.Source pane in the conversation.Source open in the conversation.Loading the source in the conversation | missing |  | no | Conversation turns render through FormattedMessage with no citation chips; wire ChatTurn carries only text/authored_text. |
| Conversation.Source pane in the conversation.Source open in the conversation.Source text shown in the conversation | missing |  | no | Conversation turns render through FormattedMessage with no citation chips; wire ChatTurn carries only text/authored_text. |
| Conversation.Source pane in the conversation.Source open in the conversation.Preview kept in the conversation, source couldn't load, with Retry | missing |  | no | Conversation turns render through FormattedMessage with no citation chips; wire ChatTurn carries only text/authored_text. |
| Conversation.Source pane in the conversation.Source open in the conversation.Preview only in the conversation, no qualified source text | missing |  | no | Conversation turns render through FormattedMessage with no citation chips; wire ChatTurn carries only text/authored_text. |
| Patient identity only | built | PatientSurface clinic-demographic card (physician-app lib.rs) | yes | Driven by clinic_demographic membership, since the wire has no separate access level; admins get Assign to panel. |
| That patient is not available | built | PatientSurface gone branch, workspace resource 404 handling (physician-app lib.rs) | yes | Shows That patient is not available. with Go to Today on any 404, but an open reader for that patient is not closed. |

## Physician session/Signed in/Screen/Patient page/Basis and limitations

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Basis closed | missing |  | no | No Basis control; PatientWorkspaceResponse has no basis or limitations field. |
| Basis open | missing |  | no | No Basis control; PatientWorkspaceResponse has no basis or limitations field. |

## Physician session/Signed in/Screen/Patient page/Sources list

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Sources closed | missing |  | no | No Sources control; only a Records as of line in the summary meta. |
| Sources open | missing |  | no | No list of sources read; ConnectedSource carries only pull state and times, no source names. |

## Physician session/Signed in/Screen/Patient page/Source pane on the patient page

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No source open on the patient page | built | PatientSurface Patient summary, CitationChip (physician-app lib.rs) | yes | Summary prose with numbered citation chips and no pane open. |
| Source open on the patient page.Citation preview on the patient page | built | CitationChip, ReaderDialog, source_reading (physician-app lib.rs); citation_inspection (physician-app shell.rs) | yes | Shown as a modal sharing the reader's single slot, so it replaces any open reading, and it has no Expand. |
| Source open on the patient page.Loading the source on the patient page | missing |  | no | No Expand and no source fetch; the client has no source endpoint. |
| Source open on the patient page.Source text shown on the patient page | missing |  | no | No qualified source text, only the preview on the citation segment. |
| Source open on the patient page.Preview kept on the patient page, source couldn't load, with Retry | missing |  | no | No source load, so no failure or Retry. |
| Source open on the patient page.Preview only on the patient page, no qualified source text | partial | citation_inspection (physician-app shell.rs), ReaderDialog (physician-app lib.rs) | no | Every preview is preview-only but nothing says so; SOURCE_UNAVAILABLE shows only when the preview prose itself is empty. |

## Physician session/Signed in/Reader

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No reader open | built | PhysicianShell reading signal, ReaderDialog (physician-app lib.rs) | yes | The reader renders only while a reading is open. |
| Following a link | partial | open_patient (physician-app main.rs), SessionState::hear (physician-app lib.rs) | no | A ?patient= link lands on the patient page by access, even after sign-in in another tab, but links carry no item, so no concern, briefing, follow-up or past work opens. |
| Reader open | partial | ReaderDialog, prose_reading (physician-app lib.rs) | no | A modal showing text already in hand, with no Discuss action, item identity or fetched reading. |

## Physician session/Signed in/Reader/Reader open/What is open

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Reading the concern | partial | TodaySurface on_open_row, prose_reading, ReaderDialog (physician-app lib.rs) | no | Shows the patient name and the row's reason only; the Today row has no concern body, Done or link to the work that raised it. |
| Reading the briefing | partial | PatientSurface ScheduleRail and Current work on_read, ReaderDialog (physician-app lib.rs) | no | Shows the visit label and preparation sentence only; VisitPreparation has no briefing body. |
| Reading the follow-up | missing |  | no | Follow-ups are plain description lines with nothing to open; FollowUp carries only a description. |
| Reading past work | partial | PastWorkDialog (physician-app lib.rs); past_work_copy (physician-app shell.rs) | no | Always says No past conversations or work are available. though offered only when has_past_work is true; the wire has a flag and no items. |

## Physician session/Signed in/Reader/Reader open/Source pane in the reader

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No source open in the reader | partial | ReaderDialog (physician-app lib.rs) | no | Trivially true since reader content has no citations; a summary citation replaces the reading instead of opening a pane in it. |
| Source open in the reader.Citation preview in the reader | missing |  | no | Reader content has no citation chips and the client has no source fetch. |
| Source open in the reader.Loading the source in the reader | missing |  | no | Reader content has no citation chips and the client has no source fetch. |
| Source open in the reader.Source text shown in the reader | missing |  | no | Reader content has no citation chips and the client has no source fetch. |
| Source open in the reader.Preview kept in the reader, source couldn't load, with Retry | missing |  | no | Reader content has no citation chips and the client has no source fetch. |
| Source open in the reader.Preview only in the reader, no qualified source text | missing |  | no | Reader content has no citation chips and the client has no source fetch. |

## Physician session/Signed in/Reader/Reader open/Reading load

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Loading the reading | missing |  | no | The reader never loads; it shows strings already on the row or visit. |
| Showing the reading | partial | ReaderDialog (physician-app lib.rs) | no | Shows in-hand text at once, and an empty body reads Nothing is open to read. |
| Showing the reading, with one quiet line and Retry | missing |  | no | No reading request, so no partial or failed refresh. |
| Couldn't load the reading, with Retry | missing |  | no | No reading request. |

## Physician session/Signed in/Updates panel

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Updates closed | built | PhysicianShell sidebar Updates nav item (physician-app lib.rs) | yes | Updates is a main-stage place that replaces Today or the patient page, not a panel over it. |
| Updates open | built | UpdatesSurface (physician-app lib.rs) | yes | No Close: you leave by picking another nav item, which does not return you to where you were. |

## Physician session/Signed in/Updates panel/Updates open/Updates view

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Updates list | partial | UpdatesSurface (physician-app lib.rs) | no | Rows show only the patient name or Update, with no what Coda did, time or seen state, and can't be opened; rows are untyped and the sample fixture is empty. |
| Reading an update.Update read.Loading the update | missing |  | no | Update rows are not openable, and the client has no call to read one update. |
| Reading an update.Update read.Showing the update | missing |  | no | Update rows are not openable, and the client has no call to read one update. |
| Reading an update.Update read.Showing the update, with one quiet line and Retry | missing |  | no | Update rows are not openable, and the client has no call to read one update. |
| Reading an update.Update read.Couldn't load the update, with Retry | missing |  | no | Update rows are not openable, and the client has no call to read one update. |

## Physician session/Signed in/Updates panel/Updates open/Source pane in Updates

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No source open in Updates | partial | UpdatesSurface (physician-app lib.rs) | no | Updates has no readings, so there are no citations to open. |
| Source open in Updates.Citation preview in Updates | missing |  | no | Updates has no readings, so there are no citations to open. |
| Source open in Updates.Loading the source in Updates | missing |  | no | Updates has no readings, so there are no citations to open. |
| Source open in Updates.Source text shown in Updates | missing |  | no | Updates has no readings, so there are no citations to open. |
| Source open in Updates.Preview kept in Updates, source couldn't load, with Retry | missing |  | no | Updates has no readings, so there are no citations to open. |
| Source open in Updates.Preview only in Updates, no qualified source text | missing |  | no | Updates has no readings, so there are no citations to open. |

## Physician session/Signed in/Updates panel/Updates open/Updates day read

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Loading the Updates day | built | UpdatesSurface, use_read_view, ReadNotice (physician-app lib.rs) | yes | Loading updates… on the first read only; later reads keep the last list. |
| Showing the Updates day | built | UpdatesSurface (physician-app lib.rs) | yes | Row list or the empty copy; rows themselves are thin (see Updates list). |
| Showing the Updates day, with one quiet line and Retry | built | UpdatesSurface, use_read_view, ReadNotice (physician-app lib.rs) | yes | A failed refresh keeps the last list under a quiet line with Try again; a part-loaded day isn't distinguished. |
| Couldn't load the Updates day, with Retry | built | UpdatesSurface, ReadNotice (physician-app lib.rs) | yes | Coda couldn’t load Updates. with Try again, which re-reads all of the shell's reads. |
| Showing the Updates day, loading more | missing |  | no | No paging or Show more; UpdatesResponse has no cursor or more flag. |

## Physician session/Signed in/Updates panel/Updates open/Which day

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Showing today | partial | UpdatesSurface (physician-app lib.rs) | no | Always today, implied only by the empty copy; no day label or Earlier control. |
| Showing an earlier day | missing |  | no | No earlier-day navigation; the wire has no day field. |

## Physician session/Signed in/Today list read

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Loading Today | built | TodaySurface, ReadNotice (physician-app lib.rs); today_view (physician-app shell.rs) | yes | Loading Today… only when nothing is cached; reloads keep the cached rows. |
| Showing Today | built | TodaySurface, TodayRows (physician-app lib.rs) | yes | Rows, or the quiet copy and detail. |
| Showing Today, with one quiet line and Retry | built | TodaySurface, ReadNotice (physician-app lib.rs); today_view (physician-app shell.rs) | yes | A failed refresh keeps the last list under a quiet line with Try again; a part-loaded list isn't distinguished. |
| Couldn't load Today, with Retry | built | TodaySurface, ReadNotice (physician-app lib.rs) | yes | Coda couldn’t load Today. with Try again. |

## Physician session/Signed in/Conversation read

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Loading the conversation | built | PatientSurface Conversation section, ReadNotice (physician-app lib.rs); answer_to (physician-app shell.rs) | yes | Opening the conversation… shows until the chat being opened answers. Not exercised in the browser: the sample's workspaces list no chats. |
| Showing the conversation | built | PatientSurface Conversation section, FormattedMessage (physician-app lib.rs); answer_to (physician-app shell.rs) | yes | Turns with the composer below; another chat's lines never stand in for the one opened. |
| Showing the conversation, with one quiet line and Retry | missing |  | yes | A failed refresh drops the shown turns to Couldn't open this conversation instead of keeping them under a quiet line. |
| Couldn't load the conversation, with Retry | built | PatientSurface Conversation section, ReadNotice (physician-app lib.rs) | yes | Coda couldn’t open this conversation. with Try again. |
| No conversation to read | built | PatientSurface Conversation section, PhysicianShell sidebar (physician-app lib.rs) | yes | Nothing to read until a chat opens; with no chats the copy says This workspace has no chats. and there is no way to start one. |

## Physician session/Signed in/Schedule read

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Loading the schedule | built | TodaySurface and PatientSurface ScheduleRail, schedule_unanswered, ReadNotice (physician-app lib.rs) | yes | Both rails show Loading the schedule… until the day answers; only an answered day can say there is no calendar or no visit. |
| Showing the schedule | built | ScheduleRail (physician-app lib.rs); upcoming_visits, schedule_empty_copy (physician-app shell.rs) | yes | Rail with visits and a left count, or the no-visits copy. |
| Showing the schedule, with one quiet line and Retry | partial | ScheduleRail, use_read_view, ReadNotice (physician-app lib.rs) | yes | A failed refresh keeps the last schedule under a quiet line with Try again, but a part-loaded day (source state not usable) still reads as No visits scheduled today. |
| Couldn't load the schedule, with Retry | built | TodaySurface and PatientSurface ScheduleRail, ReadNotice (physician-app lib.rs) | yes | A failed first read shows the rail with Coda couldn’t load the schedule. and Try again. |
| No calendar, no schedule area | built | TodaySurface (physician-app lib.rs); shows_today_schedule (physician-app shell.rs) | yes | No rail when the calendar state is unavailable or absent, though unavailable conflates a down calendar with none. |

## Physician session/Signed in/Coda window

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Coda open | built | PhysicianShell glass-app shell, Collapse Coda button (physician-app lib.rs) | yes |  |
| Collapsed to the orb.Orb only | built | PhysicianShell coda-restore button (physician-app lib.rs) | yes | Orb with the logo; the shell stays mounted but hidden. |
| Collapsed to the orb.Orb preview shown | missing |  | no | No hover or focus preview on the orb, and updates carry no content to preview. |
| Collapsed to the orb.Orb preview pinned | missing |  | no | No pinned preview or opening an update from the orb. |

## Physician session/Signed in/Find patient

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Search closed | built | PhysicianShell sidebar Find patient item (physician-app lib.rs) | yes | With the ⌘ K hint. |
| Search open.Search read.Loading the search | built | SearchDialog, ReadNotice (physician-app lib.rs); answer_to (physician-app shell.rs) | yes | Searching… shows until the query being asked answers; an earlier query's rows or No patients match. never stand in. |
| Search open.Search read.Showing the search | partial | SearchDialog (physician-app lib.rs) | no | Results with name and DOB, and No patients match. when empty, but no 50-at-a-time paging; PatientSearchResponse has no cursor. |
| Search open.Search read.Showing the search, with one quiet line and Retry | missing |  | yes | A failed search drops to the couldn't-search error instead of keeping the results under a quiet line. |
| Search open.Search read.Couldn't load the search, with Retry | built | SearchDialog, ReadNotice (physician-app lib.rs) | yes | Coda couldn’t search patients. with Try again. |
| Search open.Search read.Type to search | built | SearchDialog (physician-app lib.rs) | yes | An empty query shows no rows; the placeholder is the only prompt. |

## Physician session/Signed in/Your command on the initial understanding

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No command on the initial understanding in flight | partial | TodayRows Try again, resume_slot CommandSlot, CommandNotice (physician-app lib.rs) | no | Try again shows only on Today rows carrying a resume target (none in the sample), and a refusal says why without re-reading the view. |
| Try again on the initial understanding: sending; other commands on it wait | partial | TodayRows Try again, CommandSlot::run with resume_command, CommandNotice (physician-app lib.rs) | no | The command line is built (CommandSlot: sending, then Check again if unconfirmed) and runs from a Today row's Try again, but nothing connects a failed initial understanding to it yet: no row carries its resume target, and the sample refuses work.resume. |
| Couldn't confirm Try again on the initial understanding, with Check again | partial | CommandSlot::check_again, CommandNotice Check again, resume_command stable key (physician-app lib.rs) | no | The command line is built (CommandSlot: sending, then Check again if unconfirmed) and runs from a Today row's Try again, but nothing connects a failed initial understanding to it yet: no row carries its resume target, and the sample refuses work.resume. |

## Physician session/Signed in/Your command on the patient review

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No command on the patient review in flight | partial | TodayRows Try again, resume_slot CommandSlot, CommandNotice (physician-app lib.rs) | no | Only Try again is offered, on Today rows with a resume target; Stop, Stop and discuss, Resume and Done on what's left are not offered. |
| Try again on the patient review: sending; other commands on it wait | partial | TodayRows Try again, CommandSlot::run with resume_command, CommandNotice (physician-app lib.rs) | no | The command line is built (CommandSlot: sending, then Check again if unconfirmed) and runs from a Today row's Try again, but nothing connects a failed patient review to it yet: no row carries its resume target, and the sample refuses work.resume. |
| Couldn't confirm Try again on the patient review, with Check again | partial | CommandSlot::check_again, CommandNotice Check again, resume_command stable key (physician-app lib.rs) | no | The command line is built (CommandSlot: sending, then Check again if unconfirmed) and runs from a Today row's Try again, but nothing connects a failed patient review to it yet: no row carries its resume target, and the sample refuses work.resume. |
| Stop on the patient review: sending; other commands on it wait | missing |  | no | Stop is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Couldn't confirm Stop on the patient review, with Check again | missing |  | no | Stop is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Stop and discuss on the patient review: sending; other commands on it wait | missing |  | no | Stop and discuss is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Couldn't confirm Stop and discuss on the patient review, with Check again | missing |  | no | Stop and discuss is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Resume on the patient review: sending; other commands on it wait | missing |  | no | Resume is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Couldn't confirm Resume on the patient review, with Check again | missing |  | no | Resume is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Done on what's left on the patient review: sending; other commands on it wait | missing |  | no | Done on what's left is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Couldn't confirm Done on what's left on the patient review, with Check again | missing |  | no | Done on what's left is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |

## Physician session/Signed in/Your command on the follow-up review

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No command on the follow-up review in flight | partial | TodayRows Try again, resume_slot CommandSlot, CommandNotice (physician-app lib.rs) | no | Try again shows only on Today rows carrying a resume target (none in the sample), and a refusal says why without re-reading the view. |
| Try again on the follow-up review: sending; other commands on it wait | partial | TodayRows Try again, CommandSlot::run with resume_command, CommandNotice (physician-app lib.rs) | no | The command line is built (CommandSlot: sending, then Check again if unconfirmed) and runs from a Today row's Try again, but nothing connects a failed follow-up review to it yet: no row carries its resume target, and the sample refuses work.resume. |
| Couldn't confirm Try again on the follow-up review, with Check again | partial | CommandSlot::check_again, CommandNotice Check again, resume_command stable key (physician-app lib.rs) | no | The command line is built (CommandSlot: sending, then Check again if unconfirmed) and runs from a Today row's Try again, but nothing connects a failed follow-up review to it yet: no row carries its resume target, and the sample refuses work.resume. |

## Physician session/Signed in/Your command on the weekly review

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No command on the weekly review in flight | partial | TodayRows Try again, resume_slot CommandSlot, CommandNotice (physician-app lib.rs) | no | Only Try again is offered, on Today rows with a resume target; Stop, Stop and discuss and Resume are not offered. |
| Try again on the weekly review: sending; other commands on it wait | partial | TodayRows Try again, CommandSlot::run with resume_command, CommandNotice (physician-app lib.rs) | no | The command line is built (CommandSlot: sending, then Check again if unconfirmed) and runs from a Today row's Try again, but nothing connects a failed weekly review to it yet: no row carries its resume target, and the sample refuses work.resume. |
| Couldn't confirm Try again on the weekly review, with Check again | partial | CommandSlot::check_again, CommandNotice Check again, resume_command stable key (physician-app lib.rs) | no | The command line is built (CommandSlot: sending, then Check again if unconfirmed) and runs from a Today row's Try again, but nothing connects a failed weekly review to it yet: no row carries its resume target, and the sample refuses work.resume. |
| Stop on the weekly review: sending; other commands on it wait | missing |  | no | Stop is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Couldn't confirm Stop on the weekly review, with Check again | missing |  | no | Stop is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Stop and discuss on the weekly review: sending; other commands on it wait | missing |  | no | Stop and discuss is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Couldn't confirm Stop and discuss on the weekly review, with Check again | missing |  | no | Stop and discuss is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Resume on the weekly review: sending; other commands on it wait | missing |  | no | Resume is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Couldn't confirm Resume on the weekly review, with Check again | missing |  | no | Resume is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |

## Physician session/Signed in/Your command on the visit preparation

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No command on the visit preparation in flight | partial | TodayRows Try again, resume_slot CommandSlot, CommandNotice (physician-app lib.rs) | no | Only Try again is offered, on Today rows with a resume target; Stop, Stop and discuss, Resume and Done on what's left are not offered. |
| Try again on the visit preparation: sending; other commands on it wait | partial | TodayRows Try again, CommandSlot::run with resume_command, CommandNotice (physician-app lib.rs) | no | The command line is built (CommandSlot: sending, then Check again if unconfirmed) and runs from a Today row's Try again, but nothing connects a failed visit preparation to it yet: no row carries its resume target, and the sample refuses work.resume. |
| Couldn't confirm Try again on the visit preparation, with Check again | partial | CommandSlot::check_again, CommandNotice Check again, resume_command stable key (physician-app lib.rs) | no | The command line is built (CommandSlot: sending, then Check again if unconfirmed) and runs from a Today row's Try again, but nothing connects a failed visit preparation to it yet: no row carries its resume target, and the sample refuses work.resume. |
| Stop on the visit preparation: sending; other commands on it wait | missing |  | no | Stop is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Couldn't confirm Stop on the visit preparation, with Check again | missing |  | no | Stop is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Stop and discuss on the visit preparation: sending; other commands on it wait | missing |  | no | Stop and discuss is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Couldn't confirm Stop and discuss on the visit preparation, with Check again | missing |  | no | Stop and discuss is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Resume on the visit preparation: sending; other commands on it wait | missing |  | no | Resume is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Couldn't confirm Resume on the visit preparation, with Check again | missing |  | no | Resume is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Done on what's left on the visit preparation: sending; other commands on it wait | missing |  | no | Done on what's left is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Couldn't confirm Done on what's left on the visit preparation, with Check again | missing |  | no | Done on what's left is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |

## Physician session/Signed in/Your command on the asked reply

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No command on the asked reply in flight | partial | TodayRows Try again, resume_slot CommandSlot, CommandNotice (physician-app lib.rs) | no | Only Try again is offered, on Today rows with a resume target; Stop, Stop and discuss, Resume, Done on what's left and Update are not offered. |
| Try again on the asked reply: sending; other commands on it wait | partial | TodayRows Try again, CommandSlot::run with resume_command, CommandNotice (physician-app lib.rs) | no | The command line is built (CommandSlot: sending, then Check again if unconfirmed) and runs from a Today row's Try again, but nothing connects a failed asked reply to it yet: no row carries its resume target, and the sample refuses work.resume. |
| Couldn't confirm Try again on the asked reply, with Check again | partial | CommandSlot::check_again, CommandNotice Check again, resume_command stable key (physician-app lib.rs) | no | The command line is built (CommandSlot: sending, then Check again if unconfirmed) and runs from a Today row's Try again, but nothing connects a failed asked reply to it yet: no row carries its resume target, and the sample refuses work.resume. |
| Stop on the asked reply: sending; other commands on it wait | missing |  | no | Stop is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Couldn't confirm Stop on the asked reply, with Check again | missing |  | no | Stop is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Stop and discuss on the asked reply: sending; other commands on it wait | missing |  | no | Stop and discuss is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Couldn't confirm Stop and discuss on the asked reply, with Check again | missing |  | no | Stop and discuss is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Resume on the asked reply: sending; other commands on it wait | missing |  | no | Resume is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Couldn't confirm Resume on the asked reply, with Check again | missing |  | no | Resume is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Done on what's left on the asked reply: sending; other commands on it wait | missing |  | no | Done on what's left is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Couldn't confirm Done on what's left on the asked reply, with Check again | missing |  | no | Done on what's left is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Update on the asked reply: sending; other commands on it wait | missing |  | no | Update is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Couldn't confirm Update on the asked reply, with Check again | missing |  | no | Update is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |

## Physician session/Signed in/Your command on the proactive work

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No command on the proactive work in flight | partial | TodayRows Try again, resume_slot CommandSlot, CommandNotice (physician-app lib.rs) | no | Only Try again is offered, on Today rows with a resume target; Stop, Stop and discuss and Resume are not offered. |
| Try again on the proactive work: sending; other commands on it wait | partial | TodayRows Try again, CommandSlot::run with resume_command, CommandNotice (physician-app lib.rs) | no | The command line is built (CommandSlot: sending, then Check again if unconfirmed) and runs from a Today row's Try again, but nothing connects a failed proactive work to it yet: no row carries its resume target, and the sample refuses work.resume. |
| Couldn't confirm Try again on the proactive work, with Check again | partial | CommandSlot::check_again, CommandNotice Check again, resume_command stable key (physician-app lib.rs) | no | The command line is built (CommandSlot: sending, then Check again if unconfirmed) and runs from a Today row's Try again, but nothing connects a failed proactive work to it yet: no row carries its resume target, and the sample refuses work.resume. |
| Stop on the proactive work: sending; other commands on it wait | missing |  | no | Stop is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Couldn't confirm Stop on the proactive work, with Check again | missing |  | no | Stop is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Stop and discuss on the proactive work: sending; other commands on it wait | missing |  | no | Stop and discuss is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Couldn't confirm Stop and discuss on the proactive work, with Check again | missing |  | no | Stop and discuss is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Resume on the proactive work: sending; other commands on it wait | missing |  | no | Resume is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Couldn't confirm Resume on the proactive work, with Check again | missing |  | no | Resume is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |

## Physician session/Signed in/Your command on the answer

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No command on the answer in flight | partial | TodayRows Try again, resume_slot CommandSlot, CommandNotice (physician-app lib.rs) | no | Only Try again is offered, on Today rows with a resume target; Stop, Stop and discuss, Resume and Update are not offered. |
| Try again on the answer: sending; other commands on it wait | partial | TodayRows Try again, CommandSlot::run with resume_command, CommandNotice (physician-app lib.rs) | no | The command line is built (CommandSlot: sending, then Check again if unconfirmed) and runs from a Today row's Try again, but nothing connects a failed answer to it yet: no row carries its resume target, and the sample refuses work.resume. |
| Couldn't confirm Try again on the answer, with Check again | partial | CommandSlot::check_again, CommandNotice Check again, resume_command stable key (physician-app lib.rs) | no | The command line is built (CommandSlot: sending, then Check again if unconfirmed) and runs from a Today row's Try again, but nothing connects a failed answer to it yet: no row carries its resume target, and the sample refuses work.resume. |
| Stop on the answer: sending; other commands on it wait | missing |  | no | Stop is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Couldn't confirm Stop on the answer, with Check again | missing |  | no | Stop is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Stop and discuss on the answer: sending; other commands on it wait | missing |  | no | Stop and discuss is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Couldn't confirm Stop and discuss on the answer, with Check again | missing |  | no | Stop and discuss is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Resume on the answer: sending; other commands on it wait | missing |  | no | Resume is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Couldn't confirm Resume on the answer, with Check again | missing |  | no | Resume is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Update on the answer: sending; other commands on it wait | missing |  | no | Update is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Couldn't confirm Update on the answer, with Check again | missing |  | no | Update is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |

## Physician session/Signed in/Your command on the reply revision

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No command on the reply revision in flight | partial | TodayRows Try again, resume_slot CommandSlot, CommandNotice (physician-app lib.rs) | no | Only Try again is offered, on Today rows with a resume target; Stop, Stop and discuss and Resume are not offered. |
| Try again on the reply revision: sending; other commands on it wait | partial | TodayRows Try again, CommandSlot::run with resume_command, CommandNotice (physician-app lib.rs) | no | The command line is built (CommandSlot: sending, then Check again if unconfirmed) and runs from a Today row's Try again, but nothing connects a failed reply revision to it yet: no row carries its resume target, and the sample refuses work.resume. |
| Couldn't confirm Try again on the reply revision, with Check again | partial | CommandSlot::check_again, CommandNotice Check again, resume_command stable key (physician-app lib.rs) | no | The command line is built (CommandSlot: sending, then Check again if unconfirmed) and runs from a Today row's Try again, but nothing connects a failed reply revision to it yet: no row carries its resume target, and the sample refuses work.resume. |
| Stop on the reply revision: sending; other commands on it wait | missing |  | no | Stop is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Couldn't confirm Stop on the reply revision, with Check again | missing |  | no | Stop is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Stop and discuss on the reply revision: sending; other commands on it wait | missing |  | no | Stop and discuss is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Couldn't confirm Stop and discuss on the reply revision, with Check again | missing |  | no | Stop and discuss is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Resume on the reply revision: sending; other commands on it wait | missing |  | no | Resume is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |
| Couldn't confirm Resume on the reply revision, with Check again | missing |  | no | Resume is not offered; the wire's AutomationRun has no running, held or left-for-you state to offer it on. |

## Physician session/Signed in/Your command on the follow-up

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No command on the follow-up in flight | missing |  | no | Follow-ups render as read-only text with no controls. |
| Create on the follow-up: sending; other commands on it wait | missing |  | no | Create is not offered; the wire's FollowUp carries only an id and description. |
| Couldn't confirm Create on the follow-up, with Check again | missing |  | no | Create is not offered; the wire's FollowUp carries only an id and description. |
| Change on the follow-up: sending; other commands on it wait | missing |  | no | Change is not offered; the wire's FollowUp carries only an id and description. |
| Couldn't confirm Change on the follow-up, with Check again | missing |  | no | Change is not offered; the wire's FollowUp carries only an id and description. |
| Review now on the follow-up: sending; other commands on it wait | missing |  | no | Review now is not offered; the wire's FollowUp carries only an id and description. |
| Couldn't confirm Review now on the follow-up, with Check again | missing |  | no | Review now is not offered; the wire's FollowUp carries only an id and description. |
| Stop on the follow-up: sending; other commands on it wait | missing |  | no | Stop is not offered; the wire's FollowUp carries only an id and description. |
| Couldn't confirm Stop on the follow-up, with Check again | missing |  | no | Stop is not offered; the wire's FollowUp carries only an id and description. |
| Stop and discuss on the follow-up: sending; other commands on it wait | missing |  | no | Stop and discuss is not offered; the wire's FollowUp carries only an id and description. |
| Couldn't confirm Stop and discuss on the follow-up, with Check again | missing |  | no | Stop and discuss is not offered; the wire's FollowUp carries only an id and description. |
| Resume on the follow-up: sending; other commands on it wait | missing |  | no | Resume is not offered; the wire's FollowUp carries only an id and description. |
| Couldn't confirm Resume on the follow-up, with Check again | missing |  | no | Resume is not offered; the wire's FollowUp carries only an id and description. |
| Cancel on the follow-up: sending; other commands on it wait | missing |  | no | Cancel is not offered; the wire's FollowUp carries only an id and description. |
| Couldn't confirm Cancel on the follow-up, with Check again | missing |  | no | Cancel is not offered; the wire's FollowUp carries only an id and description. |
| Complete on the follow-up: sending; other commands on it wait | missing |  | no | Complete is not offered; the wire's FollowUp carries only an id and description. |
| Couldn't confirm Complete on the follow-up, with Check again | missing |  | no | Complete is not offered; the wire's FollowUp carries only an id and description. |

## Physician session/Signed in/Dictation

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Microphone idle | built | PatientSurface composer Dictate button, Ctrl+Shift+Space handler (physician-app lib.rs) | yes | A Dictate text button, not a microphone icon. |
| Waiting for microphone permission | missing |  | no | No microphone capture; the client posts an empty body to /dictation, which the sample answers 503. |
| Recording | missing |  | no | No capture, levels or stop control. |
| Transcribing | missing |  | no | No transcription wait state. |
| Transcript waiting to insert at your cursor | missing |  | no | No transcript preview or insert. |
| Dictation failed, your text kept | partial | PatientSurface composer notice, on_dictate (physician-app lib.rs) | no | Every attempt shows the same fixed DICTATION_UNAVAILABLE notice whatever happened; the draft is kept. |

## Physician session/Signed in/Your command on the concern

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No command on the concern in flight | missing |  | no | Today rows open the patient but offer no Done; the sample only refuses home.clear and work.done. |
| Done on the concern: sending; other commands on it wait | missing |  | no | Today rows open the patient but offer no Done; the sample only refuses home.clear and work.done. |
| Couldn't confirm Done on the concern, with Check again | missing |  | no | Today rows open the patient but offer no Done; the sample only refuses home.clear and work.done. |

## Physician session/Signed in/Your command on the conversation

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No command on the conversation in flight | partial | PatientWorkspace composer Send and Cmd/Ctrl+Enter, send_slot CommandSlot (physician-app lib.rs) | no | Only Send is offered; Steer, Send next, Archive, Restore and Undo are not, and the composer shows only when chats exist. |
| Send on the conversation: sending; other commands on it wait | built | send_slot CommandSlot, message_command, CommandNotice under the composer (physician-app lib.rs) | yes | Sending… shows and repeat presses are ignored, though the Send button itself stays enabled. |
| Couldn't confirm Send on the conversation, with Check again | built | CommandSlot::check_again, CommandNotice Check again (physician-app lib.rs) | yes | Check again resends the same message.submit with its original key. |
| Steer on the conversation: sending; other commands on it wait | missing |  | no | Steer is not offered; the chat snapshot carries only turn text. |
| Couldn't confirm Steer on the conversation, with Check again | missing |  | no | Steer is not offered; the chat snapshot carries only turn text. |
| Send next on the conversation: sending; other commands on it wait | missing |  | no | Send next is not offered; the chat snapshot carries only turn text. |
| Couldn't confirm Send next on the conversation, with Check again | missing |  | no | Send next is not offered; the chat snapshot carries only turn text. |
| Archive on the conversation: sending; other commands on it wait | missing |  | no | Archive is not offered; the chat snapshot carries only turn text. |
| Couldn't confirm Archive on the conversation, with Check again | missing |  | no | Archive is not offered; the chat snapshot carries only turn text. |
| Restore on the conversation: sending; other commands on it wait | missing |  | no | Restore is not offered; the chat snapshot carries only turn text. |
| Couldn't confirm Restore on the conversation, with Check again | missing |  | no | Restore is not offered; the chat snapshot carries only turn text. |
| Undo on the conversation: sending; other commands on it wait | missing |  | no | Undo is not offered; the chat snapshot carries only turn text. |
| Couldn't confirm Undo on the conversation, with Check again | missing |  | no | Undo is not offered; the chat snapshot carries only turn text. |

## Physician session/Signed in/Archive notice

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No archive notice | not shown |  | n/a | The default absence; the app has no archive feature. |
| Conversation archived, with Undo | missing |  | no | No Archive command or Conversation archived with Undo notice. |

## Physician session/Signed in/Feedback form

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Feedback details closed | missing |  | no | No thumbs up or down on messages. |
| Feedback details open.Showing the saved details | missing |  | no | No feedback form; the wire has no feedback fields. |
| Feedback details open.Your unsaved reasons or note | missing |  | no | No feedback form. |

## Physician session/Signed in/Your command on the feedback

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No command on the feedback in flight | missing |  | no | No feedback controls exist, and the wire and sample have no feedback command. |
| Thumbs up on the feedback: sending; other commands on it wait | missing |  | no | No feedback controls exist, and the wire and sample have no feedback command. |
| Couldn't confirm Thumbs up on the feedback, with Check again | missing |  | no | No feedback controls exist, and the wire and sample have no feedback command. |
| Thumbs down on the feedback: sending; other commands on it wait | missing |  | no | No feedback controls exist, and the wire and sample have no feedback command. |
| Couldn't confirm Thumbs down on the feedback, with Check again | missing |  | no | No feedback controls exist, and the wire and sample have no feedback command. |
| Save details on the feedback: sending; other commands on it wait | missing |  | no | No feedback controls exist, and the wire and sample have no feedback command. |
| Couldn't confirm Save details on the feedback, with Check again | missing |  | no | No feedback controls exist, and the wire and sample have no feedback command. |

## Physician session/Signed in/Your command on the queued messages

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No command on the queued messages in flight | missing |  | no | Messages go with intent queue, but no queue shows; the chat snapshot carries no queued messages. |
| Edit on the queued messages: sending; other commands on it wait | missing |  | no | Messages go with intent queue, but no queue shows; the chat snapshot carries no queued messages. |
| Couldn't confirm Edit on the queued messages, with Check again | missing |  | no | Messages go with intent queue, but no queue shows; the chat snapshot carries no queued messages. |
| Remove on the queued messages: sending; other commands on it wait | missing |  | no | Messages go with intent queue, but no queue shows; the chat snapshot carries no queued messages. |
| Couldn't confirm Remove on the queued messages, with Check again | missing |  | no | Messages go with intent queue, but no queue shows; the chat snapshot carries no queued messages. |
| Send now on the queued messages: sending; other commands on it wait | missing |  | no | Messages go with intent queue, but no queue shows; the chat snapshot carries no queued messages. |
| Couldn't confirm Send now on the queued messages, with Check again | missing |  | no | Messages go with intent queue, but no queue shows; the chat snapshot carries no queued messages. |

## Physician session/Signed in/Your command on the patient's connection

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No command on the patient's connection in flight | missing |  | yes | PhysicianClient::invite_patient and patient_link exist, but the app shows no connection controls. |
| Invite on the patient's connection: sending; other commands on it wait | missing |  | yes | The sample's invite endpoint and the client call exist, but the app never offers Invite. |
| Couldn't confirm Invite on the patient's connection, with Check again | missing |  | yes | The sample's invite endpoint and the client call exist, but the app never offers Invite. |
| Resend on the patient's connection: sending; other commands on it wait | missing |  | no | Resend is not offered, and the sample has no route for it. |
| Couldn't confirm Resend on the patient's connection, with Check again | missing |  | no | Resend is not offered, and the sample has no route for it. |
| Cancel the invitation on the patient's connection: sending; other commands on it wait | missing |  | no | Cancel the invitation is not offered, and the sample has no route for it. |
| Couldn't confirm Cancel the invitation on the patient's connection, with Check again | missing |  | no | Cancel the invitation is not offered, and the sample has no route for it. |
| Discharge on the patient's connection: sending; other commands on it wait | missing |  | no | Discharge is not offered, and the sample has no route for it. |
| Couldn't confirm Discharge on the patient's connection, with Check again | missing |  | no | Discharge is not offered, and the sample has no route for it. |

## Physician session/Signed in/Your command on the message match

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No command on the message match in flight | missing |  | no | No message match is shown, and the wire carries no match or correction state. |
| Confirm on the message match: sending; other commands on it wait | missing |  | no | No message match is shown, and the wire carries no match or correction state. |
| Couldn't confirm Confirm on the message match, with Check again | missing |  | no | No message match is shown, and the wire carries no match or correction state. |
| Another patient on the message match: sending; other commands on it wait | missing |  | no | No message match is shown, and the wire carries no match or correction state. |
| Couldn't confirm Another patient on the message match, with Check again | missing |  | no | No message match is shown, and the wire carries no match or correction state. |
| Not a patient on the message match: sending; other commands on it wait | missing |  | no | No message match is shown, and the wire carries no match or correction state. |
| Couldn't confirm Not a patient on the message match, with Check again | missing |  | no | No message match is shown, and the wire carries no match or correction state. |
| Link on the message match: sending; other commands on it wait | missing |  | no | No message match is shown, and the wire carries no match or correction state. |
| Couldn't confirm Link on the message match, with Check again | missing |  | no | No message match is shown, and the wire carries no match or correction state. |
| Move to another patient on the message match: sending; other commands on it wait | missing |  | no | No message match is shown, and the wire carries no match or correction state. |
| Couldn't confirm Move to another patient on the message match, with Check again | missing |  | no | No message match is shown, and the wire carries no match or correction state. |
| Mark not a patient on the message match: sending; other commands on it wait | missing |  | no | No message match is shown, and the wire carries no match or correction state. |
| Couldn't confirm Mark not a patient on the message match, with Check again | missing |  | no | No message match is shown, and the wire carries no match or correction state. |
| Undo the correction on the message match: sending; other commands on it wait | missing |  | no | No message match is shown, and the wire carries no match or correction state. |
| Couldn't confirm Undo the correction on the message match, with Check again | missing |  | no | No message match is shown, and the wire carries no match or correction state. |

## Physician session/Signed in/Your command on the unread message

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No command on the unread message in flight | missing |  | no | No unread messages are shown, and the wire carries none. |
| Done on the unread message: sending; other commands on it wait | missing |  | no | No unread messages are shown, and the wire carries none. |
| Couldn't confirm Done on the unread message, with Check again | missing |  | no | No unread messages are shown, and the wire carries none. |

## Physician session/Signed in/Your command on the reply

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No command on the reply in flight | partial | ReplyReviewCard, reply_slot CommandSlot (physician-app lib.rs) | no | No reply needed and Ask Coda to revise are offered (plus Copy); Ask Coda to reply, Undo, Use Coda's update, Restore my version and Send are not. |
| Ask Coda to reply on the reply: sending; other commands on it wait | missing |  | no | Ask Coda to reply is not offered; the wire's WorkspaceReply has no state for it. |
| Couldn't confirm Ask Coda to reply on the reply, with Check again | missing |  | no | Ask Coda to reply is not offered; the wire's WorkspaceReply has no state for it. |
| No reply needed on the reply: sending; other commands on it wait | built | ReplyReviewCard busy, reply_slot CommandSlot, CommandNotice in Patient replies (physician-app lib.rs) | yes | Sending… shows and the card's controls disable; the busy flag covers every reply card, not just this one. |
| Couldn't confirm No reply needed on the reply, with Check again | built | CommandSlot::check_again, CommandNotice Check again (physician-app lib.rs) | yes | Check again resends the same request with its original key. |
| Undo no reply needed on the reply: sending; other commands on it wait | missing |  | no | Undo no reply needed is not offered; the wire's WorkspaceReply has no state for it. |
| Couldn't confirm Undo no reply needed on the reply, with Check again | missing |  | no | Undo no reply needed is not offered; the wire's WorkspaceReply has no state for it. |
| Use Coda's update on the reply: sending; other commands on it wait | missing |  | no | Use Coda's update is not offered; the wire's WorkspaceReply has no state for it. |
| Couldn't confirm Use Coda's update on the reply, with Check again | missing |  | no | Use Coda's update is not offered; the wire's WorkspaceReply has no state for it. |
| Restore my version on the reply: sending; other commands on it wait | missing |  | no | Restore my version is not offered; the wire's WorkspaceReply has no state for it. |
| Couldn't confirm Restore my version on the reply, with Check again | missing |  | no | Restore my version is not offered; the wire's WorkspaceReply has no state for it. |
| Send on the reply: sending; other commands on it wait | missing |  | no | Send is not offered; the wire's WorkspaceReply has no state for it. |
| Couldn't confirm Send on the reply, with Check again | missing |  | no | Send is not offered; the wire's WorkspaceReply has no state for it. |
| Ask Coda to revise on the reply: sending; other commands on it wait | built | ReplyReviewCard busy, reply_slot CommandSlot, CommandNotice in Patient replies (physician-app lib.rs) | yes | Sending… shows and the card's controls disable; the busy flag covers every reply card, not just this one. |
| Couldn't confirm Ask Coda to revise on the reply, with Check again | built | CommandSlot::check_again, CommandNotice Check again (physician-app lib.rs) | yes | Check again resends the same request with its original key. |

## Access to the patient

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Cannot see the patient | partial | PatientSurface gone branch (PATIENT_GONE, Go to Today), PhysicianShell workspace read (physician-app lib.rs) | yes | A 404 now shows "That patient is not available." without the name, but an open reader dialog and the sidebar patient name (from the patient list) don't close. |
| Identity only | built | PatientSurface clinic-demographic card, sidebar Clinic record (physician-app lib.rs); demographic_card_view (shell.rs) | yes | The copy frames identity-only as "Not on your panel", which is how the wire models it. |
| Clinical access | built | PatientSurface (physician-app lib.rs) | yes | The full patient page; a drop to identity only shows on the next workspace read. |

## Records source

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Connecting the source | missing |  | yes | records_as_of ignores any state but usable/refreshing/failed, so nothing says Coda is connecting the source. |
| Records kept.Records current | built | summary_times, records_as_of (shell.rs); PatientSurface Patient summary meta (physician-app lib.rs) | yes | "Records as of …" shows only inside the Patient summary section, so a patient without a summary shows no records time. |
| Records kept.Checking for new records | built | records_as_of (shell.rs) | yes | A refreshing source keeps "Records as of …"; the graph wants no distinct indicator. |
| Records kept.Records couldn't be updated, retrying | partial | records_as_of (shell.rs) | yes | A failed source keeps the last good "Records as of …" but never says "Records couldn't be updated". |
| Source unavailable, last good records kept | partial | records_as_of (shell.rs) | yes | Only a "failed" source with a usable_snapshot shows a bare time; an unavailable one shows nothing, and "couldn't update" never appears. |
| Source unavailable, no records yet | missing |  | yes | Nothing renders for a down source with no records. |

## Practice records source

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Practice source up | not shown |  | n/a | Nothing shows while the practice source is up, and the app shows nothing. |
| Practice source down, one line on Today | partial | connection_lines, connection_sentence (shell.rs); TodaySurface today-connection (physician-app lib.rs) | no | A channel-less connection line could read "Coda can’t reach a connection right now.", but TodayConnection has no records-source identity, so nothing names it. |

## Practice connection

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Not connected | missing |  | yes | patient_link is on the wire and the client has invite_patient, but the app never reads it or offers Invite. |
| Invited.Sending the invitation | missing |  | no | No invitation status in the app or on the wire. |
| Invited.Invitation sent.No reminder yet | missing |  | no | No delivered or reminder state on the wire or in the app. |
| Invited.Invitation sent.Reminder sent | missing |  | no | Not on the wire and not rendered. |
| Invited.Invitation couldn't be delivered, with Resend | missing |  | no | No undelivered state or Resend on the wire or in the app. |
| Connected | partial | PatientSurface Patient-shared records (physician-app lib.rs) | yes | Shared Duet records list, but nothing says the patient is connected and there is no Discharge. |
| Connection ended.Ended, the patient left | missing |  | no | PatientLinkWire.revoked_at is unused and nothing on the wire says why a connection ended. |
| Connection ended.Ended, discharged by the practice | missing |  | no | No discharge command and no ended reason on the wire. |
| Connection ended.Ended, the patient's account was deleted | missing |  | no | Not on the wire and not rendered. |

## Initial understanding

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Initial understanding not finished | partial | PatientSurface (physician-app lib.rs) | yes | Only absence: the Patient summary section is left out, and no words say Coda has not finished its first understanding. |
| Initial understanding finished | built | PatientSurface Patient summary, synopsis_pieces, summary_times (physician-app lib.rs, shell.rs) | yes |  |

## Coda's work: initial understanding/initial understanding: lifecycle

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| initial understanding not in hand.No initial understanding owed | built | PatientSurface (physician-app lib.rs) | yes | Current work is left out when the workspace has no automation_run. |
| initial understanding in hand.initial understanding active.initial understanding owed, waiting to start | partial | PatientSurface Current work section, work_heading (physician-app lib.rs, shell.rs) | yes | The run headline shows the same as running; the app tells only "failed" apart, so owed is not distinguishable. |
| initial understanding in hand.initial understanding active.Coda is working on the initial understanding.Working, no step shown | partial | PatientSurface Current work section, work_heading (physician-app lib.rs, shell.rs) | yes | Shows the run headline with no "Coda is working" wording and no difference from owed. |
| initial understanding in hand.initial understanding active.Coda is working on the initial understanding.Showing the current step | missing |  | no | No step line; AutomationRun on the wire has no current-step field. |
| initial understanding in hand.initial understanding stalled.initial understanding waiting for the next opportunity, with Try again | missing |  | no | No waiting copy, reason or Try again; a waiting run reads as the plain headline. |
| initial understanding in hand.initial understanding stalled.initial understanding couldn't finish, with Try again | partial | PatientSurface Current work, work_heading, work_end_reason (physician-app lib.rs, shell.rs); TodayRows Try again via resume_command (physician-app lib.rs) | no | "Couldn’t finish" shows under Current work with no Try again of its own; Try again comes only from a failed_review Today row, and the sample server rejects work.resume. |

## Coda's work: initial understanding/initial understanding: news while working

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Nothing new since it started | not shown |  | n/a | Bookkeeping; the graph shows nothing for it while the work runs. |
| Something new arrived since it started | not shown |  | n/a | Bookkeeping; the graph shows nothing for it while the work runs. |

## Coda's work: patient review/patient review: lifecycle

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| patient review not in hand.No patient review owed | built | PatientSurface (physician-app lib.rs) | yes | Current work is left out when the workspace has no automation_run. |
| patient review in hand.patient review active.patient review owed, waiting to start | partial | PatientSurface Current work section, work_heading (physician-app lib.rs, shell.rs) | yes | The run headline shows the same as running; the app tells only "failed" apart, so owed is not distinguishable. |
| patient review in hand.patient review active.Coda is working on the patient review.Working, no step shown | partial | PatientSurface Current work section, work_heading (physician-app lib.rs, shell.rs) | yes | Shows the run headline with no "Coda is working" wording and no difference from owed. |
| patient review in hand.patient review active.Coda is working on the patient review.Showing the current step | missing |  | no | No step line; AutomationRun on the wire has no current-step field. |
| patient review in hand.patient review stalled.patient review waiting for the next opportunity, with Try again | missing |  | no | No waiting copy, reason or Try again; a waiting run reads as the plain headline. |
| patient review in hand.patient review stalled.patient review couldn't finish, with Try again | partial | PatientSurface Current work, work_heading, work_end_reason (physician-app lib.rs, shell.rs); TodayRows Try again via resume_command (physician-app lib.rs) | no | "Couldn’t finish" shows under Current work with no Try again of its own; Try again comes only from a failed_review Today row, and the sample server rejects work.resume. |

## Coda's work: patient review/patient review: news while working

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Nothing new since it started | not shown |  | n/a | Bookkeeping; the graph shows nothing for it while the work runs. |
| Something new arrived since it started | not shown |  | n/a | Bookkeeping; the graph shows nothing for it while the work runs. |

## Coda's work: patient review/patient review: held by you

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Not held | partial |  | no | Nothing held shows (right), but there is no Stop or Stop and discuss control and no stop command. |
| Held with Resume | missing |  | no | No held state on the wire and no Resume control. |

## Coda's work: patient review/patient review: left for you

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Nothing left for you | built | TodaySurface, TodayRows (physician-app lib.rs) | yes | No row when nothing is left. |
| Coda couldn't restart; what's left is on Today with Done.New to you | partial | TodayRows, row_did (physician-app lib.rs, shell.rs) | no | A failed_review row shows "… couldn’t finish" with Try again instead of Done and nothing about what is left. |
| Coda couldn't restart; what's left is on Today with Done.Opened by you | missing |  | no | TodayRow carries no opened state; the live dot shows on every row. |

## Coda's work: follow-up review/follow-up review: lifecycle

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| follow-up review not in hand.No follow-up review owed | built | PatientSurface (physician-app lib.rs) | yes | Current work is left out when the workspace has no automation_run. |
| follow-up review in hand.follow-up review active.follow-up review owed, waiting to start | partial | PatientSurface Current work section, work_heading (physician-app lib.rs, shell.rs) | yes | The run headline shows the same as running; the app tells only "failed" apart, so owed is not distinguishable. |
| follow-up review in hand.follow-up review active.Coda is working on the follow-up review.Working, no step shown | partial | PatientSurface Current work section, work_heading (physician-app lib.rs, shell.rs) | yes | Shows the run headline with no "Coda is working" wording and no difference from owed. |
| follow-up review in hand.follow-up review active.Coda is working on the follow-up review.Showing the current step | missing |  | no | No step line; AutomationRun on the wire has no current-step field. |
| follow-up review in hand.follow-up review stalled.follow-up review waiting for the next opportunity, with Try again | missing |  | no | No waiting copy, reason or Try again; a waiting run reads as the plain headline. |
| follow-up review in hand.follow-up review stalled.follow-up review couldn't finish, with Try again | partial | PatientSurface Current work, work_heading, work_end_reason (physician-app lib.rs, shell.rs); TodayRows Try again via resume_command (physician-app lib.rs) | no | "Couldn’t finish" shows under Current work with no Try again of its own (subject reads the generic "Work"); Try again comes only from a failed_review Today row, and the sample server rejects work.resume. |

## Coda's work: follow-up review/follow-up review: left for you

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Nothing left for you | built | TodaySurface, TodayRows (physician-app lib.rs) | yes | No row when nothing is left. |
| Coda couldn't restart; what's left is on Today with Done.New to you | partial | TodayRows, row_did (physician-app lib.rs, shell.rs) | no | A failed_review row shows "… couldn’t finish" (subject reads the generic "Work") with Try again instead of Done and nothing about what is left. |
| Coda couldn't restart; what's left is on Today with Done.Opened by you | missing |  | no | TodayRow carries no opened state; the live dot shows on every row. |

## Coda's work: weekly review/weekly review: lifecycle

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| weekly review not in hand.No weekly review owed | built | PatientSurface (physician-app lib.rs) | yes | Current work is left out when the workspace has no automation_run. |
| weekly review in hand.weekly review active.weekly review owed, waiting to start | partial | PatientSurface Current work section, work_heading (physician-app lib.rs, shell.rs) | yes | The run headline shows the same as running; the app tells only "failed" apart, so owed is not distinguishable. |
| weekly review in hand.weekly review active.Coda is working on the weekly review.Working, no step shown | partial | PatientSurface Current work section, work_heading (physician-app lib.rs, shell.rs) | yes | Shows the run headline with no "Coda is working" wording and no difference from owed. |
| weekly review in hand.weekly review active.Coda is working on the weekly review.Showing the current step | missing |  | no | No step line; AutomationRun on the wire has no current-step field. |
| weekly review in hand.weekly review stalled.weekly review waiting for the next opportunity, with Try again | missing |  | no | No waiting copy, reason or Try again; a waiting run reads as the plain headline. |
| weekly review in hand.weekly review stalled.weekly review couldn't finish, with Try again | partial | PatientSurface Current work, work_heading, work_end_reason (physician-app lib.rs, shell.rs); TodayRows Try again via resume_command (physician-app lib.rs) | no | "Couldn’t finish" shows under Current work with no Try again of its own (subject reads the generic "Work"); Try again comes only from a failed_review Today row, and the sample server rejects work.resume. |

## Coda's work: weekly review/weekly review: held by you

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Not held | partial |  | no | Nothing held shows (right), but there is no Stop or Stop and discuss control and no stop command. |
| Held with Resume | missing |  | no | No held state on the wire and no Resume control. |

## Coda's work: visit preparation/visit preparation: lifecycle

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| visit preparation not in hand.No visit preparation owed | built | PatientSurface (physician-app lib.rs) | yes | Current work is left out when the workspace has no automation_run. |
| visit preparation in hand.visit preparation active.visit preparation owed, waiting to start | partial | PatientSurface Upcoming visits rail and reader, preparation_sentence; Current work, work_heading (physician-app lib.rs, shell.rs) | yes | Only the reader sentence "This visit has not been prepared." says so; the rail shows no cue and Current work looks the same as running. |
| visit preparation in hand.visit preparation active.Coda is working on the visit preparation.Working, no step shown | built | PatientSurface Upcoming visits rail (ScheduleRail cue), preparation_cue; Current work, work_heading (physician-app lib.rs, shell.rs) | yes | "Preparing visit" shows on the patient-page rail and the run headline under Current work; the Today rail hides cues. |
| visit preparation in hand.visit preparation active.Coda is working on the visit preparation.Showing the current step | missing |  | no | No step line; AutomationRun on the wire has no current-step field. |
| visit preparation in hand.visit preparation stalled.visit preparation waiting for the next opportunity, with Try again | partial | PatientSurface Upcoming visits rail, preparation_cue, preparation_sentence (physician-app lib.rs, shell.rs) | no | Closest is "Preparation interrupted"; no waiting reason and no Try again on the visit or the patient page. |
| visit preparation in hand.visit preparation stalled.visit preparation couldn't finish, with Try again | partial | PatientSurface Upcoming visits rail cue (preparation_cue), PatientSurface Current work, work_heading, work_end_reason (physician-app lib.rs, shell.rs); TodayRows Try again via resume_command (physician-app lib.rs) | no | "Preparation failed" and "Visit preparation couldn’t finish" show; Try again comes only from a failed_review Today row, never Current work or the visit. |

## Coda's work: visit preparation/visit preparation: news while working

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Nothing new since it started | not shown |  | n/a | Bookkeeping; the graph shows nothing for it while the work runs. |
| Something new arrived since it started | not shown |  | n/a | Bookkeeping; the graph shows nothing for it while the work runs. |

## Coda's work: visit preparation/visit preparation: held by you

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Not held | partial |  | no | Nothing held shows (right), but there is no Stop or Stop and discuss control and no stop command. |
| Held with Resume | partial | PatientSurface Upcoming visits rail, preparation_cue, preparation_sentence (physician-app lib.rs, shell.rs) | no | "Preparation stopped" reads on the visit, but there is no Resume and the wire has no held flag. |

## Coda's work: visit preparation/visit preparation: left for you

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Nothing left for you | built | TodaySurface, TodayRows (physician-app lib.rs) | yes | No row when nothing is left. |
| Coda couldn't restart; what's left is on Today with Done.New to you | partial | TodayRows, row_did (physician-app lib.rs, shell.rs) | no | A failed_review row shows "… couldn’t finish" with Try again instead of Done and nothing about what is left. |
| Coda couldn't restart; what's left is on Today with Done.Opened by you | missing |  | no | TodayRow carries no opened state; the live dot shows on every row. |

## Coda's work: asked reply/asked reply: lifecycle

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| asked reply not in hand.No asked reply owed | built | PatientSurface (physician-app lib.rs) | yes | Current work is left out when the workspace has no automation_run. |
| asked reply not in hand.asked reply finished on older records, with Update | missing |  | no | No Update control and no wire field saying the finished work read older records. |
| asked reply in hand.asked reply active.asked reply owed, waiting to start | partial | PatientSurface Current work section, work_heading (physician-app lib.rs, shell.rs) | yes | The run headline shows the same as running; the app tells only "failed" apart, so owed is not distinguishable. |
| asked reply in hand.asked reply active.Coda is working on the asked reply.Working, no step shown | partial | PatientSurface Current work section, work_heading (physician-app lib.rs, shell.rs) | yes | Shows the run headline with no "Coda is working" wording and no difference from owed. |
| asked reply in hand.asked reply active.Coda is working on the asked reply.Showing the current step | missing |  | no | No step line; AutomationRun on the wire has no current-step field. |
| asked reply in hand.asked reply stalled.asked reply waiting for the next opportunity, with Try again | missing |  | no | No waiting copy, reason or Try again; a waiting run reads as the plain headline. |
| asked reply in hand.asked reply stalled.asked reply couldn't finish, with Try again | partial | PatientSurface Current work, work_heading, work_end_reason (physician-app lib.rs, shell.rs); TodayRows Try again via resume_command (physician-app lib.rs) | no | "Couldn’t finish" shows under Current work with no Try again of its own (subject reads the generic "Work"); Try again comes only from a failed_review Today row, and the sample server rejects work.resume. |

## Coda's work: asked reply/asked reply: news while working

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Nothing new since it started | not shown |  | n/a | Bookkeeping; the graph shows nothing for it while the work runs. |
| Something new arrived since it started | not shown |  | n/a | Bookkeeping; the graph shows nothing for it while the work runs. |

## Coda's work: asked reply/asked reply: held by you

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Not held | partial |  | no | Nothing held shows (right), but there is no Stop or Stop and discuss control and no stop command. |
| Held with Resume | missing |  | no | No held state on the wire and no Resume control. |

## Coda's work: asked reply/asked reply: left for you

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Nothing left for you | built | TodaySurface, TodayRows (physician-app lib.rs) | yes | No row when nothing is left. |
| Coda couldn't restart; what's left is on Today with Done.New to you | partial | TodayRows, row_did (physician-app lib.rs, shell.rs) | no | A failed_review row shows "… couldn’t finish" (subject reads the generic "Work") with Try again instead of Done and nothing about what is left. |
| Coda couldn't restart; what's left is on Today with Done.Opened by you | missing |  | no | TodayRow carries no opened state; the live dot shows on every row. |

## Coda's work: proactive work/proactive work: lifecycle

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| proactive work not in hand.No proactive work owed | built | PatientSurface (physician-app lib.rs) | yes | Current work is left out when the workspace has no automation_run. |
| proactive work in hand.proactive work active.proactive work owed, waiting to start | partial | PatientSurface Current work section, work_heading (physician-app lib.rs, shell.rs) | yes | The run headline shows the same as running; the app tells only "failed" apart, so owed is not distinguishable. |
| proactive work in hand.proactive work active.Coda is working on the proactive work.Working, no step shown | partial | PatientSurface Current work section, work_heading (physician-app lib.rs, shell.rs) | yes | Shows the run headline with no "Coda is working" wording and no difference from owed. |
| proactive work in hand.proactive work active.Coda is working on the proactive work.Showing the current step | missing |  | no | No step line; AutomationRun on the wire has no current-step field. |
| proactive work in hand.proactive work stalled.proactive work waiting for the next opportunity, with Try again | missing |  | no | No waiting copy, reason or Try again; a waiting run reads as the plain headline. |
| proactive work in hand.proactive work stalled.proactive work couldn't finish, with Try again | partial | PatientSurface Current work, work_heading, work_end_reason (physician-app lib.rs, shell.rs); TodayRows Try again via resume_command (physician-app lib.rs) | no | "Couldn’t finish" shows under Current work with no Try again of its own (subject reads the generic "Work"); Try again comes only from a failed_review Today row, and the sample server rejects work.resume. |

## Coda's work: proactive work/proactive work: held by you

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Not held | partial |  | no | Nothing held shows (right), but there is no Stop or Stop and discuss control and no stop command. |
| Held with Resume | missing |  | no | No held state on the wire and no Resume control. |

## Coda's work: answer/answer: lifecycle

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| answer not in hand.No answer owed | built | PatientSurface (physician-app lib.rs) | yes | Current work is left out when the workspace has no automation_run. |
| answer not in hand.answer finished on older records, with Update | missing |  | no | No Update control and no wire field saying the finished work read older records. |
| answer in hand.answer active.answer owed, waiting to start | partial | PatientSurface Current work section, work_heading (physician-app lib.rs, shell.rs) | yes | The run headline shows the same as running; the app tells only "failed" apart, so owed is not distinguishable. |
| answer in hand.answer active.Coda is working on the answer.Working, no step shown | partial | PatientSurface Current work section, work_heading (physician-app lib.rs, shell.rs) | yes | Shows the run headline with no "Coda is working" wording and no difference from owed. |
| answer in hand.answer active.Coda is working on the answer.Showing the current step | missing |  | no | No step line; AutomationRun on the wire has no current-step field. |
| answer in hand.answer stalled.answer waiting for the next opportunity, with Try again | missing |  | no | No waiting copy, reason or Try again; a waiting run reads as the plain headline. |
| answer in hand.answer stalled.answer couldn't finish, with Try again | partial | PatientSurface Current work, work_heading, work_end_reason (physician-app lib.rs, shell.rs); TodayRows Try again via resume_command (physician-app lib.rs) | no | "Couldn’t finish" shows under Current work with no Try again of its own; Try again comes only from a failed_review Today row, and the sample server rejects work.resume. |

## Coda's work: answer/answer: news while working

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Nothing new since it started | not shown |  | n/a | Bookkeeping; the graph shows nothing for it while the work runs. |
| Something new arrived since it started | not shown |  | n/a | Bookkeeping; the graph shows nothing for it while the work runs. |

## Coda's work: answer/answer: held by you

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Not held | partial |  | no | Nothing held shows (right), but there is no Stop or Stop and discuss control and no stop command. |
| Held with Resume | missing |  | no | No held state on the wire and no Resume control. |

## Coda's work: reply revision/reply revision: lifecycle

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| reply revision not in hand.No reply revision owed | built | PatientSurface (physician-app lib.rs) | yes | Current work is left out when the workspace has no automation_run. |
| reply revision in hand.reply revision active.reply revision owed, waiting to start | partial | PatientSurface Current work section, work_heading (physician-app lib.rs, shell.rs) | yes | The run headline shows the same as running; the app tells only "failed" apart, so owed is not distinguishable. |
| reply revision in hand.reply revision active.Coda is working on the reply revision.Working, no step shown | partial | PatientSurface Current work section, work_heading (physician-app lib.rs, shell.rs) | yes | Shows the run headline with no "Coda is working" wording and no difference from owed. |
| reply revision in hand.reply revision active.Coda is working on the reply revision.Showing the current step | missing |  | no | No step line; AutomationRun on the wire has no current-step field. |
| reply revision in hand.reply revision stalled.reply revision waiting for the next opportunity, with Try again | missing |  | no | No waiting copy, reason or Try again; a waiting run reads as the plain headline. |
| reply revision in hand.reply revision stalled.reply revision couldn't finish, with Try again | partial | PatientSurface Current work, work_heading, work_end_reason (physician-app lib.rs, shell.rs); TodayRows Try again via resume_command (physician-app lib.rs) | no | "Couldn’t finish" shows under Current work with no Try again of its own (subject reads the generic "Work"); Try again comes only from a failed_review Today row, and the sample server rejects work.resume. |

## Coda's work: reply revision/reply revision: held by you

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Not held | partial |  | no | Nothing held shows (right), but there is no Stop or Stop and discuss control and no stop command. |
| Held with Resume | missing |  | no | No held state on the wire and no Resume control. |

## Concern

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No concern needs you.No concern | built | TodaySurface (physician-app lib.rs) | yes | "Nothing needs you right now." on Today; per patient, no row. |
| No concern needs you.Marked done | missing |  | no | No Done on concern rows or in the reader; the app sends no done command. |
| No concern needs you.Closed by Coda | not shown |  | n/a | The row just leaves on the next read. |
| Needs attention | built | TodayRows; reader via prose_reading on open (physician-app lib.rs); PatientSurface rows | yes | The reader shows only the reason; Done and Discuss are covered by their own rows. |

## Concern/Needs attention/Who has opened it

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Not yet opened by you | partial | TodayRows live-dot (physician-app lib.rs) | no | The unread dot is drawn on every row; the wire carries no seen state. |
| Seen by another physician, still new to you | missing |  | no | No data about other physicians opening a concern. |
| Opened by you | missing |  | no | Opening a row records nothing and the dot never clears. |

## Concern/Needs attention/Urgency

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| For today | built | TodayRows, today_line (physician-app lib.rs, shell.rs) | yes | Rows without urgency "now" show with no now mark. |
| Needs you now | built | TodayRows urgency-now mark, today_line (physician-app lib.rs, shell.rs) | yes | "Needs you now" shows on rows whose urgency is "now". |
| For today, lowered from now before anyone opened it | missing |  | no | No opened state on the wire, so a lowered concern cannot keep its now mark until opened. |

## Concern/Needs attention/Visit link

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Not tied to a visit | built | TodayRows (physician-app lib.rs) | yes | The only form: rows carry no visit link. |
| Tied to the upcoming visit | missing |  | no | TodayRow has no visit link and visits show no nudge. |
| Its visit passed while it was marked now and unopened; ends when anyone opens it | missing |  | no | No visit link or opened state on the wire. |
| About a patient message; stays with its message | missing |  | no | TodayRow has no message link; reply cards do not carry concerns. |

## Follow-up

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No follow-up | partial | PatientSurface (physician-app lib.rs) | no | Absence is right, but there is no Create follow-up control. |
| Active follow-up.Not yet due | partial | PatientSurface Follow-ups section (physician-app lib.rs) | no | Description text only: no due date on the wire, no Change, Review now, Stop, Cancel or Complete. |
| Active follow-up.Due, its review owed | partial | PatientSurface Follow-ups section (physician-app lib.rs) | no | Same plain list; nothing marks a follow-up due. |
| Stopped, held until Resume | missing |  | no | The wire carries only active follow-ups; no held state or Resume. |
| Completed | missing |  | no | No completed follow-ups on the wire or in the app. |
| Cancelled.Cancelled, nothing new since | missing |  | no | No cancelled follow-ups on the wire or in the app. |
| Cancelled.Cancelled, new information since | not shown |  | n/a | Bookkeeping for whether Coda may recreate it; its parent Cancelled is missing. |

## Visit

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No visit scheduled | built | ScheduleRail, schedule_empty_copy (physician-app lib.rs, shell.rs); PatientSurface leaves out the rail | yes | Covers today only, so a patient with a visit next week also reads as none. |
| Visit ahead.Scheduled, not yet in preparation range | partial | ScheduleRail, preparation_sentence (physician-app lib.rs, shell.rs) | no | Only today's visits are on the wire, and "The preparation window has not opened yet." shows only in the patient-rail reader. |
| Visit ahead.Upcoming, in preparation range | built | ScheduleRail, upcoming_visits, patient_upcoming_visits, preparation_cue (physician-app lib.rs, shell.rs) | yes |  |
| Visit over | partial | upcoming_visits, schedule_empty_copy (physician-app shell.rs) | yes | An ended visit just drops off; nothing on the patient page shows it as over. |
| Visit cancelled.Cancelled, its time not yet passed | missing |  | yes | upcoming_visits keeps only status "scheduled", so cancelled visits never show. |
| Visit cancelled.Cancelled, its time has passed | missing |  | yes | Filtered out like any cancelled visit. |

## Visit patient mapping

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Visit linked to the patient | built | ScheduleRail open, visit_line (physician-app lib.rs, shell.rs) | yes |  |
| Preparation unavailable, no patient mapping | partial | ScheduleRail, preparation_cue (physician-app lib.rs, shell.rs) | yes | The "Preparation unavailable" copy never renders: unmapped visits appear only on the Today rail, which hides cues and does nothing on open. |

## Briefing

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No briefing | built | PatientSurface Upcoming visits reader, preparation_sentence (physician-app lib.rs, shell.rs) | yes | "This visit has not been prepared." in the reader; no cue on the rail. |
| Briefing ready for the upcoming visit | partial | ScheduleRail cue, preparation_cue (physician-app lib.rs, shell.rs) | no | "Briefing ready" shows on the patient rail, but opening it shows an empty reader; the wire has no briefing content. |
| Earlier briefing, kept as history | missing |  | no | Past visits are filtered out and PastWorkDialog always shows the empty copy. |

## Your draft

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Composer empty | partial | PatientWorkspace composer textarea (physician-app lib.rs) | yes | An empty composer shows, but an accepted Send doesn't clear the draft and a saved draft is never read back. |
| Draft written.Saving quietly | missing |  | yes | There is no autosave; drafts save only through the Save button, with no saving mark. |
| Draft written.Saved | partial | on_save, PhysicianClient::save_draft (physician-app lib.rs) | no | A successful manual Save only clears the notice, and the sample's draft save always answers 409. |
| Draft written.Not saved, with Retry | partial | on_save, NOT_SAVED composer-notice (physician-app lib.rs) | yes | "Not saved" shows beside the composer, but there is no Retry, only the Save button. |

## Queued messages

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Nothing queued | not shown |  | n/a | No queue area exists, which matches an empty queue. |
| Messages queued for the next answer, each editable | missing |  | no | No queue shows; the chat snapshot carries no queued messages. |

## Your feedback

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Not rated | missing |  | no | No feedback controls exist, and the wire and sample have no feedback data. |
| Rated helpful | missing |  | no | No feedback controls exist, and the wire and sample have no feedback data. |
| Rated not helpful | missing |  | no | No feedback controls exist, and the wire and sample have no feedback data. |

## Your feedback details

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No reasons or note | missing |  | no | No feedback controls exist, and the wire and sample have no feedback data. |
| Reasons or note saved | missing |  | no | No feedback controls exist, and the wire and sample have no feedback data. |

## Conversation list

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| In the active list | built | PhysicianShell sidebar conversations, PatientSurface Conversation section (physician-app lib.rs) | yes | Recent conversations on Today (RecentRow) are not clickable. |
| Archived | missing |  | no | No archived list, Archive or Restore; WorkspaceChat has no archived state. |

## This patient's update today

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Nothing written to you today | built | UpdatesSurface (physician-app lib.rs) | yes | "Nothing from Coda yet today." is practice-wide; per patient, no row. |
| Written to you, new | partial | UpdatesSurface (physician-app lib.rs) | no | Rows show only the patient name: untyped rows, no content, no new mark, not openable. |
| Opened by you | missing |  | no | Update rows cannot be opened and carry no opened state. |

## What relied on a moved message

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Nothing withheld | not shown |  | n/a | The normal state. |
| What relied on the moved message is withheld until Coda re-reviews | partial | work_end_reason (shell.rs); PatientSurface Current work (physician-app lib.rs) | no | Only the conversation_moved ended-work line relates; nothing withholds or marks what relied on the moved message. |

## Messaging channel

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Channel not connected.No channel connected | missing |  | no | Today rightly names nothing, but there is no Connect Gmail/Spruce control, and Connections can't tell this from a live channel. |
| Channel not connected.Disconnected; Coda isn't reading it, named on Today | built | connection_sentence (shell.rs); TodaySurface, ConnectionsDialog (physician-app lib.rs) | yes | "Coda isn’t reading Gmail: it’s disconnected." shows on Today and in Connections, and the all-clear waits; no Connect again control. |
| Channel connected to the practice | partial | ReplyReviewCard, ConnectionsDialog (physician-app lib.rs) | no | Seen only through problem lines and "Replying in Gmail/Spruce"; nothing says which channel is connected or who manages it. |

## Messaging channel/Channel connected to the practice/Connection

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Connecting the channel | partial | connection_sentence not_yet_read (shell.rs) | yes | "Coda hasn’t read Gmail yet." is the nearest line, but the graph never shows catching up and there is no connect flow. |
| Channel live | partial | TodaySurface, ConnectionsDialog (physician-app lib.rs); connection_sentence (shell.rs) | yes | No Today line, correctly, but temporarily_unavailable still shows "Coda can’t reach … right now." and there is no Disconnect. |
| Channel needs reconnecting | built | connection_sentence (shell.rs); TodaySurface, ConnectionsDialog (physician-app lib.rs) | yes | Reconnect and admin-block lines show; there is no Reconnect button for the manager. |
| Disconnecting | missing |  | no | No disconnect command or disconnecting state on the wire or in the app. |

## Messaging channel/Channel connected to the practice/Who manages messaging

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| You manage the messaging connection | missing |  | no | No manager fact on the wire and nothing rendered. |
| Someone else manages messaging | missing |  | no | Nothing names who manages messaging. |

## Patient message: match

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No correction waiting on Undo.No message yet | not shown |  | n/a | The absence of a message; the graph shows nothing. |
| No correction waiting on Undo.Coda is matching the message | not shown |  | n/a | A transient internal step with no copy in the graph. |
| No correction waiting on Undo.Message to match.Waiting to be matched | missing |  | no | No match row or Confirm/Another patient/Not a patient; TodayRow has no message, sender or suggestion and needs a patient. |
| No correction waiting on Undo.Message to match.Waiting to be matched, looks urgent; leads Today as Now | missing |  | no | TodayRows now marks urgency "now" rows "Needs you now", but no match row exists to carry it. |
| No correction waiting on Undo.Linked to the patient | partial | ReplyReviewCard (physician-app lib.rs) | no | Implied only by a reply card; the patient's message never shows and there is no Wrong patient?/Move/Mark not a patient. |
| No correction waiting on Undo.Another patient's message | partial | work_end_reason (shell.rs); PatientSurface Current work (physician-app lib.rs) | no | Only the ended-work line "This conversation is no longer in this patient’s record."; no Link control. |
| No correction waiting on Undo.Not a patient, until the sender writes again | missing |  | no | No not-a-patient state and no Link control. |
| Moved to another patient, Undo shown | missing |  | no | No correction command and no Undo. |
| Marked not a patient, Undo shown | missing |  | no | No correction command and no Undo. |

## Patient message: read

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Coda has read the conversation | not shown |  | n/a | The normal state, with no marker. |
| Coda couldn't read a message, on Today | missing |  | no | A generic Today row would render if sent, but there is no message link or Done, and TodayRow has no link field. |
| Couldn't read a message, marked done | missing |  | no | No Done command, so nothing reaches or renders it. |

## Practice reply in the channel

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No practice reply since the patient wrote | not shown |  | n/a | The absence of a practice reply. |
| The practice replied in the channel | missing |  | no | WorkspaceReply has no practice-reply field and the card shows none. |

## Patient message: reply

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No open reply.No reply work | not shown |  | n/a | The Patient replies section is hidden when there are no replies. |
| No open reply.Closed, no reply needed | partial | PatientSurface Patient replies notice (physician-app lib.rs) | no | After reply.close the card goes and only a session line "Marked no reply needed" may stay; closed replies aren't listed. |
| No open reply.Closed, the practice answered in the channel | missing |  | no | No answered-in-channel closure on the wire or in the app. |
| No open reply.Closed, sent by Coda for you | missing |  | no | No Send and no sent-for-you fact on the wire or in the app. |
| Reply work open | built | ReplyReviewCard, PatientSurface Patient replies (physician-app lib.rs); reply_urgency_words, reply_channel_label (shell.rs) | yes | Headline, urgency in words ("Urgent, ED"), "Replying in Gmail/Spruce" and the draft; the superseded-by-corrected-labs card is extra. |

## Patient message: reply/Reply work open/Reply status

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Reply ready.Ready to send or copy | partial | ReplyReviewCard (physician-app lib.rs); reply_review_actions (shell.rs) | no | Copy reply and No reply needed exist with a sending/couldn't-confirm/refused line; Send to the patient is missing. |
| Reply ready.Couldn't send, your text kept | missing |  | no | No Send, so no send failure state. |
| Sending, as Coda for you | missing |  | no | No Send and no sending state for a patient reply. |
| Sending, as Coda for you; the patient wrote again | missing |  | no | Not represented. |
| Reviewing a new message | missing |  | no | A non-ready reply just loses its actions; nothing says a newer message is under review. |
| Closed, no reply needed, Undo shown | partial | PatientSurface reply notice, CommandSlot notice (physician-app lib.rs) | no | "Marked no reply needed" shows once the close goes through, but the card vanishes and there is no Undo. |

## Patient message: reply/Reply work open/Your reply text

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Untouched, shows Coda's newest text | built | ReplyReviewCard FormattedMessage (physician-app lib.rs) | yes | The draft renders read-only and rereads after a revise; with no editable box it never leaves this state. |
| Your edited reply | missing |  | no | The draft can't be edited; the live sample has a reply-drafts read/save route, but the wire and client don't carry it. |
| Your reply, with Coda's update offered | missing |  | no | Not represented in the app or the wire. |
| Coda's text replaced yours, Restore available | missing |  | no | A revise silently replaces the draft; the live sample's reply-drafts route returns restore_text, but the wire and app don't use it. |

## Patient message: reply/Reply work open/Who has opened the reply

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| New to you | missing |  | no | The Today row's live-dot shows on every row, and the wire has no seen state. |
| Opened by you, unchanged since | missing |  | no | No seen state, and opening isn't recorded. |

## sidebar preference

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| sidebar expanded | built | PhysicianShell sidebar collapse buttons (physician-app lib.rs); panels.rs | yes | Remembered per physician in this browser's localStorage. |
| sidebar collapsed | built | PhysicianShell has-collapsed-sidebar (physician-app lib.rs); panels.rs | yes | Remembered per physician in this browser's localStorage. |

## schedule preference

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| schedule expanded | built | ScheduleRail (physician-app lib.rs); panels.rs | yes | Remembered per physician and shared by Today and the patient page. |
| schedule collapsed | built | ScheduleRail collapsed tile (physician-app lib.rs); panels.rs | yes | Remembered per physician and shared by Today and the patient page. |
