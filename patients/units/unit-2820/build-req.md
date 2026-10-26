# Build requirements: unit-2820 (Beth Lambert, MRN00002820)

Source: synthetic_hospital patient 2820, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz text (the occupational-health social
history about a meta-analysis of melanoma immunotherapy trials).

## Who she is (S)

Beth Lambert, born 1982-06-26, female, White, not Hispanic or Latino, English-speaking,
Holyoke MA 01040, commercial PPO (seed identity, unchanged). Single (seed gives no
partner; none is added). Biomedical research scientist (PhD). Never smoker. Alcohol
occasional, 1 to 2 drinks a week. No drug use.

- **Allergies:** penicillin (rash); dust mites (environmental, drives her rhinitis).
- **Surgical history:** tonsillectomy at age 7 (1989); wisdom teeth extraction at age 18
  (2000).
- **Family history:** mother with hypertension; father with type 2 diabetes; maternal
  grandmother with breast cancer at 68. No family history of melanoma or other skin
  cancer.
- **Joined the practice** 2022-09-20 (A1), about four years before today, after years at
  a prior practice. Brought: seasonal allergic rhinitis (since childhood; onset recorded
  1992) and migraine without aura (since her twenties; onset recorded 2006), as she
  reports them at joining; cetirizine 10 mg by mouth daily and sumatriptan 50 mg by mouth as needed
  for migraine. No personal history of skin cancer.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2022-09-20 | practice | new-patient preventive visit (A1): history as above; exam normal; baseline labs; flu vaccine (P) |
| 2024-05-21 | practice | office visit (A2): a mole on her back has been getting bigger for about 5 months and has lately started to itch; PCP sees an irregular pigmented lesion on the back and places an **urgent dermatology referral** |
| 2024-06-14 | outside | dermatology consult (S): mole on the back enlarging over 6 months, recently itchy; she is worried about skin cancer; no personal or family history of skin cancer; lesion 1.2 cm in diameter, irregular borders, varied brown and black color, a slightly raised nodular area; rest of the skin exam unremarkable; no palpable lymph nodes. Same-day **excisional biopsy** (A3). Letter received 2024-06-18 |
| 2024-06-20 | outside | biopsy pathology (A3): invasive melanoma, superficial spreading type; Breslow thickness 0.6 mm; no ulceration; mitotic rate less than 1 per mm²; no microsatellites; no lymphovascular invasion; peripheral and deep margins free, closest 1 mm; pT1a. **Malignant melanoma of the back diagnosed** (R1). Report received 2024-06-21 |
| 2024-07-12 | outside | dermatologic surgery: **wide local excision with 1 cm margins** (A4); no sentinel node biopsy and no staging imaging (not indicated for pT1a, stated in the letter) |
| 2024-07-18 | outside | excision pathology (A4): biopsy-site scar, no residual melanoma, margins clear. Final stage IA (pT1a, clinically node-negative). Plan: full-skin and node exams every 6 months, monthly self-exams, sun protection (A5). Received 2024-07-19 |
| 2025-01-17 | outside | dermatology surveillance (A5): scar well healed, no recurrence, no new suspicious lesions, no palpable nodes |
| 2025-07-18 | outside | dermatology surveillance (A5): same findings |
| 2025-12-19 | outside | occupational health consultation (S): medical clearance for a research protocol and a systematic health assessment; problem list melanoma, seasonal allergic rhinitis, migraine without aura; surgical, medication, allergy, family and social history as above. **Cleared without restrictions** (R3). Letter received 2025-12-23 |
| 2026-01-23 | outside | dermatology surveillance (A5): same findings |
| 2026-07-24 | outside | dermatology surveillance (A5): same findings; next exam in 6 months. Received 2026-07-28 |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source names malignant melanoma as the primary diagnosis and later stamps it
  as diagnosed on the dermatology visit date, but that visit's note ends with only her
  worry about skin cancer and gives no tissue diagnosis. Resolution: the dermatology letter
  of 2024-06-14 calls the lesion clinically suspicious for melanoma and biopsies it; the
  diagnosis is recorded when the pathology arrives (2024-06-21), with onset 2024-06-14.
  No record before that lists melanoma. Coded as malignant melanoma of other part of trunk
  (C43.59), active through today.
- **R2.** The occupational-health social history is a vignette about "a 38-year-old female
  researcher" running a meta-analysis. It is quiz text and the age contradicts the timeline
  (she is 43 on 2025-12-19). Not carried. The letter records only her occupation, research
  scientist. Her age at the dermatology visit (41) matches the source's age at first
  encounter.
- **R3.** The occupational-health note has no assessment or plan. Resolution: the letter
  states she is medically cleared for the protocol without restrictions and asks her to keep
  her dermatology surveillance. No tests, vitals numbers or new findings are carried from it.
- **Left as the source has it:** migraine stays on as-needed sumatriptan alone, with no
  preventive drug, and allergic rhinitis on cetirizine alone, with no nasal steroid or
  allergen immunotherapy for the dust-mite allergy. Nobody adds either, and no note
  comments on it.

## Additions (A): physician to confirm

- **A1.** 2022-09-20 new-patient preventive visit (the first source encounter is a
  specialist consult; she needs a primary-care home, and the seed gives four years with the
  practice).
- **A2.** 2024-05-21 practice visit that sends her to dermatology.
- **A3.** Excisional biopsy at the dermatology visit, with the pathology above (the source
  asserts melanoma with no tissue diagnosis; a thin, non-ulcerated tumor is the
  conservative reading of a 1.2 cm lesion with no palpable nodes).
- **A4.** Wide local excision with 1 cm margins and its pathology (standard treatment of a
  pT1a melanoma), stage IA.
- **A5.** Dermatology surveillance every 6 months after excision, each visit without
  recurrence or new suspicious lesions; sun protection and self-exam advice.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may carry
  the descriptors its kind always carries (breast density on a mammogram, a specimen's
  size and orientation, a procedure's closure), but nothing that adds a territory, a
  severity grade or a symptom, or changes the diagnosis or plan.
- Records from the prior practice (received October 2022): routine care with normal
  results, including a normal Pap with negative HPV in 2019, Tdap in 2018 and the original
  COVID-19 series with a booster in 2021.
- Yearly preventive visits (2022, 2023, 2024, 2025; the 2026 one booked) with vitals,
  PHQ-2, AUDIT-C, a skin check of the excision scar after 2024, and normal labs: CBC, CMP,
  lipids and A1c (father with diabetes), one TSH, one-time HIV and hepatitis C screens.
  Height 165 cm, weight about 61 to 63 kg; blood pressure normal.
- Migraine without aura stable on as-needed sumatriptan, a few attacks a month, no new
  features; seasonal allergic rhinitis on daily cetirizine, worse in spring; one telehealth
  visit for spring allergy symptoms (non-drug advice only).
- Cervical screening: Pap with HPV co-test at the 2024 physical, normal. Screening
  mammograms each November from 2022, BI-RADS 1. Colorectal screening not yet due (45 in
  2027).
- Influenza vaccine each fall; updated COVID-19 vaccine each fall from 2023.
- Refills, pharmacy traffic, portal messages about results, refills and scheduling, a
  cancelled visit, a work immunization-record request, and the occupational-health records
  request with the practice's summary sent under her authorization.
- Booked visit after 2026-08-27: the 2026 annual preventive visit on 2026-09-29.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.
