# Build requirements: unit-2269 (Gordon Ryan, MRN00002269)

Source: synthetic_hospital patient 2269, in `source.md` beside this file. This file and
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
"(diagnosed 2020-07-15)" stamps, or its unfinished plans.

## Who he is (S)

Gordon Ryan, born 1957-03-26, male, White, not Hispanic or Latino, English, Medicare,
lives in Westfield (01085), divorced, lives alone. Retired long-haul truck driver.
Former smoker, 40 pack-years, quit about 10 years before the 2025 events (quit 2016).
No alcohol. No known drug allergies. Family history: father MI at 61, mother
intracerebral hemorrhage at 74, a sibling with hypertension and type 2 diabetes.

Joined the practice 2022-08-16 (A1, new patient visit), from a prior practice whose
records follow. He brings: hypertension (poorly controlled), COPD (GOLD group B) and
hyperlipidemia; lisinopril 10 mg daily, atorvastatin 40 mg daily, tiotropium 18 mcg
inhaled daily (capsule inhaler), albuterol 90 mcg 2 puffs every 4 hours as needed. No
coronary disease, no aspirin, no beta-blocker at joining (R2). Height 178 cm, weight
about 91 kg throughout (P level).

Records are sparse (seed): outside care reaches the practice as the patient's
account, a short ED note, a brief faxed ICU note, a case manager's call or a fill
history, with few numbers.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2022-08-16 | practice | new patient visit (A1). History as above. BP above goal on lisinopril 10 mg (P level about 152/92); no change, recheck (left as the source has it) |
| 2022-09-13 | practice | spirometry (A2): post-bronchodilator FEV1/FVC 0.64, FEV1 68% predicted; CAT score 14; mMRC 2; no exacerbations in the past year. Confirms COPD, group B. Tiotropium and albuterol continue |
| 2022-10 to 2025-08 | practice | primary-care layer (P): HTN/COPD visits, yearly wellness visits, labs, vaccines, lung screening (A3), aortic screening (A4) |
| 2025-11-17 | outside | **acute myocardial infarction** at a regional hospital, **percutaneous coronary intervention with drug-eluting stent** (S: "3 weeks prior to first encounter"; date R1). Practice learns by the patient's call 2025-11-21 and the discharge medication sheet he brings. **Aspirin 81 mg daily and metoprolol succinate 50 mg daily start at this admission** (R2). No vessel, troponin or echo numbers reach the practice |
| 2025-12-01 | practice | post-hospital visit (P). **Coronary artery disease and acute MI recorded, presence of coronary stent recorded.** Meds reconciled: aspirin 81, metoprolol succinate 50, atorvastatin 40, lisinopril 10, tiotropium, albuterol. Cardiology follow-up arranged by the hospital |
| 2025-12-08 | outside | ED: sudden severe sharp, constant left lower-leg pain; foot pale and cold; no trauma. Vitals stable, afebrile. Left foot pulse diminished compared with right, cool, delayed capillary refill, decreased sensation, dorsiflexor and plantar flexor strength 3/5. CBC within normal limits, creatinine 1.1, troponin not elevated, D-dimer elevated (no number). Doppler ultrasound: abrupt cutoff of arterial flow in the left popliteal artery. Left ABI 0.5. **Acute peripheral arterial thrombosis/embolism, left leg.** Admitted under vascular surgery for **left femoral-popliteal thromboembolectomy with fasciotomy** (R3). The practice receives a short ED note 2025-12-10 |
| 2025-12-19 | practice | post-hospital visit (P): home after the surgery; fasciotomy wounds dressed by visiting nurse, vascular surgery follow-up booked by the hospital. Acute arterial embolism on the problem list. Same medications |
| 2026-01 to 2026-05 | practice | follow-up (P); patient reports cardiology saw him in January with no medication change (A5) and vascular surgery says the wounds have healed (A5). BP still above goal (P level about 146/88 on metoprolol and lisinopril); no change |
| 2026-06-12 | practice | **last held visit** (seed `last_seen`): blood pressure and leg follow-up; office BP 158/94 (P level); asked to keep a home BP log; no medication change. BMP and lipid panel drawn, results normal apart from the LDL level below |
| 2026-06-19 | outside | ED (date R4): found unresponsive at home, brought by EMS, no known trauma. BP 220/120. GCS 6, pupils reactive and symmetric. Non-contrast head CT: hyperdense collection in the basal ganglia extending into the internal capsule, surrounding edema, no midline shift, no crescent- or lens-shaped collection. **Intracerebral hemorrhage with airway compromise; intubated in the ED and started on mechanical ventilation**; admitted to the ICU. The practice receives a short ED note 2026-06-22 and files it (no list change) |
| 2026-06-24 | outside | ICU note (date R4), five days after intubation: over the past 24 hours a new fever of 38.9 C, more purulent tracheal secretions and worsening hypoxemia. On norepinephrine, hemodynamically stable on it. Sedated; diffuse bilateral crackles; rest of the exam unremarkable. WBC up from 12,000 to 18,000/uL. Chest x-ray: new bilateral infiltrates. **Ventilator-associated pneumonia with respiratory failure and septic shock** (R5). Cultures and empiric IV antibiotics per the ICU (R6). The practice receives the faxed note 2026-06-29 and files it |
| 2026-07-14 | outside | hospital case manager calls (A6): extubated, off norepinephrine, antibiotic course finishing, transfer to inpatient rehabilitation planned for the coming week; discharge summary to follow. The summary has not arrived by today |
| 2026-09-17 | practice | **booked**: post-hospital follow-up, booked at the case manager's request |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source dates the PCI only as "3 weeks prior to first encounter" and calls
  the MI recent. Resolution: MI and drug-eluting stent on 2025-11-17, three weeks before
  the 2025-12-08 ED visit. Before that he has no coronary disease on any list.
