# Build requirements: unit-1836 (Karen Duffy, MRN00001836)

Source: synthetic_hospital patient 1836, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its "shown in the picture below" text.

## Who she is (S)

Karen Duffy, born 1988-12-20, female, White, not Hispanic or Latino, English, commercial
PPO, lives in Easthampton (01027). Software developer working remotely from home; spends
most of her time indoors. Strict vegetarian diet. Never smoker; alcohol socially, 1 to 2
drinks a month. Allergies known on joining: penicillin (hives), sulfonamide antibiotics
(rash). Family history: mother rheumatoid arthritis; father asthma and seasonal allergies;
sister Crohn's disease.

Surgical history: laparoscopic ileocecal resection (limited right hemicolectomy) for
Crohn's disease in 2020, before she joined, at an outside surgical service; the appendix
came out with the specimen (R1).

Obstetric history: one pregnancy before she joined, with urinary tract infections in the
first trimester treated with nitrofurantoin (R8). No further obstetric detail is recorded.

Joined the practice on 2021-08-10 (A1), new-patient preventive visit. She brought:
Crohn's disease (ileocolonic, currently mild, followed by an outside GI group, on no
maintenance drug), asthma (mild intermittent), history of recurrent urinary tract
infections, status post ileocecal resection. Medications on joining: albuterol HFA 90 mcg,
2 puffs every 4 to 6 hours as needed for wheeze or shortness of breath; prenatal
multivitamin 1 tablet daily; ferrous sulfate 325 mg (65 mg elemental iron) daily with food
(R3). Height 165 cm, weight about 60 kg (BMI 22).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2021-08-10 | practice | new-patient preventive visit (A1): history as above; exam normal; prior-practice records bring a normal Pap from 2019 (P) |
| 2022-08-23 | practice | annual physical (A2): about six months of slowly worsening fatigue, muscle weakness and diffuse bone aches. Labs drawn that day: calcium 8.1 mg/dL, phosphorus 3.0 mg/dL, alkaline phosphatase 120 U/L, 25-hydroxyvitamin D 12 ng/mL, intact PTH 88 pg/mL; CBC hemoglobin 11.6 g/dL, MCV 101 fL (A2), which the PCP notes as mild and plans to recheck after the vitamin D is treated; TSH normal. **Vitamin D deficiency diagnosed** (R2). Endocrinology referral placed |
| 2022-09-20 | outside | endocrinology consult letter: same six-month fatigue, weakness and bone pain; works from home, mostly indoors; prior ileal resection; BP 130/85, HR 78, BMI 22; appears pale; tenderness on palpation of the lower legs; reviews the practice labs above. **Secondary hyperparathyroidism due to vitamin D deficiency diagnosed.** **Ergocalciferol 50,000 IU weekly for 8 weeks** (A3), **then cholecalciferol 2000 IU daily**; recheck labs with PCP in about 3 months |
| 2022-12-06 | practice | follow-up (A4): bone aches better, fatigue no better; new intermittent palpitations and a sore tongue for a few weeks. 25-hydroxyvitamin D 32 ng/mL, PTH 54 pg/mL, calcium 9.0 mg/dL (A4). CBC hemoglobin 9.8 g/dL, MCV 105 fL (S, drawn here). Hematology referral. Loading course done, now on cholecalciferol 2000 IU daily |
| 2023-01-16 | outside | hematology consult letter: about six months of fatigue, intermittent palpitations, recent painful glossitis; strict vegetarian; prior ileal resection; vitals normal; skin pale; reviews hemoglobin 9.8, MCV 105 (macrocytic anemia); homocysteine elevated, methylmalonic acid elevated (text results, drawn at hematology). **Vitamin B12 deficiency (with macrocytic anemia) diagnosed. Cyanocobalamin 1000 mcg by mouth daily started** |
| 2023-04-11 | practice | lab only: CBC hemoglobin 12.3 g/dL, MCV 93 fL (A5) |
| 2023-06-27 | practice | office visit (A6): six months of painful intercourse and on-and-off dysuria, with a feeling of fullness in the front wall of the vagina; urinalysis negative; gynecology referral |
| 2023-07-17 | outside | gynecology consult letter: the symptoms above; tender anterior vaginal wall mass, expressing it yields a small amount of purulent discharge; urinalysis unremarkable. **Urethral diverticulum diagnosed.** Plan in the letter: follow up with gynecology as needed (no imaging, no excision; see Left as the source has it) |
| 2024-11-11 | outside | ED record: three days of fever, chills and left flank pain, a week of dysuria and frequency; T 38.7 C, BP 110/68, HR 102, RR 18; mild left CVA tenderness on percussion; urinalysis with pyuria (leukocyte esterase positive, many white cells) and bacteriuria; urine culture E. coli (susceptible to ciprofloxacin, A7). **Acute pyelonephritis, left, diagnosed.** Treated (A7): acetaminophen, IV fluids and ceftriaxone 1 g IV once in the ED without reaction, discharged on **ciprofloxacin 500 mg twice daily for 7 days** |
| 2024-11-19 | practice | post-ED visit (A8): afebrile, finished antibiotics; urinalysis negative; pyelonephritis resolved |
| 2025-07-26 | outside | ED record (Saturday): two days of fever, dysuria and left flank pain, a week of more frequency and urgency; recalls UTIs in the first trimester of her pregnancy treated with nitrofurantoin (R8); T 38.5 C, BP 110/70, pulse 98; left CVA tenderness; WBC 14.0 x10^3/uL; urinalysis positive for leukocyte esterase and nitrite, with many white cells; urine culture E. coli (susceptible to ciprofloxacin, A7); renal ultrasound without hydronephrosis or stones. **Acute pyelonephritis, left, diagnosed again.** Same treatment as 2024 (A7): acetaminophen, IV fluids, ceftriaxone 1 g IV once without reaction, **ciprofloxacin 500 mg twice daily for 7 days** |
| 2025-07-31 | practice | post-ED visit (A8): improving, afebrile; urinalysis negative at the end of the course (lab 2025-08-07); pyelonephritis resolved |
| 2026-03-17 to 2026-03-20 | outside | admission (ED then ICU), discharge summary received: two days of symptoms, right-sided pyelonephritis; T 38.9 C at triage; urinalysis with pyuria, positive leukocyte esterase and nitrite; urine culture E. coli (susceptible to ciprofloxacin, A9). **Acute pyelonephritis, right, diagnosed.** IV ceftriaxone started after she reported no prior reaction to it; ten minutes in: throat tightness, dizziness, sense of impending doom, hoarse voice, forceful cough, then generalized flushing and swelling of the face and lips. Vitals at the reaction: T 37.2 C, pulse 132, RR 28, BP 68/42, SpO2 91% on room air (R6). Exam: periorbital and lip edema, diffuse urticarial plaques over chest and arms, bilateral expiratory wheezes, inspiratory stridor, anxious, using accessory muscles; JVP not elevated; tachycardic, no murmur; no peripheral edema. **Anaphylaxis due to ceftriaxone**; **ceftriaxone allergy (anaphylaxis) recorded** (R4). Infusion stopped; treated (A9) with IM epinephrine 0.5 mg (two doses), 2 L IV crystalloid, diphenhydramine 50 mg IV, methylprednisolone 125 mg IV, nebulized albuterol and oxygen; admitted to the ICU, no intubation; stridor and hypotension resolved within hours. Pyelonephritis treated with IV ciprofloxacin, switched to **oral ciprofloxacin 500 mg twice daily to complete 7 days total**; discharged home 2026-03-20 |
| 2026-03-25 | practice | post-discharge visit (A10): recovered; allergy list updated with ceftriaxone anaphylaxis; pyelonephritis resolved after the course; three pyelonephritis episodes with a known urethral diverticulum: **urology referral placed** |
| 2026-07-02 | practice | follow-up visit, last seen (P): well; vitamin D, CBC, CMP drawn and normal; asthma quiet; awaiting urology appointment |
| 2026-08-11 | outside | urology consult letter (received 2026-08-18, after last seen; filed, lists unchanged) (A11): history as above; tender anterior vaginal wall mass consistent with the known urethral diverticulum; MRI of the pelvis ordered; excision to be discussed after the MRI. Referral done |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source gives three surgeries: an "ileal resection two years ago" (2022
  note, so 2020), a "laparoscopic appendectomy 5 years ago" (2025 note, so 2020) and
  "status post right hemicolectomy". Resolution: one operation in 2020, a laparoscopic
  ileocecal resection (limited right hemicolectomy) for Crohn's disease, with the appendix
  removed in the specimen. Outside notes may call it an ileal resection.
