# unit-2782 build requirements

Every clinical fact in this chart traces to **S** (the source as carried here), **R**
(a numbered resolution), **A** (a numbered addition) or **P** (the primary-care layer),
and nothing else. Texture (voices, scheduling, refills, portal traffic, admin) is free
and carries no new clinical fact. Re-voice everything: no source sentence, clinician
name, problem-list dump, "(diagnosed ...)" stamp or quiz text reaches the chart.

## Who she is (S)

- Maureen Dunn, MRN00002782, female, born 1983-06-12, Northampton MA 01060. White, not
  Hispanic or Latino, English, Private PPO. Phone +1-347-555-0119,
  maureen.dunn@example.test.
- Elementary school teacher. Never smoker. Alcohol: social, 1-2 drinks per week. No
  history of radiation exposure. No recent travel or environmental toxin exposure.
- Allergies: seasonal pollen and dust mites (environmental, cause her rhinitis). No drug
  allergies (R9).
- Surgical history: none before 2023. Total thyroidectomy and radioactive iodine
  ablation follow on the timeline (R4).
- Family history: maternal aunt with papillary thyroid carcinoma; mother with
  hypothyroidism; father with hypertension (R5).
- Joins the practice 2021-09-14 (A1), age 38. Brings: **Seasonal allergic rhinitis**
  (pollen and dust mites) and **loratadine 10 mg by mouth daily as needed** for seasonal
  allergies (over the counter). No other problems or drugs.
- Height 165 cm, weight about 65 kg throughout; BMI 24.

## Timeline

| Date | Where | What happens |
|---|---|---|
| 2021-09-14 (Tue) | Practice, new patient | Establish care (A1). Seasonal allergic rhinitis, loratadine PRN, allergies, family and social history as above. Baseline labs and one-time screens (P). |
| 2021-09-20 | Outside records | Prior primary care records received (P): routine screening normal there. |
| 2023-07-26 (Wed) | Practice office visit (source enc 0) | Age 40. Noticed a lump on the front of her neck about two weeks ago, looking in the mirror. No pain, dysphagia or voice change; feels well, no weight change, fatigue or mood change; menses regular. Vitals normal. Thyroid not enlarged overall; **2-cm firm, non-tender nodule in the right lobe**; no cervical lymphadenopathy. No radiation exposure. Orders thyroid ultrasound and TSH with free T4 (R2); both thyroid tests normal. |
| 2023-07-31 (Mon) | Outside radiology | Thyroid ultrasound (A2): **solid hypoechoic right-lobe nodule with irregular borders**, 1.9 x 1.5 x 1.4 cm, no echogenic foci, wider than tall; ACR TI-RADS TR4 (6 points); FNA recommended. Left lobe normal. No abnormal cervical lymph nodes. |
| 2023-08-08 (Tue) | Outside radiology | Ultrasound-guided FNA of the right-lobe nodule (A3). |
| 2023-08-11 (Fri) | Outside pathology, received | FNA cytology (A3, R1): branching papillary fragments with fibrovascular cores and the nuclear features of papillary carcinoma; Bethesda VI. **Malignant neoplasm of thyroid gland (papillary thyroid carcinoma) diagnosed.** PCP calls her. |
| 2023-08-18 (Fri) | Practice office visit (source enc 1) | Returns about the neck swelling first noticed about a month ago. No pain, voice change, breathing difficulty or trouble swallowing solids or liquids; well, stable weight, normal appetite, no recent illness. Seasonal allergies on OTC antihistamine. T 36.7 C (98.1 F), pulse 74, BP 118/76, BMI 24. **Firm, non-tender, mobile right-lobe nodule about 1.8 cm**, no cervical lymphadenopathy; rest of exam normal. Thyroid function normal. Reviews cytology. **Referral to otolaryngology (head and neck surgery)** (A4). |
| 2023-09-06 (Wed) | Outside otolaryngology consult | (A4) Flexible laryngoscopy: both vocal cords mobile. Plan total thyroidectomy. |
| 2023-10-11 to 2023-10-12 | Outside hospital, overnight | **Total thyroidectomy** (A5, R4). Post-op calcium and intact PTH normal. Discharged on **levothyroxine 125 mcg by mouth daily** (started by otolaryngology, R3). |
| 2023-10-19 | Outside pathology, received | Surgical pathology (A5): papillary thyroid carcinoma, classic type, right lobe, unifocal, 1.8 cm; margins negative; no extrathyroidal extension; no lymphovascular invasion; 2 central neck lymph nodes negative (0/2); left lobe benign thyroid tissue. pT1b pN0. |
| 2023-10-25 (Wed) | Outside otolaryngology | Post-op visit (A6): incision healing, voice normal, calcium normal. Referral to nuclear medicine for radioactive iodine. |
| 2023-12-04 to 2023-12-13 | Outside nuclear medicine | **Radioactive iodine ablation** (A7, R4): recombinant TSH 12/04 and 12/05 on low-iodine diet, I-131 30 mCi on 12/06; post-therapy whole-body scan 12/13 with uptake in the thyroid bed only, no uptake elsewhere. Levothyroxine continued throughout. |
| 2024-07-10 (Wed) | Outside otolaryngology | Surveillance (A8): neck ultrasound with no residual or recurrent disease; thyroglobulin <1 ng/mL; thyroglobulin antibody negative; TSH normal. |
| 2025-07-09 (Wed) | Outside otolaryngology | Surveillance (A9, R10): **thyroglobulin <1 ng/mL**; TSH normal. |
| 2026-01-07 (Wed) | Outside otolaryngology (source enc 2) | Age 42. Routine follow-up after thyroid cancer treatment; no symptoms. Total thyroidectomy and RAI about 2 years ago. On levothyroxine (R8) and loratadine PRN. Exam unremarkable. **Thyroglobulin 10 ng/mL** (up from <1 ng/mL six months earlier); TSH normal; free T4 normal; calcium normal. Thyroglobulin antibody negative (A10). Plan: neck ultrasound and repeat thyroglobulin in 3 months (A10). |
| 2026-01-21 (Wed) | Outside radiology | Neck ultrasound (A10): post-thyroidectomy bed without mass; no suspicious cervical lymph nodes. |
| 2026-04-08 (Wed) | Outside otolaryngology | Repeat (A10): **thyroglobulin 9.8 ng/mL**; thyroglobulin antibody negative; TSH normal. Plan: repeat thyroglobulin and neck ultrasound in October 2026. Open at today. |

