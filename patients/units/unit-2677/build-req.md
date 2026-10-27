# Build requirements: unit-2677 (Gavin Michaud, MRN00002677)

Source: synthetic_hospital patient 2677, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or any quiz text.

## Who he is (S)

Gavin Michaud, born 1998-04-30, male, White, not Hispanic or Latino, English, private
insurance, graduate student, single, lives with a roommate. Never smoker, no alcohol, no
illicit drugs. No known drug allergies. Surgical history: tonsillectomy at age 8 (2006).
Family history: mother with bipolar disorder and autoimmune (chronic lymphocytic)
thyroiditis (R3); father with hypertension.

Bipolar I disorder, a long history: diagnosed in 2016, followed by an outside outpatient
psychiatry practice, on **lithium carbonate 600 mg by mouth twice daily** (prescribed by
psychiatry, two 300 mg capsules twice daily), euthymic at joining.

Joined the practice 2021-08-18 (A1, new-patient visit), bringing bipolar I disorder and
lithium. Divalproex is not on any list at joining (R1).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2021-08-18 | practice | new-patient visit (A1): history as above; vitals and exam normal; weight 76 kg, height 178 cm; P labs |
| 2021–2024 | practice | P layer: yearly physicals with lithium monitoring labs, all normal (see P) |
| 2025-01-30 | practice (last seen) | about 3 months of increasing thirst and urination; on lithium 600 mg twice daily only (R1); vitals normal; exam unremarkable; serum sodium 148 mEq/L, urine specific gravity 1.001, fasting glucose 97 mg/dL; lithium level 0.8 mmol/L (A2); the same fasting basic panel carries glucose 97 and chloride 110 mEq/L (A2). Same-day result note: **lithium-associated nephrogenic diabetes insipidus diagnosed** (R2); lithium continued; drink to thirst; nephrology referral placed; psychiatry contacted |
| 2025-02-12 | outside | psychiatry reply letter (A3): mood stable for years on lithium; prefers to continue lithium 600 mg twice daily, no change |
| 2025-03-12 | outside | nephrology consult (A4): letter reaches the practice late, 2025-06-11 |
| 2026-07 (about 07-08 to 07-15) | outside | psychiatric hospitalization for a manic episode (S, from the ED history); **divalproex sodium ER 1000 mg daily started about 2026-07-15** (R1). No record of this stay ever reaches the practice |
| 2026-07-29 | outside | ED (S): brought by his roommate for acute confusion, lethargy and vomiting; more tired over the past few days; one episode of slurred speech this morning; started a new mood medication two weeks ago after a manic hospitalization; age 28 (R4); denies alcohol and drugs; T 37.4 °C, BP 112/76, pulse 92, RR 14; mild scleral icterus and asterixis; AST 388 U/L, ALT 421 U/L, total bilirubin 3.4 mg/dL, ammonia 98 µmol/L (15–45), INR 2.1. Additional ED results (A5). **Toxic liver disease** (valproate-induced liver injury with hyperammonemia). Admitted (A5) |
| 2026-07-29 to 2026-08-03 | outside | hospital admission (A5); discharge summary and ED record received 2026-08-20 |
| 2026-08-10 to 2026-08-25 | practice | portal message from the patient, records request, records reviewed, attempts to schedule a post-hospital visit; nothing booked (P) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The profile and both encounter medication lists carry divalproex 1000 mg daily,
  yet the ED history says a new mood medication was started two weeks earlier after a
  manic hospitalization, and the liver injury with hyperammonemia that follows is the
  classic valproate picture. Resolution: divalproex sodium extended-release 1000 mg
  daily starts at that psychiatric stay, about 2026-07-15, and was never taken before.
  The 2021–2025 practice notes list lithium only. Because this falls after the last
  practice visit, divalproex never enters the practice's medication list; it appears in
  the hospital records and the patient's message only.
- **R2.** The ED list stamps nephrogenic diabetes insipidus as diagnosed at the 2025
  visit, and the 2025 visit gives the evidence (polyuria, sodium 148, specific gravity
  1.001, normal fasting glucose) on lithium. Resolution: the PCP records it as
  lithium-associated nephrogenic diabetes insipidus in a same-day result note after the
  labs return that afternoon; nephrology confirms it (A4).
- **R3.** Encounter 0 gives the mother "Hashimoto thyroiditis"; the profile and the ED
  give mother with bipolar disorder and father with hypertension. Both are carried: the
  same mother has bipolar disorder and autoimmune thyroiditis. The chart calls it
  autoimmune or chronic lymphocytic thyroiditis.
- **R4.** The ED history says 27 years old; from the seed's birth date he is 28 on
  2026-07-29. The chart says 28.
- **R5.** The profile lists "Bipolar Disorder" as a comorbidity and "Bipolar I disorder"
  as chronic: one condition, bipolar I disorder, carried from joining.
