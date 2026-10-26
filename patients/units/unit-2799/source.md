# synthetic_hospital patient 2799

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 56 at first encounter, sex F, Non-Hispanic White, Commercial preferred provider organization (PPO).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-07-29 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-04-15 | 2024-10-15 (Tue) | outpatient | Gastroenterology |
| 2 | 2021-07-15 | 2026-01-09 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial preferred provider organization (PPO)",
 "occupation": "Office administrator",
 "alcohol_use": "None",
 "family_history": [
  "Father: Lung cancer (deceased age 68)",
  "Mother: Type 2 diabetes mellitus",
  "Sister: Breast cancer"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 30 pack-years, quit 1 year ago",
 "home_medications": [
  {
   "dose": "20 mg PO twice daily",
   "name": "Omeprazole"
  },
  {
   "dose": "600 mg/400 units PO twice daily",
   "name": "Calcium carbonate with vitamin D3"
  },
  {
   "dose": "1 tablet PO daily",
   "name": "Multivitamin"
  }
 ],
 "surgical_history": [
  "Partial gastrectomy with Billroth II reconstruction for peptic ulcer disease (2 years prior)",
  "Revision distal gastrectomy for anastomotic stricture (3 months prior)"
 ],
 "chronic_conditions": [
  "Peptic ulcer disease",
  "Tobacco use disorder in remission",
  "Postoperative status following gastric surgery"
 ],
 "age_at_first_encounter": 56
}
```

Primary diagnoses: ["Vitamin B12 deficiency", "Renal Cell Carcinoma", "Dumping syndrome"]
Comorbidities: ["Cancer-related constitutional symptoms", "Upper gastrointestinal bleeding (peptic ulcer disease or erosive gastritis)"]

## Encounter 0: source 2020-01-15, practice 2024-07-29, outpatient, Internal Medicine

Chief complaint: Fatigue, weakness, and a sensation of pins and needles in her feet

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue, weakness, and a sensation of pins and needles in her feet

HISTORY OF PRESENT ILLNESS:
She presents to the clinic with a 3-month history of fatigue, weakness, and a sensation of pins and needles in her feet. She reports a loss of appetite and a 5 kg weight loss over the same period.

PAST MEDICAL HISTORY:
Active Problem List:
- Peptic ulcer disease
- Tobacco use disorder in remission
- Postoperative status following gastric surgery

Her past medical history is significant for a partial gastrectomy performed 2 years ago due to peptic ulcer disease.

PAST SURGICAL HISTORY:
- Partial gastrectomy with Billroth II reconstruction for peptic ulcer disease (2 years prior)
- Revision distal gastrectomy for anastomotic stricture (3 months prior)

MEDICATIONS:
Home Medications:
- Omeprazole 20 mg PO twice daily
- Calcium carbonate with vitamin D3 600 mg/400 units PO twice daily
- Multivitamin 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (deceased age 68)
- Mother: Type 2 diabetes mellitus
- Sister: Breast cancer

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 1 year ago. Alcohol: None. Occupation: Office administrator.

PHYSICAL EXAMINATION:
On physical examination, she appears pale, and her conjunctivae are pale. Neurological examination reveals decreased vibratory sense and proprioception in her lower extremities.

LABORATORY DATA:
Her complete blood count shows:

- Hemoglobin: 9.2 g/dL (Normal: 12-16 g/dL)
- Mean corpuscular volume (MCV): 110 fL (Normal: 80-100 fL)
- White blood cells: 4.2 x 10^9/L (Normal: 4-11 x 10^9/L)
- Platelets: 180 x 10^9/L (Normal: 150-450 x 10^9/L)

Serum vitamin B12 levels are pending.

## Encounter 1: source 2020-04-15, practice 2024-10-15, outpatient, Gastroenterology

Chief complaint: Nausea, abdominal cramps, diarrhea, and palpitations occurring 15-30 minutes after eating

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Rajesh Patel, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Nausea, abdominal cramps, diarrhea, and palpitations occurring 15-30 minutes after eating

HISTORY OF PRESENT ILLNESS:
This 56-year-old female with peptic ulcer disease and prior gastric surgery returns following treatment for vitamin B12 deficiency. She also reports episodes of palpitations and diaphoresis that occur approximately 15-30 minutes after eating.

PAST MEDICAL HISTORY:
Active Problem List:
- Vitamin B12 deficiency (diagnosed 2020-01-15)
- Peptic ulcer disease
- Tobacco use disorder in remission
- Postoperative status following gastric surgery

Her medical history is significant for peptic ulcer disease for which she underwent a partial gastrectomy 3 months ago.

PAST SURGICAL HISTORY:
- Partial gastrectomy with Billroth II reconstruction for peptic ulcer disease (2 years prior)
- Revision distal gastrectomy for anastomotic stricture (3 months prior)

MEDICATIONS:
Home Medications:
- Omeprazole 20 mg PO twice daily
- Calcium carbonate with vitamin D3 600 mg/400 units PO twice daily
- Multivitamin 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (deceased age 68)
- Mother: Type 2 diabetes mellitus
- Sister: Breast cancer

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 1 year ago. Alcohol: None. Occupation: Office administrator.

VITAL SIGNS:
On physical examination, her vital signs are within normal limits

PHYSICAL EXAMINATION:
Abdominal examination reveals no abnormalities.

LABORATORY DATA:
A postprandial glucose test shows a rapid rise and fall in blood glucose levels.

Laboratory values:
- Hemoglobin: 13.2 g/dL
- Hematocrit: 39.8%
- Fasting blood glucose: 89 mg/dL
- Postprandial blood glucose at 30 minutes: 184 mg/dL
- Postprandial blood glucose at 90 minutes: 72 mg/dL

## Encounter 2: source 2021-07-15, practice 2026-01-09, ed, Emergency Medicine

Chief complaint: Visible blood in urine over the past 3 days with dull left-sided flank pain

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Maria Gonzalez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Visible blood in urine over the past 3 days with dull left-sided flank pain

HISTORY OF PRESENT ILLNESS:
This 57-year-old female with a history of gastric surgery and prior management of dumping syndrome and vitamin B12 deficiency presents with visible hematuria over the past 3 days. She has also experienced dull left-sided flank pain and reports an unintentional 10-kg weight loss over the last 2 months.

PAST MEDICAL HISTORY:
Active Problem List:
- Vitamin B12 deficiency (diagnosed 2020-01-15)
- Dumping syndrome (diagnosed 2020-04-15)
- Peptic ulcer disease
- Tobacco use disorder in remission
- Postoperative status following gastric surgery

PAST SURGICAL HISTORY:
- Partial gastrectomy with Billroth II reconstruction for peptic ulcer disease (2 years prior)
- Revision distal gastrectomy for anastomotic stricture (3 months prior)

MEDICATIONS:
Home Medications:
- Omeprazole 20 mg PO twice daily
- Calcium carbonate with vitamin D3 600 mg/400 units PO twice daily
- Multivitamin 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (deceased age 68)
- Mother: Type 2 diabetes mellitus
- Sister: Breast cancer

SOCIAL HISTORY:
She does not smoke currently but has a 30-pack-year smoking history.

REVIEW OF SYSTEMS:
Review of systems reveals intermittent headaches and episodes of blurry vision.

VITAL SIGNS:
Vital signs are within normal limits.

PHYSICAL EXAMINATION:
Physical examination is notable for left costovertebral angle tenderness but no abdominal masses.

LABORATORY DATA:
Laboratory testing shows a hematocrit of 55%.

IMAGING:
A renal ultrasound reveals a heterogenous mass in the left kidney.
