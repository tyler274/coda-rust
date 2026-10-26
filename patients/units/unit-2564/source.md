# synthetic_hospital patient 2564

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex M, Non-Hispanic White, Employer-sponsored PPO (UnitedHealthcare).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-01-03 (Fri) | inpatient | Emergency Medicine |
| 1 | 2020-01-15 | 2025-01-03 (Fri) | icu | Critical Care |
| 2 | 2021-07-15 | 2026-06-22 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Employer-sponsored PPO (UnitedHealthcare)",
 "occupation": "Regional Sales Representative",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Father: Hypertension (diagnosed age 55, controlled)",
  "Mother: Hypothyroidism",
  "No family history of bleeding disorders or thrombotic conditions"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [],
 "chronic_conditions": [],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Traumatic hemothorax", "Disseminated intravascular coagulation", "Traumatic hemothorax with hypovolemic shock"]
Comorbidities: ["Acute posthemorrhagic anemia", "Contusion of chest wall", "Blunt chest trauma"]

## Encounter 0: source 2020-01-15, practice 2025-01-03, inpatient, Emergency Medicine

Chief complaint: Severe chest pain and difficulty breathing following high-speed motor vehicle collision

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Severe chest pain and difficulty breathing following high-speed motor vehicle collision

HISTORY OF PRESENT ILLNESS:
He presents to the emergency department following a high-speed motor vehicle collision. He complains of severe chest pain and difficulty breathing.

PAST MEDICAL HISTORY:
His past medical history is unremarkable.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension (diagnosed age 55, controlled)
- Mother: Hypothyroidism
- No family history of bleeding disorders or thrombotic conditions

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Regional Sales Representative.

VITAL SIGNS:
Respiratory rate of 28 breaths per minute. His blood pressure is 90/60 mmHg, heart rate is 120 beats per minute, and oxygen saturation is 92% on room air.

PHYSICAL EXAMINATION:
On examination, he is anxious, diaphoretic. Examination of the chest reveals decreased breath sounds on the left side with dullness to percussion. His jugular veins are not distended. The abdomen is soft and non-tender.

LABORATORY DATA:
Laboratory Values:
- Hemoglobin: 11 g/dL (decreased from a baseline of 15 g/dL)
- Hematocrit: 33% (decreased from a baseline of 45%)
- White Blood Cell Count: 12,000 /μL
- Platelets: 250,000 /μL

IMAGING:
Chest X-ray shows multiple left-sided rib fractures and a large opacification of the left hemithorax suggestive of a fluid collection.

## Encounter 1: source 2020-01-15, practice 2025-01-03, icu, Critical Care

Chief complaint: Agitation and confusion following high-speed motor vehicle collision with suspected coagulopathy

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Marcus Johnson, MD
Department: Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Agitation and confusion following high-speed motor vehicle collision with suspected coagulopathy

HISTORY OF PRESENT ILLNESS:
The patient returns following emergency department evaluation earlier today. He was the restrained driver, and airbags were deployed. On arrival, he is alert but appears agitated and confused.

PAST MEDICAL HISTORY:
Active Problem List:
- Traumatic hemothorax with hypovolemic shock (diagnosed 2020-01-15)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension (diagnosed age 55, controlled)
- Mother: Hypothyroidism
- No family history of bleeding disorders or thrombotic conditions

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Regional Sales Representative.

VITAL SIGNS:
His vital signs are as follows: blood pressure 90/60 mmHg, heart rate 120 beats/min, respiratory rate 22 breaths/min, and temperature 37.2°C (99.0°F).

PHYSICAL EXAMINATION:
Physical examination reveals multiple contusions on his chest and abdomen, a deformed right femur, and active oozing from intravenous catheter sites.

LABORATORY DATA:
Initial laboratory results are as follows:

- Hemoglobin: 10.2 g/dL (normal: 13.8-17.2 g/dL)
- Platelet count: 55 x 10^9/L (normal: 150-400 x 10^9/L)
- Prothrombin time (PT): 18 seconds (normal: 12-14 seconds)
- Partial thromboplastin time (PTT): 45 seconds (normal: 25-35 seconds)
- Fibrinogen: 150 mg/dL (normal: 200-400 mg/dL)
- D-dimer: Elevated

ASSESSMENT AND PLAN:
Given the patient&#x27;s history and laboratory findings,

## Encounter 2: source 2021-07-15, practice 2026-06-22, ed, Emergency Medicine

Chief complaint: Brought to the emergency department after being involved in a high-speed motor vehicle collision

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Elena Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Brought to the emergency department after being involved in a high-speed motor vehicle collision

HISTORY OF PRESENT ILLNESS:
The patient, who was previously admitted in January 2020 for traumatic hemothorax and subsequent critical care management for coagulopathy, presents following a high-speed motor vehicle collision. He was the restrained driver, and airbags were deployed.

PAST MEDICAL HISTORY:
Active Problem List:
- Traumatic hemothorax with hypovolemic shock (diagnosed 2020-01-15)
- Disseminated intravascular coagulation (diagnosed 2020-01-15)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension (diagnosed age 55, controlled)
- Mother: Hypothyroidism
- No family history of bleeding disorders or thrombotic conditions

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Regional Sales Representative.

VITAL SIGNS:
Blood pressure 90/60 mmHg, heart rate 120 beats per minute, respiratory rate 28 breaths per minute, and oxygen saturation 89% on room air

PHYSICAL EXAMINATION:
On arrival, he is alert but appears anxious and diaphoretic. Physical examination reveals decreased breath sounds and dullness to percussion on the left side of the chest, with visible bruising over the left lateral chest wall. The trachea is deviated to the right. The abdomen is soft and non-distended with no obvious signs of trauma. Pelvic examination is stable, and there are no signs of long bone fractures.

LABORATORY DATA:
Hemoglobin: 9.8 g/dL (normal: 13.8-17.2 g/dL)
Hematocrit: 29% (normal: 40-50%)
Platelets: 250,000/mm³ (normal: 150,000-450,000/mm³)
PT: 12 seconds (normal: 9.5-13.5 seconds)
aPTT: 30 seconds (normal: 25-35 seconds)

IMAGING:
A FAST (Focused Assessment with Sonography for Trauma) exam is negative for intra-abdominal fluid. A chest x-ray is ordered immediately.
