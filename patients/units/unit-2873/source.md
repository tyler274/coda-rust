# synthetic_hospital patient 2873

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 62 at first encounter, sex F, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-03-04 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-04-15 | 2024-05-29 (Wed) | procedure | Gastroenterology |
| 2 | 2022-01-15 | 2026-02-23 (Mon) | outpatient | Neurology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Penicillin (hives)"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Retired high school librarian",
 "alcohol_use": "None",
 "family_history": [
  "Father: Type 1 diabetes mellitus",
  "Mother: Hashimoto thyroiditis",
  "Sister: Pernicious anemia",
  "Maternal grandmother: Rheumatoid arthritis"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "22 units subcutaneous at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "sliding scale with meals",
   "name": "Insulin lispro"
  },
  {
   "dose": "112 mcg PO daily",
   "name": "Levothyroxine"
  },
  {
   "dose": "1000 mcg IM monthly",
   "name": "Cyanocobalamin"
  }
 ],
 "surgical_history": [
  "Cesarean section (1988)",
  "Tonsillectomy (age 8)"
 ],
 "chronic_conditions": [
  "Type 1 diabetes mellitus",
  "Autoimmune hypothyroidism",
  "Vitiligo",
  "Autoimmune gastritis"
 ],
 "age_at_first_encounter": 62
}
```

Primary diagnoses: ["Autoimmune gastritis with parietal cell loss", "Vitamin B12 deficiency anemia"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2024-03-04, outpatient, Internal Medicine

Chief complaint: 3-month history of fatigue, weakness, and a tingling sensation in her feet with recent unintentional weight loss of 5 kg

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
3-month history of fatigue, weakness, and a tingling sensation in her feet with recent unintentional weight loss of 5 kg

HISTORY OF PRESENT ILLNESS:
She reports a recent unintentional weight loss of 5 kg.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 1 diabetes mellitus
- Autoimmune hypothyroidism
- Vitiligo
- Autoimmune gastritis

Her past medical history includes hypothyroidism and type 1 diabetes mellitus.

PAST SURGICAL HISTORY:
- Cesarean section (1988)
- Tonsillectomy (age 8)

MEDICATIONS:
Home Medications:
- Insulin glargine 22 units subcutaneous at bedtime
- Insulin lispro sliding scale with meals
- Levothyroxine 112 mcg PO daily
- Cyanocobalamin 1000 mcg IM monthly

ALLERGIES:
Penicillin (hives)

FAMILY HISTORY:
- Father: Type 1 diabetes mellitus
- Mother: Hashimoto thyroiditis
- Sister: Pernicious anemia
- Maternal grandmother: Rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired high school librarian.

PHYSICAL EXAMINATION:
On physical examination, her tongue is smooth and shiny. Neurological examination reveals decreased vibration sense in her lower extremities.

LABORATORY DATA:
Her complete blood count shows:

- Hemoglobin: 9.8 g/dL (Normal: 12-16 g/dL)
- Mean corpuscular volume (MCV): 115 fL (Normal: 80-100 fL)
- White blood cells: 4.2 x 10^9/L (Normal: 4-11 x 10^9/L)
- Platelets: 150 x 10^9/L (Normal: 150-450 x 10^9/L)

Serum vitamin B12 level is pending.

## Encounter 1: source 2020-04-15, practice 2024-05-29, procedure, Gastroenterology

Chief complaint: progressive fatigue and difficulty concentrating over the past 2 months

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Raj Patel, MD
Department: Gastroenterology
Visit Type: procedure

CHIEF COMPLAINT:
Progressive fatigue and difficulty concentrating over the past 2 months

HISTORY OF PRESENT ILLNESS:
The patient with prior B12 deficiency anemia returns with progressive fatigue and difficulty concentrating over the past 2 months.

PAST MEDICAL HISTORY:
Active Problem List:
- Vitamin B12 deficiency anemia (diagnosed 2020-01-15)
- Type 1 diabetes mellitus
- Autoimmune hypothyroidism
- Vitiligo
- Autoimmune gastritis

PAST SURGICAL HISTORY:
- Cesarean section (1988)
- Tonsillectomy (age 8)

MEDICATIONS:
Home Medications:
- Insulin glargine 22 units subcutaneous at bedtime
- Insulin lispro sliding scale with meals
- Levothyroxine 112 mcg PO daily
- Cyanocobalamin 1000 mcg IM monthly

ALLERGIES:
Penicillin (hives)

FAMILY HISTORY:
- Father: Type 1 diabetes mellitus
- Mother: Hashimoto thyroiditis
- Sister: Pernicious anemia
- Maternal grandmother: Rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired high school librarian.

PATHOLOGY:
Upper gastrointestinal endoscopy with biopsy reveals the histologic section shown, with the area labeled X indicating the affected cell type.

## Encounter 2: source 2022-01-15, practice 2026-02-23, outpatient, Neurology

Chief complaint: fatigue, numbness in her feet, and occasional imbalance when walking with forgetfulness and mood changes over the past few months

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Maria Gonzalez, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue, numbness in her feet, and occasional imbalance when walking with forgetfulness and mood changes over the past few months

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of her autoimmune gastritis and prior vitamin B12 deficiency. She also reports some forgetfulness and mood changes over the past few months.

PAST MEDICAL HISTORY:
Active Problem List:
- Vitamin B12 deficiency anemia (diagnosed 2020-01-15)
- Autoimmune gastritis with parietal cell loss (diagnosed 2020-04-15)
- Type 1 diabetes mellitus
- Autoimmune hypothyroidism
- Vitiligo
- Autoimmune gastritis

Her past medical history is significant for vitiligo.

PAST SURGICAL HISTORY:
- Cesarean section (1988)
- Tonsillectomy (age 8)

MEDICATIONS:
Home Medications:
- Insulin glargine 22 units subcutaneous at bedtime
- Insulin lispro sliding scale with meals
- Levothyroxine 112 mcg PO daily
- Cyanocobalamin 1000 mcg IM monthly

ALLERGIES:
Penicillin (hives)

FAMILY HISTORY:
- Father: Type 1 diabetes mellitus
- Mother: Hashimoto thyroiditis
- Sister: Pernicious anemia
- Maternal grandmother: Rheumatoid arthritis

SOCIAL HISTORY:
She does not drink alcohol.

PHYSICAL EXAMINATION:
On physical exam, she has pallor, decreased vibratory sensation in the lower limbs, and a smooth red tongue. Neurologic examination shows a positive Romberg sign.

LABORATORY DATA:
Labs show hemoglobin of 9.1 g/dL, MCV of 119 µm³, and normal leukocyte and platelet counts. Peripheral smear is shown below.
