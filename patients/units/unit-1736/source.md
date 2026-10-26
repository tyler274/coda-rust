# synthetic_hospital patient 1736

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex F, Non-Hispanic White, Private PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-01-09 (Mon) | outpatient | Gastroenterology |
| 1 | 2020-04-15 | 2023-04-05 (Wed) | ed | Emergency Medicine |
| 2 | 2020-07-15 | 2023-07-17 (Mon) | outpatient | Neurology |
| 3 | 2021-01-15 | 2024-01-14 (Sun) | icu | Dermatology |
| 4 | 2022-01-15 | 2025-01-09 (Thu) | ed | Emergency Medicine |
| 5 | 2022-07-15 | 2025-07-09 (Wed) | ed | Emergency Medicine |
| 6 | 2023-01-15 | 2026-01-06 (Tue) | inpatient | Obstetrics & Gynecology |
| 7 | 2023-07-15 | 2026-06-29 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Lamotrigine (previous Toxic Epidermal Necrolysis)",
  "Sulfa drugs (rash)"
 ],
 "insurance": "Private PPO",
 "occupation": "Licensed practical nurse",
 "alcohol_use": "None",
 "family_history": [
  "Mother: Major depressive disorder, chronic migraine",
  "Maternal aunt: Epilepsy",
  "Father: Essential hypertension",
  "Sister: Irritable bowel syndrome"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "100 mg PO daily",
   "name": "Sertraline"
  },
  {
   "dose": "100 mg PO PRN migraine (max 200 mg/day)",
   "name": "Sumatriptan"
  },
  {
   "dose": "2 mg PO PRN diarrhea (max 8 mg/day)",
   "name": "Loperamide"
  }
 ],
 "surgical_history": [
  "Laparoscopic appendectomy (age 24)"
 ],
 "chronic_conditions": [
  "Major depressive disorder",
  "Chronic migraine",
  "Irritable bowel syndrome with diarrhea",
  "Epilepsy",
  "Factitious disorder imposed on self (Munchausen syndrome)"
 ],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Serotonin syndrome", "Factitious diarrhea", "Toxic epidermal necrolysis", "Severe preeclampsia without HELLP features", "Munchausen syndrome (factitious disorder imposed on self)"]
Comorbidities: ["Acute kidney injury", "Major depressive disorder", "Chronic migraine without aura", "Metformin-associated encephalopathy"]

## Encounter 0: source 2020-01-15, practice 2023-01-09, outpatient, Gastroenterology

Chief complaint: Six-month history of chronic watery diarrhea with abdominal cramping

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of chronic watery diarrhea with abdominal cramping

HISTORY OF PRESENT ILLNESS:
She reports 5-6 loose, watery bowel movements per day, which are occasionally accompanied by abdominal cramping. She denies any blood or mucus in her stool.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder
- Chronic migraine
- Irritable bowel syndrome with diarrhea
- Epilepsy
- Factitious disorder imposed on self (Munchausen syndrome)

Her past medical history is significant for depression and irritable bowel syndrome,.

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy (age 24)

MEDICATIONS:
She takes for which she takes sertraline and occasional loperamide.

Home Medications:
- Sumatriptan 100 mg PO PRN migraine (max 200 mg/day)

ALLERGIES:
Lamotrigine (previous Toxic Epidermal Necrolysis), Sulfa drugs (rash)

FAMILY HISTORY:
- Mother: Major depressive disorder, chronic migraine
- Maternal aunt: Epilepsy
- Father: Essential hypertension
- Sister: Irritable bowel syndrome

SOCIAL HISTORY:
She denies any recent travel, sick contacts, or dietary changes.

PHYSICAL EXAMINATION:
Physical examination is unremarkable, with no abdominal tenderness or distension.

LABORATORY DATA:
Recent laboratory results are as follows:

- Stool osmolality: 260 mOsm/kg (normal range: 280-310 mOsm/kg)
- Serum osmolality: 290 mOsm/kg (normal range: 275-295 mOsm/kg)
- Stool sodium: 30 mEq/L
- Stool potassium: 40 mEq/L
- Stool magnesium: 5 mg/dL (normal range: 1.5-2.5 mg/dL)
- Stool phosphate: 4 mg/dL (normal range: 2.5-4.5 mg/dL)

