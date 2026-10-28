# Build requirements: unit-2656 (Asher Kane, MRN00002656)

Source: synthetic_hospital patient 2656, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz-style lab dumps.

## Who he is (S)

Asher Kane, born 1995-02-22, male, White, not Hispanic or Latino, English, Medicaid.
Lives in an apartment with a roommate (S: the roommate at the 2026 ED visit). Unemployed;
previously worked as a maintenance worker. Never smoker, no alcohol, no illicit drugs.
No known drug allergies. No surgeries. Family history: mother with major depressive
disorder, father with hypertension; no bleeding disorders or sickle cell disease in the
family. He has no personal history of sickle cell disease or blood disorders.

Joined the practice 2022-08-23 (A1), age 27, coming from another primary-care office.
He brought: major depressive disorder, recurrent, moderate (onset 2018); insomnia
(onset 2020); medication non-adherence (prior office's records show missed refills and
self-stopped antidepressants, no drug named); trazodone 50 mg at bedtime for sleep,
which the practice takes over prescribing. Height 180 cm, weight about 82 kg, BMI
about 25, steady throughout.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2022-08-23 | practice | new-patient visit (A1): history as above; PHQ-9 15, GAD-7 8; normal exam; trazodone 50 mg at bedtime continued; **sertraline 50 mg daily started** (A2); baseline CBC, CMP, TSH, lipids, HIV and hepatitis C screening drawn, all normal (P) |
| 2022-09-27 | practice | video follow-up: stopped sertraline on his own after about ten days, felt no different; not willing to restart now; PHQ-9 14; **sertraline stopped**; psychiatry referral placed (A3) |
| 2023-01-24 | outside | outpatient psychiatry intake (A3); letter received 2023-02-01: recurrent moderate major depression confirmed, PHQ-9 13, no suicidal ideation, recommends an SSRI again with therapy at their clinic and asks primary care to prescribe |
| 2023-02-07 | practice | follow-up after psychiatry letter: **sertraline 50 mg daily restarted** (A2); PHQ-9 13 |
| 2023-04-25 | practice | phone: pharmacy reports no refill after the first fill; he stopped sertraline again ("felt flat"); **sertraline stopped** (A2) |
| 2023-06-13 | practice | depression and sleep follow-up; PHQ-9 12; declines other antidepressants; attends therapy now and then; trazodone continued |
| 2023-09-19 | practice | preventive visit (P); PHQ-9 11; flu, COVID and Tdap vaccines |
| 2024-04-16 | practice | follow-up for worse sleep; PHQ-9 16, GAD-7 9; no suicidal ideation; declines antidepressant again; sleep hygiene, therapy re-referral advice; trazodone stays 50 mg |
| 2024-09-24 | practice | preventive visit (P); PHQ-9 10; CBC and CMP normal; flu and COVID vaccines |
| 2025-06-10 | practice | video follow-up; PHQ-9 13; takes trazodone only some nights, refills late (non-adherence); no change |
| 2025-09-02 | outside | regional ED, priapism (S): painful erection for 6 hours; no trauma, no illicit drug use; takes trazodone; vitals normal; penis fully erect and tender, no trauma or injury, glans and corpus spongiosum not engorged; CBC and metabolic panel unremarkable, urinalysis normal. Corporal aspiration returned bright red blood with an arterial-pattern gas: pH 7.40, pO2 96 mmHg, pCO2 39 mmHg (A4). **Non-ischemic (arterial) priapism** diagnosed; managed with ice and observation, detumescence in the ED (R2); told to stop trazodone and see urology. ED record received 2025-09-08 |
| 2025-09-03 | practice | phone: patient calls about the ED visit; RN tells him to stay off trazodone and books a visit |
| 2025-10-02 | practice | **last seen.** Post-ED visit: no recurrence; **trazodone stopped** (R1); **priapism resolved**; urology referral placed (stays open: never scheduled, no letter); insomnia handled with sleep hygiene and CBT-I material, no new sleep drug; PHQ-9 14; declines antidepressant; flu vaccine |
| 2026-05-12 | outside | regional ED, bleeding (S): spontaneous gum bleeding and widespread bruising of arms and legs; roommate says he has been more withdrawn for weeks and may have swallowed something; he admits feeling depressed; he had been putting out over-the-counter rat poison in the apartment several days before; denies prescription drugs or supplements (the ED's home list still copies trazodone 50 mg at bedtime, R1); no alcohol; vitals normal; several bruises on the upper extremities, gum oozing, no petechiae, no hepatosplenomegaly. Serum tox screen negative for opioids, benzodiazepines and acetaminophen; platelets 180,000/µL, bleeding time 5 minutes, PT 24 seconds, PTT 61 seconds. **Rotenone poisoning** diagnosed (R3). Vitamin K1 10 mg IV in the ED, overnight observation, gum oozing stopped, discharged on **vitamin K1 5 mg by mouth daily** with an INR recheck advised in a few days (A5). ED psychiatric evaluation: depressed mood, denies suicidal intent or self-harm, exposure while baiting rats; outpatient behavioral health follow-up advised (A6). The ED record reaches the practice late (see Records) |
| 2026-05-20 | practice | pharmacy fill notice: vitamin K1 5 mg tablets filled 2026-05-13 by an ED prescriber; the practice learns of the ED visit this way; RN calls, leaves a message; front desk requests the ED records |
| 2026-05-27 | practice | phone: patient returns the call; ED visit for bleeding gums, says it has stopped; told to book a visit; says he will call back |
| 2026-08-11 | practice | ED record for 2026-05-12 received after a second request; filed; lists not changed |
| 2026-08-13 | practice | portal: patient asks for an appointment; visit booked |
| 2026-09-08 | practice | booked follow-up visit (after today) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Trazodone is on both source medication lists, but the second ED visit says he
  takes no prescription drugs, and an ED told him to stop it for priapism. Resolution:
  he holds trazodone from the 2025-09-02 ED visit and the practice stops it on
  2025-10-02. The 2026 ED's home list still copies trazodone, as outside lists do; the
  practice list never shows it after 2025-10-02.
- **R2.** The source calls the priapism non-ischemic (arterial) yet painful and in a man
  on trazodone, which usually means ischemic priapism. Resolution: the arterial label
  stands, supported by the corporal gas (A4); the tenderness stays as the ED examiner
  wrote it. No intracavernosal drug and no shunt; ice and observation. No urology
  letter ever arrives.
- **R3.** The source labels the second event rotenone poisoning, but the picture
  (bleeding, PT 24 s, PTT 61 s, normal platelets after a rat poison) is that of an
  anticoagulant rodenticide; rotenone does not cause a coagulopathy. Resolution: the ED
  record carries the source's diagnosis label (ICD-10 T60.2X1A) and treats the
  coagulopathy with vitamin K as any ED would (A5). No note in the chart relabels it or
  comments on the mismatch. Physician to confirm.
- **R4.** The source places major depressive disorder, insomnia and medication
  non-adherence on every list from the start. They are brought at joining (2022), with
  onsets before.
- The source's "PAST MEDICAL HISTORY" dumps and its encounter-1 "eight months after his
  prior ED visit" interval follow the re-dated timeline (about eight months, 2025-09 to
  2026-05). Its "vital signs within normal limits" means vitals measured, all normal.
