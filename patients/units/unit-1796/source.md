# synthetic_hospital patient 1796

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex F, White, non-Hispanic, Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-07-30 (Wed) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2026-01-10 (Sat) | ed | Emergency Medicine |
| 2 | 2021-01-15 | 2026-07-20 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Blue Cross Blue Shield PPO",
 "occupation": "International Business Development Manager",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Mother: Hypothyroidism",
  "Father: Hypertension",
  "No family history of immunodeficiency or primary GI disorders"
 ],
 "race_ethnicity": "White, non-Hispanic",
 "smoking_status": "Never",
 "home_medications": [],
 "surgical_history": [
  "Tonsillectomy, age 12",
  "Wisdom teeth extraction, age 21"
 ],
 "chronic_conditions": [],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Cryptosporidiosis", "Dehydration", "Norovirus infection"]
Comorbidities: ["Dehydration"]

## Encounter 0: source 2020-01-15, practice 2025-07-30, outpatient, Internal Medicine

Chief complaint: Six-day history of profuse watery diarrhea with up to 8 bowel movements per day and mild abdominal cramping.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Six-day history of profuse watery diarrhea with up to 8 bowel movements per day and mild abdominal cramping.

HISTORY OF PRESENT ILLNESS:
She reports up to 8 bowel movements per day, often accompanied by mild cramping but no blood, mucus, or tenesmus. She denies vomiting or fever. Her appetite is decreased, and she feels fatigued but has maintained adequate oral intake.

PAST MEDICAL HISTORY:
Previously healthy; no history of chronic illness or immunodeficiency

PAST SURGICAL HISTORY:
- Tonsillectomy, age 12
- Wisdom teeth extraction, age 21

MEDICATIONS:
She is not taking any medications

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Hypothyroidism
- Father: Hypertension
- No family history of immunodeficiency or primary GI disorders

SOCIAL HISTORY:
She recently returned from a 10-day business trip to southern Mexico, where she attended outdoor events and consumed local street food and beverages.

PHYSICAL EXAMINATION:
Physical exam shows a tired-appearing woman with dry mucous membranes and mild diffuse abdominal tenderness.

LABORATORY DATA:
Laboratory studies reveal normal electrolytes and mild leukocytosis. Routine stool cultures are negative. Acid-fast staining of stool reveals round ovoid organisms.

## Encounter 1: source 2020-07-15, practice 2026-01-10, ed, Emergency Medicine

Chief complaint: Lightheadedness and dizziness during marathon race on a warm, humid day with dry mouth and skin.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Lightheadedness and dizziness during marathon race on a warm, humid day with dry mouth and skin.

HISTORY OF PRESENT ILLNESS:
This 32-year-old female with a history of cryptosporidiosis diagnosed six months prior presents to the Emergency Department with acute symptoms. She is participating in a marathon and is currently 10 miles into the race on a warm, humid day. She has not had anything to drink since starting, and she begins to feel lightheaded and dizzy. She slows her pace and rests for a few minutes, during which she notices her mouth feels dry and her skin feels sticky. She has not urinated since the race began.

PAST MEDICAL HISTORY:
Active Problem List:
- Cryptosporidiosis (diagnosed 2020-01-15)

PAST SURGICAL HISTORY:
- Tonsillectomy, age 12
- Wisdom teeth extraction, age 21

MEDICATIONS:
She is not taking any medications

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Hypothyroidism
- Father: Hypertension
- No family history of immunodeficiency or primary GI disorders

SOCIAL HISTORY:
Smoking: Never. Alcohol: Social (1-2 drinks per week). Occupation: International Business Development Manager.

REVIEW OF SYSTEMS:
She reports no recent illness, vomiting, or diarrhea.

VITAL SIGNS:
Vital signs obtained at a first aid tent show a heart rate of 104/min, blood pressure of 98/66 mm Hg

PHYSICAL EXAMINATION:
Dry mucous membranes

OTHER STUDIES:
Her body weight today is 3 pounds lower than at her pre-race weigh-in.

ASSESSMENT AND PLAN:
The medical staff suspects volume depletion

Sends her for evaluation.

## Encounter 2: source 2021-01-15, practice 2026-07-20, ed, Emergency Medicine

Chief complaint: Profuse vomiting and watery diarrhea for the past 24 hours while on cruise vacation, unable to tolerate oral intake.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Profuse vomiting and watery diarrhea for the past 24 hours while on cruise vacation, unable to tolerate oral intake.

HISTORY OF PRESENT ILLNESS:
The patient presents to the Emergency Department approximately one year following an outpatient evaluation for profuse watery diarrhea and six months following a prior visit for lightheadedness and dizziness during exertion in warm, humid conditions with dry mouth and skin. She is currently on a cruise vacation and reports that dozens of other passengers have developed similar symptoms. Her illness began abruptly with intense nausea, followed by multiple episodes of non-bloody, watery diarrhea and vomiting. She has been unable to keep food or fluids down. Her husband, who shared all meals with her, began having identical symptoms a few hours earlier. They deny eating raw seafood, undercooked meat, or unpasteurized dairy. No one in their travel group has a fever or bloody stools.

PAST MEDICAL HISTORY:
Active Problem List:
- Cryptosporidiosis (diagnosed 2020-01-15)
- Dehydration (diagnosed 2020-07-15)

PAST SURGICAL HISTORY:
- Tonsillectomy, age 12
- Wisdom teeth extraction, age 21

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Hypothyroidism
- Father: Hypertension
- No family history of immunodeficiency or primary GI disorders

SOCIAL HISTORY:
Smoking: Never. Alcohol: Social (1-2 drinks per week). Occupation: International Business Development Manager.

VITAL SIGNS:
Vital signs are stable, with mild tachycardia and dry mucous membranes.

PHYSICAL EXAMINATION:
On examination, she appears fatigued but is alert and oriented.

LABORATORY DATA:
A stool test is negative for leukocytes and ova/parasites.

ASSESSMENT AND PLAN:
The infection is attributed to the most likely viral cause of outbreaks in crowded settings like cruise ships, nursing homes, and military barracks.
