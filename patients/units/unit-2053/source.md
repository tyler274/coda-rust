# synthetic_hospital patient 2053

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex M, Non-Hispanic White, Medicare (disability status) and Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-07-31 (Mon) | outpatient | Pulmonary & Critical Care |
| 1 | 2020-04-15 | 2023-10-25 (Wed) | outpatient | Pulmonary & Critical Care |
| 2 | 2020-07-15 | 2024-02-05 (Mon) | outpatient | Pulmonary & Critical Care |
| 3 | 2020-10-15 | 2024-04-22 (Mon) | outpatient | Pulmonary & Critical Care |
| 4 | 2021-04-15 | 2024-10-23 (Wed) | outpatient | Otolaryngology |
| 5 | 2022-01-15 | 2025-08-02 (Sat) | inpatient | Pulmonary & Critical Care |
| 6 | 2022-07-15 | 2026-01-23 (Fri) | ed | Emergency Medicine |
| 7 | 2023-01-15 | 2026-08-04 (Tue) | icu | Critical Care |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare (disability status) and Medicaid",
 "occupation": "Automotive refinishing technician (spray paint operator)",
 "alcohol_use": "Heavy daily alcohol use (approximately 6-8 beers daily), history of alcohol use disorder with prior withdrawal delirium requiring hospitalization 2 years prior",
 "family_history": [
  "Father with early-onset emphysema (diagnosed age 45, deceased)",
  "Paternal uncle with cirrhosis of unknown etiology",
  "Mother with hypertension and type 2 diabetes",
  "Family history positive for alcohol use disorder (father, brother)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 20 pack-years, quit 5 years ago",
 "home_medications": [
  {
   "dose": "2 mg orally twice daily",
   "name": "Tacrolimus"
  },
  {
   "dose": "750 mg orally twice daily",
   "name": "Mycophenolate mofetil"
  },
  {
   "dose": "5 mg orally daily",
   "name": "Prednisone"
  },
  {
   "dose": "25 mg orally daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "36,000 units orally with meals and snacks",
   "name": "Pancrelipase"
  },
  {
   "dose": "90 mcg (2 puffs) inhaled every 4-6 hours as needed for wheezing",
   "name": "Albuterol HFA inhaler"
  },
  {
   "dose": "250/50 mcg (1 inhalation) twice daily",
   "name": "Fluticasone-salmeterol inhaler"
  }
 ],
 "surgical_history": [
  "Bilateral lung transplantation (age 43)",
  "Multiple dental extractions",
  "Hospital admission for alcohol withdrawal (age 43)"
 ],
 "chronic_conditions": [
  "Alpha-1 antitrypsin deficiency",
  "Systemic arterial hypertension",
  "Status post bilateral lung transplantation (2 years prior)",
  "Chronic pancreatitis secondary to alcohol use",
  "Severe bronchiectasis",
  "Childhood asthma (resolved in teenage years, now recurrent with occupational triggers)",
  "Poor dentition with multiple missing teeth and chronic periodontal disease"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Idiopathic Pulmonary Fibrosis", "Acute lung transplant rejection", "Alpha-1 antitrypsin deficiency", "Pancreaticopleural fistula", "Delirium tremens", "Bronchiectasis", "Cervicofacial actinomycosis", "Asthma"]
Comorbidities: ["Hypoxemia", "History of recent pneumonia"]

## Encounter 0: source 2020-01-15, practice 2023-07-31, outpatient, Pulmonary & Critical Care

Chief complaint: 6-month history of progressive shortness of breath and wheezing

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonary & Critical Care
Visit Type: outpatient

CHIEF COMPLAINT:
6-month history of progressive shortness of breath and wheezing

PAST MEDICAL HISTORY:
Active Problem List:
- Alpha-1 antitrypsin deficiency
- Systemic arterial hypertension
- Status post bilateral lung transplantation (2 years prior)
- Chronic pancreatitis secondary to alcohol use
- Severe bronchiectasis
- Childhood asthma (resolved in teenage years, now recurrent with occupational triggers)
- Poor dentition with multiple missing teeth and chronic periodontal disease

His past medical history is significant for childhood asthma that resolved in his teenage years.

PAST SURGICAL HISTORY:
- Bilateral lung transplantation (age 43)
- Multiple dental extractions
- Hospital admission for alcohol withdrawal (age 43)

MEDICATIONS:
Home Medications:
- Tacrolimus 2 mg orally twice daily
- Mycophenolate mofetil 750 mg orally twice daily
- Prednisone 5 mg orally daily
- Hydrochlorothiazide 25 mg orally daily
- Pancrelipase 36,000 units orally with meals and snacks
- Albuterol HFA inhaler 90 mcg (2 puffs) inhaled every 4-6 hours as needed for wheezing
- Fluticasone-salmeterol inhaler 250/50 mcg (1 inhalation) twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
His family history is notable for his father who died at the age of 52 from liver cirrhosis and a sister with early-onset emphysema.

SOCIAL HISTORY:
He denies any history of smoking or occupational exposures.

PHYSICAL EXAMINATION:
On physical examination, he has decreased breath sounds and prolonged expiration. His liver is palpable 2 cm below the right costal margin.

LABORATORY DATA:
Laboratory tests reveal:

- Serum α1 antitrypsin level: 50 mg/dL (Normal: 100-190 mg/dL)
- Liver function tests:
- AST: 68 U/L (Normal: 10-40 U/L)
- ALT: 75 U/L (Normal: 7-56 U/L)
- Alkaline phosphatase: 120 U/L (Normal: 40-130 U/L)
- Total bilirubin: 1.2 mg/dL (Normal: 0.3-1.2 mg/dL)

OTHER STUDIES:
Pulmonary function tests show a decreased FEV1/FVC ratio with minimal response to bronchodilators.

## Encounter 1: source 2020-04-15, practice 2023-10-25, outpatient, Pulmonary & Critical Care

Chief complaint: 6-month history of episodic shortness of breath, wheezing, and cough

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Pulmonary & Critical Care
Visit Type: outpatient

CHIEF COMPLAINT:
6-month history of episodic shortness of breath, wheezing, and cough

HISTORY OF PRESENT ILLNESS:
Status post bilateral lung transplantation, the patient returns for follow-up three months after initial assessment. The symptoms are worse during the workweek and improve on weekends and vacations.

PAST MEDICAL HISTORY:
Active Problem List:
- Alpha-1 antitrypsin deficiency (diagnosed 2020-01-15)
- Systemic arterial hypertension
- Status post bilateral lung transplantation (2 years prior)
- Chronic pancreatitis secondary to alcohol use
- Severe bronchiectasis
- Childhood asthma (resolved in teenage years, now recurrent with occupational triggers)
- Poor dentition with multiple missing teeth and chronic periodontal disease

PAST SURGICAL HISTORY:
- Bilateral lung transplantation (age 43)
- Multiple dental extractions
- Hospital admission for alcohol withdrawal (age 43)

MEDICATIONS:
Home Medications:
- Tacrolimus 2 mg orally twice daily
- Mycophenolate mofetil 750 mg orally twice daily
- Prednisone 5 mg orally daily
- Hydrochlorothiazide 25 mg orally daily
- Pancrelipase 36,000 units orally with meals and snacks
- Albuterol HFA inhaler 90 mcg (2 puffs) inhaled every 4-6 hours as needed for wheezing
- Fluticasone-salmeterol inhaler 250/50 mcg (1 inhalation) twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with early-onset emphysema (diagnosed age 45, deceased)
- Paternal uncle with cirrhosis of unknown etiology
- Mother with hypertension and type 2 diabetes
- Family history positive for alcohol use disorder (father, brother)

SOCIAL HISTORY:
He has a 10-pack-year smoking history but quit 5 years ago.

PHYSICAL EXAMINATION:
On physical examination, he is in no acute distress. Lung auscultation reveals bilateral expiratory wheezes.

OTHER STUDIES:
Skin testing for common aeroallergens is negative. Spirometry shows a forced expiratory volume in 1 second (FEV1) of 75% predicted, which improves to 85% predicted after administration of a bronchodilator. Peak expiratory flow rate (PEFR) monitoring is initiated, and the following results are obtained:

- Monday (at work): 350 L/min
- Tuesday (at work): 340 L/min
- Wednesday (at work): 330 L/min
- Thursday (at work): 320 L/min
- Friday (at work): 310 L/min
- Saturday (at home): 400 L/min
- Sunday (at home): 410 L/min

## Encounter 2: source 2020-07-15, practice 2024-02-05, outpatient, Pulmonary & Critical Care

Chief complaint: Chronic cough and intermittent hemoptysis for the past 8 months

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonary & Critical Care
Visit Type: outpatient

CHIEF COMPLAINT:
Chronic cough and intermittent hemoptysis for the past 8 months

HISTORY OF PRESENT ILLNESS:
The patient is a 45-year-old male with Alpha-1 antitrypsin deficiency and severe bronchiectasis status post bilateral lung transplantation two years prior, who returns to the pulmonary clinic for evaluation of chronic cough and intermittent hemoptysis. He was last evaluated in April 2020 for episodic shortness of breath and wheezing and in January 2020 for progressive dyspnea. He reports producing large volumes of thick, yellow-green sputum every morning, which he describes as "foul-smelling and disgusting." He frequently experiences shortness of breath on exertion and has noticed worsening fatigue and occasional low-grade fevers. He denies night sweats or weight loss. He has had multiple respiratory infections over the past year, some requiring oral antibiotics.

PAST MEDICAL HISTORY:
Active Problem List:
- Alpha-1 antitrypsin deficiency (diagnosed 2020-01-15)
- Asthma (diagnosed 2020-04-15)
- Systemic arterial hypertension
- Status post bilateral lung transplantation (2 years prior)
- Chronic pancreatitis secondary to alcohol use
- Severe bronchiectasis
- Childhood asthma (resolved in teenage years, now recurrent with occupational triggers)
- Poor dentition with multiple missing teeth and chronic periodontal disease

PAST SURGICAL HISTORY:
- Bilateral lung transplantation (age 43)
- Multiple dental extractions
- Hospital admission for alcohol withdrawal (age 43)

MEDICATIONS:
Home Medications:
- Tacrolimus 2 mg orally twice daily
- Mycophenolate mofetil 750 mg orally twice daily
- Prednisone 5 mg orally daily
- Hydrochlorothiazide 25 mg orally daily
- Pancrelipase 36,000 units orally with meals and snacks
- Albuterol HFA inhaler 90 mcg (2 puffs) inhaled every 4-6 hours as needed for wheezing
- Fluticasone-salmeterol inhaler 250/50 mcg (1 inhalation) twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with early-onset emphysema (diagnosed age 45, deceased)
- Paternal uncle with cirrhosis of unknown etiology
- Mother with hypertension and type 2 diabetes
- Family history positive for alcohol use disorder (father, brother)

SOCIAL HISTORY:
He is a former smoker with a 20-pack-year history. He has no recent travel or known tuberculosis exposure.

VITAL SIGNS:
He is afebrile

PHYSICAL EXAMINATION:
Physical exam reveals bilateral coarse crackles, digital clubbing, and no lymphadenopathy. He appears tired and mildly tachypneic.

LABORATORY DATA:
Laboratory results are unremarkable, and sputum cultures are pending.

IMAGING:
A high-resolution chest CT scan is obtained and shown below.

## Encounter 3: source 2020-10-15, practice 2024-04-22, outpatient, Pulmonary & Critical Care

Chief complaint: Progressive exertional dyspnea and dry cough over 6 months

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Jennifer Kim, MD
Department: Pulmonary & Critical Care
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive exertional dyspnea and dry cough over 6 months

HISTORY OF PRESENT ILLNESS:
The patient returns for ongoing evaluation of respiratory symptoms following prior presentations this year. Status post bilateral lung transplantation for Alpha-1 antitrypsin deficiency, he presents with a 6-month history of progressive exertional dyspnea and a dry cough. He denies any chest pain, fever, or weight loss. The patient is concerned about the implications of these findings and asks about the likelihood that his condition is due to a random occurrence rather than a significant underlying disease.

PAST MEDICAL HISTORY:
Active Problem List:
- Alpha-1 antitrypsin deficiency (diagnosed 2020-01-15)
- Asthma (diagnosed 2020-04-15)
- Bronchiectasis (diagnosed 2020-07-15)
- Systemic arterial hypertension
- Status post bilateral lung transplantation (2 years prior)
- Chronic pancreatitis secondary to alcohol use
- Childhood asthma (resolved in teenage years, now recurrent with occupational triggers)
- Poor dentition with multiple missing teeth and chronic periodontal disease

His past medical history is significant for hypertension,.

PAST SURGICAL HISTORY:
- Bilateral lung transplantation (age 43)
- Multiple dental extractions
- Hospital admission for alcohol withdrawal (age 43)

MEDICATIONS:
He takes for which he takes hydrochlorothiazide.

Home Medications:
- Tacrolimus 2 mg orally twice daily
- Mycophenolate mofetil 750 mg orally twice daily
- Prednisone 5 mg orally daily
- Pancrelipase 36,000 units orally with meals and snacks
- Albuterol HFA inhaler 90 mcg (2 puffs) inhaled every 4-6 hours as needed for wheezing
- Fluticasone-salmeterol inhaler 250/50 mcg (1 inhalation) twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with early-onset emphysema (diagnosed age 45, deceased)
- Paternal uncle with cirrhosis of unknown etiology
- Mother with hypertension and type 2 diabetes
- Family history positive for alcohol use disorder (father, brother)

SOCIAL HISTORY:
He has a 20-pack-year smoking history but quit smoking 5 years ago.

VITAL SIGNS:
On physical examination, his blood pressure is 135/85 mmHg, heart rate is 78 beats per minute, and oxygen saturation is 94% on room air.

PHYSICAL EXAMINATION:
Lung auscultation reveals bibasilar inspiratory crackles.

IMAGING:
A high-resolution CT scan of the chest shows reticular opacities with honeycombing predominantly at the lung bases.

OTHER STUDIES:
Pulmonary function tests reveal a forced vital capacity (FVC) of 2.8 L (70% of predicted), a total lung capacity (TLC) of 4.2 L (80% of predicted), and a diffusing capacity for carbon monoxide (DLCO) of 15 mL/min/mmHg (60% of predicted).

## Encounter 4: source 2021-04-15, practice 2024-10-23, outpatient, Otolaryngology

Chief complaint: Slowly enlarging mass along jawline present for 3 weeks

ENCOUNTER NOTE
Date: 2021-04-15
Provider: Dr. David Patel, MD
Department: Otolaryngology
Visit Type: outpatient

CHIEF COMPLAINT:
Slowly enlarging mass along jawline present for 3 weeks

HISTORY OF PRESENT ILLNESS:
This 46-year-old male with alpha-1 antitrypsin deficiency and status post bilateral lung transplantation two years ago, currently maintained on immunosuppressive therapy, presents to the otolaryngology clinic with a slowly enlarging mass along his jawline that has been present for 3 weeks. He reports intermittent pain, swelling, and occasional drainage of thick yellow fluid. He denies fever or recent dental work but mentions having poor oral hygiene and several missing teeth. He has no history of trauma or travel. He was treated with oral clindamycin by a local urgent care provider 2 weeks ago without improvement.

PAST MEDICAL HISTORY:
Active Problem List:
- Alpha-1 antitrypsin deficiency (diagnosed 2020-01-15)
- Asthma (diagnosed 2020-04-15)
- Bronchiectasis (diagnosed 2020-07-15)
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-10-15)
- Systemic arterial hypertension
- Status post bilateral lung transplantation (2 years prior)
- Chronic pancreatitis secondary to alcohol use
- Childhood asthma (resolved in teenage years, now recurrent with occupational triggers)
- Poor dentition with multiple missing teeth and chronic periodontal disease

PAST SURGICAL HISTORY:
- Bilateral lung transplantation (age 43)
- Multiple dental extractions
- Hospital admission for alcohol withdrawal (age 43)

MEDICATIONS:
Home Medications:
- Tacrolimus 2 mg orally twice daily
- Mycophenolate mofetil 750 mg orally twice daily
- Prednisone 5 mg orally daily
- Hydrochlorothiazide 25 mg orally daily
- Pancrelipase 36,000 units orally with meals and snacks
- Albuterol HFA inhaler 90 mcg (2 puffs) inhaled every 4-6 hours as needed for wheezing
- Fluticasone-salmeterol inhaler 250/50 mcg (1 inhalation) twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with early-onset emphysema (diagnosed age 45, deceased)
- Paternal uncle with cirrhosis of unknown etiology
- Mother with hypertension and type 2 diabetes
- Family history positive for alcohol use disorder (father, brother)

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 5 years ago. Alcohol: Heavy daily alcohol use (approximately 6-8 beers daily), history of alcohol use disorder with prior withdrawal delirium requiring hospitalization 2 years prior. Occupation: Automotive refinishing technician (spray paint operator).

PHYSICAL EXAMINATION:
On physical examination, there is a firm, indurated mass near the left angle of the mandible with multiple small sinus tracts draining yellowish granular pus. Gentle pressure expresses more granules. Cervical lymph nodes are not enlarged.

LABORATORY DATA:
Gram stain of the discharge shows Actinomyces spp .

## Encounter 5: source 2022-01-15, practice 2025-08-02, inpatient, Pulmonary & Critical Care

Chief complaint: Progressive dyspnea and nonproductive cough with low-grade fever

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Emily Johnson, MD
Department: Pulmonary & Critical Care
Visit Type: inpatient

CHIEF COMPLAINT:
Progressive dyspnea and nonproductive cough with low-grade fever

HISTORY OF PRESENT ILLNESS:
This 47-year-old male with alpha-1 antitrypsin deficiency status post bilateral lung transplantation presents to the clinic with a 2-week history of progressive dyspnea and a nonproductive cough. He reports a low-grade fever for the past few days.

PAST MEDICAL HISTORY:
Active Problem List:
- Alpha-1 antitrypsin deficiency (diagnosed 2020-01-15)
- Asthma (diagnosed 2020-04-15)
- Bronchiectasis (diagnosed 2020-07-15)
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-10-15)
- Cervicofacial actinomycosis (diagnosed 2021-04-15)
- Systemic arterial hypertension
- Status post bilateral lung transplantation (2 years prior)
- Chronic pancreatitis secondary to alcohol use
- Childhood asthma (resolved in teenage years, now recurrent with occupational triggers)
- Poor dentition with multiple missing teeth and chronic periodontal disease

