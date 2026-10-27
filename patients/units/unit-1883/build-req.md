# Build requirements: unit-1883 (Amy Hale, MRN00001883)

Source: synthetic_hospital patient 1883, in `source.md` beside this file. This file and
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

Born 1993-06-23, female, White, not Hispanic or Latino, English, commercial PPO,
elementary school teacher, married (husband), never smoker, no alcohol ever, no drugs.
No known drug allergies. No surgical history. Family history: mother with autoimmune
(Hashimoto) thyroiditis, maternal aunt with type 1 diabetes, paternal grandmother with
rheumatoid arthritis, younger sister with autoimmune adrenal insufficiency (diagnosed
late 2024, first reported January 2025, R4). No family history of early menopause.

Joined the practice 2023-08-23 (A1), moving her care from a prior primary-care office.
She brought no chronic problems and no medications. Height 165 cm.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2023-08-23 | practice | new-patient visit (A1): healthy; has felt more tired than usual since early summer, put down to the end of the school year; weight 60.0 kg; routine labs normal; HIV and hepatitis C screening negative |
| 2023-11-20 | practice | (A2) fatigue for about 5 months, low mood, unintentional weight loss (57.0 kg), craving salt, skin darker including unexposed areas, periods now every 21 days; BP 102/68; PHQ-9 8; early-morning labs ordered |
| 2023-11-21 | practice draw | (A2) sodium 131, potassium 5.3, AM cortisol 2.9 mcg/dL; urgent endocrinology referral by phone 2023-11-22 |
| 2023-12-06 | outside | endocrinology consult: 6 months of fatigue, depressed mood and unintentional weight loss; periods every 21 days; salt craving; darkening skin including unexposed areas. BP 100/70, pulse 88; thin; hyperpigmentation over knuckles, elbows and knees. Sodium 130, potassium 5.5, chloride 95, bicarbonate 22, BUN 18, creatinine 0.9, fasting glucose 70, ACTH elevated, cortisol low. **Autoimmune primary adrenal insufficiency diagnosed** (R1); 21-hydroxylase antibodies positive (A3). **Hydrocortisone 15 mg every morning and 10 mg every evening, fludrocortisone 0.1 mg daily started**; **emergency hydrocortisone injection kit** and sick-day rules (A3). Letter reaches the practice late (2024-01-10); the practice first hears from the patient by portal message 2023-12-08 |
| 2024-01-17 | practice | follow-up (A4): energy and appetite better; weight 58.5 kg; BP 108/70; sodium 137, potassium 4.4; adrenal insufficiency and her endocrine medications added to the practice lists today |
| 2024-10-16 | practice | annual physical (A5): no period since about April 2024; urine pregnancy test negative; TSH normal; **referred to gynecology** |
| 2025-01-27 | outside | gynecology visit: menses absent 9 months, previously regular, stopped without lifestyle, stress or medication change; occasional night sweats and mild vaginal dryness for a few months; no weight change, excess exercise or dieting; sexually active with her husband, no contraception, unable to conceive. BMI 23; normal secondary sexual characteristics; no galactorrhea; visual fields full; normal vulva and cervix, vaginal mucosa slightly pale. Pregnancy test negative; FSH elevated, estradiol low (qualitative only; the chart never receives numbers). **Primary ovarian insufficiency diagnosed** (R1); **estradiol 1 mg daily and medroxyprogesterone 10 mg on days 1 to 12 of each month started**. Fertility discussed; no referral made. Reports younger sister recently diagnosed with autoimmune adrenal insufficiency. The visit note does not reach the practice until 2025-04-22, after a records request |
| 2025-01-30 | practice | same-day visit (R3, A6): acute pain, swelling and redness of the right first MTP joint since the night before; afebrile; uric acid 7.4 mg/dL. **Acute gouty arthritis diagnosed**; **colchicine short course**. She reports the gynecology diagnosis and new hormone pills from her bottles; POI, estradiol and medroxyprogesterone recorded today |
| 2025-02-06 | practice (portal) | toe back to normal; gout resolved; no urate-lowering therapy |
| 2025-10-08 | practice | annual physical (P): well; Pap and HPV co-test normal; labs normal |
| 2026-03-28 | outside | ED visit (Saturday): worsening fatigue, lightheaded on getting out of bed, more salt craving, unintentional weight loss despite usual diet; ROS negative for fever, cough, chest pain, GI symptoms. Temp 36.7 C; BP 96/62 supine, 80/54 standing; pulse 84 supine, 102 standing; increased pigmentation of palmar creases and buccal mucosa. Sodium 129, potassium 5.4, glucose 66. Home doses unchanged before the visit. Treated and discharged (R5, A7) |
| 2026-04-01 | practice | post-ED visit (A8): better, still tired; weight 59.2 kg; BP 104/68 seated; sodium 133, potassium 5.2; endocrinology appointment confirmed |
| 2026-04-21 | outside | endocrinology follow-up (A9): plasma renin activity elevated; **fludrocortisone increased to 0.15 mg daily**; hydrocortisone unchanged. The practice learns of the change from a pharmacy refill request 2026-05-11; the letter arrives 2026-06-03 |
| 2026-07-10 | practice | follow-up, last seen (A10): feels well; weight 61.5 kg; BP 112/72; sodium 138, potassium 4.3; **practice list updated to fludrocortisone 0.15 mg daily** |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The profile and every encounter list both chronic conditions and all four
  drugs from the first encounter, yet encounter 0 is the work-up of the adrenal
  insufficiency and encounter 1 the work-up of the ovarian insufficiency. Resolution:
  adrenal insufficiency is diagnosed, and hydrocortisone and fludrocortisone start, on
  2023-12-06; ovarian insufficiency is diagnosed, and estradiol and medroxyprogesterone
  start, on 2025-01-27. The source's "Autoimmune primary adrenal insufficiency (Addison
  disease)" and "Primary adrenal insufficiency" are one disease and one problem-list
  entry. Its shortened cycles (every 21 days) in late 2023 are the early course of the
  ovarian failure and no one names them as such at the time.
