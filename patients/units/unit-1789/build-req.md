# Build requirements: unit-1789 (Sonia Soto, MRN00001789)

Source: synthetic_hospital patient 1789, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, its truncated plan ("A genetic defect in") or its
quiz clues (the rods that "grow well at 4 degrees").

## Who she is (S)

Sonia Soto, born 1999-06-06, female, Hispanic or Latino (race recorded as other),
English-speaking, lives in Hatfield MA 01038 with her parents on the family ranch
(horses and cattle). Insured through her parent's private plan. Works part time as a
veterinary technician. Never smoker, no alcohol. No known drug allergies throughout.

Family history (S, profile): father with hereditary angioedema, confirmed C1 inhibitor
deficiency; mother with systemic lupus erythematosus and hypothyroidism; a maternal
aunt with rheumatoid arthritis; paternal grandmother with "swelling episodes", suspected
undiagnosed HAE. No other relatives are named.

Joined the practice on **2019-08-13** (A1), age 20, transferring from her pediatric
practice. She brought no active problems and no medications. Past history on joining:
one spontaneous pregnancy loss at 10 weeks in 2018, at age 19, cared for by an outside
obstetric office, no procedure (R2). Height 163 cm, weight 70 kg.

Surgical and procedure history (S, profile) is not known at joining: the renal biopsy
and the tracheal intubation both happen in the 2024 admission (R5, R6).

## Timeline (dates fixed; S unless marked)

Every source encounter is outside care and reaches the practice as a complete
letter, record or discharge summary (records complete). Practice contacts around
them are P unless marked.

