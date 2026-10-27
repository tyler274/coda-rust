# Build requirements: unit-2125 (Randall Lindqvist, MRN00002125)

Source: synthetic_hospital patient 2125, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, its unfinished HPI fragment or its quiz text (the
mean-sodium calculation).

## Who he is (S)

Randall Lindqvist, born 1970-02-28, male, White, not Hispanic or Latino, English,
Medicare (disability due to malignancy). Lives in the practice's town (01060), single
(no partner or children named; none added). Construction site supervisor by trade; not
working since the leukemia diagnosis (R5). Former smoker, 20 pack-years, quit 2019.
Alcohol: social, 1 to 2 drinks a week.

- **Allergies:** sulfamethoxazole-trimethoprim (anaphylaxis); iodinated contrast (hives).
  Both known before he joined.
- **Surgical history:** right chest wall port-a-cath (2023, R1); bone marrow biopsies,
  several, the most recent in September 2025 (S: "3 months ago" from December 2025);
  tonsillectomy in childhood.
- **Family history:** father, lung cancer, died at 68; mother, type 2 diabetes and
  hypertension; paternal uncle, acute leukemia of unspecified type, died at 55.

**Joined the practice** 2024-09-04 (new patient visit, A1), age 54, two years before
today. He brought:

- **Problems:** acute myeloid leukemia, intermediate-risk cytogenetics, diagnosed 2023
  (A2), in remission on maintenance therapy at an outside cancer center; hypertension
  (since about 2016, A3); chemotherapy-induced peripheral neuropathy (since 2023).
- **Drugs:** acyclovir 800 mg by mouth twice daily (antiviral prophylaxis);
  posaconazole delayed-release 300 mg by mouth daily (antifungal prophylaxis);
  amlodipine 5 mg by mouth daily; polyethylene glycol 3350 17 g by mouth daily as
  needed for constipation; oxycodone 5 mg by mouth every 6 hours as needed for bone
  pain. Oncology prescribes acyclovir, posaconazole and oxycodone; the practice
  prescribes amlodipine and polyethylene glycol. Plus the maintenance regimen (A2).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2024-08-27 | outside | records from his prior primary-care practice received (A3, P): hypertension on amlodipine 5 mg; screening colonoscopy 2020 normal; Tdap 2019 |
| 2024-09-04 | practice | **new patient visit (A1)**: history as above; BP on amlodipine at goal; exam with reduced vibration and pinprick sense in both feet to the ankles (A4), otherwise normal; port site clean. Labs (P). Lung cancer screening raised and deferred while on leukemia therapy (P) |
| 2024-09-18 | outside | cancer center summary received at the practice's request (A2): diagnosis 2023, port 2023, induction and consolidation 2023, remission, maintenance cycles since early 2024; prophylaxis and oxycodone as above |
| 2024-10 to 2025-10 | practice | routine visits, vaccines and a Medicare wellness visit (P) |
| 2025-03-26 | outside | hematology-oncology progress letter: remission, maintenance continues (A2) |
| 2025-09 | outside | bone marrow biopsy at the cancer center; letter 2025-09-24: continued morphologic remission, maintenance continues (A2) |
| ~2025-11-19 | outside | last chemotherapy cycle completed (S: 3 weeks before the ED visit) |
| 2025-12-10 | outside | **ED, regional hospital**: fever and chills for 2 days; last chemotherapy cycle completed 3 weeks earlier; T 38.9 °C (102 °F), BP 100/60, pulse 110, RR 22; pallor and diffuse mucosal ulcerations, otherwise normal exam; WBC 1.0 (1,000/uL), ANC 0.1 (100/uL), hemoglobin 8.2, platelets 50 (50,000/uL); blood cultures drawn, pending. **Chemotherapy-induced neutropenia** (febrile). IV cefepime started and admitted to the oncology service (A5) |
| 2025-12-10 to 2025-12-15 | outside | admission (A5): cefepime continued; blood cultures no growth at 5 days; neutrophils recovered; mucositis improving; home prophylaxis continued; discharged home. Discharge summary received 2025-12-16 |
| 2025-12-19 | practice | post-discharge visit (P): afebrile, mouth healing, oncology follow-up arranged |
| 2026-02-25 to 2026-03-05 | outside | **ICU admission, regional hospital: severe sepsis with hemodynamic instability** (no source named, R4); IV fluids, broad-spectrum IV antibiotics and norepinephrine (A6); morning sodium 136, 142, 131 and 139 on ICU days 1 to 4 (2026-02-25 to 02-28); nephrology consulted for the sodium swings (R2); IV fluid volumes varied, no hemolysis or lab error. **Hyponatremia** recorded (the 131), transient. Discharge summary received 2026-03-09 |
| 2026-01-07 | outside | oncology letter: seen 2025-12-22, maintenance resumed 2025-12-29 at the same doses (A2) |
| 2026-03-11 | practice | post-discharge visit (P): recovering; BMP with normal sodium (P); **severe sepsis and hyponatremia resolved** |
| 2026-04-02 | outside | oncology letter: seen 2026-03-19, maintenance resumed 2026-03-23 at the same doses (A2) |
| 2026-04 to 2026-07 | practice | Medicare wellness visit, refills (P) |
| 2026-07-24 | practice | **last practice visit** (P): routine blood pressure follow-up, well, alert and conversant, no labs drawn; next visit booked |
| 2026-07-25 | outside | **ICU admission, regional hospital (neurology)**, age 56 (R3): altered mental status, **severe hyponatremia, serum sodium 112**; IV hypertonic saline; sodium 137 at 48 hours (2026-07-27); on hospital day 3 (2026-07-27) acutely unresponsive; intubated and mechanically ventilated (A7). Neurology exam: complete quadriplegia; opens eyes and blinks to command; cannot speak, move the face or breathe spontaneously. MRI brain: symmetric hyperintensity in the central pons sparing the tegmentum. **Central pontine myelinolysis** diagnosed. Still in hospital at today. Because this follows last_seen, the hyponatremia and the myelinolysis live in the hospital's records and results; the practice's problem and medication lists stay as they were on 2026-07-24 |
| 2026-07-29 | practice | hospital case manager calls to report the admission and asks for the medication list (P) |
| 2026-08-14 | outside | neurology consult note and MRI report received; filed without changing the practice's lists |
| 2026-10-23 | practice | booked visit: 3-month blood pressure follow-up (booked on 2026-07-24) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source dates the port to 2023, a year later than its own encounters.
  Resolution: the re-dated timeline puts the leukemia diagnosis in March 2023, and the
  port was placed then, so "2023" stands as written.
