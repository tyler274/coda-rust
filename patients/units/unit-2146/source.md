# synthetic_hospital patient 2146

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 44 at first encounter, sex M, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-07-02 (Tue) | outpatient | Psychiatry |
| 1 | 2021-07-15 | 2026-01-02 (Fri) | icu | Internal Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Part-time inventory clerk",
 "alcohol_use": "None",
 "family_history": [
  "Mother with schizophrenia",
  "Father with chronic obstructive pulmonary disease",
  "Maternal uncle with bipolar disorder"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 12 pack-years, quit 18 months ago",
 "home_medications": [
  {
   "dose": "4 mg PO daily",
   "name": "Risperidone"
  },
  {
   "dose": "2 L/min via nasal cannula continuous",
   "name": "Oxygen"
  },
  {
   "dose": "17 mcg inhaled QID",
   "name": "Ipratropium bromide inhaler"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Schizophrenia, paranoid type",
  "Chronic hypoxemic respiratory failure",
  "History of neuroleptic-induced extrapyramidal symptoms"
 ],
 "age_at_first_encounter": 44
}
```

Primary diagnoses: ["Akathisia, drug-induced (antipsychotic-induced)", "Chronic hypoxemic respiratory failure"]
Comorbidities: ["Schizophrenia"]

## Encounter 0: source 2020-01-15, practice 2024-07-02, outpatient, Psychiatry

Chief complaint: Feeling 'jittery' and an inability to sit still for the past week

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Feeling 'jittery' and an inability to sit still for the past week

HISTORY OF PRESENT ILLNESS:
He reports that his psychiatrist increased the dose of his antipsychotic medication, risperidone, from 2 mg to 4 mg daily two weeks ago due to persistent auditory hallucinations. He denies any recent stressors, caffeine intake, or changes in his sleep pattern. He denies any muscle stiffness, tremor, or recent infections.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizophrenia, paranoid type
- Chronic hypoxemic respiratory failure
- History of neuroleptic-induced extrapyramidal symptoms

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Risperidone 4 mg PO daily
- Oxygen 2 L/min via nasal cannula continuous
- Ipratropium bromide inhaler 17 mcg inhaled QID

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with schizophrenia
- Father with chronic obstructive pulmonary disease
- Maternal uncle with bipolar disorder

SOCIAL HISTORY:
Smoking: Former smoker, 12 pack-years, quit 18 months ago. Alcohol: None. Occupation: Part-time inventory clerk.

VITAL SIGNS:
His vital signs are within normal limits.

PHYSICAL EXAMINATION:
On examination, the patient appears restless and is constantly moving in his seat. Neurological examination is unremarkable except for the inability to remain still.

LABORATORY DATA:
Laboratory values:
- Complete blood count: within normal limits
- Comprehensive metabolic panel: within normal limits
- Thyroid-stimulating hormone: within normal limits
- Urine toxicology screen: negative

ASSESSMENT AND PLAN:
Given the recent medication change and the patient's symptoms,

## Encounter 1: source 2021-07-15, practice 2026-01-02, icu, Internal Medicine

Chief complaint: Generalized tonic-clonic seizure at home requiring intubation for airway protection

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Marcus Johnson, MD
Department: Internal Medicine
Visit Type: icu

CHIEF COMPLAINT:
Generalized tonic-clonic seizure at home requiring intubation for airway protection

HISTORY OF PRESENT ILLNESS:
The patient is a 45-year-old male with schizophrenia, paranoid type, chronic hypoxemic respiratory failure requiring home oxygen supplementation, and a history of neuroleptic-induced extrapyramidal symptoms including prior akathisia. He presents to the intensive care unit intubated. In the emergency department, he was found to be febrile with severe muscle rigidity, altered mental status, and autonomic instability. He was given supportive care, and sedation was initiated for mechanical ventilation. A long-acting neuromuscular blocker was administered during intubation. Two hours later, his temperature remains elevated, and he is minimally responsive.

PAST MEDICAL HISTORY:
Active Problem List:
- Akathisia, drug-induced (antipsychotic-induced) (diagnosed 2020-01-15)
- Schizophrenia, paranoid type
- Chronic hypoxemic respiratory failure
- History of neuroleptic-induced extrapyramidal symptoms

He has a history of schizophrenia and.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He was recently started on a new antipsychotic.

Home Medications:
- Risperidone 4 mg PO daily
- Oxygen 2 L/min via nasal cannula continuous
- Ipratropium bromide inhaler 17 mcg inhaled QID

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with schizophrenia
- Father with chronic obstructive pulmonary disease
- Maternal uncle with bipolar disorder

SOCIAL HISTORY:
Smoking: Former smoker, 12 pack-years, quit 18 months ago. Alcohol: None. Occupation: Part-time inventory clerk.

VITAL SIGNS:
His respiratory rate is 7 breaths per minute while on minimal ventilator support. His oxygen saturation on room air is 87%, improving to 93% with supplemental oxygen.

PHYSICAL EXAMINATION:
On exam, his pupils are mid-sized and reactive, and he has no adventitious breath sounds. There is no jugular venous distension, leg swelling, or wheezing.

LABORATORY DATA:
Urinalysis shows dark-colored urine without infection. An arterial blood gas is obtained and shows acidemia and elevated pCO₂. Toxicology testing confirms exposure to a paralytic agent but is negative for opioids or benzodiazepines.

IMAGING:
The chest x-ray is clear. A head CT is negative for hemorrhage or mass.

OTHER STUDIES:
The ECG shows sinus tachycardia without ST-T wave changes.
