# Build requirements: unit-1783 (Nicole Rousseau, MRN00001783)

Source: synthetic_hospital patient 1783, in `source.md` beside this file. This file and
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
"(diagnosed ...)" stamps, or its "a hysterosalpingogram is shown" quiz text.

## Who she is (S)

Born 1995-09-07, female, White, not Hispanic or Latino, English, Blue Cross Blue Shield
PPO, lives in Greenfield. High school mathematics teacher. Never smoker, no alcohol.
No known drug allergies. No surgical history. Married (the source calls him partner
and husband). Family history: maternal grandmother type 2 diabetes, mother
hypothyroidism, father essential hypertension; no family history of infertility,
endometriosis or early menopause. Healthy weight (height 165 cm, about 61 kg
throughout, BMI about 22).

Joined the practice on 2022-05-10 (A1, new-patient preventive visit, age 26) with no
chronic conditions, no medications, no allergies. Regular menses.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2022-05-10 | practice | new-patient preventive visit (A1); history as above; exam normal; screening labs and HIV/HCV screening (P) |
| 2023-06-13 | practice | annual physical (P); she and her husband began trying to conceive about a month ago (S, follows from 14 months by July 2024) |
| 2024-06-11 | practice | office visit (A2): trying to conceive about 13 months without success; menses regular every 28 to 30 days; no STI history, pelvic surgery or endometriosis; exam normal; **referred to Obstetrics & Gynecology** for infertility evaluation |
| 2024-07-09 | outside | cycle-day-3 labs drawn by the gynecology office before the consult (R2): TSH, prolactin, FSH, LH and estradiol, all normal |
| 2024-07-16 | outside | OB/GYN consult letter (received 2024-07-19): 14 months of unprotected intercourse without conception; menses regular every 28 to 30 days; no STI, pelvic surgery or endometriosis; NKDA; family and social history as above; exam normal; partner's semen analysis normal; TSH, prolactin, FSH, LH normal; transvaginal ultrasound in the office: normal uterus and both ovaries with small antral follicles. **Female infertility diagnosed** (working label unexplained, pending HSG, R1). HSG ordered. **Clomiphene citrate 50 mg on cycle days 5 to 9 prescribed** to promote ovulation, to begin once HSG is done (R1, R3) |
| 2024-07-29 | outside | hysterosalpingogram report (R1): normal uterine cavity, both tubes patent with free spill |
| 2024-09-17 | outside | OB/GYN follow-up letter (received 2024-09-20): returns about infertility workup and ovarian reserve testing; cycles regular every 29 days with mild cramps, no abnormal bleeding or pelvic pain; does not smoke or drink, healthy BMI; exam unremarkable; initial labs normal TSH, prolactin, FSH, LH and estradiol; HSG normal; partner's semen analysis normal. Unexplained infertility. Two clomiphene ovulation cycles without conception (R3). **Dysmenorrhea recorded** (mild cramps, no treatment, R4). **Clomiphene citrate challenge testing indication recorded**; challenge test next cycle: **clomiphene 100 mg on cycle days 5 to 9** with day-3 and day-10 FSH (R3) |
| 2024-10-07 | outside | challenge test day-3 draw: FSH 6.8 IU/L, estradiol 38 pg/mL (A3) |
| 2024-10-14 | outside | challenge test day-10 draw: FSH 7.4 IU/L (A3) |
| 2024-10-22 | outside | OB/GYN results letter (A3): ovarian reserve normal; further clomiphene ovulation-induction cycles offered; she prefers to wait and will call the office when ready. No further gynecology visits |
| 2025-04-08 | practice | annual physical (P); Pap (cytology), negative |
| 2026-01-17 | outside | ED record (received 2026-01-20): age 30 (R5); sudden left lower abdominal pain earlier that day, sharp, intermittent, 6/10; LMP began 14 days ago, normal volume and duration; regular 28-day cycle; not currently sexually active; no contraception; no STI history; history of female infertility and prior clomiphene challenge testing; takes no medications; NKDA; no nausea, vomiting, fever or urinary symptoms; vital signs normal; mild LLQ tenderness, no rebound or guarding, no CVA tenderness; pelvic exam unremarkable without cervical motion or adnexal tenderness or masses; urinalysis negative for leukocyte esterase, nitrite and blood; urine pregnancy negative. **Discharge impression: left lower quadrant pain** (R6); discharged home, follow up with primary care |
| 2026-01-23 | practice | post-ED visit (P): the pain settled within two days of the ED visit (A4); exam normal; LLQ pain resolved on the list |
| 2026-03-09 | practice | annual physical (P), age 30; last practice visit. CBC, BMP and lipids drawn that day |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Encounter 0 calls the infertility unexplained and prescribes clomiphene while
  the hysterosalpingogram is only "shown"; encounter 1 says it "is performed".
  Resolution: the July consult records female infertility with an unexplained working
  label and orders the HSG; the HSG is done 2024-07-29 and is normal (cavity normal,
  tubes patent), which is what "unexplained" requires; the September letter confirms
  unexplained infertility. Clomiphene starts after the HSG.
