# synthetic_hospital patient 1821

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex F, Non-Hispanic White, Commercial insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-06-26 (Wed) | outpatient | Obstetrics & Gynecology |
| 1 | 2020-07-15 | 2025-01-06 (Mon) | outpatient | Otolaryngology |
| 2 | 2021-07-15 | 2026-01-05 (Mon) | outpatient | Pulmonary Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial insurance",
 "occupation": "Daycare teacher",
 "alcohol_use": "None",
 "family_history": [
  "Hypertension (maternal)",
  "Hypertension (paternal)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 5 pack-years, quit at age 26",
 "home_medications": [
  {
   "dose": "200 mg PO twice daily",
   "name": "Labetalol"
  },
  {
   "dose": "1 tablet PO daily",
   "name": "Prenatal vitamin"
  }
 ],
 "surgical_history": [
  "Bilateral tympanostomy tube placement (childhood)",
  "Vaginal delivery (G1P1, age 29)"
 ],
 "chronic_conditions": [
  "Chronic hypertension"
 ],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Chronic hypertension in pregnancy", "Cholesteatoma of middle ear (attic cholesteatoma)", "Bronchiectasis"]
Comorbidities: ["History of recent pneumonia"]

## Encounter 0: source 2020-01-15, practice 2024-06-26, outpatient, Obstetrics & Gynecology

Chief complaint: Routine prenatal visit at 28 weeks gestation with chronic hypertension management

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Johnson, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine prenatal visit at 28 weeks gestation with chronic hypertension management

HISTORY OF PRESENT ILLNESS:
Gravida 2, para 1, at 28 weeks' gestation

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic hypertension

She has a history of chronic hypertension, well-controlled with labetalol. Her first child was delivered at term without complications.

PAST SURGICAL HISTORY:
- Bilateral tympanostomy tube placement (childhood)
- Vaginal delivery (G1P1, age 29)

MEDICATIONS:
She reports compliance with her medications and prenatal vitamins.

Home Medications:
- Labetalol 200 mg PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Hypertension (maternal)
- Hypertension (paternal)

SOCIAL HISTORY:
She denies any use of tobacco, alcohol, or illicit drugs.

VITAL SIGNS:
The patient's blood pressure is 138/86 mmHg

PHYSICAL EXAMINATION:
Her physical examination is unremarkable.

IMAGING:
An ultrasound reveals an estimated fetal weight below the 10th percentile for gestational age. The amniotic fluid index is normal. Doppler ultrasound of the umbilical artery shows increased resistance with an elevated systolic/diastolic (S/D) ratio.

## Encounter 1: source 2020-07-15, practice 2025-01-06, outpatient, Otolaryngology

Chief complaint: Persistent foul-smelling right ear drainage and progressive hearing loss over 6 months

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Park, MD
Department: Otolaryngology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent foul-smelling right ear drainage and progressive hearing loss over 6 months

HISTORY OF PRESENT ILLNESS:
This 32-year-old female with chronic hypertension, last evaluated in Obstetrics six months ago, presents to Otolaryngology. She reports that the otorrhea is often foul-smelling and that her hearing loss has been gradually worsening, making it difficult to understand conversations, especially in noisy environments. She denies any vertigo or tinnitus.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic hypertension in pregnancy (diagnosed 2020-01-15)
- Chronic hypertension

Her past medical history is significant for recurrent otitis media during childhood, but she has not had any episodes in the past 20 years.

PAST SURGICAL HISTORY:
- Bilateral tympanostomy tube placement (childhood)
- Vaginal delivery (G1P1, age 29)

MEDICATIONS:
Home Medications:
- Labetalol 200 mg PO twice daily
- Prenatal vitamin 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Hypertension (maternal)
- Hypertension (paternal)

SOCIAL HISTORY:
Smoking: Former smoker, 5 pack-years, quit at age 26. Alcohol: None. Occupation: Daycare teacher.

PHYSICAL EXAMINATION:
On physical examination, the right tympanic membrane is intact but appears retracted with a pearly white mass visible in the superior quadrant. The left ear examination is unremarkable. Tuning fork tests indicate conductive hearing loss in the right ear. No other abnormalities are noted on the head and neck examination.

## Encounter 2: source 2021-07-15, practice 2026-01-05, outpatient, Pulmonary Medicine

Chief complaint: Daily productive cough with foul-smelling sputum, hemoptysis, and unintentional weight loss over the past year

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Priya Sharma, MD
Department: Pulmonary Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Daily productive cough with foul-smelling sputum, hemoptysis, and unintentional weight loss over the past year

HISTORY OF PRESENT ILLNESS:
The patient is a 33-year-old female with chronic hypertension and prior cholesteatoma who presents to Pulmonary Medicine. She reports expectorating large amounts of thick, foul-smelling, yellow-green sputum, especially in the mornings. Occasionally, she notes streaks of blood in the mucus. She endorses fatigue, exertional dyspnea, and unintentional weight loss over the past several months.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic hypertension in pregnancy (diagnosed 2020-01-15)
- Cholesteatoma of middle ear (attic cholesteatoma) (diagnosed 2020-07-15)
- Chronic hypertension

She denies asthma, allergies, or prior lung disease.

PAST SURGICAL HISTORY:
- Bilateral tympanostomy tube placement (childhood)
- Vaginal delivery (G1P1, age 29)

MEDICATIONS:
Home Medications:
- Labetalol 200 mg PO twice daily
- Prenatal vitamin 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Hypertension (maternal)
- Hypertension (paternal)

SOCIAL HISTORY:
She works in a crowded daycare and recently returned from a 2-month volunteer trip in rural India. She has a 5-pack-year smoking history but quit at age 26.

VITAL SIGNS:
Vital signs are within normal limits.

PHYSICAL EXAMINATION:
On physical exam, there are coarse crackles at the right lung base, and digital clubbing is noted.

IMAGING:
A chest X-ray is performed and shown below.
