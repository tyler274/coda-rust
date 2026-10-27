# unit-2116 build-req: Piotr Kelleher (MRN00002116)

Every clinical fact in the chart traces to **S** (the source as carried here), **R** (a
resolution below), **A** (an addition below) or **P** (the primary-care layer), and
nothing else. Texture (voice, scheduling, refills, portal traffic, admin, missed
visits) is free and carries no new clinical fact. Re-voice everything: no source
sentence, clinician name, problem-list dump, "(diagnosed ...)" stamp or quiz text.

## Who he is (S)

- Piotr Kelleher, male, born 1967-12-27, Williamsburg MA 01096. White, not Hispanic or
  Latino, English. Commercial (employer-sponsored) insurance. Phone +1-413-555-0120,
  piotr.kelleher@example.test. Single.
- Occupation: warehouse logistics coordinator.
- Tobacco: never smoker. Alcohol: occasional, 1-2 drinks per week. No drug use.
- Allergies: no known drug allergies.
- Surgical history: none before 2025-11 (see timeline).
- Family history: father with a bicuspid aortic valve, diagnosed at age 50; paternal
  uncle died of a ruptured ascending aortic aneurysm at age 64; mother with
  hypertension. No other family members are named.
- Joins the practice 2021-09-14 (A1) with no known problems and no medications.
  Earlier care at a prior primary-care office arrives as outside records (P).

## Timeline

| Date | Where | What happens |
|---|---|---|
| 2021-09-14 | Practice, new-patient physical (A1) | Establishes care, age 53. No problems, no meds, NKDA. Family and social history as above. Normal exam, normal screening labs (P). Weight 84.0 kg, height 178 cm. |
| 2023-04-27 | Practice office visit (S, encounter 0) | Age 55. About a year of shortness of breath walking uphill, stops to rest because of chest tightness. No history of coronary disease or diabetes. BP 128/78, pulse 78, RR 14. Harsh crescendo-decrescendo systolic murmur loudest at the 2nd right intercostal space radiating to both carotids, softer with Valsalva (R1); single second heart sound; no peripheral edema. Office ECG (A2). Labs (P). Echocardiogram ordered (A3). |
| 2023-05-09 | Practice-ordered TTE (A3) | **Bicuspid aortic valve** and **aortic valve stenosis** first documented (onset 2023-04-27, R2). Findings in A3. PCP calls with results and places a cardiology referral (A3). |
| 2024-07-01 | Outside cardiology consult (S, encounter 1) | Age 56. Follows his April 2023 evaluation (R3). Progressive fatigue and exertional chest pain over the past 6 months; occasional lightheadedness after stairs or walking uphill. No medications. BP 122/76, pulse 74 and regular. 3/6 crescendo-decrescendo systolic murmur loudest at the right upper sternal border radiating to the carotids. Assessment per R4; repeat TTE (A4). |
| 2024-07-09 | Outside TTE (A4) | Findings in A4. Cardiology addendum: plan per R4. |
| 2025-11-09 | Outside ED, regional hospital (S, encounter 2) | Age 57, Sunday. Sudden severe tearing chest pain radiating to the back, unrelenting, began while lifting heavy boxes. Known bicuspid valve since 2023 (R3). BP 95/50 right arm, 75/40 left arm. Diaphoretic, visibly distressed, diminished femoral pulses, muffled heart sounds, soft murmur. Chest X-ray: widened mediastinum. ECG: nonspecific ST and T wave changes without signs of infarction. Bedside TTE for complications (A5). CTA, labs, transfer (A5). |
| 2025-11-09 to 2025-11-18 | Outside tertiary cardiac surgery center (A6) | **Acute type A aortic dissection** confirmed. Emergency repair the night of 11-09. **Aortic valve stenosis resolved** by valve replacement; **presence of prosthetic heart valve** and **aortic graft** recorded. **Start metoprolol succinate 50 mg daily and aspirin 81 mg daily**; short course oxycodone-acetaminophen. Discharged home 2025-11-18. |
| 2025-11-24 | Practice transitional-care visit (A7) | Post-discharge check. Findings in A7. Cardiac rehab referral. |
| 2025-12-16 / 12-17 | Outside 1-month CTA and cardiac surgery follow-up (A8) | Findings in A8. |
| 2026-01-06 | Outside cardiology follow-up with TTE (A8) | Findings in A8. Dental prophylaxis advised (A9). |
| 2026-01 to 2026-03 | Outside cardiac rehab (A8) | Completes program. |
| 2026-02-12 | Practice 3-month post-op visit (A7) | Findings in A7. Returns to work 2026-02-16 with a lifting restriction (A10). |
| 2026-03-10 | Practice portal | **Start amoxicillin dental prophylaxis** (A9). |
| 2026-05-12 / 05-19 | Outside 6-month CTA and cardiology follow-up (A8) | Findings in A8. 12-month CTA ordered for 2026-11 (left open). |
| 2026-09-22 | Practice, booked | Annual physical (P). |

