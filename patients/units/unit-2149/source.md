# synthetic_hospital patient 2149

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 38 at first encounter, sex M, Non-Hispanic White, Blue Cross Blue Shield.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 1 | 2020-07-15 | 2024-12-21 (Sat) | ed | Emergency Medicine |
| 2 | 2021-07-15 | 2025-12-23 (Tue) | icu | Pulmonary and Critical Care |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield",
 "occupation": "Wildlife Biologist",
 "alcohol_use": "Social, 2-3 drinks per week",
 "family_history": [
  "Father: Epilepsy, hypertension",
  "Mother: Type 2 diabetes mellitus",
  "Paternal grandfather: Myocardial infarction at age 62"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1000 mg PO BID",
   "name": "Levetiracetam"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  }
 ],
 "surgical_history": [
  "Tonsillectomy (age 12)",
  "Open appendectomy (age 25)"
 ],
 "chronic_conditions": [
  "Generalized tonic-clonic seizure disorder (well-controlled)",
  "Essential hypertension"
 ],
 "age_at_first_encounter": 38
}
```

Primary diagnoses: ["Refractory hypoxemia in ARDS", "Pneumonic plague"]
Comorbidities: ["Acute hypoxemic respiratory failure without hypercapnic encephalopathy"]

## Encounter 1: source 2020-07-15, practice 2024-12-21, ed, Emergency Medicine

Chief complaint: Sudden-onset fever, chills, and difficulty breathing with hemoptysis

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden-onset fever, chills, and difficulty breathing with hemoptysis

HISTORY OF PRESENT ILLNESS:
This 38-year-old male with well-controlled generalized tonic-clonic seizure disorder on levetiracetam and essential hypertension on lisinopril is brought to the emergency department by ambulance due to sudden-onset fever, chills, and difficulty breathing. He developed a dry cough yesterday that has progressed to coughing up blood-tinged sputum. His wife reports that he became acutely ill overnight and has been increasingly confused. He lives in rural New Mexico and works in wildlife control; last week, he participated in a rodent eradication project involving prairie dogs. He has not traveled internationally.

PAST MEDICAL HISTORY:
Active Problem List:
- Generalized tonic-clonic seizure disorder (well-controlled)
- Essential hypertension

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)
- Open appendectomy (age 25)

MEDICATIONS:
Home Medications:
- Levetiracetam 1000 mg PO BID
- Lisinopril 10 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Epilepsy, hypertension
- Mother: Type 2 diabetes mellitus
- Paternal grandfather: Myocardial infarction at age 62

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 2-3 drinks per week. Occupation: Wildlife Biologist.

VITAL SIGNS:
Temperature is 39.6°C, heart rate is 124/min, blood pressure is 92/58 mmHg, and respiratory rate is 30/min

PHYSICAL EXAMINATION:
He is obtunded and coughing up frothy pink sputum. Crackles are heard throughout both lung fields.

LABORATORY DATA:
Gram stain and culture of sputum show a pleomorphic gram-negative rod.

IMAGING:
Chest x-ray reveals bilateral alveolar infiltrates with necrotizing features.

ASSESSMENT AND PLAN:
He is intubated for hypoxic respiratory failure and admitted to the ICU for presumed septic shock.

## Encounter 2: source 2021-07-15, practice 2025-12-23, icu, Pulmonary and Critical Care

Chief complaint: Refractory hypoxemia following near-drowning submersion in cold water

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Emily Watson, MD
Department: Pulmonary and Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Refractory hypoxemia following near-drowning submersion in cold water

HISTORY OF PRESENT ILLNESS:
This 39-year-old male with well-controlled seizure disorder, hypertension, and prior pneumonic plague was submerged in cold water for several minutes, rescued, and is now hypoxic despite supplemental oxygen.

PAST MEDICAL HISTORY:
Active Problem List:
- Pneumonic plague (diagnosed 2020-07-15)
- Generalized tonic-clonic seizure disorder (well-controlled)
- Essential hypertension

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)
- Open appendectomy (age 25)

MEDICATIONS:
Home Medications:
- Levetiracetam 1000 mg PO BID
- Lisinopril 10 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Epilepsy, hypertension
- Mother: Type 2 diabetes mellitus
- Paternal grandfather: Myocardial infarction at age 62

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 2-3 drinks per week. Occupation: Wildlife Biologist.

PHYSICAL EXAMINATION:
Physical examination reveals tachypnea, diffuse crackles over both lung fields, and use of accessory respiratory muscles.

LABORATORY DATA:
Arterial blood gas analysis shows hypoxemia with a widened alveolar–arterial gradient.

IMAGING:
Chest radiography demonstrates bilateral, patchy opacities without evidence of cardiomegaly.

PATHOLOGY:
Histologic examination of normal alveolar tissue from an unrelated case is shown. Damage to or loss of function of the cells labeled Y would most directly impair
