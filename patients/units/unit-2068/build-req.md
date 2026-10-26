# Build requirements: unit-2068 (Piotr Kirby, MRN00002068)

Source: synthetic_hospital patient 2068, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its "image shown below" / "is shown" text.

## Who he is (S)

Piotr Kirby, born 1967-05-11, male, White, not Hispanic or Latino, English, Medicaid,
Haydenville MA 01039. Construction foreman. Lives with his partner (S: the partner
brings him to the ED in 2025); marital status partnered.

- Tobacco: former smoker, 20 pack-years, quit about 5 years before joining (about 2016).
- Alcohol: severe alcohol use disorder; 6 to 10 beers a day for about 25 years, with
  frequent binges on distilled spirits (whiskey); prior alcohol withdrawal seizures.
  He keeps drinking throughout the chart.
- Allergies: no known drug allergies.
- Family history: father with alcohol use disorder and cirrhosis; paternal uncle with
  chronic lymphocytic leukemia; mother with hypertension.
- Surgical history: none on joining. Exploratory laparotomy with splenectomy happens
  2023-04-05 (R2); upper endoscopy with band ligation happens 2024-09-26 (R3); a
  second exploratory laparotomy happens 2026-03-25 (S).

**Joining.** First practice visit 2021-09-23 (S encounter 0 is his new-patient visit,
A1). He had been out of regular care for years and took no medications. He brings:
severe alcohol use disorder; alcoholic cirrhosis (compensated, known from an outside
admission in 2019, R1); history of recurrent aspiration pneumonia (outside admissions
2018 and 2020, R1); history of alcohol withdrawal seizures (outside admission 2019, R1).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2021-09-23 | practice | **new-patient visit** (A1). Several months of progressive fatigue and less exercise tolerance; no overt blood loss; drinks several glasses of whiskey a day on top of his beer. Exam unremarkable. Labs: hemoglobin 9.5, MCV 68, ferritin 550, iron 210, TIBC 240, transferrin saturation 88% (R4). CBC also shows a mild lymphocytosis (A2). **Anemia recorded**; **thiamine 100 mg daily and folic acid 1 mg daily started** (A3); hematology referral placed |
| 2021-10-12 | outside records | prior hospital records received (R1): 2018 and 2020 admissions for aspiration pneumonia; 2019 admission for an alcohol withdrawal seizure, at which CT showed a nodular cirrhotic liver |
| 2021-10-21 | outside | hematology consult (A2): peripheral smear and bone marrow aspiration with iron stain (S: "smear performed and bone marrow aspiration is stained"); smear dimorphic red cells with hypochromic microcytes, mature small lymphocytes with smudge cells; marrow ringed sideroblasts 22% of erythroid precursors, with an interstitial small-lymphocyte infiltrate; flow cytometry clonal B cells CD5+ CD23+ kappa-restricted. **Sideroblastic anemia (acquired, alcohol-related)** and **chronic lymphocytic leukemia** diagnosed (R5); anemia attributed to alcohol, not marrow failure; CLL needs no treatment, observation. **Pyridoxine 100 mg daily started**; abstinence advised. Letter and report received 2021-10-28 |
| 2022-01-02 | outside | ED, then admitted 3 days (R6): 2 months of progressive exertional dyspnea, orthopnea and both-leg edema; about six beers a day for 25 years, no recent increase. BP 110/70, HR 96, RR 18, SpO2 95% on room air. JVP elevated, S3. Sodium 138, potassium 4.2, creatinine 1.1, BNP 900, AST and ALT mildly elevated (R7). Chest x-ray cardiomegaly with pulmonary congestion. Echo dilated left ventricle, EF 30%. ECG sinus tachycardia with nonspecific ST-T changes. **Alcoholic cardiomyopathy** with **heart failure with reduced ejection fraction** diagnosed; IV diuresis; **furosemide 40 mg daily started** at discharge (A4). Discharge summary received 2022-01-07 |
| 2022-01-11 | practice | post-discharge visit (P); cardiology referral (A5) |
| 2022-05-31 | outside | cardiology consult (R8 moves it from Memorial Day 2022-05-30): several months of fatigue, trouble getting through his usual day, rests after short walks, occasional waking at night short of breath, mild leg swelling worse through the day, less appetite, a few pounds lost unintentionally; no chest pain or recent respiratory infection. Has taken no medicines for weeks: stopped the furosemide within a month of discharge (R9). Drinks about 8 to 10 beers a day (R10). BP 118/74, pulse 104, SpO2 93% on room air. Tired-appearing; both-leg edema, bibasilar crackles, displaced apical impulse. Chest x-ray cardiomegaly with pulmonary vascular congestion (R11). ECG atrial fibrillation, ventricular rate 104 (R8). **Nonvalvular (persistent) atrial fibrillation** and **tachycardia-mediated cardiomyopathy** diagnosed (R8). **Warfarin 5 mg daily started** (S); **metoprolol succinate 25 mg daily started** (A6); **furosemide restarted at 20 mg daily** (A4). INR monitoring handed to the practice (P) |
| 2023-04-05 | outside | admitted (general surgery), 6 days: sudden severe left upper quadrant pain into the left shoulder, lightheaded, several near-faints; on warfarin for atrial fibrillation. BP 90/60, HR 120. Pale, diaphoretic, left upper quadrant tenderness with guarding, normal bowel sounds, no rebound. Hemoglobin 8.2 (baseline 12), platelets 150,000, INR 3.5, LDH 410 U/L (R12). Ultrasound: free intra-abdominal fluid. **Atraumatic splenic rupture with hemoperitoneum** and **acute posthemorrhagic anemia**; warfarin reversed with vitamin K and prothrombin complex concentrate, red cells transfused (A7); **exploratory laparotomy with splenectomy** (R2). Pathology: ruptured spleen with capsular tear and parenchymal hemorrhage, involved by the known small lymphocytic infiltrate (A7). **Acquired absence of spleen.** Warfarin resumed at 5 mg at discharge (S: on warfarin in 2024). Discharge summary and pathology received 2023-04-13 |
| 2024-04-08 | outside | admitted (pulmonary), 7 days: 3 weeks of low-grade fever, malaise and productive cough with foul-smelling sputum, starting after a seizure about 3 to 4 weeks earlier with loss of consciousness, for which he sought no care (S: withdrawal seizure history; R16). Temp 38.2 C, HR 102, RR 22, BP 135/85. Decreased breath sounds and dullness at the right base. WBC 15,000, hemoglobin 13.2, platelets 250,000, sodium 135, potassium 4.2, BUN 18, creatinine 1.0. Chest x-ray: cavitary lesion right lower lobe. **Lung abscess (aspiration)** diagnosed. IV ampicillin-sulbactam, then **amoxicillin-clavulanate 875/125 mg twice daily** to finish 6 weeks (A8). Discharge summary received 2024-04-17 |
| 2024-09-26 | outside | ED, observed 2 nights: 2 hours of vomiting with bright red blood after forceful retching following a binge. BP 110/70, pulse 98. Pale; abdomen soft, not distended, nontender. Hemoglobin 13.2, hematocrit 39.4%, platelets 225,000, PT 12 seconds, PTT 30 seconds (R13). Urgent upper endoscopy: linear mucosal tear at the gastroesophageal junction with mild continued oozing, treated with band ligation (R3); hemostasis achieved. **Mallory-Weiss tear** diagnosed. **Pantoprazole 40 mg twice daily for 4 weeks** (A9). Warfarin held 2 days, then back to 5 mg (R13). Records received 2024-09-30 |
| 2025-03-29 | outside | admitted (pulmonary), 4 days: 4 days of worsening cough and fever after being found unconscious at home after a 3-day binge; brought to the ED by his partner; several weeks of repeated vomiting and poor intake; foul-smelling brown sputum and right-sided chest discomfort. Prior lung abscess April 2024 (re-dated). Temp 38.5 C, pulse 106, BP 104/68, SpO2 91% on room air. Decreased breath sounds and coarse crackles at the right posterior base. Chest x-ray: right lower lobe consolidation, no cavity (R14). **Aspiration pneumonia** diagnosed. IV ampicillin-sulbactam, then **amoxicillin-clavulanate** to complete 7 days (A8). Discharge summary received 2025-04-04 |
| 2026-03-25 | outside | trauma admission (ICU, then ward), 12 days: motor vehicle collision with major hemorrhage; on chronic warfarin. BP 90/60, HR 120. Alert, anxious, multiple contusions, suspected pelvic fracture, abdomen distended and tender. FAST positive for free abdominal fluid. Straight to the OR: **exploratory laparotomy** with massive transfusion protocol; bleeding mesenteric laceration controlled and repaired, no bowel resection (A10); warfarin reversed (A10). During surgery perioral numbness and carpopedal spasm. Post-transfusion: ionized calcium 0.9 mmol/L, hemoglobin 9, platelets 150,000, PT 16 seconds, PTT 40 seconds, lactate 3.2. **Hypocalcemia** (citrate) treated with IV calcium (A10); **acute posthemorrhagic anemia**. CT confirms **nondisplaced right superior and inferior pubic rami fractures**, managed without surgery, weight-bearing as tolerated (A10). **Warfarin stopped** at discharge (A11). Short course of oxycodone-acetaminophen 5/325 (A10). Discharge summary received 2026-04-08 |
| 2026-05-28 | practice | **last visit** (seed `last_seen`). Pelvic fracture follow-up: walking with a cane, pain mild; off warfarin; still drinking. CBC and CMP drawn that day |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The profile lists cirrhosis, recurrent aspiration pneumonia and withdrawal
  seizures from the start with no event behind them. Resolution: they come from outside
  admissions before he joined (aspiration pneumonia 2018 and 2020; withdrawal seizure
  2019, with CT showing cirrhosis), received as prior records in October 2021. Cirrhosis
  is compensated: albumin about 3.5 to 3.8, bilirubin about 1.0 to 1.4, no ascites,
  no encephalopathy, no varices known.