- **R2.** The profile lists vitamin D deficiency, vitamin B12 deficiency and urethral
  diverticulum from the start. Resolution: vitamin D deficiency is recorded on
  2022-08-23 (its first level), secondary hyperparathyroidism on 2022-09-20, B12
  deficiency on 2023-01-16, urethral diverticulum on 2023-07-17, acute pyelonephritis on
  2024-11-11. No note before those dates mentions them.
- **R3.** The source puts cholecalciferol and cyanocobalamin on every list, while the
  2023 hematology note says she takes no medications. Resolution: cholecalciferol starts
  after the endocrine loading course (2022), cyanocobalamin at hematology (2023-01-16);
  "takes no medications" is not carried. Ferrous sulfate and the prenatal multivitamin are
  long-standing supplements she brought; the source gives no iron deficiency and the chart
  adds none.
- **R4.** Ceftriaxone anaphylaxis is listed from the first encounter, yet in 2026 she has
  "no prior history of reaction to this drug". Resolution: the allergy is recorded at the
  2026-03-17 reaction. She received single ED doses of ceftriaxone in 2024 and 2025
  without reaction. The 2026 note's allergy line ("trimethoprim-sulfamethoxazole and
  ciprofloxacin") is not carried as written: TMP-SMX is her known sulfonamide allergy,
  and ciprofloxacin is not an allergy (she takes it in 2024, 2025 and 2026 without trouble).
