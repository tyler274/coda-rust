# Build requirements: unit-2234 (Robert Dupuis, MRN00002234)

Source: synthetic_hospital patient 2234, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its unfinished sentences.

## Who he is (S)

Robert Dupuis, born 1956-09-17, male, White, not Hispanic or Latino, English, Medicare,
lives in Granby MA 01033 with his wife (S: "His wife reports"). Retired construction
foreman. Current smoker, 1 pack per day for 45 years. Alcohol: occasional social use,
1 to 2 drinks a week. No known drug allergies. Surgical history: cholecystectomy at
age 45 (about 2001); open reduction and internal fixation of a left femoral fracture
at age 62 (about 2018 to 2019). Family history: father myocardial infarction at 72 and
hypertension; mother type 2 diabetes and chronic kidney disease; brother ischemic
stroke at 70.

Joined the practice on **2022-09-13** (A1, new-patient visit; about 4 years with the
practice), coming from a prior primary-care practice. He brought: hypertension, type 2
diabetes, chronic atrial fibrillation, chronic kidney disease stage 3 (3a, R3); and
**warfarin 5 mg daily (INR goal 2.0 to 3.0), metoprolol tartrate 50 mg twice daily,
insulin glargine 20 units subcutaneously at bedtime, amlodipine 5 mg daily** (R1).
The practice manages his INR from joining (P).

Records texture (seed): sparse. Most outside care reaches the practice only as the
patient's account, a short ED note or a brief summary, with few numbers. What reaches
the chart for each outside contact is stated in the timeline.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2022-09-13 | practice | new-patient visit (A1): history and medications as above; NKDA; current smoker counseled; labs and prevention per P; LDCT and AAA screen ordered (A6, P) |
| 2025-03-04 | practice | A2: reports a groin bulge for some months; exam a reducible right inguinal hernia (R4); referred to general surgery |
| 2025-04 | outside | general surgery consult; letter never received; patient reports elective repair set for 2025-07-08 (sparse) |
| 2025-06-24 | practice | A3: pre-operative visit; periprocedural warfarin plan: last dose 2025-07-02, no bridging, resume the evening of surgery if the surgeon agrees; **INR 2.5** (R2); CBC and BMP; ECG atrial fibrillation, rate controlled |
| 2025-07-08 | outside | general surgery, elective right inguinal hernia repair (Tue). In the operating room, before induction, the team asks for an IV antihypertensive bolus; the anesthesiologist pauses on noticing that esmolol and ephedrine vials look alike and sat side by side on the cart. Pre-induction BP 145/90. Baseline INR 2.5 cited. Near miss caught before anything was given (R5). **Risk of medication error due to look-alike drug confusion (LASA) recorded**; **inguinal hernia resolved**. Brief operative and anesthesia summary received 2025-07-10, carrying the BP, the INR and the near miss; discharged home the same day |
| 2025-07-09 | practice | RN call: home, comfortable, warfarin resumed the evening of surgery (A3) |
| 2026-01-06 | outside | ED (Tue): age 69 (R6). HTN, T2DM, chronic AF, CKD 3, recent hernia repair. Wife reports he woke from a nap with sudden right-sided weakness and trouble speaking, well before the nap. BP 160/90, HR 78, irregularly irregular. Alert, anxious; expressive aphasia, right hemiparesis. **INR 3.8**, hemoglobin 13.2, platelets 250,000, creatinine 1.1. Non-contrast head CT: left intracerebral hemorrhage. **Intracerebral hemorrhage and anticoagulation-related hemorrhage diagnosed.** Warfarin reversed (A4); urine drug screen positive for cocaine (A5); admitted. Short ED note received 2026-01-08, carrying the vitals, exam, INR, hemoglobin, creatinine and CT result (platelets not carried, sparse) |
| 2026-01-06 to 2026-01-12 | outside | admission (A5): **warfarin stopped**; CT angiography of the head shows multifocal segmental narrowing of intracranial arteries, read with the drug screen as **cocaine-induced cerebral vasculopathy (diagnosed)**; he acknowledges intermittent cocaine use; deficits improve; discharged home with outpatient physical and speech therapy. One-page discharge summary received 2026-01-13 |
| 2026-01-16 | practice | A7: transitional care visit with his wife; mild word-finding pauses and mild right-hand weakness, improving; BP 134/82; warfarin on hold; cocaine abstinence counseled, he says he has stopped; CBC, BMP |
| 2026-02-17 | outside | stroke neurology follow-up; letter never received; patient reports neurology cleared resuming warfarin (A8) |
| 2026-02-19 | practice | A8: practice confirms with neurology by phone; **warfarin 5 mg daily restarted**; INR in a week |
| 2026-04-14 | practice | A7: follow-up; strength normal, only occasional word-finding pauses; no cocaine since January; anticoagulation-related hemorrhage resolved on the list; hemoglobin 12.5 (R7, the "baseline" of July) |
| 2026-07-05 to 2026-07-12 | outside | admission (Sun): age 69. Chronic AF on warfarin, known cocaine-induced cerebral vasculopathy. **Massive pulmonary embolism diagnosed** (CT pulmonary angiogram, A9); started on IV unfractionated heparin. Day 2 (2026-07-06): sudden hematuria and persistent oozing from IV sites. BP 100/58, pulse 88, RR 18. Diffuse ecchymoses on the arms; mild abdominal tenderness without distension. **Hemoglobin 8.6 (baseline 12.5)**, platelets 220,000, aPTT prolonged, PT normal (R8). **Anticoagulation-related hemorrhage diagnosed again; gross hematuria diagnosed.** Heparin paused, then resumed at a lower target once bleeding settled; no transfusion; warfarin overlapped and continued; discharged on warfarin 5 mg daily, hemoglobin 9.4 at discharge (A9). Brief discharge summary received 2026-07-13 |
| 2026-07-14 | practice | A10: transitional care visit; hemoglobin 10.1, INR 2.3; ecchymoses fading; no visible blood in urine since discharge; **urology referral placed** for the gross hematuria |
| 2026-08-11 | practice | A10: follow-up; hemoglobin 11.6, INR 2.5; urinalysis negative for blood; urology visit booked for 2026-09-17 (open) |
| 2026-09-22 | practice | booked: chronic-care follow-up and annual wellness visit (P) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The first source note says he takes warfarin but its home list omits it; the
  last list has warfarin 5 mg daily, INR goal 2.0 to 3.0. Resolution: warfarin 5 mg
  daily is on his list from joining, apart from the hold around surgery and the stop
  after the hemorrhage (2026-01-06 to 2026-02-19). Metoprolol tartrate 50 mg twice
  daily, insulin glargine 20 units at bedtime and amlodipine 5 mg daily run unchanged
  throughout.
