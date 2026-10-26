# Writing a build-req

A unit's `build-req.md` turns one synthetic_hospital patient (`source.md`) into a
patient of our practice: an adult membership internal medicine practice in western
Massachusetts whose today is **2026-08-27T12:00** Eastern. The chart's author treats
it as the only authority for clinical facts, and an independent reader checks the
chart against it, so it must be complete, exact and coherent. A physician confirms
its resolutions and additions.

The source is a set of exam vignettes clustered into one patient. It is a good
story and an unreliable record: its profile lists everything at once, its notes
copy problem lists forward and backward, its numbers jump, its plans stop
mid-sentence, and its dates sit on the 15th of the month. Your job is to keep
every clinical fact it gives and make the whole thing true at once.

## What you write

Use this shape (`patients/units/unit-2013/build-req.md` is a worked example, if
you want one):

1. **Header.** The rules, as the pilots state them: every clinical fact traces to
   S (the source as carried here), R (a resolution), A (an addition) or P (the
   primary-care layer), and nothing else; texture is free; re-voice everything.
2. **Who they are (S).** Identity from the seed (never change it), then occupation,
   tobacco, alcohol, allergies, surgical and family history from the source. When
   they joined the practice and what they brought (problems, drugs) on joining.
3. **Timeline.** A table, one row per dated contact: date, where (practice or
   outside), and what happens with every clinical fact and number the source gives
   for it, carried exactly. Bold what starts or stops a drug or a diagnosis.
4. **Resolutions (R), numbered.** Each contradiction in the source and how the
   chart resolves it. Then **Left as the source has it:** the care gaps the source
   leaves, which the chart keeps without comment.
5. **Additions (A), numbered.** Each with its reason.
6. **Primary-care layer (P).** What may surround the timeline without listing.

## The timeline

- **Dates.** `source.md` gives each encounter a practice date: the source date
  moved to end before today and off the 15th. Use those dates. Move one only when a
  source statement needs it (the source says a cough began "six weeks after
  starting" a drug: the start and the cough visit must be about that far apart),
  and record the move as a resolution. Clinic visits fall on weekdays.
- **Where.** An internal or family medicine outpatient encounter is a practice
  visit. Every other department is outside care: a consult letter, an ED record, a
  discharge summary, an urgent care note, each received by the practice. Add the
  practice contact that plausibly sent the patient there (a referral visit) or
  followed up after (a post-ED or post-discharge visit) as P.
- **Joining.** If the first source encounter reads as established care ("routine
  follow-up", "returns"), the patient joined earlier: give a new-patient visit date
  (an addition) and what they brought. A condition the profile lists from the start
  but the encounters diagnose later is not known at joining: it starts at its
  encounter.
- **Ages and years.** Ages the source states follow from the seed's birth date and
  the practice dates. A year stated in source text ("seen in 2022") follows the
  re-dated timeline.
- **After the source ends.** The years between the last source encounter and today
  hold only the primary-care layer, with each condition's ordinary course.

## Resolutions

Resolve every contradiction. The common ones:

- A drug listed before the encounter that starts it, or continued after the one
  that stops it. Practice lists follow the resolved course; an outside letter may
  still copy a stale entry, as outside lists do.
- An allergy recorded before the reaction that caused it.
- A diagnosis "diagnosed" before its evidence, or a condition on every list from
  the first visit that a later encounter diagnoses.
- Numbers that cannot all be true (a BMI of 32 then 27 three months later; four
  LDLs on one statin dose; pack-years of 45, 40, 30 and 25). Pick the coherent set,
  usually the most clinically central value, and say which values are not carried.
- Tests a setting would not do (an arterial gas at an office visit): drop them and
  keep what carries the finding.
- Quiz text, unfinished plans and "is shown" images: say what the chart does
  instead. Never carry the quiz.
- Interobserver differences between two examiners (a bruit one heard and one did
  not) may stay as written. Say so.

Where the source changes nothing (no drug raised for an LDL of 130, no imaging
after a neurological event), the gap stays: list it under "Left as the source has
it". Do not repair the source's medicine.

## Additions

Keep them few. An addition is justified only when the source needs it to be
coherent or when any physician would do it at a source event: a drug the source
names later (it must start somewhere), potassium for a potassium of 2.9, a test
that supports a diagnosis the source asserts with no evidence (one ABI for a listed
PAD), the visit that made the referral. Give each addition's numbers or findings
exactly. Never add a diagnosis, a new problem, a family member or a life event.

## The primary-care layer

Write it for this patient, following the pilots:

- Visits for the active conditions at a realistic cadence; yearly physicals or
  Medicare wellness visits; the labs those conditions call for.
- Age- and sex-appropriate prevention with normal results: colorectal, cervical,
  breast and lung screening as indicated; influenza each fall, COVID, pneumococcal,
  shingles, RSV and Tdap as indicated; hepatitis C and HIV screening once;
  depression screens. A screen the source makes relevant (lung screening in a heavy
  smoker) may be listed as an addition with its results.
- Refills, pharmacy and prior-authorization traffic, portal messages about results,
  refills and scheduling, a missed visit.
- A booked visit after today.
- Reports and exams state ordinary normal findings for what the source does not
  mention. An abnormal finding appears only as the source or build-req gives it: a
  report may carry the descriptors its kind always carries (how a size was measured,
  a benign nodule's size and lobe within its Lung-RADS category, a procedure's
  access site), but nothing that adds a territory, a severity grade or a symptom,
  or changes the diagnosis or plan.
- Records from a prior practice may mention routine screening done there, with
  normal results.
- Close with: no acute illnesses, injuries, new diagnoses or life events beyond the
  timeline.

## Rules

- Write build-req.md before any part file, and keep the chart to it. If writing the
  chart shows build-req.md was wrong, fix build-req.md first.
- Refer to clinicians by role. Never name a real practice, hospital, lab or
  pharmacy ("a regional hospital", "an outside cardiology group").
- If the source cannot be an adult primary-care patient at all, write only
  `UNFIT: <reason>` and stop. That is rare: an ED-, ICU-, OB- or specialist-heavy
  patient still has a primary-care physician.
