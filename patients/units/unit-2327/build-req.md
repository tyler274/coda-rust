# Build requirements: unit-2327 (Raymond Becker, MRN00002327)

Source: synthetic_hospital patient 2327, in `source.md` beside this file. This file and
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
"(diagnosed ...)" stamps, or its "an ECG is shown" quiz text.

## Who he is (S)

Born 1953-05-05, male, White, not Hispanic or Latino, Medicare Part A and B, lives in
Amherst. Retired high school mathematics teacher. Former smoker, 30 pack-years, quit in
2016 (R1). Alcohol: occasional, 1 to 2 glasses of wine on weekends. Allergies:
penicillin (hives); shellfish (mild urticaria). Family history: father MI at 72; mother
hypertension and type 2 diabetes; older brother coronary artery disease. Surgical
history (R2): inguinal hernia repair 2000; PCI with a drug-eluting stent to the right
coronary artery in early 2022; left cataract extraction with lens implant in early 2023.

Joined the practice 2023-08-22 (A1: a new-patient visit, transferring from a prior
primary-care office). Brought: essential hypertension, type 2 diabetes without
complications, hyperlipidemia, coronary artery disease with a coronary stent, primary
osteoarthritis of both knees, history of tobacco dependence. Drugs on joining:
metformin 1000 mg twice daily, lisinopril 20 mg daily, atorvastatin 40 mg daily,
aspirin 81 mg daily, acetaminophen 500 mg as needed for knee pain. He has an outside
cardiology group that has followed him since the 2022 stent.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2023-08-22 | practice | new-patient visit (A1): history as above; height 178 cm, weight about 88 kg; BP at goal on lisinopril; pulse in the 60s; labs drawn (P); prior records requested |
| 2023-09-06 | outside records | prior primary-care records received: the conditions and drugs above, the 2022 stent, the 2023 cataract surgery, a normal screening colonoscopy in 2019, Tdap 2018 (P) |
| 2024-09-23 | practice | telephone (A2): several days of lightheadedness and feeling near-faint on standing quickly or walking, no chest pain, no fall; practice asks his cardiology group to see him this week |
| 2024-09-25 | outside | cardiology visit (S): about a week of dizziness and near-syncope when standing up quickly or walking; no chest pain. BP 130/70, pulse 50. 12-lead ECG: sinus rhythm with progressive PR prolongation and a dropped QRS, grouped beating (R3). **Second-degree AV block, type I, diagnosed.** No AV-nodal blocking drug to stop (he takes none). Plan in the letter: 14-day ambulatory ECG monitor (A3), rise slowly, call for syncope, cardiology review after the monitor. Letter received by the practice 2024-09-27 |
| 2024-10-18 | outside | cardiology monitor result letter (A3): worn 2024-09-27 to 2024-10-11. Sinus rhythm, average heart rate 58, minimum 41 asleep; intermittent type I second-degree AV block, more at night; longest pause 2.4 seconds; no type II block, no complete heart block, no atrial fibrillation. Two dizzy entries coincided with type I block episodes. Lightheadedness easing with slow position changes. No pacemaker indicated; observe; yearly cardiology review |
| 2026-04-03 | outside | ED (S): age 72; sudden substernal chest pain radiating to the left arm and jaw, diaphoretic, mildly nauseated. BP 92/60, heart rate 42, SpO2 97% on room air. ECG: ST elevation in II, III and aVF. Monitor: regular rhythm with grouped beating, progressive PR prolongation and dropped QRS complexes. **Inferior ST-elevation MI** (R4). ED care (A4): aspirin 324 mg chewed, clopidogrel 600 mg load, heparin, atropine 0.5 mg IV once (heart rate to 58, BP 104/66), cath lab activated. High-sensitivity troponin I 62 ng/L on arrival |
| 2026-04-03 to 2026-04-06 | outside | admission at the same regional hospital (A4): emergent coronary angiography: acute thrombotic occlusion of the proximal right coronary artery above the 2022 stent; one drug-eluting stent placed, TIMI 3 flow after; left anterior descending and circumflex with mild nonobstructive disease. Right radial access. Peak high-sensitivity troponin I 21,350 ng/L. Echocardiogram 2026-04-04: LVEF 50%, inferior wall hypokinesis, no significant valve disease, no effusion. After reperfusion the rhythm returns to his baseline (sinus with intermittent type I block, rate 52 to 64); no temporary pacing. Lisinopril held on day 1 for low pressure and resumed 2026-04-05; metformin held 48 hours after contrast and resumed. **Discharge drugs: clopidogrel 75 mg daily started (12 months), atorvastatin raised from 40 to 80 mg daily, nitroglycerin 0.4 mg sublingual as needed started**; aspirin 81, lisinopril 20, metformin 1000 twice daily and acetaminophen continued. No beta-blocker because of the AV block and slow rate. Cardiac rehabilitation referral. Discharge summary received by the practice 2026-04-07 |
| 2026-04-07 | practice | RN post-discharge call (P): no chest pain, has all new drugs, rehab intake pending |
| 2026-04-14 | practice | post-discharge visit (P): no chest pain, mild fatigue; drugs reconciled with the discharge list; BP and HR measured; BMP drawn |
| 2026-05-13 | outside | cardiology follow-up letter (A5): no angina, no dizziness; cardiac rehab started 2026-04-28; BP 122/72, HR 56; ECG sinus with type I second-degree block, inferior Q waves; continue current drugs; recheck lipids at primary care |
| 2026-06-16 | practice | last practice visit (P): diabetes and post-MI follow-up; well; labs: LDL 58 on atorvastatin 80 (A6), A1c 7.0 |
| 2026-08-12 | outside records | cardiac rehab progress note (A5): 20 of 36 sessions attended, no symptoms during exercise; filed as received |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source says he "quit 8 years ago" at both encounters, 18 months apart.
  Resolution: quit in 2016 (eight years before the 2024 cardiology visit); later notes
  say about 10 years.
