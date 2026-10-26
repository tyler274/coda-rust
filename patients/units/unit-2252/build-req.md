# Build requirements: unit-2252 (Raymond Woodward, MRN00002252)

Source: synthetic_hospital patient 2252, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz text (the ezetimibe "2% over 5 years"
trial, the targeted-therapy hazard ratio).

## Who he is (S)

Raymond Woodward, born 1957-12-31, male, White, not Hispanic or Latino, English,
Belchertown MA 01007, Blue Cross Blue Shield commercial PPO. Retired construction
foreman. Married (marital status only; no spouse appears in the story). Alcohol: social,
2 to 3 drinks a week. Tobacco: about a pack a day from his early twenties, 40 pack-years;
**quit in late April 2020** (R2). Allergies: no known drug allergies. Family history:
father MI at 68, mother ischemic stroke at 72, paternal uncle thoracic aortic aneurysm.

Joined the practice on 2019-09-17 (A1, new patient visit), from a prior primary-care
office, bringing: essential hypertension, hyperlipidemia, first-degree AV block (found
on a prior ECG, asymptomatic), current cigarette smoking. Medications on joining:
lisinopril 20 mg daily, atorvastatin 40 mg daily, multivitamin 1 tablet daily.
No surgical history on joining (R1). Height 180 cm, weight about 92 kg at joining (A6).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2019-09-17 | practice | new patient visit (A1): history as above; BP about 134/84 on lisinopril; ECG sinus rhythm with PR 224 ms, otherwise normal (A2); smoking counselling, not ready to quit; LDCT lung screening ordered (A3); colorectal screening offered, colonoscopy declined, FIT kit taken (R5) |
| 2019-10-08 | outside | LDCT lung screening: no nodules, Lung-RADS 1 (A3) |
| 2020-04 (late) | — | **quits smoking** on his own, no medication (R2) |
| 2020-10-20 | outside | LDCT: Lung-RADS 1 (A3) |
| 2021-10-21 | practice | lab draw: lipid panel LDL 98, HDL 45, TG 150 (S, "most recent lipid panel") |
| 2021-10-28 | practice | routine follow-up (source encounter 0): adherent to lisinopril and atorvastatin, feels well, no new symptoms; asks whether another medicine would lower his heart attack risk further. BP 132/84, HR 78; exam unremarkable; LDL 98, HDL 45, TG 150 reviewed. Ezetimibe discussed as a modest extra benefit with no trial figures; **he chooses to stay on atorvastatin alone; ezetimibe not started** (R3). Quit 18 months ago |
| 2021-11-16 | outside | LDCT: Lung-RADS 1 (A3) |
| 2022-11-04 | outside | ED (source encounter 1): sudden severe sharp tearing chest pain radiating to the back between the shoulder blades, began 2 hours before arrival. BP 90/60, HR 118, RR 22, SpO2 94% on room air; diaphoretic and anxious; distended neck veins, muffled heart sounds; BP falls to 85/55 with inspiration. Chest x-ray: enlarged cardiac silhouette. ECG: low-voltage QRS and electrical alternans. CT angiography of the chest: Stanford type A dissection of the ascending aorta with hemopericardium (A4). **Type A aortic dissection and cardiac tamponade diagnosed** (R1) |
| 2022-11-04 to 2022-11-13 | outside | admission (A4): emergency surgery the same night, **pericardial window drainage and ascending aortic graft replacement** (native aortic valve kept); extubated, uneventful recovery; discharged home on his prior lisinopril 20 mg, atorvastatin 40 mg and multivitamin (R4). Discharge summary received |
| 2022-11-22 | practice | post-discharge visit (P): recovering, sternal precautions, no chest pain; problem list updated: s/p type A dissection with tamponade, ascending aortic graft; acute dissection and tamponade resolved |
| 2022-12 to 2023-11 | outside | cardiac surgery follow-up letters (A5): incisions healed; cardiology takes over his follow-up, 2023-01 TTE normal LV size and function, normal native aortic valve, no pericardial effusion, intact graft; CT angiography 2023-11 shows an intact ascending graft, no residual dissection, normal caliber of the remaining aorta, lungs clear; next aortic imaging in 2 years |
| 2024-08-20 | outside | LDCT lung screening (A3, resumed by the practice because aortic CT moved to every 2 years): 4.2 cm solid right upper lobe mass, enlarged right hilar and mediastinal nodes including a left paratracheal node; Lung-RADS 4B (A6) |
| 2024-08-27 | practice | visit to discuss the LDCT (P): referral to pulmonology and oncology; PET-CT ordered |
| 2024-09 | outside | PET-CT: avid right upper lobe mass and avid right hilar, subcarinal and left paratracheal nodes, no distant uptake; MRI brain no metastasis; EBUS biopsy of the mass's nodes: adenocarcinoma; EGFR, ALK and ROS1 negative (A6). Oncology consult: **stage IIIB non-small cell lung cancer (adenocarcinoma) diagnosed**, T2b N3 M0, unresectable; plan concurrent chemoradiation, then durvalumab (A7). **Port-a-cath placed** 2024-09-24 (S, surgical history) |
| 2024-09-17 | practice | visit after the diagnosis (P): coordination; **folic acid 1 mg daily** started by oncology for pemetrexed (A7) noted |
| 2024-10-03 | outside | cycle 1 carboplatin and pemetrexed every 3 weeks with daily thoracic radiation, 60 Gy in 30 fractions (A7) |
| 2024-10-24 | outside | oncology follow-up (source encounter 2): age 66; new shortness of breath and dry cough since cycle 1 three weeks ago; no chest pain, fever or weight loss; asks about prognosis and about a targeted therapy he read about; the oncologist explains his tumor has no targetable mutation so that drug does not apply, and treatment continues as planned (R6). BP 138/86, HR 92, RR 18, SpO2 94% on room air; decreased breath sounds right lower lobe, no wheezes or crackles, rest of exam unremarkable. Chest x-ray: right lower lobe opacity consistent with pleural effusion. **Right pleural effusion**; thoracentesis, 900 mL removed (A8), fluid cytology negative for malignancy |
| 2024-12-10 | practice | follow-up (P): chemoradiation completed (4 cycles; radiation finished 2024-11-14); breathing back near baseline; fatigue; weight about 88 kg |
| 2025-01 | outside | oncology letter (A7): CT chest after chemoradiation shows partial response of the mass and nodes, no effusion; **durvalumab 1500 mg IV every 4 weeks** started 2025-01-09 for 12 months; **folic acid stopped** |
| 2025-04-28 | practice | office visit (source encounter 3): three weeks of low-grade fevers, fatigue, unintentional weight loss (about 84 kg, from about 88 in February, A9), dull lower abdominal discomfort and intermittent constipation over the past few months; temperature 37.9 C (A9); new holosystolic murmur loudest at the apex; no peripheral stigmata of endocarditis. Not up to date on colorectal screening. Blood cultures x2 drawn; CBC WBC 11.8, hemoglobin 11.2; ESR 62; CRP 48 mg/L (A9) |
| 2025-04-30 | practice / outside | cultures grow gram-positive cocci in chains after 48 hours; PCP calls him and arranges direct admission (P). Admission 2025-04-30 to 2025-05-09: organism identified as Streptococcus gallolyticus, penicillin-susceptible (A10); TEE: mobile vegetation on the aortic valve, no abscess, ascending graft intact (R7); **infective endocarditis (aortic valve) diagnosed**; **IV ceftriaxone 2 g daily for 6 weeks**, through the port, to 2025-06-10 (A10); repeat cultures clear; durvalumab held. Because of the organism, colonoscopy 2025-05-06: partially obstructing mass in the descending colon, biopsied; pathology adenocarcinoma (A11); CT abdomen and pelvis: no liver or other metastasis (A11). **Malignant neoplasm of the descending colon diagnosed** |
| 2025-05-14 | practice | post-discharge visit (P): afebrile, on home ceftriaxone; murmur softer |
| 2025-06 | outside | infectious disease letter (A10): 6 weeks completed 2025-06-10; surveillance cultures negative; ceftriaxone stopped |
| 2025-07-16 to 2025-07-20 | outside | **laparoscopic left hemicolectomy** (A11): pathology pT3 N0 (0/18 nodes), margins negative, stage IIA; oncology: no adjuvant chemotherapy, colonoscopy in 1 year; durvalumab resumed 2025-08 |
| 2025-09-03 | practice | **last seen**: follow-up after surgery (P); recovered, eating well, bowels regular, weight about 86 kg; no murmur heard; endocarditis treated, recorded as resolved |
| 2025-10-30 | practice (portal) | about two weeks of intermittent dizziness and palpitations; no chest pain, dyspnea or fainting; PCP advises his cardiologist be seen promptly; the cardiology office books him (P) |
| 2025-11-06 | outside | cardiology (source encounter 4): age 67; dizziness and palpitations for two weeks; denies chest pain, dyspnea, syncope. BP 130/80, HR 68 and irregular, RR 16; irregular rhythm, no murmurs, rubs or gallops (R8). ECG: progressive PR lengthening until a P wave is not conducted, then a shorter PR that lengthens again: **Mobitz type I second-degree AV block (Wenckebach)**. Plan (A12): 14-day ambulatory monitor, transthoracic echo, avoid AV-nodal blocking drugs. Letter filed; practice lists not updated (after last seen) |
| 2025-12 | outside | cardiology follow-up letter (A12): monitor shows sinus rhythm with first-degree AV block and intermittent Mobitz I, mostly at night, no Mobitz II, no complete heart block, no atrial fibrillation; symptoms matched only sinus rhythm or rare dropped beats. TTE: normal LV size and function, aortic valve without vegetation, trace aortic regurgitation, intact ascending graft. No pacemaker; dizziness settled; recheck in 6 months |
| 2026-01 | outside | oncology letter (A7): durvalumab completed 2025-12-18; CT chest, abdomen and pelvis stable, no new disease |
| 2026-05 | outside | cardiology letter (A12): no dizziness; ECG first-degree AV block, no dropped beats on the tracing |
| 2026-06 / 2026-07 | outside | oncology surveillance CT stable (A7); surveillance colonoscopy 2026-07: healthy anastomosis, no polyps or recurrence (A11) |
| after 2026-08-27 | practice | booked routine follow-up visit |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source's profile and first encounter already list stage IIIB lung cancer,
  status post type A dissection with tamponade, ascending aortic graft, pericardial
  window and port, yet the ED encounter is the dissection itself and the oncology note
  says the cancer was recently diagnosed. Resolution: at joining and on 2021-10-28 he has
  only hypertension, hyperlipidemia and first-degree AV block. The dissection, tamponade,
  pericardial window and graft come with the 2022-11-04 ED visit; the cancer and port in
  September 2024. The ED note's "prior type A dissection" is not carried: this is the
  first. The oncology note's "presenting in 2021" follows the re-dated timeline (2022).
