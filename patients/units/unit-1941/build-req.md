# Build requirements: unit-1941 (Angela Duffy, MRN00001941)

Source: synthetic_hospital patient 1941, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its "is shown below" image text.

## Who she is (S)

Born 1991-05-06, female, White, not Hispanic or Latino, English-speaking, Blue Cross
Blue Shield. High school science teacher. Never smoker, no alcohol, exercises
regularly. No known drug allergies. No surgical history. Family history: mother with
hypertension diagnosed at age 32; father with an ischemic stroke at age 48; a paternal
uncle with renal artery stenosis.

Joined the practice in 2021 (A1): first practice visit 2021-08-10, a new-patient
preventive visit. She brought no problems and no medications. Height 165 cm, weight
about 62.6 kg (BMI 23), steady throughout. Blood pressure normal at every contact
before December 2024.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2021-08-10 | practice | new-patient preventive visit (A1); history as above; normal exam; normal BP |
| 2024-12-23 | practice | several weeks of headaches and fatigue; no chest pain, dyspnea or palpitations; no medications; NKDA; BP 168/100 in both arms; BMI 23; right-sided abdominal bruit near the umbilicus; fundi normal; renal function and electrolytes normal. **Hypertension diagnosed** (recorded as Essential hypertension, secondary cause suspected; R2). Renal artery CT angiography ordered (R1). Lifestyle (sodium, exercise) and home BP log; no drug started (R4) |
| 2025-01-08 | outside imaging | CT angiography, abdomen and pelvis, renal artery protocol (A2): beaded appearance of the mid and distal main renal arteries on both sides, consistent with multifocal fibromuscular dysplasia; kidneys normal in size, no infarct, no aneurysm, no dissection |
| 2025-01-09 | practice (phone) | result given; **fibromuscular dysplasia diagnosed** (R1); referral to cardiology placed (A3) |
| 2025-03-10 | outside imaging | CT angiography, head and neck (ordered by cardiology after the referral): alternating areas of narrowing and dilation of the right internal carotid artery; no intracranial aneurysm; no dissection (R5) |
| 2025-03-24 | outside | cardiology consult letter: a month of persistent headaches and a pulsatile whooshing sound in the right ear; occasional neck stiffness; BP found high at a pharmacy; no asthma, sinus symptoms, hematuria, skin findings, fever, rash, neuropathy or hemoptysis; BP 160/100 in both arms; bruit over the right carotid artery; normal neurologic exam. Labs: plasma renin activity high at 8.6 ng/mL/h and aldosterone high at 26 ng/dL (A4); hepatitis B surface antigen and hepatitis C antibody negative; ANA negative; ANCA indeterminate (R6). Head and neck CTA as above; the January renal CTA reviewed, same pattern in the renal vessels. **Renal artery stenosis** and **secondary (renovascular) hypertension due to renal artery stenosis** diagnosed; the practice relabels the hypertension (R2). Plan in the letter: sodium restriction, exercise, home BP log, BP care through the PCP, cardiology follow-up (R4) |
| 2025-09-08 | practice | blood pressure still high despite lifestyle changes; occasional headaches and fatigue; no medications; BP 160/100 in both arms; high-pitched bruit just right of midline; urine pregnancy test negative. **Amlodipine 5 mg daily started** (R3) |
| 2025-11-10 | practice | BP follow-up on amlodipine 5 mg: BP 154/98; tolerating it. **Amlodipine stopped at 5 mg and started at 10 mg daily** (A5); basic metabolic panel normal |
| 2026-03-12 | practice (last seen) | preventive visit with BP follow-up: taking amlodipine 10 mg daily; BP 162/98; referred back to cardiology (A6); CBC, CMP and lipids normal |
| 2026-06-10 | outside | cardiology follow-up letter: persistent headaches; BP control worse despite taking amlodipine; home systolic pressures 160 to 170; occasional lightheadedness, otherwise negative review; clinic BP 164/98 (A7); no focal neurologic deficits; bruits over both flanks; creatinine 0.9 mg/dL; plasma renin activity elevated at 9.1 ng/mL/h (A4). Plan: renal angiography with balloon angioplasty (A8) |
| 2026-07-28 | outside | cardiology catheter lab procedure report (A8): renal angiography confirms beading of both main renal arteries; right renal artery with a significant translesional pressure gradient, treated with balloon angioplasty without a stent, gradient resolved; left renal artery beaded without a significant gradient, not treated; right femoral access, no complications; home the same day. **Aspirin 81 mg daily started** by cardiology; amlodipine 10 mg continued; renal artery duplex planned in about a month at cardiology |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source stamps fibromuscular dysplasia as diagnosed at the first internal
  medicine visit, whose imaging section is an image placeholder, and an office visit does
  not do angiography. Resolution: the 2024-12-23 visit orders a renal artery CT
  angiogram; it is done 2025-01-08 and fibromuscular dysplasia is diagnosed on the
  practice's result call 2025-01-09. The angiogram the source "shows" is this study, and
  it is the renal imaging the cardiology letter reviews.
