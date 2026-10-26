# synthetic_hospital patient 2199

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 66 at first encounter, sex M, Non-Hispanic White, Medicare Part B with supplemental Blue Cross Blue Shield.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-05-05 (Mon) | outpatient | Cardiology |
| 1 | 2020-06-15 | 2025-10-08 (Wed) | ed | Emergency Medicine |
| 2 | 2021-01-15 | 2026-04-26 (Sun) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare Part B with supplemental Blue Cross Blue Shield",
 "occupation": "Retired long-haul truck driver",
 "alcohol_use": "Social, 2-3 beers on weekends",
 "family_history": [
  "Father: Myocardial infarction at age 54, deceased from sudden cardiac arrest at age 61",
  "Mother: Ischemic stroke at age 74, hypertension",
  "Older brother: Type 2 diabetes mellitus, hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 1 pack per day for 42 years (started age 24)",
 "home_medications": [
  {
   "dose": "10 mg orally daily",
   "name": "Amlodipine"
  },
  {
   "dose": "1000 mg orally twice daily with meals",
   "name": "Metformin"
  },
  {
   "dose": "40 mg orally daily at bedtime",
   "name": "Atorvastatin"
  },
  {
   "dose": "81 mg orally daily",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "Laparoscopic cholecystectomy (2012)",
  "Left inguinal hernia repair with mesh (2008)"
 ],
 "chronic_conditions": [
  "Peripheral artery disease",
  "Hypertension",
  "Type 2 diabetes mellitus",
  "Hyperlipidemia"
 ],
 "age_at_first_encounter": 66
}
```

Primary diagnoses: ["Acute peripheral arterial thrombosis/embolism", "Non-ST elevation myocardial infarction (NSTEMI)", "Transient ischemic attack (amaurosis fugax) - antiplatelet therapy approach"]
Comorbidities: ["Tobacco use disorder", "Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2025-05-05, outpatient, Cardiology

Chief complaint: Several episodes of transient vision loss in his right eye over the past 2 weeks

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Elena Vasquez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Several episodes of transient vision loss in his right eye over the past 2 weeks

HISTORY OF PRESENT ILLNESS:
He describes the sensation as a “curtain coming down” that lasts a few minutes and then resolves completely. He has also noticed occasional dizziness and mild headaches but denies diplopia, syncope, or limb weakness.

PAST MEDICAL HISTORY:
Active Problem List:
- Peripheral artery disease
- Hypertension
- Type 2 diabetes mellitus
- Hyperlipidemia

His medical history includes long-standing hypertension, hyperlipidemia, and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (2012)
- Left inguinal hernia repair with mesh (2008)

MEDICATIONS:
Medications include amlodipine and metformin.

Home Medications:
- Atorvastatin 40 mg orally daily at bedtime
- Aspirin 81 mg orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 54, deceased from sudden cardiac arrest at age 61
- Mother: Ischemic stroke at age 74, hypertension
- Older brother: Type 2 diabetes mellitus, hypertension

SOCIAL HISTORY:
He smokes one pack of cigarettes daily and drinks alcohol socially.

VITAL SIGNS:
Blood pressure is 158/92 mmHg and pulse is 76/min

PHYSICAL EXAMINATION:
Funduscopic examination between episodes is normal. Neurologic exam shows no focal deficits.

LABORATORY DATA:
Laboratory studies reveal elevated LDL cholesterol and HbA1c above target.

## Encounter 1: source 2020-06-15, practice 2025-10-08, ed, Emergency Medicine

Chief complaint: Chest discomfort that began several hours ago

ENCOUNTER NOTE
Date: 2020-06-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Chest discomfort that began several hours ago

HISTORY OF PRESENT ILLNESS:
This patient with vascular disease and recent TIA presents with a tight, squeezing sensation behind the sternum with occasional radiation to the left shoulder.

PAST MEDICAL HISTORY:
Active Problem List:
- Transient ischemic attack (amaurosis fugax) - antiplatelet therapy approach (diagnosed 2020-01-15)
- Peripheral artery disease
- Hypertension
- Type 2 diabetes mellitus
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (2012)
- Left inguinal hernia repair with mesh (2008)

MEDICATIONS:
Home Medications:
- Amlodipine 10 mg orally daily
- Metformin 1000 mg orally twice daily with meals
- Atorvastatin 40 mg orally daily at bedtime
- Aspirin 81 mg orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 54, deceased from sudden cardiac arrest at age 61
- Mother: Ischemic stroke at age 74, hypertension
- Older brother: Type 2 diabetes mellitus, hypertension

SOCIAL HISTORY:
Tobacco use

LABORATORY DATA:
His troponin I level is elevated

OTHER STUDIES:
An ECG reveals ST depressions in the lateral leads

ASSESSMENT AND PLAN:
He is diagnosed with a non–ST-elevation myocardial infarction

Admitted to the coronary care unit. He is started on dual antiplatelet therapy, a high-intensity statin, beta-blocker, and intravenous unfractionated heparin, which requires close monitoring due to its variable pharmacokinetics. The clinical team plans to evaluate the therapeutic level of anticoagulation the following morning.

## Encounter 2: source 2021-01-15, practice 2026-04-26, ed, Emergency Medicine

Chief complaint: 6-hour history of severe pain in his left lower extremity

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Aisha Washington, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
6-hour history of severe pain in his left lower extremity

HISTORY OF PRESENT ILLNESS:
The patient is a 67-year-old male with peripheral artery disease, hypertension, type 2 diabetes mellitus, and hyperlipidemia, with prior transient ischemic attack and non-ST elevation myocardial infarction, who presents with a 6-hour history of severe left lower extremity pain. He describes the pain as sudden in onset, unrelieved by rest, and associated with a cold sensation in the foot.

PAST MEDICAL HISTORY:
Active Problem List:
- Transient ischemic attack (amaurosis fugax) - antiplatelet therapy approach (diagnosed 2020-01-15)
- Non-ST elevation myocardial infarction (NSTEMI) (diagnosed 2020-06-15)
- Peripheral artery disease
- Hypertension
- Type 2 diabetes mellitus
- Hyperlipidemia

PAST SURGICAL HISTORY:
Previous right femoral-popliteal bypass

MEDICATIONS:
Home Medications:
- Amlodipine 10 mg orally daily
- Metformin 1000 mg orally twice daily with meals
- Atorvastatin 40 mg orally daily at bedtime
- Aspirin 81 mg orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 54, deceased from sudden cardiac arrest at age 61
- Mother: Ischemic stroke at age 74, hypertension
- Older brother: Type 2 diabetes mellitus, hypertension

SOCIAL HISTORY:
History of smoking

PHYSICAL EXAMINATION:
On examination, the left foot is pale and cool to touch with diminished pulses compared to the right foot. Sensation is decreased, and motor strength is 3/5 in the left foot dorsiflexors and plantar flexors. Capillary refill time is prolonged. The patient&#x27;s ankle-brachial index (ABI) is 0.5 on the left and 0.9 on the right.

LABORATORY DATA:
Laboratory Values:
- Complete Blood Count (CBC)
- Hemoglobin: 13.2 g/dL
- Hematocrit: 39.8%
- White Blood Cells (WBC): 7.2 x 10^3/uL
- Platelets: 210 x 10^3/uL
- Basic Metabolic Panel (BMP)
- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Chloride: 102 mEq/L
- Bicarbonate: 24 mEq/L
- Blood Urea Nitrogen (BUN): 18 mg/dL
- Creatinine: 1.1 mg/dL
- Glucose: 162 mg/dL

IMAGING:
Doppler ultrasound shows monophasic flow in the left popliteal artery.
