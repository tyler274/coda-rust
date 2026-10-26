# synthetic_hospital patient 2101

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 58 at first encounter, sex M, Non-Hispanic White, Medicare with Federal Employee Health Benefits (FEHB) supplement.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-02-10 (Thu) | outpatient | Pulmonology |
| 1 | 2020-04-15 | 2022-05-23 (Mon) | outpatient | Gastroenterology |
| 2 | 2020-05-15 | 2022-06-24 (Fri) | ed | Emergency Medicine |
| 3 | 2020-07-15 | 2022-08-24 (Wed) | procedure | Gastroenterology |
| 4 | 2024-01-15 | 2026-02-13 (Fri) | outpatient | Oncology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare with Federal Employee Health Benefits (FEHB) supplement",
 "occupation": "Retired postal worker",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Father: Colorectal cancer (diagnosed age 65, deceased)",
  "Mother: Hypertension and osteoporosis",
  "Paternal uncle: Lung cancer"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 35 pack-years, quit 5 years ago",
 "home_medications": [
  {
   "dose": "40 mg PO daily (intermittently compliant)",
   "name": "Omeprazole"
  },
  {
   "dose": "801 mg PO three times daily with meals",
   "name": "Pirfenidone"
  },
  {
   "dose": "500 mg PO every 6 hours PRN for pain (max 3g/day)",
   "name": "Acetaminophen"
  }
 ],
 "surgical_history": [
  "Colonoscopy with polypectomy",
  "Esophagogastroduodenoscopy (EGD) complicated by perforation",
  "Laparoscopic appendectomy (age 12)"
 ],
 "chronic_conditions": [
  "Gastroesophageal reflux disease with Barrett's esophagus",
  "Hiatal hernia",
  "Idiopathic pulmonary fibrosis",
  "History of KRAS-mutated advanced colorectal adenoma with high-grade dysplasia",
  "Former tobacco use disorder"
 ],
 "age_at_first_encounter": 58
}
```

Primary diagnoses: ["Secondary malignant neoplasm of liver", "Esophageal rupture with pleural effusion", "Idiopathic Pulmonary Fibrosis", "KRAS-mutated advanced colorectal adenoma (tubular adenoma with high-grade dysplasia)", "Barrett esophagus"]
Comorbidities: ["Gastroesophageal reflux disease", "History of recent pneumonia", "Family history of malignant neoplasm of digestive organs"]

## Encounter 0: source 2020-01-15, practice 2022-02-10, outpatient, Pulmonology

Chief complaint: Progressive shortness of breath and reduced exercise tolerance over the past year

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive shortness of breath and reduced exercise tolerance over the past year

HISTORY OF PRESENT ILLNESS:
He comes to the clinic due to progressive shortness of breath and reduced exercise tolerance over the past year. He reports that he can no longer walk his dog around the block without stopping to catch his breath. He also has a chronic, dry cough but denies wheezing, chest pain, or sputum production.

PAST MEDICAL HISTORY:
Active Problem List:
- Gastroesophageal reflux disease with Barrett's esophagus
- Hiatal hernia
- Idiopathic pulmonary fibrosis
- History of KRAS-mutated advanced colorectal adenoma with high-grade dysplasia
- Former tobacco use disorder

PAST SURGICAL HISTORY:
- Colonoscopy with polypectomy
- Esophagogastroduodenoscopy (EGD) complicated by perforation
- Laparoscopic appendectomy (age 12)

MEDICATIONS:
Home Medications:
- Omeprazole 40 mg PO daily (intermittently compliant)
- Pirfenidone 801 mg PO three times daily with meals
- Acetaminophen 500 mg PO every 6 hours PRN for pain (max 3g/day)

ALLERGIES:
He has no known allergies

FAMILY HISTORY:
- Father: Colorectal cancer (diagnosed age 65, deceased)
- Mother: Hypertension and osteoporosis
- Paternal uncle: Lung cancer

SOCIAL HISTORY:
He smoked one pack of cigarettes daily for 35 years but quit 5 years ago. He works as a retired postal worker and has not had recent exposure to fumes or dust.

VITAL SIGNS:
Oxygen saturation is 92% on room air

PHYSICAL EXAMINATION:
On physical examination, he appears mildly dyspneic at rest. Lung auscultation reveals fine end-inspiratory crackles at the bases without wheezes.

IMAGING:
Chest x-ray shows reticulonodular opacities at the bilateral lung bases and a small volume thoracic cavity. High-resolution CT scan demonstrates subpleural reticular thickening, traction bronchiectasis, and honeycombing in a basal distribution.

## Encounter 1: source 2020-04-15, practice 2022-05-23, outpatient, Gastroenterology

Chief complaint: Worsening difficulty swallowing solid foods over the past 4 months

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Marcus Johnson, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening difficulty swallowing solid foods over the past 4 months

HISTORY OF PRESENT ILLNESS:
The patient, seen in Pulmonology three months ago for dyspnea and with known Barrett's esophagus, returns reporting he has had burning chest discomfort for years after meals but recently developed trouble swallowing solid foods over the past 4 months. He denies weight loss, vomiting, or hematemesis.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-01-15)
- Gastroesophageal reflux disease with Barrett's esophagus
- Hiatal hernia
- History of KRAS-mutated advanced colorectal adenoma with high-grade dysplasia
- Former tobacco use disorder

PAST SURGICAL HISTORY:
- Colonoscopy with polypectomy
- Esophagogastroduodenoscopy (EGD) complicated by perforation
- Laparoscopic appendectomy (age 12)

MEDICATIONS:
He has been intermittently compliant with omeprazole.

Home Medications:
- Pirfenidone 801 mg PO three times daily with meals
- Acetaminophen 500 mg PO every 6 hours PRN for pain (max 3g/day)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Colorectal cancer (diagnosed age 65, deceased)
- Mother: Hypertension and osteoporosis
- Paternal uncle: Lung cancer

SOCIAL HISTORY:
Smoking: Former smoker, 35 pack-years, quit 5 years ago. Alcohol: Social (1-2 drinks per week). Occupation: Retired postal worker.

PATHOLOGY:
Histologic analysis reveals the presence of goblet cells within a transformed epithelium.

OTHER STUDIES:
Upper endoscopy is performed and reveals a salmon-colored patch in the distal esophagus, which is biopsied.

## Encounter 2: source 2020-05-15, practice 2022-06-24, ed, Emergency Medicine

Chief complaint: Sudden onset of severe chest pain radiating to the back, fever, and shortness of breath

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. Emily Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of severe chest pain radiating to the back, fever, and shortness of breath

HISTORY OF PRESENT ILLNESS:
The patient with Barrett esophagus and idiopathic pulmonary fibrosis, evaluated by Gastroenterology last month for dysphagia, presents to the emergency department with sudden onset of severe chest pain radiating to the back, fever, and shortness of breath that began 2 hours after undergoing an upper endoscopy for evaluation of dysphagia.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-01-15)
- Barrett esophagus (diagnosed 2020-04-15)
- Gastroesophageal reflux disease with Barrett's esophagus
- Hiatal hernia
- History of KRAS-mutated advanced colorectal adenoma with high-grade dysplasia
- Former tobacco use disorder

His medical history is significant for gastroesophageal reflux disease and a hiatal hernia.

PAST SURGICAL HISTORY:
- Colonoscopy with polypectomy
- Esophagogastroduodenoscopy (EGD) complicated by perforation
- Laparoscopic appendectomy (age 12)

MEDICATIONS:
Home Medications:
- Omeprazole 40 mg PO daily (intermittently compliant)
- Pirfenidone 801 mg PO three times daily with meals
- Acetaminophen 500 mg PO every 6 hours PRN for pain (max 3g/day)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Colorectal cancer (diagnosed age 65, deceased)
- Mother: Hypertension and osteoporosis
- Paternal uncle: Lung cancer

SOCIAL HISTORY:
Smoking: Former smoker, 35 pack-years, quit 5 years ago. Alcohol: Social (1-2 drinks per week). Occupation: Retired postal worker.

VITAL SIGNS:
Temperature of 38.7°C (101.7°F), heart rate of 110 bpm, and respiratory rate of 22 breaths/min. His blood pressure is 135/85 mmHg.

PHYSICAL EXAMINATION:
On examination, the patient appears acutely ill. Subcutaneous emphysema is palpated in the neck, and auscultation of the chest reveals decreased breath sounds at the left base.

LABORATORY DATA:
White blood cell count: 18,000/mm³
- Hemoglobin: 13.2 g/dL
- Platelet count: 250,000/mm³
- Serum creatinine: 1.0 mg/dL
- Blood urea nitrogen: 21 mg/dL

IMAGING:
A chest x-ray shows a widened mediastinum and left pleural effusion.

PLAN:
The patient is started on broad-spectrum antibiotics.

## Encounter 3: source 2020-07-15, practice 2022-08-24, procedure, Gastroenterology

Chief complaint: Screening colonoscopy for intermittent mild abdominal discomfort and change in bowel habits

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Gastroenterology
Visit Type: procedure

CHIEF COMPLAINT:
Screening colonoscopy for intermittent mild abdominal discomfort and change in bowel habits

HISTORY OF PRESENT ILLNESS:
The patient returns to Gastroenterology following recent esophageal rupture. He undergoes screening colonoscopy after reporting intermittent mild abdominal discomfort and a change in bowel habits over the past few months.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-01-15)
- Barrett esophagus (diagnosed 2020-04-15)
- Esophageal rupture with pleural effusion (diagnosed 2020-05-15)
- Gastroesophageal reflux disease with Barrett's esophagus
- Hiatal hernia
- History of KRAS-mutated advanced colorectal adenoma with high-grade dysplasia
- Former tobacco use disorder

PAST SURGICAL HISTORY:
- Colonoscopy with polypectomy
- Esophagogastroduodenoscopy (EGD) complicated by perforation
- Laparoscopic appendectomy (age 12)

MEDICATIONS:
Home Medications:
- Omeprazole 40 mg PO daily (intermittently compliant)
- Pirfenidone 801 mg PO three times daily with meals
- Acetaminophen 500 mg PO every 6 hours PRN for pain (max 3g/day)

ALLERGIES:
NKDA

FAMILY HISTORY:
His family history is significant for colorectal cancer in a first-degree relative diagnosed at age 62.

SOCIAL HISTORY:
Smoking: Former smoker, 35 pack-years, quit 5 years ago. Alcohol: Social (1-2 drinks per week). Occupation: Retired postal worker.

PATHOLOGY:
The pathology report describes a tubular adenoma with areas of high-grade dysplasia but no invasive cancer.

OTHER STUDIES:
Colonoscopy reveals a 2.5-cm sessile polyp in the sigmoid colon, which is removed and sent for histopathological examination.

## Encounter 4: source 2024-01-15, practice 2026-02-13, outpatient, Oncology

Chief complaint: Progressive fatigue, weight loss, and mild right upper quadrant discomfort over 3 months

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Fatima Al-Zahra, MD
Department: Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue, weight loss, and mild right upper quadrant discomfort over 3 months

HISTORY OF PRESENT ILLNESS:
Patient with history of KRAS-mutated colorectal adenoma presents with 3-month history of progressive fatigue, weight loss, and mild right upper quadrant discomfort. He denies any fever, chills, or changes in bowel habits.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-01-15)
- Barrett esophagus (diagnosed 2020-04-15)
- Esophageal rupture with pleural effusion (diagnosed 2020-05-15)
- KRAS-mutated advanced colorectal adenoma (tubular adenoma with high-grade dysplasia) (diagnosed 2020-07-15)
- Gastroesophageal reflux disease with Barrett's esophagus
- Hiatal hernia
- History of KRAS-mutated advanced colorectal adenoma with high-grade dysplasia
- Former tobacco use disorder

PAST SURGICAL HISTORY:
- Colonoscopy with polypectomy
- Esophagogastroduodenoscopy (EGD) complicated by perforation
- Laparoscopic appendectomy (age 12)

MEDICATIONS:
Home Medications:
- Omeprazole 40 mg PO daily (intermittently compliant)
- Pirfenidone 801 mg PO three times daily with meals
- Acetaminophen 500 mg PO every 6 hours PRN for pain (max 3g/day)

ALLERGIES:
NKDA

FAMILY HISTORY:
His family history is significant for colon cancer in his father.

SOCIAL HISTORY:
History of smoking

VITAL SIGNS:
His vital signs are within normal limits

PHYSICAL EXAMINATION:
Abdominal examination reveals hepatomegaly with a firm, irregular liver edge palpable 3 cm below the right costal margin. No jaundice, ascites, or peripheral edema is noted.

LABORATORY DATA:
Hemoglobin: 13.2 g/dL
- White blood cell count: 7,800 /μL
- Platelets: 225,000 /μL
- AST: 68 U/L
- ALT: 72 U/L
- Alkaline phosphatase: 210 U/L
- Total bilirubin: 1.2 mg/dL
- Carcinoembryonic antigen (CEA): Elevated

IMAGING:
An abdominal ultrasound followed by a CT scan of the abdomen reveals multiple hypodense lesions in the liver. There is no evidence of primary liver cancer.
