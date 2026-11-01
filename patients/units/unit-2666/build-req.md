# Build requirements: unit-2666 (Cameron Hatch, MRN00002666)

Source: synthetic_hospital patient 2666, in `source.md` beside this file. This file and
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
"(diagnosed ...)" stamps, or the "bag of worms" phrase as a heading or chief complaint
copied verbatim (a clinician may describe the texture in their own words).

## Who he is (S)

Born 1996-05-31, male, White, not Hispanic or Latino, English-speaking, employer-sponsored
commercial PPO. Warehouse loader who lifts heavy boxes. Current smoker, 1 pack per day,
started about age 16 (about 10 pack-years in late 2022, about 13 by late 2025; R2).
Alcohol occasional, 1 to 2 drinks a week. No known drug allergies. No surgical history
before 2026. Takes no medications at any point. Family history: father with lung cancer
at 52; paternal grandfather with COPD; mother with hypothyroidism.

Joined the practice on 2020-09-16 (A1), age 24, after a prior primary-care office: no
problems, no medications. Tobacco use is on the problem list from joining (P).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2020-09-16 | practice | new-patient preventive visit (A1): history as above; normal exam; BP normal; height 180 cm, weight about 80 kg; smoking counseling, not ready to quit; screening CBC, CMP, lipids, HIV and hepatitis C antibody, all normal/negative; Tdap and influenza vaccine given (P) |
| 2021-10-06 | practice | annual physical (P): well; still 1 pack per day; normal exam; CBC, CMP normal; influenza vaccine |
| 2022-10-12 | practice | annual physical: about 5 months of a dull left scrotal ache, worse after long shifts on his feet, better lying down (S, onset re-dated R1); PCP feels a soft fullness above the left testis, more obvious standing (A2); testes otherwise normal; **urology referral placed** (A2); CBC, CMP, lipids normal; influenza and COVID bivalent booster |
| 2022-11-25 | outside | urology consult (letter received 2022-12-06): 6 months of mild dull left scrotal ache, worse with prolonged standing, better lying down; no trauma, fever, dysuria or discharge; irregular mass above the left testis that enlarges with Valsalva and does not transilluminate; rest of exam normal; scrotal ultrasound: dilated veins of the left pampiniform plexus with reversal of flow on Valsalva. **Left varicocele diagnosed.** Plan (R3): conservative care with supportive underwear, rest off his feet when he can, return if worse; no drug |
| 2023-10-18 | practice | annual physical (P): occasional mild left scrotal ache, supportive underwear helps; varicocele on exam as before; CBC, CMP normal; influenza and COVID updated vaccine |
| 2024-10-23 | practice | annual physical (P): varicocele quiet most days; CBC, CMP, lipids normal; influenza vaccine |
| 2025-09-17 | practice | scheduled physical: no-show (P) |
| 2025-11-22 | outside | ED visit (record arrives late, 2026-01-21; R4): 5 months of intermittent heaviness in the left groin, worse after prolonged standing and exertion; no trauma, no recent febrile illness; no urinary frequency change, dysuria or hematuria; known left varicocele; lifts heavy boxes at work. Exam: soft compressible mass above the left testis, larger standing; left testis region irregular and tortuous to palpation; right scrotum normal; abdomen soft, no masses or organomegaly; no peripheral edema or cyanosis. No imaging or labs (R5). Discharged to follow up with urology and primary care |
| 2025-11-24 | practice | portal message from patient: went to the ED Saturday for the groin; told to follow up (P) |
| 2025-12-03 | practice | post-ED office visit (P): heaviness as above; left varicocele on standing exam; ED record not yet received; **urology re-referral placed** |
| 2026-02-11 | outside | urology follow-up (letter received 2026-02-24; A3): recurrent symptomatic left varicocele after three years of conservative care, now limiting work; microsurgical subinguinal varicocelectomy offered and chosen |
| 2026-04-09 | outside | **microsurgical subinguinal left varicocelectomy** (A3), outpatient, uncomplicated. The practice learns of it from the patient (portal message 2026-04-13); the operative note arrives only on 2026-08-20 (R4) |
| 2026-08-13 | practice | annual physical, last seen (A4): groin ache gone since surgery; back to full duty; small healed left groin incision, no varicocele palpable standing or with Valsalva; **varicocele resolved**, varicocelectomy added to surgical history; still 1 pack per day, counseled, not ready; CBC, CMP, lipids drawn, normal |
| 2026-10-07 | practice | booked visit: tobacco follow-up and influenza vaccine (P) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Symptom onset follows the practice dates: the ache began about May 2022 (6
  months before the 2022-11-25 urology visit); the ED groin heaviness began about June
  2025 (5 months before 2025-11-22). The ED note's "three years after prior
  presentation" fits the re-dated interval.
