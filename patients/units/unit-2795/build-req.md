# Build requirements: unit-2795 (Doris Hastings, MRN00002795)

Source: synthetic_hospital patient 2795, in `source.md` beside this file. This file and
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
"(diagnosed ...)" stamps or its garbled fragments.

## Who she is (S)

Born 1966-11-21, female, White, not Hispanic or Latino, English-speaking, Blue Cross
Blue Shield PPO, lives in South Hadley. Elementary school teacher. Never smoker,
denies alcohol and any drug use. No known drug allergies.

Surgical history: cesarean section in 1995 through a low transverse incision; a
midline laparotomy in 2020 for small bowel obstruction (R1), with no abdominal
surgery or trauma after it until the 2026 cholecystectomy.

Family history: mother with type 2 diabetes and hypertension; father with coronary
artery disease and hyperlipidemia; a sister who had her gallbladder removed for
gallstones; a brother who had an inguinal hernia repaired.

Joined the practice 2023-08-22 (A1, new-patient visit) from a prior primary care
office, bringing: hypertension, type 2 diabetes mellitus, dyslipidemia (charted as
hyperlipidemia), hypothyroidism and obesity (BMI 33), all established for years; on
metformin 1000 mg twice daily, lisinopril 10 mg daily, atorvastatin 20 mg daily,
levothyroxine 75 mcg daily and aspirin 81 mg daily. These five drugs continue
unchanged at the same doses through every contact to today.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2023-08-22 | practice | new-patient visit (A1): history as above; height 163 cm, weight about 87.7 kg (BMI 33); baseline labs; prior records requested |
| 2025-05-13 | practice | reports a bulge in the lower abdomen first noticed around the turn of the year, slowly larger, more obvious after standing a long time or exertion, gone when lying down; no nausea, vomiting, constipation or abdominal pain. Exam: soft, nontender, reducible bulge along the lower midline laparotomy scar. Referred to general surgery (A2) |
| 2025-06-02 | outside | general surgery consult: bulge first noticed about six months earlier, gradually larger, more noticeable with prolonged standing or activity, reduces lying down; denies nausea, vomiting, constipation, abdominal pain. Laparotomy for small bowel obstruction five years earlier (2020), no later abdominal surgery or trauma. Exam: soft, nontender, reducible mass along the old lower midline scar, more prominent with straining, no overlying redness or warmth, no other abdominal or groin mass. Same five home drugs, NKDA. **Incisional hernia diagnosed.** Plan (A3): watchful waiting with elective repair offered later; return sooner for pain, enlargement or a bulge that will not reduce. The letter reaches the practice late, on 2025-07-08, after a records request |
| 2026-03-06 | practice | last practice visit: routine diabetes and blood pressure follow-up; hernia unchanged on exam; labs drawn |
| 2026-07-29 | outside | **laparoscopic cholecystectomy** by an outside general surgery group for presumed symptomatic gallstones (cholelithiasis); no intraoperative cholangiogram. The lead-up (A4) reaches the practice only as the patient's portal message of 2026-07-21. No operative note reaches the practice by today |
| 2026-08-06 | outside | ED visit at an area hospital, 8 days after the cholecystectomy, age 59: recovered well at first, then 3 days of progressive yellowing of the eyes and skin, pale stools and dark urine (R2); no fever, abdominal pain or nausea; appetite normal; bowel movements regular. No prior liver disease. Same five home drugs, NKDA. Denies alcohol and IV drug use (R3). Vital signs normal and afebrile: BP 134/78, HR 76, T 36.7 C, RR 16, SpO2 98% (A5). Exam: scleral icterus, mild skin jaundice, abdomen soft and nondistended without tenderness or hepatosplenomegaly, bowel sounds present, port-site wounds clean and healing, rectal deferred, rest unremarkable. ED labs (A5), RUQ ultrasound and MRCP (A6): **choledocholithiasis** (retained common bile duct stone). Admitted. The ED record reaches the practice on 2026-08-13 |
| 2026-08-07 | outside | ERCP (A7): biliary sphincterotomy, stone extracted; discharged 2026-08-08. The ERCP report reaches the practice on 2026-08-25; the discharge summary has not arrived by today |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The profile lists a Pfannenstiel cesarean (1995) and a laparoscopic
  cholecystectomy in 2018; encounter 0 describes a lower midline scar from a
  laparotomy for small bowel obstruction "five years ago" and no later abdominal
  surgery; encounter 1 places the cholecystectomy eight days before the ED visit.
  Resolution: cesarean 1995 (low transverse incision), midline laparotomy for small
  bowel obstruction in 2020, laparoscopic cholecystectomy on 2026-07-29. The 2018
  cholecystectomy is not carried; no list before August 2026 shows a cholecystectomy.
  The hernia sits in the midline laparotomy scar, not the cesarean scar.
