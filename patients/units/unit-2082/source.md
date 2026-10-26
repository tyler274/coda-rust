# synthetic_hospital patient 2082

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex M, Non-Hispanic White, Commercial insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-03-28 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2022-09-28 (Wed) | outpatient | Pulmonology |
| 2 | 2022-07-15 | 2024-09-27 (Fri) | outpatient | Pulmonology |
| 3 | 2023-07-15 | 2025-09-14 (Sun) | ed | Emergency Medicine |
| 4 | 2024-01-15 | 2026-03-28 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "None known"
 ],
 "insurance": "Commercial insurance",
 "occupation": "Construction worker",
 "alcohol_use": "Occasional social use",
 "family_history": [
  "Father with chronic obstructive pulmonary disease",
  "Paternal uncle with lung cancer"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 30 pack-years",
 "home_medications": [
  {
   "dose": "18 mcg inhaled once daily",
   "name": "Tiotropium"
  },
  {
   "dose": "90 mcg inhaled every 4-6 hours as needed for shortness of breath",
   "name": "Albuterol"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Chronic obstructive pulmonary disease (COPD)",
  "Tobacco use disorder"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Syndrome of inappropriate antidiuretic hormone secretion (SIADH)", "Solitary pulmonary nodule with high suspicion for malignancy", "Suspected malignant neoplasm of lung"]
Comorbidities: ["Chronic obstructive pulmonary disease", "Tobacco use disorder", "Suspected malignant neoplasm of lung"]

## Encounter 0: source 2020-01-15, practice 2022-03-28, outpatient, Internal Medicine

Chief complaint: Routine annual physical examination with concerns regarding personal lung cancer risk compared to non-smokers following a friend's recent diagnosis.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Emily Watson, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine annual physical examination with concerns regarding personal lung cancer risk compared to non-smokers following a friend's recent diagnosis.

HISTORY OF PRESENT ILLNESS:
He has no complaints but is concerned about his risk of lung cancer given his smoking history. His friend, a 52-year-old male with a similar smoking history, was recently diagnosed with lung cancer. The patient is interested in understanding his risk of developing lung cancer compared to non-smokers.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (COPD)
- Tobacco use disorder

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Tiotropium 18 mcg inhaled once daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed for shortness of breath

ALLERGIES:
None known

FAMILY HISTORY:
He has no family history of cancer.

SOCIAL HISTORY:
With a 30-pack-year smoking history

VITAL SIGNS:
His blood pressure is 130/85 mmHg, heart rate is 78 bpm, and respiratory rate is 16 breaths per minute.

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

ASSESSMENT AND PLAN:
You decide to calculate the odds ratio (OR) for developing lung cancer associated with smoking using data from a case-control study. The study includes 100 lung cancer patients (cases) and 100 patients without lung cancer (controls). Among the cases, 90 were smokers and 10 were non-smokers. Among the controls, 30 were smokers and 70 were non-smokers.

## Encounter 1: source 2020-07-15, practice 2022-09-28, outpatient, Pulmonology

Chief complaint: Routine follow-up visit to review results of screening low-dose CT chest which incidentally discovered a solitary pulmonary nodule.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. David Kim, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up visit to review results of screening low-dose CT chest which incidentally discovered a solitary pulmonary nodule.

HISTORY OF PRESENT ILLNESS:
The patient, with known COPD and tobacco use disorder, returns for follow-up evaluation of an incidentally discovered solitary pulmonary nodule identified on recent screening CT. He previously presented in January 2020 with concerns regarding personal lung cancer risk. He currently reports no complaints but remains concerned about his risk of lung cancer given his smoking history.

PAST MEDICAL HISTORY:
Active Problem List:
- Suspected malignant neoplasm of lung (diagnosed 2020-01-15)
- Chronic obstructive pulmonary disease (COPD)
- Tobacco use disorder

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Tiotropium 18 mcg inhaled once daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed for shortness of breath

ALLERGIES:
None known

FAMILY HISTORY:
- Father with chronic obstructive pulmonary disease
- Paternal uncle with lung cancer

SOCIAL HISTORY:
History of smoking

PHYSICAL EXAMINATION:
His physical examination is unremarkable.

IMAGING:
You decide to perform a low-dose computed tomography (LDCT) scan for lung cancer screening, which reveals a 6 mm non-calcified nodule in the right lower lobe.

ASSESSMENT AND PLAN:
The patient is anxious about the finding and inquires about the likelihood that this nodule represents cancer. You explain that the positive predictive value (PPV) of the test is influenced by the prevalence of lung cancer in the population being screened. To further assess the risk, you consider the patient's age, smoking history, and the characteristics of the nodule. Given this information,

## Encounter 2: source 2022-07-15, practice 2024-09-27, outpatient, Pulmonology

Chief complaint: Persistent dry cough and unintentional weight loss of 15 pounds occurring over the past three months.

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. David Kim, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent dry cough and unintentional weight loss of 15 pounds occurring over the past three months.

HISTORY OF PRESENT ILLNESS:
With COPD, tobacco use, and prior nodule evaluation, he presents to clinic with a new cough and weight loss over past 3 months.

PAST MEDICAL HISTORY:
Active Problem List:
- Suspected malignant neoplasm of lung (diagnosed 2020-01-15)
- Solitary pulmonary nodule with high suspicion for malignancy (diagnosed 2020-07-15)
- Chronic obstructive pulmonary disease (COPD)
- Tobacco use disorder

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Tiotropium 18 mcg inhaled once daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed for shortness of breath

ALLERGIES:
None known

FAMILY HISTORY:
His family history is significant for lung cancer in his father.

SOCIAL HISTORY:
Smoking: Current smoker, 30 pack-years. Alcohol: Occasional social use. Occupation: Construction worker.

REVIEW OF SYSTEMS:
He denies any fever, night sweats, or hemoptysis.

VITAL SIGNS:
On physical examination, he is afebrile, blood pressure is 130/85 mmHg, heart rate is 78 beats per minute, and respiratory rate is 16 breaths per minute.

PHYSICAL EXAMINATION:
Lung auscultation reveals decreased breath sounds in the right upper lobe.

LABORATORY DATA:
Laboratory Values:
- Hemoglobin: 14 g/dL
- White blood cell count: 7,000/uL
- Platelets: 250,000/uL
- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Creatinine: 1.0 mg/dL
- Glucose: 95 mg/dL

IMAGING:
A chest X-ray shows a right upper lobe mass. A subsequent CT scan confirms a 4 cm mass with mediastinal lymphadenopathy.

PATHOLOGY:
Biopsy of the mass is consistent with non-small cell lung cancer (NSCLC).

OTHER STUDIES:
The patient is enrolled in a case-control study to identify potential risk factors for NSCLC. The study compares the smoking history of patients with NSCLC (cases) to those without NSCLC (controls).

## Encounter 3: source 2023-07-15, practice 2025-09-14, ed, Emergency Medicine

Chief complaint: Acute worsening of shortness of breath over two days with productive cough and green sputum production.

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute worsening of shortness of breath over two days with productive cough and green sputum production.

HISTORY OF PRESENT ILLNESS:
The patient with COPD and solitary pulmonary nodule presents to the emergency department with a two-day history of increased shortness of breath and a productive cough with green sputum.

PAST MEDICAL HISTORY:
Active Problem List:
- Suspected malignant neoplasm of lung (diagnosed 2020-01-15)
- Solitary pulmonary nodule with high suspicion for malignancy (diagnosed 2020-07-15)
- Chronic obstructive pulmonary disease (COPD)
- Tobacco use disorder

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
His medications include tiotropium and albuterol inhalers.

ALLERGIES:
None known

FAMILY HISTORY:
- Father with chronic obstructive pulmonary disease
- Paternal uncle with lung cancer

SOCIAL HISTORY:
30-pack-year smoking history

VITAL SIGNS:
Respiratory rate of 22 breaths per minute, oxygen saturation is 88% on room air

PHYSICAL EXAMINATION:
On examination, he is tachypneic and he has diffuse wheezes on lung auscultation

LABORATORY DATA:
Arterial blood gas reveals pH 7.32, PaCO2 55 mmHg, and PaO2 70 mmHg.

IMAGING:
Chest X-ray shows increased bronchial wall thickness and no signs of consolidation.

PLAN:
The patient is admitted for management of a COPD exacerbation.

## Encounter 4: source 2024-01-15, practice 2026-03-28, ed, Emergency Medicine

Chief complaint: Acute onset of progressive confusion, lethargy, and poor appetite over the past week with associated hemoptysis noted by family.

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Michael Torres, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset of progressive confusion, lethargy, and poor appetite over the past week with associated hemoptysis noted by family.

HISTORY OF PRESENT ILLNESS:
Patient is a 49-year-old male with COPD and tobacco use disorder with a history of solitary pulmonary nodule and prior weight loss. He is brought to the emergency department by his sister due to progressive confusion, lethargy, and poor appetite over the past week. She reports that he has unintentionally lost nearly 7 kg over the past two months and has had a persistent cough with occasional blood-streaked sputum.

PAST MEDICAL HISTORY:
Active Problem List:
- Suspected malignant neoplasm of lung (diagnosed 2020-01-15)
- Solitary pulmonary nodule with high suspicion for malignancy (diagnosed 2020-07-15)
- Chronic obstructive pulmonary disease (COPD)
- Tobacco use disorder

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Tiotropium 18 mcg inhaled once daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed for shortness of breath

ALLERGIES:
None known

FAMILY HISTORY:
- Father with chronic obstructive pulmonary disease
- Paternal uncle with lung cancer

SOCIAL HISTORY:
He has a 30-pack-year smoking history and has not seen a physician in many years.

VITAL SIGNS:
His blood pressure is 116/74 mm Hg, heart rate is 84/min,

PHYSICAL EXAMINATION:
On exam, he appears disoriented and drowsy. and physical exam shows no peripheral edema.

IMAGING:
Chest imaging reveals a centrally located lung mass.
