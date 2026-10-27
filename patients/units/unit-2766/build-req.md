# Build requirements: unit-2766 (Colleen Romano, MRN00002766)

Source: synthetic_hospital patient 2766, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, its "kidney x-ray is shown below" image line, or its
unfinished plan.

## Who she is (S)

Colleen Romano, born 1979-12-05, female, White, not Hispanic or Latino, English,
Chicopee MA 01020, private employer-sponsored PPO. Remote data analyst (works from home).
Former smoker, quit 2015, 10 pack-years. No alcohol.

Allergies: **trimethoprim-sulfamethoxazole (Stevens-Johnson syndrome; severe, high
criticality)** and **latex** (reaction not recorded in the source; carried without a
reaction).

Surgical history: appendectomy 2005. (The source's right percutaneous nephrolithotomy
and ureteral stent entries happen in 2026 inside this timeline: R3.)

Family history: mother with recurrent kidney stones; father with essential hypertension.

Joined the practice on **2022-09-13** (new patient visit, A1), bringing:

- Neurogenic bladder secondary to a remote T12 spinal cord injury (the injury predates
  the practice; its date, mechanism and functional level are not in the record, and the
  chart does not invent them: notes say "remote T12 spinal cord injury" and record the
  lower-extremity neurological exam as at her known baseline);
- Recurrent urinary tract infections;
- Chronic Proteus mirabilis bacteriuria (cultures often grow Proteus mirabilis);
- History of tobacco dependence (former smoker, as above);
- Drugs: **oxybutynin 5 mg by mouth three times daily, methenamine hippurate 1 g by mouth
  twice daily, docusate sodium 100 mg by mouth twice daily** (all historical, prescribed
  by her outside urologist and prior PCP; the practice takes over refills);
- Bladder managed by clean intermittent self-catheterization with latex-free catheters
  (A2), followed by an outside urology group (A3).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2022-09-13 | practice | new patient visit (A1): history as above; exam normal apart from the known lower-extremity deficit at baseline; height 165 cm, weight 68 kg, BP normal; labs (P) |
