# synthetic_hospital patient 2054

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 43 at first encounter, sex M, Non-Hispanic White, Private employer-sponsored insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-06-01 (Thu) | outpatient | Gastroenterology |
| 1 | 2020-04-15 | 2023-09-06 (Wed) | outpatient | Gastroenterology |
| 2 | 2021-01-15 | 2024-06-03 (Mon) | outpatient | Endocrinology |
| 3 | 2021-07-15 | 2024-11-29 (Fri) | outpatient | Nephrology |
| 4 | 2022-01-15 | 2025-05-30 (Fri) | outpatient | Orthopedics |
| 5 | 2022-07-15 | 2025-12-01 (Mon) | ed | Emergency Medicine |
| 6 | 2022-09-15 | 2026-01-27 (Tue) | follow_up | Gastroenterology |
| 7 | 2023-01-15 | 2026-06-14 (Sun) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private employer-sponsored insurance",
 "occupation": "Warehouse forklift operator",
 "alcohol_use": "Social alcohol use (1-2 beers on weekends)",
 "family_history": [
  "Father: Autosomal dominant polycystic kidney disease",
  "Mother: Hypertension, Type 2 diabetes mellitus",
  "Paternal uncle: Gout"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "20 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg twice daily",
   "name": "Omeprazole"
  },
  {
   "dose": "1000 mg twice daily",
   "name": "Metformin"
  },
  {
   "dose": "500 mg every 6 hours as needed for pain",
   "name": "Acetaminophen"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Hypertension",
  "Type 2 Diabetes Mellitus (poorly controlled)",
  "Obesity",
  "Zollinger-Ellison syndrome",
  "Acromegaly",
  "Autosomal Dominant Polycystic Kidney Disease",
  "Recurrent peptic ulcer disease",
  "Chronic mechanical lower back pain"
 ],
 "age_at_first_encounter": 43
}
```

Primary diagnoses: ["Upper gastrointestinal bleeding (peptic ulcer disease or erosive gastritis)", "Zollinger-Ellison syndrome", "Pseudogynecomastia", "Chronic mechanical lower back pain", "Autosomal Dominant Polycystic Kidney Disease", "Acute Gout (Gouty Arthritis)", "Acromegaly"]
Comorbidities: ["Essential hypertension", "Upper gastrointestinal bleeding (peptic ulcer disease or erosive gastritis)"]

## Encounter 0: source 2020-01-15, practice 2023-06-01, outpatient, Gastroenterology

Chief complaint: Abdominal pain, diarrhea, and heartburn

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Abdominal pain, diarrhea, and heartburn

HISTORY OF PRESENT ILLNESS:
He presents to the clinic with a 6-month history of abdominal pain, diarrhea, and heartburn that has not improved with over-the-counter antacids or a prescribed proton pump inhibitor. He reports experiencing significant weight loss despite an increased appetite.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Type 2 Diabetes Mellitus (poorly controlled)
- Obesity
- Zollinger-Ellison syndrome
- Acromegaly
- Autosomal Dominant Polycystic Kidney Disease
- Recurrent peptic ulcer disease
- Chronic mechanical lower back pain

His medical history is significant for multiple duodenal ulcers diagnosed over the past year, which have been refractory to treatment.

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Omeprazole 40 mg twice daily
- Metformin 1000 mg twice daily
- Acetaminophen 500 mg every 6 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Autosomal dominant polycystic kidney disease
- Mother: Hypertension, Type 2 diabetes mellitus
- Paternal uncle: Gout

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social alcohol use (1-2 beers on weekends). Occupation: Warehouse forklift operator.

PHYSICAL EXAMINATION:
Physical examination reveals a mildly distended abdomen with epigastric tenderness. No lymphadenopathy is noted.

LABORATORY DATA:
Laboratory tests reveal:

- Serum gastrin: 1100 pg/mL (Normal: <100 pg/mL)
- Fasting serum glucose: 85 mg/dL (Normal: 70-99 mg/dL)
- Hemoglobin: 13.5 g/dL (Normal: 13.8-17.2 g/dL)
- Stool fat: Elevated

OTHER STUDIES:
An upper gastrointestinal endoscopy shows multiple ulcers in the duodenum and one ulcer in the jejunum.

## Encounter 1: source 2020-04-15, practice 2023-09-06, outpatient, Gastroenterology

Chief complaint: Epigastric pain, diarrhea, and heartburn

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Epigastric pain, diarrhea, and heartburn

HISTORY OF PRESENT ILLNESS:
The patient returns to the Gastroenterology clinic for follow-up of his Zollinger-Ellison syndrome and presents with a 6-month history of epigastric pain, diarrhea, and heartburn that is unresponsive to over-the-counter antacids and proton pump inhibitors. He reports a 10-pound weight loss over the same period.

PAST MEDICAL HISTORY:
Active Problem List:
- Zollinger-Ellison syndrome (diagnosed 2020-01-15)
- Hypertension
- Type 2 Diabetes Mellitus (poorly controlled)
- Obesity
- Acromegaly
- Autosomal Dominant Polycystic Kidney Disease
- Recurrent peptic ulcer disease
- Chronic mechanical lower back pain

His past medical history is significant for recurrent peptic ulcer disease, with three episodes in the past year.

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Omeprazole 40 mg twice daily
- Metformin 1000 mg twice daily
- Acetaminophen 500 mg every 6 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Autosomal dominant polycystic kidney disease
- Mother: Hypertension, Type 2 diabetes mellitus
- Paternal uncle: Gout

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social alcohol use (1-2 beers on weekends). Occupation: Warehouse forklift operator.

PHYSICAL EXAMINATION:
Physical examination reveals mild epigastric tenderness.

LABORATORY DATA:
Laboratory tests show:

- Hemoglobin: 13.2 g/dL
- Fasting serum gastrin: 980 pg/mL
- Gastric pH: 1.5

The patient's fasting serum gastrin level is rechecked and confirmed.

OTHER STUDIES:
Endoscopy reveals multiple stomach ulcers and thickened gastric folds.

## Encounter 2: source 2021-01-15, practice 2024-06-03, outpatient, Endocrinology

Chief complaint: Headaches, joint pain, and increased ring size

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Michael Rodriguez, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Headaches, joint pain, and increased ring size

HISTORY OF PRESENT ILLNESS:
The patient with Zollinger-Ellison syndrome previously evaluated by Gastroenterology in 2020 presents for Endocrinology consultation. He reports excessive sweating and facial feature changes including jaw protrusion and widening spaces between his teeth over the past two years. His wife has observed that his voice has deepened.

PAST MEDICAL HISTORY:
Active Problem List:
- Zollinger-Ellison syndrome (diagnosed 2020-01-15)
- Hypertension
- Type 2 Diabetes Mellitus (poorly controlled)
- Obesity
- Acromegaly
- Autosomal Dominant Polycystic Kidney Disease
- Recurrent peptic ulcer disease
- Chronic mechanical lower back pain

His medical history is significant for hypertension and type 2 diabetes mellitus, which are poorly controlled despite adherence to medications.

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Omeprazole 40 mg twice daily
- Metformin 1000 mg twice daily
- Acetaminophen 500 mg every 6 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Autosomal dominant polycystic kidney disease
- Mother: Hypertension, Type 2 diabetes mellitus
- Paternal uncle: Gout

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social alcohol use (1-2 beers on weekends). Occupation: Warehouse forklift operator.

VITAL SIGNS:
His blood pressure is 150/95 mmHg.

PHYSICAL EXAMINATION:
On physical examination, the patient has enlarged hands and feet, coarse facial features with frontal bossing, and a prominent jaw.

LABORATORY DATA:
Laboratory tests reveal the following:

- Fasting glucose: 180 mg/dL
- Hemoglobin A1c: 8.5%
- Serum insulin-like growth factor-1 (IGF-1): Elevated

## Encounter 3: source 2021-07-15, practice 2024-11-29, outpatient, Nephrology

Chief complaint: Intermittent flank pain and recent onset of headaches

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Priya Patel, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent flank pain and recent onset of headaches

HISTORY OF PRESENT ILLNESS:
Patient returns for Nephrology follow-up after Endocrinology evaluation. He mentions increased urinary frequency and sensation of incomplete bladder emptying.

PAST MEDICAL HISTORY:
Active Problem List:
- Zollinger-Ellison syndrome (diagnosed 2020-01-15)
- Acromegaly (diagnosed 2021-01-15)
- Hypertension
- Type 2 Diabetes Mellitus (poorly controlled)
- Obesity
- Autosomal Dominant Polycystic Kidney Disease
- Recurrent peptic ulcer disease
- Chronic mechanical lower back pain

He reports a history of hypertension, which was diagnosed two years ago.

MEDICATIONS:
His blood pressure is currently controlled on lisinopril.

Home Medications:
- Omeprazole 40 mg twice daily
- Metformin 1000 mg twice daily
- Acetaminophen 500 mg every 6 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
His family history is significant for his father having "kidney problems" that required dialysis and a sister with a "brain bleed" at a young age.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social alcohol use (1-2 beers on weekends). Occupation: Warehouse forklift operator.

PHYSICAL EXAMINATION:
Physical examination reveals bilateral flank tenderness.

LABORATORY DATA:
Urinalysis shows no evidence of infection or hematuria. Blood tests reveal:
- Serum creatinine: 1.8 mg/dL (Normal: 0.6-1.2 mg/dL)
- Blood urea nitrogen (BUN): 29 mg/dL (Normal: 7-20 mg/dL)
- Hemoglobin: 13.2 g/dL (Normal: 13.8-17.2 g/dL)
- Calcium: 9.2 mg/dL (Normal: 8.5-10.2 mg/dL)

IMAGING:
Abdominal ultrasound demonstrates enlarged kidneys with multiple cysts.

## Encounter 4: source 2022-01-15, practice 2025-05-30, outpatient, Orthopedics

Chief complaint: Lower back pain for the past week

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. James Okonkwo, MD
Department: Orthopedics
Visit Type: outpatient

CHIEF COMPLAINT:
Lower back pain for the past week

HISTORY OF PRESENT ILLNESS:
The patient with Zollinger-Ellison syndrome, acromegaly, and autosomal dominant polycystic kidney disease presents for Orthopedic evaluation of recurrent lower back pain. He denies any trauma or injury but mentions that his job involves heavy lifting. The pain is described as a constant dull ache, which worsens with movement and is somewhat relieved by lying down. He has tried taking acetaminophen with minimal relief.

PAST MEDICAL HISTORY:
Active Problem List:
- Zollinger-Ellison syndrome (diagnosed 2020-01-15)
- Acromegaly (diagnosed 2021-01-15)
- Autosomal Dominant Polycystic Kidney Disease (diagnosed 2021-07-15)
- Hypertension
- Type 2 Diabetes Mellitus (poorly controlled)
- Obesity
- Recurrent peptic ulcer disease
- Chronic mechanical lower back pain

His medical history is significant for peptic ulcer disease,.

MEDICATIONS:
He takes for which he takes omeprazole.

Home Medications:
- Lisinopril 20 mg daily
- Metformin 1000 mg twice daily
- Acetaminophen 500 mg every 6 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Autosomal dominant polycystic kidney disease
- Mother: Hypertension, Type 2 diabetes mellitus
- Paternal uncle: Gout

SOCIAL HISTORY:
He does not smoke and drinks alcohol socially.

VITAL SIGNS:
His vital signs are within normal limits.

PHYSICAL EXAMINATION:
On physical examination, there is paraspinal muscle tenderness in the lumbar region, but no neurological deficits are noted. Straight leg raise test is negative bilaterally.

## Encounter 5: source 2022-07-15, practice 2025-12-01, ed, Emergency Medicine

Chief complaint: 4-month history of epigastric pain

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Fatima Al-Rashid, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
4-month history of epigastric pain

HISTORY OF PRESENT ILLNESS:
The patient, with known Zollinger-Ellison syndrome managed by Gastroenterology since 2020, returns with a four-month history of epigastric pain. He describes the pain as a gnawing sensation that worsens when his stomach is empty and often wakes him up at night. He also reports occasional postprandial fullness and nausea. He denies vomiting, weight loss, or changes in bowel habits.

PAST MEDICAL HISTORY:
Active Problem List:
- Zollinger-Ellison syndrome (diagnosed 2020-01-15)
- Acromegaly (diagnosed 2021-01-15)
- Autosomal Dominant Polycystic Kidney Disease (diagnosed 2021-07-15)
- Chronic mechanical lower back pain (diagnosed 2022-01-15)
- Hypertension
- Type 2 Diabetes Mellitus (poorly controlled)
- Obesity
- Recurrent peptic ulcer disease

His past medical history is significant for hypertension.

MEDICATIONS:
He takes for which he takes hydrochlorothiazide.

Home Medications:
- Lisinopril 20 mg daily
- Omeprazole 40 mg twice daily
- Metformin 1000 mg twice daily
- Acetaminophen 500 mg every 6 hours as needed for pain

ALLERGIES:
He has no known drug allergies

FAMILY HISTORY:
- Father: Autosomal dominant polycystic kidney disease
- Mother: Hypertension, Type 2 diabetes mellitus
- Paternal uncle: Gout

SOCIAL HISTORY:
He does not smoke or consume alcohol

VITAL SIGNS:
On physical examination, his vital signs are within normal limits.

PHYSICAL EXAMINATION:
Abdominal examination reveals mild tenderness to palpation in the epigastric region without rebound or guarding.

LABORATORY DATA:
Laboratory results are as follows:

- Hemoglobin: 13.8 g/dL
- White blood cell count: 7,200/mm^3
- Platelets: 250,000/mm^3
- Serum urea: 29 mg/dL
- Serum creatinine: 0.9 mg/dL
- Fasting blood glucose: 98 mg/dL

OTHER STUDIES:
Stool guaiac test is negative.

## Encounter 6: source 2022-09-15, practice 2026-01-27, follow_up, Gastroenterology

Chief complaint: Follow-up after treatment for gastric ulcers

ENCOUNTER NOTE
Date: 2022-09-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: follow_up

CHIEF COMPLAINT:
Follow-up after treatment for gastric ulcers

HISTORY OF PRESENT ILLNESS:
This 45-year-old male with Zollinger-Ellison syndrome, acromegaly, and recurrent peptic ulcer disease presents for follow-up following his recent emergency department evaluation for upper gastrointestinal bleeding in July 2022. He was diagnosed with peptic ulcer disease six weeks ago and started on a medication that has led to significant improvement in his epigastric pain and reflux symptoms. However, he now reports new-onset breast tenderness and a small amount of breast tissue development on physical exam. He is adherent to his ulcer treatment and reports no further episodes of melena or hematemesis.

PAST MEDICAL HISTORY:
Active Problem List:
- Zollinger-Ellison syndrome (diagnosed 2020-01-15)
- Acromegaly (diagnosed 2021-01-15)
- Autosomal Dominant Polycystic Kidney Disease (diagnosed 2021-07-15)
- Chronic mechanical lower back pain (diagnosed 2022-01-15)
- Upper gastrointestinal bleeding (peptic ulcer disease or erosive gastritis) (diagnosed 2022-07-15)
- Hypertension
- Type 2 Diabetes Mellitus (poorly controlled)
- Obesity
- Recurrent peptic ulcer disease

MEDICATIONS:
He takes no other medications and does not consume any herbal products.

Home Medications:
- Lisinopril 20 mg daily
- Omeprazole 40 mg twice daily
- Metformin 1000 mg twice daily
- Acetaminophen 500 mg every 6 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Autosomal dominant polycystic kidney disease
- Mother: Hypertension, Type 2 diabetes mellitus
- Paternal uncle: Gout

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social alcohol use (1-2 beers on weekends). Occupation: Warehouse forklift operator.

REVIEW OF SYSTEMS:
He denies any testicular pain, erectile dysfunction, or changes in libido.

VITAL SIGNS:
Vital signs are within normal limits

PHYSICAL EXAMINATION:
Examination reveals bilateral, tender subareolar fullness without nodularity or nipple discharge. No signs of chronic liver disease or testicular abnormalities are present.

## Encounter 7: source 2023-01-15, practice 2026-06-14, ed, Emergency Medicine

Chief complaint: Severe pain in his left first metatarsophalangeal joint

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe pain in his left first metatarsophalangeal joint

HISTORY OF PRESENT ILLNESS:
This 46-year-old male with acromegaly, ADPKD, recurrent PUD, and poorly controlled T2DM presents to the emergency department with a 16-hour history of severe pain in his left first metatarsophalangeal joint. He reports drinking several beers at a party the night before the pain started.

PAST MEDICAL HISTORY:
Active Problem List:
- Zollinger-Ellison syndrome (diagnosed 2020-01-15)
- Acromegaly (diagnosed 2021-01-15)
- Autosomal Dominant Polycystic Kidney Disease (diagnosed 2021-07-15)
- Chronic mechanical lower back pain (diagnosed 2022-01-15)
- Upper gastrointestinal bleeding (peptic ulcer disease or erosive gastritis) (diagnosed 2022-07-15)
- Pseudogynecomastia (diagnosed 2022-09-15)
- Hypertension
- Type 2 Diabetes Mellitus (poorly controlled)
- Obesity
- Recurrent peptic ulcer disease

He has a past medical history of hypertension and obesity.

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Omeprazole 40 mg twice daily
- Metformin 1000 mg twice daily
- Acetaminophen 500 mg every 6 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Autosomal dominant polycystic kidney disease
- Mother: Hypertension, Type 2 diabetes mellitus
- Paternal uncle: Gout

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social alcohol use (1-2 beers on weekends). Occupation: Warehouse forklift operator.

VITAL SIGNS:
He is afebrile and denies any recent trauma or history of autoimmune disease.

PHYSICAL EXAMINATION:
Physical examination reveals a swollen, erythematous, and exquisitely tender big toe joint with markedly restricted range of motion.

LABORATORY DATA:
Laboratory studies are negative for rheumatoid factor and antinuclear antibodies.

OTHER STUDIES:
A joint aspirate is shown.
