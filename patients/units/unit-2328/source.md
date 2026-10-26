# synthetic_hospital patient 2328

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 68 at first encounter, sex M, Non-Hispanic White, Medicare with supplemental Blue Cross Blue Shield.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-12-04 (Wed) | outpatient | Neurology |
| 1 | 2020-07-15 | 2025-06-20 (Fri) | ed | Emergency Medicine |
| 2 | 2021-07-15 | 2026-06-05 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare with supplemental Blue Cross Blue Shield",
 "occupation": "Retired civil engineer",
 "alcohol_use": "Social, 1-2 drinks per week",
 "family_history": [
  "Father with Alzheimer disease",
  "Father with bicuspid aortic valve and premature coronary artery disease",
  "Mother with type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 20 pack-years, quit 10 years ago",
 "home_medications": [
  {
   "dose": "10 mg oral daily",
   "name": "Donepezil"
  },
  {
   "dose": "10 mg oral twice daily",
   "name": "Memantine"
  },
  {
   "dose": "40 mg oral nightly",
   "name": "Atorvastatin"
  },
  {
   "dose": "5 mg oral daily",
   "name": "Lisinopril"
  },
  {
   "dose": "1000 mg oral twice daily",
   "name": "Metformin"
  },
  {
   "dose": "81 mg oral daily",
   "name": "Aspirin"
  },
  {
   "dose": "325 mg oral daily",
   "name": "Ferrous sulfate"
  }
 ],
 "surgical_history": [
  "Laparoscopic cholecystectomy (age 45)",
  "Appendectomy (age 12)"
 ],
 "chronic_conditions": [
  "Severe calcific aortic stenosis",
  "Alzheimer disease",
  "Hypertension",
  "Hyperlipidemia",
  "Type 2 diabetes mellitus"
 ],
 "age_at_first_encounter": 68
}
```

Primary diagnoses: ["Microangiopathic hemolytic anemia", "Babesiosis", "Alzheimer disease"]
Comorbidities: ["History of rheumatic fever"]

## Encounter 0: source 2020-01-15, practice 2024-12-04, outpatient, Neurology

Chief complaint: gradually worsening problems with memory and daily functioning

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Gradually worsening problems with memory and daily functioning

HISTORY OF PRESENT ILLNESS:
His family reports that he frequently repeats conversations, becomes disoriented in familiar places, and has shown changes in behavior and mood.

PAST MEDICAL HISTORY:
Active Problem List:
- Severe calcific aortic stenosis
- Alzheimer disease
- Hypertension
- Hyperlipidemia
- Type 2 diabetes mellitus

He has a history of cardiovascular risk factors but no major neurologic illnesses or prior head trauma.

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (age 45)
- Appendectomy (age 12)

MEDICATIONS:
Home Medications:
- Donepezil 10 mg oral daily
- Memantine 10 mg oral twice daily
- Atorvastatin 40 mg oral nightly
- Lisinopril 5 mg oral daily
- Metformin 1000 mg oral twice daily
- Aspirin 81 mg oral daily
- Ferrous sulfate 325 mg oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with Alzheimer disease
- Father with bicuspid aortic valve and premature coronary artery disease
- Mother with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 10 years ago. Alcohol: Social, 1-2 drinks per week. Occupation: Retired civil engineer.

PHYSICAL EXAMINATION:
On exam, he appears inattentive and struggles with recall and organizing tasks, though his motor and sensory findings are intact.

LABORATORY DATA:
Basic laboratory workup is unremarkable.

IMAGING:
Brain imaging shows generalized volume loss.

PATHOLOGY:
Histologic analysis of affected brain regions demonstrates increased activity and hypertrophy of supporting cells clustering around areas of neuronal loss. These cells are known to produce a characteristic protein that serves as a marker of their activation.

## Encounter 1: source 2020-07-15, practice 2025-06-20, ed, Emergency Medicine

Chief complaint: worsening fatigue, fever, chills, and dark urine

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening fatigue, fever, chills, and dark urine

HISTORY OF PRESENT ILLNESS:
The patient is a 68-year-old male with severe calcific aortic stenosis and Alzheimer disease seen January 2020 for cognitive decline. He presents to the ED with 5 days of worsening fatigue, fever, chills, and dark urine. He recently returned from a hiking trip in rural Connecticut, where he reports multiple insect bites but no rash.

PAST MEDICAL HISTORY:
Active Problem List:
- Alzheimer disease (diagnosed 2020-01-15)
- Severe calcific aortic stenosis
- Hypertension
- Hyperlipidemia
- Type 2 diabetes mellitus

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (age 45)
- Appendectomy (age 12)

MEDICATIONS:
Home Medications:
- Donepezil 10 mg oral daily
- Memantine 10 mg oral twice daily
- Atorvastatin 40 mg oral nightly
- Lisinopril 5 mg oral daily
- Metformin 1000 mg oral twice daily
- Aspirin 81 mg oral daily
- Ferrous sulfate 325 mg oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with Alzheimer disease
- Father with bicuspid aortic valve and premature coronary artery disease
- Mother with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 10 years ago. Alcohol: Social, 1-2 drinks per week. Occupation: Retired civil engineer.

VITAL SIGNS:
His temperature is 38.6°C (101.5°F), blood pressure is 122/78 mm Hg, pulse is 98/min, and respiratory rate is 16/min.

PHYSICAL EXAMINATION:
Physical examination shows mild scleral icterus and splenomegaly.

LABORATORY DATA:
Laboratory studies show show low hemoglobin and high LDH.

OTHER STUDIES:
A peripheral blood smear is shown.

## Encounter 2: source 2021-07-15, practice 2026-06-05, ed, Emergency Medicine

Chief complaint: fatigue and shortness of breath on exertion with dark urine

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fatigue and shortness of breath on exertion with dark urine

HISTORY OF PRESENT ILLNESS:
This 69-year-old male with severe calcific aortic stenosis and prior babesiosis presents with recurrent symptoms. He reports a recent onset of dark urine, especially in the morning.

PAST MEDICAL HISTORY:
Active Problem List:
- Alzheimer disease (diagnosed 2020-01-15)
- Babesiosis (diagnosed 2020-07-15)
- Severe calcific aortic stenosis
- Hypertension
- Hyperlipidemia
- Type 2 diabetes mellitus

PAST SURGICAL HISTORY:
He underwent mechanical aortic valve replacement 5 years ago

MEDICATIONS:
Home Medications:
- Donepezil 10 mg oral daily
- Memantine 10 mg oral twice daily
- Atorvastatin 40 mg oral nightly
- Lisinopril 5 mg oral daily
- Metformin 1000 mg oral twice daily
- Aspirin 81 mg oral daily
- Ferrous sulfate 325 mg oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with Alzheimer disease
- Father with bicuspid aortic valve and premature coronary artery disease
- Mother with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 10 years ago. Alcohol: Social, 1-2 drinks per week. Occupation: Retired civil engineer.

PHYSICAL EXAMINATION:
On physical examination, he appears pale and jaundiced. A systolic ejection murmur is noted.

LABORATORY DATA:
Laboratory tests reveal:

- Hemoglobin: 9.8 g/dL (Normal: 13.8-17.2 g/dL)
- Mean corpuscular volume (MCV): 85 fL (Normal: 80-96 fL)
- Lactate dehydrogenase (LDH): 450 U/L (Normal: 140-280 U/L)
- Haptoglobin: <10 mg/dL (Normal: 30-200 mg/dL)
- Total bilirubin: 3.2 mg/dL (Normal: 0.3-1.2 mg/dL), with indirect bilirubin predominating
- Platelet count: 110,000/µL (Normal: 150,000-450,000/µL)

Peripheral blood smear shows schistocytes.

IMAGING:
An echocardiogram is ordered to assess the valvular function.
