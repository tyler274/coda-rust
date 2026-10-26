# Build requirements: unit-2013 (Gary Kowalczyk, MRN00001101)

Source: synthetic_hospital patient 2013, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz text ("You recall a recent study...").

## Who he is (S)

Born 1961-05-11, male, White, commercial PPO, office administrator, never smoker,
no alcohol. Family history: father MI at 65, mother type 2 diabetes, a sibling with
hypertension. Joined the practice in 2019 (first practice visit 2019-03-12, a new
patient visit; type 2 diabetes about 14 years by then, on metformin 1000 mg twice
daily and atorvastatin 20 mg daily).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2019-03-12 | practice | new patient visit (A1: establishes care; history as above; BP elevated, recheck) |
| 2020-01-13 | practice | HTN follow-up; occasional headaches for a month; office BP 158/96; BMI 32; exam without retinopathy or heart-failure signs; Cr 1.0, BUN 14, fasting glucose 146, TC 190, HDL 40, LDL 100, TG 250, UA without protein or blood. White-coat effect suspected (R1): home BP log asked for, no drug change |
| 2020-04-14 | practice | follow-up; office BP 132/84; BMI 31 (R2); asks about aspirin for primary prevention and starts aspirin 81 mg after a discussion of small benefit and bleeding risk (no trial figures); home log averages above goal, so **lisinopril 20 mg daily is started today** (R1) |
| 2020-06-02 | outside | urgent care chest x-ray for cough, normal (S: "obtained 1 week ago at an urgent care visit"); the practice receives the report |
| 2020-06-09 | practice | dry cough for 2 weeks, began about 6 weeks after starting lisinopril; no fever, sputum, dyspnea or wheeze; SpO2 98%; BP 138/84; lungs clear. ACE-inhibitor cough: lisinopril stopped, **ACE inhibitor intolerance (cough) recorded** (R3), **amlodipine 5 mg daily started** (R4) |
| 2021-07-21 | outside | endocrinology consult letter (practice referred him in spring 2021 when A1c rose): numbness in both feet for a few months without pain or tingling; dry skin, metatarsal calluses, reduced vibration and monofilament sensation both feet; pedal pulses palpable but diminished; A1c 8.2, fasting glucose 172, Cr 1.1, TC 210, LDL 130, HDL 40, TG 250. Diabetic peripheral neuropathy. Plan in the letter: diet and weight, keep metformin (R5) |
| 2021-08-17 | practice | ankle-brachial index for the diminished pulses: 0.86 right, 0.88 left (A2); peripheral arterial disease added to the problem list |
| 2022-08-03 | outside | urology consult letter: 6 months of difficulty maintaining erections; BP 138/86, pulse 78; diminished pedal pulses; most recent A1c 7.2. Organic ED from diabetes; **sildenafil as needed started** (A3) |
| 2023-04-18 | practice | reports about 1 month of exertional chest tightness on stairs or uphill, resolving with rest; resting ECG normal; referred to cardiology (A4) |
| 2023-07-26 | outside | cardiology consult: 4 months of exertional central chest pressure; BP 132/78, pulse 72, BMI 27; troponin normal; resting ECG normal; exercise stress test positive for exercise-induced ST depression, no arrhythmia. Stable angina, medical management (R6) |
| 2024-07-17 | outside | cardiology follow-up: same exertional pressure with stairs, settles in minutes; BP 138/84, HR 72; ECG nonspecific ST-T changes; repeat stress test positive for inducible ischemia. **Sublingual nitroglycerin 0.4 mg as needed** started, sit down after taking; **never within 24 hours of sildenafil**. A later cardiology note says episodes are fewer and milder |
| 2026-01-12 | practice | routine follow-up after the cardiology visits; no new symptoms; adherent; saw a waiting-room flyer for a trial of a new lipid-lowering drug and asks about joining; the physician explains randomization to drug or placebo. BP 138/86; A1c 7.2, fasting glucose 154, TC 210, LDL 130, HDL 45, TG 250. Trial information given (P) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source lists lisinopril from the start yet says the cough began "6 weeks
  after starting lisinopril". Resolution: lisinopril starts on 2020-04-14 and the
  cough visit is dated about 8 weeks later. Before that, his blood pressure is not on
  a drug. The "suspected white coat" label maps to the January plan for a home log.