## Resolutions (R)

1. **Tumor type.** Encounter 0's cytology (Congo red staining with apple-green
   birefringence, i.e. amyloid) points to medullary carcinoma; encounter 1's cytology
   (branching structures with fibrovascular cores), the radioactive iodine, the
   thyroglobulin surveillance, the encounter 2 history and the maternal aunt all say
   papillary. The chart carries **papillary thyroid carcinoma**. The amyloid finding is
   not carried, and nobody orders calcitonin.
2. **What an office visit can do.** Encounter 0 reports the ultrasound and FNA as if done
   at the visit. The visit examines her and orders the ultrasound and thyroid tests; the
   ultrasound (2023-07-31) and ultrasound-guided FNA (2023-08-08) happen at outside
   radiology, and the cytology is received 2023-08-11. Encounter 1's FNA statement is the
   same aspirate, reviewed at that visit. The diagnosis is recorded when the cytology is
   received, not at encounter 0 as the source's stamp says. Encounter 0 records the
   finding as a thyroid nodule, which closes when the cytology names it.
3. **Levothyroxine before surgery.** Encounters 0 and 1 list levothyroxine 125 mcg daily
   with normal thyroid tests and an intact gland. Before surgery she takes no thyroid
   medicine. Levothyroxine 125 mcg daily starts at discharge from the thyroidectomy and
   continues unchanged to today.
4. **Treatment timing.** "Total thyroidectomy 2 years after initial encounter" conflicts
   with encounter 2's "2 years ago" and with the usual pace of care. Thyroidectomy
   2023-10-11 and radioactive iodine 2023-12-06; at encounter 2 both were about two
   years earlier. The profile's stamp is not carried.
5. **Family history.** Encounter 0 says no family thyroid cancer or endocrine disease and
   encounter 1 calls it unremarkable; encounter 2 lists the maternal aunt's papillary
   thyroid carcinoma, the mother's hypothyroidism and the father's hypertension. The
   chart carries encounter 2's list from establishing care onward.
6. **Nodule size.** 2 cm at encounter 0 and about 1.8 cm at encounter 1 are two
   examiners' palpation estimates; both stay as written.
