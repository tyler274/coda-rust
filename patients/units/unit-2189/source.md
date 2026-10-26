# synthetic_hospital patient 2189

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 69 at first encounter, sex M, Non-Hispanic White, Medicare with supplemental Medigap.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-08-11 (Wed) | outpatient | Pulmonology |
| 1 | 2020-07-15 | 2022-01-27 (Thu) | outpatient | Gastroenterology |
| 2 | 2021-07-15 | 2023-02-03 (Fri) | outpatient | Dermatology |
| 3 | 2022-07-15 | 2024-02-08 (Thu) | outpatient | Gastroenterology |
| 4 | 2023-01-15 | 2024-07-30 (Tue) | ed | Emergency Medicine |
| 5 | 2023-07-15 | 2025-02-11 (Tue) | icu | Critical Care |
| 6 | 2023-09-15 | 2025-04-10 (Thu) | inpatient | Gastroenterology |
| 7 | 2024-07-15 | 2026-02-02 (Mon) | icu | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare with supplemental Medigap",
 "occupation": "Retired construction foreman",
 "alcohol_use": "Former heavy use (4-6 standard drinks daily), quit 5 years ago",
 "family_history": [
  "Father: Chronic obstructive pulmonary disease, lung cancer",
  "Mother: Type 2 diabetes mellitus",
  "Brother: Cirrhosis"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, approximately 45 pack-years, quit 10 years ago (reports brief relapse 5 years ago)",
 "home_medications": [
  {
   "dose": "2 L/min via nasal cannula continuous",
   "name": "Oxygen concentrator"
  },
  {
   "dose": "2.5 mg inhaled every 4-6 hours as needed for wheezing",
   "name": "Albuterol nebulizer solution"
  },
  {
   "dose": "50 mcg inhaled twice daily",
   "name": "Salmeterol"
  },
  {
   "dose": "18 mcg inhaled daily",
   "name": "Tiotropium"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "25 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "1000 mg PO twice daily",
   "name": "Metformin"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Chronic obstructive pulmonary disease (COPD) with chronic respiratory failure",
  "Idiopathic pulmonary fibrosis",
  "Hypertension",
  "Type 2 diabetes mellitus",
  "Zenker diverticulum",
  "Cirrhosis secondary to alcohol use",
  "Hepatocellular carcinoma (awaiting liver transplantation evaluation)",
  "History of severe Clostridioides difficile infection"
 ],
 "age_at_first_encounter": 69
}
```

Primary diagnoses: ["Tension pneumothorax", "Fulminant Clostridioides difficile infection", "Acute-on-chronic respiratory failure secondary to COPD exacerbation with superimposed bacterial pneumonia", "Idiopathic Pulmonary Fibrosis", "Zenker diverticulum", "Hepatocellular carcinoma or end-stage liver disease (liver transplantation candidate)", "COPD exacerbation with acute hypercapnic respiratory failure without altered mental status", "Pancreatic carcinoma"]
Comorbidities: ["Chronic obstructive pulmonary disease", "History of recent pneumonia", "Essential hypertension", "Long-term oxygen therapy"]

## Encounter 0: source 2020-01-15, practice 2021-08-11, outpatient, Pulmonology

Chief complaint: Progressive dyspnea on exertion and nonproductive cough for 6 months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive dyspnea on exertion and nonproductive cough for 6 months

HISTORY OF PRESENT ILLNESS:
He denies any fever, chest pain, or recent travel.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (COPD) with chronic respiratory failure
- Idiopathic pulmonary fibrosis
- Hypertension
- Type 2 diabetes mellitus
- Zenker diverticulum
- Cirrhosis secondary to alcohol use
- Hepatocellular carcinoma (awaiting liver transplantation evaluation)
- History of severe Clostridioides difficile infection

His past medical history includes hypertension,.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He takes for which he takes hydrochlorothiazide,.

Home Medications:
- Oxygen concentrator 2 L/min via nasal cannula continuous
- Albuterol nebulizer solution 2.5 mg inhaled every 4-6 hours as needed for wheezing
- Salmeterol 50 mcg inhaled twice daily
- Tiotropium 18 mcg inhaled daily
- Lisinopril 20 mg PO daily
- Metformin 1000 mg PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Chronic obstructive pulmonary disease, lung cancer
- Mother: Type 2 diabetes mellitus
- Brother: Cirrhosis

SOCIAL HISTORY:
A 40-pack-year smoking history, although he quit smoking 10 years ago.

PHYSICAL EXAMINATION:
On physical examination, you note clubbing of the fingers and inspiratory crackles at the lung bases bilaterally.

IMAGING:
High-resolution computed tomography (HRCT) of the chest shows reticular opacities with honeycombing predominantly at the lung bases and subpleural regions. There is no evidence of pulmonary embolism or other acute processes.

OTHER STUDIES:
Pulmonary function tests reveal the following:

- Total lung capacity (TLC): Decreased
- Functional residual capacity (FRC): Decreased
- Residual volume (RV): Decreased
- Forced expiratory volume in 1 second (FEV1): Preserved
- Forced vital capacity (FVC): Preserved
- FEV1/FVC ratio: Increased
- Diffusion capacity of carbon monoxide (DLCO): Reduced

## Encounter 1: source 2020-07-15, practice 2022-01-27, outpatient, Gastroenterology

Chief complaint: Dysphagia with solids and occasional regurgitation of undigested food

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Dysphagia with solids and occasional regurgitation of undigested food

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of his Zenker diverticulum six months after his Pulmonology evaluation for idiopathic pulmonary fibrosis. He presents with a 6-month history of dysphagia, predominantly with solids, and occasional regurgitation of undigested food, especially when lying down. He also reports a sensation of a lump in his throat and bad breath.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-01-15)
- Chronic obstructive pulmonary disease (COPD) with chronic respiratory failure
- Hypertension
- Type 2 diabetes mellitus
- Zenker diverticulum
- Cirrhosis secondary to alcohol use
- Hepatocellular carcinoma (awaiting liver transplantation evaluation)
- History of severe Clostridioides difficile infection

His medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He takes for which he takes lisinopril.

Home Medications:
- Oxygen concentrator 2 L/min via nasal cannula continuous
- Albuterol nebulizer solution 2.5 mg inhaled every 4-6 hours as needed for wheezing
- Salmeterol 50 mcg inhaled twice daily
- Tiotropium 18 mcg inhaled daily
- Hydrochlorothiazide 25 mg PO daily
- Metformin 1000 mg PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Chronic obstructive pulmonary disease, lung cancer
- Mother: Type 2 diabetes mellitus
- Brother: Cirrhosis

SOCIAL HISTORY:
He has a 20-pack-year smoking history but quit 10 years ago.

PHYSICAL EXAMINATION:
On physical examination, there is no palpable neck mass, and neurological examination is normal.

IMAGING:
A barium swallow study shows a posterior outpouching of the esophagus just above the esophageal sphincter.

## Encounter 2: source 2021-07-15, practice 2023-02-03, outpatient, Dermatology

Chief complaint: Slowly enlarging lesion on right upper back

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Priya Patel, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Slowly enlarging lesion on right upper back

HISTORY OF PRESENT ILLNESS:
The patient, with idiopathic pulmonary fibrosis and Zenker diverticulum diagnosed in 2020, presents with a lesion on his right upper back that has been slowly increasing in size over the past year. He reports no pain or itching associated with the lesion.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-01-15)
- Zenker diverticulum (diagnosed 2020-07-15)
- Chronic obstructive pulmonary disease (COPD) with chronic respiratory failure
- Hypertension
- Type 2 diabetes mellitus
- Cirrhosis secondary to alcohol use
- Hepatocellular carcinoma (awaiting liver transplantation evaluation)
- History of severe Clostridioides difficile infection

His medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He takes hydrochlorothiazide.

Home Medications:
- Oxygen concentrator 2 L/min via nasal cannula continuous
- Albuterol nebulizer solution 2.5 mg inhaled every 4-6 hours as needed for wheezing
- Salmeterol 50 mcg inhaled twice daily
- Tiotropium 18 mcg inhaled daily
- Lisinopril 20 mg PO daily
- Metformin 1000 mg PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Chronic obstructive pulmonary disease, lung cancer
- Mother: Type 2 diabetes mellitus
- Brother: Cirrhosis

SOCIAL HISTORY:
A 45-pack-year smoking history, although he quit smoking 10 years ago

PHYSICAL EXAMINATION:
On physical examination, you note a 1.5 cm flesh-colored nodule with a pearly appearance and telangiectasias on the surface. The lesion has raised, rolled borders and a central ulceration. There is no palpable lymphadenopathy. The remainder of the skin examination is unremarkable.

ASSESSMENT AND PLAN:
Given the patient's presentation and physical examination findings,

## Encounter 3: source 2022-07-15, practice 2024-02-08, outpatient, Gastroenterology

Chief complaint: Persistent midepigastric pain radiating to back with 10-pound weight loss over 3 months

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Marcus Johnson, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent midepigastric pain radiating to back with 10-pound weight loss over 3 months

HISTORY OF PRESENT ILLNESS:
The patient with cirrhosis awaiting transplantation and prior Zenker diverticulum presents with new complaints. He describes the pain as a dull ache that worsens after meals and when lying down. He also reports unintentional weight loss of 10 pounds over the same period.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-01-15)
- Zenker diverticulum (diagnosed 2020-07-15)
- Hepatocellular carcinoma or end-stage liver disease (liver transplantation candidate) (diagnosed 2021-07-15)
- Chronic obstructive pulmonary disease (COPD) with chronic respiratory failure
- Hypertension
- Type 2 diabetes mellitus
- Cirrhosis secondary to alcohol use
- Hepatocellular carcinoma (awaiting liver transplantation evaluation)
- History of severe Clostridioides difficile infection

His past medical history is significant for type 2 diabetes mellitus and hypertension, which are well-controlled with metformin and lisinopril, respectively.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He takes metformin and lisinopril.

Home Medications:
- Oxygen concentrator 2 L/min via nasal cannula continuous
- Albuterol nebulizer solution 2.5 mg inhaled every 4-6 hours as needed for wheezing
- Salmeterol 50 mcg inhaled twice daily
- Tiotropium 18 mcg inhaled daily
- Hydrochlorothiazide 25 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Chronic obstructive pulmonary disease, lung cancer
- Mother: Type 2 diabetes mellitus
- Brother: Cirrhosis

SOCIAL HISTORY:
He has a 30-pack-year smoking history but quit 5 years ago.

PHYSICAL EXAMINATION:
On physical examination, he appears jaundiced. His abdomen is soft, with mild tenderness in the epigastric region without rebound or guarding.

LABORATORY DATA:
Laboratory tests reveal:
- Hemoglobin: 12.1 g/dL (normal: 13.8-17.2 g/dL)
- Total bilirubin: 3.2 mg/dL (normal: 0.3-1.2 mg/dL)
- Alkaline phosphatase: 280 U/L (normal: 40-130 U/L)
- CA 19-9: 150 U/mL (normal: <37 U/mL)

## Encounter 4: source 2023-01-15, practice 2024-07-30, ed, Emergency Medicine

Chief complaint: Worsening dyspnea, persistent wheezing, and nocturnal cough following upper respiratory infection

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening dyspnea, persistent wheezing, and nocturnal cough following upper respiratory infection

HISTORY OF PRESENT ILLNESS:
The patient is a 72-year-old male with idiopathic pulmonary fibrosis, chronic obstructive pulmonary disease on continuous home oxygen therapy, and recently diagnosed pancreatic carcinoma who presents with worsening dyspnea, persistent wheezing, and nocturnal cough. He recently experienced an upper respiratory tract infection with symptoms persisting despite maximal inhaled β2-agonist therapy.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-01-15)
- Zenker diverticulum (diagnosed 2020-07-15)
- Hepatocellular carcinoma or end-stage liver disease (liver transplantation candidate) (diagnosed 2021-07-15)
- Pancreatic carcinoma (diagnosed 2022-07-15)
- Chronic obstructive pulmonary disease (COPD) with chronic respiratory failure
- Hypertension
- Type 2 diabetes mellitus
- Cirrhosis secondary to alcohol use
- Hepatocellular carcinoma (awaiting liver transplantation evaluation)
- History of severe Clostridioides difficile infection

He is on home oxygen.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Oxygen concentrator 2 L/min via nasal cannula continuous
- Albuterol nebulizer solution 2.5 mg inhaled every 4-6 hours as needed for wheezing
- Salmeterol 50 mcg inhaled twice daily
- Tiotropium 18 mcg inhaled daily
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Metformin 1000 mg PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Chronic obstructive pulmonary disease, lung cancer
- Mother: Type 2 diabetes mellitus
- Brother: Cirrhosis

SOCIAL HISTORY:
55 pack-year smoking history

PHYSICAL EXAMINATION:
Tachypneic with prolonged expiratory phase and diffuse bilateral wheezing

IMAGING:
Chest radiograph reveals hyperinflated lungs with flattened diaphragms

PLAN:
His treatment regimen is escalated to include systemic corticosteroids, resulting in marked improvement in airflow and wheezing within 48 hours. The medical team discusses the mechanism by which glucocorticoids such as prednisone facilitate better clinical response to β-agonists in this setting

## Encounter 5: source 2023-07-15, practice 2025-02-11, icu, Critical Care

Chief complaint: Acute shortness of breath with altered mental status and productive cough

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Angela Rossi, MD
Department: Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Acute shortness of breath with altered mental status and productive cough

HISTORY OF PRESENT ILLNESS:
The patient with COPD with chronic respiratory failure and idiopathic pulmonary fibrosis presents six months after a prior exacerbation. His wife reports that he has been increasingly lethargic over the past two days and has had a productive cough with greenish sputum. The patient becomes progressively somnolent and is unable to provide any history or respond to questions.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-01-15)
- Zenker diverticulum (diagnosed 2020-07-15)
- Hepatocellular carcinoma or end-stage liver disease (liver transplantation candidate) (diagnosed 2021-07-15)
- Pancreatic carcinoma (diagnosed 2022-07-15)
- COPD exacerbation with acute hypercapnic respiratory failure without altered mental status (diagnosed 2023-01-15)
- Chronic obstructive pulmonary disease (COPD) with chronic respiratory failure
- Hypertension
- Type 2 diabetes mellitus
- Cirrhosis secondary to alcohol use
- Hepatocellular carcinoma (awaiting liver transplantation evaluation)
- History of severe Clostridioides difficile infection

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Oxygen concentrator 2 L/min via nasal cannula continuous
- Albuterol nebulizer solution 2.5 mg inhaled every 4-6 hours as needed for wheezing
- Salmeterol 50 mcg inhaled twice daily
- Tiotropium 18 mcg inhaled daily
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Metformin 1000 mg PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Chronic obstructive pulmonary disease, lung cancer
- Mother: Type 2 diabetes mellitus
- Brother: Cirrhosis

SOCIAL HISTORY:
Smoking: Former smoker, approximately 45 pack-years, quit 10 years ago (reports brief relapse 5 years ago). Alcohol: Former heavy use (4-6 standard drinks daily), quit 5 years ago. Occupation: Retired construction foreman.

PHYSICAL EXAMINATION:
On examination, the patient is tachypneic with a respiratory rate of 28 breaths per minute, uses accessory muscles to breathe, and is unable to speak in full sentences. His oxygen saturation is 88% on room air.

LABORATORY DATA:
Arterial blood gas shows pH 7.28, PaCO2 60 mmHg, PaO2 55 mmHg, and HCO3- 26 mEq/L.

Laboratory Values:
- pH: 7.28
- PaCO2: 60 mmHg
- PaO2: 55 mmHg
- HCO3-: 26 mEq/L
- Sodium: 138 mEq/L
- Potassium: 4.2 mEq/L
- Chloride: 100 mEq/L
- Blood Urea Nitrogen: 18 mg/dL
- Creatinine: 1.0 mg/dL
- White Blood Cell Count: 15,000 cells/mm^3

IMAGING:
Chest X-ray reveals increased bronchovascular markings and a small right lower lobe infiltrate.

## Encounter 6: source 2023-09-15, practice 2025-04-10, inpatient, Gastroenterology

Chief complaint: Watery diarrhea, abdominal pain, and fever 2 weeks after completing clindamycin

ENCOUNTER NOTE
Date: 2023-09-15
Provider: Dr. James O'Brien, MD
Department: Gastroenterology
Visit Type: inpatient

CHIEF COMPLAINT:
Watery diarrhea, abdominal pain, and fever 2 weeks after completing clindamycin

HISTORY OF PRESENT ILLNESS:
This 72-year-old male with a history of severe Clostridioides difficile infection, cirrhosis, and chronic respiratory failure presents to the emergency department with a 5-day history of watery diarrhea, abdominal pain, and fever. He completed a course of clindamycin 2 weeks ago. Despite receiving oral vancomycin and intravenous metronidazole for 48 hours, the patient's abdominal pain has worsened, and he has become increasingly tachycardic and hypotensive.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-01-15)
- Zenker diverticulum (diagnosed 2020-07-15)
- Hepatocellular carcinoma or end-stage liver disease (liver transplantation candidate) (diagnosed 2021-07-15)
- Pancreatic carcinoma (diagnosed 2022-07-15)
- COPD exacerbation with acute hypercapnic respiratory failure without altered mental status (diagnosed 2023-01-15)
- Acute-on-chronic respiratory failure secondary to COPD exacerbation with superimposed bacterial pneumonia (diagnosed 2023-07-15)
- Chronic obstructive pulmonary disease (COPD) with chronic respiratory failure
- Hypertension
- Type 2 diabetes mellitus
- Cirrhosis secondary to alcohol use
- Hepatocellular carcinoma (awaiting liver transplantation evaluation)
- History of severe Clostridioides difficile infection

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He takes oral vancomycin and intravenous metronidazole.

Home Medications:
- Oxygen concentrator 2 L/min via nasal cannula continuous
- Albuterol nebulizer solution 2.5 mg inhaled every 4-6 hours as needed for wheezing
- Salmeterol 50 mcg inhaled twice daily
- Tiotropium 18 mcg inhaled daily
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Metformin 1000 mg PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Chronic obstructive pulmonary disease, lung cancer
- Mother: Type 2 diabetes mellitus
- Brother: Cirrhosis

SOCIAL HISTORY:
Smoking: Former smoker, approximately 45 pack-years, quit 10 years ago (reports brief relapse 5 years ago). Alcohol: Former heavy use (4-6 standard drinks daily), quit 5 years ago. Occupation: Retired construction foreman.

VITAL SIGNS:
His temperature is 38.7°C (101.7°F), blood pressure is 90/60 mmHg, and heart rate is 110 bpm.

PHYSICAL EXAMINATION:
Physical examination reveals diffuse abdominal tenderness with rebound tenderness in the lower quadrants. Bowel sounds are hypoactive.

LABORATORY DATA:
Laboratory tests show:

- White blood cell count: 18,000/mm³ (normal: 4,500-11,000/mm³)
- Serum lactate: 4.5 mmol/L (normal: 0.5-2.2 mmol/L)
- Creatinine: 1.8 mg/dL (baseline 1.0 mg/dL)
- Blood urea nitrogen: 35 mg/dL (normal: 7-20 mg/dL)
- C-reactive protein: 25 mg/L (normal: <10 mg/L)

IMAGING:
A computed tomography (CT) scan of the abdomen shows colonic wall thickening without evidence of free air.

OTHER STUDIES:
Stool testing is positive for Clostridioides difficile toxin.

## Encounter 7: source 2024-07-15, practice 2026-02-02, icu, Emergency Medicine

Chief complaint: Acute onset of shortness of breath and left-sided chest pain after coughing

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Lisa Wong, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Acute onset of shortness of breath and left-sided chest pain after coughing

HISTORY OF PRESENT ILLNESS:
This 73-year-old male with chronic obstructive pulmonary disease with chronic respiratory failure and idiopathic pulmonary fibrosis, who required ICU admission for acute-on-chronic respiratory failure in July 2023, is brought to the emergency department with acute onset of shortness of breath and left-sided chest pain that started suddenly after coughing. He is on home oxygen therapy and has been using a home nebulizer.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-01-15)
- Zenker diverticulum (diagnosed 2020-07-15)
- Hepatocellular carcinoma or end-stage liver disease (liver transplantation candidate) (diagnosed 2021-07-15)
- Pancreatic carcinoma (diagnosed 2022-07-15)
- COPD exacerbation with acute hypercapnic respiratory failure without altered mental status (diagnosed 2023-01-15)
- Acute-on-chronic respiratory failure secondary to COPD exacerbation with superimposed bacterial pneumonia (diagnosed 2023-07-15)
- Fulminant Clostridioides difficile infection (diagnosed 2023-09-15)
- Chronic obstructive pulmonary disease (COPD) with chronic respiratory failure
- Hypertension
- Type 2 diabetes mellitus
- Cirrhosis secondary to alcohol use
- Hepatocellular carcinoma (awaiting liver transplantation evaluation)
- History of severe Clostridioides difficile infection

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Oxygen concentrator 2 L/min via nasal cannula continuous
- Albuterol nebulizer solution 2.5 mg inhaled every 4-6 hours as needed for wheezing
- Salmeterol 50 mcg inhaled twice daily
- Tiotropium 18 mcg inhaled daily
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Metformin 1000 mg PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Chronic obstructive pulmonary disease, lung cancer
- Mother: Type 2 diabetes mellitus
- Brother: Cirrhosis

SOCIAL HISTORY:
Smoking: Former smoker, approximately 45 pack-years, quit 10 years ago (reports brief relapse 5 years ago). Alcohol: Former heavy use (4-6 standard drinks daily), quit 5 years ago. Occupation: Retired construction foreman.

VITAL SIGNS:
His blood pressure is 145/90 mmHg. Oxygen saturation on room air is 86%.

PHYSICAL EXAMINATION:
On examination, he is anxious, tachypneic with a respiratory rate of 28 breaths per minute, and tachycardic with a heart rate of 122 beats per minute.
Auscultation of the chest reveals decreased breath sounds on the left side without wheezing. Percussion of the left chest is hyperresonant. The rest of the physical examination is unremarkable.

IMAGING:
A chest X-ray is performed immediately.
