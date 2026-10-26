# synthetic_hospital patient 1847

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 24 at first encounter, sex F, Non-Hispanic White, University student health insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-02-10 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-04-15 | 2025-05-08 (Thu) | ed | Emergency Medicine |
| 2 | 2021-01-15 | 2026-02-12 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Sulfamethoxazole-trimethoprim (Stevens-Johnson syndrome)"
 ],
 "insurance": "University student health insurance",
 "occupation": "Graduate student",
 "alcohol_use": "None",
 "family_history": [
  "Mother with migraine disorder",
  "Father with familial hypercholesterolemia",
  "Paternal grandfather with premature coronary artery disease (MI at age 46)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "50 mg oral PRN migraine headache",
   "name": "Sumatriptan"
  },
  {
   "dose": "40 mg oral daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "300 mg oral daily",
   "name": "Isoniazid"
  },
  {
   "dose": "50 mg oral daily",
   "name": "Pyridoxine"
  }
 ],
 "surgical_history": [
  "Laparoscopic cholecystectomy"
 ],
 "chronic_conditions": [
  "Migraine without aura",
  "Familial hypercholesterolemia",
  "Latent tuberculosis infection"
 ],
 "age_at_first_encounter": 24
}
```

Primary diagnoses: ["Secondary dystonia due to intracranial lesion", "Statin-induced hepatotoxicity", "Stevens-Johnson syndrome"]
Comorbidities: ["Statin-induced hepatotoxicity"]

## Encounter 0: source 2020-01-15, practice 2025-02-10, outpatient, Internal Medicine

Chief complaint: presents to the clinic for a routine follow-up after starting treatment for latent tuberculosis infection 3 weeks ago.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
She presents to the clinic for a routine follow-up after starting treatment for latent tuberculosis infection 3 weeks ago.

HISTORY OF PRESENT ILLNESS:
She has no complaints and is adherent to her medication regimen.

PAST MEDICAL HISTORY:
Active Problem List:
- Migraine without aura
- Familial hypercholesterolemia
- Latent tuberculosis infection

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy

MEDICATIONS:
Home Medications:
- Sumatriptan 50 mg oral PRN migraine headache
- Atorvastatin 40 mg oral daily
- Isoniazid 300 mg oral daily
- Pyridoxine 50 mg oral daily

ALLERGIES:
Sulfamethoxazole-trimethoprim (Stevens-Johnson syndrome)

FAMILY HISTORY:
- Mother with migraine disorder
- Father with familial hypercholesterolemia
- Paternal grandfather with premature coronary artery disease (MI at age 46)

SOCIAL HISTORY:
She does not smoke or consume alcohol.

VITAL SIGNS:
On examination, her vital signs are within normal limits,

PHYSICAL EXAMINATION:
Her physical examination is unremarkable.

LABORATORY DATA:
Laboratory tests reveal the following:

- AST: 70 U/L (Normal: 10-40 U/L)
- ALT: 85 U/L (Normal: 7-56 U/L)
- Total bilirubin: 0.8 mg/dL (Normal: 0.1-1.2 mg/dL)
- Alkaline phosphatase: 76 U/L (Normal: 44-147 U/L)

## Encounter 1: source 2020-04-15, practice 2025-05-08, ed, Emergency Medicine

Chief complaint: presents to the emergency department with a 2-hour history of involuntary, sustained upward gaze and neck extension.

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
She presents to the emergency department with a 2-hour history of involuntary, sustained upward gaze and neck extension.

HISTORY OF PRESENT ILLNESS:
This 24-year-old female with migraine without aura, familial hypercholesterolemia, and latent tuberculosis infection on isoniazid was previously evaluated for statin-induced hepatotoxicity three months ago. She presents with 2-hour involuntary sustained upward gaze and neck extension and reports nausea and vomiting for three days following laparoscopic cholecystectomy and was prescribed medication for symptoms. She denies recent head injury, fever, or illicit drugs.

PAST MEDICAL HISTORY:
Active Problem List:
- Statin-induced hepatotoxicity (diagnosed 2020-01-15)
- Migraine without aura
- Familial hypercholesterolemia
- Latent tuberculosis infection

Her medical history is significant for migraines,.

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy

MEDICATIONS:
She takes for which she takes sumatriptan as needed.

Home Medications:
- Atorvastatin 40 mg oral daily
- Isoniazid 300 mg oral daily
- Pyridoxine 50 mg oral daily

ALLERGIES:
Sulfamethoxazole-trimethoprim (Stevens-Johnson syndrome)

FAMILY HISTORY:
- Mother with migraine disorder
- Father with familial hypercholesterolemia
- Paternal grandfather with premature coronary artery disease (MI at age 46)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Graduate student.

VITAL SIGNS:
On examination, her vital signs are stable,

PHYSICAL EXAMINATION:
There is no evidence of trauma. Neurological examination reveals bilateral sustained contraction of the neck extensors and upward deviation of the eyes, which she is unable to voluntarily control. She appears anxious and reports feeling restless.

## Encounter 2: source 2021-01-15, practice 2026-02-12, ed, Emergency Medicine

Chief complaint: painful oral ulcers, fever, and a rapidly progressing rash

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Painful oral ulcers, fever, and a rapidly progressing rash

HISTORY OF PRESENT ILLNESS:
This 25-year-old female with migraine, familial hypercholesterolemia, latent tuberculosis infection, and prior hepatotoxicity and dystonia presents to the emergency department with a 2-day history of painful oral ulcers, fever, and a rapidly progressing rash. She recently began treatment for urinary tract infection.

PAST MEDICAL HISTORY:
Active Problem List:
- Statin-induced hepatotoxicity (diagnosed 2020-01-15)
- Secondary dystonia due to intracranial lesion (diagnosed 2020-04-15)
- Migraine without aura
- Familial hypercholesterolemia
- Latent tuberculosis infection

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy

MEDICATIONS:
Home Medications:
- Sumatriptan 50 mg oral PRN migraine headache
- Atorvastatin 40 mg oral daily
- Isoniazid 300 mg oral daily
- Pyridoxine 50 mg oral daily

ALLERGIES:
Sulfamethoxazole-trimethoprim (Stevens-Johnson syndrome)

FAMILY HISTORY:
- Mother with migraine disorder
- Father with familial hypercholesterolemia
- Paternal grandfather with premature coronary artery disease (MI at age 46)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Graduate student.

PHYSICAL EXAMINATION:
Physical examination reveals diffuse erythematous macules with dusky centers and several flaccid bullae involving her trunk, face, and extremities. Nikolsky sign is positive. Mucosal involvement is evident in the oral cavity and genital area.

LABORATORY DATA:
Her lab results include an elevated ESR and transaminases.

PATHOLOGY:
A skin biopsy shows keratinocyte apoptosis with full-thickness epidermal necrosis and minimal dermal inflammation.