| Date | Where | What happens |
|---|---|---|
| 2019-08-13 | practice | establish care (A1): history above; HIV and hepatitis C screens negative; CBC and CMP normal |
| 2021-06-10 | practice | office visit (P): about 6 weeks of hand joint pain, fatigue, a facial rash that flares after sun, and mouth sores; exam malar rash and swollen tender small joints of both hands; CBC shows WBC 3.4 and hemoglobin 11.2; CMP and urinalysis normal. Referred to dermatology (rash, quick slot) and to rheumatology |
| 2021-06-29 | outside | dermatology consult letter: photosensitivity and oral ulcers; malar rash and synovitis of both hands; ANA positive (1:640, homogeneous, A8), anti-dsDNA positive (186 IU/mL), anti-Smith positive, aPTT elevated (41 s), VDRL reactive (1:2) though she reports no sexual activity (R2, R3), mild leukopenia and anemia (the practice CBC), ESR elevated (48), C3 low (64 mg/dL); chest x-ray with no acute disease. **Systemic lupus erythematosus diagnosed.** Sun protection; rheumatology referral already placed |
| 2021-07-27 | outside | rheumatology consult letter (A2): confirms SLE (rash, oral ulcers, arthritis, leukopenia, serologies); treponemal antibody negative, so the VDRL is a biologic false positive (A2). **Hydroxychloroquine 200 mg twice daily, prednisone started at 20 mg daily and tapered by 5 mg every 2 weeks to 5 mg daily, cholecalciferol 2000 IU daily** started. Baseline retinal exam requested |
| 2021-08-10 | practice | annual physical (P): reports swelling episodes since 2020 (hands, feet, once the face), each lasting 2 to 3 days, painless, no hives or itch, resolving on their own (S, R16); father has HAE. **Referred to allergy and immunology** (A3) |
| 2021-09-14 | outside | ophthalmology baseline hydroxychloroquine exam letter: normal (P) |
| 2021-09-23 | outside | allergy and immunology consult letter: came for the swelling evaluation while in a four-day flare of generalized joint pain and facial rash; rash fairly symmetric across both cheeks and the bridge of the nose; platelet count 100 (x10^3/uL). **Thrombocytopenia** noted. C4 and C1 inhibitor studies sent |
| 2021-10-05 | outside | allergy and immunology results letter (A3): C4 6 mg/dL (ref 10-40), C1 inhibitor antigen 7 mg/dL (ref 21-39), C1 inhibitor function 24% (normal above 67%). **Hereditary angioedema type I diagnosed. Danazol 200 mg three times daily started** for long-term prophylaxis (S drug) |
| 2021-11-16 | outside | rheumatology follow-up (A2): the September flare settled within about 10 days with no change; prednisone 5 mg; platelets back to normal (186), **thrombocytopenia resolved** |
| 2022-03-24 | practice | office visit (P): two weeks of fatigue, breathlessness on exertion, dizziness and dark urine; pale, heart rate 104; hemoglobin 8.1, total bilirubin 2.4. Same-week hematology referral |
| 2022-03-29 | outside | hematology consult letter: two weeks of progressive fatigue, exertional dyspnea, dizziness and dark urine; recent joint pain and more sun sensitivity; intermittent low-grade fevers; history of photosensitive rash; no recent infection, no NSAIDs. Pale, mildly tachycardic, no jaundice, no lymphadenopathy. Hemoglobin 7.9, reticulocytes 8%, LDH elevated (486 U/L, A4), haptoglobin low (<10 mg/dL, A4), indirect hyperbilirubinemia (total 2.6, direct 0.4), spherocytes on smear, direct Coombs positive (C3d positive, IgG negative, A4), cold agglutinin titer 1:512 (A4). **Cold autoimmune hemolytic anemia, secondary to SLE, diagnosed. Folic acid 1 mg daily started** (S drug); keep warm, gloves and layers for barn work in cold weather; no transfusion |
| 2022-04-26 | outside | hematology follow-up (A4): hemoglobin 9.8, reticulocytes 4.1%, LDH 298 |
| 2022-07-12 | outside | hematology follow-up (A4): hemoglobin 11.6, bilirubin normal; compensated; yearly follow-up |
| 2023-02-24 | outside | dermatology follow-up letter: lupus flare with photosensitivity, oral ulcers, joint pain and fatigue; malar rash and synovitis of both hands; anti-dsDNA positive (142 IU/mL), C3 low (66 mg/dL), ESR elevated (42), mild leukopenia (WBC 3.5) and anemia (hemoglobin 10.9) (R4). Sun protection; rheumatology informed |
| 2023-03-14 | outside | rheumatology follow-up (A2): flare settling with sun protection; no drug change |
| 2024-02-24 | practice | early-morning text and call to the physician (P): two days of reddish urine, puffy face and sore joints; sent to the emergency department |
| 2024-02-24 to 2024-03-05 | outside | admission under nephrology at a regional hospital (discharge summary, biopsy pathology and ultrasound report received): two days of fatigue, joint pain, reddish urine and facial puffiness; morning stiffness of fingers and knees about an hour; photosensitivity and a past episode of pleuritic chest pain by her report (R15). BP 148/94; mild periorbital edema; tender small joints of both hands. Creatinine 2.0, anti-dsDNA elevated (410 IU/mL), low complements (C3 48, C4 4 mg/dL), urinalysis with dysmorphic red cells and red cell casts (dipstick protein 2+ and blood 3+, A5). Added (A5): urine protein/creatinine 2.4 g/g; renal ultrasound normal-sized kidneys without hydronephrosis, with incidental gallstones (R12); urine pregnancy test negative. C1 inhibitor 1000 units IV before the biopsy (A5). **Renal biopsy 2024-02-27: lupus nephritis ISN/RPS class IV-G (A), activity index 9/24, chronicity index 1/12 (A5). Lupus nephritis class IV diagnosed; cholelithiasis noted.** Methylprednisolone 500 mg IV daily 2024-02-27 to 02-29 (A5). On 2024-02-28 laryngeal angioedema (voice change, stridor): C1 inhibitor 20 units/kg IV, intubated in the ICU, extubated 2024-03-01 (R6). **Mycophenolate mofetil 1000 mg twice daily started 2024-02-29** (S drug, A5). Discharge creatinine 1.4 (A5). **Prednisone 5 mg stopped; prednisone 40 mg daily started with a taper** (by 5 mg weekly to 20 mg, then by 2.5 mg every 2 weeks to 5 mg) |
| 2024-03-12 | practice | post-discharge visit (P): BP 132/84, weight 79 kg, creatinine 1.3; cholelithiasis and lupus nephritis added to the list |
| 2024-03-26 | outside | nephrology follow-up (A5): creatinine 1.2, UPCR 1.2 g/g, C3 71 |
| 2024-04-09 | outside | allergy and immunology follow-up after the airway attack: danazol continued unchanged |
| 2024-06-11 | outside | nephrology follow-up (A5): creatinine 0.9, UPCR 0.42 g/g, C3 88, anti-dsDNA 92; partial response; prednisone 7.5 mg in the taper |
| 2024-07-30 | outside | rheumatology follow-up (A2): **taper finished late June; prednisone 5 mg daily resumed** (S dose) |
| 2024-09-17 | outside | nephrology follow-up (A5): creatinine 0.82, UPCR 0.16 g/g, urine without blood, C3 98, anti-dsDNA 44: **complete renal response; lupus nephritis class IV in remission** (S) |
| 2025-01-21 | outside | nephrology follow-up (A5): remission, UPCR 0.14 g/g |
| 2025-03-09 | practice | evening call to the physician (P): lip swelling since the day before; told to go to the emergency department given her airway history |
| 2025-03-10 | outside | emergency department record: two days of progressive swelling of the face and limbs, no hives or itch; sporadic similar episodes over the past five years, most resolving on their own; the airway attack a year earlier needing intubation (R6). Vital signs stable. Marked non-pitting swelling of lips and eyelids without redness or warmth; mild diffuse abdominal tenderness without rebound or guarding. CBC normal, basic metabolic panel normal, C4 low (5 mg/dL), IgE normal (38 IU/mL), C1 inhibitor sent. Treated with C1 inhibitor 20 units/kg IV and observed overnight, discharged 2025-03-11 (A6, R7) |
| 2025-03-14 | practice | post-ED visit (P): swelling gone |
| 2025-04-08 | outside | allergy and immunology follow-up: the emergency C1 inhibitor antigen came back 6 mg/dL (A6); danazol continued unchanged |
| 2026-02-03 | practice | portal message (R9): leaving in mid-February for several weeks of veterinary technician work at a cattle ranch near Flagstaff, asks for 90-day refills |
| 2026-03-05 to 2026-03-11 | outside | admission under infectious disease at a regional hospital near Flagstaff (discharge summary received 2026-03-13): four days of increasing fatigue, chills, dry cough turning blood-streaked, and left groin pain; a painful ankle sore that appeared soon after she pulled a flea off her sock; works with livestock and stray cats there (R9). On arrival febrile and short of breath: T 39.2 C, pulse 110, respirations 30, BP 92/58, SpO2 91% on room air (A7). 1.5 cm necrotic ulcer on the lateral left ankle, scattered red macules on both lower legs, a tender fluctuant 4 cm left inguinal mass, crackles in both lungs. Leukocytosis (WBC 21.4, neutrophils 88%, A7) and thrombocytopenia (platelets 92, A7). Chest x-ray bilateral patchy airspace opacities (A7). Groin aspirate: pus with many neutrophils and a few gram-negative rods; culture Yersinia pestis, confirmed by the state public health laboratory (R10). **Bubonic plague, leukocytosis and thrombocytopenia diagnosed.** Treated (A7): IV ciprofloxacin and IV doxycycline, droplet precautions for the first 48 hours, warmed IV fluids, oxygen 2 L by nasal cannula for the first days, hydrocortisone stress dosing while hypotensive; **mycophenolate held from admission**; public health notified. Discharged on **ciprofloxacin 750 mg twice daily to complete 14 days (through 2026-03-18)** |
| 2026-03-17 | practice | post-discharge visit (P): afebrile, groin node smaller and non-tender, ankle ulcer healing; WBC and platelets back to her normal; leukocytosis and thrombocytopenia resolved |
| 2026-03-19 | outside | infectious disease telehealth follow-up letter (A7): course finished, well; **mycophenolate restarted at 1000 mg twice daily**; bubonic plague resolved |
| 2026-04-28 | practice | follow-up chest x-ray (A7): clear |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The profile lists every problem, drug and procedure from the first encounter.
  Resolution: each condition starts at the encounter that diagnoses it (SLE 2021-06,
  HAE 2021-10, cold AIHA 2022-03, lupus nephritis 2024-02, plague 2026-03), and each
  home drug starts where the timeline bolds it. Outside letters copy the medication
  list current on their date; by 2024-09 her list matches the profile's six drugs.