## Resolutions (R)

1. **Handgrip.** The source says the murmur grows louder with handgrip, which does
   not fit aortic stenosis. The handgrip response is not carried; the chart keeps the
   murmur softening with Valsalva.
2. **Diagnosis dates.** The source stamps the bicuspid valve "diagnosed" at encounter 0
   on exam alone. Both bicuspid aortic valve and aortic valve stenosis are recorded at
   the 2023-05-09 echocardiogram (A3) with onset 2023-04-27.
3. **Years in source text.** "A January 2020 evaluation" becomes the April 2023
   evaluation; "bicuspid aortic valve since 2020" becomes since 2023. The cardiology
   visit falls 14 months after the referral because he did not schedule it; the
   office re-sends the referral in 2024-05 when his symptoms worsen (P texture).
4. **Unfinished cardiology plan.** "Suspects a valvular abnormality" with a known
   bicuspid valve is replaced by: symptomatic moderate aortic stenosis of a bicuspid
   valve, repeat TTE (A4); after the echo, continued surveillance with clinic and
   echo in 12-18 months, sooner for worse symptoms. His surveillance visit is booked
   for 2025-12 when the dissection occurs.
5. **Exam differences.** The harsh murmur with a single S2 (2023), the 3/6 murmur
   (2024) and the soft murmur in shock (2025) stay as each examiner wrote them.
6. **ED encounter without a diagnosis.** The presentation is carried as written and
   its workup confirms acute type A aortic dissection (A5, A6).

**Left as the source has it:** no echocardiogram before 2023 despite his father's
bicuspid valve; no screening of relatives; no stress test or coronary evaluation for
exertional chest pain with moderate stenosis; no drug for the valve before the
dissection; no statin. Nobody remarks on these.

## Additions (A)

1. **Joining date** 2021-09-14 (seed: about five years with the practice). He brings
   no problems and no medications. Weight 84.0 kg, height 178 cm.
2. **Office ECG 2023-04-27** for exertional chest tightness: normal sinus rhythm, rate
   76, normal axis and intervals, no ST or T wave changes.
3. **TTE 2023-05-09** to evaluate the murmur: bicuspid aortic valve with fusion of the
   right and left coronary cusps, mildly calcified, moderate aortic stenosis (peak
   velocity 3.4 m/s, mean gradient 25 mmHg, valve area 1.2 cm2), no aortic
   regurgitation; normal LV size and wall motion, LVEF 60%; aortic root 3.6 cm,
   ascending aorta 3.7 cm; no pericardial effusion. PCP refers to cardiology
   2023-05-11.
4. **TTE 2024-07-09** (cardiology): bicuspid aortic valve, moderate aortic stenosis
   (peak velocity 3.7 m/s, mean gradient 30 mmHg, valve area 1.1 cm2), no aortic
   regurgitation; LVEF 60%; aortic root 3.7 cm, ascending aorta 3.8 cm. Plan per R4.
5. **ED workup 2025-11-09.** Vitals on arrival: HR 112, RR 22, SpO2 96% on room air,
   temperature 36.6 C, with the source pressures. Bedside TTE: intimal flap in the
   proximal ascending aorta, moderate pericardial effusion without right ventricular
   collapse, calcified bicuspid aortic valve, normal LV function. CTA chest, abdomen
   and pelvis: Stanford type A dissection from the sinotubular junction through the
   arch into the descending thoracic aorta, ending above the diaphragm; left
   subclavian origin involved; ascending aorta 4.6 cm; hemopericardium; abdominal
   aorta and visceral, renal and iliac arteries normal. Labs: CBC, BMP and PT/INR within normal limits,
   high-sensitivity troponin I 6 ng/L. Cardiac surgery consulted; transferred
   to a tertiary center by critical-care transport.
