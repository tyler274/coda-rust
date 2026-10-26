# Build requirements: unit-1725 (Denise Harris, MRN00001725)

Source: synthetic_hospital patient 1725, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its unfinished sentences ("She is.", "Based on the
clinical presentation and test results,").

## Who she is (S)

Born 1993-12-20, female, Black, not Hispanic, English-speaking, commercial PPO,
administrative assistant. No alcohol. Allergies: no known drug allergies. Family
history: mother with type 2 diabetes and gallstones, a sister with polycystic ovary
syndrome, father with essential hypertension.

Tobacco: a cigarette smoker when she joins (no amount recorded); quits in mid-April
2021, when she starts trying to conceive (R2). Former smoker after that.

Joined the practice on 2020-09-14 (A1), age 26, with:
- obesity, BMI 36 (R1);
- a history of recurrent urinary tract infections, three or more a year, on
  **nitrofurantoin macrocrystals 100 mg nightly** for prophylaxis from her prior
  primary-care office (R3);
- acne in her teens and early twenties treated with topical medicines (no current drug).

No surgical history at joining (R5).

## Timeline (dates fixed; S unless marked)

LMP for the pregnancy is 2025-07-28 (A10), so 10 weeks 0 days on 2025-10-06 and
24 weeks 4 days on 2026-01-16.

| Date | Where | What happens |
|---|---|---|
| 2020-09-14 | practice | new patient visit (A1): history as above; BMI 36; current smoker, cessation advised; screening labs (P) |
| 2021-04-19 | practice | office visit (A2): quit smoking last week, now that she has started trying to conceive; **prenatal vitamin daily started** (R4); nitrofurantoin continued |
| 2022-03-07 | practice | office visit (A3): about 11 months trying to conceive; periods every 2 to 3 months, often heavy and long; more hair on face and abdomen over about two years; recurrent painful lumps in both armpits and the groin that drain. BMI 32. Referred to OB/GYN and to dermatology |
| 2022-04-18 | outside | OB/GYN consult letter: cycles every 2 to 3 months, heavy and prolonged; 12 months trying to conceive without success; increasing facial and abdominal hair over two years; no thyroid symptoms, no galactorrhea. BMI 32. Acanthosis nigricans of the neck; mild hirsutism of upper lip and chin. Fasting glucose 98, total testosterone 82 ng/dL (ref 15 to 70), prolactin 14, TSH 2.1, lipid panel normal. Pelvic ultrasound: multiple small peripheral follicles in both ovaries. **Polycystic ovary syndrome diagnosed; metformin started, to 1000 mg twice daily** (R6) |
| 2022-06-13 | outside | dermatology consult letter: painful recurrent nodules and abscesses in both axillae and the groin with sinus tracts. **Hidradenitis suppurativa, Hurley stage II, diagnosed; clindamycin phosphate 1% gel twice daily started** for the HS lesions and acne (R7) |
| 2022-08-15 | practice | office visit (A4): about four months of crampy lower abdominal pain more than once a week, often after eating, eased by a bowel movement, with alternating diarrhea and constipation; no travel, sick contacts or recent antibiotics. Referred to gastroenterology |
| 2022-10-03 | outside | gastroenterology consult letter: six months of that pattern; abdomen soft, nondistended, nontender, no masses, normal bowel sounds. Hemoglobin 11.2 g/dL, MCV 86, WBC 7.0, CRP 2 mg/L, ESR 15. **Irritable bowel syndrome, mixed type, diagnosed; dicyclomine 10 mg three times daily as needed for cramping started** (R8) |
| 2023-04-06 | outside | dermatology letter (a different dermatologist in the same group): progressive thinning at the crown over the past year; cycles 35 to 60 days; trouble losing weight despite exercise and diet. BP 125/78, BMI 32; excess facial hair in a male pattern; no thyroid enlargement or nodules. Fasting glucose 96, total testosterone 82 ng/dL (ref 8 to 60), TSH 2.5, lipid panel normal. Assessment: crown thinning and hirsutism from the androgen excess of her PCOS (R9). No new drug |
| 2024-04-04 | outside | ED record: five hours of severe intermittent right upper quadrant pain to the back after a large meal, with nausea, no vomiting, no bowel or urine change. Vitals stable. Marked RUQ tenderness without rebound or guarding, positive Murphy's sign. Hemoglobin 12.5, WBC 10.0, total bilirubin 1.2, alkaline phosphatase 120, AST 40, ALT 45, amylase 90, lipase 45. RUQ ultrasound: multiple gallstones, no sign of cholecystitis. **Symptomatic cholelithiasis (biliary colic) diagnosed** (R10); discharged with ondansetron as needed (A5) and advice to see her primary care physician |
| 2024-04-11 | practice | post-ED visit (A6): pain gone; general surgery referral placed for elective cholecystectomy. The referral is never scheduled before October (P texture: the surgery office cannot reach her) |
| 2024-10-04 | outside | ED and admission record (A7 for everything after the ED findings): two days of worsening intermittent RUQ pain to the back with nausea, fever and chills since the day before. T 38.9 C, BP 100/60, pulse 110, RR 22. Jaundiced; marked RUQ tenderness without rebound or guarding; positive Murphy's sign. Hemoglobin 12.2, WBC 15.0, platelets 210, total bilirubin 3.2 (direct 2.5), alkaline phosphatase 190, AST 80, ALT 75, amylase 90, lipase 45. Ultrasound: multiple gallstones, thickened gallbladder wall, no perforation, dilated common bile duct. **Acute cholangitis diagnosed.** Admitted 2024-10-04 to 2024-10-08: blood cultures no growth; IV piperacillin-tazobactam; ERCP 2024-10-05 with a dilated common bile duct, sludge and small stones cleared, and a tight, stenotic papilla: **sphincter of Oddi dysfunction (papillary stenosis) diagnosed**, biliary sphincterotomy done (R11). Discharged on **amoxicillin-clavulanate 875/125 mg twice daily for 5 days**, and interval cholecystectomy planned |
| 2024-10-14 | practice | post-discharge visit (A8): feeling well, no fever, no pain; CMP and CBC with normal liver tests |
| 2024-11-18 | outside | **laparoscopic cholecystectomy** at an outside surgery group (A9); pathology: chronic cholecystitis with cholelithiasis. Uneventful recovery |
| 2025-09-03 | practice | office visit (A10): positive home pregnancy test, LMP 2025-07-28; urine pregnancy test positive in the office. **Pregnancy recorded.** Metformin, nitrofurantoin, prenatal vitamin, dicyclomine as needed and clindamycin gel continue (S). Referred to OB |
| 2025-10-06 | outside | OB prenatal visit letter, 10 weeks: no complaints, uneventful so far. BP 120/78, HR 72, T 37.0 C; exam unremarkable. Urinalysis: leukocyte esterase 1+, nitrite positive, 20 to 29 WBC/hpf; urine culture E. coli more than 100,000 CFU/mL. **Asymptomatic bacteriuria in pregnancy diagnosed; amoxicillin 500 mg three times daily for 7 days** (R12) |
| 2025-10-27 | outside | OB test-of-cure urine culture two weeks after the course: no growth (A11). Bacteriuria resolved |
| 2026-01-12 | practice | last practice visit (P): routine follow-up at 24 weeks; for the past few days more tired than usual and a poorer appetite, which she puts down to pregnancy. Exam without jaundice noted. No labs drawn (P; see Left as the source has it) |
| 2026-01-16 | outside | ED record: one week of fatigue, poor appetite and dark urine; no known liver disease, no alcohol or drug use, no travel, no sick contacts; first-trimester prenatal screening unremarkable. Afebrile, scleral icterus, mild RUQ tenderness, no rash, rest of exam normal. Total bilirubin 3.2, direct 1.5, AST 850, ALT 920, alkaline phosphatase 210, total protein 7.1, albumin 4.0, PT 12 seconds (control 11), WBC 5.0, hemoglobin 12.1, platelets 210. **Hepatitis serologies sent.** Seen by OB in the ED, fetal heart tones normal; discharged to hepatology follow-up within the week (A12) |
| 2026-01-23 | outside | hepatology consult letter (A12): HAV IgM, HBsAg, HBc IgM and HCV antibody nonreactive. AST 312, ALT 405, alkaline phosphatase 182, total bilirubin 1.6 (direct 0.8). **Toxic liver disease, drug-induced, most likely from long-term nitrofurantoin; nitrofurantoin stopped** (R13) |
| 2026-02-20 | outside | hepatology follow-up letter (A12): feeling well, urine clear. AST 44, ALT 61, alkaline phosphatase 150, total bilirubin 0.8 (direct 0.2). Improving; recheck with OB at delivery |
| 2026-05-01 | outside | spontaneous vaginal delivery at 39 weeks 4 days, uncomplicated, healthy infant; liver tests normal on admission; discharged 2026-05-03 (A13). Discharge summary filed |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The profile says BMI 36, but both measured visits (2022-04 OB, 2023-04
  dermatology) say BMI 32. Resolution: BMI 36 when she joins in 2020 (height 163 cm,
  about 95.6 kg), down to about 32 by early 2022 through diet and exercise and around 32
  to 33 after that, with ordinary pregnancy gain from late 2025. Obesity stays on the list.
- **R2.** "Quit 12 months ago when attempting conception" in April 2022 puts the quit
  date in mid-April 2021; every later note says former smoker. Before then she smokes.
- **R3.** Nitrofurantoin 100 mg daily is on every source list. Resolution: she arrives on
  it from her prior office and it continues unchanged until hepatology stops it in January
  2026 (R13). No urinary infection is treated by the practice in between.
- **R4.** The prenatal vitamin is on the source list from the first encounter.
  Resolution: started on 2021-04-19 when she starts trying to conceive.
- **R5.** The source lists a laparoscopic cholecystectomy "at age 26" (2020), yet the 2024
  ultrasounds show a gallbladder full of stones with a thickened wall. Resolution: the
  gallbladder is in place until 2024-11-18 (A9), at age 30. The age-26 surgery is not
  carried; no note before November 2024 mentions a cholecystectomy.
- **R6.** Metformin is on the source list before the PCOS diagnosis that explains it.
  Resolution: the OB/GYN starts it at the 2022-04-18 consult, and the practice records it
  at 1000 mg twice daily from that letter.
- **R7.** Clindamycin gel and hidradenitis are on the source lists before the
  dermatology encounter. Resolution: hidradenitis is diagnosed and clindamycin gel
  started on 2022-06-13. The source's "Hidradenitis suppurativa" twice on one list is one
  condition, Hurley stage II.
- **R8.** Irritable bowel syndrome and dicyclomine are on the lists from 2022-04, before
  the GI encounter. Resolution: both start at the 2022-10-03 GI consult. The GI letter's
  "noncontributory" family history may stay as written (that writer did not take it).
- **R9.** "Idiopathic hirsutism" contradicts the PCOS diagnosis it would exclude.
  Resolution: not carried. Hirsutism is a finding of her PCOS. The dermatology letter's
  "March 2020" and "January 2020" become June 2022 and April 2022. No diagnosis is given
  for the crown thinning beyond the PCOS androgen excess.
- **R10.** The source labels the 2024-04 ED visit "sphincter of Oddi dysfunction" with an
  ultrasound full of gallstones and an intact gallbladder. Resolution: the ED diagnoses
  symptomatic cholelithiasis (biliary colic); sphincter of Oddi dysfunction is diagnosed
  at the October ERCP (R11). The source's 2024-10 "prior sphincter of Oddi dysfunction"
  is not carried in that ED note.
- **R11.** Sphincter of Oddi dysfunction needs evidence and a place to be diagnosed.
  Resolution: the October 2024 ERCP finds a dilated duct and a stenotic papilla and the
  endoscopist names it (papillary stenosis), treated by sphincterotomy. It stays on the
  practice's list as a history item afterwards.
- **R12.** The 2024-10 ED note's "gestational diabetes, diet-controlled" comes before any
  pregnancy. Not carried. The OB note's "two weeks after completing the antibiotic
  course, she returns" becomes the test of cure (A11).
- **R13.** The source ends the January 2026 ED visit with serologies ordered and names
  "toxic liver disease" as the diagnosis. Resolution: serologies are negative and the
  hepatologist attributes the injury to nitrofurantoin, which is stopped and not to be
  restarted; the hepatologist asks that it be listed as an intolerance. The practice
  receives this after its last visit, so its own allergy list does not change. The ED note's
  age "31" follows the birth date: 32.
- Ages elsewhere follow the birth date: 28 at the GI consult, 30 at both 2024 ED visits.
- **Left as the source has it:** hidradenitis Hurley stage II stays on topical
  clindamycin alone; nobody adds oral or biologic therapy and no note remarks on it. The
  GI hemoglobin of 11.2 is never worked up. No infertility treatment is offered despite
  years of trying. At the 2026-01-12 visit her few days of fatigue and poor appetite draw no labs.

## Additions (A): physician to confirm

- **A1.** 2020-09-14 new-patient visit (six years with the practice; the source opens
  with established care elsewhere).
- **A2.** 2021-04-19 visit that records the quit and starts the prenatal vitamin.
- **A3.** 2022-03-07 visit that sends her to OB/GYN and dermatology.
- **A4.** 2022-08-15 visit that sends her to gastroenterology.
- **A5.** In the April 2024 ED: urine hCG negative, IV ketorolac and IV ondansetron for
  symptoms, and ondansetron 4 mg orally disintegrating tablet as needed for nausea at
  discharge, a short course.
- **A6.** 2024-04-11 post-ED visit with the general surgery referral.
- **A7.** The October 2024 admission: blood cultures without growth, IV
  piperacillin-tazobactam, ERCP with sphincterotomy and duct clearance on 2024-10-05,
  discharge on 2024-10-08 with amoxicillin-clavulanate for 5 days. Urine hCG negative in the
  ED; on 2024-10-07 total bilirubin 1.3 (direct 0.7), alkaline phosphatase 141, AST 38, ALT 52. The usual care for
  acute cholangitis with a dilated duct.
- **A8.** 2024-10-14 post-discharge visit.
- **A9.** Interval laparoscopic cholecystectomy on 2024-11-18, same-day discharge with
  acetaminophen as needed, pathology of chronic cholecystitis and cholelithiasis (the usual
  next step after gallstone cholangitis).
- **A10.** 2025-09-03 pregnancy visit and the LMP that dates the pregnancy.
- **A11.** Test-of-cure urine culture 2025-10-27, no growth.
- **A12.** The January 2026 ED disposition and the hepatology letters of 2026-01-23 and
  2026-02-20 with their numbers, the usual next steps after the source's serologies.
- **A13.** Term delivery on 2026-05-01: today is past her due date (2026-05-04), so the
  pregnancy's ordinary end must be in the chart. Uncomplicated, no new diagnosis. Tdap in the third trimester at OB; hepatic panel and
  CBC normal on admission; discharged on the prenatal vitamin, metformin and acetaminophen
  as needed, with nitrofurantoin listed as an intolerance (liver injury).

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention
  (uterus size, ovarian size, liver echotexture, kidneys, pancreas where seen). An abnormal
  finding appears only as the source or this file gives it. A report may carry the
  descriptors its kind always carries (how a size was measured, a duct diameter, a
  procedure's access site), but nothing that adds a territory, a severity grade or a
  symptom, or changes the diagnosis or plan.
- Records from her prior office may mention routine screening done there, with normal
  results (a normal Pap in 2019).
- Practice visits about yearly to twice yearly: annual physicals in 2021, 2022, 2023 and
  2024 with weight, blood pressure, CMP, lipids and A1c in the normal range, and a PHQ-9;
  a follow-up visit in early 2023 and in March 2025.
- Prevention: Tdap at joining; cervical cytology at the 2022 physical, normal; hepatitis C
  and HIV screening once, negative; influenza vaccine each fall; COVID vaccines (primary
  series spring 2021 at a pharmacy, a booster late 2021, updated doses in later falls).
- Refills, pharmacy and prior-authorization traffic, portal messages about results,
  refills and scheduling, a missed visit, a surgery-office scheduling note.
- After 2026-01-12 nothing is reconciled: outside records are filed, refills go out, and
  the practice's lists stay as they were on 2026-01-12 (nitrofurantoin still on the list,
  pregnancy still open).
- Booked visit after 2026-08-27: a follow-up after the delivery and the liver injury.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.


## Last seen

The practice last saw MRN00001725 on 2026-01-12. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. A visit is booked after today.
