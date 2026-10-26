# Build requirements: unit-1843 (Maricel Chanthavong, MRN00001843)

Source: synthetic_hospital patient 1843, in `source.md` beside this file. This file and
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

## Who she is (S)

Born 1996-11-04, female, Asian (Thai family background), English-speaking, lives in
the seed's town, university student health insurance plan. Student throughout (college,
later continuing study), never smoker, no alcohol, no drug allergies (NKDA), no
surgical history. Family history: father with beta-thalassemia trait, paternal
grandmother with beta-thalassemia major, thalassemia in the extended family in
Thai relatives. Married by 2025 (her husband is part of the 2025 genetics consult).

Joined the practice on 2017-08-22 (A1), age 20: a new-patient visit; no known
problems; takes a multivitamin 1 tablet daily; family history as above. Height 157 cm,
weight 52 kg.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2017-08-22 | practice | new-patient visit (A1); history as above; normal exam; no labs |
| 2019-05-08 | practice | mild fatigue; exam unremarkable; CBC same day: **hemoglobin 11.0 g/dL, MCV 68 fL**; TSH normal (A2). Thalassemia trait suspected from the family history and microcytosis; hemoglobin electrophoresis and HBB gene analysis sent |
| 2019-05-14 | practice (result) | electrophoresis: **HbA2 increased (5.4%), HbF mildly increased (2.3%)** (numbers A3), no variant hemoglobins. **Beta thalassemia minor diagnosed** (R1) |
| 2019-06-04 | practice (result) | HBB gene analysis: heterozygous for a promoter-region HBB variant, a beta-plus allele with reduced (about half) beta-globin output (R2) |
| 2020-11-16 | practice | fatigue and mild exertional breathlessness for 6 months; she puts it down to a busy school schedule; balanced diet; no gum bleeding, nosebleeds or heavy periods. Well-nourished, no distress, no pallor of skin or conjunctivae, no icterus, no lymphadenopathy, no hepatosplenomegaly. CBC: **Hb 10.2, MCV 72, RDW 12%, platelets 320, WBC 6.0**; iron studies drawn: **iron 90, TIBC 300, ferritin 200**. No iron deficiency; anemia from the trait; no iron prescribed (R3) |
| 2022-11-14 | outside (ED) | after finishing her first marathon: headache, nausea, confusion; drank large amounts of water before and during the race, salt tablets now and then per friends. BP 130/85, HR 88, RR 16, T 37.2 °C (99.0 °F). Disoriented, no focal neurologic deficit. **Na 125, K 4.2, Cl 96, HCO3 24, BUN 10, Cr 0.9, glucose 90, serum osmolality 260, urine specific gravity 1.005**. **Exercise-associated hyponatremia**. ED course (A4): 100 mL 3% saline bolus, Na 129 two hours later, held overnight in observation, Na 133 the next morning, mental status back to normal; discharged 2022-11-15 |
| 2022-11-16 | practice (portal) | she messages about the ED stay; follow-up booked; records requested (P) |
| 2022-11-21 | practice | post-ED visit (A5): well, no confusion or headache; BMP sodium 139 (A5); **hyponatremia resolved**; hydration-by-thirst advice for future races |
| 2022-12-06 | practice (records) | ED record received after the request (late, per chart texture) |
| 2025-03-11 | practice | planning pregnancy with her husband (A6): referred to medical genetics for preconception counseling given her trait; suggests her husband get carrier testing; **multivitamin stopped, prenatal vitamin 1 tablet daily started** (R5) |
| 2025-05-19 | outside (genetics) | preconception counseling with her husband; they ask what the beta-globin outcomes could be for a child. Both have elevated HbA2 on electrophoresis. Husband: heterozygous for a null HBB mutation (no beta-globin from that allele, beta-zero). Her: heterozygous for a different HBB allele, promoter-region, about 50% reduced beta-globin output. Letter states the standard counseling answer (R6). Letter received 2025-06-03 |
| 2025-10-06 | practice (portal) | positive home pregnancy test; last period began 2025-09-01; has an obstetric intake booked (A7) |
| 2025-11-06 | outside (OB) | initial prenatal visit (about 9 weeks): feels well, excited; no fatigue, dizziness or dyspnea; taking prenatal vitamins regularly; vital signs normal; exam unremarkable except **mild conjunctival pallor**, no hepatosplenomegaly or lymphadenopathy. Routine prenatal labs drawn. Hemoglobin electrophoresis with elevated HbA2, consistent with her known beta-thalassemia trait (R7). **Oral iron started empirically (ferrous sulfate 325 mg daily, A8)**. Plan: repeat bloodwork, partner's carrier result reviewed, referred back to genetics (R8). No letter sent at the time |
| 2025-11-25 | outside (OB) | follow-up about three weeks later (R9): taking the iron consistently, feels no different; repeat bloodwork shows no meaningful change from the initial labs |
| 2026-04-03 | practice (last seen) | office visit at about 30 weeks: asks whether she needs the iron; well; medication list reconciled (ferrous sulfate 325 mg daily from OB recorded as historical; prenatal vitamin continues); **pregnancy recorded** (A7); CBC and ferritin drawn: **Hb 10.0, MCV 70, ferritin 168** (A9); OB records requested |
| 2026-04-06 | practice (result) | result message: iron stores full; a note to the OB suggesting the iron may not be needed; she is to follow the OB's advice (A9). Practice list unchanged |
| 2026-04-16 | practice (records) | OB prenatal summary (2025-11-06 and 2025-11-25 visits) received and filed after last seen |
| 2026-06-22 | practice (portal) | she reports a term vaginal delivery on 2026-06-10 at a regional hospital, she and the baby well, postpartum care with her OB (A10). No hospital summary ever arrives. Lists not updated |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source lists beta-thalassemia minor as chronic from the start and also
  "Thalassemia minor (diagnosed 2020-01-15)". These are one condition, recorded once as
  Beta thalassemia minor, first documented when the 2019-05-14 electrophoresis returns.
  The 2017 new-patient visit knows only the family history.
