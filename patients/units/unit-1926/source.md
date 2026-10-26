# synthetic_hospital patient 1926

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 24 at first encounter, sex F, Non-Hispanic White, Parental employer-based PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-06-17 (Mon) | outpatient | Internal Medicine |
| 1 | 2021-07-15 | 2025-12-16 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Parental employer-based PPO",
 "occupation": "Graduate student",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Mother: generalized anxiety disorder",
  "Father: migraine disorder",
  "Maternal grandmother: irritable bowel syndrome"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "50 mg by mouth daily",
   "name": "Sertraline"
  },
  {
   "dose": "0.25 mg by mouth as needed for anxiety (max 2 tablets daily)",
   "name": "Alprazolam"
  },
  {
   "dose": "50 mg by mouth as needed for migraine headache",
   "name": "Sumatriptan"
  },
  {
   "dose": "500 mg by mouth as needed for nausea or abdominal cramping",
   "name": "Ginger extract"
  }
 ],
 "surgical_history": [
  "Wisdom teeth extraction, age 18"
 ],
 "chronic_conditions": [
  "Generalized anxiety disorder",
  "Migraine without aura",
  "Irritable bowel syndrome with diarrhea (IBS-D)"
 ],
 "age_at_first_encounter": 24
}
```

Primary diagnoses: ["Benzodiazepine overdose", "Irritable Bowel Syndrome with Diarrhea"]
Comorbidities: ["Generalized Anxiety Disorder"]

## Encounter 0: source 2020-01-15, practice 2024-06-17, outpatient, Internal Medicine

Chief complaint: intermittent crampy abdominal pain and loose stools

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Patel, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent crampy abdominal pain and loose stools

HISTORY OF PRESENT ILLNESS:
6-month history of intermittent crampy abdominal pain and loose stools. She reports having 3–4 bowel movements daily, especially during periods of stress, and describes the pain as diffuse and colicky, improving after defecation. She denies weight loss, bloody or nocturnal stools, recent antibiotic use, or use of over-the-counter laxatives.

PAST MEDICAL HISTORY:
Active Problem List:
- Generalized anxiety disorder
- Migraine without aura
- Irritable bowel syndrome with diarrhea (IBS-D)

Her medical history includes anxiety and migraines.

PAST SURGICAL HISTORY:
- Wisdom teeth extraction, age 18

MEDICATIONS:
She takes sertraline and over-the-counter ginger extract.

Home Medications:
- Alprazolam 0.25 mg by mouth as needed for anxiety (max 2 tablets daily)
- Sumatriptan 50 mg by mouth as needed for migraine headache

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: generalized anxiety disorder
- Father: migraine disorder
- Maternal grandmother: irritable bowel syndrome

SOCIAL HISTORY:
She drinks socially, does not smoke, and has no recent travel history.

VITAL SIGNS:
Vitals are within normal limits

PHYSICAL EXAMINATION:
Physical exam is unremarkable.

LABORATORY DATA:
Laboratory testing, including complete blood count, C-reactive protein, and tissue transglutaminase IgA, is normal.

## Encounter 1: source 2021-07-15, practice 2025-12-16, ed, Emergency Medicine

Chief complaint: Difficult to arouse

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Difficult to arouse

HISTORY OF PRESENT ILLNESS:
This 25-year-old female with generalized anxiety disorder, migraine, and IBS-D, seen in January 2020 for abdominal pain, is brought to the emergency department by her roommate, who found her difficult to arouse in bed. The roommate reports the patient may have taken "a handful of pills" after a recent emotional breakup.

PAST MEDICAL HISTORY:
Active Problem List:
- Irritable Bowel Syndrome with Diarrhea (diagnosed 2020-01-15)
- Generalized anxiety disorder
- Migraine without aura
- Irritable bowel syndrome with diarrhea (IBS-D)

The roommate reports the patient has a history of anxiety.

PAST SURGICAL HISTORY:
- Wisdom teeth extraction, age 18

MEDICATIONS:
Home Medications:
- Sertraline 50 mg by mouth daily
- Alprazolam 0.25 mg by mouth as needed for anxiety (max 2 tablets daily)
- Sumatriptan 50 mg by mouth as needed for migraine headache
- Ginger extract 500 mg by mouth as needed for nausea or abdominal cramping

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: generalized anxiety disorder
- Father: migraine disorder
- Maternal grandmother: irritable bowel syndrome

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Graduate student.

VITAL SIGNS:
Blood pressure 106/64 mm Hg, heart rate 56/min, respiratory rate 10/min, and oxygen saturation 93% on room air

PHYSICAL EXAMINATION:
On arrival, the patient is somnolent but arousable to voice. Physical examination shows slurred speech and ataxia. Her pupils are 3 mm and reactive.

OTHER STUDIES:
Naloxone administration has no effect. However, the patient becomes more alert within minutes of receiving flumazenil.