OTHER STUDIES:
Extensive workup including stool cultures, ova and parasite examination, colonoscopy with biopsy, and celiac serologies have all returned negative.

ASSESSMENT AND PLAN:
The stool osmotic gap is calculated to be 90 mOsm/kg.

## Encounter 1: source 2020-04-15, practice 2023-04-05, ed, Emergency Medicine

Chief complaint: Recurrent episodes of confusion, diaphoresis, and tremors over two months

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Recurrent episodes of confusion, diaphoresis, and tremors over two months

HISTORY OF PRESENT ILLNESS:
This 32-year-old female with a known history of factitious disorder imposed on self, previously evaluated in Gastroenterology three months prior for reported chronic watery diarrhea, presents to the emergency department with recurrent episodes of confusion, diaphoresis, and tremors, which she states have been occurring intermittently over the past two months. She reports that these episodes often happen in the late mornings and are sometimes accompanied by palpitations.

PAST MEDICAL HISTORY:
Active Problem List:
- Factitious diarrhea (diagnosed 2020-01-15)
- Major depressive disorder
- Chronic migraine
- Irritable bowel syndrome with diarrhea
- Epilepsy
- Factitious disorder imposed on self (Munchausen syndrome)

Her past medical history is significant for depression and.

PAST SURGICAL HISTORY:
A complex surgical history, including multiple abdominal operations for unclear indications.

MEDICATIONS:
She denies any use of medications

Home Medications:
- Sertraline 100 mg PO daily
- Sumatriptan 100 mg PO PRN migraine (max 200 mg/day)
- Loperamide 2 mg PO PRN diarrhea (max 8 mg/day)

ALLERGIES:
Lamotrigine (previous Toxic Epidermal Necrolysis), Sulfa drugs (rash)

FAMILY HISTORY:
- Mother: Major depressive disorder, chronic migraine
- Maternal aunt: Epilepsy
- Father: Essential hypertension
- Sister: Irritable bowel syndrome

SOCIAL HISTORY:
She denies any use of alcohol, or illicit drugs.

VITAL SIGNS:
Her vital signs are within normal limits.

PHYSICAL EXAMINATION:
On physical examination, she appears anxious but is otherwise unremarkable.

LABORATORY DATA:
A fingerstick glucose measurement during an episode reveals a blood glucose level of 40 mg/dL. Additional laboratory tests are ordered:

- Hemoglobin A1c: 5.2%
- C-peptide: Elevated
- Insulin: Elevated
- Sulfonylurea screen: Positive

## Encounter 2: source 2020-07-15, practice 2023-07-17, outpatient, Neurology

Chief complaint: Enrollment in crossover clinical trial for refractory chronic migraine

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Emily Johnson, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Enrollment in crossover clinical trial for refractory chronic migraine

HISTORY OF PRESENT ILLNESS:
This 32-year-old female with major depressive disorder, chronic migraine, irritable bowel syndrome, epilepsy, and factitious disorder imposed on self returns following gastroenterology and emergency encounters for trial enrollment. She has been experiencing 15 headache days per month for the past year. The trial involves the patient taking the new medication for 3 months and then switching to a placebo for another 3 months, without knowing the order. The patient's headache frequency is recorded monthly. The patient reports no significant side effects during the trial.

PAST MEDICAL HISTORY:
Active Problem List:
- Factitious diarrhea (diagnosed 2020-01-15)
- Munchausen syndrome (factitious disorder imposed on self) (diagnosed 2020-04-15)
- Major depressive disorder
- Chronic migraine
- Irritable bowel syndrome with diarrhea
- Epilepsy

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy (age 24)

MEDICATIONS:
Home Medications:
- Sertraline 100 mg PO daily
- Sumatriptan 100 mg PO PRN migraine (max 200 mg/day)
- Loperamide 2 mg PO PRN diarrhea (max 8 mg/day)

ALLERGIES:
Lamotrigine (previous Toxic Epidermal Necrolysis), Sulfa drugs (rash)

FAMILY HISTORY:
- Mother: Major depressive disorder, chronic migraine
- Maternal aunt: Epilepsy
- Father: Essential hypertension
- Sister: Irritable bowel syndrome

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Licensed practical nurse.

