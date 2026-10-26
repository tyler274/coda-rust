# unit-2583 build requirements

Every clinical fact in this chart traces to **S** (the source as carried here), **R**
(a numbered resolution), **A** (a numbered addition) or **P** (the primary-care
layer below), and to nothing else. Texture (voice, scheduling, refills, portal and
admin traffic, missed visits) is free and carries no clinical fact. Re-voice
everything: no source sentence, clinician name, problem-list dump or diagnosis stamp
is copied. Clinicians are named by role only; no real practice, hospital, lab or
pharmacy is named.

## Who he is (S)

- Rowan Doherty, MRN00002583, male, born 2002-04-08, Deerfield MA 01342. White, not
  Hispanic or Latino, English. Phone +1-860-555-0135, rowan.doherty@example.test.
  Single. Private insurance through a parent's plan.
- University student; runs track at school. Rides a motorcycle (it appears only in
  the 2026 crash).
- Never smoker. Occasional social alcohol.
- Allergies: no known drug allergies.
- Surgical history: none before 2026-03-06 (R1). Craniotomy with evacuation of
  epidural hematoma on 2026-03-06.
- Family history: father died suddenly at age 41 (sudden cardiac death);
  paternal uncle has hypertrophic cardiomyopathy (R4, R5).
- **Joins the practice 2022-08-22** (A1), age 20, healthy: no problems, no
  medications, NKDA. Height about 180 cm, weight about 72 kg (P).

## Timeline

| Date | Where | What happens |
|---|---|---|
| 2022-08-22 (Mon) | Practice, establish care | A1. New patient, healthy, no meds, NKDA. Father's sudden death at 41 recorded by patient report, cause unknown to him (R5). HIV and hepatitis C screening, lipid panel, CBC, CMP: normal (P). |
| 2022-09 | Outside records | Prior pediatric practice records: healthy childhood, routine immunizations complete (P). |
| 2023-03-02 (Thu) | (not a contact) | Collapses at track practice after brief chest tightness. |
| 2023-03-03 (Fri) | Practice, office visit | A2. Reports yesterday's collapse with chest tightness just before he lost consciousness; well now. Soft systolic murmur at left lower sternal border. Office 12-lead ECG: normal sinus rhythm, no abnormality. Told to stop running and exertion until cardiology sees him. **Urgent cardiology referral.** |
| 2023-03-13 (Mon) | Outside, cardiology consult (S, enc 0) | Collapse at track practice with brief chest tightness before losing consciousness. Vitals stable. Systolic murmur at left lower sternal border. Transthoracic echo: asymmetric septal hypertrophy with systolic anterior motion of the mitral valve, no structural valve abnormality (measurements A3). **Diagnosis: hypertrophic obstructive cardiomyopathy. Starts metoprolol succinate 50 mg daily** (R2). Family history taken: father sudden cardiac death at 41, paternal uncle with HCM. Advised to stay out of competitive running and strenuous exertion (A3). Cardiology follow-up in a year. |
| 2024-03-06 (Wed) | Outside, ED (S, enc 1) | Age 21. Recurrent transient loss of consciousness: several episodes, often preceded by nausea, warmth and lightheadedness; today friends saw him pale and sweaty before he slumped to the ground. On metoprolol succinate 50 mg daily. BP 120/75, HR 70. Alert and oriented. ECG while asymptomatic: normal sinus rhythm, no abnormality. Has an episode on the monitor: sinus bradycardia, then a brief asystolic pause, then back to sinus rhythm. **Diagnosis: syncope.** Disposition R7. Labs A4. |
| 2024-03-11 (Mon) | Practice, office visit | P. Post-ED visit; ED record reviewed; cardiology follow-up already arranged. |
| 2024-03-18 (Mon) | Outside, cardiology follow-up | A5. Echo unchanged. Reflex mechanism favored. Metoprolol unchanged. |
| 2025-03-24 (Mon) | Outside, cardiology annual | A6. No further syncope. ECG normal sinus rhythm. Metoprolol unchanged. |
| 2026-03-06 (Fri) | Outside, admission, neurosurgery (S, enc 2) | Age 23. Motorcycle crash without a helmet, found unconscious at the scene, regains consciousness on arrival at the ED. Severe headache. BP 140/90, HR 100, RR 18, T 37.0 °C (98.6 °F). Alert and oriented but restless; no focal neurological deficit. CT head: hyperdense biconvex extra-axial collection that does not cross suture lines (location and size A7). **Diagnosis: traumatic epidural hematoma. Emergency craniotomy with evacuation the same day.** Course and discharge A7. Metoprolol continued. |
| 2026-03-10 (Tue) | Outside, discharge | A7. Discharged home. |
| 2026-03-13 (Fri) | Practice, office visit | P. Post-discharge (transitional care) visit. |
| 2026-03-17 (Tue) | Practice, nurse visit | A8. Staple removal. |
| 2026-04-13 (Mon) | Outside, neurosurgery follow-up | A9. **Epidural hematoma resolved.** |
| 2026-06-08 (Mon) | Outside, cardiology annual | A10. Echo unchanged. Metoprolol unchanged. |
| 2026-09-14 (Mon) | Practice, booked | Annual physical, booked (P). |

## Resolutions (R)

1. The source lists "craniotomy with evacuation of epidural hematoma" as surgical
   history from the first encounter (2023). The injury happens on 2026-03-06; the
   craniotomy is done at that admission. No earlier record lists any surgery.
2. Metoprolol succinate 50 mg daily is listed as a home medication at the cardiology
   encounter that first diagnoses HOCM. It starts at that consult (2023-03-13) and
   continues unchanged at that dose to today.
3. The first encounter puts HOCM on an active problem list while the patient "denies
   prior medical issues". HOCM is diagnosed at that consult; before it he has no
   problems.
