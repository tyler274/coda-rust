# synthetic_hospital patient 2105

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex M, Non-Hispanic White, Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-12-18 (Mon) | outpatient | Gastroenterology |
| 1 | 2021-07-15 | 2025-06-05 (Thu) | ed | Emergency Medicine |
| 2 | 2022-07-15 | 2026-06-10 (Wed) | icu | Critical Care Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield PPO",
 "occupation": "Administrative Coordinator",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Hereditary spherocytosis (father)",
  "Hereditary spherocytosis (paternal aunt)",
  "Osteoarthritis (mother)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "400 mg every 6-8 hours as needed for joint pain",
   "name": "Ibuprofen"
  },
  {
   "dose": "1 mg daily",
   "name": "Folic acid"
  }
 ],
 "surgical_history": [
  "Laparoscopic cholecystectomy (age 34)"
 ],
 "chronic_conditions": [
  "Hereditary spherocytosis",
  "Chronic hemolytic anemia",
  "Osteoarthritis",
  "Cholelithiasis (s/p cholecystectomy)"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Helicobacter pylori-associated duodenal ulcer", "Acute hemolytic transfusion reaction"]
Comorbidities: ["Anemia", "Hereditary spherocytosis"]

## Encounter 0: source 2020-01-15, practice 2023-12-18, outpatient, Gastroenterology

Chief complaint: 2-month history of epigastric gnawing pain that occurs 2-3 hours after eating and wakes him at night, temporarily relieved by food intake

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
2-month history of epigastric gnawing pain that occurs 2-3 hours after eating and wakes him at night, temporarily relieved by food intake

HISTORY OF PRESENT ILLNESS:
He describes the pain as a gnawing sensation that occurs 2-3 hours after eating and often wakes him up at night. The pain temporarily improves with food intake. He denies any recent weight loss, vomiting, or changes in bowel habits.

PAST MEDICAL HISTORY:
Active Problem List:
- Hereditary spherocytosis
- Chronic hemolytic anemia
- Osteoarthritis
- Cholelithiasis (s/p cholecystectomy)

He has a past medical history of osteoarthritis.

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (age 34)

MEDICATIONS:
He takes for which he takes ibuprofen as needed.

Home Medications:
- Folic acid 1 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Hereditary spherocytosis (father)
- Hereditary spherocytosis (paternal aunt)
- Osteoarthritis (mother)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Administrative Coordinator.

VITAL SIGNS:
His vital signs are within normal limits.

PHYSICAL EXAMINATION:
Physical examination reveals mild tenderness to palpation in the epigastric region.

LABORATORY DATA:
Laboratory tests show:

- Hemoglobin: 13.2 g/dL (normal: 13.8-17.2 g/dL)
- Leukocyte count: 7,000/mm³ (normal: 4,500-11,000/mm³)
- Platelets: 250,000/mm³ (normal: 150,000-450,000/mm³)
- Serum urea breath test: Positive

OTHER STUDIES:
Upper endoscopy reveals a 1 cm ulcer in the first part of the duodenum. No signs of bleeding are noted.

## Encounter 1: source 2021-07-15, practice 2025-06-05, ed, Emergency Medicine

Chief complaint: Sudden onset of fever, severe lower back pain, and chest tightness 30 minutes after receiving a red blood cell transfusion

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of fever, severe lower back pain, and chest tightness 30 minutes after receiving a red blood cell transfusion

HISTORY OF PRESENT ILLNESS:
The patient with hereditary spherocytosis and history of duodenal ulcer presents to the emergency department 30 minutes after receiving a red blood cell transfusion.

PAST MEDICAL HISTORY:
Active Problem List:
- Helicobacter pylori-associated duodenal ulcer (diagnosed 2020-01-15)
- Hereditary spherocytosis
- Chronic hemolytic anemia
- Osteoarthritis
- Cholelithiasis (s/p cholecystectomy)

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (age 34)

MEDICATIONS:
Home Medications:
- Ibuprofen 400 mg every 6-8 hours as needed for joint pain
- Folic acid 1 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Hereditary spherocytosis (father)
- Hereditary spherocytosis (paternal aunt)
- Osteoarthritis (mother)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Administrative Coordinator.

VITAL SIGNS:
His temperature is 38.9°C (102°F), blood pressure is 90/60 mmHg, pulse is 120/min, and respiratory rate is 28/min.

PHYSICAL EXAMINATION:
Physical examination reveals jaundice and dark urine.

LABORATORY DATA:
His pre-transfusion hemoglobin was 7.8 g/dL, and his current laboratory values are as follows:

- Hemoglobin: 6.5 g/dL
- Lactate dehydrogenase (LDH): Elevated
- Haptoglobin: Decreased
- Direct Coombs test: Positive
- Serum bilirubin: Elevated
- Urinalysis: Hemoglobinuria, no red blood cells

## Encounter 2: source 2022-07-15, practice 2026-06-10, icu, Critical Care Medicine

Chief complaint: Fever, severe lower back pain, and dark urine approximately 1 hour after red blood cell transfusion

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Priya Sharma, MD
Department: Critical Care Medicine
Visit Type: icu

CHIEF COMPLAINT:
Fever, severe lower back pain, and dark urine approximately 1 hour after red blood cell transfusion

HISTORY OF PRESENT ILLNESS:
He has hereditary spherocytosis and 2021 transfusion reaction. He received a red blood cell transfusion approximately 1 hour ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Helicobacter pylori-associated duodenal ulcer (diagnosed 2020-01-15)
- Acute hemolytic transfusion reaction (diagnosed 2021-07-15)
- Hereditary spherocytosis
- Chronic hemolytic anemia
- Osteoarthritis
- Cholelithiasis (s/p cholecystectomy)

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (age 34)

MEDICATIONS:
Home Medications:
- Ibuprofen 400 mg every 6-8 hours as needed for joint pain
- Folic acid 1 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Hereditary spherocytosis (father)
- Hereditary spherocytosis (paternal aunt)
- Osteoarthritis (mother)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Administrative Coordinator.

VITAL SIGNS:
Temperature is 38.9°C (102°F), blood pressure is 90/60 mmHg, pulse is 120/min, and respirations are 22/min

PHYSICAL EXAMINATION:
Jaundice and tenderness over the bilateral flanks

LABORATORY DATA:
- Hemoglobin: 7 g/dL (decreased from 8 g/dL pre-transfusion)
- Total bilirubin: 3.2 mg/dL (elevated)
- Lactate dehydrogenase (LDH): 600 U/L (elevated)
- Haptoglobin: <10 mg/dL (decreased)
- Direct Coombs test: Positive
