# synthetic_hospital patient 2545

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 28 at first encounter, sex M, Caucasian, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-11-09 (Sat) | ed | Emergency Medicine |
| 1 | 2020-09-15 | 2025-07-10 (Thu) | icu | Emergency Medicine |
| 2 | 2021-09-15 | 2026-07-16 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Penicillin (rash)",
  "Contrast dye (urticaria)"
 ],
 "insurance": "Medicaid",
 "occupation": "Freelance graphic designer",
 "alcohol_use": "Occasional (1-2 drinks per month)",
 "family_history": [
  "Father with Marfan syndrome, deceased at age 42 from aortic dissection (diagnosed posthumously)",
  "Mother with essential hypertension",
  "Paternal aunt with Marfan syndrome, status post prophylactic aortic root replacement"
 ],
 "race_ethnicity": "Caucasian",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "50 mg PO daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "Ascending aortic replacement with valve resuspension (hemiarch repair)",
  "Emergency thoracotomy with pericardial window (during prior dissection repair)"
 ],
 "chronic_conditions": [
  "Marfan syndrome",
  "Stimulant use disorder, severe, cocaine",
  "Hypertension",
  "Status post Type A aortic dissection repair"
 ],
 "age_at_first_encounter": 28
}
```

Primary diagnoses: ["Type A Aortic Dissection", "Diffuse alveolar hemorrhage"]
Comorbidities: ["Aortic Regurgitation", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2024-11-09, ed, Emergency Medicine

Chief complaint: Sudden onset of severe, tearing chest pain radiating to the back rated 10/10 intensity

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of severe, tearing chest pain radiating to the back rated 10/10 intensity

HISTORY OF PRESENT ILLNESS:
He describes the pain as "tearing" and rates it 10/10 in intensity. He also reports mild shortness of breath but denies cough, fever, or leg swelling.

PAST MEDICAL HISTORY:
Active Problem List:
- Marfan syndrome
- Stimulant use disorder, severe, cocaine
- Hypertension
- Status post Type A aortic dissection repair

PAST SURGICAL HISTORY:
- Ascending aortic replacement with valve resuspension (hemiarch repair)
- Emergency thoracotomy with pericardial window (during prior dissection repair)

MEDICATIONS:
Home Medications:
- Metoprolol succinate 50 mg PO daily
- Lisinopril 10 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
Penicillin (rash), Contrast dye (urticaria)

FAMILY HISTORY:
- Father with Marfan syndrome, deceased at age 42 from aortic dissection (diagnosed posthumously)
- Mother with essential hypertension
- Paternal aunt with Marfan syndrome, status post prophylactic aortic root replacement

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per month). Occupation: Freelance graphic designer.

VITAL SIGNS:
His blood pressure is 145/90 mmHg in the right arm and 130/85 mmHg in the left arm. His pulse is 110 bpm and regular.

PHYSICAL EXAMINATION:
Physical examination reveals a tall, thin man with long extremities, arachnodactyly, and a high-arched palate. A diastolic murmur is heard along the right sternal border.

LABORATORY DATA:
Laboratory Values:
- Hemoglobin: 14.2 g/dL
- White blood cell count: 7.8 x 10^3/uL
- Platelets: 250 x 10^3/uL
- Creatinine: 1.0 mg/dL
- Troponin I: <0.04 ng/mL (normal)

IMAGING:
Chest X-ray demonstrates a widened mediastinum. A bedside transthoracic echocardiogram reveals a dilated aortic root with evidence of aortic regurgitation.

OTHER STUDIES:
His electrocardiogram shows no evidence of ischemia or infarction.

## Encounter 1: source 2020-09-15, practice 2025-07-10, icu, Emergency Medicine

Chief complaint: Sudden onset of difficulty breathing and coughing up blood after inhaling cocaine at a party

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Sudden onset of difficulty breathing and coughing up blood after inhaling cocaine at a party

HISTORY OF PRESENT ILLNESS:
This 28-year-old male with Marfan syndrome and prior Type A aortic dissection repair returns anxious and diaphoretic. His friends report that he was at a party earlier where he inhaled a substance they believed to be cocaine.

PAST MEDICAL HISTORY:
Active Problem List:
- Type A Aortic Dissection (diagnosed 2020-01-15)
- Marfan syndrome
- Stimulant use disorder, severe, cocaine
- Hypertension
- Status post Type A aortic dissection repair

PAST SURGICAL HISTORY:
- Ascending aortic replacement with valve resuspension (hemiarch repair)
- Emergency thoracotomy with pericardial window (during prior dissection repair)

MEDICATIONS:
Home Medications:
- Metoprolol succinate 50 mg PO daily
- Lisinopril 10 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
He has no known drug allergies

FAMILY HISTORY:
- Father with Marfan syndrome, deceased at age 42 from aortic dissection (diagnosed posthumously)
- Mother with essential hypertension
- Paternal aunt with Marfan syndrome, status post prophylactic aortic root replacement

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per month). Occupation: Freelance graphic designer.

VITAL SIGNS:
His blood pressure is 150/95 mmHg, heart rate is 110 beats per minute, respiratory rate is 28 breaths per minute, and oxygen saturation is 88% on room air

PHYSICAL EXAMINATION:
Auscultation of the lungs reveals bilateral crackles. The rest of the physical examination is unremarkable.

LABORATORY DATA:
Laboratory studies are as follows:

- Hemoglobin: 13.2 g/dL
- White blood cell count: 12,000/uL
- Platelets: 250,000/uL
- Prothrombin time: 12 seconds (control 11-13.5 seconds)
- Partial thromboplastin time: 30 seconds (control 25-35 seconds)
- D-dimer: 500 ng/mL (normal <500 ng/mL)
- Urine toxicology screen: Positive for cocaine metabolites

IMAGING:
A chest X-ray shows diffuse bilateral alveolar infiltrates.

## Encounter 2: source 2021-09-15, practice 2026-07-16, ed, Emergency Medicine

Chief complaint: Severe chest discomfort and difficulty breathing beginning 40 minutes prior to arrival

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. Fatima Al-Hassan, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe chest discomfort and difficulty breathing beginning 40 minutes prior to arrival

HISTORY OF PRESENT ILLNESS:
The patient is a 29-year-old male with Marfan syndrome and history of Type A aortic dissection repair and prior diffuse alveolar hemorrhage associated with cocaine use who presents to the emergency department 40 minutes after abrupt onset of severe chest discomfort and difficulty breathing.

PAST MEDICAL HISTORY:
Active Problem List:
- Type A Aortic Dissection (diagnosed 2020-01-15)
- Diffuse alveolar hemorrhage (diagnosed 2020-09-15)
- Marfan syndrome
- Stimulant use disorder, severe, cocaine
- Hypertension
- Status post Type A aortic dissection repair

He has no prior cardiovascular disease or known connective tissue disorder.

PAST SURGICAL HISTORY:
- Ascending aortic replacement with valve resuspension (hemiarch repair)
- Emergency thoracotomy with pericardial window (during prior dissection repair)

MEDICATIONS:
Home Medications:
- Metoprolol succinate 50 mg PO daily
- Lisinopril 10 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
Penicillin (rash), Contrast dye (urticaria)

FAMILY HISTORY:
- Father with Marfan syndrome, deceased at age 42 from aortic dissection (diagnosed posthumously)
- Mother with essential hypertension
- Paternal aunt with Marfan syndrome, status post prophylactic aortic root replacement

SOCIAL HISTORY:
He has a history of chronic stimulant abuse, including intravenous and intranasal cocaine use.

VITAL SIGNS:
His temperature is 37.9°C (100.2°F), heart rate is 112/min, and blood pressure is 160/55 mm Hg.

PHYSICAL EXAMINATION:
On exam, his left radial pulse is significantly weaker than his right, and his left arm is cool to touch. Inspiratory crackles are audible throughout both lung fields, and a high-pitched, decrescendo diastolic murmur is best heard at the left third intercostal space. There are no signs of trauma to the chest or external wounds.

IMAGING:
Chest X-ray demonstrates widening of the mediastinum and prominent aortic knob. A bedside echocardiogram reveals moderate aortic regurgitation and a mobile intimal flap within the aorta.