PAST SURGICAL HISTORY:
He underwent a lung transplant 4 months ago for idiopathic pulmonary fibrosis. His postoperative course was uneventful,

MEDICATIONS:
He takes he has been compliant with his immunosuppressive regimen.

Home Medications:
- Tacrolimus 2 mg orally twice daily
- Mycophenolate mofetil 750 mg orally twice daily
- Prednisone 5 mg orally daily
- Hydrochlorothiazide 25 mg orally daily
- Pancrelipase 36,000 units orally with meals and snacks
- Albuterol HFA inhaler 90 mcg (2 puffs) inhaled every 4-6 hours as needed for wheezing
- Fluticasone-salmeterol inhaler 250/50 mcg (1 inhalation) twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with early-onset emphysema (diagnosed age 45, deceased)
- Paternal uncle with cirrhosis of unknown etiology
- Mother with hypertension and type 2 diabetes
- Family history positive for alcohol use disorder (father, brother)

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 5 years ago. Alcohol: Heavy daily alcohol use (approximately 6-8 beers daily), history of alcohol use disorder with prior withdrawal delirium requiring hospitalization 2 years prior. Occupation: Automotive refinishing technician (spray paint operator).

VITAL SIGNS:
On examination, his temperature is 37.8°C (100°F), blood pressure is 130/85 mmHg, heart rate is 92 beats/min, respiratory rate is 22 breaths/min, and oxygen saturation is 89% on room air.