- **R2.** The profile's spontaneous abortion at 10 weeks has no date, and the
  dermatology note says she "denies any sexual history". Resolution: the loss was in
  2018 at an outside obstetric office, before she joined; since then she has had no
  sexual activity, which is what the dermatology letter and later notes record.
- **R3.** The VDRL is reactive. Resolution: biologic false positive, shown by a
  negative treponemal antibody at rheumatology (A2). The elevated aPTT is carried as
  the source gives it.
- **R4.** The 2023 dermatology note copies the 2021 note word for word, including
  first-diagnosis serologies and a chest x-ray. Resolution: the 2023 visit is a flare
  with the findings in the timeline; ANA, anti-Smith, aPTT, VDRL and chest x-ray are
  not repeated.
- **R5.** The profile carries "class IV nephritis in remission" and a renal biopsy
  "on a prior admission" from 2021, and the 2024 note says "previously in remission".
  Resolution: the 2024 admission is her first nephritis; its biopsy is the one the
  profile lists; remission comes in 2024-09. "Previously in remission" is not carried.
- **R6.** The 2025 ED note dates the intubation for laryngeal swelling "one year ago".
  Resolution: the laryngeal attack happens on 2024-02-28 in the nephrology admission,
  the day after the biopsy, and is the profile's intubation.
