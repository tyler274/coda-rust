# synthetic_hospital patient 2314

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 67 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-06-20 (Tue) | inpatient | General Surgery |
| 1 | 2020-04-15 | 2023-09-27 (Wed) | inpatient | General Surgery |
| 2 | 2021-07-15 | 2025-01-02 (Thu) | inpatient | General Surgery |
| 3 | 2022-07-15 | 2025-12-27 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Penicillin (anaphylaxis, age 12)",
  "Sulfa drugs (rash)"
 ],
 "insurance": "Medicare",
 "occupation": "Retired postal worker",
 "alcohol_use": "Social, 1-2 beers on weekends",
 "family_history": [
  "Father: Colon cancer (diagnosed age 70, deceased)",
  "Mother: Rheumatoid arthritis, hypothyroidism",
  "Sister: Breast cancer (diagnosed age 62)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 12 years ago (30 pack-year history)",
 "home_medications": [
  {
   "dose": "20 mg PO every Friday",
   "name": "Methotrexate"
  },
  {
   "dose": "1 mg PO daily",
   "name": "Folic acid"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Prednisone"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "1000 mg PO twice daily",
   "name": "Metformin"
  },
  {
   "dose": "40 mg PO nightly",
   "name": "Atorvastatin"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  },
  {
   "dose": "650 mg PO every 6 hours PRN pain",
   "name": "Acetaminophen"
  }
 ],
 "surgical_history": [
  "Right hemicolectomy with ileocolic anastomosis",
  "Open cholecystectomy",
  "Exploratory laparotomy with Graham patch repair of perforated gastric ulcer",
  "Left knee arthroscopy with meniscectomy (age 55)"
 ],
 "chronic_conditions": [
  "Rheumatoid arthritis (25-year history, poorly controlled)",
  "History of stage II colon cancer (status post resection)",
  "Type 2 diabetes mellitus",
  "Essential hypertension",
  "Hyperlipidemia",
  "Cervical spondylosis"
 ],
 "age_at_first_encounter": 67
}
```

Primary diagnoses: ["Post-obstructive atelectasis", "Prerenal acute kidney injury secondary to hypovolemia", "Cervical myelopathy due to atlantoaxial subluxation"]
Comorbidities: ["Colorectal neoplasia", "Status post laparoscopic cholecystectomy"]

## Encounter 0: source 2020-01-15, practice 2023-06-20, inpatient, General Surgery

Chief complaint: Mild shortness of breath and fatigue one day after undergoing open cholecystectomy for acute cholecystitis

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: General Surgery
Visit Type: inpatient

CHIEF COMPLAINT:
Mild shortness of breath and fatigue one day after undergoing open cholecystectomy for acute cholecystitis

HISTORY OF PRESENT ILLNESS:
He is evaluated in the hospital one day after undergoing an open cholecystectomy for acute cholecystitis. He has had no major complications since surgery but is reporting mild shortness of breath and fatigue. He is reluctant to take deep breaths because it “hurts to move the incision.” He has not been using the incentive spirometer provided by nursing staff.

PAST MEDICAL HISTORY:
Active Problem List:
- Rheumatoid arthritis (25-year history, poorly controlled)
- History of stage II colon cancer (status post resection)
- Type 2 diabetes mellitus
- Essential hypertension
- Hyperlipidemia
- Cervical spondylosis

PAST SURGICAL HISTORY:
- Right hemicolectomy with ileocolic anastomosis
- Open cholecystectomy
- Exploratory laparotomy with Graham patch repair of perforated gastric ulcer
- Left knee arthroscopy with meniscectomy (age 55)

MEDICATIONS:
Home Medications:
- Methotrexate 20 mg PO every Friday
- Folic acid 1 mg PO daily
- Prednisone 10 mg PO daily
- Lisinopril 20 mg PO daily
- Metformin 1000 mg PO twice daily
- Atorvastatin 40 mg PO nightly
- Aspirin 81 mg PO daily
- Acetaminophen 650 mg PO every 6 hours PRN pain

ALLERGIES:
Penicillin (anaphylaxis, age 12), Sulfa drugs (rash)

FAMILY HISTORY:
- Father: Colon cancer (diagnosed age 70, deceased)
- Mother: Rheumatoid arthritis, hypothyroidism
- Sister: Breast cancer (diagnosed age 62)

SOCIAL HISTORY:
Smoking: Former smoker, quit 12 years ago (30 pack-year history). Alcohol: Social, 1-2 beers on weekends. Occupation: Retired postal worker.

VITAL SIGNS:
His temperature is 37.4°C (99.3°F), blood pressure is 130/80 mmHg, heart rate is 88/min, and respiratory rate is 20/min. Oxygen saturation is 93% on room air.

PHYSICAL EXAMINATION:
On physical exam, he is alert and oriented. Breath sounds are mildly decreased at the bilateral lung bases. There are no wheezes or crackles.

IMAGING:
Chest x-ray shows subtle elevation of the right hemidiaphragm and patchy opacities in the right lower lobe without pleural effusion. There is no pneumothorax or consolidation.

PLAN:
He is advised to increase use of his incentive spirometer and begin walking with assistance.

## Encounter 1: source 2020-04-15, practice 2023-09-27, inpatient, General Surgery

Chief complaint: Fever of 38.2°C and mild surgical site pain 16 hours after right hemicolectomy for colon cancer

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: General Surgery
Visit Type: inpatient

CHIEF COMPLAINT:
Fever of 38.2°C and mild surgical site pain 16 hours after right hemicolectomy for colon cancer

HISTORY OF PRESENT ILLNESS:
This 67-year-old male with poorly controlled rheumatoid arthritis on methotrexate and prednisone, and prior post-obstructive atelectasis following cholecystectomy three months ago, presents 16 hours after right hemicolectomy with fever and surgical site pain. Procedure was uncomplicated, and the patient has been stable since extubation in the recovery room. Approximately 16 hours after surgery, his temperature is noted to be 38.2°C (100.8°F). He has mild pain at the surgical site but is otherwise alert and breathing comfortably. He has not ambulated yet and admits he has not been using incentive spirometer due to incisional discomfort.

PAST MEDICAL HISTORY:
Active Problem List:
- Post-obstructive atelectasis (diagnosed 2020-01-15)
- Rheumatoid arthritis (25-year history, poorly controlled)
- History of stage II colon cancer (status post resection)
- Type 2 diabetes mellitus
- Essential hypertension
- Hyperlipidemia
- Cervical spondylosis

PAST SURGICAL HISTORY:
Right hemicolectomy

MEDICATIONS:
Home Medications:
- Methotrexate 20 mg PO every Friday
- Folic acid 1 mg PO daily
- Prednisone 10 mg PO daily
- Lisinopril 20 mg PO daily
- Metformin 1000 mg PO twice daily
- Atorvastatin 40 mg PO nightly
- Aspirin 81 mg PO daily
- Acetaminophen 650 mg PO every 6 hours PRN pain

ALLERGIES:
Penicillin (anaphylaxis, age 12), Sulfa drugs (rash)

FAMILY HISTORY:
- Father: Colon cancer (diagnosed age 70, deceased)
- Mother: Rheumatoid arthritis, hypothyroidism
- Sister: Breast cancer (diagnosed age 62)

SOCIAL HISTORY:
Smoking: Former smoker, quit 12 years ago (30 pack-year history). Alcohol: Social, 1-2 beers on weekends. Occupation: Retired postal worker.

REVIEW OF SYSTEMS:
He denies chills, cough, or urinary symptoms.

VITAL SIGNS:
Temperature is noted to be 38.2°C (100.8°F). He is on room air and has an oxygen saturation of 95%.

PHYSICAL EXAMINATION:
Physical examination reveals decreased breath sounds at the right lung base but no crackles, wheezing, or rales. The surgical dressing is clean and dry with no surrounding erythema. Foley catheter is in place, draining clear yellow urine.

LABORATORY DATA:
Complete blood count is within normal limits.

IMAGING:
Chest x-ray reveals subtle elevation of the right hemidiaphragm with patchy opacity in the right lower lobe and no evidence of infiltrates or effusion.

## Encounter 2: source 2021-07-15, practice 2025-01-02, inpatient, General Surgery

Chief complaint: Oliguria on postoperative day 2 following emergent laparotomy for perforated gastric ulcer

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Priya Patel, MD
Department: General Surgery
Visit Type: inpatient

CHIEF COMPLAINT:
Oliguria on postoperative day 2 following emergent laparotomy for perforated gastric ulcer

HISTORY OF PRESENT ILLNESS:
The 68-year-old male with poorly controlled rheumatoid arthritis and prior postoperative atelectasis presents following emergent laparotomy. During surgery, he experiences significant intraoperative blood loss and transient hypotension with a systolic blood pressure of 70 mm Hg for several minutes. On postoperative day 2, he becomes oliguric.

PAST MEDICAL HISTORY:
Active Problem List:
- Post-obstructive atelectasis (diagnosed 2020-01-15)
- Rheumatoid arthritis (25-year history, poorly controlled)
- History of stage II colon cancer (status post resection)
- Type 2 diabetes mellitus
- Essential hypertension
- Hyperlipidemia
- Cervical spondylosis

PAST SURGICAL HISTORY:
- Right hemicolectomy with ileocolic anastomosis
- Open cholecystectomy
- Exploratory laparotomy with Graham patch repair of perforated gastric ulcer
- Left knee arthroscopy with meniscectomy (age 55)

MEDICATIONS:
Home Medications:
- Methotrexate 20 mg PO every Friday
- Folic acid 1 mg PO daily
- Prednisone 10 mg PO daily
- Lisinopril 20 mg PO daily
- Metformin 1000 mg PO twice daily
- Atorvastatin 40 mg PO nightly
- Aspirin 81 mg PO daily
- Acetaminophen 650 mg PO every 6 hours PRN pain

ALLERGIES:
Penicillin (anaphylaxis, age 12), Sulfa drugs (rash)

FAMILY HISTORY:
- Father: Colon cancer (diagnosed age 70, deceased)
- Mother: Rheumatoid arthritis, hypothyroidism
- Sister: Breast cancer (diagnosed age 62)

SOCIAL HISTORY:
Smoking: Former smoker, quit 12 years ago (30 pack-year history). Alcohol: Social, 1-2 beers on weekends. Occupation: Retired postal worker.

LABORATORY DATA:
His serum creatinine rises from 0.9 mg/dL preoperatively to 2.3 mg/dL. Urine sodium is 42 mEq/L.

## Encounter 3: source 2022-07-15, practice 2025-12-27, ed, Emergency Medicine

Chief complaint: Worsening difficulty walking, progressive leg weakness, urinary hesitancy, and ascending numbness with tight band sensation around abdomen

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. James Okonkwo, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening difficulty walking, progressive leg weakness, urinary hesitancy, and ascending numbness with tight band sensation around abdomen

HISTORY OF PRESENT ILLNESS:
The patient is a 69-year-old male with a 25-year history of poorly controlled rheumatoid arthritis, cervical spondylosis, and prior postoperative complications including atelectasis and acute kidney injury who presents to the Emergency Department with progressive neurological symptoms. He reports that over the past several days, he has experienced numbness beginning around his belly button, which has now spread downward to both legs. He also notes new-onset urinary hesitancy and a feeling of incomplete bladder emptying. His bowel movements have become less frequent, and he describes a dull, aching mid-to-lower back pain that is worse when lying flat.

PAST MEDICAL HISTORY:
Active Problem List:
- Post-obstructive atelectasis (diagnosed 2020-01-15)
- Prerenal acute kidney injury secondary to hypovolemia (diagnosed 2021-07-15)
- Rheumatoid arthritis (25-year history, poorly controlled)
- History of stage II colon cancer (status post resection)
- Type 2 diabetes mellitus
- Essential hypertension
- Hyperlipidemia
- Cervical spondylosis

PAST SURGICAL HISTORY:
- Right hemicolectomy with ileocolic anastomosis
- Open cholecystectomy
- Exploratory laparotomy with Graham patch repair of perforated gastric ulcer
- Left knee arthroscopy with meniscectomy (age 55)

MEDICATIONS:
Home Medications:
- Methotrexate 20 mg PO every Friday
- Folic acid 1 mg PO daily
- Prednisone 10 mg PO daily
- Lisinopril 20 mg PO daily
- Metformin 1000 mg PO twice daily
- Atorvastatin 40 mg PO nightly
- Aspirin 81 mg PO daily
- Acetaminophen 650 mg PO every 6 hours PRN pain

ALLERGIES:
Penicillin (anaphylaxis, age 12), Sulfa drugs (rash)

FAMILY HISTORY:
- Father: Colon cancer (diagnosed age 70, deceased)
- Mother: Rheumatoid arthritis, hypothyroidism
- Sister: Breast cancer (diagnosed age 62)

SOCIAL HISTORY:
Smoking: Former smoker, quit 12 years ago (30 pack-year history). Alcohol: Social, 1-2 beers on weekends. Occupation: Retired postal worker.

PHYSICAL EXAMINATION:
On neurologic examination, his upper extremity function is intact. However, he exhibits bilateral lower extremity weakness, hyperreflexia, and sustained ankle clonus. Babinski signs are present bilaterally. Sensory testing reveals diminished pinprick and temperature sensation beginning at the level of the umbilicus and extending caudally. Light touch, proprioception, and vibration are preserved. Rectal tone is reduced.

IMAGING:
An urgent MRI of the spine reveals a compressive epidural mass impinging on the spinal cord at a thoracic level.