- **R2.** Encounter 0 lists normal TSH, prolactin, FSH and LH; encounter 1 adds
  estradiol to the same initial labs. Resolution: one cycle-day-3 draw on 2024-07-09
  by the gynecology office before the consult included all five, all normal: LH 5.6
  mIU/mL and estradiol 44 pg/mL (A5); TSH, prolactin and FSH values from code within
  normal ranges. The July letter may mention four; the September
  letter lists five.
- **R3.** Clomiphene is prescribed "to promote ovulation" at encounter 0
  (2024-07-16), then encounter 1 (2024-09-17) is about ovarian reserve testing and
  records the challenge-test indication, and the ED says "prior clomiphene citrate
  challenge testing" and "takes no medications". Resolution: two short ovulation
  cycles of clomiphene 50 mg on days 5 to 9 (August and early September 2024) without
  conception; then one challenge cycle of clomiphene 100 mg on days 5 to 9 in October
  2024 (A3 results). Both are short courses; she takes nothing after October 2024.
- **R4.** Dysmenorrhea is a primary diagnosis the source supports only with "mild
  cramps" at encounter 1. Resolution: the September 2024 gynecology letter records
  dysmenorrhea, mild, no treatment; the practice carries it on its list. No drug.
- **R5.** The ED note says age 29; on 2026-01-17 she is 30. Ages follow the birth date.
  The source's "January 2020" initial presentation reads as July 2024; "14 months"
  stays as the source wrote it in the September letter (a copied figure, not updated).
  Cycle length is 28 to 30, 29 and 28 days in three notes; these are the patient's
  own reports and stay as written.
- **R6.** The ED note gives no diagnosis or disposition and says she "presents to the
  clinic". Resolution: an ED visit; discharge impression left lower quadrant pain,
  home with primary-care follow-up. No ED medication or imaging.
- The source's "Clomiphene citrate challenge testing indication" is recorded by the
  practice from the September 2024 letter and marked resolved when the October results
  letter arrives; the ED's copied list may still carry it, as outside lists do.
- **Left as the source has it:** the ED does no pelvic ultrasound and no blood work;
  nobody comments. No prenatal vitamin or folic acid is ever started. Infertility stays
  an active problem with treatment paused; nobody pushes it further.

## Additions (A): physician to confirm

- **A1.** 2022-05-10 new-patient visit (the practice has her about 4 years by today).
- **A2.** 2024-06-11 visit that refers her to Obstetrics & Gynecology.
- **A3.** The challenge test's results (needed because the ED calls it prior testing):
  day-3 FSH 6.8 IU/L and estradiol 38 pg/mL on 2024-10-07, day-10 FSH 7.4 IU/L on
  2024-10-14; normal ovarian reserve; the results letter of 2024-10-22 and her choice
  to wait before more cycles.
- **A4.** At the post-ED visit the pain had settled within two days.
- **A5.** The July 2024 day-3 LH (5.6 mIU/mL) and estradiol (44 pg/mL), tests the
  catalog cannot fill, given as normal day-3 values.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Records from her prior primary-care office (received May 2022): routine care there,
  a normal Pap in 2021, Tdap in 2017, HPV series completed in adolescence.
- Yearly physicals with vitals, PHQ-2, CBC, metabolic panel, lipids; TSH once given her
  mother's hypothyroidism (normal). HIV and hepatitis C screening once (negative).
  Cervical cytology every 3 years (2025, negative).
- Influenza vaccine each fall (2022 to 2025), COVID vaccines each fall.
- Portal messages about results, scheduling, forms; records requests; a cancelled visit.
- No booked visit after today.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00001783 on 2026-03-09. Practice contacts after that day are
only messages and that day's results; nothing is booked.