- **Left as the source has it:** after the second ED visit no one at the practice
  rechecks the INR or adjusts vitamin K: the patient is not seen again before today,
  and the ED record arrives late. The urology referral after the priapism is never
  completed. No antidepressant is in use from 2023 on. None of this is remarked on.

## Additions (A): physician to confirm

- **A1.** New-patient visit 2022-08-23 (the source's first encounter treats him as an
  established patient with a long-standing problem list).
- **A2.** Sertraline 50 mg daily, started 2022-08-23, stopped 2022-09-27, restarted
  2023-02-07, stopped 2023-04-25: standard treatment for the source's recurrent major
  depression, and the concrete form of its "medication non-adherence".
- **A3.** Psychiatry referral 2022-09-27 and one intake letter (2023-01-24 visit),
  findings as in the timeline. He goes to therapy there irregularly afterward; no
  further letters.
- **A4.** Corporal aspiration and blood gas at the 2025-09-02 ED visit (pH 7.40, pO2
  96 mmHg, pCO2 39 mmHg) to support the source's arterial priapism.
- **A5.** Vitamin K1 10 mg IV in the ED, overnight observation, discharge on vitamin
  K1 5 mg by mouth daily with an INR recheck advised: standard treatment for a
  bleeding coagulopathy after a rodenticide. No repeat PT value is given.
- **A6.** ED psychiatric evaluation at the 2026-05-12 visit (depression, possible
  ingestion reported by the roommate): denies suicidal intent; outpatient behavioral
  health advised.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Depression and sleep visits every few months at first, then spaced out as he engages
  less; PHQ-9 at each, GAD-7 now and then, values as in the timeline; vitals normal.
- Preventive visits 2023 and 2024 for a man in his late twenties: blood pressure,
  weight, depression screen, vaccines. Baseline labs 2022 and a CBC and CMP in 2024,
  normal. HIV and hepatitis C screening once (2022), negative.
- Influenza vaccine each fall 2022 to 2025; COVID vaccines 2022, 2023, 2024; Tdap 2023.
- Trazodone refills through the pharmacy until 2025, often late; pharmacy traffic;
  portal messages about results and scheduling; a missed visit (no-show) in late 2022
  and one in mid-2024.
- After 2025-10-02: only messages, phone calls, the pharmacy fill notice and the late
  ED record, filed without updating the practice's lists.
- Booked visit after 2026-08-27: follow-up of the ED visit, mood and sleep.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Records

The chart has gaps. The 2025 ED record arrives within a week. The 2026 ED visit is
first known to the practice from a pharmacy fill and the patient's call; its record
arrives three months later after a second request. The psychiatry intake sends one
letter and nothing after. Urology never sends anything (he never went).

## Last seen

The practice last saw MRN00002656 on 2025-10-02. Practice contacts after that day are
only calls, messages, the pharmacy notice and records; outside care after it still
happens, and its records are filed as received without changing the practice's lists.
A visit is booked after today.