- **R2.** "Baseline INR 2.5" on the day of surgery cannot be the morning value after a
  5-day hold. Resolution: 2.5 is the practice's pre-operative INR of 2025-06-24, which
  the operative summary cites as his baseline. No day-of-surgery INR reaches the chart.
- **R3.** CKD stage 3 with a creatinine of 1.1 gives an eGFR near 70 at 69. Resolution:
  his practice creatinine runs about 1.40 to 1.55 (eGFR about 47 to 52, stage 3a); the
  ED's 1.1 stays as that lab reported it, and no note comments.
- **R4.** The source names only an "elective hernia repair". Resolution: a right
  inguinal hernia, found at the 2025-03-04 visit and repaired 2025-07-08.
- **R5.** The source stops at the anesthesiologist's hesitation. Resolution: the
  look-alike vials were caught before any drug was given; the correct agent was
  confirmed; no harm; the summary records the near miss and the LASA risk.
- **R6.** The source gives age 68 at the ED visit; with the seed's birth date he is 68
  at surgery and 69 at both 2026 events. Notes follow the seed.
- **R7.** Hemoglobin 13.2 in January 2026 and a "baseline 12.5" in July. Both stand:
  practice values drift from about 13.3 at joining to about 12.5 by April 2026.
- **R8.** A PE on listed warfarin with a normal PT at admission. Resolution: the summary
  states the normal PT and the listed warfarin side by side; nobody explains it, and
  the chart does not either (an unexplained gap a physician may notice).
- **Left as the source has it:** no statin in a 69-year-old diabetic smoker (lipids are
  drawn yearly, LDL about 118, and no note raises a statin); no SGLT2 inhibitor or ACE
  inhibitor/ARB for diabetic CKD; warfarin rather than another anticoagulant is never
  discussed; no left atrial appendage or other alternative to anticoagulation is raised
  after the hemorrhages; smoking continues (counseling is P, no cessation drug is
  started).

