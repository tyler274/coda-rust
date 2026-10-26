# Build requirements: unit-2143 (Lloyd Jacobs, MRN00002143)

Source: synthetic_hospital patient 2143, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its copied home-medication lists.

## Who he is (S)

Born 1968-10-26, male, White, not Hispanic or Latino, English, private insurance,
lives in Easthampton. Financial analyst with no known occupational exposures; no
recent travel and no tuberculosis contact. Lifelong nonsmoker; social alcohol, 1 to 2
drinks a week. No known drug allergies. Family history: father with interstitial lung
disease, mother with rheumatoid arthritis. No surgical history before the 2026
transplant (R1). Divorced; no other people appear in the chart.

Joined the practice on 2023-09-12 (A1, new-patient visit), healthy at 54: no
problems, no regular medications. Height 180 cm, weight 84 kg at joining.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2023-09-12 | practice | new-patient visit (A1); healthy; no medications; NKDA; screening labs (P) |
| 2023-09-21 | outside records | prior primary care records arrive by fax: screening colonoscopy 2019 normal, Tdap 2019 (P) |
| 2024-06-11 | practice | annual physical (P): well, no breathing complaints; Shingrix dose 1 |
| 2024-12-10 | practice | referral visit (A2): about 5 months of breathlessness on a single flight of stairs and a dry cough that over-the-counter remedies have not helped; no wheeze, sputum, chest pain or leg swelling; SpO2 95% on room air at rest; fine bibasilar crackles; no edema; CBC and BMP normal. Chest X-ray ordered and **pulmonary referral placed** |
| 2024-12-12 | practice (imaging) | chest X-ray, 2 views: subtle interstitial pattern, predominantly at the lung bases (S, moved here by R2) |
| 2025-01-23 | outside: pulmonary consult | 6 months of exertional dyspnea and dry cough unhelped by OTC treatment; no wheeze, sputum, chest pain or leg swelling. Exam: fine bibasilar inspiratory crackles, no wheeze, no fluid overload, normal cardiac exam, no peripheral edema. SpO2 94% on room air at rest, 88% with exertion on a 6-minute walk test. High-resolution CT chest: reticulonodular opacities with peripheral and basal predominance and subpleural honeycombing. Pulmonary function tests: reduced total lung capacity, no airway obstruction (values A3). ANA, rheumatoid factor and anti-CCP negative (A4). **Idiopathic pulmonary fibrosis diagnosed.** Letter received 2025-01-29 |
| 2025-01-29 | practice | consult letter filed; **IPF added to the problem list** |
| 2025-02-27 | outside: pulmonary follow-up | **nintedanib 150 mg twice daily started** (A5) and **ambulatory oxygen 2 L/min with exertion** (A6); liver tests to be followed by pulmonary. Letter received 2025-03-04 |
| 2025-04-16 | practice | IPF follow-up (P); on nintedanib (recorded as an outside medication) and exertional oxygen; SpO2 94% at rest; crackles as before; CMP normal; PCV20 given |
| 2025-06-24 | practice | follow-up visit cancelled by patient, rescheduled (P) |
| 2025-07-24 | outside: pulmonary follow-up | PFT: FVC 61% predicted, DLCO 44% predicted (A3); SpO2 93% at rest; nintedanib continued; **referred to a lung transplant center for evaluation** (A7). Letter received 2025-07-29 |
| 2025-09-18 | outside: transplant center | transplant evaluation started (A7); asks the practice to confirm age-appropriate screening and vaccines. Letter received 2025-09-23 |
| 2025-10-31 | practice | **last seen**: IPF follow-up; screening and vaccine summary sent to the transplant center; influenza and updated COVID vaccines; CBC, CMP, lipids, A1c normal; SpO2 93% at rest. No change to lists |
| 2025-12-11 | outside: transplant center | evaluation complete: FVC 55% predicted, DLCO 38% predicted (A3); accepted as a candidate for bilateral lung transplantation (A7). Letter received 2025-12-16 |
| 2026-01-22 | outside: transplant center | **listed for bilateral lung transplantation** (A7). Letter received 2026-01-27 |
| 2026-05-19 to 2026-06-02 | outside: transplant center admission | **bilateral lung transplantation** 2026-05-19 (S, R1); uneventful postoperative course; **nintedanib stopped**; discharged on **tacrolimus 2 mg twice daily (goal trough 8-12 ng/mL), mycophenolate mofetil 750 mg twice daily, prednisone 5 mg daily** (S). Discharge summary received 2026-06-05 |
| 2026-06-09 | practice (portal) | patient writes that he is home and the transplant team is running his medicines; PCP replies (P) |
| 2026-07-20 | outside: transplant pulmonary clinic | returns for follow-up 2 months after transplant; 2 weeks of progressive dyspnea, nonproductive cough and fever; no sick contacts or travel. T 38.7 °C (101.7 °F), BP 130/85, HR 102, RR 26, SpO2 89% on room air; bilateral crackles. WBC 9.0 K/uL, Hgb 12.5 g/dL, platelets 250 K/uL, creatinine 1.2 mg/dL, BUN 21 mg/dL, liver enzymes normal, CMV PCR positive. Chest X-ray: diffuse interstitial infiltrates. **Cytomegalovirus pneumonitis diagnosed**; admitted the same day (A8) |
| 2026-07-20 to 2026-07-27 | outside: transplant center admission | **IV ganciclovir 5 mg/kg every 12 hours** (A8); afebrile with SpO2 94% on room air at discharge; **valganciclovir 900 mg twice daily** started at discharge; immunosuppression unchanged. Discharge summary received 2026-07-30 |
| 2026-08-13 | outside: transplant clinic | afebrile, SpO2 95% on room air; CMV PCR detected below the limit of quantitation (A8); valganciclovir 900 mg twice daily continued with weekly CMV PCR. Letter received 2026-08-18 |
| 2026-08-20 | practice (portal) | patient asks about this fall's flu shot; reply: inactivated vaccine is fine, timing to be cleared with the transplant team (P) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Encounter 0 (2025-01-23) is the visit that diagnoses IPF, yet the source lists a
  bilateral lung transplant and tacrolimus, mycophenolate and prednisone there. Encounter 1
  (2026-07-20) says the transplant was 2 months earlier and that he was discharged on those
  three drugs. Resolution: the transplant is on 2026-05-19, and the three drugs start at the
  transplant discharge. No note before May 2026 mentions a transplant or immunosuppression.
  The 2025-01-23 letter lists no medications.
