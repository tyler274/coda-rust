# Build requirements: unit-2778 (Erin Brooks, MRN00002778)

Source: synthetic_hospital patient 2778, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its unfinished plan text.

## Who she is (S)

Erin Brooks, born 1978-07-21, female, White, not Hispanic or Latino, English-speaking,
lives in Holyoke MA 01040, Blue Cross Blue Shield PPO. Administrative assistant. Never
smoker, no alcohol. No known drug allergies. Surgical history: tonsillectomy as a child.
Family history: mother with bipolar disorder, sister with migraine headaches, maternal
grandmother with hypothyroidism. Marital status recorded as single (no partner or
children appear in the chart).

Joined the practice 2017-06-12 (new-patient visit, A1). She brought:

- **Bipolar disorder**, long-standing, on **lithium carbonate 300 mg twice daily**,
  prescribed and followed by an outside psychiatry practice; mood stable (R1).
- **Migraine**, type not specified (R13), with **sumatriptan 100 mg as needed, maximum
  200 mg/day**, which the practice refills.
- **Raynaud phenomenon** since her twenties, followed about yearly by an outside
  rheumatology group; cold avoidance, no drug (R2, R4).

Height 165 cm; weight about 65 kg (BMI about 24) through 2025.

## Records (chart texture: gaps)

Some outside care never sends its letter or sends it late, and the practice learns of it
from the patient. Specifically:

- Outside psychiatry sees her about every 3 to 6 months; only three of its letters ever
  reach the chart (2018, 2020, 2025). The practice draws her lithium monitoring labs at
  psychiatry's request and copies results to them.
- The 2021 rheumatology letter arrives seven weeks late, after a records request; the
  practice first hears of amlodipine from the patient.
- The 2023 cardiology consult sends no letter until the practice requests it; the
  practice list carries vasospastic angina and nitroglycerin only from the letter's
  arrival (2023-12-11), seven weeks after the visit. The 2024 cardiology follow-up never
  sends a letter (known from her report only).
