# synthetic_hospital patient 2112

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 46 at first encounter, sex M, Non-Hispanic White, UnitedHealthcare PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-02-20 (Thu) | outpatient | Internal Medicine |
| 1 | 2021-01-15 | 2026-02-16 (Mon) | outpatient | Neurology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "UnitedHealthcare PPO",
 "occupation": "High school mathematics teacher",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Father with Charcot-Marie-Tooth disease",
  "Paternal grandfather with myocardial infarction at age 52",
  "Mother with type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never",
 "home_medications": [
  {
   "dose": "20 mg daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "300 mg at bedtime",
   "name": "Gabapentin"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Charcot-Marie-Tooth disease type 2",
  "Dyslipidemia",
  "Elevated blood pressure"
 ],
 "age_at_first_encounter": 46
}
```

Primary diagnoses: ["Microtubule motor protein dysfunction (Kinesin/Dynein defect)", "Lifestyle Modification for Cardiovascular Risk Reduction"]
Comorbidities: ["Cardiovascular disease risk factors"]

## Encounter 0: source 2020-01-15, practice 2025-02-20, outpatient, Internal Medicine

Chief complaint: Routine check-up with no acute complaints; interested in understanding cardiovascular health risks

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine check-up with no acute complaints; interested in understanding cardiovascular health risks

HISTORY OF PRESENT ILLNESS:
He is an avid runner. He is interested in understanding his health risks as

PAST MEDICAL HISTORY:
Active Problem List:
- Charcot-Marie-Tooth disease type 2
- Dyslipidemia
- Elevated blood pressure

MEDICATIONS:
Home Medications:
- Atorvastatin 20 mg daily
- Gabapentin 300 mg at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
His father had a myocardial infarction at age 50.

SOCIAL HISTORY:
He does not smoke and drinks alcohol socially.

VITAL SIGNS:
His blood pressure is 125/80 mmHg, and his BMI is 23 kg/m^2.

PHYSICAL EXAMINATION:
His physical examination is within normal limits.

LABORATORY DATA:
Laboratory results are as follows:

- Total cholesterol: 195 mg/dL
- HDL cholesterol: 55 mg/dL
- LDL cholesterol: 120 mg/dL
- Triglycerides: 100 mg/dL
- Fasting glucose: 88 mg/dL

ASSESSMENT AND PLAN:
The patient inquires about the relationship between his running habit and his cholesterol levels. You recall a recent study that reported a correlation coefficient (r) of 0.4 between regular aerobic exercise and HDL cholesterol levels in middle-aged men. Based on this information,

## Encounter 1: source 2021-01-15, practice 2026-02-16, outpatient, Neurology

Chief complaint: Progressive numbness and tingling in hands and feet for the past 8 months with mild weakness climbing stairs and difficulty with fine motor tasks

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Marcus Johnson, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive numbness and tingling in hands and feet for the past 8 months with mild weakness climbing stairs and difficulty with fine motor tasks

HISTORY OF PRESENT ILLNESS:
The patient with Charcot-Marie-Tooth disease type 2 returns one year after his cardiovascular risk assessment. He reports mild weakness when climbing stairs and occasional difficulty with fine motor tasks, such as buttoning his shirt. He has no history of trauma, diabetes, alcohol use, or recent infections.

PAST MEDICAL HISTORY:
Active Problem List:
- Lifestyle Modification for Cardiovascular Risk Reduction (diagnosed 2020-01-15)
- Charcot-Marie-Tooth disease type 2
- Dyslipidemia
- Elevated blood pressure

MEDICATIONS:
Home Medications:
- Atorvastatin 20 mg daily
- Gabapentin 300 mg at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
His family history is notable for a brother diagnosed with a hereditary neuropathy in his 30s.

SOCIAL HISTORY:
Smoking: Never. Alcohol: Social (1-2 drinks per week). Occupation: High school mathematics teacher.

PHYSICAL EXAMINATION:
On examination, there is reduced vibration and pinprick sensation in a stocking-glove distribution, mild distal muscle weakness in the lower limbs, and absent ankle reflexes. Gait is broad-based but stable. The remainder of the neurologic examination is unremarkable.

OTHER STUDIES:
Nerve conduction studies show slowed conduction velocities in both sensory and motor nerves of the upper and lower extremities.

PLAN:
The neurologist decides to obtain a nerve biopsy to confirm the diagnosis but wants to avoid any additional motor impairment in this patient.
