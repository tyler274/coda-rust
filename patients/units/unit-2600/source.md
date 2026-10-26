# synthetic_hospital patient 2600

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 24 at first encounter, sex M, Non-Hispanic White, TRICARE Prime.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-10-19 (Sat) | ed | Emergency Medicine |
| 1 | 2021-07-15 | 2026-04-11 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Blood products containing IgA (anti-IgA antibodies) - anaphylaxis",
  "Plasma transfusion - anaphylaxis"
 ],
 "insurance": "TRICARE Prime",
 "occupation": "Active duty military service member (Army infantry)",
 "alcohol_use": "Occasional (1-2 drinks monthly)",
 "family_history": [
  "Mother with autoimmune thyroiditis (Hashimoto's)",
  "Maternal uncle with common variable immunodeficiency",
  "Father with type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "250mg orally three times weekly (prophylaxis against recurrent infections)",
   "name": "Azithromycin"
  },
  {
   "dose": "2 sprays (100mcg) each nostril once daily",
   "name": "Fluticasone propionate nasal spray"
  },
  {
   "dose": "90mcg (2 puffs) every 4-6 hours as needed for wheezing",
   "name": "Albuterol inhaler"
  }
 ],
 "surgical_history": [
  "Tonsillectomy and adenoidectomy age 12",
  "Appendectomy age 19"
 ],
 "chronic_conditions": [
  "Selective IgA deficiency",
  "Recurrent respiratory infections"
 ],
 "age_at_first_encounter": 24
}
```

Primary diagnoses: ["Acute liver failure", "Selective IgA deficiency"]
Comorbidities: ["Traumatic coagulopathy"]

## Encounter 0: source 2020-01-15, practice 2024-10-19, ed, Emergency Medicine

Chief complaint: Diffuse skin redness and chills 45 minutes after receiving a plasma transfusion for coagulopathy. Denies chest pain, dyspnea, or swelling of the tongue or lips.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Diffuse skin redness and chills 45 minutes after receiving a plasma transfusion for coagulopathy. Denies chest pain, dyspnea, or swelling of the tongue or lips.

HISTORY OF PRESENT ILLNESS:
He is brought to the emergency department 45 minutes after developing diffuse skin redness and chills. He reports that the symptoms began shortly after receiving a plasma transfusion for coagulopathy. He denies chest pain, dyspnea, or swelling of the tongue or lips. He has never had similar symptoms with food, medications, or insect stings.

PAST MEDICAL HISTORY:
Active Problem List:
- Selective IgA deficiency
- Recurrent respiratory infections

His past medical history is notable for recurrent episodes of sinusitis and pneumonia requiring multiple courses of antibiotics.

PAST SURGICAL HISTORY:
- Tonsillectomy and adenoidectomy age 12
- Appendectomy age 19

MEDICATIONS:
Home Medications:
- Azithromycin 250mg orally three times weekly (prophylaxis against recurrent infections)
- Fluticasone propionate nasal spray 2 sprays (100mcg) each nostril once daily
- Albuterol inhaler 90mcg (2 puffs) every 4-6 hours as needed for wheezing

ALLERGIES:
Blood products containing IgA (anti-IgA antibodies) - anaphylaxis, Plasma transfusion - anaphylaxis

FAMILY HISTORY:
- Mother with autoimmune thyroiditis (Hashimoto's)
- Maternal uncle with common variable immunodeficiency
- Father with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks monthly). Occupation: Active duty military service member (Army infantry).

VITAL SIGNS:
His temperature is 38.4°C (101.1°F), blood pressure is 108/64 mm Hg, pulse is 98/min, and respirations are 20/min.

PHYSICAL EXAMINATION:
Physical examination shows generalized urticaria over the trunk and arms without wheezing or stridor.

## Encounter 1: source 2021-07-15, practice 2026-04-11, ed, Emergency Medicine

Chief complaint: Collapse during a military training exercise conducted on a hot summer day.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Collapse during a military training exercise conducted on a hot summer day.

HISTORY OF PRESENT ILLNESS:
The patient is a 25-year-old male with selective IgA deficiency and recurrent respiratory infections initially diagnosed following a transfusion reaction in January 2020. He is brought to the emergency department after collapsing during a military training exercise on a hot summer day.

PAST MEDICAL HISTORY:
Active Problem List:
- Selective IgA deficiency (diagnosed 2020-01-15)
- Recurrent respiratory infections

PAST SURGICAL HISTORY:
- Tonsillectomy and adenoidectomy age 12
- Appendectomy age 19

MEDICATIONS:
Home Medications:
- Azithromycin 250mg orally three times weekly (prophylaxis against recurrent infections)
- Fluticasone propionate nasal spray 2 sprays (100mcg) each nostril once daily
- Albuterol inhaler 90mcg (2 puffs) every 4-6 hours as needed for wheezing

ALLERGIES:
Blood products containing IgA (anti-IgA antibodies) - anaphylaxis, Plasma transfusion - anaphylaxis

FAMILY HISTORY:
- Mother with autoimmune thyroiditis (Hashimoto's)
- Maternal uncle with common variable immunodeficiency
- Father with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks monthly). Occupation: Active duty military service member (Army infantry).

VITAL SIGNS:
Hypotensive, and has a core temperature of 40.5°C (104.9°F).

PHYSICAL EXAMINATION:
Confused

LABORATORY DATA:
Creatinine is 2.3 mg/dL. AST and ALT are elevated. INR is increased.

PLAN:
He is admitted to the ICU.
