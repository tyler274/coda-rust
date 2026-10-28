# Build requirements: unit-2309 (Charles Fenton, MRN00002309)

Source: synthetic_hospital patient 2309, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz text ("A peripheral smear is obtained
and shown below").

## Who he is (S)

Charles Fenton, born 1959-07-22, male, White, not Hispanic or Latino, English,
Greenfield 01301, Medicare. Retired construction foreman. Current cigarette smoker,
50 pack-years (R4). Alcohol socially, 2 to 3 drinks a week. No known drug allergies.
Divorced, lives alone (A0, demographic field only; no other person in the chart).
Family history: father colon cancer diagnosed at 68, deceased; mother breast cancer
diagnosed at 72; an older brother with lung cancer. Height 178 cm.

Joined the practice on 2021-09-14 (A1) at 62, from a prior primary-care office whose
records arrive a few weeks later. He brings: tobacco use (nicotine dependence,
cigarettes) and no medications. No prior colonoscopy. Weight 82 kg at joining.

None of the source's cancers, the Lambert-Eaton syndrome, the constipation or the
erectile dysfunction exists at joining; each starts at the contact below (R1).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2021-09-14 | practice | new patient visit (A1); history as above; normal exam; colonoscopy advised because of his father's colon cancer, he prefers a stool test first (P); FIT ordered; low-dose CT lung screening ordered (A2); Tdap; CBC, CMP, lipids, hepatitis C and HIV screening, all normal (P) |
| 2021-09-28 | practice | FIT negative (P) |
| 2021-10-05 | outside | low-dose CT lung screening: Lung-RADS 1, no nodules (A2) |
| 2021-10-19 | practice | nurse visit: influenza and COVID booster (P) |
| 2022-03-08, 2022-05-24 | practice | nurse visits: shingles recombinant doses 1 and 2 (P) |
| 2022-09-20 | practice | annual preventive visit; still smoking, not ready to quit; colonoscopy advised again, FIT instead; labs normal (P) |
| 2022-10-04 | practice | FIT negative; influenza and bivalent COVID at a nurse visit (P) |
| 2022-10-18 | outside | low-dose CT lung screening: Lung-RADS 1 (A2) |
| 2023-06-13 | practice | annual preventive visit; agrees to colonoscopy; **gastroenterology referral** (A3); weight 81.6 kg |
| 2023-08-10 | outside | screening colonoscopy (high-risk indication, father): **4 cm sessile lesion in the ascending colon**, biopsied. Pathology: invasive adenocarcinoma with marked atypia, nuclear pleomorphism and increased mitoses, invading the submucosa, arising in a tubulovillous adenoma. **Adenocarcinoma of the ascending colon diagnosed** |
| 2023-08-21 | outside | staging CT chest, abdomen and pelvis with contrast: no distant metastases (S enc 0 imaging) |
| 2023-08-24 | outside | gastroenterology consult letter (enc 0): occasional abdominal discomfort, no weight loss, bleeding or change in bowel habits; reviews the colonoscopy, pathology and CT; refers to oncology and colorectal surgery |
| 2023-08-29 | practice | telephone: PCP reviews the GI letter with him; oncology referral confirmed (P) |
| 2023-10-10 | practice | nurse visit: influenza and updated COVID (P) |
| 2023-11-14 | practice | office visit cancelled by patient (P) |
| 2024-02-23 | outside | oncology consult letter (enc 1): 3 months of progressive fatigue and weight loss, unintentional 7 kg (15 lb) loss, poor appetite, dull abdominal discomfort, occasional constipation, no blood in stool or melena. T 37.0 °C, BP 118/70, pulse 92, RR 16; weight 75 kg. Thin and tired, no acute distress; abdomen soft, mildly distended, diffusely tender, no rebound. **Iron deficiency anemia**: Hb 10.6, MCV 74, ferritin 8 (A4 numbers). CT abdomen and pelvis with contrast: irregular ascending colon mass with spread to regional lymph nodes (no distant disease). The biopsy is re-reviewed: malignant epithelial cells invading through the basement membrane into surrounding tissue. Plan: right hemicolectomy, then adjuvant chemotherapy (A5) |
| 2024-03-01 | practice | pre-operative visit; weight 75 kg; **ferrous sulfate 325 mg every other day started** (A4); CBC, CMP |
| 2024-03-12 to 2024-03-16 | outside | **right hemicolectomy with ileocolic anastomosis** (A5); pathology: adenocarcinoma of the ascending colon, pT1 N1a (1 of 16 lymph nodes), margins negative. Discharge summary reaches the practice late (2024-04-16) |
| 2024-03-26 | practice | post-operative follow-up: healing incision, eating, bowels moving; weight 73.5 kg (P) |
| 2024-04-02 | outside | **port-a-cath placement** (A6), right chest; procedure note |
| 2024-04-09 to 2024-09-17 | outside | **adjuvant FOLFOX, 12 cycles every 2 weeks** through the port (A6); a mid-course oncology letter (2024-06-04) says he is tolerating it; the end-of-treatment letter never reaches the practice |
| 2024-06-18 | practice | constipation on chemotherapy, hard stools every 3 to 4 days; **chronic constipation diagnosed; polyethylene glycol 3350 17 g daily started** (A7); weight 73.0 kg |
| 2024-09-24 | practice | **last practice visit**: finished FOLFOX last week; constipation better but not resolved: **senna 8.6 mg twice daily as needed added** (A7); Hb 12.9, ferritin 38: **ferrous sulfate stopped, iron deficiency anemia resolved** (A4); weight 74.2 kg; high-dose influenza and PCV20 given; still smoking |
| 2025-02-24 | outside | neurology consult (enc 2): 3 months of progressive muscle weakness, fatigue, dry mouth and trouble climbing stairs; recent erectile dysfunction and constipation. Exam: proximal weakness, absent deep tendon reflexes, normal sensation, strength improves with repeated effort. Chest x-ray: right hilar mass. Same-day EMG/NCS (A8): low motor amplitudes with more than 100% increment after 10 seconds of exercise. **Lambert-Eaton myasthenic syndrome, presumed paraneoplastic**; **amifampridine 15 mg three times daily and pyridostigmine 60 mg three times daily started**; **erectile dysfunction: sildenafil 25 mg as needed started** (A9); CT chest ordered, oncology informed. P/Q-type VGCC antibody drawn, resulted 2025-03-04 at 0.42 nmol/L (positive; A8), in an addendum |
| 2025-03-06 | outside | CT chest with contrast (A10): 4.6 cm right hilar mass, enlarged subcarinal lymph node, 2.8 cm right adrenal nodule; no other lung lesions |
| 2025-03-12 | practice | portal message from patient telling the practice of the new diagnosis, the two new pills and the lung spot (records texture); neurology letter arrives 2025-04-02 |
| 2025-03-13 | outside | EBUS needle aspiration of the subcarinal node (A10): small cell carcinoma |
| 2025-03-20 | outside | oncology (A10): **small cell lung cancer, extensive stage (right adrenal metastasis)**; MRI brain negative for metastases; carboplatin, etoposide and atezolizumab for 4 cycles, then atezolizumab maintenance. This letter never reaches the practice; the patient tells the practice by phone on 2025-04-08 |
| 2025-08-27 | outside | oncology follow-up (enc 3): metastatic small cell lung cancer, on atezolizumab maintenance after 4 cycles; 50 pack-years (R4); family history as above. **Oxycodone 5 mg every 6 hours as needed started** for aching right chest pain (A11). Letter arrives late, 2025-11-04 |
| 2026-01-08 | outside | restaging CT abdomen and pelvis with contrast (A12): new 3.4 cm mass in the pancreatic body, several new liver lesions |
| 2026-01-20 | outside | EUS-guided fine needle aspiration of the pancreatic mass (A12): adenocarcinoma. **Metastatic pancreatic adenocarcinoma (liver)**; chemotherapy not yet started |
| 2026-02-12 to 2026-02-19 | outside | admission to the regional hospital's oncology service (enc 4, R5): progressive weakness, spontaneous bruising, intermittent confusion; significant weight loss over months; no chest pain, trauma or recent infection; no personal history of bleeding disorders. On 2026-02-13: oozing from IV sites and new dark patches on the lower legs over the past day; temperature normal, BP 118/70, HR 96; alert but slow to respond; petechiae, scattered ecchymoses, bleeding from a venipuncture site; nonfocal neurological exam. **Disseminated intravascular coagulation**: platelets 41, PT 21.6 s, INR 1.9, aPTT 47 s, fibrinogen 88 mg/dL, D-dimer 18.4 µg/mL FEU, Hb 8.6; smear with schistocytes and few platelets (A13). CT head without contrast: no hemorrhage (A13). Platelets, cryoprecipitate and plasma given; atezolizumab held. At discharge: platelets 72, fibrinogen 162, Hb 9.1; oozing stopped; oncology to discuss pancreatic treatment. Discharge summary reaches the practice 2026-03-09 |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The profile and every note carry all six chronic conditions and all six home
  drugs from the first encounter, including a hemicolectomy and a port before the
  colon cancer is found. Resolution: each condition starts at the contact above and each
  drug at the contact that prescribes it. The GI letter (2023) lists no medications and
  none of the later cancers; outside letters list only what has started by then.
- **R2.** The GI note says "no family history of colorectal disease"; every other note
  says his father had colon cancer at 68. Resolution: the father's colon cancer stands;
  the GI letter records it.
- **R3.** "Right-sided colon adenocarcinoma" and "right-sided colon cancer" are one
  tumor (adenocarcinoma of the ascending colon); "Lambert-Eaton myasthenic syndrome" and
  its paraneoplastic form are one condition. The source's two biopsy descriptions are the
  colonoscopy biopsy (2023-08) and the oncology review of it (2024-02). The source's
  "imaging shows no distant metastases" (enc 0) and "spread to regional lymph nodes"
  (enc 1) are both true: regional nodes are not distant disease.
- **R4.** Pack-years 50 and 40. Resolution: 50 throughout; 40 is not carried.
- **R5.** Enc 4 reads as an inpatient day where nursing reports overnight oozing.
  Resolution: admitted 2026-02-12, DIC recognized on the 2026-02-13 hospital day (the
  source date). The "smear shown below" becomes a smear report (A13).
- **R6.** Enc 3 "diagnosed with metastatic lung cancer" in August 2025 while a hilar
  mass was seen in February. Resolution: the mass is worked up in March 2025 (A10) and
  enc 3 is the oncology follow-up whose assessment carries metastatic small cell lung
  cancer.
- **R7.** Enc 2's "recent episodes" of erectile dysfunction and constipation: the
  constipation is the chronic constipation known since 2024 (A7); the erectile
  dysfunction is new in 2025.
- **Left as the source has it:** the source places six months between the colon cancer
  diagnosis (August 2023) and the oncology visit where the mass is still in place
  (February 2024); no surgery happens in between and no record explains the interval.
  No surveillance imaging reaches the practice between the hemicolectomy and the
  February 2025 chest x-ray. No smoking-cessation drug is ever started; he is counseled
  and declines. No AAA ultrasound at 65.

## Additions (A): physician to confirm

- **A0.** Marital status divorced, living alone: the ledger needs a value; it adds no
  person or event.
- **A1.** New-patient visit 2021-09-14 (seed: about 5 years with the practice).
- **A2.** Annual low-dose CT lung screening in 2021 and 2022 (50 pack-years, current
  smoker), Lung-RADS 1 both times; the 2023 staging CT takes the place of the 2023 scan.
- **A3.** The 2023-06-13 visit that refers him for the colonoscopy the source calls
  "screening".
- **A4.** Numbers for the source's iron deficiency anemia (Hb 10.6, MCV 74, ferritin 8)
  and its treatment: ferrous sulfate 325 mg every other day from 2024-03-01, stopped
  2024-09-24 at Hb 12.9, ferritin 38.
- **A5.** Right hemicolectomy with ileocolic anastomosis (source surgical history) on
  2024-03-12, after the oncology visit where the mass is still present; pT1 N1a, 1 of
  16 nodes, margins negative.
- **A6.** Port placement (source surgical history) on 2024-04-02 for adjuvant FOLFOX,
  12 cycles (standard care for node-positive colon cancer), 2024-04-09 to 2024-09-17.
- **A7.** Chronic constipation (source condition) diagnosed 2024-06-18 on
  chemotherapy; polyethylene glycol 3350 17 g daily then, senna 8.6 mg twice daily as
  needed on 2024-09-24 (source drugs).
- **A8.** Evidence for the source's LEMS: same-day EMG/NCS with low motor amplitudes and
  more than 100% post-exercise increment; P/Q-type VGCC antibody 0.42 nmol/L.
- **A9.** Amifampridine, pyridostigmine (source doses) and sildenafil 25 mg as needed
  start at the neurology visit, where the ED and LEMS are first documented.
- **A10.** Workup of the hilar mass: CT chest 2025-03-06 (4.6 cm right hilar mass,
  subcarinal node, 2.8 cm right adrenal nodule), EBUS aspiration 2025-03-13 (small cell
  carcinoma), oncology 2025-03-20 (extensive stage, MRI brain negative, carboplatin,
  etoposide and atezolizumab x4 then atezolizumab maintenance).
- **A11.** Oxycodone 5 mg every 6 hours as needed (source drug) starts at the 2025-08-27
  oncology visit for aching right chest pain.
- **A12.** Finding the source's pancreatic cancer: restaging CT 2026-01-08 (3.4 cm
  pancreatic body mass, several liver lesions), EUS-FNA 2026-01-20 (adenocarcinoma).
