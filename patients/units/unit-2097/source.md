# synthetic_hospital patient 2097

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex M, Non-Hispanic White, Medicare primary (ESRD eligibility), Blue Cross Blue Shield secondary.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-09-09 (Tue) | outpatient | Gastroenterology |
| 1 | 2020-04-15 | 2025-11-26 (Wed) | outpatient | Infectious Diseases |
| 2 | 2020-07-15 | 2026-02-28 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare primary (ESRD eligibility), Blue Cross Blue Shield secondary",
 "occupation": "Office administrator at a local community college",
 "alcohol_use": "Lifelong abstinence following family history of alcohol use disorder",
 "family_history": [
  "Father: Hereditary spherocytosis, cholecystectomy, hypertension",
  "Mother: Type 2 diabetes mellitus, hypothyroidism",
  "Sister: Hereditary spherocytosis, splenectomy",
  "Paternal uncle: Major depressive disorder, completed suicide"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 5 years ago during psychiatric hospitalization",
 "home_medications": [
  {
   "dose": "5 mg orally every 12 hours (therapeutic drug monitoring required)",
   "name": "Tacrolimus"
  },
  {
   "dose": "750 mg orally every 12 hours",
   "name": "Mycophenolate mofetil"
  },
  {
   "dose": "10 mg orally daily",
   "name": "Prednisone"
  },
  {
   "dose": "1 mg orally daily",
   "name": "Folic acid"
  },
  {
   "dose": "40 mg orally daily",
   "name": "Omeprazole"
  },
  {
   "dose": "25 mg orally twice daily",
   "name": "Metoprolol"
  }
 ],
 "surgical_history": [
  "Laparoscopic splenectomy (age 22)",
  "Living donor renal transplantation (3 months ago)",
  "Multiple esophageal balloon dilations (ages 41-44)",
  "Percutaneous endoscopic gastrostomy tube placement (age 41, subsequently removed)"
 ],
 "chronic_conditions": [
  "Hereditary spherocytosis",
  "Status post splenectomy",
  "Chronic anemia secondary to hemolytic disease",
  "Status post renal transplantation (3 months prior)",
  "Gastric outlet obstruction secondary to caustic stricture",
  "History of major depressive disorder, in remission",
  "Chronic kidney disease stage T (post-transplant)"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Cytomegalovirus reactivation", "Acute hemolytic transfusion reaction", "Gastric outlet obstruction or Peptic ulcer disease"]
Comorbidities: ["Acute kidney injury", "Anemia"]

## Encounter 0: source 2020-01-15, practice 2025-09-09, outpatient, Gastroenterology

Chief complaint: Two-month history of early satiety, nausea, and episodes of nonbilious vomiting with associated 10 kg weight loss.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Elena Vasquez, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Two-month history of early satiety, nausea, and episodes of nonbilious vomiting with associated 10 kg weight loss.

HISTORY OF PRESENT ILLNESS:
He presents to the clinic with a 2-month history of early satiety, nausea, and episodes of nonbilious vomiting. He reports a significant weight loss of 10 kg over the same period.

PAST MEDICAL HISTORY:
Active Problem List:
- Hereditary spherocytosis
- Status post splenectomy
- Chronic anemia secondary to hemolytic disease
- Status post renal transplantation (3 months prior)
- Gastric outlet obstruction secondary to caustic stricture
- History of major depressive disorder, in remission
- Chronic kidney disease stage T (post-transplant)

His past medical history is notable for a severe depressive episode 5 years ago, during which he ingested a strong acid in a suicide attempt.

PAST SURGICAL HISTORY:
- Laparoscopic splenectomy (age 22)
- Living donor renal transplantation (3 months ago)
- Multiple esophageal balloon dilations (ages 41-44)
- Percutaneous endoscopic gastrostomy tube placement (age 41, subsequently removed)

MEDICATIONS:
Home Medications:
- Tacrolimus 5 mg orally every 12 hours (therapeutic drug monitoring required)
- Mycophenolate mofetil 750 mg orally every 12 hours
- Prednisone 10 mg orally daily
- Folic acid 1 mg orally daily
- Omeprazole 40 mg orally daily
- Metoprolol 25 mg orally twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hereditary spherocytosis, cholecystectomy, hypertension
- Mother: Type 2 diabetes mellitus, hypothyroidism
- Sister: Hereditary spherocytosis, splenectomy
- Paternal uncle: Major depressive disorder, completed suicide

SOCIAL HISTORY:
Smoking: Former smoker, quit 5 years ago during psychiatric hospitalization. Alcohol: Lifelong abstinence following family history of alcohol use disorder. Occupation: Office administrator at a local community college.

PHYSICAL EXAMINATION:
On physical examination, his abdomen is distended with succussion splash heard on auscultation.

LABORATORY DATA:
Laboratory tests reveal:
- Hemoglobin: 13.2 g/dL (normal: 13.8-17.2 g/dL)
- White blood cell count: 7,000/uL (normal: 4,500-11,000/uL)
- Serum electrolytes: within normal limits
- BUN: 21 mg/dL (normal: 7-20 mg/dL)
- Creatinine: 0.9 mg/dL (normal: 0.6-1.2 mg/dL)

IMAGING:
An upper gastrointestinal series shows delayed gastric emptying and a narrowed pyloric canal.

## Encounter 1: source 2020-04-15, practice 2025-11-26, outpatient, Infectious Diseases

Chief complaint: One-week history of shortness of breath, non-productive cough, and fever with associated nausea and diarrhea for the past 3 days.

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. James Okonkwo, MD
Department: Infectious Diseases
Visit Type: outpatient

CHIEF COMPLAINT:
One-week history of shortness of breath, non-productive cough, and fever with associated nausea and diarrhea for the past 3 days.

HISTORY OF PRESENT ILLNESS:
The patient is a 45-year-old male with hereditary spherocytosis who is three months status post renal transplantation and has a history of gastric outlet obstruction. He presents with a 1-week history of shortness of breath, non-productive cough, and fever. He also reports nausea and diarrhea for the past 3 days.

PAST MEDICAL HISTORY:
Active Problem List:
- Gastric outlet obstruction or Peptic ulcer disease (diagnosed 2020-01-15)
- Hereditary spherocytosis
- Status post splenectomy
- Chronic anemia secondary to hemolytic disease
- Status post renal transplantation (3 months prior)
- Gastric outlet obstruction secondary to caustic stricture
- History of major depressive disorder, in remission
- Chronic kidney disease stage T (post-transplant)

He underwent a renal transplant 3 months ago.

PAST SURGICAL HISTORY:
- Laparoscopic splenectomy (age 22)
- Living donor renal transplantation (3 months ago)
- Multiple esophageal balloon dilations (ages 41-44)
- Percutaneous endoscopic gastrostomy tube placement (age 41, subsequently removed)

MEDICATIONS:
He has been on immunosuppressive therapy with tacrolimus, mycophenolate, and prednisone.

Home Medications:
- Mycophenolate mofetil 750 mg orally every 12 hours
- Folic acid 1 mg orally daily
- Omeprazole 40 mg orally daily
- Metoprolol 25 mg orally twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hereditary spherocytosis, cholecystectomy, hypertension
- Mother: Type 2 diabetes mellitus, hypothyroidism
- Sister: Hereditary spherocytosis, splenectomy
- Paternal uncle: Major depressive disorder, completed suicide

SOCIAL HISTORY:
Smoking: Former smoker, quit 5 years ago during psychiatric hospitalization. Alcohol: Lifelong abstinence following family history of alcohol use disorder. Occupation: Office administrator at a local community college.

VITAL SIGNS:
On examination, he is febrile with a temperature of 38.7°C, tachypneic with a respiratory rate of 22 breaths per minute

PHYSICAL EXAMINATION:
He has diffuse crackles on lung auscultation. His abdomen is soft but mildly tender in the epigastric region.

LABORATORY DATA:
WBC count: 4,500/µL (Normal: 4,500-11,000/µL)
- Hemoglobin: 12.2 g/dL (Normal: 13.8-17.2 g/dL)
- Platelets: 150,000/µL (Normal: 150,000-450,000/µL)
- Serum creatinine: 1.9 mg/dL (Baseline post-transplant: 1.2 mg/dL)
- AST: 68 U/L (Normal: 10-40 U/L)
- ALT: 55 U/L (Normal: 7-56 U/L)
- CRP: 50 mg/L (Normal: <10 mg/L)

IMAGING:
Chest X-ray: Diffuse interstitial infiltrates

ASSESSMENT AND PLAN:
Given the patient's symptoms and findings,

## Encounter 2: source 2020-07-15, practice 2026-02-28, ed, Emergency Medicine

Chief complaint: Sudden onset of fever, lower back pain, and feeling of 'impending doom' occurring 30 minutes after receiving a red blood cell transfusion.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of fever, lower back pain, and feeling of 'impending doom' occurring 30 minutes after receiving a red blood cell transfusion.

HISTORY OF PRESENT ILLNESS:
This 45-year-old male with hereditary spherocytosis status post splenectomy and gastric outlet obstruction is three months status post renal transplantation complicated by recent cytomegalovirus reactivation. He presents to the emergency department 30 minutes after receiving a red blood cell transfusion with sudden onset of fever, lower back pain, and feeling of impending doom.

PAST MEDICAL HISTORY:
Active Problem List:
- Gastric outlet obstruction or Peptic ulcer disease (diagnosed 2020-01-15)
- Cytomegalovirus reactivation (diagnosed 2020-04-15)
- Hereditary spherocytosis
- Status post splenectomy
- Chronic anemia secondary to hemolytic disease
- Status post renal transplantation (3 months prior)
- Gastric outlet obstruction secondary to caustic stricture
- History of major depressive disorder, in remission
- Chronic kidney disease stage T (post-transplant)

PAST SURGICAL HISTORY:
- Laparoscopic splenectomy (age 22)
- Living donor renal transplantation (3 months ago)
- Multiple esophageal balloon dilations (ages 41-44)
- Percutaneous endoscopic gastrostomy tube placement (age 41, subsequently removed)

MEDICATIONS:
Home Medications:
- Tacrolimus 5 mg orally every 12 hours (therapeutic drug monitoring required)
- Mycophenolate mofetil 750 mg orally every 12 hours
- Prednisone 10 mg orally daily
- Folic acid 1 mg orally daily
- Omeprazole 40 mg orally daily
- Metoprolol 25 mg orally twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hereditary spherocytosis, cholecystectomy, hypertension
- Mother: Type 2 diabetes mellitus, hypothyroidism
- Sister: Hereditary spherocytosis, splenectomy
- Paternal uncle: Major depressive disorder, completed suicide

SOCIAL HISTORY:
Smoking: Former smoker, quit 5 years ago during psychiatric hospitalization. Alcohol: Lifelong abstinence following family history of alcohol use disorder. Occupation: Office administrator at a local community college.

VITAL SIGNS:
His temperature is 38.9°C (102°F), blood pressure is 90/60 mmHg, pulse is 120/min, and respiratory rate is 22/min.

PHYSICAL EXAMINATION:
Physical examination reveals jaundice and dark urine.

LABORATORY DATA:
His pre-transfusion hemoglobin was 7.8 g/dL, and his current laboratory results are as follows:

- Hemoglobin: 5.2 g/dL
- Lactate dehydrogenase (LDH): Elevated
- Haptoglobin: <10 mg/dL (decreased)
- Direct Coombs test: Positive
- Serum creatinine: 1.9 mg/dL (elevated from baseline of 0.9 mg/dL)
- Peripheral blood smear: Schistocytes present
