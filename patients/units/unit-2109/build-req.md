# Build requirements: unit-2109 (Adam Upton, MRN00002109)

Source: synthetic_hospital patient 2109, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz-like prose (the desert, the stars, the
cottonwood, the windmill ladder).

## Who he is (S)

Adam Upton, born 1966-11-06, male, White, not Hispanic or Latino, English, Belchertown
MA 01007, +1-413-555-0165, adam.upton@example.test. Married; his wife comes to some
visits and calls the office. Runs a small beef cattle operation (rancher), outdoors in
all seasons. Never smoker. Alcohol: rare, holidays only, 1 to 2 drinks; cut back over
the last decade. No cocaine, stimulants, anabolic steroids or herbal supplements.
Eats simply (beans, eggs, beef). Black coffee.

Allergies: no known drug allergies.

Surgical history: wide local excision of a soft-tissue sarcoma of the right thigh
(2014, R4); left forearm arteriovenous fistula creation (2022-10-07, A3). Never had
chest radiation, chest trauma or cardiac surgery.

Family history (R5): father had hypertension and a myocardial infarction at 68, died
in his late 80s ("old age"), before the patient joined; mother living, type 2 diabetes,
chronic kidney disease, osteoarthritis; brother with hyperlipidemia; paternal
grandfather with congestive heart failure. No family history of hypertrophic
cardiomyopathy or sudden cardiac death.

Insurance: commercial Blue Cross Blue Shield (individual plan) when he joins; Medicare
primary with the Blue Cross Blue Shield plan as supplement once dialysis starts in 2020
(texture only: a front-desk note updates it).

Joined the practice 2019-09-10 (new patient visit, A1). Brought:

- Chronic kidney disease stage 4 (R1), followed by an outside nephrology group; cause
  never named in the chart.
- Anemia of chronic kidney disease (R2), on epoetin alfa 10,000 units subcutaneously
  monthly from nephrology.
- Hyperphosphatemia, on calcium acetate 667 mg three times daily with meals from
  nephrology.
- Chemotherapy-induced (doxorubicin) cardiomyopathy with asymptomatic left ventricular
  dysfunction, found on echo in 2015 (LVEF 45%, A2), followed by an outside cardiology
  group about every year or two; on lisinopril 10 mg daily from cardiology.
- History of soft-tissue sarcoma of the right thigh, 2014, wide local excision and
  doxorubicin-based chemotherapy at an outside cancer center, in remission, under
  surveillance (R4, A9).
- Furosemide 40 mg daily, aspirin 81 mg daily, a daily multivitamin, acetaminophen
  500 mg as needed for back soreness.

## Timeline (dates fixed; S unless marked)

Records are sparse (seed): outside care mostly reaches the practice as the patient's
or his wife's account, a short note or a fax, with few numbers. Each row says what
arrives.

