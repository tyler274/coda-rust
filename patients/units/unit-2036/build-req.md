# Build requirements: unit-2036 (Brian Goodwin, MRN00002036)

Source: synthetic_hospital patient 2036, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz text (the confidence-interval and
power questions, the cytochrome c / apoptosis teaching).

## Who he is (S)

Born 1963-03-31, male, White, not Hispanic or Latino, English-speaking, private
insurance, lives in Westhampton, MA 01027. Office administrator. Married (the source's
"relationship stress"; the spouse is never named and has no clinical story). Former
smoker, 20 pack-years, quit in 2013 (R4). Alcohol: 1 to 2 drinks a week; he notices his
tremor is better after a drink. No known drug allergies throughout.

Surgical history: drug-eluting stent to the LAD after a diagnostic cardiac
catheterization for stable angina in 2016, at an outside hospital, before he joined
(R1); laparoscopic right hemicolectomy with primary anastomosis in July 2022 (R2).

Family history: father with hypertension and essential tremor, father MI at 62;
mother with type 2 diabetes; older brother (born 1962) diagnosed with colorectal
cancer at 61, in 2023 (R5).

Joined the practice 2019-09-10 (A1, new-patient visit). He brought: essential
hypertension (poorly controlled), coronary artery disease with stable angina and the
2016 LAD stent, paroxysmal atrial fibrillation (diagnosed 2018 by an outside
cardiology group, R3), former smoking. Medications on joining: lisinopril 20 mg daily,
hydrochlorothiazide 25 mg daily, metoprolol succinate 50 mg daily, aspirin 81 mg
daily, isosorbide mononitrate ER 30 mg daily. Height 178 cm, weight 89 kg.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2019-09-10 | practice | new-patient visit (A1): history as above; BP 148/92; labs; colorectal screening recommended, he defers (A3); lung screening ordered (A4) |
| 2021-03-09 | practice | (P) referral visit: hand tremor for about 5 months; TSH and chemistry drawn, normal; referred to neurology |
| 2021-04-14 | outside | neurology consult letter: bilateral hand tremor for 6 months, worse holding a cup or writing, better after alcohol; no head injury, no seizures; BP 142/88; postural bilateral upper-extremity tremor, no rigidity, no bradykinesia, rest of the neurological exam normal; routine bloods and thyroid tests normal. **Essential tremor diagnosed.** Already on metoprolol; no new drug (R6) |
| 2021-05-04 | practice | evaluated for enrollment in an outside randomized trial of a new antihypertensive (pressure poorly controlled on lisinopril and hydrochlorothiazide); eligibility reviewed, information given, he chooses to enroll (R7) |
| 2021-06-01 | practice | trial baseline visit: randomized to the study drug or placebo (blinded, dispensed by the study site); usual medications unchanged; baseline 12-lead ECG shows normal sinus rhythm (R3, R7); primary outcome is the drop in systolic pressure at 6 months |
| 2021-11-23 | outside | (R7) final 6-month trial visit; study drug stops; the practice hears of it by portal message |
| 2021-12-02 | practice | (A5) three months of increasing fatigue, pale skin, unexplained weight loss (89 to 83 kg), night sweats and occasional low-grade fevers. CBC: hemoglobin 8.5 g/dL, WBC 64.0 x10^3/uL with a left shift (metamyelocytes and promyelocytes on smear), platelets 600 x10^3/uL. Urgent hematology/oncology referral |
| 2021-12-10 | outside | hematology/oncology consult letter: same symptoms; splenomegaly, pale conjunctiva, mild tachycardia (HR 104, A5); hemoglobin 8.5, leukocytosis with left shift, platelets 600,000; bone marrow biopsy hypercellular with increased granulopoiesis, BCR-ABL1 fusion detected. **Chronic myeloid leukemia, BCR-ABL positive, chronic phase, diagnosed. Imatinib 400 mg daily started** (A6) |
| 2022-03 to 2025 | outside | (A6) hematology letters: BCR-ABL1 (IS) 7.1% at 3 months (2022-03), 0.86% (2022-06), 0.07% (2022-12, major molecular response), 0.03% (2023-12), 0.02% (2024-12); counts normal on imatinib by 2022-03 (hemoglobin 13.4, WBC 7.6, platelets 286) |
| 2022-05-24 | practice | (A7) referral visit: 3 months of a little bright blood on the toilet paper after bowel movements, no pain, no change in bowel habit, no weight loss; GI referral |
| 2022-06-13 | outside | gastroenterology consult letter: same history; external hemorrhoids; abdomen soft, nontender, no masses; rectal exam without blood on the glove, prostate nontender and normal size. Plan: colonoscopy, given his age and that he has never been screened (R8) |
| 2022-06-28 | outside | (R2) colonoscopy: hemorrhoids, and a partly circumferential mass in the ascending colon, biopsied; rest of the colon normal. Pathology: moderately differentiated invasive adenocarcinoma. **Colorectal adenocarcinoma diagnosed** |
| 2022-07-06 | outside | (R2) CT chest, abdomen and pelvis with contrast: ascending colon wall thickening at the known mass; two subcentimeter liver lesions too small to characterize; chest clear |
| 2022-07-21 to 07-24 | outside | (R2) admission for **laparoscopic right hemicolectomy with primary anastomosis**; uneventful recovery. Pathology pT3 N1b (2 of 18 nodes), margins negative |
| 2022-07-29 | practice | (P) post-discharge visit |
| 2022-08-10 | outside | (R2) MRI liver: three metastases in both lobes, largest 1.4 cm. **Stage IV (liver metastases)**; not resectable at this point per oncology |
| 2022-08-24 | outside | (R2) oncology letter: **capecitabine 1500 mg twice daily, 14 days on and 7 off, starting 2022-09-06**; imatinib continues; CT every 3 to 4 months |
| 2022-11-08 | practice | (P) referral visit: erectile dysfunction about 5 months; urology referral |
| 2022-12-06 | outside | urology consult letter: erectile dysfunction for 6 months, causing strain in his relationship; BP 135/85, pulse 78; rest of the exam unremarkable; renal and liver function normal. **Erectile dysfunction diagnosed.** PDE-5 inhibitors not offered because of his daily isosorbide mononitrate; non-drug options discussed; no drug started (R9) |
| 2023 to 2026 | outside | (A8) oncology surveillance CTs of chest, abdomen and pelvis: liver lesions smaller on the December 2022 CT (largest 1.1 cm), then stable; no new disease; capecitabine continues |
| 2023-06 | practice | (R5) patient reports his older brother has been diagnosed with colorectal cancer at 61; family history updated |
| 2024-11-05 | outside | (A9) cardiology letter: more frequent palpitation spells; ECG during a spell shows atrial fibrillation with normal QRS duration; **flecainide 100 mg twice daily started** for rhythm control; metoprolol continues |
| 2025-04-08 | outside | ED record: age 62; sudden palpitations and lightheadedness while jogging; BP 130/85, HR 160, RR 18; sodium 140, potassium 4.2, creatinine 1.0, troponin I <0.01 ng/mL; ECG atrial fibrillation with rapid ventricular response and a widened QRS, earlier ECGs atrial fibrillation with normal QRS. **Flecainide adverse effect.** Flecainide held; converted on its own to sinus rhythm with a narrow QRS after a few hours of monitoring; discharged home (A10) |
| 2025-04-14 | practice | (A10) post-ED visit: **flecainide stopped** on the practice list; metoprolol continues |
| 2025-04-24 | outside | (A10) cardiology letter: flecainide permanently discontinued; ECG sinus rhythm, normal QRS; rate control with metoprolol |
| 2025-09-29 | outside | hematology/oncology letter: two weeks of profound fatigue; pancytopenia on routine chemotherapy labs (WBC 1.9 x10^3/uL, ANC 0.8, hemoglobin 8.7, platelets 64, A11); bone marrow biopsy: markedly increased apoptotic bodies in hematopoietic precursors, increased p53 and BAX expression by immunohistochemistry, blasts not increased (R10). **Chemotherapy-induced pancytopenia. Capecitabine held**; imatinib continues; weekly CBC (A11) |
| 2025-10-14 | practice | (A11) follow-up: fatigue easing; CBC WBC 2.9, hemoglobin 9.4, platelets 108 |
| 2025-11-10 | outside | (A11) oncology letter: counts recovered (WBC 4.3, ANC 2.4, hemoglobin 11.3, platelets 161); **pancytopenia resolved; capecitabine restarted at 1000 mg twice daily, 14 days on and 7 off** |
| 2026-06-19 | practice | last practice visit (annual physical); lists current as of that day |
| 2026-07 | outside | (A8) oncology letter and CT, stable; filed, lists not changed |
| 2026-09-22 | practice | booked: routine follow-up |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The profile lists CAD with stable angina, the stent and isosorbide from the
  first encounter. Resolution: the stent was placed in 2016, before he joined; stable
  angina is quiescent on metoprolol and isosorbide mononitrate throughout the chart.
