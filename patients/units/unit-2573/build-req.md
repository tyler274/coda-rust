# Build requirements: unit-2573 (Gavin Davidson, MRN00002573)

Source: synthetic_hospital patient 2573, in `source.md` beside this file. This file and
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
never carry a source clinician's name, its "Active Problem List" dumps, or its
"(diagnosed 2020-...)" stamps.

## Who he is (S)

Gavin Davidson, born 2002-09-26, male, White, not Hispanic or Latino, English,
university student health insurance plan, university undergraduate student. Never
smoker. Social alcohol, 2 to 3 drinks on weekends. No known drug allergies. No
surgical history. Family history: father with hypertension diagnosed at 45; mother
with major depressive disorder. No other family members.

Joined the practice in 2022 (A1: new-patient visit 2022-09-13, age 19). He brought no
problems and no medications.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2022-09-13 | practice | new-patient visit (A1): healthy, no problems, no medications, NKDA, history as above. Baseline CBC and CMP normal (P) |
| 2023-08-22 | practice | annual physical (P) |
| 2024-08-27 | practice | annual physical (P); Tdap; lipid panel normal |
| 2025-01-21 (Tue) | practice | same-day visit: burning on urination and a white discharge from the penis for 3 days; unprotected intercourse with a new partner 2 weeks before. PMH unremarkable, no medications. Exam: no rash, no lymphadenopathy, no joint swelling; clear urethral discharge (R1); no penile lesions; testes non-tender, not swollen. Urinalysis: leukocyte esterase positive, nitrite negative, WBC 10-15/hpf, RBC 0-2/hpf. Urine NAAT for gonorrhea and chlamydia sent, pending. **Gonococcal urethritis diagnosed** (treated as presumed). **Ceftriaxone 500 mg IM once given; doxycycline 100 mg twice daily for 7 days started** (A2) |
| 2025-01-23 | practice | NAAT resulted: Neisseria gonorrhoeae detected, Chlamydia trachomatis not detected (A3). Result note; partner told to be tested and treated |
| 2025-07-28 (Mon) | practice | returns about 6 months after the urethritis visit: a painless ulcer on the penis that appeared 3 weeks after unprotected intercourse with a new partner; no other symptoms. Exam: one well-demarcated, indurated, painless ulcer on the penile shaft; no inguinal lymphadenopathy; rest of exam unremarkable. Draw: RPR, HIV antigen/antibody, hepatitis B surface antigen, hepatitis C antibody, FTA-ABS. **Primary syphilis diagnosed; benzathine penicillin G 2.4 million units IM once given** (A4). **High risk sexual behavior added to the problem list.** Gonococcal urethritis marked resolved (R2) |
| 2025-07-29 | practice | results: RPR positive; HIV antigen/antibody negative; hepatitis B surface antigen negative; hepatitis C antibody negative. FTA-ABS pending |
| 2025-07-31 | practice | FTA-ABS reactive (A5), confirming primary syphilis |
| 2026-02-03 | practice | 6-month syphilis follow-up visit (A6): ulcer long healed; RPR non-reactive. Primary syphilis marked resolved |
| 2026-06-19 | practice | annual physical (P), last seen. Screening HIV antigen/antibody non-reactive, RPR non-reactive, urine gonorrhea and chlamydia NAAT not detected (P) |
| 2026-07-25 (Sat) | outside (ED) | regional hospital emergency department: rectal pain, mucopurulent anal discharge and intermittent rectal bleeding with tenesmus for 1 week; anal itching; pain with defecation. Prior gonococcal urethritis and primary syphilis. Unprotected intercourse with multiple partners, including receptive anal intercourse; last encounter about 2 weeks before. Vital signs within normal limits. Appears uncomfortable; anal canal erythematous and tender with a small amount of purulent discharge; no masses; prostate non-tender on digital rectal exam. WBC 12,000/µL; rectal swab NAAT positive for Neisseria gonorrhoeae; RPR non-reactive; HIV 1/2 antigen/antibody non-reactive. **Gonococcal proctitis diagnosed. Ceftriaxone 500 mg IM once given in the ED; doxycycline 100 mg twice daily for 7 days prescribed** (A7). Discharged home |

