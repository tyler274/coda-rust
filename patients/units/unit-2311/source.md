# synthetic_hospital patient 2311

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 75 at first encounter, sex M, Non-Hispanic White, Medicare and Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 1 | 2023-01-15 | 2023-11-12 (Sun) | ed | Emergency Medicine |
| 2 | 2025-01-15 | 2025-10-27 (Mon) | icu | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Penicillin (anaphylaxis)",
  "Sulfa drugs (rash)"
 ],
 "insurance": "Medicare and Medicaid",
 "occupation": "Retired postal worker",
 "alcohol_use": "None (abstinent since 2018 due to cognitive decline)",
 "family_history": [
  "Mother: Alzheimer dementia (onset age 79)",
  "Father: Coronary artery disease with myocardial infarction at age 76",
  "Sister: Type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 2005 (25 pack-years)",
 "home_medications": [
  {
   "dose": "10 mg by mouth daily at bedtime",
   "name": "Donepezil"
  },
  {
   "dose": "10 mg by mouth twice daily",
   "name": "Memantine"
  },
  {
   "dose": "5 mg by mouth daily",
   "name": "Amlodipine"
  },
  {
   "dose": "0.4 mg by mouth daily at bedtime",
   "name": "Tamsulosin"
  },
  {
   "dose": "500 mg by mouth every 6 hours as needed for pain",
   "name": "Acetaminophen"
  },
  {
   "dose": "17 g by mouth daily as needed for constipation",
   "name": "Polyethylene glycol 3350"
  }
 ],
 "surgical_history": [
  "Cataract extraction with intraocular lens placement, bilateral (2020)",
  "Inguinal hernia repair, open (2016)"
 ],
 "chronic_conditions": [
  "Alzheimer's disease, moderate stage",
  "Essential hypertension",
  "Osteoarthritis, bilateral knees",
  "Benign prostatic hyperplasia"
 ],
 "age_at_first_encounter": 75
}
```

Primary diagnoses: ["Nonexertional (classic) heat stroke", "Delirium superimposed on dementia"]
Comorbidities: ["Alzheimer's disease"]

## Encounter 1: source 2023-01-15, practice 2023-11-12, ed, Emergency Medicine

Chief complaint: Increased confusion and agitation over the past two days with nocturnal wandering and inability to recognize family members.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. James Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Increased confusion and agitation over the past two days with nocturnal wandering and inability to recognize family members.

HISTORY OF PRESENT ILLNESS:
The patient is a 75-year-old male with a known history of Alzheimer's disease (moderate stage), essential hypertension, osteoarthritis of the bilateral knees, and benign prostatic hyperplasia who presents to the emergency department for initial evaluation. He is brought by his daughter due to increased confusion and agitation over the past two days. She reports that he has been wandering at night and not recognizing family members. The patient becomes more agitated when he is restrained for an IV line placement.

PAST MEDICAL HISTORY:
Active Problem List:
- Alzheimer's disease, moderate stage
- Essential hypertension
- Osteoarthritis, bilateral knees
- Benign prostatic hyperplasia

PAST SURGICAL HISTORY:
- Cataract extraction with intraocular lens placement, bilateral (2020)
- Inguinal hernia repair, open (2016)

MEDICATIONS:
His medications include donepezil and memantine.

Home Medications:
- Amlodipine 5 mg by mouth daily
- Tamsulosin 0.4 mg by mouth daily at bedtime
- Acetaminophen 500 mg by mouth every 6 hours as needed for pain
- Polyethylene glycol 3350 17 g by mouth daily as needed for constipation

ALLERGIES:
Penicillin (anaphylaxis), Sulfa drugs (rash)

FAMILY HISTORY:
- Mother: Alzheimer dementia (onset age 79)
- Father: Coronary artery disease with myocardial infarction at age 76
- Sister: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, quit 2005 (25 pack-years). Alcohol: None (abstinent since 2018 due to cognitive decline). Occupation: Retired postal worker.

VITAL SIGNS:
His vital signs are stable

PHYSICAL EXAMINATION:
On examination, the patient is disoriented to time and place but oriented to person. He is unable to follow complex commands and is easily distracted by the environment. His vital signs are stable, and physical examination is unremarkable.

LABORATORY DATA:
Laboratory tests reveal:

- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Chloride: 102 mEq/L
- Bicarbonate: 24 mEq/L
- Blood urea nitrogen: 18 mg/dL
- Creatinine: 1.0 mg/dL
- Glucose: 98 mg/dL
- Calcium: 9.2 mg/dL
- Complete blood count: within normal limits
- Urinalysis: normal

IMAGING:
A non-contrast head CT scan shows no acute abnormalities.

## Encounter 2: source 2025-01-15, practice 2025-10-27, icu, Emergency Medicine

Chief complaint: Found less responsive and confused during heatwave; altered mental status and hyperthermia.

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Amara Okafor, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Found less responsive and confused during heatwave; altered mental status and hyperthermia.

HISTORY OF PRESENT ILLNESS:
The patient is a 77-year-old male with moderate-stage Alzheimer's disease, essential hypertension, osteoarthritis, and benign prostatic hyperplasia who is brought to the emergency department by his family during a heatwave in July, two years after a prior episode of delirium superimposed on dementia. He resides in a nursing home without air conditioning. His family reports that he has been less responsive and more confused over the past day.

PAST MEDICAL HISTORY:
Active Problem List:
- Delirium superimposed on dementia (diagnosed 2023-01-15)
- Alzheimer's disease, moderate stage
- Essential hypertension
- Osteoarthritis, bilateral knees
- Benign prostatic hyperplasia

PAST SURGICAL HISTORY:
- Cataract extraction with intraocular lens placement, bilateral (2020)
- Inguinal hernia repair, open (2016)

MEDICATIONS:
Home Medications:
- Donepezil 10 mg by mouth daily at bedtime
- Memantine 10 mg by mouth twice daily
- Amlodipine 5 mg by mouth daily
- Tamsulosin 0.4 mg by mouth daily at bedtime
- Acetaminophen 500 mg by mouth every 6 hours as needed for pain
- Polyethylene glycol 3350 17 g by mouth daily as needed for constipation

ALLERGIES:
Penicillin (anaphylaxis), Sulfa drugs (rash)

FAMILY HISTORY:
- Mother: Alzheimer dementia (onset age 79)
- Father: Coronary artery disease with myocardial infarction at age 76
- Sister: Type 2 diabetes mellitus

SOCIAL HISTORY:
Resides in a nursing home without air conditioning

VITAL SIGNS:
His temperature is 41.1 C (106 F), heart rate is 110/min, blood pressure is 90/60 mmHg, and respiratory rate is 24/min.

PHYSICAL EXAMINATION:
On examination, the patient is lethargic, disoriented, and unable to answer questions coherently. His skin is hot and dry to the touch.

LABORATORY DATA:
- Sodium: 150 mEq/L
- Potassium: 3.2 mEq/L
- Creatinine: 1.8 mg/dL
- Blood urea nitrogen (BUN): 30 mg/dL
- Arterial blood gas:
- pH: 7.30
- PaCO2: 30 mmHg
- PaO2: 95 mmHg
- HCO3-: 18 mEq/L

ASSESSMENT AND PLAN:
The patient's core temperature and mental status do not improve significantly after initial cooling measures and fluid resuscitation.
