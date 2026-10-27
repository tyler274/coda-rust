# Build requirements: unit-2648 (Declan Merrill, MRN00002648)

Source: synthetic_hospital patient 2648, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its unfinished-plan wording.

## Who he is (S)

Declan Merrill, born 1989-05-09, male, White, not Hispanic or Latino, English, lives in
Leeds MA 01053, commercial employer-sponsored plan. High school physical education
teacher and varsity athletics coach. Lives with a partner (marital status partnered).
Never smoker (0 pack-years). Alcohol: occasional social use, 1 to 2 drinks a month (R3).
No illicit drugs. Allergies: NKDA. Surgical history: none.

Family history (R2): father had early-onset emphysema diagnosed at age 42 and died at 48
of cirrhosis, attributed at the time to long-standing alcohol use; paternal uncle with
COPD; paternal grandmother with cryptogenic cirrhosis. No other relatives are named.

Joined the practice 2021-06-22 (A1, new-patient visit) at age 32: healthy, no
problems, no medications, symptom-free. Height 181 cm; weight 73.5 kg at joining,
70 kg by December 2023, 67 kg from autumn 2025 on ("thin", R5).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2021-06-22 | practice | new-patient visit and physical (A1): history as above, normal exam, no medications; CBC, CMP, lipids, HIV and hepatitis C screening, all normal |
| 2022-08-16 | practice | annual physical (P): no complaints, normal exam |
| 2022-10 | — | breathlessness and fatigue begin (onset only, no contact; follows from the 14 months in Dec 2023 and the 3 years in Oct 2025) |
| 2023-07-25 | practice | annual physical **cancelled** by the patient (P) |
| 2023-12-05 | practice | office visit: 14 months of worsening shortness of breath and fatigue; winded after one flight of stairs or a few minutes of brisk walking; has had to stop demonstrating drills at work; mild dry cough, occasional wheeze. Partner took up smoking again more than a year ago (secondhand exposure at home). T 36.8 C, pulse 82, RR 18, BP 122/78, SpO2 91% on room air. Scattered end-expiratory wheezes and bibasilar rhonchi; rest of exam normal. **Chest X-ray, 2 views, same afternoon** (R1): hyperlucency with bilateral basal bullous changes. CBC and CMP normal; **serum alpha-1 antitrypsin level 24 mg/dL** (low; A2), resulted 2023-12-07. Full PFTs ordered |
| 2023-12-12 | outside | hospital pulmonary function lab (R1): obstructive ventilatory pattern with decreased DLCO. No numeric values are carried |
| 2023-12-13 | practice | telephone: PCP reviews the X-ray, low AAT level and PFTs; **alpha-1-antitrypsin deficiency diagnosed** (R4); pulmonology referral placed; asked that the partner not smoke in the house or car |
| 2024-02-06 | outside | first pulmonology consult (A3). **No letter ever reaches the practice** (records gap); the practice knows only what the patient reports on 2024-03-05: seen, more breathing tests, no medicines started, see them again in about a year and a half. Referral closed as done |
| 2024-09-10 | practice | annual physical (P): breathing about the same; SpO2 93%; lungs with mildly diminished breath sounds, no wheeze. PCV20 given (P, chronic lung disease), influenza and COVID vaccines |
| 2025-08-19 | practice | annual physical (P): breathing slowly worse, now avoids jogging; partner smokes only outdoors and away from him (R3); SpO2 92%; pulmonology follow-up already booked for October |
| 2025-10-20 | outside | pulmonology follow-up (S): breathlessness worse over about 3 years; cannot keep up with colleagues on stairs; avoids jogging because of fatigue and chest tightness; takes no daily medicines; denies secondhand smoke or occupational exposure; occasional alcohol. SpO2 93% on room air; thin, mild tachypnea; diminished breath sounds both sides with prolonged expiration, hyperresonance, mildly increased AP diameter; no wheeze, rales or clubbing. Chest X-ray, 2 views: bilateral hyperinflation and basilar bullae. PFTs: obstructive pattern, increased TLC, markedly increased residual volume. Labs: **ALT 62, AST 47 U/L** (mildly elevated; A4); **repeat AAT level 22 mg/dL**, confirming deficiency (A4). SERPINA1 genotyping ordered (R6) |
| 2025-11-06 | outside | pulmonology addendum (R6): genotype **PiZZ**, the most severe form; told by phone; advised to consider augmentation therapy and to avoid further liver and lung insults (alcohol, hepatotoxic drugs, smoke) |
| 2025-11-24 | practice | the 2025-10-20 letter with the 2025-11-06 addendum arrives by fax, about five weeks late (records gap) |
| 2025-12-09 | practice | office visit (A5): reviews the pulmonology letter; **elevated liver enzymes recorded**; hepatic panel ALT 59, AST 43 U/L, alkaline phosphatase and bilirubin normal; abdominal ultrasound ordered; **hepatitis A (dose 1) and Heplisav-B (dose 1)** given (A6); advised to avoid alcohol and to check before any new drug or supplement; SpO2 93% |
| 2025-12-18 | outside | abdominal ultrasound (A5): normal liver size and echotexture, no focal lesion, normal spleen, no ascites |
| 2026-01-13 | practice | nurse visit: Heplisav-B dose 2 (A6) |
| 2026-02-03 | outside | **weekly IV alpha-1 proteinase inhibitor (human) 60 mg/kg started** by pulmonology at an outside infusion center after insurer approval (A7). No letter reaches the practice; the patient reports it by portal message 2026-02-10 |
| 2026-04-20 | practice | office visit, **last seen**: on weekly infusions since February, tolerating them; breathing no worse; SpO2 93%. Augmentation therapy added to the medication list as a pulmonology-prescribed drug. Hepatic panel: ALT 64, AST 46 U/L, alkaline phosphatase and bilirubin normal. Hepatitis A dose 2 due from June 2026 (left open) |
| 2026-09-10 | practice | annual physical **booked** (P) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source puts a chest X-ray and PFTs inside the internal medicine visit. An
  office would not do full PFTs that day. Resolution: the X-ray is taken at an outside
  imaging site the same afternoon; full PFTs with DLCO are done on 2023-12-12 at a
  hospital pulmonary function lab. Findings exactly as the source gives them.
