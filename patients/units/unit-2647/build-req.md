# Build requirements: unit-2647 (Silas Mahoney, MRN00002647)

Source: synthetic_hospital patient 2647, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its "as shown in the image" text.

## Who he is (S)

Silas Mahoney, born 1997-08-13, male, White, not Hispanic or Latino, English, Medicaid,
lives in South Hadley. Farm laborer, works outdoors in a rural area most of the day.
Never smoker, no alcohol, no illicit drugs. No known drug allergies. No surgical
history. Family history: mother with lifelong recurrent nosebleeds; maternal
grandfather with frequent nosebleeds and unexplained iron deficiency anemia (R1).
Had not seen a doctor in several years before joining. Joins the practice at the
first source encounter, 2024-01-08 (a new-patient visit), with no problems on any list
and no medications.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2023-12 | practice | calls to become a patient (P) |
| 2024-01-08 | practice | new-patient visit. About a year of spontaneous nosebleeds, several episodes, usually stop with pressure, occasionally needed packing; increasing fatigue and shortness of breath climbing stairs. No known bleeding disorder. Exam: pale conjunctivae; numerous pinpoint red lesions (telangiectasias) on the lips, tongue and oral mucosa. Hemoglobin 9.1 g/dL, MCV 72 fL. Iron studies (A1): ferritin 6 ng/mL, iron 22 ug/dL, TIBC 452 ug/dL, transferrin saturation 5%. **Hereditary hemorrhagic telangiectasia diagnosed** on clinical grounds (recurrent epistaxis, mucosal telangiectasias, affected first-degree relative) and **iron deficiency anemia diagnosed**. CT chest ordered to look for pulmonary vascular malformations (R2). Hematology referral (A2) |
| 2024-01-09 | practice | results message; **ferrous sulfate 325 mg daily started** (A3) |
| 2024-01-12 | outside imaging | CT chest with contrast: multiple pulmonary arteriovenous malformations (R2). **Pulmonary arteriovenous malformations added** to the list on 2024-01-16 |
| 2024-01-16 | practice | phone call with the result; referral to an HHT center for the pulmonary AVMs (A4) |
| 2024-05-24 | practice | follow-up (last seen; A5): stopped the iron after about three weeks for constipation; nosebleeds continue; still tired. Pale, oral telangiectasias. Hemoglobin 9.4, MCV 71. **Daily ferrous sulfate stopped; ferrous sulfate 325 mg every other day started.** Hematology and HHT center referrals re-sent |
| 2024-12-23 | outside | hematology consult (first visit there; R3): worsening fatigue and lower exercise tolerance for 4 months; outdoor rural farm work; takes no medications (R4); denies overt bleeding, easy bruising, night sweats, recent illness; occasional mild abdominal discomfort and intermittent loose stools, no nausea, vomiting or jaundice. Vitals normal. Pale and thin, mild conjunctival pallor, spoon-shaped nails, grade 2/6 systolic flow murmur at the left sternal border, mild diffuse abdominal tenderness without organomegaly. Hemoglobin 9.3, MCV 70, ferritin 7, WBC 7,800, eosinophils 8%. Stool microscopy: oval, thin-shelled ova (hookworm). **Necator americanus infection diagnosed** (R5). **Albendazole 400 mg once** (A6); resume the prescribed iron every other day; hematology follow-up in 3 months. Letter reaches the practice late (Records) |
| 2026-02-24 | outside | ED visit at age 28: sudden severe shortness of breath on exertion and profound fatigue; returned about three weeks earlier from 6 months of volunteer rural agricultural work in Southeast Asia, often barefoot in fields, a mildly itchy rash on the soles that went away by itself. Denies overt bleeding, weight loss, diet change; no fever, melena or hematochezia. Pale, slightly tachycardic (heart rate 104, R6), pale conjunctivae, mild systolic flow murmur, abdomen unremarkable, no lymphadenopathy. Hemoglobin 9.6, MCV 72, ferritin low (5 ng/mL, R6), reticulocytes mildly elevated (2.6%, R6). Stool ova and parasite exam ordered. Discharged home to follow up with primary care and hematology |
| 2026-02-27 | outside | ED result callback (A7): stool O&P shows hookworm ova; **hookworm infection (recurrent) diagnosed**; **albendazole 400 mg once** sent |

