# synthetic_hospital patient 2319

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 63 at first encounter, sex M, Non-Hispanic White, Private PPO (employer retiree health plan).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-08-13 (Tue) | outpatient | Pulmonology |
| 1 | 2021-01-15 | 2025-08-20 (Wed) | follow_up | Oncology |
| 2 | 2021-07-15 | 2026-02-16 (Mon) | follow_up | Oncology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Iodinated contrast - urticaria",
  "Sulfa antibiotics - rash"
 ],
 "insurance": "Private PPO (employer retiree health plan)",
 "occupation": "Retired construction supervisor",
 "alcohol_use": "Social, 1-2 drinks per week",
 "family_history": [
  "Father: Lung cancer, deceased age 67",
  "Paternal uncle: Renal cell carcinoma, age 72",
  "Mother: Hypertension, age 85"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 40 pack-years, quit 2 years ago",
 "home_medications": [
  {
   "dose": "2L continuous",
   "name": "Oxygen via nasal cannula"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Prednisone"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Furosemide"
  },
  {
   "dose": "650 mg PO every 6 hours PRN pain",
   "name": "Acetaminophen"
  }
 ],
 "surgical_history": [
  "Left upper lobectomy (2019)",
  "Left radical orchiectomy (2022)",
  "Left radical nephrectomy (2023)"
 ],
 "chronic_conditions": [
  "Bleomycin-induced pulmonary fibrosis",
  "Chronic kidney disease stage 3b",
  "Hypertension",
  "History of stage I non-small cell lung cancer (status post left upper lobectomy)",
  "History of left testicular non-seminomatous germ cell tumor",
  "Metastatic clear cell renal cell carcinoma"
 ],
 "age_at_first_encounter": 63
}
```

Primary diagnoses: ["Suspected malignant neoplasm of lung", "Bleomycin-induced pulmonary fibrosis", "Clear cell renal cell carcinoma"]
Comorbidities: ["Iatrogenic immunosuppression"]

## Encounter 0: source 2020-01-15, practice 2024-08-13, outpatient, Pulmonology

Chief complaint: Persistent nonproductive cough and progressive shortness of breath on exertion

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent nonproductive cough and progressive shortness of breath on exertion

HISTORY OF PRESENT ILLNESS:
A 63-year-old man with newly diagnosed testicular cancer is started on combination chemotherapy that includes cisplatin, etoposide, and bleomycin. Six months later, he returns with a persistent nonproductive cough and progressive shortness of breath on exertion.

PAST MEDICAL HISTORY:
Active Problem List:
- Bleomycin-induced pulmonary fibrosis
- Chronic kidney disease stage 3b
- Hypertension
- History of stage I non-small cell lung cancer (status post left upper lobectomy)
- History of left testicular non-seminomatous germ cell tumor
- Metastatic clear cell renal cell carcinoma

PAST SURGICAL HISTORY:
- Left upper lobectomy (2019)
- Left radical orchiectomy (2022)
- Left radical nephrectomy (2023)

MEDICATIONS:
He takes combination chemotherapy that includes cisplatin, etoposide, and bleomycin.

Home Medications:
- Oxygen via nasal cannula 2L continuous
- Prednisone 10 mg PO daily
- Lisinopril 5 mg PO daily
- Furosemide 20 mg PO daily
- Acetaminophen 650 mg PO every 6 hours PRN pain

ALLERGIES:
Iodinated contrast - urticaria, Sulfa antibiotics - rash

FAMILY HISTORY:
- Father: Lung cancer, deceased age 67
- Paternal uncle: Renal cell carcinoma, age 72
- Mother: Hypertension, age 85

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 2 years ago. Alcohol: Social, 1-2 drinks per week. Occupation: Retired construction supervisor.

VITAL SIGNS:
Oxygen saturation is 91% on room air.

PHYSICAL EXAMINATION:
Physical examination reveals fine, dry crackles at the lung bases.

IMAGING:
A high-resolution CT scan of the chest is shown.

OTHER STUDIES:
Pulmonary function testing shows a reduced total lung capacity and reduced diffusing capacity for carbon monoxide (DLCO).

## Encounter 1: source 2021-01-15, practice 2025-08-20, follow_up, Oncology

Chief complaint: Follow-up appointment after completing second course of chemotherapy for metastatic renal cell carcinoma

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. James Park, MD
Department: Oncology
Visit Type: follow_up

CHIEF COMPLAINT:
Follow-up appointment after completing second course of chemotherapy for metastatic renal cell carcinoma

HISTORY OF PRESENT ILLNESS:
The patient returns to the oncology clinic for follow-up care with history of bleomycin-induced pulmonary fibrosis. He recently finished a 2nd course of chemotherapy for metastatic malignancy.

PAST MEDICAL HISTORY:
Active Problem List:
- Bleomycin-induced pulmonary fibrosis (diagnosed 2020-01-15)
- Chronic kidney disease stage 3b
- Hypertension
- History of stage I non-small cell lung cancer (status post left upper lobectomy)
- History of left testicular non-seminomatous germ cell tumor
- Metastatic clear cell renal cell carcinoma

PAST SURGICAL HISTORY:
- Left upper lobectomy (2019)
- Left radical orchiectomy (2022)
- Left radical nephrectomy (2023)

MEDICATIONS:
Home Medications:
- Oxygen via nasal cannula 2L continuous
- Prednisone 10 mg PO daily
- Lisinopril 5 mg PO daily
- Furosemide 20 mg PO daily
- Acetaminophen 650 mg PO every 6 hours PRN pain

ALLERGIES:
Iodinated contrast - urticaria, Sulfa antibiotics - rash

FAMILY HISTORY:
- Father: Lung cancer, deceased age 67
- Paternal uncle: Renal cell carcinoma, age 72
- Mother: Hypertension, age 85

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 2 years ago. Alcohol: Social, 1-2 drinks per week. Occupation: Retired construction supervisor.

LABORATORY DATA:
Serum calcium is 10.8 mg/dL. Hemoglobin is 18 g/dL.

OTHER STUDIES:
A biopsy is performed.

## Encounter 2: source 2021-07-15, practice 2026-02-16, follow_up, Oncology

Chief complaint: Follow-up appointment to discuss next steps in treatment for suspected lung malignancy

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. James Park, MD
Department: Oncology
Visit Type: follow_up

CHIEF COMPLAINT:
Follow-up appointment to discuss next steps in treatment for suspected lung malignancy

HISTORY OF PRESENT ILLNESS:
The patient is a 64-year-old male with bleomycin-induced pulmonary fibrosis, metastatic clear cell renal cell carcinoma, and history of stage I non-small cell lung cancer status post left upper lobectomy who returns for follow-up to discuss next steps in treatment for suspected lung malignancy. He has recently completed a course of chemotherapy and appears fatigued. When the physician begins to discuss next steps, the patient states, "I know the cancer is bad, but my wife says I shouldn't give up. She's read about new treatments online and wants me to try everything." He then pauses and adds, "But honestly, I'm just so tired." His wife, who is seated beside him, immediately interrupts and states, "Don't say that. You can't give up now."

PAST MEDICAL HISTORY:
Active Problem List:
- Bleomycin-induced pulmonary fibrosis (diagnosed 2020-01-15)
- Clear cell renal cell carcinoma (diagnosed 2021-01-15)
- Chronic kidney disease stage 3b
- Hypertension
- History of stage I non-small cell lung cancer (status post left upper lobectomy)
- History of left testicular non-seminomatous germ cell tumor
- Metastatic clear cell renal cell carcinoma

PAST SURGICAL HISTORY:
- Left upper lobectomy (2019)
- Left radical orchiectomy (2022)
- Left radical nephrectomy (2023)

MEDICATIONS:
Home Medications:
- Oxygen via nasal cannula 2L continuous
- Prednisone 10 mg PO daily
- Lisinopril 5 mg PO daily
- Furosemide 20 mg PO daily
- Acetaminophen 650 mg PO every 6 hours PRN pain

ALLERGIES:
Iodinated contrast - urticaria, Sulfa antibiotics - rash

FAMILY HISTORY:
- Father: Lung cancer, deceased age 67
- Paternal uncle: Renal cell carcinoma, age 72
- Mother: Hypertension, age 85

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 2 years ago. Alcohol: Social, 1-2 drinks per week. Occupation: Retired construction supervisor.