6. **Repair and hospital course 2025-11-09 to 2025-11-18.** Emergency sternotomy:
   hemopericardium evacuated; aortic root replacement with a 25 mm bioprosthetic valved
   conduit (Bentall) and hemiarch replacement under hypothermic circulatory arrest.
   Pathology: bicuspid aortic valve with calcific degeneration; aortic wall with acute
   dissection. ICU 3 days, extubated on post-op day 1. Pre-discharge TTE: well-seated
   bioprosthesis, mean gradient 10 mmHg, no paravalvular leak, LVEF 55%, no pericardial
   effusion. Pre-discharge CTA: intact graft, residual dissection of the distal arch and
   descending thoracic aorta with a patent false lumen, descending aorta 3.4 cm.
   Discharge hemoglobin 10.4. Discharge medications: metoprolol succinate 50 mg daily,
   aspirin 81 mg daily, oxycodone 5 mg-acetaminophen 325 mg every 6 hours as needed
   (20 tablets). Blood pressure goal below 120/80. Sternal precautions: no lifting
   over 10 lb for 8 weeks, no driving for 4 weeks. Cardiac rehab recommended.
7. **Post-op primary care.** 2025-11-24 (TCM): BP 112/70, HR 62, weight 80.6 kg, sternal
   incision healing; CBC and BMP, hemoglobin 10.9, otherwise normal; oxycodone short course
   winding down; cardiac rehab referral placed. Short-term disability paperwork.
   2026-02-12: BP 114/72, HR 60, weight 82.4 kg; CBC with hemoglobin 13.8.
8. **Surveillance.** 2025-12-16 CTA: graft intact, residual dissection unchanged,
   descending aorta 3.4 cm. 2025-12-17 cardiac surgery: sternum stable, cleared for
   cardiac rehab and driving. 2026-01-06 cardiology with TTE: bioprosthesis normal, mean
   gradient 11 mmHg, no paravalvular leak, LVEF 55-60%; BP 116/72; continue metoprolol
   and aspirin; CTA at 6 and 12 months after repair. Cardiac rehab 2026-01-08 to
   2026-03-31, 36 sessions, completed without events. 2026-05-12 CTA: stable, descending
   aorta 3.5 cm, false lumen patent. 2026-05-19 cardiology: stable, BP 118/74, next CTA
   2026-11.
9. **Endocarditis prophylaxis** for the prosthetic valve: amoxicillin 2 g (four 500 mg
   capsules) by mouth 30-60 minutes before dental procedures, prescribed by PCP
   2026-03-10 before a dental cleaning.
10. **Return to work** 2026-02-16 with a standing restriction from cardiac surgery: no
    lifting over 40 lb and no straining or breath-holding when lifting.

## Primary-care layer (P)

- Annual physicals each fall (2021 establish-care physical, 2022, 2023, 2024, 2025;
  2026-09-22 booked), with normal exams apart from the murmur once it is known, and
  vitals taken. BP normal throughout before the dissection.
- Labs with normal results: CBC, CMP, lipid panel and hemoglobin A1c at physicals
  (his own levels: total cholesterol 170, LDL 98, HDL 48, triglycerides 120, A1c 5.2,
  TSH 1.9);
  hepatitis C antibody and HIV screen once (2021, negative); CBC, BMP, lipid panel and
  TSH at the 2023-04-27 visit.
- Prior-practice records (received 2021-09): routine physicals with normal labs,
  screening colonoscopy 2018 normal (repeat in 10 years), Tdap 2016, COVID-19 primary
  series (Pfizer) spring 2021.
- Immunizations: influenza each fall; COVID boosters at a pharmacy each fall;
  Shingrix two doses (2021-09-14 and 2021-11-30).
- PHQ-2 at physicals, negative. Normal AUDIT-C consistent with occasional drinking.
- Post-op: metoprolol refills by PCP, pharmacy requests, portal messages on results,
  scheduling and forms; one missed visit (2022-09-13, rescheduled).
- His exertional symptoms after 2024-07 are as cardiology describes, no worse and no new
  symptoms, until the dissection.
- No acute illnesses, injuries, new diagnoses or life events beyond the timeline.


## Last seen

The practice last saw MRN00002116 on 2026-02-12. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. Nothing is booked after today.
