# Build requirements: unit-2723 (Patricia Desrosiers, MRN00002723)

Source: synthetic_hospital patient 2723, in `source.md` beside this file. This file and
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
"(diagnosed 2020-03-15)" stamps, or its quiz text (trial power, p-values, alpha,
type I and type II errors, sample sizes).

## Who she is (S)

Born 1966-10-26, female, White, not Hispanic or Latino, English-speaking, commercial
PPO, lives in Florence (01062). Office administrator. Never smoker. Alcohol: occasional
social use, 1 to 2 drinks a month. No surgical history. Family history: father with
hypertension and type 2 diabetes; mother with hyperlipidemia and a stroke at 68; a
maternal aunt with bipolar disorder; a brother with premature coronary artery disease
(MI at 52).

Allergy on joining: shellfish (reaction not recorded; R3). ACE-inhibitor intolerance is
recorded only in May 2023 (R2).

Joined the practice 2021-09-14 (new patient visit, A1), coming from another practice.
Brought on joining:

- Essential hypertension, on metoprolol succinate 50 mg daily, amlodipine 10 mg daily,
  hydrochlorothiazide 25 mg daily.
- Type 2 diabetes mellitus without complications, on no drug (diet; R4).
- Hyperlipidemia, on atorvastatin 40 mg daily; aspirin 81 mg daily.
- Bipolar I disorder, on lithium carbonate 300 mg twice daily, managed by an outside
  psychiatrist (P).
- Major depressive disorder, on sertraline 100 mg daily (same psychiatrist). Both mood
  diagnoses stay on the list as the prior practice had them (Left as the source has it).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2021-09-14 | practice | new patient visit (A1): history as above; BP at goal on three drugs; baseline labs (P) |
| 2023-02-15 | practice | asks about joining a cardiovascular risk-reduction research study (a trial of a new lipid-lowering drug against standard statin); the physician explains what randomization would mean for her atorvastatin and gives the study contact (R6). BP above goal on three drugs, about 146/90 (R1). **Lisinopril 10 mg daily started** (R1) |
| 2023-05-16 | practice | (moved, R1) dry cough for about 2 months, began after lisinopril was started 3 months ago; non-productive, no dyspnea, wheeze or chest pain; BP 132/84, HR 78, RR 16; lungs without wheezes, rales or rhonchi, rest of exam unremarkable; CBC normal, electrolytes normal, creatinine 1.0; chest x-ray no acute cardiopulmonary process. ACE-inhibitor cough: **lisinopril stopped**, **ACE-inhibitor intolerance (cough) recorded** (R2), adverse effect of ACE inhibitor on the problem list. No replacement drug (Left as the source has it) |
| 2023-07-25 | practice | (A2) BP follow-up: cough gone; home and office pressures up since lisinopril stopped, office about 156/96 on metoprolol, amlodipine and hydrochlorothiazide, taken as directed; referred to nephrology for hard-to-control hypertension |
| 2023-10-11 | outside | nephrology consult letter: about five months of hard-to-control hypertension since lisinopril stopped (R1), adherent to metoprolol, amlodipine and hydrochlorothiazide; no chest pain, palpitations or headaches; BP 162/98; prominent bruit to the right of the umbilicus; creatinine 1.2 (elevated), BUN 22, potassium 3.4 (low); abdominal ultrasound with Doppler: right kidney smaller than the left. **Renal artery stenosis (right) diagnosed.** Plan: renal vein renin sampling and aldosterone (R5); potassium-rich diet (P) |
| 2023-11-08 | outside | (R5) renal vein renin sampling by interventional radiology: renin activity high from the right kidney, low (suppressed) from the left; serum aldosterone elevated. Report and a short nephrology follow-up letter (2023-11-21) received: lateralizing to the right; medical management, no drug change |
| 2024-02-20 | practice | (A3) routine follow-up; still wants to pursue the lipid study; referred to the outside cardiology group that runs it |
| 2024-04-12 | outside | cardiology research-clinic letter: seen about enrolling in the randomized trial of a new lipid-lowering drug against standard statin therapy; family history of her brother's MI at 52 reviewed; exam unremarkable; continue atorvastatin 40 mg and aspirin. She decided not to enroll (R6) |
| 2024-10-18 | practice | routine follow-up; asks about a placebo-controlled trial of a new blood-pressure drug she saw advertised; the physician explains placebo control and her renal artery stenosis; trial information given, no enrollment (R6) |
| 2025-04-24 | outside | ED record (age 58): two days of nausea, vomiting, diarrhea and hand tremor, increasingly confused, unsteady gait from imbalance; says she recently started a new medication whose name she cannot recall (R7); vital signs normal; mild ataxia, fine tremor of both hands; lithium 2.5 mEq/L (therapeutic 0.6 to 1.2), creatinine 1.1, sodium 140, potassium 4.2, calcium 9.6. **Lithium toxicity diagnosed.** Observation stay (A4) |
| 2025-04-30 | practice | post-ED visit (P): symptoms gone; brings bottles, which match the practice list; lithium back on at 300 mg twice daily per psychiatry (A4) |
| 2025-09-17 | outside | (A5) admission for flash pulmonary edema, discharge summary received |
| 2025-10-10 | outside | admission (Internal Medicine), discharge summary received: since the September admission, recurrent episodes of acute breathlessness that wake her at night and ease when she sits up; BP 162/98; bruit over the right upper quadrant (R8); creatinine 1.5 (elevated), BUN 28 (elevated), potassium 4.2; urinalysis without protein or blood; renal artery Doppler ultrasound: significant stenosis of the right renal artery. **Secondary (renovascular) hypertension due to renal artery stenosis diagnosed.** Right renal artery stent (A6). Home medicines continue |
| 2026-06-12 | outside | ICU admission (Critical Care, age 59), discharge summary received: two days of severe, worsening nausea, vomiting, diarrhea and hand tremor, confused and unsteady walking; prior lithium toxicity in April 2025 (re-dated, R9); denies recent illness, travel or diet change; BP 130/85, HR 92, RR 18, temperature 37.1 °C (98.8 °F); alert but distressed; mild ataxia and fine tremor of both hands; sodium 140, potassium 4.2, creatinine 1.1, lithium 2.5 mEq/L, TSH 2.5. **Severe lithium toxicity diagnosed.** ICU course and discharge plan (A7): **hydrochlorothiazide stopped** |
| 2026-06-19 | outside | (A7) psychiatry letter (seen 2026-06-18): lithium restarted at 300 mg twice daily, level in a week |
| 2026-06-22 | practice | (A7) post-discharge visit: tremor and gait back to normal; BP higher off hydrochlorothiazide (about 140/88); no new BP drug yet, recheck planned |
| 2026-06-29 | practice lab | (A7) lithium 0.7, creatinine 1.0 |
| 2026-09-22 | practice | booked: blood pressure and lithium follow-up |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Lisinopril. The source's cough visit says the "new blood pressure medication"
  was started at her last visit 3 months earlier and the cough has lasted 2 months, and
  the problem is "lisinopril adverse effect". The first source visit (2023-02-15) is that
  last visit. Resolution: lisinopril 10 mg daily (dose not in the source) starts on
  2023-02-15 because BP was above goal on three drugs (about 146/90), and the cough visit
  moves from 2023-04-11 to 2023-05-16, three months later. The nephrology letter's "six
  months" since lisinopril stopped becomes about five months. BP was otherwise controlled before
  2023 (about 134/84 on three drugs).
