# synthetic_hospital patient 2548

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex M, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-11-08 (Fri) | ed | Emergency Medicine |
| 1 | 2020-07-15 | 2025-05-27 (Tue) | ed | Emergency Medicine |
| 2 | 2021-01-15 | 2025-11-13 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Methimazole (agranulocytosis)",
  "Sulfonamides (anaphylaxis)"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Freelance music producer",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Bipolar disorder in mother",
  "Graves disease in maternal aunt",
  "Autoimmune thyroiditis in sister"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never",
 "home_medications": [
  {
   "dose": "300 mg PO BID",
   "name": "Lithium carbonate"
  },
  {
   "dose": "125 mcg PO daily",
   "name": "Levothyroxine"
  },
  {
   "dose": "20 mg PO PRN tremor/anxiety",
   "name": "Propranolol"
  }
 ],
 "surgical_history": [
  "Total thyroidectomy (age 32, following methimazole-induced agranulocytosis)"
 ],
 "chronic_conditions": [
  "Bipolar I disorder",
  "Graves disease (hyperthyroidism)"
 ],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Bipolar disorder, current episode manic", "Agranulocytosis"]
Comorbidities: ["Sinus tachycardia secondary to burns"]

## Encounter 0: source 2020-01-15, practice 2024-11-08, ed, Emergency Medicine

Chief complaint: Fever, sore throat, and 2-month history of palpitations, unintentional weight loss, and heat intolerance

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fever, sore throat, and 2-month history of palpitations, unintentional weight loss, and heat intolerance

HISTORY OF PRESENT ILLNESS:
He also reports increased anxiety and difficulty sleeping. He denies chest pain, drug use, or recent travel.

PAST MEDICAL HISTORY:
Active Problem List:
- Bipolar I disorder
- Graves disease (hyperthyroidism)

PAST SURGICAL HISTORY:
- Total thyroidectomy (age 32, following methimazole-induced agranulocytosis)

MEDICATIONS:
Home Medications:
- Lithium carbonate 300 mg PO BID
- Levothyroxine 125 mcg PO daily
- Propranolol 20 mg PO PRN tremor/anxiety

ALLERGIES:
Methimazole (agranulocytosis), Sulfonamides (anaphylaxis)

FAMILY HISTORY:
- Bipolar disorder in mother
- Graves disease in maternal aunt
- Autoimmune thyroiditis in sister

SOCIAL HISTORY:
Smoking: Never. Alcohol: Social (1-2 drinks per week). Occupation: Freelance music producer.

VITAL SIGNS:
Vital signs are: temperature 37.3°C (99.1°F), blood pressure 124/72 mm Hg, pulse 102/min, and respiratory rate 18/min.

PHYSICAL EXAMINATION:
Physical exam reveals moist skin, fine tremor of the hands, and a diffusely enlarged, non-tender thyroid. There is no thyroid nodule, bruit, or ophthalmopathy.

LABORATORY DATA:
Thyroid function tests reveal a suppressed TSH and elevated free T4.

PLAN:
The patient is started on methimazole therapy.

## Encounter 1: source 2020-07-15, practice 2025-05-27, ed, Emergency Medicine

Chief complaint: Severe headache with increasing irritability and decreased sleep over the past week

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe headache with increasing irritability and decreased sleep over the past week

HISTORY OF PRESENT ILLNESS:
This 32-year-old male with Bipolar I disorder and Graves disease, previously evaluated in this emergency department six months ago for agranulocytosis, presents to the emergency department (ED). He is accompanied by his partner, who reports that he has been increasingly irritable and has had difficulty sleeping over the past week.

PAST MEDICAL HISTORY:
Active Problem List:
- Agranulocytosis (diagnosed 2020-01-15)
- Bipolar I disorder
- Graves disease (hyperthyroidism)

The patient has a history of bipolar disorder.

