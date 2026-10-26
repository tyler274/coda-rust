# Build requirements: unit-2866 (Judith Zielinski, MRN00002866)

Source: synthetic_hospital patient 2866, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its fragmentary prose.

## Who she is (S)

Born 1957-04-09, female, White, not Hispanic or Latino, English, Medicare with
supplemental Plan G. Retired elementary school teacher. Former smoker, 20 pack-years,
quit about 12 years before the 2024 ED visit (so about 2012). Wine 1 to 2 glasses a
week. No known drug allergies until heparin-induced thrombocytopenia in September
2024 (R3). Surgical history: hysterectomy for uterine fibroids 1998, appendectomy
1975, left total knee arthroplasty 2024-09-16 (R1). Family history: father atrial
fibrillation and hypertension, died at 81; mother type 2 diabetes, stroke at 76;
maternal uncle pulmonary embolism after hip surgery; sister hypothyroidism.

Joined the practice in 2021 (A1: establish-care visit 2021-08-24). She brought chronic
atrial fibrillation (rate control and warfarin), hypertension, hyperlipidemia and
osteoarthritis of the knees, left worse, on:
warfarin 5 mg daily, metoprolol succinate 100 mg daily, lisinopril 10 mg daily,
rosuvastatin 20 mg daily, acetaminophen 650 mg every 6 hours as needed for pain.
The practice manages her warfarin (INR goal 2.0 to 3.0).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2021-08-24 | practice | establish-care visit (A1): history as above; AF rate controlled, irregularly irregular; BP at goal on lisinopril and metoprolol; INR, CMP, CBC, lipids drawn. Height 163 cm, weight about 74 kg |
| 2024-03-19 | practice | left knee pain worse over the winter, limiting walking despite acetaminophen; knee X-ray shows tricompartmental osteoarthritis, left worse than right; orthopedics referral (A2) |
| 2024-05-14 | outside | orthopedic consult letter: end-stage left knee osteoarthritis, failed conservative care; plans left total knee arthroplasty (A2) |
| 2024-08-27 | practice | pre-operative evaluation (A3): AF rate controlled; ECG atrial fibrillation, rate in the 70s, no acute change; **preoperative platelet count 250,000/µL** (S), CBC, BMP, INR. Warfarin to be held 5 days before surgery without bridging, restart per surgeon |
| 2024-09-16 | outside | **left total knee arthroplasty** at a regional hospital (R1). Warfarin held perioperatively; **subcutaneous heparin started for VTE prophylaxis** the day of surgery. No operative note ever reaches the practice; the practice learns of the surgery from the patient |
| 2024-09-22 | outside | ED (S): 2 days of left calf pain and swelling, no trauma or prolonged immobilization beyond the surgery; vitals stable; left calf erythematous, warm, tender, circumference 3 cm greater than right. Hemoglobin 13.2 g/dL, **platelets 55,000/µL** (from 250,000 preoperatively), WBC 7.8, PT 12 s, aPTT 28 s, D-dimer elevated (R4). Venous duplex: **deep vein thrombosis of the left popliteal vein**. **Heparin stopped. Heparin-induced thrombocytopenia type II diagnosed**; admitted (A4) |
| 2024-09-22 to 2024-10-02 | outside | admission (A4): **PF4-heparin antibody positive** (optical density 1.84); **argatroban infusion** (inpatient only); platelets recover to 171,000/µL by 2024-09-27; **warfarin 5 mg daily restarted** 2024-09-27 with argatroban overlap, argatroban stopped once INR therapeutic; discharged 2024-10-02 with INR 2.4 and platelets 212,000/µL. **Heparin recorded as an allergy (HIT)**. Discharge summary reaches the practice late, on 2024-10-21, after a records request |
| 2024-10-07 | practice | post-hospital visit (A4): calf swelling improving; practice adds heparin allergy and HIT and left popliteal DVT to its lists (the practice list lagged until this visit); weekly INR then every 2 to 4 weeks; CBC with platelets 236,000/µL |
| 2024-11-05 | outside | orthopedic 6-week post-op letter (A5): incision healed, range of motion 0 to 110 degrees, walking with a cane, outpatient physical therapy continuing |
| 2025-01-14 | practice | 3-month DVT follow-up (A6): calf symptoms gone; DVT marked resolved; warfarin continues indefinitely for atrial fibrillation |
| 2025-08-19 | practice | INR check missed (no-show); rescheduled check not done before the ED visit |
| 2025-09-23 | outside | ED (S): abrupt sharp right-sided chest pain worse with deep breaths, productive cough with blood-tinged sputum and shortness of breath for 2 hours; about one year after the left calf episode. HR 110, RR 22, BP 135/85, SpO2 92% on room air; decreased breath sounds at the right lung base; no leg swelling or redness, tenderness of the right calf on deep palpation. D-dimer elevated (R4). **INR 1.6** (A7). CT pulmonary angiogram: filling defects in right lower lobe segmental pulmonary arteries with a wedge-shaped peripheral opacity in the right lower lobe consistent with infarction; no right heart strain (A7). **Pulmonary embolism**; admitted (A7) |
| 2025-09-23 to 2025-09-27 | outside | admission (A7): heparin avoided for HIT history; **fondaparinux 7.5 mg subcutaneously daily** (inpatient and until INR therapeutic) overlapping warfarin 5 mg daily; discharged 2025-09-27 on warfarin alone, INR 2.3, SpO2 95% on room air. The practice first hears of it from the patient's portal message on 2025-09-29; the discharge summary, ED record and CT report arrive 2025-10-20 after a records request |
| 2025-10-01 | practice | post-hospital visit (A7): pleuritic pain easing, no further blood in sputum, SpO2 97% on room air; INR 2.5; pulmonary embolism added; INR every 2 weeks for 3 months, then monthly |
| 2026-01-13 | practice | 3-month PE follow-up (A7): breathing back to baseline, no chest pain; warfarin continues indefinitely; pulmonary embolism moved to history |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source names the operation two ways: total knee arthroplasty (left) in the
  profile and encounter 0, and "status post hip replacement surgery" in its
  comorbidities. Resolution: one operation, a left total knee arthroplasty on
  2024-09-16. The hip replacement is not carried (the maternal uncle's PE followed hip
  surgery; that stays family history only). The source says heparin began "7 days ago"
  at the Sunday ED visit; surgery falls on the Monday before, and the chart says about
  a week.
