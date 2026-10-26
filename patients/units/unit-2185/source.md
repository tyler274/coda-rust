# synthetic_hospital patient 2185

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 68 at first encounter, sex M, Non-Hispanic White, Medicare with supplemental private insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-02-10 (Fri) | outpatient | Gastroenterology |
| 1 | 2020-02-15 | 2023-03-22 (Wed) | inpatient | General Surgery |
| 2 | 2020-02-15 | 2023-03-22 (Wed) | ed | Emergency Medicine |
| 3 | 2020-02-15 | 2023-03-22 (Wed) | ed | Emergency Medicine |
| 4 | 2020-07-15 | 2023-08-15 (Tue) | icu | General Surgery |
| 5 | 2021-01-15 | 2024-02-17 (Sat) | inpatient | General Surgery |
| 6 | 2021-01-15 | 2024-02-17 (Sat) | ed | Emergency Medicine |
| 7 | 2023-01-15 | 2026-02-04 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare with supplemental private insurance",
 "occupation": "Retired construction foreman",
 "alcohol_use": "None",
 "family_history": [
  "Father: Colorectal cancer diagnosed age 72",
  "Mother: Hypertension and chronic bronchitis",
  "Maternal grandfather: Type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 30 pack-years, quit 10 years ago",
 "home_medications": [
  {
   "dose": "18 mcg inhaled once daily",
   "name": "Tiotropium"
  },
  {
   "dose": "250/50 mcg inhaled twice daily",
   "name": "Fluticasone/Salmeterol"
  },
  {
   "dose": "90 mcg inhaled every 4-6 hours as needed for wheezing or shortness of breath",
   "name": "Albuterol"
  },
  {
   "dose": "10 mg oral once daily",
   "name": "Lisinopril"
  },
  {
   "dose": "25 mg oral once daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "1000 mg oral twice daily",
   "name": "Metformin"
  },
  {
   "dose": "500-1000 mg oral every 6 hours as needed for pain",
   "name": "Acetaminophen"
  }
 ],
 "surgical_history": [
  "Right hemicolectomy with ileocolic anastomosis for colonic adenocarcinoma",
  "Open cholecystectomy",
  "Laparoscopic cholecystectomy (converted from laparoscopic to open)"
 ],
 "chronic_conditions": [
  "Chronic obstructive pulmonary disease (COPD)",
  "Hypertension",
  "Type 2 diabetes mellitus",
  "Right-sided colon cancer (adenocarcinoma)"
 ],
 "age_at_first_encounter": 68
}
```

Primary diagnoses: ["Post-obstructive atelectasis", "Transfusion-related acute lung injury", "Opioid-induced respiratory depression", "Hypercapnic respiratory failure", "Right-sided colon cancer"]
Comorbidities: ["Chronic obstructive pulmonary disease", "Essential hypertension", "Type 2 diabetes mellitus", "Status post laparoscopic cholecystectomy", "Right-sided colon cancer"]

## Encounter 0: source 2020-01-15, practice 2023-02-10, outpatient, Gastroenterology

Chief complaint: Six-month history of fatigue, shortness of breath on exertion, and 10-pound unintentional weight loss.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of fatigue, shortness of breath on exertion, and 10-pound unintentional weight loss.

HISTORY OF PRESENT ILLNESS:
He reports a 10-pound unintentional weight loss over the same period but denies any fever or night sweats.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (COPD)
- Hypertension
- Type 2 diabetes mellitus
- Right-sided colon cancer (adenocarcinoma)

His past medical history is significant for hypertension, controlled with hydrochlorothiazide.

PAST SURGICAL HISTORY:
- Right hemicolectomy with ileocolic anastomosis for colonic adenocarcinoma
- Open cholecystectomy
- Laparoscopic cholecystectomy (converted from laparoscopic to open)

MEDICATIONS:
He takes hydrochlorothiazide.

Home Medications:
- Tiotropium 18 mcg inhaled once daily
- Fluticasone/Salmeterol 250/50 mcg inhaled twice daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed for wheezing or shortness of breath
- Lisinopril 10 mg oral once daily
- Metformin 1000 mg oral twice daily
- Acetaminophen 500-1000 mg oral every 6 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
He has no family history of cancer.

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 10 years ago. Alcohol: None. Occupation: Retired construction foreman.

PHYSICAL EXAMINATION:
On physical examination, he appears pale and has a systolic ejection murmur. Rectal examination reveals no masses, but the stool guaiac test is positive.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 9.2 g/dL (Normal: 13.8-17.2 g/dL)
- Mean corpuscular volume (MCV): 72 fL (Normal: 80-96 fL)
- Serum iron: 30 µg/dL (Normal: 65-176 µg/dL)
- Total iron-binding capacity (TIBC): 400 µg/dL (Normal: 250-450 µg/dL)
- Ferritin: 20 ng/mL (Normal: 24-336 ng/mL)

## Encounter 1: source 2020-02-15, practice 2023-03-22, inpatient, General Surgery

Chief complaint: Admitted for elective right hemicolectomy; postoperative day 1 with surgical site pain and reluctance to deep breathe or cough.

ENCOUNTER NOTE
Date: 2020-02-15
Provider: Dr. Michael Rodriguez, MD
Department: General Surgery
Visit Type: inpatient

CHIEF COMPLAINT:
Admitted for elective right hemicolectomy; postoperative day 1 with surgical site pain and reluctance to deep breathe or cough.

HISTORY OF PRESENT ILLNESS:
The patient is a 68-year-old male with COPD, hypertension, and type 2 diabetes mellitus who was diagnosed with right-sided colon cancer one month prior and returns on postoperative day 1 following right hemicolectomy. The surgery is uneventful, and he is transferred to postoperative care. He complains of moderate pain at the surgical site, rating 6 out of 10 on the pain scale. He is reluctant to take deep breaths or cough due to the pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Right-sided colon cancer (diagnosed 2020-01-15)
- Chronic obstructive pulmonary disease (COPD)
- Hypertension
- Type 2 diabetes mellitus
- Right-sided colon cancer (adenocarcinoma)

PAST SURGICAL HISTORY:
- Right hemicolectomy with ileocolic anastomosis for colonic adenocarcinoma
- Open cholecystectomy
- Laparoscopic cholecystectomy (converted from laparoscopic to open)

MEDICATIONS:
His current medications include acetaminophen and a salbutamol inhaler as needed.

Home Medications:
- Tiotropium 18 mcg inhaled once daily
- Fluticasone/Salmeterol 250/50 mcg inhaled twice daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed for wheezing or shortness of breath
- Lisinopril 10 mg oral once daily
- Hydrochlorothiazide 25 mg oral once daily
- Metformin 1000 mg oral twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Colorectal cancer diagnosed age 72
- Mother: Hypertension and chronic bronchitis
- Maternal grandfather: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 10 years ago. Alcohol: None. Occupation: Retired construction foreman.

VITAL SIGNS:
His temperature is 37.8°C (100°F), blood pressure is 135/85 mmHg, pulse is 90/min, and respiratory rate is 22/min. His oxygen saturation is 92% on room air.

PHYSICAL EXAMINATION:
On auscultation, there are decreased breath sounds at the left lung base without wheezes, rales, or rhonchi.

LABORATORY DATA:
Laboratory Values:
- White blood cell count: 11,000/uL
- Hemoglobin: 13.5 g/dL
- Platelet count: 250,000/uL
- Serum creatinine: 1.0 mg/dL
- Blood urea nitrogen: 18 mg/dL

IMAGING:
Chest X-ray shows a small left lower lobe atelectasis.

## Encounter 2: source 2020-02-15, practice 2023-03-22, ed, Emergency Medicine

Chief complaint: Fever of 38.5°C and chills approximately 4 hours after undergoing right hemicolectomy with intraoperative transfusion of packed red blood cells.

ENCOUNTER NOTE
Date: 2020-02-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fever of 38.5°C and chills approximately 4 hours after undergoing right hemicolectomy with intraoperative transfusion of packed red blood cells.

HISTORY OF PRESENT ILLNESS:
68-year-old with COPD, hypertension, diabetes, colon cancer presents approximately 4 hours after undergoing a right hemicolectomy for colon cancer. During the operation, he received two units of packed red blood cells due to intraoperative blood loss.

PAST MEDICAL HISTORY:
Active Problem List:
- Right-sided colon cancer (diagnosed 2020-01-15)
- Post-obstructive atelectasis (diagnosed 2020-02-15)
- Chronic obstructive pulmonary disease (COPD)
- Hypertension
- Type 2 diabetes mellitus
- Right-sided colon cancer (adenocarcinoma)

His past medical history is significant for hypertension, controlled with hydrochlorothiazide, and type 2 diabetes mellitus, managed with metformin.

PAST SURGICAL HISTORY:
- Right hemicolectomy with ileocolic anastomosis for colonic adenocarcinoma
- Open cholecystectomy
- Laparoscopic cholecystectomy (converted from laparoscopic to open)

MEDICATIONS:
Home Medications:
- Tiotropium 18 mcg inhaled once daily
- Fluticasone/Salmeterol 250/50 mcg inhaled twice daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed for wheezing or shortness of breath
- Lisinopril 10 mg oral once daily
- Hydrochlorothiazide 25 mg oral once daily
- Metformin 1000 mg oral twice daily
- Acetaminophen 500-1000 mg oral every 6 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Colorectal cancer diagnosed age 72
- Mother: Hypertension and chronic bronchitis
- Maternal grandfather: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 10 years ago. Alcohol: None. Occupation: Retired construction foreman.

VITAL SIGNS:
On examination, his blood pressure is 135/85 mmHg, heart rate is 98 beats per minute, and respiratory rate is 20 breaths per minute.

PHYSICAL EXAMINATION:
His surgical site appears clean without erythema or discharge. Auscultation of the lungs reveals clear breath sounds bilaterally. The remainder of the physical examination is unremarkable.

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 12,000/µL
- Hemoglobin: 10.2 g/dL (postoperative baseline: 9.8 g/dL)
- Platelet count: 250,000/µL
- Serum creatinine: 1.0 mg/dL
- Lactate dehydrogenase (LDH): Normal
- Direct antiglobulin test (DAT): Negative
- Urinalysis: Normal

## Encounter 3: source 2020-02-15, practice 2023-03-22, ed, Emergency Medicine

Chief complaint: Increasing shortness of breath and productive cough with white sputum on postoperative day 3 after right hemicolectomy.

ENCOUNTER NOTE
Date: 2020-02-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Increasing shortness of breath and productive cough with white sputum on postoperative day 3 after right hemicolectomy.

HISTORY OF PRESENT ILLNESS:
The patient is a 68-year-old male with COPD, hypertension, diabetes, and colon cancer s/p right hemicolectomy 3 days prior presenting to the emergency department on postoperative day 3. He denies chest pain, fever, or chills.

PAST MEDICAL HISTORY:
Active Problem List:
- Right-sided colon cancer (diagnosed 2020-01-15)
- Post-obstructive atelectasis (diagnosed 2020-02-15)
- Transfusion-related acute lung injury (diagnosed 2020-02-15)
- Chronic obstructive pulmonary disease (COPD)
- Hypertension
- Type 2 diabetes mellitus
- Right-sided colon cancer (adenocarcinoma)

PAST SURGICAL HISTORY:
- Right hemicolectomy with ileocolic anastomosis for colonic adenocarcinoma
- Open cholecystectomy
- Laparoscopic cholecystectomy (converted from laparoscopic to open)

MEDICATIONS:
He takes lisinopril.

Home Medications:
- Tiotropium 18 mcg inhaled once daily
- Fluticasone/Salmeterol 250/50 mcg inhaled twice daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed for wheezing or shortness of breath
- Hydrochlorothiazide 25 mg oral once daily
- Metformin 1000 mg oral twice daily
- Acetaminophen 500-1000 mg oral every 6 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Colorectal cancer diagnosed age 72
- Mother: Hypertension and chronic bronchitis
- Maternal grandfather: Type 2 diabetes mellitus

SOCIAL HISTORY:
30-pack-year smoking history, although he quit smoking 10 years ago

VITAL SIGNS:
Temperature is 37.2°C (99°F), blood pressure is 145/90 mmHg, pulse is 102 beats/min, and respiratory rate is 28 breaths/min. Oxygen saturation is 89% on room air.

PHYSICAL EXAMINATION:
Auscultation of the lungs reveals decreased breath sounds at the right lung base with fine crackles. The remainder of the physical examination is unremarkable.

LABORATORY DATA:
Arterial blood gas (ABG) shows:

- pH: 7.49
- PaCO2: 28 mmHg
- PaO2: 60 mmHg
- HCO3-: 24 mEq/L

IMAGING:
Chest X-ray demonstrates right lower lobe collapse.

## Encounter 4: source 2020-07-15, practice 2023-08-15, icu, General Surgery

Chief complaint: Unresponsive with shallow, infrequent breathing two hours after elective laparoscopic cholecystectomy.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Aisha Washington, MD
Department: General Surgery
Visit Type: icu

CHIEF COMPLAINT:
Unresponsive with shallow, infrequent breathing two hours after elective laparoscopic cholecystectomy.

HISTORY OF PRESENT ILLNESS:
The 68-year-old male with COPD, hypertension, type 2 diabetes, and colon cancer s/p hemicolectomy undergoes a 3-hour elective laparoscopic cholecystectomy under general anesthesia. Postoperatively, the patient is transferred to the recovery room. Two hours after the surgery, the patient remains unresponsive and exhibits shallow, infrequent breathing.

PAST MEDICAL HISTORY:
Active Problem List:
- Right-sided colon cancer (diagnosed 2020-01-15)
- Post-obstructive atelectasis (diagnosed 2020-02-15)
- Transfusion-related acute lung injury (diagnosed 2020-02-15)
- Chronic obstructive pulmonary disease (COPD)
- Hypertension
- Type 2 diabetes mellitus
- Right-sided colon cancer (adenocarcinoma)

PAST SURGICAL HISTORY:
- Right hemicolectomy with ileocolic anastomosis for colonic adenocarcinoma
- Open cholecystectomy
- Laparoscopic cholecystectomy (converted from laparoscopic to open)

MEDICATIONS:
Home Medications:
- Tiotropium 18 mcg inhaled once daily
- Fluticasone/Salmeterol 250/50 mcg inhaled twice daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed for wheezing or shortness of breath
- Lisinopril 10 mg oral once daily
- Hydrochlorothiazide 25 mg oral once daily
- Metformin 1000 mg oral twice daily
- Acetaminophen 500-1000 mg oral every 6 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Colorectal cancer diagnosed age 72
- Mother: Hypertension and chronic bronchitis
- Maternal grandfather: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 10 years ago. Alcohol: None. Occupation: Retired construction foreman.

VITAL SIGNS:
BP 110/70 mmHg, HR 55 bpm, RR 6/min, and SpO2 88% on room air

PHYSICAL EXAMINATION:
Physical examination shows a patient who is unarousable with pinpoint pupils and decreased breath sounds bilaterally without wheezing.

LABORATORY DATA:
Arterial blood gas analysis reveals pH 7.28, pCO2 60 mmHg, and pO2 55 mmHg

## Encounter 5: source 2021-01-15, practice 2024-02-17, inpatient, General Surgery

Chief complaint: Difficulty breathing and mild chest discomfort on postoperative day 1 following elective open cholecystectomy.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. David Kim, MD
Department: General Surgery
Visit Type: inpatient

CHIEF COMPLAINT:
Difficulty breathing and mild chest discomfort on postoperative day 1 following elective open cholecystectomy.

HISTORY OF PRESENT ILLNESS:
The patient is a 69-year-old male with a history of right-sided colon cancer status post hemicolectomy, chronic obstructive pulmonary disease, and prior postoperative pulmonary complications including recurrent atelectasis and respiratory depression following abdominal surgeries. He presents to the postoperative care unit following a 4-hour elective open cholecystectomy. He is currently on postoperative day 1. His postoperative pain is well-controlled with scheduled analgesics.

PAST MEDICAL HISTORY:
Active Problem List:
- Right-sided colon cancer (diagnosed 2020-01-15)
- Post-obstructive atelectasis (diagnosed 2020-02-15)
- Transfusion-related acute lung injury (diagnosed 2020-02-15)
- Opioid-induced respiratory depression (diagnosed 2020-07-15)
- Chronic obstructive pulmonary disease (COPD)
- Hypertension
- Type 2 diabetes mellitus
- Right-sided colon cancer (adenocarcinoma)

PAST SURGICAL HISTORY:
- Right hemicolectomy with ileocolic anastomosis for colonic adenocarcinoma
- Open cholecystectomy
- Laparoscopic cholecystectomy (converted from laparoscopic to open)

MEDICATIONS:
Home Medications:
- Tiotropium 18 mcg inhaled once daily
- Fluticasone/Salmeterol 250/50 mcg inhaled twice daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed for wheezing or shortness of breath
- Lisinopril 10 mg oral once daily
- Hydrochlorothiazide 25 mg oral once daily
- Metformin 1000 mg oral twice daily
- Acetaminophen 500-1000 mg oral every 6 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Colorectal cancer diagnosed age 72
- Mother: Hypertension and chronic bronchitis
- Maternal grandfather: Type 2 diabetes mellitus

SOCIAL HISTORY:
30-pack-year smoking history

VITAL SIGNS:
Temperature is 37.8°C, blood pressure is 145/90 mmHg, heart rate is 102 beats per minute, respiratory rate is 26 breaths per minute, and oxygen saturation is 92% on room air

PHYSICAL EXAMINATION:
Decreased breath sounds at the left lung base with dullness to percussion

LABORATORY DATA:
White blood cell count: 12,000/mm³
- Hemoglobin: 13.5 g/dL
- Platelet count: 250,000/mm³
- Serum sodium: 140 mEq/L
- Serum potassium: 4.2 mEq/L
- Serum creatinine: 1.0 mg/dL
- Blood urea nitrogen: 18 mg/dL

IMAGING:
Chest X-ray shows blunting of the left costophrenic angle.

## Encounter 6: source 2021-01-15, practice 2024-02-17, ed, Emergency Medicine

Chief complaint: Difficulty breathing and mild chest pain on postoperative day 2 after open cholecystectomy, reluctant to move due to pain.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Maria Gonzalez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Difficulty breathing and mild chest pain on postoperative day 2 after open cholecystectomy, reluctant to move due to pain.

HISTORY OF PRESENT ILLNESS:
The patient with COPD, colon cancer, and history of postoperative atelectasis presents to the hospital on postoperative day 2 after an open cholecystectomy. The patient has been reluctant to move due to pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Right-sided colon cancer (diagnosed 2020-01-15)
- Post-obstructive atelectasis (diagnosed 2020-02-15)
- Transfusion-related acute lung injury (diagnosed 2020-02-15)
- Opioid-induced respiratory depression (diagnosed 2020-07-15)
- Chronic obstructive pulmonary disease (COPD)
- Hypertension
- Type 2 diabetes mellitus
- Right-sided colon cancer (adenocarcinoma)

He has a history of hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Right hemicolectomy with ileocolic anastomosis for colonic adenocarcinoma
- Open cholecystectomy
- Laparoscopic cholecystectomy (converted from laparoscopic to open)

MEDICATIONS:
His medications include metformin and lisinopril.

Home Medications:
- Tiotropium 18 mcg inhaled once daily
- Fluticasone/Salmeterol 250/50 mcg inhaled twice daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed for wheezing or shortness of breath
- Hydrochlorothiazide 25 mg oral once daily
- Acetaminophen 500-1000 mg oral every 6 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Colorectal cancer diagnosed age 72
- Mother: Hypertension and chronic bronchitis
- Maternal grandfather: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 10 years ago. Alcohol: None. Occupation: Retired construction foreman.

VITAL SIGNS:
On examination, his temperature is 37.8°C (100°F), blood pressure is 145/90 mmHg, pulse is 98/min, and respiratory rate is 22/min. Oxygen saturation is 92% on room air.

PHYSICAL EXAMINATION:
The patient appears uncomfortable and is taking shallow breaths. Auscultation of the lungs reveals decreased breath sounds at the left lung base. The abdominal incision is clean, dry, and intact with moderate tenderness on palpation.

IMAGING:
Chest X-ray shows a small left lower lobe infiltrate with elevation of the left hemidiaphragm.

## Encounter 7: source 2023-01-15, practice 2026-02-04, ed, Emergency Medicine

Chief complaint: Found unresponsive at home after one week of increasing lethargy and progressive difficulty breathing.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Robert Singh, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Found unresponsive at home after one week of increasing lethargy and progressive difficulty breathing.

HISTORY OF PRESENT ILLNESS:
This 71-year-old male with COPD and respiratory complications presents. His family reports that he has been increasingly lethargic over the past week and has had difficulty breathing today.

PAST MEDICAL HISTORY:
Active Problem List:
- Right-sided colon cancer (diagnosed 2020-01-15)
- Post-obstructive atelectasis (diagnosed 2020-02-15)
- Transfusion-related acute lung injury (diagnosed 2020-02-15)
- Opioid-induced respiratory depression (diagnosed 2020-07-15)
- Chronic obstructive pulmonary disease (COPD)
- Hypertension
- Type 2 diabetes mellitus
- Right-sided colon cancer (adenocarcinoma)

PAST SURGICAL HISTORY:
- Right hemicolectomy with ileocolic anastomosis for colonic adenocarcinoma
- Open cholecystectomy
- Laparoscopic cholecystectomy (converted from laparoscopic to open)

MEDICATIONS:
His medications include tiotropium, fluticasone/salmeterol, and lisinopril.

Home Medications:
- Albuterol 90 mcg inhaled every 4-6 hours as needed for wheezing or shortness of breath
- Hydrochlorothiazide 25 mg oral once daily
- Metformin 1000 mg oral twice daily
- Acetaminophen 500-1000 mg oral every 6 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Colorectal cancer diagnosed age 72
- Mother: Hypertension and chronic bronchitis
- Maternal grandfather: Type 2 diabetes mellitus

SOCIAL HISTORY:
The patient's daughter provides a document that she found in his belongings, which appears to be a living will stating the patient's desire to avoid mechanical ventilation or resuscitative efforts in the event of severe illness. The family is divided on whether to honor the document, with some members insisting on full resuscitative measures.

VITAL SIGNS:
His vitals are: BP 90/60 mmHg, HR 118 bpm, RR 30/min, T 37.8°C, and SpO2 88% on room air.

PHYSICAL EXAMINATION:
On examination, the patient is unresponsive with a Glasgow Coma Scale of 6. Physical examination reveals bilateral wheezes and decreased breath sounds.

LABORATORY DATA:
Arterial blood gas shows pH 7.28, PaCO2 60 mmHg, PaO2 55 mmHg, and HCO3- 24 mEq/L.
