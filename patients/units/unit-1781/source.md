# synthetic_hospital patient 1781

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 24 at first encounter, sex F, Non-Hispanic White, Private insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2018-03-26 (Mon) | outpatient | Psychiatry |
| 1 | 2024-01-15 | 2022-03-30 (Wed) | outpatient | Endocrinology |
| 2 | 2024-07-15 | 2022-10-10 (Mon) | outpatient | Endocrinology |
| 3 | 2028-01-15 | 2026-04-01 (Wed) | ed | Emergency Medicine |
| 4 | 2028-04-15 | 2026-06-25 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private insurance",
 "occupation": "Graduate student",
 "alcohol_use": "None",
 "family_history": [
  "Mother with bipolar disorder",
  "Paternal grandfather with alcohol use disorder"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "300 mg twice daily",
   "name": "Lithium carbonate"
  },
  {
   "dose": "50 mg daily",
   "name": "Sertraline"
  }
 ],
 "surgical_history": [
  "Tonsillectomy and adenoidectomy, age 12"
 ],
 "chronic_conditions": [
  "Bipolar I Disorder"
 ],
 "age_at_first_encounter": 24
}
```

Primary diagnoses: ["Nephrogenic Diabetes Insipidus", "Central diabetes insipidus (inadequately controlled)", "Catatonia", "Toxic liver disease"]
Comorbidities: ["Hypernatremia or free water deficit (maintenance fluid indication)", "Bipolar Disorder"]

## Encounter 0: source 2020-01-15, practice 2018-03-26, outpatient, Psychiatry

Chief complaint: Follow-up for bipolar disorder with tremor and increased urination

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up for bipolar disorder with tremor and increased urination

HISTORY OF PRESENT ILLNESS:
She reports that her mood has been relatively stable, but she has been feeling mildly shaky over the past few weeks and notes that she is urinating more than usual, including waking up at night to void. She denies drinking excessive amounts of water but has noticed increasing thirst.

PAST MEDICAL HISTORY:
Active Problem List:
- Bipolar I Disorder

PAST SURGICAL HISTORY:
- Tonsillectomy and adenoidectomy, age 12

MEDICATIONS:
Her medications include lithium and sertraline, and she has been on lithium therapy for the past six months.

Home Medications:
- Lithium carbonate 300 mg twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with bipolar disorder
- Paternal grandfather with alcohol use disorder

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Graduate student.

VITAL SIGNS:
Vital signs are normal

PHYSICAL EXAMINATION:
On exam, she has a fine postural tremor in her hands but no other neurologic deficits. Her mucous membranes appear dry.

LABORATORY DATA:
A basic metabolic panel reveals a serum sodium level of 148 mEq/L. A urine collection shows low urine osmolality and specific gravity.

## Encounter 1: source 2024-01-15, practice 2022-03-30, outpatient, Endocrinology

Chief complaint: Excessive urination and persistent thirst for 6 months

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Michael Okonkwo, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Excessive urination and persistent thirst for 6 months

HISTORY OF PRESENT ILLNESS:
The patient is a 28-year-old woman with Bipolar I Disorder and a history of similar presentations dating to 2020 who returns to the clinic with a 6-month history of excessive urination and persistent thirst. She reports that she has been drinking more water than usual but denies any relief in her symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Nephrogenic Diabetes Insipidus (diagnosed 2020-01-15)
- Bipolar I Disorder

PAST SURGICAL HISTORY:
- Tonsillectomy and adenoidectomy, age 12

MEDICATIONS:
Home Medications:
- Lithium carbonate 300 mg twice daily
- Sertraline 50 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with bipolar disorder
- Paternal grandfather with alcohol use disorder

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Graduate student.

PHYSICAL EXAMINATION:
On physical examination, her vital signs are within normal limits, and no abnormalities are noted.

LABORATORY DATA:
Laboratory tests reveal:

- Serum sodium: 145 mEq/L (normal 135-145 mEq/L)
- Serum osmolality: 300 mOsm/kg (normal 275-295 mOsm/kg)
- Urine osmolality: 150 mOsm/kg (normal 500-800 mOsm/kg after water deprivation)
- Urine specific gravity: 1.005 (normal 1.010-1.025 after water deprivation)

OTHER STUDIES:
The patient undergoes a water deprivation test, and her urine osmolality remains unchanged.

## Encounter 2: source 2024-07-15, practice 2022-10-10, outpatient, Endocrinology

Chief complaint: Persistent excessive thirst and hourly urination despite treatment

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Michael Okonkwo, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent excessive thirst and hourly urination despite treatment

HISTORY OF PRESENT ILLNESS:
This 28-year-old woman with Bipolar I Disorder returns to the Endocrinology clinic for follow-up of excessive urination and persistent thirst previously evaluated in January 2024 and initially reported during psychiatric care in 2020. She reports these symptoms have persisted for 6 months, noting she must urinate every hour, even at night, and cannot quench her thirst despite drinking more than 4 liters of water daily. She denies any recent head trauma or surgery.

PAST MEDICAL HISTORY:
Active Problem List:
- Nephrogenic Diabetes Insipidus (diagnosed 2020-01-15)
- Bipolar I Disorder

PAST SURGICAL HISTORY:
- Tonsillectomy and adenoidectomy, age 12

MEDICATIONS:
She is.

Home Medications:
- Lithium carbonate 300 mg twice daily
- Sertraline 50 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with bipolar disorder
- Paternal grandfather with alcohol use disorder

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Graduate student.

PHYSICAL EXAMINATION:
Physical examination is normal, with no signs of dehydration.

LABORATORY DATA:
Laboratory tests reveal:

- Serum sodium: 147 mEq/L (normal 135-145 mEq/L)
- Serum osmolality: 300 mOsm/kg (normal 275-295 mOsm/kg)
- Urine osmolality: 150 mOsm/kg (normal 500-800 mOsm/kg after overnight fast)

OTHER STUDIES:
A water deprivation test fails to increase her urine osmolality. Subsequent administration of desmopressin acetate tablets results in an increase in urine osmolality to 300 mOsm/kg.

## Encounter 3: source 2028-01-15, practice 2026-04-01, ed, Emergency Medicine

Chief complaint: 3-day history of mutism, refusing food, and unresponsiveness

ENCOUNTER NOTE
Date: 2028-01-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
3-day history of mutism, refusing food, and unresponsiveness

HISTORY OF PRESENT ILLNESS:
Patient with Bipolar I Disorder and diabetes insipidus presents. Her family reports that she was recently in a "very high" mood but abruptly became withdrawn and immobile.

PAST MEDICAL HISTORY:
Active Problem List:
- Nephrogenic Diabetes Insipidus (diagnosed 2020-01-15)
- Central diabetes insipidus (inadequately controlled) (diagnosed 2024-07-15)
- Bipolar I Disorder

Her family reports that she has a history of bipolar disorder.

PAST SURGICAL HISTORY:
- Tonsillectomy and adenoidectomy, age 12

MEDICATIONS:
Home Medications:
- Lithium carbonate 300 mg twice daily
- Sertraline 50 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with bipolar disorder
- Paternal grandfather with alcohol use disorder

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Graduate student.

VITAL SIGNS:
Her vital signs are stable

PHYSICAL EXAMINATION:
On examination, the patient is lying in bed with a rigid posture and does not respond to verbal commands. When the physician moves the patient's arm, the patient resists and maintains the arm in the same position.

LABORATORY DATA:
- Sodium: 140 mEq/L
- Potassium: 4.0 mEq/L
- Chloride: 100 mEq/L
- Bicarbonate: 24 mEq/L
- Blood urea nitrogen: 18 mg/dL
- Creatinine: 0.9 mg/dL
- Glucose: 98 mg/dL
- Calcium: 9.2 mg/dL
- Complete blood count: within normal limits

## Encounter 4: source 2028-04-15, practice 2026-06-25, ed, Emergency Medicine

Chief complaint: Progressive lethargy, nausea, and vomiting over 3 days

ENCOUNTER NOTE
Date: 2028-04-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive lethargy, nausea, and vomiting over 3 days

PAST MEDICAL HISTORY:
Active Problem List:
- Nephrogenic Diabetes Insipidus (diagnosed 2020-01-15)
- Central diabetes insipidus (inadequately controlled) (diagnosed 2024-07-15)
- Catatonia (diagnosed 2028-01-15)
- Bipolar I Disorder

PAST SURGICAL HISTORY:
- Tonsillectomy and adenoidectomy, age 12

MEDICATIONS:
She was started on valproate therapy 4 months ago and has been compliant with her medication regimen.

Home Medications:
- Lithium carbonate 300 mg twice daily
- Sertraline 50 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with bipolar disorder
- Paternal grandfather with alcohol use disorder

SOCIAL HISTORY:
She denies alcohol use, recent travel, or any sick contacts.

VITAL SIGNS:
Her temperature is 37.2°C (99.0°F), blood pressure is 130/85 mmHg, pulse is 78/min, and respiratory rate is 16/min.

PHYSICAL EXAMINATION:
Physical examination is notable for mild right upper quadrant tenderness without rebound or guarding. There is no jaundice or scleral icterus.

LABORATORY DATA:
Laboratory studies are as follows:

- Aspartate aminotransferase (AST): 250 U/L (Normal: 10-40 U/L)
- Alanine aminotransferase (ALT): 300 U/L (Normal: 7-56 U/L)
- Alkaline phosphatase (ALP): 100 U/L (Normal: 44-147 U/L)
- Total bilirubin: 1.2 mg/dL (Normal: 0.3-1.2 mg/dL)
- International normalized ratio (INR): 1.1 (Normal: 0.8-1.2)
