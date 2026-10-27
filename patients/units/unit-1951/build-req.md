# Build requirements: unit-1951 (Beth Boucher, MRN00001951)

Source: synthetic_hospital patient 1951, in `source.md` beside this file. This file and
the source are the only authority for clinical facts. Every diagnosis, medication,
result, procedure and specialist finding in the chart must trace to one of:

- **S** a fact in source.md, carried as listed below;
- **R** a resolution below, where the source contradicts itself or leaves a plan unfinished;
- **A** an addition below, listed one by one;
- **P** the primary-care layer described at the end.

Nothing else. No diagnosis, medication, test result, procedure, family member or
life event that is not on these lists. Texture (wording, scheduling, refills, portal
traffic, who signed what) is free, as long as it carries no new clinical fact.

Re-voice everything. The source notes are exam vignettes: never copy a sentence, and
never carry a source clinician's name, its "Active Problem List" dump, its
"(diagnosed ...)" stamp, or its "you note" exam phrasing.

## Who she is (S)

Born 1994-06-11, female, White, not Hispanic or Latino, English-speaking, employer
PPO, lives in the seed town. Marketing coordinator. Never smoker. Social alcohol, 2 to
3 drinks a week. No known drug allergies. Surgical history: tonsillectomy at age 8
(about 2002). Family history: mother with breast cancer at 55; father with type 2
diabetes. No chronic conditions. In a monogamous relationship with one male partner
(together about a year by early 2025), no prior sexually transmitted infections;
inconsistent condom use (stated in 2026).

