# synthetic_hospital patient 2804

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 46 at first encounter, sex F, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-06-13 (Fri) | ed | Emergency Medicine |
| 1 | 2020-09-15 | 2026-02-13 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Office administrator",
 "alcohol_use": "None",
 "family_history": [
  "Mother with diverticulosis",
  "Sister with deep vein thrombosis",
  "Father with type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "17 g PO daily",
   "name": "Polyethylene glycol 3350"
  },
  {
   "dose": "3.4 g PO daily",
   "name": "Psyllium husk"
  },
  {
   "dose": "100 mg PO twice daily",
   "name": "Docusate sodium"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Diverticulosis",
  "Chronic constipation",
  "History of necrotizing pancreatitis"
 ],
 "age_at_first_encounter": 46
}
```

Primary diagnoses: ["Diverticulitis", "Portal vein thrombosis"]
Comorbidities: ["Upper Gastrointestinal Hemorrhage"]

## Encounter 0: source 2020-01-15, practice 2025-06-13, ed, Emergency Medicine

Chief complaint: 2-day history of left lower quadrant abdominal pain with low-grade fever and decreased appetite

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
2-day history of left lower quadrant abdominal pain with low-grade fever and decreased appetite

HISTORY OF PRESENT ILLNESS:
The pain is constant, sharp, and worsened with movement. She reports a low-grade fever and a decrease in appetite. She denies any recent travel, sick contacts, or similar episodes in the past.

PAST MEDICAL HISTORY:
Active Problem List:
- Diverticulosis
- Chronic constipation
- History of necrotizing pancreatitis

She has a history of constipation and occasional bloody stools.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Polyethylene glycol 3350 17 g PO daily
- Psyllium husk 3.4 g PO daily
- Docusate sodium 100 mg PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with diverticulosis
- Sister with deep vein thrombosis
- Father with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Office administrator.

VITAL SIGNS:
Her vital signs are: temperature 38.2°C, blood pressure 130/85 mmHg, heart rate 92 bpm, and respiratory rate 18 breaths/min.

PHYSICAL EXAMINATION:
Physical examination reveals localized tenderness in the left lower quadrant with voluntary guarding but no rebound tenderness. Bowel sounds are normal.

LABORATORY DATA:
Laboratory tests show:

- White blood cell count: 14,000 /µL
- Hemoglobin: 13.2 g/dL
- Platelets: 250,000 /µL
- C-reactive protein: 25 mg/L (normal <5 mg/L)

## Encounter 1: source 2020-09-15, practice 2026-02-13, ed, Emergency Medicine

Chief complaint: Hematemesis and black stools with associated lightheadedness and fatigue

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. James Okafor, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Hematemesis and black stools with associated lightheadedness and fatigue

HISTORY OF PRESENT ILLNESS:
The patient with diverticulosis and necrotizing pancreatitis returns eight months after her diverticulitis ED visit. She reports black stools over the past two days and has now developed lightheadedness and fatigue.

PAST MEDICAL HISTORY:
Active Problem List:
- Diverticulitis (diagnosed 2020-01-15)
- Diverticulosis
- Chronic constipation
- History of necrotizing pancreatitis

Her medical history includes an episode of necrotizing pancreatitis six months ago, which required hospitalization but resolved without surgical intervention.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Polyethylene glycol 3350 17 g PO daily
- Psyllium husk 3.4 g PO daily
- Docusate sodium 100 mg PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with diverticulosis
- Sister with deep vein thrombosis
- Father with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Office administrator.

VITAL SIGNS:
Blood pressure is 98/60 mmHg, heart rate is 108/min, respiratory rate is 16/min, and temperature is 37.1°C.

PHYSICAL EXAMINATION:
On examination, she is pale and appears fatigued. Abdominal examination reveals mild tenderness in the left upper quadrant without guarding or rebound.

OTHER STUDIES:
Esophagogastroduodenoscopy reveals isolated gastric varices and active bleeding is noted from the fundus.
