# unit-1738 build requirements

Every clinical fact in this chart traces to **S** (the source as carried here), **R**
(a numbered resolution), **A** (a numbered addition) or **P** (the primary-care
layer below), and to nothing else. Texture (voice, scheduling, refills, portal
traffic, admin) is free and carries no clinical fact. Everything is re-voiced: no
source sentence, clinician name, problem-list dump or "(diagnosed ...)" stamp is
copied.

## Who she is (S)

- **Identity (seed, unchanged):** Christine Hartley, MRN00001738, female, born
  1991-07-14, Amherst MA 01002, White, not Hispanic or Latino, English,
  Commercial PPO (Blue Cross Blue Shield), phone +1-413-555-0128,
  christine.hartley@example.test. Single; shares an apartment with a roommate (the
  roommate is the person who brings her to the ED in February 2025).
- **Occupation:** elementary school teacher.
- **Tobacco:** never smoker. **Alcohol:** social, 1-2 drinks per month. No drug use.
- **Allergies:** no known drug allergies.
- **Surgical history:** wisdom teeth extraction at age 18.
- **Family history:** mother Hashimoto thyroiditis; father type 2 diabetes;
  maternal aunt rheumatoid arthritis; sister celiac disease. No other relatives
  are named.
- **Joins the practice 2022-08-17** (A1), age 31, transferring from a prior primary
  care office. She brings:
  - Type 1 diabetes mellitus, long-standing (onset before joining, date not given).
  - Hashimoto hypothyroidism.
  - Insulin glargine (Lantus SoloStar) 18 units subcutaneously at bedtime.
  - Insulin lispro (Humalog KwikPen) with meals, 1 unit per 12 g carbohydrate plus
    a correction scale (R4).
  - Levothyroxine 88 mcg by mouth daily.
  - Alopecia areata is **not** known at joining (R1).

These three drugs and doses run unchanged through the whole chart. The practice
manages her insulin; no outside endocrinologist appears.

## Timeline

| Date | Where | What happens |
|---|---|---|
| 2022-08-17 | Practice, new-patient visit (A1) | Establishes care with T1DM, Hashimoto hypothyroidism, the three home drugs, NKDA, the family and social history above. Baseline labs. |
| 2024-05-20 | Practice office visit (P) | About 6 weeks of patches of hair loss on the scalp (R2). Diabetes follow-up; **Hemoglobin A1c 6.8%** (R3). **Dermatology referral placed.** |
| 2024-07-01 | Outside dermatology consult (S, E0) | 3-month history of several round, well-demarcated, completely bald scalp patches; no pain, itching or burning. Exam: three coin-sized patches without scaling or erythema, exclamation-mark hairs at the margins, no lymphadenopathy, nail pitting, no other skin lesions, otherwise unremarkable. Labs drawn by dermatology: **TSH 2.5 mIU/L** (ref 0.4-4.0), **CBC within normal limits**, **ANA negative**; the letter also quotes the practice's A1c 6.8% (R3). **Alopecia areata diagnosed.** **Clobetasol 0.05% topical solution started** (A7). Referral closed. |
| 2024-10-07 | Outside dermatology follow-up (A7) | Per A7. |
| 2025-01-13 | Outside dermatology follow-up (A7) | Per A7; **clobetasol stopped**. |
| 2025-02-18 to 2025-02-21 | Outside hospital: ED, then ICU, then floor (S, E1) | Brought by her roommate with confusion, vomiting and rapid breathing after several days of a viral upper respiratory infection during which she had not been taking her insulin regularly. Exam: dry mucous membranes, tachypnea with deep respirations, fruity breath odor. **Arterial blood gas: high anion gap metabolic acidosis.** **Urinalysis: ketones positive, glucose positive.** Admission chemistry and course per A2. **Diabetic ketoacidosis diagnosed; acute upper respiratory infection diagnosed.** Discharged on unchanged home insulin and levothyroxine. Age 33 (R6). |
| 2025-02-25 | Practice post-discharge visit (P) | Recovered; URI resolved; DKA resolved; sick-day rules reviewed. |
| 2025-10-29 | Practice telephone, early morning (P) | She calls with leg weakness and trouble urinating; the PCP sends her to the ED. |
| 2025-10-29 to 2025-11-04 | Outside hospital: ED, then neurology inpatient (S, E2; A3) | Two days of progressive leg weakness and difficulty urinating, tingling that began in the toes and has ascended to the umbilicus. A mild respiratory illness about 3 weeks earlier that resolved without treatment; denies other recent infections or vaccinations. Exam: strength 3/5 in both lower extremities, hyperreflexia, bilateral Babinski signs, light touch and pinprick diminished below the umbilicus. Labs: **WBC 7.0 x10^3/uL** (7,000/uL), **ESR 20 mm/h**, **CRP 1 mg/L**, **comprehensive metabolic panel normal**, **aquaporin-4 antibody negative**, **MOG antibody negative**. **MRI spine: T2-hyperintense lesion from T5 to T7, no enhancement with gadolinium.** **Acute transverse myelitis diagnosed.** Admission course per A3. Age 34 (R6). |
| 2025-11-04 to 2025-11-20 | Outside inpatient rehabilitation (A4) | Per A4. |
| 2025-11-24 | Practice post-discharge visit (P) | Reviews hospital and rehab course; outpatient PT referral; influenza vaccine. |
| 2025-12-01 | Outside physical therapy evaluation (A4) | Per A4. |
| 2025-12-16 | Outside neurology follow-up (A5) | Per A5. |
| 2026-03-27 | Outside PT discharge summary (A5) | Per A5. |
| 2026-04-14 | Outside MRI thoracic spine (A5) | Per A5. |
| 2026-04-21 | Outside neurology follow-up (A5) | Per A5. |
| 2026-11-18 | Practice, booked (P) | Diabetes follow-up. |

