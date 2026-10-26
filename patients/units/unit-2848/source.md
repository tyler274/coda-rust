# synthetic_hospital patient 2848

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 68 at first encounter, sex F, Hispanic, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-01-21 (Sat) | ed | Emergency Medicine |
| 1 | 2020-04-15 | 2023-04-27 (Thu) | inpatient | Gastroenterology |
| 2 | 2020-10-15 | 2023-10-18 (Wed) | outpatient | Oncology |
| 3 | 2023-01-15 | 2026-01-22 (Thu) | ed | Emergency Medicine |
| 4 | 2023-04-15 | 2026-04-23 (Thu) | outpatient | Cardiology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired administrative assistant",
 "alcohol_use": "None",
 "family_history": [
  "Father with coronary artery disease and hypertension",
  "Mother with type 2 diabetes mellitus",
  "Older sister with cholelithiasis"
 ],
 "race_ethnicity": "Hispanic",
 "smoking_status": "Never",
 "home_medications": [
  {
   "dose": "40 mg oral daily",
   "name": "Furosemide"
  },
  {
   "dose": "10 mg oral daily",
   "name": "Amlodipine"
  },
  {
   "dose": "500 mg oral twice daily",
   "name": "Metformin"
  },
  {
   "dose": "125 mg/m\u00b2 IV weekly (chemotherapy protocol)",
   "name": "Irinotecan"
  }
 ],
 "surgical_history": [
  "Biliary decompression (endoscopic stenting)"
 ],
 "chronic_conditions": [
  "Essential hypertension",
  "Type 2 diabetes mellitus",
  "Chronic kidney disease stage 3",
  "Chronic heart failure with reduced ejection fraction",
  "Obesity",
  "Chronic cholelithiasis",
  "Gallbladder carcinoma"
 ],
 "age_at_first_encounter": 68
}
```

Primary diagnoses: ["Stable Chronic Heart Failure", "Acute pulmonary edema", "Essential hypertension", "Gallbladder carcinoma"]
Comorbidities: ["Anemia", "Essential hypertension", "Left ventricular hypertrophy", "Obesity", "Cholelithiasis"]

## Encounter 0: source 2020-01-15, practice 2023-01-21, ed, Emergency Medicine

Chief complaint: Progressively worsening exertional dyspnea and fatigue over the past several months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. James Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressively worsening exertional dyspnea and fatigue over the past several months

HISTORY OF PRESENT ILLNESS:
She presents to the emergency department with complaints of progressively worsening exertional dyspnea and fatigue over the past several months.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension
- Type 2 diabetes mellitus
- Chronic kidney disease stage 3
- Chronic heart failure with reduced ejection fraction
- Obesity
- Chronic cholelithiasis
- Gallbladder carcinoma

She has a history of poorly controlled hypertension but has not been compliant with medications.

PAST SURGICAL HISTORY:
- Biliary decompression (endoscopic stenting)

MEDICATIONS:
Home Medications:
- Furosemide 40 mg oral daily
- Amlodipine 10 mg oral daily
- Metformin 500 mg oral twice daily
- Irinotecan 125 mg/m² IV weekly (chemotherapy protocol)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with coronary artery disease and hypertension
- Mother with type 2 diabetes mellitus
- Older sister with cholelithiasis

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Retired administrative assistant.

REVIEW OF SYSTEMS:
She denies chest pain, palpitations, or syncope.

VITAL SIGNS:
On examination, her blood pressure is 160/92 mm Hg, pulse is 88/min, and respirations are 18/min.

PHYSICAL EXAMINATION:
Cardiac examination reveals a sustained point of maximal impulse and an S4 gallop. Pulmonary auscultation is clear.

## Encounter 1: source 2020-04-15, practice 2023-04-27, inpatient, Gastroenterology

Chief complaint: Progressive weight loss, abdominal discomfort, and jaundice

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Sarah Patel, MD
Department: Gastroenterology
Visit Type: inpatient

CHIEF COMPLAINT:
Progressive weight loss, abdominal discomfort, and jaundice

HISTORY OF PRESENT ILLNESS:
This 68-year-old female with gallbladder carcinoma and essential hypertension diagnosed during an emergency department presentation three months ago for exertional dyspnea presents with progressive weight loss, abdominal discomfort, and jaundice. She undergoes biliary decompression and is initiated on combination chemotherapy that includes a newly added drug, irinotecan. During a follow-up visit, she reports mild diarrhea and fatigue but is otherwise tolerating the regimen. The patient expresses interest in understanding how this new medication works.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Type 2 diabetes mellitus
- Chronic kidney disease stage 3
- Chronic heart failure with reduced ejection fraction
- Obesity
- Chronic cholelithiasis
- Gallbladder carcinoma

PAST SURGICAL HISTORY:
- Biliary decompression (endoscopic stenting)

MEDICATIONS:
She takes irinotecan.

Home Medications:
- Furosemide 40 mg oral daily
- Amlodipine 10 mg oral daily
- Metformin 500 mg oral twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with coronary artery disease and hypertension
- Mother with type 2 diabetes mellitus
- Older sister with cholelithiasis

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Retired administrative assistant.

LABORATORY DATA:
Laboratory tests show stable liver function and mild anemia

IMAGING:
Imaging reveals a mass in the gallbladder with invasion into adjacent hepatic tissue

PATHOLOGY:
Biopsy confirms adenocarcinoma of the gallbladder

PLAN:
The oncologist explains that the new drug works by interfering with DNA replication and preventing tumor cell proliferation.

## Encounter 2: source 2020-10-15, practice 2023-10-18, outpatient, Oncology

Chief complaint: Mild diarrhea and fatigue during chemotherapy treatment

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Michael Thompson, MD
Department: Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Mild diarrhea and fatigue during chemotherapy treatment

HISTORY OF PRESENT ILLNESS:
This 68-year-old with gallbladder carcinoma, seen in April for jaundice, returns for irinotecan follow-up, reporting mild diarrhea and fatigue but otherwise tolerating the regimen.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Type 2 diabetes mellitus
- Chronic kidney disease stage 3
- Chronic heart failure with reduced ejection fraction
- Obesity
- Chronic cholelithiasis
- Gallbladder carcinoma

PAST SURGICAL HISTORY:
- Biliary decompression (endoscopic stenting)

MEDICATIONS:
She is initiated on combination chemotherapy that includes a newly added drug, topotecan.

Home Medications:
- Furosemide 40 mg oral daily
- Amlodipine 10 mg oral daily
- Metformin 500 mg oral twice daily
- Irinotecan 125 mg/m² IV weekly (chemotherapy protocol)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with coronary artery disease and hypertension
- Mother with type 2 diabetes mellitus
- Older sister with cholelithiasis

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Retired administrative assistant.

LABORATORY DATA:
Laboratory tests show stable liver function and mild anemia

IMAGING:
Imaging reveals a mass in the gallbladder with invasion into adjacent hepatic tissue

PATHOLOGY:
Biopsy confirms adenocarcinoma of the gallbladder

OTHER STUDIES:
She undergoes biliary decompression

ASSESSMENT AND PLAN:
The oncologist explains that the new drug works by interfering with DNA replication and preventing tumor cell proliferation. The patient expresses interest in understanding how this new medication works.

## Encounter 3: source 2023-01-15, practice 2026-01-22, ed, Emergency Medicine

Chief complaint: Acute respiratory distress with sensation of drowning

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Emily Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute respiratory distress with sensation of drowning

HISTORY OF PRESENT ILLNESS:
The patient with chronic heart failure and gallbladder carcinoma on weekly chemotherapy returns with acute complaints and is brought to the emergency department by ambulance for acute respiratory distress that began suddenly while she was watching television. She describes the sensation as “drowning” and says she could not lie flat in the ambulance due to worsening breathlessness.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Gallbladder carcinoma (diagnosed 2020-10-15)
- Type 2 diabetes mellitus
- Chronic kidney disease stage 3
- Chronic heart failure with reduced ejection fraction
- Obesity
- Chronic cholelithiasis

She has a history of long-standing hypertension, type 2 diabetes mellitus, and chronic kidney disease stage 3.

PAST SURGICAL HISTORY:
- Biliary decompression (endoscopic stenting)

MEDICATIONS:
Her medications include amlodipine and metformin.

Home Medications:
- Furosemide 40 mg oral daily
- Irinotecan 125 mg/m² IV weekly (chemotherapy protocol)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with coronary artery disease and hypertension
- Mother with type 2 diabetes mellitus
- Older sister with cholelithiasis

SOCIAL HISTORY:
She does not smoke or use alcohol.

VITAL SIGNS:
Blood pressure is 228/112 mm Hg, heart rate is 104/min, and respiratory rate is 32/min. Oxygen saturation is 85% on room air.

PHYSICAL EXAMINATION:
On examination, she appears visibly distressed, gasping for air. She is using accessory muscles of respiration. Auscultation of the lungs reveals diffuse crackles bilaterally. Cardiac exam shows an S4 gallop but no murmur. There is no peripheral edema.

LABORATORY DATA:
ABG shows hypoxemia with a widened A–a gradient.

IMAGING:
Chest x-ray reveals prominent pulmonary vasculature, perihilar batwing opacities, and pleural effusions.

OTHER STUDIES:
ECG shows left ventricular hypertrophy and sinus tachycardia.

PLAN:
She is given intravenous nitrates and loop diuretics.

## Encounter 4: source 2023-04-15, practice 2026-04-23, outpatient, Cardiology

Chief complaint: Shortness of breath on exertion and paroxysmal nocturnal dyspnea

ENCOUNTER NOTE
Date: 2023-04-15
Provider: Dr. Maria Rodriguez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Shortness of breath on exertion and paroxysmal nocturnal dyspnea

HISTORY OF PRESENT ILLNESS:
The patient is a 71-year-old female with chronic heart failure, hypertension, and gallbladder carcinoma on chemotherapy, returning for cardiac follow-up. She presents complaining of shortness of breath on exertion and paroxysmal nocturnal dyspnea for the past 6 months. She reports a recent hospitalization for acute decompensated heart failure, from which she was discharged on furosemide.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension (diagnosed 2020-01-15)
- Gallbladder carcinoma (diagnosed 2020-10-15)
- Acute pulmonary edema (diagnosed 2023-01-15)
- Type 2 diabetes mellitus
- Chronic kidney disease stage 3
- Chronic heart failure with reduced ejection fraction
- Obesity
- Chronic cholelithiasis

PAST SURGICAL HISTORY:
- Biliary decompression (endoscopic stenting)

MEDICATIONS:
She takes furosemide.

Home Medications:
- Amlodipine 10 mg oral daily
- Metformin 500 mg oral twice daily
- Irinotecan 125 mg/m² IV weekly (chemotherapy protocol)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with coronary artery disease and hypertension
- Mother with type 2 diabetes mellitus
- Older sister with cholelithiasis

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Retired administrative assistant.

VITAL SIGNS:
Her blood pressure is 142/90 mmHg, heart rate is 78 bpm, and BMI is 32 kg/m^2

PHYSICAL EXAMINATION:
Physical examination reveals jugular venous distension, a laterally displaced apical impulse, and bilateral lower extremity edema. Lung auscultation is notable for bibasilar crackles.

LABORATORY DATA:
Serum sodium: 138 mEq/L
Serum potassium: 4.2 mEq/L
Serum creatinine: 1.1 mg/dL
B-type natriuretic peptide (BNP): 450 pg/mL
Hemoglobin: 13.2 g/dL

IMAGING:
A transthoracic echocardiogram reveals a left ventricular ejection fraction of 55%, with evidence of diastolic dysfunction including an elevated E/e' ratio.

OTHER STUDIES:
An electrocardiogram shows left ventricular hypertrophy.