- **R2.** The profile lists stage IV colorectal adenocarcinoma on capecitabine and the
  hemicolectomy from the first encounter, yet "colorectal neoplasia" is first diagnosed
  at the 2022 GI visit for minimal rectal bleeding without weight loss. Resolution: the
  cancer is unknown until the 2022 colonoscopy; the hemicolectomy and capecitabine
  follow it; staging becomes IV on the liver MRI. The 2021 neurology letter, trial
  visits and December 2021 hematology letter carry neither the cancer nor capecitabine.
  The bleeding is attributed to hemorrhoids; the right-sided mass is found because the
  colonoscopy was done. "Colorectal neoplasia" and "stage IV colorectal adenocarcinoma"
  are one condition (malignant neoplasm of colon), with liver metastases as a second
  coded condition.
- **R3.** The profile lists atrial fibrillation from the start, and "normal sinus
  rhythm" is a diagnosis in 2020. Resolution: atrial fibrillation is paroxysmal, known
  since 2018; the trial baseline ECG shows sinus rhythm. Normal sinus rhythm is an ECG
  reading, never a condition.
- **R4.** "Quit 8 years ago" is repeated at every encounter from 2020 to 2024.
  Resolution: he quit in 2013 (8 years before the 2021 neurology visit).
- **R5.** The 2022 GI note says no family history of colorectal cancer; the profile
  says an older brother had it at 61. Resolution: the brother (born 1962) is diagnosed
  in 2023 at 61, after Brian's own diagnosis; the GI letter was right when written.
  The neurology letter's "denies family history of movement disorders" conflicts with
  the father's tremor; resolution: the neurology letter records the father's tremor.
