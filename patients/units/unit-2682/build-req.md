# Build requirements: unit-2682 (Rodney Howland, MRN00002682)

Source: synthetic_hospital patient 2682, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its unfinished lines ("Is currently.").

## Who he is (S)

Born 1991-02-07, male, White, not Hispanic or Latino, English-speaking, lives in
Westfield (01085), employer-sponsored commercial insurance. High school mathematics
teacher. Never smoker. Alcohol: occasional social use, 1 to 2 drinks a month. No known
drug allergies. Family history: father and paternal grandfather with hereditary
spherocytosis. No surgery before the 2024 splenectomy.

Joined the practice on 2023-09-12 (A1, new-patient visit), age 32. Brought:
hereditary spherocytosis (known since childhood, R2) with chronic compensated
hemolysis, on **folic acid 1 mg daily** (historical, continued). Spleen intact on
joining.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2023-09-12 | practice | new-patient visit (A1): history as above; no symptoms; weight about 79 kg, height 180 cm; BP normal. Labs: CBC, CMP, hepatic panel (hemoglobin about 11.2, MCV about 92, total bilirubin about 2.1 with direct about 0.3: his baseline, see A6), HIV and hepatitis C screens negative. Influenza vaccine. His hematology follow-up had lapsed; **hematology referral** placed to re-establish care and revisit splenectomy (A2) |
| 2023-09-26 | outside records | prior primary-care records received: hereditary spherocytosis since childhood, folic acid, Tdap 2018, no other problems (P) |
| 2023-10-10 | practice | nurse visit: COVID-19 vaccine (P) |
| 2024-01-23 | outside | hematology consult letter (A2), received 2024-01-29: hereditary spherocytosis with chronic hemolysis and splenomegaly; options discussed; he elects laparoscopic splenectomy during the summer school break; referred to general surgery; folic acid continued; vaccines to be completed through primary care at least 2 weeks before surgery |
| 2024-06-14 | outside | hematology pre-operative visit (S), letter received 2024-06-17: elective splenectomy scheduled in 3 weeks; vaccinations up to date on the standard adult schedule; T 37.2 C (99 F), BP 130/80, pulse 78, RR 16; spleen palpable 3 cm below the left costal margin; hemoglobin 11.2 g/dL, MCV 92 fL, total bilirubin 2.1 mg/dL (direct 0.3 mg/dL), LDH elevated, haptoglobin decreased (no values carried). Plan: splenectomy; asplenia vaccines at the practice (A3); folic acid continued; penicillin V prophylaxis to begin after surgery (R1) |
| 2024-06-18 | practice | nurse visit (A3): pneumococcal conjugate PCV20, MenACWY, MenB-4C dose 1, Hib, 15 days before surgery |
| 2024-07-03 to 2024-07-05 | outside | general surgery admission (A4): **elective laparoscopic splenectomy** 2024-07-03, uncomplicated, home 2024-07-05; **asplenia (acquired absence of spleen) begins**; **penicillin V potassium 250 mg twice daily started** at discharge (R1); folic acid continued. Discharge summary and operative note received 2024-07-09 |
| 2024-07-19 | practice | post-operative visit (P): incisions healing; no fever; asplenia and overwhelming post-splenectomy infection (OPSI) risk counseled (fever means same-day evaluation, carry a card, medical alert); MenB-4C dose 2 (A3); CBC (hemoglobin 12.6, A6) |
| 2024-09-24 | outside | hematology follow-up letter (A5), received 2024-09-30: well after splenectomy; hemoglobin 13.4 g/dL, total bilirubin 0.9 mg/dL; continue penicillin V and folic acid; hematology yearly |
| 2024-10-08 | practice | nurse visit: influenza and COVID-19 vaccines (P) |
| 2024-12-19 | practice | **last seen.** Annual physical (P): well; vitals normal; CBC, CMP, lipid panel normal range (hemoglobin about 13.4, A6); PHQ-2 0; problem list: hereditary spherocytosis, asplenia after splenectomy; meds: folic acid, penicillin V |
| 2025 to 2026 | messages | refills of penicillin V and folic acid through the pharmacy; a portal message about scheduling his next physical that he does not book; a fall 2025 influenza vaccine given at a retail pharmacy, reported to the practice (P) |
| 2026-06-12 | outside | ED visit (S), record received 2026-06-16: age 35; 2 days of fever and sore throat, home temperature 101.5 F (38.6 C); asplenic since the July 2024 splenectomy (R3); meds folic acid 1 mg daily and penicillin V 250 mg twice daily; NKDA; T 102.2 F (39 C), BP 130/80, pulse 88, RR 16; oropharynx red without exudate, no cervical nodes, rest of exam normal; WBC 12,000/uL, hemoglobin 13.2 g/dL, platelets 250,000/uL; blood cultures drawn. **Ceftriaxone 2 g IV once** after cultures, discharged home with return precautions and primary-care follow-up (A7) |
| 2026-06-13 | message | patient portal message that he went to the ED and feels better; PCP reply (P) |
| 2026-06-18 | outside records | ED blood culture final: no growth at 5 days (A7), filed |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The pre-operative note (splenectomy "in 3 weeks") already lists asplenia,
  the splenectomy as done, and penicillin V prophylaxis, while the spleen is palpable.
  Resolution: on 2024-06-14 the spleen is intact and enlarged; asplenia and the
  splenectomy start on 2024-07-03, and penicillin V 250 mg twice daily starts at
  discharge on 2024-07-05. The hematology letter of June plans penicillin for after
  surgery; it does not list it as a current drug.
