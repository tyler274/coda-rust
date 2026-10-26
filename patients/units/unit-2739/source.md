# synthetic_hospital patient 2739

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex F, Non-Hispanic White, Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-06-21 (Fri) | outpatient | Obstetrics & Gynecology |
| 1 | 2020-07-15 | 2024-12-09 (Mon) | outpatient | Orthopedics & Sports Medicine |
| 2 | 2021-07-15 | 2025-12-17 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Blue Cross Blue Shield PPO",
 "occupation": "High School Science Teacher",
 "alcohol_use": "None",
 "family_history": [
  "Mother: Type 2 diabetes mellitus, hypothyroidism",
  "Father: Essential hypertension, hyperlipidemia",
  "Sister: Polycystic ovary syndrome, prediabetes"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1000 mg orally twice daily",
   "name": "Metformin"
  },
  {
   "dose": "10 mg orally daily",
   "name": "Lisinopril"
  }
 ],
 "surgical_history": [
  "Laparoscopic cholecystectomy (4 months prior)",
  "Tonsillectomy (age 12)"
 ],
 "chronic_conditions": [
  "Type 2 diabetes mellitus",
  "Chronic hypertension",
  "Polycystic ovary syndrome",
  "Obesity"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Plantar fasciitis", "Polycystic ovary syndrome", "Bile leak"]
Comorbidities: ["Obesity"]

## Encounter 0: source 2020-01-15, practice 2024-06-21, outpatient, Obstetrics & Gynecology

Chief complaint: Intermittent episodes of heavy vaginal bleeding over the past two months with irregular menstrual cycles occurring every 2-3 months.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Sharma, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent episodes of heavy vaginal bleeding over the past two months with irregular menstrual cycles occurring every 2-3 months.

HISTORY OF PRESENT ILLNESS:
Her menstrual periods have been irregular for the past year, occurring once every 2–3 months with varying flow. Menarche occurred at age 10. She has never been pregnant and has not used hormonal contraception.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus
- Chronic hypertension
- Polycystic ovary syndrome
- Obesity

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (4 months prior)
- Tonsillectomy (age 12)

MEDICATIONS:
Metformin and lisinopril

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus, hypothyroidism
- Father: Essential hypertension, hyperlipidemia
- Sister: Polycystic ovary syndrome, prediabetes

SOCIAL HISTORY:
She does not smoke or drink alcohol.

VITAL SIGNS:
BMI of 33 kg/m²

PHYSICAL EXAMINATION:
Her pelvic examination shows no abnormalities except a small amount of blood at the cervical os.

IMAGING:
Transvaginal ultrasound shows an endometrial stripe measuring 14 mm.

OTHER STUDIES:
Endometrial biopsy is pending.

## Encounter 1: source 2020-07-15, practice 2024-12-09, outpatient, Orthopedics & Sports Medicine

Chief complaint: Two-month history of heel pain severe with first steps in the morning or after rest, exacerbated by recent increase in running mileage.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Orthopedics & Sports Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Two-month history of heel pain severe with first steps in the morning or after rest, exacerbated by recent increase in running mileage.

HISTORY OF PRESENT ILLNESS:
The patient is a 45-year-old female with type 2 diabetes mellitus, chronic hypertension, polycystic ovary syndrome, and obesity, last seen in January 2020 in Obstetrics & Gynecology for irregular menstrual cycles, who now presents to the Orthopedics & Sports Medicine clinic with a 2-month history of heel pain, particularly severe with the first steps in the morning or after periods of rest. She is an avid runner and recently increased her mileage in preparation for a marathon. She denies any recent trauma or injury.

PAST MEDICAL HISTORY:
Active Problem List:
- Polycystic ovary syndrome (diagnosed 2020-01-15)
- Type 2 diabetes mellitus
- Chronic hypertension
- Obesity

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (4 months prior)
- Tonsillectomy (age 12)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg orally twice daily
- Lisinopril 10 mg orally daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus, hypothyroidism
- Father: Essential hypertension, hyperlipidemia
- Sister: Polycystic ovary syndrome, prediabetes

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: High School Science Teacher.

VITAL SIGNS:
Her BMI is 30

PHYSICAL EXAMINATION:
On examination, there is tenderness at the anteromedial aspect of the calcaneus. Passive dorsiflexion of the toes elicits pain, and there is no swelling or erythema. Ankle dorsiflexion strength is 5/5 bilaterally. No neurological deficits are noted.

IMAGING:
Plain radiographs of the foot show no fractures or calcifications.

## Encounter 2: source 2021-07-15, practice 2025-12-17, ed, Emergency Medicine

Chief complaint: Acute worsening of nausea, postprandial vomiting, and difficulty swallowing solids and liquids with associated epigastric pain.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Elena Vasquez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute worsening of nausea, postprandial vomiting, and difficulty swallowing solids and liquids with associated epigastric pain.

HISTORY OF PRESENT ILLNESS:
Patient with T2DM, HTN, PCOS, obesity, prior OBGYN and ortho visits, presents with worsening GI symptoms. She reports progressive worsening of these symptoms. However, she notes a weight plateau for 6 months despite adherence to dietary recommendations.

PAST MEDICAL HISTORY:
Active Problem List:
- Polycystic ovary syndrome (diagnosed 2020-01-15)
- Plantar fasciitis (diagnosed 2020-07-15)
- Type 2 diabetes mellitus
- Chronic hypertension
- Obesity

PAST SURGICAL HISTORY:
She underwent Roux-en-Y gastric bypass surgery 18 months ago and initially experienced significant weight loss.

MEDICATIONS:
Home Medications:
- Metformin 1000 mg orally twice daily
- Lisinopril 10 mg orally daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Type 2 diabetes mellitus, hypothyroidism
- Father: Essential hypertension, hyperlipidemia
- Sister: Polycystic ovary syndrome, prediabetes

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: High School Science Teacher.

PHYSICAL EXAMINATION:
On physical examination, her BMI is 32 kg/m^2, and abdominal examination reveals no abnormalities.

OTHER STUDIES:
Upper endoscopy shows a narrowed gastrojejunal anastomosis with a diameter of 7 mm.