- **R2.** Encounter 1 is a quiz (four sodiums, a mean "to two decimal places" for
  nephrology). Resolution: the four morning sodiums are carried as results; nephrology
  is consulted for the fluctuation and advises daily chemistries; no mean is computed
  and no chronic-hyponatremia label is applied. Only the 131 is low, so hyponatremia
  is recorded as a transient finding of that stay and resolved at the March visit.
- **R3.** Encounter 2 calls him 55; by 2026-07-25 he is 56 (born 1970-02-28). The
  January and April admissions in its history are the December 2025 and February 2026
  admissions. Encounter 1's "January 15, 2020" ED visit is the 2025-12-10 ED visit.
- **R4.** The source never names the sepsis source or organism. Resolution: the
  discharge summary records severe sepsis with no source identified; no organism, no
  culture result and no counts are given.
- **R5.** The source gives his occupation as construction site supervisor, and his
  insurance is Medicare for disability due to malignancy. Resolution: that is his
  trade; he has not worked since the 2023 diagnosis.
- **R6.** The ED vignette's HPI breaks off mid-sentence. Resolution: the chief
  complaint (fever and chills for 2 days) carries the presentation.
- **R7.** Lab panels carry only the source's values; the chloride on each low-sodium
  draw follows the sodium (as an analyzer would report it) so the panel is internally
  consistent, and the ED differential's neutrophil percentage follows the ANC and WBC
  (10%). No other value is added.
- **Left as the source has it:** no allogeneic transplant is discussed for an
  intermediate-risk AML in a man in his early fifties; no drug is started for the
  neuropathy; no cause of the July hyponatremia is named; the July rise from 112 to 137
  in 48 hours is recorded as it happened and nobody in the practice comments on it.

## Additions (A): physician to confirm

- **A1.** The 2024-09-04 new patient visit (two years with the practice per the seed).
- **A2.** The leukemia's course so that ongoing cycles in late 2025 make sense:
  diagnosed March 2023 at an outside cancer center (marrow biopsy), port placed then;
  intensive cytarabine-based induction and consolidation in 2023 with morphologic
  remission; the neuropathy began during consolidation; maintenance azacitidine
  (subcutaneous, days 1 to 7 of 28-day cycles) with venetoclax 70 mg by mouth daily on
  days 1 to 14 (dose reduced for posaconazole) since early 2024, run by oncology;
  marrow biopsies, the latest in September 2025, show continued morphologic remission.
  After each of the 2025-12 and 2026-02 admissions he resumes maintenance at the same
  doses (2025-12-29 and 2026-03-23).
- **A3.** Hypertension since about 2016 on amlodipine 5 mg, from prior-practice records.
- **A4.** Neuropathy exam: reduced vibration and pinprick sense in both feet to the
  ankles, at the practice's exams.
- **A5.** Febrile neutropenia care: IV cefepime from the ED, a 5-day oncology admission,
  blood cultures no growth at 5 days, neutrophil recovery before discharge.
- **A6.** Severe sepsis care: IV fluids, broad-spectrum IV antibiotics and norepinephrine
  in the ICU, weaned; discharged home 2026-03-05.
- **A7.** Intubation and mechanical ventilation on 2026-07-27, since he cannot breathe
  spontaneously.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may
  carry the descriptors its kind always carries, but nothing that adds a territory, a
  severity grade or a symptom, or changes the diagnosis or plan.
- Records from a prior practice may mention routine screening done there, with normal
  results (colonoscopy 2020 normal, repeat at 10 years).
- Blood pressure visits about every 4 to 6 months on amlodipine 5 mg, BP mostly 120s to
  130s over 70s to 80s; weight stable; yearly CMP, lipid panel and A1c with normal
  results; oncology runs the blood counts. Hepatitis C and HIV screening once, negative.
- Vaccines for an immunocompromised adult: inactivated influenza each fall, COVID each
  fall, PCV20 once, recombinant zoster two doses. Tdap from 2019 is current.
- Lung cancer screening (eligible by age and pack-years) raised and deferred while on
  leukemia therapy; no CT is done.
- Medicare annual wellness visits (2025 initial, 2026 subsequent) with PHQ-2/PHQ-9
  screens, negative or mild.
- Refills, pharmacy traffic, portal messages about results, refills and scheduling, a
  visit cancelled around a chemotherapy cycle.
- Booked visit after 2026-08-27: a 3-month blood pressure follow-up on 2026-10-23.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002125 on 2026-07-24. Practice contacts after that day do not
happen beyond the case manager's call and filing; outside care after it still does, and
its records are filed as received without changing the practice's lists. A visit is
booked after today.
