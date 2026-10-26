# synthetic_hospital patient 2551

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 29 at first encounter, sex M, Non-Hispanic White, Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-01-15 (Wed) | icu | Trauma Surgery |
| 1 | 2020-03-15 | 2025-03-14 (Fri) | ed | Emergency Medicine |
| 2 | 2021-03-15 | 2026-03-12 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield PPO",
 "occupation": "Public Health Research Coordinator",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Maternal cousin with Type 1 Diabetes Mellitus",
  "Paternal aunt with Celiac disease",
  "Father with Hashimoto thyroiditis"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "22 units subcutaneous at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "Sliding scale with meals per carbohydrate counting ratio (1:12)",
   "name": "Insulin lispro"
  }
 ],
 "surgical_history": [
  "Exploratory laparotomy with splenectomy and damage control laparotomy for intra-abdominal hemorrhage secondary to motor vehicle collision",
  "Temporary abdominal closure with subsequent fascial closure"
 ],
 "chronic_conditions": [
  "Type 1 Diabetes Mellitus (diagnosed age 29 following motor vehicle collision)",
  "Status post splenectomy"
 ],
 "age_at_first_encounter": 29
}
```

Primary diagnoses: ["Diabetic ketoacidosis", "Traumatic hemorrhagic shock with acute coagulopathy", "Cholera"]
Comorbidities: ["Lactic acidosis"]

## Encounter 0: source 2020-01-15, practice 2025-01-15, icu, Trauma Surgery

Chief complaint: Severe abdominal pain following high-speed motor vehicle collision with persistent hypotension despite fluid resuscitation

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Trauma Surgery
Visit Type: icu

CHIEF COMPLAINT:
Severe abdominal pain following high-speed motor vehicle collision with persistent hypotension despite fluid resuscitation

HISTORY OF PRESENT ILLNESS:
He is brought to the emergency department after being involved in a high-speed motor vehicle collision. He is alert but appears anxious and is complaining of severe abdominal pain. The patient's blood pressure drops to 85/55 mmHg despite initial fluid resuscitation with 1 liter of 0.9% saline.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 1 Diabetes Mellitus (diagnosed age 29 following motor vehicle collision)
- Status post splenectomy

PAST SURGICAL HISTORY:
- Exploratory laparotomy with splenectomy and damage control laparotomy for intra-abdominal hemorrhage secondary to motor vehicle collision
- Temporary abdominal closure with subsequent fascial closure

MEDICATIONS:
Home Medications:
- Insulin glargine 22 units subcutaneous at bedtime
- Insulin lispro Sliding scale with meals per carbohydrate counting ratio (1:12)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal cousin with Type 1 Diabetes Mellitus
- Paternal aunt with Celiac disease
- Father with Hashimoto thyroiditis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Public Health Research Coordinator.

VITAL SIGNS:
On examination, his heart rate is 120 beats per minute, blood pressure is 90/60 mmHg, respiratory rate is 22 breaths per minute, and his temperature is 36.2°C.

PHYSICAL EXAMINATION:
His abdomen is distended, with guarding and rebound tenderness noted diffusely.

LABORATORY DATA:
Initial laboratory tests reveal:

- Hemoglobin: 10 g/dL (normal: 13.8-17.2 g/dL)
- Platelet count: 250,000/mm³ (normal: 150,000-450,000/mm³)
- Prothrombin time (PT): 16 seconds (normal: 11-13.5 seconds)
- Partial thromboplastin time (PTT): 40 seconds (normal: 25-35 seconds)
- Lactate: 4 mmol/L (normal: 0.5-1 mmol/L)

IMAGING:
FAST (Focused Assessment with Sonography for Trauma) ultrasound shows free fluid in the abdomen.

## Encounter 1: source 2020-03-15, practice 2025-03-14, ed, Emergency Medicine

Chief complaint: Two days of nausea, polyuria, and increasing lethargy

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two days of nausea, polyuria, and increasing lethargy

PAST MEDICAL HISTORY:
Active Problem List:
- Traumatic hemorrhagic shock with acute coagulopathy (diagnosed 2020-01-15)
- Type 1 Diabetes Mellitus (diagnosed age 29 following motor vehicle collision)
- Status post splenectomy

PAST SURGICAL HISTORY:
- Exploratory laparotomy with splenectomy and damage control laparotomy for intra-abdominal hemorrhage secondary to motor vehicle collision
- Temporary abdominal closure with subsequent fascial closure

MEDICATIONS:
Home Medications:
- Insulin glargine 22 units subcutaneous at bedtime
- Insulin lispro Sliding scale with meals per carbohydrate counting ratio (1:12)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal cousin with Type 1 Diabetes Mellitus
- Paternal aunt with Celiac disease
- Father with Hashimoto thyroiditis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Public Health Research Coordinator.

PHYSICAL EXAMINATION:
On examination, he appears dehydrated with deep, rapid breathing.

LABORATORY DATA:
Laboratory studies reveal: glucose 505 mg/dL, bicarbonate 14 mEq/L, pH 7.21, and elevated serum ketones.

ASSESSMENT AND PLAN:
As part of the adaptive response to starvation and low insulin, increased lipolysis releases fatty acids and glycerol into circulation. Fatty acids undergo β-oxidation to form ketones, while glycerol is directed toward gluconeogenesis. In hepatocytes,

## Encounter 2: source 2021-03-15, practice 2026-03-12, ed, Emergency Medicine

Chief complaint: Worsening fatigue and dizziness with abrupt-onset profuse watery diarrhea three days after returning from humanitarian trip to rural coastal Guatemala

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Priya Sharma, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening fatigue and dizziness with abrupt-onset profuse watery diarrhea three days after returning from humanitarian trip to rural coastal Guatemala

HISTORY OF PRESENT ILLNESS:
The patient is a 30-year-old male with Type 1 Diabetes Mellitus diagnosed at age 29 following motor vehicle collision and status post splenectomy, with history of prior diabetic ketoacidosis, who presents with acute gastrointestinal symptoms. He returned from a humanitarian trip to rural coastal Guatemala 3 days ago. On his final day there, he developed abrupt-onset watery diarrhea and has since had near-constant bowel movements with large stool volumes. He denies blood in the stool, fever, or abdominal pain, but states that he has become too weak to eat or stand for long periods. He is unable to keep up with drinking fluids due to the volume of his diarrhea. He took no antibiotics or vaccines prior to travel.

PAST MEDICAL HISTORY:
Active Problem List:
- Traumatic hemorrhagic shock with acute coagulopathy (diagnosed 2020-01-15)
- Diabetic ketoacidosis (diagnosed 2020-03-15)
- Type 1 Diabetes Mellitus (diagnosed age 29 following motor vehicle collision)
- Status post splenectomy

PAST SURGICAL HISTORY:
- Exploratory laparotomy with splenectomy and damage control laparotomy for intra-abdominal hemorrhage secondary to motor vehicle collision
- Temporary abdominal closure with subsequent fascial closure

MEDICATIONS:
Home Medications:
- Insulin glargine 22 units subcutaneous at bedtime
- Insulin lispro Sliding scale with meals per carbohydrate counting ratio (1:12)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal cousin with Type 1 Diabetes Mellitus
- Paternal aunt with Celiac disease
- Father with Hashimoto thyroiditis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Public Health Research Coordinator.

VITAL SIGNS:
Temperature is 36.8°C (98.2°F), blood pressure is 86/52 mm Hg, and pulse is 116/min.

PHYSICAL EXAMINATION:
On examination, he appears fatigued and dry. He is alert but speaks slowly. Mucous membranes are dry, his eyes appear sunken, and his capillary refill is delayed. Bowel sounds are hyperactive.

LABORATORY DATA:
Lab testing reveals sodium 132 mEq/L, potassium 2.7 mEq/L, bicarbonate 12 mEq/L, and creatinine 2.4 mg/dL.
