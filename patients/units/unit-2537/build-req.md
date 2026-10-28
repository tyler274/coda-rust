# Build requirements: unit-2537 (Henry Silva, MRN00002537)

Source: synthetic_hospital patient 2537, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its vignette phrasing.

## Who he is (S)

Henry Silva, born 1995-09-05, male, White, not Hispanic or Latino, English, lives in
Amherst (01002), employer-sponsored commercial insurance. Construction worker.
Smoked 1 pack per day from his late teens; quit in late April 2024 (R2). Alcohol:
social, 2 to 3 drinks a week. No known drug allergies throughout. Surgical history:
appendectomy at age 14 (about 2010). Family history: father with gout and major
depressive disorder; mother with hypertension and type 2 diabetes; one older brother,
healthy.

Joined the practice 2022-07-12 (a new-patient visit, A1), age 26, bringing major
depressive disorder (diagnosed by his prior primary care office in early 2020) on
sertraline 50 mg daily. Hyperuricemia is found on his first practice labs (A2).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2022-07-12 | practice | new-patient visit (A1): history as above; current smoker 1 ppd, not ready to quit; height 180 cm, weight 84 kg; PHQ-9 6; exam normal. Labs: CMP, CBC, lipid panel, uric acid (father's gout), HIV and hepatitis C screening |
| 2022-07-14 | practice | results: CMP, CBC, lipids normal, creatinine normal; HIV and hepatitis C nonreactive; **uric acid 8.4 mg/dL: hyperuricemia added** (A2), no drug, recheck yearly |
| 2022-07-26 | outside | prior primary care records: MDD diagnosed early 2020, sertraline started then and settled at 50 mg; appendectomy at 14; Tdap 2017 |
| 2023-01-24 | practice | mood and tobacco follow-up; PHQ-9 5; still smoking 1 ppd |
| 2023-08-10 | practice | annual physical (after a missed 2023-07-20 visit); uric acid 8.9; other labs normal; PHQ-9 4 |
| 2024-05-09 | practice | portal: he stopped smoking about two weeks ago, without medication (R2) |
| 2024-06-12 | practice | telehealth mood/tobacco check; **nicotine dependence resolved, history of tobacco dependence recorded**; PHQ-9 4 |
| 2024-10-09 | practice | pre-physical labs: uric acid 9.5; creatinine normal; others normal |
| 2024-10-16 | practice | annual physical: **allopurinol 300 mg daily started** for the rising uric acid with his father's gout in mind (R1); flu vaccine |
| 2024-10-31 | practice | Source encounter 0. Several days of new ankle swelling and fatigue, about two weeks into allopurinol; denies rash, fever, dysuria, urgency, flank pain; vitals normal; bilateral pitting edema of the lower legs. Creatinine 2.3 mg/dL, BUN 34 mg/dL. Urinalysis: mild proteinuria (protein 1+) and hematuria (blood 1+, urine RBC 8/hpf), no RBC casts, no bacteria, nitrite and leukocyte esterase negative (urine WBC 2/hpf). **Acute kidney injury and acute tubulo-interstitial nephritis, suspected allopurinol-related, diagnosed** (R3); **allopurinol stopped**. Urine culture sent (negative, resulted 2024-11-02). Renal ultrasound and nephrology referral ordered (A3) |
| 2024-11-05 | practice-ordered | renal ultrasound: normal-sized kidneys, no hydronephrosis, no stones or masses (A3) |
| 2024-11-07 | practice | repeat BMP: creatinine 1.7, BUN 25 (A3) |
| 2024-11-20 | outside | nephrology consult letter (A3): picture fits drug-induced interstitial nephritis from allopurinol, improving off the drug; creatinine 1.3; no biopsy and no steroids while it keeps improving; avoid allopurinol |
| 2024-12-10 | practice | follow-up: edema and fatigue gone; creatinine 1.0, BUN 15; **AKI and interstitial nephritis resolved**; uric acid left untreated for now (gap, below) |
| 2025-05-13 | outside | Source encounter 1, regional hospital ED. Age 29 (R4). High-voltage electrical injury from a power line at a construction site; alert and oriented; severe left arm pain, the arm swollen and red; left forearm tense and tender; passive finger stretch causes severe pain. BP 135/85, HR 110, RR 18, temp 37.2 C. Creatinine 1.8 (ED: baseline unknown), potassium 5.5, calcium 7.9, phosphorus 6.2, CK 20,000; urine dipstick 3+ blood with no RBCs on microscopy. **Electrical injury, traumatic compartment syndrome of the left forearm and hand (R5), rhabdomyolysis with acute kidney injury (R6) diagnosed**. Tdap given (A4) |
| 2025-05-13 to 2025-05-20 | outside | admission (A4): emergency fasciotomy of the left forearm and hand the night of arrival; IV fluids; delayed primary closure of the fasciotomy wounds 2025-05-17; potassium normal by the next day; at discharge creatinine 1.0, CK 1,850; discharged home with **acetaminophen-oxycodone 325/5 mg as needed, short course**, hand surgery and occupational therapy follow-up, off work |
| 2025-05-21 | practice | post-discharge phone call (P) |
| 2025-05-27 | practice | post-discharge visit (P): wounds closed and healing; creatinine 0.9, CK 240; **rhabdomyolysis, AKI and electrical injury resolved; compartment syndrome resolved after fasciotomy**; PHQ-9 6 |
| 2025-06-11 | outside | hand surgery letter (A4): wounds healed, stiffness improving with occupational therapy |
| 2025-08-06 | outside | hand surgery letter (A4): near-full motion and grip, discharged from hand surgery care; cleared for modified duty |
| 2025-11-12 | outside | house fire; admitted to a regional burn center with 40% TBSA burns; wound care and escharotomy on arrival; IV fluids, morphine for pain, prophylactic antibiotics; enteral feeding (S) |
| 2025-11-15 | outside | Source encounter 2, burn ICU, hospital day 4 (age 30): fever 38.9 C, HR 120, BP 90/60; abdominal discomfort, vomited twice, NG output 600 mL in 6 hours; lethargic; abdomen distended and tender, hypoactive bowel sounds. WBC 16.0, hemoglobin 10.2, platelets 250, sodium 135, potassium 4.5, chloride 100, bicarbonate 22, BUN 28, creatinine 1.4, glucose 110, lactate 4.2. **Septic shock diagnosed** (R7) |
| 2025-11-15 to 2025-12-23 | outside | rest of the stay (A5): fluids, norepinephrine for about 48 hours, vancomycin and piperacillin-tazobactam for 7 days; blood and wound cultures grew no pathogen; feeds held with gastric decompression, restarted on day 7 and tolerated; excision and split-thickness skin grafting 2025-11-19 and 2025-12-02, grafts took; discharged home 2025-12-23 with creatinine 0.9, hemoglobin 10.9, WBC 8.4, on **acetaminophen-oxycodone 325/5 mg as needed, short course**, burn clinic and occupational therapy follow-up, pressure garments |
| 2025-11-17 | practice | his brother phones the practice to say he is in the burn unit (P) |
| 2025-12-24 | practice | post-discharge phone call (P) |
| 2026-01-06 | practice | post-discharge visit: weight 77 kg; grafted areas healing; hemoglobin 12.1, creatinine 0.9; **septic shock resolved**; burn center list still showed allopurinol, he confirms he has not taken it since October 2024 (R1); PHQ-9 9, sertraline unchanged |
| 2026-01-28 | outside | burn clinic letter (A5): grafts healed, pressure garments, scar massage, OT; remains off work |
| 2026-03-11 | practice | telehealth mood check; PHQ-9 7 |
| 2026-04-15 | outside | burn clinic letter (A5): grafts stable, range of motion good, continue garments; return to work to be decided by burn clinic |
| 2026-06-04 | practice | **last seen**: annual physical; weight 81 kg; PHQ-9 5; labs CMP, CBC, lipids, uric acid 9.1; hemoglobin 14.0 |
| 2026-07-15 | outside | burn clinic letter (A5), filed after last seen: stable, may start a graded return to work in the fall |
| 2026-09-17 | practice | booked: follow-up visit |

## Resolutions of source contradictions (R): physician to confirm

- **R1. Allopurinol.** The source says he "recently completed a 2-week course" of
  allopurinol, yet lists allopurinol 300 mg daily as a home medication at every
  encounter. Resolution: the practice starts allopurinol 300 mg daily on 2024-10-16
  and stops it at the 2024-10-31 visit; it is never restarted. The ED, hospital and
  burn center lists (2025) still copy allopurinol 300 mg daily, as outside lists do;
  the practice list never shows it after October 2024, and the 2026-01-06 visit
  confirms he does not take it. Starting at 300 mg rather than a low dose for an
  asymptomatic uric acid is an ordinary decision that is not the best one; no note
  remarks on it.
- **R2. Smoking.** Every encounter says he quit "6 months ago". Resolution: quit in
  late April 2024, six months before encounter 0; before that he smoked 1 ppd. Later
  outside records say "former smoker" without re-dating.
- **R3. "Acute pyelonephritis" at encounter 0.** The source labels the encounter acute
  pyelonephritis, but there is no fever, flank pain, pyuria, bacteria or positive
  culture; the picture (two weeks into allopurinol, edema, creatinine 2.3, mild
  proteinuria and hematuria, negative culture) is drug-induced acute
  tubulo-interstitial nephritis, the same ICD-10 code (N10). Resolution: the chart
  records acute tubulo-interstitial nephritis (N10), suspected allopurinol-related,
  with acute kidney injury (the source comorbidity). The word "pyelonephritis" is
  not used. The 2025 outside notes mention prior kidney injury from allopurinol.
- **R4. Ages.** The ED note says 32; by the seed's birth date he is 29 on 2025-05-13
  and 30 on 2025-11-15 (the ICU note's 30 stands). "Seen in January 2020" becomes
  October 2024.
- **R5. Compartment syndrome.** The source names the hand but examines the forearm.
  Resolution: traumatic compartment syndrome of the left upper extremity involving
  forearm and hand (T79.A12A), treated by fasciotomy of both (A4).
- **R6. Kidney injury in the ED.** CK 20,000 and 3+ blood with no RBCs is myoglobinuria:
  the comorbid acute kidney injury is from rhabdomyolysis. ED says "baseline unknown";
  the practice knows a baseline near 1.0 (2024-12-10).
- **R7. Septic shock source.** The source gives no source of sepsis. Resolution:
  cultures grow no pathogen and no source is named (A5); the abdominal findings settle
  with gastric decompression and are not given a separate diagnosis. "Previously
  treated for acute pyelonephritis" in the ICU history is re-voiced per R3.
- **Left as the source has it:** after October 2024 no one offers another urate-lowering
  drug; uric acid stays about 9 with no gout attacks and no note revisits it. The
  practice allergy list stays "No known drug allergies"; the allopurinol reaction is
  in the notes only. No one changes the sertraline dose after the burns despite a
  PHQ-9 of 9.

## Additions (A): physician to confirm

- **A1.** 2022-07-12 new-patient visit (four years with the practice; the source's first
  encounter reads as an established patient with known hyperuricemia).
- **A2.** Hyperuricemia found on practice labs: uric acid 8.4 (2022-07), 8.9 (2023-08),
  9.5 (2024-10), 9.1 (2026-06), creatinine normal on each.
- **A3.** Encounter 0 workup: urine culture negative (source), renal ultrasound normal,
  creatinine 1.7 / BUN 25 on 2024-11-07, nephrology letter with creatinine 1.3, and
  creatinine 1.0 / BUN 15 on 2024-12-10.
- **A4.** Encounter 1 care: Tdap in the ED; admission 2025-05-13 to 2025-05-20 with
  fasciotomy of left forearm and hand, delayed primary closure 2025-05-17; discharge
  creatinine 1.0 and CK 1,850; acetaminophen-oxycodone short course; hand surgery letters
  2025-06-11 and 2025-08-06; post-discharge creatinine 0.9 and CK 240; return to
  modified duty August 2025.
- **A5.** Encounter 2 care: norepinephrine about 48 hours, vancomycin and
  piperacillin-tazobactam 7 days, culture-negative; split-thickness skin grafting
  2025-11-19 and 2025-12-02; discharge 2025-12-23 with creatinine 0.9, hemoglobin 10.9,
  WBC 8.4; acetaminophen-oxycodone short course; burn clinic letters 2026-01-28,
  2026-04-15, 2026-07-15; hemoglobin 12.1 (2026-01) and 14.0 (2026-06). Burn locations
  and depth are not stated anywhere ("grafted areas").

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Records from a prior practice may mention routine screening done there, with normal results.
- Visits for mood, tobacco and the kidney and injury follow-ups as above; yearly physicals
  with CMP, CBC, lipids and uric acid; PHQ-9 at most visits (values above; others 3 to 6).
  Weight 84 kg until the burns, 77 kg in January 2026, 81 kg in June 2026. BP normal.
- Prevention: HIV and hepatitis C screening once (nonreactive); influenza each fall
  (2022 to 2025), COVID vaccines 2022, 2023 and 2025; Tdap from prior records (2017) and the
  2025 ED.
- Refills of sertraline, pharmacy traffic, workers' compensation and work-status forms after
  the electrical injury and burns, portal messages about results and scheduling, one missed
  visit (2023-07-20).
- Booked visit after 2026-08-27: follow-up 2026-09-17.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002537 on 2026-06-04. Practice contacts after that day are only
messages, refills and filed outside records, without changing the practice's lists. A visit
is booked after today.