- **R7.** The 2026 note says she was "hospitalized 2022, 2023". Resolution: re-dated
  to the 2024 admission and the 2025 emergency visit with its overnight observation.
- **R8.** The ED note's "her mother has a history of similar symptoms" is not carried:
  the father has HAE and the paternal grandmother had swelling episodes.
- **R9.** The 2026 note puts her parents' ranch in northern Arizona, while she lives in
  Hatfield. Resolution: home is the family ranch in Hatfield; she was on a several-week
  work stay at a cattle ranch near Flagstaff when the flea bite and admission happened.
- **R10.** Not carried: growth "at 4 degrees" (a quiz clue). The culture is
  identified as Yersinia pestis.
- **R11.** Duplicate names collapse: one hereditary angioedema (type I), one lupus
  nephritis (class IV, flare then remission), one cold autoimmune hemolytic anemia.
- **R12.** The comorbidity "personal history of cholelithiasis" has no event.
  Resolution: gallstones seen incidentally on the 2024 renal ultrasound; never
  symptomatic, no surgery, kept on the list.
- **R13.** Leukocytosis and thrombocytopenia (comorbidities) are the plague-admission
  counts; thrombocytopenia also appears in 2021 (platelets 100) and resolves both times.
- **R14.** The hematology note says she "returns" after "January" and "April" visits.
  Resolution: it is her first hematology visit, after the June dermatology and
  September allergy visits.
- **R15.** The 2024 note's past pleuritic chest pain is her report in that admission;
  no earlier record of it exists and no one charted it.
- **R16.** "Episodes over the past 5 years" (2025) dates her swelling from 2020, which
  is what she tells the practice in 2021.
- **R17.** Every outside list shows prednisone 5 mg. Resolution: 5 mg is her
  maintenance dose outside the two tapers (2021, 2024); a letter written during the
  2024 taper may still copy 5 mg.
- Interobserver: spherocytes on the 2022 smear stay as the hematologist reported them.
- **Left as the source has it:** danazol 600 mg a day stays her only HAE prophylaxis
  and no one prescribes an on-demand HAE drug, even after the airway attack; no
  antiphospholipid testing despite the long aPTT, the false-positive VDRL and the
  pregnancy loss; no ACE inhibitor or ARB for the nephritis or the 148/94; no
  transfusion at hemoglobin 7.9; hydroxychloroquine 400 mg daily runs a little above
  5 mg/kg at her weight. No note comments on any of these.

## Additions (A): physician to confirm

- **A1.** 2019 new-patient visit (seven years with the practice).
- **A2.** Rheumatology runs her SLE from 2021-07 (the source asserts SLE and its drugs
  but has no rheumatologist): the consult and treponemal test above, then visits about
  every 6 months (2021-11, 2022-05, 2022-11, 2023-03, 2023-11, 2024-04, 2024-07,
  2024-11, 2025-05, 2025-11, 2026-05), with their serologies drawn at the practice the
  week before. Serologies, exact: 2021-11-09 C3 82, anti-dsDNA 74; 2022-05-10 C3 86,
  dsDNA 58; 2022-11-08 C3 94, dsDNA 41; 2023-11-07 C3 92, dsDNA 36; 2024-04-16 C3 79,
  dsDNA 128; 2024-11-05 C3 97, dsDNA 40; 2025-05-06 C3 101, dsDNA 33; 2025-11-04 C3 96,
  dsDNA 39; 2026-05-05 C3 99, dsDNA 35 (C3 mg/dL, ref 90-180; dsDNA IU/mL, ref <30).