- **R2.** The gynecology note says she was evaluated for fatigue "a year ago" with no
  definitive diagnosis, and names a "January 2020" endocrinology visit. Resolution: that
  visit is the 2023-12-06 consult, which made the diagnosis; the "no diagnosis" sentence
  is not carried. Encounter 0's "denies any use of medications" is true: she took none
  before the diagnosis.
- **R3.** Acute gouty arthritis carries the gynecology visit's date as its diagnosis
  date, but that visit says nothing about joints. Resolution: a single flare is
  diagnosed at a practice same-day visit three days later (A6) and resolves within a
  week. Later outside letters (the 2026 ED record) list it as a past problem; it is not
  active on the practice list after February 2025.
- **R4.** Family history: encounters 0 and 2 give mother, maternal aunt and paternal
  grandmother; encounter 1 gives only a younger sister with autoimmune adrenal
  insufficiency. Resolution: all four are true. The sister is known to the practice
  from January 2025; earlier notes do not mention her.
- **R5.** The ED note gives no treatment or plan. Resolution: intravenous normal
  saline 2 L and hydrocortisone 100 mg IV, oral juice and a meal for the glucose of 66;
  lightheadedness settled and she was discharged after about 6 hours with instructions
  to double her hydrocortisone for 3 days and to see endocrinology within a week and
  primary care within days. No admission. No repeat ED numbers are carried.
- **R6.** Weight: "thin" in December 2023, BMI 23 in January 2025, weight loss again in
  March 2026. Resolution: height 165 cm; 60.0 kg (Aug 2023), 57.0 kg (Nov 2023), about
  56.5 kg at the endocrinology visit, 58.5 kg (Jan 2024), about 62 kg (Oct 2024), 62.6 kg (BMI 23, Jan 2025),
  62 to 63 kg through 2025, 59.2 kg (Apr 2026), 61.5 kg (Jul 2026).
- **Left as the source has it:** no bone density scan, karyotype or FMR1 testing after
  the ovarian insufficiency diagnosis; no fertility referral; no antidepressant for the
  low mood of late 2023 (it lifts with replacement). No note remarks on these.

## Additions (A): physician to confirm

- **A1.** 2023-08-23 new-patient visit (three years with the practice before the first
  source encounter's referral). Prior-office records arrive 2023-09-06: normal Pap 2022,
  Tdap 2021, HPV series in adolescence.
- **A2.** The November 2023 practice visit and early-morning draw that sent her to
  endocrinology: sodium 131, potassium 5.3, AM cortisol 2.9 mcg/dL; PHQ-9 8.
- **A3.** At the endocrinology consult: 21-hydroxylase antibodies positive (supports the
  source's "autoimmune"); hydrocortisone sodium succinate 100 mg emergency injection kit;
  sick-day dosing and a medical alert ID advised.
- **A4.** 2024-01-17 follow-up after the diagnosis: sodium 137, potassium 4.4.
- **A5.** 2024-10-16 physical where she reports no period since April; negative urine
  pregnancy test, normal TSH; the gynecology referral that leads to encounter 1.
- **A6.** 2025-01-30 gout flare findings (right first MTP, uric acid 7.4 mg/dL),
  colchicine 1.2 mg then 0.6 mg an hour later, then 0.6 mg twice daily until settled.
- **A7.** ED treatment as in R5.
- **A8.** 2026-04-01 post-ED visit: sodium 133, potassium 5.2.
- **A9.** 2026-04-21 endocrinology follow-up (the usual next step after the ED visit):
  renin elevated, fludrocortisone raised to 0.15 mg daily.
- **A10.** 2026-07-10 visit: sodium 138, potassium 4.3. Routine endocrinology
  follow-ups in mid-2024 and mid-2025, and a gynecology follow-up in July 2025, happen
  outside; no letters are received and she
  reports unchanged doses.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Visits for the adrenal insufficiency about every 6 to 12 months with a basic metabolic
  panel; yearly physicals (18 to 39) with CBC, metabolic panel, TSH; lipids once.
- Prevention: Pap with HPV co-test in 2025 (normal); influenza vaccine each fall and
  updated COVID vaccine (some at a community pharmacy); depression screens normal after
  2024.
- Refills (hydrocortisone, fludrocortisone, the hormone pills, the emergency kit),
  pharmacy and prior-authorization traffic, portal messages about results, refills,
  sick-day questions and scheduling, a cancelled visit, records requests.
- Booked visit after 2026-08-27: annual physical 2026-10-14.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Records

Chart texture is "gaps": the December 2023 endocrinology letter arrives five weeks late;
the January 2025 gynecology note arrives three months late and only after a request,
with qualitative FSH and estradiol; the 2024 and 2025 endocrinology follow-ups never send
letters; the ED record arrives by fax a few days after the visit; the April 2026
endocrinology change reaches the practice first as a pharmacy request.

## Last seen

The practice last saw MRN00001883 on 2026-07-10. Practice contacts after that day are
only messages, refills and records; a visit is booked after today.