7. **Onset wording.** Encounter 0 has her noticing the lump in the mirror two weeks
   earlier; encounter 1 says swelling noticed several weeks ago, adjusting her necklace,
   first observed one month ago. The chart: first noticed in the mirror around
   2023-07-12; at 2023-08-18 that is about a month. The necklace detail is not carried.
8. **Encounter 2 medication list** omits levothyroxine while stating she takes it. She
   takes levothyroxine 125 mcg daily and loratadine PRN.
9. **Allergies.** The source lists only seasonal pollen and dust mites. The chart records
   those as environmental allergies (rhinitis) and no known drug allergies.
10. **"Six months ago."** The prior thyroglobulin of <1 ng/mL is the 2025-07-09
    otolaryngology surveillance draw.
11. **Year in the source text.** "Initially identified in January 2020" becomes July 2023.

**Left as the source has it:** no calcitonin or CEA, no genetic testing or counseling
for the family history, no endocrinology referral (otolaryngology follows the cancer),
no levothyroxine dose change, no allergy testing or allergist, no nasal steroid.

## Additions (A)

1. **Establishing care 2021-09-14**, a new-patient preventive visit, since the seed has
   her with the practice about five years.
2. **Thyroid ultrasound 2023-07-31** with the findings in the timeline, to carry the
   source's ultrasound description at a real imaging study.
3. **Ultrasound-guided FNA 2023-08-08 and cytology 2023-08-11** (Bethesda VI, papillary
   carcinoma), the evidence for the diagnosis.
4. **Otolaryngology referral at encounter 1 and consult 2023-09-06** (vocal cords
   mobile; plan total thyroidectomy): the source's surgery needs a surgeon and a
   referral.
5. **Total thyroidectomy 2023-10-11 (overnight stay) and surgical pathology** as in the
   timeline; post-op calcium and intact PTH normal; levothyroxine 125 mcg daily started.
6. **Otolaryngology post-op visit 2023-10-25** and nuclear medicine referral.
7. **Radioactive iodine 2023-12-06** with recombinant TSH preparation, I-131 30 mCi, and
   the post-therapy scan as in the timeline.
8. **Otolaryngology surveillance 2024-07-10**: neck ultrasound without disease,
   thyroglobulin <1 ng/mL, thyroglobulin antibody negative, TSH normal.
9. **Otolaryngology surveillance 2025-07-09**, the visit that carries the source's
   earlier thyroglobulin <1 ng/mL (TSH normal).
10. **Next steps after the rising thyroglobulin**: thyroglobulin antibody negative on the
    2026-01-07 draw; neck ultrasound 2026-01-21 without mass or suspicious nodes;
    repeat 2026-04-08 thyroglobulin 9.8 ng/mL, antibody negative, TSH normal; plan to
    repeat thyroglobulin and neck ultrasound in October 2026. No recurrence is
    diagnosed.

## Primary-care layer (P)

- Visits: establish care 2021; annual preventive visits 2022-09, 2024-09 and 2025-09
  (a 2023-11 physical cancelled during her treatment and not rebooked that year); a
  primary-care follow-up 2024-01 after treatment; the two source visits. Booked annual
  physical 2026-09-23.
- Labs: CBC, CMP and lipid panel at establishing care and the physicals, all normal;
  TSH with the 2024 and 2025 physicals, normal on levothyroxine; hepatitis C antibody and
  HIV screen once at establishing care, negative.
- Prevention, all normal: prior practice records with a normal Pap in 2018 and Tdap in
  2017; Pap with HPV co-test 2022-09 (NILM, HPV negative); screening mammograms 2024-03
  and 2026-03 (BI-RADS 1); influenza vaccine each fall 2021-2025; COVID-19 primary series
  early 2021 (by prior record), booster 2021-11, updated vaccines fall 2022, 2023 and
  2024; PHQ-2 negative at visits. Colorectal screening not yet due.
- Seasonal allergic rhinitis: spring portal questions about loratadine, which she uses
  daily through pollen season; no new drugs.
- Levothyroxine refills from primary care after otolaryngology's first prescription;
  pharmacy traffic; result notes; portal messages about scheduling and the
  thyroglobulin results; outside-records requests.
- No acute illnesses, injuries, new diagnoses or life events beyond the timeline.


## Last seen

The practice last saw MRN00002782 on 2025-09-17. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. Nothing is booked after today.
