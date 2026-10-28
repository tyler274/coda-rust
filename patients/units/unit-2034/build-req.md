# Build requirements: unit-2034 (Ousmane Diallo, MRN00002034)

Source: synthetic_hospital patient 2034, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its unfinished plans.

## Who he is (S)

Ousmane Diallo, born 1965-06-15, male, Black or African American, not Hispanic,
English-speaking, lives in South Hadley MA 01075. Medicare (ESRD entitlement).
Married; his wife brings him to the hospital in October 2023 (S, encounter 6).
Retired postal worker. Former smoker, quit in 2016 ("quit 5 years ago" as of 2021).
No alcohol. Avid fisherman: competed in regional bass tournaments in his 20s and 30s,
still fishes two or three times a week (S, encounter 0; texture only). NKDA.

Family history (S, the list): father with hypertension and chronic kidney disease;
sister with bipolar disorder; maternal uncle with end-stage renal disease.

Surgical and renal history (S as resolved in R2, R3): end-stage renal disease from
hypertensive nephrosclerosis, on hemodialysis before a first deceased-donor kidney
transplant in 2016. That graft had several rejection episodes; one, in 2017, was
treated with high-dose cyclophosphamide, which caused hemorrhagic cystitis (severe
hematuria and suprapubic discomfort) treated with aggressive hydration and mesna,
after which he finished the cyclophosphamide without recurrence. The first graft
failed in early 2019 and he went back on in-center hemodialysis (A1). Deceased-donor
re-transplant 2020-12-03 (S: "6 months prior" to 2021-06-04).

Bipolar disorder type I, stable on lithium for about ten years by 2021 (lithium since
2011), managed by an outside psychiatrist (A4).