| Date | Where | What happens |
|---|---|---|
| 2019-09-10 | practice | **new patient visit (A1)**: history as above; prior PCP sends a few pages (colonoscopy 2017 normal, P). Baseline labs (P): CKD-range creatinine (about 3.6), hemoglobin about 10.8 on epoetin, phosphorus about 5.0, HIV and hepatitis C screens nonreactive, hepatitis B surface antigen negative. **Conditions recorded: CKD stage 4, anemia of CKD, hyperphosphatemia, drug-induced cardiomyopathy, history of soft-tissue sarcoma. Drugs recorded as historical: lisinopril, furosemide, aspirin, calcium acetate, epoetin alfa, multivitamin, acetaminophen** |
| 2019-10 to 2019-11 | practice | nurse visits: influenza, Tdap, hepatitis B vaccine (Heplisav-B) dose 1 and dose 2 (P) |
| 2020-02-11 | practice | follow-up: more tired; nephrology has started talking about dialysis; creatinine about 4.9 (P/R1) |
| 2020-04 (early) | outside | **outpatient hemodialysis started** by nephrology, Monday/Wednesday/Friday, through a tunneled right internal jugular catheter (A3). Practice learns from his wife's phone call 2020-04-14. **ESRD on hemodialysis replaces CKD stage 4. Sevelamer carbonate 800 mg three times daily with meals added by nephrology (R3)** |
| 2020-06-16 | practice | telehealth follow-up: settling into dialysis; med list updated (P) |
| 2020 to 2022 | practice | routine visits, wellness visits, vaccines (P); oncology surveillance letter 2021 (A9) |
| 2022-10-07 | outside | **left forearm AV fistula created** by vascular surgery (A3); no operative note reaches the practice |
| 2022-10-21 | outside | vascular surgery post-op note (faxed): two weeks after left forearm AV fistula creation; new fatigue and mild breathlessness on stairs, which he blames on his anemia; no chest pain, edema or orthopnea. T 98.7 F (37.1 C), HR 104, BP 132/64, RR 16, SpO2 98% room air. Extremities warm with bounding peripheral pulses, prominent bruit over the fistula, apex displaced laterally. Plan: fistula maturing; cannulation by the dialysis unit when ready; fatigue back to nephrology and primary care (R6). **AV fistula recorded** |
| 2022-11-08 | practice | follow-up (P): same fatigue and stair breathlessness since surgery; fistula with thrill and bruit; lungs clear, no edema; hemoglobin about 10.0; plan to share with nephrology |
| 2023-02-22 | outside | **tunneled catheter removed** once the fistula is in use (A3); his wife's portal message the next day |
| 2023-03-21 | practice | **referral visit (A4)**: numbness, tingling and burning in the right thumb, index and middle fingers since about October 2022, worse at night and during dialysis; referred to neurology |
| 2023-04-27 | outside | neurology consult letter: six months of intermittent numbness, tingling and burning pain in the right hand, worse at night and during dialysis, thumb, index and middle fingers. Age 56 (R7). On dialysis about 3 years (S, matches R1). Tinel sign positive over the right carpal tunnel, Phalen test reproduces symptoms at 30 seconds, decreased sensation in the median distribution, weak thumb opposition, no edema or erythema. Recent dialysis-unit labs quoted: creatinine 6.5, BUN 55, hemoglobin 10.2, calcium 9.2, phosphorus 5.4, PTH 300 pg/mL (elevated). **Carpal tunnel syndrome, right hand, diagnosed**. Plan: nerve conduction study, night wrist splint (A5). The fistula is in the left forearm (R8) |
| 2023-05-23 | outside | nerve conduction study and EMG (A5): right median mononeuropathy at the wrist; right ulnar and radial nerves normal; no other abnormality. Reaches the practice as a short neurology addendum letter in June: keep the splint; hand surgery referral if splinting fails |
| 2023-11-06 | outside | nephrology note (dialysis unit; faxed only after a records request, received January 2024): routine ESRD follow-up; several weeks of worsening fatigue and exertional breathlessness limiting his daily walks; no chest pain, palpitations or dizziness. BP 138/82, HR 88. Pallor, otherwise normal exam. Hemoglobin 8.2, MCV 86, ferritin 100, transferrin saturation 15%, creatinine 6.5, BUN 55. Assessment: anemia of CKD, worse, with low iron saturation. **IV iron sucrose 100 mg at 10 consecutive dialysis sessions (A6)**; epoetin unchanged (left as the source has it). Calcium acetate continues (R3) |
| 2023-12-05 | practice | follow-up (P): his account of the nephrology visit ("blood count down to 8, they're giving me iron"); no new findings |
| 2024-03-19 | practice | follow-up (P): energy somewhat better; hemoglobin about 9.8 (A6) |
| 2024-05 | outside | oncology surveillance letter (A9): 10 years from treatment, no evidence of recurrence on exam and chest CT; discharged from routine surveillance |
| 2024-09-24 | practice | **referral visit (A7)**: several months of slowly worsening breathlessness, extra pillows at night, waking with coughing fits, ankle swelling by evening; exam: bilateral ankle pitting edema and fine crackles at both bases; cardiology asked for its first opening (the 2024-10-21 visit stands); asks nephrology to reassess his target weight |
| 2024-10-21 | outside | cardiology note (short, faxed): age 57; gradually progressive dyspnea over months, orthopnea needing extra pillows, paroxysmal nocturnal dyspnea with coughing fits that settle when he sits up by an open window, evening ankle swelling (boots tight), more easily tired with chores, vague dull exertional chest pressure that is not sharp or pleuritic, breathless walking the fence line, pauses to rest; neighbors say his face looks puffy; belt tighter, weight roughly steady. No fever, URI, palpitations, syncope, orthostatic dizziness, viral illness, tick bites, rash, calf pain, claudication, or recent travel. No chest radiation, chest trauma or cardiac surgery. Exam: comfortable at rest, winded changing position; JVP mildly distended reclining, softer sitting; fine basilar crackles clearing only partly with deep breaths; laterally displaced apical impulse; soft low-pitched early diastolic extra sound (S3); holosystolic apical murmur radiating to the axilla; abdomen soft with mild right upper quadrant fullness, nontender; pitting edema to the mid-shins; peripheral pulses intact and symmetric. Chest x-ray and ECG done in clinic; their reports never reach the practice (R9). Transthoracic echo ordered. Impression: progression of his anthracycline cardiomyopathy to symptomatic heart failure (R10) |
| 2024-11-12 | outside | **transthoracic echocardiogram (A8)**: LVEF 35% (45% in 2015), dilated left ventricle, moderate functional mitral regurgitation, no pericardial effusion; other findings ordinary. Reaches the practice only as a sentence in the cardiology letter |
| 2024-12-10 | outside | cardiology follow-up letter (A8): echo result as above; **heart failure with reduced ejection fraction recorded**; continue lisinopril 10 mg, furosemide 40 mg and aspirin; nephrology lowers his target (dry) weight by 2 kg; cardiology follow-up in a year |
| 2025-01-14 | practice | follow-up (P): breathing somewhat better since the target weight was lowered; ankle edema less |
| 2025-09-16 | practice | scheduled follow-up: **no-show** (haying; texture) |
| 2025-10-25 | outside | ED record (short): age 58; two months of worsening exertional breathlessness, swelling of both legs, waking at night short of breath, more tired per his wife; no chest pain, cough or fever; no recent infection; no missed or changed dialysis. BP 138/72, HR 102, RR 20 (other vitals not given). JVP visible at 45 degrees; soft systolic murmur over the precordium; bilateral pitting edema to the knees; bibasilar crackles; palpable thrill over the left forearm fistula. Hemodialysis three times weekly through the left forearm fistula. **Impression: acute decompensated heart failure. Admitted for urgent hemodialysis with ultrafiltration (A10)** |
| 2025-10-28 | outside | discharged (A10); no discharge summary reaches the practice. His account at the next visit: three dialysis sessions in the hospital, about 5 kg removed, target weight lowered again, no medication changes |
| 2025-11-04 | practice | post-hospital visit (P): breathing back near his baseline; trace ankle edema; lungs clear; acute decompensation resolved on the problem list; HFrEF remains |
| 2026-01-20 | outside | cardiology follow-up letter (A11): euvolemic at the new target weight; no medication changes; follow-up in six months |
| 2026-02 to 2026-08 | practice | routine follow-up, annual wellness visit, refills (P) |
| 2026-09-22 | practice | booked follow-up visit (P) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source calls him "on maintenance hemodialysis" at his first encounter,
  which is two weeks after his fistula was made, and says in April 2023 that he has
  been on dialysis "for 3 years". Resolution: he joins in 2019 with CKD stage 4,
  progresses (creatinine about 3.6 in 2019, about 4.9 in February 2020), and starts
  hemodialysis in early April 2020 through a tunneled catheter (A3); the fistula comes
  in October 2022. ESRD on hemodialysis is recorded from April 2020. Pre-dialysis
  creatinines rise from 3.6 toward 5; on dialysis, creatinine about 6.5 and BUN about 55
  (pre-dialysis draws).
