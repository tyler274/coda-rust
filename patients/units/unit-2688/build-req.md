# Build requirements: unit-2688 (August Curran, MRN00002688)

Source: synthetic_hospital patient 2688, in `source.md` beside this file. This file and
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

## Who he is (S)

August Curran, born 2002-03-08, male, White, not Hispanic or Latino, English, lives
in Amherst (01002), private insurance through his employer. Construction worker.
Never smoker. Alcohol: social, 2 to 3 drinks a week. No known drug allergies. No
surgical history. No past medical history before the timeline.

Family history: father with hypertension, and (elicited only on further questioning
on 2025-09-08) a hospitalization for pancreatitis about five years before that visit,
so around 2020. Mother with migraine. No known inherited metabolic disorders.

Joined the practice 2024-09-17 (A1, a new-patient visit), age 22, healthy, taking two
over-the-counter supplements on his own: zinc gluconate 50 mg daily and a
multivitamin, one tablet daily. Problem list empty on joining.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2024-09-05 | outside | records from his prior practice arrive (P): pediatric/adolescent care, routine, no problems, childhood vaccines complete |
| 2024-09-17 | practice | new-patient visit (A1): healthy, no complaints; history as above (family history at this visit is father hypertension, mother migraine); exam normal; height 180 cm, weight 82 kg; BP normal; HIV and hepatitis C screens (P, nonreactive), CBC and CMP (P, normal). Zinc and multivitamin recorded as patient-reported |
| 2024-10-22 | practice | nurse visit, influenza vaccine (P) |
| 2025-08-28 | practice | annual physical (P): well, exam normal; fasting draw for lipid panel, CMP and CBC (P), and a **serum zinc** because he takes 50 mg a day (A2) |
| 2025-09-02 | practice | results released; portal note asks him to come in to go over them (P). Values: total cholesterol 198, LDL 88, HDL 42, triglycerides 486 mg/dL (R2); serum zinc 162 mcg/dL (ref 60 to 120) (A2); CMP and CBC normal; the lab's refrigerated-plasma check on the fasting sample shows no chylomicron layer (S) |
| 2025-09-08 | practice | follow-up to review the lab report (S, encounter 0). New on further questioning: father hospitalized for pancreatitis about five years ago. Exam unremarkable: no xanthomas, no hepatosplenomegaly. Total cholesterol, LDL and HDL normal; triglycerides elevated; no chylomicrons on refrigerated plasma. **Hyperzincemia diagnosed** (R1); **hypertriglyceridemia diagnosed** (R2). Advised to **stop the zinc supplement** (R3); multivitamin continues. Repeat zinc in about 3 months; fasting lipids at next year's physical. No triglyceride drug (see Left as the source has it) |
| 2025-10-16 | practice | nurse visit, influenza vaccine (P) |
| 2025-12-02 | practice | lab only: repeat serum zinc 94 mcg/dL, off the supplement (A3). **Hyperzincemia resolved** |
| 2026-03-12 | outside | ED visit (S, encounter 1), age 24: three months of recurrent, severe, sharp upper abdominal pain radiating to the back with nausea and vomiting; earlier episodes settled at home (R4). Epigastric tenderness without rebound or guarding. Triglycerides 2000 mg/dL, amylase 2000 U/L, lipase 1800 U/L; CBC and CMP otherwise normal (P). Right upper quadrant ultrasound: no gallstones, no biliary tract obstruction. **Acute pancreatitis** due to hypertriglyceridemia (R5). The ED's medication list still carries the zinc and its problem list carries hyperzincemia (R3). Admitted (A4). The practice receives the ED record and the ultrasound report the next day |
| 2026-03-12 to 2026-03-15 | outside | hospital admission (A4): bowel rest, IV lactated Ringer's, IV antiemetic and IV analgesia as needed, diet advanced to low fat by day 3; triglycerides 640 mg/dL on the day of discharge; **fenofibrate 145 mg daily started**; told to stop alcohol and keep a low-fat diet; PCP follow-up within a week. Discharge summary received 2026-03-16 |
| 2026-03-16 | practice | RN transition call after discharge (P): feeling better, eating, has the fenofibrate, visit booked |
| 2026-03-19 | practice | post-discharge visit (P): pain gone, eating a low-fat diet, no alcohol since discharge (A5); epigastrium soft, nontender. Lipid panel and CMP drawn: triglycerides 380 mg/dL, total cholesterol 192, HDL 40, LDL 86 (A6), CMP normal. **Acute pancreatitis resolved** (episode over). Return-to-work letter for the following Monday (P). Continue fenofibrate |
| 2026-05-18 | practice | lab only (P): lipid panel and hepatic panel and creatinine for fenofibrate: triglycerides 260 mg/dL, total cholesterol 180, HDL 44, LDL 84 (A6); liver tests and creatinine normal |
| 2026-05-21 | practice | telehealth follow-up (P): feels well, no abdominal pain, still not drinking, taking fenofibrate daily; plan fasting lipids at the physical |
| 2026-06 to 2026-08 | practice | fenofibrate refill (P); portal scheduling of the physical (P) |
| 2026-09-14 | practice | booked annual physical (P), after today |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source's labelled diagnosis is hyperzincemia, but it gives no zinc level
  and its labs are all about lipids. Resolution: hyperzincemia rests on a serum zinc
  drawn at the August 2025 physical (A2) in a man taking a daily 50 mg zinc
  supplement, and is diagnosed at the 2025-09-08 review of that report. It resolves
  once the supplement stops (A3).