## Additions (A): physician to confirm

- **A1.** 2022-09-13 new-patient visit (the first source encounter shows an established
  outpatient with a long history and home medications).
- **A2.** 2025-03-04 visit that finds the hernia and refers him (R4).
- **A3.** Pre-operative visit 2025-06-24 with the warfarin hold plan (last dose
  2025-07-02, no bridging) and resumption the evening of 2025-07-08; post-op INR 1.7 on
  2025-07-17, back to 2.3 on 2025-07-29.
- **A4.** ICH on warfarin with INR 3.8: reversal with 4-factor prothrombin complex
  concentrate and IV vitamin K in the ED (inpatient only, never on the practice list).
- **A5.** Evidence for the source's cocaine-induced cerebral vasculopathy, which the
  source asserts with none: urine drug screen positive for cocaine in the ED, CT
  angiography during the admission with multifocal segmental narrowing of intracranial
  arteries, and his acknowledgment of intermittent cocaine use. Admission 2026-01-06 to
  2026-01-12.
- **A6.** Annual low-dose CT lung cancer screening (current smoker, 45 pack-years,
  age 66 to 69): 2022-10-06, 2023-10-10, 2024-10-08, 2025-10-07, each Lung-RADS 1. One
  abdominal ultrasound for AAA screening (male ever-smoker 65 to 75), 2022-10-20, aorta
  normal caliber.
- **A7.** Post-ICH course: transitional care visit 2026-01-16; deficits (word-finding
  pauses, mild right-hand weakness) improve, and by 2026-04-14 only occasional
  word-finding pauses remain; outpatient physical and speech therapy finish by spring
  2026. He reports no cocaine use after January 2026.
- **A8.** Warfarin resumed 2026-02-19 after neurology clearance (the source has him on
  warfarin again in July 2026); INR 1.9 on 2026-02-26, then in range.
- **A9.** PE course: CT pulmonary angiogram confirms it; IV unfractionated heparin
  paused for the bleeding and resumed at a lower aPTT target; bleeding settles; no
  transfusion; warfarin overlapped; discharged 2026-07-12 on warfarin 5 mg daily,
  hemoglobin 9.4.
- **A10.** Post-PE follow-up: hemoglobin 10.1 (2026-07-14) and 11.6 (2026-08-11); INR
  2.3 and 2.5; urinalysis negative for blood on 2026-08-11, when the anticoagulation-related
  hemorrhage is resolved on the list (gross hematuria stays open for urology); urology referral for the
  gross hematuria (standard in a smoker even on anticoagulation), visit booked
  2026-09-17 and still open today.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may
  carry the descriptors its kind always carries, but nothing that adds a territory, a
  severity grade or a symptom, or changes the diagnosis or plan.
- Levels: weight about 94 kg, height 178 cm; office BP about 136/82 on his drugs; heart
  rate in the 70s, irregularly irregular; A1c about 7.4 to 7.8; creatinine as R3;
  urine albumin/creatinine about 18 mg/g; LDL about 118; INR mostly 2.0 to 3.0 with the
  odd value just outside it handled by a dose skip or boost and no lasting dose change.
- Warfarin monitoring by the practice: INR every 4 to 6 weeks, more often around
  surgery and after restarts.
- Diabetes and hypertension visits about every 3 months; Medicare annual wellness visits
  each fall; A1c, BMP or CMP, CBC; lipids and urine albumin/creatinine yearly; foot
  exams; yearly diabetic eye exam letters from an optometrist with no retinopathy;
  PHQ-2 negative; smoking cessation advice at most visits, which he declines.
- Records from the prior practice: colonoscopy 2018 with no polyps (10-year recall),
  Tdap 2017, COVID-19 primary series and booster in 2021.
- Vaccines: influenza (high-dose) each fall, PCV20 at joining, Shingrix two doses
  (2022-12 and 2023-03), updated COVID vaccines each fall, RSV once (2024, pharmacy).
  Hepatitis C antibody nonreactive once (2022).
- Refills, Medicare Part D and pharmacy traffic, portal messages about results, refills
  and scheduling, one cancelled and rescheduled visit, reminders.
- Booked visit after 2026-08-27: 2026-09-22 chronic-care follow-up and annual wellness
  visit.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.