## Resolutions (R)

1. **Alopecia areata on every list.** The profile lists it as chronic and the
   dermatology note lists it on the active problem list at the visit that
   diagnoses it. It is diagnosed at the 2024-07-01 dermatology consult and appears
   on no list before that.
2. **Hair-loss start.** The dermatology note gives a 3-month history on
   2024-07-01, so the patches began about early April 2024; she mentions them to
   the PCP on 2024-05-20 as about 6 weeks old.
3. **A1c in a dermatology letter.** Dermatology would not draw an A1c. The 6.8% is
   the practice's 2024-05-20 result, quoted in the consult letter. TSH, CBC and ANA
   are dermatology's own draw on 2024-07-01.
4. **"Sliding scale" lispro.** The source calls the mealtime lispro a sliding scale
   with an I:C ratio of 1:12. The chart carries it as carbohydrate counting at 1
   unit per 12 g plus a correction scale.
5. **No alopecia plan in the source.** The source gives no plan. The chart carries
   dermatology's standard first-line treatment and follow-up (A7).
6. **Ages.** The source calls her 32 at the DKA admission and 33 at the myelitis
   ED visit. With the seed birth date she is 33 on 2025-02-18 and 34 on
   2025-10-29; the chart uses those ages. She is 32 at the dermatology visit, as
   the source says.
7. **"Diagnosed earlier this year."** The DKA note says the alopecia was diagnosed
   earlier that year; on the re-dated timeline it was diagnosed the previous July
   (2024-07-01).
8. **Diagnosed stamps.** The myelitis ED note's problem list stamps alopecia and
   DKA with dates; the chart carries prior DKA (February 2025) and alopecia areata
   as history without stamps.
9. **Arterial gas.** The ABG is an inpatient test and stays in the hospital
   record; it never appears at a practice visit.
10. **Sensory level vs lesion.** The exam level (umbilicus) and the MRI lesion
    (T5-T7) are carried as written; nobody comments on the difference.
11. **ICU HPI is cut off mid-sentence.** The chart carries what precedes the break
    and the rest of the course from A2.

**Left as the source has it:** no outside endocrinology; no
continuous glucose monitor or insulin pump; no change in insulin doses after the DKA;
no glucagon prescription; no disease-modifying therapy after the myelitis.

## Additions (A)

1. **New-patient visit 2022-08-17.** The source opens with an established patient
   on insulin and levothyroxine and the seed gives 4 years with the practice.
   Prior-office records follow (2022-09-06): T1DM and Hashimoto on the same drugs,
   childhood vaccines including hepatitis B series complete, normal Pap in 2021.
2. **DKA admission detail** (needed for a coherent discharge summary). Admission
   labs: serum glucose 438 mg/dL, bicarbonate 9 mmol/L, anion gap 25, potassium
   5.3 mmol/L, arterial pH 7.14, beta-hydroxybutyrate 6.4 mmol/L, Hemoglobin A1c
   7.8%; SARS-CoV-2 PCR not detected. Treated with IV fluids, IV insulin infusion
   and potassium replacement in the ICU; anion gap closed overnight; transferred to
   the floor 2025-02-19; transitioned to her home glargine and lispro; tolerating
   diet; discharged 2025-02-21 on unchanged home drugs with sick-day teaching.
