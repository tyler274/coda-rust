# Build requirements: unit-2218 (Edward Bennett, MRN00002218)

Source: synthetic_hospital patient 2218, in `source.md` beside this file. This file and
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

## Who he is (S)

Edward Bennett, born 1952-01-28, male, White, not Hispanic or Latino, English,
Medicare, lives in Belchertown (01007), phone and email from the seed. Retired high
school mathematics teacher. Never smoker. Alcohol socially, 1 to 2 drinks a week.
No known drug allergies. Married: his wife is the person who brings him to the ED in
2025 (S, enc 7). Family history: father type 2 diabetes, and died of prostate cancer
at 75 (R1); mother hypertension; older brother benign prostatic hyperplasia.
Surgical history: elective inguinal hernia repair 2023-10-09 and right total knee
arthroplasty 2024-04-22 (R2, R3); nothing before joining.

Joined the practice 2021-06-08 (A1, new patient visit), from a prior practice. He
brought: type 2 diabetes on metformin 1000 mg twice daily; essential hypertension on
lisinopril 10 mg daily, well controlled; atorvastatin 20 mg daily (for diabetes,
primary prevention; no lipid diagnosis is listed); right knee osteoarthritis followed
by an outside orthopedic group (A2). No urinary symptoms at joining. Not on tamsulosin
at joining (R4). Height 178 cm, weight about 92 kg (BMI 29, S enc 0), stable throughout.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2021-06-08 | practice | new patient visit (A1): history as above; diabetes and blood pressure stable; labs |
| 2021-11-16 | practice | dysuria, frequency; urinalysis leukocyte esterase and nitrite positive, culture E. coli; **urinary tract infection; cephalexin 500 mg four times daily for 7 days** (A3, first of the source's "two UTIs in the past year"); resolved |
| 2022-02-08 | practice | about 4 months of frequency, nocturia, hesitancy and a weak stream, no dysuria; urinalysis negative; **tamsulosin 0.4 mg daily started** (R4, A4); urology referral |
| 2022-04-20 | outside | urology consult letter (enc 0): 6 months of frequency, nocturia, hesitancy, weak stream, no dysuria; BP 135/85, HR 78, BMI 29; DRE symmetrically enlarged, firm, non-tender prostate; urinalysis without infection or hematuria; Cr 1.0; fasting glucose 140; PSA 2.8 ng/mL (A5). **Benign prostatic hyperplasia with lower urinary tract symptoms diagnosed** (R5); continue tamsulosin |
| 2022-07-19 | practice | dysuria again; urinalysis positive, culture E. coli; **urinary tract infection; cephalexin 500 mg four times daily for 7 days** (A3, second); resolved |
| 2022-10-10 | outside | urology follow-up (enc 1), letter arrives late (received 2022-12-19): 6 months of frequency, urgency, nocturia, incomplete emptying; two UTIs in the past year treated with antibiotics; abdomen soft, non-tender; DRE symmetrically enlarged, firm, non-tender; urinalysis without infection; Cr 1.2, BUN 18; post-void residual by bladder ultrasound 120 mL; continue tamsulosin |
| 2023-07-25 | practice | right groin bulge on lifting, reducible; **inguinal hernia diagnosed** (A6); general surgery referral |
| 2023-10-09 | outside | elective right inguinal hernia repair (S, R3); no operative note ever reaches the practice |
| 2023-10-11 | outside | ED (enc 2): severe lower abdominal pain, unable to void for 8 hours, 2 days after the hernia repair; a year of worsening weak stream, nocturia, incomplete emptying; urology visits in April and October 2022; BP 145/90, HR 88; uncomfortable; abdomen distended, suprapubic tenderness; DRE enlarged, firm, non-tender; bladder ultrasound about 600 mL. **Acute urinary retention diagnosed**; urethral catheter placed, sent home with it for urology (A7). The ED medication list includes hydrochlorothiazide (R6) |
| 2023-10-18 | outside | urology office voiding trial passed, catheter out (A7); no letter; the patient reports it by portal |
| 2023-12-13 | practice | (enc 3) 2 months of lightheadedness on standing from sitting, sometimes with dizziness and blurred vision; BP 145/85 seated, 120/70 after 3 minutes standing; HR 78 seated, 88 standing; rest of exam unremarkable; A1c 7.2, Cr 1.1, Na 140, K 4.2. **Orthostatic hypotension diagnosed**; counseling to rise slowly, hydrate, take tamsulosin at bedtime; no drug change (Left as source) |
| 2024-03-12 | practice | preoperative medical evaluation for the planned right knee replacement (A8): ECG normal sinus rhythm; labs; cleared |
| 2024-04-22 | outside | right total knee arthroplasty at an outside orthopedic hospital, same-day discharge (R2); operative note arrives late (received 2024-06-25) |
| 2024-04-23 | outside | ED (enc 4), received 2024-04-26: lower abdominal discomfort, unable to void since the knee replacement 18 hours earlier; prior acute retention October 2023; age 72; BP 165/90, HR 102; in distress; abdomen distended, tender suprapubic; urethral catheter attempt unsuccessful; portable bladder ultrasound 750 mL. Urology on call places a coude catheter; home with it (A7) |
| 2024-04-30 | outside | urology voiding trial passed (A7); no letter; patient reports it at the post-op visit |
| 2024-05-07 | practice | follow-up after the knee replacement and ED visit; **status post right total knee replacement** recorded; knee osteoarthritis resolved |
| 2024-10-14 | outside | ED (enc 5), record arrives late (received 2024-11-20); practice hears from the patient by phone 2024-10-15: 24 hours of severe lower abdominal pain and inability to void; 6 months of trouble starting, weak stream, nocturia; abdomen distended, suprapubic tenderness; DRE enlarged, firm, non-tender; Cr 1.2, BUN 18; bladder scan 300 mL (R7); catheter placed, urology follow-up (A7). **Urinary retention, recurrent** |
| 2024-10-22 | practice | post-ED visit; urology voiding trial passed 2024-10-21 (A7, patient report); continue tamsulosin |
| 2025-04-14 | outside | urgent care visit for urinary symptoms (A9): **sulfamethoxazole-trimethoprim 800/160 mg twice daily for 7 days**; urgent care sends no note; the practice learns of it from the patient and the pharmacy fill |
| 2025-04-21 | practice | routine follow-up (enc 6) of diabetes, hypertension, BPH with prior retention; 2 weeks of fatigue and muscle weakness; no chest pain, palpitations or syncope; taking the sulfamethoxazole-trimethoprim (last doses today); BP 135/85; exam unremarkable; K 5.8, Cr 1.4 (baseline 1.0), BUN 18, Na 140, glucose 160. **Hyperkalemia diagnosed**; ECG today normal sinus rhythm without peaked T waves (A10); finish and do not refill the antibiotic; repeat BMP in a week; lisinopril continued |
| 2025-04-28 | practice | repeat BMP (A10): K 4.7, Cr 1.1; **hyperkalemia resolved**; UTI resolved |
| 2025-10-16 | outside | ED (enc 7) then admission to 2025-10-19 (A11): 2 days of increasing confusion, agitation and nighttime visual hallucinations, per his wife; T 37.8 C, BP 160/90, HR 102, RR 18; disoriented to time and place, fluctuating consciousness; otherwise unremarkable except mild suprapubic tenderness; Na 140, K 4.2, Cl 102, HCO3 24, BUN 18, Cr 1.2, glucose 180, WBC 9.0, Hgb 13.5; urinalysis leukocyte esterase and nitrite positive. **Delirium due to medical condition** and **urinary tract infection** diagnosed; IV ceftriaxone, then **cephalexin 500 mg four times daily to complete 10 days** at discharge (A11); clear by discharge. Wife phones the practice 2025-10-17; discharge summary received late (2025-11-12) |
| 2025-10-24 | practice | post-discharge visit (P): back to himself; delirium resolved; UTI resolved after the course |
| 2026-04-17 | practice | last visit: annual wellness and diabetes follow-up; given three retention episodes and the UTI-related delirium, **referral back to urology to discuss definitive treatment** (A12), still open at today; labs |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Encounter 0 says his father died of prostate cancer at 75; every other
  encounter lists the father's type 2 diabetes. Both can be true: the chart carries
  father with type 2 diabetes who died of prostate cancer at 75.
- **R2.** The profile lists a right total knee arthroplasty from the first encounter,
  but encounter 4 places it 18 hours before that ED visit. The arthroplasty happens on
  2024-04-22 and the encounter-4 ED visit moves one day to 2024-04-23 (early morning),
  so the elective surgery falls on a weekday. Earlier notes carry right knee
  osteoarthritis (A2), never the replacement.
- **R3.** The hernia repair appears only in encounter 2 ("2 days ago") and no side is
  given. It happens 2023-10-09 and is recorded as right-sided; the profile's surgical
  history, which omits it, is not followed.
- **R4.** Tamsulosin is on every source list, including the first urology visit for a
  6-month complaint. It starts at the practice on 2022-02-08 when the symptoms brought
  him in, before the urology referral. Not at joining.
- **R5.** "Benign Prostatic Hyperplasia" and "Recurrent benign prostatic hyperplasia"
  are one condition, recorded once as benign prostatic hyperplasia with lower urinary
  tract symptoms, diagnosed in the 2022-04-20 urology letter. The profile's "chronic"
  BPH from the start is not followed.
- **R6.** Encounter 2 says he takes hydrochlorothiazide; no other encounter lists it.
  The practice never prescribed it. The ED record's medication list carries it as
  written (an outside list error), nobody remarks on it, and it is never on the
  practice list.
- **R7.** Encounter 5 calls 300 mL a "post-void residual" while he cannot void. The ED
  record gives it as the bladder scan volume, 300 mL, with no comment.
- **R8.** Encounters 3, 6 and 7 split the same four home drugs between "current" and
  "home" lists. All four continue unchanged throughout the practice years.
- **R9.** Urinary retention is "diagnosed" at encounter 2 and recurs at 4 and 5: one
  condition, diagnosed 2023-10-11, kept active. Orthostatic hypotension stays active.
- **R10.** The source's two outpatient UTIs before October 2022 have no dates: they are
  the 2021-11 and 2022-07 practice visits (A3). The encounter-6 "recent UTI" is the
  urgent care visit (A9).
- **Left as the source has it:** tamsulosin stays at 0.4 mg alone through three
  episodes of retention; no 5-alpha-reductase inhibitor and no outlet procedure are
  raised during the source's span. Tamsulosin continues after orthostatic hypotension
  is diagnosed. Lisinopril continues through the hyperkalemia. No post-void residual
  is checked at the 2025 admission. Nobody comments on any of these.

## Additions (A): physician to confirm

- **A1.** 2021-06-08 new-patient visit (five years with the practice; the source opens
  on established conditions).
- **A2.** Right knee osteoarthritis, followed by outside orthopedics, as the indication
  for the source's knee replacement. Recorded at joining from his report; resolved at
  the replacement. No knee drug or injection.
- **A3.** The two UTIs the source says he had in the year before October 2022:
  2021-11-16 and 2022-07-19, each with a positive urinalysis, a culture growing E. coli
  over 100,000 CFU/mL susceptible to cephalexin, and cephalexin 500 mg four times daily
  for 7 days.
- **A4.** The 2022-02-08 visit that starts tamsulosin and makes the urology referral.
- **A5.** PSA 2.8 ng/mL at the first urology visit (BPH workup; father's prostate
  cancer). No other PSA.
- **A6.** The 2023-07-25 visit that finds the inguinal hernia and refers to surgery.
- **A7.** The ordinary course after each retention episode: catheter placed (coude by
  urology on call in 2024-04, after the failed attempt), home with the catheter, a
  voiding trial at outside urology about a week later that he passes. Urology sends no
  letter for these; the practice knows from the patient.
- **A8.** Preoperative evaluation 2024-03-12 with a normal ECG and routine labs.
- **A9.** Urgent care 2025-04-14 prescribing sulfamethoxazole-trimethoprim for the
  source's "recent UTI"; no urgent care note in the chart.
- **A10.** ECG at the hyperkalemia visit (normal sinus rhythm, no peaked T waves) and
  the repeat BMP a week later: K 4.7, Cr 1.1.
- **A11.** Admission 2025-10-16 to 2025-10-19 for delirium with UTI: urine culture
  E. coli over 100,000 CFU/mL susceptible to ceftriaxone and cephalexin; IV ceftriaxone,
  discharged on cephalexin 500 mg four times daily to complete 10 days; oriented at
  discharge.
- **A12.** 2026-04-17 urology re-referral for definitive treatment of BPH with
  recurrent retention (the usual next step after the source ends). Open at today; no
  urology visit has happened.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Records from the prior practice (received 2021) may mention routine screening done
  there with normal results: colonoscopy 2017 without polyps, Tdap 2018, shingles
  series 2019.
- Diabetes and blood pressure visits about every 3 to 6 months, some by video, with
  vitals, A1c (about 6.6 to 7.7), BMP, yearly lipids (LDL in the 80s to 90s on
  atorvastatin 20), yearly urine albumin/creatinine (normal), foot exams (normal),
  urinalysis at urinary visits. Creatinine runs 1.0 to 1.1 at the practice outside the
  source values. BP at the practice about 130 to 136 / 78 to 86.
- Yearly diabetic eye exam letters from an outside optometrist: no retinopathy.
- Annual wellness visits with PHQ-2 (negative), a brief cognitive screen (normal),
  fall-risk questions; influenza (high-dose) each fall, updated COVID vaccines,
  PCV20 once, RSV once; hepatitis C screening once, negative.
- Refills (90-day), pharmacy traffic, portal messages about results, refills and
  scheduling, a cancelled visit, records requests for the late outside records.
- Records gaps as above: some outside letters late, some never sent; the problem list
  lags (the inguinal hernia stays listed until the 2024-03-12 visit resolves it).
- No booked visit after 2026-08-27.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002218 on 2026-04-17. Practice contacts after that day do
not happen except messages, refills and the results of that day's draw; outside
records after it are filed as received without changing the practice's lists. No
visit is booked after today.
