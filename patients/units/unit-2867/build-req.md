# Build requirements: unit-2867 (Irene Whitney, MRN00002867)

Source: synthetic_hospital patient 2867, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, its truncated plan ("Considering her electrolyte
abnormalities,") or its normal-range parentheses.

## Who she is (S)

Irene Whitney, born 1953-03-06, female, White, not Hispanic or Latino, English,
Medicare. Lives in Leeds (01053), widowed, lives alone; a daughter lives nearby and
is the one who brings her in during the March 2026 illness (S: the daughter reports
the history). Retired postal clerk. No alcohol. No known drug allergies.

Tobacco: cigarettes, about a pack a day; 45 pack-years when she quit in early March
2025 (S: "quit 6 months ago" at the September 2025 ED visit). Still smoking when she
joins, and declines cessation help in 2023 and 2024 (P).

Surgical history (S): bronchoscopy with transbronchial biopsy (July 2025, A3) and
port-a-cath placement (July 2025, A4). Nothing else.

Family history (S): father lung cancer; mother emphysema; brother COPD.

Joined the practice 2023-06-20 (new-patient visit, A1), age 70, after years at
another primary-care office. She brings:

- severe COPD, GOLD stage III (spirometry A2), on **tiotropium 18 mcg inhaled once
  daily** (HandiHaler), **albuterol 90 mcg/actuation, 2 puffs every 4 to 6 hours as
  needed** for shortness of breath, and **fluticasone propionate 250 mcg inhaled
  twice daily** (Diskus). These three continue unchanged through today.
- current cigarette smoking (nicotine dependence).

Small cell lung cancer, SIADH and the metabolic alkalosis are not known at joining;
each starts at its own date below. Ondansetron is not a home drug at joining (R2).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2023-06-20 | practice | new-patient visit (A1): COPD and smoking history as above; NKDA; family history; LDCT and spirometry planned |
| 2023-06-30 | outside | prior primary-care records arrive by fax (P): COPD, smoking, colonoscopy 2018 normal, screening mammogram 2022 normal, flu and COVID vaccines through 2022 |
| 2023-07-11 | practice | follow-up with office **spirometry (A2)**: post-bronchodilator FEV1/FVC 0.52, FEV1 0.94 L, 42% predicted; GOLD III confirmed; baseline labs (P); declines cessation help |
| 2023-07-19 | practice-ordered | **LDCT lung screening (A5)**: centrilobular emphysema, upper-lobe predominant; no nodule; Lung-RADS 1 |
| 2024-07-23 | practice-ordered | annual LDCT (A5): emphysema unchanged; no nodule; Lung-RADS 1 |
| 2025-03-04 | practice | COPD follow-up: **she quit smoking on about 2025-03-01** on her own; nicotine dependence resolved, history of tobacco dependence recorded |
| 2025-06-10 | practice-ordered | annual LDCT (A5): **new 3.4 cm right hilar mass with enlarged right hilar and subcarinal lymph nodes**; emphysema unchanged; Lung-RADS 4X. PCP calls her the same day; urgent pulmonology referral (A5) |
| 2025-07-01 | outside | pulmonology consult letter (A3): reviews the LDCT; plans bronchoscopy |
| 2025-07-03 | outside | **bronchoscopy with transbronchial biopsy** of the right hilar mass (S procedure, A3). Pathology: **small cell carcinoma** (TTF-1 positive, synaptophysin and chromogranin positive, Ki-67 about 90%) |
| 2025-07-10 | outside | staging (A3): PET-CT with FDG-avid right hilar mass and right hilar and subcarinal nodes, plus a **2.1 cm FDG-avid right adrenal lesion**, read as metastasis; MRI brain with and without contrast negative for metastasis. **Extensive-stage small cell lung cancer**; adrenal metastasis on the problem list |
| 2025-07-17 | outside | oncology consult letter (A4): extensive-stage SCLC; plan carboplatin AUC 5 day 1 + etoposide 100 mg/m2 days 1 to 3 + atezolizumab 1200 mg day 1, every 21 days for 4 cycles, then maintenance atezolizumab; port first. **Ondansetron 8 mg by mouth every 8 hours as needed for nausea** prescribed (R2) |
| 2025-07-22 | practice | PCP visit to go over the diagnosis and plan (P); health care proxy: daughter (P) |
| 2025-07-24 | outside | **port-a-cath placement** by interventional radiology (S procedure, A4); no complication |
| 2025-07-28 | outside | **cycle 1** carboplatin/etoposide/atezolizumab (A4); practice learns from the patient |
| 2025-08-18 | outside | cycle 2 (A4); no infusion summary sent (known from later letters) |
| 2025-08-25 | outside | oncology same-day visit (A6): about 3 days of vomiting after cycle 2, eating and drinking little; orthostatic; BMP sodium 136, potassium 3.3, chloride 89, bicarbonate 34, BUN 30, creatinine 1.2, glucose 104. **Hypochloremic metabolic alkalosis secondary to vomiting with hypovolemia** diagnosed (R1). 2 L IV normal saline with 20 mEq KCl in clinic; **potassium chloride 20 mEq by mouth daily for 3 days** (short course); ondansetron continued. Practice learns from a portal message 08-26; the oncology note arrives 09-03 |
| 2025-09-02 | practice | lab-only BMP (A6): sodium 138, potassium 4.1, chloride 101, bicarbonate 26, BUN 16, creatinine 0.9. Vomiting stopped; **alkalosis resolved** |
| 2025-09-08 | outside | ED (S): 3 days of worsening shortness of breath, cough with yellow sputum and fever. T 38.6 C (101.5 F), BP 135/85, pulse 102, RR 28. Crackles in the right lower lung field. ABG pH 7.48, PaCO2 28 mmHg, PaO2 60 mmHg, HCO3 24 mmol/L. Chest x-ray: right lower lobe infiltrate. Denies chest pain, vomiting, diarrhea. **Community-acquired pneumonia, right lower lobe** (R3). CBC without neutropenia (A7). Ceftriaxone 1 g IV and azithromycin 500 mg IV, nebulized albuterol-ipratropium, nasal oxygen 2 L; overnight observation; discharged 2025-09-09 with SpO2 92% on room air on **amoxicillin-clavulanate 875/125 mg twice daily for 7 days and azithromycin 250 mg daily for 4 days** (A7). Oncology delays cycle 3 one week |
| 2025-09-12 | practice | post-ED visit (P): improving; lungs with fewer right basilar crackles |
| 2025-09-16 | outside | cycle 3 (A4) |
| 2025-10-02 | outside | ED record received after a records request (records gap) |
| 2025-10-07 | outside | cycle 4 (A4) |
| 2025-10-30 | outside | oncology letter (A4): restaging CT chest/abdomen/pelvis 2025-10-28 shows **partial response** (right hilar mass 3.4 to 1.9 cm, adrenal lesion 2.1 to 1.2 cm, nodes smaller); **maintenance atezolizumab 1680 mg IV every 4 weeks from 2025-11-04** |
| 2025-12-06 to 2025-12-10 | outside | admission, pulmonary and critical care service (S): 3 days of progressive shortness of breath, cough with yellow sputum and generalized weakness; no chest pain, fever or recent travel. Afebrile, BP 130/80, HR 88, RR 22, SpO2 92% on room air. Bilateral wheezes and crackles at both bases. Sodium 125, potassium 4.2, chloride 95, bicarbonate 24, BUN 18, creatinine 0.9, glucose 98; serum osmolality 260, urine osmolality 500, urine sodium 80. Chest x-ray: increased interstitial markings and a small right pleural effusion. **COPD exacerbation with secondary bacterial pneumonia** (S). **SIADH diagnosed** (S; R4). Ceftriaxone and azithromycin IV, scheduled nebulized bronchodilators (S: "antibiotics and bronchodilators"; agents A8); **fluid restriction 1.2 L/day** (R4). Sodium 127 on 12-08, 130 at discharge 12-10 (A8). Discharged on **amoxicillin-clavulanate 875/125 mg twice daily for 3 more days** (A8) |
| 2025-12-11 | outside | discharge summary arrives; its medication list leaves out tiotropium (R5) |
| 2025-12-12 | practice | transitional-care visit (P): breathing back near baseline; BMP sodium 131 (A9); medication reconciliation; fluid restriction reviewed |
| 2026-01-09 | practice | lab-only BMP: sodium 130 (A9) |
| 2026-01-20 | outside | restaging CT (A4): response maintained; no letter is sent; the patient reports it at the February visit (records gap) |
| 2026-02-10 | practice | COPD and sodium follow-up (P): sodium 128 (A9); PCP tightens fluid restriction to 1 L/day, no drug added (ordinary decision) |
| 2026-03-07 to 2026-03-12 | outside | admission via ED to the ICU (S): 2 days of confusion and lethargy; daughter reports she has been less responsive and eating and drinking little for a week. Vital signs stable. Oriented to person only, not time or place; no sign of dehydration; neurologic exam non-focal. Sodium 120, serum osmolality 260, urine sodium 45, urine osmolality 500. **Severe symptomatic hyponatremia from SIADH** (R6). CT head without contrast: no acute abnormality, no mass (A10). TSH 1.8 and AM cortisol 14.2 (A10). 3% saline with sodium checks every 2 to 4 hours, correction held to 8 or less in 24 hours: sodium 125 at 24 hours, 128 on 03-10, 130 at discharge 03-12 (R6). Mental status back to baseline by 03-09. Discharged on **sodium chloride 1 g tablets three times daily** and fluid restriction 1 L/day (R6) |
| 2026-03-09 | practice | daughter phones the practice about the admission (records gap) |
| 2026-03-16 | practice | transitional-care visit (P): daughter brings the discharge paperwork; alert and oriented; BMP sodium 131 (A9); sodium chloride tablets added to the practice list |
| 2026-04-02 | outside | ICU discharge summary arrives (records gap: late) |
| 2026-04-13 | practice | lab-only BMP: sodium 133 (A9) |
| 2026-04-21 | outside | oncology letter (A4): restaging CT 2026-04-17 with response maintained; aware of SIADH admission; continue maintenance atezolizumab |
| 2026-05-19 | practice | Medicare annual wellness visit, subsequent (P); sodium 134 (A9) |
| 2026-07-21 | outside | restaging CT (A4): response maintained; patient reports it; letter not received by today |
| 2026-08-18 | practice | COPD and sodium follow-up (P): stable; sodium 133 (A9); continues salt tablets, fluid restriction, atezolizumab every 4 weeks |
| 2026-09-22 | practice | booked follow-up visit (P) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source stamps hypochloremic metabolic alkalosis from vomiting with
  hypovolemia as diagnosed at the September 2025 ED visit, but that visit's patient
  denies vomiting and its gas (pH 7.48, PaCO2 28, HCO3 24) is a respiratory alkalosis
  with a normal bicarbonate. Resolution: the alkalosis is its own episode two weeks
  earlier (2025-08-25, vomiting after chemotherapy cycle 2, A6), resolved by
  2025-09-02. The September ED record lists it as a recent problem. The ED gas is
  carried as written and read as respiratory alkalosis with hypoxemia from pneumonia.
- **R2.** The source lists ondansetron as a home drug at every encounter.
  Resolution: oncology prescribes it at the 2025-07-17 consult for chemotherapy
  nausea, and it stays on the list through today (as needed).
- **R3.** The source names no diagnosis at the September 2025 ED visit. Resolution:
  right lower lobe community-acquired pneumonia, treated in the ED with observation
  overnight and discharged home (the December note calls it an "ED visit for similar
  symptoms", so she was not admitted). Resolved by the 2025-09-12 visit's follow-up
  (recorded resolved there).
- **R4.** The source stamps SIADH as diagnosed at the December admission (its later
  "(diagnosed 2020-04-15)" stamp) and its plan stops mid-sentence. Resolution: SIADH
  is diagnosed on that admission from the sodium 125, serum osmolality 260, urine
  osmolality 500 and urine sodium 80 with a normal creatinine and glucose, in the
  setting of small cell lung cancer; treated with fluid restriction 1.2 L/day. The
  December summary is the first record of it. The pleural effusion is small and not
  tapped.
- **R5.** The December source medication list leaves out tiotropium while its prose
  says she takes it. Resolution: tiotropium continues throughout; the discharge
  summary's list omits it, and the practice's reconciliation on 2025-12-12 confirms
  she still takes it.
- **R6.** The March 2026 note gives no diagnosis or plan. Resolution: severe
  symptomatic hyponatremia from SIADH (no new diagnosis; SIADH is already on the list),
  low solute intake that week contributing; ICU care with 3% saline and slow
  correction as in the timeline; discharge on sodium chloride tablets 1 g three times
  daily plus a 1 L fluid restriction. "No evidence of dehydration" and "vital signs
  stable" are carried as written; no vital numbers are carried for that admission.
- **R7.** The source says she is 72 at all three encounters. Resolution: ages follow
  the birth date: 72 in September and December 2025, 73 on 2026-03-07 (her birthday
  was the day before). "Hospitalized in April 2020" becomes December 2025; "prior ED
  visit" refers to September 2025.
- **R8.** "Quit 6 months ago" is copied to every encounter. Resolution: she quit about
  2025-03-01; the September 2025 ED record says six months; later records say she
  quit in March 2025. 45 pack-years throughout; earlier practice notes say "over 40
  pack-years" while she still smokes.
- **R9.** The source calls her treatment "chemotherapy" at all three encounters.
  Resolution: chemo-immunotherapy induction (4 cycles, July to October 2025), then
  maintenance atezolizumab from November 2025; outside notes may still call it
  chemotherapy.
- **Left as the source has it:** her severe COPD stays on tiotropium plus fluticasone
  propionate alone, with no long-acting beta-agonist, through two exacerbations; the
  December admission names antibiotics and bronchodilators and no systemic steroid;
  no pulmonary rehabilitation is raised. Nobody changes these and no note comments on
  them.

## Additions (A): physician to confirm

- **A1.** The 2023-06-20 new-patient visit (the seed gives about 3 years with the
  practice; she has a primary-care physician before the source events begin).
- **A2.** One office spirometry (2023-07-11): post-bronchodilator FEV1/FVC 0.52, FEV1
  0.94 L, 42% predicted, to support the source's GOLD stage III.
- **A3.** The diagnostic course of the source's small cell lung cancer: pulmonology
  consult 2025-07-01, the source's bronchoscopy with transbronchial biopsy on
  2025-07-03 with the pathology above, staging PET-CT and brain MRI 2025-07-10 with
  the findings above, giving extensive stage (right adrenal metastasis).
- **A4.** The source's "chemotherapy", run by an outside oncology group:
  carboplatin/etoposide/atezolizumab for 4 cycles (2025-07-28, 08-18, 09-16 after a
  one-week delay for the pneumonia, 10-07), the source's port placement on
  2025-07-24, restaging 2025-10-28 with the partial response above, maintenance
  atezolizumab 1680 mg every 4 weeks from 2025-11-04 through today, and restaging CTs
  in January, April and July 2026 with response maintained (no new lesion, no
  measurements beyond those given).
- **A5.** Annual low-dose CT lung screening (eligible: age 70, over 40 pack-years,
  current smoker): 2023-07-19 and 2024-07-23 Lung-RADS 1 with emphysema (the CT
  descriptor of her COPD), 2025-06-10 the right hilar mass with right hilar and
  subcarinal nodes, Lung-RADS 4X, and the urgent pulmonology referral.
- **A6.** The vomiting episode that carries the source's hypochloremic metabolic
  alkalosis with hypovolemia (2025-08-25, values above), and the practice's repeat
  BMP on 2025-09-02 showing it resolved.
- **A7.** At the September 2025 ED visit: CBC without neutropenia; the ED treatment,
  overnight observation, discharge SpO2 92% on room air, and the discharge
  antibiotics above.
- **A8.** At the December 2025 admission: IV ceftriaxone and azithromycin, scheduled
  nebulized albuterol and ipratropium, sodium 127 on 12-08 and 130 at discharge, and
  3 more days of amoxicillin-clavulanate at discharge.
- **A9.** Sodium follow-up in primary care after SIADH is diagnosed: 131 (2025-12-12),
  130 (2026-01-09), 128 (2026-02-10), 131 (2026-03-16), 133 (2026-04-13), 134
  (2026-05-19), 133 (2026-08-18). Chloride runs low alongside (about 93 to 99) with a normal
  anion gap; the other BMP members stay normal.
- **A10.** In the March 2026 ICU stay: CT head without contrast (no acute
  abnormality, no mass) for confusion in a patient with small cell lung cancer, and
  TSH 1.8 mIU/L and AM cortisol 14.2 mcg/dL to support SIADH while on atezolizumab.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not
  mention (heart sounds, abdomen, no edema, breast tissue, bone density). An abnormal
  finding appears only as the source or this file gives it. A report may carry the
  descriptors its kind always carries, but nothing that adds a territory, a severity
  grade or a symptom, or changes the diagnosis or plan.
- Records from the prior practice may mention routine screening done there, with
  normal results (colonoscopy 2018, mammogram 2022).
- COPD visits about every 4 to 6 months with vitals (resting SpO2 runs 93 to 95% on
  room air outside the illnesses), inhaler technique, refills; after July 2025 the
  PCP visits also cover the cancer course as oncology reports it, and after December
  2025 the sodium.
- Baseline labs at joining (CBC, CMP, lipid panel, TSH, hepatitis C antibody
  nonreactive); yearly CMP and CBC; lipid panel at the wellness visits. All normal
  except the sodium values above. No HIV screen (age over 65).
- Medicare annual wellness visits (initial 2024-01-16; subsequent 2025-01 and
  2026-05-19) with PHQ-2 or PHQ-9 (low scores), fall and functional screening (she is
  independent), advance care planning (health care proxy: daughter; she wants full
  treatment).
- Prevention: screening mammogram 2023 and 2024 normal; DXA 2023 normal; colonoscopy
  not due (2018 normal); influenza vaccine each fall (high-dose or adjuvanted); COVID
  vaccines yearly; PCV20 once (2023); RSV once (2024); Shingrix series 2024; Tdap
  (2023).
- Refills, pharmacy and Part D prior-authorization traffic for the inhalers, portal
  messages about results, refills and scheduling, one missed visit (2024), one
  cancelled visit.
- Records texture (seed: gaps): letters arrive late or not at all as the timeline
  marks; the practice learns of the September 2025 ED visit from a pharmacy fill
  notice and her phone call; the practice medication list keeps carboplatin,
  etoposide and atezolizumab 1200 mg after induction ends and updates them only at
  the 2025-12-12 reconciliation.
- Booked visit after 2026-08-27: COPD and sodium follow-up on 2026-09-22.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.


## Last seen

The practice last saw MRN00002867 on 2025-12-12. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. Nothing is booked after today.