3. **Transverse myelitis admission 2025-10-29 to 2025-11-04** (the usual next steps).
   Foley catheter placed in the ED for a post-void residual of 520 mL; removed on
   2025-11-02 and she voided with post-void residuals under 100 mL thereafter.
   MRI brain with and without contrast 2025-10-30: normal, no demyelinating lesions.
   Lumbar puncture 2025-10-30: CSF WBC 11 /uL (lymphocyte predominant), protein
   49 mg/dL, glucose 71 mg/dL, oligoclonal bands absent. IV methylprednisolone 1 g
   daily for 5 days (2025-10-29 to 2025-11-02), no oral taper. Glucose ran high on
   steroids and was covered with inpatient correction insulin; home doses unchanged
   at discharge. Final impression: idiopathic acute transverse myelitis (AQP4 and MOG
   negative, normal brain MRI). At transfer to rehab: strength 4-/5 in both legs,
   walking short distances with a walker.
4. **Inpatient rehabilitation 2025-11-04 to 2025-11-20; outpatient PT.** At rehab
   discharge: hip flexors 4/5, distal leg strength 4+/5 bilaterally, walks about
   150 feet with a rolling walker, voiding independently, numbness of both feet
   persists. Outpatient PT evaluation 2025-12-01: findings as at rehab discharge
   (slow walker gait, needs arms to rise from a chair); twice weekly for 12 weeks.
5. **Recovery course.** Neurology 2025-12-16: walking with a cane, strength 4+/5 in
   the legs, numbness of both feet; repeat thoracic spine MRI planned. Back to
   teaching on 2026-01-12 after the winter break (return-to-work letter with
   temporary limits through February: seat as needed, no recess or bus duty, time
   for PT). PCP 2026-02-25: cane only for longer walks and ice, slight hip flexor
   weakness, feet numb but less so. PT discharge 2026-03-27 (care 2025-12-01 to
   2026-03-24):
   walking without a device, independent home program. MRI thoracic spine with and
   without contrast 2026-04-14: T2 signal at T5-T7 smaller than before, no
   enhancement, no new lesions. Neurology 2026-04-21: strength 5/5 except hip
   flexors 5-/5, mild residual numbness of both feet, gait normal; no further
   treatment; neurology follow-up in 6 months.
6. **Celiac screen.** T1DM with a sister with celiac disease: tissue
   transglutaminase IgA at the 2023 physical, negative.
7. **Alopecia areata treatment** (standard care for a diagnosis the source never
   treats). At the 2024-07-01 consult dermatology starts clobetasol propionate 0.05%
   (0.5 mg/mL) topical solution, a thin layer to each bald patch twice daily, not on
   the face, and warns about skin thinning. Dermatology follow-up 2024-10-07: fine
   new hair in all three patches, no new patches, no skin atrophy or folliculitis;
   continue twice daily; recheck in 3 months. Dermatology follow-up 2025-01-13: all
   three patches filled in with regrown hair, no new patches, no atrophy;
   clobetasol stopped; restart it and call on any new patch; follow-up as needed.
   Practice notes carry the drug from 2024-07-05 to its stop and the regrown scalp
   afterward; alopecia areata stays on problem lists, in remission.

## Primary-care layer (P)

- Diabetes visits about every 3 months (some by video), each with an A1c; A1c
  runs about 6.5-7.8% (7.1 at joining, 6.8% 2024-05-20, 7.8% at the DKA admission,
  near 7 otherwise; higher in late fall and winter). BMP or CMP, lipids, TSH and urine albumin/creatinine yearly, all
  normal. Annual diabetic foot exam normal (monofilament intact at every exam before
  the myelitis; after it the feet carry the residual numbness from A4-A5). Annual
  outside diabetic eye exam letters each spring: no retinopathy.
- Yearly physicals in August before school starts. Weight steady around 62 kg,
  height 165 cm, normal blood pressure.
- Prevention: Pap with HPV co-test 2024 normal; HIV and hepatitis C screening once
  (2023), negative; Tdap and PCV20 at the 2023 physical (diabetes); influenza each
  fall (2025's given 2025-11-24, after the myelitis); COVID updated vaccine at a
  pharmacy fall 2022, 2023 and 2024 (patient report); PHQ-2 at each physical, 0-1.
- Texture: insulin pen prior authorization at the start of plan years, 90-day
  levothyroxine refills, result notes, referral scheduling, a cancelled visit in
  late May 2023 (end of school year) rebooked by video, an RN portal exchange in
  early October 2025 about a mild cold (sick-day reminders; she reports her home
  urine ketone checks are negative), short-term disability and return-to-work
  paperwork after the myelitis.
- No acute illnesses, injuries, new diagnoses or life events beyond the timeline.
