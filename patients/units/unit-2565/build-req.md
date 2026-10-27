# Build requirements: unit-2565 (Ethan Vaillancourt, MRN00002565)

Source: synthetic_hospital patient 2565, in `source.md` beside this file. This file and
the source are the only authority for clinical facts. Every diagnosis, medication,
result, procedure and specialist finding in the chart must trace to one of:

- **S** a fact in source.md, carried as listed below;
- **R** a resolution below, where the source contradicts itself;
- **A** an addition below, listed one by one;
- **P** the primary-care layer described at the end.

Nothing else. No diagnosis, medication, test result, procedure, family member or
life event that is not on these lists. Texture (wording, scheduling, refills, portal
traffic, who signed what) is free, as long as it carries no new clinical fact.

Re-voice everything. The source notes are exam vignettes: never copy a sentence, and
never carry a source clinician's name, its "Active Problem List" dumps, its
"(diagnosed 2020-01-15)" stamps, or its quiz text.

## Who he is (S)

Ethan Vaillancourt, born 1989-07-09, male, White, not Hispanic or Latino, English,
commercial PPO. Senior financial analyst. Never smoker; occasional social alcohol,
1 to 2 drinks a week. No known drug allergies. No surgical history. Served in the
military earlier in adult life, with a combat deployment (R2). Family history:
father with hypertension at 58; mother with major depressive disorder; maternal aunt
with generalized anxiety disorder.

Joined the practice in August 2023 (A1). He brought essential hypertension, diagnosed
at his prior practice (onset about 2021), on **hydrochlorothiazide 25 mg daily** (R1).
No psychiatric diagnoses or medications on joining.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2023-08-22 | practice | new patient visit (A1): history as above; BP controlled on hydrochlorothiazide 25 mg; continues it. Weight 82 kg, height 180 cm. Screening labs (P) |
| 2023-09-12 | outside | prior practice records received: hypertension, hydrochlorothiazide, normal routine labs (P) |
| 2023-12-12 | practice | visit for work stress (A2): perfectionism, long hours rechecking his reports, trouble delegating; **referred to outpatient psychiatry** |
| 2024-01-30 | outside | psychiatry evaluation: for months has spent excessive hours checking financial reports for errors, stays late so they are flawless, dissatisfied despite good reviews, cannot delegate for fear the work will fall short; denies recurrent intrusive thoughts and repetitive acts done to relieve anxiety. On hydrochlorothiazide. BP 138/86. Well groomed, tense; exam otherwise unremarkable. CBC, TSH and fasting glucose within normal limits. **Obsessive-compulsive personality disorder diagnosed.** Psychotherapy recommended (A3). Letter reaches the practice late, 2024-03-19, after a records request (Records) |
| 2024-06-11 | practice | hypertension follow-up missed (no-show), rebooked (P) |
| 2024-08-20 | practice | annual physical (P): BP controlled on hydrochlorothiazide; Tdap; screening labs normal |
| 2024-11-05 | practice | visit (A4): about 3 months (since early August 2024) of nightmares and flashbacks reliving combat, waking in panic, poor sleep onset, feeling constantly on alert; no suicidal ideation. **Sertraline 50 mg daily started**; **referred back to psychiatry** |
| 2025-02-03 | outside | psychiatry: 6 months of recurrent vivid combat nightmares and flashbacks with panic and broken sleep; on sertraline 3 months, only partly better; trouble falling asleep, persistent hyperalertness; no suicidal ideation. On sertraline and hydrochlorothiazide (R1). Vital signs within normal limits (no numbers); exam unremarkable. **Post-traumatic stress disorder diagnosed**; continue sertraline 50 mg; trauma-focused psychotherapy recommended (A3). No letter sent at the time; the practice hears of the diagnosis from him on 2025-03-18 and gets the letter 2025-08-26 (Records) |
| 2025-03-18 | practice | follow-up (P): tells the PCP psychiatry diagnosed PTSD and kept sertraline; BP check; sertraline refilled |
| 2025-05-20 | practice (portal) | he writes that he stopped sertraline on his own about three weeks earlier, feeling it was not doing much (R3); **sertraline stopped** on the practice list; PCP asks him to keep his psychiatry follow-up |
| 2025-07-25 | outside | psychiatry: 3 months of irritability, poor sleep, snapping at colleagues over small things, headaches and muscle tension; feels overwhelmed by responsibilities at home, where he is the main caregiver for his ailing mother; denies recent job or personal changes; appears anxious. No psychiatric medications (R1). BP 140/90, HR 88; exam otherwise within normal limits. CBC, TSH and urinalysis normal. **Adjustment disorder with anxiety diagnosed.** Supportive psychotherapy recommended (A3). Letter received 2025-08-26 with the February letter |
| 2025-09-09 | practice | annual physical (P): reviews both psychiatry letters; adjustment disorder and PTSD on the problem list; hydrochlorothiazide continued |
| 2026-03-10 | practice | last seen: hypertension follow-up (P); BP check, BMP; hydrochlorothiazide refilled |
| 2026-07-31 | outside | ED, by ambulance: while eating lunch at work he suddenly became unresponsive and fell, with convulsive movements for about 1 minute witnessed by a coworker, then several minutes of confusion, no memory of the event. Age 37 (R4). No cardiac history, no prior episodes. On hydrochlorothiazide. BP 140/85, HR 78, RR 16. Alert and oriented; exam unremarkable apart from a laceration on the lateral tongue. Sodium, potassium, calcium and magnesium within normal limits. ECG normal sinus rhythm, no abnormalities. **Seizure (recent seizure episode) diagnosed** in the ED record; the practice's problem list is not updated (after last seen). Discharged home with outpatient neurology follow-up advised and no driving until neurology clears him (A5) |
| 2026-08-03 | practice (portal) | he writes about the seizure and ED visit; front desk books a post-ED visit; ED record requested (P) |
| 2026-08-24 | outside | ED record received by fax, filed as received (Records) |
| 2026-09-02 | practice | booked visit after today: follow-up after ED visit for seizure |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Blood-pressure drug: the source lists hydrochlorothiazide in January 2024 and
  July 2026, "well controlled with lisinopril" in February 2025, and "does not take any
  medications" in July 2025. Resolution: hydrochlorothiazide 25 mg daily throughout, from
  before joining to today. Lisinopril is not carried. The July 2025 psychiatry note's "no
  medications" is carried as no psychiatric medications (sertraline already stopped);
  its letter simply omits hydrochlorothiazide, as outside lists do. No dose change ever.