- **R2.** The 2019 genetic analysis says "splice-site mutation"; the 2025 genetics
  consult says her allele is a promoter-region mutation with about 50% reduced output.
  One heterozygous allele cannot be both. The chart carries the promoter-region beta-plus
  variant in both; "splice-site" is not carried. No specific variant name is given.
- **R3.** The source gives no plan at the 2020 visit. Resolution: iron studies rule out
  iron deficiency; fatigue attributed to the trait plus her schedule; no iron, return if
  worse. MCV 68 in 2019 and 72 in 2020 are both carried (lab scatter).
- **R4.** "Delayed management of life-threatening respiratory failure" (comorbidity) is
  quiz residue with no supporting encounter: not carried. The source's 2022 age of 25
  becomes 26 from the seed birth date. Hyponatremia is resolved on 2022-11-21; later
  outside lists (genetics 2025, OB 2025) may still copy it as a past problem.
- **R5.** The OB note says she takes prenatal vitamins and also lists the multivitamin.
  Resolution: the prenatal vitamin replaces the multivitamin at the 2025-03-11 practice
  visit; the OB summary may still copy the multivitamin, as outside lists do.
- **R6.** The genetics source poses a question. Resolution: the letter says each
  pregnancy has a 1 in 4 chance of inheriting both altered alleles (beta-zero/beta-plus,
  a thalassemia intermedia to major range), 1 in 2 of carrying one trait like a parent,
  1 in 4 of neither; prenatal diagnostic options are described. No quiz text carried.
- **R7.** The OB note says electrophoresis is "consistent with α-thalassemia trait",
  which contradicts the elevated HbA2 found in 2019 and 2025 and her known HBB variant.
  Not carried: the OB electrophoresis shows elevated HbA2 consistent with her known
  beta-thalassemia trait. No alpha-thalassemia diagnosis anywhere.
- **R8.** The OB note says she "recently immigrated", that relatives had anemia but she
  is unsure of details, and that her partner is referred for screening and counseling
  is arranged. She has been a practice patient since 2017 and the couple were already
  tested and counseled. Resolution: no immigration is carried; the OB summary lists the
  family history as known; the partner's carrier result is noted as on file and she is
  referred back to genetics. Whether that genetics follow-up happened never reaches
  the practice.
- **R9.** The OB "follow-up three weeks later" is dated 2025-11-25 (three weeks less
  two days, avoiding a holiday). The OB review of systems (no fatigue) and the follow-up
  ("no improvement in how she feels") are read together: she felt well and the iron
  changed nothing.
- **Left as the source has it:** no head imaging at the ED despite confusion; empiric
  iron started by the OB despite her 2020 iron studies (until the practice's 2026 note,
  A9, which only suggests); no prenatal diagnostic testing appears in the records the
  practice receives. No note comments on these.

## Additions (A): physician to confirm

- **A1.** The 2017-08-22 new-patient visit (the source's second visit says she
  "returns", and the seed gives about 9 years with the practice).
- **A2.** A TSH with the 2019 fatigue CBC, normal.
- **A3.** Numbers for the 2019 electrophoresis the source describes only in words:
  HbA2 5.4%, HbF 2.3%.
- **A4.** The ED course and disposition, which the source omits: one 100 mL bolus of
  3% saline, Na 129 at two hours, overnight observation, Na 133 next morning,
  discharged 2022-11-15.
- **A5.** The post-ED practice visit on 2022-11-21 with sodium 139.
- **A6.** The 2025-03-11 practice visit that refers her to genetics.
- **A7.** The pregnancy the OB encounter needs: last period 2025-09-01, due about
  2026-06-08; recorded on the practice list as Supervision of normal pregnancy at the
  2026-04-03 visit (the list lags, per chart texture).
- **A8.** The iron the OB starts, given its usual form: ferrous sulfate 325 mg by mouth
  daily.
- **A9.** The 2026-04-03 practice visit (last seen) and its draw: Hb 10.0, MCV 70,
  ferritin 168; the result note to the OB suggesting the iron may be unnecessary.
- **A10.** The pregnancy's course to today: term vaginal delivery on 2026-06-10, mother
  and baby well, known to the practice only from her portal message.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Preventive visits about yearly but uneven: 2018-06, 2021-06, 2023-05 and 2024-05; a
  physical cancelled in 2022. Vitals each visit, weight steady 51 to 54 kg before
  pregnancy (about 61 kg at 30 weeks). A CBC now and then shows her own baseline:
  hemoglobin about 10.5 to 11.0, MCV about 68 to 72.
- Cervical cytology at 21 and every three years (2018, 2021, 2024), all normal. HIV and
  hepatitis C screening once (2018), negative. PHQ-2 at physicals, negative.
- Influenza vaccine most falls (some at a pharmacy, reported later); COVID-19 primary
  series in 2021 and boosters as recommended, reported by her and recorded late.
- Portal messages about results, scheduling, school forms; a records request.
- Booked visit after 2026-08-27: a postpartum preventive visit.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00001843 on 2026-04-03. Practice contacts after that day are
only messages, the result of that day's draw, and records filed as received; no
practice visit follows, and the problem and medication lists stay as they were on
2026-04-03. A visit is booked after today.