| 2022-11-08 | outside | yearly urology letter (A3): neurogenic bladder stable on intermittent catheterization, oxybutynin and methenamine; asymptomatic Proteus bacteriuria not treated |
| 2023-09-19, 2024-09-24, 2025-09-23 | practice | annual physicals (P) |
| 2023-11, 2024-11 | outside | yearly urology letters (A3), unchanged |
| 2024-02-20 | practice | symptomatic UTI (A4): two days of dysuria and cloudy urine, afebrile, no flank pain. UA cloudy, pH 7.5, leukocyte esterase 2+, nitrite positive, urine WBC 40 /HPF, urine RBC 3 /HPF; culture >100,000 CFU/mL Proteus mirabilis, susceptible to cephalexin, ciprofloxacin and ceftriaxone, resistant to nitrofurantoin. **Cephalexin 500 mg twice daily for 7 days** (short course) |
| 2025-06-17 | outside | ED (Emergency Medicine) record, received 2025-06-18: age 45; 2 days of right flank pain and dysuria; pain constant, radiating to the groin; frequent urge to void with only small amounts of cloudy urine; no fever, nausea or vomiting; right costovertebral angle tenderness. UA pH 8.5, pyuria, crystals present; urine stain shows no bacteria. **Urolithiasis (renal colic) diagnosed.** **Tamsulosin 0.4 mg daily started** (R1); acetaminophen as needed for pain (A5); urine strainer; follow up with urology and PCP. No imaging (left as the source has it) |
| 2025-06-24 | practice | post-ED visit (P): pain much better, no stone seen in the strainer yet; continues tamsulosin; BMP (creatinine at her 0.9 baseline); urology aware |
| 2025-09-04 | outside | urology letter (A3, R2): seen after the June ED visit; bladder management changed from intermittent catheterization to an **indwelling urethral catheter (latex-free silicone), changed monthly**, because of persistent urgency with small-volume voids since June; continue tamsulosin, oxybutynin, methenamine |
| 2025-10-19 | outside | ED record, received 2025-10-20: fever, flank pain and dysuria; indwelling catheter for the past several weeks for her neurogenic bladder; right costovertebral angle tenderness; UA alkaline (pH 8.5, R4) with rectangular prism-shaped crystals on microscopy; kidney x-ray (KUB) read as a radiopaque calculus over the right renal shadow (R5). ED treatment (A6): triage temperature 38.6 °C, HR 102, BP 124/76, SpO2 98%; ceftriaxone 1 g IV once; catheter exchanged; urine culture sent; **ciprofloxacin 500 mg twice daily for 7 days** (short course); urology follow-up. Culture (resulted 2025-10-21): >100,000 CFU/mL Proteus mirabilis, susceptible to ciprofloxacin and ceftriaxone, resistant to nitrofurantoin |
| 2025-10-28 | practice | post-ED visit (P): afebrile, flank pain resolved, finishing ciprofloxacin |
| 2025-12-09 | outside | urology letter (A3): reviewed the October KUB; plans a non-contrast CT abdomen and pelvis to plan stone treatment; catheter changes continue |
| 2026-02-14 to 2026-02-18 | outside | Urology inpatient admission (discharge summary received 2026-02-19): age 46 (R6); 2 weeks of right flank pain and intermittent fevers; burning on urination and urgency; returns after the June and October 2025 ED visits. Temperature 38.3 °C; right costovertebral angle tenderness. WBC 12,000/mm³ (12.0); UA pH 8.0, leukocyte esterase positive, nitrite positive, numerous WBCs; urine culture sent; creatinine 1.2 mg/dL (baseline 0.9); BUN 18. **Non-contrast CT abdomen and pelvis: large staghorn calculus in the right renal pelvis.** Hospital course (R7, A7): ceftriaxone 1 g IV daily; culture (resulted 2026-02-16) >100,000 CFU/mL Proteus mirabilis with the same susceptibilities as October; fevers settle; catheter exchanged; creatinine 1.0 at discharge; discharged on **ciprofloxacin 500 mg twice daily to complete 14 days** (short course); right percutaneous nephrolithotomy planned |
| 2026-02-24 | practice | post-discharge visit (P): afebrile, completing ciprofloxacin; BMP with creatinine 1.0 |
| 2026-03-10 | practice | preoperative evaluation for PCNL (P): exam normal; CBC, BMP normal (creatinine 0.9); ECG normal sinus rhythm; acceptable risk; latex precautions and TMP-SMX allergy flagged for the surgical team |
| 2026-03-24 | outside | Urology: **right percutaneous nephrolithotomy** with an antegrade **right ureteral stent** placed (R3); overnight stay, discharged 2026-03-25; operative note received. Stone analysis (A8): struvite (magnesium ammonium phosphate) with carbonate apatite |
| 2026-04-07 | outside | Urology office: right ureteral stent removed by cystoscopy (R3) |
| 2026-05-12 | outside | Urology: post-operative non-contrast CT (A9): no residual calculus in the right kidney, no hydronephrosis. Indwelling catheter removed and **back to clean intermittent self-catheterization** (A10); continue oxybutynin, methenamine and tamsulosin |
| 2026-07-03 | practice | last held visit (P): follow-up after the stone surgery; well, cathing again without trouble; BMP (creatinine 0.9); cervical cancer screening due (last co-test 2021 at the prior practice) and done today, normal |
| after 2026-07-03 | — | results portal message, refills; no further practice visit. Booked: annual physical 2026-09-22 |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source lists tamsulosin as a home drug from the first ED visit, but in a
  woman it belongs to the stone story. Resolution: tamsulosin 0.4 mg daily starts at the
  June 2025 ED visit as stone-expulsion therapy and urology keeps it on; the practice's
  lists show it from that visit. The June ED record's own home list does not carry it.
- **R2.** The source says she "has been using an indwelling urinary catheter for the
  past several weeks" at the October 2025 ED visit and gives no prior method. Resolution:
  she self-catheterized intermittently (A2) until urology placed an indwelling catheter on
  2025-09-04, about six weeks before the October visit.
- **R3.** The source lists a right percutaneous nephrolithotomy "2020" and "ureteral stent
  placement/exchanges (multiple)" from the very first encounter, before her stones are
  diagnosed, and then finds an untreated staghorn calculus. Resolution: the PCNL is the
  treatment of that staghorn, on 2026-03-24 (the source's year 2020 is the year of its
  encounters, re-dated). The stent entries are carried as one antegrade stent placed at
  the PCNL and removed on 2026-04-07; "multiple" is not carried. Earlier records (the 2025
  ED records, practice lists before March 2026) never list either procedure.
