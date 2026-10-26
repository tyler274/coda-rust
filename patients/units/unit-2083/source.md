# synthetic_hospital patient 2083

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 58 at first encounter, sex M, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-10-12 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-09-15 | 2021-06-11 (Fri) | outpatient | Gastroenterology |
| 2 | 2022-03-15 | 2022-12-23 (Fri) | outpatient | Cardiology |
| 3 | 2023-11-15 | 2024-08-19 (Mon) | ed | Emergency Medicine |
| 4 | 2024-11-15 | 2025-08-23 (Sat) | ed | Emergency Medicine |
| 5 | 2025-10-15 | 2026-07-13 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Sales representative (requires periodic business travel)",
 "alcohol_use": "Occasional (1-2 drinks per week)",
 "family_history": [
  "Father: Lung cancer (deceased age 62)",
  "Mother: Schizophrenia",
  "Brother: COPD"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Active smoker, 40 pack-year history (1 pack per day)",
 "home_medications": [
  {
   "dose": "90 mcg, 2 inhalations every 4-6 hours as needed for shortness of breath",
   "name": "Albuterol sulfate inhaler"
  },
  {
   "dose": "34 mcg, 2 inhalations four times daily",
   "name": "Ipratropium bromide inhaler"
  },
  {
   "dose": "500 mg chewable tablet as needed for heartburn",
   "name": "Calcium carbonate (Tums)"
  },
  {
   "dose": "3 mg by mouth at bedtime (prescribed, adherence variable)",
   "name": "Risperidone"
  },
  {
   "dose": "1 tablet by mouth daily",
   "name": "Multivitamin"
  },
  {
   "dose": "200-400 mg by mouth every 6 hours as needed for pain",
   "name": "Ibuprofen"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Chronic obstructive pulmonary disease (COPD)",
  "Obstructive sleep apnea (non-compliant with CPAP)",
  "Schizophrenia, chronic paranoid type",
  "Gastroesophageal reflux disease (GERD)",
  "Tobacco use disorder"
 ],
 "age_at_first_encounter": 58
}
```

Primary diagnoses: ["Cor pulmonale (chronic right-sided heart failure secondary to pulmonary disease)", "Lung cancer screening candidate (high-risk smoker)", "Legionnaires' disease", "Schizophrenia with acute exacerbation", "Esophageal stricture", "Superior pulmonary sulcus tumor (Pancoast tumor)"]
Comorbidities: ["Chronic obstructive pulmonary disease", "Tobacco use disorder"]

## Encounter 0: source 2020-01-15, practice 2020-10-12, outpatient, Internal Medicine

Chief complaint: routine check-up with inquiry regarding lung cancer screening given smoking history

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine check-up with inquiry regarding lung cancer screening given smoking history

HISTORY OF PRESENT ILLNESS:
He has no complaints but is concerned about his risk of lung cancer given his smoking history. The patient inquires about screening for lung cancer.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (COPD)
- Obstructive sleep apnea (non-compliant with CPAP)
- Schizophrenia, chronic paranoid type
- Gastroesophageal reflux disease (GERD)
- Tobacco use disorder

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Albuterol sulfate inhaler 90 mcg, 2 inhalations every 4-6 hours as needed for shortness of breath
- Ipratropium bromide inhaler 34 mcg, 2 inhalations four times daily
- Calcium carbonate (Tums) 500 mg chewable tablet as needed for heartburn
- Risperidone 3 mg by mouth at bedtime (prescribed, adherence variable)
- Multivitamin 1 tablet by mouth daily
- Ibuprofen 200-400 mg by mouth every 6 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (deceased age 62)
- Mother: Schizophrenia
- Brother: COPD

SOCIAL HISTORY:
History of smoking

PHYSICAL EXAMINATION:
His physical examination is unremarkable.

IMAGING:
His recent chest X-ray showed no acute findings.

PLAN:
You discuss the potential benefits and risks of low-dose computed tomography (LDCT) screening and explain that the criteria for screening have recently been updated to include younger individuals and those with a lesser smoking history. You mention that this change in criteria will affect the sensitivity and specificity of the screening.

## Encounter 1: source 2020-09-15, practice 2021-06-11, outpatient, Gastroenterology

Chief complaint: progressive difficulty swallowing solid foods over the past 6 months

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Raj Patel, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive difficulty swallowing solid foods over the past 6 months

HISTORY OF PRESENT ILLNESS:
Patient with GERD and tobacco use seen in January 2020 for lung cancer screening presents to clinic with a 6-month history of progressive difficulty swallowing solids. He denies any pain with swallowing or significant weight loss.

PAST MEDICAL HISTORY:
Active Problem List:
- Lung cancer screening candidate (high-risk smoker) (diagnosed 2020-01-15)
- Chronic obstructive pulmonary disease (COPD)
- Obstructive sleep apnea (non-compliant with CPAP)
- Schizophrenia, chronic paranoid type
- Gastroesophageal reflux disease (GERD)
- Tobacco use disorder

He has a history of chronic heartburn.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He takes for which he takes over-the-counter antacids with partial relief.

Home Medications:
- Albuterol sulfate inhaler 90 mcg, 2 inhalations every 4-6 hours as needed for shortness of breath
- Ipratropium bromide inhaler 34 mcg, 2 inhalations four times daily
- Calcium carbonate (Tums) 500 mg chewable tablet as needed for heartburn
- Risperidone 3 mg by mouth at bedtime (prescribed, adherence variable)
- Multivitamin 1 tablet by mouth daily
- Ibuprofen 200-400 mg by mouth every 6 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (deceased age 62)
- Mother: Schizophrenia
- Brother: COPD

SOCIAL HISTORY:
He has a 30-pack-year smoking history and consumes alcohol occasionally.

VITAL SIGNS:
Vital signs are within normal limits.

PHYSICAL EXAMINATION:
On physical examination, he appears well-nourished and

OTHER STUDIES:
An upper endoscopy is performed, which reveals a symmetric and circumferential narrowing of the distal esophagus with a smooth mucosal surface. Biopsies are taken from the narrowed area.

## Encounter 2: source 2022-03-15, practice 2022-12-23, outpatient, Cardiology

Chief complaint: progressive shortness of breath and bilateral leg swelling over the past several months

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. Michael Torres, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive shortness of breath and bilateral leg swelling over the past several months

HISTORY OF PRESENT ILLNESS:
The patient with established COPD, non-compliant obstructive sleep apnea, and prior evaluation for esophageal stricture returns for cardiology assessment of progressive symptoms. He reports increasing fatigue and exertional dyspnea, requiring rest after walking half a block.

PAST MEDICAL HISTORY:
Active Problem List:
- Lung cancer screening candidate (high-risk smoker) (diagnosed 2020-01-15)
- Esophageal stricture (diagnosed 2020-09-15)
- Chronic obstructive pulmonary disease (COPD)
- Obstructive sleep apnea (non-compliant with CPAP)
- Schizophrenia, chronic paranoid type
- Gastroesophageal reflux disease (GERD)
- Tobacco use disorder

His past medical history includes a 40-year history of smoking and chronic cough productive of white sputum. He also has long-standing obstructive sleep apnea but does not use CPAP regularly.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Albuterol sulfate inhaler 90 mcg, 2 inhalations every 4-6 hours as needed for shortness of breath
- Ipratropium bromide inhaler 34 mcg, 2 inhalations four times daily
- Calcium carbonate (Tums) 500 mg chewable tablet as needed for heartburn
- Risperidone 3 mg by mouth at bedtime (prescribed, adherence variable)
- Multivitamin 1 tablet by mouth daily
- Ibuprofen 200-400 mg by mouth every 6 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (deceased age 62)
- Mother: Schizophrenia
- Brother: COPD

SOCIAL HISTORY:
Smoking: Active smoker, 40 pack-year history (1 pack per day). Alcohol: Occasional (1-2 drinks per week). Occupation: Sales representative (requires periodic business travel).

VITAL SIGNS:
His blood pressure is 120/76 mmHg, pulse 84/min, and respiratory rate 18/min.

PHYSICAL EXAMINATION:
On examination, he appears mildly dyspneic. jugular venous pressure (JVP) is 6cm above the sternal angle, and there is pitting edema of the ankles. Auscultation reveals wheezing and a prominent second heart sound (P2).

IMAGING:
A chest radiograph shows hyperinflated lungs and a long, narrow cardiac silhouette. Echocardiography shows signs of right ventricular hypertrophy.

OTHER STUDIES:
Pulmonary artery catheterization reveals elevated pulmonary artery pressure and normal pulmonary capillary wedge pressure (PCWP).

PLAN:
The patient is started on a vasodilator targeting pulmonary arterial hypertension.

## Encounter 3: source 2023-11-15, practice 2024-08-19, ed, Emergency Medicine

Chief complaint: severe, unrelenting pain in left shoulder progressively worsening over the past 3 months with persistent cough and weight loss

ENCOUNTER NOTE
Date: 2023-11-15
Provider: Dr. Emily Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe, unrelenting pain in left shoulder progressively worsening over the past 3 months with persistent cough and weight loss

HISTORY OF PRESENT ILLNESS:
Patient with COPD and prior lung screening candidacy presents. He also reports persistent cough and unintentional weight loss of 10 pounds during this period.

PAST MEDICAL HISTORY:
Active Problem List:
- Lung cancer screening candidate (high-risk smoker) (diagnosed 2020-01-15)
- Esophageal stricture (diagnosed 2020-09-15)
- Cor pulmonale (chronic right-sided heart failure secondary to pulmonary disease) (diagnosed 2022-03-15)
- Chronic obstructive pulmonary disease (COPD)
- Obstructive sleep apnea (non-compliant with CPAP)
- Schizophrenia, chronic paranoid type
- Gastroesophageal reflux disease (GERD)
- Tobacco use disorder

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Albuterol sulfate inhaler 90 mcg, 2 inhalations every 4-6 hours as needed for shortness of breath
- Ipratropium bromide inhaler 34 mcg, 2 inhalations four times daily
- Calcium carbonate (Tums) 500 mg chewable tablet as needed for heartburn
- Risperidone 3 mg by mouth at bedtime (prescribed, adherence variable)
- Multivitamin 1 tablet by mouth daily
- Ibuprofen 200-400 mg by mouth every 6 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (deceased age 62)
- Mother: Schizophrenia
- Brother: COPD

SOCIAL HISTORY:
40-pack-year smoking history

PHYSICAL EXAMINATION:
On physical examination, ptosis and miosis are noted on the left side of his face, along with decreased sweating. Neurological examination reveals weakness in the muscles of the left hand and forearm, as well as diminished sensation over the ulnar distribution.

LABORATORY DATA:
Laboratory Values:
- Hemoglobin: 13.2 g/dL
- White blood cell count: 7,500 /μL
- Platelets: 250,000 /μL
- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Creatinine: 1.0 mg/dL
- Calcium: 9.8 mg/dL

IMAGING:
A chest X-ray reveals a mass in the apex of the left lung. CT scan of the chest confirms the presence of a 5 cm mass in the superior pulmonary sulcus with evidence of local invasion.

## Encounter 4: source 2024-11-15, practice 2025-08-23, ed, Emergency Medicine

Chief complaint: acute onset of auditory hallucinations and paranoid delusions

ENCOUNTER NOTE
Date: 2024-11-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset of auditory hallucinations and paranoid delusions

HISTORY OF PRESENT ILLNESS:
The patient is a 62-year-old male with schizophrenia, chronic obstructive pulmonary disease, and superior pulmonary sulcus tumor who presents to the emergency department with acute onset of auditory hallucinations and paranoid delusions. He has been non-compliant with his outpatient medications for several months.

PAST MEDICAL HISTORY:
Active Problem List:
- Lung cancer screening candidate (high-risk smoker) (diagnosed 2020-01-15)
- Esophageal stricture (diagnosed 2020-09-15)
- Cor pulmonale (chronic right-sided heart failure secondary to pulmonary disease) (diagnosed 2022-03-15)
- Superior pulmonary sulcus tumor (Pancoast tumor) (diagnosed 2023-11-15)
- Chronic obstructive pulmonary disease (COPD)
- Obstructive sleep apnea (non-compliant with CPAP)
- Schizophrenia, chronic paranoid type
- Gastroesophageal reflux disease (GERD)
- Tobacco use disorder

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
His current medications include a multivitamin and ibuprofen as needed

Home Medications:
- Albuterol sulfate inhaler 90 mcg, 2 inhalations every 4-6 hours as needed for shortness of breath
- Ipratropium bromide inhaler 34 mcg, 2 inhalations four times daily
- Calcium carbonate (Tums) 500 mg chewable tablet as needed for heartburn
- Risperidone 3 mg by mouth at bedtime (prescribed, adherence variable)

ALLERGIES:
NKDA

FAMILY HISTORY:
Family history of sudden cardiac death

SOCIAL HISTORY:
He smokes a pack of cigarettes daily

VITAL SIGNS:
His vital signs are stable

PHYSICAL EXAMINATION:
Physical examination is unremarkable except for mild psychomotor agitation

LABORATORY DATA:
Laboratory studies reveal normal electrolytes, renal function, and complete blood count

OTHER STUDIES:
An ECG is performed, which shows a QTc interval of 470 milliseconds

## Encounter 5: source 2025-10-15, practice 2026-07-13, ed, Emergency Medicine

Chief complaint: 3-day history of watery diarrhea, low-grade fever, and malaise followed by nonproductive cough and shortness of breath after recent business travel

ENCOUNTER NOTE
Date: 2025-10-15
Provider: Dr. Lisa Wong, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
3-day history of watery diarrhea, low-grade fever, and malaise followed by nonproductive cough and shortness of breath after recent business travel

HISTORY OF PRESENT ILLNESS:
This 63-year-old with COPD and Pancoast tumor presents with diarrhea, fever, and cough. He returned six days ago from a business trip where he stayed at a hotel with an indoor pool and sauna.

PAST MEDICAL HISTORY:
Active Problem List:
- Lung cancer screening candidate (high-risk smoker) (diagnosed 2020-01-15)
- Esophageal stricture (diagnosed 2020-09-15)
- Cor pulmonale (chronic right-sided heart failure secondary to pulmonary disease) (diagnosed 2022-03-15)
- Superior pulmonary sulcus tumor (Pancoast tumor) (diagnosed 2023-11-15)
- Schizophrenia with acute exacerbation (diagnosed 2024-11-15)
- Chronic obstructive pulmonary disease (COPD)
- Obstructive sleep apnea (non-compliant with CPAP)
- Schizophrenia, chronic paranoid type
- Gastroesophageal reflux disease (GERD)
- Tobacco use disorder

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Albuterol sulfate inhaler 90 mcg, 2 inhalations every 4-6 hours as needed for shortness of breath
- Ipratropium bromide inhaler 34 mcg, 2 inhalations four times daily
- Calcium carbonate (Tums) 500 mg chewable tablet as needed for heartburn
- Risperidone 3 mg by mouth at bedtime (prescribed, adherence variable)
- Multivitamin 1 tablet by mouth daily
- Ibuprofen 200-400 mg by mouth every 6 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (deceased age 62)
- Mother: Schizophrenia
- Brother: COPD

SOCIAL HISTORY:
He has a 30-pack-year smoking history.

VITAL SIGNS:
His temperature is 38.8°C (101.8°F), pulse is 92/min, respirations are 20/min, and blood pressure is 118/76 mm Hg.

PHYSICAL EXAMINATION:
Pulmonary examination reveals diffuse crackles bilaterally.

LABORATORY DATA:
Laboratory studies show a leukocyte count of 15,800/mm3 with neutrophil predominance and mild hyponatremia. Gram stain of sputum shows numerous neutrophils but no predominant organism.

IMAGING:
Chest x-ray reveals bilateral patchy alveolar infiltrates.