- **R2.** The source places the chest X-ray in encounter 0 with the CT and PFTs. Resolution:
  the practice orders the X-ray at its referral visit (A2) and it is done 2024-12-12; the
  pulmonary letter reviews it. Same finding, worded the same way.
- **R3.** Encounter 1's problem list stamps IPF as diagnosed at encounter 0; that date is
  2025-01-23 in the practice timeline. The stamp itself is not carried.
- **R4.** Encounter 1 says "returns for IPF follow-up" while describing an acute febrile
  illness. Resolution: it is a scheduled transplant clinic visit, moved up by his symptoms.
- **R5.** The source lists IPF as a problem after the transplant. Resolution: the transplant
  center's letters call it "IPF, status post bilateral lung transplant"; the practice list,
  frozen at 2025-10-31, keeps "Idiopathic pulmonary fibrosis".
- **Left as the source has it:** the discharge regimen names only tacrolimus, mycophenolate
  and prednisone (no antimicrobial prophylaxis is named), and prednisone is 5 mg daily at
  discharge. No letter adds or comments on prophylaxis. Hepatitis B immunity is never
  checked. The practice lists are not updated after 2025-10-31: the transplant, the three
  immunosuppressants, valganciclovir and CMV pneumonitis appear only in the outside records.

## Additions (A): physician to confirm

- **A1.** 2023-09-12 new-patient visit (three years with the practice before today).
- **A2.** 2024-12-10 practice visit that hears the crackles, orders the X-ray and refers to
  pulmonary (the source's 6-month history implies someone sent him).
- **A3.** PFT values (the source says only "TLC reduced, no obstruction"): 2025-01-23 TLC 66%
  predicted, FVC 68% predicted, FEV1/FVC 0.83, DLCO 51% predicted; 2025-07-24 FVC 61%, DLCO
  44%; 2025-12-11 FVC 55%, DLCO 38%. The decline supports transplant listing within a year.
- **A4.** ANA, rheumatoid factor and anti-CCP negative at the pulmonary consult (mother's
  rheumatoid arthritis; connective tissue disease excluded before calling it IPF).
- **A5.** Nintedanib 150 mg twice daily, the standard antifibrotic for IPF, run by
  pulmonary from 2025-02-27 until the transplant.
- **A6.** Ambulatory oxygen 2 L/min with exertion, for the 88% exertional saturation.
- **A7.** Transplant referral (2025-07), evaluation (2025-09 to 2025-12) and listing
  (2026-01), the steps that lead to the source's transplant.
- **A8.** CMV pneumonitis treatment after the source ends: same-day admission from the
  transplant clinic, IV ganciclovir 5 mg/kg every 12 hours, discharge 2026-07-27 afebrile
  with SpO2 94% on room air on valganciclovir 900 mg twice daily; 2026-08-13 SpO2 95%, CMV
  PCR detected below the limit of quantitation. Treatment continues today.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Records from the prior practice may mention routine screening done there, with normal
  results (colonoscopy 2019 normal, repeat in 10 years; Tdap 2019).
- Yearly labs at physicals: CBC, CMP, lipids, A1c, all normal. Hepatitis C and HIV
  screening once, negative. PHQ-2 negative.
- Influenza each fall, updated COVID vaccines, Shingrix two doses (2024), PCV20 once (2025).
- Portal messages about results and scheduling, a cancelled visit, records requests and
  forms for the transplant center.
- No booked visit after 2026-08-27.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002143 on 2025-10-31. Practice visits after that day do not
happen; outside care after it still does, and its records are filed as received without
changing the practice's lists. Nothing is booked.
