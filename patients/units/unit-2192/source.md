# synthetic_hospital patient 2192

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 68 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-11-14 (Thu) | inpatient | Internal Medicine |
| 1 | 2020-07-15 | 2025-05-02 (Fri) | ed | Emergency Medicine |
| 2 | 2021-07-15 | 2026-05-01 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare",
 "occupation": "Retired postal service supervisor",
 "alcohol_use": "None",
 "family_history": [
  "Father: Sudden cardiac death at age 71",
  "Mother: Hypertension and type 2 diabetes mellitus, alive at age 91",
  "Older brother: Chronic obstructive pulmonary disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 48 pack-year history, quit 18 months ago",
 "home_medications": [
  {
   "dose": "20 mg PO once daily",
   "name": "Lisinopril"
  },
  {
   "dose": "250/50 mcg inhaled, one puff twice daily",
   "name": "Fluticasone propionate/salmeterol"
  },
  {
   "dose": "90 mcg inhaled, 2 puffs every 4 hours as needed for dyspnea",
   "name": "Albuterol sulfate"
  },
  {
   "dose": "20 mg PO once daily",
   "name": "Omeprazole"
  }
 ],
 "surgical_history": [
  "Open reduction and internal fixation of right femoral neck fracture (age 62)",
  "Tonsillectomy (age 8)"
 ],
 "chronic_conditions": [
  "Chronic obstructive pulmonary disease (COPD), GOLD Group C",
  "Hypertension",
  "Gastroesophageal reflux disease (GERD)"
 ],
 "age_at_first_encounter": 68
}
```

Primary diagnoses: ["Cardiac arrest", "Prerenal acute kidney injury secondary to hypovolemia"]
Comorbidities: ["Chronic obstructive pulmonary disease", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2024-11-14, inpatient, Internal Medicine

Chief complaint: Treatment of gram-negative bacteremia with new onset acute kidney injury

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Patel, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Treatment of gram-negative bacteremia with new onset acute kidney injury

HISTORY OF PRESENT ILLNESS:
His symptoms began 6 days ago with high-grade fevers, chills, and malaise, and blood cultures grew a motile, non–lactose-fermenting, gram-negative rod. He was started on an intravenous antibiotic regimen tailored to the organism's susceptibility profile. On day 5 of therapy, the patient reports decreased urine output and mild swelling of his ankles.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (COPD), GOLD Group C
- Hypertension
- Gastroesophageal reflux disease (GERD)

Medical history includes hypertension and chronic obstructive pulmonary disease.

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of right femoral neck fracture (age 62)
- Tonsillectomy (age 8)

MEDICATIONS:
He takes he takes lisinopril and an inhaled corticosteroid/long-acting beta agonist. He does not use nonsteroidal anti-inflammatory drugs.

Home Medications:
- Fluticasone propionate/salmeterol 250/50 mcg inhaled, one puff twice daily
- Albuterol sulfate 90 mcg inhaled, 2 puffs every 4 hours as needed for dyspnea
- Omeprazole 20 mg PO once daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Sudden cardiac death at age 71
- Mother: Hypertension and type 2 diabetes mellitus, alive at age 91
- Older brother: Chronic obstructive pulmonary disease

SOCIAL HISTORY:
Smoking: Former smoker, 48 pack-year history, quit 18 months ago. Alcohol: None. Occupation: Retired postal service supervisor.

REVIEW OF SYSTEMS:
He has no dysuria, hematuria, flank pain, rash, or joint pain.

VITAL SIGNS:
Temperature is 37.1°C (98.8°F), blood pressure is 152/88 mm Hg, pulse is 82/min, and respiratory rate is 16/min

PHYSICAL EXAMINATION:
There is mild bilateral ankle edema. Lungs are clear to auscultation, and the cardiovascular exam is normal. Abdominal exam is benign, with no costovertebral angle tenderness.

LABORATORY DATA:
Creatinine 2.3 mg/dL, blood urea nitrogen 38 mg/dL, and urinalysis with mild proteinuria, granular casts, and no eosinophils

## Encounter 1: source 2020-07-15, practice 2025-05-02, ed, Emergency Medicine

Chief complaint: Found unconscious at home after 24 hours of severe abdominal pain and vomiting

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. James Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Found unconscious at home after 24 hours of severe abdominal pain and vomiting

HISTORY OF PRESENT ILLNESS:
This 68-year-old with COPD presents following his January hospitalization. His wife reports that he has been complaining of severe abdominal pain and vomiting for the past 24 hours.

PAST MEDICAL HISTORY:
Active Problem List:
- Prerenal acute kidney injury secondary to hypovolemia (diagnosed 2020-01-15)
- Chronic obstructive pulmonary disease (COPD), GOLD Group C
- Hypertension
- Gastroesophageal reflux disease (GERD)

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of right femoral neck fracture (age 62)
- Tonsillectomy (age 8)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO once daily
- Fluticasone propionate/salmeterol 250/50 mcg inhaled, one puff twice daily
- Albuterol sulfate 90 mcg inhaled, 2 puffs every 4 hours as needed for dyspnea
- Omeprazole 20 mg PO once daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Sudden cardiac death at age 71
- Mother: Hypertension and type 2 diabetes mellitus, alive at age 91
- Older brother: Chronic obstructive pulmonary disease

SOCIAL HISTORY:
Smoking: Former smoker, 48 pack-year history, quit 18 months ago. Alcohol: None. Occupation: Retired postal service supervisor.

VITAL SIGNS:
Blood pressure of 90/60 mmHg, heart rate of 118 bpm, and respiratory rate of 30 breaths per minute

PHYSICAL EXAMINATION:
On examination, the patient is lethargic. His skin is cool and clammy. Lung auscultation reveals diffuse wheezing. Abdominal examination shows diffuse tenderness without rebound or guarding.

PLAN:
Intravenous fluids are initiated, and the patient is intubated due to worsening respiratory distress and altered mental status. Shortly after initiating positive-pressure mechanical ventilation, the patient becomes pulseless.

## Encounter 2: source 2021-07-15, practice 2026-05-01, ed, Emergency Medicine

Chief complaint: Found unresponsive at home with unsuccessful resuscitation

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Maria Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Found unresponsive at home with unsuccessful resuscitation

HISTORY OF PRESENT ILLNESS:
This 69-year-old male with COPD, hypertension, GERD, and prior arrest returns. Cardiopulmonary resuscitation (CPR) is initiated, and advanced cardiac life support (ACLS) protocols are followed, but after 30 minutes of resuscitative efforts, the patient is pronounced dead.

PAST MEDICAL HISTORY:
Active Problem List:
- Prerenal acute kidney injury secondary to hypovolemia (diagnosed 2020-01-15)
- Cardiac arrest (diagnosed 2020-07-15)
- Chronic obstructive pulmonary disease (COPD), GOLD Group C
- Hypertension
- Gastroesophageal reflux disease (GERD)

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of right femoral neck fracture (age 62)
- Tonsillectomy (age 8)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO once daily
- Fluticasone propionate/salmeterol 250/50 mcg inhaled, one puff twice daily
- Albuterol sulfate 90 mcg inhaled, 2 puffs every 4 hours as needed for dyspnea
- Omeprazole 20 mg PO once daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Sudden cardiac death at age 71
- Mother: Hypertension and type 2 diabetes mellitus, alive at age 91
- Older brother: Chronic obstructive pulmonary disease

SOCIAL HISTORY:
Smoking: Former smoker, 48 pack-year history, quit 18 months ago. Alcohol: None. Occupation: Retired postal service supervisor.

PHYSICAL EXAMINATION:
On arrival, he is pulseless and apneic.

PLAN:
A junior resident who was part of the resuscitation team approaches you, the attending physician, with a request to perform endotracheal intubation on the deceased patient to improve their skills. The patient's family is in the waiting room, grieving.
