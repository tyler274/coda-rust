# synthetic_hospital patient 2533

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex M, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-02-17 (Mon) | inpatient | Cardiology |
| 1 | 2020-05-15 | 2025-07-02 (Wed) | inpatient | Nephrology |
| 2 | 2020-11-15 | 2025-12-20 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Penicillin - anaphylaxis",
  "Iodinated contrast - urticaria"
 ],
 "insurance": "Medicaid",
 "occupation": "Construction laborer (currently on medical leave)",
 "alcohol_use": "Social alcohol use, 2-3 drinks per week; History of intravenous heroin and methamphetamine use, active within past week",
 "family_history": [
  "Paternal uncle with multiple myeloma (diagnosed age 58)",
  "Father with alcohol use disorder and chronic obstructive pulmonary disease",
  "Mother with systemic lupus erythematosus",
  "Maternal grandfather with coronary artery disease (myocardial infarction age 65)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 1 pack per day for 15 years",
 "home_medications": [
  {
   "dose": "25 mg PO daily on days 1-21 of 28-day cycle",
   "name": "Lenalidomide"
  },
  {
   "dose": "40 mg PO once weekly",
   "name": "Dexamethasone"
  },
  {
   "dose": "400 mg PO twice daily (antiviral prophylaxis)",
   "name": "Acyclovir"
  },
  {
   "dose": "20 mg PO daily (gastric protection)",
   "name": "Omeprazole"
  },
  {
   "dose": "5-10 mg PO every 4-6 hours as needed for pain",
   "name": "Oxycodone"
  },
  {
   "dose": "8 mg PO every 8 hours as needed for nausea",
   "name": "Ondansetron"
  }
 ],
 "surgical_history": [
  "Blunt abdominal trauma (age 31, motor vehicle collision with seatbelt sign, managed conservatively without surgical intervention)",
  "Open reduction internal fixation of left distal radius (age 19)"
 ],
 "chronic_conditions": [
  "Multiple myeloma (IgG kappa, diagnosed 6 months ago, currently on maintenance therapy)",
  "Opioid use disorder (intravenous)",
  "Chronic hepatitis C infection (genotype 1a)",
  "Chronic kidney disease stage 3b",
  "Chronic pain syndrome (post-traumatic)",
  "Post-traumatic stress disorder"
 ],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Multiple myeloma cast nephropathy", "Infective Endocarditis (Aortic Valve) with Complications", "Pneumoperitoneum"]
Comorbidities: ["Acute kidney injury", "Intravenous Drug Use (Substance Use Disorder)"]

## Encounter 0: source 2020-01-15, practice 2025-02-17, inpatient, Cardiology

Chief complaint: Two-week history of fever, fatigue, and a new-onset heart murmur

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Cardiology
Visit Type: inpatient

CHIEF COMPLAINT:
Two-week history of fever, fatigue, and a new-onset heart murmur

HISTORY OF PRESENT ILLNESS:
He reports intermittent fevers reaching up to 38.9°C, night sweats, and an unintentional 5 kg weight loss over the past month.

PAST MEDICAL HISTORY:
Active Problem List:
- Multiple myeloma (IgG kappa, diagnosed 6 months ago, currently on maintenance therapy)
- Opioid use disorder (intravenous)
- Chronic hepatitis C infection (genotype 1a)
- Chronic kidney disease stage 3b
- Chronic pain syndrome (post-traumatic)
- Post-traumatic stress disorder

PAST SURGICAL HISTORY:
- Blunt abdominal trauma (age 31, motor vehicle collision with seatbelt sign, managed conservatively without surgical intervention)
- Open reduction internal fixation of left distal radius (age 19)

MEDICATIONS:
Home Medications:
- Lenalidomide 25 mg PO daily on days 1-21 of 28-day cycle
- Dexamethasone 40 mg PO once weekly
- Acyclovir 400 mg PO twice daily (antiviral prophylaxis)
- Omeprazole 20 mg PO daily (gastric protection)
- Oxycodone 5-10 mg PO every 4-6 hours as needed for pain
- Ondansetron 8 mg PO every 8 hours as needed for nausea

ALLERGIES:
Penicillin - anaphylaxis, Iodinated contrast - urticaria

FAMILY HISTORY:
- Paternal uncle with multiple myeloma (diagnosed age 58)
- Father with alcohol use disorder and chronic obstructive pulmonary disease
- Mother with systemic lupus erythematosus
- Maternal grandfather with coronary artery disease (myocardial infarction age 65)

SOCIAL HISTORY:
History of intravenous drug use

VITAL SIGNS:
Temperature is 38.5°C, blood pressure is 110/70 mmHg, heart rate is 102 beats per minute, and respiratory rate is 18 breaths per minute

PHYSICAL EXAMINATION:
Cardiac auscultation reveals a holosystolic murmur at the left lower sternal border that increases in intensity with inspiration. There are no signs of peripheral stigmata of endocarditis.

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 14,000 /µL
- Hemoglobin: 10.2 g/dL
- Platelet count: 250,000 /µL
- Erythrocyte sedimentation rate: 85 mm/hr
- Blood cultures: Pending

IMAGING:
A transthoracic echocardiogram shows vegetations on the tricuspid valve with mild tricuspid regurgitation.

## Encounter 1: source 2020-05-15, practice 2025-07-02, inpatient, Nephrology

Chief complaint: Two-week history of intermittent fever, chills, and night sweats with new-onset headache and shortness of breath

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. Michael Okafor, MD
Department: Nephrology
Visit Type: inpatient

CHIEF COMPLAINT:
Two-week history of intermittent fever, chills, and night sweats with new-onset headache and shortness of breath

HISTORY OF PRESENT ILLNESS:
The patient is a 32-year-old male with multiple myeloma on maintenance lenalidomide and dexamethasone therapy who was hospitalized four months prior for infective endocarditis. He presents with a two-week history of intermittent fever, chills, and night sweats. He also reports a new-onset headache and shortness of breath.

PAST MEDICAL HISTORY:
Active Problem List:
- Infective Endocarditis (Aortic Valve) with Complications (diagnosed 2020-01-15)
- Multiple myeloma (IgG kappa, diagnosed 6 months ago, currently on maintenance therapy)
- Opioid use disorder (intravenous)
- Chronic hepatitis C infection (genotype 1a)
- Chronic kidney disease stage 3b
- Chronic pain syndrome (post-traumatic)
- Post-traumatic stress disorder

PAST SURGICAL HISTORY:
- Blunt abdominal trauma (age 31, motor vehicle collision with seatbelt sign, managed conservatively without surgical intervention)
- Open reduction internal fixation of left distal radius (age 19)

MEDICATIONS:
Home Medications:
- Lenalidomide 25 mg PO daily on days 1-21 of 28-day cycle
- Dexamethasone 40 mg PO once weekly
- Acyclovir 400 mg PO twice daily (antiviral prophylaxis)
- Omeprazole 20 mg PO daily (gastric protection)
- Oxycodone 5-10 mg PO every 4-6 hours as needed for pain
- Ondansetron 8 mg PO every 8 hours as needed for nausea

ALLERGIES:
Penicillin - anaphylaxis, Iodinated contrast - urticaria

FAMILY HISTORY:
- Paternal uncle with multiple myeloma (diagnosed age 58)
- Father with alcohol use disorder and chronic obstructive pulmonary disease
- Mother with systemic lupus erythematosus
- Maternal grandfather with coronary artery disease (myocardial infarction age 65)

SOCIAL HISTORY:
With a history of intravenous drug use

VITAL SIGNS:
On examination, his temperature is 38.6°C (101.5°F), blood pressure is 110/70 mmHg, heart rate is 105 beats per minute, and respiratory rate is 22 breaths per minute.

PHYSICAL EXAMINATION:
Cardiac auscultation reveals a grade III/VI systolic murmur best heard at the left lower sternal border. Neurological examination is unremarkable. Fundoscopic examination shows Roth spots.

LABORATORY DATA:
Laboratory studies reveal:
- White blood cell count: 14,000/µL
- Hemoglobin: 10.2 g/dL
- Platelet count: 150,000/µL
- Blood urea nitrogen: 28 mg/dL
- Serum creatinine: 1.9 mg/dL
- Urinalysis: Red blood cell casts, proteinuria

IMAGING:
A transthoracic echocardiogram shows a 0.5 cm mobile vegetation on the tricuspid valve.

OTHER STUDIES:
Three sets of blood cultures are drawn

PLAN:
The patient is started on empiric antibiotic therapy.

## Encounter 2: source 2020-11-15, practice 2025-12-20, ed, Emergency Medicine

Chief complaint: Severe abdominal pain and distension following a motor vehicle collision 2 hours ago

ENCOUNTER NOTE
Date: 2020-11-15
Provider: Dr. Emily Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe abdominal pain and distension following a motor vehicle collision 2 hours ago

HISTORY OF PRESENT ILLNESS:
The patient is a 32-year-old male with IgG kappa multiple myeloma on maintenance therapy, previously hospitalized for cast nephropathy and infective endocarditis, who presents with severe abdominal pain and distension following a motor vehicle collision 2 hours ago. A urinary catheter is placed, and the urine output is noted to be low despite adequate fluid resuscitation. The patient's blood pressure does not improve significantly with fluid resuscitation.

PAST MEDICAL HISTORY:
Active Problem List:
- Infective Endocarditis (Aortic Valve) with Complications (diagnosed 2020-01-15)
- Multiple myeloma cast nephropathy (diagnosed 2020-05-15)
- Multiple myeloma (IgG kappa, diagnosed 6 months ago, currently on maintenance therapy)
- Opioid use disorder (intravenous)
- Chronic hepatitis C infection (genotype 1a)
- Chronic kidney disease stage 3b
- Chronic pain syndrome (post-traumatic)
- Post-traumatic stress disorder

The patient has a history of blunt abdominal trauma 1 year ago, which was managed conservatively.

PAST SURGICAL HISTORY:
- Blunt abdominal trauma (age 31, motor vehicle collision with seatbelt sign, managed conservatively without surgical intervention)
- Open reduction internal fixation of left distal radius (age 19)

MEDICATIONS:
Home Medications:
- Lenalidomide 25 mg PO daily on days 1-21 of 28-day cycle
- Dexamethasone 40 mg PO once weekly
- Acyclovir 400 mg PO twice daily (antiviral prophylaxis)
- Omeprazole 20 mg PO daily (gastric protection)
- Oxycodone 5-10 mg PO every 4-6 hours as needed for pain
- Ondansetron 8 mg PO every 8 hours as needed for nausea

ALLERGIES:
Penicillin - anaphylaxis, Iodinated contrast - urticaria

FAMILY HISTORY:
- Paternal uncle with multiple myeloma (diagnosed age 58)
- Father with alcohol use disorder and chronic obstructive pulmonary disease
- Mother with systemic lupus erythematosus
- Maternal grandfather with coronary artery disease (myocardial infarction age 65)

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day for 15 years. Alcohol: Social alcohol use, 2-3 drinks per week; History of intravenous heroin and methamphetamine use, active within past week. Occupation: Construction laborer (currently on medical leave).

VITAL SIGNS:
He is hypotensive and tachycardic.

PHYSICAL EXAMINATION:
His abdomen is tense and tender to palpation.
Physical examination reveals muffled heart sounds and jugular venous distension.

LABORATORY DATA:
His laboratory values are as follows:

- Hemoglobin: 9.8 g/dL (normal: 13.8-17.2 g/dL)
- Hematocrit: 29.4% (normal: 40.7-50.3%)
- White blood cell count: 18,000/mm³ (normal: 4,500-11,000/mm³)
- Platelets: 225,000/mm³ (normal: 150,000-450,000/mm³)
- Serum lactate: 5.2 mmol/L (normal: 0.5-2.2 mmol/L)
- Creatinine: 1.6 mg/dL (normal: 0.7-1.3 mg/dL)

IMAGING:
An urgent CT scan of the abdomen shows no free air but significant ascites and hemoperitoneum.
Bedside echocardiography shows a small, hyperdynamic left ventricle with normal systolic function and no pericardial effusion.
