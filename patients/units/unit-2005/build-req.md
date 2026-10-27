# Build requirements: unit-2005 (Isaiah Coleman, MRN00002005)

Source: synthetic_hospital patient 2005, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz text ("You recall a recent study...",
"Based on the information provided,").

## Who he is (S)

Isaiah Coleman, born 1966-01-02, male, Black (not Hispanic), English, Granby MA 01033,
Blue Cross Blue Shield PPO. Postal service supervisor. Former smoker, quit in 2016
(about 5 years before the October 2021 physical; pack-years never recorded). Alcohol:
social drinking; by October 2021 he has started one 5-oz glass of red wine with dinner
nightly, and keeps to that. No known drug allergies. No prior abdominal surgery (until
October 2025). Family history: father MI at 58, mother type 2 diabetes, a brother with
hypertension.

Joined the practice 2018-08-21 (A1, new-patient visit), age 52, from a prior practice.
Brought: essential hypertension, type 2 diabetes, mixed hyperlipidemia, recurrent
diverticulitis (R4) and hyperzincemia (R5); taking lisinopril 20 mg daily, atorvastatin
40 mg daily, metformin 1000 mg twice daily.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2018-08-21 | practice | new-patient visit (A1): history as above; prior records requested; labs; zinc level 134 mcg/dL (ref 60-120) (A3) |
| 2021-10-15 | practice | annual physical. He has recently started one glass of red wine nightly after reading that moderate drinking helps the heart, and asks if that is wise. PCP explains the studies are observational and confounded (drinkers differ from non-drinkers in other ways), does not recommend alcohol for heart health, and leaves the one glass as his choice (R3). BP 135/85; exam unremarkable. Labs drawn 2021-10-08: TC 195, HDL 40, LDL 120, TG 175, fasting glucose 98. Meds unchanged: lisinopril 20, atorvastatin 40, metformin 1000 BID (R1) |
| 2022-04-22 | practice | follow-up for lipids and diabetes after the October physical; no complaints; has read online about statins hurting the liver and causing diabetes and is thinking of stopping atorvastatin. Counseled on benefit vs risk; LFTs normal; he agrees to continue. BP 132/84; exam unremarkable. Labs drawn 2022-04-19: TC 190, LDL 100, HDL 45, TG 150, fasting glucose 98, hepatic panel normal |
| 2023-02-14 | practice | diabetes/BP follow-up (A2); asks what more he can do about heart risk given his father's MI; **referred to cardiology** for risk assessment |
| 2023-04-21 | outside | cardiology consult (letter received 2023-06-20 after a second request): returns from the practice's diet and statin counseling worried about heart disease; no chest pain, dyspnea or visual change; exam unremarkable. Says he takes metformin and lisinopril faithfully; **had stopped atorvastatin on his own about three months earlier** (R2). Labs: A1c 7.2%, fasting glucose 154, TC 220, HDL 35, LDL 145, TG 200, creatinine 1.0, urine albumin/creatinine 250 mg/g. Plan: **restart atorvastatin 40 mg daily**, continue the rest, diet and activity, cardiology follow-up in a year (R2) |
| 2024-04-15 | outside | cardiology follow-up (letter received 2024-04-23): no symptoms, adherent to lisinopril, atorvastatin, metformin. Asks about a trial he heard of comparing a new lipid-lowering drug with placebo for heart attack prevention in high-risk patients. Cardiologist explains randomization, placebo on top of his usual statin, and the visit burden; gives trial contact information; the decision is his (R6). BP 132/84, HR 78; exam unremarkable. Labs: TC 190, HDL 45, LDL 100, TG 225, fasting glucose 98 |
| 2024-09-06 | practice | annual physical, the last practice visit (P). Has not enrolled in the trial. BP and exam as usual; fasting labs drawn |
| 2025-10-11 | outside | regional hospital ED (records received 2025-12-04 after two requests): age 59; severe abdominal pain began about 4 hours after a meal, sharp and intense at first, eased briefly, then diffuse and constant; nausea and low-grade fever. T 38.2 C, HR 102, BP 135/85. Generalized tenderness, rebound, guarding, diminished bowel sounds. Leukocytosis with left shift (A4: WBC 16.4, neutrophils 88%). Upright chest x-ray: free air under the diaphragm. **Perforated diverticulitis with peritonitis** |
| 2025-10-11 to 2025-10-17 | outside | admission (A4): emergency laparotomy the night of arrival, purulent peritonitis from a perforated sigmoid diverticulum, **sigmoid colectomy with primary anastomosis**, no stoma. Pathology: diverticulitis with perforation, no dysplasia or malignancy. IV piperacillin-tazobactam for 4 days after surgery; metformin and lisinopril held, resumed at discharge; discharged day 6 on acetaminophen and a short course of oxycodone 5 mg as needed; surgical clinic follow-up in 2 weeks |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Encounter 0's prose says he takes hydrochlorothiazide for blood pressure; its
  own medication list and every later note name lisinopril only. Resolution:
  hydrochlorothiazide is dropped; lisinopril 20 mg daily is his only BP drug throughout.
