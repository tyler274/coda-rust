# unit-1910 build requirements

Every clinical fact in this chart (diagnosis, medication, dose, result, finding,
procedure, referral outcome, family member, life event) traces to one of: **S** the
source as carried here, **R** a numbered resolution, **A** a numbered addition, or
**P** the primary-care layer below. Nothing else. Texture (voices, scheduling,
portal traffic, admin, missed visits, the unevenness of real notes) is free and
carries no new clinical fact. Re-voice everything: no source sentence, clinician
name, problem-list dump, "(diagnosed ...)" stamp or quiz text reaches the chart.

## Who she is (S)

- Megan Foster, female, born 2001-05-13, Chicopee MA 01020, English, White, not
  Hispanic or Latino. Phone +1-413-555-0192, megan.foster@example.test. Single.
- Insurance: University Student Health Plan.
- Occupation: graduate student.
- Tobacco: never. Alcohol: none. No drug use.
- Allergies: no known drug allergies.
- Surgical history: none. Family history: no significant family history.
- Long-distance runner (the source's "endurance athlete status" and "long-distance
  running"), carried as social history, not as problem-list entries.
- **Joins the practice 2022-09-07** (age 21), healthy, no problems, no medications.

## Timeline

| Date | Where | What happens |
|---|---|---|
| 2022-09-07 | Practice (P) | New-patient visit. Healthy, no problems, no meds, NKDA. Baseline weight 57.5 kg, height 165 cm. Baseline labs normal. HIV and hepatitis C screening, negative. |
| 2022-09-28 | Prior primary care records (P) | Childhood vaccines complete including HPV series and Tdap in 2013; COVID-19 primary series 2021. Nothing else. |
| 2023-09-20 | Practice (P) | Annual physical. First Pap: NILM. Chlamydia/gonorrhea NAAT negative. Tdap given. |
| 2024-09-18 | Practice (P) | Annual physical (2024-09-11 slot cancelled and rebooked). Normal labs. |
| 2025-06-05 | Practice (P) | Right foot pain about one week, began after starting an intensive running program (several miles daily); no trauma; worse with activity, better with rest; OTC ibuprofen with little relief. Tender over the right second metatarsal, no swelling. **Referral to sports medicine placed.** Advised to stop running until seen. |
| 2025-06-11 | Outside orthopedics & sports medicine (S0) | Progressive right foot pain for 2 weeks after starting an intensive training program with several miles of running daily; no trauma; worse with activity, relieved by rest; OTC ibuprofen with minimal relief. PMH unremarkable, no other medications, NKDA, no significant family history, never smoker, no alcohol, graduate student. Exam: tenderness to palpation over the midfoot, no swelling, erythema or warmth; point tenderness most notable at the second metatarsal. Right foot X-ray unremarkable. **Diagnosis: right second metatarsal stress fracture** (R2). Plan per R2. Letter received 2025-06-13; referral done. |
| 2025-07-16 | Outside sports medicine (A1) | Follow-up: no tenderness over the second metatarsal, walking without pain. Cleared for gradual return to running. Letter received 2025-07-18 with the **metatarsal stress fracture resolved** as of the visit. |
| 2025-09-17 | Practice (P) | Annual physical. Foot healed; back to running and training for a marathon. Normal labs. |
| 2025-11-06 | Practice portal (P) | Right shin pain for about two weeks since increasing her mileage for an upcoming marathon. **Referral to sports medicine placed.** |
| 2025-11-12 | Outside orthopedics & sports medicine (S1) | Returns five months after the foot evaluation with right shin pain for 3 weeks, began after increasing running distance for an upcoming marathon; worse with activity, relieved by rest; no trauma or falls. No medications, NKDA. Exam: mild swelling over the distal third of the right tibia, focal tenderness on palpation, no erythema or warmth, normal ankle range of motion. Right tibia X-ray: no acute fracture or dislocation. **Diagnosis: right distal tibial stress fracture** (R2). Plan per R3. Letter received 2025-11-14; referral done. |
| 2025-11-19 | Practice lab (A2) | 25-hydroxy vitamin D 34 ng/mL (normal), after a portal check-in on the second stress fracture. |
| 2025-12-07 (Sun) | Outside ED (S2, R4) | Collapse and altered mental status at a marathon finish line. Known prior right metatarsal and tibial stress fractures. Shivering, disoriented, slurred speech. Friends report she was adequately dressed but struggled in the last few miles. No medications, NKDA. Vitals on arrival: temperature 34.5 °C (94.1 °F), HR 58, BP 100/60, RR 16 (SpO2 per A3). Skin cold to touch, extremities pale, capillary refill 3 seconds; remainder of exam noncontributory. Labs: sodium 140 mEq/L, potassium 4.2 mEq/L, creatinine 0.9 mg/dL, glucose 90 mg/dL, CBC within normal limits. **Diagnosis: accidental hypothermia.** Course per R5/A3. Record received 2025-12-08. |
| 2025-12-10 | Practice (P) | Post-ED visit. Well, no residual symptoms, normal temperature. **Hypothermia resolved.** Tibial stress fracture still under sports medicine. |
| 2026-01-14 | Outside sports medicine (A4) | Follow-up: no tibial tenderness or swelling, pain-free single-leg hop. Cleared for graded walk-run return. Letter received 2026-01-16 with the **tibial stress fracture resolved** as of the visit. |
| 2026-09-16 | Practice (P) | Annual physical, booked (Pap due). |

## Resolutions (R)

1. **ED date.** The source's practice date for the ED visit is 2025-12-10, a
   Wednesday; a marathon finish is a weekend event. The ED visit moves to Sunday
   2025-12-07. The other source dates stay as source.md gives them.
2. **Stress fractures with normal X-rays.** Both orthopedic encounters stop before
   an assessment, and both X-rays are normal. The later source problem lists carry
   "metatarsal stress fracture" from 2025-06-11 and "tibial stress fracture" from
   2025-11-12, so each is a clinical diagnosis made at its encounter despite a
   normal early X-ray (the letters say so). Side and bone follow the exams: right
   second metatarsal; right distal tibia. Metatarsal plan: stiff-soled shoe or
   walking boot as needed for comfort, no running for about six weeks, cross-train
   without impact, acetaminophen rather than ibuprofen for pain as needed (advice,
   no prescription), follow-up in about five weeks.
3. **Tibial plan and the marathon.** The 2025-11-12 plan is unfinished. The chart's
   plan: no running for six to eight weeks, low-impact cross-training, walking boot
   if walking hurts, and advice to withdraw from the upcoming marathon; follow-up
   in about eight weeks. The source then has her finish a marathon four weeks
   later; the chart lets that stand without explanation (she ran it).
4. **ED setting.** ED record from an outside regional hospital, written in that
   ED's voice. The source's comorbidity list is not carried as diagnoses.
5. **ED course (unfinished in source).** Mild accidental hypothermia treated with
   removal of wet clothing, warm blankets, forced-air warming and warmed IV normal
   saline 1 liter; mental status and speech normalized as she rewarmed; discharged
   home with friends the same day. No other tests are added.
6. **Comorbidities.** "Endurance athlete status" and "long-distance running" are
   social history, not conditions.
7. **OTC ibuprofen** before encounter 0 stays narrative only (tried, little
   relief); it is not on the medication list. Medication list stays empty
   throughout.
8. **Problem-list stamps.** The source's "(diagnosed ...)" dates are dropped; the
   conditions start at their encounters as above.

**Left as the source has it:** no MRI or bone scan for either stress fracture; no
bone density testing, no bone-health or nutrition workup beyond A2; no ECG or core
temperature probe at the ED; no CK. Nobody remarks on these.

## Additions (A)

1. **Metatarsal follow-up, 2025-07-16, outside sports medicine.** The fracture
   must heal before the source's next encounter has her training again: no
   tenderness over the right second metatarsal, pain-free walking and hopping,
   cleared for gradual return to running. No imaging.
2. **25-hydroxy vitamin D, 2025-11-19: 34 ng/mL.** Any physician checks it after a
   young runner's second stress fracture. The PCP's portal check-in that orders it
   records regular menstrual cycles and an ordinary diet by patient report.
   Nothing else is ordered.
3. **ED vitals and rewarming.** SpO2 98% room air on arrival. At 2 hours:
   temperature 35.9 °C, HR 66, BP 106/64, RR 16, SpO2 99%. At 4 hours (discharge):
   temperature 36.8 °C, HR 72, BP 112/70, RR 14, SpO2 99%, alert and oriented,
   speech clear. ED length of stay about 4.5 hours.
4. **Tibial follow-up, 2026-01-14, outside sports medicine.** The usual next step:
   no tenderness or swelling over the distal right tibia, pain-free single-leg hop,
   cleared for a graded walk-run return over several weeks. No imaging.

## Primary-care layer (P)

- New-patient visit 2022-09-07; annual physicals in September 2023, 2024 and 2025;
  next one booked 2026-09-16. Visits for the foot (2025-06-05) and after the ED
  (2025-12-10). Weight wobbles around 56–58 kg; blood pressure and other vitals
  normal.
- Labs, all normal: CBC with differential, CMP, lipid panel, TSH, HIV-1/2 antigen
  and antibody, hepatitis C antibody at joining; CBC and BMP in 2024; CBC and CMP in
  2025; chlamydia and gonorrhea NAAT at each physical while under 25, negative.
- Cervical cytology (Pap) alone at 2023-09-20, NILM; next due 2026.
- PHQ-2 at each physical, 0 or 1; PHQ-9 at joining, low; GAD-7 once, low.
- Influenza vaccine each fall (2022 nurse visit; 2023 and 2025 at the physical;
  2024 at a pharmacy after a missed nurse visit). COVID-19 updated vaccines at a
  pharmacy each fall 2022–2025 by patient report. Tdap 2023-09-20.
- Prior primary care records received 2022-09-28 with childhood vaccines
  (HPV completed 2013–2014, Tdap 2013) and COVID-19 primary series 2021, normal.
- Texture: records request, immunization forms for her program, portal results
  notes, scheduling, a cancelled physical slot (2024-09-11), a no-show nurse visit
  for a flu shot (2024-10-09), RN call after the ED record arrives.
- No acute illnesses, injuries, new diagnoses or life events beyond the timeline.


## Last seen

The practice last saw MRN00001910 on 2025-09-17. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. Nothing is booked after today.