- **A3.** The 2021 referral to allergy and immunology, and the C4 and C1 inhibitor
  values that confirm HAE type I (the source's plan stops mid-sentence).
- **A4.** The direct Coombs pattern, cold agglutinin titer, LDH and haptoglobin values
  that support cold AIHA, and the hemoglobin recovery (9.1 on 2022-04-12, 9.8 on 2022-04-26, 11.6 in July 2022).
  Hematology then sees her yearly (2023-07-18, 2024-07-23, 2025-07-29): hemoglobin
  about 11.5, no further hemolysis.
- **A5.** The work-up and induction of the 2024 nephritis as listed in the timeline,
  and its course to remission: creatinine 2.0, 1.4, 1.3, 1.2, 0.9, 0.82 and about 0.8 (urinalysis trace protein and trace blood in 2024-06, negative from 2024-09)
  after; UPCR 2.4, 1.2, 0.42, 0.16, 0.14, then 0.12 (2025-07-22), 0.15 (2026-01-20),
  0.13 (2026-07-21). Nephrology sees her every 6 months after remission.
- **A6.** ED treatment of the 2025 attack with C1 inhibitor and overnight observation;
  the pending C1 inhibitor resulted at 6 mg/dL.
- **A7.** Plague course: SpO2 91%, chest x-ray, WBC 21.4, platelets 92, culture
  identification, the antibiotics, mycophenolate held and restarted, the follow-up
  telehealth and a clear chest x-ray on 2026-04-28. On 2026-03-17 WBC 5.6, platelets 238,
  hemoglobin 11.5.
- **A8.** The ANA pattern and titer, and the dsDNA, C3, aPTT and VDRL numbers given at
  2021-06-29, 2023-02-24 and 2024-02-24 (the source says only positive, low, elevated).

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not
  mention. An abnormal finding appears only as the source or this file gives it. A
  report may carry the descriptors its kind always carries, but nothing that adds a
  territory, a severity grade or a symptom, or changes the diagnosis or plan.
- Records from her pediatric practice: childhood vaccines complete, including
  hepatitis B, MMR, varicella, HPV (3 doses), MenACWY and Tdap in 2010.
- Yearly physicals in August; office visits for the timeline above; phone, portal and
  pharmacy traffic for refills (hydroxychloroquine, danazol, mycophenolate, folic acid,
  vitamin D), a danazol prior authorization, scheduling, records requests, result notes.
- Labs drawn at the practice for rheumatology and nephrology: CBC with differential,
  CMP and urinalysis with each rheumatology or nephrology visit; with danazol, a hepatic
  panel and lipid panel yearly (on danazol total cholesterol about 184, HDL about 44, LDL about 118, triglycerides about 112, 25-hydroxy vitamin D about 34 on D3, normal
  liver tests; no lipid drug and no diagnosis).
- Levels: weight 70 kg in 2019, 72 in 2021, 74 in 2022 and 2023, 79 in 2024-03, 78 in 2024-08, 77 by
  2025, 73 after the 2026 illness. Blood pressure normal except 148/94 (S) in
  the 2024 admission and 132/84 at the post-discharge visit. WBC runs low (3.4 to 3.6)
  only in the 2021 and 2023 flares; hemoglobin 11.2 (2021-06), 11.4 (2021-09), about
  12 late 2021, the AIHA values in 2022, 10.9 (2023-02), about 11.5 thereafter (10.6 in
  the 2024 admission, 11.0 in the 2026 admission). ESR 48 (2021-06), 22 (2021-11), 42
  (2023-02), 56 (2024-02), about 20 otherwise.
- Prevention: influenza vaccine each fall (inactivated only; no live vaccines once on
  immunosuppression); COVID vaccines (Pfizer original April and May 2021 at a pharmacy,
  booster December 2021, bivalent fall 2022, updated formula each fall from 2023);
  pneumococcal PCV20 in 2024-08; recombinant zoster 2024-08 and 2024-10 (immunosuppressed
  adult); cervical cytology 2020-08 and 2023-08, normal; HIV and hepatitis C once (2019);
  PHQ-2 or PHQ-9 yearly with low scores. Counseling while on mycophenolate and danazol:
  no pregnancy; she reports no sexual activity.
- Hydroxychloroquine retinal screening: baseline 2021-09 and yearly from 2026 (2026-07-14),
  normal.
- One missed or cancelled visit.
- Booked visit after 2026-08-27: annual physical on 2026-09-17.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.