- **R2.** Anemia of CKD is stamped as diagnosed at the November 2023 nephrology visit,
  but epoetin is on every source list from the first encounter, his October 2022 note
  blames his fatigue on anemia, and hemoglobin is 10.2 in April 2023. Resolution:
  anemia of CKD is known at joining (on epoetin); November 2023 is when it worsens,
  with low iron saturation. Hemoglobin about 10.8 in 2019, about 10.0 in late 2022,
  10.2 in April 2023, 8.2 in November 2023, about 9.8 after iron and staying 9.5 to
  10.5 after that.
- **R3.** Sevelamer and calcium acetate both appear from the first encounter; the
  November 2023 nephrology list leaves calcium acetate off while its own prose names
  it. Resolution: calcium acetate from before joining; sevelamer added when dialysis
  starts in April 2020; both continue throughout. The 2023 nephrology list's omission
  is not carried as a stop.
- **R4.** The source gives no dates for the sarcoma, the excision or the doxorubicin.
  Resolution: 2014, right thigh, wide local excision and doxorubicin-based chemotherapy
  at an outside cancer center; in remission since.
- **R5.** Family history: three source lists give father with hypertension and MI at
  68, mother with type 2 diabetes and CKD; the cardiology note says the father died of
  "old age" in his late 80s and the mother is living with osteoarthritis. Resolution:
  both are true (father had the MI at 68 and died in his late 80s; mother living with
  diabetes, CKD and osteoarthritis). No family HCM or sudden cardiac death.
