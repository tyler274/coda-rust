# synthetic_hospital patient 1900

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 34 at first encounter, sex F, White, non-Hispanic, Private insurance (Blue Cross Blue Shield).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-07-30 (Wed) | outpatient | Internal Medicine |
| 1 | 2020-10-15 | 2026-04-29 (Wed) | outpatient | Endocrinology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private insurance (Blue Cross Blue Shield)",
 "occupation": "Elementary school teacher",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "No significant family history"
 ],
 "race_ethnicity": "White, non-Hispanic",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [],
 "age_at_first_encounter": 34
}
```

Primary diagnoses: ["Upper airway cough syndrome (Postnasal drip syndrome)", "Subacute thyroiditis"]
Comorbidities: ["Upper respiratory tract infection"]

## Encounter 0: source 2020-01-15, practice 2025-07-30, outpatient, Internal Medicine

Chief complaint: Persistent cough for 3 weeks following an upper respiratory tract infection, worse at night with sensation of mucus dripping down the back of the throat

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent cough for 3 weeks following an upper respiratory tract infection, worse at night with sensation of mucus dripping down the back of the throat

HISTORY OF PRESENT ILLNESS:
She reports the cough is worse at night and is associated with a sensation of mucus dripping down the back of her throat. She has tried using over-the-counter cough syrups with no relief.

PAST MEDICAL HISTORY:
She denies any history of pulmonary disease. Her medical history is unremarkable,

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
She takes no medications.

ALLERGIES:
NKDA

FAMILY HISTORY:
- No significant family history

SOCIAL HISTORY:
She denies smoking

VITAL SIGNS:
On physical examination, her vital signs are within normal limits.

PHYSICAL EXAMINATION:
Lung auscultation reveals no wheezes, rales, or rhonchi. The rest of the examination is unremarkable.

## Encounter 1: source 2020-10-15, practice 2026-04-29, outpatient, Endocrinology

Chief complaint: Neck pain and feeling of fullness in the throat for 2 weeks with low-grade fever, malaise, palpitations, and intermittent sweating

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Amara Okafor, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Neck pain and feeling of fullness in the throat for 2 weeks with low-grade fever, malaise, palpitations, and intermittent sweating

HISTORY OF PRESENT ILLNESS:
She returns following her January visit for cough, reporting a low-grade fever and malaise. She also mentions experiencing palpitations and intermittent sweating.

PAST MEDICAL HISTORY:
Active Problem List:
- Upper airway cough syndrome (Postnasal drip syndrome) (diagnosed 2020-01-15)

Her medical history is significant for an upper respiratory tract infection 3 weeks ago, which resolved without intervention.

PAST SURGICAL HISTORY:
- None

ALLERGIES:
NKDA

FAMILY HISTORY:
- No significant family history

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Elementary school teacher.

VITAL SIGNS:
Her heart rate is 88 beats per minute, and her blood pressure is 132/85 mmHg.

PHYSICAL EXAMINATION:
On examination, she has a tender, enlarged thyroid gland without any nodularity.

LABORATORY DATA:
Laboratory tests reveal the following:

- Thyroid-stimulating hormone (TSH): <0.01 mIU/L (normal range: 0.4-4.0 mIU/L)
- Free thyroxine (FT4): 2.5 ng/dL (normal range: 0.9-1.7 ng/dL)
- Erythrocyte sedimentation rate (ESR): 70 mm/hr (normal range: 0-20 mm/hr)
- C-reactive protein (CRP): 8 mg/L (normal range: <5 mg/L)
