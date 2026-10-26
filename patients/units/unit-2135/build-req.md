# Build requirements: unit-2135 (Connor Warner, MRN00002135)

Source: synthetic_hospital patient 2135, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, its "shown in the image below" text or its quiz framing.

## Who he is (S)

Connor Warner, born 1984-10-03, male, White, not Hispanic or Latino, English, lives in
Greenfield MA 01301, commercial PPO (Blue Cross Blue Shield). Landscaper (works long
hours in the sun). Never smoker. Social alcohol, 1 to 2 drinks a week. No known drug
allergies. No chronic conditions and no home medications before the timeline.

Family history: father had malignant melanoma diagnosed at 55 and is deceased; mother
has type 2 diabetes; a paternal uncle had basal cell carcinoma.

Surgical history (as it accrues on the timeline, R1): emergency bilateral lower-extremity
fasciotomies with escharotomies at 40 (2025-08-02); wide local excision of the left
forearm melanoma with sentinel lymph node biopsy at 41 (2026-08-19).

Joined the practice 2024-09-10 (A1, seed: about 2 years with the practice) as a healthy
39-year-old with no problems and no medications, wanting a doctor he can reach.
Height 180 cm, weight 82 kg; blood pressure normal throughout.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2024-09-10 | practice | new-patient visit (A1): history as above, normal exam; routine labs (P) |
| 2025-08-02 (Sat) | outside, ED of a regional hospital with a burn center | rescued from a house fire; circumferential burns to both lower legs; severe leg pain not relieved by IV morphine. BP 130/85, HR 110, RR 22, T 37.2 °C (99.0 °F). Burned areas white and leathery (full-thickness); unable to move his toes; tense lower-leg compartments with diminished pulses. Hgb 14.5, WBC 16.0, platelets 250, Na 140, K 4.2, Cr 1.0, BUN 18. Smoke inhalation exposure (S comorbidity): supplemental oxygen, no intubation (A2). **Diagnosed: acute compartment syndrome of both lower legs due to circumferential full-thickness burns; full-thickness burns of both lower legs; smoke inhalation exposure.** Taken to the operating room the same day for **emergency bilateral escharotomies and four-compartment fasciotomies**; admitted to the burn unit (A2) |
| 2025-08-08 | outside, burn unit | excision of the leg burns and split-thickness skin grafting with fasciotomy wound coverage, donor sites both thighs (A2) |
| 2025-08-22 | outside, burn unit | discharged home (A2); grafts taking; walking with a walker then cane; **oxycodone-acetaminophen 5/325 short course** for pain (A3); outpatient burn clinic and physical therapy. Discharge summary and operative notes reach the practice |
| 2025-08-27 | practice | post-hospital visit (P): grafts healing, walking with a cane, pain improving; compartment syndrome treated (resolved on the list). Off work |
| 2025-09-03 to 2025-10 | outside | physical therapy for ankle and knee range of motion and gait (P); notes received |
| 2025-10-22 | outside, burn clinic | grafts and donor sites healed, full-thickness burns healed (resolved on the list); compression garments; cleared for light duty, full duty from January 2026 (A2) |
| 2025-11-12 | practice | annual physical (P); back at light duty; sun protection and skin-check counseling given his work and father's melanoma (P texture, no exam finding) |
| 2026-07-08 | practice | visit for a pigmented spot on the left forearm he first noticed several months earlier that has grown and darkened; **dermatology referral** (P, the referral visit) |
| 2026-07-29 | outside, dermatology | consult letter: lesion noticed months ago, recently larger and changed in color; a year out from the fire and burn care. Left forearm pigmented lesion about 8 mm with irregular border and variegated brown-black color (A4); dermoscopy irregular pigment network with atypical dots and globules. **Excisional biopsy performed today; pathology pending** |
| 2026-08-04 | outside, dermatopathology | pathology report (A5): invasive malignant melanoma, superficial spreading type, tumor thickness 1.2 mm, no ulceration, mitotic rate 2 per mm², margins involved by melanoma in situ at one edge; pT2a. **Malignant melanoma of left forearm diagnosed.** Dermatology refers to surgical oncology |
| 2026-08-10 | outside, surgical oncology | consult (A5): no palpable axillary or epitrochlear nodes; plan wide local excision with 1–2 cm margins and sentinel lymph node biopsy |
| 2026-08-19 | outside, surgical oncology | **wide local excision of the left forearm melanoma site with sentinel lymph node biopsy** (lymphoscintigraphy maps to the left axilla) (A5) |
| 2026-08-24 | outside, pathology | report (A5): no residual melanoma, margins clear; two left axillary sentinel nodes negative for melanoma (0/2). Final stage IB (pT2a N0). Surgical oncology letter: no adjuvant therapy, no routine imaging; full-skin exams with dermatology every 3 to 6 months; post-op wound check booked |
| 2026-08-25 | practice | portal message: patient shares the good news, asks about returning to mowing; PCP replies (P). Today nothing else is open |
| after today | booked | dermatology full-skin exam in the fall (letter mentions it); practice annual physical 2026-11-11 (P) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source profile and both notes list both surgeries from the start, including
  the wide local excision with sentinel node biopsy at the 2025 ED visit, before the
  melanoma existed and while the dermatology visit only takes a biopsy. Resolution: each
  surgery enters the chart on its own date (fasciotomy 2025-08-02, wide excision with
  sentinel node biopsy 2026-08-19). The ED record and the dermatology letter never mention
  the later surgery.