- **R2.** The source lists "ACE inhibitors (angioedema/cough)" as an allergy from the
  first encounter, and the cough visit says "no history of allergies". Resolution: the
  intolerance (ACE inhibitors, cough) is recorded on 2023-05-16. Angioedema is not
  carried; no event documents it.
- **R3.** Shellfish allergy is on every source list except the cough visit's "no history
  of allergies". Resolution: shellfish is on the list from joining (patient report, no
  reaction recorded); the cough visit's statement is not carried.
- **R4.** Type 2 diabetes is listed throughout but no diabetes drug is ever named.
  Resolution: diet-controlled diabetes; A1c runs about 6.3 to 7.1 with no drug. No one
  starts metformin.
- **R5.** The nephrology encounter lists "plasma renin activity: high on the right, low
  on the left" with the office labs. Lateralized renin needs renal vein sampling, which
  an office visit does not do. Resolution: the consult (2023-10-11) carries the exam,
  creatinine, BUN, potassium and Doppler; renal vein renin sampling and serum aldosterone
  follow on 2023-11-08, reported qualitatively as the source gives them (no numbers).
- **R6.** Three source visits (2023-02-15, 2024-04-12, 2024-10-18) are research visits
  whose text is statistics quiz. The 2023 note says she is "enrolled" while the 2024
  cardiology note says she is enrolling. Resolution: she is never enrolled in any trial
  and no investigational drug appears in the chart. The chart keeps her interest (asks in
  February 2023; cardiology research-clinic consult April 2024, after which she decides
  not to enroll; asks about a blood-pressure trial in October 2024, information only).
- **R7.** The ED record says she had recently started a new medication she could not
  name. No new prescription exists in the practice record. Resolution: the ED record
  carries her statement as she gave it; at the post-ED visit her bottles match the
  practice list and the physician leaves it there. The medication is never identified.
- **R8.** The bruit is right periumbilical at nephrology (2023) and right upper quadrant
  at the 2025 admission. Interobserver difference; both stay as written.
- **R9.** The source's lists are inconsistent about amlodipine, hydrochlorothiazide and
  sertraline (left off some home lists while the prose says she takes them).
  Resolution: she takes all seven home drugs continuously until hydrochlorothiazide stops
  in June 2026 (A7). Outside letters may copy the stale lists. The ICU note's "prior
  lithium toxicity in March 2022" follows the re-dated timeline: April 2025.
- **R10.** The source has a "flash pulmonary edema" hospitalization only as history in the
  October 2025 admission. Resolution: it is the September 2025 admission (A5).