- **R2.** The source's comorbidity "secondary hypertension due to renal artery stenosis"
  cannot be known on the first day. Resolution: hypertension is recorded on 2024-12-23 as
  Essential hypertension (the only hypertension code the practice has), with a note that
  a secondary cause is suspected. When the March 2025 cardiology letter is filed, the
  practice resolves that entry and records renovascular hypertension (secondary
  hypertension due to renal artery stenosis) and renal artery stenosis, as the source
  stamps renal artery stenosis at that encounter.
- **R3.** The source has her on no medication in September 2025 and on amlodipine in June
  2026, but never starts it. Resolution: amlodipine 5 mg daily starts at the 2025-09-08
  visit, after the negative urine pregnancy test the source gives that day. No note
  explains the choice of amlodipine over another agent.
- **R4.** BP of 168/100 and 160/100 goes untreated for about nine months while the
  source describes lifestyle measures. Resolution: the December 2024 and March 2025 plans
  are lifestyle and home monitoring; no drug is raised. See "Left as the source has it".
- **R5.** The source says "the internal carotid artery" without a side. Resolution: the
  right one, matching the right carotid bruit and the right-ear whooshing. The study is
  done two weeks before the consult, ordered by cardiology on the referral.
- **R6.** "ANCA results are unclear." Resolution: the ANCA is reported as indeterminate;
  the letter does not pursue it, given imaging typical of fibromuscular dysplasia.
- **R7.** Years in source text follow the re-dated timeline: the "January 2020
  evaluation" is the December 2024 practice visit; "presented in 2020" is late 2024 into
  2025; "diagnosed with hypertension 2 years ago" becomes "since late 2024" (about 18
  months).
- **R8.** The June 2026 cardiology note says no family history of cardiovascular disease,
  against the profile's father with a stroke at 48, mother with hypertension at 32 and
  uncle with renal artery stenosis. Resolution: the profile's family history stands; the
  June letter does not repeat the negative line.
- **Interobserver differences stay as written:** a right abdominal bruit near the
  umbilicus (December 2024), a high-pitched bruit just right of midline (September 2025),
  bruits over both flanks (June 2026).
- **Left as the source has it:** no antihypertensive from December 2024 to September
  2025 despite pressures of 160 to 168 over 100; no ACE inhibitor or ARB at any point;
  no antiplatelet before the angioplasty; the indeterminate ANCA is never repeated. No
  note comments on any of these.

## Additions (A): physician to confirm

- **A1.** 2021-08-10 new-patient visit (the seed gives about 5 years with the practice,
  and the source's first encounter is an internal medicine follow-up of an established
  patient).
- **A2.** The renal artery CT angiogram of 2025-01-08 with its findings above (supports
  the source's fibromuscular dysplasia diagnosis and its "renal artery imaging").
- **A3.** The practice's cardiology referral on 2025-01-09 (the source sends her to
  cardiology next).
- **A4.** Values for the source's "high renin and aldosterone" (renin activity 8.6 ng/mL/h,
  aldosterone 26 ng/dL, March 2025) and "elevated plasma renin activity" (9.1 ng/mL/h,
  June 2026).
- **A5.** The 2025-11-10 BP visit: BP 154/98 on amlodipine 5 mg; dose raised to 10 mg
  daily (the source says control is worse despite adherence by June 2026).
- **A6.** The 2026-03-12 visit that sends her back to cardiology: BP 162/98 on amlodipine
  10 mg daily.
- **A7.** Clinic BP 164/98 in the June 2026 cardiology letter.
- **A8.** Next step of the source's renal artery stenosis with uncontrolled pressure:
  renal angiography and right renal artery balloon angioplasty without stent on
  2026-07-28 at a regional hospital, as above, with aspirin 81 mg daily started by
  cardiology afterwards. The practice files the report as received and does not change
  its lists (after last seen). The duplex is still ahead at today.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention
  (kidney size, no hydronephrosis, aorta caliber, heart sounds). An abnormal finding appears
  only as the source or this file gives it. A report may carry the descriptors its kind
  always carries (how a size was measured, a procedure's access site), but nothing that
  adds a territory, a severity grade or a symptom, or changes the diagnosis or plan.
- Records from a prior practice may mention routine screening done there, with normal
  results (a normal Pap in 2019; Tdap in 2017; COVID primary series in spring 2021).
- Yearly preventive visits (2022-08, 2023-08; the 2024 one cancelled; 2026-03 combined
  with the BP visit), with CBC, metabolic panel and lipids at some of them, all normal.
- At the 2024-12-23 visit, the usual first workup for new hypertension with normal
  results: urinalysis, ECG, glucose, lipids, TSH.
- Blood pressure follow-up with a metabolic panel on amlodipine.
- Prevention: Pap with HPV co-test in 2022, normal; hepatitis C and HIV screening once
  (2021), negative; influenza vaccine each fall; COVID vaccines as offered; PHQ-2 or PHQ-9
  yearly, negative.
- Refills, pharmacy and prior-authorization traffic (the CT angiograms), portal messages
  about results, refills and scheduling, the cancelled 2024 physical, school-form and
  records requests.
- No visit is booked after 2026-08-27.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00001941 on 2026-03-12. Practice contacts after that day are
only messages, refills and outside records filed as received; the June 2026 cardiology
letter and the July 2026 procedure report do not change the practice's problem or
medication lists. Nothing is booked after today.