- **R6.** The profile lists essential tremor from the start; the neurology visit
  establishes it. Resolution: tremor begins about October 2020 and is diagnosed
  2021-04-14. No tremor drug is added; neurology notes he already takes a beta-blocker.
- **R7.** The trial visits are vignette quiz text. Resolution: the practice visits
  carry eligibility review and the baseline visit only; the study drug is blinded and
  supplied by the outside study site, appears in notes only, never on the medication
  list, and stops at the 6-month visit (2021-11-23). Results are never reported to the
  practice. No confidence-interval or power text is carried.
- **R8.** The GI plan is cut off ("Given the patient's age and presentation,").
  Resolution: the plan is colonoscopy.
- **R9.** The source names no ED treatment. Resolution: the urologist does not offer a
  PDE-5 inhibitor because of the nitrate.
- **R10.** "Elevated cytosolic cytochrome c" is not a clinical test: not carried. The
  marrow's apoptotic bodies and p53/BAX staining are carried in the pathology report.
- **Outside medication lists** copy stale or incomplete entries as outside lists do
  (the neurology list puts lisinopril and HCTZ only in prose, the urology list omits
  aspirin, the ED list omits flecainide though the ED narrative names it). The
  practice list follows the resolved course.
- **Left as the source has it:** blood pressure stays above goal on lisinopril 20 mg
  and hydrochlorothiazide 25 mg for the whole chart; no third agent is added and no
  note explains why. He has coronary disease with a stent and is never on a statin or
  sublingual nitroglycerin. He has paroxysmal atrial fibrillation with hypertension and
  vascular disease and is on aspirin only, no anticoagulant. Flecainide is started by
  cardiology despite his coronary disease. Nobody comments on any of these.

