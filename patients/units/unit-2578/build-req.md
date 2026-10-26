# Build requirements: unit-2578 (Gavin Fournier, MRN00002578)

Source: synthetic_hospital patient 2578, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its unfinished plan text.

## Who he is (S)

Gavin Fournier, born 2004-08-21, male, White, not Hispanic or Latino, English-speaking,
single, home address in Easthampton, MA 01027 (his family's house). University student
health insurance plan. Undergraduate college student living in a campus dormitory at a
nearby university (R3). Never smoker. Alcohol: social, weekend drinking (R2). No known
drug allergies.

Surgical history: tonsillectomy at age 6 (about 2010-2011); wisdom teeth extraction at
age 17 (about 2021-2022).

Family history: a maternal first cousin had meningococcal disease as a child and was
hospitalized; paternal grandmother with rheumatoid arthritis; father with
hypertension. No other family history.

Joined the practice on 2022-09-13 (A1), age 18, a new-patient visit. Brought:
seasonal allergic rhinitis (cetirizine 10 mg by mouth daily as needed) and acne vulgaris
(adapalene 0.1% gel applied to the face nightly). Both are over-the-counter products he
buys himself; both stay on his list unchanged through today.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2022-09-13 | practice | new-patient visit (A1): history as above; exam normal; HIV and hepatitis C screens (P) |
| 2024-08-03 (Sat) | outside, regional hospital ED | 12 hours of fever, severe headache and neck stiffness; lives in a dormitory and a few friends have been sick with similar symptoms. T 39.2 °C, HR 110, BP 130/75. Ill-appearing, photophobic; Kernig and Brudzinski signs positive; petechial rash on the trunk and lower extremities. Lumbar puncture: CSF WBC 1200/mm³ with 80% neutrophils, glucose 20 mg/dL, protein 180 mg/dL, Gram stain gram-negative diplococci. **Meningococcal meningitis diagnosed.** IV ceftriaxone, admitted (R6). The practice receives a short ED physician note on 2024-08-09 |
| 2024-08-10 | outside | discharged home after about a week of IV antibiotics; no antibiotic continued after discharge (R6; known to the practice from the patient's account) |
| 2024-08-20 | practice | post-hospital visit (P): headaches gone, tired; exam normal, no rash, neck supple, neuro nonfocal; CBC normal. **Meningococcal meningitis resolved** (history after this) |
| 2025-03-18 (Tue) | outside, regional hospital ED (R1) | age 20; brought in by his roommate, who noticed a rash on his arms and legs. Back 2 days from a spring-break trip; malaise and muscle aches he blamed on travel, then over several hours high fever, severe headache and difficulty bending his neck; disoriented, history limited. T 39.2 °C, BP 90/60, pulse 120, RR 22. Petechial rash with some purpura on the extremities, nuchal rigidity, Brudzinski sign positive, capillary refill 3 seconds. WBC 15,000/mm³, platelets 100,000/mm³, Na 130, K 4.5, BUN 18, Cr 1.2, CRP 24 mg/L, blood cultures pending. CSF WBC 2,000/mm³ with 90% neutrophils, protein 150 mg/dL, glucose 20 mg/dL. **Acute bacterial meningitis, meningococcemia and thrombocytopenia diagnosed** (R4, R5). IV ceftriaxone, admitted (R6). The ED note reaches the practice on 2025-03-28 |
| 2025-03-26 | outside | discharged home after about 8 days of IV antibiotics; told his platelets came back up before discharge; no antibiotic after discharge (R6; patient's account) |
| 2025-04-08 | practice | post-hospital visit (P): feels well, back in classes; rash faded; exam normal; CBC normal (platelets normal), BMP normal (sodium and creatinine normal). **Acute bacterial meningitis, meningococcemia and thrombocytopenia resolved** |
| 2026-01-31 (Sat) | outside, regional hospital ED | age 21; 2 days of fever, headache and neck stiffness, and a non-itchy rash that began on the trunk and spread to the limbs; his roommate was recently diagnosed with bacterial meningitis. Prior meningitis 8/2024 and 3/2025. T 39.2 °C (R7), HR 110, BP 90/60. Petechial rash with areas coalescing into purpura; nuchal rigidity. WBC 15,000/µL, hemoglobin 13.5 g/dL, platelets 50,000/µL, PT 18 s (reference 11-13.5), PTT 45 s (reference 25-35), fibrinogen 150 mg/dL (reference 200-400). Lumbar puncture deferred for the platelet count and clotting times (R6). **Meningococcemia with thrombocytopenia and clinical bacterial meningitis diagnosed again** (R4, R5). IV ceftriaxone, admitted (R6). The ED note reaches the practice late, on 2026-02-24 |
| 2026-02-09 | outside | discharged home after about 9 days of IV antibiotics; no antibiotic after discharge (R6; patient's account) |
| 2026-02-17 | practice | post-hospital visit (P): recovering, tired, no headache; rash faded; exam normal; CBC normal (platelets normal), PT/INR normal. **Acute bacterial meningitis, meningococcemia and thrombocytopenia resolved** |
| 2026-09-10 | practice | booked annual physical (P) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The second ED visit's practice date (2025-02-04) does not fit "returned from a
  spring break trip 2 days ago". Resolution: that visit moves to 2025-03-18, a Tuesday
  in mid-March, two days after he came home from the trip.
- **R2.** The source's alcohol line ("heavier consumption during recent spring break
  trip") sits on every encounter, including the August 2024 and January 2026 ones.
  Resolution: social weekend drinking throughout; the heavier drinking belongs to the
  March 2025 spring-break trip only, mentioned at and after that visit.
- **R3.** The source calls him a sophomore at three encounters a year and a half apart.
  Resolution: the chart says college student living in a campus dormitory and never
  names a class year.
- **R4.** The source's later problem lists carry "meningococcal meningitis" and "acute
  bacterial meningitis" as active, stamped with source dates, and its profile lists
  meningococcemia and thrombocytopenia from the start. Resolution: each condition is
  diagnosed at the ED visit that shows it and resolved at the post-hospital visit that
  follows; meningitis history is carried as history in between. Meningococcal
  meningitis (August 2024) is the Gram-stain-proven first episode. The March 2025
  episode is recorded as acute bacterial meningitis with meningococcemia (purpura,
  hypotension, slow capillary refill) and thrombocytopenia (platelets 100,000). The
  January 2026 episode is recorded as meningococcemia with thrombocytopenia (platelets
  50,000) and clinical bacterial meningitis (nuchal rigidity). No coagulopathy or DIC
  diagnosis is added; the clotting numbers stay as numbers in the ED note.
- **R5.** The March 2025 blood cultures are "pending" in the source, and no organism is
  given for March 2025 or January 2026. Resolution: the chart never receives a culture
  result; the ED notes say what they say and nobody names a serogroup.
- **R6.** The first and third ED notes have no plan and the second stops mid-sentence.
  Resolution: each ED note ends with IV ceftriaxone given in the ED (with IV fluids at
  the two hypotensive visits) and admission to the medicine service. In January 2026 the ED defers the lumbar puncture because of the
  platelet count and prolonged clotting times. The chart has no discharge summaries
  (sparse records): the stays (about 7, 8 and 9 days; discharge 2024-08-10,
  2025-03-26 and 2026-02-09) and "IV antibiotics, nothing to take home" come from the
  patient at the post-hospital visits. No inpatient numbers, ICU course, procedures or
  complications reach the chart.
- **R7.** January 2026 gives 39.2 °C as both 102.5 and 102.6 °F. Resolution: 39.2 °C.
- **R8.** Ages follow the seed (born 2004-08-21): 19 at the August 2024 visit, 20 in
  March 2025 (source says 19), 21 in January 2026 (source says 20). Source years follow
  the re-dated timeline: "January 2020" becomes August 2024, "1/2020, 7/2020" becomes
  8/2024 and 3/2025.
- **Left as the source has it:** after three episodes of meningococcal-type disease,
  nobody orders a complement or other immune evaluation, no further meningococcal
  vaccine (MenACWY booster or MenB) is given, and no note raises either. The physician
  who inherits the chart can find that gap.

## Additions (A): physician to confirm

- **A1.** The 2022-09-13 new-patient visit (four years with the practice; the source
  begins with an ED visit, so he had a primary-care home before it).

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. Post-hospital
  exams are normal: no rash or scarring, neck supple, normal mental status, nonfocal
  neuro exam, normal hearing to conversation.
- Records from his prior pediatric practice arrive after he joins: routine childhood
  care, the tonsillectomy, normal growth, and his immunizations (childhood series,
  MMR ×2, varicella ×2, Tdap at 11, MenACWY at 11 and 16, HPV 9-valent ×2, COVID-19
  primary series in 2021). Nothing abnormal.
- Annual physicals (2023, 2024, 2025) with vitals, PHQ-2 and AUDIT-C screens (negative),
  allergy and acne discussed, no labs beyond those listed here. One missed or
  rescheduled visit. Tdap booster at the 2025 physical (10 years after age 11).
- HIV and hepatitis C screening once (2022-09-13), nonreactive. No lipid or diabetes
  screening at his age.
- Influenza vaccine each fall from 2022; an updated COVID-19 vaccine in some years.
- Post-hospital CBCs (and the BMP in April 2025 and PT/INR in February 2026) with normal
  results.
- College health and housing forms, immunization record requests, portal messages about
  allergy season (stays on cetirizine as needed), acne (stays on adapalene), results and
  scheduling, refills of nothing prescription-only. Family calls after each ED visit
  (he gave permission at joining to share information with his mother).
- Booked visit after 2026-08-27: annual physical 2026-09-10. A portal message in August
  2026 asks for a college health form before the fall semester.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.


## Last seen

The practice last saw MRN00002578 on 2026-02-17. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. A visit is booked after today.
