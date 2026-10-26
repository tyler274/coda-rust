# synthetic_hospital patient 2787

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 41 at first encounter, sex F, Non-Hispanic White, Private commercial insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2021-01-15 | 2025-03-31 (Mon) | outpatient | Cardiology |
| 1 | 2022-01-15 | 2026-03-31 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Private commercial insurance",
 "occupation": "International business development consultant",
 "alcohol_use": "None",
 "family_history": [
  "Mother: Migraine disorder",
  "Maternal uncle: Congenital heart defect (atrial septal defect)",
  "Father: Hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 0.5 packs per day",
 "home_medications": [
  {
   "dose": "1 tablet by mouth daily",
   "name": "Combined oral contraceptive pills (ethinyl estradiol/norgestimate)"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Migraine with aura",
  "Tobacco use disorder"
 ],
 "age_at_first_encounter": 41
}
```

Primary diagnoses: ["Secundum atrial septal defect"]
Comorbidities: []

## Encounter 0: source 2021-01-15, practice 2025-03-31, outpatient, Cardiology

Chief complaint: Progressive shortness of breath with moderate physical activity over several months

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Sarah Chen, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive shortness of breath with moderate physical activity over several months

HISTORY OF PRESENT ILLNESS:
A 42-year-old woman presents to the clinic with progressive shortness of breath, especially during moderate physical activity. She reports no chest pain, orthopnea, or peripheral edema.

PAST MEDICAL HISTORY:
Active Problem List:
- Migraine with aura
- Tobacco use disorder

MEDICATIONS:
She is.

Home Medications:
- Combined oral contraceptive pills (ethinyl estradiol/norgestimate) 1 tablet by mouth daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Migraine disorder
- Maternal uncle: Congenital heart defect (atrial septal defect)
- Father: Hypertension

SOCIAL HISTORY:
Smoking: Current smoker, 0.5 packs per day. Alcohol: None. Occupation: International business development consultant.

PHYSICAL EXAMINATION:
On physical examination, there is a prominent cardiac impulse palpable along the left sternal border. Cardiac auscultation reveals a fixed S2 splitting that does not vary with respiration and a low-pitched diastolic murmur heard best at the lower left sternal border. There are no signs of pulmonary edema, jugular venous distension, or peripheral cyanosis.

IMAGING:
A chest x-ray reveals increased pulmonary vascular markings.

## Encounter 1: source 2022-01-15, practice 2026-03-31, ed, Emergency Medicine

Chief complaint: Sudden-onset right arm weakness and slurred speech

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Marcus Williams, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden-onset right arm weakness and slurred speech

HISTORY OF PRESENT ILLNESS:
Following 2021 ASD diagnosis, she returned yesterday from 10-hour international flight.

PAST MEDICAL HISTORY:
Active Problem List:
- Secundum atrial septal defect (diagnosed 2021-01-15)
- Migraine with aura
- Tobacco use disorder

She has a history of migraines with aura.

MEDICATIONS:
She takes combined oral contraceptives.

Home Medications:
- Combined oral contraceptive pills (ethinyl estradiol/norgestimate) 1 tablet by mouth daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Migraine disorder
- Maternal uncle: Congenital heart defect (atrial septal defect)
- Father: Hypertension

SOCIAL HISTORY:
She smokes half a pack of cigarettes daily but denies alcohol intake.

PHYSICAL EXAMINATION:
On exam, she is alert but has expressive aphasia and 2/5 strength in the right upper limb. Her left calf is swollen, tender, and warm compared to her right calf. Left calf pain is elicited on passive dorsiflexion of the foot.

IMAGING:
A non-contrast CT scan of the head shows no acute changes. MRI shows an acute infarct in the left middle cerebral artery.

OTHER STUDIES:
Cardiac monitoring shows normal sinus rhythm. A transesophageal echocardiogram is performed to evaluate for a possible embolic source.
