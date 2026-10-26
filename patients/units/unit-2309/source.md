# synthetic_hospital patient 2309

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 64 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-08-24 (Thu) | outpatient | Gastroenterology |
| 1 | 2020-07-15 | 2024-02-23 (Fri) | outpatient | Hematology and Oncology |
| 2 | 2021-07-15 | 2025-02-24 (Mon) | outpatient | Neurology |
| 3 | 2022-01-15 | 2025-08-27 (Wed) | outpatient | Hematology and Oncology |
| 4 | 2022-07-15 | 2026-02-13 (Fri) | inpatient | Hematology and Oncology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired construction foreman",
 "alcohol_use": "Social, 2-3 drinks per week",
 "family_history": [
  "Father: colon cancer diagnosed age 68, deceased",
  "Mother: breast cancer diagnosed age 72",
  "Older brother: lung cancer"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 50 pack-year history",
 "home_medications": [
  {
   "dose": "15 mg PO TID",
   "name": "Amifampridine"
  },
  {
   "dose": "60 mg PO TID",
   "name": "Pyridostigmine"
  },
  {
   "dose": "17 g PO daily",
   "name": "Polyethylene glycol 3350"
  },
  {
   "dose": "8.6 mg PO BID PRN",
   "name": "Senna"
  },
  {
   "dose": "5 mg PO q6h PRN pain",
   "name": "Oxycodone"
  },
  {
   "dose": "25 mg PO PRN",
   "name": "Sildenafil"
  }
 ],
 "surgical_history": [
  "Right hemicolectomy with ileocolic anastomosis",
  "Port-a-cath placement"
 ],
 "chronic_conditions": [
  "Right-sided colon adenocarcinoma",
  "Metastatic small cell lung cancer",
  "Metastatic pancreatic adenocarcinoma",
  "Lambert-Eaton myasthenic syndrome (paraneoplastic)",
  "Chronic constipation",
  "Erectile dysfunction"
 ],
 "age_at_first_encounter": 64
}
```

Primary diagnoses: ["Right-sided colon cancer", "Lambert-Eaton myasthenic syndrome", "Disseminated intravascular coagulation"]
Comorbidities: ["Cancer-related constitutional symptoms", "Tobacco use disorder"]

## Encounter 0: source 2020-01-15, practice 2023-08-24, outpatient, Gastroenterology

Chief complaint: Evaluation after screening colonoscopy

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Evaluation after screening colonoscopy

HISTORY OF PRESENT ILLNESS:
He reports occasional abdominal discomfort but no weight loss, bleeding, or change in bowel habits.

PAST MEDICAL HISTORY:
Active Problem List:
- Right-sided colon adenocarcinoma
- Metastatic small cell lung cancer
- Metastatic pancreatic adenocarcinoma
- Lambert-Eaton myasthenic syndrome (paraneoplastic)
- Chronic constipation
- Erectile dysfunction

PAST SURGICAL HISTORY:
- Right hemicolectomy with ileocolic anastomosis
- Port-a-cath placement

MEDICATIONS:
Home Medications:
- Amifampridine 15 mg PO TID
- Pyridostigmine 60 mg PO TID
- Polyethylene glycol 3350 17 g PO daily
- Senna 8.6 mg PO BID PRN
- Oxycodone 5 mg PO q6h PRN pain
- Sildenafil 25 mg PO PRN

ALLERGIES:
NKDA

FAMILY HISTORY:
No family history of colorectal disease

SOCIAL HISTORY:
Smoking: Current smoker, 50 pack-year history. Alcohol: Social, 2-3 drinks per week. Occupation: Retired construction foreman.

IMAGING:
No distant metastases are identified on imaging.

PATHOLOGY:
Biopsy of the lesion shows glandular structures with marked cellular atypia, nuclear pleomorphism, increased mitotic activity, and invasion into the submucosa. The lesion is surrounded by a tubulovillous adenomatous component.

OTHER STUDIES:
Colonoscopy reveals a 4-cm sessile lesion in the ascending colon.

## Encounter 1: source 2020-07-15, practice 2024-02-23, outpatient, Hematology and Oncology

Chief complaint: Progressive weight loss and fatigue over the past three months

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Rodriguez, MD
Department: Hematology and Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive weight loss and fatigue over the past three months

HISTORY OF PRESENT ILLNESS:
The patient returns for Hematology and Oncology follow-up of right-sided colon cancer diagnosed in January 2020. He notes a dull abdominal discomfort and occasional constipation, but no blood in the stool or melena. His appetite has been poor, with unintentional loss of 7 kg (15 lb).

PAST MEDICAL HISTORY:
Active Problem List:
- Right-sided colon cancer (diagnosed 2020-01-15)
- Right-sided colon adenocarcinoma
- Metastatic small cell lung cancer
- Metastatic pancreatic adenocarcinoma
- Lambert-Eaton myasthenic syndrome (paraneoplastic)
- Chronic constipation
- Erectile dysfunction

PAST SURGICAL HISTORY:
- Right hemicolectomy with ileocolic anastomosis
- Port-a-cath placement

MEDICATIONS:
Home Medications:
- Amifampridine 15 mg PO TID
- Pyridostigmine 60 mg PO TID
- Polyethylene glycol 3350 17 g PO daily
- Senna 8.6 mg PO BID PRN
- Oxycodone 5 mg PO q6h PRN pain
- Sildenafil 25 mg PO PRN

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: colon cancer diagnosed age 68, deceased
- Mother: breast cancer diagnosed age 72
- Older brother: lung cancer

SOCIAL HISTORY:
He has a long history of cigarette smoking and drinks alcohol socially.

VITAL SIGNS:
Temperature is 37.0 °C (98.6 °F), blood pressure is 118/70 mmHg, pulse is 92/min, and respirations are 16/min.

PHYSICAL EXAMINATION:
On examination, he appears thin and tired but in no acute distress. The abdomen is soft with mild distension and diffuse tenderness but no rebound.

LABORATORY DATA:
Laboratory studies reveal iron deficiency anemia.

IMAGING:
CT scan of the abdomen shows an irregular mass in the ascending colon with evidence of spread to regional lymph nodes.

PATHOLOGY:
A biopsy of the lesion demonstrates malignant epithelial cells invading through the basement membrane into the surrounding tissue.

## Encounter 2: source 2021-07-15, practice 2025-02-24, outpatient, Neurology

Chief complaint: Progressive muscle weakness, fatigue, dry mouth, and difficulty climbing stairs

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Aisha Patel, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive muscle weakness, fatigue, dry mouth, and difficulty climbing stairs

HISTORY OF PRESENT ILLNESS:
The patient with right-sided colon adenocarcinoma evaluated in 2020 presents with a 3-month history of progressive muscle weakness, fatigue, dry mouth, and difficulty climbing stairs. He reports recent episodes of erectile dysfunction and constipation.

PAST MEDICAL HISTORY:
Active Problem List:
- Right-sided colon cancer (diagnosed 2020-01-15)
- Right-sided colon adenocarcinoma
- Metastatic small cell lung cancer
- Metastatic pancreatic adenocarcinoma
- Lambert-Eaton myasthenic syndrome (paraneoplastic)
- Chronic constipation
- Erectile dysfunction

PAST SURGICAL HISTORY:
- Right hemicolectomy with ileocolic anastomosis
- Port-a-cath placement

MEDICATIONS:
Home Medications:
- Amifampridine 15 mg PO TID
- Pyridostigmine 60 mg PO TID
- Polyethylene glycol 3350 17 g PO daily
- Senna 8.6 mg PO BID PRN
- Oxycodone 5 mg PO q6h PRN pain
- Sildenafil 25 mg PO PRN

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: colon cancer diagnosed age 68, deceased
- Mother: breast cancer diagnosed age 72
- Older brother: lung cancer

SOCIAL HISTORY:
50-pack-year smoking history

PHYSICAL EXAMINATION:
Neurologic examination reveals proximal muscle weakness and absent deep tendon reflexes. Sensory examination is normal. After repeated testing of muscle strength, his power improves noticeably with continued effort.

IMAGING:
A chest x-ray shows a right hilar mass.

## Encounter 3: source 2022-01-15, practice 2025-08-27, outpatient, Hematology and Oncology

Chief complaint: Metastatic lung cancer

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Marcus Rodriguez, MD
Department: Hematology and Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Metastatic lung cancer

PAST MEDICAL HISTORY:
Active Problem List:
- Right-sided colon cancer (diagnosed 2020-01-15)
- Lambert-Eaton myasthenic syndrome (diagnosed 2021-07-15)
- Right-sided colon adenocarcinoma
- Metastatic small cell lung cancer
- Metastatic pancreatic adenocarcinoma
- Lambert-Eaton myasthenic syndrome (paraneoplastic)
- Chronic constipation
- Erectile dysfunction

PAST SURGICAL HISTORY:
- Right hemicolectomy with ileocolic anastomosis
- Port-a-cath placement

MEDICATIONS:
Home Medications:
- Amifampridine 15 mg PO TID
- Pyridostigmine 60 mg PO TID
- Polyethylene glycol 3350 17 g PO daily
- Senna 8.6 mg PO BID PRN
- Oxycodone 5 mg PO q6h PRN pain
- Sildenafil 25 mg PO PRN

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: colon cancer diagnosed age 68, deceased
- Mother: breast cancer diagnosed age 72
- Older brother: lung cancer

SOCIAL HISTORY:
40-pack-year smoking history

ASSESSMENT AND PLAN:
Diagnosed with metastatic lung cancer

## Encounter 4: source 2022-07-15, practice 2026-02-13, inpatient, Hematology and Oncology

Chief complaint: Progressive weakness, spontaneous bruising, and intermittent confusion

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Marcus Rodriguez, MD
Department: Hematology and Oncology
Visit Type: inpatient

CHIEF COMPLAINT:
Progressive weakness, spontaneous bruising, and intermittent confusion

HISTORY OF PRESENT ILLNESS:
The patient is a 66-year-old male with a history of right-sided colon adenocarcinoma, metastatic small cell lung cancer, and Lambert-Eaton myasthenic syndrome returning for evaluation. He has experienced significant weight loss over the last several months and was recently diagnosed with metastatic pancreatic cancer. He has not started chemotherapy. Nursing staff report that over the past day, he has developed persistent oozing from IV catheter sites and several new dark patches on his lower extremities. The patient denies chest pain, trauma, or recent infections.

PAST MEDICAL HISTORY:
Active Problem List:
- Right-sided colon cancer (diagnosed 2020-01-15)
- Lambert-Eaton myasthenic syndrome (diagnosed 2021-07-15)
- Right-sided colon adenocarcinoma
- Metastatic small cell lung cancer
- Metastatic pancreatic adenocarcinoma
- Lambert-Eaton myasthenic syndrome (paraneoplastic)
- Chronic constipation
- Erectile dysfunction

He has no personal history of bleeding disorders.

PAST SURGICAL HISTORY:
- Right hemicolectomy with ileocolic anastomosis
- Port-a-cath placement

MEDICATIONS:
Home Medications:
- Amifampridine 15 mg PO TID
- Pyridostigmine 60 mg PO TID
- Polyethylene glycol 3350 17 g PO daily
- Senna 8.6 mg PO BID PRN
- Oxycodone 5 mg PO q6h PRN pain
- Sildenafil 25 mg PO PRN

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: colon cancer diagnosed age 68, deceased
- Mother: breast cancer diagnosed age 72
- Older brother: lung cancer

SOCIAL HISTORY:
Smoking: Current smoker, 50 pack-year history. Alcohol: Social, 2-3 drinks per week. Occupation: Retired construction foreman.

VITAL SIGNS:
On exam, his temperature is normal, blood pressure is 118/70 mm Hg, and heart rate is 96/min.

PHYSICAL EXAMINATION:
He is alert but slow to respond. Skin exam shows petechiae, scattered ecchymoses, and active bleeding from a venipuncture site. Neurologic exam is nonfocal.

LABORATORY DATA:
A peripheral smear is obtained and shown below.