PHYSICAL EXAMINATION:
Physical examination and neurological evaluation are unremarkable

OTHER STUDIES:
- Months 1-3: 8, 7, 6 headache days
- Months 4-6: 14, 15, 15 headache days

## Encounter 3: source 2021-01-15, practice 2024-01-14, icu, Dermatology

Chief complaint: Three-day history of fever, sore throat, and painful progressive skin lesions with detachment

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. David Kim, MD
Department: Dermatology
Visit Type: icu

CHIEF COMPLAINT:
Three-day history of fever, sore throat, and painful progressive skin lesions with detachment

HISTORY OF PRESENT ILLNESS:
The patient, with established factitious disorder imposed on self and prior factitious diarrhea, presents with skin lesions. She reports that the lesions began as red macules on her trunk and have rapidly progressed to involve her face, trunk, and extremities with areas of flaccid blisters and skin detachment.

PAST MEDICAL HISTORY:
Active Problem List:
- Factitious diarrhea (diagnosed 2020-01-15)
- Munchausen syndrome (factitious disorder imposed on self) (diagnosed 2020-04-15)
- Major depressive disorder
- Chronic migraine
- Irritable bowel syndrome with diarrhea
- Epilepsy

She has a history of epilepsy.

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy (age 24)

MEDICATIONS:
She takes she started taking lamotrigine 2 weeks ago.

Home Medications:
- Sertraline 100 mg PO daily
- Sumatriptan 100 mg PO PRN migraine (max 200 mg/day)
- Loperamide 2 mg PO PRN diarrhea (max 8 mg/day)

ALLERGIES:
Lamotrigine (previous Toxic Epidermal Necrolysis), Sulfa drugs (rash)

FAMILY HISTORY:
- Mother: Major depressive disorder, chronic migraine
- Maternal aunt: Epilepsy
- Father: Essential hypertension
- Sister: Irritable bowel syndrome

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Licensed practical nurse.

VITAL SIGNS:
Her temperature is 38.9°C (102°F), heart rate is 110/min, and blood pressure is 100/60 mmHg

PHYSICAL EXAMINATION:
Nikolsky sign is positive. Mucosal involvement is noted with erosions on the buccal mucosa and conjunctival injection.
Body surface area involvement is estimated to be 35%.

LABORATORY DATA:
- White blood cell count: 15,000/mm³ (Normal: 4,500-11,000/mm³)
- Hemoglobin: 13.2 g/dL (Normal: 12-16 g/dL)
- Platelets: 250,000/mm³ (Normal: 150,000-450,000/mm³)
- Serum creatinine: 1.2 mg/dL (Normal: 0.6-1.1 mg/dL)
- Alanine aminotransferase (ALT): 45 U/L (Normal: 7-56 U/L)
- Aspartate aminotransferase (AST): 47 U/L (Normal: 8-48 U/L)

## Encounter 4: source 2022-01-15, practice 2025-01-09, ed, Emergency Medicine

Chief complaint: Six-hour history of confusion, agitation, and tremors with morning vomiting

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Six-hour history of confusion, agitation, and tremors with morning vomiting

HISTORY OF PRESENT ILLNESS:
The patient returns with a history of factitious disorder and prior confusion and tremors, and was recently started on a new medication regimen by her psychiatrist. Her husband reports that she has been more anxious over the past week and had several episodes of vomiting this morning.

PAST MEDICAL HISTORY:
Active Problem List:
- Factitious diarrhea (diagnosed 2020-01-15)
- Munchausen syndrome (factitious disorder imposed on self) (diagnosed 2020-04-15)
- Toxic epidermal necrolysis (diagnosed 2021-01-15)
- Major depressive disorder
- Chronic migraine
- Irritable bowel syndrome with diarrhea
- Epilepsy

She has a past medical history of major depressive disorder.

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy (age 24)

MEDICATIONS:
Her current medications include fluoxetine, sumatriptan for occasional migraines, and over-the-counter cough syrup that contains dextromethorphan.

Home Medications:
- Sertraline 100 mg PO daily
- Loperamide 2 mg PO PRN diarrhea (max 8 mg/day)

ALLERGIES:
Lamotrigine (previous Toxic Epidermal Necrolysis), Sulfa drugs (rash)