- **R2.** The source stamps hereditary spherocytosis "diagnosed 2020-01-15" (re-dated:
  the June 2024 visit), yet the same visit is a pre-splenectomy visit for an
  established diagnosis with an affected father. Resolution: known since childhood,
  brought to the practice on joining; the stamp is dropped.
- **R3.** The ED note says the splenectomy was "in 2020" and also "5 years ago".
  Resolution: the splenectomy was 2024-07-03, about two years before the ED visit;
  the ED record says so. "Presents to the clinic" in an ED note becomes an ED
  presentation.
- **R4.** The source puts "Overwhelming post-splenectomy infection (OPSI) risk" as a
  diagnosis on the pre-operative visit. Resolution: OPSI risk is counseling attached
  to asplenia from July 2024, written in notes; it is not a separate problem-list
  entry, and it does not exist before surgery.
- The two blood pressures of 130/80 are carried in the outside records as written;
  practice readings run normal (interobserver/setting difference).
- **Left as the source has it:** the ED source gives no throat swab, no named
  infection and no change in prophylaxis; the chart adds none. Folic acid continues
  after splenectomy as the source lists it in 2026; nobody stops it or comments.
  No standby antibiotic is prescribed.

## Additions (A): physician to confirm

- **A1.** 2023-09-12 new-patient visit (years with the practice: 3; the source starts
  with a specialist pre-operative visit, so he had a primary-care home before it).
- **A2.** Hematology referral at A1 and the January 2024 hematology consult that sets
  up the elective splenectomy (the source's June visit is a pre-operative one, so the
  decision came earlier).
- **A3.** Pre-splenectomy vaccines at least 14 days before elective surgery, as any
  physician would give: PCV20, MenACWY, MenB-4C (2 doses, 2024-06-18 and
  2024-07-19), Hib. This is how "vaccinations up to date" is made true for an asplenic
  patient.
- **A4.** The splenectomy admission itself (the source states the operation happened).
  Uncomplicated, laparoscopic, 2 nights.
- **A5.** One hematology follow-up after surgery: hemoglobin 13.4 g/dL, total
  bilirubin 0.9 mg/dL.
- **A6.** Lab levels around the source numbers: before surgery hemoglobin about 11.2,
  MCV about 92, total bilirubin about 2.1 (direct about 0.3), reticulocytes not
  measured; two weeks after surgery hemoglobin about 12.6; from September 2024
  hemoglobin about 12.8 to 13.6 and total bilirubin under 1.2; platelets normal
  throughout. The ED's 13.2 sits in that range.
- **A7.** ED course for a febrile asplenic patient: ceftriaxone 2 g IV once after
  blood cultures, discharged; blood cultures final no growth at 5 days.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not
  mention. An abnormal finding appears only as the source or this file gives it.
- Records from a prior practice may mention routine screening done there, with normal
  results (Tdap 2018).
- A yearly physical with CBC, CMP and lipids; PHQ-2; HIV and hepatitis C screening
  once (2023, negative).
- Influenza vaccine each fall (2023 at the practice, 2024 at the practice, 2025 at a
  pharmacy); COVID-19 vaccines fall 2023 and 2024.
- Refills, pharmacy traffic, portal messages about results, refills and scheduling.
- No booked visit after 2026-08-27.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002682 on 2024-12-19. Practice contacts after that day are
messages, refills and outside records filed as received, without changing the
practice's lists. Nothing is booked.
