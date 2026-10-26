# synthetic_hospital patient 2064

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 53 at first encounter, sex M, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-12-19 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-03-15 | 2023-02-10 (Fri) | ed | Emergency Medicine |
| 2 | 2020-04-15 | 2023-03-12 (Sun) | ed | Emergency Medicine |
| 3 | 2020-09-15 | 2023-08-20 (Sun) | ed | Emergency Medicine |
| 4 | 2021-03-15 | 2024-02-15 (Thu) | outpatient | Psychiatry |
| 5 | 2022-03-15 | 2025-02-19 (Wed) | ed | Emergency Medicine |
| 6 | 2022-09-15 | 2025-08-18 (Mon) | ed | Emergency Medicine |
| 7 | 2023-03-15 | 2026-02-15 (Sun) | inpatient | Pulmonology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Unemployed (previously construction laborer)",
 "alcohol_use": "Severe alcohol use disorder with daily consumption of approximately one fifth of vodka, frequent binge episodes resulting in loss of consciousness, and history of falling asleep in supine position while intoxicated",
 "family_history": [
  "Father: Alcohol use disorder with cirrhosis (deceased age 61)",
  "Mother: Major depressive disorder",
  "Brother: Chronic obstructive pulmonary disease",
  "Sister: Hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 1 pack per day for 35 years",
 "home_medications": [
  {
   "dose": "100 mg oral daily",
   "name": "Thiamine (Vitamin B1)"
  },
  {
   "dose": "500 mg oral daily",
   "name": "Ascorbic acid (Vitamin C)"
  },
  {
   "dose": "1 mg oral daily",
   "name": "Folic acid"
  },
  {
   "dose": "15 mg oral at bedtime",
   "name": "Mirtazapine"
  },
  {
   "dose": "20 mg oral daily",
   "name": "Omeprazole"
  },
  {
   "dose": "400 mg oral twice daily",
   "name": "Magnesium oxide"
  },
  {
   "dose": "1 tablet oral daily",
   "name": "Multivitamin"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Alcohol use disorder, severe",
  "Major depressive disorder, recurrent",
  "Wernicke-Korsakoff syndrome",
  "Protein-energy malnutrition",
  "Multiple vitamin deficiency states (thiamine, vitamin C, folate, B12)",
  "Gastroesophageal reflux disease with chronic aspiration risk"
 ],
 "age_at_first_encounter": 53
}
```

Primary diagnoses: ["Scurvy", "Wernicke-Korsakoff syndrome", "Lung abscess (aspiration)", "Tuberculosis", "Korsakoff syndrome", "Wernicke encephalopathy", "Hypomagnesemia"]
Comorbidities: ["Protein-energy malnutrition", "Anemia", "Alcohol use disorder"]

## Encounter 0: source 2020-01-15, practice 2022-12-19, outpatient, Internal Medicine

Chief complaint: Fatigue, easy bruising, and painful gums

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue, easy bruising, and painful gums

HISTORY OF PRESENT ILLNESS:
He reports a diet consisting mainly of canned food and beer.

PAST MEDICAL HISTORY:
Active Problem List:
- Alcohol use disorder, severe
- Major depressive disorder, recurrent
- Wernicke-Korsakoff syndrome
- Protein-energy malnutrition
- Multiple vitamin deficiency states (thiamine, vitamin C, folate, B12)
- Gastroesophageal reflux disease with chronic aspiration risk

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Thiamine (Vitamin B1) 100 mg oral daily
- Ascorbic acid (Vitamin C) 500 mg oral daily
- Folic acid 1 mg oral daily
- Mirtazapine 15 mg oral at bedtime
- Omeprazole 20 mg oral daily
- Magnesium oxide 400 mg oral twice daily
- Multivitamin 1 tablet oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis (deceased age 61)
- Mother: Major depressive disorder
- Brother: Chronic obstructive pulmonary disease
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day for 35 years. Alcohol: Severe alcohol use disorder with daily consumption of approximately one fifth of vodka, frequent binge episodes resulting in loss of consciousness, and history of falling asleep in supine position while intoxicated. Occupation: Unemployed (previously construction laborer).

PHYSICAL EXAMINATION:
On physical examination, the patient has swollen, bleeding gums, and his teeth appear loose. There are multiple ecchymoses of varying ages on his arms and legs, as well as hyperkeratotic papules surrounding hair follicles.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 11.2 g/dL (Normal: 13.8-17.2 g/dL)
- Mean corpuscular volume (MCV): 92 fL (Normal: 80-100 fL)
- Platelet count: 250,000/µL (Normal: 150,000-450,000/µL)
- Plasma vitamin C level: Pending

## Encounter 1: source 2020-03-15, practice 2023-02-10, ed, Emergency Medicine

Chief complaint: Acute confusion, unsteady gait, and visual disturbances

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Michael Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute confusion, unsteady gait, and visual disturbances

HISTORY OF PRESENT ILLNESS:
The patient, with a history of scurvy diagnosed two months prior and severe alcohol use disorder, is brought to the emergency department by his brother due to increasing confusion, unsteady gait, and visual disturbances over the past few days. He has recently been eating poorly.

PAST MEDICAL HISTORY:
Active Problem List:
- Scurvy (diagnosed 2020-01-15)
- Alcohol use disorder, severe
- Major depressive disorder, recurrent
- Wernicke-Korsakoff syndrome
- Protein-energy malnutrition
- Multiple vitamin deficiency states (thiamine, vitamin C, folate, B12)
- Gastroesophageal reflux disease with chronic aspiration risk

The patient has a long history of alcohol use disorder.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Thiamine (Vitamin B1) 100 mg oral daily
- Ascorbic acid (Vitamin C) 500 mg oral daily
- Folic acid 1 mg oral daily
- Mirtazapine 15 mg oral at bedtime
- Omeprazole 20 mg oral daily
- Magnesium oxide 400 mg oral twice daily
- Multivitamin 1 tablet oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis (deceased age 61)
- Mother: Major depressive disorder
- Brother: Chronic obstructive pulmonary disease
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day for 35 years. Alcohol: Severe alcohol use disorder with daily consumption of approximately one fifth of vodka, frequent binge episodes resulting in loss of consciousness, and history of falling asleep in supine position while intoxicated. Occupation: Unemployed (previously construction laborer).

PHYSICAL EXAMINATION:
On examination, he is disoriented to time and place, has horizontal nystagmus, and exhibits a wide-based gait. He also displays impaired short-term memory and makes up information when asked about recent events.

## Encounter 2: source 2020-04-15, practice 2023-03-12, ed, Emergency Medicine

Chief complaint: Muscle cramps, fatigue, and tingling in hands

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Muscle cramps, fatigue, and tingling in hands

HISTORY OF PRESENT ILLNESS:
The patient with severe alcohol use disorder, malnutrition, and recent scurvy and Wernicke encephalopathy presents with muscle cramps, fatigue, and tingling in his hands. He recently began eating less due to poor appetite. Despite several doses, his symptoms persist. There are no signs of recent neck surgery or trauma.

PAST MEDICAL HISTORY:
Active Problem List:
- Scurvy (diagnosed 2020-01-15)
- Wernicke encephalopathy (diagnosed 2020-03-15)
- Alcohol use disorder, severe
- Major depressive disorder, recurrent
- Wernicke-Korsakoff syndrome
- Protein-energy malnutrition
- Multiple vitamin deficiency states (thiamine, vitamin C, folate, B12)
- Gastroesophageal reflux disease with chronic aspiration risk

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He is started on intravenous calcium gluconate

Home Medications:
- Thiamine (Vitamin B1) 100 mg oral daily
- Ascorbic acid (Vitamin C) 500 mg oral daily
- Folic acid 1 mg oral daily
- Mirtazapine 15 mg oral at bedtime
- Omeprazole 20 mg oral daily
- Magnesium oxide 400 mg oral twice daily
- Multivitamin 1 tablet oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis (deceased age 61)
- Mother: Major depressive disorder
- Brother: Chronic obstructive pulmonary disease
- Sister: Hypertension

SOCIAL HISTORY:
He has a long history of alcohol use

PHYSICAL EXAMINATION:
Physical examination reveals mild confusion and a positive Trousseau sign.

LABORATORY DATA:
Serum calcium is found to be low at 7.1 mg/dL. repeat calcium levels remain low. His serum albumin is 3.9 g/dL, and renal function is normal. His phosphate and vitamin D levels are within normal limits.

OTHER STUDIES:
ECG shows a prolonged QT interval.

## Encounter 3: source 2020-09-15, practice 2023-08-20, ed, Emergency Medicine

Chief complaint: Increasing confusion, unsteady gait, and visual disturbances over past few days

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Increasing confusion, unsteady gait, and visual disturbances over past few days

HISTORY OF PRESENT ILLNESS:
The patient returns with increasing confusion, unsteady gait, and visual disturbances similar to his March 2020 presentation, and has recently been eating poorly.

PAST MEDICAL HISTORY:
Active Problem List:
- Scurvy (diagnosed 2020-01-15)
- Wernicke encephalopathy (diagnosed 2020-03-15)
- Hypomagnesemia (diagnosed 2020-04-15)
- Alcohol use disorder, severe
- Major depressive disorder, recurrent
- Wernicke-Korsakoff syndrome
- Protein-energy malnutrition
- Multiple vitamin deficiency states (thiamine, vitamin C, folate, B12)
- Gastroesophageal reflux disease with chronic aspiration risk

The patient has a long history of alcohol use disorder.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Thiamine (Vitamin B1) 100 mg oral daily
- Ascorbic acid (Vitamin C) 500 mg oral daily
- Folic acid 1 mg oral daily
- Mirtazapine 15 mg oral at bedtime
- Omeprazole 20 mg oral daily
- Magnesium oxide 400 mg oral twice daily
- Multivitamin 1 tablet oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis (deceased age 61)
- Mother: Major depressive disorder
- Brother: Chronic obstructive pulmonary disease
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day for 35 years. Alcohol: Severe alcohol use disorder with daily consumption of approximately one fifth of vodka, frequent binge episodes resulting in loss of consciousness, and history of falling asleep in supine position while intoxicated. Occupation: Unemployed (previously construction laborer).

PHYSICAL EXAMINATION:
On examination, he is disoriented to time and place, has horizontal nystagmus, and exhibits a wide-based gait. He also displays impaired short-term memory and makes up information when asked about recent events.

## Encounter 4: source 2021-03-15, practice 2024-02-15, outpatient, Psychiatry

Chief complaint: Progressive forgetfulness and memory gaps reported by family

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Maria Gonzalez, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive forgetfulness and memory gaps reported by family

HISTORY OF PRESENT ILLNESS:
The patient returns for psychiatric follow-up following prior episodes of acute confusion, unsteady gait, and visual disturbances. He presents to the clinic with his sister, who reports that he has been increasingly forgetful over the past few months. She also mentions that he often makes up stories to fill in gaps in his memory and seems indifferent to his deteriorating condition.

PAST MEDICAL HISTORY:
Active Problem List:
- Scurvy (diagnosed 2020-01-15)
- Wernicke encephalopathy (diagnosed 2020-03-15)
- Hypomagnesemia (diagnosed 2020-04-15)
- Korsakoff syndrome (diagnosed 2020-09-15)
- Alcohol use disorder, severe
- Major depressive disorder, recurrent
- Wernicke-Korsakoff syndrome
- Protein-energy malnutrition
- Multiple vitamin deficiency states (thiamine, vitamin C, folate, B12)
- Gastroesophageal reflux disease with chronic aspiration risk

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Thiamine (Vitamin B1) 100 mg oral daily
- Ascorbic acid (Vitamin C) 500 mg oral daily
- Folic acid 1 mg oral daily
- Mirtazapine 15 mg oral at bedtime
- Omeprazole 20 mg oral daily
- Magnesium oxide 400 mg oral twice daily
- Multivitamin 1 tablet oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis (deceased age 61)
- Mother: Major depressive disorder
- Brother: Chronic obstructive pulmonary disease
- Sister: Hypertension

SOCIAL HISTORY:
The patient admits to consuming a fifth of vodka daily for several years but denies any recent changes in his drinking pattern.

PHYSICAL EXAMINATION:
On examination, the patient is disheveled, has a normal gait, and is oriented to person but not to time or place. He is unable to recall events from earlier in the day and repeatedly asks why he is at the clinic. His neurological examination is unremarkable, and there are no signs of focal deficits.

LABORATORY DATA:
Hemoglobin: 10.2 g/dL (normal: 13.8-17.2 g/dL)
- Mean corpuscular volume (MCV): 98 fL (normal: 80-96 fL)
- Thiamine level: 50 nmol/L (normal: 70-180 nmol/L)
- Liver function tests: Mildly elevated AST and ALT

## Encounter 5: source 2022-03-15, practice 2025-02-19, ed, Emergency Medicine

Chief complaint: Progressive fatigue, fever, and persistent cough over one month

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive fatigue, fever, and persistent cough over one month

HISTORY OF PRESENT ILLNESS:
The patient returns to the Emergency Department with progressive respiratory symptoms in the context of his severe alcohol use disorder, Wernicke-Korsakoff syndrome, and history of scurvy and Wernicke encephalopathy. He reports night sweats, decreased appetite, and a 7-kg (15-lb) unintentional weight loss. The cough is occasionally productive of dark-tinged sputum, and he has felt increasingly weak and short of breath. He denies recent travel or known sick contacts.

PAST MEDICAL HISTORY:
Active Problem List:
- Scurvy (diagnosed 2020-01-15)
- Wernicke encephalopathy (diagnosed 2020-03-15)
- Hypomagnesemia (diagnosed 2020-04-15)
- Korsakoff syndrome (diagnosed 2020-09-15)
- Wernicke-Korsakoff syndrome (diagnosed 2021-03-15)
- Alcohol use disorder, severe
- Major depressive disorder, recurrent
- Protein-energy malnutrition
- Multiple vitamin deficiency states (thiamine, vitamin C, folate, B12)
- Gastroesophageal reflux disease with chronic aspiration risk

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Thiamine (Vitamin B1) 100 mg oral daily
- Ascorbic acid (Vitamin C) 500 mg oral daily
- Folic acid 1 mg oral daily
- Mirtazapine 15 mg oral at bedtime
- Omeprazole 20 mg oral daily
- Magnesium oxide 400 mg oral twice daily
- Multivitamin 1 tablet oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis (deceased age 61)
- Mother: Major depressive disorder
- Brother: Chronic obstructive pulmonary disease
- Sister: Hypertension

SOCIAL HISTORY:
He is currently living in a homeless shelter and has a history of chronic alcohol use.

VITAL SIGNS:
Temperature is 38.4°C (101.1°F), blood pressure is 100/64 mm Hg, pulse is 108/min, and respiratory rate is 22/min.

PHYSICAL EXAMINATION:
On examination, the patient appears thin and fatigued. He has dry mucous membranes, coarse breath sounds in the upper posterior lung zones, and no peripheral edema or lymphadenopathy.

LABORATORY DATA:
Laboratory evaluation shows normocytic anemia and mildly elevated liver enzymes.

OTHER STUDIES:
A sputum stain reveals acid-fast bacilli organized in a serpentine pattern.

## Encounter 6: source 2022-09-15, practice 2025-08-18, ed, Emergency Medicine

Chief complaint: Several weeks of worsening cough, fever, and foul-smputum after heavy drinking and vomiting

ENCOUNTER NOTE
Date: 2022-09-15
Provider: Dr. Lisa Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Several weeks of worsening cough, fever, and foul-smputum after heavy drinking and vomiting

HISTORY OF PRESENT ILLNESS:
The patient returns with worsening cough and fever, six months after tuberculosis treatment, in the setting of heavy alcohol use and Wernicke-Korsakoff syndrome. He describes the sputum as thick and putrid, occasionally blood-tinged, and notes that his symptoms began after a night of heavy drinking during which he vomited multiple times. He denies any recent travel, tuberculosis exposure, or known immunodeficiency, but reports ongoing fatigue, weight loss, and pleuritic chest pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Scurvy (diagnosed 2020-01-15)
- Wernicke encephalopathy (diagnosed 2020-03-15)
- Hypomagnesemia (diagnosed 2020-04-15)
- Korsakoff syndrome (diagnosed 2020-09-15)
- Wernicke-Korsakoff syndrome (diagnosed 2021-03-15)
- Tuberculosis (diagnosed 2022-03-15)
- Alcohol use disorder, severe
- Major depressive disorder, recurrent
- Protein-energy malnutrition
- Multiple vitamin deficiency states (thiamine, vitamin C, folate, B12)
- Gastroesophageal reflux disease with chronic aspiration risk

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Thiamine (Vitamin B1) 100 mg oral daily
- Ascorbic acid (Vitamin C) 500 mg oral daily
- Folic acid 1 mg oral daily
- Mirtazapine 15 mg oral at bedtime
- Omeprazole 20 mg oral daily
- Magnesium oxide 400 mg oral twice daily
- Multivitamin 1 tablet oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis (deceased age 61)
- Mother: Major depressive disorder
- Brother: Chronic obstructive pulmonary disease
- Sister: Hypertension

SOCIAL HISTORY:
He has a long-standing history of alcoholism and frequently falls asleep in a supine position while intoxicated.

VITAL SIGNS:
Currently afebrile

PHYSICAL EXAMINATION:
On examination, he appears cachectic and mildly diaphoretic. Breath sounds are decreased at the right lung base with dullness to percussion and occasional crackles.

IMAGING:
Chest imaging reveals a cavitary lesion with an air-fluid level in the right lower lobe, consistent with a lung abscess.

OTHER STUDIES:
Bronchoalveolar lavage is performed, and anaerobic cultures grow Bacteroides spp .

## Encounter 7: source 2023-03-15, practice 2026-02-15, inpatient, Pulmonology

Chief complaint: Three-week history of fever, night sweats, and productive cough with foul-smelling sputum

ENCOUNTER NOTE
Date: 2023-03-15
Provider: Dr. Robert Thompson, MD
Department: Pulmonology
Visit Type: inpatient

CHIEF COMPLAINT:
Three-week history of fever, night sweats, and productive cough with foul-smelling sputum

HISTORY OF PRESENT ILLNESS:
The patient returns with recurrent aspiration symptoms akin to those six months prior. He presents to the emergency department with a 3-week history of fever, night sweats, and a productive cough with foul-smelling sputum. He reports a 10-pound weight loss over the last month.

PAST MEDICAL HISTORY:
Active Problem List:
- Scurvy (diagnosed 2020-01-15)
- Wernicke encephalopathy (diagnosed 2020-03-15)
- Hypomagnesemia (diagnosed 2020-04-15)
- Korsakoff syndrome (diagnosed 2020-09-15)
- Wernicke-Korsakoff syndrome (diagnosed 2021-03-15)
- Tuberculosis (diagnosed 2022-03-15)
- Lung abscess (aspiration) (diagnosed 2022-09-15)
- Alcohol use disorder, severe
- Major depressive disorder, recurrent
- Protein-energy malnutrition
- Multiple vitamin deficiency states (thiamine, vitamin C, folate, B12)
- Gastroesophageal reflux disease with chronic aspiration risk

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Thiamine (Vitamin B1) 100 mg oral daily
- Ascorbic acid (Vitamin C) 500 mg oral daily
- Folic acid 1 mg oral daily
- Mirtazapine 15 mg oral at bedtime
- Omeprazole 20 mg oral daily
- Magnesium oxide 400 mg oral twice daily
- Multivitamin 1 tablet oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder with cirrhosis (deceased age 61)
- Mother: Major depressive disorder
- Brother: Chronic obstructive pulmonary disease
- Sister: Hypertension

SOCIAL HISTORY:
The patient admits to frequent episodes of binge drinking and occasional loss of consciousness.

VITAL SIGNS:
On physical examination, he is febrile with a temperature of 38.7°C (101.7°F), tachycardic with a heart rate of 102 beats per minute

PHYSICAL EXAMINATION:
He has decreased breath sounds with dullness to percussion at the right lung base.

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 15,000 /µL (normal: 4,500-11,000 /µL)
- Hemoglobin: 13.2 g/dL (normal: 13.8-17.2 g/dL)
- Sodium: 135 mEq/L (normal: 136-145 mEq/L)
- Blood urea nitrogen: 18 mg/dL (normal: 7-20 mg/dL)
- Creatinine: 1.0 mg/dL (normal: 0.6-1.2 mg/dL)

IMAGING:
Chest X-ray reveals a thick-walled cavitary lesion in the right lower lobe with an air-fluid level.
