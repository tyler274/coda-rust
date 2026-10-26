# Build requirements: unit-2698 (Valerie Labrie, MRN00002698)

Source: synthetic_hospital patient 2698, in `source.md` beside this file. This file and
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

## Who she is (S)

Born 1971-05-03, female, White, not Hispanic or Latino, English, private commercial
insurance, lives in the seed's town. Regional sales manager (drives and flies for work).
Never smoker. Alcohol: social, 1 to 2 glasses of wine a week. No known drug allergies.
Surgical history: total abdominal hysterectomy (2010); right carpal tunnel release (2015).
Family history: mother with rheumatoid arthritis; father with deep vein thrombosis at 62;
maternal grandmother with type 2 diabetes.

Chronic conditions on arrival: type 2 diabetes on metformin 1000 mg twice daily;
hypertension on lisinopril 10 mg daily; rheumatoid arthritis on prednisone 5 mg daily
(R1 for onsets). She joins the practice in June 2025 (A1), right after the first source
encounter.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2025-05-21 | practice | calls to join; new-patient visit booked for 2025-06-20; prior records requested (P) |
| 2025-06-04 | outside records | prior primary care records received: the three chronic conditions, the three drugs, surgical and family history, NKDA; routine screening there normal (P) |
| 2025-06-09 | outside (ED, regional hospital) | 3 days of worsening left calf pain and swelling; long car trip 2 weeks earlier; no trauma. Vitals stable. Left calf red, warm and tender, pitting edema ankle to mid-calf, left calf 3 cm larger than right; rest of exam unremarkable. Renal function normal. Venous duplex: noncompressible left popliteal vein, DVT. **Acute left popliteal DVT diagnosed; apixaban started** (A2: 10 mg twice daily for 7 days, then 5 mg twice daily, planned 3 months as a travel-provoked clot). Discharged from the ED. ED record received 2025-06-11 |
| 2025-06-20 | practice | establish-care visit (A1): history as above, calf improving on apixaban; baseline labs; prevention reviewed |
| 2025-09-12 | practice | DVT and diabetes follow-up: swelling gone, 3 months of apixaban complete; **apixaban stopped; popliteal DVT resolved** (A2) |
| 2025-11-28 to 2025-12-01 | outside (admission, regional hospital internal medicine) | the morning of 11-28 she messages the practice about the leg and is sent to the ED (P). 2 days of worsening left calf and thigh pain and swelling; back from a transatlantic flight 3 days earlier; no trauma; on prednisone; prior left popliteal DVT. Vitals stable. Left calf red, warm, tender; pitting edema calf to ankle; left calf 3 cm larger than right; calf pain on passive dorsiflexion (positive Homans sign; the chart describes the sign without the eponym). CBC within normal limits, creatinine 1.0, D-dimer elevated (R3). Venous duplex (A3): thrombus in the left common femoral vein extending into the femoral and popliteal veins, with extension into the left external iliac vein. **Left iliofemoral DVT diagnosed; enoxaparin then apixaban; apixaban continued indefinitely as a second clot** (A2). Discharge summary received 2025-12-02 |
| 2025-12-05 | practice | post-discharge visit (P): leg improving, on apixaban 5 mg twice daily long-term |
| 2026-06-12 | practice | **last seen**: routine diabetes/blood pressure follow-up and yearly labs in the morning, well (P) |
| 2026-06-12 to 2026-06-17 | outside (ED then ICU, regional hospital) | that afternoon she slips and falls at home, then sudden severe abdominal pain and vomiting; no head injury or loss of consciousness. In the ED BP 85/50, HR 110, afebrile; abdomen tender without rebound or guarding. Na 130, K 5.5, BUN 25, creatinine 1.2, glucose 92, WBC 12.0. Normal saline started, pressure stays low. **Adrenal crisis** in a patient on chronic prednisone; hydrocortisone 100 mg IV given, then stress dosing (A4); ICU admission. CT abdomen and pelvis (A5) without acute findings. Lisinopril, metformin and apixaban held, then resumed (A4). Discharge 06-17 (A4). Discharge summary received 2026-06-18, filed without changing the practice's lists |
| 2026-07-24 | outside (endocrinology) | consult letter (A6), received 2026-07-28, filed |
| after today | practice | booked follow-up visit (P) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source gives no onset for the chronic conditions. The chart uses:
  hypertension since about 2013, type 2 diabetes since about 2016, rheumatoid arthritis
  since about 2008 with prednisone 5 mg daily for years, all managed elsewhere before she
  joined. Prior records carry these as history; no outside dates are invented beyond that.
- **R2.** The source's profile and encounter 1 problem lists show the popliteal DVT as
  active in November 2025, and encounter 2 lists both DVTs as active in June 2026.
  Resolution: the popliteal DVT is treated and marked resolved in September 2025; the
  iliofemoral DVT is the active problem from 2025-11-28. Outside lists may still copy both,
  as outside lists do.
