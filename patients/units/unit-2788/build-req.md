# Build requirements: unit-2788 (Megan Larkin, MRN00002788)

Source: synthetic_hospital patient 2788, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz text ("you suspect an underlying genetic
cause").

## Who she is (S)

Megan Larkin, born 1987-01-09, female, White, not Hispanic or Latino, English,
Holyoke MA 01040, commercial PPO. Graphic designer. Never smoker, no alcohol, no drugs,
no occupational exposures. **No known drug allergies.** Surgical history:
tonsillectomy and adenoidectomy at age 7. Family history: father with early-onset
emphysema diagnosed at 44; maternal aunt with rheumatoid arthritis; mother with eczema
(R1).

Joined the practice 2023-09-12 (A1, a new-patient visit). She brought:

- mild persistent asthma (childhood asthma, quiet through her teens, back in her
  mid-twenties, R2): fluticasone propionate HFA 110 mcg, 2 puffs twice daily;
  albuterol HFA 90 mcg, 2 puffs every 4 to 6 hours as needed for wheeze or shortness
  of breath;
- atopic dermatitis: triamcinolone acetonide 0.1% cream to affected areas twice daily
  for flares.

Ibuprofen is not on her list at joining (R3). Alpha-1 antitrypsin deficiency and
rheumatoid arthritis are not known at joining; each starts at its encounter.

Records texture (seed): sparse. Outside care reaches the practice as short letters,
the patient's own account and fill notices, with few numbers. What reaches the
practice for each outside contact is stated in the timeline.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2023-09-12 | practice | new-patient visit (A1): history as above; asthma well controlled on fluticasone, albuterol rarely; eczema flares in winter. Baseline labs, flu vaccine (P) |
| 2023-09-26 | outside records | a brief summary from her prior practice (P): asthma history (R2), Pap with HPV in 2022 normal, Tdap in 2019 |
| 2024-05-21 | practice | annual physical (P): no respiratory symptoms; asthma controlled |
| 2024-09-24 | practice | office visit (A2): about 4 months of progressively worse shortness of breath on exertion and a daily cough with clear sputum despite taking fluticasone as prescribed; using albuterol more often; prolonged expiratory phase on exam; SpO2 normal. **Referred to pulmonology.** Flu vaccine (P) |
| 2024-12-04 | outside (pulmonology) | consult: 6 months of progressive shortness of breath and chronic cough with clear sputum despite adherence to asthma maintenance; never smoker, no occupational exposures; father's emphysema at 44 noted (R1); decreased breath sounds at the lung bases and prolonged expiratory phase; CBC within normal limits; chest x-ray with hyperinflation and basilar-predominant emphysema; spirometry FEV1/FVC 65% (normal >70%), obstructive, with minimal improvement after bronchodilator. **Emphysema recorded.** Young never smoker without COPD risk factors: alpha-1 antitrypsin deficiency suspected, serum AAT level drawn, pending (R4, R5). Reaches the practice as a short consult letter received 2024-12-16 |
| 2024-12-19 | outside (pulmonology) | serum AAT 21 mg/dL (reference 100 to 190), resulted 2024-12-11; phenotype PiZZ, resulted 2024-12-19 (A3). **Alpha-1 antitrypsin deficiency diagnosed.** The practice learns from the patient's portal message 2024-12-20; a faxed copy of both results arrives 2025-01-06 |
| 2025-01-08 | practice | office visit after the diagnosis (P): discussion; PCV20, hepatitis A dose 1, Heplisav-B dose 1 (A4 plan, P) |
| 2025-01-22 | outside (pulmonology) | follow-up (A4): full PFTs with post-bronchodilator FEV1 58% predicted, FEV1/FVC 0.65, DLCO 63% predicted; CT chest with lower-lobe predominant panlobular emphysema, no nodules, no bronchiectasis; serum IgA 210 mg/dL (normal). Plan: weekly IV alpha-1 proteinase inhibitor 60 mg/kg by home infusion once insurance approves; inhaler regimen unchanged; hepatitis A and B and pneumococcal vaccines through primary care; yearly liver tests and one liver ultrasound; strict smoke avoidance. Reaches the practice as a short note received 2025-02-05 |
| 2025-02-05 | practice | nurse visit: Heplisav-B dose 2 (P) |
| 2025-02-19 | practice | hepatic panel normal; abdominal ultrasound: normal liver size and echotexture, no mass, normal spleen (A5) |
| 2025-03-11 | outside (home infusion) | **alpha-1 proteinase inhibitor (human) 60 mg/kg IV weekly started** by pulmonology, first home infusion (A6). The practice learns from the home-infusion pharmacy's notice |
| 2025-06-10 | practice | annual physical (P): weekly infusions going well, breathing a little better; for about a month some aching and morning stiffness in her hands, which she puts down to long days at the computer; **takes over-the-counter ibuprofen 200 mg, 1 to 2 tablets every 6 to 8 hours as needed (max 1200 mg/day)** (R3); physician agrees with as-needed use and asks her to come back if it persists |
| 2025-07-15 | practice | nurse visit: hepatitis A dose 2 (P) |
| 2025-08-20 | outside (pulmonology) | routine follow-up (A7): stable, infusions continue, no change. Reaches the practice only as the patient's portal account |
| 2025-10-14 | practice | office visit (A8): since about May, her fingers turn white then blue in the cold or under stress, painful, sometimes with tingling; joint pain and morning stiffness in hands and knees; ibuprofen as needed helps partly; no digital ulcers, capillary refill brisk. **Referred to rheumatology.** Flu and COVID vaccines (P) |
| 2025-11-24 | outside (rheumatology) | consult: 6 months of episodic cold- and stress-triggered white-then-blue fingers, painful, sometimes tingling; occasional joint pain and morning stiffness in hands and knees; history of mild persistent asthma, atopic dermatitis and alpha-1 antitrypsin deficiency; family history as above; denies smoking, alcohol or drugs; vital signs normal; pallor of the fingertips bilaterally without ulceration or gangrene, capillary refill under 2 seconds; rest of exam unremarkable. Labs drawn that day (R6): CBC normal, ESR 50 mm/hr (0 to 20), CRP 24 mg/L (<10), ANA positive, anti-centromere negative, anti-Scl-70 negative, RF positive, anti-CCP positive. **Raynaud phenomenon recorded** |
| 2025-12-02 | outside (rheumatology) | results call (A9): **rheumatoid arthritis, seropositive, diagnosed; hydroxychloroquine 200 mg twice daily started**; Raynaud managed with warm gloves and cold avoidance, no drug; no features of scleroderma; ibuprofen as needed continues; baseline eye exam within the year. The practice learns from the patient's portal message 2025-12-03; a short rheumatology letter with the 2025-11-24 lab values arrives 2025-12-17 |
| 2026-01-13 | outside (eye care) | baseline eye exam for hydroxychloroquine (A10): normal, no retinal toxicity, visual fields full. Letter received 2026-01-20 |
| 2026-03-23 | outside (rheumatology) | follow-up (A11): morning stiffness down to about 15 minutes, joint pain improved, fewer Raynaud episodes with gloves; continue hydroxychloroquine. Reaches the practice only as the patient's account at the June physical |
| 2026-06-09 | practice | annual physical (P): RA better on hydroxychloroquine; weekly infusions continue; asthma stable on fluticasone. CBC, CMP (P) |
| 2026-10-13 | practice | booked follow-up visit (P) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The pulmonology note says she has no family history of lung disease; the
  rheumatology note and profile give a father with early-onset emphysema at 44.
  Resolution: the father's emphysema is in her history from joining and in the
  pulmonology letter, where it adds to the suspicion of an inherited cause. The
  "no family history of lung disease" statement is not carried.
- **R2.** The source says her childhood asthma resolved in her teens, yet she carries
  active mild persistent asthma on a daily inhaled steroid. Resolution: childhood
  asthma, quiet through her teens, returned in her mid-twenties and was treated as
  mild persistent asthma at her prior practice, on fluticasone since.
- **R3.** Ibuprofen "for joint pain" is on the home list at the December 2024
  pulmonology visit, but the joint pain began about six months before the November
  2025 rheumatology visit. Resolution: she starts over-the-counter ibuprofen for hand
  aches around May 2025; the practice first records it at the 2025-06-10 physical as
  a patient-reported drug. The pulmonology letters do not list it.
- **R4.** The rheumatology note stamps alpha-1 antitrypsin deficiency as diagnosed on
  the pulmonology visit date, when the level was still pending. Resolution: the
  diagnosis is made on 2024-12-19 when the phenotype returns (A3).
- **R5.** The pulmonology note ends on quiz text with no plan. Resolution: the letter
  states that AATD is suspected in a young never smoker with basilar emphysema and
  that a serum AAT level was sent, with phenotyping to follow if low.
- **R6.** The rheumatology labs are "ordered" and resulted in the same note.
  Resolution: drawn at the 2025-11-24 visit, resulted over the next few days, and the
  diagnosis and plan follow by phone on 2025-12-02 (A9).
- **Left as the source has it:** the inhaler regimen is not stepped up for emphysema
  with progressive dyspnea; nobody adds a long-acting bronchodilator or pulmonary
  rehabilitation, and no note comments on it.

## Additions (A): physician to confirm

- **A1.** 2023-09-12 new-patient visit: the seed gives about 3 years with the
  practice, and the source's first encounter is a specialist referral.
- **A2.** 2024-09-24 practice visit that sends her to pulmonology (onset about four
  months earlier, so six months by the consult).
- **A3.** Serum AAT 21 mg/dL and PiZZ phenotype: the source asserts the deficiency
  with the level pending; ZZ fits basilar emphysema in a never smoker at 37.
- **A4.** Pulmonology follow-up 2025-01-22 with the staging tests augmentation needs
  (FEV1 58% predicted post-bronchodilator, FEV1/FVC 0.65, DLCO 63%; CT lower-lobe
  panlobular emphysema without nodules or bronchiectasis; IgA 210 mg/dL) and the
  standard plan (vaccines, liver monitoring, smoke avoidance).
- **A5.** Liver monitoring for PiZZ: hepatic panel normal and one abdominal
  ultrasound normal (2025-02-19); liver enzymes stay normal on yearly CMPs.
- **A6.** Weekly IV alpha-1 proteinase inhibitor (human) 60 mg/kg from 2025-03-11:
  the standard ongoing therapy for ZZ AATD with emphysema and FEV1 58% predicted, run
  by pulmonology through home infusion. No infusion reactions.
- **A7.** Pulmonology follow-up 2025-08-20, stable, known only from the patient.
- **A8.** 2025-10-14 practice visit that sends her to rheumatology.
- **A9.** Rheumatology results call 2025-12-02: seropositive RA diagnosed;
  hydroxychloroquine 200 mg twice daily (chosen conservatively over methotrexate given
  her lung disease and ZZ liver risk); non-drug Raynaud measures.
- **A10.** Baseline eye exam for hydroxychloroquine, normal.
- **A11.** Rheumatology follow-up 2026-03-23 with improvement, known only from the
  patient.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may
  carry the descriptors its kind always carries, but nothing that adds a territory, a
  severity grade or a symptom, or changes the diagnosis or plan.
- Records from a prior practice may mention routine screening done there, with normal
  results (Pap with HPV 2022 normal; Tdap 2019).
- Yearly physicals with vitals, PHQ-2, CBC and CMP; lipids, HIV and hepatitis C
  screening once at joining, all normal. Weight about 61 kg, height 165 cm, steady.
  Blood pressure normal throughout.
- Vaccines: influenza each fall, updated COVID each fall, PCV20 once (chronic lung
  disease), hepatitis A two doses and Heplisav-B two doses in 2025 (pulmonology's
  recommendation for AATD). Cervical screening next due 2027; mammography discussed
  for when she turns 40.
- Refills of fluticasone, albuterol and triamcinolone; home-infusion pharmacy calls
  and notices; portal messages about results, refills and scheduling; one cancelled
  visit.
- Booked visit after 2026-08-27: a routine follow-up on 2026-10-13 (flu and COVID
  vaccines, asthma and AATD check).

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.


## Last seen

The practice last saw MRN00002788 on 2025-06-10. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. Nothing is booked after today.