- **R6.** The vascular surgery note breaks off mid-sentence ("Compared to his
  presurgical state,") and has no plan. Resolution: the cut-off sentence is dropped; the
  note ends with an ordinary plan (fistula maturing, cannulation by the dialysis unit,
  fatigue back to nephrology and primary care). No test is ordered there.
- **R7.** Ages follow the birth date: 55 at the October 2022 vascular visit, 56 in
  April 2023 (the source says 55), 57 in October 2024, 58 in October 2025.
- **R8.** The neurology note puts the fistula in the right arm; every other record
  says left forearm. Resolution: left forearm throughout.
- **R9.** The cardiology note says a chest image and an ECG were done, with no
  results. Resolution: they were done; their reports never reach the practice (sparse
  records), and no one in the chart states their findings.
- **R10.** "Trastuzumab-induced cardiotoxicity with asymptomatic left ventricular
  dysfunction" is stamped at the cardiology visit, but trastuzumab is not a sarcoma
  drug and he received doxorubicin; at that visit he is symptomatic. Resolution:
  trastuzumab never appears in the chart. The entry maps to his known doxorubicin
  cardiomyopathy with asymptomatic LV dysfunction (2015 to 2024), which the October
  2024 cardiology visit finds has become symptomatic; it is recorded as heart failure
  with reduced ejection fraction once the echo returns (A8). "Chemotherapy-induced
  cardiomyopathy" stays on the problem list throughout.
- **R11.** The cardiology note says he has lived most of his adult life on a ranch in
  Nevada. Resolution: dropped; his ranch is in Belchertown. The desert and night-sky
  prose is not carried.
- **R12.** The source's medication list never changes across the five encounters,
  including after the 2024 heart-failure workup and the 2025 decompensation.
  Resolution: the list stays as the source has it, plus the dated changes in R3 and
  A6; the dialysis prescription (target weight) carries the volume changes.
- **R13.** Interobserver differences stay as written: an S3 and an apical holosystolic
  murmur radiating to the axilla in 2024, a soft precordial systolic murmur in 2025; a
  bruit at the fistula in 2022 and a thrill in 2025.
- **Left as the source has it:** no beta-blocker or other added heart-failure drug is
  ever started, despite the LVEF of 35% and two volume episodes; nobody raises one.
  Epoetin stays at 10,000 units subcutaneously monthly, an unusual route and schedule
  for an in-center hemodialysis patient, and is never adjusted. Kidney transplant is
  never discussed. The 2022 tachycardia and bounding pulses after the fistula are not
  worked up. No one comments on any of these.

## Additions (A): physician to confirm

- **A1.** 2019-09-10 new-patient visit (the source's first encounter already has
  established ESRD care and seven years with the practice per the seed).
- **A2.** The 2015 echo showing LVEF 45% after doxorubicin, which put him on
  lisinopril; gives the "asymptomatic LV dysfunction" a basis and a baseline.
- **A3.** Dialysis access history the source needs: outpatient hemodialysis started in
  early April 2020 through a tunneled right internal jugular catheter, Monday,
  Wednesday and Friday; left forearm AV fistula created 2022-10-07 (the source's "two
  weeks" before 2022-10-21); catheter removed in February 2023 once the fistula is in
  use.
- **A4.** 2023-03-21 practice visit that refers him to neurology.
- **A5.** Carpal tunnel next steps: nerve conduction study and EMG 2023-05-23 showing
  right median mononeuropathy at the wrist with normal ulnar and radial nerves (no
  severity grade); a night wrist splint; hand surgery referral offered if splinting
  fails. At later practice visits the symptoms are tolerable with the splint, and no
  referral or release happens.
- **A6.** Iron for the November 2023 anemia with transferrin saturation 15%: iron
  sucrose 100 mg IV at 10 consecutive dialysis sessions, given by the dialysis unit
  (recorded at the practice as a historical short course). Hemoglobin about 9.8 by
  March 2024.
- **A7.** 2024-09-24 practice visit that asks cardiology for its first opening, with
  bilateral ankle pitting edema and fine bibasilar crackles.
- **A8.** The echo the cardiology note arranges: 2024-11-12, LVEF 35%, dilated left
  ventricle, moderate functional mitral regurgitation, no pericardial effusion; and the
  2024-12-10 cardiology letter: HFrEF; continue lisinopril, furosemide and aspirin;
  target weight lowered 2 kg by nephrology; follow-up in a year.
- **A9.** Sarcoma surveillance, its standard care: oncology letters in 2021 and May
  2024, each with no evidence of recurrence on exam and chest CT; the 2024 letter
  discharges him from routine surveillance at 10 years.
- **A10.** ED disposition for acute decompensated heart failure in a dialysis patient:
  admitted 2025-10-25 for urgent hemodialysis with ultrafiltration, discharged
  2025-10-28; per his account three sessions, about 5 kg removed, target weight lowered
  again, no medication changes.
- **A11.** Cardiology follow-up letter 2026-01-20: euvolemic at the new target weight,
  no changes, follow-up in six months.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may
  carry the descriptors its kind always carries, but nothing that adds a territory, a
  severity grade or a symptom, or changes the diagnosis or plan. Practice exams after
  2022-10 note the left forearm fistula with thrill and bruit; between the volume
  episodes, lungs clear and no edema; trace ankle edema only at the 2025-11-04 visit.
- Records from a prior practice may mention routine screening done there, with normal
  results (colonoscopy 2017, normal, repeat in 10 years).
- Visits every 3 to 6 months (fewer during 2020, some by video), on Tuesdays or
  Thursdays around dialysis; Medicare annual wellness visits from 2021 (a preventive
  visit at joining in 2019). Blood pressure about 130s/70s on lisinopril. Height 180 cm,
  weight about 86 kg at joining, drifting within a few kilograms; up about 3 kg at the
  2024-09-24 visit and back down after the target-weight change; down about 5 kg after
  the October 2025 admission.
- Labs: the dialysis unit draws his monthly labs and rarely sends them; the practice
  draws a CBC and metabolic panel once or twice a year, a lipid panel at joining and
  in 2024 (normal), phosphorus now and then. Hemoglobin, creatinine, BUN and phosphorus
  follow R1 and R2; phosphorus about 5.0 to 5.6 on dialysis; calcium normal.
- Prevention: influenza each fall (some given at the dialysis unit and reported);
  COVID-19 primary series at the dialysis unit in early 2021, then boosters and updated
  doses each fall; Heplisav-B two doses in 2019 (hepatitis B surface antibody immune in
  early 2020); Tdap 2019; Shingrix two doses 2021 to 2022; PCV20 in 2022; RSV (Arexvy)
  in 2025. HIV and hepatitis C screening once (2019). PHQ-2 negative at wellness visits.
- Refills, pharmacy traffic, portal messages (often from his wife) about results,
  scheduling and dialysis-day conflicts, a records request to the dialysis unit, a
  missed visit (2025-09-16), an insurance update in 2020.
- Booked visit after 2026-08-27: routine follow-up 2026-09-22.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.


## Last seen

The practice last saw MRN00002109 on 2025-11-04. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. Nothing is booked after today.
