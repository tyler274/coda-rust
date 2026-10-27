# Build requirements: unit-1950 (Yolanda Soto, MRN00001950)

Source: synthetic_hospital patient 1950, in `source.md` beside this file. This file and
the source are the only authority for clinical facts. Every diagnosis, medication,
result, procedure and specialist finding in the chart must trace to one of:

- **S** a fact in source.md, carried as listed below;
- **R** a resolution below, where the source contradicts itself or stops short;
- **A** an addition below, listed one by one;
- **P** the primary-care layer described at the end.

Nothing else. No diagnosis, medication, test result, procedure, family member or
life event that is not on these lists. Texture (wording, scheduling, refills, portal
traffic, who signed what) is free, as long as it carries no new clinical fact.

Re-voice everything. The source notes are exam vignettes: never copy a sentence, and
never carry a source clinician's name, its "Active Problem List" dumps, its
"(diagnosed ...)" stamps, or its quoted phrases word for word.

## Who she is (S)

Born 2003-01-02, female, Hispanic or Latina (race recorded as other), English-speaking,
covered on a parent's employer-sponsored plan. Undergraduate biology major and
part-time barista. Never smoker; social alcohol, 1 to 2 drinks on weekends; no drug use
(her urine toxicology is negative at every ED visit).

- **Allergies:** penicillin (reaction not recorded in the source; the chart leaves it
  unrecorded); dust mites (environmental, text allergen).
- **Surgical history:** tonsillectomy at age 7 (2010); wisdom teeth extraction at age 18
  (2021). History only, no dates beyond the year.
- **Family history:** maternal uncle with schizophrenia; paternal grandmother with
  bipolar I disorder; mother with generalized anxiety disorder.
- **No psychiatric history** before February 2026.

**Joined the practice 2023-08-22** (A1), age 20, after aging out of a pediatric and
adolescent practice. She brought:

- Problems: seasonal allergic rhinitis; acne vulgaris.
- Drugs: cetirizine 10 mg by mouth daily (over the counter); drospirenone 3 mg /
  ethinyl estradiol 0.02 mg, 1 tablet by mouth daily (R1); clindamycin 1% gel to the
  face twice daily.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2023-08-22 | practice | new-patient visit (A1): history, allergies and family history as above; the three drugs continued and the practice takes over prescribing the contraceptive and clindamycin gel; normal exam and normal BP; PHQ-2 negative |
| 2023-09-06 | outside records | prior pediatric practice records received: immunizations up to date (Tdap, MenACWY with booster, HPV series), acne and allergic rhinitis followed there, tonsillectomy in childhood (P) |
| 2023-10-17 | practice | nurse visit: influenza and COVID vaccines (P) |
| 2024-02-06 | practice | follow-up: acne steady on clindamycin gel and the contraceptive; BP normal on the estrogen-containing pill; no changes (P) |
| 2024-06-18 | practice | annual physical: no-show (P) |
| 2024-08-20 | practice | annual physical (P): cervical cytology (first, at 21), chlamydia and gonorrhea NAAT, HIV and hepatitis C screening, CBC, CMP and lipid panel, all normal; PHQ-9 and GAD-7 low |
| 2024-10-22 | practice | nurse visit: influenza and COVID vaccines (P) |
| 2024-12-13 | practice | **last seen**: routine follow-up for contraception, acne and allergies; BP normal; PHQ-2 negative; refills for the year (P) |
| 2026-02-10 | outside (ED) | brought by her roommate for strange behavior: increasingly withdrawn, alone in her room, believes her thoughts are broadcast on the radio, hears voices commenting on her actions; no prior psychiatric history. Vitals normal. Disheveled, appears to respond to internal stimuli, oriented to person, place and time, memory intact, speech coherent but slow and quiet. CBC, BMP and TSH normal; urine toxicology negative. **Schizophreniform disorder diagnosed.** Discharged home with her roommate and an outpatient psychiatry referral; no medication started (R2, R3). Record received 2026-02-13 |
| 2026-03-21 | outside (ED) | returns about a month later with her roommate: more withdrawn, talks to herself, believes the government put a chip in her brain; has stopped going to classes and left her barista job, believing coworkers were watching her as part of a plot. First episode of psychiatric illness. Family history as above. Unkempt, anxious, poor eye contact, internally preoccupied and mumbling, disorganized speech, paranoid delusions. Urine toxicology negative. Discharged as in R2. Record received 2026-03-25 |
| 2026-05-23 | outside (ED) | third ED visit since February, brought by her roommate: over 3 months more withdrawn, mutters to herself in her room, neglected then stopped going to work, laughs inappropriately, appears to respond to unseen stimuli, jumps between unrelated topics, disorganized speech, stopped bathing, unwashed clothes and hair. Poor eye contact, flat affect, mumbled speech with loose associations; neurologic exam unremarkable. Urine drug screen negative; BMP normal. Discharged as in R2. Record received 2026-05-27 |
| 2026-06-09 | outside (psychiatry) | outpatient psychiatry initial evaluation (A2): history as in the ED records; diagnosis schizophreniform disorder; **aripiprazole 5 mg by mouth daily started**; living with her roommate, mother involved; not attending classes or working. Letter received 2026-06-12 |
| 2026-07-21 | outside (psychiatry) | psychiatry follow-up (A2): taking aripiprazole 5 mg daily; voices less frequent and less distressing; no restlessness or stiffness; weight stable; continue the same dose, return in about 6 weeks. Letter received 2026-07-24 |
| 2026-08-11 | practice (portal) | she writes asking to come back in and for a contraceptive refill; 3-month refill sent and a visit booked (P) |
| 2026-09-17 | practice | booked visit (P): follow-up after the ED visits and psychiatric care, medication review |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source gives the combined pill only as "1 tablet daily". Resolution: the
  drospirenone 3 mg / ethinyl estradiol 0.02 mg tablet, taken daily, on every list.