- **R2.** The source ED note says the past history is unremarkable yet lists melanoma
  surgery. Resolution: before 2025-08-02 his history is unremarkable.
- **R3.** The dermatology letter's problem list carries the compartment syndrome as active a
  year later. Resolution: the practice list resolves it at the 2025-08-27 visit after
  fasciotomy; the dermatology letter mentions it as past history only.
- **R4.** The dermatology note says he "returns", though dermatology never saw him.
  Resolution: this is his first dermatology visit, a year after the fire; the letter says so.
- **R5.** The ED note's "unable to move his toes" and the white leathery burns are carried; no
  limb loss or nerve deficit follows (the source names none), and recovery runs to full duty.
- **Left as the source has it:** no one arranged a baseline full-skin exam at joining despite
  his father's melanoma and outdoor work; the chart does not comment. No creatine kinase or
  carboxyhemoglobin values reach the chart.

## Additions (A): physician to confirm

- **A1.** 2024-09-10 new-patient visit (seed: about 2 years with the practice; the source
  begins at the ED).
- **A2.** The usual burn and compartment syndrome course: same-day escharotomies and
  fasciotomies (S surgical history), supplemental oxygen without intubation for the smoke
  exposure, burn-unit admission 2025-08-02 to 2025-08-22, excision and split-thickness
  grafting with fasciotomy wound coverage on 2025-08-08 (full-thickness burns need it), burn
  clinic follow-up to healing by 2025-10-22, light duty then full duty from January 2026.
- **A3.** Oxycodone-acetaminophen 5/325 mg, 1 to 2 tablets every 6 hours as needed, short
  course at discharge (pain control after grafting).
- **A4.** The lesion description (about 8 mm, irregular border, variegated brown-black),
  replacing the source's image.
- **A5.** The melanoma's usual next steps after the biopsy: pathology as above (1.2 mm, no
  ulceration, pT2a, the thickness that calls for sentinel node biopsy), surgical oncology
  consult, wide local excision with sentinel node biopsy on 2026-08-19 (S surgical history,
  age 41), negative margins and 0/2 nodes, stage IB, surveillance plan.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention. An
  abnormal finding appears only as the source or this file gives it.
- New-patient labs 2024: CBC, CMP, lipids, hemoglobin A1c (mother's diabetes), HIV and
  hepatitis C screening once, all normal. Annual physical 2025 with CMP, lipids, A1c normal.
- Post-hospital CBC/BMP at the 2025-08-27 visit, normal.
- Influenza vaccine each fall (2024, 2025); COVID vaccine fall 2024 and 2025; Tdap given once,
  in the ED on 2025-08-02 for the burns (in the ED record). PHQ-2/PHQ-9 screens low.
- Records request to his prior practice, prior records with routine normal care.
- Work notes, disability and return-to-work forms, portal messages about results and
  scheduling, a cancelled or rescheduled visit, refill-free (no chronic drugs).
- Booked visit after 2026-08-27: annual physical 2026-11-11.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.