- **R2.** The profile lists aspirin and metoprolol succinate as home medications from the
  start. Resolution: both start at the November 2025 MI admission. Lisinopril 10 mg,
  atorvastatin 40 mg, tiotropium and albuterol are his from before joining.
- **R3.** The source lists the left femoral-popliteal thromboembolectomy with fasciotomy
  as past surgery at the first encounter that diagnoses the limb ischemia, and the ED
  plan stops mid-sentence. Resolution: the surgery is the treatment of the 2025-12-08
  occlusion, done that admission. The chart does not carry the unfinished plan.
- **R4.** The source puts the ED visit and the ICU note on one day, yet the ICU note says
  he was intubated five days earlier, and the seed's last practice visit (2026-06-12)
  falls inside the given dates. Resolution: ED 2026-06-19 (intubated that day), ICU
  note 2026-06-24, both after the last practice visit. The source's "six months prior"
  holds (December to June). Ages follow the birth date: 68 in December 2025, 69 in June
  2026 (the source says 68 at both).
- **R5.** The ICU chief complaint names community-acquired pneumonia, while its history
  describes a pneumonia arising after five days on the ventilator. Resolution:
  ventilator-associated pneumonia; "community-acquired" is not carried. "Septic shock"
  stays with the norepinephrine; "hemodynamically stable" means stable on it.
- **R6.** The ICU plan stops mid-sentence. Resolution: the faxed note says cultures were
  sent and empiric IV antibiotics started, with no drug names or culture results. No
  ICU drug enters the practice's list.
- **Left as the source has it:** no P2Y12 inhibitor is named after the drug-eluting stent,
  no anticoagulation after the arterial embolism, atorvastatin stays 40 mg after the MI,
  and blood pressure stays above goal on lisinopril 10 mg (with metoprolol from November
  2025) through June 2026 with no drug change. No one comments on any of these. No
  embolic source workup is recorded.

## Additions (A): physician to confirm

- **A1.** New patient visit 2022-08-16 (seed: about 4 years with the practice).
- **A2.** One spirometry (FEV1/FVC 0.64, FEV1 68% predicted, CAT 14, mMRC 2) to support the
  listed COPD group B.
- **A3.** Lung cancer screening (40 pack-years, quit 2016): low-dose CT 2022-10, 2023-11 and
  2024-11, each Lung-RADS 1, no nodules. The 2025 scan is ordered in October 2025 and
  cancelled after the MI and limb surgery.
- **A4.** One-time aortic ultrasound (male ever-smoker, 65 to 75): 2022-11, abdominal aorta
  normal caliber (2.1 cm), no aneurysm.
- **A5.** Post-event specialist follow-up by the patient's account only: cardiology in
  January 2026, no medication change; vascular surgery, fasciotomy wounds healed by
  February 2026. No letters reach the practice.
- **A6.** The hospital course after the source ends (case manager call 2026-07-14):
  extubated, norepinephrine stopped, antibiotics finishing, transfer to inpatient
  rehabilitation planned. No neurological findings, numbers or drugs are given.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may carry
  the descriptors its kind always carries, but nothing that adds a territory, a severity
  grade or a symptom, or changes the diagnosis or plan.
- Records from the prior practice may mention routine screening done there with normal
  results: a colonoscopy in 2019 without polyps, repeat in 10 years.
- Hypertension and COPD visits about every 4 to 6 months, with vitals; BP runs about
  152/92 on lisinopril alone and about 146/88 after metoprolol; SpO2 ordinary; yearly
  Medicare wellness visits; yearly CMP or BMP, CBC and lipid panel (LDL about 94 on
  atorvastatin 40, TC about 172, HDL about 41, TG about 186); A1c normal once or twice
  (sibling with diabetes); hepatitis C and HIV screening once, negative; PHQ-2 negative.
- Vaccines: high-dose influenza each fall 2022 to 2025, COVID boosters each fall,
  pneumococcal PCV20 and Tdap in 2022, shingles series 2023 to 2024, RSV in 2024.
- Refills, pharmacy fill reports, portal messages about results, refills and scheduling,
  one missed visit, reminders.
- After 2026-06-12 only records filed as received, calls and refill requests; the
  practice's problem and medication lists stay as they were at that visit.
- Booked visit after 2026-08-27: the post-hospital follow-up on 2026-09-17.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.


## Last seen

The practice last saw MRN00002269 on 2026-06-12. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. A visit is booked after today.
