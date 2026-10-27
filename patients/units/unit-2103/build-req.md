# Build requirements: unit-2103 (Steven Sears, MRN00002103)

Source: synthetic_hospital patient 2103, in `source.md` beside this file. This file and
the source are the only authority for clinical facts. Every diagnosis, medication,
result, procedure and specialist finding in the chart must trace to one of:

- **S** a fact in source.md, carried as listed below;
- **R** a resolution below, where the source contradicts itself or stops short;
- **A** an addition below, listed one by one;
- **P** the primary-care layer described at the end.

Nothing else. No diagnosis, medication, test result, procedure, family member or
life event that is not on these lists. Texture (wording, scheduling, refills, portal
traffic, who signed what) is free, as long as it carries no new clinical fact.

Re-voice everything. The source notes are exam vignettes: never copy a sentence, and
never carry a source clinician's name, its "Active Problem List" dumps, its
"(diagnosed 2020-01-15)" stamps, or its quiz text (the "previous physician in a
clinical trial" line, the unfinished assessment).

## Who he is (S)

Steven Sears, born 1976-11-11, male, White, not Hispanic or Latino, English, lives in
Leeds, Medicare (SSDI) with Aetna supplemental. Former operator at a flavoring
manufacturing plant, now on disability leave (already on leave when he joined).
Current smoker, 1.5 packs per day for 30 years, 45 pack-years in the 2025 and 2026
records (R6); coded on joining as nicotine dependence, cigarettes. Alcohol rare, one or two drinks on holidays. No known drug allergies.
Surgical history: arthroscopic repair of a knee meniscus in 2018. Family history:
father COPD diagnosed at 55; mother hypertension and type 2 diabetes; an older brother
with asthma. Marital status single; no other people enter the chart.

Joined the practice 2022-06-14 (A1, new-patient visit). He brought COPD, diagnosed at
a prior practice before he joined (no year carried), on tiotropium 18 mcg inhaled once
daily and albuterol 90 mcg inhaler 1 to 2 puffs every 4 to 6 hours as needed. He was
already seen by an outside pulmonology group, which continues to follow him.
Height 178 cm, weight about 84 kg, blood pressure normal throughout (P). Resting SpO2 at the
practice runs about 95% on room air, about 93% at the two 2024 exacerbation visits (P).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2022-06-14 | practice | new-patient visit (A1): history as above; COPD on **tiotropium and albuterol (historical)**; screening labs (P); PCV20 given (P) |
| 2024-03-11 | practice | COPD exacerbation (A2): more cough, wheeze and sputum for several days; **prednisone 40 mg daily for 5 days** |
| 2024-08-21 | outside | pulmonology follow-up (A3): spirometry FEV1 60% predicted with an obstructive pattern; COPD, GOLD Group C; continue tiotropium and albuterol; cessation advised. Letter received 2024-09-10 |
| 2024-11-25 | practice | second COPD exacerbation of the year (A2): **prednisone 40 mg daily for 5 days and doxycycline 100 mg twice daily for 5 days** |
| 2025-01-06 | practice | follow-up after the second exacerbation (R1): still short of breath with exertion and coughing; **fluticasone propionate/salmeterol 250/50 mcg one inhalation twice daily started**, added to tiotropium and albuterol; referred back to pulmonology |
| 2025-02-03 | outside | pulmonology follow-up, age 48: 4 weeks into the ICS/LABA; feels he breathes easier and has had fewer flares, but still coughs and wheezes now and then, worse at night. SpO2 94% on room air; mild expiratory wheezes both sides; spirometry FEV1 60% predicted, unchanged from the August 2024 visit. **Bronchiolitis obliterans diagnosed** (R2), given fixed obstruction despite therapy and his flavoring-plant exposure; HRCT with expiratory views ordered (A4); continue all three inhalers; cessation urged. Letter received late (2025-04-02) |
| 2025-02-24 | outside | HRCT chest with inspiratory and expiratory images (A4): mosaic attenuation with air trapping on expiratory images, consistent with bronchiolitis obliterans; no nodule, mass, effusion or pneumothorax. Report received with the letter on 2025-04-02 |
| 2025-03-10 | practice | COPD follow-up: patient reports the lung specialist told him his small airways are scarred from the plant; PCP notes the bronchiolitis obliterans from the patient's account (R2); records requested |
| 2026-05-15 | practice | last practice visit (P): COPD and bronchiolitis obliterans follow-up on the three inhalers; still smoking; no exacerbation since November 2024 |
| 2026-08-05 | outside | ED, age 49: sudden severe shortness of breath and left chest pain after a hard coughing fit; anxious, sweating, cannot finish sentences. BP 90/60, HR 122, RR 30, SpO2 88% on room air, T 37.2 °C. Jugular venous distension, trachea deviated to the right, hyperresonance and decreased breath sounds on the left. ABG on room air: pH 7.32, PaCO2 50 mmHg, PaO2 60 mmHg, HCO3 24 mEq/L. Chest x-ray: large left pneumothorax with mediastinal shift. **Tension pneumothorax diagnosed.** Rapid deterioration, airway secured (R3). ED course after that (A5) |
| 2026-08-12 | outside | discharged home (A5), as the patient reports; the discharge summary has not reached the practice by today |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source says fluticasone/salmeterol was started "4 weeks ago" by a
  "previous physician" in a clinical trial. Resolution: the practice PCP starts it
  on 2025-01-06, four weeks before the pulmonology visit, after two exacerbations in
  2024. The trial and the "previous physician" are quiz text and are not carried. It
  is an ordinary prescription.
- **R2.** The source stamps bronchiolitis obliterans as diagnosed on encounter 0's
  date but the encounter's assessment stops mid-sentence. Resolution: the
  pulmonologist diagnoses it at that visit (fixed FEV1 at 60% despite added ICS/LABA,
  occupational flavoring exposure) and orders an HRCT (A4) that supports it. The
  condition dates from that visit (onset 2025-02-03), but the practice's own notes
  first mention it on 2025-03-10, from the patient's account; the letter arrives
  later. Coded as J44.81.
- **R3.** The ED note ends with "requires airway protection" and gives a chest x-ray
  before decompression. Resolution: the x-ray stays as the source has it (a portable
  film at the bedside as the team prepares), then the course in A5.
- **R4.** The ED note calls fluticasone/salmeterol "recently started" with the
  "started 4 weeks ago" stamp copied forward. Resolution: by August 2026 he has used
  it since January 2025; the ED record says so. The ED's "diagnosed January 2020"
  for bronchiolitis obliterans follows the re-dated timeline (February 2025).
- **R5.** The source profile lists the ICS/LABA and bronchiolitis obliterans from the
  start. Resolution: neither exists before its event above; notes before 2025-01-06
  carry only tiotropium and albuterol, and notes before 2025-03-10 carry no
  bronchiolitis obliterans.
- **R6.** "30 years at 1.5 packs per day" holds for 2025 and 2026. Earlier notes say
  he has smoked about a pack and a half a day since his late teens and give no
  pack-year figure other than 45, which first appears in the 2025 pulmonology letter.
- **Left as the source has it:** he keeps smoking 1.5 packs a day through August
  2026. Cessation is advised at visits, he is not ready, and no cessation drug is
  ever prescribed; no note remarks on the absence. No pulmonary rehabilitation is
  ordered. Lung cancer screening does not apply (he is under 50 through today).

## Additions (A): physician to confirm

- **A1.** 2022-06-14 new-patient visit (four years with the practice; COPD and two
  inhalers already in place).
- **A2.** Two moderate COPD exacerbations in 2024, treated at the practice as above
  (prednisone 40 mg daily x5 days each; doxycycline 100 mg twice daily x5 days with the
  second). They make GOLD Group C and the "fewer exacerbations" on ICS/LABA coherent.
  Each is coded as COPD with acute exacerbation and resolved at the next contact.
- **A3.** The 2024-08-21 pulmonology visit with FEV1 60% predicted, the "last visit"
  the source's unchanged spirometry refers to.
- **A4.** The 2025-02-24 HRCT supporting the bronchiolitis obliterans diagnosis, with
  the findings above.
- **A5.** ED course after the source ends: needle decompression of the left chest
  (second intercostal space, midclavicular line) with a rush of air and a rise in
  pressure; rapid-sequence intubation; left tube thoracostomy; repeat portable chest
  x-ray with the left lung re-expanded, chest tube and endotracheal tube in good
  position. After decompression BP 112/70, HR 104, SpO2 95% on the ventilator.
  Admitted to the ICU. By the patient's account he was extubated after two days, the
  tube came out, and he went home 2026-08-12 on his same three inhalers, told not to
  fly, to stop smoking, and to see pulmonology and a thoracic surgeon about
  preventing another collapse.
- **A6.** Practice contact after the admission, without a visit: the patient's portal
  message on 2026-08-14, a records request, the ED record received 2026-08-20, and a
  booked post-hospital visit. The practice's lists are not updated (after last seen).

## Records (gaps)

The pulmonology group's letters are irregular: the 2024 letter arrives about three
weeks late, the 2025-02-03 letter and HRCT report arrive two months late and only
after a request, and a later pulmonology visit the patient mentions in the fall of 2025
("stable, keep going") sends no letter at all; the chart carries only his account,
with no numbers. The hospital stay reaches the practice first through the patient,
then as the ED physician record; the discharge summary is still outstanding today.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it; mild scattered
  wheeze on exam is allowed at exacerbation visits only. A report may carry the
  descriptors its kind always carries, but nothing that adds a territory, a severity
  grade or a symptom, or changes the diagnosis or plan.
- Records from the prior practice (received 2022-07-06) with COPD, the two inhalers and
  routine normal labs.
- COPD visits every 4 to 8 months with vitals, inhaler technique, refills.
- Medicare annual wellness visits (initial 2023, then subsequent), with CBC, CMP, lipid
  panel and A1c (mother with diabetes), all normal; hepatitis C and HIV screening once
  at joining, negative; PHQ-2 negative; yearly FIT negative 2022 to 2025 (none due yet in 2026 by today).
- Influenza vaccine each fall, updated COVID vaccines, PCV20 once (2022), Tdap once.
- Refills, pharmacy and prior-authorization traffic for the inhalers, portal messages,
  a missed visit.
- Booked visit after 2026-08-27: post-hospital follow-up on 2026-09-03.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002103 on 2026-05-15. Practice visits after that day do
not happen; outside care after it still does, and its records are filed as received
without changing the practice's lists. A visit is booked after today.