- **Left as the source has it:**
  - No ARB or other replacement after lisinopril stops, and no drug change while BP runs
    about 155 to 160 over 95 to 98 from mid-2023 until the October 2025 stent; no one
    comments.
  - After the first lithium toxicity (April 2025), lithium and hydrochlorothiazide both
    continue unchanged; no one remarks on the thiazide until the second toxicity.
  - Bipolar I disorder and major depressive disorder both stay on the problem list.
  - Essential hypertension stays on the list beside renovascular hypertension.
  - eGFR runs in the high 50s to 60s from late 2023 (creatinine 1.0 to 1.2); no kidney
    disease diagnosis is added.

## Additions (A): physician to confirm

- **A1.** 2021-09-14 new-patient visit (the first source encounter reads as established
  care; the seed gives five years with the practice).
- **A2.** 2023-07-25 practice visit that refers her to nephrology (office BP about 156/96).
- **A3.** 2024-02-20 practice visit that refers her to the cardiology group running the
  lipid study.
- **A4.** The April 2025 ED stay: observation 2025-04-24 to 2025-04-26; ED vital signs normal as
  recorded (BP 128/76, HR 84, RR 16, 36.9 °C, SpO2 98%); IV isotonic fluids, ondansetron
  and lithium held; no dialysis needed; lithium 1.4 at about 12 hours and 0.8 the next morning;
  tremor and confusion resolved, gait steady at discharge; psychiatry restarted lithium
  300 mg twice daily on 2025-04-28 by phone; level back in range on 2025-05-06.
- **A5.** Flash pulmonary edema admission 2025-09-17 to 2025-09-19: woke at night acutely
  short of breath; BP 176/100 on arrival, SpO2 89% on room air; chest x-ray with
  pulmonary edema; high-sensitivity troponin I normal (6 ng/L); NT-proBNP elevated (1,240 pg/mL); creatinine 1.3;
  transthoracic echo with normal LV size and systolic function (EF 60%), no significant
  valve disease; IV furosemide and oxygen; discharged on her home drugs unchanged.
  Acute pulmonary edema recorded, then resolved.
- **A6.** During the October 2025 admission (2025-10-10 to 2025-10-15): one dose of IV
  furosemide on arrival for breathlessness; renal angiography
  on 2025-10-13 by interventional radiology confirms the significant right renal artery
  stenosis; one stent placed, good result, right femoral access; left renal artery without
  significant stenosis. Clopidogrel 75 mg daily with aspirin for 3 months (stopped
  2026-01-13 per the interventional team). Follow-up renal artery duplex 2025-11-18: stent
  patent. After the stent, BP runs about 130/82 and creatinine about 1.1 on the same home
  drugs; no further night breathlessness.
- **A7.** June 2026 ICU stay 2026-06-12 to 2026-06-16: IV isotonic fluids, lithium held,
  nephrology consulted; levels fell without dialysis (1.6 the next day, 0.9 on 2026-06-14);
  transferred to the floor 2026-06-14. Discharge: hydrochlorothiazide stopped for its
  effect on lithium clearance; lithium held until psychiatry review. Psychiatry restarts
  lithium 300 mg twice daily on 2026-06-18 (letter received 2026-06-19), after she
  chooses to stay on lithium over an alternative mood stabilizer; lithium 0.7 and creatinine 1.0 on 2026-06-29.
  Off the thiazide BP runs about 140/88; the physician plans a recheck before adding
  anything.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention
  (left kidney size, no hydronephrosis, heart sounds, lung fields). An abnormal finding
  appears only as the source or this file gives it. A report may carry the descriptors
  its kind always carries (how a kidney was measured, a stent's access site), but nothing
  that adds a territory, a severity grade or a symptom, or changes the diagnosis or plan.
- Records from the prior practice (received October 2021) may mention routine screening
  done there, with normal results: colonoscopy 2018 without polyps (repeat in 10 years),
  Pap with HPV co-test 2019 negative, Tdap 2017, COVID primary series spring 2021.
- Hypertension and diabetes visits about every 3 to 6 months with vitals, BMP, A1c (6.3
  to 7.1), lipids yearly on atorvastatin (LDL about 85 to 95), urine albumin/creatinine
  yearly (normal), foot exams; yearly physicals with PHQ-9 (low scores).
- Lithium monitoring: lithium level about every 6 months in range (0.6 to 0.9 outside the
  two toxicities), TSH and calcium yearly, normal.
- An outside psychiatrist manages lithium and sertraline; yearly letters say mood is
  stable with no dose change, plus the April 2025 and June 2026 restarts above.
- Yearly diabetic eye exam letters from an optometrist: no retinopathy.
- Prevention: yearly screening mammograms (normal); Pap with HPV co-test in 2024
  (negative); hepatitis C and HIV screening once (negative); influenza each fall; COVID
  vaccines; PCV20 (diabetes); Shingrix series.
- Refills, pharmacy and prior-authorization traffic, portal messages about results,
  refills and scheduling, home BP logs, a missed visit, reminders.
- Booked visit after 2026-08-27: blood pressure and lithium follow-up on 2026-09-22.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.


## Last seen

The practice last saw MRN00002723 on 2026-06-22. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. Nothing is booked after today.
