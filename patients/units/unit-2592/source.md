# synthetic_hospital patient 2592

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 33 at first encounter, sex M, White, non-Hispanic, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-02-05 (Wed) | outpatient | Psychiatry |
| 1 | 2020-07-15 | 2025-08-12 (Tue) | outpatient | Psychiatry |
| 2 | 2020-09-15 | 2025-09-29 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Part-time library assistant",
 "alcohol_use": "None",
 "family_history": [
  "Maternal uncle with schizophrenia",
  "Mother with major depressive disorder"
 ],
 "race_ethnicity": "White, non-Hispanic",
 "smoking_status": "Never",
 "home_medications": [
  {
   "dose": "200 mg PO at bedtime",
   "name": "Clozapine"
  },
  {
   "dose": "1 tablet PO daily",
   "name": "Multivitamin"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Schizophrenia"
 ],
 "age_at_first_encounter": 33
}
```

Primary diagnoses: ["Schizophrenia", "Agranulocytosis"]
Comorbidities: ["Schizophrenia"]

## Encounter 0: source 2020-01-15, practice 2025-02-05, outpatient, Psychiatry

Chief complaint: Increasingly paranoid and verbally aggressive behavior over the past two weeks, noted by family members.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Increasingly paranoid and verbally aggressive behavior over the past two weeks, noted by family members.

HISTORY OF PRESENT ILLNESS:
A 32-year-old male with a history of schizophrenia is brought to the clinic by his sister. She reports that he has been increasingly paranoid and verbally aggressive over the past two weeks.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizophrenia

He was diagnosed with schizophrenia five years ago and has had four psychiatric hospitalizations for psychotic exacerbations.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He is currently prescribed oral risperidone, which he claims to take regularly, but his sister doubts his adherence due to his recent behavior and past patterns.

Home Medications:
- Clozapine 200 mg PO at bedtime
- Multivitamin 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal uncle with schizophrenia
- Mother with major depressive disorder

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Part-time library assistant.

VITAL SIGNS:
His vital signs are within normal limits

PHYSICAL EXAMINATION:
On examination, the patient is disheveled, has poor eye contact, and exhibits auditory hallucinations. no abnormalities are noted on physical examination.

LABORATORY DATA:
His most recent laboratory tests are as follows:

- Complete blood count: within normal limits
- Liver function tests: within normal limits
- Serum creatinine: 0.9 mg/dL
- Fasting blood glucose: 98 mg/dL
- Lipid profile: within normal limits

## Encounter 1: source 2020-07-15, practice 2025-08-12, outpatient, Psychiatry

Chief complaint: Minimal response to two consecutive antipsychotic trials; evaluation for treatment-resistant schizophrenia.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Sarah Chen, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Minimal response to two consecutive antipsychotic trials; evaluation for treatment-resistant schizophrenia.

HISTORY OF PRESENT ILLNESS:
He returns following his January presentation. He has had minimal response to two trials of antipsychotics.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizophrenia (diagnosed 2020-01-15)

He was diagnosed due to symptoms of social withdrawal, auditory hallucinations, and disorganized speech.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Clozapine 200 mg PO at bedtime
- Multivitamin 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal uncle with schizophrenia
- Mother with major depressive disorder

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Part-time library assistant.

PHYSICAL EXAMINATION:
His physical exam and labs are unremarkable. On examination today, h e continues to experience persistent auditory hallucinations and agitation.

LABORATORY DATA:
His physical exam and labs are unremarkable. Following up a month later, complete blood count shows reduced leukocytes .

PLAN:
Based on his treatment history and diagnosis, he is started on a medication that acts as an antagonist at dopamine D2 receptors and serotonin 5-HT2A receptors.

## Encounter 2: source 2020-09-15, practice 2025-09-29, ed, Emergency Medicine

Chief complaint: Severe fatigue, fever, and sore throat for one week; started new antipsychotic six weeks ago.

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe fatigue, fever, and sore throat for one week; started new antipsychotic six weeks ago.

HISTORY OF PRESENT ILLNESS:
The patient is a 33-year-old male with schizophrenia who was evaluated in Psychiatry on 2020-07-15 for treatment-resistant symptoms after two failed antipsychotic trials. He presents to the Emergency Department six weeks after initiating a new medication, which he says has been "life-changing." He reports fewer voices and a significant reduction in paranoid thoughts. His mood is improved, and he has resumed part-time work. However, over the past week, he has experienced increasing fatigue and reports a mild sore throat. He denies cough, chest pain, fever, chills, or night sweats.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizophrenia (diagnosed 2020-01-15)

He has a long history of schizophrenia with persistent auditory hallucinations and paranoid delusions despite trials of multiple antipsychotic medications.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Clozapine 200 mg PO at bedtime
- Multivitamin 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal uncle with schizophrenia
- Mother with major depressive disorder

SOCIAL HISTORY:
He has no history of recent travel, sick contacts, or dental procedures. He is not sexually active and does not use drugs or alcohol.

VITAL SIGNS:
His temperature is 37.3°C (99.1°F), heart rate is 90/min, and blood pressure is 118/76 mm Hg

PHYSICAL EXAMINATION:
He appears tired but is in no acute distress. Oral examination reveals mild pharyngeal erythema and two mouth ulcers. Lungs are clear.
