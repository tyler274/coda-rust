# synthetic_hospital patient 2131

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 39 at first encounter, sex M, Non-Hispanic White, UnitedHealthcare Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-07-18 (Thu) | outpatient | Internal Medicine |
| 1 | 2020-03-15 | 2024-09-20 (Fri) | outpatient | Internal Medicine |
| 2 | 2020-07-15 | 2025-01-23 (Thu) | outpatient | Neurology |
| 3 | 2021-07-15 | 2026-02-03 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "UnitedHealthcare Commercial PPO",
 "occupation": "Independent construction contractor",
 "alcohol_use": "1-2 glasses of homemade whiskey daily, distilled using an old family kit inherited from his grandfather",
 "family_history": [
  "Father: Alcohol use disorder",
  "Mother: Major depressive disorder",
  "Paternal grandfather: History of moonshine production"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit approximately 5 years ago",
 "home_medications": [
  {
   "dose": "50 mg PO daily",
   "name": "Sertraline"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Major depressive disorder"
 ],
 "age_at_first_encounter": 39
}
```

Primary diagnoses: ["Lead poisoning", "Tetrodotoxin poisoning"]
Comorbidities: ["Peripheral neuropathy", "Occupational exposure to arsenic"]

## Encounter 0: source 2020-01-15, practice 2024-07-18, outpatient, Internal Medicine

Chief complaint: Progressive fatigue, difficulty concentrating at work, and intermittent abdominal discomfort over the past few weeks

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue, difficulty concentrating at work, and intermittent abdominal discomfort over the past few weeks

HISTORY OF PRESENT ILLNESS:
A 38-year-old man comes to the clinic due to progressive fatigue, difficulty concentrating at work, and intermittent abdominal discomfort over the past few weeks. He denies any recent illnesses, weight loss, or overt gastrointestinal bleeding. His wife recently noted that he has become increasingly irritable and forgetful.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He does not take any medications or supplements.

Home Medications:
- Sertraline 50 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder
- Mother: Major depressive disorder
- Paternal grandfather: History of moonshine production

SOCIAL HISTORY:
He drinks 1–2 glasses of homemade whiskey daily, which he distills using an old family kit inherited from his grandfather.

PHYSICAL EXAMINATION:
On examination, he appears pale but is in no acute distress. Neurological exam is normal.

LABORATORY DATA:
Labs reveal:
Hemoglobin 9.6 g/dL
MCV 70 fL
Serum iron Elevated
Ferritin                                       Elevated
Peripheral smear is shown below.

## Encounter 1: source 2020-03-15, practice 2024-09-20, outpatient, Internal Medicine

Chief complaint: Fatigue, irritability, difficulty sleeping, and new onset of abdominal pain with constipation

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Marcus Johnson, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue, irritability, difficulty sleeping, and new onset of abdominal pain with constipation

HISTORY OF PRESENT ILLNESS:
He returns for follow-up. He also reports a recent onset of abdominal pain and constipation.

PAST MEDICAL HISTORY:
Active Problem List:
- Lead poisoning (diagnosed 2020-01-15)
- Major depressive disorder

His past medical history is significant for depression.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
For which he takes sertraline.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder
- Mother: Major depressive disorder
- Paternal grandfather: History of moonshine production

SOCIAL HISTORY:
Smoking: Former smoker, quit approximately 5 years ago. Alcohol: 1-2 glasses of homemade whiskey daily, distilled using an old family kit inherited from his grandfather. Occupation: Independent construction contractor.

VITAL SIGNS:
His blood pressure is 130/85 mmHg, heart rate is 78 bpm, and temperature is 37.2°C (99.0°F).

PHYSICAL EXAMINATION:
On physical examination, he appears pale and has mild diffuse abdominal tenderness without rebound or guarding. Neurological examination reveals mild motor weakness in the upper extremities and diminished sensation in a glove-and-stocking distribution.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 10.2 g/dL (Normal: 13.8-17.2 g/dL)
- Hematocrit: 30.5% (Normal: 40.7-50.3%)
- Mean corpuscular volume (MCV): 79 fL (Normal: 80-96 fL)
- Blood lead level: Pending

ASSESSMENT AND PLAN:
Based on the patient's occupational history and clinical presentation,

## Encounter 2: source 2020-07-15, practice 2025-01-23, outpatient, Neurology

Chief complaint: Progressive fatigue, memory problems, and persistent abdominal discomfort

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Priya Patel, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue, memory problems, and persistent abdominal discomfort

HISTORY OF PRESENT ILLNESS:
The patient with major depressive disorder on sertraline returns for follow-up of symptoms seen in January and March. A 44-year-old man is brought to clinic by his wife due to progressive fatigue, memory problems, and abdominal discomfort, and has felt increasingly irritable with difficulty concentrating at work.

PAST MEDICAL HISTORY:
Active Problem List:
- Lead poisoning (diagnosed 2020-01-15)
- Major depressive disorder

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Sertraline 50 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder
- Mother: Major depressive disorder
- Paternal grandfather: History of moonshine production

SOCIAL HISTORY:
He works as a recreational hunter and frequently makes his own ammunition at home.

PHYSICAL EXAMINATION:
Examination shows pallor and mild tremor of the hands. Neurologic exam reveals decreased sensation in both feet.

LABORATORY DATA:
Labs reveal:
Hemoglobin 9.4 g/dL
MCV 72 fL
Ferritin 280 ng/mL
Iron 160 µg/dL
TIBC 180 µg/dL

OTHER STUDIES:
A peripheral smear is shown below.

## Encounter 3: source 2021-07-15, practice 2026-02-03, ed, Emergency Medicine

Chief complaint: Sudden-onset dizziness, nausea, perioral numbness, and muscle weakness after eating pufferfish

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden-onset dizziness, nausea, perioral numbness, and muscle weakness after eating pufferfish

HISTORY OF PRESENT ILLNESS:
The patient is a 40-year-old male with major depressive disorder on sertraline and a history of lead poisoning evaluated in 2020, who is brought to the emergency department after developing sudden-onset dizziness, nausea, and perioral numbness approximately 30 minutes after eating a traditional pufferfish (fugu) dish while visiting Japan. Shortly afterward, he developed muscle weakness, slurred speech, and difficulty walking. He denies any vomiting or diarrhea.

PAST MEDICAL HISTORY:
Active Problem List:
- Lead poisoning (diagnosed 2020-01-15)
- Major depressive disorder

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Sertraline 50 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder
- Mother: Major depressive disorder
- Paternal grandfather: History of moonshine production

SOCIAL HISTORY:
Smoking: Former smoker, quit approximately 5 years ago. Alcohol: 1-2 glasses of homemade whiskey daily, distilled using an old family kit inherited from his grandfather. Occupation: Independent construction contractor.

VITAL SIGNS:
On examination, his blood pressure is 110/70 mmHg, heart rate is 88/min, and respiratory rate is 22/min.

PHYSICAL EXAMINATION:
He is alert but appears anxious. Neurologic exam reveals symmetrical weakness in the upper and lower extremities and decreased sensation to light touch in a stocking-glove distribution. His deep tendon reflexes are present but diminished. Respiratory effort is mildly labored. Cardiac and pulmonary exams are otherwise unremarkable.
