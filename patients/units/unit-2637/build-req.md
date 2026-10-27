# Build requirements: unit-2637 (Andre Mensah, MRN00002637)

Source: synthetic_hospital patient 2637, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz text ("approximately 1 in 13...").

## Who he is (S)

Andre Mensah, born 1997-11-14, male, Black, not Hispanic, English-speaking, lives in
Amherst (01002), employer-sponsored private insurance, software developer, never
smoker, no alcohol. No known drug allergies. No surgical history. Married (his wife
appears in 2026; no children). Family history: father with sickle cell trait, a
maternal cousin with sickle cell disease, mother with hypertension.

Sickle cell labelled since newborn screening; painful limb and back episodes since
childhood, several of them admitted for IV pain control in childhood and adolescence;
none admitted in the last few years; milder episodes every few weeks that settle with
rest and oral fluids at home. Vaccines reported up to date.

Joined the practice 2021-09-08 (new patient visit, A1). Brought: sickle cell trait (his
records' label), recurrent vaso-occlusive pain crises, sickle cell disease without
crisis (R1); hydroxyurea 1000 mg by mouth daily and folic acid 1 mg by mouth daily,
both prescribed before he joined (the practice takes over the refills, P).

Height 178 cm. Weight about 71 to 73 kg throughout.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2021-09-08 | practice | new patient visit (A1); history as above; exam normal apart from mild pallor; baseline CBC, reticulocytes, CMP, urinalysis; HIV and hepatitis C screening (P) |
| 2023-03-08 | practice | several months of fatigue and generalized weakness, tires on stairs, occasional lightheadedness on standing; no bleeding, fever or recent illness; fair appetite, mixed diet; milder pain episodes every few weeks, no admission in the past year; **admits he sometimes forgets the folic acid**; vitals normal; pale conjunctivae, systolic flow murmur, no icterus, no lymphadenopathy, abdomen benign, spleen not palpable, neuro normal. Hemoglobin 8.8 g/dL, MCV 112 fL, reticulocytes 8%, WBC 7.2, platelets 430, total bilirubin 2.3 mg/dL, LDH 418 U/L; serum folate 2.4 ng/mL, B12 486 pg/mL (A2). **Folate deficiency due to increased requirement from erythroid hyperplasia diagnosed**; folic acid continued at 1 mg daily with an adherence plan (pill organizer, phone reminder); CBC recheck in about 8 weeks |
| 2023-05-03 | practice | recheck lab and short visit: taking folate daily; less tired; hemoglobin 9.4, MCV 104, folate 9.8 ng/mL (A3) |
| 2024-04-17 | practice | about 2 months of more frequent urination and getting up at night to void, more tired again; no dysuria, fever or back pain; urinalysis dilute (specific gravity 1.006), no protein, glucose or blood; BMP glucose normal, sodium normal, creatinine 1.21; referred to nephrology (A4) |
| 2024-08-27 | outside | nephrology consult (letter received by the practice): six months of increased urination, nocturia, generalized fatigue; no dysuria, fever or back pain; BP 130/80, HR 78, afebrile; abdomen soft, non-tender, no CVA tenderness; sodium 140, potassium 4.2, creatinine 1.2 (R2), urine specific gravity 1.005. **Hyposthenuria secondary to sickle cell trait** and **chronic kidney disease, stage 2** diagnosed (R3). Plan in the letter (R4): steady fluid intake through the day, front-load fluids earlier and limit them in the last two hours before bed, avoid NSAIDs and dehydration, creatinine and urine albumin/creatinine yearly through primary care, nephrology as needed |
| 2026-03-09 | outside | preconception counseling at his wife's obstetrics practice, he attends as the partner (note copied to the practice): the couple worry about a child who would carry the sickle allele without disease; his records were reviewed; **his wife's hemoglobin electrophoresis has not been done and is recommended**; counseling on inheritance by the partner's carrier status (R5) |
| 2026-07-17 | practice | last practice visit (P): routine follow-up; pain episodes unchanged; urination pattern improved with fluid timing; CBC, reticulocytes, CMP; refills |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source calls him sickle cell trait in every note (newborn screen,
  nephrology, preconception counseling) yet gives him a disease course: crises since
  childhood with admissions, hydroxyurea, hemoglobin 8.8 with reticulocytes 8%,
  bilirubin 2.3 and LDH 418, and it lists "sickle cell anemia without crisis" as a
  comorbidity. Resolution: the chart carries both labels as the source does, from
  joining: "Sickle-cell trait" (his records' label) and "Sickle-cell disease without
  crisis", with recurrent vaso-occlusive pain crises. Hydroxyurea continues. His
  baseline counts are those of a hemolytic state on hydroxyurea: hemoglobin about 9.3
  to 9.8, MCV about 102 to 106, reticulocytes about 5 to 7%, total bilirubin about 1.6
  to 2.0, LDH about 300 to 360, platelets about 350 to 420, WBC about 6 to 7.5.
- **R2.** The source's nephrology creatinine of 0.9 gives an eGFR near 120 in a man of
  26, which cannot carry a stage 2 label. Resolution: 0.9 is not carried; the
  nephrology creatinine is 1.2 (eGFR about 85). Practice creatinine runs about 1.0 to
  1.1 before 2024 and about 1.15 to 1.25 from 2024 (eGFR roughly 80 to 90).
- **R3.** The source lists hyposthenuria and CKD stage 2 from the first encounter.
  Resolution: both are diagnosed at the 2024 nephrology consult; the 2021 to 2023
  notes never mention them. The 2023 fatigue visit's problem list does not carry them.
  The folate deficiency stamp ("diagnosed" at the first visit) maps to 2023-03-08.
- **R4.** The nephrology plan stops mid-sentence. Resolution: the plan above; no drug,
  no imaging.
- **R5.** The counseling note says his earlier electrophoresis showed "only HbA
  bands", which fits neither trait nor disease. Resolution: dropped; the note says his
  records were reviewed and does not quote a pattern. The source's quiz text on the
  missense mutation and carrier frequency becomes ordinary counseling (each child's
  chance depends on whether his wife carries an abnormal hemoglobin; her
  electrophoresis first). The source's "son" concern is carried as the couple's
  question about a child.
- **R6.** The nephrology note says "does not take any medications" and then lists
  hydroxyurea and folic acid. Resolution: he takes both; the letter's medication list
  carries both.
- **Left as the source has it:** the trait label and the disease picture are never
  reconciled; no repeat hemoglobin electrophoresis or hemoglobin pattern study is
  ordered; no hematology referral. Hydroxyurea continues at 1000 mg without dose
  review. Nobody comments on any of it.

## Additions (A): physician to confirm

- **A1.** 2021-09-08 new patient visit (the 2023 encounter reads as an established
  patient's problem visit; the seed gives about five years with the practice).
- **A2.** Serum folate 2.4 ng/mL and B12 486 pg/mL at the 2023 fatigue visit, to support
  the source's folate deficiency.
- **A3.** The 2023-05-03 recheck (hemoglobin 9.4, MCV 104, folate 9.8).
- **A4.** The 2024-04-17 practice visit that sends him to nephrology (urine specific
  gravity 1.006, creatinine 1.21).

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it (pallor and a
  flow murmur may recur on later exams as his baseline).
- Records from a prior practice (received at joining) may mention routine care there
  with normal results; they carry the same labels and drugs he brought.
- Follow-up about every 4 to 6 months and a yearly physical; CBC with reticulocytes
  every 3 to 4 months for hydroxyurea; CMP or BMP at least yearly; from 2024 urine
  albumin/creatinine yearly (normal); lipids once (normal).
- Influenza vaccine each fall, COVID vaccine per season, Tdap if due. HIV and hepatitis C
  screening once (negative). Depression screens negative.
- Refills of hydroxyurea and folic acid, pharmacy traffic, portal messages about
  results, refills and scheduling, a missed visit, reminders.
- Nothing booked after today.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002637 on 2026-07-17. Practice contacts after that day do
not happen; outside care after it still does, and its records are filed as received
without changing the practice's lists. Nothing is booked.
