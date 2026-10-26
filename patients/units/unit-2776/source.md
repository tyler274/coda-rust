# synthetic_hospital patient 2776

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 46 at first encounter, sex F, Non-Hispanic White, Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-04-18 (Thu) | outpatient | Gynecologic Oncology |
| 1 | 2021-07-15 | 2025-10-20 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield PPO",
 "occupation": "Administrative assistant",
 "alcohol_use": "Occasional (less than 1 drink per week)",
 "family_history": [
  "Mother: Breast cancer at age 54",
  "Maternal aunt: Ovarian cancer at age 61",
  "Father: Chronic obstructive pulmonary disease",
  "Paternal grandfather: Pancreatic cancer"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 30 pack-years, quit 3 months ago",
 "home_medications": [
  {
   "dose": "125 mcg PO daily",
   "name": "Levothyroxine"
  },
  {
   "dose": "300 mg PO TID",
   "name": "Gabapentin"
  },
  {
   "dose": "5 mg PO q6h PRN pain",
   "name": "Oxycodone"
  },
  {
   "dose": "8 mg PO q8h PRN nausea",
   "name": "Ondansetron"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Pantoprazole"
  },
  {
   "dose": "0.5 mg PO BID PRN anxiety",
   "name": "Lorazepam"
  }
 ],
 "surgical_history": [
  "Staging laparotomy with splenectomy (15 years prior)",
  "Total abdominal hysterectomy with bilateral salpingo-oophorectomy and optimal tumor debulking",
  "Port-a-cath placement (right subclavian)"
 ],
 "chronic_conditions": [
  "History of Hodgkin lymphoma in remission",
  "Post-radiation hypothyroidism",
  "Chemotherapy-induced peripheral neuropathy",
  "Chronic pain syndrome",
  "Tobacco use disorder",
  "Generalized anxiety disorder"
 ],
 "age_at_first_encounter": 46
}
```

Primary diagnoses: ["Malignant neoplasm of ovary", "Esophageal carcinoma"]
Comorbidities: ["Iatrogenic immunosuppression"]

## Encounter 0: source 2020-01-15, practice 2024-04-18, outpatient, Gynecologic Oncology

Chief complaint: Follow-up visit for high-grade serous ovarian carcinoma with recent CT showing disease progression despite platinum-based chemotherapy

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Gynecologic Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up visit for high-grade serous ovarian carcinoma with recent CT showing disease progression despite platinum-based chemotherapy

HISTORY OF PRESENT ILLNESS:
Her tumor initially responded to platinum-based therapy, but now shows continued growth despite treatment.

PAST MEDICAL HISTORY:
Active Problem List:
- History of Hodgkin lymphoma in remission
- Post-radiation hypothyroidism
- Chemotherapy-induced peripheral neuropathy
- Chronic pain syndrome
- Tobacco use disorder
- Generalized anxiety disorder

PAST SURGICAL HISTORY:
- Staging laparotomy with splenectomy (15 years prior)
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy and optimal tumor debulking
- Port-a-cath placement (right subclavian)

MEDICATIONS:
Home Medications:
- Levothyroxine 125 mcg PO daily
- Gabapentin 300 mg PO TID
- Oxycodone 5 mg PO q6h PRN pain
- Ondansetron 8 mg PO q8h PRN nausea
- Pantoprazole 40 mg PO daily
- Lorazepam 0.5 mg PO BID PRN anxiety

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer at age 54
- Maternal aunt: Ovarian cancer at age 61
- Father: Chronic obstructive pulmonary disease
- Paternal grandfather: Pancreatic cancer

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 3 months ago. Alcohol: Occasional (less than 1 drink per week). Occupation: Administrative assistant.

PATHOLOGY:
Increased mitotic figures on biopsy

OTHER STUDIES:
Molecular studies demonstrate intact p53 and p21, but accelerated entry into mitosis with persistently elevated mitotic cyclin levels. Flow cytometry reveals a normal G1 and S-phase population, but an abnormally shortened G2 phase. Further investigation shows dysregulated cyclin gene transcription just prior to mitotic entry.

## Encounter 1: source 2021-07-15, practice 2025-10-20, ed, Emergency Medicine

Chief complaint: Progressive dysphagia to solids and liquids with 10-pound unintentional weight loss over 3 months

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Kwame Okafor, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive dysphagia to solids and liquids with 10-pound unintentional weight loss over 3 months

HISTORY OF PRESENT ILLNESS:
Patient with Hodgkin lymphoma and ovarian carcinoma presents with dysphagia initially to solids and now to liquids. She also reports a 10-pound weight loss during this period.

PAST MEDICAL HISTORY:
Active Problem List:
- Malignant neoplasm of ovary (diagnosed 2020-01-15)
- History of Hodgkin lymphoma in remission
- Post-radiation hypothyroidism
- Chemotherapy-induced peripheral neuropathy
- Chronic pain syndrome
- Tobacco use disorder
- Generalized anxiety disorder

She has a past medical history significant for Hodgkin lymphoma, for which she received chemotherapy and radiation therapy 15 years ago. She has been in remission since then.

PAST SURGICAL HISTORY:
- Staging laparotomy with splenectomy (15 years prior)
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy and optimal tumor debulking
- Port-a-cath placement (right subclavian)

MEDICATIONS:
Home Medications:
- Levothyroxine 125 mcg PO daily
- Gabapentin 300 mg PO TID
- Oxycodone 5 mg PO q6h PRN pain
- Ondansetron 8 mg PO q8h PRN nausea
- Pantoprazole 40 mg PO daily
- Lorazepam 0.5 mg PO BID PRN anxiety

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer at age 54
- Maternal aunt: Ovarian cancer at age 61
- Father: Chronic obstructive pulmonary disease
- Paternal grandfather: Pancreatic cancer

SOCIAL HISTORY:
She has a 30-pack-year smoking history.

PHYSICAL EXAMINATION:
On physical examination, she appears cachectic. There is no palpable lymphadenopathy.

LABORATORY DATA:
Laboratory studies are unremarkable.

IMAGING:
A barium swallow study shows an irregular narrowing of the distal esophagus.

PATHOLOGY:
Histopathology reveals a poorly differentiated adenocarcinoma.

OTHER STUDIES:
An esophagogastroduodenoscopy (EGD) with biopsy is performed,