- **R5.** "History of septic shock" is not carried: the 2026 hypotension was
  anaphylactic. "Anesthetic medication reaction" is carried as anaphylaxis due to
  ceftriaxone (a drug reaction, not an anesthetic).
- **R6.** The 2026 note gives a temperature of 38.9 C on presentation and 37.2 C with the
  other vitals. Resolution: 38.9 C at triage, 37.2 C at the time of the reaction, after
  acetaminophen in the ED.
- **R7.** The 2024 ED note refers to "imaging findings from a renal biopsy" shown in a
  picture. Dropped: no biopsy and no imaging at that visit.
- **R8.** The 2025 ED note speaks of "pregnancy-related UTIs" in her first trimester,
  treated with nitrofurantoin. Resolution: one pregnancy before she joined, recorded as
  her own history (first-trimester UTIs, nitrofurantoin); nothing more about it is
  recorded, and no child appears.
- **R9.** Years in source text follow the re-dated timeline: the "March 2022" and "2022"
  pyelonephritis episodes become November 2024 (and July 2025). Ages follow the birth date
  (33 in the gynecology note becomes 34; 35 becomes 36; 36 becomes 37).
- **R10.** The endocrinology labs are drawn at the practice on 2022-08-23 and reviewed in
  the endocrinology letter; the hemoglobin 9.8 and MCV 105 are drawn at the practice on
  2022-12-06 and reviewed by hematology. Each number appears once as a result.
- **Left as the source has it:** no maintenance drug for Crohn's disease after resection;
  oral rather than injected B12 despite the ileal resection; ferrous sulfate with no iron
  studies; no imaging, prophylaxis or excision for the urethral diverticulum between the
  2023 diagnosis and the 2026 referral. Nobody comments on these.

## Additions (A): physician to confirm

- **A1.** The 2021-08-10 new-patient visit (seed: about 5 years with the practice).
- **A2.** The 2022-08-23 physical that finds the low vitamin D and sends her to
  endocrinology; its CBC hemoglobin 11.6, MCV 101, noted but not worked up then.
- **A3.** The ergocalciferol 50,000 IU weekly loading course for 8 weeks before
  maintenance cholecalciferol (standard for a level of 12).
- **A4.** The 2022-12-06 recheck: 25-hydroxyvitamin D 32, PTH 54, calcium 9.0; the visit
  that finds the anemia and refers to hematology.
- **A5.** CBC recovery on oral B12: hemoglobin 12.3, MCV 93 on 2023-04-11; later CBCs
  normal.
- **A6.** The 2023-06-27 visit that refers to gynecology.
- **A7.** Treatment of the 2024 and 2025 pyelonephritis: acetaminophen, IV fluids and one ED dose of ceftriaxone 1 g IV,
  then ciprofloxacin 500 mg twice daily for 7 days; cultures E. coli susceptible to
  ciprofloxacin.
- **A8.** Post-ED practice visits (2024-11-19, 2025-07-31) with negative urinalyses.
- **A9.** Anaphylaxis treatment and the pyelonephritis course in 2026 as in the timeline;
  culture E. coli susceptible to ciprofloxacin; discharge 2026-03-20.
- **A10.** The 2026-03-25 post-discharge visit and urology referral (third episode with a
  known diverticulum).
- **A11.** The 2026-08-11 urology consult: findings as above; pelvic MRI ordered, not yet
  done today.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may carry
  the descriptors its kind always carries, but nothing that adds a territory, a severity
  grade or a symptom, or changes the diagnosis or plan.
- Records from a prior practice may mention routine screening done there, with normal
  results (a 2019 Pap).
- Yearly physicals (2022-08-23, 2023-09-12, 2024-09-17, 2025-09-23) with CBC, CMP and
  25-hydroxyvitamin D; vitamin B12 levels after 2023 normal; lipid panel once, normal.
- Yearly outside GI letters (each spring): Crohn's disease mild and quiet, no drug change.
- Asthma: mild intermittent, albuterol refills, no exacerbations.
- Prevention: Pap with HPV at the 2024 physical, normal; influenza vaccine each fall; COVID
  vaccines; Tdap at joining; hepatitis C and HIV screening once (negative); PHQ-2 yearly.
- Refills, pharmacy traffic, portal messages about results, refills and scheduling, a
  missed visit, records requests and received outside records.
- Booked visit after 2026-08-27: annual physical.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00001836 on 2026-07-02. Practice contacts after that day do not
happen; outside care after it still does, and its records are filed as received without
changing the practice's lists. A visit is booked after today.
