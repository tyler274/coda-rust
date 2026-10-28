# Build requirements: unit-2277 (Russell Abbott, MRN00002277)

Source: synthetic_hospital patient 2277, in `source.md` beside this file. This file and
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
"(diagnosed 2020-04-15)" stamps, or its quiz text (the Kaplan-Meier curves and p-value).

## Who he is (S)

Russell Abbott, born 1959-11-25, male, White, not Hispanic or Latino, English,
Medicare, Northampton MA 01060. Retired construction foreman. Former smoker, 30
pack-years, quit about 10 years before the source (quit in 2015). Occasional alcohol,
1 to 2 drinks a week. No known drug allergies. Family history: father with lung
cancer, mother with type 2 diabetes, an older brother with COPD.

Joined the practice 2023-09-12 (A1, a new-patient visit) with COPD and type 2
diabetes, on metformin 1000 mg twice daily, tiotropium 18 mcg inhaled once daily
(capsule inhaler) and albuterol 90 mcg/puff, 2 puffs every 4 to 6 hours as needed for
wheezing. No surgical history at joining (the bronchoscopy and mediastinoscopy come
in 2025). Height 178 cm, weight 93 kg at joining (A6).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2023-09-12 | practice | new patient visit (A1): history as above; COPD and diabetes carried; eligible for lung cancer screening; low-dose CT ordered; labs (A1c, CMP, lipids, urine albumin/creatinine, CBC, hepatitis C and HIV screens) |
| 2023-10-17 | outside imaging | baseline low-dose CT for lung cancer screening: no lung nodules, Lung-RADS 1 (A2) |
| 2025-01-21 | outside imaging | annual screening low-dose CT (delayed from the fall): **4.1 cm spiculated right lower lobe mass with enlarged mediastinal lymph nodes, Lung-RADS 4X** (A2); practice calls him and refers him to pulmonology urgently (A3) |
| 2025-02-11 | outside | pulmonology: **bronchoscopy with transbronchial biopsy** of the mass: non-small cell carcinoma (S surgical history; A3) |
| 2025-02-20 | outside | thoracic surgery: **mediastinoscopy** for nodal staging; mediastinal nodes involved; **stage IIIB non-small cell lung cancer**, not resectable (S; A3) |
| 2025-03-03 to 2025-04-25 | outside | concurrent chemotherapy and radiation at an outside cancer center, completed 2025-04-25 (S "completed a course of chemotherapy and radiation"; R1). The practice knows it from one short oncology letter and the patient |
| 2025-10-27 | outside | oncology surveillance visit (encounter 0): new cough and shortness of breath; vitals stable; decreased breath sounds at the right lower lung field; WBC 12.0 K/uL, hemoglobin 13.2 g/dL, platelets 250 K/uL, sodium 140, potassium 4.2, creatinine 1.0; chest x-ray with a new right lower lobe consolidation. He is enrolled in a clinical trial comparing two new targeted therapies for NSCLC (R4). The practice receives a short clinic note |
| 2026-02-09 | outside | ED (encounter 1): 2 days of high fever, shaking chills, productive cough with yellowish sputum; T 39.2 C, BP 135/85, pulse 102, RR 22, SpO2 92% on room air; decreased breath sounds and dullness at the right lower lobe; WBC 18.0 K/uL, neutrophils 80%, lymphocytes 10%, hemoglobin 14 g/dL, platelets 250 K/uL, glucose 180 mg/dL; chest x-ray with dense right lower lobe consolidation. **Opportunistic pulmonary infection** (R5). Admitted 2026-02-09 to 2026-02-13 with **bronchoscopy and bronchoalveolar lavage** for sampling and antibiotics (A4). Practice receives only the ED note |
| 2026-02-24 | practice | post-hospital visit (P): his account of the admission (A4); opportunistic pulmonary infection added to the problem list; finishing oral antibiotics he cannot name; discharge summary requested and never received |
| 2026-05-13 | outside | ED (encounter 2): 3 days of fever, productive cough and increasing shortness of breath, three months after the February visit; RR 28, T 38.7 C, BP 90/60, SpO2 88% on room air; bilateral crackles; arterial gas pH 7.25, PaCO2 60, PaO2 55, HCO3 22; WBC 18.0 K/uL; lactate 5 mmol/L; blood cultures pending. Admitted to the ICU 2026-05-13 to 2026-05-21 (A5). During resuscitation with large volumes of normal saline he develops a **hyperchloremic metabolic acidosis** (S; R6), which clears after fluids are changed to a balanced crystalloid. Practice receives the ED note and, on 2026-06-02, a brief discharge summary |
| 2026-06-23 | practice | post-hospital visit, last seen (P): weight 82 kg (A6); hyperchloremic metabolic acidosis recorded as resolved history; BMP and CBC drawn, unremarkable; oncology continues to follow |
| 2026-09-22 | practice | booked: diabetes and COPD follow-up |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source says the cancer was "diagnosed 6 months ago" and also that he is
  "6 months post-treatment" at the same visit. Resolution: diagnosis in February 2025,
  chemoradiation March to 2025-04-25, so he is six months past treatment on
  2025-10-27. "Diagnosed 6 months ago" is not carried. The later ED notes' copied "6
  months post-treatment" is not carried; they state the real interval (about 10 and
  12 months) or none.