- **R3.** Encounter 1 gives the D-dimer only as "elevated". The admission record reports
  it as 2.6 mg/L FEU (reference below 0.5); no other D-dimer is drawn.
- **R4.** The comorbidity "Delayed management of life-threatening respiratory failure" has
  no event in any encounter: she never has respiratory failure. Dropped.
- **R5.** Encounter 0 says "vital signs stable" and "renal function normal" without
  numbers; the ED record carries ordinary normal vitals and a normal creatinine and BMP.
  Encounter 1 and the 2025 practice visits likewise carry ordinary vitals.
- **R6.** Encounter 2 occurs on the seed's last-seen day. Resolution: the practice visit
  is that morning and she is well at it; the fall and the crisis follow in the afternoon.
  The practice learns of the admission only from the hospital and her messages.
- **R7.** Encounter 2 gives sodium and potassium without chloride or bicarbonate. The
  ED panel reports chloride 99 and bicarbonate 20 so the panel is internally coherent;
  no other ED chemistry values are added.
- **Left as the source has it:** rheumatoid arthritis on prednisone 5 mg alone, with no
  DMARD and no rheumatology care in the chart; no statin in a diabetic in her fifties; no
  thrombophilia testing after two clots with a father's DVT. Nobody raises or comments on
  any of these.

## Additions (A): physician to confirm

- **A1.** Establish-care visit on 2025-06-20 and the joining call; she had no primary care
  physician locally when the first clot happened.
- **A2.** Anticoagulation (the source diagnoses two DVTs and names no treatment):
  apixaban 10 mg twice daily for 7 days then 5 mg twice daily from 2025-06-09, stopped
  2025-09-12 after 3 months; restarted 2025-11-28 (enoxaparin 1 mg/kg twice daily in
  hospital, then apixaban 10 mg twice daily for 7 days, 5 mg twice daily after) and
  continued indefinitely as a recurrent clot.
- **A3.** The two venous duplex reports: 2025-06-09 as the source states; 2025-11-28 with
  the thrombus extent above, which is what "iliofemoral" means.
- **A4.** Adrenal crisis care: hydrocortisone 100 mg IV in the ED, then 50 mg IV every 6
  hours, BP above 100 systolic within hours after it; out of the ICU on day 2; tapered to
  prednisone 10 mg daily for 5 days then back to 5 mg daily; apixaban held until the CT,
  then resumed; lisinopril and metformin held and resumed before discharge. Discharge
  labs: Na 136, K 4.3, creatinine 0.9. Discharged with sick-day rules (double the
  prednisone dose for fever or acute illness), an intramuscular hydrocortisone 100 mg
  emergency kit, and endocrinology follow-up.
- **A5.** CT abdomen and pelvis with contrast in the ED (fall on an anticoagulant with
  abdominal pain): no free fluid or hemorrhage, adrenal glands normal, no acute process.
- **A6.** Endocrinology consult 2026-07-24: steroid-dependent adrenal insufficiency from
  long-term prednisone; doing well; prednisone 5 mg daily continued; sick-day rules and
  emergency kit reviewed; medical alert ID advised; follow up in 6 months. No new tests
  with values.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may carry
  the descriptors its kind always carries, but nothing that adds a territory, a severity
  grade or a symptom, or changes the diagnosis or plan.
- Records from the prior practice may mention routine screening done there, with normal
  results (a screening colonoscopy in 2021 without polyps; mammograms normal).
- Diabetes and blood pressure visits about every 3 to 6 months: vitals, A1c around 7.2 to
  7.6 on metformin, BMP or CMP, lipids and urine albumin/creatinine yearly, foot exams;
  blood pressure at goal on lisinopril. Height about 165 cm, weight about 77 kg, stable.
- A yearly diabetic eye exam letter from an optometrist: no retinopathy.
- Prevention: screening mammogram in 2025, normal; no cervical screening after the
  hysterectomy; influenza vaccine in the fall, an updated COVID vaccine, PCV20 (diabetes,
  chronic steroid), the two-dose shingles series; hepatitis C and HIV screening once,
  negative; PHQ-2/PHQ-9 screens negative.
- Prednisone, metformin, lisinopril and apixaban refills; an apixaban prior
  authorization; portal messages about results and scheduling; a cancelled visit.
- After last seen: portal messages and a nurse's post-discharge phone call; refills; the
  outside records filed as received. The practice's problem and medication lists stay as
  they were on 2026-06-12.
- Booked visit after 2026-08-27: hospital follow-up and diabetes check.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002698 on 2026-06-12. Practice contacts after that day do not
happen; outside care after it still does, and its records are filed as received without
changing the practice's lists. A visit is booked after today.
