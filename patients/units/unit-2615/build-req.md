# Build requirements: unit-2615 (Min-jun Tanaka, MRN00002615)

Source: synthetic_hospital patient 2615, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz text.

## Who he is (S)

Min-jun Tanaka, born 2005-12-12, male, Asian (seed; R1), English-speaking, university
student health insurance. Undergraduate student; during term he lives in a dormitory.
Never smoker. Alcohol: occasional social use, 1 to 2 drinks a month. No known drug
allergies. No surgical history. No home medications. Family history: maternal uncle
with nasopharyngeal carcinoma diagnosed at 42; paternal grandfather with gastric cancer.
Single.

Joined the practice on 2024-01-03 (A1), shortly after turning 18, from a pediatric
practice, as a high school senior living with his parents; he starts college and dorm
life in fall 2024 (the source's undergraduate at 19). Brought no problems and no
medications.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2024-01-03 | practice | new-patient preventive visit (A1): healthy; history as above; normal exam; lipid panel, HIV and hepatitis C screening, all normal (P); MenB dose 1 (P) |
| 2024-01-17 | outside records | pediatric practice records: childhood series complete, Tdap and MenACWY at 11, MenACWY booster at 16, HPV series, COVID primary series (P) |
| 2024-02-07 | practice | nurse visit, MenB dose 2 (P) |
| 2024-10-09 | practice | nurse visit, influenza and COVID vaccines (P) |
| 2025-01-06 (Mon) | practice | age 19. One week of fatigue, low-grade fever and sore throat, with mild upper abdominal discomfort and decreased appetite. Exam: enlarged cervical lymph nodes, mild splenomegaly, tonsillar exudates. Heterophile antibody positive; atypical lymphocytes on the peripheral smear. **Infectious mononucleosis diagnosed.** Supportive care, no drug; no contact sports while the spleen is enlarged (P) |
| 2025-01-21 | practice (portal) | symptoms improved over the 2 weeks since the visit without treatment |
| 2025-02-10 | practice | follow-up (P): recovered; spleen not palpable, no adenopathy; cleared for activity. **Mononucleosis resolved** |
| 2025-07-05 (Sat) | outside (ED) | 10 days of fever, sore throat and fatigue, like his January illness; fever peaks late afternoon, not relieved by over-the-counter acetaminophen; painful swallowing, liquids only for a few days; malaise, missed several days of classes; lives in a dormitory and several friends have had similar symptoms. T 38.5 °C, BP 120/75, pulse 88. Tired, no acute distress; bilateral tonsillar enlargement with exudates; tender cervical lymphadenopathy; spleen palpable 2 cm below the left costal margin; rest unremarkable. WBC 12.0 (×10^3/µL), atypical lymphocytes 15%, heterophile positive. **ED diagnosis infectious mononucleosis (R2)**; discharged home with supportive care, fluids, no contact sports. The ED record reaches the practice 2025-07-08 |
| 2025-07-09 | practice | post-ED telehealth visit (P): improving, swallowing better; same precautions |
| 2025-08-20 | practice | annual physical (P): well; spleen not palpable; CBC and CMP normal. **Mononucleosis resolved**; cleared for activity |
| 2025-10-08 | practice | nurse visit, influenza and COVID vaccines (P) |
| 2026-01-02 (Fri) | practice | 2 weeks of fever, sore throat and fatigue, like the January and July 2025 illnesses; not helped by over-the-counter pain relievers or throat lozenges; a "puffy" feeling in the upper abdomen. No medications. T 38.5 °C. Exam: pharyngeal erythema with exudates, tender cervical lymphadenopathy, mild splenomegaly. CBC: hemoglobin 10.2 g/dL, WBC 15.0 (×10^3/µL), platelets 90 (×10^3/µL), atypical lymphocytes 20%; the smear confirms atypical lymphocytes; direct antiglobulin (Coombs) test positive. **Infectious mononucleosis diagnosed again (R2); thrombocytopenia and anemia recorded.** Same-day hemolysis labs (A2). Late that day, with the DAT and A2 results back, **autoimmune hemolytic anemia recorded (R3)** and an urgent hematology referral placed (A3) |
| 2026-01-08 | outside (hematology) | consult (A3): fatigue, sore throat easing; mild splenomegaly, cervical nodes still tender; hemoglobin 10.6, platelets 104, reticulocytes 4.1%. Impression: mild autoimmune hemolysis and thrombocytopenia with the mononucleosis illness, expected to settle; observation, no steroid; weekly CBC for now; no contact sports; return for jaundice, dark urine or bleeding. Letter received 2026-01-09 |
| 2026-01-16 | practice (lab) | CBC (A3): hemoglobin 11.5, platelets 131 |
| 2026-02-06 | outside (hematology) | follow-up (A3): energy improving, back at classes; spleen not palpable; hemoglobin 12.6, platelets 178, reticulocytes 2.2%. Plan: CBC in about 2 months through the practice; discharge if normal. Letter received 2026-02-10 |
| 2026-04-15 | practice | **last seen** (A4): well; no adenopathy, spleen not palpable; CBC hemoglobin 14.1, platelets 228, WBC normal. **Mononucleosis, autoimmune hemolytic anemia and thrombocytopenia resolved**; cleared for activity |
| 2026-04-22 | outside records | hematology discharge letter after reviewing the April CBC (A4); filed, lists unchanged |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source calls him Chinese-American; the seed records race as Asian and
  gives his name. The seed identity governs; the chart records Asian and states no
  further ancestry.
- **R2.** The source diagnoses infectious mononucleosis three times in a year, with a
  positive heterophile each time. Resolution: each clinician labels the illness as the
  source does (the ED in July 2025, the practice in January 2026), and the practice
  problem list resolves the condition after each recovery and records it again at the
  next presentation. Notes treat the later episodes as a relapsing course of the same
  illness; the January 2026 note and the hematology letter may say a heterophile can
  stay positive for months. Nobody adds an explanation beyond that.
- **R3.** The source gives hemoglobin 10.2 with a positive direct Coombs test and
  platelets 90 but names no diagnosis for them. Resolution: the practice records
  anemia and thrombocytopenia at the visit and, once the DAT and hemolysis labs (A2)
  are back that day, autoimmune hemolytic anemia (unspecified type, D59.10) in place
  of the plain anemia. Both are treated as complications of the mononucleosis illness
  and resolve by April 2026.
- **R4.** The source's encounter 0 says his symptoms "improve over the next 2 weeks"
  inside the visit note. Resolution: the visit records the presentation; the
  improvement is reported later, in a portal message on 2025-01-21.
- **R5.** Encounter 0's past history says that "several years later" he is diagnosed
  with a nasopharyngeal carcinoma. That is quiz text, and on the re-dated timeline it
  would fall after today. Not carried: no nasopharyngeal mass, no ENT referral, no
  oncology. The maternal uncle's nasopharyngeal carcinoma stays in family history.
- **R6.** Source years follow the re-dated timeline: "January 2020" is January 2025,
  "January and July 2020" are January and July 2025, and the "diagnosed 2020-01-15"
  stamp is 2025-01-06.
- **Left as the source has it:** no rapid strep test or throat culture at any of the
  three presentations; no EBV-specific serology or viral load despite three
  heterophile-positive illnesses; no liver enzymes at the source visits (A2's bilirubin is a hemolysis test). Nobody orders
  them and no note comments.

## Additions (A): physician to confirm

- **A1.** The 2024-01-03 new-patient visit (seed: about 4 years with the practice; he
  is an adult practice patient from 18). Lipid panel, HIV and hepatitis C screening
  normal.
- **A2.** Same-day hemolysis labs on 2026-01-02, which any physician would send for a
  Coombs-positive anemia: reticulocytes 4.6%, LDH 388 U/L, haptoglobin under 10 mg/dL
  (reported "<10"), total bilirubin 1.9 mg/dL with direct bilirubin 0.3 mg/dL.
- **A3.** Urgent hematology referral for the Coombs-positive anemia with
  thrombocytopenia; the two hematology visits and the 2026-01-16 CBC, with the values
  in the timeline. No drug is started.
- **A4.** The 2026-04-15 recovery visit with its normal CBC (hemoglobin 14.1,
  platelets 228), and hematology's discharge letter.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Records from the pediatric practice may mention routine care and vaccines done there,
  with normal results.
- A yearly physical (2024 new-patient visit, 2025 physical) with vitals, depression
  screen and alcohol screen; lipids and HIV/hepatitis C screening once; CBC and CMP at
  the 2025 physical.
- Influenza and COVID vaccines each fall; the MenB series in 2024 (a college student).
- Portal messages about results, scheduling, school forms and an immunization record;
  a cancelled or rescheduled visit; front-desk calls.
- No booked visit after 2026-08-27.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002615 on 2026-04-15. Practice contacts after that day are
only messages and outside records; outside records are filed as received without
changing the practice's lists. Nothing is booked.
