# Build requirements: unit-1837 (Jasmine Norris, MRN00001837)

Source: synthetic_hospital patient 1837, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its unfinished plan text.

## Who she is (S)

Jasmine Norris, born 1996-10-14, female, White, not Hispanic or Latino, English,
Blue Cross Blue Shield PPO, lives in Easthampton (01027). High school mathematics
teacher. Never smoker, never drinks alcohol. No known drug allergies. No surgical
history. Family history: father with severe hypertriglyceridemia, paternal
grandfather with acute pancreatitis, mother with type 2 diabetes. Married.

Joined the practice on 2024-08-13 (A1, a new-patient visit). She brought two
problems: **recurrent urinary tract infections** (several cystitis episodes a year
at her prior practice, each treated with a short antibiotic course; no prophylaxis)
and **severe hypertriglyceridemia** known since her early twenties, on **fenofibrate
145 mg daily**.

Records texture (seed): sparse. Outside care reaches the practice as the patient's
own account, a short ED or triage note, a brief discharge summary and pharmacy fill
history, with few numbers. Each outside contact below says what arrives.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2024-08-13 | practice | new-patient visit (A1); history as above; fenofibrate 145 mg continued; labs drawn (P), lipid panel on fenofibrate TG 410, HDL 34 (A2) |
| 2024-08-26 | outside records | prior primary care records arrive by fax: recurrent cystitis, hypertriglyceridemia on fenofibrate, Pap 2023 normal, HPV series complete (P) |
| 2025-02-03 | practice | evening phone call: several days of fever, chills, right flank pain, burning and frequency; told to go to the ED tonight (P) |
| 2025-02-04 to 2025-02-08 | outside | ED then admission (S encounter 0, moved, R1). Five days of fever, chills and right flank pain with dysuria and frequency; no nausea, vomiting or diarrhea. T 38.9 °C (102 °F), HR 110, BP 130/85. Right CVA tenderness. WBC 15,000/mm³, Hgb 13.2 g/dL, platelets 250,000/mm³. Urinalysis: pyuria, leukocyte esterase positive, nitrite positive. Blood cultures no growth to date; urine culture pending. Urine pregnancy test negative (A3). **Acute pyelonephritis**. Admitted on **IV ceftriaxone**; at 72 hours still febrile with unchanged flank pain (R2), renal ultrasound normal (A3); afebrile on day 4; discharged 2025-02-08 on **ciprofloxacin 500 mg twice daily** to finish 10 days in all (A3). What arrives: a short ED physician record (the numbers above) and, from the pharmacy, the ciprofloxacin fill. The ultrasound and the rest of the stay reach the chart only as the patient tells it. The urine culture result never arrives |
| 2025-02-12 | practice | post-hospital visit (P): afebrile, flank pain nearly gone, finishing ciprofloxacin; CBC, BMP, urinalysis normal. Pyelonephritis resolved |
| 2025-04-03 | practice | portal message: positive home pregnancy test, last period 2025-02-27. **Pregnancy** recorded; **fenofibrate stopped** (S: she stopped it on confirming pregnancy); obstetrics referral (P) |
| 2025-08-19 | practice | annual physical (P) at about 24 weeks; well; prenatal care with an outside obstetrics group |
| 2025-09-25 to 2025-09-27 | outside | obstetric triage at the regional hospital (S encounter 1), 30 weeks gestation: two days of fever, chills and constant right flank pain radiating to the lower abdomen; nausea, no vomiting. T 38.6 °C (101.5 °F), BP 120/78, HR 110, fetal heart rate 160. Right CVA tenderness. Urinalysis: pyuria, bacteriuria, hematuria. CBC: leukocytosis with a left shift (no count given). **Acute pyelonephritis in pregnancy**. Admitted, **IV ceftriaxone** (A4), discharged 2025-09-27 on **cephalexin 500 mg four times daily** to finish 14 days in all (A4). What arrives: a short obstetric triage note (the findings above, no CBC numbers) and the patient's portal message |
| 2025-10-02 | practice | telephone follow-up (P): afebrile, taking cephalexin, flank pain gone. Pyelonephritis in pregnancy resolved at the next visit with a normal urinalysis |
| 2025-11-29 | outside | spontaneous vaginal delivery at 39 weeks of a healthy infant (A5); uncomplicated. Known from the patient's portal message 2025-12-03. Pregnancy resolved. Fenofibrate not restarted (R3) |
| 2025-12-30 to 2026-02-12 | outside | hospital admission (S encounter 2, R5, A6), 4½ weeks postpartum. Presentation: 24 hours of severe constant epigastric pain radiating to the back, worsening; no nausea, vomiting or fever; vital signs stable; abdomen soft, tender in the epigastrium without rebound or guarding; normal bowel sounds; no jaundice or scleral icterus. Hgb 12.5 g/dL, WBC 11,000/mm³, platelets 250,000/mm³, amylase 450 U/L, lipase 600 U/L, triglycerides 1,200 mg/dL, calcium 9.2 mg/dL, liver tests normal. Abdominal ultrasound: no gallstones, no biliary ductal dilatation. Hypertriglyceridemia-induced acute pancreatitis. Course as A6: insulin infusion, necrotizing pancreatitis on CT, **infected pancreatic necrosis** with **septic shock** in the ICU, endoscopic drainage and necrosectomy. Discharged 2026-02-12 with **fenofibrate 145 mg daily restarted** (A6). What arrives: a brief discharge summary (2026-02-13) with the numbers above and in A6 |
| 2026-02-17 | practice | post-discharge visit (P, A7): tired, eating small low-fat meals, weight down; TG 690 on restarted fenofibrate; PHQ-9 normal |
| 2026-03-11 | outside | GI follow-up (A6): CT 2026-03-04 shows the collection nearly resolved, 2.1 cm residual; stent removed endoscopically. Known from the patient's portal message 2026-03-12 |
| 2026-04-14 | practice | follow-up (P, A7): recovered, back at work; TG 455. Infected pancreatic necrosis resolved; septic shock is history |
| 2026-08-11 | practice | annual physical (P, A7): well; TG 372; Pap normal |
| 2026-11-10 | practice | booked follow-up for triglycerides (P) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Encounter 0 (practice date 2025-06-03) is a non-pregnant pyelonephritis in
  a woman taking fenofibrate, which she stopped on confirming pregnancy (encounter 2),
  yet encounter 1 puts her at 30 weeks on 2025-09-25, which makes her about 14 weeks
  pregnant on 2025-06-03. Resolution: encounter 0 moves to 2025-02-04, before the
  last menstrual period of 2025-02-27 (30 weeks 0 days on 2025-09-25, due date
  2025-12-04). Encounter 1's "treated for pyelonephritis in January" becomes
  February 2025. Her stated age of 28 holds at both encounters.