- **R2.** The profile lists "exploratory laparotomy with splenectomy" from the first
  visit, but the spleen ruptures in 2023. Resolution: the splenectomy is the 2023-04-05
  operation. No surgical history before that.
- **R3.** The profile lists "upper endoscopy with esophageal banding" from the first
  visit, and no varices appear anywhere. Resolution: the banding is the band ligation of
  the oozing Mallory-Weiss tear at the 2024-09-26 endoscopy. No varices.
- **R4.** Iron 210 with TIBC 190 gives a saturation over 100%, which cannot be.
  Resolution: iron 210 and ferritin 550 are carried; TIBC is 240 and saturation 88%.
  The TIBC of 190 is not carried.
- **R5.** CLL and sideroblastic anemia: the profile lists CLL from the start; the
  source "diagnoses" sideroblastic anemia at encounter 0 from a smear and marrow that a
  primary-care visit would not perform. Resolution: the practice draws the labs; the
  hematology consult (A2) does the smear, marrow and flow and names both conditions.
  The source's "(diagnosed 2020-01-15)" maps to onset 2021-09-23.
- **R6.** An echo in the ED: resolution: he is admitted from the ED for 3 days; the echo
  is done on the ward.
- **R7.** "Mildly elevated AST and ALT": carried as AST about 60 to 80 and ALT about 35
  to 45 U/L (AST about twice ALT), which is also his level on routine draws.
