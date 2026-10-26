# Build requirements: unit-2796 (Robin Goldberg, MRN00002796)

Source: synthetic_hospital patient 2796, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its "shown below" image text.

## Who she is (S)

Born 1980-07-04, female, White, not Hispanic or Latino, English, commercial PPO, lives
in Florence. Elementary school teacher. Never smoker, no alcohol. No known drug
allergies. No surgical history before the practice. Family history: father with
kidney stones, mother with hypertension. No other family history.

Joined the practice on 2023-09-12 (A1), a new-patient visit at age 43. She brought
two problems: recurrent kidney stones (earlier stones passed on her own, no
procedures) and recurrent urinary tract infections, each treated with short antibiotic
courses at urgent care clinics. No medications on joining. Records: complete; every
outside letter, report and summary below reaches the practice and is filed.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2023-09-12 | practice | new-patient visit (A1): history as above; weight 68 kg, height 165 cm (P); normal exam; screening labs (P) |
| 2023-09-20 | outside | prior primary-care records received (P): the two problems above, routine screening normal |
| 2024-04-11 | outside | ED visit: several months of intermittent left flank pain and tiredness, occasional low-grade fevers, malaise, cloudy foul-smelling urine; no dysuria, no visible blood. Afebrile, vital signs normal. Mild left costovertebral angle tenderness. Urinalysis: pH 8.5, numerous white cells (reported >50/hpf), cloudy, leukocyte esterase 2+, nitrite positive, blood negative (A2). Urine culture sent, pending at discharge. Serum creatinine 1.0 mg/dL. Non-contrast CT abdomen and pelvis and a plain abdominal radiograph (A2 findings). **Urolithiasis (renal colic) diagnosed.** IV fluids (normal saline 1 L) and pain control (IV ketorolac 15 mg); discharged home to follow up with the practice and urology. No antibiotic started in the ED |
| 2024-04-12 | practice | ED record and imaging reports received (P) |
| 2024-04-13 | practice | urine culture result (A3): Klebsiella pneumoniae >100,000 CFU/mL, susceptible to ciprofloxacin, ceftriaxone and trimethoprim-sulfamethoxazole, resistant to ampicillin. Covering PCP phones her: **ciprofloxacin 500 mg twice daily for 7 days** (A3) |
| 2024-04-18 | practice | ED follow-up (P): fevers gone, flank ache milder; BMP; **urology referral** (A4) |
| 2024-05-21 | outside | urology consult letter (A4): 14 mm left renal pelvic stone, alkaline urine and a urease-producing organism suggest an infection stone; plan ureteroscopy with laser lithotripsy |
| 2024-06-11 | outside | ureteroscopy with laser lithotripsy, left, with ureteral stent (A4); stent removed at urology 2024-06-18 |
| 2024-06-25 | outside | stone analysis (A4): magnesium ammonium phosphate (struvite) 70%, carbonate apatite 30% |
| 2024-08-06 | outside | urology follow-up with renal ultrasound (A4): no residual calculus, no hydronephrosis either kidney; yearly ultrasound |
| 2025-02-11 | practice | same-day visit (A5): two days of urinary frequency and burning, no fever, no flank pain; urinalysis leukocyte esterase 2+, nitrite positive; **acute cystitis diagnosed**; **nitrofurantoin 100 mg twice daily for 5 days**; urine culture 2025-02-13: Klebsiella pneumoniae >100,000 CFU/mL, susceptible to nitrofurantoin and ciprofloxacin; cystitis resolved at follow-up call 2025-02-18 |
| 2025-06-10 | outside | urology yearly follow-up with renal ultrasound (A4): no calculus, no hydronephrosis |
| 2026-04-13 | practice | evening phone call (P): fever, shaking chills, left flank pain since that morning; told to go to the ED now |
| 2026-04-14 | outside | admitted through the ED to the ICU (A6): **septic shock from a complicated urinary tract infection**. Temperature 39.2 C, BP 82/48 before fluids, HR 124; lactate 4.2 mmol/L, WBC 19.6, creatinine 1.3. Non-contrast CT abdomen and pelvis: no hydronephrosis, no urinary tract calculus. Right internal jugular central venous catheter placed; norepinephrine for about 48 hours; IV ceftriaxone through the catheter. Blood and urine cultures: Klebsiella pneumoniae, susceptible to ceftriaxone and ciprofloxacin. Lactate 1.4 by 2026-04-16 |
| 2026-04-18 | outside | ICU, age 45: right after the central venous catheter is removed, sudden shortness of breath and low oxygen. RR 28, SpO2 88% on room air, BP 110/70, HR 110. Anxious; diffuse wheezes both lungs. Bedside transthoracic echo: no intracardiac shunt; a hyperechoic area in the right ventricle suggesting air. **Venous air embolism from the catheter (iatrogenic vascular injury) diagnosed.** Management (A7) |
| 2026-04-21 | outside | discharged (A6): WBC 8.4, creatinine 0.9; **ciprofloxacin 500 mg twice daily through 2026-04-25** to finish 14 days; discharge summary received 2026-04-22 |
| 2026-04-24 | practice | post-discharge visit (P): well, no shortness of breath, lungs clear, SpO2 normal; CBC and BMP normal |
| 2026-06-09 | outside | urology follow-up with renal ultrasound (A4): no calculus, no hydronephrosis |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Encounter 1 says she "is brought to the emergency department" yet the event
  follows catheter removal in the ICU. Resolution: it happens at the ICU bedside during
  her septic-shock admission; there is no ED visit for it.