- **R2.** Every source note says "quit 18 months ago". Resolution: that is true only on
  2021-10-28; he quit in late April 2020, and later notes say he quit in 2020. He was a
  current smoker when he joined. 40 pack-years throughout.
- **R3.** Encounter 0 ends in quiz text with no plan. Resolution: the PCP discusses
  ezetimibe in plain terms (no trial numbers) and he decides to stay on atorvastatin
  alone; every later list (S) carries only lisinopril, atorvastatin and multivitamin.
- **R4.** Later source lists never add a drug after the dissection, the cancer or the
  endocarditis. Resolution: his long-term list stays lisinopril 20 mg, atorvastatin 40 mg
  and multivitamin; time-limited cancer and infection drugs (folic acid, ceftriaxone,
  durvalumab) come and go as above; infusion chemotherapy and premedication appear only
  in oncology letters.
- **R5.** Encounter 3 says he is not up to date on cancer screening. Resolution: colonoscopy
  is offered and declined at joining, a FIT kit is never returned, and later visits note
  screening still outstanding and his reluctance; no colorectal screening is ever done
  before 2025.
- **R6.** Encounter 2's targeted-therapy trial (hazard ratio and interval) is quiz text.
  Resolution: he asks about a targeted drug; the oncologist explains his tumor has no
  driver mutation for it (A6) and gives no figures.
