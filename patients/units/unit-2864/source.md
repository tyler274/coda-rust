# synthetic_hospital patient 2864

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 78 at first encounter, sex F, Non-Hispanic White, Medicare and Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-07-02 (Tue) | outpatient | Obstetrics & Gynecology |
| 1 | 2020-07-15 | 2024-12-26 (Thu) | outpatient | Psychiatry |
| 2 | 2021-07-15 | 2026-01-09 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare and Medicaid",
 "occupation": "Retired elementary school teacher",
 "alcohol_use": "None",
 "family_history": [
  "Alzheimer's disease in mother",
  "Osteoporosis in mother",
  "Hypertension in father",
  "Type 2 diabetes mellitus in older sister"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never",
 "home_medications": [
  {
   "dose": "10 mg PO daily",
   "name": "Donepezil"
  },
  {
   "dose": "10 mg PO twice daily",
   "name": "Memantine"
  },
  {
   "dose": "1 mg PO twice daily",
   "name": "Risperidone"
  },
  {
   "dose": "600 mg/400 units PO twice daily",
   "name": "Calcium carbonate with Vitamin D3"
  },
  {
   "dose": "17 g PO daily",
   "name": "Polyethylene glycol 3350"
  },
  {
   "dose": "100 mg PO daily",
   "name": "Docusate sodium"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Omeprazole"
  }
 ],
 "surgical_history": [
  "Cholecystectomy, 1995",
  "Bilateral cataract extraction with intraocular lens placement, 2018 (right) and 2019 (left)"
 ],
 "chronic_conditions": [
  "Alzheimer's disease (advanced)",
  "Osteoporosis",
  "Chronic constipation",
  "Dysphagia secondary to neurodegenerative disease",
  "Multiparity (G4P4)"
 ],
 "age_at_first_encounter": 78
}
```

Primary diagnoses: ["Recurrent aspiration pneumonia", "Symptomatic Pelvic Organ Prolapse", "Neuroleptic-induced akathisia"]
Comorbidities: ["Alzheimer's disease"]

## Encounter 0: source 2020-01-15, practice 2024-07-02, outpatient, Obstetrics & Gynecology

Chief complaint: increased difficulty with urination and a visible bulge protruding from her vagina

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Elena Vasquez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Increased difficulty with urination and a visible bulge protruding from her vagina

HISTORY OF PRESENT ILLNESS:
She is brought to the clinic by her caregiver due to concerns about
, where the nursing staff first noticed the mass one week ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Alzheimer's disease (advanced)
- Osteoporosis
- Chronic constipation
- Dysphagia secondary to neurodegenerative disease
- Multiparity (G4P4)

The patient has a history of Alzheimer disease and is nonverbal. Her medical history includes four prior vaginal deliveries, osteoporosis, and chronic constipation.

PAST SURGICAL HISTORY:
- Cholecystectomy, 1995
- Bilateral cataract extraction with intraocular lens placement, 2018 (right) and 2019 (left)

MEDICATIONS:
Home Medications:
- Donepezil 10 mg PO daily
- Memantine 10 mg PO twice daily
- Risperidone 1 mg PO twice daily
- Calcium carbonate with Vitamin D3 600 mg/400 units PO twice daily
- Polyethylene glycol 3350 17 g PO daily
- Docusate sodium 100 mg PO daily
- Omeprazole 20 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Alzheimer's disease in mother
- Osteoporosis in mother
- Hypertension in father
- Type 2 diabetes mellitus in older sister

SOCIAL HISTORY:
She resides in a long-term care facility

PHYSICAL EXAMINATION:
Pelvic examination reveals a soft, smooth mass protruding from the vaginal opening that is easily repositioned with gentle pressure. The vaginal mucosa appears intact with no ulceration or bleeding.

LABORATORY DATA:
Urinalysis is unremarkable.

## Encounter 1: source 2020-07-15, practice 2024-12-26, outpatient, Psychiatry

Chief complaint: brought to the clinic by her daughter due to concerns about her mother's recent behavior

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. David Park, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Brought to the clinic by her daughter due to concerns about her mother's recent behavior

HISTORY OF PRESENT ILLNESS:
The patient is a 78-year-old female with advanced Alzheimer's disease and a history of symptomatic pelvic organ prolapse evaluated in January 2020 who presents for assessment of behavioral changes. Over the past month, she has been increasingly agitated and aggressive, which prompted her primary care physician to start treatment with risperidone 1 mg twice daily. The daughter reports that while her mother's aggression has somewhat improved, she has developed a new problem. The patient has been unable to sit still, frequently pacing around the room, and appears restless.

PAST MEDICAL HISTORY:
Active Problem List:
- Symptomatic Pelvic Organ Prolapse (diagnosed 2020-01-15)
- Alzheimer's disease (advanced)
- Osteoporosis
- Chronic constipation
- Dysphagia secondary to neurodegenerative disease
- Multiparity (G4P4)

PAST SURGICAL HISTORY:
- Cholecystectomy, 1995
- Bilateral cataract extraction with intraocular lens placement, 2018 (right) and 2019 (left)

MEDICATIONS:
Her other medications include donepezil and memantine.

Home Medications:
- Risperidone 1 mg PO twice daily
- Calcium carbonate with Vitamin D3 600 mg/400 units PO twice daily
- Polyethylene glycol 3350 17 g PO daily
- Docusate sodium 100 mg PO daily
- Omeprazole 20 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Alzheimer's disease in mother
- Osteoporosis in mother
- Hypertension in father
- Type 2 diabetes mellitus in older sister

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Retired elementary school teacher.

VITAL SIGNS:
Vital signs are within normal limits.

PHYSICAL EXAMINATION:
On examination, the patient is unable to remain seated, constantly shifting in her chair, and reports a subjective feeling of inner restlessness. Neurological examination does not reveal any rigidity or tremor.

## Encounter 2: source 2021-07-15, practice 2026-01-09, ed, Emergency Medicine

Chief complaint: a 2-day history of fever, cough, and increased somnolence

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Amara Okafor, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
A 2-day history of fever, cough, and increased somnolence

HISTORY OF PRESENT ILLNESS:
This 79-year-old with advanced Alzheimer's, dysphagia, and akathisia presents. The caregiver reports that the patient has had difficulty swallowing her food and has been coughing during meals.

PAST MEDICAL HISTORY:
Active Problem List:
- Symptomatic Pelvic Organ Prolapse (diagnosed 2020-01-15)
- Neuroleptic-induced akathisia (diagnosed 2020-07-15)
- Alzheimer's disease (advanced)
- Osteoporosis
- Chronic constipation
- Dysphagia secondary to neurodegenerative disease
- Multiparity (G4P4)

PAST SURGICAL HISTORY:
- Cholecystectomy, 1995
- Bilateral cataract extraction with intraocular lens placement, 2018 (right) and 2019 (left)

MEDICATIONS:
Home Medications:
- Donepezil 10 mg PO daily
- Memantine 10 mg PO twice daily
- Risperidone 1 mg PO twice daily
- Calcium carbonate with Vitamin D3 600 mg/400 units PO twice daily
- Polyethylene glycol 3350 17 g PO daily
- Docusate sodium 100 mg PO daily
- Omeprazole 20 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Alzheimer's disease in mother
- Osteoporosis in mother
- Hypertension in father
- Type 2 diabetes mellitus in older sister

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Retired elementary school teacher.

VITAL SIGNS:
The patient's temperature is 38.6°C (101.5°F), blood pressure is 130/80 mmHg, pulse is 102/min, and respiratory rate is 22/min.

PHYSICAL EXAMINATION:
Physical examination reveals crackles at the right lung base. The patient is not oriented to time, place, or person.

LABORATORY DATA:
Laboratory studies show:

- White blood cell count: 15,000/mm³
- Neutrophils: 80%
- Bands: 12%
- Hemoglobin: 12.2 g/dL
- Platelet count: 250,000/mm³
- Blood urea nitrogen: 28 mg/dL
- Creatinine: 1.1 mg/dL
- Arterial blood gas on room air:
- pH: 7.45
- PaCO2: 45 mmHg
- PaO2: 60 mmHg

IMAGING:
Chest X-ray shows an infiltrate in the right lower lobe.