- **R6.** The source gives the ED findings and stops. Its disposition and course are A5.
- **Left as the source has it:** lithium is continued at 600 mg twice daily after the
  nephrogenic diabetes insipidus diagnosis and after the hospitalization; no thiazide or
  amiloride is started; no note argues the choice. The practice never sees him after
  2025-01-30, so the post-hospital liver tests the discharge summary asks for are not
  done by today.

## Additions (A): physician to confirm

- **A1.** 2021-08-18 new-patient visit (years with the practice; the 2025 visit reads as
  an established patient).
- **A2.** Lithium level 0.8 mmol/L drawn with the 2025-01-30 labs (any physician checks
  it in a lithium patient with polyuria); the fasting glucose 97 is the basic panel's
  glucose, and its chloride is 110 so the panel is internally consistent with sodium 148.
- **A3.** Outpatient psychiatry reply, 2025-02-12, as in the timeline.
- **A4.** Nephrology consult 2025-03-12 (the referral from 2025-01-30): persistent thirst
  and polyuria, drinks 4 to 5 liters a day; exam normal; serum sodium 146 mEq/L,
  creatinine 0.86 mg/dL, chloride 108 mEq/L, serum osmolality 302 mOsm/kg, urine osmolality 104 mOsm/kg,
  and 116 mOsm/kg two hours after desmopressin 2 mcg subcutaneously: no meaningful
  concentrating response. Lithium-associated nephrogenic diabetes insipidus confirmed.
  Plan: free access to water, avoid dehydration, lithium continued by agreement with
  psychiatry, amiloride discussed and deferred while symptoms are tolerable, sodium
  with his routine labs, nephrology as needed. The letter arrives 2025-06-11 after the
  practice asks for it.
- **A5.** 2026-07-29 ED results beyond the source: lithium level 0.9 mmol/L, valproic
  acid level 88 µg/mL, prothrombin time 23.8 s (with the INR 2.1), direct bilirubin 2.1 mg/dL, acetaminophen undetectable, CT head without contrast normal.
  Admitted 2026-07-29 to medicine with hepatology and psychiatry consults: divalproex
  stopped; IV levocarnitine and lactulose; lithium held the first day, then resumed at
  600 mg twice daily on psychiatry's advice; hepatitis A IgM, hepatitis B surface
  antigen, hepatitis B core IgM and hepatitis C antibody negative (2026-07-30); abdominal
  ultrasound 2026-07-30 normal liver size and echotexture, patent portal and hepatic
  veins on duplex, normal gallbladder and bile ducts, no ascites. Mental status clear by
  the second day. Discharged 2026-08-03 on lithium 600 mg twice daily alone: ammonia 36
  µmol/L, AST 104 U/L, ALT 168 U/L, total bilirubin 1.5 mg/dL, direct bilirubin 0.7 mg/dL, prothrombin time 15.1 s, INR 1.3. Discharge
  diagnosis: valproate-induced liver injury with hyperammonemic encephalopathy;
  valproate recorded by the hospital as an adverse drug reaction (do not rechallenge).
  Follow-up asked for: hepatic panel and INR with the PCP in 1 to 2 weeks, outpatient
  psychiatry within a week. Both records reach the practice 2026-08-20 after a records
  request on 2026-08-11.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may
  carry the descriptors its kind always carries, but nothing that adds a territory, a
  severity grade or a symptom, or changes the diagnosis or plan.
- Records from his prior student health service may mention routine care there with
  normal results: Tdap 2019; COVID-19 primary series (Pfizer) April 2021.
- Yearly preventive visits 2022, 2023 (one missed and rescheduled), 2024, with vitals,
  PHQ-9 screens in the low range (2 to 4), and lithium monitoring labs: basic metabolic
  panel (sodium, creatinine and calcium normal), TSH normal (level about 1.8 mIU/L), CBC normal, and lithium
  levels 0.7 mmol/L (2021), 0.8 (2022), 0.7 (2023), 0.8 (2024). Lipid panel once, 2021,
  normal. Hepatitis C and HIV screening once, 2021, negative. Weight steady 75 to 78 kg.
- Influenza vaccine each fall 2021–2024 at the practice and 2025 at a pharmacy (record
  received); COVID-19 vaccines fall 2022, 2023, 2024.
- One outpatient psychiatry summary letter in 2021 in answer to the records request:
  bipolar I since 2016, lithium since then, euthymic, no recent admissions.
- Refills (lithium is prescribed by psychiatry; one bridge request redirected), forms,
  portal messages about results and scheduling, a missed physical in 2023 and one in
  October 2025, reminders.
- Records texture (seed: gaps): the psychiatric hospitalization summary never arrives;
  the nephrology letter comes three months late; the practice learns of the July 2026
  hospital stay from the patient's portal message before any record. The practice's
  lists stay as they were on 2025-01-30.
- No visit is booked after 2026-08-27.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002677 on 2025-01-30. Practice contacts after that day do not
happen; outside care after it still does, and its records are filed as received without
changing the practice's lists. Nothing is booked after today.
