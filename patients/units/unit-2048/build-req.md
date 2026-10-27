# Build requirements: unit-2048 (Derek Mayhew, MRN00002048)

Source: synthetic_hospital patient 2048, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz text (the "recent ecological study" line).

## Who he is (S)

Derek Mayhew, born 1978-02-21, male, White, not Hispanic or Latino, English, lives in
Easthampton (01027), Blue Cross Blue Shield commercial plan. Construction worker.
Former smoker, 30 pack-years, quit in late 2018 at age 40 (R1). Alcohol 1 to 2 drinks a
month, socially. No known drug allergies. Surgical history: tonsillectomy at age 12.
Family history: father with hypertension (72 in 2024), mother with type 2 diabetes (75
in 2024); no family history of interstitial lung disease or pulmonary fibrosis.

Joined the practice 2021-09-14 (A1, a new-patient visit; seed: about 5 years with the
practice). He brought: essential hypertension (onset about 2016) on hydrochlorothiazide
25 mg daily; GERD without esophagitis (onset about 2019) on omeprazole 20 mg daily;
history of tobacco dependence (the source's "tobacco use disorder", in a former smoker,
R1). Height 178 cm, weight about 87 kg, steady throughout the chart (P).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2021-09-14 | practice | new-patient visit (A1); history as above; lungs clear; BP at goal on hydrochlorothiazide; baseline labs, HIV and hepatitis C screening (P) |
| 2024-02-19 | practice | about 2 months (since mid-December 2023) of progressive exertional dyspnea and a nonproductive cough; no recent travel or sick contacts; no prior lung disease (R2). BP 135/85, HR 88, RR 20, SpO2 94% on room air; bilateral inspiratory crackles at the bases. Labs: CBC within normal limits (Hgb 14.5, WBC 7.0, platelets 250; R3), creatinine 0.9, ESR 20, ANA negative, RF negative. Chest x-ray the same day: bilateral reticular opacities, lower-lobe predominant. HRCT ordered (R4) |
| 2024-02-22 | practice (imaging) | HRCT chest: subpleural honeycombing and traction bronchiectasis (R4). **Idiopathic pulmonary fibrosis entered on the problem list** (onset 2023-12), urgent pulmonology referral placed |
| 2024-04-16 | outside | pulmonology consult letter (received 2024-04-23): returns the February work-up; no chest pain, fever or weight loss; BP 135/85, HR 78, SpO2 94% on room air; bibasilar inspiratory crackles, no clubbing, no peripheral edema; reviews the February labs (Hgb 14.5, WBC 7.0, platelets 250, creatinine 0.9, ANA and RF negative) and its own read of the February HRCT: bilateral, subpleural, basal-predominant reticular abnormality with minimal honeycombing (R3, R4). PFTs today: FVC 70% predicted, DLCO 55% predicted. IPF confirmed; **nintedanib 150 mg twice daily started** (R5, A3), liver tests to be monitored by pulmonology |
| 2024-05-14 | practice | follow-up after pulmonology (A2): reviews the letter; nintedanib added to the practice list; pneumococcal vaccine for chronic lung disease (P) |
| 2024-06-03 | outside | pulmonology follow-up letter (received 2024-06-10): exertional dyspnea and dry cough worse over the past 2 weeks; worried about lung cancer given his smoking and asks how likely it is. BP 135/85, HR 88, RR 20, SpO2 94% on room air; bilateral inspiratory crackles at the bases. Labs: Hgb 14.5, WBC 7.0, platelets 250; sodium 140, potassium 4.2, creatinine 1.0, calcium 9.8, liver tests normal; CEA within normal limits. ANA and RF negative (restated, not redrawn; R3). Chest x-ray: bilateral interstitial infiltrates. Repeat HRCT: interstitial lung disease as before, no mass or nodule (P normal finding). PFTs: restrictive pattern with reduced diffusion capacity (no numbers). Reassured on cancer; nintedanib continued |
| 2025-02-05 | outside | pulmonology follow-up letter (received 2025-02-12): exertional dyspnea and nonproductive cough slowly progressing over the past 6 months (R6); no recent travel or sick contacts; again asks about his lung cancer risk given his smoking history. BP 135/85, HR 88, RR 20, SpO2 94% on room air; bilateral end-inspiratory crackles. Chest x-ray: diffuse bilateral interstitial infiltrates. HRCT: honeycombing and traction bronchiectasis, no mass or nodule. PFTs: restrictive pattern with reduced diffusion capacity. Nintedanib continued |
| 2026-02-05 | practice | **last seen**: annual physical (P). SpO2 94%, crackles at the bases as before; BP at goal; labs and FIT kit |
| 2026-02-17 | outside | pulmonology follow-up letter (received 2026-02-24, filed; the practice lists are not changed): 47-year-old with hypertension and GERD; progressive exertional dyspnea and nonproductive cough; no recent travel or sick contacts; construction work noted (R7). SpO2 94% on room air; bilateral end-inspiratory crackles. Chest x-ray: diffuse bilateral interstitial infiltrates. HRCT: subpleural reticular changes and honeycombing, no significant lymphadenopathy. PFTs: restrictive pattern with reduced diffusion capacity. Nintedanib continued |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Pack-years are 10, 20, 30 and "20 years of a pack a day" across the notes, and
  the first note reads as if he still smokes. Resolution: 30 pack-years (profile and
  seed), quit in late 2018 at age 40, former smoker throughout. The 10 and 20 figures
  are not carried. "Tobacco use disorder" is history of tobacco dependence.