- **R7.** Encounter 3's TEE names the mitral valve, while the diagnosis and the cardiology
  note name the aortic valve. Resolution: the vegetation is on the aortic valve (two
  source statements, and the later conduction disease fits); the mitral valve has no
  vegetation. The apical holosystolic murmur stays as the PCP heard it.
- **R8.** Encounter 4 calls the rhythm "irregularly irregular", which is not Wenckebach,
  and hears no murmur after the earlier murmur. Resolution: the exam records an
  irregular rhythm; the ECG gives the diagnosis; the murmur had faded after treatment
  (gone by 2025-09-03).
- **R9.** The source gives Mobitz I only at the 2025-11-06 cardiology visit, after the
  practice last saw him. Resolution: it appears in the cardiology letters, not on the
  practice's problem list.
- **Left as the source has it:** no beta-blocker or added blood pressure drug after the
  dissection (pressures in the 130s on lisinopril alone), no aspirin, statin unchanged.
  No one comments on it.

## Additions (A): physician to confirm

- **A1.** 2019-09-17 new-patient visit (encounter 0 is a routine follow-up).
- **A2.** Joining ECG with PR 224 ms to support the listed first-degree AV block.
- **A3.** Annual low-dose CT lung screening for a 40 pack-year smoker: Lung-RADS 1 in
  2019, 2020 and 2021; not repeated in 2022 to 2023 because of his aortic CT imaging;
  resumed 2024-08-20.
