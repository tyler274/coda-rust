# Build requirements: unit-1896 (Kelly Bergeron, MRN00001896)

Source: synthetic_hospital patient 1896, in `source.md` beside this file. This file and
the source are the only authority for clinical facts. Every diagnosis, medication,
result, procedure and specialist finding in the chart must trace to one of:

- **S** a fact in source.md, carried as listed below;
- **R** a resolution below, where the source contradicts itself or leaves a gap it needs filled;
- **A** an addition below, listed one by one;
- **P** the primary-care layer described at the end.

Nothing else. No diagnosis, medication, test result, procedure, family member or
life event that is not on these lists. Texture (wording, scheduling, refills, portal
traffic, who signed what) is free, as long as it carries no new clinical fact.

Re-voice everything. The source notes are exam vignettes: never copy a sentence, and
never carry a source clinician's name, its "Active Problem List" dumps, its
"(diagnosed 2020-01-15)" stamps, or its "is shown below" image text.

## Who she is (S)

Born 1990-10-13, female, White, not Hispanic or Latino, English-speaking, Blue Cross
Blue Shield PPO. Elementary school teacher. Never smoker. Alcohol occasionally, 1 to 2
drinks a month. No known drug allergies. No surgical history before this chart. Family
history: mother with kidney stones; father with hypertension. No other family history.

Joined the practice in September 2024 (A1: new-patient visit 2024-09-17). She brought
one problem, recurrent urinary tract infections (treated at her prior practice), and no
medications.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2024-09-09 | outside records | prior primary-care records received (P/A2): two cystitis episodes in 2024 (February and May) treated with short antibiotic courses; cervical cytology 2023 negative; no other problems |
| 2024-09-17 | practice | new-patient visit (A1). **Recurrent urinary tract infections carried as a history problem from the prior practice** (S). No medications; NKDA. Normal exam. Height 166 cm, weight 63.5 kg. CBC, CMP, lipids, HIV and hepatitis C screening, all normal. Tdap and influenza vaccines |
| 2024-11-12 | practice (telehealth) | cystitis symptoms (dysuria, frequency) without fever or flank pain (A3); urine dropped off: UA cloudy, pH 7.5, leukocyte esterase positive, nitrite positive, 30 WBC/hpf, no blood; culture sent; **nitrofurantoin 100 mg twice daily for 5 days** started |
| 2024-11-15 | practice (telephone) | culture: Proteus mirabilis >100,000 CFU/mL, resistant to nitrofurantoin, susceptible to cephalexin and trimethoprim-sulfamethoxazole (A3). **Nitrofurantoin stopped; cephalexin 500 mg twice daily for 7 days** |
| 2025-02-18 | practice | cystitis again, cloudy urine (A3); afebrile; no CVA tenderness. UA: cloudy, pH 8.0, leukocyte esterase positive, nitrite positive, 60 WBC/hpf, no blood. **Trimethoprim-sulfamethoxazole 800/160 mg twice daily for 3 days**. Culture: Proteus mirabilis >100,000 CFU/mL, same susceptibilities |
| 2025-06-10 | practice | a few months of intermittent right-sided flank pain with cloudy, foul-smelling urine; infections keep coming back despite treatment; no fever, chills or hematuria (S, as the July urology letter tells it); mild right CVA tenderness (S). UA: cloudy, pH 8.0, leukocyte esterase positive, nitrite positive, 20 WBC/hpf, no blood. Culture Proteus mirabilis again. **Cephalexin 500 mg twice daily for 7 days**. BMP normal. **Urology referral** for recurrent UTIs with flank pain (A4) |
| 2025-07-21 | outside | urology consult letter (received 2025-07-24): history as above; no medications; NKDA; family history as above; vital signs normal; mild right CVA tenderness. Abdominal x-ray (KUB, R1): a 7 mm radiopaque calculus over the lower pole of the right kidney; no other calculi. **Kidney stone diagnosed** (right renal calculus). Urologist notes the alkaline urine and repeated Proteus cultures suggest an infection-related stone. Plan (R2): fluids at least 2.5 L a day, culture with any symptoms, elective ureteroscopy discussed and she will decide, urology review in 6 months |
| 2025-10-07 | practice (nurse visit) | influenza and COVID vaccines (P) |
| 2026-01-12 | outside | regional hospital ED record (received 2026-01-14): sudden severe right flank pain radiating to the groin, nausea, visible hematuria. Temperature 37.8 °C (100 °F), BP 130/85, HR 98 (S). Urinalysis alkaline (pH 8.0) with numerous white cells (S; reported >100 WBC/hpf), cloudy, leukocyte esterase positive, nitrite positive, large blood, 75 RBC/hpf. CBC and BMP normal, urine pregnancy test negative (R3). Non-contrast CT abdomen and pelvis (R1): 7 mm calculus in the proximal right ureter with mild right hydronephrosis and perinephric stranding; no calculus left in the right kidney; left kidney normal. **Urinary tract infection with obstructing right ureteral stone** (S comorbidity). Ceftriaxone 1 g IV, IV ketorolac and ondansetron in the ED; urology placed a **right ureteral stent** by cystoscopy that evening (A5); observed overnight, discharged 2026-01-13 on **cephalexin 500 mg four times daily for 10 days**, oxycodone/acetaminophen 5/325 mg as needed (8 tablets) and ondansetron 4 mg ODT as needed. Urine culture (final 2026-01-14): Proteus mirabilis >100,000 CFU/mL, susceptible to ceftriaxone and cephalexin |
| 2026-01-20 | practice | post-ED visit (P): afebrile, pain controlled, mild stent discomfort, finishing cephalexin. BP normal. BMP normal |
| 2026-02-03 | outside | urology: right ureteroscopy with laser lithotripsy and basket extraction, stent exchanged for a stent on a string, removed one week later (A5); operative note received 2026-02-05. Pre-procedure urine culture no growth |
| 2026-02-12 | outside | stone analysis (A6): magnesium ammonium phosphate (struvite) 80%, carbonate apatite 20% |
| 2026-03-16 | outside | urology follow-up letter (A7): no symptoms; renal ultrasound with no stones and no hydronephrosis in either kidney; urine culture no growth. Plan: fluids, culture with any urinary symptoms, renal ultrasound in 6 months |
| 2026-06-09 | practice | annual physical cancelled by the patient (end-of-year school week) and rebooked (P) |
| 2026-06-30 | practice | annual physical (P), last seen. No UTIs since January. **Kidney stone marked resolved** (stone-free on March ultrasound, R4); recurrent UTIs stay on the list. CBC, CMP, lipids, urinalysis normal. Cervical cytology (due 3 years after 2023) negative, released 2026-07-06 |
| after 2026-08-27 | practice | booked follow-up visit (P) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Both source images are "shown below" with no findings, yet the July visit
  diagnoses urolithiasis and the ED visit is classic renal colic. Resolution: the July
  x-ray is a KUB showing a single 7 mm radiopaque right lower-pole renal calculus; the
  January CT shows that stone (7 mm) in the proximal right ureter with mild hydronephrosis
  and no stone left in the kidney. The alkaline urine with numerous white cells and the
  repeated urease-producing organism make it an infection stone, confirmed by analysis (A6).