- **R8.** Atrial fibrillation is on every list from the start, yet the January 2022
  ECG shows sinus tachycardia and he takes no warfarin until later. Resolution: atrial
  fibrillation is found at the 2022-05-31 cardiology visit (pulse 104; ECG atrial
  fibrillation, rate 104), and tachycardia-mediated cardiomyopathy is named there (the
  source's stamp falls on this encounter). Warfarin starts that day. The source's
  practice date 2022-05-30 is Memorial Day; the visit is moved one day. The practice's
  lists carry no atrial fibrillation or warfarin before then; earlier outside notes
  (encounter 0, 2022 ED) do not mention it.
- **R9.** Cardiology's "does not take any medications and has not seen a physician in
  years" conflicts with its own warfarin line and with his 2021 to 2022 care.
  Resolution: he had stopped the furosemide within a month and was not taking his
  vitamins regularly; "not seen a physician in years" is dropped. The practice's list
  stops the first furosemide at that visit. Thiamine, folic acid and pyridoxine stay on
  the list.
- **R10.** Drinking amounts differ (6 beers a day at the ED, 8 to 10 at cardiology,
  6 to 10 with spirit binges in the profile; "over a decade" vs 25 years). Resolution:
  each writer records what he told them; the 25-year history stands.
- **R11.** "A chest x-ray is shown" (cardiology): carried as cardiomegaly with pulmonary
  vascular congestion, no effusion, no consolidation.
- **R12.** "LDH elevated": carried as 410 U/L (reference up to 250).
- **R13.** He is on warfarin 5 mg, yet the 2024-09-26 PT is a normal 12 seconds.
  Resolution: he had taken no warfarin for about a week during the binge (his account);
  it is held 2 more days after the endoscopy and restarted at 5 mg.
- **R14.** "A chest x-ray is obtained" (2025): carried as right lower lobe consolidation
  without cavity or effusion.
- **R15.** Hemoglobin 9.5 in 2021, "baseline 12" in 2023 and 13.2 in 2024 while still
  drinking: the rise follows pyridoxine and folic acid from late 2021 (about 10.5 early
  2022, 11 to 11.5 later 2022, about 12 in early 2023, about 12.5 to 13.3 from late 2023
  to early 2026). MCV stays low (about 70 to 76).
- **R16.** The source gives the 2024 seizure no cause. Resolution: by his account it came
  after he stopped drinking for about two days on his own, a withdrawal seizure like the
  2019 one; his partner found him.
- **Left as the source has it:** heart failure therapy is furosemide and metoprolol
  only: no ACE inhibitor, ARB, ARNI, mineralocorticoid antagonist or SGLT2 inhibitor,
  no repeat echocardiogram and no ischemic evaluation of the cardiomyopathy. No alcohol
  use disorder medication or treatment program is started. No hepatology referral. No
  one explains the INR of 3.5. No note comments on any of these.

## Additions (A): physician to confirm

- **A1.** Encounter 0 is his new-patient visit (5 years with the practice).
- **A2.** The hematology consult that performs the smear, marrow and flow cytometry the
  source implies, and its lymphocytosis trigger on the practice CBC (WBC about 10 to
  11, lymphocytes about 58 to 62%). Hematology follows the CLL about yearly (letters
  2022-04, 2023-05, 2024-05, 2025-06): no treatment indication, no new findings. After
  the splenectomy WBC runs about 12 to 13.5 with lymphocytes about 60%.
- **A3.** Thiamine 100 mg and folic acid 1 mg daily for severe alcohol use disorder.
- **A4.** Furosemide 40 mg daily at the January 2022 discharge (decompensated heart
  failure); restarted at 20 mg daily by cardiology in May 2022 and continued.
- **A5.** The practice referral to cardiology after the admission.
- **A6.** Metoprolol succinate 25 mg daily for rate control (tachycardia-mediated
  cardiomyopathy calls for it); office heart rate about 72 to 84 after that. Cardiology
  follow-up letters 2022-09, 2023-07, 2024-07, 2025-07: rate controlled, no changes.
- **A7.** Reversal and transfusion for the 2023 rupture, and the splenic pathology.
  Post-splenectomy vaccines at the practice: meningococcal ACWY two doses 8 weeks
  apart, meningococcal B two doses, Hib once (pneumococcal PCV20 already given 2021).
- **A8.** Antibiotics for the lung abscess (ampicillin-sulbactam, then
  amoxicillin-clavulanate to complete 6 weeks) and the 2025 aspiration pneumonia (7
  days). Follow-up CT chest 2024-06-11: cavity resolved, small linear scar at the right
  lung base; lung abscess resolved. Follow-up chest x-ray 2025-05-13: clear; pneumonia
  resolved.
- **A9.** Pantoprazole 40 mg twice daily for 4 weeks after the Mallory-Weiss tear.
- **A10.** Trauma care around the source facts: the laparotomy finding (mesenteric
  laceration, repaired), warfarin reversal, IV calcium, the CT confirming the
  suspected pelvic fracture as nondisplaced right superior and inferior pubic rami
  fractures, and oxycodone-acetaminophen 5/325 for 7 days. Orthopedic fracture-clinic
  letters with pelvis x-rays 2026-04-23 (early callus, alignment kept; walker to cane) and 2026-07-09 (healed with bridging callus; full weight
  bearing; filed after the last visit).
- **A11.** Warfarin is stopped at trauma discharge after a third major bleed on it
  (CHA2DS2-VASc 1 from heart failure); cardiology letter 2026-05-12 agrees to stay
  off anticoagulation. Metoprolol and furosemide continue.
- **A12.** Low-dose CT lung screening 2022-02-22 and 2023-02-09 (20 pack-years, quit
  about 5 years): Lung-RADS 1 both times. Not repeated after the 2024 abscess.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may
  carry the descriptors its kind always carries, but nothing that adds a territory, a
  severity grade or a symptom, or changes the diagnosis or plan.
- Visits every 2 to 4 months with an occasional no-show; post-discharge visits after
  each admission. Office BP about 104 to 122 over 64 to 78; weight about 82 kg at
  joining (height 180 cm), about 79 kg by mid-2022, 78 to 81 after.
- Warfarin monitored by the practice (June 2022 to March 2026): INR every 2 to 6
  weeks, mostly 2.0 to 3.0 with occasional readings a little out of range, dose kept
  at 5 mg; INR about 1.1 to 1.2 off warfarin (cirrhosis).
- CBC with differential and CMP every few months (platelets about 135 to 170 before
  the splenectomy and about 230 to 280 after, AST/ALT as R7, albumin and bilirubin as
  R1, creatinine about 0.9 to 1.1); BMP after diuretic changes.
- Liver ultrasound for cirrhosis surveillance (right upper quadrant) 2021-11, 2022-07,
  2023-11, 2024-11, 2025-11: cirrhotic liver, no mass, no ascites, patent portal vein.
- Prevention: influenza vaccine each fall; COVID vaccines (updated formula each fall
  from 2022); PCV20 and Tdap 2021; hepatitis A (2 doses) and hepatitis B (3 doses);
  Shingrix 2 doses. Hepatitis C, hepatitis B surface antigen and HIV screening once
  (negative; hepatitis B surface antibody negative before the series). FIT yearly,
  negative. Brief alcohol counseling at visits; AUDIT-C high; PHQ-9 low.
- Refills, pharmacy traffic, portal messages about results and scheduling, records
  requests, reminders.
- Nothing booked after 2026-08-27.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002068 on 2026-05-28. Practice contacts after that day are
only messages, refills and results of that day's draw; outside records after it (the
2026-07-09 orthopedic letter) are filed as received without changing the practice's lists.
