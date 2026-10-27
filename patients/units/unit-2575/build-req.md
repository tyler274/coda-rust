# Build requirements: unit-2575 (Ethan Pickett, MRN00002575)

Source: synthetic_hospital patient 2575, in `source.md` beside this file. This file and
the source are the only authority for clinical facts. Every diagnosis, medication,
result, procedure and specialist finding in the chart must trace to one of:

- **S** a fact in source.md, carried as listed below;
- **R** a resolution below, where the source contradicts itself or leaves a hole;
- **A** an addition below, listed one by one;
- **P** the primary-care layer described at the end.

Nothing else. No diagnosis, medication, test result, procedure, family member or
life event that is not on these lists. Texture (wording, scheduling, refills, portal
traffic, who signed what) is free, as long as it carries no new clinical fact.

Re-voice everything. The source notes are exam vignettes: never copy a sentence, and
never carry a source clinician's name, its "Active Problem List" dumps or its
"(diagnosed 2020-01-15)" stamps. Years in source text follow the re-dated timeline
(psychotic symptoms "since January 2020" become since spring 2024, first psychiatry
visit October 2024).

## Who he is (S)

Ethan Pickett, born 2001-09-13, male, White, not Hispanic or Latino, English, lives in
Amherst (01002) with his parents, on his parents' PPO plan. University student; on
medical leave from the spring 2025 semester on (R3). Never smoker, no alcohol. Past
cannabis use, none currently (R2). Allergies: no known drug allergies. No surgical
history. No chronic conditions and no medications on joining.

Family history: maternal uncle with schizophrenia, paternal grandmother with bipolar
disorder, mother with major depressive disorder.

Joined the practice 2023-08-22 (age 21, A1): a new-patient preventive visit after
aging out of his pediatric practice. Healthy, no problems, no drugs.

Records texture: sparse. Psychiatry sends short letters; the ED sends a short note;
the inpatient stay reaches the practice as the family's account and a fill history,
and its discharge summary arrives only months later.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2023-08-22 (Tue) | practice | new-patient preventive visit (A1); history as above; normal exam; height 180 cm, weight 74 kg, BP normal; PHQ-9 low; HIV and hepatitis C screening negative; Tdap given (P) |
| 2023-10-12 (Thu) | practice | nurse visit: influenza and COVID vaccines (P) |
| 2024-07-23 (Tue) | practice | preventive visit, no-show (P) |
| 2024-09-24 (Tue) | practice | visit with his parents (A2): about five months of pulling away from friends and family, staying in his room, heard talking and laughing to himself; brief answers, flat; denies sadness, self-harm or harm to others; neuro exam normal; CBC, CMP and TSH normal (A2); **urgent referral to psychiatry** |
| 2024-10-16 (Wed) | outside | psychiatry evaluation, brought by his parents: six months of increasing isolation, talking to himself, laughing without reason; past cannabis use, none recent; vitals normal; disheveled, poor eye contact, monotone, brief answers; says mood is "fine", no sadness or anhedonia; denies self-harm or harm to others; thought process hard to follow, preoccupied; neurological exam normal. **Schizophrenia diagnosed.** Antipsychotic recommended and declined (R1). Letter received 2024-10-24 |
| 2025-02-20 (Thu) | outside | psychiatry follow-up, brought by his father, about four months after the first visit: ten months of strange behavior; withdrawn, has stopped attending classes; believes neighbors have hidden cameras in the house to spy on him and the television sends him secret messages; talks and laughs to himself alone in his room; no alcohol or drugs; denies mood symptoms. Antipsychotic again recommended and declined (R1); medical leave from the university (R3). The father's portal message reaches the practice 2025-02-24; a short psychiatry note arrives 2025-03-06 |
| 2025-09-16 (Tue) | practice | preventive visit with his mother (P): on medical leave, still followed by psychiatry, takes no medications; PHQ-9 low; vitals normal; influenza vaccine declined to a later nurse visit |
| 2025-10-09 (Thu) | practice | nurse visit: influenza and COVID vaccines (P) |
| 2026-02-22 (Sun) | outside | ED, brought by his parents: two weeks of erratic behavior and voices; responding to internal stimuli; increasingly paranoid that the neighbors are conspiring to harm him; takes no medications; vitals normal; disheveled, poor eye contact, internally preoccupied; CBC, CMP and urine toxicology screen unremarkable; CT head without contrast normal. **Admitted to inpatient psychiatry** (A3). Short ED note received 2026-02-24 |
| 2026-02-22 to 2026-03-04 | outside | voluntary inpatient psychiatric stay (A3); **risperidone started, 2 mg at bedtime at discharge** (A3); outpatient psychiatry follow-up arranged. The practice learns of it from his mother's call (2026-02-24) and the pharmacy fill; the discharge summary arrives 2026-05-14 (after last seen, filed as received) |
| 2026-03-12 (Thu) | practice | post-discharge visit with his mother (A4): home since 2026-03-04, taking risperidone 2 mg nightly, voices quieter, seeing outpatient psychiatry; vitals normal, weight unchanged; risperidone added to the list as prescribed by psychiatry; baseline antipsychotic monitoring: fasting lipid panel and A1c normal (A4) |
| 2026-04-17 (Fri) | practice | follow-up, **last seen** (A4): on risperidone 2 mg nightly from psychiatry, taking it; vitals normal, weight unchanged; repeat fasting lipids and A1c ordered for about three months on the drug (A5) |
| 2026-06-11 (Thu) | practice lab | the draw ordered on 2026-04-17: fasting lipid panel and A1c normal (A5); result sent by portal |
| 2026-07-09 | outside | outpatient psychiatry note received: continues risperidone 2 mg nightly, still on medical leave (A3). Filed as received |
| 2026-09-24 (Thu) | practice | booked: routine follow-up (P) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source diagnoses schizophrenia in October 2024 and follows him in
  psychiatry, yet sixteen months later he "takes no medications" and the source never
  says why. Resolution: at both psychiatry visits an antipsychotic is recommended and
  he declines it (he does not accept that he is ill); psychiatry follows him without a
  drug. No antipsychotic is started before the 2026 admission. The practice does not
  second-guess psychiatry's plan.
