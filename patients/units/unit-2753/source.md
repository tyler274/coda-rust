# synthetic_hospital patient 2753

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex F, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-08-24 (Mon) | outpatient | Psychiatry |
| 1 | 2020-04-15 | 2020-12-08 (Tue) | outpatient | Neurology |
| 2 | 2020-09-15 | 2021-05-10 (Mon) | outpatient | Pain Management |
| 3 | 2021-03-15 | 2021-10-25 (Mon) | outpatient | Orthopedics |
| 4 | 2022-03-15 | 2022-11-04 (Fri) | ed | Emergency Medicine |
| 5 | 2023-03-15 | 2023-10-29 (Sun) | ed | Emergency Medicine |
| 6 | 2024-03-15 | 2024-10-25 (Fri) | ed | Psychiatry |
| 7 | 2025-03-15 | 2025-11-09 (Sun) | icu | Internal Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NSAIDs (ineffective and poor gastrointestinal tolerance)",
  "Gabapentin (ineffective for neuropathic pain)"
 ],
 "insurance": "Medicaid",
 "occupation": "Unemployed (disability leave); former administrative assistant",
 "alcohol_use": "Social (1-2 cocktails on weekends)",
 "family_history": [
  "Mother: Major depressive disorder, type 2 diabetes",
  "Father: Crohn's disease",
  "Sister: Generalized anxiety disorder (primary support person)",
  "Maternal aunt: Deep vein thrombosis"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former light smoker (quit 3 months ago; previously 1 pack per week)",
 "home_medications": [
  {
   "dose": "100 mg oral daily",
   "name": "Sertraline"
  },
  {
   "dose": "150 mg oral daily",
   "name": "Bupropion XL"
  },
  {
   "dose": "0.25 mg oral PRN anxiety (patient reports occasional use of 2 tablets at night)",
   "name": "Alprazolam"
  },
  {
   "dose": "25 mg oral nightly as needed for sleep",
   "name": "Diphenhydramine"
  },
  {
   "dose": "1 tablet oral daily (combined oral contraceptive)",
   "name": "Ethinyl estradiol/norethindrone"
  },
  {
   "dose": "1 tablet oral daily",
   "name": "Multivitamin"
  },
  {
   "dose": "Infusion 12 hours nightly via central line",
   "name": "Total parenteral nutrition (TPN)"
  }
 ],
 "surgical_history": [
  "Multiple prior small bowel resections for Crohn's disease",
  "Laparoscopic cholecystectomy",
  "Right knee arthroscopy",
  "Insertion of tunneled central venous catheter for TPN"
 ],
 "chronic_conditions": [
  "Crohn's disease with short bowel syndrome",
  "Total parenteral nutrition (TPN) dependence",
  "Major depressive disorder, recurrent, severe",
  "Generalized anxiety disorder",
  "Chronic mechanical lower back pain",
  "Temporal lobe epilepsy",
  "History of pulmonary embolism (provoked, post-operative)",
  "Status post laparoscopic cholecystectomy",
  "Status post right knee arthroscopy"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Chronic mechanical lower back pain", "Central pontine myelinolysis", "Acute cholecystitis", "Pulmonary embolism", "Phrenic nerve injury", "Major Depressive Disorder with Psychotic Features", "Temporal lobe epilepsy", "Major depressive disorder"]
Comorbidities: ["Major depressive disorder", "Status post laparoscopic cholecystectomy"]

## Encounter 0: source 2020-01-15, practice 2020-08-24, outpatient, Psychiatry

Chief complaint: Follow-up appointment for persistent depressive symptoms.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up appointment for persistent depressive symptoms.

HISTORY OF PRESENT ILLNESS:
She reports a 4-month history of persistent sadness, loss of interest in activities she previously enjoyed, and difficulty concentrating at work. She also mentions significant weight loss, insomnia, and feelings of worthlessness.

PAST MEDICAL HISTORY:
Active Problem List:
- Crohn's disease with short bowel syndrome
- Total parenteral nutrition (TPN) dependence
- Major depressive disorder, recurrent, severe
- Generalized anxiety disorder
- Chronic mechanical lower back pain
- Temporal lobe epilepsy
- History of pulmonary embolism (provoked, post-operative)
- Status post laparoscopic cholecystectomy
- Status post right knee arthroscopy

She has a history of two prior major depressive episodes, one in her late 20s and another in her mid-30s, both of which required pharmacological intervention.

PAST SURGICAL HISTORY:
- Multiple prior small bowel resections for Crohn's disease
- Laparoscopic cholecystectomy
- Right knee arthroscopy
- Insertion of tunneled central venous catheter for TPN

MEDICATIONS:
Home Medications:
- Sertraline 100 mg oral daily
- Bupropion XL 150 mg oral daily
- Alprazolam 0.25 mg oral PRN anxiety (patient reports occasional use of 2 tablets at night)
- Diphenhydramine 25 mg oral nightly as needed for sleep
- Ethinyl estradiol/norethindrone 1 tablet oral daily (combined oral contraceptive)
- Multivitamin 1 tablet oral daily
- Total parenteral nutrition (TPN) Infusion 12 hours nightly via central line

ALLERGIES:
NSAIDs (ineffective and poor gastrointestinal tolerance), Gabapentin (ineffective for neuropathic pain)

FAMILY HISTORY:
Her family history is notable for depression in her mother.

SOCIAL HISTORY:
She denies any current substance abuse.

PHYSICAL EXAMINATION:
Her physical examination is unremarkable.

LABORATORY DATA:
Her most recent laboratory tests are as follows:

- CBC: within normal limits
- Thyroid-stimulating hormone (TSH): within normal limits
- Basic metabolic panel: within normal limits

## Encounter 1: source 2020-04-15, practice 2020-12-08, outpatient, Neurology

Chief complaint: Evaluation of strange staring episode that occurred earlier in the week.

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Patel, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Evaluation of strange staring episode that occurred earlier in the week.

HISTORY OF PRESENT ILLNESS:
The patient returns for neurologic evaluation of a recent staring spell, with prior Psychiatry follow-up three months ago for persistent depressive symptoms. She carries a history of temporal lobe epilepsy, Crohn's disease with short bowel syndrome requiring TPN, and recurrent major depression. While sitting at the kitchen table earlier this week, she suddenly stopped talking and stared into space for about 45 seconds. She was unresponsive during the episode and could not recall it afterward, though she seemed confused for several minutes. Her sister denies any shaking or loss of consciousness. She reports recent trouble concentrating and feeling "slowed down," which she attributed to her mood.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder (diagnosed 2020-01-15)
- Crohn's disease with short bowel syndrome
- Total parenteral nutrition (TPN) dependence
- Major depressive disorder, recurrent, severe
- Generalized anxiety disorder
- Chronic mechanical lower back pain
- Temporal lobe epilepsy
- History of pulmonary embolism (provoked, post-operative)
- Status post laparoscopic cholecystectomy
- Status post right knee arthroscopy

The patient has a history of major depressive disorder and generalized anxiety disorder.

PAST SURGICAL HISTORY:
- Multiple prior small bowel resections for Crohn's disease
- Laparoscopic cholecystectomy
- Right knee arthroscopy
- Insertion of tunneled central venous catheter for TPN

MEDICATIONS:
Her current medications include bupropion, sertraline, alprazolam as needed, and over-the-counter diphenhydramine for sleep.

Home Medications:
- Bupropion XL 150 mg oral daily
- Ethinyl estradiol/norethindrone 1 tablet oral daily (combined oral contraceptive)
- Multivitamin 1 tablet oral daily
- Total parenteral nutrition (TPN) Infusion 12 hours nightly via central line

ALLERGIES:
NSAIDs (ineffective and poor gastrointestinal tolerance), Gabapentin (ineffective for neuropathic pain)

FAMILY HISTORY:
- Mother: Major depressive disorder, type 2 diabetes
- Father: Crohn's disease
- Sister: Generalized anxiety disorder (primary support person)
- Maternal aunt: Deep vein thrombosis

SOCIAL HISTORY:
She denies alcohol use but admits to sometimes taking more than one alprazolam tablet at night when she can't sleep.

VITAL SIGNS:
Vital signs are normal.

PHYSICAL EXAMINATION:
On physical examination, she is oriented but has a flat affect and pauses often during the interview. Her neurologic exam is non-focal.

LABORATORY DATA:
Labs, including CBC, BMP, TSH, and B12, are normal.

## Encounter 2: source 2020-09-15, practice 2021-05-10, outpatient, Pain Management

Chief complaint: Exacerbation of lower back pain described as unbearable despite multiple analgesics.

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. James O'Brien, MD
Department: Pain Management
Visit Type: outpatient

CHIEF COMPLAINT:
Exacerbation of lower back pain described as unbearable despite multiple analgesics.

HISTORY OF PRESENT ILLNESS:
The patient, with Crohn's disease, short bowel syndrome, TPN dependence, and Psychiatry (January 2020) and Neurology (April 2020) encounters, returns for evaluation of worsening lower back pain described as unbearable despite trying "everything." She has been on multiple analgesics, including NSAIDs and a trial of gabapentin, without significant relief. She reports feeling hopeless about the possibility of improvement. She has been referred to physical therapy in the past but states it was "useless." She is tearful during the encounter and expresses frustration that "no one can help."

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder (diagnosed 2020-01-15)
- Temporal lobe epilepsy (diagnosed 2020-04-15)
- Crohn's disease with short bowel syndrome
- Total parenteral nutrition (TPN) dependence
- Major depressive disorder, recurrent, severe
- Generalized anxiety disorder
- Chronic mechanical lower back pain
- History of pulmonary embolism (provoked, post-operative)
- Status post laparoscopic cholecystectomy
- Status post right knee arthroscopy

PAST SURGICAL HISTORY:
- Multiple prior small bowel resections for Crohn's disease
- Laparoscopic cholecystectomy
- Right knee arthroscopy
- Insertion of tunneled central venous catheter for TPN

MEDICATIONS:
Home Medications:
- Sertraline 100 mg oral daily
- Bupropion XL 150 mg oral daily
- Alprazolam 0.25 mg oral PRN anxiety (patient reports occasional use of 2 tablets at night)
- Diphenhydramine 25 mg oral nightly as needed for sleep
- Ethinyl estradiol/norethindrone 1 tablet oral daily (combined oral contraceptive)
- Multivitamin 1 tablet oral daily
- Total parenteral nutrition (TPN) Infusion 12 hours nightly via central line

ALLERGIES:
NSAIDs (ineffective and poor gastrointestinal tolerance), Gabapentin (ineffective for neuropathic pain)

FAMILY HISTORY:
- Mother: Major depressive disorder, type 2 diabetes
- Father: Crohn's disease
- Sister: Generalized anxiety disorder (primary support person)
- Maternal aunt: Deep vein thrombosis

SOCIAL HISTORY:
Smoking: Former light smoker (quit 3 months ago; previously 1 pack per week). Alcohol: Social (1-2 cocktails on weekends). Occupation: Unemployed (disability leave); former administrative assistant.

PHYSICAL EXAMINATION:
Pain on palpation of the lumbar paraspinal muscles without neurological deficits

LABORATORY DATA:
CBC: within normal limits
- ESR: 20 mm/hr (normal <20 mm/hr)
- TSH: 2.5 mIU/L (normal 0.4-4.0 mIU/L)
- Vitamin D: 30 ng/mL (normal 20-50 ng/mL)

## Encounter 3: source 2021-03-15, practice 2021-10-25, outpatient, Orthopedics

Chief complaint: Six-month history of difficulty walking and instability of the right leg.

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Elena Rodriguez, MD
Department: Orthopedics
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of difficulty walking and instability of the right leg.

HISTORY OF PRESENT ILLNESS:
The patient with chronic mechanical lower back pain seen in pain management September 2020 presents with a 6-month history of difficulty walking and a sensation of instability when standing on her right leg. She reports her right hip often "gives way" when she attempts to climb stairs.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder (diagnosed 2020-01-15)
- Temporal lobe epilepsy (diagnosed 2020-04-15)
- Chronic mechanical lower back pain (diagnosed 2020-09-15)
- Crohn's disease with short bowel syndrome
- Total parenteral nutrition (TPN) dependence
- Major depressive disorder, recurrent, severe
- Generalized anxiety disorder
- History of pulmonary embolism (provoked, post-operative)
- Status post laparoscopic cholecystectomy
- Status post right knee arthroscopy

PAST SURGICAL HISTORY:
- Multiple prior small bowel resections for Crohn's disease
- Laparoscopic cholecystectomy
- Right knee arthroscopy
- Insertion of tunneled central venous catheter for TPN

MEDICATIONS:
Home Medications:
- Sertraline 100 mg oral daily
- Bupropion XL 150 mg oral daily
- Alprazolam 0.25 mg oral PRN anxiety (patient reports occasional use of 2 tablets at night)
- Diphenhydramine 25 mg oral nightly as needed for sleep
- Ethinyl estradiol/norethindrone 1 tablet oral daily (combined oral contraceptive)
- Multivitamin 1 tablet oral daily
- Total parenteral nutrition (TPN) Infusion 12 hours nightly via central line

ALLERGIES:
NSAIDs (ineffective and poor gastrointestinal tolerance), Gabapentin (ineffective for neuropathic pain)

FAMILY HISTORY:
- Mother: Major depressive disorder, type 2 diabetes
- Father: Crohn's disease
- Sister: Generalized anxiety disorder (primary support person)
- Maternal aunt: Deep vein thrombosis

SOCIAL HISTORY:
Smoking: Former light smoker (quit 3 months ago; previously 1 pack per week). Alcohol: Social (1-2 cocktails on weekends). Occupation: Unemployed (disability leave); former administrative assistant.

PHYSICAL EXAMINATION:
A slight limp in her gait. When asked to stand on her right leg, her left hemipelvis droops below the horizontal plane. There is no sensory loss, and her reflexes are intact. Strength testing reveals weakness in hip abduction on the right side.

## Encounter 4: source 2022-03-15, practice 2022-11-04, ed, Emergency Medicine

Chief complaint: Constant severe right upper quadrant pain radiating to the back with nausea and fever.

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Constant severe right upper quadrant pain radiating to the back with nausea and fever.

HISTORY OF PRESENT ILLNESS:
The patient is a 47-year-old female with Crohn's disease, short bowel syndrome, and status post laparoscopic cholecystectomy who presents to the emergency department with a 24-hour history of constant, severe right upper quadrant pain that radiates to her back. She also reports experiencing nausea and a low-grade fever. She denies changes in bowel habits or urinary symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder (diagnosed 2020-01-15)
- Temporal lobe epilepsy (diagnosed 2020-04-15)
- Chronic mechanical lower back pain (diagnosed 2020-09-15)
- Phrenic nerve injury (diagnosed 2021-03-15)
- Crohn's disease with short bowel syndrome
- Total parenteral nutrition (TPN) dependence
- Major depressive disorder, recurrent, severe
- Generalized anxiety disorder
- History of pulmonary embolism (provoked, post-operative)
- Status post laparoscopic cholecystectomy
- Status post right knee arthroscopy

PAST SURGICAL HISTORY:
- Multiple prior small bowel resections for Crohn's disease
- Laparoscopic cholecystectomy
- Right knee arthroscopy
- Insertion of tunneled central venous catheter for TPN

MEDICATIONS:
Home Medications:
- Sertraline 100 mg oral daily
- Bupropion XL 150 mg oral daily
- Alprazolam 0.25 mg oral PRN anxiety (patient reports occasional use of 2 tablets at night)
- Diphenhydramine 25 mg oral nightly as needed for sleep
- Ethinyl estradiol/norethindrone 1 tablet oral daily (combined oral contraceptive)
- Multivitamin 1 tablet oral daily
- Total parenteral nutrition (TPN) Infusion 12 hours nightly via central line

ALLERGIES:
NSAIDs (ineffective and poor gastrointestinal tolerance), Gabapentin (ineffective for neuropathic pain)

FAMILY HISTORY:
- Mother: Major depressive disorder, type 2 diabetes
- Father: Crohn's disease
- Sister: Generalized anxiety disorder (primary support person)
- Maternal aunt: Deep vein thrombosis

SOCIAL HISTORY:
Smoking: Former light smoker (quit 3 months ago; previously 1 pack per week). Alcohol: Social (1-2 cocktails on weekends). Occupation: Unemployed (disability leave); former administrative assistant.

VITAL SIGNS:
Temperature is 38.2°C (100.8°F), heart rate is 102 beats per minute, and blood pressure is 135/85 mmHg

PHYSICAL EXAMINATION:
Abdominal examination reveals right upper quadrant tenderness with a positive Murphy's sign

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 14,000 /µL
- Hemoglobin: 13.2 g/dL
- Platelets: 250,000 /µL
- Total bilirubin: 1.8 mg/dL
- Alkaline phosphatase: 120 U/L
- Aspartate aminotransferase (AST): 50 U/L
- Alanine aminotransferase (ALT): 45 U/L
- Amylase: 90 U/L
- Lipase: 85 U/L

IMAGING:
An abdominal ultrasound shows gallbladder wall thickening and pericholecystic fluid without evidence of gallstones.

## Encounter 5: source 2023-03-15, practice 2023-10-29, ed, Emergency Medicine

Chief complaint: Sudden-onset shortness of breath five days after right knee arthroscopy.

ENCOUNTER NOTE
Date: 2023-03-15
Provider: Dr. Aisha Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden-onset shortness of breath five days after right knee arthroscopy.

HISTORY OF PRESENT ILLNESS:
The patient is a 48-year-old female with Crohn's disease with short bowel syndrome on total parenteral nutrition and history of postoperative pulmonary embolism who is brought to the emergency department by her partner with sudden-onset shortness of breath. She recently underwent knee surgery 5 days ago and has been recovering at home with limited mobility.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder (diagnosed 2020-01-15)
- Temporal lobe epilepsy (diagnosed 2020-04-15)
- Chronic mechanical lower back pain (diagnosed 2020-09-15)
- Phrenic nerve injury (diagnosed 2021-03-15)
- Acute cholecystitis (diagnosed 2022-03-15)
- Crohn's disease with short bowel syndrome
- Total parenteral nutrition (TPN) dependence
- Major depressive disorder, recurrent, severe
- Generalized anxiety disorder
- History of pulmonary embolism (provoked, post-operative)
- Status post laparoscopic cholecystectomy
- Status post right knee arthroscopy

PAST SURGICAL HISTORY:
- Multiple prior small bowel resections for Crohn's disease
- Laparoscopic cholecystectomy
- Right knee arthroscopy
- Insertion of tunneled central venous catheter for TPN

MEDICATIONS:
Medications include a daily multivitamin and a combined oral contraceptive pill.

Home Medications:
- Sertraline 100 mg oral daily
- Bupropion XL 150 mg oral daily
- Alprazolam 0.25 mg oral PRN anxiety (patient reports occasional use of 2 tablets at night)
- Diphenhydramine 25 mg oral nightly as needed for sleep
- Ethinyl estradiol/norethindrone 1 tablet oral daily (combined oral contraceptive)
- Total parenteral nutrition (TPN) Infusion 12 hours nightly via central line

ALLERGIES:
NSAIDs (ineffective and poor gastrointestinal tolerance), Gabapentin (ineffective for neuropathic pain)

FAMILY HISTORY:
- Mother: Major depressive disorder, type 2 diabetes
- Father: Crohn's disease
- Sister: Generalized anxiety disorder (primary support person)
- Maternal aunt: Deep vein thrombosis

SOCIAL HISTORY:
She smokes one pack of cigarattes per week. She drinks a cocktail occasionally on the weekends.

VITAL SIGNS:
BP is 125/85. HR is 110.

## Encounter 6: source 2024-03-15, practice 2024-10-25, ed, Psychiatry

Chief complaint: Acute behavioral changes with delusions and withdrawn behavior over the past two weeks.

ENCOUNTER NOTE
Date: 2024-03-15
Provider: Dr. Robert Smith, MD
Department: Psychiatry
Visit Type: ed

CHIEF COMPLAINT:
Acute behavioral changes with delusions and withdrawn behavior over the past two weeks.

HISTORY OF PRESENT ILLNESS:
The patient, a 49-year-old female with recurrent severe major depressive disorder and temporal lobe epilepsy previously evaluated in 2020, presents with acute behavioral changes developing over the past two weeks. She has refused to leave her apartment, saying that her neighbors are “discussing her sins through the walls” and that “God is punishing her for being a terrible mother.” She cries often, says she feels “unfixable,” and believes she deserves to be in jail. She reports poor appetite, frequent early morning awakening, and low energy. She endorses passive thoughts of death but denies active suicidal plans.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder (diagnosed 2020-01-15)
- Temporal lobe epilepsy (diagnosed 2020-04-15)
- Chronic mechanical lower back pain (diagnosed 2020-09-15)
- Phrenic nerve injury (diagnosed 2021-03-15)
- Acute cholecystitis (diagnosed 2022-03-15)
- Pulmonary embolism (diagnosed 2023-03-15)
- Crohn's disease with short bowel syndrome
- Total parenteral nutrition (TPN) dependence
- Major depressive disorder, recurrent, severe
- Generalized anxiety disorder
- History of pulmonary embolism (provoked, post-operative)
- Status post laparoscopic cholecystectomy
- Status post right knee arthroscopy

Her sister states that the patient has had several episodes of depression in the past, but none this severe. There is no known history of mania, hypomania, or psychosis.

PAST SURGICAL HISTORY:
- Multiple prior small bowel resections for Crohn's disease
- Laparoscopic cholecystectomy
- Right knee arthroscopy
- Insertion of tunneled central venous catheter for TPN

MEDICATIONS:
The patient takes no psychiatric medications

Home Medications:
- Sertraline 100 mg oral daily
- Bupropion XL 150 mg oral daily
- Alprazolam 0.25 mg oral PRN anxiety (patient reports occasional use of 2 tablets at night)
- Diphenhydramine 25 mg oral nightly as needed for sleep
- Ethinyl estradiol/norethindrone 1 tablet oral daily (combined oral contraceptive)
- Multivitamin 1 tablet oral daily
- Total parenteral nutrition (TPN) Infusion 12 hours nightly via central line

ALLERGIES:
NSAIDs (ineffective and poor gastrointestinal tolerance), Gabapentin (ineffective for neuropathic pain)

FAMILY HISTORY:
- Mother: Major depressive disorder, type 2 diabetes
- Father: Crohn's disease
- Sister: Generalized anxiety disorder (primary support person)
- Maternal aunt: Deep vein thrombosis

SOCIAL HISTORY:
She has no substance use. She is unemployed and currently lives alone.

VITAL SIGNS:
Vital signs and physical examination are unremarkable.

PHYSICAL EXAMINATION:
Her affect is flat, and she avoids eye contact throughout the interview. Cognitive testing shows no signs of dementia, and neurologic examination is intact.

LABORATORY DATA:
Urine drug screen is negative.

## Encounter 7: source 2025-03-15, practice 2025-11-09, icu, Internal Medicine

Chief complaint: Altered mental status progressing to unresponsiveness during hospitalization for hyponatremia.

ENCOUNTER NOTE
Date: 2025-03-15
Provider: Dr. Fatima Al-Zahra, MD
Department: Internal Medicine
Visit Type: icu

CHIEF COMPLAINT:
Altered mental status progressing to unresponsiveness during hospitalization for hyponatremia.

HISTORY OF PRESENT ILLNESS:
This 50-year-old female with Crohn's disease and short bowel syndrome requiring TPN, temporal lobe epilepsy, and recent treatment for major depressive disorder with psychotic features presents with acute neurological decline. She is admitted after being found confused and lethargic. She is treated with intravenous fluids. Over the next 48 hours, her sodium rises to 129 mEq/L. On hospital day 3, she becomes unresponsive and is transferred to the ICU.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder (diagnosed 2020-01-15)
- Temporal lobe epilepsy (diagnosed 2020-04-15)
- Chronic mechanical lower back pain (diagnosed 2020-09-15)
- Phrenic nerve injury (diagnosed 2021-03-15)
- Acute cholecystitis (diagnosed 2022-03-15)
- Pulmonary embolism (diagnosed 2023-03-15)
- Major Depressive Disorder with Psychotic Features (diagnosed 2024-03-15)
- Crohn's disease with short bowel syndrome
- Total parenteral nutrition (TPN) dependence
- Major depressive disorder, recurrent, severe
- Generalized anxiety disorder
- History of pulmonary embolism (provoked, post-operative)
- Status post laparoscopic cholecystectomy
- Status post right knee arthroscopy

PAST SURGICAL HISTORY:
- Multiple prior small bowel resections for Crohn's disease
- Laparoscopic cholecystectomy
- Right knee arthroscopy
- Insertion of tunneled central venous catheter for TPN

MEDICATIONS:
Home Medications:
- Sertraline 100 mg oral daily
- Bupropion XL 150 mg oral daily
- Alprazolam 0.25 mg oral PRN anxiety (patient reports occasional use of 2 tablets at night)
- Diphenhydramine 25 mg oral nightly as needed for sleep
- Ethinyl estradiol/norethindrone 1 tablet oral daily (combined oral contraceptive)
- Multivitamin 1 tablet oral daily
- Total parenteral nutrition (TPN) Infusion 12 hours nightly via central line

ALLERGIES:
NSAIDs (ineffective and poor gastrointestinal tolerance), Gabapentin (ineffective for neuropathic pain)

FAMILY HISTORY:
- Mother: Major depressive disorder, type 2 diabetes
- Father: Crohn's disease
- Sister: Generalized anxiety disorder (primary support person)
- Maternal aunt: Deep vein thrombosis

SOCIAL HISTORY:
Smoking: Former light smoker (quit 3 months ago; previously 1 pack per week). Alcohol: Social (1-2 cocktails on weekends). Occupation: Unemployed (disability leave); former administrative assistant.

PHYSICAL EXAMINATION:
On examination, she opens her eyes to pain and can blink to command but is otherwise motionless. She does not speak or move her limbs. Pupillary reflexes are intact.

LABORATORY DATA:
Initial labs show a serum sodium level of 112 mEq/L. Over the next 48 hours, her sodium rises to 129 mEq/L.
