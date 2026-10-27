# Build requirements: unit-2139 (Norman Hale, MRN00002139)

Source: synthetic_hospital patient 2139, in `source.md` beside this file. This file and
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
"(diagnosed ...)" stamps, or its truncated sentences.

## Who he is (S)

Norman Hale, born 1967-11-28, male, White, not Hispanic or Latino, English, commercial
PPO, lives in Hatfield (01038). Civil service administrator. Former smoker, 25
pack-years, quit in 2022 (three years before the 2025 admission). Alcohol: 1 to 2 beers
on weekends. Allergies: **penicillin (anaphylaxis)**, **shellfish (pruritus/itching)**.
Surgical history: laparoscopic cholecystectomy at age 45 (about 2013); cardiac
catheterization by femoral access complicated by a retroperitoneal hematoma (R2);
mechanical aortic valve replacement (R1, R2). Family history: father MI at 62 and
hypertension; mother atrial fibrillation and type 2 diabetes; older brother coronary
artery disease.

**Joins the practice 2024-09-18** (new-patient visit, A1). Records are sparse: the
cardiology, cardiac surgery and anticoagulation care before and after joining reach the
practice mostly as his own account. He brings:

- Problems: atrial fibrillation (R3), coronary artery disease, essential hypertension,
  hyperlipidemia, GERD, presence of a mechanical aortic valve (placed 2024-06), history
  of retroperitoneal hematoma after cardiac catheterization (2024-05), long-term
  anticoagulant use.
- Drugs: warfarin, variable daily dose, target INR 2.0 to 3.0, dosed by an outside
  cardiology anticoagulation clinic; metoprolol succinate 100 mg daily; aspirin 81 mg
  daily; atorvastatin 40 mg daily; lisinopril 20 mg daily; pantoprazole 40 mg daily.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2024-05 | outside, before joining | coronary angiography by femoral access before valve surgery, complicated by a retroperitoneal hematoma (R2). Known only from his account at joining |
| 2024-06 | outside, before joining | mechanical aortic valve replacement (R1); warfarin started then, target INR 2.0 to 3.0. Known only from his account |
| 2024-09-18 | practice | new-patient visit (A1); history and drugs as above; labs, prevention; lung screening ordered (A2) |
| 2025-02-17 | practice | office visit: about a week of loose watery stools after a stomach illness, eating little. **Ciprofloxacin 500 mg twice daily for 10 days started** (R4); told to let the anticoagulation clinic know. Diarrhea recorded |
| 2025-02-26 to 2025-03-01 | outside | admission to internal medicine at a regional hospital: worsening shortness of breath and fatigue; 2 weeks of poor appetite from the diarrheal illness; on the antibiotic from his primary physician. Exam: multiple ecchymoses on the arms and legs, scattered petechiae on the oral mucosa. **PT 29 seconds, PTT 48 seconds, platelets 210,000/mm3.** **Coagulopathy due to vitamin K deficiency diagnosed.** Warfarin held, low-dose oral vitamin K given, **ciprofloxacin stopped**, warfarin resumed before discharge with the anticoagulation clinic following (A3). The practice receives a short discharge summary 2025-03-03 |
| 2025-03-05 | practice | post-discharge visit (P): bruises fading, appetite back, stools normal; INR back in range; coagulopathy and diarrhea resolved |
| 2025-11-07 | outside | ED visit: 3 days of fatigue and dark urine; denies trauma, medication changes or infection. T 37.2 C (99 F), BP 130/80, pulse 90, RR 16. Skin and sclerae anicteric; mechanical valve click, no murmur, rub or gallop; abdomen normal. Urinalysis red/brown, heme positive, no RBCs on microscopy. **Hemoglobin 9.2 g/dL, hematocrit 27.8%, LDH elevated, haptoglobin decreased, direct Coombs negative, platelets 250,000/uL, INR 3.5.** ED impression: Coombs-negative hemolysis in a patient with a mechanical valve (R5); one warfarin dose held, INR recheck with the anticoagulation clinic, cardiology follow-up. The practice receives a short ED note 2025-11-10 |
| 2025-11-12 | practice | post-ED visit (P): urine lighter, still tired; **hemolytic anemia due to the mechanical valve** on the list from the ED impression (R5); CBC and LDH repeated; cardiology echo already arranged |
| 2025-12 | outside | cardiology transthoracic echo and visit (A4): known only from his portal message: mechanical aortic valve working normally, no leak around it, mild hemolysis from the valve itself; **folic acid 1 mg daily started** by cardiology |
| 2026-01-21 | practice | follow-up: folic acid added to the list as an outside start; CBC improving |
| 2026-04-24 | practice | last visit (seed `last_seen`): routine follow-up, labs. No practice visit after it |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Encounter 0 calls the prosthesis a mechanical **mitral** valve placed 8 months
  earlier; encounter 1 calls it a mechanical **aortic** valve placed 2 years earlier.
  Resolution: one valve, a mechanical aortic valve replacement in 2024-06 (8 months
  before the February 2025 admission). The aortic site fits the INR target of 2.0 to 3.0
  the source gives twice. "2 years ago" is not carried (it is about 17 months by the ED
  visit). The source gives no indication for the surgery, and the chart names none.