- **R2.** Father's death: "age 58 from liver failure" (encounter 0) vs "age 48 from
  cirrhosis attributed to alcohol, with early-onset respiratory issues" (encounter 1) vs
  "early-onset emphysema diagnosed age 42" (profile). Resolution: emphysema at 42, died at
  48 of cirrhosis attributed to alcohol. Age 58 is not carried. The chart never says the
  father's illness was AATD.
- **R3.** Alcohol: "does not use alcohol" (encounter 0) vs occasional use (profile,
  encounter 1). Resolution: occasional social use throughout. Secondhand smoke: present
  in 2023 (partner resumed smoking), denied in 2025. Resolution: after the December 2023
  advice the partner smokes only outdoors and away from him, so by 2025 he has no
  exposure. The partner's smoking appears only as his exposure.
- **R4.** The source stamps AATD as diagnosed at encounter 0 yet orders confirmatory
  testing at encounter 1. Resolution: the practice diagnoses AATD in December 2023 on
  basal bullous changes, obstruction with low DLCO in a never smoker, the family history
  and a low serum level (A2); pulmonology confirms the severe genotype in 2025 (R6).
- **R5.** "Thin male" in 2025. Resolution: a slim build at joining (BMI about 22.4) and a
  gradual weight loss of about 6 kg by autumn 2025 (BMI about 20.5) as the breathlessness
  progresses. No other cause is sought or named.
- **R6.** The source orders genetic testing and tells him the results in the same note.
  Resolution: genotyping is sent on 2025-10-20 and the PiZZ result and counseling come in
  a 2025-11-06 addendum by phone.
- Exam differences: wheezes and rhonchi in 2023, no wheeze in 2025. Different examiners
  two years apart; kept as written.
- **Left as the source has it:** no inhaled bronchodilator or other inhaler is ever
  prescribed, and no pulmonary rehabilitation is arranged; nobody comments. The first
  pulmonology letter never arrives. No relative is offered testing.

## Additions (A): physician to confirm

- **A1.** 2021-06-22 new-patient visit (seed: about 5 years with the practice).
- **A2.** Serum AAT level 24 mg/dL (reference about 100 to 190) on 2023-12-05, the test
  that supports the diagnosis the source stamps at that visit.
- **A3.** 2024-02-06 first pulmonology consult (encounter 1 is a "return" visit); its
  letter never reaches the practice.
- **A4.** Values for the source's "mildly elevated liver enzymes" (ALT 62, AST 47) and
  for its confirmed serum deficiency (AAT 22 mg/dL).
- **A5.** 2025-12-09 practice follow-up after the pulmonology letter: repeat hepatic
  panel (ALT 59, AST 43) and one abdominal ultrasound, normal, for the liver side of AATD
  with raised enzymes.
- **A6.** Hepatitis A (2 doses; second not yet given) and Heplisav-B (2 doses, complete)
  for "avoid further hepatic insults".
- **A7.** Augmentation therapy, the standard care pulmonology advised: alpha-1
  proteinase inhibitor (human) 60 mg/kg IV weekly from 2026-02-03, run by pulmonology at
  an infusion center. Repeat hepatic panel 2026-04-20: ALT 64, AST 46.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention
  (heart size, mediastinum, pleura, no effusion; heart sounds, abdomen). An abnormal
  finding appears only as the source or this file gives it. A report may carry the
  descriptors its kind always carries, but nothing that adds a territory, a severity
  grade or a symptom, or changes the diagnosis or plan.
- Records from a prior practice may mention routine care there, with normal results.
- Yearly physicals (2021, 2022, 2024, 2025; 2023 cancelled), CBC and CMP at physicals,
  lipids once or twice (normal), PHQ-2 screens negative.
- Influenza vaccine each fall (nurse visits or at physicals; one fall at a pharmacy,
  reported), COVID vaccines (primary series before joining, reported; later boosters),
  Tdap at joining, PCV20 in 2024. HIV and hepatitis C screening once (2021), negative.
- Portal messages about results, scheduling, school-year forms (fitness to coach, with no
  clinical content beyond the timeline), records requests to pulmonology, reminders.
- Booked visit after 2026-08-27: annual physical 2026-09-10.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002648 on 2026-04-20. Practice contacts after that day are
only messages, refills and scheduling; outside records after it are filed as received
without changing the practice's lists. A visit is booked after today.
