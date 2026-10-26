# synthetic_hospital patient 2818

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 58 at first encounter, sex F, Non-Hispanic White, Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-07-15 | 2026-08-03 (Mon) | inpatient | Orthopedic Surgery |
| 1 | 2020-07-15 | 2026-08-03 (Mon) | inpatient | Orthopedic Surgery |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield PPO",
 "occupation": "Retired elementary school teacher",
 "alcohol_use": "Social (1-2 glasses of wine per week)",
 "family_history": [
  "Mother: Osteoporosis with hip fracture at age 72",
  "Father: Hypertension, Type 2 diabetes",
  "Sister: Breast cancer (age 61)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "75 mcg PO daily",
   "name": "Levothyroxine"
  },
  {
   "dose": "70 mg PO weekly",
   "name": "Alendronate"
  },
  {
   "dose": "600 mg/400 units PO twice daily",
   "name": "Calcium carbonate with Vitamin D3"
  },
  {
   "dose": "500 mg PO as needed for pain",
   "name": "Acetaminophen"
  }
 ],
 "surgical_history": [
  "Cesarean section x2 (remote)",
  "Laparoscopic cholecystectomy (2005)",
  "Arthroscopic meniscectomy, left knee (2015)"
 ],
 "chronic_conditions": [
  "Severe osteoarthritis, right hip",
  "Osteoporosis",
  "Hypertension",
  "Hypothyroidism"
 ],
 "age_at_first_encounter": 58
}
```

Primary diagnoses: ["Transfusion-related acute lung injury", "Opioid-induced respiratory depression"]
Comorbidities: ["Status post hip replacement surgery"]

## Encounter 0: source 2020-07-15, practice 2026-08-03, inpatient, Orthopedic Surgery

Chief complaint: Acute dyspnea and hypoxia occurring 45 minutes into intraoperative blood transfusion during elective hip arthroplasty

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Sarah Chen, MD
Department: Orthopedic Surgery
Visit Type: inpatient

CHIEF COMPLAINT:
Acute dyspnea and hypoxia occurring 45 minutes into intraoperative blood transfusion during elective hip arthroplasty

HISTORY OF PRESENT ILLNESS:
Undergoing elective hip arthroplasty receives a unit of packed red blood cells for intraoperative blood loss. Approximately 45 minutes into the transfusion, she develops hypoxia.

PAST MEDICAL HISTORY:
Active Problem List:
- Severe osteoarthritis, right hip
- Osteoporosis
- Hypertension
- Hypothyroidism

PAST SURGICAL HISTORY:
- Cesarean section x2 (remote)
- Laparoscopic cholecystectomy (2005)
- Arthroscopic meniscectomy, left knee (2015)

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily
- Levothyroxine 75 mcg PO daily
- Alendronate 70 mg PO weekly
- Calcium carbonate with Vitamin D3 600 mg/400 units PO twice daily
- Acetaminophen 500 mg PO as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Osteoporosis with hip fracture at age 72
- Father: Hypertension, Type 2 diabetes
- Sister: Breast cancer (age 61)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per week). Occupation: Retired elementary school teacher.

VITAL SIGNS:
Oxygen saturation drops to 81% on room air.
Her blood pressure remains normal,

PHYSICAL EXAMINATION:
Bilateral crackles on auscultation.
and there is no evidence of urticaria or wheezing.

LABORATORY DATA:
Laboratory studies show normal mucosal immunoglobulin levels. A Coombs test is negative.

IMAGING:
Chest X-ray reveals bilateral pulmonary infiltrates without signs of cardiomegaly or volume overload.

PLAN:
She is intubated and admitted to the ICU.

## Encounter 1: source 2020-07-15, practice 2026-08-03, inpatient, Orthopedic Surgery

Chief complaint: Increased somnolence and difficulty arousing on postoperative day 2 following right hip replacement

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Sarah Chen, MD
Department: Orthopedic Surgery
Visit Type: inpatient

CHIEF COMPLAINT:
Increased somnolence and difficulty arousing on postoperative day 2 following right hip replacement

HISTORY OF PRESENT ILLNESS:
This 58-year-old female with severe right hip osteoarthritis, osteoporosis, hypertension, and hypothyroidism returns postoperative day two after right-sided hip replacement with intraoperative dyspnea and hypoxia. She is recovering in hospital due to a recent fall at home with initially unremarkable postoperative course, and her pulse oximetry shows a slight decline from baseline, prompting bedside evaluation.

PAST MEDICAL HISTORY:
Active Problem List:
- Transfusion-related acute lung injury (diagnosed 2020-07-15)
- Severe osteoarthritis, right hip
- Osteoporosis
- Hypertension
- Hypothyroidism

PAST SURGICAL HISTORY:
- Cesarean section x2 (remote)
- Laparoscopic cholecystectomy (2005)
- Arthroscopic meniscectomy, left knee (2015)

MEDICATIONS:
She takes she has been receiving intravenous morphine for pain control. Her pain regimen is reviewed, and morphine is identified as the only sedating agent currently administered.

Home Medications:
- Lisinopril 10 mg PO daily
- Levothyroxine 75 mcg PO daily
- Alendronate 70 mg PO weekly
- Calcium carbonate with Vitamin D3 600 mg/400 units PO twice daily
- Acetaminophen 500 mg PO as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Osteoporosis with hip fracture at age 72
- Father: Hypertension, Type 2 diabetes
- Sister: Breast cancer (age 61)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per week). Occupation: Retired elementary school teacher.

REVIEW OF SYSTEMS:
She has not complained of any chest pain, cough, or shortness of breath.

VITAL SIGNS:
Her respiratory rate is 8 breaths per minute and shallow. and her oxygen saturation is 91% on room air. Her temperature is within normal limits,

PHYSICAL EXAMINATION:
On examination, she is arousable to voice but appears drowsy. Her lungs are clear to auscultation anteriorly but reveal mild inspiratory crackles at the posterior bases bilaterally. She is not using accessory muscles of respiration, Cardiac examination reveals normal heart sounds with no murmurs. and there is no peripheral edema or signs of volume overload.

LABORATORY DATA:
There is no significant leukocytosis, and D-dimer is normal.

IMAGING:
A chest x-ray shows mild bibasilar atelectasis without consolidation or effusion.
