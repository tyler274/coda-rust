# synthetic_hospital patient 2092

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex M, Non-Hispanic White, Private commercial insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-12-02 (Mon) | inpatient | Orthopedic Surgery |
| 1 | 2020-09-15 | 2025-08-01 (Fri) | ed | Emergency Medicine |
| 2 | 2021-07-15 | 2026-06-07 (Sun) | icu | Critical Care Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Private commercial insurance",
 "occupation": "Construction site supervisor",
 "alcohol_use": "Social, 3-4 beers per week",
 "family_history": [
  "Father: Mitral valve prolapse",
  "Mother: Cholelithiasis",
  "Maternal uncle: Chronic pancreatitis"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit at age 42, 12 pack-year history",
 "home_medications": [
  {
   "dose": "36,000 units lipase with meals and snacks",
   "name": "Pancrelipase"
  },
  {
   "dose": "81 mg daily",
   "name": "Aspirin"
  },
  {
   "dose": "20 mg daily",
   "name": "Omeprazole"
  }
 ],
 "surgical_history": [
  "Open reduction and internal fixation, right femoral shaft fracture (age 45)",
  "Laparoscopic cholecystectomy (age 52)"
 ],
 "chronic_conditions": [
  "Obesity (BMI 34)",
  "Chronic gallstone pancreatitis",
  "Mitral valve prolapse with mild regurgitation"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Refractory hypoxemia in ARDS", "Infective Endocarditis (Aortic Valve) with Complications", "Hypoalbuminemia"]
Comorbidities: ["Status post laparoscopic cholecystectomy"]

## Encounter 0: source 2020-01-15, practice 2024-12-02, inpatient, Orthopedic Surgery

Chief complaint: Severe right thigh pain and deformity after motorcycle collision; unable to bear weight.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Orthopedic Surgery
Visit Type: inpatient

CHIEF COMPLAINT:
Severe right thigh pain and deformity after motorcycle collision; unable to bear weight.

HISTORY OF PRESENT ILLNESS:
He is undergoing emergency open reduction and internal fixation of a femoral fracture following a motorcycle accident. On arrival to the emergency department, he is alert but in severe pain and requires urgent surgical intervention. Rapid sequence induction is performed for intubation using propofol and a neuromuscular blocking agent. Within seconds of the paralytic administration, transient facial and chest fasciculations are noted, followed by full body paralysis. Surgery is completed without complication. However, over an hour after discontinuation of anesthesia, the patient remains paralyzed and is unable to breathe spontaneously.

PAST MEDICAL HISTORY:
Active Problem List:
- Obesity (BMI 34)
- Chronic gallstone pancreatitis
- Mitral valve prolapse with mild regurgitation

PAST SURGICAL HISTORY:
- Open reduction and internal fixation, right femoral shaft fracture (age 45)
- Laparoscopic cholecystectomy (age 52)

MEDICATIONS:
Home Medications:
- Pancrelipase 36,000 units lipase with meals and snacks
- Aspirin 81 mg daily
- Omeprazole 20 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Mitral valve prolapse
- Mother: Cholelithiasis
- Maternal uncle: Chronic pancreatitis

SOCIAL HISTORY:
Smoking: Former smoker, quit at age 42, 12 pack-year history. Alcohol: Social, 3-4 beers per week. Occupation: Construction site supervisor.

VITAL SIGNS:
Vital signs are stable

PHYSICAL EXAMINATION:
Repeat neurologic examination shows flaccid paralysis without sedation overdose

ASSESSMENT AND PLAN:
The anesthesiologist suspects an underlying metabolic or genetic issue. A deficiency in

## Encounter 1: source 2020-09-15, practice 2025-08-01, ed, Emergency Medicine

Chief complaint: Two-week history of intermittent fevers, night sweats, and fatigue; noticed new heart murmur.

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Marcus Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-week history of intermittent fevers, night sweats, and fatigue; noticed new heart murmur.

HISTORY OF PRESENT ILLNESS:
This 45-year-old male with chronic gallstone pancreatitis and mitral valve prolapse with mild regurgitation, status post prior orthopedic surgery, presents to the Emergency Department. He reports having undergone a dental extraction 3 weeks ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypoalbuminemia (diagnosed 2020-01-15)
- Obesity (BMI 34)
- Chronic gallstone pancreatitis
- Mitral valve prolapse with mild regurgitation

PAST SURGICAL HISTORY:
- Open reduction and internal fixation, right femoral shaft fracture (age 45)
- Laparoscopic cholecystectomy (age 52)

MEDICATIONS:
Home Medications:
- Pancrelipase 36,000 units lipase with meals and snacks
- Aspirin 81 mg daily
- Omeprazole 20 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Mitral valve prolapse
- Mother: Cholelithiasis
- Maternal uncle: Chronic pancreatitis

SOCIAL HISTORY:
He denies any recent travel or intravenous drug use.

VITAL SIGNS:
His temperature is 38.5°C, heart rate is 102 bpm, and blood pressure is 130/85 mmHg

PHYSICAL EXAMINATION:
A dental examination reveals good oral hygiene without evidence of ongoing infection. Auscultation of the heart reveals a blowing, high-pitched murmur best heard at the apex and radiating to the axilla.

LABORATORY DATA:
Laboratory findings are as follows:

- White blood cell count: 12,000/µL
- Hemoglobin: 13.2 g/dL
- Platelet count: 250,000/µL
- Erythrocyte sedimentation rate: 45 mm/hr
- Blood cultures: Pending

IMAGING:
Transthoracic echocardiography shows vegetations on the mitral valve.

## Encounter 2: source 2021-07-15, practice 2026-06-07, icu, Critical Care Medicine

Chief complaint: Acute onset of severe shortness of breath and refractory hypoxemia immediately following laparoscopic cholecystectomy.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Emily Watson, MD
Department: Critical Care Medicine
Visit Type: icu

CHIEF COMPLAINT:
Acute onset of severe shortness of breath and refractory hypoxemia immediately following laparoscopic cholecystectomy.

HISTORY OF PRESENT ILLNESS:
The patient is a 46-year-old male with obesity, chronic gallstone pancreatitis, and prior admissions for traumatic right thigh injury with hypoalbuminemia and subsequent infective endocarditis involving the aortic valve, who presents to the ICU following elective laparoscopic cholecystectomy. During the procedure, a 26-year-old female surgical resident participates as part of the operative team; the attending surgeon becomes visibly frustrated with the resident's technique and loudly criticizes her in front of the team, causing her to become anxious. Despite this, the surgery is completed without immediate complications. Postoperatively, the patient develops tachypnea and hypoxia. The resident suggests a diagnosis, but the attending surgeon dismisses her concern without consideration.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypoalbuminemia (diagnosed 2020-01-15)
- Infective Endocarditis (Aortic Valve) with Complications (diagnosed 2020-09-15)
- Obesity (BMI 34)
- Chronic gallstone pancreatitis
- Mitral valve prolapse with mild regurgitation

PAST SURGICAL HISTORY:
- Open reduction and internal fixation, right femoral shaft fracture (age 45)
- Laparoscopic cholecystectomy (age 52)

MEDICATIONS:
Home Medications:
- Pancrelipase 36,000 units lipase with meals and snacks
- Aspirin 81 mg daily
- Omeprazole 20 mg daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Mitral valve prolapse
- Mother: Cholelithiasis
- Maternal uncle: Chronic pancreatitis

SOCIAL HISTORY:
Smoking: Former smoker, quit at age 42, 12 pack-year history. Alcohol: Social, 3-4 beers per week. Occupation: Construction site supervisor.

VITAL SIGNS:
Afebrile, with a respiratory rate of 28 breaths/min, oxygen saturation of 92% on room air

PHYSICAL EXAMINATION:
Diffuse crackles on lung auscultation

LABORATORY DATA:
Postoperative day 1 laboratory values are as follows:

- White blood cell count: 12,000/uL
- Hemoglobin: 13.5 g/dL
- Platelet count: 250,000/uL
- Arterial blood gas: pH 7.48, PaCO2 32 mmHg, PaO2 70 mmHg
- Serum creatinine: 1.0 mg/dL
- B-type natriuretic peptide (BNP): 50 pg/mL

IMAGING:
Chest X-ray shows bilateral pulmonary infiltrates
