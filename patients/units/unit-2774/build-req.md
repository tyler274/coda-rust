# Build requirements: unit-2774 (Anjali Rahman, MRN00002774)

Source: synthetic_hospital patient 2774, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, its second-person exam text ("you note..."), or the
broken "She is." under medications.

## Who she is (S)

Born 1970-05-03, female, Asian (South Asian, Indian), not Hispanic or Latino, English
speaking, lives in Williamsburg, Massachusetts; private employer-sponsored insurance.
Administrative assistant. Emigrated from India about 2012 (ten years before the
November 2022 cardiology consult). Never smoker, no alcohol, no drug use. No known drug allergies.

Acute rheumatic fever at age 8 (1978), treated in hospital, never followed by long-term
antibiotic prophylaxis; no records from that time. Surgical history: tonsillectomy at
11 (1981), wisdom teeth extraction at 22 (1992). No known coronary or lung disease.
Family history: father hypertension and a stroke at 70; mother type 2 diabetes and
hypothyroidism; no family history of congenital heart disease or rheumatic fever.

Joined the practice on 2022-05-02 (a new-patient visit, the source's first encounter;
R1), at 51, about four years and four months before today. The seed's
years_with_practice of 5 is a size target, not a date; the source's first encounter is a
first presentation, so the chart keeps that date and does not invent an earlier joining
(R1). She brings only the
history of childhood rheumatic fever and six months of breathlessness; no prescription
drugs (R2). Her prior US primary care records come with her (P).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2022-05-02 | practice | new-patient visit (R1). Six months (since about November 2021) of progressive exertional dyspnea, orthopnea and paroxysmal nocturnal dyspnea, with a mild persistent cough mostly when lying down; no chest pain, palpitations or leg swelling. Low-pitched diastolic rumble at the apex with her in the left lateral decubitus position. Labs: Hb 13.2 g/dL, BNP 150 pg/mL (R4), creatinine 0.9 mg/dL. Office ECG: sinus rhythm with left atrial enlargement. Transthoracic echo ordered (R3). For the congestion, **furosemide 40 mg daily and potassium chloride 20 mEq daily start today** (R2) |
| 2022-05-09 | outside | transthoracic echo at a regional hospital echo lab (R3): thickened mitral valve, narrowed orifice, diastolic doming of the anterior leaflet; estimated pulmonary artery systolic pressure 45 mmHg; left ventricular end-diastolic pressure normal; measurements per A1. Report received by the practice |
| 2022-05-16 | practice | echo review and diuretic follow-up; basic metabolic panel on furosemide (P). **Rheumatic heart disease with severe mitral stenosis** and **heart failure from the stenosis, NYHA class III** go on the problem list; the elevated pulmonary pressure estimate is noted in the assessment (R5). Referral to cardiology placed (P) |
| 2022-10-17 to 2022-10-19 | outside | admission at a regional hospital for atrial fibrillation of sudden onset (the hospital stay the November consult mentions); course per A2. **Paroxysmal atrial fibrillation** diagnosed. **Metoprolol succinate 50 mg daily** and **warfarin 5 mg daily, INR goal 2.0 to 3.0** start here (R2). Discharge summary received |
| 2022-10-25 | practice | post-discharge visit (P); still in rate-controlled atrial fibrillation, pulse irregular in the 80s (A4); INR monitoring at the practice begins |
| 2022-11-14 | outside | first cardiology consult (R6): a year of progressive breathlessness and falling exercise tolerance, occasional palpitations, the October admission. Vitals recorded as stable (irregular pulse under 100, P). Low-pitched apical diastolic rumble in left lateral decubitus with an opening snap after S2 (R7); bibasilar crackles. Office echo: restricted mitral opening, enlarged left atrium. ECG: irregularly irregular rhythm. Severe rheumatic mitral stenosis, NYHA class III. Medications unchanged: warfarin, furosemide, metoprolol succinate, potassium chloride |
| 2023-01-23 | practice | heart failure and anticoagulation visit (P): pulse regular; office ECG shows sinus rhythm with left atrial enlargement, the paroxysm over (A4) |
| 2024-10-08 | practice | referral visit (P): breathlessness worse over recent months, with the symptoms the November letter gives; referred back to cardiology. No drug change |
| 2024-11-01 | outside | cardiology follow-up, her first since November 2022 (R6). Exertional breathlessness has progressed to breathlessness at rest; stairs are hard; paroxysmal nocturnal dyspnea; a dry cough on and off; ankles swollen lately; no chest pain, palpitations, syncope or orthopnea (R8). T 36.8 °C (98.2 °F), BP 112/76, pulse 84 and regular, RR 20. Mildly dyspneic at rest; JVP elevated; bibasilar crackles without wheeze; loud S2; grade 4/6 low-pitched apical diastolic rumble in left lateral decubitus, louder with handgrip, with a palpable thrill and no opening snap (R7); mild bilateral ankle edema, pitting. Office echo: mitral leaflets thick and fixed, opening only slightly; findings of raised left atrial pressure; mitral regurgitation insignificant; aortic valve normal. ECG: sinus rhythm, left atrial enlargement, right ventricular hypertrophy. **Pulmonary hypertension secondary to the valve disease** diagnosed (R5). Medications unchanged |
| 2025-10-27 | outside | routine cardiology follow-up: orthopnea and paroxysmal nocturnal dyspnea progressing; a single recent bout of small-volume hemoptysis; no chest pain, fever or weight loss. BP 130/85, HR 92 and irregularly irregular, SpO2 96% on room air. Bibasilar crackles; apical diastolic rumble in left lateral decubitus. Office echo: thickened mitral valve, narrowed orifice, left atrial enlargement. ECG: atrial fibrillation, no ST changes or T-wave inversions. Medications unchanged |
| 2025-11-04 | practice | routine follow-up after the cardiology letter (P); still in rate-controlled atrial fibrillation, pulse irregular in the 80s (A4); INR in range; no further blood in sputum (P) |
| 2026-02-02 | practice | heart failure and anticoagulation visit (P): pulse regular; office ECG shows sinus rhythm, the paroxysm over (A4) |
| 2026-04-26 to 2026-04-30 | outside | regional hospital, admitted to the cardiac ICU on a Sunday: breathlessness and palpitations of sudden onset, coughing up pink frothy sputum. BP 130/80, HR 120 and irregularly irregular, SpO2 89% on room air. Bibasilar crackles; apical diastolic rumble with an opening snap after S2 (R7). Chest x-ray: pulmonary edema, enlarged left atrium. ECG: atrial fibrillation, no ST changes. Acute pulmonary edema with atrial fibrillation and rapid ventricular response; course and discharge per A3. Discharge summary received |
| 2026-05-05 | practice | post-discharge visit (P): breathing back to her usual exertional limit, lungs clear, no edema, pulse regular; BMP and INR. Work note for the days missed. Cardiology follow-up booked for 2026-05-18 (A5) |
| 2026-05-18 | outside | cardiology follow-up after the admission; valve intervention planned, transesophageal echo ordered (A5) |
| 2026-06-03 | outside | transesophageal echo and structural heart consult at the regional hospital; commissurotomy booked, warfarin hold planned (A5) |
| 2026-06-16 to 2026-06-17 | outside | percutaneous mitral balloon commissurotomy at the regional hospital, one night; discharge summary received (A5) |
| 2026-06-23 | practice | INR after the warfarin hold (A5) |
| 2026-07-28 | outside | cardiology follow-up with office echo six weeks after the commissurotomy (A5) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The first encounter reads as a first presentation ("presents to the clinic with a
  6-month history"), and the cardiology note calls it her "presentation". Resolution: it is
  her new-patient visit; she joined the practice that day. The source gives her age as 52 at
  that encounter; from the seed's birth date she is 51 on 2022-05-02 (52 the next day), and
  every note uses the seed's ages. The seed gives about five years with the practice; joining
  on 2022-05-02 gives about four years and four months by today. Joining stays at the source's
  first presentation: an earlier joining would need an invented visit at which her severe
  stenosis went unheard or unrecorded. The seed's figure is a size target and is not carried.
- **R2.** The source lists warfarin, furosemide, metoprolol succinate and potassium chloride
  from the first visit, before any diagnosis that calls for them. Resolution: she joins on no
  prescription drugs. Furosemide 40 mg and potassium chloride 20 mEq start at the 2022-05-02
  visit for her congestion. Warfarin 5 mg (INR goal 2.0 to 3.0) and metoprolol succinate 50 mg
  start at the October 2022 atrial fibrillation admission. From then on all four drugs are on
  every list, as the source has them, at the source's doses.
- **R3.** The first encounter places a transthoracic echo inside an office visit. Resolution:
  the practice orders it and it is done a week later, on 2022-05-09, at a regional hospital echo
  lab, carrying the source's findings. The office ECG stays at the visit. The cardiology echoes
  are done in the cardiology office on the day of each consult (A5's hospital echoes aside).
- **R4.** The first encounter lists BNP twice, once as 150 pg/mL and once as "Elevated".
  Resolution: one BNP, 150 pg/mL (reference under 100), flagged high.
- **R5.** The profile lists every condition from the start, while the source's stamps place the
  mitral stenosis at the first encounter and pulmonary hypertension at the third (practice date
  2024-11-01), although the first echo already estimates a pulmonary artery systolic pressure
  of 45 mmHg. Resolution: at joining only the childhood rheumatic fever is known. Rheumatic
  heart disease with severe mitral stenosis and heart failure enter the problem list in May
  2022 from the echo; paroxysmal atrial fibrillation enters in October 2022. The 2022 pressure
  estimate is noted as elevated pulmonary pressure from the stenosis, and pulmonary
  hypertension secondary to the valve disease becomes a diagnosis at the November 2024
  cardiology visit, with the loud S2, raised JVP and right ventricular hypertrophy on ECG. The
  source's duplicate entries are one problem each: "rheumatic heart disease (severe mitral
  stenosis)" and "rheumatic mitral stenosis"; the two "pulmonary hypertension" lines; "chronic
  heart failure (NYHA class III)" and "stable chronic heart failure".
- **R6.** The cardiology notes say she "returns for follow-up ... following January 2020
  presentation" and was "last evaluated in 2020". Resolution: on the re-dated timeline the
  practice refers her in May 2022, the November 2022 consult is her first cardiology visit, and
  the November 2024 visit is the first since then.
- **R7.** An opening snap is heard in November 2022 and April 2026 and is absent in November
  2024, when the valve is immobile and a thrill is felt. Different examiners; each finding stays
  as written, and no note reconciles them.
- **R8.** In November 2024 she reports paroxysmal nocturnal dyspnea but denies orthopnea,
  having had orthopnea in 2022 and again in 2025. It is her report at that visit and stays as
  written.
- **R9.** The source's comorbidity list includes left ventricular hypertrophy, which no
  encounter supports: her blood pressure is never high, every echo is silent on it, left
  ventricular end-diastolic pressure is normal, and the only hypertrophy any ECG shows is right
  ventricular, which fits her pulmonary hypertension. Resolution: left ventricular hypertrophy
  is not carried. Echo reports describe normal left ventricular wall thickness.
- **Left as the source has it:**
  - No drug or dose changes after October 2022: furosemide stays 40 mg through the 2024 edema
    and raised JVP, and metoprolol succinate stays 50 mg after the 2026 rapid ventricular
    response.
  - The 2025 hemoptysis on warfarin gets no chest imaging or other workup.
  - Six months pass between the diagnosis and the first cardiology consult, and two years
    between the first and second cardiology visits.
  - The heart failure stays NYHA class III on every list, including when she is breathless at
    rest in 2024 and after the 2026 pulmonary edema.

## Additions (A): physician to confirm

- **A1.** The first echo (2022-05-09) states what the source's "severe mitral stenosis" needs:
  mitral valve area 1.2 cm² by planimetry, mean transmitral gradient 9 mmHg, left atrium
  enlarged (matching the ECG), normal left ventricular size, wall thickness and systolic
  function (ejection fraction 60%), no significant mitral regurgitation, normal aortic valve.
  The later cardiology echoes up to April 2026 carry only the source's descriptions plus normal left ventricular
  size and function; they give no new valve area, gradient or pressure figure.
- **A2.** The October 2022 admission (the source's "recent hospital visit"): she came to the
  emergency department with palpitations, was found in atrial fibrillation with a rapid rate,
  and was admitted for two nights. Rate controlled with metoprolol, then metoprolol succinate
  50 mg daily; warfarin 5 mg daily started because of the mitral stenosis, INR goal 2.0 to 3.0;
  TSH and troponin normal; no cardioversion. Discharged in rate-controlled atrial fibrillation
  with INR checks at the practice and a cardiology appointment.
- **A3.** The April 2026 ICU stay: oxygen, intravenous furosemide and intravenous then oral
  metoprolol; INR 2.4 on arrival, warfarin continued; troponin normal; potassium and creatinine
  normal. The rhythm returns to sinus on the second day and oxygen is weaned; she moves to a
  cardiac floor on 2026-04-28 and goes home on 2026-04-30 on her unchanged home regimen
  (warfarin 5 mg, furosemide 40 mg, metoprolol succinate 50 mg, potassium chloride 20 mEq),
  with cardiology follow-up advised.
- **A4.** The rhythm between the source's atrial fibrillation findings, so every visit has one.
  The source places her in atrial fibrillation in October and November 2022, in October 2025
  and in April 2026, and in sinus rhythm (regular pulse, P waves of left atrial enlargement) in
  May 2022 and November 2024; a paroxysmal course needs each episode to end. The course:
  - October 2022 paroxysm: she stays in rate-controlled atrial fibrillation through the
    2022-10-25 visit and the 2022-11-14 consult, and converts on her own; at the 2023-01-23
    practice visit her pulse is regular and an office ECG shows sinus rhythm with left atrial
    enlargement. Sinus rhythm, regular pulse, at every visit from then until October 2025.
  - October 2025 paroxysm: still in rate-controlled atrial fibrillation (pulse irregular in the
    80s) at the 2025-11-04 practice visit; converts on her own; at the 2026-02-02 practice visit
    her pulse is regular and an office ECG shows sinus rhythm. Sinus rhythm from then until the
    April 2026 admission.
  - April 2026 paroxysm: ends on the second hospital day (A3); sinus rhythm, regular pulse, at
    every visit from discharge to today.
  No cardioversion, no drug change for the rhythm.
- **A5.** Valve intervention for the severe symptomatic rheumatic mitral stenosis, which the
  source asserts and never treats beyond diuretic, rate control and anticoagulation. Run by her
  cardiologist and the regional hospital's structural heart team; the practice manages the
  warfarin around it. Every source encounter through the April 2026 admission shows the valve
  still severely narrowed (fixed leaflets in 2024, pulmonary edema in 2026), so the intervention
  starts at the cardiology follow-up the April 2026 discharge summary advises, after the last
  source event; before then no note discusses valve intervention, as the source has it.
  - 2026-05-18, cardiology clinic (letter received 2026-05-20): back at her usual exertional
    limit; BP 116/76, pulse 70 and regular, SpO2 97% on room air; lungs clear, apical diastolic
    rumble, no edema; ECG sinus rhythm with left atrial enlargement. Valve intervention is
    indicated: percutaneous mitral balloon commissurotomy if the anatomy suits and the left
    atrium is free of thrombus. Transesophageal echo ordered on warfarin; referred to the
    structural heart team. Medications unchanged.
  - 2026-06-03, regional hospital echo lab, transesophageal echo under moderate sedation, then
    the structural heart cardiologist the same day (report and letter received 2026-06-05).
    Sinus rhythm, heart rate 66. No left atrial or left atrial appendage thrombus. Leaflets
    thickened with fusion of both commissures; Wilkins score 8 (mobility 2, thickening 2,
    calcification 2, subvalvular 2); mitral valve area 1.1 cm² by planimetry; mean gradient
    10 mmHg; trivial mitral regurgitation; left atrium enlarged; normal left ventricle, ejection
    fraction 60%; normal aortic valve. Commissurotomy recommended (surgical replacement
    discussed as the alternative) and booked for 2026-06-16. Warfarin: last dose the evening of
    2026-06-11, no bridging, INR on arrival must be under 1.5, 5 mg restarted the evening of the
    procedure, INR at the practice on 2026-06-23. Other drugs continue.
  - 2026-06-16 to 2026-06-17, regional hospital, one night: percutaneous mitral balloon
    commissurotomy by the structural heart cardiologist; INR 1.3 on arrival; right femoral
    venous access, transseptal puncture, intravenous heparin during the case, Inoue balloon;
    mean transmitral gradient 10 mmHg before and 4 mmHg after; no complications; sinus rhythm
    throughout. Next-morning transthoracic echo: mitral valve area 1.9 cm² by planimetry, mean
    gradient 4 mmHg, mild mitral regurgitation, no pericardial effusion, ejection fraction 60%.
    Home on the unchanged regimen, warfarin 5 mg restarted 2026-06-16. Discharge summary
    received 2026-06-18.
  - 2026-06-23, practice INR 1.8, below goal a week after the restart; daily dose stays 5 mg
    with no extra dose; back in range at the 2026-06-29 check.
  - 2026-07-28, cardiology clinic with office transthoracic echo (letter received 2026-07-31):
    breathing easier than before the procedure (stairs still slow her but she no longer stops
    partway; two pillows; no night waking since); BP 118/74, pulse 68 and regular, SpO2 98% on
    room air; lungs clear, softer apical rumble, groin healed, no edema. Echo: mitral valve area
    1.9 cm², mean gradient 4 mmHg, mild mitral regurgitation, estimated pulmonary artery
    systolic pressure 35 mmHg, left atrium enlarged, normal left ventricle. Plan: medications
    unchanged; warfarin indefinitely, INR goal 2.0 to 3.0, monitored at the practice;
    transthoracic echo yearly; cardiology clinic in six months (January 2027).
  No new diagnosis: the problem list keeps the source's conditions. No drug or dose change; the
  warfarin hold of 2026-06-12 to 2026-06-15 is the only interruption.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention
  (kidney function, heart rate under 100, no wheeze, normal aortic valve). An abnormal finding
  appears only as the source or this file gives it. A report may carry the descriptors its kind
  always carries (how a size was measured, a procedure's access site), but nothing that adds a
  territory, a severity grade or a symptom, or changes the diagnosis or plan.
- Records from her prior US primary care practice may mention routine screening done there,
  with normal results (a cervical co-test in 2019, a mammogram in 2021).
- Heart failure and anticoagulation visits about every 3 months, with a yearly physical.
  Practice vitals: BP in the 110s to 130s over 70s to 80s, pulse 60s to 80s, regular in sinus rhythm
  and irregular only in the atrial fibrillation spans A4 sets out, SpO2 95 to 98% on room air, weight steady.
  Practice notes describe stable exertional breathlessness between the source's visits and the
  worsening the source gives before November 2024 and October 2025; she follows a low-sodium
  diet and weighs herself daily. After May 2026 she is back to her usual exertional limit, and breathes
  easier after the June commissurotomy (A5).
- INR checks every 2 to 4 weeks at the practice, mostly 2.0 to 3.0 on warfarin 5 mg daily; an
  occasional value just outside range is handled with a one-day dose adjustment and a recheck,
  with the daily dose staying 5 mg. Warfarin counseling on vitamin K foods, bleeding signs and
  interacting drugs.
- Basic metabolic panel every 3 to 6 months on furosemide and potassium chloride, potassium and
  creatinine normal; CBC yearly, hemoglobin about 13; lipids once or twice, normal; A1c every
  year or two for her South Asian ancestry and her mother's diabetes, normal; TSH normal (A2).
- Age- and sex-appropriate prevention: cervical co-test in 2024, normal; mammogram every one to
  two years, normal; FIT each year from 2022, negative; influenza each fall; COVID vaccines;
  pneumococcal conjugate vaccine once for her heart disease; shingles series; RSV vaccine in
  2025 for her heart failure; Tdap if due; hepatitis C and HIV screening once; depression
  screens at physicals, negative. Lung cancer screening does not apply (never smoker).
- Refills, pharmacy traffic, portal messages about INR results, refills and scheduling, a
  missed visit, reminders; a work note after the April 2026 admission.
- Booked visit after 2026-08-27: a routine heart failure and anticoagulation follow-up on
  2026-09-14, with an INR check and her influenza vaccine.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.


## Last seen

The practice last saw MRN00002774 on 2026-05-05. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. Nothing is booked after today.