- **A4.** CT angiography confirming the type A dissection, and the admission with its
  emergency repair (the source lists the graft and window but not the event).
- **A5.** Cardiac surgery follow-up letters, the 2023-01 cardiology visit with a normal TTE (cardiology follows him from then on), and the 2023-11 CT angiography.
- **A6.** How the stage IIIB cancer was found and staged: the 2024-08-20 LDCT (4.2 cm right
  upper lobe mass, nodes as above, Lung-RADS 4B), PET-CT, MRI brain, EBUS biopsy, and
  EGFR/ALK/ROS1 negative.
- **A7.** Standard treatment of unresectable stage IIIB adenocarcinoma: concurrent
  carboplatin/pemetrexed with 60 Gy radiation (2024-10-03 to 2024-11-14, 4 cycles), folic
  acid and B12 injections with pemetrexed, then durvalumab 2025-01-09 to 2025-12-18
  (held during the 2025 admissions); CTs showing partial response then stability.
- **A8.** 900 mL removed at the thoracentesis.
- **A9.** Encounter 3 numbers the source leaves out: weight about 84 kg, temperature 37.9 C,
  WBC 11.8, hemoglobin 11.2, ESR 62, CRP 48 mg/L. Weights otherwise: about 92 kg 2019 to
  2024, about 89 kg October 2024, 88 kg December 2024 to February 2025, 86 kg September 2025.
- **A10.** Organism Streptococcus gallolyticus, penicillin-susceptible; ceftriaxone 2 g IV
  daily for 6 weeks through the port; the admission and ID follow-up.
- **A11.** The colon mass's course: biopsy adenocarcinoma, staging CT negative, laparoscopic
  left hemicolectomy 2025-07-16 (pT3 N0, 0/18 nodes, stage IIA), no adjuvant therapy,
  1-year colonoscopy normal.
- **A12.** Cardiology's usual next steps for symptomatic Mobitz I: 14-day monitor, TTE,
  avoid nodal blockers, no pacemaker, follow-up letters as above.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may carry
  the descriptors its kind always carries, but nothing that adds a territory, a severity
  grade or a symptom, or changes the diagnosis or plan.
- Records from the prior practice may mention routine care there, with normal results.
- Hypertension and lipid visits about every 6 months until 2022, then visits around each
  event; yearly physical with CMP, CBC, lipids; BP in the 120s to 130s on lisinopril, LDL
  around 95 to 105 on atorvastatin 40 mg; mild anemia only during chemoradiation and the
  2025 infection (hemoglobin 11 to 12.5), normal otherwise.
- Prevention: influenza each fall, COVID vaccines, Shingrix series, PCV20 at 65, RSV once
  (2024), Tdap; hepatitis C and HIV screening once (negative); PHQ-2 screens, mildly
  raised once after the cancer diagnosis without a new diagnosis.
- Refills, pharmacy and prior-authorization traffic, portal messages about results and
  scheduling, a cancelled visit, oncology and cardiology letters copied to the PCP.
- Booked visit after 2026-08-27: a routine follow-up.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002252 on 2025-09-03. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. A visit is booked after today.