- **R2.** The source gives no disposition or plan at any ED visit and starts no drug.
  Resolution: each ED record ends with discharge home with her roommate and an
  outpatient psychiatry referral, and no medication started.
- **R3.** The first ED record calls the family history noncontributory, while later
  records and the practice list a schizophrenic uncle, a bipolar grandmother and an
  anxious mother. Resolution: the first ED record states that no family psychiatric
  history was obtained at that visit; the practice has the family history from 2023.
- **R4.** The source's problem lists stamp schizophreniform disorder as diagnosed at
  the first ED visit and carry it on all later ED lists; source dates in the text
  ("since January 2020") follow the re-dated timeline (since February 2026).
- **R5.** After `last_seen` the practice's lists are not reconciled: schizophreniform
  disorder and aripiprazole appear in the ED and psychiatry records only, and the
  practice's problem and medication lists stay as they were on 2024-12-13.
- **Left as the source has it:** three ED visits for worsening first-episode psychosis
  with no antipsychotic started and no brain imaging; nobody comments on either.
  Topical clindamycin is used alone without benzoyl peroxide; no one changes it.
  Penicillin's reaction is never recorded.

## Additions (A): physician to confirm

- **A1.** The 2023-08-22 new-patient visit (the seed gives about 3 years with the
  practice; the ED records show established problems and drugs).
- **A2.** Standard ongoing care for the source's untreated schizophreniform disorder,
  run by outpatient psychiatry after the third ED visit: initial evaluation 2026-06-09
  starting aripiprazole 5 mg daily (a conservative, metabolically lighter choice in a
  young woman on a combined pill), and one follow-up 2026-07-21 with partial response
  and no side effects. Both reach the practice as letters only.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Records from the prior pediatric practice may mention routine immunizations and
  screening done there, with normal results.
- Visits for contraception, acne and allergic rhinitis about every 6 to 10 months, BP
  checked on the estrogen-containing pill; a yearly physical with cervical cytology at
  21, chlamydia and gonorrhea screening, HIV and hepatitis C screening once, baseline
  CBC, CMP and lipids, PHQ and GAD screens (all low before 2026).
- Influenza each fall and COVID vaccines through 2024.
- Refills (contraceptive, clindamycin gel), pharmacy traffic, portal messages about
  results and scheduling, one missed physical, reminders. A refill after `last_seen`
  is allowed; no practice visit after 2024-12-13.
- Booked visit after 2026-08-27: follow-up after the ED visits and psychiatric care.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00001950 on 2024-12-13. Practice contacts after that day are
only messages and refills; outside care after it still happens, and its records are
filed as received without changing the practice's lists. A visit is booked after today.