- **R2.** "1 pack per day for 10 years" is carried as stated at the 2022 urology visit
  (age 26). Later notes advance the count with time (about 13 pack-years by 2025-26);
  the ED note may still copy "10 years".
- **R3.** The source urology note stops after the ultrasound with no plan. Resolution:
  the letter recommends conservative care (supportive underwear, rest when possible,
  return if worse). No drug, no semen analysis, no surgery in 2022.
- **R4.** Chart texture is "gaps": the ED record is not sent until two months after
  the visit, and the operative note arrives four months late. The practice learns of
  the surgery from the patient; the problem list keeps varicocele active until the
  2026-08-13 visit.
- **R5.** The source lists "Superior vena cava syndrome secondary to lung cancer" as a
  primary diagnosis, but no encounter documents a lung mass, imaging, biopsy or any
  SVC sign; the ED exam explicitly finds no edema or cyanosis, and both encounters are
  about the varicocele. Resolution: lung cancer and SVC syndrome are **not carried**.
  He has no cancer, no chest imaging and no oncology care. The ED does no imaging or
  labs, as the source has none.
- **Left as the source has it:** the ED does no scrotal ultrasound or labs. No one
  prescribes smoking-cessation medication; counseling only, and he is never ready to
  set a quit date. No lung cancer screening (he is under 50). Nobody connects his
  father's lung cancer to his smoking beyond routine counseling.

## Additions (A): physician to confirm

- **A1.** 2020-09-16 new-patient visit (six years with the practice; the source's first
  encounter is a specialist referral, so he had primary care).
- **A2.** The 2022-10-12 practice visit where the PCP finds left scrotal fullness and
  refers to urology (the visit that made the referral).
- **A3.** Recurrent symptomatic varicocele despite three years of conservative care
  takes its usual next step: urology follow-up and microsurgical subinguinal
  varicocelectomy on 2026-04-09, uncomplicated, outpatient, with a short lifting
  restriction set by urology. No pathology, no complication.
- **A4.** At the 2026-08-13 visit the varicocele is resolved after repair: no residual
  varicocele, healed incision, symptoms gone.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention
  (right testis normal, no hernia, normal heart and lungs). An abnormal finding appears
  only as the source or this file gives it.
- Records from a prior practice may mention routine care there, with normal results.
- Yearly preventive visits about every fall with vitals; CBC and CMP most years and
  lipids every two years, normal. HIV and hepatitis C screening once (2020, negative).
  PHQ-2/PHQ-9 and AUDIT-C screens, negative/low.
- Tobacco counseling at every visit; he is not ready to quit.
- Influenza vaccine each fall (at the visit or reported from a pharmacy), COVID primary
  series in spring 2021 at a community site and boosters, Tdap once (2020).
- Portal messages about results and scheduling, reminders, a no-show, records requests.
- Booked visit after 2026-08-27: tobacco follow-up and flu shot.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002666 on 2026-08-13. Practice contacts after that day do
not happen except result messages and records filed as received; outside records filed
after it do not change the practice's lists. A visit is booked after today.