- **R4.** The October ED urinalysis is "alkaline" with no number. Resolution: recorded as
  pH 8.5, the dipstick's top reading, as in June.
- **R5.** The October ED "kidney x-ray is shown below" has no reading. Resolution: the KUB
  report reads a radiopaque calculus over the right renal shadow (in keeping with the
  struvite crystals and the staghorn found in February), with no other abnormality. The
  image line is not carried.
- **R6.** The source gives age 45 at the February 2026 admission; by the re-dated timeline
  she is 46 (born 1979-12-05). Ages follow the seed: 45 at both ED visits, 46 in February.
- **R7.** The admission note stops mid-sentence with the culture pending. Resolution: the
  discharge summary completes it as in A7.
- The source's "Urolithiasis (diagnosed 2020-01-15)" stamp maps to the June 2025 ED
  diagnosis; urolithiasis is not on any list before it.
- **Left as the source has it:** no imaging at the June 2025 ED visit; methenamine
  continued throughout, including with an indwelling catheter and persistently alkaline
  urine; no blood work recorded at the October ED visit. No one comments on these.

## Additions (A): physician to confirm

- **A1.** 2022-09-13 new patient visit (the source's patient has an established history;
  years with the practice about 4).
- **A2.** Baseline bladder management by clean intermittent self-catheterization with
  latex-free catheters (neurogenic bladder needs a drainage method; the source names only
  the later indwelling catheter).
- **A3.** An outside urology group that follows her neurogenic bladder, with yearly letters
  (2022, 2023, 2024) and letters on 2025-09-04 and 2025-12-09; urology runs the stone course
  in 2026.
- **A4.** One practice-treated UTI on 2024-02-20 with the results above, so that the
  recurrent UTIs and Proteus cultures the source asserts appear in the practice's own
  record. Cephalexin chosen because of the TMP-SMX allergy and the nitrofurantoin
  resistance.
- **A5.** Acetaminophen 500 mg, 1 to 2 tablets every 6 hours as needed, from the June 2025
  ED (short course); renal colic needs an analgesic.
- **A6.** The October 2025 ED vitals, ceftriaxone, catheter exchange, culture and
  ciprofloxacin course as in the timeline (a febrile catheterized patient with flank pain
  is treated).
- **A7.** The February 2026 hospital course as in the timeline (IV ceftriaxone, culture
  result, creatinine 1.0 at discharge, ciprofloxacin to 14 days, PCNL planned).
- **A8.** Stone analysis: struvite (magnesium ammonium phosphate) with carbonate apatite,
  matching the source's alkaline urine, prism-shaped crystals and Proteus.
- **A9.** Post-operative CT 2026-05-12: stone-free, no hydronephrosis.
- **A10.** Return to intermittent self-catheterization after the stone is cleared.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may carry
  the descriptors its kind always carries, but nothing that adds a territory, a severity
  grade or a symptom, or changes the diagnosis or plan.
- Records from a prior practice may mention routine screening done there, with normal
  results (cervical co-test 2021 normal; screening mammogram 2021 normal).
- Creatinine runs 0.85 to 1.0 apart from the February 2026 admission (1.2). BP normal.
  Weight about 67 to 69 kg. Lipids normal.
- Annual physicals each September with CBC, CMP, lipids every year or two, PHQ-2/PHQ-9.
  Hepatitis C and HIV screening once (2022), negative.
- Screening: mammogram yearly (2023-03, 2024-03, 2025-03, 2026-01), all normal; colorectal
  screening by FIT in 2025 after she turned 45, negative; cervical co-test 2026-07-03,
  normal. Lung cancer screening not indicated (10 pack-years).
- Immunizations: influenza each fall, updated COVID each fall, Tdap 2023.
- Refills (oxybutynin, methenamine, docusate, tamsulosin, catheter supplies), pharmacy and
  prior-authorization traffic, portal messages about results, refills and scheduling, a
  cancelled visit.
- Booked visit after 2026-08-27: annual physical 2026-09-22.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002766 on 2026-07-03. Practice contacts after that day do not
happen; outside care after it still does, and its records are filed as received without
changing the practice's lists. A visit is booked after today.