**Joined the practice 2019-09-17** (A1: new-patient visit; the first source encounter
is a "routine follow-up"). On joining he was on hemodialysis three times weekly via a
left arm fistula and brought: essential hypertension, end-stage renal disease on
dialysis, kidney transplant status (failed 2016 graft left in place), bipolar disorder
type I, former smoker, history of cyclophosphamide hemorrhagic cystitis (resolved).
Medications on joining: prednisone 5 mg daily (kept on after graft failure, transplant
team), lithium carbonate 300 mg after each dialysis session (psychiatry, A4),
amlodipine 10 mg daily. Height 180 cm, weight 86 kg.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2019-09-17 | practice | new-patient visit (A1); history as above; BP 138/86 on amlodipine; dialysis unit draws his monthly labs |
| 2020-12-03 to 2020-12-10 | outside | transplant center admission, **deceased-donor kidney re-transplant** (A2); discharge summary received 2020-12-14. Dialysis stops. **Start tacrolimus 5 mg twice daily, mycophenolate mofetil 750 mg twice daily, trimethoprim-sulfamethoxazole single strength (400/80 mg) daily, valganciclovir 450 mg daily for 6 months**; prednisone back to 5 mg daily after a short taper; **lithium back to 300 mg twice daily** (psychiatry, A4). Discharge creatinine 1.4 |
| 2021-02-16 | outside | transplant clinic letter: BP up after transplant, **hydrochlorothiazide 25 mg daily started** (A3); creatinine 1.2 |
| 2021-06-04 | outside | transplant clinic routine follow-up (letter received 2021-06-10): adherent to immunosuppressants and prophylactic antibiotic; no hematuria, dysuria, frequency, fevers, chills or night sweats; BP 128/82, pulse 76, RR 14; healthy-appearing, normal heart and lungs, well-healed scar, no tenderness or organomegaly, no edema, nonfocal neuro; creatinine (1.2), electrolytes, CBC and urinalysis normal. **Valganciclovir stopped** (6-month course complete, A2) |
| 2021-08-13 | practice | 1 week of thirst and frequent urination (P referral visit); glucose normal; referred to nephrology |
| 2021-08-23 | outside | nephrology consult (letter received 2021-08-30): two weeks of excessive urination and thirst, drinking more than 4 liters a day and still feels dry; lithium for ten years with hydrochlorothiazide; vitals normal, exam normal; serum sodium 145, serum osmolality 300, urine osmolality 150, urine specific gravity 1.005; no meaningful rise in urine osmolality (to 160) after desmopressin (A5). **Nephrogenic diabetes insipidus from lithium.** Plan: free access to water; lithium continued after discussion with his psychiatrist (left as the source has it) |
| 2021-12-08 | outside | transplant clinic (letter received 2021-12-13): 2 weeks of hand tremor, more thirst and frequent urination, like the August episode; BP 150/95; fine tremor both hands; creatinine 2.1, BUN 35, fasting glucose 160, tacrolimus level 15 ng/mL (within the lab's range); lithium level 0.8 mmol/L (A6). **Calcineurin inhibitor nephrotoxicity. Tacrolimus reduced to 3 mg twice daily** (A6) |
| 2022-01-11 | practice | follow-up: tremor gone; BP 134/84; creatinine 1.2; A1c 5.5 (A7); PCV20 and first Shingrix |
| 2022-03-29 | practice | BP rising over about two months, 154/94 today; creatinine 1.6 (A8); PCP messages transplant, who move his visit up (P referral contact) |
| 2022-04-21 | outside | transplant clinic (letter received 2022-04-25): three months of worsening blood pressure despite hydrochlorothiazide and amlodipine; less urine; creatinine up from 1.2 to 1.9 over two months; BP 162/98; 3/6 systolic bruit loudest over the right upper quadrant of the abdomen, rest of exam unremarkable; creatinine 1.9, BUN 36, potassium 4.8, hemoglobin 13.2; urinalysis without protein or blood. Impression: **acute rejection of the kidney allograft suspected** (R4); biopsy arranged |
| 2022-04-26 | outside | allograft biopsy (A9): acute T-cell-mediated rejection, Banff grade IA. **Methylprednisolone 500 mg IV daily for 3 days** (2022-04-27 to 2022-04-29), then prednisone 5 mg daily as before. Transplant letter received 2022-05-02. **Kidney transplant rejection diagnosed** |
| 2022-05-10 | outside | transplant lab: creatinine 1.4 (A9) |
| 2022-06-14 | practice | initial annual wellness visit; BP 132/82; creatinine 1.2 (A9); rejection treated; second Shingrix |
| 2022-10-18 to 2022-10-24 | outside | ED, then admission (records received 2022-10-26): a week of new dry cough, shortness of breath on exertion and low-grade fever; no travel or sick contacts; T 37.8 C, BP 135/85, pulse 88, RR 20, SpO2 94% on room air; bibasilar crackles; WBC 6,500, hemoglobin 12.5, platelets 250,000, creatinine 1.2, BUN 21, LDH 220 (elevated); arterial gas on room air pH 7.45, PaCO2 35, PaO2 72; chest x-ray with diffuse interstitial infiltrates. Induced sputum Pneumocystis jirovecii PCR positive (A10). **Pneumocystis pneumonia**, treated with **trimethoprim-sulfamethoxazole 800/160 mg, 2 tablets three times daily for 21 days** (A10), single-strength prophylaxis held during treatment; no steroids (PaO2 above 70) |
| 2022-10-28 | practice | post-discharge visit (P); SpO2 97% |
| 2022-11-08 | practice | telephone: treatment course finished; **back on single-strength daily** (A10); pneumonia resolved |
| 2023-04-17 to 2023-05-01 | outside | ED, then admission (records received 2023-05-03): 10 days of worsening cough, fever and weight loss; a week of dull headaches and some trouble concentrating; no recent travel or sick contacts; T 38.4 C (101.1 F), RR mildly elevated; crackles at the right upper lobe; neuro exam nonfocal; sputum Gram stain shows Nocardia; chest x-ray with a cavitary right upper lobe lesion; brain MRI with a solitary ring-enhancing frontal lobe lesion. Sputum culture grows Nocardia species (A11). **Nocardiosis, pulmonary and brain** (R5). Treated with IV trimethoprim-sulfamethoxazole and imipenem for 2 weeks, then **oral trimethoprim-sulfamethoxazole 800/160 mg, 2 tablets twice daily, to complete 12 months** (single-strength stopped) (A11); infectious disease follow-up |
| 2023-05-05 | practice | post-discharge visit (P); weight 81 kg (down from 86) |
| 2023-07-18 | outside | infectious disease letter (A11): repeat brain MRI, frontal lesion smaller; cough gone |
| 2023-10-15 to 2023-10-31 | outside | ED and ICU, then floor (records received 2023-11-02): age 58, brought in by his wife with a day of fever and neck pain; T 103 F. Lumbar puncture opening pressure 31 cm H2O; CSF cryptococcal antigen positive, titer 1:640; CSF fungal culture Cryptococcus neoformans; serum cryptococcal antigen positive (A12). **Cryptococcal meningitis.** Induction with liposomal amphotericin B and flucytosine for 14 days, repeat lumbar punctures for pressure (A12); discharged on **fluconazole 400 mg daily for 8 weeks**, **tacrolimus reduced to 2 mg twice daily** for the fluconazole interaction (A12); nocardia course continues |
| 2023-11-07 | practice | post-discharge visit (P) |
| 2023-12-28 | outside | infectious disease letter (A12): consolidation done, **fluconazole 200 mg daily maintenance for 12 months** |
| 2024-05-07 | outside | infectious disease letter (A11): 12-month nocardia course complete 2024-04-30; repeat brain MRI, lesion resolved; **back to single-strength trimethoprim-sulfamethoxazole daily** |
| 2025-01-07 | outside | transplant letter (A12): **fluconazole stopped 2024-12-27**; **tacrolimus back to 3 mg twice daily**; cryptococcal meningitis treated |
| 2024-02-13, 2025-06-11 | practice | creatinine 1.3 in February 2024 and 1.4 in June 2025 (A13) |
| **2025-06-11** | practice | **last seen**: subsequent annual wellness visit and chronic care; BP 136/86; creatinine 1.4; no new complaints |
| 2026-06-15 | outside | transplant clinic (letter received 2026-06-22, filed): about 8 months of progressive fatigue, less exercise tolerance and ankle swelling; blood pressure harder to control on his antihypertensives; admits occasional missed doses; ROS negative for fever, dysuria, hematuria, abdominal pain; creatinine 2.6, urine protein/creatinine 0.6 g/g (mild proteinuria) (A14); BP 158/96, ankle edema both sides (A14); allograft ultrasound 2026-05-28 asymmetrically small with loss of corticomedullary differentiation; allograft biopsy 2026-06-04 with fibrous intimal thickening of vessels, mononuclear infiltrates and interstitial fibrosis. **Chronic rejection of the kidney allograft**; function declining despite optimized immunosuppression. Plan in the letter (A14): adherence plan, same immunosuppression, furosemide 40 mg daily for the swelling, vascular surgery referral for dialysis access, re-transplant evaluation started. Practice lists are not updated |
| 2026-08-24 | practice | portal message from patient about the June letter, asking to talk with the PCP; RN replies the PCP will call; open today |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source lists cyclosporine and tacrolimus together (two calcineurin
  inhibitors). Every prose passage and the tacrolimus level say tacrolimus.
  Resolution: tacrolimus is his calcineurin inhibitor; cyclosporine is not carried
  anywhere.
- **R2.** Transplant timing: "initial 5 years prior" and "re-transplant 6 months prior"
  at 2021-06-04, then "18 months ago" (April 2022), "2 months after receiving a kidney
  transplant" (October 2022) and "6 years ago" (October 2023). Resolution: first graft
  2016, re-transplant 2020-12-03; the other intervals are not carried. A re-transplant
  means the first graft failed: it failed in early 2019 and he returned to
  hemodialysis until the re-transplant (A1). The source's "ESRD on hemodialysis"
  comorbidity is that interval and the years before 2016.
- **R3.** The source labels a problem "interstitial cystitis, diagnosed 2020-01-15",
  while its own narrative describes cyclophosphamide hemorrhagic cystitis years
  earlier. Resolution: history of cyclophosphamide-induced hemorrhagic cystitis
  (2017), resolved, recorded at joining. No interstitial cystitis.
- **R4.** "Acute lung transplant rejection": he has no lung transplant. The encounter
  is a kidney recipient with rising creatinine, falling urine output and worsening
  pressure. Resolution: suspected acute rejection of the kidney allograft, confirmed
  by biopsy (A9). No lung disease of that kind anywhere.
- **R5.** "Pelvic actinomycosis": the encounter's own Gram stain shows Nocardia, with a
  cavitary lung lesion and a ring-enhancing brain lesion and nothing pelvic.
  Resolution: nocardiosis (lung and brain). The October 2023 history says "prior
  nocardiosis", not pelvic actinomycosis.
- **R6.** Problem lists carry every later diagnosis with "diagnosed" stamps, and the
  profile lists everything at once. Resolution: each condition starts at its own
  encounter; nothing appears in a note before then.
- **R7.** Family history: encounter 0 says hypertension in both parents; every later
  note gives the father/sister/maternal-uncle list. Resolution: the list; the mother's
  hypertension is not carried.
- **R8.** Medication lists drop and restore drugs at random (hydrochlorothiazide
  missing in August 2021, prednisone and tacrolimus missing from several lists).
  Resolution: the course in the timeline. The October 2022 ED note's mention of
  valganciclovir is a stale entry from the 2020-2021 course; the ED record may copy it,
  the practice list does not carry it after June 2021.
- **R9.** Re-dated references: "March 2020" becomes the August 2021 nephrology visit,
  "July 2020" the December 2021 transplant visit, "January" the June 2021 visit.
  Stated ages follow the birth date (56 in December 2021, 57 in 2022 and April 2023,
  58 in October 2023, 61 in June 2026).
- **R10.** Encounter 5's chief complaint (cough, fever, weight loss) and its HPI
  (headaches, poor concentration) differ; both are carried in one ED record.
- **R11.** The December 2021 tacrolimus level of 15 is called "within therapeutic
  range" yet the visit diagnoses calcineurin inhibitor toxicity. Resolution: the
  value is carried as reported within the lab's reference interval; the transplant
  clinic attributes the creatinine rise, tremor and high glucose to tacrolimus and
  lowers the dose (A6).
- **R12.** Encounter 7 reports a biopsy at the visit itself. Resolution: ultrasound
  2026-05-28 and biopsy 2026-06-04, both reviewed at the 2026-06-15 visit. The
  profile's "CKD with volume overload" is the ankle swelling at this visit.
- **R13.** The abdominal bruit is carried where the source puts it (right upper
  quadrant).
- **Left as the source has it:** lithium continues through every year despite
  nephrogenic diabetes insipidus and a transplanted kidney, together with
  hydrochlorothiazide; nobody changes it or comments on the combination. The April
  2022 bruit is never imaged. No antihypertensive is added in April 2022 or June 2026;
  his regimen stays amlodipine and hydrochlorothiazide. The prophylaxis regimen is not
  changed after the breakthrough Pneumocystis pneumonia.

## Additions (A): physician to confirm

- **A1.** The 2019-09-17 new-patient visit, and the failure of the first graft in early
  2019 with return to hemodialysis (needed for a re-transplant in December 2020).
  Lithium, prednisone and amlodipine are his drugs on joining; tacrolimus and
  mycophenolate had been stopped when the first graft failed.
- **A2.** The December 2020 re-transplant admission and its regimen, including
  valganciclovir 450 mg daily for 6 months (the source names valganciclovir later).
  Discharge creatinine 1.4.
- **A3.** Hydrochlorothiazide 25 mg started by the transplant clinic 2021-02-16 (the
  source has it by June 2021; it would not be used on dialysis). Creatinine 1.2 then.
- **A4.** An outside psychiatrist manages lithium: 300 mg after each dialysis session
  while on hemodialysis, 300 mg twice daily after the re-transplant. Yearly letters:
  mood stable.
- **A5.** Desmopressin challenge at nephrology: urine osmolality 150 to 160 (supports
  the nephrogenic diagnosis the source asserts).
- **A6.** Tacrolimus reduced from 5 mg to 3 mg twice daily at the December 2021
  visit; lithium level 0.8 mmol/L that day.
- **A7.** January 2022 practice labs: creatinine back to 1.2, A1c 5.5 (the fasting
  glucose of 160 is not followed by a diabetes diagnosis).
- **A8.** The 2022-03-29 practice visit with BP 154/94 and creatinine 1.6 (the source's
  rise from 1.2 to 1.9 over two months).
- **A9.** April 2022 biopsy (acute T-cell-mediated rejection, Banff IA) and
  methylprednisolone 500 mg IV for 3 days; creatinine 1.4 on 2022-05-10 and 1.2 in
  June 2022. Rejection resolved by June 2022.
- **A10.** Pneumocystis PCR on induced sputum, positive; admission 2022-10-18 to
  2022-10-24; trimethoprim-sulfamethoxazole 800/160 mg 2 tablets three times daily for
  21 days (to 2022-11-07); single strength resumed 2022-11-08.
- **A11.** Nocardia admission 2023-04-17 to 2023-05-01: sputum culture Nocardia species;
  IV trimethoprim-sulfamethoxazole and imipenem for 2 weeks; oral
  trimethoprim-sulfamethoxazole 800/160 mg 2 tablets twice daily to 2024-04-30; brain
  MRI smaller in July 2023, resolved in May 2024; single strength resumed. Weight
  86 kg to 81 kg by May 2023, back to about 84 kg by 2024.
- **A12.** Cryptococcal meningitis admission 2023-10-15 to 2023-10-31: opening pressure
  31 cm H2O, CSF cryptococcal antigen titer 1:640, CSF culture Cryptococcus neoformans,
  serum antigen positive; liposomal amphotericin B with flucytosine for 14 days and
  repeat lumbar punctures; fluconazole 400 mg daily to 2023-12-24, then 200 mg daily
  to 2024-12-27; tacrolimus 2 mg twice daily while on fluconazole, 3 mg twice daily
  after.
- **A13.** Creatinine 1.3 in February 2024 and 1.4 in June 2025 (the start of the
  "gradual increase").
- **A14.** June 2026 transplant letter numbers and plan: creatinine 2.6, urine
  protein/creatinine 0.6 g/g, BP 158/96, bilateral ankle edema; furosemide 40 mg daily;
  vascular surgery referral for access; re-transplant evaluation.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not
  mention. An abnormal finding appears only as the source or this file gives it. A
  report may carry the descriptors its kind always carries, but nothing that adds a
  territory, a severity grade or a symptom, or changes the diagnosis or plan.
- Records from a prior practice may mention routine screening done there, with normal
  results: a normal screening colonoscopy in 2017 (transplant listing), repeat at 10
  years; hepatitis C and HIV screening negative during listing.
- Practice visits every 3 to 6 months for blood pressure, transplant coordination and
  medication reconciliation; annual wellness visits; BMP, CBC, A1c and lipids yearly,
  TSH and calcium yearly for lithium; PHQ-9 screens low.
- Outside transplant clinic letters between the source visits, reporting stable
  function and tacrolimus levels in target, and yearly psychiatry letters (A4).
- Influenza each fall, COVID vaccines (primary series plus the additional dose for
  transplant recipients and updated doses), PCV20, recombinant zoster, Tdap, RSV at 60
  for immunosuppression; no live vaccines. Yearly dermatology skin checks for a
  transplant recipient, normal.
- After 2025-06-11 only portal messages, refills, pharmacy traffic (influenza vaccine
  given at the pharmacy in fall 2025) and outside records.
- Refills, pharmacy and prior-authorization traffic, portal messages about results,
  refills and scheduling, a missed visit, reminders.
- No booked visit after 2026-08-27.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002034 on 2025-06-11. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. Nothing is booked.