- **R2.** The urology note stops after the image with no plan. Resolution: the plan
  listed in the timeline. Observation of a struvite stone in a patient with recurrent
  Proteus infections is not the best choice; the chart keeps it and nobody remarks on it.
- **R3.** The source gives the ED only vital signs and a urinalysis. Resolution: the ED
  CBC and BMP are normal (no leukocytosis, normal creatinine); the culture result is A3's
  organism. The low-grade temperature of 37.8 °C is the only systemic sign.
- **R4.** The source carries urolithiasis as active. After clearance (A5, A7) the practice
  marks the kidney stone resolved at the June 2026 physical; recurrent UTIs stay active.
- The source dates urolithiasis to the first urology visit; the chart diagnoses it there
  (2025-07-21), never earlier. Recurrent UTIs, on the profile from the start, are known
  at joining.
- **Left as the source has it:** no one orders imaging for the flank pain before urology,
  no one starts antibiotic prophylaxis, and the urologist observes the renal stone rather
  than removing it in July. No note comments on these.

## Additions (A): physician to confirm

- **A1.** The September 2024 new-patient visit (the urology letter says infections were
  treated over the past year, so she had a primary-care physician who treated them).
- **A2.** Prior-practice records: cystitis February and May 2024, short antibiotic
  courses; cervical cytology 2023 negative.
- **A3.** The specific episodes behind "multiple episodes ... treated with antibiotics but
  never fully resolving": November 2024, February 2025 and June 2025, each with the culture
  given (Proteus mirabilis, nitrofurantoin-resistant), and the drugs given. The first
  empiric nitrofurantoin choice and its switch explain part of "never fully resolving".
- **A4.** The June 2025 practice visit that refers her to urology.
- **A5.** The next steps of an obstructing ureteral stone with infection: ureteral stent
  the night of the ED visit, ureteroscopy with laser lithotripsy three weeks later (cefazolin 2 g IV as surgical prophylaxis), stent
  removed a week after that.
- **A6.** Stone analysis: struvite 80%, carbonate apatite 20%.
- **A7.** March 2026 urology follow-up: stone-free on renal ultrasound, culture no growth.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention
  (kidney size, bladder, bowel gas, bones, appendix, pelvic organs). An abnormal finding
  appears only as the source or this file gives it. A report may carry the descriptors its
  kind always carries (how a size was measured, a stone's density), but nothing that adds
  a territory, a severity grade or a symptom, or changes the diagnosis or plan.
- Records from a prior practice may mention routine screening done there, with normal results.
- Vitals at each visit; BP normal throughout; weight steady around 63 to 65 kg.
- Yearly physical-type labs as above; urinalysis and culture with UTI visits.
- Prevention: influenza each fall, COVID vaccine yearly, Tdap once (2024), HIV and
  hepatitis C screening once (2024), cervical cytology 2026, PHQ-2 screens negative.
- Refills, portal messages about results and scheduling, records requests, a cancelled
  visit, reminders.
- Booked visit after 2026-08-27: a routine follow-up.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00001896 on 2026-06-30. Practice contacts after that day do not
happen beyond messages and results of the draw ordered that day; outside care after it
still does, and its records are filed as received without changing the practice's lists.
A visit is booked after today.
