# synthetic_hospital patient 1971

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 52 at first encounter, sex M, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-11-20 (Thu) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2026-05-12 (Tue) | ed | Emergency Medicine |
| 2 | 2020-07-15 | 2026-05-23 (Sat) | inpatient | Thoracic Surgery |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Sulfa antibiotics (rash)",
  "Latex (contact dermatitis)"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Automotive parts manager",
 "alcohol_use": "Heavy episodic consumption (5-6 drinks on weekends and holidays)",
 "family_history": [
  "Father with hypertension and myocardial infarction (age 65)",
  "Mother with GERD and type 2 diabetes (age 78)",
  "Older brother with alcohol use disorder"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 3 years ago, 15 pack-year history",
 "home_medications": [
  {
   "dose": "20 mg PO daily",
   "name": "Omeprazole"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Amlodipine"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "Inguinal hernia repair (age 44)",
  "Tonsillectomy (age 8)"
 ],
 "chronic_conditions": [
  "Gastroesophageal reflux disease (GERD)",
  "Hypertension",
  "Obesity (BMI 31)"
 ],
 "age_at_first_encounter": 52
}
```

Primary diagnoses: ["Esophageal rupture with pleural effusion"]
Comorbidities: ["Gastroesophageal reflux disease", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2025-11-20, outpatient, Internal Medicine

Chief complaint: Routine follow-up for gastroesophageal reflux disease and hypertension management

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for gastroesophageal reflux disease and hypertension management

HISTORY OF PRESENT ILLNESS:
Sudden onset of severe chest pain and shortness of breath following an episode of intense vomiting after a large meal and alcohol consumption. He describes the pain as sharp and radiating to his back.

PAST MEDICAL HISTORY:
Active Problem List:
- Gastroesophageal reflux disease (GERD)
- Hypertension
- Obesity (BMI 31)

PAST SURGICAL HISTORY:
- Inguinal hernia repair (age 44)
- Tonsillectomy (age 8)

MEDICATIONS:
He takes omeprazole.

Home Medications:
- Amlodipine 5 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
Sulfa antibiotics (rash), Latex (contact dermatitis)

FAMILY HISTORY:
- Father with hypertension and myocardial infarction (age 65)
- Mother with GERD and type 2 diabetes (age 78)
- Older brother with alcohol use disorder

SOCIAL HISTORY:
Smoking: Former smoker, quit 3 years ago, 15 pack-year history. Alcohol: Heavy episodic consumption (5-6 drinks on weekends and holidays). Occupation: Automotive parts manager.

VITAL SIGNS:
Blood pressure is 145/90 mmHg

PHYSICAL EXAMINATION:
On examination, the patient appears in distress, with tachypnea and tachycardia. Auscultation of the chest reveals decreased breath sounds at the left base. Palpation of the neck and upper chest elicits crepitus.

LABORATORY DATA:
White blood cell count: 18,000 /µL
- Hemoglobin: 13.5 g/dL
- Platelets: 250,000 /µL
- Serum amylase: 90 U/L (normal range: 30-110 U/L)
- Arterial blood gas: pH 7.35, PaCO2 45 mmHg, PaO2 60 mmHg

IMAGING:
A chest X-ray shows mediastinal widening and left-sided pleural effusion.

## Encounter 1: source 2020-07-15, practice 2026-05-12, ed, Emergency Medicine

Chief complaint: Presents to the emergency department with sudden onset of severe chest pain and shortness of breath

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Presents to the emergency department with sudden onset of severe chest pain and shortness of breath

HISTORY OF PRESENT ILLNESS:
This 52-year-old male with GERD and hypertension was seen in January. He presents following an episode of intense vomiting after a large meal and alcohol consumption, describing the pain as sharp and radiating to his back.

PAST MEDICAL HISTORY:
Active Problem List:
- Esophageal rupture with pleural effusion (diagnosed 2020-01-15)
- Gastroesophageal reflux disease (GERD)
- Hypertension
- Obesity (BMI 31)

His medical history is significant for gastroesophageal reflux disease (GERD) and hypertension.

PAST SURGICAL HISTORY:
- Inguinal hernia repair (age 44)
- Tonsillectomy (age 8)

MEDICATIONS:
Home Medications:
- Omeprazole 20 mg PO daily
- Amlodipine 5 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
Sulfa antibiotics (rash), Latex (contact dermatitis)

FAMILY HISTORY:
- Father with hypertension and myocardial infarction (age 65)
- Mother with GERD and type 2 diabetes (age 78)
- Older brother with alcohol use disorder

SOCIAL HISTORY:
Smoking: Former smoker, quit 3 years ago, 15 pack-year history. Alcohol: Heavy episodic consumption (5-6 drinks on weekends and holidays). Occupation: Automotive parts manager.

VITAL SIGNS:
Blood pressure of 150/90 mmHg, heart rate of 110 bpm, and a temperature of 37.8°C (100°F)

PHYSICAL EXAMINATION:
On examination, he appears in distress. His breath sounds are diminished on the left side, and there is crepitus palpable in the left supraclavicular area.

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 18,000 /µL
- Hemoglobin: 13.5 g/dL
- Platelets: 250,000 /µL
- Serum amylase: 90 U/L (normal range: 30-110 U/L)
- Arterial blood gas:
- pH: 7.45
- PaCO2: 32 mmHg
- PaO2: 60 mmHg
- Bicarbonate: 24 mEq/L

IMAGING:
A chest X-ray reveals a large left-sided pleural effusion and pneumomediastinum.

## Encounter 2: source 2020-07-15, practice 2026-05-23, inpatient, Thoracic Surgery

Chief complaint: Presents to the emergency department with sudden onset of severe chest pain and shortness of breath

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Elena Rodriguez, MD
Department: Thoracic Surgery
Visit Type: inpatient

CHIEF COMPLAINT:
Presents to the emergency department with sudden onset of severe chest pain and shortness of breath

HISTORY OF PRESENT ILLNESS:
The patient with known gastroesophageal reflux disease returns for admission following an episode of intense vomiting after a large meal and alcohol consumption. He describes the pain as sharp and radiating to his back.

PAST MEDICAL HISTORY:
Active Problem List:
- Esophageal rupture with pleural effusion (diagnosed 2020-01-15)
- Gastroesophageal reflux disease (GERD)
- Hypertension
- Obesity (BMI 31)

His medical history is significant for chronic gastroesophageal reflux disease (GERD).

PAST SURGICAL HISTORY:
- Inguinal hernia repair (age 44)
- Tonsillectomy (age 8)

MEDICATIONS:
Home Medications:
- Omeprazole 20 mg PO daily
- Amlodipine 5 mg PO daily
- Aspirin 81 mg PO daily

ALLERGIES:
Sulfa antibiotics (rash), Latex (contact dermatitis)

FAMILY HISTORY:
- Father with hypertension and myocardial infarction (age 65)
- Mother with GERD and type 2 diabetes (age 78)
- Older brother with alcohol use disorder

SOCIAL HISTORY:
Smoking: Former smoker, quit 3 years ago, 15 pack-year history. Alcohol: Heavy episodic consumption (5-6 drinks on weekends and holidays). Occupation: Automotive parts manager.

VITAL SIGNS:
His blood pressure is 145/90 mmHg.

PHYSICAL EXAMINATION:
On examination, he appears in distress, with tachypnea and tachycardia. Auscultation reveals decreased breath sounds in the left lower lung field. Subcutaneous emphysema is palpated in the neck.

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 18,000 /µL
- Hemoglobin: 13.5 g/dL
- Blood urea nitrogen: 21 mg/dL
- Serum creatinine: 1.0 mg/dL
- Serum amylase: 3000 IU/L
- Pleural fluid pH: 6.0

IMAGING:
A chest x-ray shows mediastinal widening and a left-sided pleural effusion.
