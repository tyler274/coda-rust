# Build requirements: unit-1838 (Sadie Nakamura, MRN00001838)

Source: synthetic_hospital patient 1838, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its wording.

## Who she is (S)

Sadie Nakamura, born 1997-11-10, female, White, not Hispanic or Latino, English,
employer-based PPO, lives in Williamsburg (01096). Elementary school teacher. Never
smoker, no alcohol, no drug use. No known drug allergies. No surgical history. Family
history: mother with hypothyroidism, father with hypertension. Has a supportive family.
No psychiatric history before the crash.

Joined the practice on 2024-09-10 (new-patient preventive visit, A1), healthy, on no
medications, with no problems on her list.

Records texture (seed): sparse. Outside care reaches the practice mostly as the
patient's own account, a short letter or a short ED note, with few numbers.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2024-09-10 | practice | new-patient preventive visit (A1): history as above, normal exam and vitals, height 165 cm, weight 61 kg; screening labs (P) normal; records requested from her prior practice |
| 2025-07-21 | (no record) | motor vehicle collision: a severe crash in which a friend of hers was killed; she witnessed the death and had only minor injuries (R1, R2). Known to the chart only through her account |
| 2025-07-28 | practice | visit a week after the crash (A2): trouble falling and staying asleep, nightmares, poor concentration; minor injuries healing; vitals normal, exam unremarkable. **Urgent psychiatry referral placed**. No medication |
| 2025-08-04 | outside | psychiatry intake, received as a short consult letter: two weeks after the crash; recurrent distressing memories, avoids driving and talking about it, feels emotionally numb and detached from people, hypervigilant, startles easily; interfering with work and social life; no prior psychiatric illness; vitals within normal limits, exam unremarkable. **Acute stress reaction diagnosed (F43.0).** No medication. Plan: weekly trauma-focused therapy with an outside therapist (A3), labs before the next visit, follow-up in about a month |
| 2025-08-27 | practice | labs drawn at the practice at psychiatry's request (R3): CBC within normal limits, TSH within normal limits, urine drug screen negative. Copies sent to psychiatry |
| 2025-09-02 | outside | psychiatry follow-up (R4). Reaches the chart only through her portal message the next day: flashbacks, poor sleep, constantly on edge; still not driving; has not gone back to work since the crash (the school year has started). Still no medication; therapy continues. The psychiatry exam (anxious, heart rate up) never reaches the chart |
| 2025-09-05 | practice | leave-of-absence paperwork from her school district completed (P) |
| 2026-01-17 | outside | ED visit (a Saturday), received as a short ED note: returns after the August and September 2025 psychiatry visits for the crash; trouble sleeping, startles at loud noises, still avoids driving, not back at work, feels detached from her surroundings with a sense that things are not real. BP 130/85, HR 98, RR 18, temperature 37.0 C (98.6 F); appears anxious, heart rate up; CBC and basic metabolic panel within normal limits. Acute stress reaction; no medication; discharged home to follow up with psychiatry and primary care |
| 2026-01-21 | practice | post-ED visit (P): same symptoms; appears anxious; vitals otherwise unremarkable; exam unremarkable; denies thoughts of self-harm. Continue therapy; she will ask psychiatry for an earlier visit. No medication |
| 2026-02-10 | outside | psychiatry visit (A4), known only from her portal message: continues weekly therapy, no medication |
| 2026-05-12 | practice | follow-up (A5): still in weekly therapy; nightmares less frequent; still not driving, family drives her; vitals normal; exam unremarkable. Annual physical to be booked |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Encounter 0 says she "witnessed a fatality" and had "minor injuries";
  encounter 1 calls it a "severe car accident in which her friend passed away".
  Resolution: one crash, severe, in which a friend died at the scene in front of her;
  her own injuries were minor. No injury is named and none is treated in the chart.
- **R2.** The intake (2025-08-04) puts the crash two weeks earlier; the follow-up a
  month later says "3 weeks after" the accident. Resolution: the crash is on
  2025-07-21; the follow-up is about six weeks after it, and the "3 weeks" is dropped.
- **R3.** The source lists a blood alcohol level of 0.00% with the follow-up labs, a test
  an outpatient psychiatry visit would not send. Resolution: dropped. The CBC, TSH and
  urine drug screen are drawn at the practice on 2025-08-27 at psychiatry's request,
  and psychiatry reviews them at the follow-up.
- **R4.** The source's follow-up date falls on Labor Day (2025-09-01). Resolution:
  the psychiatry follow-up is on Tuesday 2025-09-02.
- **R5.** The source lists "Motor vehicle accident" as a comorbidity. Resolution: the
  crash is carried as history in notes, not as a problem-list entry.
- **Left as the source has it:** the label stays acute stress reaction through the
  ED visit and after, although the symptoms run well past a month; nobody relabels it.
  No medication is ever started (not for sleep, not an SSRI), and no note comments on it.
  Her driving avoidance and time off work are recorded, not resolved.

## Additions (A): physician to confirm

- **A1.** 2024-09-10 new-patient preventive visit (the seed gives her about two years
  with the practice).
- **A2.** 2025-07-28 practice visit that sends her to psychiatry.
- **A3.** Weekly trauma-focused psychotherapy with an outside therapist from August
  2025, the standard care for acute stress reaction, which the source never treats.
  Known only from her account; no therapy notes reach the chart.
- **A4.** A psychiatry visit after the ED visit (2026-02-10), the usual next step; known
  only from her message; no medication.
- **A5.** 2026-05-12 practice follow-up: in therapy, nightmares less frequent, still not
  driving. No new diagnosis, no medication.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not
  mention. An abnormal finding appears only as the source or this file gives it.
- Records from her prior practice may mention routine screening there with normal
  results: a normal Pap in 2023, childhood and adolescent vaccines complete, Tdap as an adult.
- Establish-care labs: CBC, CMP, lipid panel, TSH, HIV and hepatitis C screens, all
  normal. PHQ-2 of 0 at joining.
- Influenza vaccine each fall (2024, 2025) and an updated COVID vaccine each fall.
- Portal messages, scheduling, records requests, leave-of-absence forms, a cancelled
  visit.
- Booked visit after 2026-08-27: annual physical (cervical screening due).

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.


## Last seen

The practice last saw MRN00001838 on 2026-05-12. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. A visit is booked after today.
