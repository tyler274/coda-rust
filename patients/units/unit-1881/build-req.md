# Build requirements: unit-1881 (Christine Jenkins, MRN00001881)

Source: synthetic_hospital patient 1881, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz text.

## Who she is (S)

Born 1997-06-18, female, White, not Hispanic or Latino, English-speaking, Blue Cross
Blue Shield PPO, lives in Easthampton. Administrative assistant. Never smoker, no
alcohol, no recreational drugs. No known drug allergies. No surgical history. Family
history noncontributory. Never pregnant, no prior sexually transmitted infections,
regular menses, no medications.

Joined the practice on 2021-06-08 (A1), age 23, healthy, with no problems and no
medications. Height 165 cm, weight about 63 kg, normal blood pressure throughout her
practice years (P).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2021-06-08 | practice | new-patient preventive visit (A1): healthy, history as above; HIV and hepatitis C screening once, negative; chlamydia and gonorrhea NAAT negative (P); Tdap given (P); prior-practice records requested |
| 2021-06-24 | outside | prior-practice records received: Pap 2019 normal, HPV series completed as a teenager (P) |
| 2022-05-12 | practice | annual physical: Pap (cytology) normal; chlamydia and gonorrhea NAAT negative (P) |
| 2023-05-23 | practice | annual physical: chlamydia and gonorrhea NAAT negative (P) |
| 2024-06-04 | practice | annual physical (P); no STI testing that year (she declined, not in a relationship) |
| 2024-12-06 | practice | contraceptive counseling (S, encounter 0): new sexual relationship for 2 months, sexually active with that partner, no consistent contraception; menses regular, last menstrual period 2 weeks earlier; worried about pregnancy and about STIs because she does not know her partner's status. No prior STI, never pregnant, no medications, NKDA, family history noncontributory, no tobacco, alcohol or drugs. No genitourinary symptoms: no abnormal discharge, pelvic pain or dysuria. Pelvic exam normal. Chlamydia trachomatis and Neisseria gonorrhoeae testing negative. **Barrier contraception (condoms) for dual protection against pregnancy and STIs** recorded and counseled (R1) |
| 2025-02-20 | practice | referral visit (A2): a painless swelling near the vaginal opening, noticed in the shower about 2 months earlier and slowly growing; no pain, bleeding or discharge; no fever or dysuria. Exam shows a soft swelling at the vestibule consistent with a Bartholin gland cyst; referred to gynecology |
| 2025-03-11 | outside | gynecology consult letter (S, encounter 1): 3 months of painless swelling, first noticed while showering, slowly enlarging, no pain, bleeding or discharge; no fever, dysuria or systemic symptoms. One male partner, condoms used inconsistently. NKDA, family history noncontributory. Exam: 3-cm fluctuant, soft, nontender mass at the posterior vaginal vestibule, lateral to the vaginal orifice; overlying mucosa intact, no erythema or induration; no other lesions on labia majora or minora; inguinal nodes not palpable; rest of the pelvic exam normal. **Bartholin's gland cyst** diagnosed. Plan: observation (R2) |
| 2025-11-14 | practice | annual physical, last practice visit (P): Bartholin cyst still painless and not bothering her; Pap (cytology) normal; chlamydia and gonorrhea NAAT negative; influenza vaccine given |
| 2026-03-03 | outside | ED record (S, encounter 2): 3 days of fever, a painful swollen right knee and multiple small pustular lesions on the extremities; polyarthralgia; trouble walking because of the knee; no trauma. Sexually active with multiple partners, condoms not used consistently (**high-risk sexual behavior**, recorded by the hospital). T 38.7 °C (101.7 °F), HR 102, BP 120/78. Right knee erythematous, warm, tender, reduced range of motion; no urethral discharge. WBC 12.0 (12,000/mm³), ESR 50 mm/hr, CRP 25 mg/L. Right knee aspirated: synovial fluid WBC 25,000/mm³ with 90% neutrophils, Gram stain negative. Blood cultures no growth. Urine NAAT for N. gonorrhoeae sent, pending at ED departure. Admitted (A3) |
| 2026-03-03 to 2026-03-09 | outside | hospital admission (A3): urine pregnancy test negative; urine N. gonorrhoeae NAAT positive on 2026-03-04; chlamydia NAAT negative; HIV-1/2 antigen/antibody nonreactive; RPR nonreactive; synovial fluid culture no growth. **Disseminated gonococcal infection** diagnosed. **Ceftriaxone 1 g IV every 24 hours for 7 days**, the course completed in hospital; acetaminophen as needed for fever and pain. Afebrile by hospital day 2, knee pain and swelling better, skin lesions faded. Discharge advice: partners notified and treated, no sex until 7 days after treatment and partners treated, condoms, repeat gonorrhea testing in 3 months, primary-care follow-up in 1 to 2 weeks. Discharge summary received by the practice 2026-03-12 |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source lists "barrier contraception (condom) for dual protection" as a
  diagnosis rather than a choice. Resolution: it is the plan the patient chooses at the
  2024-12-06 counseling visit and is recorded as a problem-list entry that day
  (encounter for counseling on barrier contraception). The profile's listing of it,
  the Bartholin cyst and the gonococcal infection as if all present at once does not
  carry: each starts at its own encounter.