- **R2.** The source gives his age as 65 at all three encounters. Ages follow the
  birth date: 65 on 2025-10-27, 66 at both ED visits.
- **R3.** Source text says "last evaluated in January 2020"; on the re-dated timeline
  that is the October 2025 oncology visit.
- **R4.** The oncology plan is quiz text (Kaplan-Meier curves, an event-free survival
  endpoint, p = 0.03). Resolution: the note says only that he has enrolled in a trial
  comparing two targeted agents for NSCLC, run by the oncology research team. The study
  drug is never named and is not on the practice medication list (oncology dispenses
  it). The trial statistics are not carried.
- **R5.** The source stamps the opportunistic infection "diagnosed" at the February
  ED visit and names neither organism nor sampling. Resolution: the ED treats it as a
  pneumonia in a recently treated cancer patient and admits him; bronchoscopy with
  lavage during that stay is the diagnostic sampling the label names (A4). The
  organism never reaches the practice (sparse records). The practice lists it from
  2026-02-24, onset 2026-02-09, and it stays on the list. "History of recent
  pneumonia" at encounter 2 is this February episode.
- **R6.** The source lists hyperchloremic metabolic acidosis from excess normal
  saline but its May arterial gas has HCO3 22 (the low pH there is from PaCO2 60).
  Resolution: the acidosis develops in the ICU after the ED's saline boluses, is named
  in the discharge summary without numbers, and has resolved by discharge. No chloride
  or bicarbonate values reach the chart from the admission.
- **R7.** Encounter 0 gives no vital numbers ("stable"); the oncology note says
  stable and carries none.
- **Left as the source has it:** no statin in a diabetic over 40 (lipids are drawn,
  nobody starts one or comments). Oncology documents the new right lower lobe
  consolidation with WBC 12.0 in October 2025 and records no antibiotic or further
  imaging. No durvalumab consolidation (he goes to a trial instead). The May blood
  culture results never reach the chart.

## Additions (A): physician to confirm

- **A1.** 2023-09-12 new-patient visit (the source starts with established specialist
  care and three years with the practice).
- **A2.** Lung cancer screening, which his age and 30 pack-years quit within 15 years
  call for: baseline low-dose CT 2023-10-17 Lung-RADS 1; next one delayed to
  2025-01-21 (he cancelled the November slot), showing a 4.1 cm spiculated right lower
  lobe mass with enlarged mediastinal nodes, Lung-RADS 4X. This is how the source's
  cancer was found.
- **A3.** The path from that scan to the source's surgical history: urgent pulmonology
  referral, bronchoscopy with transbronchial biopsy 2025-02-11 (non-small cell
  carcinoma), mediastinoscopy 2025-02-20 (involved mediastinal nodes, stage IIIB, not
  resectable), oncology referral.
- **A4.** February 2026 admission, 2026-02-09 to 2026-02-13, with bronchoscopy and
  bronchoalveolar lavage, antibiotics, discharge home on oral antibiotics. Known to the
  practice from the ED note and the patient.
- **A5.** May 2026 ICU admission, 2026-05-13 to 2026-05-21: IV fluids, IV
  antibiotics, supplemental oxygen, the acidosis of R6, discharged home on room air.
- **A6.** Weight 93 kg at joining (BMI about 29), about 86 kg after chemoradiation
  (June 2025), about 82 kg after the May 2026 admission. A1c about 7.0 to 7.3 on
  metformin throughout. LDL about 110, untreated.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may carry
  the descriptors its kind always carries, but nothing that adds a territory, a severity
  grade or a symptom, or changes the diagnosis or plan.
- Records from a prior practice may mention routine screening done there, with normal
  results (a 2019 screening colonoscopy without polyps, repeat in 10 years).
- Diabetes and COPD visits every 3 to 6 months, with vitals, A1c, metabolic panel,
  lipids and urine albumin/creatinine yearly, foot exams, inhaler technique.
- Medicare: Welcome to Medicare exam after he turns 65, then annual wellness visits;
  depression screens (negative).
- Yearly diabetic eye exam letters from an optometrist: no retinopathy.
- Influenza each fall, COVID each fall, PCV20 once, Shingrix series, RSV once (COPD).
  Hepatitis C and HIV screening once (negative).
- Refills, pharmacy traffic, portal messages about results, refills and scheduling, a
  cancelled imaging appointment, reminders, records requests.
- Booked visit after 2026-08-27: a routine diabetes and COPD follow-up.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002277 on 2026-06-23. Practice contacts after that day do
not happen; outside care after it still does, and its records are filed as received
without changing the practice's lists. A visit is booked after today.
