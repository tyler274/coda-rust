# unit-1790 build requirements

Source: synthetic_hospital patient 1790 (`source.md`). Every clinical fact in the chart
traces to **S** (the source as carried here), **R** (a resolution below), **A** (an
addition below) or **P** (the primary-care layer), and nothing else. Texture is free:
voices, scheduling, refills, portal traffic, admin, missed visits. Re-voice
everything; never copy source sentences, clinician names, problem-list dumps or
"(diagnosed ...)" stamps. Clinicians are named by role only; outside places are
generic ("an outside endocrinology group", "a regional hospital").

## Who she is (S)

- **Identity (seed):** Gretchen Kelleher, female, born 1990-05-22, MRN00001790,
  Holyoke MA 01040, +1-413-555-0178, gretchen.kelleher@example.test, White, not
  Hispanic or Latino, English, Medicaid. Single.
- **Occupation:** graphic designer. **Tobacco:** never smoker. **Alcohol:**
  occasional social use. No other substance use.
- **Allergies:** sulfonamide antibiotics (anaphylaxis); latex (contact dermatitis).
- **Surgical history:** laparoscopic salpingectomy for a ruptured ectopic pregnancy
  (age 26, 2016); dilation and curettage (age 27, 2017). Reason for the D&C is not
  recorded (left as the source has it).
- **Family history:** father epilepsy; mother rheumatoid arthritis and type 2
  diabetes; sister major depressive disorder; maternal aunt breast cancer at 45.
- **Living situation:** lives in Holyoke with a roommate (the roommate of the 2023,
  2024 and 2026 events). After 2021-10 her daughter (R4) lives with her too.

### Joining the practice

She **establishes care 2018-04-17** (A1), moving her primary care from a prior
practice; about 8 years 4 months with the practice by today. She brings:

- **Epilepsy**, childhood onset, generalized seizure disorder, followed by an outside
  neurology group. On **phenytoin 300 mg PO at bedtime** (on phenytoin since about
  2015). Last seizure spring 2017 (R2).
- **HIV infection**, diagnosed 2016, CDC stage 3, followed by an outside infectious
  disease (ID) clinic. On **Biktarvy (bictegravir/emtricitabine/tenofovir alafenamide)
  1 tablet daily**, which ID switched her to in early 2018 (earlier regimen not named).
- **Major depressive disorder, recurrent, severe**, on **sertraline 100 mg daily**,
  managed in primary care.
- **History of methotrexate-induced folate deficiency** (R3), on **folic acid 1 mg
  daily**.
- **Prenatal vitamin with iron 1 tablet daily** (taking it since the ectopic
  pregnancy; she has wanted a pregnancy "someday").

Not known at joining: central diabetes insipidus (starts 2019-05-15, R1);
desmopressin (starts 2019-05-15).

## Timeline

Practice dates from source.md. "Practice" = our office; everything else is outside
care received as a letter or record.

