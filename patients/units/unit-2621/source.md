# synthetic_hospital patient 2621

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 29 at first encounter, sex M, Non-Hispanic White, Employer-sponsored PPO (UnitedHealthcare).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-09-08 (Mon) | follow_up | Endocrinology |
| 1 | 2020-07-15 | 2026-02-22 (Sun) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Employer-sponsored PPO (UnitedHealthcare)",
 "occupation": "Financial Analyst",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Mother: Type 1 Diabetes Mellitus, Hashimoto thyroiditis",
  "Father: Generalized anxiety disorder, hypertension",
  "Older brother: Major depressive disorder"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "20 units subcutaneous at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "sliding scale (1 unit per 8g carbohydrate) with meals",
   "name": "Insulin lispro"
  },
  {
   "dose": "50 mg PO daily",
   "name": "Sertraline"
  },
  {
   "dose": "50 mg PO PRN headache (max 2 doses/24hr)",
   "name": "Sumatriptan"
  }
 ],
 "surgical_history": [
  "Wisdom teeth extraction (age 22)"
 ],
 "chronic_conditions": [
  "Type 1 Diabetes Mellitus (diagnosed 3 weeks ago)",
  "Generalized Anxiety Disorder",
  "Episodic migraine"
 ],
 "age_at_first_encounter": 29
}
```

Primary diagnoses: ["Cocaine intoxication", "Diabetic ketoacidosis"]
Comorbidities: ["Situational anxiety"]

## Encounter 0: source 2020-01-15, practice 2025-09-08, follow_up, Endocrinology

Chief complaint: Follow-up after a recent hospitalization for diabetic ketoacidosis

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Sharma, MD
Department: Endocrinology
Visit Type: follow_up

CHIEF COMPLAINT:
Follow-up after a recent hospitalization for diabetic ketoacidosis

HISTORY OF PRESENT ILLNESS:
During the visit, he seems anxious and asks whether his new diagnosis of type 1 diabetes will be documented in his chart. When the physician confirms that all medical diagnoses are included in the electronic record, the patient becomes visibly upset and states, “I’m applying for life insurance and I don’t want them to find out about this.” He asks the physician to remove the diagnosis from his record and instead document that he was admitted for “dehydration.” The physician explains that all medical documentation must reflect the actual clinical diagnosis and that altering records could have legal consequences. The patient insists he will go to another doctor if this request is denied.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 1 Diabetes Mellitus (diagnosed 3 weeks ago)
- Generalized Anxiety Disorder
- Episodic migraine

PAST SURGICAL HISTORY:
- Wisdom teeth extraction (age 22)

MEDICATIONS:
Home Medications:
- Insulin glargine 20 units subcutaneous at bedtime
- Insulin lispro sliding scale (1 unit per 8g carbohydrate) with meals
- Sertraline 50 mg PO daily
- Sumatriptan 50 mg PO PRN headache (max 2 doses/24hr)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 1 Diabetes Mellitus, Hashimoto thyroiditis
- Father: Generalized anxiety disorder, hypertension
- Older brother: Major depressive disorder

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Financial Analyst.

## Encounter 1: source 2020-07-15, practice 2026-02-22, ed, Emergency Medicine

Chief complaint: Severe headache and nosebleed

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe headache and nosebleed

HISTORY OF PRESENT ILLNESS:
This 29-year-old with Type 1 Diabetes and history of DKA presents with severe headache and nosebleed. He denies history of trauma. His friend, who accompanied him, mentions that the patient has been acting "out of sorts" lately but denies knowledge of drug use.

PAST MEDICAL HISTORY:
Active Problem List:
- Diabetic ketoacidosis (diagnosed 2020-01-15)
- Type 1 Diabetes Mellitus (diagnosed 3 weeks ago)
- Generalized Anxiety Disorder
- Episodic migraine

He has a past medical history of episodic headaches and has been treated for anxiety in the past.

PAST SURGICAL HISTORY:
- Wisdom teeth extraction (age 22)

MEDICATIONS:
Home Medications:
- Insulin glargine 20 units subcutaneous at bedtime
- Insulin lispro sliding scale (1 unit per 8g carbohydrate) with meals
- Sertraline 50 mg PO daily
- Sumatriptan 50 mg PO PRN headache (max 2 doses/24hr)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Type 1 Diabetes Mellitus, Hashimoto thyroiditis
- Father: Generalized anxiety disorder, hypertension
- Older brother: Major depressive disorder

SOCIAL HISTORY:
The patient reluctantly admits to occasional recreational drug use but does not specify which substances.

VITAL SIGNS:
His blood pressure is 180/110 mmHg. His heart rate is 102 beats per minute and regular.

PHYSICAL EXAMINATION:
On physical examination, you note a tremor in his hands and a small amount of blood in his right nostril. He appears anxious and has dilated pupils.

PLAN:
A urine drug screen is ordered.