- **R2.** "Following military service" with symptoms for only six months: carried as
  delayed-onset PTSD from a combat deployment earlier in adult life, before he joined
  the practice; symptoms begin early August 2024. No branch, dates or deployment details
  are given. The January 2024 evaluation (no intrusive thoughts) predates them.
- **R3.** Sertraline is "for the past three months" in February 2025 and absent by July
  2025. Resolution: the practice starts it 2024-11-05 (A4), psychiatry continues it, and
  he stops it himself in late April 2025, telling the practice by portal on 2025-05-20.
  No side effect is named.
- **R4.** The ED note calls him 36; from the birth date he is 37 on 2026-07-31. The
  chart uses 37. February 2025 "35" and January 2024 "34" follow the dates as given.
- **R5.** Source years in text follow the re-dated timeline: the "2020" evaluation is
  January 2024 and "early 2021" is February 2025.
- **Left as the source has it:** the ED does no head imaging, no EEG, no glucose or
  other labs beyond the four electrolytes, and starts no antiseizure drug; nobody
  comments. Blood pressure runs at 138 to 140 over 85 to 90 at outside visits on
  hydrochlorothiazide and is never intensified. The caregiving for his mother is not
  explored further by the practice. Outpatient neurology has not happened by today.

## Additions (A): physician to confirm

- **A1.** The 2023-08-22 new-patient visit (seed: about 3 years with the practice;
  the first source encounter is a specialist visit, so he needs a PCP before it).
- **A2.** The 2023-12-12 practice visit that refers him to psychiatry.
- **A3.** Psychiatry's recommendation of psychotherapy at each of its three visits
  (standard care for OCPD, PTSD and adjustment disorder). He tells the practice he
  has a therapy intake planned (March 2025) and attends therapy from then on (reported
  September 2025 and March 2026); no therapist records reach the chart.
- **A4.** The 2024-11-05 practice visit that starts sertraline 50 mg daily and refers
  him back to psychiatry (the source has him three months on sertraline by February
  2025, before psychiatry diagnosed PTSD).
- **A5.** The ED's discharge advice: outpatient neurology follow-up and no driving
  until cleared (usual first-seizure discharge). No neurology visit by today.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Records from a prior practice may mention routine screening done there, with normal results.
- Practice blood pressure on hydrochlorothiazide about 124/78, weight steady about 82 kg,
  height 180 cm. Visits as in the timeline, plus fall vaccine nurse visits; BMP yearly; lipids and CBC at the 2023 visit and
  physicals, normal.
- Prevention for a man in his thirties: HIV and hepatitis C screening once (negative),
  Tdap once if not documented, influenza each fall, COVID vaccines in the fall, depression
  screening. Screening scores: PHQ-9 3 and GAD-7 2 on 2023-08-22; PHQ-9 5 and GAD-7 9 on
  2023-12-12; PHQ-9 8 and GAD-7 11 on 2024-11-05; PHQ-9 6 and GAD-7 8 on 2025-09-09;
  PHQ-2 1 on 2026-03-10.
- Refills, pharmacy traffic, portal messages about results, refills and scheduling,
  records requests, a missed visit.
- Booked visit after 2026-08-27: follow-up after the ED visit for seizure.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Records

Chart texture is "gaps". The prior practice's records arrive in September 2023. The
January 2024 psychiatry letter comes only after a records request in March 2024. The
February 2025 psychiatry visit sends nothing at the time: the practice learns of the PTSD
diagnosis from him in March 2025, and both 2025 psychiatry letters arrive together in
late August 2025. The ED visit reaches the practice first as his portal message, then as
the faxed ED record on 2026-08-24, filed without changing the practice's lists.

## Last seen

The practice last saw MRN00002565 on 2026-03-10. Practice contacts after that day are
only messages, refills and records; outside care after it still happens, and its records
are filed as received without changing the practice's lists. A visit is booked after today.
