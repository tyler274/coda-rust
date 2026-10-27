# Build requirements: unit-2188 (Lyle Haskins, MRN00001102)

Source: synthetic_hospital patient 2188, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its unfinished quiz stems ("According to the
current guidelines...", "In addition to smoking cessation,").

## Who he is (S)

Born 1953-10-02, male, White, Medicare, retired construction foreman. Smokes a pack a
day since age 21 (R1). One or two drinks a week. NKDA. Family history: father MI at 58,
mother hypertension and a stroke at 72, brother with an abdominal aortic aneurysm.
Surgical history: diagnostic laparoscopy for abdominal pain at 45 (1998, no records);
cardiac catheterization with PCI for his MI (below). Rheumatic fever as a child, with
no valve disease since.

Joined the practice in 2018 (first visit 2018-05-08, a new-patient visit): hypertension,
hyperlipidemia, COPD on olodaterol 5 mcg inhaled daily, PAD and a known small abdominal
aortic aneurysm on the problem list (S, profile). Medications then: lisinopril 20 mg,
hydrochlorothiazide 25 mg, atorvastatin 40 mg, aspirin 81 mg, olodaterol (S).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2018-05-08 | practice | new-patient visit (A1) |
| 2019-01-22 | outside | admission at a regional hospital for an MI, treated with cardiac catheterization and PCI; the practice receives the discharge summary; **metoprolol succinate 50 mg daily** starts here (R2); practice post-hospital visit within 2 weeks (P). The chart says nothing either way about cardiology follow-up after the admission; the 2023 cardiology visit is a new referral (R7) |
| 2020-01-14 | practice | routine follow-up, a year after the MI; adherent; no chest pain or dyspnea; BP 132/84, HR 68; lipid panel TC 180, LDL 70, HDL 45, TG 150. History of rheumatic fever reviewed |
| 2020-04-22 | outside | vascular surgery surveillance visit: occasional mild abdominal discomfort he calls indigestion; BP 142/88, HR 78; soft abdomen, palpable pulsatile periumbilical mass, no bruit, peripheral pulses intact; Hb 14.2, Hct 42, fasting glucose 98; ultrasound **infrarenal AAA 4.2 cm** (R3) |
| 2020-07-21 | practice | annual wellness visit; no complaints; BP 132/84; Hb 14.2, fasting glucose 98, Cr 1.0, lipids per R3. The physician finds no cancer screening in five years: **low-dose CT for lung cancer screening** ordered (A2) and colorectal screening (P) |
| 2021-07-28 | outside | vascular medicine consult: calf cramping walking more than two blocks, relieved by rest; BP 142/88, HR 78; diminished pulses both feet; **ABI 0.65 right, 0.70 left**; aneurysm rescanned 4.3 cm (A3). Advice: walking program and stopping smoking |
| 2022-07-13 | outside | pulmonology consult: progressive dyspnea over a few years, chronic cough with white sputum, no chest pain, fever or weight loss; SpO2 92% on room air; barrel chest, diminished breath sounds, prolonged expiration, occasional wheeze. PFTs: obstruction with FEV1 severely reduced, FVC and FEV1/FVC reduced, TLC, FRC and RV increased (R4) |
| 2023-07-19 | outside | cardiology consult: exertional dyspnea and chest discomfort on hills and stairs, occasional palpitations, mild fatigue; no orthopnea, PND or edema; BP 168/96, pulse 78, BMI 29; sustained, laterally displaced apical impulse, normal heart sounds, no murmur, clear lungs. Echo: thickened LV wall, normal EF, no wall-motion abnormality. ECG: increased QRS voltage, tall R in V5 to V6, lateral T inversions. **Hypertensive heart disease** (R5) |
| 2024-07-24 | outside | pulmonology follow-up: dyspnea worse over the year, occasional productive cough; using olodaterol only as needed (S); SpO2 88% at rest, other vitals normal; decreased breath sounds, prolonged expiration, mild accessory muscle use; FEV1/FVC 60%. Plan: stop smoking (R6) |
| 2025-01-27 | practice | acute bronchitis visit (S: "two episodes of acute bronchitis in the past six months"); treatment per A4 |
| 2025-04-08 | practice | second acute bronchitis visit; treatment per A4 |
| 2025-07-16 | outside | pulmonology follow-up: worse dyspnea, chronic productive cough, frequent chest infections for two years, the two bronchitis episodes; says he wants to get healthier and will consider changes; SpO2 92%; prolonged expiration, wheeze, reduced breath sounds at the bases. Chest x-ray: hyperinflation, flattened diaphragms. Spirometry FEV1/FVC 0.65, minimally reversible |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Pack-years read 45, 40, 30 and 25 across notes. Resolution: a pack a day since 21,
  so 45 pack-years in 2020, rising each year. Practice notes say he still smokes; outside
  notes may round it.
- **R2.** The source has metoprolol on the list and an MI "one year ago". Resolution:
  metoprolol starts at the 2019 MI admission.
- **R3.** The 2020 lipid panels read LDL 70, 130 and 95 within six months, with 160 in 2021,
  all on the same atorvastatin. Resolution: only the January 2020 panel is carried. His
  lipid level stays near it (LDL about 70 to 80, HDL about 45, TG about 150) with ordinary
  variation. The other source panels are not used.
- **R4.** The source lists COPD from the start and "diagnoses" it at the 2022 pulmonology
  visit. Resolution: COPD was known on joining (he was on olodaterol), and 2022 is the
  first formal PFT.
- **R5.** BP 132/84 in 2020 and "poorly controlled hypertension for many years" in 2023.
  Resolution: his BP drifts up from 2022 (140s to 160s over 90s at practice visits), on the
  same drugs. No one changes them, and no note explains why. The source changes none, and
  the physician who inherits the chart can find that gap.
- **R6.** The source's "in addition to smoking cessation," is cut off. Resolution: the plan
  is smoking cessation counseling only. No oxygen and no new inhaler. He stays on olodaterol
  alone, used as needed. That gap is left too.

- **R7.** The source has no cardiology contact between the 2019 MI and 2023. Resolution:
  silence. No note claims a follow-up happened or failed to happen.
- **R8.** The January 2020 panel's TC 180 exceeds LDL 70 + HDL 45 + any calculated share of
  TG 150. Resolution: that panel reports LDL by direct assay (lipid panel with direct LDL),
  which is why its total runs above the parts, as the result message says.

## Additions (A): physician to confirm

- **A1.** The 2018 new-patient visit.
- **A2.** Low-dose CT lung screening: first scan August 2020, then yearly 2021 to 2025, each
  Lung-RADS 2 (benign-appearing nodules or none), with emphysema noted.
- **A3.** AAA surveillance: 4.2 cm (2020), 4.3 cm (2021), 4.4 cm (2023), 4.6 cm (2025), all
  under the repair threshold, scanned by vascular surgery.
- **A4.** The two 2025 acute bronchitis visits are treated as a primary-care physician
  would treat bronchitis in COPD: a short prednisone course, plus an antibiotic once
  (doxycycline) for purulent sputum. Chest exam and SpO2 at each visit.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention
  (kidney size, no hydronephrosis, aorta caliber, heart sounds). An abnormal finding appears
  only as the source or this file gives it. A report may carry the descriptors its kind
  always carries (how a size was measured, an aneurysm's shape, a benign nodule's size and
  lobe within its Lung-RADS category, a procedure's access site), but nothing that adds a
  territory, a severity grade or a symptom, or changes the diagnosis or plan.
- Records from a prior practice may mention routine screening done there, with normal results.
- Visits for blood pressure, lipids, CAD and COPD about every 3 to 6 months; Medicare annual
  wellness visits; basic metabolic panel and lipids yearly; CBC sometimes.
- Smoking counseling at most visits; he declines to set a quit date most years.
- Age-appropriate prevention: colorectal screening (Cologuard or FIT, negative), influenza
  each fall, pneumococcal, shingles series, COVID vaccines, RSV vaccine (2023 or later),
  Tdap if due; a fall-risk and depression screen at wellness visits; hepatitis C screening once.
- Refills, pharmacy traffic, portal messages about results and refills, a missed visit.
- Booked visit after 2026-08-27: a routine follow-up.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.


## Last seen

The practice last saw MRN00001102 on 2025-08-26. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. Nothing is booked after today.
