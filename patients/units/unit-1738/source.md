# synthetic_hospital patient 1738

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex F, Non-Hispanic White, Commercial PPO (Blue Cross Blue Shield).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-07-01 (Mon) | outpatient | Dermatology |
| 1 | 2020-09-15 | 2025-02-18 (Tue) | icu | Internal Medicine |
| 2 | 2021-05-15 | 2025-10-29 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO (Blue Cross Blue Shield)",
 "occupation": "Elementary school teacher",
 "alcohol_use": "Social (1-2 drinks per month)",
 "family_history": [
  "Mother: Hashimoto's thyroiditis",
  "Father: Type 2 Diabetes Mellitus",
  "Maternal aunt: Rheumatoid arthritis",
  "Sister: Celiac disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "18 units subcutaneous at bedtime",
   "name": "Insulin glargine (Lantus)"
  },
  {
   "dose": "Sliding scale subcutaneous with meals (I:C ratio 1:12)",
   "name": "Insulin lispro (Humalog)"
  },
  {
   "dose": "88 mcg orally daily",
   "name": "Levothyroxine (Synthroid)"
  }
 ],
 "surgical_history": [
  "Wisdom teeth extraction (age 18)"
 ],
 "chronic_conditions": [
  "Type 1 Diabetes Mellitus",
  "Hashimoto's Hypothyroidism",
  "Alopecia areata"
 ],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Diabetic ketoacidosis", "Acute transverse myelitis", "Alopecia areata"]
Comorbidities: ["Upper respiratory tract infection", "Type 1 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2024-07-01, outpatient, Dermatology

Chief complaint: 3-month history of hair loss with round, well-demarcated bald patches on scalp

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Emily Chen, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
3-month history of hair loss with round, well-demarcated bald patches on scalp

HISTORY OF PRESENT ILLNESS:
She reports the development of several round, well-demarcated, completely bald patches on her scalp. She denies any associated pain, itching, or burning.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 1 Diabetes Mellitus
- Hashimoto's Hypothyroidism
- Alopecia areata

PAST SURGICAL HISTORY:
- Wisdom teeth extraction (age 18)

MEDICATIONS:
Insulin and levothyroxine

Home Medications:
- Insulin glargine (Lantus) 18 units subcutaneous at bedtime
- Insulin lispro (Humalog) Sliding scale subcutaneous with meals (I:C ratio 1:12)
- Levothyroxine (Synthroid) 88 mcg orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hashimoto's thyroiditis
- Father: Type 2 Diabetes Mellitus
- Maternal aunt: Rheumatoid arthritis
- Sister: Celiac disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
On physical examination, there are three coin-sized areas of hair loss on the scalp without scaling or erythema. Exclamation mark hairs are noted at the periphery of the bald patches. No lymphadenopathy is palpable. Nail examination reveals pitting. The remainder of the examination is unremarkable. No other skin lesions are noted.

LABORATORY DATA:
Thyroid-stimulating hormone (TSH): 2.5 mIU/L (normal range: 0.4-4.0 mIU/L)
- Hemoglobin A1c: 6.8%
- Complete blood count (CBC): within normal limits
- Antinuclear antibodies (ANA): Negative

## Encounter 1: source 2020-09-15, practice 2025-02-18, icu, Internal Medicine

Chief complaint: Confusion, vomiting, and rapid breathing during viral illness

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. James Okafor, MD
Department: Internal Medicine
Visit Type: icu

CHIEF COMPLAINT:
Confusion, vomiting, and rapid breathing during viral illness

HISTORY OF PRESENT ILLNESS:
The patient is a 32-year-old female with Type 1 Diabetes Mellitus, Hashimoto's hypothyroidism, and alopecia areata diagnosed earlier this year, who is brought to the emergency department by her roommate due to confusion, vomiting, and rapid breathing. She reports having been ill with a viral upper respiratory infection for the past several days and has not been taking her insulin regularly. During this metabolic state,

PAST MEDICAL HISTORY:
Active Problem List:
- Alopecia areata (diagnosed 2020-01-15)
- Type 1 Diabetes Mellitus
- Hashimoto's Hypothyroidism

PAST SURGICAL HISTORY:
- Wisdom teeth extraction (age 18)

MEDICATIONS:
Home Medications:
- Insulin glargine (Lantus) 18 units subcutaneous at bedtime
- Insulin lispro (Humalog) Sliding scale subcutaneous with meals (I:C ratio 1:12)
- Levothyroxine (Synthroid) 88 mcg orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hashimoto's thyroiditis
- Father: Type 2 Diabetes Mellitus
- Maternal aunt: Rheumatoid arthritis
- Sister: Celiac disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
Physical exam reveals dry mucous membranes, tachypnea with deep respirations, and fruity breath odor.

LABORATORY DATA:
Arterial blood gas shows a high anion gap metabolic acidosis. Urinalysis is positive for ketones and glucose.

## Encounter 2: source 2021-05-15, practice 2025-10-29, ed, Emergency Medicine

Chief complaint: Progressive leg weakness and difficulty urinating for 2 days with ascending sensory changes

ENCOUNTER NOTE
Date: 2021-05-15
Provider: Dr. Sofia Martinez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive leg weakness and difficulty urinating for 2 days with ascending sensory changes

HISTORY OF PRESENT ILLNESS:
33-year-old female with Type 1 Diabetes Mellitus, Hashimoto's Hypothyroidism, Alopecia areata, and prior DKA, presents with progressive leg weakness. She reports a tingling sensation that started in her toes and has ascended to her umbilicus. She denies any recent infections or vaccinations but mentions that she had a mild respiratory illness 3 weeks ago that resolved without treatment.

PAST MEDICAL HISTORY:
Active Problem List:
- Alopecia areata (diagnosed 2020-01-15)
- Diabetic ketoacidosis (diagnosed 2020-09-15)
- Type 1 Diabetes Mellitus
- Hashimoto's Hypothyroidism

PAST SURGICAL HISTORY:
- Wisdom teeth extraction (age 18)

MEDICATIONS:
Home Medications:
- Insulin glargine (Lantus) 18 units subcutaneous at bedtime
- Insulin lispro (Humalog) Sliding scale subcutaneous with meals (I:C ratio 1:12)
- Levothyroxine (Synthroid) 88 mcg orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Hashimoto's thyroiditis
- Father: Type 2 Diabetes Mellitus
- Maternal aunt: Rheumatoid arthritis
- Sister: Celiac disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
On examination, her strength is 3/5 in both lower extremities, with hyperreflexia and bilateral Babinski signs. Sensation to light touch and pinprick is diminished below the level of the umbilicus.

LABORATORY DATA:
Laboratory Values:
- White Blood Cell Count: 7,000/uL (normal range: 4,500-11,000/uL)
- Erythrocyte Sedimentation Rate (ESR): 20 mm/hr (normal range: 0-20 mm/hr)
- C-reactive Protein (CRP): 1 mg/L (normal range: <3 mg/L)
- Comprehensive Metabolic Panel: Normal
- Aquaporin-4 antibody: Negative
- Myelin Oligodendrocyte Glycoprotein (MOG) antibody: Negative

IMAGING:
An MRI of the spine reveals a hyperintense lesion on T2-weighted images extending from T5 to T7. No enhancement is seen with gadolinium.
