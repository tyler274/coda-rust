# Build requirements: unit-2798 (Fatou Diallo, MRN00002798)

Source: synthetic_hospital patient 2798, in `source.md` beside this file. This file and
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
never carry a source clinician's name, its "Active Problem List" dumps or its
"(diagnosed ...)" stamps.

## Who she is (S)

Born 1983-05-07, female, Black, not Hispanic, English-speaking, lives in Westfield
(01085), commercial PPO (Blue Cross Blue Shield). Administrative assistant. Never
smoker; alcohol socially, 1 to 2 drinks a month. Allergies: none known (NKDA).
Surgical history: appendectomy at 15 (about 1998), cesarean section at 28 (about
2011). Family history: mother rheumatoid arthritis, father hypertension, sister
autoimmune (Hashimoto) thyroiditis.

Joined the practice in August 2024 (A1). She brought: systemic sclerosis, limited
cutaneous subtype, diagnosed about 2019 (5 years before December 2024); Raynaud
phenomenon; gastroesophageal reflux disease. No daily medications (S, R3).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2024-08-12 | practice | phone call to join the practice (P) |
| 2024-08-20 | practice | **new patient visit** (A1): history as above; about 2 months of breathlessness on exertion and a dry cough (the start of the source's 6-month history); lungs clear, no edema; weight 68 kg, height 165 cm, BP normal. Baseline CBC, CMP, lipids, TSH, HIV and hepatitis C screening, all normal (P; CBC and renal function normal per R2). **Pulmonology referral** for exertional dyspnea and cough in a patient with SSc (A2). GERD and Raynaud recorded as mild, managed without drugs (R3) |
| 2024-08-28 | outside | prior primary-care records received: Pap with HPV 2022 normal, screening mammogram 2022 normal, Tdap 2019 (P) |
| 2024-09-17 | practice | screening mammogram, BI-RADS 1 (P) |
| 2024-10-08 | practice | nurse visit: influenza and COVID vaccines (P) |
| 2024-12-02 | outside | **pulmonology consult** (letter received 2024-12-10): 6 months of progressive exertional dyspnea and dry cough; no chest pain, orthopnea or PND; no recent change in skin or joint symptoms. BP 130/80, HR 88, RR 18, SpO2 96% on room air. Lungs without crackles, wheezes or rhonchi; loud P2; no peripheral edema. CBC and renal function normal (the practice's August draw, R2). Echo: normal LV size and function, RV hypertrophy, estimated RVSP 55 mmHg. PFTs: normal FEV1 and FEV1/FVC, reduced DLCO (no number). Impression: probable pulmonary arterial hypertension associated with SSc; right heart catheterization planned (R1) |
| 2025-01-14 | practice | follow-up to review the letter: **PAH associated with SSc added to the problem list**, catheterization pending at pulmonology (R1); loud P2 on exam, lungs clear, no edema; PCV20 given (P) |
| 2025-03-11 | practice | portal message: fingers blanching in the cold; glove and warming advice, no drug (P, R3) |
| 2025-05-20 | practice | follow-up visit, no-show (P) |
| 2025-08-01 | practice | **last practice visit** (annual physical): breathlessness on stairs slowly worse, tired; no chest pain; cath not yet done, pulmonology follow-up already booked for December; she is told to call pulmonology to come in sooner if worse (R4). Lungs clear, loud P2, no JVD, no edema. CBC, CMP, lipids, normal. PHQ-9 negative |
| 2025-10-14 | outside | influenza vaccine at a retail pharmacy, record received (P) |
| 2025-12-09 | outside | **pulmonology follow-up** (learned first by her portal message 2025-12-11; short note received 2026-01-27): a year of gradually worsening breathlessness and fatigue, winded on stairs, sometimes lightheaded walking quickly; no daily medications; never smoker. Lungs clear, elevated JVP, loud P2. Chest x-ray: enlarged central pulmonary arteries, clear lung fields. Right heart catheterization: elevated pulmonary arterial pressures, normal wedge pressure (no numbers). Diagnosis: pulmonary arterial hypertension associated with SSc (R5). **Bosentan started** (A3); monthly liver tests through pulmonology |
| 2025-12-16 | outside | pharmacy fill history: bosentan 62.5 mg twice daily (A3) |
| 2026-01-13 | outside | pharmacy fill history: bosentan 125 mg twice daily (A3) |
| 2026-03-17 | outside | pulmonology follow-up, known only by her portal message 2026-03-19: staying on bosentan, liver tests normal so far (A4) |
| 2026-06-23 | practice | portal reminder that her yearly physical is due; she says she will call after the summer; nothing booked (P) |

Because the last practice visit is 2025-08-01, bosentan never enters the practice's
medication list, and the problem list stays as it was that day. The December letter,
the fill histories and her messages are filed as received.

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The 2025 note says PAH was "diagnosed" at the first pulmonology visit, yet
  the right heart catheterization that confirms it comes a year later. Resolution:
  the first visit gives a probable diagnosis on echo and DLCO and plans the
  catheterization; the practice lists the condition from 2025-01-14 with "cath
  pending"; the catheterization is done at the 2025-12-09 visit.
- **R2.** The first pulmonology visit cites a normal CBC and renal function with no
  source. Resolution: they are the practice's 2024-08-20 draw, sent with the referral.
- **R3.** The source lists GERD and Raynaud phenomenon but says she takes no daily
  medication. Resolution: both are mild and managed without drugs (meal timing,
  head of bed raised; gloves and warming). No drug is started for either.
- **R4.** A year passes between suspected PAH and catheterization. Resolution: the
  chart keeps the interval; the August 2025 practice note records that the follow-up
  is booked for December and asks her to call sooner if worse. No one explains why
  the catheterization waited.
- **R5.** The 2025-12-09 plan says "idiopathic" PAH while every other line ties it to
  her systemic sclerosis. Resolution: the letter names PAH associated with SSc;
  "idiopathic" is not carried.
- **Left as the source has it:** no drug for GERD or Raynaud; no disease-modifying
  drug for her SSc; no formal ILD imaging beyond the chest x-ray; the year's wait for
  catheterization. No note comments on these.

## Additions (A): physician to confirm

- **A1.** 2024-08-20 new-patient visit (the seed gives 2 years with the practice; the
  source's history predates the pulmonology visit).
- **A2.** The pulmonology referral from that visit, which sends her to the source's
  first encounter.
- **A3.** Bosentan, which the source starts without a dose: 62.5 mg twice daily for
  about 4 weeks, then 125 mg twice daily, with monthly liver tests at pulmonology.
  The practice sees it only as fill history and letters.
- **A4.** One pulmonology follow-up on bosentan (2026-03-17): continues it, liver
  tests normal so far; reported only by the patient.
- **A5.** Her SSc was diagnosed and is followed at an outside rheumatology group about
  yearly, with no drug; known only from her account (records never received).

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not
  mention. An abnormal finding appears only as the source or this file gives it. A
  report may carry the descriptors its kind always carries, but nothing that adds a
  territory, a severity grade or a symptom, or changes the diagnosis or plan.
- Records from a prior practice may mention routine screening done there, with normal results.
- Visits as in the timeline; vitals at each; yearly CBC, CMP, lipids; TSH once; HIV
  and hepatitis C screening once; PHQ-2/PHQ-9 screens negative.
- Prevention: screening mammogram (2024, BI-RADS 1); cervical screening current from
  2022; influenza each fall, COVID 2024, PCV20 in 2025 for her lung and heart disease.
- Refills, scheduling, portal messages about results and reminders, a missed visit.
- No booked visit after 2026-08-27.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002798 on 2025-08-01. Practice contacts after that day do
not happen except messages and outside records; outside care after it still does,
and its records are filed as received without changing the practice's lists. Nothing
is booked after today.