PAST SURGICAL HISTORY:
- Total thyroidectomy (age 32, following methimazole-induced agranulocytosis)

MEDICATIONS:
He is currently on lithium.

Home Medications:
- Lithium carbonate 300 mg PO BID
- Levothyroxine 125 mcg PO daily
- Propranolol 20 mg PO PRN tremor/anxiety

ALLERGIES:
Methimazole (agranulocytosis), Sulfonamides (anaphylaxis)

FAMILY HISTORY:
- Bipolar disorder in mother
- Graves disease in maternal aunt
- Autoimmune thyroiditis in sister

SOCIAL HISTORY:
Smoking: Never. Alcohol: Social (1-2 drinks per week). Occupation: Freelance music producer.

VITAL SIGNS:
His vital signs are as follows: temperature 37.1°C, blood pressure 145/90 mmHg, heart rate 98 bpm, and respiratory rate 18 breaths/min.

PHYSICAL EXAMINATION:
On examination, the patient is pacing, has a loud voice, and is using profanity. His fists are clenched, and he occasionally pounds his fist into his other hand.
The neurological examination is nonfocal.

LABORATORY DATA:
The patient's lithium level is within the therapeutic range.

## Encounter 2: source 2021-01-15, practice 2025-11-13, ed, Emergency Medicine

Chief complaint: Several days of erratic behavior, pressured speech, and reckless financial spending

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Several days of erratic behavior, pressured speech, and reckless financial spending

HISTORY OF PRESENT ILLNESS:
The patient is a 33-year-old male with Bipolar I disorder and Graves disease who returns to the emergency department. He previously presented six months prior with severe headache, irritability, and decreased sleep, and one year prior with fever, sore throat, and palpitations, and now presents with several days of erratic behavior. Over the past week, the patient has barely slept but claims he feels more energized than ever. He has been talking rapidly, jumping between topics, and insisting that he is “finally seeing the world clearly.” According to the roommate, he recently drained his savings to buy studio equipment, announcing that he was launching a new podcast that would “change global discourse.” He also began filming himself on social media proclaiming spiritual revelations, and reached out to multiple high school classmates with messages about “building a new utopia.” The roommate says this is not the first time something like this has happened. About a year ago, the patient had a similar episode but refused to seek medical help. He also recalls an extended period a few months ago when the patient isolated himself in his room for nearly three weeks, barely ate, and frequently cried. At the time, he said he “just didn’t see the point of anything.”

PAST MEDICAL HISTORY:
Active Problem List:
- Agranulocytosis (diagnosed 2020-01-15)
- Bipolar disorder, current episode manic (diagnosed 2020-07-15)
- Bipolar I disorder
- Graves disease (hyperthyroidism)

PAST SURGICAL HISTORY:
- Total thyroidectomy (age 32, following methimazole-induced agranulocytosis)

MEDICATIONS:
Is.

Home Medications:
- Lithium carbonate 300 mg PO BID
- Levothyroxine 125 mcg PO daily
- Propranolol 20 mg PO PRN tremor/anxiety

ALLERGIES:
Methimazole (agranulocytosis), Sulfonamides (anaphylaxis)

FAMILY HISTORY:
Family history is notable for a mother with “mood swings” and a maternal uncle who died by suicide.

SOCIAL HISTORY:
There is no history of alcohol or drug use.
The patient recently quit his job impulsively and says he’s “meant for greater things.”

VITAL SIGNS:
Vitals and labs are within normal limits.

PHYSICAL EXAMINATION:
On exam, the patient is alert, euphoric, and very talkative. He paces around the room, displays pressured speech, and frequently interrupts. He becomes frustrated when asked clarifying questions but does not appear paranoid or hallucinating. Neurologic exam is unremarkable, and there are no signs of intoxication.

LABORATORY DATA:
A urine drug screen is negative.

PLAN:
He agrees to inpatient psychiatric admission and is stabilized with a low-dose antipsychotic.
