# synthetic_hospital patient 2557

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 25 at first encounter, sex M, Caucasian, Non-Hispanic, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-05-24 (Tue) | ed | Emergency Medicine |
| 1 | 2020-02-15 | 2022-06-20 (Mon) | ed | Emergency Medicine |
| 2 | 2021-01-15 | 2023-05-19 (Fri) | ed | Emergency Medicine |
| 3 | 2023-01-15 | 2025-05-22 (Thu) | inpatient | Internal Medicine |
| 4 | 2023-07-15 | 2025-11-28 (Fri) | outpatient | Psychiatry |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Haloperidol (oculogyric crisis/acute dystonia)",
  "Trimethoprim-sulfamethoxazole (agranulocytosis)"
 ],
 "insurance": "Medicaid",
 "occupation": "Part-time warehouse associate (previously enrolled in college, withdrew due to illness)",
 "alcohol_use": "None",
 "family_history": [
  "Father with schizophrenia",
  "Maternal aunt with bipolar disorder"
 ],
 "race_ethnicity": "Caucasian, Non-Hispanic",
 "smoking_status": "Never",
 "home_medications": [
  {
   "dose": "300 mg orally daily",
   "name": "Clozapine"
  },
  {
   "dose": "100 mg orally daily",
   "name": "Docusate sodium"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Schizophrenia, paranoid type, treatment-resistant"
 ],
 "age_at_first_encounter": 25
}
```

Primary diagnoses: ["Febrile neutropenia", "Agranulocytosis", "Schizophrenia", "Oculogyric crisis"]
Comorbidities: ["Schizophrenia"]

## Encounter 0: source 2020-01-15, practice 2022-05-24, ed, Emergency Medicine

Chief complaint: Patient brought to emergency department by his father due to increasingly bizarre behavior over the past several months, including social withdrawal, paranoid delusions about hidden cameras, and withdrawal from college.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Patient brought to emergency department by his father due to increasingly bizarre behavior over the past several months, including social withdrawal, paranoid delusions about hidden cameras, and withdrawal from college.

HISTORY OF PRESENT ILLNESS:
The patient has withdrawn from his friends and dropped out of college, claiming others were spreading rumors about him and watching him through hidden cameras. He often locks himself in his room and talks to himself for hours. At home, he has become neglectful of personal hygiene and no longer engages in routine chores or family discussions. The father reports that he sometimes sees his son gesturing or whispering as if responding to someone who isn't there.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizophrenia, paranoid type, treatment-resistant

MEDICATIONS:
Home Medications:
- Clozapine 300 mg orally daily
- Docusate sodium 100 mg orally daily

ALLERGIES:
Haloperidol (oculogyric crisis/acute dystonia), Trimethoprim-sulfamethoxazole (agranulocytosis)

FAMILY HISTORY:
Family history is negative for psychiatric disorders.

SOCIAL HISTORY:
The patient denies any substance use

PHYSICAL EXAMINATION:
On evaluation, the patient speaks in short phrases with long pauses and appears emotionally detached.

## Encounter 1: source 2020-02-15, practice 2022-06-20, ed, Emergency Medicine

Chief complaint: Acute onset of neck stiffness and upward deviation of the eyes that began two hours ago, following initiation of a new antipsychotic medication during a psychiatric clinic visit one week prior.

ENCOUNTER NOTE
Date: 2020-02-15
Provider: Dr. Michael Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset of neck stiffness and upward deviation of the eyes that began two hours ago, following initiation of a new antipsychotic medication during a psychiatric clinic visit one week prior.

HISTORY OF PRESENT ILLNESS:
The patient returns one month after presentation for psychotic symptoms with acute onset of neck stiffness and upward deviation of the eyes that began two hours ago. He reports starting a new medication during his last psychiatric clinic visit one week ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizophrenia (diagnosed 2020-01-15)
- Schizophrenia, paranoid type, treatment-resistant

MEDICATIONS:
His recent medication list includes haloperidol, started 7 days ago

Home Medications:
- Clozapine 300 mg orally daily
- Docusate sodium 100 mg orally daily

ALLERGIES:
Haloperidol (oculogyric crisis/acute dystonia), Trimethoprim-sulfamethoxazole (agranulocytosis)

FAMILY HISTORY:
- Father with schizophrenia
- Maternal aunt with bipolar disorder

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Part-time warehouse associate (previously enrolled in college, withdrew due to illness).

VITAL SIGNS:
His vital signs are within normal limits

PHYSICAL EXAMINATION:
Physical examination reveals sustained muscle contractions causing twisting and repetitive movements or abnormal postures

## Encounter 2: source 2021-01-15, practice 2023-05-19, ed, Emergency Medicine

Chief complaint: Brought to the emergency department by his roommate who reports the patient has been exhibiting increasingly bizarre behavior over the past six months, including social withdrawal and beliefs that his thoughts are being controlled by external forces.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Brought to the emergency department by his roommate who reports the patient has been exhibiting increasingly bizarre behavior over the past six months, including social withdrawal and beliefs that his thoughts are being controlled by external forces.

HISTORY OF PRESENT ILLNESS:
The patient with treatment-resistant paranoid schizophrenia returns one year after initial diagnosis and oculogyric crisis. Over the past 6 months, the patient has become socially withdrawn, often talking to himself, and has expressed beliefs that his thoughts are being controlled by external force.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizophrenia (diagnosed 2020-01-15)
- Oculogyric crisis (diagnosed 2020-02-15)
- Schizophrenia, paranoid type, treatment-resistant

MEDICATIONS:
Home Medications:
- Clozapine 300 mg orally daily
- Docusate sodium 100 mg orally daily

ALLERGIES:
Haloperidol (oculogyric crisis/acute dystonia), Trimethoprim-sulfamethoxazole (agranulocytosis)

FAMILY HISTORY:
- Father with schizophrenia
- Maternal aunt with bipolar disorder

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Part-time warehouse associate (previously enrolled in college, withdrew due to illness).

PHYSICAL EXAMINATION:
On examination, the patient is disheveled, avoids eye contact, and appears to be responding to internal stimuli. He is oriented to person but not to place or time. Neurological examination is unremarkable.

LABORATORY DATA:
A urine toxicology screen is negative.

IMAGING:
A non-contrast head CT scan shows enlargement of the lateral cerebral ventricles.

## Encounter 3: source 2023-01-15, practice 2025-05-22, inpatient, Internal Medicine

Chief complaint: Three-day history of high fever, profound fatigue, and painful oral sores following treatment with trimethoprim-sulfamethoxazole for a skin abscess ten days ago.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Amara Okafor, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Three-day history of high fever, profound fatigue, and painful oral sores following treatment with trimethoprim-sulfamethoxazole for a skin abscess ten days ago.

HISTORY OF PRESENT ILLNESS:
The patient is a 28-year-old male with treatment-resistant paranoid schizophrenia on clozapine with a history of oculogyric crisis, who presents to the emergency department with a 3-day history of fever, profound fatigue, and painful sores in his mouth. He was treated with trimethoprim-sulfamethoxazole for a skin abscess 10 days ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizophrenia (diagnosed 2020-01-15)
- Oculogyric crisis (diagnosed 2020-02-15)
- Schizophrenia, paranoid type, treatment-resistant

MEDICATIONS:
Home Medications:
- Clozapine 300 mg orally daily
- Docusate sodium 100 mg orally daily

ALLERGIES:
Haloperidol (oculogyric crisis/acute dystonia), Trimethoprim-sulfamethoxazole (agranulocytosis)

FAMILY HISTORY:
- Father with schizophrenia
- Maternal aunt with bipolar disorder

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Part-time warehouse associate (previously enrolled in college, withdrew due to illness).

VITAL SIGNS:
Temperature of 38.9°C (102°F)

PHYSICAL EXAMINATION:
Multiple ulcerative lesions along the gingival margin and buccal mucosa

PLAN:
Blood cultures and empiric antibiotics are initiated.

## Encounter 4: source 2023-07-15, practice 2025-11-28, outpatient, Psychiatry

Chief complaint: Scheduled routine follow-up appointment to assess control of auditory hallucinations and tolerability of current psychiatric medication regimen.

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Priya Patel, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Scheduled routine follow-up appointment to assess control of auditory hallucinations and tolerability of current psychiatric medication regimen.

HISTORY OF PRESENT ILLNESS:
The patient with treatment-resistant schizophrenia and agranulocytosis returns for follow-up. He reports significant improvement in his auditory hallucinations and delusional thinking. He denies any fever, sore throat, or recent infections.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizophrenia (diagnosed 2020-01-15)
- Oculogyric crisis (diagnosed 2020-02-15)
- Agranulocytosis (diagnosed 2023-01-15)
- Schizophrenia, paranoid type, treatment-resistant

MEDICATIONS:
He has been on clozapine 300 mg daily for the past 6 months

Home Medications:
- Docusate sodium 100 mg orally daily

ALLERGIES:
Haloperidol (oculogyric crisis/acute dystonia), Trimethoprim-sulfamethoxazole (agranulocytosis)

FAMILY HISTORY:
- Father with schizophrenia
- Maternal aunt with bipolar disorder

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Part-time warehouse associate (previously enrolled in college, withdrew due to illness).

PHYSICAL EXAMINATION:
His physical examination is unremarkable.

LABORATORY DATA:
- White Blood Cell Count (WBC): 3,200/mm³
- Absolute Neutrophil Count (ANC): 800/mm³
- Hemoglobin: 14 g/dL
- Platelets: 250,000/mm³
