# synthetic_hospital patient 1860

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 35 at first encounter, sex F, African American, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-01-17 (Mon) | outpatient | Obstetrics & Gynecology |
| 1 | 2020-04-15 | 2022-04-04 (Mon) | outpatient | Internal Medicine |
| 2 | 2020-07-15 | 2022-07-13 (Wed) | outpatient | Internal Medicine |
| 3 | 2022-07-15 | 2024-07-04 (Thu) | outpatient | Pulmonology |
| 4 | 2024-01-15 | 2026-01-09 (Fri) | outpatient | Internal Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Administrative assistant",
 "alcohol_use": "None",
 "family_history": [
  "Uterine leiomyomas in mother",
  "Restless legs syndrome in mother",
  "Pulmonary arterial hypertension in maternal aunt"
 ],
 "race_ethnicity": "African American",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "325 mg by mouth daily",
   "name": "Ferrous sulfate"
  },
  {
   "dose": "0.5 mg by mouth at bedtime",
   "name": "Ropinirole"
  },
  {
   "dose": "40 mg by mouth daily",
   "name": "Tadalafil"
  }
 ],
 "surgical_history": [
  "Vaginal delivery x1"
 ],
 "chronic_conditions": [
  "Submucosal uterine leiomyoma",
  "Iron deficiency anemia",
  "Pulmonary arterial hypertension",
  "Restless legs syndrome"
 ],
 "age_at_first_encounter": 35
}
```

Primary diagnoses: ["Restless Legs Syndrome", "Iron deficiency anemia", "Submucosal uterine leiomyoma", "Assessment of decision-making capacity", "Pulmonary Hypertension"]
Comorbidities: ["Menorrhagia", "Fatigue"]

## Encounter 0: source 2020-01-15, practice 2022-01-17, outpatient, Obstetrics & Gynecology

Chief complaint: 6-month history of increasingly heavy and prolonged menstrual bleeding with severe cramping

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Maria Santos, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
6-month history of increasingly heavy and prolonged menstrual bleeding with severe cramping

HISTORY OF PRESENT ILLNESS:
She reports that her menstrual periods now last for 10-12 days and she often passes large clots. She also experiences severe cramping during her periods, which she describes as similar to labor pains.

PAST MEDICAL HISTORY:
Active Problem List:
- Submucosal uterine leiomyoma
- Iron deficiency anemia
- Pulmonary arterial hypertension
- Restless legs syndrome

PAST SURGICAL HISTORY:
- Vaginal delivery x1

MEDICATIONS:
Home Medications:
- Ferrous sulfate 325 mg by mouth daily
- Ropinirole 0.5 mg by mouth at bedtime
- Tadalafil 40 mg by mouth daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Uterine leiomyomas in mother
- Restless legs syndrome in mother
- Pulmonary arterial hypertension in maternal aunt

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative assistant.

PHYSICAL EXAMINATION:
On physical examination, her abdomen is soft and non-tender. A pelvic examination reveals a non-tender, enlarged uterus with a smooth contour. A firm, irregular mass is palpable at the cervix.

LABORATORY DATA:
Her complete blood count shows:

- Hemoglobin: 10.2 g/dL (Normal: 12-16 g/dL)
- Hematocrit: 30.5% (Normal: 36-46%)
- Mean corpuscular volume (MCV): 82 fL (Normal: 80-100 fL)
- Platelets: 250,000/mm³ (Normal: 150,000-400,000/mm³)

IMAGING:
Transvaginal ultrasound shows a 3 cm intracavitary mass with a stalk attached to the anterior uterine wall. The endometrial lining is thickened and heterogeneous.

## Encounter 1: source 2020-04-15, practice 2022-04-04, outpatient, Internal Medicine

Chief complaint: Three-month history of fatigue and shortness of breath on exertion

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. David Kim, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Three-month history of fatigue and shortness of breath on exertion

HISTORY OF PRESENT ILLNESS:
She presents three months after submucosal uterine leiomyoma diagnosis, reporting fatigue and shortness of breath on exertion for three months. She reports a strong craving for chewing ice cubes and occasionally consumes small pieces of paper.

PAST MEDICAL HISTORY:
Active Problem List:
- Submucosal uterine leiomyoma (diagnosed 2020-01-15)
- Iron deficiency anemia
- Pulmonary arterial hypertension
- Restless legs syndrome

She has a past medical history of menorrhagia.

PAST SURGICAL HISTORY:
- Vaginal delivery x1

MEDICATIONS:
Home Medications:
- Ferrous sulfate 325 mg by mouth daily
- Ropinirole 0.5 mg by mouth at bedtime
- Tadalafil 40 mg by mouth daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Uterine leiomyomas in mother
- Restless legs syndrome in mother
- Pulmonary arterial hypertension in maternal aunt

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative assistant.

PHYSICAL EXAMINATION:
On physical examination, her skin and conjunctivae appear pale. Her nails show koilonychia.

LABORATORY DATA:
Laboratory values are as follows:

- Hemoglobin: 9.2 g/dL (Normal: 12-16 g/dL)
- Mean corpuscular volume (MCV): 70 fL (Normal: 80-100 fL)
- Serum iron: 30 µg/dL (Normal: 50-170 µg/dL)
- Total iron-binding capacity (TIBC): 600 µg/dL (Normal: 250-450 µg/dL)
- Ferritin: 8 ng/mL (Normal: 15-200 ng/mL)

## Encounter 2: source 2020-07-15, practice 2022-07-13, outpatient, Internal Medicine

Chief complaint: Poor sleep for the past 3 months due to urge to move legs and buzzing sensation in calves

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Lisa Thompson, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Poor sleep for the past 3 months due to urge to move legs and buzzing sensation in calves

HISTORY OF PRESENT ILLNESS:
The patient with known submucosal uterine leiomyoma and recently diagnosed iron deficiency anemia returns for evaluation of persistent sleep disturbances. She describes an overwhelming urge to move her legs when lying down at night, accompanied by an uncomfortable “buzzing” sensation deep in her calves. These symptoms are relieved by movement but return shortly after lying still. She reports needing to get out of bed multiple times each night to walk around in order to relieve the discomfort, resulting in fatigue the next day. She has no similar symptoms during the day. She follows a vegetarian diet and notes that her periods last 6–7 days and are heavy.

PAST MEDICAL HISTORY:
Active Problem List:
- Submucosal uterine leiomyoma (diagnosed 2020-01-15)
- Iron deficiency anemia (diagnosed 2020-04-15)
- Pulmonary arterial hypertension
- Restless legs syndrome

PAST SURGICAL HISTORY:
- Vaginal delivery x1

MEDICATIONS:
She does not take any medications.

Home Medications:
- Ferrous sulfate 325 mg by mouth daily
- Ropinirole 0.5 mg by mouth at bedtime
- Tadalafil 40 mg by mouth daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Uterine leiomyomas in mother
- Restless legs syndrome in mother
- Pulmonary arterial hypertension in maternal aunt

SOCIAL HISTORY:
She denies alcohol or drug use

REVIEW OF SYSTEMS:
She does not snore or have witnessed apneas.

PHYSICAL EXAMINATION:
Neurologic exam is normal, and there are no signs of peripheral edema or venous disease.

ASSESSMENT AND PLAN:
Based on clinical presentation, she is diagnosed with restless legs syndrome.

The physician is considering initiating treatment.

## Encounter 3: source 2022-07-15, practice 2024-07-04, outpatient, Pulmonology

Chief complaint: Progressive shortness of breath and fatigue over the past 5 months

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Robert Chen, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive shortness of breath and fatigue over the past 5 months

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up with history of submucosal uterine leiomyoma, iron deficiency anemia, restless legs syndrome. She presents with progressive shortness of breath and fatigue over past 5 months, reporting difficulty walking more than a block without stopping and occasional lightheadedness with exertion. She denies chest pain or leg swelling.

PAST MEDICAL HISTORY:
Active Problem List:
- Submucosal uterine leiomyoma (diagnosed 2020-01-15)
- Iron deficiency anemia (diagnosed 2020-04-15)
- Restless Legs Syndrome (diagnosed 2020-07-15)
- Pulmonary arterial hypertension

PAST SURGICAL HISTORY:
- Vaginal delivery x1

MEDICATIONS:
Home Medications:
- Ferrous sulfate 325 mg by mouth daily
- Ropinirole 0.5 mg by mouth at bedtime
- Tadalafil 40 mg by mouth daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Uterine leiomyomas in mother
- Restless legs syndrome in mother
- Pulmonary arterial hypertension in maternal aunt

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative assistant.

VITAL SIGNS:
Her vital signs are within normal limits.

PHYSICAL EXAMINATION:
Physical examination reveals clear lungs, elevated jugular venous pressure, a right-sided S3 gallop, and a loud P2.

IMAGING:
Chest x-ray shows prominent pulmonary arteries.

OTHER STUDIES:
Transthoracic echocardiography reveals a dilated right ventricle with reduced systolic function and preserved left ventricular size and function. Right heart catheterization demonstrates a mean pulmonary artery pressure of 45 mm Hg, a pulmonary capillary wedge pressure of 10 mm Hg, and reduced cardiac output.

## Encounter 4: source 2024-01-15, practice 2026-01-09, outpatient, Internal Medicine

Chief complaint: Routine wellness visit with reports of constant fatigue and sedentary lifestyle

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Angela Washington, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine wellness visit with reports of constant fatigue and sedentary lifestyle

HISTORY OF PRESENT ILLNESS:
The patient returns for routine wellness follow-up in the context of longitudinal management for submucosal uterine leiomyoma, iron deficiency anemia, restless legs syndrome, and pulmonary arterial hypertension initially evaluated across encounters from 2020 to 2022. She reports being "constantly tired" and not exercising as much as she used to. She works a sedentary office job and says she "misses feeling fit" but hasn't been physically active in over a year. When asked about starting an exercise routine, she says, "I really should, I know it would help me feel better. But I don't know if I have the time or energy to do it yet." She asks if there are simple ways to get started but doesn't commit to any plan.

PAST MEDICAL HISTORY:
Active Problem List:
- Submucosal uterine leiomyoma (diagnosed 2020-01-15)
- Iron deficiency anemia (diagnosed 2020-04-15)
- Restless Legs Syndrome (diagnosed 2020-07-15)
- Pulmonary Hypertension (diagnosed 2022-07-15)
- Pulmonary arterial hypertension

PAST SURGICAL HISTORY:
- Vaginal delivery x1

MEDICATIONS:
Home Medications:
- Ferrous sulfate 325 mg by mouth daily
- Ropinirole 0.5 mg by mouth at bedtime
- Tadalafil 40 mg by mouth daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Uterine leiomyomas in mother
- Restless legs syndrome in mother
- Pulmonary arterial hypertension in maternal aunt

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative assistant.
