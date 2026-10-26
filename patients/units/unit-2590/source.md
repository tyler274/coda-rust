# synthetic_hospital patient 2590

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 22 at first encounter, sex M, African American, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-11-04 (Mon) | procedure | General Surgery |
| 1 | 2020-07-15 | 2025-05-14 (Wed) | ed | Emergency Medicine |
| 2 | 2021-03-15 | 2026-01-12 (Mon) | inpatient | Hematology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "College student (part-time retail associate)",
 "alcohol_use": "Occasional (1-2 drinks per month)",
 "family_history": [
  "Father with sickle cell trait",
  "Mother with sickle cell trait",
  "Maternal uncle with history of anesthesia complications (suspected malignant hyperthermia)",
  "Paternal grandmother with asthma"
 ],
 "race_ethnicity": "African American",
 "smoking_status": "Never",
 "home_medications": [
  {
   "dose": "1000 mg daily (taken intermittently per hematology recommendations)",
   "name": "Hydroxyurea"
  },
  {
   "dose": "90 mcg/spray inhaler, 2 puffs every 4-6 hours as needed for wheezing",
   "name": "Albuterol"
  },
  {
   "dose": "1 mg daily",
   "name": "Folic acid"
  }
 ],
 "surgical_history": [
  "Cholecystectomy (complicated by intraoperative malignant hyperthermia crisis)"
 ],
 "chronic_conditions": [
  "Sickle cell anemia (HbSS genotype)",
  "Asthma",
  "Malignant hyperthermia susceptibility"
 ],
 "age_at_first_encounter": 22
}
```

Primary diagnoses: ["Renal papillary necrosis", "Malignant Hyperthermia"]
Comorbidities: ["Family History of Malignant Hyperthermia"]

## Encounter 0: source 2020-01-15, practice 2024-11-04, procedure, General Surgery

Chief complaint: Intraoperative tachypnea, tachycardia, and jaw stiffness (trismus) ten minutes after anesthesia induction for elective cholecystectomy

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: General Surgery
Visit Type: procedure

CHIEF COMPLAINT:
Intraoperative tachypnea, tachycardia, and jaw stiffness (trismus) ten minutes after anesthesia induction for elective cholecystectomy

HISTORY OF PRESENT ILLNESS:
He is undergoing a cholecystectomy.

PAST MEDICAL HISTORY:
Active Problem List:
- Sickle cell anemia (HbSS genotype)
- Asthma
- Malignant hyperthermia susceptibility

PAST SURGICAL HISTORY:
- Cholecystectomy (complicated by intraoperative malignant hyperthermia crisis)

MEDICATIONS:
He takes his only medication is an albuterol inhaler as needed.

Home Medications:
- Hydroxyurea 1000 mg daily (taken intermittently per hematology recommendations)
- Folic acid 1 mg daily

ALLERGIES:
He has no known drug allergies

FAMILY HISTORY:
The patient's brother had a similar reaction during a surgery years ago but the details are vague.

SOCIAL HISTORY:
Smoking: Never. Alcohol: Occasional (1-2 drinks per month). Occupation: College student (part-time retail associate).

## Encounter 1: source 2020-07-15, practice 2025-05-14, ed, Emergency Medicine

Chief complaint: Left flank pain and gross hematuria for 2 days

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Left flank pain and gross hematuria for 2 days

HISTORY OF PRESENT ILLNESS:
This 22-year-old male with sickle cell anemia and prior malignant hyperthermia presents to the emergency department with a 2-day history of left flank pain and gross hematuria. He denies any recent trauma or engaging in strenuous exercise.

PAST MEDICAL HISTORY:
Active Problem List:
- Malignant Hyperthermia (diagnosed 2020-01-15)
- Sickle cell anemia (HbSS genotype)
- Asthma
- Malignant hyperthermia susceptibility

He reports a history of mild anemia but has not required any blood transfusions.

PAST SURGICAL HISTORY:
- Cholecystectomy (complicated by intraoperative malignant hyperthermia crisis)

MEDICATIONS:
Home Medications:
- Hydroxyurea 1000 mg daily (taken intermittently per hematology recommendations)
- Albuterol 90 mcg/spray inhaler, 2 puffs every 4-6 hours as needed for wheezing
- Folic acid 1 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
The patient mentions that his father had 'blood problems' but is unsure of the details.

SOCIAL HISTORY:
Smoking: Never. Alcohol: Occasional (1-2 drinks per month). Occupation: College student (part-time retail associate).

VITAL SIGNS:
His temperature is 37.2°C (99°F), blood pressure is 130/80 mmHg, pulse is 88/min, and respiratory rate is 16/min.

PHYSICAL EXAMINATION:
Physical examination reveals mild tenderness to palpation over the left costovertebral angle. There is no abdominal guarding or rebound tenderness.

LABORATORY DATA:
Urinalysis shows no evidence of infection but reveals red blood cell casts.
Hemoglobin electrophoresis results are as follows:

- Hemoglobin A: 55%
- Hemoglobin S: 43%
- Hemoglobin A2: 2.5%
- Hemoglobin F: 0.5%

## Encounter 2: source 2021-03-15, practice 2026-01-12, inpatient, Hematology

Chief complaint: Visible hematuria and right flank pain developing on day 3 of hospitalization for acute sickle cell pain crisis

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Priya Patel, MD
Department: Hematology
Visit Type: inpatient

CHIEF COMPLAINT:
Visible hematuria and right flank pain developing on day 3 of hospitalization for acute sickle cell pain crisis

HISTORY OF PRESENT ILLNESS:
The patient is a 23-year-old male with sickle cell anemia (HbSS) and history of renal papillary necrosis who is hospitalized for management of an acute pain crisis. He suddenly develops visible hematuria and right flank pain on day three of admission. He reports no history of recent trauma, kidney stones, or urinary tract infections. His baseline renal function has been stable.

PAST MEDICAL HISTORY:
Active Problem List:
- Malignant Hyperthermia (diagnosed 2020-01-15)
- Renal papillary necrosis (diagnosed 2020-07-15)
- Sickle cell anemia (HbSS genotype)
- Asthma
- Malignant hyperthermia susceptibility

His history includes episodes of acute chest syndrome during prior hospitalizations.

PAST SURGICAL HISTORY:
- Cholecystectomy (complicated by intraoperative malignant hyperthermia crisis)

MEDICATIONS:
He takes hydroxyurea.

Home Medications:
- Albuterol 90 mcg/spray inhaler, 2 puffs every 4-6 hours as needed for wheezing
- Folic acid 1 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with sickle cell trait
- Mother with sickle cell trait
- Maternal uncle with history of anesthesia complications (suspected malignant hyperthermia)
- Paternal grandmother with asthma

SOCIAL HISTORY:
Smoking: Never. Alcohol: Occasional (1-2 drinks per month). Occupation: College student (part-time retail associate).

PHYSICAL EXAMINATION:
Physical examination reveals right costovertebral angle tenderness but no fever or suprapubic tenderness.

LABORATORY DATA:
Urinalysis shows gross hematuria, rare white blood cells, and no casts. Hematocrit at admission was 22% but is now 10%.