PHYSICAL EXAMINATION:
Auscultation of the lungs reveals bilateral crackles.

LABORATORY DATA:
Laboratory tests reveal:
- White blood cell count: 11,000/µL
- Hemoglobin: 13.5 g/dL
- Platelet count: 250,000/µL
- Serum creatinine: 1.1 mg/dL
- Blood urea nitrogen: 21 mg/dL
- C-reactive protein: 25 mg/L (normal <10 mg/L)
- Blood cultures: No growth to date

IMAGING:
Chest x-ray shows perihilar opacities and interstitial edema.

ASSESSMENT AND PLAN:
Given the patient's history and presentation,

## Encounter 6: source 2022-07-15, practice 2026-01-23, ed, Emergency Medicine

Chief complaint: Progressive dyspnea and sharp left-sided chest pain worsening with inspiration

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. James Wong, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive dyspnea and sharp left-sided chest pain worsening with inspiration

HISTORY OF PRESENT ILLNESS:
This 47-year-old male with chronic pancreatitis, two years post bilateral lung transplant, presents with a 2-week history of progressive dyspnea and sharp left-sided chest pain worsening with deep breaths and relieved by sitting forward. He denies fever, chills, or cough.

PAST MEDICAL HISTORY:
Active Problem List:
- Alpha-1 antitrypsin deficiency (diagnosed 2020-01-15)
- Asthma (diagnosed 2020-04-15)
- Bronchiectasis (diagnosed 2020-07-15)
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-10-15)
- Cervicofacial actinomycosis (diagnosed 2021-04-15)
- Acute lung transplant rejection (diagnosed 2022-01-15)
- Systemic arterial hypertension
- Status post bilateral lung transplantation (2 years prior)
- Chronic pancreatitis secondary to alcohol use
- Childhood asthma (resolved in teenage years, now recurrent with occupational triggers)
- Poor dentition with multiple missing teeth and chronic periodontal disease