No practice visit after 2024-05-24.

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Encounter 0 puts the similar symptoms in his father; the profile and both later
  notes give his mother and maternal grandfather. Resolution: mother and maternal
  grandfather only; the father is not carried.
- **R2.** The source puts a chest CT inside the 2024-01-08 visit. Resolution: the CT is
  ordered at that visit and done at an outside imaging center on 2024-01-12, read as
  multiple pulmonary arteriovenous malformations with no added sizes or locations. The
  "as shown in the image" lesions are described in words only.
- **R3.** The hematology note says he "returns for evaluation". Resolution: it is his
  first hematology visit, from the practice's referral; "returns" means returning to
  care after a lapse. Its "has not seen a doctor in several years" refers to the years
  before he joined the practice.
- **R4.** The practice prescribed iron in 2024 (A3), but hematology records no
  medications. Resolution: he had stopped it again by December; the practice's list
  still carries every-other-day ferrous sulfate, because nothing was reconciled after
  2024-05-24.
- **R5.** The source lists "Ancylostomiasis (hookworm infection)" and "Necator americanus
  infection" as two diagnoses. Resolution: the 2024 infection is recorded by hematology
  as Necator americanus infection (B76.1; the reference parasitology lab reported the
  species); the 2026 reinfection after travel is recorded as hookworm infection
  (ancylostomiasis, species not reported; B76.9). One infection each time.
- **R6.** The ED note says "slightly tachycardic", "ferritin low" and "reticulocyte count
  mildly elevated" without values. Resolution: heart rate 104, ferritin 5 ng/mL,
  reticulocytes 2.6%. Other ED vitals ordinary (BP 116/70, RR 18, SpO2 97%, 36.8 C).
- **Left as the source has it:** the pulmonary AVMs are never treated or evaluated by a
  specialist (the referral in A4 is never completed), and no one after 2024 remarks on
  them; iron stays oral and he is mostly not taking it; the hookworm cure is never
  confirmed by repeat stool testing; the hematology 3-month follow-up never reaches the
  chart. The physician who inherits the chart can find these gaps.

## Additions (A): physician to confirm

- **A1.** Iron studies at the first visit (ferritin 6, iron 22, TIBC 452, saturation 5%)
  to support the iron deficiency anemia the source names.
- **A2.** Hematology referral at the first visit (the source sends him to hematology).
- **A3.** Ferrous sulfate 325 mg daily for a hemoglobin of 9.1 with iron deficiency.
- **A4.** HHT center referral for pulmonary AVM evaluation after the CT. He never
  schedules; it stays open.
- **A5.** The 2024-05-24 follow-up visit (last seen): hemoglobin 9.4, MCV 71; iron
  changed to every other day for constipation.
- **A6.** Albendazole 400 mg as a single dose from hematology (the ED note says the 2024
  infection was "managed").
- **A7.** The O&P result and treatment after the ED visit (the source's plan stops at
  ordering it): hookworm ova, albendazole 400 mg once, from the ED.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- At the first visit: CMP, HIV and hepatitis C screening, all normal; PHQ-9 low;
  influenza vaccine. Height 178 cm, weight 64 kg; vitals otherwise normal.
- Records requests (he had no recent prior primary care), referral coordination,
  portal messages about results, nosebleed first aid and scheduling, a missed
  follow-up in March 2024, an iron refill, flu reminders, a records copy he asks for
  before travelling.
- Records (chart texture: gaps): the practice learns of the hematology visit from a
  pharmacy fill history showing albendazole, requests the letter twice, and receives
  it in April 2025. The ED record and callback arrive by fax in March 2026. Neither
  changes the practice's lists. Outreach to schedule after each goes unanswered.
- Nothing booked after today.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002647 on 2024-05-24. Practice contacts after that day do not
happen; outside care after it still does, and its records are filed as received without
changing the practice's lists. Nothing is booked after today.