- **R2.** Encounter 0 says he "denies significant past medical history" while listing
  hypertension and GERD. Resolution: he has no prior lung or heart history; the note
  lists his hypertension and GERD.
- **R3.** Encounter 0 gives "CBC within normal limits" and encounter 1 the same labs
  with values. Resolution: the February 2024 CBC is Hgb 14.5, WBC 7.0, platelets 250,
  creatinine 0.9; the April letter reviews those results rather than redrawing them.
  The June 2024 draw is pulmonology's own, with the values it lists. ANA and RF were
  drawn once (February 2024); later letters restate them.
- **R4.** Encounter 0 puts a chest x-ray and an HRCT at a single office visit, and
  encounter 1 describes an HRCT two months later with "minimal" honeycombing.
  Resolution: chest x-ray the day of the visit, HRCT three days later; the April
  description is pulmonology's own read of that same February scan (an interobserver
  difference in wording, left as written). The source's IPF stamp at encounter 0 maps
  to the problem-list entry on the HRCT day.
- **R5.** Encounter 1's plan stops mid-sentence. Resolution: pulmonology confirms IPF
  and starts an antifibrotic (A3).
- **R6.** Encounter 3's "6-month history" is read as the course over the past six
  months, not a new start; the years it names follow the re-dated timeline (seen
  February, April and June 2024).
- **R7.** Encounter 4's ecological study of metal dust is quiz text and is not carried.
  The letter notes his construction work only.
- **Profile vs. notes.** Encounters 1 and 2 drop hydrochlorothiazide from the home list
  while the prose says he takes it. Resolution: hydrochlorothiazide 25 mg daily
  throughout. The pulmonology letters list it.
- **Left as the source has it:** blood pressure runs about 135/85 on hydrochlorothiazide
  alone at every contact; no one changes it. No supplemental oxygen and no pulmonary
  rehabilitation are raised; resting SpO2 stays 94%. Nobody remarks on either.

## Additions (A): physician to confirm

- **A1.** 2021-09-14 new-patient visit (seed: about 5 years with the practice), with
  prior-practice records received 2021-10 (hypertension and GERD history, tonsillectomy,
  normal routine labs).
- **A2.** 2024-05-14 practice follow-up after the pulmonology consult.
- **A3.** Nintedanib 150 mg by mouth twice daily, started by pulmonology 2024-04-16 and
  continued through the source (standard antifibrotic for IPF, which the source asserts
  but never treats). The source's pulmonology medication lists were copies of the
  profile; the chart's letters list nintedanib. Pulmonology checks liver tests about
  every 3 months; all normal; the practice receives the hepatic panels.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention
  (heart size, no effusion, no mass or nodule, mediastinum). An abnormal finding appears
  only as the source or this file gives it. A report may carry the descriptors its kind
  always carries, but nothing that adds a territory, a severity grade or a symptom, or
  changes the diagnosis or plan.
- Records from a prior practice may mention routine screening done there, with normal results.
- Before 2024 the lungs are clear and SpO2 is normal (97 to 98%). From 2024-02 his
  resting SpO2 at practice visits is 94% on room air and the exam keeps the basal
  crackles.
- Blood pressure about 135/85 at every visit; hypertension and GERD visits about every
  6 to 12 months; yearly physicals; basic or comprehensive metabolic panel yearly on the
  thiazide, lipids, A1c (mother with diabetes), CBC; all normal.
- Prevention: FIT yearly from 2023 (age 45), negative. Influenza vaccine each fall;
  COVID primary series in spring 2021 (patient report) and updated COVID vaccines in the
  fall from 2023; PCV20 in 2024 for chronic lung disease. HIV and hepatitis C screening
  once (2021), negative. PHQ-2 and AUDIT-C at physicals, low. No lung cancer screening
  (he is under 50 and has HRCTs anyway).
- Refills (hydrochlorothiazide, omeprazole), portal messages about results, refills and
  scheduling, a missed visit, reminders. Nintedanib is filled by a specialty pharmacy
  through pulmonology.
- No visit is booked after today.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002048 on 2026-02-05. Practice contacts after that day are
only messages, refills and the results of that day's orders; outside care after it
still happens (the 2026-02-17 pulmonology visit), and its records are filed as received
without changing the practice's lists. Nothing is booked after today.