- **R2.** The gynecology note ends with an exam and no plan. Resolution: the letter
  advises observation for an asymptomatic cyst (warm sitz baths if it becomes
  uncomfortable, return if painful, enlarging or infected). No drainage, no
  procedure, no culture.
- **R3.** The source's "High risk sexual behavior" comorbidity is first documented
  at the 2026-03-03 ED visit (multiple partners, inconsistent condoms). Resolution:
  the hospital records it in its letters; the practice's own list, last updated 2025-11-14, does not, and neither the gonococcal infection nor the hospital's drugs reach the practice's structured lists (they arrive after the last visit).
  Earlier practice notes record her one partner and inconsistent condom use only as
  social history.
- **R4.** The ED note ends with the gonorrhea NAAT pending. Resolution: it resolves in
  hospital (A3) as positive, and the hospital names the disseminated gonococcal
  infection. The source's sexual history moves from "one male partner" (March 2025)
  to "multiple partners" (March 2026); both stand, as reports a year apart.
- **Left as the source has it:** the gynecology visit records no STI testing. No one
  comments on it.

## Additions (A): physician to confirm

- **A1.** 2021-06-08 new-patient visit: the source's first encounter is a routine
  visit at an established practice, and the seed gives five years with the practice.
- **A2.** 2025-02-20 practice visit that sends her to gynecology (source encounter 1
  is a specialty visit for a lesion she noticed after the December visit).
- **A3.** The hospital admission and its course: the next step after the ED visit for
  a febrile monoarthritis with pustules and a pending gonorrhea NAAT. The NAAT result
  (positive), the standard tests sent with a gonococcal diagnosis (urine pregnancy
  test negative, chlamydia NAAT negative, HIV-1/2 Ag/Ab nonreactive, RPR nonreactive),
  synovial culture no growth, and ceftriaxone 1 g IV every 24 hours for 7 days in
  hospital (CDC regimen for DGI; no doxycycline once chlamydia is negative). Discharge
  2026-03-09 with the advice listed in the timeline.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may
  carry the descriptors its kind always carries, but nothing that adds a territory, a
  severity grade or a symptom, or changes the diagnosis or plan.
- Records from the prior practice may mention routine screening done there, with
  normal results (Pap 2019 normal; HPV vaccine series complete in adolescence).
- Yearly preventive visits with vitals; normal BMI and blood pressure; PHQ-2 or PHQ-9
  screens negative; a CBC and CMP once at joining, normal.
- Cervical cytology every 3 years (2022, 2025), normal. Chlamydia and gonorrhea
  screening with negative results as listed in the timeline. HIV and hepatitis C
  screening once at joining, negative.
- Influenza vaccine most falls (nurse visit or reported from a pharmacy), COVID
  vaccines reported or given, Tdap once at joining.
- Portal messages about results and scheduling, a missed or rescheduled visit,
  records requests and filing, front-desk calls.
- After 2025-11-14: the hospital records filed as received, a front-desk call offering
  the post-hospital follow-up she does not book then, and an August 2026 portal
  message in which she books a visit. The 3-month repeat gonorrhea test has not been
  done by today.
- Booked visit after 2026-08-27: 2026-09-10, follow-up after the March hospitalization
  and repeat STI testing.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.


## Last seen

The practice last saw MRN00001881 on 2025-11-14. Practice contacts after that day do
not happen; outside care after it still does, and its records are filed as received
without changing the practice's lists. A visit is booked after today.