PAST SURGICAL HISTORY:
- Bilateral lung transplantation (age 43)
- Multiple dental extractions
- Hospital admission for alcohol withdrawal (age 43)

MEDICATIONS:
Home Medications:
- Tacrolimus 2 mg orally twice daily
- Mycophenolate mofetil 750 mg orally twice daily
- Prednisone 5 mg orally daily
- Hydrochlorothiazide 25 mg orally daily
- Pancrelipase 36,000 units orally with meals and snacks
- Albuterol HFA inhaler 90 mcg (2 puffs) inhaled every 4-6 hours as needed for wheezing
- Fluticasone-salmeterol inhaler 250/50 mcg (1 inhalation) twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with early-onset emphysema (diagnosed age 45, deceased)
- Paternal uncle with cirrhosis of unknown etiology
- Mother with hypertension and type 2 diabetes
- Family history positive for alcohol use disorder (father, brother)

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 5 years ago. Alcohol: Heavy daily alcohol use (approximately 6-8 beers daily), history of alcohol use disorder with prior withdrawal delirium requiring hospitalization 2 years prior. Occupation: Automotive refinishing technician (spray paint operator).

VITAL SIGNS:
Afebrile, with a respiratory rate of 22 breaths per minute, heart rate of 110 beats per minute, and oxygen saturation of 92% on room air