- The 2025-12 endocrinology follow-up letter arrives six weeks late.
- Rheumatology's routine yearly visits otherwise leave no letters.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2017-06-12 | practice | **new-patient visit (A1)**: history as above; exam normal; labs drawn (lithium level, CMP, TSH, CBC, lipids, HIV and hepatitis C screens); prior-practice records requested |
| 2017-07-10 | practice | prior-practice records received: Pap with HPV co-test 2016 normal, Tdap 2015 (P) |
| 2017 to 2020 | practice | routine care (P): lithium level, BMP and TSH about every 6 months, all in range; physicals; migraine and Raynaud follow-up within physicals; mammograms from 2018 |
| 2018-11-26 | outside | psychiatry letter received: stable on lithium, continue 300 mg twice daily (P) |
| 2020-10-07 | outside | psychiatry routine follow-up: stable throughout the past two years, adherent, no manic or depressive episodes; no complaints except increased thirst and urination over the past few weeks; exam unremarkable. Labs drawn by psychiatry that day: Na 142, K 4.1, creatinine 1.0, calcium 10.2, lithium 1.2 (top of range), TSH 5.5. Psychiatry calls this drug-induced (lithium) hypothyroidism and plans monitoring with a repeat TSH and free T4 through the practice in 6 to 8 weeks; lithium continued at 300 mg twice daily (R8) |
| 2020-10-19 | practice | psychiatry letter received with those labs; **drug-induced hypothyroidism added to the problem list** (onset 2020-10-07); portal note to her about a December recheck. The thirst is not taken up (Left as is) |
| 2020-12-07 | practice | repeat TSH still above range (about 5), free T4 normal, lithium level and BMP in range; monitoring, no levothyroxine (R8) |
| 2021-04-09 | outside | **rheumatology starts amlodipine 5 mg daily by phone (A2)** for more frequent vasospastic attacks; she already had a follow-up booked |
| 2021-04-12 | practice | portal message: since starting amlodipine she notices her heartbeat at rest and some facial flushing; amlodipine **entered as an outside medication**; advised to keep her rheumatology visit and call for chest pain, fainting or dizziness |
| 2021-04-14 | outside | rheumatology follow-up (age 42): fewer vasospastic episodes on the new drug; awareness of heartbeat at rest and occasional facial flushing; denies chest pain, dizziness, leg swelling; no recent migraine; vitals stable; heart without murmur, rub or gallop, lungs clear, extremities warm with good pulses and brisk capillary refill, no edema; baseline 12-lead ECG unremarkable. Assessment: chronic vasospastic disorder (her Raynaud phenomenon, R4); palpitations and flushing are expected effects of this drug class and do not warrant stopping; **amlodipine 5 mg continued** (R12) |
| 2021-06-07 | practice | rheumatology letter received after a 2021-05-24 records request |
| 2022-04-13 | practice | **telehealth visit (A3)**: about 9 days of 4 to 5 watery stools a day; same-day lithium level and BMP, both in range; referred to gastroenterology because it is persisting. Supplements are not asked about |
| 2022-04-18 | outside | gastroenterology consult (age 43): two weeks of 4 to 5 watery stools daily without blood or mucus; denies abdominal pain, weight loss, travel, sick contacts, food allergy, diet change; about three weeks earlier she began a magnesium oxide pill nightly (400 mg) after reading about natural remedies for migraine; vitals normal; abdomen soft, nontender, bowel sounds increased. CBC, electrolytes and TSH normal; stool culture, ova and parasites, and fecal leukocytes negative. **Magnesium-induced diarrhea**; told to **stop the magnesium oxide** (R3); colonoscopy deferred for her age and the absence of red flags |
| 2022-04-27 | practice | GI letter received; **magnesium-induced diarrhea added** (onset 2022-04-04); magnesium oxide recorded as a self-started supplement, now stopped |
| 2022-05-02 | practice | portal message: stopped the magnesium on 04-18, stools normal within a few days; **magnesium-induced diarrhea resolved** |
| 2022-06-13 | practice | physical; results 2022-06-14: TSH in range on a second draw without treatment; **drug-induced hypothyroidism resolved** on the problem list 2022-06-14 (R8) |
| 2023-10-02 | practice | **office visit (A4)**: about 6 weeks of brief tight chest discomfort at rest, mostly early morning, about 15 minutes, settling on its own; office 12-lead ECG normal sinus rhythm, no ST change; **referred to cardiology**; told to call 911 for pain lasting longer than 20 minutes |
| 2023-10-23 | outside | cardiology consult (early-morning appointment): chest pain at rest, usually early morning, a constricting feeling about 15 minutes long that resolves by itself; fingers turn white then blue in the cold or with emotional stress; history of migraine with sumatriptan as needed; BP 130/85, HR 72, RR 16; heart without murmur, rub or gallop, no peripheral edema. Troponin I normal (R5); triglycerides 100. A 12-lead ECG taken during an episode in clinic shows **transient ST-segment elevation** that settles with the pain (R6). **Vasospastic angina**; **amlodipine 5 mg continued; nitroglycerin 0.4 mg sublingual as needed added (A5)** |
| 2023-10-25 | practice | portal message from her: cardiology said the pain is spasm of the heart arteries and gave her nitroglycerin; a letter is to follow. Practice list not yet updated (records gap) |
| 2023-11-27 | practice | front desk requests the cardiology records |
| 2023-12-11 | practice | cardiology letter received; **vasospastic angina added** (onset 2023-08-21, diagnosed 2023-10-23) and **nitroglycerin entered as an outside medication** |
| 2024-04-22 | outside | cardiology follow-up (A6), no letter ever received; at her 2024-06-17 physical she reports episodes are less frequent and she has used nitroglycerin twice since October, amlodipine unchanged |
| 2025-09-29 | practice | portal message: her thirst has become much heavier, drinking 4 to 5 liters a day, up at night to urinate, poor sleep, feeling hot, some weight loss; offered a visit |
| 2025-10-06 | practice | **office visit (A7)**: those symptoms; weight about 2 kg below her usual; fine hand tremor; no thyroid enlargement or tenderness; labs: **TSH 0.06, free T4 2.4, sodium 145, creatinine 1.0, lithium 0.9**, glucose normal. Results back 10-07: **urgent endocrinology referral** |
| 2025-10-13 | outside | endocrinology consult (age 47): drinks about 4 to 5 liters of water a day, often up at night to urinate, difficulty sleeping, about 5 lb weight loss with unchanged appetite, heat intolerance; prior drug-induced hypothyroidism; on chronic lithium; vitals normal, BMI 23 (weight 62.6 kg); fine tremor of the hands and mild lid lag, thyroid not tender or enlarged. Endocrinology draws (A8): **TSH receptor antibody negative, TPO antibody 12 IU/mL (normal), serum osmolality 296, urine osmolality 145** |
| 2025-10-16 | outside | thyroid ultrasound (A8): normal-sized gland, no nodules, no increased vascularity |
| 2025-10-17 | outside | endocrinology calls with results: **lithium-induced thyroiditis (painless, thyrotoxic phase) with nephrogenic diabetes insipidus**; **amiloride 5 mg daily started (A9)**, lithium continued after discussion with psychiatry; no beta blocker because of vasospastic angina and Raynaud; repeat thyroid tests in about 6 weeks; lithium level and BMP 1 week after starting amiloride through the practice |
| 2025-10-20 | practice | endocrinology letter and ultrasound report received; **lithium-induced thyroiditis and nephrogenic diabetes insipidus added**; **amiloride entered as an outside medication** |
| 2025-10-24 | practice | lab visit (A9): **potassium 4.6, sodium 143, lithium 1.0**; copied to endocrinology and psychiatry |
| 2025-11-10 | practice | psychiatry letter received (A10): mood stable; continue lithium 300 mg twice daily with amiloride; level monitoring through the practice as before |
| 2025-12-01 | outside | endocrinology follow-up (A11): intake down to about 3 liters a day, up once a night, sleeping better; weight 63.4 kg; **TSH 0.4, free T4 1.3** |
| 2026-01-12 | practice | the December endocrinology letter received late |
| 2026-03-02 | outside | endocrinology follow-up (A11): **TSH 2.2, free T4 1.0, sodium 142, urine osmolality 240**; thyroiditis has run its course; continue amiloride and lithium; yearly endocrinology review |
| 2026-03-09 | practice | endocrinology letter received; **lithium-induced thyroiditis resolved**; nephrogenic diabetes insipidus stays active |
| 2026-06-22 | practice | physical (P): stable; Pap with HPV co-test; labs in range on lithium and amiloride |
| 2026-09-14 | practice | booked: lithium and nephrogenic diabetes insipidus follow-up with labs |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The 2020 psychiatry note says she has been stable on lithium "for the past two
  years" while the profile carries lithium from the start. Resolution: lithium is
  long-standing from before she joined in 2017; the 2020 letter says she has been stable
  throughout the past two years, and every earlier contact also finds her mood stable.
  No mood episode appears anywhere in the chart.