FAMILY HISTORY:
- Mother: Major depressive disorder, chronic migraine
- Maternal aunt: Epilepsy
- Father: Essential hypertension
- Sister: Irritable bowel syndrome

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Licensed practical nurse.

VITAL SIGNS:
Tachycardic with a heart rate of 120 bpm, and hypertensive with a blood pressure of 160/95 mmHg. Her temperature is 38.9°C (102°F).

PHYSICAL EXAMINATION:
On examination, she is diaphoretic. Neurological examination reveals hyperreflexia, inducible clonus, and bilateral dilated pupils that are reactive to light. She is oriented to person but not to time or place.

LABORATORY DATA:
Laboratory values:
- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Chloride: 102 mEq/L
- Bicarbonate: 24 mEq/L
- Blood urea nitrogen: 18 mg/dL
- Creatinine: 0.9 mg/dL
- Glucose: 98 mg/dL
- Complete blood count: Unremarkable
- Urinalysis: Normal

## Encounter 5: source 2022-07-15, practice 2025-07-09, ed, Emergency Medicine

Chief complaint: Two-hour history of confusion, agitation, tremors, profuse sweating, and fever

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. James Wilson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-hour history of confusion, agitation, tremors, profuse sweating, and fever

HISTORY OF PRESENT ILLNESS:
Returning with symptoms similar to her January 2022 visit, this patient with factitious disorder recently started taking a new medication prescribed by her psychiatrist. Her husband reports she is sweating profusely and seems to have a fever.

PAST MEDICAL HISTORY:
Active Problem List:
- Factitious diarrhea (diagnosed 2020-01-15)
- Munchausen syndrome (factitious disorder imposed on self) (diagnosed 2020-04-15)
- Toxic epidermal necrolysis (diagnosed 2021-01-15)
- Serotonin syndrome (diagnosed 2022-01-15)
- Major depressive disorder
- Chronic migraine
- Irritable bowel syndrome with diarrhea
- Epilepsy

She has a history of major depressive disorder.

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy (age 24)

MEDICATIONS:
Her current medications include fluoxetine and sumatriptan for migraines.

Home Medications:
- Sertraline 100 mg PO daily
- Loperamide 2 mg PO PRN diarrhea (max 8 mg/day)

ALLERGIES:
Lamotrigine (previous Toxic Epidermal Necrolysis), Sulfa drugs (rash)

FAMILY HISTORY:
- Mother: Major depressive disorder, chronic migraine
- Maternal aunt: Epilepsy
- Father: Essential hypertension
- Sister: Irritable bowel syndrome

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Licensed practical nurse.

VITAL SIGNS:
Her temperature is 39.2°C (102.5°F), heart rate is 120 beats per minute, blood pressure is 160/90 mmHg.

PHYSICAL EXAMINATION:
She is diaphoretic. Neurological examination reveals hyperreflexia, inducible clonus, and ocular clonus. She is oriented to person but not to place or time.

LABORATORY DATA:
Sodium: 140 mEq/L, Potassium: 4.2 mEq/L, Chloride: 102 mEq/L, Bicarbonate: 24 mEq/L, Blood urea nitrogen: 18 mg/dL, Creatinine: 0.9 mg/dL, Glucose: 98 mg/dL, Complete blood count: within normal limits.

## Encounter 6: source 2023-01-15, practice 2026-01-06, inpatient, Obstetrics & Gynecology

Chief complaint: Acute shortness of breath with pink frothy sputum and decreased urine output

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Priya Sharma, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
Acute shortness of breath with pink frothy sputum and decreased urine output

HISTORY OF PRESENT ILLNESS:
Over the past two days, this patient with depression, epilepsy, and prior serotonin syndrome has noticed swelling in her legs and a significant decrease in urine output.

PAST MEDICAL HISTORY:
Active Problem List:
- Factitious diarrhea (diagnosed 2020-01-15)
- Munchausen syndrome (factitious disorder imposed on self) (diagnosed 2020-04-15)
- Toxic epidermal necrolysis (diagnosed 2021-01-15)
- Serotonin syndrome (diagnosed 2022-01-15)
- Major depressive disorder
- Chronic migraine
- Irritable bowel syndrome with diarrhea
- Epilepsy