- **R2.** The 2023 cardiology note lists atorvastatin 40 mg and calls him compliant,
  yet LDL 145 and TC 220 jump from 100 and 190 on the same dose, and its prose names
  only metformin and lisinopril as what he takes faithfully. Resolution: after the April
  2022 counseling he kept taking it through the October 2022 physical, then stopped it
  himself around January 2023 without telling the practice. The practice list keeps
  atorvastatin (the list lags; refills simply stop for a few months). Cardiology finds
  it, he restarts, and the 2024 numbers (LDL 100) are back on it.
- **R3.** Encounter 0's plan is quiz text about confounding. Resolution: the PCP's
  counseling as in the timeline. No quiz in the chart.
- **R4.** "Recurrent diverticulitis" is listed from the start with no episode in the
  source before 2025. Resolution: two outpatient-treated episodes before he joined,
  the last in 2017, known from the patient and prior records (no dates or numbers
  beyond that); none during 2018-2024.
- **R5.** Hyperzincemia appears on every list from 2021 but is stamped "diagnosed"
  at the 2024 cardiology visit. Resolution: known from prior records at joining; the
  one practice zinc level (A3) confirms it; no treatment and no further workup.
- **R6.** Encounter 3's plan stops mid-sentence. Resolution: the cardiologist's
  counseling as in the timeline. No enrollment is recorded.
- **R7.** "African American Ethnicity" is a primary diagnosis in the source. Resolution:
  it is demographics (race), not a problem-list entry.
- **R8.** Years in source text follow the re-dated timeline: the "January 2020" exam is
  the October 2021 physical, the "2021" cardiology consult is April 2023, and the ED
  note's "care since 2020" becomes primary care since 2018. "Mixed hyperlipidemia
  (diagnosed 2020-07-15)" is dropped: it is known at joining, on atorvastatin.
- **R9.** The 2024 cardiology note's family history names only his father's coronary
  disease; the outside letter may say so; the practice's family history keeps all three.
- **Left as the source has it:** the urine albumin/creatinine of 250 mg/g appears only in
  the 2023 cardiology letter. Nobody repeats it, adds an SGLT2 inhibitor or changes the
  ACE inhibitor, and no practice note comments on it; the practice never orders a urine
  albumin. Triglycerides of 200 to 225 get no added drug. No aspirin is raised. Nobody
  remarks on these.

## Additions (A): physician to confirm

- **A1.** 2018-08-21 new-patient visit (the October 2021 physical is a routine annual
  visit, and the seed gives 8 years with the practice).
- **A2.** The 2023-02-14 practice visit that sends him to cardiology.
- **A3.** One zinc level (blood) at joining, 134 mcg/dL (ref 60-120), to support the listed
  hyperzincemia.
- **A4.** The course after the source ends: ED labs (WBC 16.4, neutrophils 88%), the
  admission, emergency sigmoid colectomy with primary anastomosis, pathology without
  malignancy, 4 days of piperacillin-tazobactam, short oxycodone course, as in the
  timeline. The surgical follow-up letter never reaches the practice.
- **A5.** Prior-practice colonoscopy in 2017 after the second diverticulitis episode:
  sigmoid diverticulosis, no polyps, repeat in 10 years (supports R4; screening not due
  again before today).

## Records (chart_texture: gaps)

- Prior-practice records arrive about two months after joining.
- The 2023 cardiology letter arrives only after a second request; the 2024 letter
  arrives promptly.
- Optometry letters arrive for 2019, 2021 and 2023 only (no retinopathy); other years
  nothing comes.
- The October 2025 ED visit and admission: the practice first hears of it from his
  portal message on 2025-10-20; records arrive 2025-12-04 after two requests. They are
  filed as received; the practice's problem and medication lists stay as they were on
  2024-09-06 (oxycodone and the surgery appear only in the records).
- After 2024-09-06 only messages, refills, records requests, recall reminders and the
  received records. He does not schedule the post-hospital or diabetes visit asked of him.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may carry
  the descriptors its kind always carries, but nothing that adds a territory, a severity
  grade or a symptom, or changes the diagnosis or plan.
- Records from a prior practice may mention routine screening done there, with normal results.
- Height 178 cm, weight about 92 to 94 kg throughout 2018-2024. Office BP about 130-136
  over 82-86 on lisinopril. A1c about 6.8 to 7.0 through 2022, about 7.2 in 2023, about
  7.0 in 2024; fasting glucose about 95 to 110 at practice draws; creatinine about 1.0;
  LDL about 100 to 120 on atorvastatin, TG 150 to 225, HDL 40 to 45; liver tests normal.
- Diabetes and BP visits roughly every 6 months, often less; yearly physicals; A1c,
  CMP or BMP, lipids, CBC; foot exams normal; PHQ-2 negative.
- Prevention: Tdap 2018; influenza each fall at the practice through 2024; COVID mRNA
  primary series spring 2021 at a pharmacy, boosters fall 2022 and 2023; Shingrix two
  doses 2019-2020; PCV20 in 2022 (diabetes); hepatitis C and HIV screening once at
  joining, negative; PSA after shared decision-making, normal.
- Refills (90-day), pharmacy fills, portal messages about results, refills and
  scheduling, one missed visit (2024-03-12 no-show), recall reminders.
- No visit is booked after today.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002005 on 2024-09-06. Practice contacts after that day do not
happen; outside care after it still does, and its records are filed as received without
changing the practice's lists. No visit is booked after today.