- **R2.** The source stamps urolithiasis "diagnosed 2020-01-15" and calls the ED visit
  "2020 flank pain". Resolution: both follow the re-dated timeline (2024-04-11).
- **R3.** "History of septic shock" is a comorbidity, and encounter 1 says she was being
  treated for septic shock through the catheter. Resolution: the septic shock is the
  2026-04-14 admission (A6), the same stay as the air embolism; it is recorded as
  diagnosed on admission and resolved at discharge, then carried as history.
- **R4.** The ED imaging is "shown below" with no finding stated. Resolution: the CT
  and radiograph reports carry the findings in A2; no image text is carried.
- **R5.** The ED note gives "normal vital signs" with no numbers. Resolution: an ordinary
  normal set, afebrile.
- **Left as the source has it:** the ED starts no antibiotic despite pyuria and fevers
  (the practice starts one on the culture). No one starts urinary-infection prophylaxis,
  no metabolic stone workup (24-hour urine) is done, and no stone-preventive drug is
  started. No note comments on these.

## Additions (A): physician to confirm

- **A1.** 2023-09-12 new-patient visit (the seed gives about three years with the
  practice; the first source encounter is an ED visit).
- **A2.** ED findings the source implies but does not state: urinalysis cloudy,
  leukocyte esterase 2+, nitrite positive, blood negative, urine red cells 0 to 2/hpf.
  Non-contrast CT: a 14 mm calculus in the left renal pelvis, no hydronephrosis, no
  ureteral calculus, right kidney normal, no perinephric stranding. Abdominal
  radiograph: a radiopaque calculus over the left renal shadow matching the CT.
- **A3.** The pending urine culture's result (above) and the ciprofloxacin course the
  practice starts on it (the source names no treatment of the infection).
- **A4.** The usual course of a 14 mm renal pelvic stone: urology consult, left
  ureteroscopy with laser lithotripsy and a temporary stent, stone analysis (struvite
  70%, carbonate apatite 30%), and yearly renal ultrasounds, each normal as given.
- **A5.** One cystitis episode in 2025 with its culture and nitrofurantoin course, to
  carry the source's recurrent urinary tract infections in the practice years.
- **A6.** The septic-shock admission the source names (R3), with its numbers, CT,
  cultures, catheter, ceftriaxone, norepinephrine and the oral ciprofloxacin finish.
- **A7.** Air embolism management: 100% oxygen by non-rebreather, left side down with
  the head lowered, and the line site occluded; she stays hemodynamically stable; SpO2
  96% on 2 L nasal cannula within 2 hours and room air the next day; wheezes clear;
  repeat transthoracic echo 2026-04-19 shows no intracardiac air and normal function
  of both ventricles. The embolism is resolved by discharge; no lasting effects.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention
  (kidney size, bladder, bowel, heart sounds). An abnormal finding appears only as the
  source or this file gives it. A report may carry the descriptors its kind always
  carries (how a stone was measured, its density, a procedure's access), but nothing
  that adds a territory, a severity grade or a symptom, or changes the diagnosis or plan.
- Records from the prior practice may mention routine screening done there, normal.
- Yearly physicals (September), with CMP, CBC, lipids, urinalysis; depression screens.
- Prevention at 43 to 46: screening mammogram yearly, normal; cervical cytology with
  HPV in 2025, normal; colorectal screening from 45 by FIT, negative; HIV and hepatitis
  C screening once, nonreactive; influenza vaccine each fall; COVID vaccine each fall;
  Tdap if due.
- Hydration advice for stones; refills of nothing chronic; portal messages about
  results, school forms and scheduling; a cancelled or missed visit; reminders.
- Booked visit after 2026-08-27: her yearly physical in September 2026.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.