- **R2.** The profile lists amlodipine and magnesium oxide from the first encounter, but
  the 2021 rheumatology note says a new drug for the vasospasm began 5 days earlier, and
  the 2022 GI note says she recently began magnesium on her own. Resolution: amlodipine
  starts 2021-04-09 (A2); magnesium oxide is self-started about 2022-03-28 and stopped
  2022-04-18. Lithium and sumatriptan run from joining. The 2023 cardiology and 2025
  endocrinology letters still copy magnesium oxide onto their medication lists, as
  outside lists do; the practice list never carries it after 2022-04-27. The cardiology
  letter's list leaves sumatriptan off while its history mentions it.
- **R3.** The GI note says she "takes no prescription medications" yet lists three, and
  its plan never addresses the magnesium. Resolution: the GI letter lists her
  prescriptions, names the magnesium as a supplement and advises stopping it; the
  diarrhea resolves within days (2022-05-02). The "no prescription" line is not carried.
- **R4.** "Chronic vasospastic disorder (Raynaud's phenomenon/Buerger's disease)" is
  diagnosed in 2021 while Raynaud phenomenon is a chronic condition from the start.
  Resolution: one condition, Raynaud phenomenon, on the list from joining; the 2021
  rheumatology letter calls it a chronic vasospastic disorder. Buerger disease is not
  carried (never smoker, warm extremities, good pulses).
- **R5.** The cardiology troponin reads "50 mg/dL)" while the same note says cardiac
  biomarkers are normal. Resolution: troponin I within normal limits; the 50 is not
  carried. Triglycerides 100 carried.
- **R6.** An ECG "during an episode" at an outpatient visit. Resolution: she has an
  episode in the cardiology clinic at an early-morning appointment and a 12-lead taken
  then shows transient ST elevation that settles as the pain passes.
- **R7.** Source stamps: drug-induced hypothyroidism "diagnosed" at the psychiatry
  visit, vasospastic disorder at rheumatology, magnesium diarrhea at GI, vasospastic
  angina at cardiology. Resolution: each enters the practice list when its letter
  arrives (dates above). Outside letters may keep "drug-induced hypothyroidism" on
  their lists after the practice resolves it; the 2025 endocrinology letter calls it
  prior.
- **R8.** The source names drug-induced hypothyroidism but no thyroid drug, and the GI
  note in 2022 finds TSH normal. Resolution: subclinical, monitored without
  levothyroxine; TSH 5.5 (Oct 2020), about 5 in December 2020, 4 to 5 through 2021,
  back in range from April 2022; resolved on the problem list 2022-06-14.
- **R9.** "Long term (current) use of immunosuppressants" is not carried: she takes no
  immunosuppressant.
- **R10.** The rheumatology note's "January 2020" psychiatry visit follows the re-dated
  timeline (October 2020). Stated ages (42, 42, 43, 47) match the birth date.
- **R11.** The endocrinology note stops at the exam with no results or plan. Resolution:
  the chart completes it with A8 and A9.
- **R12.** The rheumatology plan ends mid-sentence. Resolution: palpitations and
  flushing are explained as expected dihydropyridine effects and amlodipine continues
  at 5 mg.
