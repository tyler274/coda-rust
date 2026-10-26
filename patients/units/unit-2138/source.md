# synthetic_hospital patient 2138

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex M, Hispanic/Latino (Guatemalan), Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-01-31 (Wed) | outpatient | Gastroenterology |
| 1 | 2022-01-15 | 2026-01-31 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicaid",
 "occupation": "Construction worker",
 "alcohol_use": "None",
 "family_history": [
  "Mother: deceased at age 58 from suspected chagasic cardiomyopathy/heart failure",
  "Father: hypertension, type 2 diabetes",
  "Sister: hypothyroidism"
 ],
 "race_ethnicity": "Hispanic/Latino (Guatemalan)",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "10 mg PO daily",
   "name": "Enalapril"
  },
  {
   "dose": "6.25 mg PO twice daily",
   "name": "Carvedilol"
  },
  {
   "dose": "17 g PO daily as needed for constipation",
   "name": "Polyethylene glycol 3350"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Omeprazole"
  }
 ],
 "surgical_history": [
  "Colonoscopy with colonic mucosal biopsies (age 42)",
  "None otherwise"
 ],
 "chronic_conditions": [
  "Chagas disease with cardiodigestive manifestations",
  "Megacolon",
  "Megaesophagus",
  "Chagas cardiomyopathy"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Chagas disease (Chronic, with cardiac and digestive manifestations)"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2024-01-31, outpatient, Gastroenterology

Chief complaint: Progressive difficulty swallowing with sensation of food sticking in chest, accompanied by intermittent constipation and 10-pound weight loss over 6 months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Carmen Morales, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive difficulty swallowing with sensation of food sticking in chest, accompanied by intermittent constipation and 10-pound weight loss over 6 months

HISTORY OF PRESENT ILLNESS:
He also reports a 10-pound weight loss and intermittent constipation over the same period.

PAST MEDICAL HISTORY:
Active Problem List:
- Chagas disease with cardiodigestive manifestations
- Megacolon
- Megaesophagus
- Chagas cardiomyopathy

PAST SURGICAL HISTORY:
- Colonoscopy with colonic mucosal biopsies (age 42)
- None otherwise

MEDICATIONS:
Home Medications:
- Enalapril 10 mg PO daily
- Carvedilol 6.25 mg PO twice daily
- Polyethylene glycol 3350 17 g PO daily as needed for constipation
- Omeprazole 20 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: deceased at age 58 from suspected chagasic cardiomyopathy/heart failure
- Father: hypertension, type 2 diabetes
- Sister: hypothyroidism

SOCIAL HISTORY:
He denies any tobacco, alcohol, or illicit drug use.

PHYSICAL EXAMINATION:
On physical examination, the patient appears cachectic. Auscultation of the heart reveals a soft systolic murmur at the apex. Abdominal examination is normal.

LABORATORY DATA:
Serologic testing for Trypanosoma cruzi antibodies is positive.

IMAGING:
A barium swallow study shows significant dilation of the esophagus with a "bird's beak" appearance at the distal end.

OTHER STUDIES:
An ECG demonstrates right bundle branch block and apical left ventricular aneurysm.

## Encounter 1: source 2022-01-15, practice 2026-01-31, ed, Emergency Medicine

Chief complaint: Palpitations, lightheadedness, and occasional episodes of chest discomfort

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. James Okonkwo, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Palpitations, lightheadedness, and occasional episodes of chest discomfort

PAST MEDICAL HISTORY:
Active Problem List:
- Chagas disease (Chronic, with cardiac and digestive manifestations) (diagnosed 2020-01-15)
- Chagas disease with cardiodigestive manifestations
- Megacolon
- Megaesophagus
- Chagas cardiomyopathy

He reports a history of severe constipation and abdominal bloating for years and underwent evaluation in the past that revealed a massively dilated colon.

PAST SURGICAL HISTORY:
- Colonoscopy with colonic mucosal biopsies (age 42)
- None otherwise

MEDICATIONS:
Home Medications:
- Enalapril 10 mg PO daily
- Carvedilol 6.25 mg PO twice daily
- Polyethylene glycol 3350 17 g PO daily as needed for constipation
- Omeprazole 20 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
He denies any family history of heart disease

SOCIAL HISTORY:
He recently moved to the United States from rural Brazil.
does not use alcohol or drugs

VITAL SIGNS:
Vital signs are stable

PHYSICAL EXAMINATION:
There are no murmurs or signs of heart failure on examination

IMAGING:
Echocardiography demonstrates apical thinning and mild left ventricular systolic dysfunction.
A chest x-ray is shown.

OTHER STUDIES:
His current ECG shows conduction abnormalities including right bundle branch block and frequent premature ventricular complexes.