PHYSICAL EXAMINATION:
Auscultation of the chest reveals decreased breath sounds at the left base with dullness to percussion

LABORATORY DATA:
Thoracentesis yields an exudative effusion with the following characteristics:

- Appearance: Cloudy
- pH: 7.20
- Protein (pleural): 4.5 g/dL
- LDH (pleural): 600 U/L
- Amylase (pleural): 2500 U/L
- Glucose (pleural): 60 mg/dL

Serum amylase and lipase levels are elevated.

IMAGING:
Chest X-ray shows a large left-sided pleural effusion

## Encounter 7: source 2023-01-15, practice 2026-08-04, icu, Critical Care

Chief complaint: Episodes of shaking, nausea, vomiting, and visual hallucinations

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Maria Gonzalez, MD
Department: Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Episodes of shaking, nausea, vomiting, and visual hallucinations

HISTORY OF PRESENT ILLNESS:
The patient is a 48-year-old male with chronic pancreatitis secondary to alcohol use who was treated six months prior for a pancreaticopleural fistula. He is brought to the emergency department by his partner due to episodes of shaking, nausea, and vomiting over the past day. He has been irritable and unable to sleep, frequently pacing around the house and complaining of seeing shadows move in the corners of the room. The partner reports he has not eaten in two days and last consumed his usual daily beverages about 36 hours ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Alpha-1 antitrypsin deficiency (diagnosed 2020-01-15)
- Asthma (diagnosed 2020-04-15)
- Bronchiectasis (diagnosed 2020-07-15)
- Idiopathic Pulmonary Fibrosis (diagnosed 2020-10-15)
- Cervicofacial actinomycosis (diagnosed 2021-04-15)
- Acute lung transplant rejection (diagnosed 2022-01-15)
- Pancreaticopleural fistula (diagnosed 2022-07-15)
- Systemic arterial hypertension
- Status post bilateral lung transplantation (2 years prior)
- Chronic pancreatitis secondary to alcohol use
- Childhood asthma (resolved in teenage years, now recurrent with occupational triggers)
- Poor dentition with multiple missing teeth and chronic periodontal disease