- **R2.** Encounter 0 reports "elevated triglycerides" without a number, and the
  source never names the lipid diagnosis even though its pancreatitis visit turns on
  a triglyceride of 2000. Resolution: the August 2025 draw gives total cholesterol
  198, LDL 88, HDL 42 (all normal) and triglycerides 486 mg/dL, and hypertriglyceridemia
  is diagnosed at the same 2025-09-08 visit. No other source number is changed.
- **R3.** The source keeps zinc gluconate 50 mg daily on the ED's home list after
  hyperzincemia was diagnosed. Resolution: the PCP tells him to stop it on
  2025-09-08 and the practice list ends it there; the ED record and discharge
  summary copy a stale outside list that still names zinc and an active
  hyperzincemia. The discharge summary may list it as "not taking".
- **R4.** The ED history says he had similar past episodes "that required
  hospitalization", but he had no past medical history in September 2025 and the
  practice holds no record of any admission. Resolution: no earlier pancreatitis
  admission. The episodes of the prior three months were similar but milder and
  settled at home without care; this ED visit is his first presentation.
- **R5.** The source names no diagnosis for encounter 1. Resolution: its findings
  (typical pain, lipase 1800, triglycerides 2000, no gallstones) make the ED and
  inpatient diagnosis acute pancreatitis due to hypertriglyceridemia, without
  necrosis or infection.
- **R6.** Encounter 0's family history (father hospitalized for pancreatitis) and
  encounter 1's (father hypertension, mother migraine, no inherited metabolic
  disorders) are both kept: the father has hypertension and had the pancreatitis
  admission. The pancreatitis history first appears on 2025-09-08.
- **Left as the source has it:** no triglyceride-lowering drug and no repeat lipid
  panel between the September 2025 diagnosis and the ED visit. No one comments on
  it. No genetic or lipoprotein workup is done after the pancreatitis.

## Additions (A): physician to confirm

- **A1.** 2024-09-17 new-patient visit. Encounter 0 is a follow-up to review a lab
  report, so he was already a patient; the seed gives about two years with the
  practice.
- **A2.** The August 2025 physical's serum zinc, 162 mcg/dL (reference 60 to 120),
  the evidence the source's hyperzincemia lacks, drawn with the fasting lipid panel
  whose report encounter 0 reviews.
- **A3.** Repeat serum zinc on 2025-12-02: 94 mcg/dL off the supplement.
  Hyperzincemia resolved.
- **A4.** Admission 2026-03-12 to 2026-03-15 for the ED's pancreatitis: IV fluids,
  IV ondansetron and IV hydromorphone begun in the ED; on the ward bowel rest,
  IV lactated Ringer's, ondansetron and hydromorphone as needed (weaned by day 2), low-fat diet by day
  3, triglycerides 640 mg/dL on discharge day, fenofibrate 145 mg daily started,
  alcohol abstinence and low-fat diet advised. Standard care after
  hypertriglyceridemia-induced pancreatitis; the drug continues to today.
- **A5.** He stops drinking after discharge, as advised; notes from 2026-03-19 on
  record no alcohol since.
- **A6.** Triglycerides on fenofibrate and diet: 380 mg/dL on 2026-03-19 and 260
  mg/dL on 2026-05-18, with normal liver tests and creatinine. The rest of those
  lipid panels stays in range: 2026-03-19 total cholesterol 192, HDL 40, LDL 86;
  2026-05-18 total cholesterol 180, HDL 44, LDL 84.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention
  (liver, spleen, kidneys, common bile duct caliber, gallbladder wall). An abnormal
  finding appears only as the source or this file gives it. A report may carry the
  descriptors its kind always carries (a pancreas partly obscured by bowel gas, how a
  duct was measured), but nothing that adds a territory, a severity grade or a
  symptom, or changes the diagnosis or plan.
- Records from a prior practice may mention routine care done there, with normal results.
- Height 180 cm; weight about 82 kg throughout; blood pressure normal throughout.
- Yearly physical; visits after the pancreatitis as above; the labs above.
- Prevention for a man in his twenties: influenza vaccine each fall (he may decline a
  COVID vaccine), HIV and hepatitis C screening once (nonreactive), PHQ-2 at the
  establish visit and the physical (negative).
- Refills and pharmacy traffic for fenofibrate, portal messages about results and
  scheduling, one physical cancelled and rebooked (2025-08-12 to 2025-08-28), a
  return-to-work letter, an RN call after discharge.
- Booked visit after 2026-08-27: annual physical on 2026-09-14.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.
