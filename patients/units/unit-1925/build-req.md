# Build requirements: unit-1925 (Chloe Eldridge, MRN00001925)

Source: synthetic_hospital patient 1925, in `source.md` beside this file. This file and
the source are the only authority for clinical facts. Every diagnosis, medication,
result, procedure and specialist finding in the chart must trace to one of:

- **S** a fact in source.md, carried as listed below;
- **R** a resolution below, where the source contradicts itself or stops short;
- **A** an addition below, listed one by one;
- **P** the primary-care layer described at the end.

Nothing else. No diagnosis, medication, test result, procedure, family member or
life event that is not on these lists. Texture (wording, scheduling, refills, portal
traffic, who signed what) is free, as long as it carries no new clinical fact.

Re-voice everything. The source notes are exam vignettes: never copy a sentence, and
never carry a source clinician's name, its "Active Problem List" dumps, its
"(diagnosed ...)" stamps, or its unfinished plan sentences.

## Who she is (S)

Born 1999-09-26, female, White, not Hispanic or Latino, English-speaking, single.
University student health insurance plan. Graduate student in environmental
science. Never smoker. Alcohol socially, one or two glasses of wine on weekends. No
drug allergies (NKDA). No surgical history. No chronic conditions and no regular
medications. Family history: no primary immunodeficiency, autoimmune disease or
chronic lung disease in the family. Raised in upstate New York; her family keeps a
property in Montana where she spends much of her time outdoors (snowmobiling, guided ice climbing,
horseback riding, canoeing, fishing, ATV trips with overnight camping); in December
2025 she had just been on a weekend hiking trip with friends.

Joined the practice 2024-09-04 (A1), healthy, on no medications, nothing on her
problem list. Records are sparse (seed): outside care reaches the practice late and
thin, mostly as the patient's account and short letters.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2024-09-04 | practice | new-patient preventive visit (A1): healthy, no medications, NKDA; baseline labs normal (P) |
| 2025-03-21 | practice | 10 days of persistent dry cough, headache and malaise; low-grade fever for about a week; denies sick contacts but several people in her graduate housing have had the same illness (R2). T 37.8 °C (100.0 °F), HR 88, RR 18, BP 120/75. Fine crackles at both bases, no wheezes or rhonchi; rest of exam normal. CBC: WBC 7.5 (normal differential), Hgb 13.8, platelets 250; CRP 21 mg/L, mildly elevated (R1). Chest x-ray: interstitial infiltrates in both lower lobes, no consolidation. **Atypical pneumonia diagnosed**; **azithromycin 5-day course** (A2) |
| 2025-04-07 | practice | portal: recovered; **pneumonia resolved** (P) |
| 2025-06-18 | practice | about three months later: 5 days of worsening cough (began dry, now more persistent, still not productive), fever, malaise; no hemoptysis or significant chest pain; mild breathlessness on stairs. T 38.5 °C (101.3 °F), HR 96, RR 18, BP 112/70, SpO2 95% on room air. Scattered crackles at the right base, no wheezes or rhonchi; no murmur, rub or gallop; rest of exam normal. Chest x-ray: patchy right lower lobe infiltrate, consistent with community-acquired pneumonia. Outdoor exposure history taken (R6). **Atypical pneumonia diagnosed again**; **erythromycin 500 mg four times daily for 7 days** (S, dose A3), counseled about stomach upset and finishing the course |
| 2025-07-08 | practice | follow-up (A4): well, lungs clear; **pneumonia resolved**; two pneumonias in three months, **pulmonology referral** placed; first new-patient opening is 2025-12-29 |
| 2025-12-29 | outside | pulmonology consult and procedure (records reach the practice 2026-01-07): 4 days of dry cough with mild chest discomfort, low-grade fever, malaise and mild shortness of breath, no hemoptysis; back from a weekend hiking trip with friends; no sick contacts. T 38.1 °C (100.6 °F), HR 92, BP 112/68, SpO2 96% on room air. Scattered crackles and faint expiratory wheezes over the left upper lung field; no egophony, no dullness to percussion. Chest x-ray: patchy left upper lobe infiltrate, no pleural effusion. **Atypical pneumonia diagnosed a third time**. A third pneumonia in a new lobe raises concern for an opportunistic infection, so a **bronchoscopy with bronchoalveolar lavage of the left upper lobe** is done the same day (R5, A5); blood for CBC with differential and immunoglobulins drawn (A5); **doxycycline 100 mg twice daily for 7 days** started (A5) |
| 2026-02-23 | outside | pulmonology follow-up (A6; letter reaches the practice 2026-03-04): well, lungs clear, repeat chest x-ray clear. Lavage and blood results as in A5: no opportunistic organism, immunity normal; opportunistic infection set aside; **pneumonia resolved**; discharged to primary care, return if it recurs |
| 2026-09-14 | practice | booked annual physical (P) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Encounter 0's plan stops mid-sentence and gives the CRP only as "mildly
  elevated". Resolution: the CRP is reported as 21 mg/L (reference under 10); the visit
  ends with atypical pneumonia diagnosed and treated (A2).
