# synthetic_hospital patient 1863

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex F, Caucasian, Private PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-10-12 (Wed) | outpatient | Pulmonology |
| 1 | 2021-01-15 | 2023-10-12 (Thu) | outpatient | Neurology |
| 2 | 2023-01-15 | 2025-10-12 (Sun) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Environmental: Pollen (trees, grasses), Dust mites"
 ],
 "insurance": "Private PPO",
 "occupation": "Corporate Project Manager",
 "alcohol_use": "Social (2-3 drinks per month)",
 "family_history": [
  "Mother: Migraine disorder",
  "Father: Asthma and eczema",
  "Maternal aunt: Hypertension"
 ],
 "race_ethnicity": "Caucasian",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "50 mg oral tablet, 1 tablet at onset of migraine, may repeat once after 2 hours if needed (max 2 doses per day)",
   "name": "Sumatriptan"
  },
  {
   "dose": "10 mg oral tablet once daily",
   "name": "Cetirizine"
  },
  {
   "dose": "110 mcg inhaler, 2 puffs twice daily",
   "name": "Fluticasone propionate HFA"
  },
  {
   "dose": "90 mcg inhaler, 2 puffs every 4-6 hours as needed for cough or wheezing",
   "name": "Albuterol HFA"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Asthma",
  "Migraine without aura",
  "Seasonal allergic rhinitis"
 ],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Migraine without aura", "Internal carotid artery dissection", "Asthma"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2022-10-12, outpatient, Pulmonology

Chief complaint: Chronic dry cough for 3 months, worse at night, with occasional wheezing

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Chronic dry cough for 3 months, worse at night, with occasional wheezing

HISTORY OF PRESENT ILLNESS:
She presents to the clinic with a 3-month history of chronic cough that is more pronounced at night. She reports that the cough is dry and is occasionally associated with wheezing. She denies any recent upper respiratory infections, smoking history, or use of ACE inhibitors.

PAST MEDICAL HISTORY:
Active Problem List:
- Asthma
- Migraine without aura
- Seasonal allergic rhinitis

Her past medical history is significant for seasonal allergies, which are currently well-controlled with cetirizine.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Sumatriptan 50 mg oral tablet, 1 tablet at onset of migraine, may repeat once after 2 hours if needed (max 2 doses per day)
- Cetirizine 10 mg oral tablet once daily
- Fluticasone propionate HFA 110 mcg inhaler, 2 puffs twice daily
- Albuterol HFA 90 mcg inhaler, 2 puffs every 4-6 hours as needed for cough or wheezing

ALLERGIES:
Environmental: Pollen (trees, grasses), Dust mites

FAMILY HISTORY:
- Mother: Migraine disorder
- Father: Asthma and eczema
- Maternal aunt: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (2-3 drinks per month). Occupation: Corporate Project Manager.

VITAL SIGNS:
On physical examination, her vital signs are within normal limits,

PHYSICAL EXAMINATION:
Lung auscultation reveals mild expiratory wheezing bilaterally. No other abnormalities are noted.

OTHER STUDIES:
Spirometry is performed, and the pre-bronchodilator forced expiratory volume in 1 second (FEV1) is 75% of the predicted value. After administration of a bronchodilator, the FEV1 increases by 15%.

## Encounter 1: source 2021-01-15, practice 2023-10-12, outpatient, Neurology

Chief complaint: Recurrent severe headaches for the past 6 months, occurring 1-2 times monthly

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Michael Okafor, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Recurrent severe headaches for the past 6 months, occurring 1-2 times monthly

HISTORY OF PRESENT ILLNESS:
The patient is a 33-year-old female with a history of migraine without aura, asthma, and seasonal allergic rhinitis presenting for evaluation of recurrent headaches. Following her Pulmonology encounter, she describes severe, unilateral, throbbing headaches often accompanied by nausea and photophobia lasting 4-6 hours and occurring 1-2 times per month. She notes that these are often triggered by periods of high stress at work or when she skips meals. She has tried acetaminophen with minimal relief.

PAST MEDICAL HISTORY:
Active Problem List:
- Asthma (diagnosed 2020-01-15)
- Migraine without aura
- Seasonal allergic rhinitis

Her past medical history is significant for asthma.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Sumatriptan 50 mg oral tablet, 1 tablet at onset of migraine, may repeat once after 2 hours if needed (max 2 doses per day)
- Cetirizine 10 mg oral tablet once daily
- Fluticasone propionate HFA 110 mcg inhaler, 2 puffs twice daily
- Albuterol HFA 90 mcg inhaler, 2 puffs every 4-6 hours as needed for cough or wheezing

ALLERGIES:
Environmental: Pollen (trees, grasses), Dust mites

FAMILY HISTORY:
A family history of migraines

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (2-3 drinks per month). Occupation: Corporate Project Manager.

REVIEW OF SYSTEMS:
She denies any recent head trauma or changes in vision.

VITAL SIGNS:
Blood pressure is 120/78 mmHg, heart rate is 72 beats per minute

PHYSICAL EXAMINATION:
The neurological examination is non-focal

## Encounter 2: source 2023-01-15, practice 2025-10-12, ed, Emergency Medicine

Chief complaint: Sudden onset severe headache and left-sided neck pain starting 3 hours ago

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Elena Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset severe headache and left-sided neck pain starting 3 hours ago

HISTORY OF PRESENT ILLNESS:
This 35-year-old female with asthma and migraine without aura, previously evaluated by Pulmonology and Neurology, presents with sudden onset severe headache and left-sided neck pain starting 3 hours ago. She reports that the headache is different from any she has had before, describing it as a sharp and tearing sensation. She also notes partial drooping of her right eyelid and a constricted pupil on the same side but denies any change in sweating. She denies any recent trauma or illness but mentions that she recently started a yoga class that involves neck manipulations.

PAST MEDICAL HISTORY:
Active Problem List:
- Asthma (diagnosed 2020-01-15)
- Migraine without aura (diagnosed 2021-01-15)
- Seasonal allergic rhinitis

Her past medical history is significant for migraines without aura,.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
She takes for which she takes sumatriptan as needed.

Home Medications:
- Cetirizine 10 mg oral tablet once daily
- Fluticasone propionate HFA 110 mcg inhaler, 2 puffs twice daily
- Albuterol HFA 90 mcg inhaler, 2 puffs every 4-6 hours as needed for cough or wheezing

ALLERGIES:
Environmental: Pollen (trees, grasses), Dust mites

FAMILY HISTORY:
- Mother: Migraine disorder
- Father: Asthma and eczema
- Maternal aunt: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (2-3 drinks per month). Occupation: Corporate Project Manager.

VITAL SIGNS:
On examination, her blood pressure is 135/85 mmHg, heart rate is 78 beats per minute, and temperature is 37.1°C (98.8°F).

PHYSICAL EXAMINATION:
Neurological examination reveals mild right-sided weakness and decreased sensation.

IMAGING:
A non-contrast head CT scan is unremarkable.
