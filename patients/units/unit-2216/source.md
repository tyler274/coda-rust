# synthetic_hospital patient 2216

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 67 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-10-29 (Fri) | outpatient | Gastroenterology |
| 1 | 2020-07-15 | 2022-04-21 (Thu) | outpatient | Hematology & Oncology |
| 2 | 2021-07-15 | 2023-05-08 (Mon) | outpatient | Ophthalmology |
| 3 | 2022-01-15 | 2023-11-08 (Wed) | inpatient | Cardiology |
| 4 | 2022-07-15 | 2024-05-06 (Mon) | ed | Emergency Medicine |
| 5 | 2023-01-15 | 2024-10-30 (Wed) | ed | Emergency Medicine |
| 6 | 2023-07-15 | 2025-04-24 (Thu) | ed | Emergency Medicine |
| 7 | 2024-01-15 | 2025-10-30 (Thu) | icu | Critical Care Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare",
 "occupation": "Metal fabrication plant worker",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Father: Type 2 diabetes mellitus, myocardial infarction at age 72",
  "Mother: Hypertension, stroke at age 78",
  "Paternal uncle: Colorectal cancer diagnosed age 65",
  "No known family history of hematologic malignancies"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1000 mg orally twice daily",
   "name": "Metformin"
  },
  {
   "dose": "20 mg orally daily",
   "name": "Lisinopril"
  },
  {
   "dose": "25 mg orally daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "40 mg orally daily",
   "name": "Furosemide"
  },
  {
   "dose": "12.5 mg orally twice daily",
   "name": "Carvedilol"
  },
  {
   "dose": "500 mg orally daily",
   "name": "Hydroxyurea"
  },
  {
   "dose": "81 mg orally daily",
   "name": "Aspirin"
  },
  {
   "dose": "40 mg orally daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "50000 units orally weekly",
   "name": "Ergocalciferol"
  }
 ],
 "surgical_history": [
  "Colonoscopy with polypectomy",
  "Inguinal hernia repair"
 ],
 "chronic_conditions": [
  "Type 2 diabetes mellitus (15-year duration)",
  "Hypertension (history of poor control)",
  "Heart failure with reduced ejection fraction",
  "Polycythemia vera",
  "Atherosclerotic cardiovascular disease",
  "History of colorectal tubular adenoma with high-grade dysplasia"
 ],
 "age_at_first_encounter": 67
}
```

Primary diagnoses: ["Heart failure with reduced ejection fraction", "Bacterial meningitis due to Streptococcus pneumoniae", "Cholesterol embolism", "Tubular adenoma of descending colon with high-grade dysplasia", "Leukocytosis", "Polycythemia vera"]
Comorbidities: ["Upper respiratory tract infection", "Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2021-10-29, outpatient, Gastroenterology

Chief complaint: Intermittent rectal bleeding and change in bowel habits over 3 months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent rectal bleeding and change in bowel habits over 3 months

HISTORY OF PRESENT ILLNESS:
He presents to the clinic with a 3-month history of intermittent rectal bleeding and a recent change in bowel habits, including increased constipation. He denies any abdominal pain, weight loss

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (15-year duration)
- Hypertension (history of poor control)
- Heart failure with reduced ejection fraction
- Polycythemia vera
- Atherosclerotic cardiovascular disease
- History of colorectal tubular adenoma with high-grade dysplasia

His past medical history is significant for hypertension, controlled with lisinopril, and type 2 diabetes mellitus, managed with metformin.

PAST SURGICAL HISTORY:
- Colonoscopy with polypectomy
- Inguinal hernia repair

MEDICATIONS:
Home Medications:
- Metformin 1000 mg orally twice daily
- Lisinopril 20 mg orally daily
- Hydrochlorothiazide 25 mg orally daily
- Furosemide 40 mg orally daily
- Carvedilol 12.5 mg orally twice daily
- Hydroxyurea 500 mg orally daily
- Aspirin 81 mg orally daily
- Atorvastatin 40 mg orally daily
- Ergocalciferol 50000 units orally weekly

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
Family history of colorectal cancer

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Metal fabrication plant worker.

VITAL SIGNS:
On physical examination, his vital signs are within normal limits

PHYSICAL EXAMINATION:
Abdominal examination reveals no masses or tenderness. Digital rectal examination is notable for the presence of occult blood.

PATHOLOGY:
The pathology report indicates a tubular adenoma with high-grade dysplasia.

OTHER STUDIES:
Sigmoidoscopy reveals a 2 cm sessile polyp in the descending colon, which is biopsied.

## Encounter 1: source 2020-07-15, practice 2022-04-21, outpatient, Hematology & Oncology

Chief complaint: Six-month history of fatigue, pruritus after hot showers, ruddy complexion, and early satiety

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Priya Naidu, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of fatigue, pruritus after hot showers, ruddy complexion, and early satiety

HISTORY OF PRESENT ILLNESS:
The patient with polycythemia vera returns following his January gastroenterology evaluation. He reports a 6-month history of increasing fatigue, pruritus after hot showers, and a ruddy complexion. He notes left upper quadrant fullness and early satiety.

PAST MEDICAL HISTORY:
Active Problem List:
- Tubular adenoma of descending colon with high-grade dysplasia (diagnosed 2020-01-15)
- Type 2 diabetes mellitus (15-year duration)
- Hypertension (history of poor control)
- Heart failure with reduced ejection fraction
- Polycythemia vera
- Atherosclerotic cardiovascular disease
- History of colorectal tubular adenoma with high-grade dysplasia

His past medical history is significant for type 2 diabetes mellitus and hypertension, which are well-controlled with metformin and lisinopril, respectively.

PAST SURGICAL HISTORY:
- Colonoscopy with polypectomy
- Inguinal hernia repair

MEDICATIONS:
He takes metformin and lisinopril.

Home Medications:
- Hydrochlorothiazide 25 mg orally daily
- Furosemide 40 mg orally daily
- Carvedilol 12.5 mg orally twice daily
- Hydroxyurea 500 mg orally daily
- Aspirin 81 mg orally daily
- Atorvastatin 40 mg orally daily
- Ergocalciferol 50000 units orally weekly

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 72
- Mother: Hypertension, stroke at age 78
- Paternal uncle: Colorectal cancer diagnosed age 65
- No known family history of hematologic malignancies

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Metal fabrication plant worker.

PHYSICAL EXAMINATION:
On physical examination, his spleen is palpable 3 cm below the left costal margin. His skin appears plethoric. There is no lymphadenopathy.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 18.5 g/dL (Normal: 13.8-17.2 g/dL)
- Hematocrit: 55% (Normal: 40.7-50.3%)
- Mean corpuscular volume (MCV): 89 fL (Normal: 80-96 fL)
- White blood cell count: 12,000 /µL (Normal: 4,500-11,000 /µL)
- Platelet count: 450,000 /µL (Normal: 150,000-450,000 /µL)
- Serum erythropoietin level: Low

## Encounter 2: source 2021-07-15, practice 2023-05-08, outpatient, Ophthalmology

Chief complaint: Scheduled ophthalmic assessment revealing retinal findings

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Ophthalmology
Visit Type: outpatient

CHIEF COMPLAINT:
Scheduled ophthalmic assessment revealing retinal findings

HISTORY OF PRESENT ILLNESS:
The patient with atherosclerotic cardiovascular disease and polycythemia vera returns for ophthalmic assessment. He denies acute vision changes, eye pain, or trauma. He reports strict adherence to antihypertensive therapy but admits to occasional hyperglycemia.

PAST MEDICAL HISTORY:
Active Problem List:
- Tubular adenoma of descending colon with high-grade dysplasia (diagnosed 2020-01-15)
- Polycythemia vera (diagnosed 2020-07-15)
- Type 2 diabetes mellitus (15-year duration)
- Hypertension (history of poor control)
- Heart failure with reduced ejection fraction
- Atherosclerotic cardiovascular disease
- History of colorectal tubular adenoma with high-grade dysplasia

PAST SURGICAL HISTORY:
- Colonoscopy with polypectomy
- Inguinal hernia repair

MEDICATIONS:
Home Medications:
- Metformin 1000 mg orally twice daily
- Lisinopril 20 mg orally daily
- Hydrochlorothiazide 25 mg orally daily
- Furosemide 40 mg orally daily
- Carvedilol 12.5 mg orally twice daily
- Hydroxyurea 500 mg orally daily
- Aspirin 81 mg orally daily
- Atorvastatin 40 mg orally daily
- Ergocalciferol 50000 units orally weekly

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 72
- Mother: Hypertension, stroke at age 78
- Paternal uncle: Colorectal cancer diagnosed age 65
- No known family history of hematologic malignancies

SOCIAL HISTORY:
He works in a metal fabrication plant, where he is frequently exposed to metal shavings and dust.

PHYSICAL EXAMINATION:
On fundoscopic examination, a single glistening, refractile lesion is noted in the right retinal artery. His exam is otherwise unremarkable. There are no signs of conjunctival injection. The ophthalmologist notes mild arteriolar narrowing.

## Encounter 3: source 2022-01-15, practice 2023-11-08, inpatient, Cardiology

Chief complaint: Progressive shortness of breath, orthopnea, and lower extremity edema with 10-pound weight gain over two weeks

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. David Kim, MD
Department: Cardiology
Visit Type: inpatient

CHIEF COMPLAINT:
Progressive shortness of breath, orthopnea, and lower extremity edema with 10-pound weight gain over two weeks

HISTORY OF PRESENT ILLNESS:
This 69-year-old male with heart failure with reduced ejection fraction and polycythemia vera presents to the emergency department with progressive shortness of breath, orthopnea, and lower extremity edema over the past two weeks. He reports a 10-pound weight gain during this period.

PAST MEDICAL HISTORY:
Active Problem List:
- Tubular adenoma of descending colon with high-grade dysplasia (diagnosed 2020-01-15)
- Polycythemia vera (diagnosed 2020-07-15)
- Cholesterol embolism (diagnosed 2021-07-15)
- Type 2 diabetes mellitus (15-year duration)
- Hypertension (history of poor control)
- Heart failure with reduced ejection fraction
- Atherosclerotic cardiovascular disease
- History of colorectal tubular adenoma with high-grade dysplasia

PAST SURGICAL HISTORY:
- Colonoscopy with polypectomy
- Inguinal hernia repair

MEDICATIONS:
He takes metformin, lisinopril, and hydrochlorothiazide.

Home Medications:
- Furosemide 40 mg orally daily
- Carvedilol 12.5 mg orally twice daily
- Hydroxyurea 500 mg orally daily
- Aspirin 81 mg orally daily
- Atorvastatin 40 mg orally daily
- Ergocalciferol 50000 units orally weekly

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 72
- Mother: Hypertension, stroke at age 78
- Paternal uncle: Colorectal cancer diagnosed age 65
- No known family history of hematologic malignancies

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Metal fabrication plant worker.

VITAL SIGNS:
Blood pressure is 150/90 mmHg, heart rate is 102 beats per minute, respiratory rate is 22 breaths per minute, and oxygen saturation is 90% on room air

PHYSICAL EXAMINATION:
Jugular venous pressure is elevated. Cardiac auscultation reveals an S3 gallop and bibasilar crackles are heard on lung auscultation. Extremities show 2+ pitting edema up to the mid-shins.

LABORATORY DATA:
Laboratory Values:
- Sodium: 138 mEq/L
- Potassium: 4.5 mEq/L
- Creatinine: 1.2 mg/dL
- B-type natriuretic peptide (BNP): 900 pg/mL
- Hemoglobin A1c: 7.2%

IMAGING:
Chest X-ray shows cardiomegaly and pulmonary congestion. An echocardiogram reveals a left ventricular ejection fraction of 30%, increased left ventricular end-diastolic volume, and no valvular abnormalities.

## Encounter 4: source 2022-07-15, practice 2024-05-06, ed, Emergency Medicine

Chief complaint: Progressive difficulty breathing when lying flat requiring three pillows and paroxysmal nocturnal dyspnea

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Emily Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive difficulty breathing when lying flat requiring three pillows and paroxysmal nocturnal dyspnea

HISTORY OF PRESENT ILLNESS:
This 69-year-old male with heart failure with reduced ejection fraction (hospitalized January 2022), polycythemia vera, and atherosclerotic cardiovascular disease complicated by cholesterol embolism returns for evaluation. He reports that he now needs to sleep propped up on three pillows to avoid waking up gasping for air. Over the past few weeks, he has also experienced episodes of sudden breathlessness during the night that awaken him from sleep. He denies fevers, recent illnesses, or coughing up blood. He reports occasional leg swelling, especially in the evenings.

PAST MEDICAL HISTORY:
Active Problem List:
- Tubular adenoma of descending colon with high-grade dysplasia (diagnosed 2020-01-15)
- Polycythemia vera (diagnosed 2020-07-15)
- Cholesterol embolism (diagnosed 2021-07-15)
- Heart failure with reduced ejection fraction (diagnosed 2022-01-15)
- Type 2 diabetes mellitus (15-year duration)
- Hypertension (history of poor control)
- Atherosclerotic cardiovascular disease
- History of colorectal tubular adenoma with high-grade dysplasia

He has a history of hypertension.

PAST SURGICAL HISTORY:
- Colonoscopy with polypectomy
- Inguinal hernia repair

MEDICATIONS:
The patient is compliant with his antihypertensive medications

Home Medications:
- Metformin 1000 mg orally twice daily
- Lisinopril 20 mg orally daily
- Hydrochlorothiazide 25 mg orally daily
- Furosemide 40 mg orally daily
- Carvedilol 12.5 mg orally twice daily
- Hydroxyurea 500 mg orally daily
- Aspirin 81 mg orally daily
- Atorvastatin 40 mg orally daily
- Ergocalciferol 50000 units orally weekly

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 72
- Mother: Hypertension, stroke at age 78
- Paternal uncle: Colorectal cancer diagnosed age 65
- No known family history of hematologic malignancies

SOCIAL HISTORY:
He does not smoke and drinks alcohol socially.

PHYSICAL EXAMINATION:
On physical examination, he appears tired but is not in acute distress. Jugular venous distension is observed when the patient is positioned at a 45-degree angle. Bibasilar crackles are heard on lung auscultation. The heart rate is regular, with a soft S1 and a prominent S3 gallop. There is mild bilateral ankle edema. No wheezing or rhonchi are noted. Neurologic and abdominal exams are unremarkable.

IMAGING:
Chest x-ray reveals prominent pulmonary vessels and mild hazy opacities in the lower lung fields.

## Encounter 5: source 2023-01-15, practice 2024-10-30, ed, Emergency Medicine

Chief complaint: Acute worsening shortness of breath, orthopnea, and lower extremity edema following upper respiratory infection

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute worsening shortness of breath, orthopnea, and lower extremity edema following upper respiratory infection

HISTORY OF PRESENT ILLNESS:
The patient is a 70-year-old male with heart failure with reduced ejection fraction, hypertension, and type 2 diabetes mellitus previously hospitalized in January 2022 and seen in the emergency department in July 2022 for similar complaints, who now presents to the emergency department with a 2-day history of increasing shortness of breath, orthopnea, and lower extremity edema. He reports a recent upper respiratory tract infection that resolved a week ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Tubular adenoma of descending colon with high-grade dysplasia (diagnosed 2020-01-15)
- Polycythemia vera (diagnosed 2020-07-15)
- Cholesterol embolism (diagnosed 2021-07-15)
- Heart failure with reduced ejection fraction (diagnosed 2022-01-15)
- Type 2 diabetes mellitus (15-year duration)
- Hypertension (history of poor control)
- Atherosclerotic cardiovascular disease
- History of colorectal tubular adenoma with high-grade dysplasia

PAST SURGICAL HISTORY:
- Colonoscopy with polypectomy
- Inguinal hernia repair

MEDICATIONS:
Home Medications:
- Metformin 1000 mg orally twice daily
- Lisinopril 20 mg orally daily
- Hydrochlorothiazide 25 mg orally daily
- Furosemide 40 mg orally daily
- Carvedilol 12.5 mg orally twice daily
- Hydroxyurea 500 mg orally daily
- Aspirin 81 mg orally daily
- Atorvastatin 40 mg orally daily
- Ergocalciferol 50000 units orally weekly

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 72
- Mother: Hypertension, stroke at age 78
- Paternal uncle: Colorectal cancer diagnosed age 65
- No known family history of hematologic malignancies

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Metal fabrication plant worker.

VITAL SIGNS:
Blood pressure is 160/90 mmHg, heart rate is 102 beats per minute, respiratory rate is 28 breaths per minute, and oxygen saturation is 88% on room air

PHYSICAL EXAMINATION:
Lung auscultation reveals bibasilar crackles, and cardiac examination shows an S3 gallop. Jugular venous pressure is elevated.

LABORATORY DATA:
Arterial blood gas on room air shows:
- pH: 7.48
- PaCO2: 32 mmHg
- PaO2: 58 mmHg
- HCO3-: 24 mEq/L

IMAGING:
Chest X-ray demonstrates cardiomegaly and bilateral pleural effusions.

## Encounter 6: source 2023-07-15, practice 2025-04-24, ed, Emergency Medicine

Chief complaint: High-grade fever, fatigue, and left upper quadrant pain for two weeks in patient with known polycythemia vera

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
High-grade fever, fatigue, and left upper quadrant pain for two weeks in patient with known polycythemia vera

HISTORY OF PRESENT ILLNESS:
This 70-year-old male with polycythemia vera diagnosed in 2020 and heart failure with reduced ejection fraction presents to the emergency department with a 2-week history of high-grade fever, fatigue, and left upper quadrant pain. The patient's recent history is negative for any exposure to cytotoxic agents or ionizing radiation.

PAST MEDICAL HISTORY:
Active Problem List:
- Tubular adenoma of descending colon with high-grade dysplasia (diagnosed 2020-01-15)
- Polycythemia vera (diagnosed 2020-07-15)
- Cholesterol embolism (diagnosed 2021-07-15)
- Heart failure with reduced ejection fraction (diagnosed 2022-01-15)
- Type 2 diabetes mellitus (15-year duration)
- Hypertension (history of poor control)
- Atherosclerotic cardiovascular disease
- History of colorectal tubular adenoma with high-grade dysplasia

His past medical history is significant for type 2 diabetes mellitus and hypertension.

PAST SURGICAL HISTORY:
- Colonoscopy with polypectomy
- Inguinal hernia repair

MEDICATIONS:
Home Medications:
- Metformin 1000 mg orally twice daily
- Lisinopril 20 mg orally daily
- Hydrochlorothiazide 25 mg orally daily
- Furosemide 40 mg orally daily
- Carvedilol 12.5 mg orally twice daily
- Hydroxyurea 500 mg orally daily
- Aspirin 81 mg orally daily
- Atorvastatin 40 mg orally daily
- Ergocalciferol 50000 units orally weekly

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 72
- Mother: Hypertension, stroke at age 78
- Paternal uncle: Colorectal cancer diagnosed age 65
- No known family history of hematologic malignancies

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Metal fabrication plant worker.

VITAL SIGNS:
On examination, his temperature is 39.2°C (102.5°F), blood pressure is 135/85 mmHg, pulse is 98/min, and respiratory rate is 20/min

PHYSICAL EXAMINATION:
Abdominal examination reveals splenomegaly without hepatomegaly. No lymphadenopathy is noted.

LABORATORY DATA:
Laboratory studies are ordered:

- White blood cell count: 55,000/mm3
- Hemoglobin: 12.2 g/dL
- Platelet count: 300,000/mm3
- Differential: 6% myelocytes, 12% metamyelocytes, 40% bands, 30% segmented neutrophils, 2% eosinophils, 1% basophils
- Alkaline phosphatase: 250 U/L (Normal: 40-130 U/L)
- Lactate dehydrogenase (LDH): Elevated
- BCR-ABL fusion gene: Not detected

OTHER STUDIES:
A peripheral blood smear shows increased mature and immature granulocytes with a left shift. No blasts are seen.

## Encounter 7: source 2024-01-15, practice 2025-10-30, icu, Critical Care Medicine

Chief complaint: Acute confusion, fever, headache, neck stiffness, and vomiting over two days following recent upper respiratory infection

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Robert Thompson, MD
Department: Critical Care Medicine
Visit Type: icu

CHIEF COMPLAINT:
Acute confusion, fever, headache, neck stiffness, and vomiting over two days following recent upper respiratory infection

HISTORY OF PRESENT ILLNESS:
The patient is a 71-year-old male with a history of recurrent heart failure exacerbations and polycythemia vera who is brought to the emergency department by his daughter due to confusion, fever, and headache that started two days ago. He has had worsening drowsiness, neck stiffness, and vomiting over the past day. His daughter reports that he had an upper respiratory infection one week ago, which initially improved.

PAST MEDICAL HISTORY:
Active Problem List:
- Tubular adenoma of descending colon with high-grade dysplasia (diagnosed 2020-01-15)
- Polycythemia vera (diagnosed 2020-07-15)
- Cholesterol embolism (diagnosed 2021-07-15)
- Heart failure with reduced ejection fraction (diagnosed 2022-01-15)
- Leukocytosis (diagnosed 2023-07-15)
- Type 2 diabetes mellitus (15-year duration)
- Hypertension (history of poor control)
- Atherosclerotic cardiovascular disease
- History of colorectal tubular adenoma with high-grade dysplasia

His medical history includes hypertension and type 2 diabetes mellitus. He has not received any vaccinations in recent years.

PAST SURGICAL HISTORY:
- Colonoscopy with polypectomy
- Inguinal hernia repair

MEDICATIONS:
Home Medications:
- Metformin 1000 mg orally twice daily
- Lisinopril 20 mg orally daily
- Hydrochlorothiazide 25 mg orally daily
- Furosemide 40 mg orally daily
- Carvedilol 12.5 mg orally twice daily
- Hydroxyurea 500 mg orally daily
- Aspirin 81 mg orally daily
- Atorvastatin 40 mg orally daily
- Ergocalciferol 50000 units orally weekly

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus, myocardial infarction at age 72
- Mother: Hypertension, stroke at age 78
- Paternal uncle: Colorectal cancer diagnosed age 65
- No known family history of hematologic malignancies

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Metal fabrication plant worker.

VITAL SIGNS:
Temperature is 38.9 °C (102.0 °F), blood pressure is 112/68 mm Hg, pulse is 102/min, and respirations are 20/min.

PHYSICAL EXAMINATION:
On examination, he is disoriented to time and place. Physical exam shows photophobia and resistance to passive neck flexion.

LABORATORY DATA:
CSF reveals: opening pressure 300 mm H2O, WBC 1,050/mm³ (90% neutrophils), glucose 28 mg/dL, protein 200 mg/dL. Gram stain shows gram-positive lancet-shaped diplococci.

OTHER STUDIES:
A lumbar puncture is performed.