- **R2.** Encounter 1 gives "one week" of yellowing in the complaint but says the
  discoloration began three days before, after a good initial recovery, and also says
  "presents to clinic". Resolution: an ED visit; jaundice for 3 days, beginning about
  5 days after surgery. The "one week" is not carried.
- **R3.** Encounter 1's social history is a fragment ("Alcohol use, or intravenous
  drug use."). Resolution: she denies alcohol and IV drug use, matching the profile.
- **R4.** Incisional hernia is on encounter 1's problem list "diagnosed 2020-01-15"
  and called "prior". Resolution: diagnosed at the 2025-06-02 surgery consult and
  still present (unrepaired) at the ED visit.
- **R5.** The source carries "Status post laparoscopic cholecystectomy" as a
  comorbidity. Resolution: true only from 2026-07-29; it appears in the August 2026
  outside records, not on the practice's lists (nothing is reconciled after
  2026-03-06).
- **Left as the source has it:** atorvastatin stays 20 mg (moderate intensity) in a
  diabetic woman with a father with coronary disease, and aspirin 81 mg continues for
  primary prevention; nobody changes either or comments. No intraoperative
  cholangiogram was done at the cholecystectomy; the chart states it as the outside
  records do, without comment. The hernia is never repaired.

## Additions (A): physician to confirm

- **A1.** The 2023-08-22 new-patient visit (the source's patient arrives with five
  established conditions and drugs; the seed gives three years with the practice).
- **A2.** The 2025-05-13 practice visit that refers her to general surgery.
- **A3.** The surgeon's plan for a small, reducible, minimally symptomatic incisional
  hernia: watchful waiting, elective repair offered when she wants it, warning signs
  given. She chooses to wait.
- **A4.** The lead-up to the cholecystectomy, known only from her 2026-07-21 portal
  message: a few episodes of upper abdominal pain after meals in spring 2026, an
  outside ultrasound showing gallstones, and the outside surgeon booking surgery for
  2026-07-29. No report of that ultrasound reaches the practice.
- **A5.** ED vital signs as above, and ED labs on 2026-08-06: total bilirubin 4.7
  mg/dL, direct bilirubin 3.6 mg/dL, alkaline phosphatase 312 U/L, AST 96 U/L, ALT
  148 U/L, albumin 4.0 g/dL, lipase 34 U/L (normal), WBC 7.4, hemoglobin 13.1 g/dL,
  platelets 236, INR 1.1, creatinine 0.82 mg/dL, glucose 168 mg/dL. These support the
  source's obstructive jaundice; no cholangitis or pancreatitis.
- **A6.** ED right upper quadrant ultrasound: gallbladder surgically absent; common
  bile duct 9 mm; distal duct obscured by bowel gas; no fluid collection in the
  gallbladder fossa; liver normal in size and echotexture. MRCP the same evening: an
  8 mm stone in the distal common bile duct, duct 9 mm, no other filling defect, no
  fluid collection, pancreatic duct normal. MRCP findings reach the practice inside
  the ED record and the ERCP report.
- **A7.** ERCP 2026-08-07 (the usual next step): deep biliary cannulation, biliary
  sphincterotomy, one 8 mm stone extracted with a balloon, occlusion cholangiogram
  clear with no leak from the cystic duct stump; no immediate complication. She went
  home 2026-08-08 with no medication change. Repeat liver tests are left to the
  outside surgeon and GI; no result reaches the practice by today.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level here says
otherwise:

- Reports and exams state ordinary normal findings for what the source does not
  mention. An abnormal finding appears only as the source or this file gives it.
- Records from the prior primary care office (received 2023-09) may mention routine
  screening done there with normal results: a colonoscopy in 2019 with no polyps
  (repeat in 10 years), a Pap with HPV cotest in 2021 negative, a 2022 screening
  mammogram negative, Tdap in 2017, COVID primary series and booster.
- Diabetes and blood pressure visits every 4 to 8 months, a yearly physical; A1c
  running 6.6 to 7.5 on metformin (lower in summers when she walks); blood pressure in the low 130s over low 80s on
  lisinopril; LDL in the 80s on atorvastatin; TSH normal on levothyroxine; creatinine,
  eGFR, liver tests and urine albumin/creatinine normal at every practice draw; weight
  86 to 89 kg (BMI about 33). Foot exams normal.
- Yearly diabetic eye exam letters from an outside optometrist: no retinopathy (the
  2024 letter never arrives; she reports the exam was normal).
- Screening mammograms 2024 and 2025, negative. Hepatitis C and HIV screening once,
  negative. PHQ-2 negative. Influenza and COVID vaccines each fall, Shingrix series
  (2023 to 2024), PCV20 once.
- Refills (90-day mail and local pharmacy), portal messages about results, refills
  and scheduling, one missed visit (2024-06-11), records requests.
- Nothing is booked after today.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002795 on 2026-03-06. Practice contacts after that day are
only messages, refills, records requests and outside records filed as received; the
practice's problem and medication lists stay as they were on 2026-03-06. Nothing is
booked.