## Records (chart_texture: gaps)

- The practice's own visits and draws are complete.
- 2022: the prior pediatric office sends only an immunization printout (Tdap 2014,
  MenACWY 2018, childhood series noted complete); the rest of the requested records
  never arrive, and nobody chases them.
- Fall 2023: he gets his influenza vaccine at a pharmacy; the practice learns of it
  from him and adds it to the immunization history.
- ED 2026-07-25: he tells the practice by portal message on 2026-07-27 that he was
  treated in the ED; the practice requests the record; the ED note and labs arrive on
  2026-08-11 and are filed as received. Nothing on the practice's problem or
  medication lists changes after 2026-06-19.

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The chief complaint calls the discharge white; the examiner saw a clear
  urethral discharge. Resolution: the patient's description (white) stays in the
  history and the examiner's finding (clear) in the exam. Both are carried as written.
- **R2.** The ED copies gonococcal urethritis forward as active a year and a half after
  it was treated. Resolution: the practice marks it resolved at the July 2025 visit
  (treated in January, symptoms gone). The ED note may still list both prior infections
  in its history, as outside notes do.
- **R3.** Source dates sit on the 15th; the practice dates in source.md are used as
  given. Ages follow the seed birth date: 22 at both 2025 visits, 23 at the ED.
- **Left as the source has it:** no HIV, syphilis or extragenital testing at the January
  2025 urethritis visit; no three-month retest after the gonorrhea; no HIV pre-exposure
  prophylaxis is raised at any visit; the rectal site is never swabbed at the practice.
  No note remarks on any of these.

## Additions (A): physician to confirm

- **A1.** 2022-09-13 new-patient visit. The seed has him with the practice about 4 years,
  so he joined before the first source encounter.
- **A2.** Treatment at the January 2025 visit (the source names none): ceftriaxone 500 mg
  IM once and doxycycline 100 mg by mouth twice daily for 7 days, the standard regimen
  while chlamydia is not yet excluded.
- **A3.** The NAAT the source leaves pending: gonorrhea detected, chlamydia not detected.
  This is the evidence for the diagnosis the next source note lists.
- **A4.** Benzathine penicillin G 2.4 million units IM once at the ulcer visit (NKDA), the
  standard treatment for the primary syphilis the next source note lists.
- **A5.** FTA-ABS, pending in the source: reactive.
- **A6.** One 6-month follow-up visit after syphilis treatment with RPR non-reactive,
  consistent with the ED's non-reactive RPR a year after treatment.
- **A7.** ED treatment for the gonococcal proctitis (the source names none): ceftriaxone
  500 mg IM once and doxycycline 100 mg twice daily for 7 days. The ED note may say a
  rectal chlamydia NAAT was sent, with no result in the record received.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Yearly physicals in 2023, 2024 and 2026 (none in 2025, when the infection visits took
  their place), with vitals; height about 180 cm, weight about 76 to 79 kg, BP normal.
  PHQ-2 and AUDIT-C screens (AUDIT-C low, in keeping with weekend social drinking).
- Labs: baseline CBC and CMP in 2022, one lipid panel in 2024, all normal; the June 2026
  STI screen above. HIV and hepatitis C screening happen at the July 2025 source draw.
- Vaccines: influenza each fall (2022, 2023 at a pharmacy, 2024, 2025); COVID-19 in fall
  2022 and fall 2024; Tdap in 2024 (ten years after 2014).
- Portal messages about results and scheduling, a school immunization-form request, a
  missed physical in August 2024 rescheduled two weeks later, a public health follow-up
  call after the syphilis report, reminders.
- Booked visit after 2026-08-27: an ED follow-up visit in early September 2026.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002573 on 2026-06-19. Practice contacts after that day are
only messages and records; the ED visit of 2026-07-25 reaches the chart as a record
filed on 2026-08-11 without changing the practice's lists. A visit is booked after today.
