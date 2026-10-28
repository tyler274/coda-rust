# Build requirements: unit-2046 (Adam Walsh, MRN00002046)

Source: synthetic_hospital patient 2046, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz text ("The pain in this patient is most
likely to radiate to").

## Who he is (S)

Adam Walsh, born 1978-04-16, male, White, not Hispanic or Latino, English, lives in
Amherst (01002), commercial PPO through his employer. Logistics coordinator. Never
smoker. Alcohol: occasional social use, 1 to 2 drinks a month. No known drug
allergies. Single; no household members are named in the chart.

Surgical history: open appendectomy for a perforated appendix in 2013, age 35 (R1).
Family history: father with recurrent kidney stones; mother with essential
hypertension. No other relatives are named.

Joined the practice 2023-08-22 (A1), aged 45, bringing:
- **Essential hypertension**, on **hydrochlorothiazide 25 mg by mouth daily**,
  prescribed by his prior primary care office (recorded as historical at joining, then
  refilled by the practice).
- **Recurrent calcium oxalate nephrolithiasis** (catalog "Kidney stone"): stones before
  joining that passed on their own, with calcium oxalate on an analysis done at prior
  care (his account; no dates, sizes or reports reach the chart).
- The 2013 open appendectomy.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2023-08-22 | practice | new patient visit (A1): history as above; on HCTZ 25 mg; BP in range (P); height and weight taken; prevention labs and first FIT kit (P) |
| 2024-06-04 | outside, ED | 2 days of severe intermittent colicky right flank pain radiating to the groin, nausea, vomited twice; no fever, dysuria or change in urine color. Afebrile, BP 135/85. Right costovertebral angle tenderness. Urinalysis: hematuria without pyuria or bacteriuria. Non-contrast CT abdomen and pelvis: 5 mm calculus in the right ureter. **Right ureterolithiasis diagnosed.** Asks about diet to prevent further stones. ED treatment and discharge prescriptions per A2. The practice receives a short ED note with the CT impression on 2024-06-05 |
| 2024-06-11 | practice | post-ED visit (A3): passed the stone at home 2024-06-07, not retrieved; pain free. Stone-prevention diet counseling at his request (fluids to about 2.5 L of urine a day, less salt, moderate animal protein, normal dietary calcium; HCTZ continues). Repeat urinalysis normal (P). **Ureterolithiasis resolved** |
| 2024-12-14 | outside, ED | six months after the right-sided stone: sudden severe left flank pain for 3 hours, began at work, constant with waves of intense severity, no relief from rest or position; nausea, vomited twice; no diarrhea, dysuria or fever. Drinks little water at work, a lot of tea and red meat. BP 152/92, pulse 104, temperature 37.1 °C. Uncomfortable, pacing, shifting position; left costovertebral angle tenderness, no rebound or guarding. Urinalysis: microscopic hematuria without pyuria or bacteriuria. No imaging (R4). **Left ureterolithiasis diagnosed** clinically. Discharge prescriptions per A2. He messages the practice 2024-12-15; the short ED note arrives 2024-12-17 |
| 2024-12-19 | practice, video | post-ED follow-up (A3): passed the stone 2024-12-17, not retrieved; pain free. Counseling on water intake, tea and red meat. **Ureterolithiasis resolved** |
| 2025-11-26 to 2025-11-30 | outside, hospital (general surgery) | aged 47: several days of progressive abdominal discomfort and poor intake, no stool or flatus since the day before, 12 hours of diffuse pain, bloating and persistent vomiting; no fever, hematemesis, melena, hematochezia or urinary complaints. Temperature 37.3 °C, BP 104/68, pulse 112, respiratory rate 22. Uncomfortable, mildly dehydrated; distended abdomen, diffuse high-pitched bowel sounds, diffuse tenderness without rebound or guarding; rectal exam unremarkable. Upright abdominal radiograph per R5. **Adhesive small bowel obstruction** and **dehydration diagnosed**. Course per A4; discharged 2025-11-30. HCTZ held in hospital, resumed at discharge. He calls the practice 2025-11-28 from the ward; the discharge summary arrives 2025-12-05, after the follow-up visit |
| 2025-12-03 | practice | post-discharge visit (A5): eating normally, bowels moving, no pain or vomiting; back on HCTZ; BMP normal (P). **Small bowel obstruction and dehydration resolved** |
| 2026-08-19 | practice | last visit: blood pressure follow-up on HCTZ; labs drawn (P) |
| after 2026-08-27 | practice | booked annual physical (P) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Surgical history reads "laparoscopic appendectomy (age 44)" in two notes and
  "open appendectomy 12 years ago for a perforated appendix" in the surgical admission.
  Resolution: one operation, an open appendectomy for perforated appendicitis in 2013
  (age 35), which explains the adhesions. The laparoscopic, age-44 version is not
  carried.
- **R2.** The comorbidity "sinus tachycardia secondary to burns" has no burn anywhere in
  the source. Resolution: not carried. The pulse of 112 at the 2025 admission stands as
  a vital sign and goes with the dehydration; no tachycardia diagnosis.
- **R3.** "Hypertension" and "Essential hypertension" are one condition: Essential
  hypertension, present at joining.
- **R4.** The 2024-12-14 ED note ends in quiz text and gives no imaging or plan.
  Resolution: the quiz is dropped. The ED diagnoses left ureteral stone on the history,
  exam and urinalysis in a known stone former six months after a CT-proven stone, with
  no imaging, and discharges him per A2. The stone is never imaged.
- **R5.** "An upright abdominal radiograph is shown." Resolution: the radiograph report
  reads dilated small-bowel loops with multiple air-fluid levels and no free
  intraperitoneal air, consistent with small bowel obstruction. No CT or other imaging.
- **R6.** Source notes stamp ureterolithiasis "diagnosed 2020-01-15" and carry it as an
  active problem. Resolution: each ureteral stone is diagnosed at its ED visit and
  resolved when it passes; the 2025 surgical record calls them prior episodes.
- **R7.** The source's urinalysis findings are qualitative. The chart carries them as
  blood positive, red cells present, white cells 0-2/HPF, no bacteria, leukocyte
  esterase and nitrite negative; no other numbers from either ED urinalysis.
- **Left as the source has it:** two stones in six months in a recurrent stone former
  on HCTZ, with no 24-hour urine or metabolic stone evaluation, no stone analysis of the
  2024 stones, no urology referral and no potassium citrate. Prevention stays diet and
  fluids. No one comments on it.

## Additions (A): physician to confirm

- **A1.** 2023-08-22 new patient visit: the source has him on HCTZ with established
  hypertension and prior stones at his first encounter, and about 3 years with the
  practice. A brief medication and problem list from the prior primary care office
  arrives 2023-09-12, with no numbers.
- **A2.** ED stone treatment, each visit: IV ketorolac and IV ondansetron in the ED; at
  discharge **tamsulosin 0.4 mg daily** (short course, up to 4 weeks or until the stone
  passes), **ibuprofen 600 mg every 6 hours as needed** (short), **ondansetron 4 mg ODT
  every 8 hours as needed** (short), and a strainer. The second visit gets the same
  three prescriptions.
- **A3.** The two post-ED practice contacts and the passage dates (2024-06-07,
  2024-12-17), so each ureteral stone closes.
- **A4.** Adhesive small bowel obstruction after the source ends: managed without
  surgery: nothing by mouth, nasogastric decompression, IV fluids, HCTZ held. Flatus
  returned on hospital day 3 (2025-11-28), the tube came out, the diet advanced, and he
  went home 2025-11-30 on a regular diet with HCTZ resumed. No operation, no further
  imaging, no discharge prescriptions beyond his HCTZ.
- **A5.** 2025-12-03 post-discharge practice visit, with a BMP after the vomiting and
  IV fluids (normal).

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention
  (no hydronephrosis on the CT, normal kidneys and bladder, appendix surgically absent,
  heart and lung sounds). An abnormal finding appears only as the source or this file
  gives it. A report may carry the descriptors its kind always carries, but nothing
  that adds a territory, a severity grade or a symptom, or changes the diagnosis or plan.
- Records from a prior practice may mention routine screening done there, with normal
  results.
- Blood pressure on HCTZ runs about 128 to 134 over 80 to 84 at practice visits; height
  about 179 cm, weight about 86 kg, steady. Practice visits for hypertension every 6 to
  9 months, a yearly physical (2024-09, 2025-09), BMP or CMP yearly and after the
  admission, lipids, CBC, A1c and a urinalysis now and then, all normal.
- Prevention: yearly FIT, negative; hepatitis C and HIV screening once, nonreactive;
  Tdap at joining; influenza each fall; COVID vaccine in 2023 and 2024 (declined in
  2025); PHQ-2 at physicals, negative. Shingles vaccine not yet due.
- Refills of HCTZ, pharmacy traffic, portal messages about results, refills and
  scheduling, a cancelled visit, reminders.
- Booked visit after 2026-08-27: the annual physical.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002046 on 2026-08-19. Practice contacts after that day do
not happen; outside care after it still does, and its records are filed as received
without changing the practice's lists. A visit is booked after today.
