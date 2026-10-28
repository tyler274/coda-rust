# Build requirements: unit-1927 (Deborah Lipschitz, MRN00001927)

Source: synthetic_hospital patient 1927, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz voice ("you note").

## Who she is (S)

Born 1990-10-21, female, White, not Hispanic or Latino, Jewish ancestry of Eastern
European origin (the source's ethnicity; relevant to the genetics history), English,
commercial PPO, software engineer. Never smoker, no alcohol. No known drug allergies.
Nulligravida; menarche at 11. Surgical history: tonsillectomy at age 8 (about 1999),
wisdom teeth extraction at age 21 (about 2012). Fair skin, Fitzpatrick type II
(recorded in notes and the skin exams; not a coded problem).

Family history: mother with breast cancer at 42 and ovarian cancer at 51; maternal
aunt with breast cancer at 39; maternal grandmother with breast cancer at 44; father
with hypertension. Father's melanoma is first recorded at the 2026 dermatology visit
(R2).

Joined the practice in August 2023 (A1), aged 32, on ethinyl estradiol 0.035 mg /
norgestimate 0.25 mg one tablet daily (combined oral contraceptive) and cholecalciferol
2000 IU daily. Problems brought: family history of breast cancer and of ovarian
cancer. No BRCA testing yet. Height 165 cm, weight about 60 kg, stable throughout;
blood pressure normal throughout.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2023-08-21 | practice | new-patient preventive visit (A1). History as above; normal exam including breast exam. Genetic counseling raised for the maternal history; she prefers to wait (A2). Screening labs, HIV and hepatitis C screening negative, Tdap (P) |
| 2024-07-29 | practice | annual physical (P); normal exam; genetics again offered, still deferred (A2) |
| 2024-12-04 | practice | portal message: she has felt a lump in the right breast and has a gynecology visit booked (P) |
| 2024-12-09 | outside | gynecology visit (letter received 2024-12-12): firm mass upper outer quadrant right breast, found on self-exam, no symptoms. Exam: 2.5 cm nonmobile mass, no skin changes, no nipple discharge, no lymphadenopathy, rest of exam normal. Diagnostic mammogram the same day: suspicious irregular mass right breast upper outer quadrant, BI-RADS 4 (R5); targeted ultrasound: irregular hypoechoic mass at 10 o'clock, 2.3 x 1.8 x 1.6 cm, no abnormal axillary nodes. Core biopsy arranged. **Genetic testing sent** for personal and family history. Lump in right breast, upper outer quadrant on the practice list |
| 2024-12-13 | outside | ultrasound-guided core needle biopsy, right breast 10 o'clock, clip placed (A3). Pathology 2024-12-17: fibroadenoma, no atypia, no malignancy; radiology-pathology concordant; follow-up ultrasound in 6 months. Received 2024-12-19. **Fibroadenoma (benign neoplasm of right breast) replaces the lump** |
| 2025-01-06 | outside | genetic test result (multigene hereditary cancer panel, sent 2024-12-09), received via the gynecologist and a genetic counselor's letter 2025-01-09: **BRCA1 pathogenic variant, heterozygous**; no other pathogenic variants. **Hereditary breast and ovarian cancer syndrome (BRCA1 gene mutation positive) diagnosed** (R1) |
| 2025-01-21 | practice | visit to go over the result (P); referral to a high-risk breast program (oncology) (A4) |
| 2025-02-18 | outside | high-risk breast program consult (letter received 2025-02-24) (A4): breast MRI and mammogram each yearly, staggered six months apart; clinical breast exam every 6 to 12 months; risk-reducing mastectomy discussed as an option, she is not pursuing it now; risk-reducing salpingo-oophorectomy advised between 35 and 40 once childbearing is settled, with a gynecologic oncology referral when she is ready; may continue the oral contraceptive |
| 2025-06-11 | outside | bilateral breast MRI with contrast and right breast ultrasound (A4): biopsy-proven fibroadenoma right breast 10 o'clock with clip, unchanged in size; no suspicious enhancement either breast; BI-RADS 2 |
| 2025-07-28 | practice | annual physical (P); normal exam |
| 2025-12-10 | outside | bilateral mammogram (A4): clip and known fibroadenoma right breast, stable; no new findings; BI-RADS 2 |
| 2026-06-02 | practice | visit for a mole on the upper back that she thinks has changed (A5); she reports it has been there for years; no pain or itching. Exam: about 1 cm pigmented lesion, asymmetric, irregular border, brown to black; no lymphadenopathy. Urgent dermatology referral |
| 2026-06-10 | outside | bilateral breast MRI (A4): stable fibroadenoma with clip, no suspicious enhancement, BI-RADS 2 |
| 2026-06-15 | outside | dermatology consult (letter received 2026-06-17): mole on the back for many years, recently changed, no pain or itch; father with melanoma (R2). Exam: 1 cm pigmented lesion of the upper back, asymmetric, irregular borders, brown to black colors, in an area sun-exposed on vacations; dermoscopy with irregular streaks and a blue-white veil; no palpable lymphadenopathy; rest of the skin exam normal. **Excisional biopsy the same day** (A6) |
| 2026-06-19 | outside | biopsy pathology (A6), received 2026-06-23: nodular melanoma, Breslow thickness 1.6 mm, no ulceration, mitotic rate 3 per mm2, no microsatellites, margins clear but narrow; pT2a. **Nodular melanoma of the upper back diagnosed.** Dermatology refers to surgical oncology |
| 2026-07-08 | outside | surgical oncology (A7): wide local excision with 2 cm margins and sentinel lymph node biopsy after lymphoscintigraphy; two right axillary sentinel nodes removed. Pathology 2026-07-13: no residual melanoma, margins clear; 0 of 2 sentinel nodes involved. Stage IB (pT2a N0 M0). No adjuvant therapy; skin and node exams every 3 to 6 months, alternating dermatology and surgical oncology. Received 2026-07-16 |
| 2026-07-22 | practice | annual physical and post-surgery check, last visit (P): incisions healing; father's melanoma added to the family history; labs |
| 2026-07-28 | outside | surgical oncology post-op visit (letter received 2026-07-31, filed): healing well, plan unchanged |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The profile and both notes list the BRCA1 variant from the first encounter,
  yet that same encounter "initiates" genetic testing. Resolution: testing is sent on
  2024-12-09 and the BRCA1 pathogenic variant is known from 2025-01-06. Nothing before
  that date mentions it. The dermatology note's "(diagnosed 2020-01-15)" stamp and its
  duplicated listing become one problem dated January 2025.
- **R2.** Three family histories: the profile (mother breast 42 and ovarian 51, maternal
  aunt breast 39, maternal grandmother breast 44, father hypertension), the gynecology
  note (mother ovarian at 49, an older sister with bilateral breast cancer in her early
  40s) and the dermatology note (father with melanoma). Resolution: the profile's
  history is carried throughout; the mother's ovarian cancer is at 51; the sister is
  not carried (a 34-year-old's older sister in her early 40s is implausible next to the
  profile, which names no sister). The father's melanoma does not contradict the profile
  and is carried, first recorded at dermatology in June 2026 and added by the practice
  in July 2026.
- **R3.** The gynecology note says no prior surgeries; the profile and dermatology note
  give tonsillectomy at 8 and wisdom teeth at 21. Resolution: the surgical history is
  carried everywhere, including the gynecology letter.
- **R4.** The dermatology note dates the breast mass to 2020. Resolution: re-dated to
  December 2024.
- **R5.** The mammogram is only "suspicious irregular mass". Resolution: BI-RADS 4, the
  category that word carries; ultrasound size from the targeted ultrasound as above.
- **R6.** Both notes stop before a plan (gynecology: biopsy pending; dermatology: exam
  only). Resolution: their next steps are additions A3 and A6.
- **Left as the source has it:** before December 2024 no BRCA testing is done and no
  high-risk breast imaging is started, although the maternal history would justify both;
  the practice offers genetics and she defers (A2), and no one presses further. Her
  combined oral contraceptive continues through the BRCA1 diagnosis and the melanoma,
  as the source keeps it.

## Additions (A): physician to confirm

- **A1.** New-patient visit 2023-08-21 (seed: about 3 years with the practice; both
  source encounters are outside care).
- **A2.** Genetic counseling offered by the practice in 2023 and 2024 and deferred by
  her, so the practice notes are coherent with the strong maternal history and with
  testing first happening in December 2024.
- **A3.** The pending biopsy: ultrasound-guided core biopsy, benign fibroadenoma,
  concordant (the dermatology note in 2026 lists no breast cancer and no cancer
  treatment, so the mass must be benign).
- **A4.** Standard ongoing care for the BRCA1 syndrome: the referral visit, the
  high-risk breast program consult and its surveillance (MRI June 2025, mammogram
  December 2025, MRI June 2026, all BI-RADS 2), with the risk-reducing surgery
  discussion but no surgery.
- **A5.** The practice visit that sends her to dermatology.
- **A6.** Excisional biopsy confirming the source's nodular melanoma, with its pathology
  as given.
- **A7.** Wide local excision and sentinel node biopsy, the standard next step for a
  1.6 mm melanoma, with results as given; no adjuvant therapy for stage IB.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may
  carry the descriptors its kind always carries (how a size was measured, a clip's
  location, breast density as scattered fibroglandular), but nothing that adds a
  territory, a severity grade or a symptom, or changes the diagnosis or plan.
- Records from a prior practice may mention routine screening done there, with normal
  results: a cervical cytology with HPV co-test in 2022, negative.
- Yearly physicals with PHQ-2 screens (negative); screening labs at joining (CBC,
  metabolic panel, lipids, TSH) and a CBC and metabolic panel in July 2026, all normal;
  TSH 1.84 at joining; HIV and hepatitis C screening once, negative; a 25-hydroxy vitamin D once in July 2024, 38, normal.
- Influenza and updated COVID vaccines each fall (2023, 2024, 2025); Tdap in 2023.
- Oral contraceptive refills and pharmacy traffic, portal messages about results and
  scheduling, a cancelled or rescheduled visit, reminders.
- Booked visit after 2026-08-27: a follow-up visit with the flu vaccine in October 2026.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00001927 on 2026-07-22. Practice contacts after that day do
not happen; outside care after it still does, and its records are filed as received
without changing the practice's lists. A visit is booked after today.