- **R2.** Encounter 1 says she was hospitalized for a total knee replacement "four weeks
  ago", while its own HPI and the timeline put the knee surgery a year earlier.
  Resolution: the "four weeks ago" hospitalization is not carried. There was one
  operation, in September 2024.
- **R3.** The source keeps "NKDA" after HIT. Resolution: heparin is recorded as an
  allergy (reaction: heparin-induced thrombocytopenia, high criticality) by the
  hospital in September 2024 and by the practice on 2024-10-07. Before then, no known
  drug allergies.
- **R4.** D-dimer is given only as "elevated". Resolution: each ED report carries a
  numeric value above the reference range: 2,840 ng/mL FEU (2024-09-22) and 3,410
  ng/mL FEU (2025-09-23).
- **R5.** Encounter 1 copies warfarin 5 mg daily "(held perioperatively)" a year after
  the operation. Resolution: warfarin was restarted in the 2024 admission and taken
  daily since; the parenthetical is not carried.
- **R6.** The source lists HIT on the active problem list in 2025 with a stamp. The chart
  keeps HIT on the problem list from 2024-09-22 onward, without a stamp.
- **Left as the source has it:** no right leg venous duplex is done for the right calf
  tenderness in 2025, and no hypercoagulability workup or anticoagulant change follows
  the second clot; warfarin stays. No note comments on either.

## Additions (A): physician to confirm

- **A1.** 2021-08-24 establish-care visit (seed: five years with the practice).
- **A2.** The 2024 knee visit with X-ray and orthopedic referral, and the orthopedic
  consult that planned the operation the source gives.
- **A3.** The 2024-08-27 pre-operative evaluation where the source's preoperative
  platelet count of 250,000 was drawn, with ECG.
- **A4.** The 2024 HIT admission, the treatment any physician would give: PF4-heparin
  antibody positive (OD 1.84), argatroban, platelet recovery (171,000 on 2024-09-27,
  212,000 at discharge, 236,000 on 2024-10-07), warfarin restarted with overlap,
  discharge INR 2.4. Then the post-hospital practice visit.
- **A5.** The 6-week orthopedic post-op letter (usual course after TKA).
- **A6.** The 3-month DVT follow-up; DVT resolved.
- **A7.** The 2025 PE's usual course: INR 1.6 at the ED (the reason for a clot on
  warfarin, after a missed check), CTPA filling defects as above, admission with
  fondaparinux bridge (heparin avoided), discharge INR 2.3, post-hospital visit,
  3-month follow-up; PE moved to history.
- **A8.** Low-dose CT lung cancer screening yearly from 2022 (20 pack-years, quit
  2012): Lung-RADS 1 each year (2022-03, 2023-03, 2024-04, 2026-04; the 2025 screen is
  deferred after the September CT and not done before today).

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention
  (heart sounds apart from the irregular rhythm, no murmur, clear lungs outside the PE
  episode, normal deep veins elsewhere on the duplex). An abnormal finding appears only as
  the source or this file gives it. A report may carry the descriptors its kind always
  carries, but nothing that adds a territory, a severity grade or a symptom, or changes the
  diagnosis or plan.
- Records from her prior practice may mention routine screening done there with normal
  results: colonoscopy 2019 without polyps, Shingrix series 2019, Tdap 2017.
- Visits for atrial fibrillation, hypertension and lipids every 4 to 6 months; yearly
  Medicare annual wellness visits with PHQ-2; CMP, CBC and lipids yearly; INR checks
  every 4 to 6 weeks when stable (more often around the 2024 and 2025 admissions), with
  ordinary warfarin texture (an INR of about 3.2 or 1.8 now and then; the standing dose
  stays 5 mg daily, and a single held dose or one extra half tablet phoned in for an
  out-of-range INR is allowed). Blood pressure at goal on her drugs, LDL around 80 on rosuvastatin.
- Prevention: screening mammogram 2022, 2024, 2026 normal; DXA 2022 normal; hepatitis C
  screening once, negative; influenza high-dose each fall; COVID vaccines; PCV20 once;
  RSV vaccine once.
- Refills, pharmacy traffic, portal messages about results and scheduling, a cancelled
  visit, records requests.
- Booked visit after 2026-08-27: annual wellness visit 2026-09-10.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.


## Last seen

The practice last saw MRN00002866 on 2026-05-12. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. A visit is booked after today.
