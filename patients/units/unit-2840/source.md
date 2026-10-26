# synthetic_hospital patient 2840

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 78 at first encounter, sex F, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-12-15 (Sun) | ed | Emergency Medicine |
| 1 | 2020-07-15 | 2025-06-09 (Mon) | inpatient | Internal Medicine |
| 2 | 2021-04-15 | 2026-03-05 (Thu) | inpatient | General Surgery |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired elementary school teacher",
 "alcohol_use": "Never",
 "family_history": [
  "Hypertension (mother)",
  "Gallstone disease (sister)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never",
 "home_medications": [
  {
   "dose": "25 mg oral daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "500 mg oral daily",
   "name": "Calcium carbonate"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Cholelithiasis"
 ],
 "age_at_first_encounter": 78
}
```

Primary diagnoses: ["Hyponatremia", "Dehydration", "Gallstone ileus"]
Comorbidities: ["Orthostatic hypotension", "Essential hypertension", "Dehydration"]

## Encounter 0: source 2020-01-15, practice 2024-12-15, ed, Emergency Medicine

Chief complaint: Recurrent episodes of dizziness, particularly upon standing from a seated position

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Recurrent episodes of dizziness, particularly upon standing from a seated position

HISTORY OF PRESENT ILLNESS:
She presents to the emergency department with a history of recurrent episodes of dizziness, particularly upon standing from a seated position. She reports that these episodes have been accompanied by blurred vision and a feeling of weakness. Her family notes that she has been less active lately and drinking less water.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Cholelithiasis

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Calcium carbonate 500 mg oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Hypertension (mother)
- Gallstone disease (sister)

SOCIAL HISTORY:
Smoking: Never. Alcohol: Never. Occupation: Retired elementary school teacher.

REVIEW OF SYSTEMS:
She denies any chest pain, palpitations, or loss of consciousness.

VITAL SIGNS:
Her blood pressure is 110/70 mmHg while seated and drops to 90/50 mmHg upon standing. Her heart rate increases from 78 bpm to 98 bpm with the change in position.

PHYSICAL EXAMINATION:
Her mucous membranes are dry, and her skin turgor is decreased.

LABORATORY DATA:
Serum sodium: 139 mEq/L
- Serum potassium: 3.5 mEq/L
- Serum creatinine: 1.2 mg/dL
- Blood urea nitrogen (BUN): 28 mg/dL
- Urine sodium: 10 mEq/L

## Encounter 1: source 2020-07-15, practice 2025-06-09, inpatient, Internal Medicine

Chief complaint: Vomiting and diarrhea with lightheadedness and decreased urine output

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. James Okonkwo, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Vomiting and diarrhea with lightheadedness and decreased urine output

HISTORY OF PRESENT ILLNESS:
This patient with hypertension and cholelithiasis returns six months after an emergency department visit for dizziness. She presents to the emergency department with 3-day history of vomiting and diarrhea, reporting lightheadedness upon standing and decreased urine output.

PAST MEDICAL HISTORY:
Active Problem List:
- Dehydration (diagnosed 2020-01-15)
- Hypertension
- Cholelithiasis

Her past medical history includes hypertension, for which she takes hydrochlorothiazide.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Calcium carbonate 500 mg oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Hypertension (mother)
- Gallstone disease (sister)

SOCIAL HISTORY:
Smoking: Never. Alcohol: Never. Occupation: Retired elementary school teacher.

VITAL SIGNS:
On examination, her blood pressure is 95/60 mmHg when lying down and 85/50 mmHg upon standing, with a heart rate increase from 88 to 102 bpm.

PHYSICAL EXAMINATION:
Her mucous membranes are dry, and her skin turgor is decreased.

LABORATORY DATA:
Laboratory results are as follows:

- Sodium: 128 mEq/L
- Potassium: 3.9 mEq/L
- Chloride: 95 mEq/L
- Bicarbonate: 24 mEq/L
- Blood urea nitrogen (BUN): 35 mg/dL
- Creatinine: 1.2 mg/dL
- Urine sodium: 10 mEq/L
- Urine osmolality: 500 mOsm/kg

## Encounter 2: source 2021-04-15, practice 2026-03-05, inpatient, General Surgery

Chief complaint: Worsening abdominal pain, nausea, and repeated episodes of bilious vomiting

ENCOUNTER NOTE
Date: 2021-04-15
Provider: Dr. Maria Rodriguez, MD
Department: General Surgery
Visit Type: inpatient

CHIEF COMPLAINT:
Worsening abdominal pain, nausea, and repeated episodes of bilious vomiting

HISTORY OF PRESENT ILLNESS:
The 79-year-old female with hypertension, cholelithiasis, and prior admissions for dehydration in January 2020 and hyponatremia in July 2020 is brought to the emergency department because of worsening abdominal pain, nausea, and repeated episodes of bilious vomiting over the past 2 days. She notes progressive abdominal bloating and inability to pass stool or flatus during this time.

PAST MEDICAL HISTORY:
Active Problem List:
- Dehydration (diagnosed 2020-01-15)
- Hyponatremia (diagnosed 2020-07-15)
- Hypertension
- Cholelithiasis

Her medical history includes hypertension and long-standing right upper quadrant discomfort after fatty meals, but she never pursued surgical evaluation.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
She takes a daily calcium supplement and no other medications.

Home Medications:
- Hydrochlorothiazide 25 mg oral daily
- Calcium carbonate 500 mg oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Hypertension (mother)
- Gallstone disease (sister)

SOCIAL HISTORY:
Smoking: Never. Alcohol: Never. Occupation: Retired elementary school teacher.

REVIEW OF SYSTEMS:
She denies hematemesis, melena, hematochezia, or urinary complaints.

VITAL SIGNS:
Temperature is 37.5 °C (99.5 °F), blood pressure is 96/58 mmHg, pulse is 114/min, and respiratory rate is 22/min.

PHYSICAL EXAMINATION:
On examination, she appears dehydrated and mildly confused. The abdomen is distended with diffuse tenderness but no rebound or guarding; bowel sounds are high-pitched and hyperactive. Digital rectal exam shows an empty vault.

LABORATORY DATA:
Laboratory studies reveal mild leukocytosis.

IMAGING:
Imaging is obtained and demonstrates evidence of small-bowel obstruction with air in the biliary tree.
