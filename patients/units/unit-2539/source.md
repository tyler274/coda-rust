# synthetic_hospital patient 2539

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 24 at first encounter, sex M, Non-Hispanic White, University Student Health Insurance Plan.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2019-11-26 (Tue) | outpatient | Cardiology |
| 1 | 2020-04-15 | 2020-02-10 (Mon) | outpatient | Endocrinology |
| 2 | 2021-01-15 | 2020-11-23 (Mon) | ed | Emergency Medicine |
| 3 | 2022-01-15 | 2021-11-14 (Sun) | ed | Emergency Medicine |
| 4 | 2023-01-15 | 2022-11-26 (Sat) | ed | Emergency Medicine |
| 5 | 2024-01-15 | 2023-11-21 (Tue) | ed | Emergency Medicine |
| 6 | 2025-01-15 | 2024-11-23 (Sat) | ed | Emergency Medicine |
| 7 | 2026-01-15 | 2025-11-24 (Mon) | outpatient | Internal Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "University Student Health Insurance Plan",
 "occupation": "Graduate Student",
 "alcohol_use": "None",
 "family_history": [
  "Type 1 Diabetes Mellitus in father",
  "Autoimmune thyroid disease in mother",
  "Hypertension in paternal grandfather"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never",
 "home_medications": [
  {
   "dose": "24 units subcutaneous at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "1 unit per 10 grams carbohydrates with meals",
   "name": "Insulin lispro"
  }
 ],
 "surgical_history": [
  "Craniotomy with evacuation of traumatic epidural hematoma"
 ],
 "chronic_conditions": [
  "Type 1 Diabetes Mellitus"
 ],
 "age_at_first_encounter": 24
}
```

Primary diagnoses: ["Orthostatic hypotension", "Diabetic ketoacidosis", "Traumatic epidural hematoma", "Traumatic brain injury", "Acute Salicylate Toxicity (Aspirin Overdose)", "Bacillus cereus food poisoning", "Enterotoxigenic Escherichia coli (ETEC) infection or Yersinia enterocolitica infection"]
Comorbidities: ["Sinus tachycardia secondary to burns", "Loss of consciousness (syncope)", "Dehydration"]

## Encounter 0: source 2020-01-15, practice 2019-11-26, outpatient, Cardiology

Chief complaint: Feeling slightly lightheaded and has a brief moment of blurry vision during physiology experiment

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Feeling slightly lightheaded and has a brief moment of blurry vision during physiology experiment

HISTORY OF PRESENT ILLNESS:
He volunteers for a clinical physiology experiment. He is hooked up to non-invasive hemodynamic monitoring equipment while resting comfortably on a table in a supine position. After 10 minutes of rest, he is asked to stand up quickly and remain upright. Within a few seconds, he reports feeling slightly lightheaded and has a brief moment of blurry vision. These symptoms resolve spontaneously within 15–20 seconds without any intervention. He is able to continue standing without support, and subsequent testing shows normalization of perfusion. The research team observes predictable changes in his cardiovascular parameters as part of the physiologic response to orthostasis.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 1 Diabetes Mellitus

He has never had similar symptoms before.

PAST SURGICAL HISTORY:
- Craniotomy with evacuation of traumatic epidural hematoma

MEDICATIONS:
He does not take any medications.

Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro 1 unit per 10 grams carbohydrates with meals

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 1 Diabetes Mellitus in father
- Autoimmune thyroid disease in mother
- Hypertension in paternal grandfather

SOCIAL HISTORY:
He exercises regularly

REVIEW OF SYSTEMS:
He reports no chest pain, palpitations, or shortness of breath.

VITAL SIGNS:
His blood pressure decreases mildly upon standing and then quickly returns to baseline. His heart rate increases moderately but appropriately.

PHYSICAL EXAMINATION:
No abnormalities are noted on physical exam or subsequent neurologic assessment.

OTHER STUDIES:
Cognitive testing reveals intact memory, orientation, and attention.

ASSESSMENT AND PLAN:
These changes are transient and reflect normal autonomic reflexes in healthy individuals. The researchers explain that his symptoms were caused by a brief mismatch in perfusion and autonomic compensation. The protocol notes state that this is an expected physiologic phenomenon.

## Encounter 1: source 2020-04-15, practice 2020-02-10, outpatient, Endocrinology

Chief complaint: Suddenly feels dizzy and lightheaded with dim vision during prolonged fasting study

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Marcus Johnson, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Suddenly feels dizzy and lightheaded with dim vision during prolonged fasting study

HISTORY OF PRESENT ILLNESS:
24-year-old male with T1DM returns following January 2020 orthostatic hypotension. Participates in prolonged fasting study with fluid restriction. After several hours standing quietly for blood draw,

PAST MEDICAL HISTORY:
Active Problem List:
- Orthostatic hypotension (diagnosed 2020-01-15)
- Type 1 Diabetes Mellitus

PAST SURGICAL HISTORY:
- Craniotomy with evacuation of traumatic epidural hematoma

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro 1 unit per 10 grams carbohydrates with meals

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 1 Diabetes Mellitus in father
- Autoimmune thyroid disease in mother
- Hypertension in paternal grandfather

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Graduate Student.

VITAL SIGNS:
His temperature is 37°C (98.6°F), blood pressure is 98/64 mm Hg supine and 78/50 mm Hg standing, and pulse increases from 72/min to 110/min.

PHYSICAL EXAMINATION:
Cardiac examination shows a rapid, regular rhythm without murmurs, and the lungs are clear. Neurologic examination is unremarkable.

ASSESSMENT AND PLAN:
This response is attributed to activation of homeostatic reflexes that maintain cerebral perfusion during postural change.

## Encounter 2: source 2021-01-15, practice 2020-11-23, ed, Emergency Medicine

Chief complaint: Severe headache and nausea after motorcycle accident

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Emily Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe headache and nausea after motorcycle accident

HISTORY OF PRESENT ILLNESS:
This 25-year-old male with Type 1 Diabetes Mellitus and prior Cardiology and Endocrinology encounters for orthostatic hypotension is brought to the emergency department after being involved in a motorcycle accident. He was not wearing a helmet and was found unconscious at the scene. On arrival, he regains consciousness and is alert and oriented.

PAST MEDICAL HISTORY:
Active Problem List:
- Orthostatic hypotension (diagnosed 2020-01-15)
- Type 1 Diabetes Mellitus

PAST SURGICAL HISTORY:
- Craniotomy with evacuation of traumatic epidural hematoma

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro 1 unit per 10 grams carbohydrates with meals

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 1 Diabetes Mellitus in father
- Autoimmune thyroid disease in mother
- Hypertension in paternal grandfather

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Graduate Student.

PHYSICAL EXAMINATION:
His Glasgow Coma Scale score is 15.
On examination, there is a contusion over his right temporal area, but no other obvious injuries. His pupils are equal and reactive to light.

IMAGING:
A CT scan of the head is performed and reveals a biconvex, hyperdense collection adjacent to the right temporal bone.

## Encounter 3: source 2022-01-15, practice 2021-11-14, ed, Emergency Medicine

Chief complaint: Unconscious after high-speed motor vehicle collision

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. David Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Unconscious after high-speed motor vehicle collision

HISTORY OF PRESENT ILLNESS:
The patient with Type 1 Diabetes Mellitus and prior traumatic epidural hematoma was the unrestrained driver, and there was significant damage to the driver's side of the vehicle. On arrival, he is unconscious.

PAST MEDICAL HISTORY:
Active Problem List:
- Orthostatic hypotension (diagnosed 2020-01-15)
- Traumatic epidural hematoma (diagnosed 2021-01-15)
- Type 1 Diabetes Mellitus

PAST SURGICAL HISTORY:
- Craniotomy with evacuation of traumatic epidural hematoma

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro 1 unit per 10 grams carbohydrates with meals

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 1 Diabetes Mellitus in father
- Autoimmune thyroid disease in mother
- Hypertension in paternal grandfather

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Graduate Student.

VITAL SIGNS:
Blood pressure 90/60 mmHg, heart rate 120 beats/min, respiratory rate 22 breaths/min, and oxygen saturation 92% on room air

PHYSICAL EXAMINATION:
Physical examination reveals a laceration to the forehead, multiple abrasions on the chest, and a deformed left leg. The patient does not open his eyes or make any sounds. When pain is applied to his sternum, he withdraws his right arm. No other movements are noted.

- Glasgow Coma Scale (GCS):
- Eye opening: None (1)
- Verbal response: None (1)
- Motor response: Withdraws to pain (4)

Based on the GCS score,

## Encounter 4: source 2023-01-15, practice 2022-11-26, ed, Emergency Medicine

Chief complaint: Disoriented and vomiting after reported aspirin ingestion

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Jennifer Walsh, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Disoriented and vomiting after reported aspirin ingestion

HISTORY OF PRESENT ILLNESS:
This 27-year-old male with Type 1 Diabetes Mellitus and prior traumatic brain injury and epidural hematoma sustained in 2021 and 2022 motor vehicle accidents is brought to the emergency department by his roommate after being found disoriented and vomiting in their apartment. Earlier in the day, the patient had mentioned taking "a bunch of aspirin" after a heated argument with his parents. About three hours later, he became increasingly confused, complained of abdominal discomfort, and was noted to be breathing rapidly and deeply.

PAST MEDICAL HISTORY:
Active Problem List:
- Orthostatic hypotension (diagnosed 2020-01-15)
- Traumatic epidural hematoma (diagnosed 2021-01-15)
- Traumatic brain injury (diagnosed 2022-01-15)
- Type 1 Diabetes Mellitus

He has no prior psychiatric history.

PAST SURGICAL HISTORY:
- Craniotomy with evacuation of traumatic epidural hematoma

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro 1 unit per 10 grams carbohydrates with meals

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 1 Diabetes Mellitus in father
- Autoimmune thyroid disease in mother
- Hypertension in paternal grandfather

SOCIAL HISTORY:
He denies alcohol or recreational drug use

VITAL SIGNS:
Temperature of 38.5°C (101.3°F), heart rate of 122/min, blood pressure of 108/66 mm Hg, and respiratory rate of 34/min. Pulse oximetry is 99% on room air.

PHYSICAL EXAMINATION:
On arrival, he is alert but drowsy and intermittently irritable. His mucous membranes are dry and his breath has a faint vinegar-like odor. Neurological exam shows no focal deficits, but the patient is slow to respond to questions.

LABORATORY DATA:
Blood tests show an elevated anion gap metabolic acidosis and low serum bicarbonate. Serum salicylate level is 66 mg/dL. Arterial blood gas reveals low PaCO₂ and low pH.

OTHER STUDIES:
ECG shows sinus tachycardia.

PLAN:
The patient is started on intravenous fluids and activated charcoal.

## Encounter 5: source 2024-01-15, practice 2023-11-21, ed, Emergency Medicine

Chief complaint: Progressive fatigue, generalized weakness, and dizziness when standing

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Michael Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive fatigue, generalized weakness, and dizziness when standing

HISTORY OF PRESENT ILLNESS:
This 28-year-old male with Type 1 Diabetes Mellitus has a history of orthostatic hypotension and multiple prior emergency department visits for traumatic injuries and medication toxicity. He is brought to the emergency department by his roommate due to a 2-day history of progressive fatigue, generalized weakness, and dizziness when standing. Earlier today, he nearly fainted while getting out of bed. He reports excessive thirst and frequent urination over the past week, and he has unintentionally lost 4 kg (9 lb) during this time.

PAST MEDICAL HISTORY:
Active Problem List:
- Orthostatic hypotension (diagnosed 2020-01-15)
- Traumatic epidural hematoma (diagnosed 2021-01-15)
- Traumatic brain injury (diagnosed 2022-01-15)
- Acute Salicylate Toxicity (Aspirin Overdose) (diagnosed 2023-01-15)
- Type 1 Diabetes Mellitus

PAST SURGICAL HISTORY:
- Craniotomy with evacuation of traumatic epidural hematoma

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro 1 unit per 10 grams carbohydrates with meals

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 1 Diabetes Mellitus in father
- Autoimmune thyroid disease in mother
- Hypertension in paternal grandfather

SOCIAL HISTORY:
He recently moved to a new city for university and has been under significant academic stress.

VITAL SIGNS:
He is 180 cm (5 ft 11 in) tall and weighs 62 kg (136 lb), with a BMI of 19.1 kg/m². His temperature is 37.2°C (99°F), pulse is 122/min, respirations are 30/min, and blood pressure is 98/50 mm Hg.

PHYSICAL EXAMINATION:
On examination, the patient is drowsy but responds to verbal stimuli. His mucous membranes are dry, and skin turgor is reduced. The lungs are clear and the abdomen is soft, without tenderness.

PLAN:
Laboratory evaluation is initiated.

## Encounter 6: source 2025-01-15, practice 2024-11-23, ed, Emergency Medicine

Chief complaint: Sudden nausea, vomiting, and watery diarrhea started 4 hours ago

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden nausea, vomiting, and watery diarrhea started 4 hours ago

HISTORY OF PRESENT ILLNESS:
The patient is a 29-year-old male with Type 1 Diabetes Mellitus who presents to the Emergency Department with acute gastrointestinal symptoms, returning approximately one year after a prior admission for diabetic ketoacidosis. He reports that the symptoms began suddenly while he was at work and have been persistent throughout the day. He denies fever, chills, abdominal pain, or blood in the stool. He has not traveled recently, had no recent sick contacts, and has not taken antibiotics in the past year. He states that the only unusual thing he ate was leftover fried rice from a takeout container that he reheated in the microwave the night before.

PAST MEDICAL HISTORY:
Active Problem List:
- Orthostatic hypotension (diagnosed 2020-01-15)
- Traumatic epidural hematoma (diagnosed 2021-01-15)
- Traumatic brain injury (diagnosed 2022-01-15)
- Acute Salicylate Toxicity (Aspirin Overdose) (diagnosed 2023-01-15)
- Diabetic ketoacidosis (diagnosed 2024-01-15)
- Type 1 Diabetes Mellitus

PAST SURGICAL HISTORY:
- Craniotomy with evacuation of traumatic epidural hematoma

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro 1 unit per 10 grams carbohydrates with meals

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 1 Diabetes Mellitus in father
- Autoimmune thyroid disease in mother
- Hypertension in paternal grandfather

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Graduate Student.

VITAL SIGNS:
Vital signs are: temperature 36.9°C (98.4°F), blood pressure 114/72 mm Hg, pulse 98/min

PHYSICAL EXAMINATION:
On exam, the patient is mildly dehydrated but alert and oriented. Mucous membranes are dry, but the abdominal exam is benign with normal bowel sounds and no tenderness.

LABORATORY DATA:
Lab studies show no leukocytosis and normal renal function.

OTHER STUDIES:
A rapid stool pathogen panel is pending.

PLAN:
He is given oral rehydration and discharged home.

## Encounter 7: source 2026-01-15, practice 2025-11-24, outpatient, Internal Medicine

Chief complaint: Three days of profuse watery diarrhea after returning from Mexico

ENCOUNTER NOTE
Date: 2026-01-15
Provider: Dr. James Foster, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Three days of profuse watery diarrhea after returning from Mexico

HISTORY OF PRESENT ILLNESS:
This 30-year-old male with Type 1 Diabetes Mellitus, previously treated for Bacillus cereus food poisoning in January 2025, presents with profuse watery diarrhea. He recently returned from a 2-week backpacking trip through rural areas of southern Mexico. During the trip, he drank untreated water and ate from several street food vendors. He reports 7–8 loose, brownish-green bowel movements daily, accompanied by mild abdominal cramping, but denies blood in the stool, fever, or vomiting. His appetite is reduced, and he feels fatigued, but he is still able to stay hydrated with oral fluids.

PAST MEDICAL HISTORY:
Active Problem List:
- Orthostatic hypotension (diagnosed 2020-01-15)
- Traumatic epidural hematoma (diagnosed 2021-01-15)
- Traumatic brain injury (diagnosed 2022-01-15)
- Acute Salicylate Toxicity (Aspirin Overdose) (diagnosed 2023-01-15)
- Diabetic ketoacidosis (diagnosed 2024-01-15)
- Bacillus cereus food poisoning (diagnosed 2025-01-15)
- Type 1 Diabetes Mellitus

PAST SURGICAL HISTORY:
- Craniotomy with evacuation of traumatic epidural hematoma

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneous at bedtime
- Insulin lispro 1 unit per 10 grams carbohydrates with meals

ALLERGIES:
NKDA

FAMILY HISTORY:
- Type 1 Diabetes Mellitus in father
- Autoimmune thyroid disease in mother
- Hypertension in paternal grandfather

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Graduate Student.

VITAL SIGNS:
Vitals show a low-grade tachycardia but are otherwise normal

PHYSICAL EXAMINATION:
On exam, he appears tired but is alert and oriented. Mucous membranes are slightly dry. Abdominal examination reveals mild diffuse tenderness without guarding, rigidity, or rebound.

LABORATORY DATA:
Laboratory testing is unremarkable except for mildly elevated hematocrit. Stool studies are pending.
