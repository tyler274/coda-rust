# Build requirements: unit-1868 (Iris Desrosiers, MRN00001868)

Source: synthetic_hospital patient 1868, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz-like quoted questions verbatim.

## Who she is (S)

Iris Desrosiers, born 1999-08-10, female, White, not Hispanic or Latino, English,
lives in Florence (01062), university student health plan. Graduate student. Never
smoker; social alcohol; no drug use. No known drug allergies. No surgical history.
Family history: mother with generalized anxiety disorder, father with hypertension.
No chronic conditions and no home medications.

Joined the practice on 2021-09-14 (A1), at the start of graduate school, with no
active problems and no medications.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2021-09-14 | practice | new-patient preventive visit (A1). History as above. Height 165 cm, weight 61 kg. Prior records requested from her college health service. Tdap given (P). HIV and hepatitis C screening once, nonreactive (P); CBC and CMP normal (P) |
| 2021-09-20 | outside records | college health and childhood immunization records received (P): childhood series complete, HPV series and MenACWY done in adolescence, COVID-19 primary series spring 2021, cervical cytology 2020 normal (NILM) |
| 2022-04-12 | practice | annual preventive visit (P); lipid panel normal |
| 2022-08-01 | outside, regional hospital ED | seen early morning the day after a college party the night of 2022-07-31 (R1): she was forcibly given alcohol and possibly drugged, remembers little, woke in a dorm room with torn clothing and soreness. Visibly distressed, flat affect, poor eye contact; bruising of both upper arms and thighs; pelvic exam shows vaginal trauma. Urine toxicology positive for benzodiazepines (R2); urine pregnancy test negative; CBC normal (R3). Worried about sexually transmitted infections and unsure what to do next. **Sexual assault** and **contusion of thigh** diagnosed. Post-assault care (A2): STI prophylaxis, emergency contraception and 28-day HIV post-exposure prophylaxis, with baseline tests. Sent to outpatient psychiatry the same day. ED record received 2022-08-03 |
| 2022-08-01 | outside, outpatient psychiatry | afternoon evaluation after the ED visit: poor sleep since the assault, flashbacks, intense fear with loud noises and crowds, feeling detached, persistent guilt. Denies substance misuse. Vitals normal; anxious, flat affect; bruises on upper extremities and thighs; pelvic exam deferred at her request. ED labs reviewed (R2, R3). **Acute stress reaction** diagnosed. Plan: trauma-focused counseling (connected with the university counseling center), no medication (R4). Letter received 2022-08-03 |
| 2022-08-02 | practice | telephone call; post-ED visit booked (P) |
| 2022-08-04 | practice | post-ED visit (A3): records reviewed; taking PEP and doxycycline and metronidazole as prescribed; symptoms as in the psychiatry letter; bruises fading. PHQ-9 11, GAD-7 13. Sexual assault, contusion of thigh and acute stress reaction on the problem list |
| 2022-08-29 | practice | follow-up (A3): PEP course completed 2022-08-28; sleeping better, flashbacks and fear fading, weekly counseling. PHQ-9 5, GAD-7 6. Bruises gone. **Acute stress reaction resolved; contusion of thigh resolved** (R4) |
| 2022-09-13 | practice | 6-week post-exposure draw (A2): HIV-1/2 Ag/Ab nonreactive, RPR nonreactive |
| 2022-11-01 | practice | 3-month follow-up (A2): HIV-1/2 Ag/Ab, RPR and hepatitis C antibody nonreactive. Doing well, still in counseling. PHQ-9 3, GAD-7 3. **Sexual assault resolved; personal history of adult physical and sexual abuse recorded** (R5) |
| 2023-04-18 | practice | annual preventive visit (P); cervical cytology normal (NILM) |
| 2024-04-09 | practice | annual physical missed (no-show) (P) |
| 2024-04-30 | practice | annual preventive visit (P) |
| 2025-04-22 | practice | annual preventive visit (P) |
| 2026-05-08 | practice | annual preventive visit (P); last seen. Cervical cytology normal (NILM); CBC, CMP and lipid panel normal |
| 2026-07-21 | outside, regional hospital ED | painful genital ulcers with fever and dysuria; tender inguinal lymphadenopathy; several coalescing vesicles on an erythematous base. Temperature 38.4 °C (A4). Tzanck smear of a lesion: multinucleated giant cells. **Primary genital herpes simplex virus infection** diagnosed; **empiric acyclovir 400 mg three times daily for 10 days** started (A4). Told to follow up with gynecology. Record received 2026-07-23; practice lists not updated |
| 2026-07-24 | outside, gynecology | follow-up 3 days after the ED visit (age 26): pain modestly better; very anxious and embarrassed; worried about passing it on, recurrence and long-term effects; asks whether the medicine is lifelong, whether she can have children, and what it means for her partner. In a new monogamous relationship. On acyclovir only. Exam: lesions partly healed. Tzanck result reviewed. History of sexual assault in 2022 with acute stress reaction (R6). Counseling (R7). Letter received 2026-07-29; practice lists not updated |
| 2026-09-10 | practice | booked visit: follow-up after the gynecology visit (P) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The ED says the assault was "last night"; the psychiatry note, written "today"
  after the ED visit, says "2 nights ago". Resolution: the assault is the night of
  2022-07-31, the ED visit early on 2022-08-01 and the psychiatry evaluation that
  afternoon. "2 nights ago" is not carried.