- **R2.** The source lists the catheterization with its retroperitoneal hematoma only in
  surgical history, with no date. Resolution: it was the coronary angiogram before the
  valve surgery, 2024-05, before he joined. Recorded as history at joining. No
  transfusion or other detail is carried.
- **R3.** The source calls the atrial fibrillation "nonvalvular" while he has a mechanical
  valve, which by current definition makes it no longer nonvalvular. Resolution: the AF
  predates the valve; the practice lists it as atrial fibrillation (unspecified type).
  Outside notes may keep the old "nonvalvular" label, as outside lists do. Warfarin
  covers both indications; no DOAC is ever raised.
- **R4.** The source says only "broad-spectrum antibiotics prescribed by his primary
  physician" for a diarrheal illness. Resolution: ciprofloxacin 500 mg twice daily for 10
  days, started 2025-02-17 at a practice visit (penicillin anaphylaxis). No stool testing.
  The interaction with warfarin is not discussed by anyone, and no INR is drawn before the
  admission. The ordering decision goes unexplained.
- **R5.** The ED encounter gives the findings of hemolysis but names no diagnosis.
  Resolution: the ED impression and the practice name it hemolytic anemia related to the
  mechanical valve (nonautoimmune hemolytic anemia, D59.4), the form the source's
  findings take. No transfusion, no admission.
- **R6.** Encounter 1's medication list leaves out warfarin and aspirin while its prose
  says he takes both. Resolution: he stays on both throughout.
- **R7.** The admission note gives PT and PTT but no INR. Resolution: the discharge summary
  the practice receives carries PT 29 s, PTT 48 s and platelets 210,000 only; no INR value
  from the admission appears anywhere.
- **Left as the source has it:** the INR target of 2.0 to 3.0 for a mechanical valve with
  AF stays as written and nobody comments on it. Aspirin continues with warfarin, and
  nobody revisits it after either bleeding event.

## Additions (A): physician to confirm

- **A1.** New-patient visit 2024-09-18 (encounter 0 has him on antibiotics from his
  primary physician, so he joined before it; seed gives about 2 years with the practice).
- **A2.** Lung cancer screening (25 pack-years, quit 2022, age 56 to 58): low-dose CT
  2024-10-08 and 2025-10-07, both Lung-RADS 1, negative.
- **A3.** Admission treatment: warfarin held, oral vitamin K 2.5 mg once, ciprofloxacin
  stopped, warfarin resumed before discharge 2025-03-01. Any physician would do this for
  the source's coagulopathy.
- **A4.** Cardiology work-up of the hemolysis (the usual next step): transthoracic echo
  in 2025-12 showing a normally functioning mechanical aortic prosthesis with no
  paravalvular leak; mild valve-related hemolysis; folic acid 1 mg daily started. Known to
  the practice only from the patient.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level here says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  The heart exam carries a crisp mechanical valve click after joining. An abnormal finding
  appears only as the source or this file gives it.
- Prior-practice records (received 2024-09) may mention routine screening: colonoscopy at
  50 (2018), normal, repeat in 10 years; Tdap 2019.
- Visits about every 3 to 6 months for blood pressure, CAD and anticoagulation; one
  annual physical (2025-09-24). Height 178 cm, weight about 92 kg, steady. Blood pressure
  controlled, about 126/78 on lisinopril and metoprolol. LDL in the low 80s on
  atorvastatin 40 mg. Kidney and liver tests normal.
- INR is drawn by the practice only now and then: about 2.2 to 2.4 at joining (code),
  then 2.6 on 2025-03-05, 2.7 on 2025-11-12 and 2.5 on 2026-04-24. Anticoagulation clinic values otherwise reach the chart only as his
  report ("in range"), without numbers.
- After the ED visit, hemoglobin 9.6 on 2025-11-12 with LDH 412 U/L, 10.6
  with LDH 318 U/L on 2026-01-21, and 11.1 with LDH 296 U/L on 2026-04-24
  (reference up to about 250); platelets normal. Hemoglobin before the ED visit is his
  normal.
- Prevention: influenza each fall (2024, 2025), COVID updated vaccine each fall at a
  retail pharmacy, PCV20 once, Shingrix series (2024-12 and 2025-06), hepatitis C, HIV
  and hepatitis B screening once (negative), PHQ-2/PHQ-9 negative.
- Refills, pharmacy traffic, portal messages about results, refills and scheduling, one
  no-show (2025-08-20).
- Booked visit after 2026-08-27: a routine follow-up on 2026-09-14.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002139 on 2026-04-24. Practice contacts after that day are
only messages, refills and results of that day's draw; outside records after it are
filed as received without changing the practice's lists. A visit is booked after today.
