# synthetic_hospital patient 2313

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 65 at first encounter, sex M, White, non-Hispanic, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-08-08 (Mon) | outpatient | Psychiatry |
| 1 | 2020-04-15 | 2022-10-27 (Thu) | outpatient | Hematology |
| 2 | 2020-07-15 | 2023-01-23 (Mon) | outpatient | Gastroenterology |
| 3 | 2021-01-15 | 2023-07-24 (Mon) | outpatient | Internal Medicine |
| 4 | 2021-07-15 | 2024-01-22 (Mon) | outpatient | Neurology |
| 5 | 2022-01-15 | 2024-07-22 (Mon) | ed | Emergency Medicine |
| 6 | 2023-01-15 | 2025-07-26 (Sat) | ed | Emergency Medicine |
| 7 | 2024-01-15 | 2026-07-25 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare",
 "occupation": "Retired high school principal",
 "alcohol_use": "Heavy daily consumption for 30 years (6-12 standard drinks daily, primarily beer), with physiologic dependence and history of withdrawal seizures; multiple failed rehabilitation attempts; periods of denial regarding use",
 "family_history": [
  "Mother: Alzheimer disease (onset age 78)",
  "Father: Unknown, estranged",
  "No family history of colorectal cancer",
  "No family history of hematologic malignancies"
 ],
 "race_ethnicity": "White, non-Hispanic",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "10 mg PO daily",
   "name": "Donepezil"
  },
  {
   "dose": "10 mg PO twice daily",
   "name": "Memantine"
  },
  {
   "dose": "100 mg PO daily",
   "name": "Thiamine (Vitamin B1)"
  },
  {
   "dose": "1 mg PO daily",
   "name": "Folic acid"
  },
  {
   "dose": "5 mg PO twice daily",
   "name": "Ruxolitinib"
  },
  {
   "dose": "500 mg PO daily",
   "name": "Ascorbic acid (Vitamin C)"
  }
 ],
 "surgical_history": [
  "No prior major surgeries"
 ],
 "chronic_conditions": [
  "Alzheimer disease",
  "Alcohol use disorder, severe, with physiological dependence",
  "Cirrhosis with hepatocellular carcinoma (transplant evaluation candidate)",
  "Primary myelofibrosis",
  "Seizure disorder secondary to alcohol withdrawal",
  "Chronic malnutrition with vitamin deficiencies (B1, C, folate)"
 ],
 "age_at_first_encounter": 65
}
```

Primary diagnoses: ["Scurvy", "Wernicke encephalopathy", "Carbon monoxide poisoning", "Lung abscess (aspiration)", "Alzheimer disease", "Carbon monoxide poisoning without airway compromise", "Primary myelofibrosis", "Hepatocellular carcinoma or end-stage liver disease (liver transplantation candidate)"]
Comorbidities: ["Social isolation", "Protein-energy malnutrition", "Anemia", "Alcohol use disorder", "Bereavement"]

## Encounter 0: source 2020-01-15, practice 2022-08-08, outpatient, Psychiatry

Chief complaint: Brought to the clinic by his son, who reports that his father has become increasingly forgetful over the past year.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Brought to the clinic by his son, who reports that his father has become increasingly forgetful over the past year.

HISTORY OF PRESENT ILLNESS:
He has misplaced his keys multiple times, gotten lost on familiar walking routes, and recently left the stove on overnight. The patient insists these incidents are normal aging. He retired two years ago from his job as a high school principal and now spends most of his time at home reading or watching television. He used to enjoy crossword puzzles and woodworking but has recently stopped engaging in these hobbies, saying they feel "too complicated." His wife passed away three years ago, and although he initially managed well, his son feels he has become more withdrawn and less socially active over the past 12 months.

PAST MEDICAL HISTORY:
Active Problem List:
- Alzheimer disease
- Alcohol use disorder, severe, with physiological dependence
- Cirrhosis with hepatocellular carcinoma (transplant evaluation candidate)
- Primary myelofibrosis
- Seizure disorder secondary to alcohol withdrawal
- Chronic malnutrition with vitamin deficiencies (B1, C, folate)

PAST SURGICAL HISTORY:
- No prior major surgeries

MEDICATIONS:
Home Medications:
- Donepezil 10 mg PO daily
- Memantine 10 mg PO twice daily
- Thiamine (Vitamin B1) 100 mg PO daily
- Folic acid 1 mg PO daily
- Ruxolitinib 5 mg PO twice daily
- Ascorbic acid (Vitamin C) 500 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Alzheimer disease (onset age 78)
- Father: Unknown, estranged
- No family history of colorectal cancer
- No family history of hematologic malignancies

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Heavy daily consumption for 30 years (6-12 standard drinks daily, primarily beer), with physiologic dependence and history of withdrawal seizures; multiple failed rehabilitation attempts; periods of denial regarding use. Occupation: Retired high school principal.

REVIEW OF SYSTEMS:
There are no falls or balance issues reported.

VITAL SIGNS:
Vital signs are within normal limits.

PHYSICAL EXAMINATION:
The patient arrives neatly groomed and is pleasant and cooperative during the interview. General physical examination is unremarkable. Neurologic exam reveals no focal deficits. On cognitive testing, he is oriented to person and place but gives the wrong year and cannot name the current president. He recalls only 1 of 3 objects after 5 minutes and struggles with tasks requiring visuospatial processing, such as drawing a clock or copying a cube. He also has difficulty following multi-step instructions and becomes mildly frustrated during the testing.

ASSESSMENT AND PLAN:
A diagnosis of early-stage dementia is suspected. Dysregulation of

## Encounter 1: source 2020-04-15, practice 2022-10-27, outpatient, Hematology

Chief complaint: Progressive fatigue and early satiety with unintentional weight loss.

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Hematology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue and early satiety with unintentional weight loss.

HISTORY OF PRESENT ILLNESS:
The patient returns for hematology follow-up three months after his initial psychiatry evaluation for cognitive concerns. He reports that over the past few months he has felt increasingly tired and has lost 4 kg (9 lb) unintentionally. He also reports abdominal discomfort and feels full quickly during meals. He denies fever, night sweats, or recent infections.

PAST MEDICAL HISTORY:
Active Problem List:
- Alzheimer disease (diagnosed 2020-01-15)
- Alcohol use disorder, severe, with physiological dependence
- Cirrhosis with hepatocellular carcinoma (transplant evaluation candidate)
- Primary myelofibrosis
- Seizure disorder secondary to alcohol withdrawal
- Chronic malnutrition with vitamin deficiencies (B1, C, folate)

PAST SURGICAL HISTORY:
- No prior major surgeries

MEDICATIONS:
Home Medications:
- Donepezil 10 mg PO daily
- Memantine 10 mg PO twice daily
- Thiamine (Vitamin B1) 100 mg PO daily
- Folic acid 1 mg PO daily
- Ruxolitinib 5 mg PO twice daily
- Ascorbic acid (Vitamin C) 500 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Alzheimer disease (onset age 78)
- Father: Unknown, estranged
- No family history of colorectal cancer
- No family history of hematologic malignancies

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Heavy daily consumption for 30 years (6-12 standard drinks daily, primarily beer), with physiologic dependence and history of withdrawal seizures; multiple failed rehabilitation attempts; periods of denial regarding use. Occupation: Retired high school principal.

VITAL SIGNS:
His temperature is 36.8°C (98.2°F), blood pressure is 130/78 mmHg, pulse is 82/min, and respirations are 16/min

PHYSICAL EXAMINATION:
Cardiopulmonary examination is unremarkable. Abdominal examination shows a firm mass below the left costal margin. There is no lymphadenopathy or skin rash.

LABORATORY DATA:
Hemoglobin 9.0 g/dL, Mean corpuscular volume 85 fL, WBCs 11,500 /mm³, Platelet count 95,000 /mm³

## Encounter 2: source 2020-07-15, practice 2023-01-23, outpatient, Gastroenterology

Chief complaint: Unintentional weight loss and early satiety with vague right upper quadrant discomfort.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Emily Watson, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Unintentional weight loss and early satiety with vague right upper quadrant discomfort.

HISTORY OF PRESENT ILLNESS:
The patient is a 65-year-old male with Alzheimer disease diagnosed in January 2020, primary myelofibrosis evaluated in April 2020, and cirrhosis who presents to the Gastroenterology clinic for evaluation of unintentional weight loss and early satiety over the past few months. He has also noticed vague right upper quadrant discomfort that has slowly worsened. He denies fever, jaundice, or change in bowel habits. His last visit was over a year ago, and he has not undergone any recent imaging.

PAST MEDICAL HISTORY:
Active Problem List:
- Alzheimer disease (diagnosed 2020-01-15)
- Primary myelofibrosis (diagnosed 2020-04-15)
- Alcohol use disorder, severe, with physiological dependence
- Cirrhosis with hepatocellular carcinoma (transplant evaluation candidate)
- Seizure disorder secondary to alcohol withdrawal
- Chronic malnutrition with vitamin deficiencies (B1, C, folate)

PAST SURGICAL HISTORY:
- No prior major surgeries

MEDICATIONS:
Home Medications:
- Donepezil 10 mg PO daily
- Memantine 10 mg PO twice daily
- Thiamine (Vitamin B1) 100 mg PO daily
- Folic acid 1 mg PO daily
- Ruxolitinib 5 mg PO twice daily
- Ascorbic acid (Vitamin C) 500 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Alzheimer disease (onset age 78)
- Father: Unknown, estranged
- No family history of colorectal cancer
- No family history of hematologic malignancies

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Heavy daily consumption for 30 years (6-12 standard drinks daily, primarily beer), with physiologic dependence and history of withdrawal seizures; multiple failed rehabilitation attempts; periods of denial regarding use. Occupation: Retired high school principal.

PHYSICAL EXAMINATION:
On exam, he has mild hepatomegaly without tenderness.

LABORATORY DATA:
Laboratory testing reveals a mild normocytic anemia and elevated alkaline phosphatase. Liver function tests are otherwise within normal limits.

IMAGING:
A contrast-enhanced CT scan of the abdomen is obtained and shows a single, large, heterogeneous mass in the right lobe of the liver.

PATHOLOGY:
The liver findings in this patient are similar to those shown in the representative gross pathology specimen below.

## Encounter 3: source 2021-01-15, practice 2023-07-24, outpatient, Internal Medicine

Chief complaint: Increasing confusion and lethargy over the past week with spontaneous bruising on extremities.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. James Park, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Increasing confusion and lethargy over the past week with spontaneous bruising on extremities.

HISTORY OF PRESENT ILLNESS:
The patient with Alzheimer disease, primary myelofibrosis, and cirrhosis returns for follow-up assessment. His daughter reports that he has been eating mostly canned food and toast, and that he has become progressively withdrawn and forgetful. She notes that he has had spontaneous bruising on his arms and legs for the past month.

PAST MEDICAL HISTORY:
Active Problem List:
- Alzheimer disease (diagnosed 2020-01-15)
- Primary myelofibrosis (diagnosed 2020-04-15)
- Hepatocellular carcinoma or end-stage liver disease (liver transplantation candidate) (diagnosed 2020-07-15)
- Alcohol use disorder, severe, with physiological dependence
- Cirrhosis with hepatocellular carcinoma (transplant evaluation candidate)
- Seizure disorder secondary to alcohol withdrawal
- Chronic malnutrition with vitamin deficiencies (B1, C, folate)

The patient has a history of chronic alcohol use.

PAST SURGICAL HISTORY:
- No prior major surgeries

MEDICATIONS:
Home Medications:
- Donepezil 10 mg PO daily
- Memantine 10 mg PO twice daily
- Thiamine (Vitamin B1) 100 mg PO daily
- Folic acid 1 mg PO daily
- Ruxolitinib 5 mg PO twice daily
- Ascorbic acid (Vitamin C) 500 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Alzheimer disease (onset age 78)
- Father: Unknown, estranged
- No family history of colorectal cancer
- No family history of hematologic malignancies

SOCIAL HISTORY:
He has recently been living alone after the death of his spouse

PHYSICAL EXAMINATION:
On physical examination, the patient appears disheveled, with multiple ecchymoses over his forearms and shins. He is oriented only to person. Neurological exam shows no focal deficits. Skin is dry and shows scattered perifollicular hemorrhages. Oral exam reveals swollen gums that bleed on contact.

LABORATORY DATA:
Hemoglobin                                        9.8 g/dL
Platelets                                              210,000/mm³
Leukocytes                                          6,200/mm³
Prothrombin time                                12.1 sec
aPTT                                                     29 sec
Serum creatinine                                0.9 mg/dL
Serum albumin                                     3.8 g/dL

## Encounter 4: source 2021-07-15, practice 2024-01-22, outpatient, Neurology

Chief complaint: Worsening memory problems and confabulation six weeks after hospitalization for acute confusion.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Lisa Thompson, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening memory problems and confabulation six weeks after hospitalization for acute confusion.

HISTORY OF PRESENT ILLNESS:
The patient is a 66-year-old male with Alzheimer disease who returns to the neurology clinic for reassessment of cognitive symptoms. He was hospitalized six weeks ago for severe confusion and unsteady gait. Since discharge, he has remained forgetful and often repeats the same questions. He frequently tells elaborate stories about recent events that his sister claims are implausible. She also reports that he failed to recognize his nephew during a recent family visit, despite previously being very close to him.

PAST MEDICAL HISTORY:
Active Problem List:
- Alzheimer disease (diagnosed 2020-01-15)
- Primary myelofibrosis (diagnosed 2020-04-15)
- Hepatocellular carcinoma or end-stage liver disease (liver transplantation candidate) (diagnosed 2020-07-15)
- Scurvy (diagnosed 2021-01-15)
- Alcohol use disorder, severe, with physiological dependence
- Cirrhosis with hepatocellular carcinoma (transplant evaluation candidate)
- Seizure disorder secondary to alcohol withdrawal
- Chronic malnutrition with vitamin deficiencies (B1, C, folate)

PAST SURGICAL HISTORY:
- No prior major surgeries

MEDICATIONS:
He does not take any medications.

Home Medications:
- Donepezil 10 mg PO daily
- Memantine 10 mg PO twice daily
- Thiamine (Vitamin B1) 100 mg PO daily
- Folic acid 1 mg PO daily
- Ruxolitinib 5 mg PO twice daily
- Ascorbic acid (Vitamin C) 500 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Alzheimer disease (onset age 78)
- Father: Unknown, estranged
- No family history of colorectal cancer
- No family history of hematologic malignancies

SOCIAL HISTORY:
He has a 30-year history of heavy daily alcohol intake His nutritional intake is poor, and he often skips meals, subsisting mostly on canned beer and occasional snacks. He was briefly enrolled in a rehabilitation program but left after two weeks against medical advice.

VITAL SIGNS:
The patient is afebrile, with a blood pressure of 125/76 mm Hg, pulse of 88/min, and respiratory rate of 15/min.

PHYSICAL EXAMINATION:
On mental status examination, he is alert but disoriented to date and place. He is unable to recall three objects after five minutes and does not seem to recognize family photographs. Neurologic examination reveals a broad-based gait and horizontal nystagmus.

IMAGING:
MRI of the brain reveals bilateral atrophy of the mammillary bodies.

## Encounter 5: source 2022-01-15, practice 2024-07-22, ed, Emergency Medicine

Chief complaint: Persistent productive cough for several weeks with fatigue, low-grade fevers, and unintentional weight loss.

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Persistent productive cough for several weeks with fatigue, low-grade fevers, and unintentional weight loss.

HISTORY OF PRESENT ILLNESS:
This 67-year-old male with Alzheimer disease, alcohol use disorder, cirrhosis with hepatocellular carcinoma, primary myelofibrosis, and chronic malnutrition complicated by prior scurvy and Wernicke encephalopathy is brought to the emergency department by his daughter due to persistent productive cough for the past several weeks. He reports fatigue, low-grade fevers, and unintentional weight loss of about 5 pounds. His daughter says he occasionally wakes up coughing at night and has been coughing up thick sputum, which he describes as yellowish and sometimes with a bad taste. He also complains of occasional right-sided chest discomfort that worsens when he coughs.

PAST MEDICAL HISTORY:
Active Problem List:
- Alzheimer disease (diagnosed 2020-01-15)
- Primary myelofibrosis (diagnosed 2020-04-15)
- Hepatocellular carcinoma or end-stage liver disease (liver transplantation candidate) (diagnosed 2020-07-15)
- Scurvy (diagnosed 2021-01-15)
- Wernicke encephalopathy (diagnosed 2021-07-15)
- Alcohol use disorder, severe, with physiological dependence
- Cirrhosis with hepatocellular carcinoma (transplant evaluation candidate)
- Seizure disorder secondary to alcohol withdrawal
- Chronic malnutrition with vitamin deficiencies (B1, C, folate)

He has a long-standing history of alcohol use disorder and was hospitalized 8 months ago after a generalized tonic-clonic seizure.

PAST SURGICAL HISTORY:
- No prior major surgeries

MEDICATIONS:
Home Medications:
- Donepezil 10 mg PO daily
- Memantine 10 mg PO twice daily
- Thiamine (Vitamin B1) 100 mg PO daily
- Folic acid 1 mg PO daily
- Ruxolitinib 5 mg PO twice daily
- Ascorbic acid (Vitamin C) 500 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Alzheimer disease (onset age 78)
- Father: Unknown, estranged
- No family history of colorectal cancer
- No family history of hematologic malignancies

SOCIAL HISTORY:
He lives alone

VITAL SIGNS:
His temperature is 37.7°C (99.9°F), blood pressure is 112/64 mmHg, pulse is 96/min, and oxygen saturation is 96% on room air.

PHYSICAL EXAMINATION:
He has poor dentition with multiple missing teeth noted on exam. Pulmonary exam reveals decreased breath sounds over the right lower lung field with occasional crackles. Cardiac exam is normal. No clubbing or cyanosis is noted.

LABORATORY DATA:
Laboratory studies show mild leukocytosis.

IMAGING:
A chest x-ray is obtained and shown below.

## Encounter 6: source 2023-01-15, practice 2025-07-26, ed, Emergency Medicine

Chief complaint: Found confused with headache and chest discomfort in mid-January.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Maria Garcia, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Found confused with headache and chest discomfort in mid-January.

HISTORY OF PRESENT ILLNESS:
The patient, with established Alzheimer disease, severe alcohol use disorder with physiological dependence, cirrhosis with hepatocellular carcinoma under transplant evaluation, primary myelofibrosis, and history of Wernicke encephalopathy and scurvy from chronic malnutrition, is brought to the emergency department in mid-January after his neighbor found him confused and complaining of a headache and chest discomfort. He denies cough or fever but reports dizziness and vague chest pressure that began earlier in the day. There is no history of trauma, and no recent medication or substance changes.

PAST MEDICAL HISTORY:
Active Problem List:
- Alzheimer disease (diagnosed 2020-01-15)
- Primary myelofibrosis (diagnosed 2020-04-15)
- Hepatocellular carcinoma or end-stage liver disease (liver transplantation candidate) (diagnosed 2020-07-15)
- Scurvy (diagnosed 2021-01-15)
- Wernicke encephalopathy (diagnosed 2021-07-15)
- Lung abscess (aspiration) (diagnosed 2022-01-15)
- Alcohol use disorder, severe, with physiological dependence
- Cirrhosis with hepatocellular carcinoma (transplant evaluation candidate)
- Seizure disorder secondary to alcohol withdrawal
- Chronic malnutrition with vitamin deficiencies (B1, C, folate)

PAST SURGICAL HISTORY:
- No prior major surgeries

MEDICATIONS:
Home Medications:
- Donepezil 10 mg PO daily
- Memantine 10 mg PO twice daily
- Thiamine (Vitamin B1) 100 mg PO daily
- Folic acid 1 mg PO daily
- Ruxolitinib 5 mg PO twice daily
- Ascorbic acid (Vitamin C) 500 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Alzheimer disease (onset age 78)
- Father: Unknown, estranged
- No family history of colorectal cancer
- No family history of hematologic malignancies

SOCIAL HISTORY:
He lives alone and uses an old gas heater in his poorly ventilated home.

VITAL SIGNS:
His oxygen saturation is 98% while breathing room air, and cardiac monitoring shows sinus tachycardia. His temperature is 36.8°C (98.2°F), blood pressure is 108/68 mm Hg, heart rate is 102 beats per minute, and respiratory rate is 20 breaths per minute.

PHYSICAL EXAMINATION:
He appears flushed and mildly agitated.
Physical exam reveals no focal neurologic deficits.

IMAGING:
Chest x-ray is unremarkable.

PLAN:
The emergency team initiates high-flow oxygen via non-rebreather mask.

## Encounter 7: source 2024-01-15, practice 2026-07-25, ed, Emergency Medicine

Chief complaint: Confused with headache, chest discomfort, and dizziness in mid-January.

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Maria Garcia, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Confused with headache, chest discomfort, and dizziness in mid-January.

HISTORY OF PRESENT ILLNESS:
The patient, with Alzheimer disease, alcohol use disorder, cirrhosis with hepatocellular carcinoma, and primary myelofibrosis, returns to the emergency department in mid-January one year after a similar presentation, after his neighbor found him confused and complaining of a headache and chest discomfort. He denies cough or fever but reports dizziness and vague chest pressure that began earlier in the day. There is no history of trauma, and no recent medication or substance changes.

PAST MEDICAL HISTORY:
Active Problem List:
- Alzheimer disease (diagnosed 2020-01-15)
- Primary myelofibrosis (diagnosed 2020-04-15)
- Hepatocellular carcinoma or end-stage liver disease (liver transplantation candidate) (diagnosed 2020-07-15)
- Scurvy (diagnosed 2021-01-15)
- Wernicke encephalopathy (diagnosed 2021-07-15)
- Lung abscess (aspiration) (diagnosed 2022-01-15)
- Carbon monoxide poisoning (diagnosed 2023-01-15)
- Alcohol use disorder, severe, with physiological dependence
- Cirrhosis with hepatocellular carcinoma (transplant evaluation candidate)
- Seizure disorder secondary to alcohol withdrawal
- Chronic malnutrition with vitamin deficiencies (B1, C, folate)

PAST SURGICAL HISTORY:
- No prior major surgeries

MEDICATIONS:
Home Medications:
- Donepezil 10 mg PO daily
- Memantine 10 mg PO twice daily
- Thiamine (Vitamin B1) 100 mg PO daily
- Folic acid 1 mg PO daily
- Ruxolitinib 5 mg PO twice daily
- Ascorbic acid (Vitamin C) 500 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Alzheimer disease (onset age 78)
- Father: Unknown, estranged
- No family history of colorectal cancer
- No family history of hematologic malignancies

SOCIAL HISTORY:
He lives alone and uses an old gas heater in his poorly ventilated home.

VITAL SIGNS:
His oxygen saturation is 98% while breathing room air, and cardiac monitoring shows sinus tachycardia. His temperature is 36.8°C (98.2°F), blood pressure is 108/68 mm Hg, heart rate is 102 beats per minute, and respiratory rate is 20 breaths per minute.

PHYSICAL EXAMINATION:
He appears flushed and mildly agitated. Physical exam reveals no focal neurologic deficits.

IMAGING:
Chest x-ray is unremarkable.

PLAN:
The emergency team initiates high-flow oxygen via non-rebreather mask.