- **R2.** Encounter 0 says fever persisted "despite 72 hours of antibiotic therapy",
  then that she was admitted and started on IV ceftriaxone. Resolution: one course,
  read in order of the stay. She is admitted from the ED on IV ceftriaxone, is still
  febrile with unchanged flank pain at 72 hours, has a renal ultrasound (A3), and
  improves after it. There was no outpatient antibiotic before the ED.
- **R3.** Encounter 1's history says she is "on fenofibrate therapy" and encounter
  2 lists fenofibrate as a home medication, while encounter 2's history says she
  stopped it on confirming pregnancy. Resolution: fenofibrate stops on 2025-04-03
  and is not taken again until the hospital restarts it on 2026-02-12. The plan at
  delivery was to recheck lipids and restart it at the six-week postpartum mark; the
  pancreatitis came first. Outside notes (the obstetric triage note, the discharge
  summary's admission list) may copy fenofibrate as a home drug; the practice's own
  list does not carry it between April 2025 and February 2026.
- **R4.** Encounter 2 lists pregnancy as a comorbidity in a patient who, on the
  re-dated timeline, delivered on 2025-11-29 (A5). Resolution: she is 4½ weeks
  postpartum on 2025-12-30. The discharge summary says postpartum, not pregnant.
- **R5.** Encounter 2 describes a patient with stable vitals, no fever and a soft
  abdomen, yet its setting is the ICU and its diagnosis is infected pancreatic
  necrosis, with "history of septic shock" among its comorbidities. Infected
  necrosis does not exist on the day a pancreatitis presents. Resolution: the
  encounter 2 findings are the admission presentation on 2025-12-30. Necrosis,
  infection of the necrosis and septic shock develop during the same admission (A6);
  the ICU stay is that episode. After discharge, septic shock is history.
- **R6.** Encounter 2's assessment and plan stop mid-sentence. Resolution: the
  chart gives the hospital course in A6 instead.
- **R7.** The profile lists recurrent UTIs and severe hypertriglyceridemia from the
  start, and its diagnosis list carries both pyelonephritis episodes and the
  pancreatitis at once. Resolution: recurrent UTIs and hypertriglyceridemia are known
  at joining; each pyelonephritis and the pancreatitis begins at its own encounter
  and resolves afterwards.
- **R8.** Encounter 0's urine culture is pending and its blood cultures show no
  growth to date. Resolution: the chart carries them as the ED record states them;
  no final culture result ever reaches the practice.
- **Left as the source has it:** nobody checks triglycerides during the pregnancy or
  after delivery, and fenofibrate is not restarted before the pancreatitis (R3). No
  one starts antibiotic prophylaxis for the recurrent UTIs, orders a urine culture
  test of cure, or refers her to urology. No note comments on any of these.

## Additions (A): physician to confirm

- **A1.** 2024-08-13 new-patient visit. The source opens with an established
  patient with chronic problems and a home drug; the seed puts her with the practice
  about two years.
- **A2.** Triglycerides on fenofibrate before pregnancy: TG 410 mg/dL and HDL 34
  mg/dL at joining (2024-08-13); a recheck in December 2024 runs 350 to 480. LDL is
  reported as not calculated when TG is above 400. These give "severe
  hypertriglyceridemia" a treated baseline.
- **A3.** Encounter 0's hospital course (R2): urine pregnancy test negative in the
  ED; renal ultrasound on hospital day 3 with normal kidneys, no hydronephrosis, no
  perinephric collection; afebrile from day 4; discharged on oral ciprofloxacin 500 mg
  twice daily to complete 10 days of antibiotics. The source starts IV ceftriaxone
  and must stop it somewhere.
- **A4.** Encounter 1's treatment, which the source does not give: admission, IV
  ceftriaxone, discharged on cephalexin 500 mg four times daily to finish 14 days.
  Standard care for pyelonephritis at 30 weeks.
- **A5.** Delivery, because a 30-week pregnancy in September ends before encounter 2:
  spontaneous vaginal delivery at 39 weeks on 2025-11-29 at the regional hospital, a
  healthy infant, no complications. Nothing more about the infant enters the chart.
- **A6.** Course of the 2025-12-30 admission, which the source gives only as its
  final diagnosis and comorbidity (R5, R6):
  - insulin and dextrose infusion for TG 1,200; TG 430 mg/dL on hospital day 3, infusion
    stopped; IV fluids, analgesia, low-fat diet later;
  - CT abdomen and pelvis with contrast on 2026-01-03 for persisting pain:
    necrotizing pancreatitis, about 30% of the pancreatic parenchyma (body) not
    enhancing, with an acute necrotic collection in the lesser sac;
  - 2026-01-20 (week 3): fever and hypotension, septic shock, transferred to the
    ICU, norepinephrine for 3 days, IV meropenem; CT that day: walled-off necrotic
    collection, 9 cm, with gas within it: infected pancreatic necrosis;
  - 2026-01-23 endoscopic ultrasound-guided transgastric drainage with a
    lumen-apposing metal stent; one endoscopic necrosectomy on 2026-01-30; out of
    the ICU 2026-01-25; meropenem 3 weeks in all, finished in hospital;
  - discharged 2026-02-12 on fenofibrate 145 mg daily (restart) and a low-fat diet,
    with GI follow-up;
  - GI follow-up: CT 2026-03-04, collection nearly resolved, 2.1 cm residual; stent
    removed by endoscopy 2026-03-11.
  - Nothing else: no diabetes, no pancreatic enzyme replacement, no surgery.
- **A7.** Triglycerides after the restart, on fenofibrate 145 mg and a low-fat diet:
  TG 690 mg/dL on 2026-02-17, 455 on 2026-04-14, 372 on 2026-08-11. HDL stays in the
  30s. Fenofibrate dose unchanged.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may
  carry the descriptors its kind always carries, but nothing that adds a territory, a
  severity grade or a symptom, or changes the diagnosis or plan.
- Records from a prior practice may mention routine screening done there, with normal
  results (Pap 2023 normal, HPV series complete in adolescence).
- Height 165 cm. Weight about 63 to 64 kg before pregnancy, about 70 kg at 24 weeks,
  about 57 kg after the February 2026 discharge, back to about 62 kg by August 2026.
  Blood pressure, heart rate and oxygen saturation normal at practice visits.
- Labs with normal results: CMP, CBC, TSH, hemoglobin A1c, urinalysis; HIV and
  hepatitis C screening once (2024). Lipid panels as A2 and A7 give.
- Prevention: influenza vaccine each fall (2024, 2025 while pregnant); COVID vaccine
  fall 2024; Pap (cytology) in August 2026, normal; PHQ-9 at physicals and after
  the February 2026 discharge, all below 10.
- Refills of fenofibrate, pharmacy traffic (including an auto-refill request
  declined while fenofibrate was stopped), portal messages about results and
  scheduling, a leave form for her school after the long admission and a return-to-work
  letter for April 1, 2026 (reduced load the first week), two cancelled visits
  (March 2025; the January 2026 postpartum lipid visit, cancelled by the admission),
  records faxed to the OB office on request.
- Booked visit after 2026-08-27: a triglyceride follow-up on 2026-11-10.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.


## Last seen

The practice last saw MRN00001837 on 2026-04-14. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. A visit is booked after today.