## Additions (A): physician to confirm

- **A1.** New-patient visit 2019-09-10 (seed: 7 years with the practice; the first
  internal medicine source encounter reads as established care).
- **A2.** Outside cardiology follow-up letters about yearly (late each year, 2019 to
  2025): stable exertional angina, rare and unchanged; resting ECG sinus rhythm; plan
  unchanged.
- **A3.** Colorectal screening: recommended from 2019; a FIT kit sent at the 2020
  physical is never returned; no colonoscopy before 2022.
- **A4.** Lung cancer screening (20 pack-years, quit 2013): low-dose CT 2019-10,
  2020-10 and 2021-10, each Lung-RADS 1. After 2022 the oncology chest CTs replace it.
- **A5.** The December 2021 practice visit that finds the blood count, with the
  numbers above (hemoglobin and platelets from the source; WBC 64.0 added), and the
  hematology heart rate of 104.
- **A6.** Imatinib 400 mg daily (the source lists it; it starts at CML diagnosis) and
  its monitoring with the BCR-ABL1 values above.
- **A7.** The May 2022 referral visit for the bleeding.
- **A8.** The colon cancer's course after diagnosis: colonoscopy, CT, surgery, liver
  MRI, capecitabine (the dose the source lists), and surveillance CTs reported as
  smaller, then stable.
- **A9.** Flecainide (the source's "flecainide adverse effect" needs it to start
  somewhere), started by cardiology 2024-11-05.
- **A10.** The ED course after the source's findings (spontaneous conversion,
  flecainide held), the post-ED practice visit and the cardiology letter stopping it.
- **A11.** The pancytopenia's numbers, capecitabine held, weekly CBCs, recovery by
  2025-11-10 and restart at the reduced dose of 1000 mg twice daily.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may
  carry the descriptors its kind always carries, but nothing that adds a territory, a
  severity grade or a symptom, or changes the diagnosis or plan.
- Records from a prior practice may mention routine screening done there, with normal
  results (none of it colorectal).
- Blood pressure and chronic-care visits every 3 to 6 months (some by video in 2020),
  with vitals; office BP runs about 140 to 150 over 88 to 94. Basic metabolic panel
  twice a year, lipids and A1c yearly (normal), CBC yearly before the CML and as needed
  after it; weight about 89 kg to late 2021, 83 kg at the CML diagnosis, back to about
  86 kg by mid-2022, about 82 kg after surgery, about 84 kg on capecitabine, 81 kg at the
  pancytopenia and 83 kg by 2026.
- Annual physicals; PHQ-2 depression screens, negative; AUDIT-C low.
- Influenza vaccine each fall, COVID vaccines (primary series 2021, boosters and
  updated doses each fall), Shingrix series 2019 to 2020, Tdap 2019, PCV20 in January
  2022 after the CML diagnosis. Hepatitis C and HIV screening once (2019), negative.
- Refills, pharmacy and prior-authorization traffic, portal messages about results,
  refills and scheduling, one missed visit (2024), reminders.
- Booked visit after 2026-08-27: a routine follow-up.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002036 on 2026-06-19. Practice contacts after that day are
messages and refills only; outside care after it still happens, and its records are
filed as received without changing the practice's lists. A visit is booked after today.
