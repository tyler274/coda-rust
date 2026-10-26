# synthetic_hospital patient 1822

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 30 at first encounter, sex F, Non-Hispanic White, Employer-sponsored commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-02-05 (Wed) | ed | Emergency Medicine |
| 1 | 2020-07-15 | 2025-08-05 (Tue) | ed | Emergency Medicine |
| 2 | 2021-01-15 | 2026-02-18 (Wed) | inpatient | Internal Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Employer-sponsored commercial PPO",
 "occupation": "Global health aid worker",
 "alcohol_use": "Social (1-2 drinks per month)",
 "family_history": [
  "Mother: Major depressive disorder",
  "Father: Hypertension",
  "Sister: Hypothyroidism"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "20 mg PO daily",
   "name": "Fluoxetine"
  },
  {
   "dose": "1 tablet PO daily",
   "name": "Levonorgestrel/ethinyl estradiol"
  }
 ],
 "surgical_history": [
  "Tonsillectomy (age 8)"
 ],
 "chronic_conditions": [
  "Major depressive disorder (recently diagnosed)"
 ],
 "age_at_first_encounter": 30
}
```

Primary diagnoses: ["Serotonin syndrome", "Malaria", "Enterotoxigenic Escherichia coli infection"]
Comorbidities: ["Delayed management of life-threatening respiratory failure"]

## Encounter 0: source 2020-01-15, practice 2025-02-05, ed, Emergency Medicine

Chief complaint: Agitation, fever, and diarrhea

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Agitation, fever, and diarrhea

HISTORY OF PRESENT ILLNESS:
She was recently started on fluoxetine for depression and began taking an over-the-counter supplement recommended by a friend. Her husband believes it was St. John’s wort.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder (recently diagnosed)

PAST SURGICAL HISTORY:
- Tonsillectomy (age 8)

MEDICATIONS:
Home Medications:
- Fluoxetine 20 mg PO daily
- Levonorgestrel/ethinyl estradiol 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Major depressive disorder
- Father: Hypertension
- Sister: Hypothyroidism

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Global health aid worker.

VITAL SIGNS:
Temperature of 105.2°F (40.7°C), blood pressure of 160/90 mm Hg, and heart rate of 130/min

PHYSICAL EXAMINATION:
On exam, she is diaphoretic and flushed. She has hyperreflexia, tremor, and myoclonus in her lower limbs. There are no murmurs or signs of endocarditis.

## Encounter 1: source 2020-07-15, practice 2025-08-05, ed, Emergency Medicine

Chief complaint: Watery diarrhea and mild abdominal cramping for 2 days

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Watery diarrhea and mild abdominal cramping for 2 days

HISTORY OF PRESENT ILLNESS:
This 30-year-old female with major depressive disorder on fluoxetine, who presented six months prior with agitation, fever, and diarrhea, reports five to six loose stools per day without blood or mucus. She returned from a 2-week vacation in a resource-limited country 3 days ago, where she recalls eating street food. She denies any sick contacts.

PAST MEDICAL HISTORY:
Active Problem List:
- Serotonin syndrome (diagnosed 2020-01-15)
- Major depressive disorder (recently diagnosed)

PAST SURGICAL HISTORY:
- Tonsillectomy (age 8)

MEDICATIONS:
Home Medications:
- Fluoxetine 20 mg PO daily
- Levonorgestrel/ethinyl estradiol 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Major depressive disorder
- Father: Hypertension
- Sister: Hypothyroidism

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Global health aid worker.

VITAL SIGNS:
On examination, her vital signs are stable. She is afebrile

PHYSICAL EXAMINATION:
Abdominal examination reveals mild diffuse tenderness without rebound or guarding. maintains normal hydration status.

LABORATORY DATA:
Her stool is negative for occult blood.

## Encounter 2: source 2021-01-15, practice 2026-02-18, inpatient, Internal Medicine

Chief complaint: Intermittent fever, chills, and profuse sweating for 10 days

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Amara Okafor, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Intermittent fever, chills, and profuse sweating for 10 days

HISTORY OF PRESENT ILLNESS:
The patient has a history of major depressive disorder and prior ED visits in 2020 for serotonin syndrome and E coli infection. She recently returned from a 2-week trip to sub-Saharan Africa where she volunteered in rural communities. She reports she did not take any prophylactic medications prior to her travel. She complains of fatigue, generalized weakness, and mild abdominal discomfort.

PAST MEDICAL HISTORY:
Active Problem List:
- Serotonin syndrome (diagnosed 2020-01-15)
- Enterotoxigenic Escherichia coli infection (diagnosed 2020-07-15)
- Major depressive disorder (recently diagnosed)

PAST SURGICAL HISTORY:
- Tonsillectomy (age 8)

MEDICATIONS:
Home Medications:
- Fluoxetine 20 mg PO daily
- Levonorgestrel/ethinyl estradiol 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Major depressive disorder
- Father: Hypertension
- Sister: Hypothyroidism

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Global health aid worker.

VITAL SIGNS:
Her temperature is 38.9°C (102°F), blood pressure is 110/70 mmHg, pulse is 95/min, and respiratory rate is 18/min

PHYSICAL EXAMINATION:
She appears pale and mildly jaundiced. No lymphadenopathy is noted. Abdominal examination reveals mild splenomegaly.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 9.8 g/dL (Normal: 12-16 g/dL)
- Hematocrit: 29% (Normal: 36-46%)
- Platelets: 85,000/uL (Normal: 150,000-400,000/uL)
- White blood cells (WBC): 4,500/uL (Normal: 4,500-11,000/uL)
- Peripheral blood smear: Presence of ring forms within red blood cells