- **R2.** The ED urine toxicology is positive for benzodiazepines; the psychiatry note,
  the same day, lists a negative screen. Resolution: one urine toxicology screen, in the
  ED, positive for benzodiazepines (other classes negative). The negative screen is not
  carried; the psychiatry letter refers to the ED result.
- **R3.** An outpatient psychiatry evaluation lists CBC, toxicology and pregnancy results.
  Resolution: these were drawn in the ED (CBC normal, urine pregnancy negative) and the
  psychiatry letter reviews them. No labs are drawn at psychiatry.
- **R4.** The source gives no plan at psychiatry and lists acute stress reaction as active
  in 2026. Resolution: counseling only, no medication. Symptoms settle within about four
  weeks and the practice resolves the diagnosis on 2022-08-29 (within the one-month
  window of an acute stress reaction, so no later diagnosis is needed). The 2026 outside
  letters may still carry it on their copied history lists, as outside lists do.
- **R5.** The source lists sexual assault as an active problem in 2026. Resolution: the
  acute injury code (sexual assault, initial encounter) is resolved at the 3-month visit
  and carried afterward as personal history of adult physical and sexual abuse.
- **R6.** The 2026 gynecology note dates the assault to 2020; on the re-dated timeline it
  is 2022. Age 26 at that visit fits the birth date.
- **R7.** The gynecology note ends with the patient's questions and no answers.
  Resolution: the letter records counseling in its own words: finish the acyclovir
  course; recurrences can be treated episodically or prevented with daily suppressive
  therapy, which is a choice and not required for life; pregnancy remains possible, and
  she should tell her obstetric clinician, who would offer suppressive therapy late in
  pregnancy; her partner may be offered testing; avoid sex while lesions are present and
  use condoms. No new drug is started.
- **Left as the source has it:** no HSV PCR, type-specific serology or other STI testing
  at the 2026 ED or gynecology visits; no medication for sleep or anxiety in 2022; no
  psychiatry follow-up letter after the first evaluation. Nobody fills or remarks on these.

## Additions (A): physician to confirm

- **A1.** The 2021-09-14 new-patient visit (the seed gives five years with the practice).
- **A2.** Standard post-sexual-assault care at the 2022 ED, which the source leaves
  unfinished ("unsure what steps to take next"): ceftriaxone 500 mg IM once, doxycycline
  100 mg twice daily for 7 days, metronidazole 500 mg twice daily for 7 days,
  levonorgestrel 1.5 mg once, a forensic examination with evidence kit and a victim
  advocate present (whether to report is her choice; by November she decides not to),
  and HIV PEP with bictegravir/emtricitabine/tenofovir
  alafenamide one tablet daily for 28 days. Baseline tests, all normal or negative:
  BMP, hepatic panel, HIV-1/2 Ag/Ab, hepatitis B surface antigen negative, hepatitis B
  surface antibody positive (vaccinated), hepatitis C antibody, RPR, gonorrhea,
  chlamydia and trichomonas NAAT. Follow-up HIV and RPR at 6 weeks and 3 months, and
  hepatitis C antibody at 3 months, at the practice, all nonreactive.
- **A3.** The two practice follow-up visits after the ED (2022-08-04 and 2022-08-29).
- **A4.** The 2026 ED's fever is given as 38.4 °C, and its "empirical acyclovir" as
  acyclovir 400 mg three times daily for 10 days (a standard primary-episode course).
  The practice lists never carry it (after last seen).

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Records from a prior practice (college health) may mention routine screening done there,
  with normal results.
- Yearly preventive visits with vitals, PHQ-9 and GAD-7 (scores 0 to 3 outside the 2022
  values above), alcohol screening consistent with social drinking; cervical cytology
  every 3 years (2023, 2026, normal); a lipid panel in 2022 and 2026, CBC and CMP at
  joining and in 2026, all normal. HIV and hepatitis C screening once (2021).
- Influenza vaccine each fall and COVID-19 vaccines (booster late 2021, updated doses
  each fall), Tdap at joining.
- Portal messages about results and scheduling, an academic accommodation letter in
  August 2022, a school immunization form, a missed physical in April 2024.
- After 2026-05-08: result messages, the patient's portal message after the 2026 ED
  visit and the physician's reply, outside records filed as received, a scheduling call.
- Booked visit after 2026-08-27: follow-up after the gynecology visit.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00001868 on 2026-05-08. Practice contacts after that day are
messages and calls only; outside care after it still happens, and its records are filed
as received without changing the practice's lists. A visit is booked after today.