| Date | Where | What happens |
|---|---|---|
| 2018-04-17 | Practice (A1) | **Establish care.** History above. Weight 64.0 kg, height 165 cm. PHQ-9 8. Labs: CBC, CMP, lipid panel, TSH, hepatitis C antibody (negative), hepatitis B surface antigen (negative), hepatitis B surface antibody (negative, non-immune). Records requested. |
| 2018-05-08 | Outside records | Prior PCP records and ID note of 2018-04-10: HIV on Biktarvy, CD4 310 cells/uL, viral load <20 copies/mL (undetectable). Neurology note: epilepsy controlled on phenytoin. |
| 2019-02 | — | Onset of excessive thirst and urination (three months before 2019-05-15). |
| 2019-04-09 | Practice (P) | Visit for two months of thirst and frequent urination, nocturia. Urinalysis dilute (specific gravity 1.003) with no glucose; BMP with sodium at the upper limit of normal; normal glucose and A1c. **Referral to endocrinology.** |
| 2019-05-15 | Outside endocrinology (S, enc 0) | Three months of polyuria and thirst, drinking up to 6 L/day, waking several times nightly to urinate; denies head trauma or surgery. Exam normal. **Serum sodium 148 mEq/L, serum osmolality 300 mOsm/kg, urine osmolality 150 mOsm/kg.** **Diagnosis: central diabetes insipidus** (inadequately controlled at presentation). **Starts desmopressin acetate 0.1 mg PO twice daily** (R1). Letter received 2019-05-20. |
| 2019-05-22 | Practice lab (P) | BMP one week on desmopressin: sodium normal. |
| 2019-08-28 | Practice portal (P) | Patient writes about tiredness and gums that bleed. PCP orders labs. |
| 2019-09-03 | Practice lab (S) | **Hemoglobin 10.2 g/dL, MCV 102 fL, serum folate 3 ng/mL, calcium 9.2 mg/dL, 25-hydroxyvitamin D 30 ng/mL.** B12 normal (P). |
| 2019-09-10 | Practice (S, enc 1) | Follow-up, four months after endocrinology. Fatigue for two months; gums swollen and bleed easily when brushing. Denies diet change, bowel change or weight loss; menses regular. Exam: gums erythematous and hyperplastic. **Diagnosis: methotrexate-induced folate deficiency (anemia)** (R3). She has been missing folic acid doses (R5); folic acid 1 mg daily continued with adherence plan. Recheck in three months. |
| 2019-12-10 | Practice lab (P) | CBC and folate normal. |
| 2020-11-17 | Outside OB/GYN (S, enc 2) | Preconception counseling: wants to start a family, worried about medication effects on a pregnancy. Denies recent illness, medication changes or missed phenytoin doses. Epilepsy well controlled on phenytoin for five years; last seizure more than three years ago. Exam unremarkable. Reviews her most recent labs (the practice's 2020-10-13 draw): CBC normal, liver tests normal, phenytoin level therapeutic. Plan R6. Letter received 2020-11-23. |
| 2021-02-26 | Practice portal (P) | Positive home pregnancy test; LMP 2021-01-22. Urine hCG positive at nurse visit 2021-03-02; **pregnancy recorded**; referred to OB for prenatal care. |
| 2021-07-23 | Outside OB/GYN (S, enc 3) | At 26 weeks: waking 5–6 times nightly to urinate, cannot drink enough to quench thirst. DI had been well controlled on desmopressin until recently (the OB calls it mild). Vitals normal, exam normal. **Serum sodium 140 mEq/L, serum osmolality 280 mOsm/kg, urine specific gravity 1.005, 24-hour urine volume 4.5 L.** **Diagnosis: diabetes insipidus in pregnancy (gestational exacerbation of central DI).** Plan R7. Letter received 2021-07-29. |
| 2021-10-25 | Outside hospital (A2) | Term vaginal delivery of a healthy daughter. Discharge summary received 2021-11-01. |
| 2021-12-07 | Practice (P) | Postpartum visit: thirst and nocturia back to her pre-pregnancy baseline; sodium normal. **Pregnancy and DI-in-pregnancy resolved.** |
| 2023-05-11 | Outside ED → ICU (S, enc 4) | Age 32. Witnessed seizure with fall from a second-story window. Obtunded and postictal on arrival. Roommate: epilepsy since childhood, **recently stopped taking her medications** (all of them, R5). Intubated for airway protection. Over two hours receives 120 mL isotonic saline; urine output 1.4 L. **T 36.5 °C, pulse 108/min, BP 138/90.** Irregular respirations, sluggishly reactive pupils, no purposeful movement to pain, decerebrate posturing, brisk reflexes. **Sodium 151 mEq/L, serum osmolality 302 mOsm/kg, urine specific gravity 1.002, urine osmolality 68 mOsm/kg, glucose 110 mg/dL.** **Head CT: bilateral cerebral contusions and a small intraventricular hemorrhage.** **Diagnoses: traumatic brain injury; medication nonadherence.** Hospital course A3. |
| 2023-05-22 | Outside hospital (A3) | Discharged home; summary received 2023-05-24. |
| 2024-05-22 | Outside ED → admission (S, enc 5) | Age 33. Several days of worsening dull occipital headache, new confusion, trouble keeping attention, increasing lethargy per roommate. Poorly adherent to ART (R5). **Afebrile, BP 110/65, pulse 94, RR 16.** Oriented to person only; no papilledema. **CD4 50 cells/uL.** **Head imaging: four ring-enhancing lesions in the basal ganglia and subcortical white matter** (R8). **Diagnosis: cerebral toxoplasmosis.** Course A4. |
| 2024-06-03 | Outside hospital (A4) | Discharged home; summary received 2024-06-05. |
| 2026-05-25 | Outside ED → psychiatric admission (S, enc 6) | Age 36. Brought by roommate who found a suicide note. Six months of depressed mood, anhedonia, significant weight loss, insomnia, poor concentration; feels worthless and hopeless; wrote the note and wants to die; detailed plan to overdose on the roommate's prescription medications and has already collected the pills. **Stopped sertraline three months ago (about 2026-02-25).** Denies substance use. Vitals stable. Tearful, poor eye contact, low monotone voice; exam otherwise unremarkable. CBC, CMP and thyroid tests normal. **Diagnosis: major depressive disorder, recurrent, severe, with psychotic features** (A5). Course A5. |
| 2026-06-12 | Outside psychiatric unit (A5) | Discharged home; summary received 2026-06-15. |

## Resolutions (R)

1. **Central DI and desmopressin.** The profile lists central DI and desmopressin
   from the first encounter, but encounter 0 is the presentation (three months of
   new polyuria, untreated-looking labs). Central DI is diagnosed and desmopressin
   0.1 mg PO twice daily starts at endocrinology 2019-05-15. Before that she has
   neither. The source's "(inadequately controlled)" describes her state at
   presentation; the chart condition is "Central diabetes insipidus".
2. **Epilepsy history.** The "(diagnosed 2021-07-15)" stamp on epilepsy is a
   copy artifact: epilepsy is childhood onset and known from joining. Phenytoin
   "for the past 5 years" and "last seizure over 3 years ago" (stated 2020-11) give
   phenytoin since about 2015 and a last seizure in spring 2017. The OB note of
   2020-11-17 that lists only "phenytoin and a multivitamin" and leaves phenytoin
   off its list is not carried: her full regimen continues.
3. **Methotrexate-induced folate deficiency.** The profile has it as history from
   the start and encounter 1 diagnoses it. The methotrexate exposure is the medical
   treatment of her 2016 ectopic pregnancy before it ruptured and needed
   salpingectomy; folate deficiency followed and she has taken folic acid since. In
   2019-09 the deficiency recurs (folate 3, macrocytic anemia) while she is missing
   folic acid; the PCP records it under the source's label. No methotrexate after
   2016.
4. **Pregnancy outcome.** The 2021 pregnancy is never concluded in the source; it
   ends as a term delivery (A2). Her daughter is mentioned only where a writer would
   record her (social history, who cares for the child during admissions). No
   pediatric detail.
5. **Medication noncompliance (source comorbidity).** It shows as missed folic acid
   (2019-09), stopping all medications before the 2023-05-11 seizure, inconsistent
   ART adherence from 2022 (viral load detectable 2022-08 and 2023-09) leading to the
   2024 presentation, and stopping sertraline about 2026-02-25. The 2024 ED phrase
   "diagnosed with HIV 3 years ago" is not carried; HIV dates from 2016.
6. **Encounter 2 plan (unfinished).** The OB counsels about phenytoin's fetal risks
   and the importance of folic acid, keeps her regimen unchanged, and asks her to
   review antiseizure and ART plans with neurology and ID before conceiving. No drug
   changes.
7. **Encounter 3.** The OB note's "no known drug allergies" and "no medications
   other than prenatal vitamins" are not carried; allergies and regimen stand. The
   plan is unfinished: the OB informs endocrinology, advises free access to water and
   sodium checks; desmopressin stays 0.1 mg twice daily as every later source list
   shows (left as the source has it). "Mild" DI is the OB's word.
8. **Encounter 5 imaging.** A non-contrast CT cannot show ring enhancement. The ED
   does a non-contrast head CT (four hypodense lesions in the basal ganglia and
   subcortical white matter), then an MRI brain with and without contrast that shows
   the four ring-enhancing lesions. The source's four lesions and locations are
   carried exactly.
9. **Ages and years.** Ages 28 (2019-05), 29 (2020-11), 32 (2023-05), 33 (2024-05)
   follow the birth date. The ICU note's "pregnancy in 2022" is the 2021 pregnancy.
10. **Profile med list in 2026.** The 2026 ED list (sertraline absent, others
    present) is carried: she stopped only sertraline. Prenatal vitamin and folic acid
    continue throughout.

**Left as the source has it:** no pituitary imaging or cause found for central DI;
desmopressin dose never changed, including in pregnancy; gum hyperplasia recorded on
exam without a separate diagnosis, referral or comment on cause; phenytoin continues
before and through pregnancy and alongside Biktarvy without regimen change; no
psychiatry involvement before 2026; D&C reason not stated.

## Additions (A)

1. **Establish-care visit 2018-04-17** and what she brought (above). Encounter 1
   reads as established care.
2. **Delivery 2021-10-25**, term vaginal delivery of a healthy daughter at a
   regional hospital, uncomplicated; DI managed on her usual desmopressin with
   sodium normal at discharge. Postpartum her thirst returns to baseline.
3. **2023 hospital course** (regional hospital, neuroscience ICU): fosphenytoin
   load then phenytoin resumed; **phenytoin level undetectable on arrival**;
   desmopressin given IV then resumed orally, urine output falls and sodium returns to
   normal over about 48 hours; trauma survey otherwise without fractures or other
   injury; repeat head CT at 24 hours stable; extubated hospital day 3; neurology
   and neurosurgery follow (no surgery); discharged home 2023-05-22 on her full home
   regimen. She tells the team she stopped her pills because she was tired of being a
   patient; the inpatient psychiatry liaison finds no suicidal ideation. Driving restriction for six months. Post-discharge PCP visit 2023-05-30
   and neurology follow-up 2023-06-20 with a normal neurological exam.
4. **2024 toxoplasmosis course** (regional hospital medicine service with ID):
   **Toxoplasma IgG positive; HIV-1 viral load 214,000 copies/mL.** Sulfadiazine
   avoided for the sulfonamide anaphylaxis. **Starts pyrimethamine 200 mg once then
   50 mg daily, clindamycin 600 mg PO every 6 hours and leucovorin 10 mg daily**
   (six-week acute course), **atovaquone 1500 mg daily** for Pneumocystis
   prophylaxis, and Biktarvy resumed with adherence support. Orientation improves;
   discharged home 2024-06-03 oriented to person, place and time.
   - ID 2024-07-09: repeat MRI 2024-07-02 shows the four lesions smaller with less
     enhancement; acute course complete; **changes to maintenance pyrimethamine 25 mg
     daily, clindamycin 600 mg every 8 hours, leucovorin 10 mg daily**.
   - ID 2025-09-09: CD4 above 200 for over three months; **stops atovaquone**.
   - ID 2026-01-13: CD4 above 200 for over six months with suppressed virus; **stops
     pyrimethamine, clindamycin and leucovorin**.
   - HIV values at ID visits (all A): 2018-04-10 CD4 310, VL <20; 2019-01-22 CD4 342,
     VL <20; 2020-01-21 CD4 388, VL <20; 2021-06-08 CD4 401, VL <20; 2022-08-09 CD4
     276, VL 1,240 copies/mL; 2023-09-12 CD4 214, VL 8,900 copies/mL (ID moves Biktarvy to bedtime with
     phenytoin); 2024-07-09 CD4
     88, VL 640 copies/mL; 2024-11-12 CD4 148, VL <20; 2025-05-13 CD4 228, VL <20;
     2025-09-09 CD4 251, VL <20; 2026-01-13 CD4 274, VL <20; 2026-07-14 CD4 281, VL
     <20. (CD4 in cells/uL; VL <20 = not detected.)
5. **2026 psychiatric course.** The psychotic-features diagnosis needs evidence: the
   psychiatric evaluation finds a **fixed, mood-congruent belief that her illnesses
   are a punishment she deserves**, without hallucinations. Voluntary admission to an
   inpatient psychiatric unit 2026-05-26; the collected pills are removed by the
   roommate and disposed of. **Sertraline restarted at 50 mg and raised to 100 mg daily
   on day 5; olanzapine started at 5 mg at bedtime and raised to 10 mg on day 4.**
   Sleep and appetite improve in the second week, the guilt belief fades, suicidal
   ideation resolves by day 12. Discharged 2026-06-12 with
   a safety plan and outpatient psychiatry. Outpatient psychiatry 2026-08-11: mood
   improving, the guilt belief has faded, no suicidal ideation; regimen unchanged.
   Weight 53 kg at the ED (down from 61.5 kg in 2025-06), 53.5 kg 2026-06-16,
   55.5 kg 2026-07-21.

## Primary-care layer (P)

- **Visits:** chronic follow-up every 4–8 months for depression, epilepsy, DI and
  medication adherence, with ID, neurology and endocrinology letters between;
  physicals 2018-05-29, 2019-06-11, 2020-10-13, 2022-06-14, 2023-10-17, 2025-06-10;
  post-discharge visits within a week or two of each discharge.
- **Labs:** BMP/sodium every 6–12 months on desmopressin (normal); CBC, CMP and
  lipids yearly (normal); phenytoin levels at least yearly (therapeutic) except the
  2023 arrival level; folate/CBC rechecks after 2019; CBC during pyrimethamine
  (every 2 weeks in the acute course, monthly to 2024-10, then about every 3 months;
  normal); fasting lipids and A1c after olanzapine (2026-06-16,
  normal). HIV labs are drawn by ID.
- **Depression screening (PHQ-9):** 2018-04-17 8; 2018-11-13 7; 2019-06-11 6;
  2019-09-10 7; 2020-05-12 9; 2020-10-13 6; 2021-12-07 9; 2022-06-14 10; 2022-11-15
  12; 2023-05-30 10; 2023-10-17 9; 2024-01-23 11; 2024-06-11 12; 2025-01-14 9;
  2025-06-10 8; 2026-06-16 17; 2026-07-21 11. A behavioral health (therapy) referral
  in 2022-11: telehealth therapy from 2023-01, tapering off by spring 2025; weekly therapy
  again after the 2026 admission.
- **Prevention:** cervical cytology yearly ×3 (2018-05-29, 2019-06-11, 2020-10-13,
  all NILM) then cytology with HPV co-test 2023-10-17 (NILM, HPV negative), next due
  2026-10. PCV13 2018-05-29, PPSV23 2018-07-27 and PCV20 2023-10-17; MenACWY 2018-05-29 and 2018-07-27;
  hepatitis B (Heplisav-B) 2018-05-29 and 2018-06-29; influenza every fall; COVID-19
  mRNA primary series 2021-04, booster 2021-12, bivalent 2022-10, updated 2023-10,
  2024-10, 2025-10; Tdap at 28 weeks 2021-08-20 (OB); Shingrix 2022-06-14 and
  2022-08-16. Hepatitis C antibody once (2018, negative). No breast or colorectal
  screening (under 40/45).
- **Texture:** refills and pharmacy calls (phenytoin, desmopressin, Biktarvy), Medicaid prior authorizations for pyrimethamine (2024, renewed 2024-12 and 2025-04) and
  for olanzapine (2026), records requests, a cancelled visit (2025-12-09) and a no-show
  (2026-03-17), portal messages about results and scheduling, driving and work notes.
- **Today:** psychiatry follow-up arranged; a booked PCP visit **2026-09-15** for
  depression follow-up and medication review. Nothing else open.
- No acute illnesses, injuries, new diagnoses or life events beyond the timeline.