- **R13.** "Migraine disorder" / "benign headache or migraine" has no stated type.
  Resolution: one condition, migraine (unspecified type). The GI note's "vitamin
  supplementation" for migraine is the magnesium oxide.
- **Left as the source has it:**
  - The increased thirst and urination of October 2020, with a lithium level at the top
    of range, is not worked up; lithium stays 300 mg twice daily. No later note asks
    about thirst or urination until 2025.
  - Calcium 10.2 (upper limit) is never commented on; no PTH is drawn.
  - Sumatriptan stays on her list and is refilled after vasospastic angina is diagnosed;
    nobody stops it or remarks on it.
  - No coronary angiography or CT coronary angiography; amlodipine stays 5 mg.

## Additions (A): physician to confirm

- **A1.** 2017-06-12 new-patient visit (the first source encounter is a routine follow-up
  of a long-treated patient; seed: about 9 years with the practice).
- **A2.** Amlodipine 5 mg daily started by rheumatology by phone on 2021-04-09 (the
  source's "started 5 days ago").
- **A3.** 2022-04-13 telehealth visit that sends her to GI, with a same-day lithium level
  and BMP in range (diarrhea on lithium).
- **A4.** 2023-10-02 office visit with a normal office ECG and the cardiology referral;
  symptoms began about 2023-08-21.
- **A5.** Nitroglycerin 0.4 mg sublingual as needed from cardiology (standard care for
  vasospastic angina, which the source never treats beyond amlodipine).
- **A6.** 2024-04-22 cardiology follow-up, known only from her report: fewer episodes,
  nitroglycerin used twice since October 2023, amlodipine unchanged.
- **A7.** 2025-10-06 office visit and labs that send her to endocrinology: TSH 0.06,
  free T4 2.4, sodium 145, creatinine 1.0, lithium 0.9.
- **A8.** Endocrinology workup supporting the source's diagnosis: TSH receptor antibody
  negative, TPO antibody 12 IU/mL, serum osmolality 296, urine osmolality 145
  (2025-10-13); thyroid ultrasound normal size, no nodules, no increased vascularity
  (2025-10-16).
- **A9.** Amiloride 5 mg daily for nephrogenic diabetes insipidus, started by
  endocrinology 2025-10-17 with lithium continued; no beta blocker; 1-week check
  potassium 4.6, sodium 143, lithium 1.0 (2025-10-24).
- **A10.** Psychiatry letter (received 2025-11-10) agreeing to continue lithium with
  amiloride.
- **A11.** Endocrinology follow-ups: 2025-12-01 (about 3 L/day, nocturia once, weight
  63.4 kg, TSH 0.4, free T4 1.3) and 2026-03-02 (TSH 2.2, free T4 1.0, sodium 142, urine
  osmolality 240; thyroiditis resolved; continue amiloride).

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may
  carry the descriptors its kind always carries, but nothing that adds a territory, a
  severity grade or a symptom, or changes the diagnosis or plan.
- Records from the prior practice may mention routine screening done there, with normal
  results (Pap with HPV 2016, Tdap 2015).
- Lithium monitoring: lithium level, BMP and TSH about every 6 months (June and December,
  plus the draws above), copied to psychiatry; lithium 0.8 to 1.0 except as given;
  creatinine 0.9 to 1.0; calcium 9.6 to 10.2 without comment; sodium 139 to 143 before
  2025; TSH 2 to 3 in 2017 to 2019, about 3.5 to 4 by mid-2020, as R8 through 2022,
  2 to 3.5 from 2022 to mid-2025. Free T4 normal on every draw that includes it except
  2025-10-06.
- Yearly physicals with vitals, PHQ-9 (low scores), CBC and a metabolic panel; lipids
  in 2017, 2022 and 2026 (normal); migraine and Raynaud reviewed within physicals;
  sumatriptan refills about yearly, and a quantity-limit prior authorization.
- Screening: mammograms about every two years from 2018, normal; Pap with HPV co-test
  2021 and 2026, normal; colorectal screening by yearly FIT from age 45 (2023 through
  2026), negative, with one reminder for an unreturned kit. HIV and hepatitis C
  screening once (2017), nonreactive.
- Vaccines: influenza each fall; COVID-19 primary series spring 2021, booster late 2021,
  then yearly updated doses; Tdap 2025.
- Refills, pharmacy and prior-authorization traffic, portal messages about results,
  refills and scheduling, records requests, one missed visit.
- Booked visit after 2026-08-27: 2026-09-14 lithium and nephrogenic diabetes insipidus
  follow-up.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.


## Last seen

The practice last saw MRN00002778 on 2024-10-14. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. Nothing is booked after today.