- **R2.** Encounter 0 records a history of cannabis use with no recent use; encounter 1
  says he uses no recreational drugs; the ED urine toxicology screen is unremarkable.
  Resolution: past cannabis use that ended before 2024, none since. No cannabis
  diagnosis is listed.
- **R3.** The source names him a student on medical leave only at the ED visit, and
  says in February 2025 he has stopped attending classes. Resolution: he went on
  medical leave from the university after the February 2025 psychiatry visit and is
  still on leave.
- **R4.** The source's "since January 2020" and "January 15, 2020" follow the re-dated
  timeline: symptoms from about April 2024, first psychiatry visit 2024-10-16.
- **Left as the source has it:** no antipsychotic during the source span (R1); the
  diagnosis is schizophrenia from the first psychiatry visit, with six months of
  symptoms, and is not revisited.

## Additions (A): physician to confirm

- **A1.** 2023-08-22 new-patient visit (three years with the practice; the source
  begins in psychiatry, so he had a primary-care home before it).
- **A2.** 2024-09-24 practice visit that sends him to psychiatry (the parents brought
  him to a clinic; someone referred him), with CBC, CMP and TSH, all normal, as the
  routine medical screen for new behavioral change.
- **A3.** ED disposition: voluntary admission to inpatient psychiatry at the regional
  hospital, 2026-02-22 to 2026-03-04, risperidone started and discharged on 2 mg at
  bedtime with outpatient psychiatry follow-up (the usual next step for acute
  psychosis with paranoia in untreated schizophrenia). The July 2026 psychiatry note
  carries the same dose; no other psychiatric drug.
- **A4.** 2026-03-12 post-discharge visit and 2026-04-17 follow-up, with baseline
  antipsychotic metabolic monitoring (weight, BP, fasting lipids, A1c), all normal; A1c 5.1%.
- **A5.** Repeat fasting lipids and A1c about three months into risperidone, drawn
  2026-06-11, normal; A1c 5.2%.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results:

- Reports and exams state ordinary normal findings for what the source does not
  mention. An abnormal finding appears only as the source or this file gives it.
- Records from his pediatric practice may mention routine childhood and adolescent
  immunizations and normal well visits.
- A yearly preventive visit (2024's was missed); PHQ-9 screens with low scores; vitals.
- Influenza each fall (fall 2024 not given), COVID vaccine with it; Tdap 2023.
- Portal messages and phone calls from him and his parents about scheduling, forms and
  results; a records request to the hospital; pharmacy fill traffic for risperidone,
  which psychiatry prescribes.
- Booked visit after 2026-08-27: routine follow-up on 2026-09-24.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002575 on 2026-04-17. Practice visits after that day do
not happen; the ordered June draw, messages and outside records do, and outside
records are filed as received without changing the practice's lists. A visit is
booked after today.