His past medical history is notable for a prior hospital admission two years ago for similar symptoms, though he has not consistently followed up with a physician.

PAST SURGICAL HISTORY:
- Bilateral lung transplantation (age 43)
- Multiple dental extractions
- Hospital admission for alcohol withdrawal (age 43)

MEDICATIONS:
Home Medications:
- Tacrolimus 2 mg orally twice daily
- Mycophenolate mofetil 750 mg orally twice daily
- Prednisone 5 mg orally daily
- Hydrochlorothiazide 25 mg orally daily
- Pancrelipase 36,000 units orally with meals and snacks
- Albuterol HFA inhaler 90 mcg (2 puffs) inhaled every 4-6 hours as needed for wheezing
- Fluticasone-salmeterol inhaler 250/50 mcg (1 inhalation) twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with early-onset emphysema (diagnosed age 45, deceased)
- Paternal uncle with cirrhosis of unknown etiology
- Mother with hypertension and type 2 diabetes
- Family history positive for alcohol use disorder (father, brother)

SOCIAL HISTORY:
He has a long-standing pattern of heavy daily alcohol use

VITAL SIGNS:
Vital signs on admission reveal an elevated heart rate, low-grade fever, and increased blood pressure.

PHYSICAL EXAMINATION:
Physical examination reveals diaphoretic skin and a coarse tremor in both hands. During evaluation, he develops a generalized tonic-clonic seizure that resolves spontaneously.

LABORATORY DATA:
Basic labs show mild hypokalemia, hypomagnesemia, and an elevated anion gap.

IMAGING:
There is no evidence of head trauma on imaging

OTHER STUDIES:
Toxicology screen is negative for recreational drugs

ASSESSMENT AND PLAN:
He is now showing signs consistent with alcohol withdrawal

He is stabilized and admitted to the hospital with close monitoring and symptom-targeted therapy.
