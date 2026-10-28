# Build requirements: unit-2801 (Jessenia Castro, MRN00002801)

Source: synthetic_hospital patient 2801, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz-style phrasing ("you note ...").

## Who she is (S)

Born 1980-04-05, female, Hispanic, Medicaid, English-speaking, lives in the seed's
town. Administrative assistant. Never smoker, no alcohol. No known drug allergies.
Surgical history: tonsillectomy at age 8. Family history: mother had rheumatic fever;
father has type 2 diabetes; a maternal uncle had an ischemic stroke at 65.

Rheumatic fever in childhood; told in adolescence that the mitral valve was narrowed
(rheumatic), and has had no cardiology follow-up for years. Her symptoms first became
noticeable during her most recent pregnancy, in 2017, when a diuretic and a low-salt
diet were started (R1). She has stayed on hydrochlorothiazide with potassium since.

Joined the practice in 2020 (first practice visit 2020-09-22, a new-patient visit, A1),
bringing: rheumatic heart disease (history), hydrochlorothiazide 25 mg daily and
potassium chloride 20 mEq daily. She is in sinus rhythm and not anticoagulated at
joining; metoprolol and warfarin start later (R2).

Height 160 cm; weight about 71 to 74 kg throughout the practice years (P).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2020-09-22 | practice | new-patient visit (A1): history as above; exam notes an apical diastolic murmur, lungs clear, no edema; BP 116/74. Routine labs. Cardiology referral to re-establish valve follow-up placed (A2); a screening mammogram ordered |
| 2021-09-21 | practice | routine follow-up; feels well, active; the 2020 cardiology referral was never scheduled and is closed, she will call if symptoms change (A2) |
| 2022-09-13 | practice | several months of exertional breathlessness on stairs and with housework, fatigue, occasional palpitations, ankles swollen by evening (A3); no chest pain, cough or orthopnea. Apical diastolic murmur, mild ankle edema. CBC, BMP and TSH normal. **Cardiology referral** placed |
| 2022-10-11 | outside | cardiology consult (age 42): progressive exertional dyspnea and fatigue over months, occasional palpitations, evening leg swelling; denies chest pain, cough, orthopnea; no cardiology follow-up for years. HR 92, BP 118/72. Loud S1, low-pitched apical diastolic rumble heard best with the bell in left lateral decubitus; bibasilar crackles; mild bilateral ankle edema. Chest x-ray: mild pulmonary vascular congestion. ECG and echo (A4). **Rheumatic mitral stenosis (severe) and chronic (rheumatic) heart failure diagnosed.** **Metoprolol succinate 50 mg daily started**; hydrochlorothiazide and potassium continued; sodium restriction. 48-hour Holter ordered. Letter received 2022-10-24 |
| 2022-10-18 | outside | Holter hooked up at cardiology (A5) |
| 2022-11-08 | outside | cardiology reviews the Holter (A5): **paroxysmal atrial fibrillation diagnosed**; **warfarin 5 mg daily started, adjusted to INR 2.0 to 3.0**, managed by the cardiology group's anticoagulation clinic. The letter and Holter report do not come at the time; they arrive 2023-01-10 after the practice asks (records gaps) |
| 2022-11-22 | practice | post-cardiology visit: she reports the new diagnoses and warfarin; the practice learns of warfarin from her and the pharmacy fill and requests the records; long-term anticoagulant use added; warfarin counseling (bleeding, no NSAIDs, not safe in pregnancy) |
| 2024-04-15 | outside | cardiology follow-up: exertional dyspnea and fatigue gradually worse over the last several years, now new orthopnea, despite taking her hydrochlorothiazide as prescribed. Mild tachycardia and tachypnea (HR 104, RR 21, R3), BP 112/82. Loud S1, prominent opening snap after S2, low-pitched late apical diastolic rumble, palpable right ventricular heave at the left sternal border; no peripheral edema, no JVD. Chest x-ray: mild left atrial enlargement, pulmonary vascular redistribution. Repeat echo (A6). **Pulmonary hypertension due to mitral stenosis (WHO group 2) diagnosed.** No medication change; continue sodium restriction, follow-up in cardiology. This letter is not sent; it arrives 2024-08-13 after the practice asks |
| 2024-07-24 | practice | **last practice visit** (seed last_seen): breathing worse since spring, now sleeps on two pillows; saw cardiology in April and says they found pressure in the lungs; the letter is not in the chart and is requested. BMP and CBC drawn, normal. No medication change; return in 3 months (she does not) |
| 2026-03-31 | outside | ED (age 45): acute shortness of breath, palpitations and a cough worse over a few hours. BP 130/85, HR 120 irregularly irregular, RR 22, SpO2 92% on room air. Apical diastolic rumble in left lateral decubitus; bibasilar crackles. Chest x-ray: elevated left mainstem bronchus and left atrial enlargement, with interstitial edema (R4). ECG: atrial fibrillation, no ST changes. ED labs and treatment (A7). **Acute decompensated heart failure with pulmonary edema**, atrial fibrillation with rapid ventricular response. Admitted. ED record received 2026-04-21 |
| 2026-03-31 to 2026-04-03 | outside | hospital admission (A8). Discharge summary arrives late, 2026-08-11 |
| 2026-05-19 | outside | structural heart consult (A9); letter received 2026-06-02 |
| 2026-07-21 | outside | percutaneous balloon mitral valvotomy (A9); the practice knows only from her portal message of 2026-07-28; no report has arrived by today |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source places the start of symptoms in "her most recent pregnancy" without
  a date, yet she presents in 2022 at 42 having had no follow-up for years.
  Resolution: the most recent pregnancy was in 2017, before she joined; an outside
  obstetric team started the diuretic and low-salt diet then, and her prior primary
  care continued hydrochlorothiazide 25 mg with potassium chloride 20 mEq. No other
  obstetric detail is carried and no pregnancy occurs in the chart.