She has a history of mild hypertension,.

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy (age 24)

MEDICATIONS:
She takes which has been managed with labetalol since the beginning of her pregnancy.

Home Medications:
- Sertraline 100 mg PO daily
- Sumatriptan 100 mg PO PRN migraine (max 200 mg/day)
- Loperamide 2 mg PO PRN diarrhea (max 8 mg/day)

ALLERGIES:
Lamotrigine (previous Toxic Epidermal Necrolysis), Sulfa drugs (rash)

FAMILY HISTORY:
- Mother: Major depressive disorder, chronic migraine
- Maternal aunt: Epilepsy
- Father: Essential hypertension
- Sister: Irritable bowel syndrome

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Licensed practical nurse.

VITAL SIGNS:
Her blood pressure is 165/110 mmHg, heart rate is 102 beats per minute, respiratory rate is 28 breaths per minute, and oxygen saturation is 89% on room air.

PHYSICAL EXAMINATION:
Physical examination reveals bilateral crackles halfway up both lung fields, 2+ pitting edema in the lower extremities, and brisk deep tendon reflexes.

LABORATORY DATA:
Laboratory results are as follows:

- Hemoglobin: 12.5 g/dL
- Platelet count: 190,000/uL
- Serum creatinine: 1.2 mg/dL (baseline 0.6 mg/dL)
- Serum albumin: 2.8 g/dL
- Urinalysis: 3+ proteinuria

IMAGING:
Chest X-ray shows bilateral pulmonary infiltrates consistent with pulmonary edema.

## Encounter 7: source 2023-07-15, practice 2026-06-29, ed, Emergency Medicine

Chief complaint: Two-hour history of acute confusion, agitation, and tremors after starting cough medication

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Robert Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-hour history of acute confusion, agitation, and tremors after starting cough medication

HISTORY OF PRESENT ILLNESS:
The patient returns to the emergency department with a history of two prior episodes of serotonin syndrome and presents with a 2-hour history of confusion, agitation, and tremors. Her husband reports that she was recently prescribed dextromethorphan for a persistent cough.

PAST MEDICAL HISTORY:
Active Problem List:
- Factitious diarrhea (diagnosed 2020-01-15)
- Munchausen syndrome (factitious disorder imposed on self) (diagnosed 2020-04-15)
- Toxic epidermal necrolysis (diagnosed 2021-01-15)
- Serotonin syndrome (diagnosed 2022-01-15)
- Severe preeclampsia without HELLP features (diagnosed 2023-01-15)
- Major depressive disorder
- Chronic migraine
- Irritable bowel syndrome with diarrhea
- Epilepsy

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy (age 24)

MEDICATIONS:
She takes sertraline and sumatriptan, respectively.

Home Medications:
- Loperamide 2 mg PO PRN diarrhea (max 8 mg/day)

ALLERGIES:
Lamotrigine (previous Toxic Epidermal Necrolysis), Sulfa drugs (rash)

FAMILY HISTORY:
- Mother: Major depressive disorder, chronic migraine
- Maternal aunt: Epilepsy
- Father: Essential hypertension
- Sister: Irritable bowel syndrome

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Licensed practical nurse.

VITAL SIGNS:
Heart rate of 120 bpm, blood pressure of 160/95 mmHg

PHYSICAL EXAMINATION:
On examination, she is diaphoretic, tachycardic with a heart rate of 120 bpm, and hypertensive with a blood pressure of 160/95 mmHg. Neurological examination reveals hyperreflexia, inducible clonus, and bilateral pupil dilation.

LABORATORY DATA:
Creatine kinase: 5200 U/L (normal 38-174 U/L)
- Serum creatinine: 2.1 mg/dL (normal 0.6-1.1 mg/dL)
- White blood cell count: 12,000/mm³ (normal 4,500-11,000/mm³)
- Platelet count: 95,000/mm³ (normal 150,000-450,000/mm³)
- Prothrombin time: 14 seconds (normal 11-13.5 seconds)
- Partial thromboplastin time: 35 seconds (normal 25-35 seconds)
- Fibrinogen: 150 mg/dL (normal 200-400 mg/dL)
- D-dimer: Elevated
