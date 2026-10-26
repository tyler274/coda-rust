# synthetic_hospital patient 2636

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 34 at first encounter, sex M, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-05-23 (Mon) | outpatient | Internal Medicine |
| 1 | 2021-01-15 | 2023-05-30 (Tue) | outpatient | Pulmonology |
| 2 | 2022-01-15 | 2024-05-20 (Mon) | outpatient | Vascular Surgery |
| 3 | 2023-01-15 | 2025-05-29 (Thu) | outpatient | Vascular Surgery |
| 4 | 2024-01-15 | 2026-05-18 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Construction laborer",
 "alcohol_use": "None",
 "family_history": [
  "Father: Autosomal dominant polycystic kidney disease with end-stage renal disease",
  "Mother: Emphysema/COPD (former smoker)",
  "Older brother: Alpha-1 antitrypsin deficiency"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Active smoker, 1 to 1.5 packs per day for approximately 15 years (22.5 pack-year history)",
 "home_medications": [
  {
   "dose": "90 mcg, 2 puffs every 4-6 hours as needed for dyspnea",
   "name": "Albuterol inhaler"
  },
  {
   "dose": "400-600 mg orally every 6-8 hours as needed for pain",
   "name": "Ibuprofen"
  }
 ],
 "surgical_history": [
  "Tonsillectomy at age 8"
 ],
 "chronic_conditions": [
  "Alpha-1 antitrypsin deficiency",
  "Autosomal dominant polycystic kidney disease",
  "Thromboangiitis obliterans (Buerger disease)",
  "Tobacco use disorder"
 ],
 "age_at_first_encounter": 34
}
```

Primary diagnoses: ["Alpha-1 antitrypsin deficiency", "Buerger disease (Thromboangiitis obliterans)", "Autosomal Dominant Polycystic Kidney Disease", "Helicobacter pylori-associated gastric ulcer with hemorrhage"]
Comorbidities: ["Family history of kidney disease", "Tobacco use disorder"]

## Encounter 0: source 2020-01-15, practice 2022-05-23, outpatient, Internal Medicine

Chief complaint: Headaches and intermittent palpitations over the past 6 months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Headaches and intermittent palpitations over the past 6 months

HISTORY OF PRESENT ILLNESS:
He presents to the clinic with a 6-month history of headaches and intermittent palpitations

PAST MEDICAL HISTORY:
Active Problem List:
- Alpha-1 antitrypsin deficiency
- Autosomal dominant polycystic kidney disease
- Thromboangiitis obliterans (Buerger disease)
- Tobacco use disorder

PAST SURGICAL HISTORY:
- Tonsillectomy at age 8

MEDICATIONS:
Home Medications:
- Albuterol inhaler 90 mcg, 2 puffs every 4-6 hours as needed for dyspnea
- Ibuprofen 400-600 mg orally every 6-8 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
Mother was diagnosed with kidney disease in her 40s

SOCIAL HISTORY:
Smoking: Active smoker, 1 to 1.5 packs per day for approximately 15 years (22.5 pack-year history). Alcohol: None. Occupation: Construction laborer.

VITAL SIGNS:
His blood pressure is 155/95 mmHg on two separate occasions

PHYSICAL EXAMINATION:
Bilateral flank masses and a soft bruit over the left upper quadrant

LABORATORY DATA:
Serum creatinine of 1.2 mg/dL (normal: 0.6-1.3 mg/dL) and a mildly elevated serum aldosterone level

IMAGING:
Abdominal ultrasound demonstrates multiple bilateral renal cysts

## Encounter 1: source 2021-01-15, practice 2023-05-30, outpatient, Pulmonology

Chief complaint: Progressive shortness of breath over the past year

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Michael Rodriguez, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive shortness of breath over the past year

PAST MEDICAL HISTORY:
Active Problem List:
- Autosomal Dominant Polycystic Kidney Disease (diagnosed 2020-01-15)
- Alpha-1 antitrypsin deficiency
- Thromboangiitis obliterans (Buerger disease)
- Tobacco use disorder

PAST SURGICAL HISTORY:
- Tonsillectomy at age 8

MEDICATIONS:
Home Medications:
- Albuterol inhaler 90 mcg, 2 puffs every 4-6 hours as needed for dyspnea
- Ibuprofen 400-600 mg orally every 6-8 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
37-year-old brother recently diagnosed with lung disease. His father died at age 55 from alcoholic cirrhosis.

SOCIAL HISTORY:
5-year history of smoking one pack per day

PHYSICAL EXAMINATION:
On physical exam, breath sounds are diminished bilaterally, and percussion is hyperresonant at the lung bases.

IMAGING:
Chest imaging shows diffuse basilar-predominant emphysema.

PATHOLOGY:
Liver biopsy reveals periodic acid–Schiff (PAS)-positive, diastase-resistant granules within periportal hepatocytes.

## Encounter 2: source 2022-01-15, practice 2024-05-20, outpatient, Vascular Surgery

Chief complaint: Pain in both hands and feet that worsens with walking and improves with rest

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. James O'Brien, MD
Department: Vascular Surgery
Visit Type: outpatient

CHIEF COMPLAINT:
Pain in both hands and feet that worsens with walking and improves with rest

HISTORY OF PRESENT ILLNESS:
The patient returns for thromboangiitis obliterans follow-up after Internal Medicine and Pulmonology care. Over the past 6 months, he has developed painful ulcerations on his fingertips and notes fingers turn white and blue in cold weather.

PAST MEDICAL HISTORY:
Active Problem List:
- Autosomal Dominant Polycystic Kidney Disease (diagnosed 2020-01-15)
- Alpha-1 antitrypsin deficiency (diagnosed 2021-01-15)
- Thromboangiitis obliterans (Buerger disease)
- Tobacco use disorder

He has no personal history of diabetes, hypertension, or hyperlipidemia.

PAST SURGICAL HISTORY:
- Tonsillectomy at age 8

MEDICATIONS:
Home Medications:
- Albuterol inhaler 90 mcg, 2 puffs every 4-6 hours as needed for dyspnea
- Ibuprofen 400-600 mg orally every 6-8 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Autosomal dominant polycystic kidney disease with end-stage renal disease
- Mother: Emphysema/COPD (former smoker)
- Older brother: Alpha-1 antitrypsin deficiency

SOCIAL HISTORY:
He smokes 1.5 packs of cigarettes per day and has done so for 15 years.

REVIEW OF SYSTEMS:
He denies fever, joint pain, or weight loss.

VITAL SIGNS:
His temperature is 36.8°C (98.2°F), blood pressure is 124/78 mm Hg, pulse is 76/min, and respiratory rate is 14/min.

PHYSICAL EXAMINATION:
Physical examination reveals shallow ulcerations at the tips of multiple fingers and toes. There is no joint swelling or erythema. Radial and dorsalis pedis pulses are diminished bilaterally. Capillary refill is delayed. The rest of the physical exam is unremarkable.

## Encounter 3: source 2023-01-15, practice 2025-05-29, outpatient, Vascular Surgery

Chief complaint: Worsening pain in hands and feet at rest, especially at night, with color changes in cold weather

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. James O'Brien, MD
Department: Vascular Surgery
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening pain in hands and feet at rest, especially at night, with color changes in cold weather

HISTORY OF PRESENT ILLNESS:
The patient is a 37-year-old male with established Thromboangiitis obliterans, Alpha-1 antitrypsin deficiency, and Autosomal dominant polycystic kidney disease who returns to Vascular Surgery approximately one year after his initial evaluation with progression of his extremity symptoms. He reports that the pain initially occurred during walking and resolved with rest, but now he feels it even at rest, especially at night. He has also noticed his fingers turning pale and then blue when exposed to cold temperatures, followed by redness and throbbing as they rewarm. He reports that over the past month, he has developed painful ulcerations on the tips of two fingers and one toe. He denies fever, weight loss, or joint pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Autosomal Dominant Polycystic Kidney Disease (diagnosed 2020-01-15)
- Alpha-1 antitrypsin deficiency (diagnosed 2021-01-15)
- Buerger disease (Thromboangiitis obliterans) (diagnosed 2022-01-15)
- Tobacco use disorder

PAST SURGICAL HISTORY:
- Tonsillectomy at age 8

MEDICATIONS:
Home Medications:
- Albuterol inhaler 90 mcg, 2 puffs every 4-6 hours as needed for dyspnea
- Ibuprofen 400-600 mg orally every 6-8 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Autosomal dominant polycystic kidney disease with end-stage renal disease
- Mother: Emphysema/COPD (former smoker)
- Older brother: Alpha-1 antitrypsin deficiency

SOCIAL HISTORY:
He has a 15-pack-year smoking history and currently smokes one pack of cigarettes per day. He does not use alcohol or illicit drugs.

PHYSICAL EXAMINATION:
On physical examination, there are small, shallow ulcers on the fingertips of the right hand and the great toe of the left foot. There is diminished hair growth on both lower extremities and mildly decreased dorsalis pedis pulses bilaterally. Radial and ulnar pulses are palpable. Digital Allen test is abnormal in both hands.

## Encounter 4: source 2024-01-15, practice 2026-05-18, ed, Emergency Medicine

Chief complaint: Epigastric discomfort for 2 months with dark stools for the past week

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Epigastric discomfort for 2 months with dark stools for the past week

HISTORY OF PRESENT ILLNESS:
This 38-year-old with Alpha-1 antitrypsin deficiency, ADPKD, and Buerger disease presents to the ED. He reports that the pain is burning in nature and worsens several hours after meals but improves temporarily with eating or antacids. He has also noticed dark stools over the past week.

PAST MEDICAL HISTORY:
Active Problem List:
- Autosomal Dominant Polycystic Kidney Disease (diagnosed 2020-01-15)
- Alpha-1 antitrypsin deficiency (diagnosed 2021-01-15)
- Buerger disease (Thromboangiitis obliterans) (diagnosed 2022-01-15)
- Tobacco use disorder

PAST SURGICAL HISTORY:
- Tonsillectomy at age 8

MEDICATIONS:
He takes occasionally takes ibuprofen for muscle pain.

Home Medications:
- Albuterol inhaler 90 mcg, 2 puffs every 4-6 hours as needed for dyspnea

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Autosomal dominant polycystic kidney disease with end-stage renal disease
- Mother: Emphysema/COPD (former smoker)
- Older brother: Alpha-1 antitrypsin deficiency

SOCIAL HISTORY:
He smokes one pack of cigarettes daily

VITAL SIGNS:
Vital signs are normal.

PHYSICAL EXAMINATION:
Examination reveals mild epigastric tenderness.

LABORATORY DATA:
Stool guaiac test is positive.

PATHOLOGY:
Gastric biopsy is shown below:

OTHER STUDIES:
Upper endoscopy reveals a 1.2-cm ulcer along the lesser curvature of the stomach.