Joined the practice on **2022-08-23** (A1, new-patient physical), age 28, healthy,
bringing one drug: **norgestimate 0.25 mg / ethinyl estradiol 0.035 mg tablet, 1 daily**
(the source's "ethinyl estradiol/norgestimate 1 tablet PO daily"; R1), which the
practice continues and refills throughout. No problems on joining.

Height 166 cm; weight about 63 kg, stable (BMI about 23). Blood pressure normal throughout.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2022-08-23 | practice | new-patient preventive visit (A1): history as above; on the combined pill, continued; normal exam; prior records requested |
| 2022-09 | outside records | prior primary-care office records: a normal Pap in 2021 (cytology, negative for intraepithelial lesion), Tdap in 2019, healthy (P) |
| 2023-08-22 | practice | annual physical: well; CBC, CMP, lipid panel normal; HIV and hepatitis C screening once, both nonreactive (P) |
| 2024-08-20 | practice | annual physical: well; cervical cytology with HPV co-test: negative for intraepithelial lesion, high-risk HPV not detected (P; the source's "last Pap smear within normal limits") |
| 2025-01-21 | practice | office visit: several painless flesh-colored bumps around the anus for about 2 months (onset about early November 2024), no bleeding, self-conscious about them; exam shows small verrucous perianal papules, no inguinal nodes; **dermatology referral** (A2). STI screening drawn (A3) |
| 2025-02-10 | outside | dermatology consult letter: 3 months of multiple painless flesh-colored perianal growths, no bleeding, self-conscious; past history unremarkable; tonsillectomy at 8; on the combined pill; NKDA; family history as above; monogamous for the past year, no prior STIs; several 1 to 2 cm verrucous papules around the anus, no lymphadenopathy, rest of exam normal. Biopsy of one lesion. Received 2025-02-12 |
| 2025-02-14 | outside | surgical pathology report received: perianal skin biopsy with koilocytosis, consistent with condyloma acuminatum. **Condylomata acuminata diagnosed** in the chart (onset 2024-11) |
| 2025-03-10 | outside | dermatology: liquid nitrogen cryotherapy to the perianal warts (A4); letter received |
| 2025-04-07 | outside | dermatology: lesions much smaller; second cryotherapy session to the few remaining papules (A4); letter received |
| 2025-05-12 | outside | dermatology recheck: perianal area clear, no remaining lesions; return as needed (A4); letter received |
| 2025-08-19 | practice | annual physical: no recurrence since May; **condylomata acuminata resolved** on the problem list (A4); CBC and CMP normal |
| 2026-01-29 | practice | portal message: a new bump near the vaginal opening noticed about a week ago, painless; PCP places a **gynecology referral** (A5) |
| 2026-02-16 | outside | gynecology consult letter: returns with a new growth near the external genitalia a year after the perianal lesions of February 2025; first noticed a few weeks earlier, slowly growing; no pain, discharge or itching; otherwise healthy, last Pap normal; tonsillectomy at 8; on the combined pill; NKDA; family history as above; one male partner, inconsistent condom use, nonsmoker. Exam: soft exophytic lesion near the perineum made of multiple projections with an irregular surface; no inguinal lymphadenopathy; no other lesions on speculum exam. Recurrent condyloma; **cryotherapy in the office that day** and condom counseling (A6). Received 2026-02-18; **condylomata acuminata diagnosed again** in the chart (onset 2026-01) |
| 2026-03-16 | outside | gynecology recheck: treated lesion resolved, no new lesions; return as needed (A6); letter received |
| 2026-08-11 | practice | annual physical, last seen: well; no recurrence since March; **condylomata acuminata resolved** again (A6); pill continued; CBC and CMP drawn, normal, released 2026-08-12 |
| after 2026-08-27 | practice | booked office visit (P) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source names "ethinyl estradiol/norgestimate" without a strength. Resolution:
  the standard monophasic norgestimate 0.25 mg / ethinyl estradiol 0.035 mg tablet, one daily.
- **R2.** The source's gynecology note says the perianal lesions were in "January 2020". The
  re-dated dermatology visit is February 2025, so the chart says February 2025 (a year
  before the gynecology visit).
- **R3.** The source's gynecology note carries the condyloma as an active problem a year
  after the first lesions, with no treatment recorded between. Resolution: the perianal
  warts were treated and cleared in spring 2025 (A4); the gynecology visit is a recurrence
  at a new site, and the chart records the condition again then.
- **R4.** Neither source note states a plan (both end at the exam or pathology). The chart
  gives each specialist the usual next step (A4, A6) and carries no source quiz text.
- **Left as the source has it:** HPV vaccination is never raised or given (she was past
  26 and nobody brings it up). No anal cytology or partner evaluation is mentioned. Neither
  gap is commented on.

## Additions (A): physician to confirm

- **A1.** New-patient visit 2022-08-23 (four years with the practice; the source's first
  encounter is a specialist).
- **A2.** The 2025-01-21 practice visit that sends her to dermatology.
- **A3.** STI screening at the 2025-01-21 visit for new anogenital warts: HIV-1/2 antigen
  and antibody nonreactive, RPR nonreactive, urine chlamydia and gonorrhea NAAT negative
  (consistent with the source's "no prior history of sexually transmitted infections").
- **A4.** Dermatology treats the perianal warts with liquid nitrogen cryotherapy on
  2025-03-10 and 2025-04-07; clear on 2025-05-12. The practice resolves the problem at
  the 2025-08-19 physical.
- **A5.** The 2026-01-29 portal exchange and gynecology referral that lead to the
  2026-02-16 visit.
- **A6.** Gynecology treats the recurrent lesion with in-office cryotherapy on
  2026-02-16 and counsels on condoms; resolved at the 2026-03-16 recheck. The practice
  resolves the problem again at the 2026-08-11 physical.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Records from a prior practice may mention routine screening done there, with normal
  results (the 2021 Pap, 2019 Tdap).
- Yearly physicals with vitals, PHQ-2 and blood pressure checks on the pill; CBC, CMP
  and lipids at some of them, normal. Cervical screening as above (next due 2029).
  Breast screening discussed for age 40 given her mother's history; none done yet.
- Influenza vaccine each fall (2022 to 2025) and COVID vaccines some falls, at nurse
  visits. HIV and hepatitis C screening once (2023).
- Pill refills, pharmacy traffic, portal messages about results and scheduling, a
  missed or rescheduled visit, reminders.
- Booked visit after 2026-08-27: a fall office visit for the flu shot and a
  contraception check.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00001951 on 2026-08-11. Practice contacts after that day do not happen beyond messages and refills; outside records after it are filed as received without changing the practice's lists. A visit is booked after today.
