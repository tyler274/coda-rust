# synthetic_hospital patient 2001

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 54 at first encounter, sex M, Non-Hispanic White, Commercial.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-05-07 (Tue) | inpatient | General Surgery |
| 1 | 2021-01-15 | 2025-04-22 (Tue) | ed | Emergency Medicine |
| 2 | 2021-07-15 | 2025-10-23 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Blood products (acute hemolytic transfusion reaction)"
 ],
 "insurance": "Commercial",
 "occupation": "Landscaping supervisor",
 "alcohol_use": "Occasional social use (1-2 drinks per month)",
 "family_history": [
  "Father: Myocardial infarction at age 62",
  "Mother: Hypertension",
  "Brother: Hyperlipidemia"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "30 pack-year history, currently smoking 1 pack per day",
 "home_medications": [
  {
   "dose": "20 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg daily",
   "name": "Atorvastatin"
  }
 ],
 "surgical_history": [
  "Surgical repair of bleeding gastric ulcer"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Hyperlipidemia",
  "Coronary artery disease",
  "Peptic ulcer disease",
  "Tobacco use disorder"
 ],
 "age_at_first_encounter": 54
}
```

Primary diagnoses: ["ST-segment elevation myocardial infarction, inferior wall", "Acute Hemolytic Transfusion Reaction"]
Comorbidities: ["Tobacco use disorder", "Hyperlipidemia", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2024-05-07, inpatient, General Surgery

Chief complaint: Admitted after episode of hematemesis with hypotension and pallor

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: General Surgery
Visit Type: inpatient

CHIEF COMPLAINT:
Admitted after episode of hematemesis with hypotension and pallor

HISTORY OF PRESENT ILLNESS:
On arrival, he is pale, diaphoretic, and hypotensive, and resuscitation with intravenous fluids is initiated. Endoscopy reveals a bleeding gastric ulcer, and surgical repair is performed. During the procedure, the patient receives 2 units of packed red blood cells. Within minutes of initiating the second unit, he develops chills, shortness of breath, and diffuse flank pain. His blood pressure drops further, and he becomes acutely restless and tachycardic.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Hyperlipidemia
- Coronary artery disease
- Peptic ulcer disease
- Tobacco use disorder

He has a history of hypertension, hyperlipidemia,.

PAST SURGICAL HISTORY:
- Surgical repair of bleeding gastric ulcer

MEDICATIONS:
He takes lisinopril and atorvastatin.

ALLERGIES:
Blood products (acute hemolytic transfusion reaction)

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Hypertension
- Brother: Hyperlipidemia

SOCIAL HISTORY:
Chronic tobacco use,

PHYSICAL EXAMINATION:
Urine draining from the catheter is dark and reddish. On examination, scleral icterus develops over the next few hours.

LABORATORY DATA:
Laboratory studies show rising creatinine, increased plasma free hemoglobin, and elevated indirect bilirubin. A direct antiglobulin (Coombs) test is positive, and peripheral smear shows fragmented red blood cells.

PLAN:
Despite aggressive supportive care, the patient requires close monitoring in the intensive care unit.

## Encounter 1: source 2021-01-15, practice 2025-04-22, ed, Emergency Medicine

Chief complaint: Severe, crushing chest pain radiating to left arm

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe, crushing chest pain radiating to left arm

HISTORY OF PRESENT ILLNESS:
This patient with coronary artery disease and prior 2020 admission for hematemesis presents to the emergency department with a 2-hour history of severe, crushing chest pain radiating to his left arm. He reports associated shortness of breath, nausea, and diaphoresis.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute Hemolytic Transfusion Reaction (diagnosed 2020-01-15)
- Hypertension
- Hyperlipidemia
- Coronary artery disease
- Peptic ulcer disease
- Tobacco use disorder

His past medical history includes hypertension and hyperlipidemia.

PAST SURGICAL HISTORY:
- Surgical repair of bleeding gastric ulcer

MEDICATIONS:
His medications include lisinopril and atorvastatin.

ALLERGIES:
Blood products (acute hemolytic transfusion reaction)

FAMILY HISTORY:
He has a family history of coronary artery disease.

SOCIAL HISTORY:
He smokes a pack of cigarettes daily

VITAL SIGNS:
On examination, his blood pressure is 150/90 mmHg, heart rate is 98 beats per minute, respiratory rate is 22 breaths per minute, and oxygen saturation is 94% on room air.

PHYSICAL EXAMINATION:
His physical examination is notable for diaphoresis and a regular heart rhythm without murmurs, rubs, or gallops.

LABORATORY DATA:
Laboratory results are as follows:

- Troponin I: 0.5 ng/mL (normal <0.04 ng/mL)
- Creatine kinase-MB: 25 U/L (normal 0-24 U/L)
- Complete blood count: within normal limits
- Basic metabolic panel: within normal limits

IMAGING:
His chest X-ray is unremarkable.

OTHER STUDIES:
An electrocardiogram (ECG) shows ST-segment elevations in leads II, III, and aVF.

## Encounter 2: source 2021-07-15, practice 2025-10-23, ed, Emergency Medicine

Chief complaint: Severe chest pain while mowing lawn, radiating to left arm and jaw

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe chest pain while mowing lawn, radiating to left arm and jaw

HISTORY OF PRESENT ILLNESS:
The patient with known CAD and prior inferior wall MI in January 2021 presents with recurrent chest pain that started 45 minutes ago while he was mowing the lawn. The pain is described as a heavy pressure in the center of his chest, radiating to his left arm and jaw. He is diaphoretic and appears anxious.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute Hemolytic Transfusion Reaction (diagnosed 2020-01-15)
- ST-segment elevation myocardial infarction, inferior wall (diagnosed 2021-01-15)
- Hypertension
- Hyperlipidemia
- Coronary artery disease
- Peptic ulcer disease
- Tobacco use disorder

His medical history includes hypertension and hyperlipidemia,.

PAST SURGICAL HISTORY:
- Surgical repair of bleeding gastric ulcer

MEDICATIONS:
For which he takes lisinopril and atorvastatin.

ALLERGIES:
Blood products (acute hemolytic transfusion reaction)

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Hypertension
- Brother: Hyperlipidemia

SOCIAL HISTORY:
He has a 30-pack-year smoking history.

VITAL SIGNS:
On examination, his blood pressure is 150/90 mmHg, heart rate is 98 beats per minute, respiratory rate is 22 breaths per minute, and oxygen saturation is 94% on room air.

PHYSICAL EXAMINATION:
The cardiac examination reveals a regular rhythm without murmurs, rubs, or gallops. Lung fields are clear to auscultation.

LABORATORY DATA:
Laboratory values are as follows:

- Troponin I: 0.5 ng/mL (normal <0.04 ng/mL)
- Creatine kinase-MB: 25 U/L (normal 0-24 U/L)
- Complete blood count and electrolytes are within normal limits.

IMAGING:
His chest X-ray is unremarkable.

OTHER STUDIES:
The ECG shows ST-segment elevation in leads II, III, and aVF.