- **R2.** The source's surgical years sit on its own timeline (PCI 2018, cataract
  2021 listed at a 2020 visit, hernia 1995). Resolution: they move with the re-dated
  timeline: hernia repair 2000, PCI with DES to the RCA early 2022, and the left
  cataract surgery, which must precede the first source encounter, in early 2023.
  All three reach the chart as history from prior records and the patient.
- **R3.** "An ECG is shown." Resolution: the cardiology ECG shows what the source's
  diagnosis and later monitor describe: progressive PR prolongation, a dropped beat,
  grouped beating; rate about 50. No quiz text is carried.
- **R4.** The source gives inferior ST elevation and type I block in the ED with no
  diagnosis or course. Resolution: inferior STEMI (right coronary artery), treated as
  in A4. Coronary artery disease and the stent stay on the list; the MI is added, and at the June 2026 visit the acute STEMI entry is resolved and recorded as old myocardial infarction (the same event).
- The source's ED note places the AV block "in January 2020"; on this timeline it
  was documented in September 2024.
- **Left as the source has it:** no beta-blocker after the MI (cardiology withholds it
  for the block); no other change to the diabetes regimen.

## Additions (A): physician to confirm

- **A1.** The 2023 new-patient visit (he joined three years before today; the first
  source encounter is a specialist visit).
- **A2.** The telephone call that sent him to cardiology.
- **A3.** The ambulatory monitor and its result letter, the usual next step for a new
  type I block with near-syncope, with the findings given above.
- **A4.** The STEMI treatment course: ED drugs, angiography and stent, troponins,
  echocardiogram, discharge drugs, cardiac rehab referral, as given above.
- **A5.** Cardiology follow-up after the MI and the cardiac rehab progress note.
- **A7.** Approximate onsets for the conditions he brought, from prior records: hypertension and hyperlipidemia about 2010, type 2 diabetes about 2013 (metformin about ten years by joining), knee osteoarthritis about 2017, coronary disease 2022.
- **A6.** LDL 58 on atorvastatin 80 in June 2026. Before the MI, LDL runs 72 to 86 on
  atorvastatin 40.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may carry
  the descriptors its kind always carries, but nothing that adds a territory, a severity
  grade or a symptom, or changes the diagnosis or plan.
- Records from a prior practice may mention routine screening done there, with normal
  results (screening colonoscopy 2019, normal; no repeat needed).
- Diabetes and blood pressure visits about every 4 to 6 months; yearly annual wellness
  visits. A1c runs 6.8 to 7.3 on metformin; BP about 118 to 138 over 68 to 88 on
  lisinopril 20; weight about 87 to 89 kg; creatinine, potassium, eGFR, urine
  albumin/creatinine, CBC, TSH and liver tests normal. Pulse in the 60s before September
  2024, about 45 to 60 after (the block). Yearly foot exams normal.
- Yearly diabetic eye exam letters from an outside ophthalmology office: no diabetic
  retinopathy; left lens implant in good position.
- Lung cancer screening (30 pack-years, quit 2016): low-dose CT chest yearly in October
  2023, 2024 and 2025, Lung-RADS 1; each may note coronary artery calcification and the
  right coronary stent.
- One-time abdominal aortic aneurysm ultrasound (male former smoker 65 to 75), 2023:
  normal caliber aorta, 2.0 cm.
- Prevention: high-dose influenza each fall, updated COVID vaccines, PCV20 once, Shingrix
  series 2024, RSV vaccine once (2024), Tdap 2018 from prior records. Hepatitis C
  antibody once, negative. PHQ-2 each wellness visit, 0 or 1.
- Refills, pharmacy and prior-authorization traffic, portal messages about results,
  refills and scheduling, a missed visit, reminders.
- Booked visit after 2026-08-27: a routine diabetes and cardiac follow-up.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002327 on 2026-06-16. Practice contacts after that day are
only messages, refills and records filed as received; no practice visit comes after
it, and the practice's lists stay as they were that day. A visit is booked after today.