- **R2.** The source lists metoprolol succinate 50 mg and warfarin 5 mg (INR 2.0 to 3.0)
  as home medications from the first cardiology encounter, and paroxysmal atrial
  fibrillation, severe mitral stenosis, pulmonary hypertension and chronic heart
  failure on every list, while the first note says she had no follow-up for years.
  Resolution: at joining she has rheumatic heart disease and the thiazide only.
  Severe rheumatic mitral stenosis and chronic heart failure are diagnosed at the
  2022-10-11 cardiology consult, where metoprolol starts; paroxysmal atrial
  fibrillation is found on the Holter and warfarin starts 2022-11-08. Pulmonary
  hypertension is diagnosed 2024-04-15 (the source's own stamp). The source's
  "Rheumatic heart disease", "Severe mitral stenosis" and "Rheumatic mitral
  stenosis" are one valve disease: rheumatic heart disease (history) from joining and
  rheumatic mitral stenosis from 2022-10-11. "Secondary pulmonary hypertension (WHO
  Group 2)" and "Pulmonary hypertension secondary to mitral stenosis" are one
  condition. The 2022-10-11 cardiology letter may list all four drugs as current
  (copied from that day's plan), as outside lists do.
- **R3.** The 2024 visit gives "mild tachycardia and tachypnea" without numbers.
  Resolution: HR 104 and RR 21.
- **R4.** The ED diagnosis is pulmonary edema, but the source's chest x-ray names only
  the left atrial signs. Resolution: the ED chest x-ray also reports interstitial
  pulmonary edema, which the diagnosis and crackles carry. No effusion, no
  consolidation.
- **R5.** The 2024 note dates the first presentation "January 2020"; on the re-dated
  timeline that is October 2022, and the 2024 letter says so.
- **R6.** Interobserver differences stand as written: the 2022 consult hears bibasilar
  crackles and ankle edema, the 2024 consult finds no edema or JVD and does not
  mention the lungs (chart: lungs clear in 2024).
- **Left as the source has it:** from 2022 to the March 2026 ED visit she stays on
  hydrochlorothiazide 25 mg as her only diuretic, and no valve intervention is
  offered, even with severe mitral stenosis, new orthopnea and pulmonary
  hypertension in 2024. No visit comments on it. The practice, not seeing her after
  July 2024, never sees colorectal screening (due at 45) done.

## Additions (A): physician to confirm

- **A1.** 2020-09-22 new-patient visit (years with the practice: 6).
- **A2.** A 2020 cardiology referral to re-establish valve follow-up that she never
  schedules, closed 2021-09-21; it reconciles "no follow-up for years" with a known
  valve disease in a primary-care chart.
- **A3.** The 2022-09-13 practice visit that sends her to cardiology.
- **A4.** At the 2022-10-11 consult: 12-lead ECG, sinus rhythm 92, no ST changes.
  Transthoracic echo: rheumatic mitral valve with commissural fusion and diastolic
  doming of the anterior leaflet; mitral valve area 1.1 cm² by planimetry; mean
  transmitral gradient 10 mmHg at HR 92; left atrium enlarged (LA volume index
  52 mL/m²); LV normal size, LVEF 60%; RV normal size and function; estimated PA
  systolic pressure 38 mmHg; no significant regurgitation of any valve; no
  pericardial effusion. Supports the source's severe mitral stenosis.
- **A5.** 48-hour Holter (2022-10-18 to 2022-10-20): predominantly sinus rhythm, mean HR
  88; three episodes of atrial fibrillation, longest 3 h 10 min, AF burden 6%,
  ventricular rate in AF up to 138; palpitations logged during AF. Supports the
  source's paroxysmal atrial fibrillation and the warfarin it lists.
- **A6.** Repeat echo 2024-04-15: mitral valve area 0.9 cm²; mean gradient 13 mmHg at
  HR 104; LA volume index 58 mL/m²; LVEF 60%; RV mildly dilated with preserved
  function; estimated PA systolic pressure 56 mmHg. Supports the pulmonary
  hypertension diagnosis and the RV heave.
- **A7.** ED 2026-03-31: NT-proBNP 2860 pg/mL; high-sensitivity troponin I 9 ng/L
  (normal); INR 2.4; sodium 137, potassium 3.7, creatinine 0.84; hemoglobin 12.4,
  WBC 7.6. Furosemide 40 mg IV, metoprolol tartrate 5 mg IV twice, oxygen 2 L by nasal
  cannula (SpO2 96%). Admitted to cardiology telemetry.
- **A8.** Admission 2026-03-31 to 2026-04-03: IV furosemide; spontaneous return to sinus
  rhythm on 2026-04-01; warfarin continued (INR in range). Inpatient echo
  2026-04-01: mitral valve area 0.8 cm², mean gradient 14 mmHg at HR 78, estimated PA
  systolic pressure 62 mmHg, LVEF 60%. Discharge medications: **furosemide 40 mg daily
  started, hydrochlorothiazide stopped, metoprolol succinate increased to 100 mg
  daily**, warfarin 5 mg daily (INR 2.0 to 3.0) and potassium chloride 20 mEq daily
  continued. Referred to the structural heart program for balloon valvotomy.
- **A9.** Structural heart consult 2026-05-19: severe symptomatic rheumatic mitral
  stenosis, breathing better since discharge (walks a block, stairs slowly), valve
  anatomy suitable on the echoes; recommends percutaneous balloon mitral valvotomy
  after a transesophageal echo to exclude left atrial thrombus; warfarin continued.
  Valvotomy performed 2026-07-21; per her message, home the next day and breathing
  easier. No numbers reach the chart.
- **A10.** Warfarin counseling at the 2022-11-22 visit includes that warfarin is not safe
  in pregnancy; she does not plan a pregnancy. No contraceptive drug is added.

## Records

The seed's texture is gaps. The 2022-10-11 consult letter arrives on time. The
2022-11-08 Holter and warfarin letter comes only after the practice asks (2023-01-10).
The 2024-04-15 letter and echo arrive 2024-08-13 after a request. The anticoagulation
clinic faxes INR logs only now and then (P). The ED record arrives three weeks late;
the discharge summary arrives in August 2026; the valvotomy report has not come.
After 2024-07-24 nothing is reconciled: the practice's lists still show
hydrochlorothiazide 25 mg, metoprolol succinate 50 mg, warfarin and potassium, and the
admission's changes appear only in the letters, a pharmacy call and her messages.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may
  carry the descriptors its kind always carries, but nothing that adds a territory, a
  severity grade or a symptom, or changes the diagnosis or plan.
- Records from her prior primary care may mention a normal Pap in 2018 and normal
  basic labs.
- Practice visits every 6 to 12 months before 2022, every 4 to 8 months after; yearly
  physicals (2021, 2022, 2023, 2024-03-19); BMP for the thiazide and potassium, CBC,
  lipids, A1c (father's diabetes), TSH once, all normal; PT/INR results from the
  anticoagulation clinic's faxed logs, mostly in range with ordinary wobble.
- Prevention: screening mammograms 2020, 2022 and 2024, normal; Pap with HPV co-test
  2021, negative; Tdap 2021; influenza each fall 2020 to 2023 at nurse visits; COVID
  primary series and booster at a pharmacy in 2021 (reported) and an updated dose in
  2023; PCV20 in 2023 for her heart disease; HIV and hepatitis C screening once
  (negative); PHQ-2 or PHQ-9 yearly, negative.
- Refills (electronic requests, 90-day fills, a short fill with "needs visit" after she
  stops coming), pharmacy calls, portal messages about results and scheduling, a
  cancelled visit in 2023, recall messages after July 2024 that she does not act on.
- No visit is booked after today.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002801 on 2024-07-24. Practice contacts after that day do
not happen except messages, refills and records; outside care after it still does, and
its records are filed as received without changing the practice's lists. No visit is
booked after today.
