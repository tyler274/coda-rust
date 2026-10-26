# Build requirements: unit-2555 (Dylan Mazur, MRN00002555)

Source: synthetic_hospital patient 2555, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz-style phrasing ("disrupts enterocyte
chloride channels").

## Who he is (S)

Dylan Mazur, born 1996-03-26, male, White, not Hispanic or Latino, English, lives in
South Hadley (01075), single. University Student Health Insurance Plan. Graduate
student in a master of public health program. Never smoker; social alcohol, 1 to 2
drinks a week; no drug use. Allergies: no known drug allergies. No surgical history.
No chronic conditions, no home medications. Family history: father hypertension
(diagnosed at 54), mother hypothyroidism, no siblings.

Joined the practice 2021-09-14 (A1), age 25, healthy, no problems and no medications.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2021-09-14 | practice | new-patient preventive visit (A1); healthy; history as above; screening labs (P) |
| 2024-12-31 | outside (ED, a regional hospital) | 5 hours of nausea, vomiting, abdominal cramping and watery diarrhea after reheated fried rice (left out for hours at room temperature, then microwaved) at a university potluck the night before; friends who ate other dishes were well; no fever, no blood in stool, no recent travel; no GI history. Vitals within normal limits. Mildly dehydrated, alert and oriented; abdomen soft, mildly tender, nondistended, normal bowel sounds. Labs unremarkable (CBC and BMP, normal values). Improved substantially with supportive care; discharged from the ED. **Bacillus cereus food poisoning** (clinical). Record arrives late (R3) |
| 2025-07-12 | outside (ED, same hospital) | restrained driver, car struck a tree at moderate speed; left upper quadrant pain and dizziness; age 29 (R1). BP 110/70, HR 105, RR 20, SpO2 97% on room air. Abdomen soft, LUQ tender, no rebound or guarding, normal bowel sounds, no flank ecchymosis. Hemoglobin 13.2 g/dL, hematocrit 39.5%, platelets 250 K/uL, WBC 9.0 K/uL, lactate 2.8 mmol/L. FAST negative for free fluid. CT (A2) shows the **traumatic splenic laceration (occult)**; admitted for nonoperative management (A2) |
| 2025-07-12 to 2025-07-14 | outside (admission) | observation with serial hemoglobin, stable; no transfusion, no procedure; discharged with **acetaminophen 500 mg as needed** and activity restrictions (A3) |
| 2025-07-17 | practice | post-hospital visit (P): improving LUQ soreness, no dizziness; CBC normal (A4) |
| 2025-09-09 | practice | 8-week follow-up (A3): asymptomatic; cleared to return gradually to full activity; splenic laceration resolved on the list |
| 2026-02-27 | outside (ED, same hospital) | age 29. Returned 2026-02-25 from a 2-week humanitarian trip to Bangladesh (untreated well water, street-vendor food); sudden profuse, painless watery diarrhea from the evening of 2026-02-26, nearly hourly, no blood or mucus (R2); increasing thirst, lightheadedness, muscle cramps; no fever, no abdominal pain, no recent antibiotics; no medications. BP 90/54, HR 116. Lethargic, sunken eyes, dry mucous membranes, delayed capillary refill, cool extremities, decreased skin turgor; abdomen normal, nontender, nondistended. Stool large-volume, non-bloody, grayish-white, without fecal odor ("rice water"). Stool microscopy: no leukocytes, no ova. Stool culture and toxin testing sent. Suspected toxin-mediated secretory diarrhea, cholera suspected. **Dehydration and hypovolemic shock**; admitted (A5) |
| 2026-02-27 to 2026-03-01 | outside (admission) | IV isotonic fluid resuscitation then oral rehydration; **doxycycline 300 mg once** (A5); shock resolved, diarrhea slowed and stopped. Stool culture 2026-03-01: **Vibrio cholerae O1** isolated, toxigenic (A5) — **cholera** confirmed; reported to the state health department |
| 2026-03-04 | practice | post-discharge visit (P): diarrhea resolved, eating and drinking, no dizziness; BMP normal (A6). Cholera, dehydration and hypovolemic shock resolved |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source calls him 28 at the collision. By the seed's birth date
  (1996-03-26) he is 29 on 2025-07-12, and 29 on 2026-02-27 as the source says. The
  chart uses 29 for both and 28 for 2024-12-31.
- **R2.** The source says the diarrhea began "2 days after returning" and also "the
  previous evening" with arrival home "2 days ago". Resolution: home 2026-02-25, onset
  the evening of 2026-02-26, ED on 2026-02-27.
- **R3.** The source profile lists all three diagnoses at once and its later notes
  carry earlier ones forward as active. Resolution: each starts at its own encounter.
  Bacillus cereus food poisoning is self-limited; the practice marks it resolved at
  the 2025-07-17 visit (the list lags). The splenic laceration resolves on
  2025-09-09. Cholera, dehydration and hypovolemic shock resolve on 2026-03-04. The
  hospital's 2026 ED note may still copy the earlier two as history, as outside
  notes do.
- **R4.** The source's "Laboratory studies are unremarkable" (2024-12-31) is carried
  as a normal CBC and BMP; "vital signs within normal limits" as normal vitals.
- **R5.** The 2026 ED plan stops at a suspected toxin-producing organism with culture
  pending. The chart carries the ED's suspicion in plain words and completes the
  course in A5.
- **Records (chart texture: gaps).** The 2024-12-31 ED visit sends nothing at the
  time; the practice learns of it from a portal message on 2025-01-02, and the ED
  note arrives 2025-08-08 bundled with the July records. The July 2025 ED and
  admission records arrive 2025-08-08, after a records request; the 2025-07-17 visit
  works from the patient's account and his discharge paperwork. The 2026 ED and
  admission records arrive 2026-04-09 after a second request; the 2026-03-04 visit
  works from the patient's account and the culture result he saw on the hospital
  portal. Admission labs beyond those listed here never reach the chart.
- **Left as the source has it:** no pre-travel consultation, travel vaccines or
  traveler's-diarrhea counseling are recorded before the Bangladesh trip; the
  practice did not know of the trip in advance, and nobody remarks on it.

## Additions (A): physician to confirm

- **A1.** 2021-09-14 new-patient visit (five years with the practice per the seed).
- **A2.** CT abdomen and pelvis with IV contrast in the ED, 2025-07-12: laceration of
  the spleen without active contrast extravasation and without hemoperitoneum; liver,
  kidneys, pancreas, bowel and bones without injury. "Occult" = missed by FAST, found
  on CT. Admission for observation and nonoperative management.
- **A3.** At discharge: acetaminophen 500 mg 1 to 2 tablets every 6 hours as needed
  (short course), avoid NSAIDs; no contact sports or heavy lifting for 8 weeks. The
  8-week practice visit clears him.
- **A4.** CBC on 2025-07-17: hemoglobin 14.1 g/dL; otherwise normal.
- **A5.** Cholera course: admission 2026-02-27 to 2026-03-01; IV fluids then oral
  rehydration; doxycycline 300 mg orally once; stool culture positive for Vibrio
  cholerae O1, toxigenic; public health report by the hospital.
- **A6.** BMP on 2026-03-04, normal values.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Records from a prior practice (college health service) may mention routine care
  there, with normal results.
- Yearly preventive visits (2022, 2023, 2024, 2025); one no-show in fall 2023; weight
  and BP normal throughout (height 180 cm, weight about 76 kg).
- Prevention: HIV and hepatitis C screening once (2021, nonreactive); a lipid panel
  once (2021, normal); Tdap 2021; influenza each fall; COVID primary series before
  joining (reported) and boosters/updated doses at a pharmacy; PHQ-2 at physicals,
  negative.
- Portal messages about results and scheduling, a school-absence letter, records
  requests, a missed visit, reminders.
- Booked visit after 2026-08-27: annual physical 2026-10-13.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.