- **R2.** BMI 32 in January 2020 and 27 three months later is not possible. Resolution:
  BMI 32 in January 2020, then slow weight loss to 27 by mid-2023 (diet; he walks), and
  about 27 to 28 after that. Height 178 cm.
- **R3.** The source lists "ACE inhibitors (cough)" as an allergy from the first visit
  and keeps lisinopril on later lists. Resolution: the intolerance is recorded on
  2020-06-09. Outside letters (endocrinology 2021, urology 2022, cardiology 2023 and
  2024) may still carry lisinopril on their copied medication lists, as outside lists
  do; the practice's own list never does after June 2020.
- **R4.** The source names no replacement for lisinopril; amlodipine appears on the 2024
  cardiology list. Resolution: amlodipine 5 mg starts at the cough visit. (An ARB would
  be the usual choice for a diabetic with albuminuria. The source says amlodipine, so the
  chart keeps it, and no visit explains the choice.)
- **R5.** The source changes no diabetes drug, yet A1c falls from 8.2 to 7.2.
  Resolution: the drop goes with the weight loss in R2. No new diabetes drug.
- **R6.** The source puts stable angina on the problem list from 2020. Resolution:
  angina starts in early 2023 and is diagnosed in July 2023. The 2020 to 2022 notes
  never mention it. The source's "simvastatin" (encounter 0 prose) is dropped:
  atorvastatin 20 mg throughout.
- **Left as the source has it:** atorvastatin stays 20 mg and the LDL stays 130 in a
  diabetic with angina. No one intensifies it, and no note comments on it. The
  physician who inherits the chart can find that gap.

## Additions (A): physician to confirm

- **A1.** 2019 new-patient visit (the source starts with a "routine follow-up", so he was
  already a patient).
- **A2.** One ABI (0.86 / 0.88) to support the source's peripheral arterial disease.
- **A3.** Sildenafil, inferred from the 2024 warning about ED drugs with nitroglycerin.
- **A4.** The April 2023 practice visit that sends him to cardiology.
- **A5.** Chronic kidney disease stage 1 needs albuminuria with normal filtration:
  eGFR runs above 90 on most draws (the source's creatinine 1.0 in January 2020 gives
  about 87, a wobble the chart may note) and urine albumin/creatinine runs 35 to 60 mg/g
  from 2019.
  The January 2020 urinalysis stays negative for protein, as the source says.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention
  (kidney size, no hydronephrosis, aorta caliber, heart sounds). An abnormal finding appears
  only as the source or this file gives it. A report may carry the descriptors its kind
  always carries (how a size was measured, an aneurysm's shape, a benign nodule's size and
  lobe within its Lung-RADS category, a procedure's access site), but nothing that adds a
  territory, a severity grade or a symptom, or changes the diagnosis or plan.
- Records from a prior practice may mention routine screening done there, with normal results.
- Diabetes and blood pressure visits about every 3 to 6 months, with vitals, A1c,
  basic metabolic panel, lipids yearly, urine albumin/creatinine yearly, foot exams.
- Yearly diabetic eye exam letters from an optometrist: no retinopathy.
- Age-appropriate prevention: colorectal screening (a colonoscopy in 2019 or 2020 with
  no polyps, or yearly FIT, negative), influenza vaccine each fall, pneumococcal vaccine,
  shingles series, COVID vaccines, Tdap if due. Hepatitis C and HIV screening once.
- Refills, pharmacy and prior-authorization traffic, portal messages about results,
  refills and scheduling, a missed visit, reminders.
- Booked visit after 2026-08-27: a routine diabetes follow-up.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.