- **A13.** Evidence and care for the source's DIC: the coagulation numbers and smear
  above, CT head negative, transfusion support, atezolizumab held, discharge numbers.

## Records (seed: gaps)

The practice receives: prior practice records (2021-10), both lung screening reports,
the colonoscopy and pathology reports, the 2023 CT report, the GI letter, the
February 2024 oncology letter (with its CT report), the hemicolectomy discharge summary
(late), the port procedure note, the June 2024 oncology letter, the neurology letter
with the EMG report and antibody addendum (2025-04-02), the August 2025 oncology letter
(2025-11-04) and the February 2026 discharge summary (2026-03-09), which also
summarizes the January CT and pancreatic biopsy. It never receives the FOLFOX
end-of-treatment letter, the March 2025 CT, EBUS or oncology letter; it hears of the
lung cancer from the patient. After 2024-09-24 nothing is reconciled: LEMS, the lung
and pancreatic cancers, DIC and the drugs started by neurology and oncology appear in
the letters, not on the practice's lists.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may
  carry the descriptors its kind always carries, but nothing that adds a territory, a
  severity grade or a symptom, or changes the diagnosis or plan.
- Prior practice records may mention routine labs there, normal.
- Preventive visits yearly through 2023; FIT 2021 and 2022 negative; Tdap 2021;
  influenza each fall 2021 to 2024; COVID boosters 2021 to 2023; shingles series 2022;
  PCV20 in 2024; hepatitis C and HIV screening once; PHQ-2 negative; AUDIT-C low;
  smoking counseling at visits, not ready to quit.
- CBC and CMP at visits from 2024; lipids normal.
- Refills, pharmacy traffic, portal messages about results, refills and scheduling, a
  cancelled visit. After 2024-09-24: only messages, refills of polyethylene glycol and
  senna, and outside records filed as received.
- No visit booked after 2026-08-27.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002309 on 2024-09-24. Practice contacts after that day are
only messages, refills and records; outside care after it still happens, and its records
are filed as received without changing the practice's lists. No visit is booked.
