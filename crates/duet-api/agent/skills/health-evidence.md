---
name: health-evidence
description: Investigate connected records and measurements, resolve dates and revisions, and cite the exact evidence actually read.
---

The person's records are what Duet has actually received about them. Your
health overview lists their sources, the measurements with daily values, the
groups of records with the filter that finds each.

For any question about a measurement over time, such as last week against the
week before or since a visit, call read_measurement with the dates the
question is about. It returns each source separately, with its name, every day's value and code's totals, daily
averages, highest and lowest days and days with data, for the whole span or
each week or month, trend and variability where defined, and comparisons between two spans. Use its numbers rather than
adding up days yourself, and attach the values you use by their `fact_id`.
Never splice or average sources together. Judge their coverage over the dates
the person asked about; a newer device does not replace older history from
another source. Hospital and clinic measurements use this same read.

read_health_summary reads the phone's workout, specialized-record and demographic
summaries named in phone_summaries. Its original source fields and calculations
are evidence you may cite once a fact is fully read. These are summaries;
raw waveforms, workout routes and full phone-only records remain on the phone.

find_health_records lists records with their dates and values; read up to 20
whole records at once with read_health_records. A search can filter on when a
measurement started or when a record was recorded; build each day's start and
end in the person's timezone. The order records arrived says nothing about
either date.

Each measurement read names the calendar days and exact calculation. A daily
average is not a period total. A highest or lowest day has its own date; the
period's boundaries do not locate that reading. An earlier comparison period
is a comparison, never a goal, normal range or the person's usual level unless
a cited source establishes that. These distinctions guide your interpretation.

Available facts and visual options have not been screened for significance or
usefulness. Use their sample counts, coverage, fit strength and comparison
baselines to judge what they support and what is worth bringing up. A sparse
history, weak fit, unchanged value or large percentage from a small baseline
needs that context; availability alone is no reason to make a claim or a visual.

A record read with a newer version is history: a newer revision of it exists.
Say so when it matters, and read the current revision before the person acts
on an instruction from the old one.

read_health_sources shows source access, import progress and coverage; read it
when they change the answer.

A measurement the person attached to their message arrives with its value and
dates as they were when attached. Cite it through that message instead of a
current summary, and read the message first when the attachment is older.

Cite the sources your answer rests on, naming each by its identity. A card you
reopened is cited with the same message and source id, and stays the value and
version from that time. The card the person opens has to be the one you
actually used.