- **R2.** "Dorm mates" for a 25-year-old graduate student: carried as people in her
  graduate housing building.
- **R3.** Encounter 2 says prior evaluations were "in January and April 2020". The
  timeline is re-dated: March and June 2025.
- **R4.** Encounters 1 and 2 list atypical pneumonia as an active problem "diagnosed"
  in January 2020. Resolution: it is three separate episodes, each diagnosed at its own
  visit (2025-03-21, 2025-06-18, 2025-12-29) and resolved after recovery (2025-04-07,
  2025-07-08, 2026-02-23). Between episodes it is history, not active.
- **R5.** The source's second primary diagnosis, "opportunistic pulmonary infection
  requiring diagnostic sampling", sits in a healthy, immunocompetent never-smoker on no
  medications, and encounter 2 is a pulmonology "procedure" visit. Resolution: it is the
  pulmonologist's working concern that justifies sampling. Bronchoscopy with lavage is
  done at that visit; the results (A5) find no opportunistic organism and normal
  immunity, so the concern is set aside. It never enters the problem list as a
  diagnosis.
- **R6.** Encounter 1's social history is a long travel narrative. The chart keeps
  only what a clinician records as exposure history: raised in upstate New York, family
  property in Montana, frequent outdoor activity (snowmobiling, guided ice climbing, horseback riding,
  canoeing, fishing, ATV camping trips). No named lakes or rivers.
- **Temperature.** 37.8 °C is carried as 100.0 °F (the source's "100°F").
- **Left as the source has it:** no sputum culture or organism-specific testing at the
  practice for the first two episodes; no follow-up chest x-ray after them; a second
  macrolide (erythromycin) chosen in June without comment on the first course; no
  pneumococcal vaccination raised. No note comments on any of these.

## Additions (A): physician to confirm

- **A1.** New-patient preventive visit 2024-09-04 (seed: about two years with the
  practice). Healthy, no medications, NKDA. Height 166 cm, weight 61 kg.
- **A2.** Azithromycin 500 mg on day 1, then 250 mg daily on days 2 to 5, started
  2025-03-21: the source's plan stops before a treatment, and encounter 1 treats the
  episode as a completed atypical pneumonia.
- **A3.** Erythromycin dose: 500 mg by mouth four times daily for 7 days (the source
  names the drug only).
- **A4.** The 2025-07-08 follow-up visit that refers her to pulmonology for recurrent
  pneumonia (the source sends her to pulmonology without a referral).
- **A5.** Pulmonology work-up at the 2025-12-29 visit, giving the source's
  "diagnostic sampling" its findings: bronchoscopy with left upper lobe lavage, airways
  normal without endobronchial lesion; lavage bacterial culture with normal
  oropharyngeal flora only; fungal culture no growth; acid-fast smear negative and
  culture no growth at 6 weeks; PJP stain negative; respiratory viral PCR panel
  negative; lavage cytology without malignant cells. Blood: CBC with differential
  normal; IgG 1040 mg/dL, IgA 210 mg/dL, IgM 115 mg/dL, all normal. Empiric
  doxycycline 100 mg twice daily for 7 days after the procedure.
- **A6.** Pulmonology follow-up 2026-02-23 with a clear repeat chest x-ray (2 views),
  closing the work-up.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may
  carry the descriptors its kind always carries, but nothing that adds a territory, a
  severity grade or a symptom, or changes the diagnosis or plan.
- Baseline labs at joining: CBC, comprehensive metabolic panel, lipid panel, all normal;
  HIV and hepatitis C screens nonreactive. PHQ-2 screens negative.
- Records from university student health (received 2024): childhood and college
  vaccines (MMR, varicella, HPV series, MenACWY, Tdap 2017), cervical cytology 2023
  normal. Cervical screening next due 2026.
- Yearly preventive visit (2025-09-10). Influenza and COVID vaccines each fall.
- Portal messages, phone calls, records requests, a form for a summer field course, a
  cancelled nurse visit, scheduling.
- Booked visit after 2026-08-27: annual physical 2026-09-14.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.


## Last seen

The practice last saw MRN00001925 on 2025-10-14. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. A visit is booked after today.
