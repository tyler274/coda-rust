# Build requirements: unit-2630 (Eli Delaney, MRN00002630)

Source: synthetic_hospital patient 2630, in `source.md` beside this file. This file and
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

Eli Delaney, born 2005-04-09, male, White, not Hispanic or Latino, English-speaking,
lives in Haydenville MA 01039 (parents' home; dormitory during the school year), on his
parent's employer-sponsored private plan. Single. College student, a freshman living in
a dormitory in the 2024-25 school year (R4). Never smoker, no alcohol.
Family history: mother hypothyroidism, father essential hypertension; no family history
of autoimmune disorders or hematologic malignancies. No surgical history.
Allergies: none known until amoxicillin (maculopapular rash) is recorded on 2025-05-18 (R1).

Joined the practice at 18 (A1): new-patient visit 2023-06-20, no problems, no
medications, no known drug allergies. Height 180 cm, weight 72 kg.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2023-06-20 | practice | new-patient preventive visit (A1); well; history as above; labs per P |
| 2023-07 | outside records | pediatric records arrive (P): childhood vaccines complete including Tdap, MenACWY (age 11 and 16 doses), HPV series, MMR, varicella; COVID primary series 2021; nothing else of note |
| 2024-11-27 (Wed) | practice | same-day visit, home from college for the holiday week: about 1 week of malaise, sore throat, fever and a "puffy" neck; began with a mild headache and fatigue and worsened; now painful swallowing. Denies cough, rhinorrhea, chest pain. No medications. Temp 38.5 C. Exudative tonsillitis, marked cervical lymphadenopathy, spleen palpable 2 cm below the left costal margin. Peripheral smear with atypical lymphocytes; rapid strep negative. A2: heterophile antibody (Monospot) positive; CBC WBC 10.8 K/uL, lymphocytes 58%, atypical lymphocytes 14%, hemoglobin and platelets normal. **Infectious mononucleosis diagnosed.** Supportive care; no contact sport or heavy lifting while the spleen is enlarged |
| 2024-12-18 | practice | mononucleosis follow-up: no-show (he was back at school) |
| 2025-01-07 (Tue) | practice | follow-up: symptoms gone, back to full energy; no lymphadenopathy of note, spleen not palpable; CBC normal (A3). **Mononucleosis resolved**; cleared for full activity |
| 2025-05-17 (Sat) | outside | campus-area urgent care visit for sore throat: **amoxicillin 500 mg three times daily for 10 days started** (A4). The urgent care never sends a record; the practice learns of the drug from the pharmacy fill and the patient |
| 2025-05-18 (Sun) | practice | portal message: diffuse red rash over trunk and limbs a day after starting amoxicillin, still sore throat and tired. PCP: **amoxicillin stopped; amoxicillin allergy (maculopapular rash) recorded** (R1); because this is his second exudative throat illness in six months, ENT referral with a next-day opening (A5) |
| 2025-05-19 (Mon) | outside | otolaryngology consult (S): about a week of sore throat, fatigue and mild fever, poor appetite and malaise; rash over trunk and extremities after amoxicillin begun two days earlier; prior presentation in November 2024 with malaise, sore throat, fever and neck swelling. Denies shortness of breath and joint pain. Temp 38.4 C. Bilateral cervical lymphadenopathy, enlarged tonsils with white exudates, lungs clear, no cough or wheeze. ENT impression: infectious mononucleosis again (R2); supportive care, avoid amoxicillin, return as needed (R3) |
| 2025-05-21 | practice | phone check-in: patient reports ENT called it mono again; **infectious mononucleosis recorded again** by report |
| 2025-06-23 | outside records | ENT consult letter arrives after the practice asks for it (late letter) |
| 2025-08 | practice | annual physical; mononucleosis long settled, **resolved** |
| 2025-11-30 (Sun) | outside | emergency department (S): 10 days of fever up to 38.9 C (102 F), severe sore throat, trouble swallowing, marked fatigue; began as a mild sore throat and worsened; denies cough, rhinorrhea, rash. No medications. Bilateral anterior and posterior cervical lymphadenopathy, erythematous oropharynx with enlarged exudative tonsils, spleen palpable 2 cm below the left costal margin, no hepatomegaly, rest unremarkable. WBC 12,000/uL, atypical lymphocytes 20%, ALT 70 U/L, AST 65 U/L. ED impression: infectious mononucleosis, third episode (R2). Discharged home with supportive care (R3) |
| 2025-12-02 (Tue) | practice | post-ED visit (P): **infectious mononucleosis recorded again**; improving; spleen still palpable; splenic precautions repeated; A6: CBC normal, hepatic panel ALT 61, AST 47, rest normal |
| 2025-12-19 | outside records | ED record with its labs arrives after a records request (late record) |
| 2026-01-06 (Tue) | practice | follow-up: well, spleen not palpable; A6: ALT 29, AST 24, CBC normal. **Mononucleosis resolved**; cleared for activity |
| 2026-08 | practice | annual physical, well |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The profile and every note list the amoxicillin rash from the first visit, but
  the rash happens in May 2025. Resolution: no known drug allergies until 2025-05-18,
  when the practice records amoxicillin (maculopapular rash). The ENT letter and ED
  record carry it.
- **R2.** The source calls each of three illnesses (November 2024, May 2025, November
  2025) infectious mononucleosis and the ED speaks of two prior episodes "diagnosed in
  January and July 2020", which follow the re-dated timeline as November 2024 and May
  2025. Resolution: the chart keeps each outside clinician's label as written; the
  practice records the condition at each episode and resolves it when he recovers. No
  one in the chart explains or questions the recurrence.
- **R3.** The ENT and ED notes give no plan or disposition. Resolution: ENT advises
  supportive care, avoiding amoxicillin and return as needed; the ED discharges him home
  with supportive care and no contact sports. No drug is prescribed at either.
- **R4.** The source calls him a "college freshman" at all three encounters, a year
  apart. Resolution: freshman in the 2024-25 school year, living in a dormitory; notes
  from fall 2025 call him a college student in the dorm, without naming a year.
- **R5.** The ENT note says it is a return to "the Otolaryngology clinic", but the first
  illness was seen in the practice. Resolution: the May 2025 ENT visit is his first ENT
  visit, by the practice's referral; the letter refers to the November visit as a prior
  presentation.
- **R6.** Ages: the source's age 19 at first encounter fits 2024-11-27 (age 19).
- **Left as the source has it:** no EBV-specific serologies, no imaging of the spleen,
  no hematology or infectious disease referral for the repeated episodes, and no
  re-examination of whether the amoxicillin rash was a true allergy. Nobody raises any
  of these.

## Additions (A): physician to confirm

- **A1.** Joining the practice: new-patient preventive visit 2023-06-20 at age 18
  (seed: about 4 years with the practice).
- **A2.** Heterophile antibody positive and CBC (WBC 10.8 K/uL, lymphocytes 58%,
  atypical lymphocytes 14%) at the November 2024 visit, supporting the diagnosis the
  source asserts from a smear.
- **A3.** The January 2025 follow-up visit and its normal CBC (usual recovery check).
- **A4.** The amoxicillin course the ENT note says was started two days earlier: an
  outside urgent care on 2025-05-17, amoxicillin 500 mg capsule three times daily for 10
  days.
- **A5.** The 2025-05-18 portal exchange that stops amoxicillin and makes the ENT referral.
- **A6.** Post-ED liver enzyme follow-up to normal: 2025-12-02 ALT 61, AST 47;
  2026-01-06 ALT 29, AST 24; CBCs normal.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Records from the pediatric practice may mention routine care there, normal.
- Establish-care labs: CBC, CMP, lipid panel, HIV and hepatitis C screening, all normal.
  Annual physicals 2024-07, 2025-08, 2026-08 with vitals and PHQ-2/PHQ-9 (negative);
  CBC and CMP at the 2025 physical, normal. College health forms.
- Vaccines: influenza each fall (fall 2024 given at the campus health service and
  reported by the patient), COVID updated vaccine in fall 2023 and fall 2025,
  meningococcal B two-dose series in summer 2024 before college.
- Acetaminophen or ibuprofen over the counter as needed during the throat illnesses,
  mentioned in notes and not on the medication list.
- Portal messages about results, forms, scheduling and immunization records; a missed
  visit; records requests to outside care; insurance card updates.
- Booked visit after 2026-08-27: a nurse visit for the fall influenza vaccine.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.