4. The father's age at sudden death is 39 in the first encounter and 41 in the
   profile and later encounters. **41** is carried; 39 is not.
5. The paternal uncle's HCM first appears in the 2024 ED note. It is recorded in the
   family history the cardiologist takes at the 2023-03-13 consult. At establish care
   (2022) the patient knows only that his father died suddenly at 41, cause unknown
   to him; after the diagnosis it is recorded as sudden cardiac death.
6. The source stamps syncope as diagnosed at the ED encounter (re-dated 2024-03-06).
   Syncope enters the problem list there. The 2023 collapse is charted as the
   presentation of HOCM, not as a separate syncope diagnosis.
7. The ED encounter has no disposition. He is kept on telemetry in the ED with no
   further episodes after the recorded one, cardiology is consulted by phone, and he
   is discharged home the same night with cardiology follow-up within two weeks,
   metoprolol unchanged.
8. The source CT is "performed and shows"; the chart carries it as a CT head report
   with the source's finding, plus location and size (A7). The admission has no
   plan; the craniotomy from the surgical history is the plan.
9. The source gives no duration for loss of consciousness at the scene; the
   epidural hematoma is coded with loss of consciousness of unspecified duration.

**Left as the source has it** (kept without comment):
- No ICD, pacemaker or other device; no genetic testing; no cardiac MRI; no
  ambulatory rhythm monitor or exercise test.
- Metoprolol dose never changed, including after the recorded bradycardia and pause.
- No documented screening of other family members.
- Nobody links the 2026 crash to syncope.

## Additions (A)

1. **Establish care 2022-08-22**, so that the four years with the practice and the
   referral before the cardiology consult exist. Healthy, no medications, NKDA.
2. **PCP visit 2023-03-03**, the visit that made the cardiology referral: soft
   systolic murmur at the left lower sternal border; office 12-lead ECG normal sinus
   rhythm with no abnormality (matches the source's later normal ECG); exertion held;
   urgent cardiology referral.
3. **Echo measurements 2023-03-13**, to support the source's obstructive HCM:
   maximal septal wall thickness 21 mm, posterior wall 10 mm, LVEF 70%, resting LVOT
   peak gradient 38 mmHg rising to 64 mmHg with Valsalva. Normal LV cavity size, no
   structural valve abnormality. Advice to avoid competitive running and strenuous
   exertion is the consult's plan.
4. **ED labs 2024-03-06**: CBC, basic metabolic panel and high-sensitivity troponin
   I, all normal.
5. **Cardiology follow-up 2024-03-18** (the follow-up the ED arranged): prodrome of
   nausea, warmth and lightheadedness favors a reflex mechanism, with the recorded
   bradycardia and pause during an episode; no further episodes since the ED.
   Metoprolol succinate 50 mg daily continued. Counseled on hydration, sitting or
   lying down at the first warning, and leg-crossing and handgrip counter-pressure.
   Echo unchanged: septum 21 mm, LVEF 70%, resting gradient 36 mmHg, 60 mmHg with
   Valsalva. Next visit in a year.
6. **Cardiology annual 2025-03-24**: no syncope since 2024-03; exam unchanged
   (systolic murmur at left lower sternal border); ECG normal sinus rhythm with no
   abnormality; metoprolol continued; next visit in a year with echo.
7. **Admission 2026-03-06 to 2026-03-10**: CT head shows a right temporal epidural
   hematoma, maximal thickness 16 mm, estimated volume 34 mL, midline shift 3 mm; no
   other intracranial hemorrhage, no fracture identified. CT cervical spine: no
   fracture or malalignment. Admission CBC, basic metabolic panel and PT/INR normal.
   Emergency right temporal craniotomy with evacuation the same day. Postoperative
   day 1 CT head: satisfactory evacuation, no residual or new hemorrhage. Two nights
   in the neuro ICU, then the ward. Levetiracetam 500 mg twice daily for 7 days
   (seizure prophylaxis) and acetaminophen 1000 mg every 8 hours as needed (short
   courses). Metoprolol continued throughout. Neurologically intact at discharge,
   2026-03-10, home. Staples out in 7 to 10 days; neurosurgery follow-up with CT in
   about a month; helmet counseling.
8. **Nurse visit 2026-03-17**: staples removed, incision healed.
9. **Neurosurgery follow-up 2026-04-13**: CT head shows resolution of the epidural
   hematoma with postoperative changes only; neurologically intact; cleared for
   normal activity and classes; helmet counseling. Epidural hematoma resolved.
10. **Cardiology annual 2026-06-08** (moved from March by the admission): no syncope
    since 2024-03; echo unchanged: septum 21 mm, LVEF 70%, resting gradient 35 mmHg,
    62 mmHg with Valsalva; metoprolol continued; next visit in a year.

## Primary-care layer (P)

- Annual physicals late August 2023, 2024 and 2025, and one booked 2026-09-14. CBC
  and basic metabolic panel at physicals, normal. PHQ-2 or PHQ-9 and AUDIT-C, low.
  Blood pressure and heart rate normal.
- HIV and hepatitis C screening once (2022), negative. One lipid panel (2022),
  normal.
- Immunizations: childhood series complete per prior records (including Tdap at 12
  and HPV); influenza each fall; updated COVID-19 vaccine each fall; Tdap 2024.
- Metoprolol refills and pharmacy traffic, cardiology letters reviewed and
  acknowledged, portal messages about scheduling, forms for school, one missed visit,
  insurance (parental plan) admin.
- Normal exams except the systolic murmur at the left lower sternal border, heard at
  every visit from 2023-03-03 on, and the healing then healed right temporal
  craniotomy scar from 2026-03.
- No acute illnesses, injuries, new diagnoses or life events beyond the timeline.
