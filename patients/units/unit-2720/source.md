# synthetic_hospital patient 2720

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 44 at first encounter, sex F, Non-Hispanic White, Commercial preferred provider organization (PPO).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-04-19 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-04-15 | 2021-07-29 (Thu) | outpatient | Endocrinology |
| 2 | 2020-10-15 | 2022-01-20 (Thu) | ed | Emergency Medicine |
| 3 | 2021-07-15 | 2022-10-27 (Thu) | outpatient | Ophthalmology |
| 4 | 2022-07-15 | 2023-10-13 (Fri) | ed | Emergency Medicine |
| 5 | 2023-01-15 | 2024-04-22 (Mon) | ed | Emergency Medicine |
| 6 | 2024-01-15 | 2025-04-20 (Sun) | ed | Emergency Medicine |
| 7 | 2025-01-15 | 2026-04-18 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Sulfonamide antibiotics (e.g., trimethoprim-sulfamethoxazole) - Toxic Epidermal Necrolysis",
  "Pollen (seasonal)"
 ],
 "insurance": "Commercial preferred provider organization (PPO)",
 "occupation": "Administrative assistant",
 "alcohol_use": "None",
 "family_history": [
  "Familial hyperaldosteronism type I (autosomal dominant) in father and paternal grandmother",
  "Premature coronary artery disease (father, myocardial infarction at age 48)",
  "Type 2 diabetes mellitus (mother)",
  "Age-related macular degeneration (maternal grandmother)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1000 mg PO twice daily",
   "name": "Metformin"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg PO nightly",
   "name": "Atorvastatin"
  },
  {
   "dose": "0.5 mg PO daily",
   "name": "Dexamethasone"
  },
  {
   "dose": "10 cm H2O nightly",
   "name": "Continuous Positive Airway Pressure (CPAP)"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Hypertension",
  "Type 2 Diabetes Mellitus",
  "Hyperlipidemia",
  "Morbid Obesity (Class III)",
  "Obstructive Sleep Apnea",
  "Glucocorticoid-Remediable Aldosteronism (Familial Hyperaldosteronism Type I)",
  "Age-related Macular Degeneration",
  "Seasonal Allergic Rhinitis"
 ],
 "age_at_first_encounter": 44
}
```

Primary diagnoses: ["ST-segment elevation myocardial infarction, inferior wall", "Morbid obesity", "Glucocorticoid-Remediable Aldosteronism (Familial Hyperaldosteronism Type I)", "Age-related macular degeneration", "Choledocholithiasis with cholangitis", "Acute cholecystitis", "Choledocholithiasis", "Toxic epidermal necrolysis"]
Comorbidities: ["Cardiovascular disease risk factors", "Obesity", "Hyperlipidemia", "Essential hypertension", "Cholelithiasis"]

## Encounter 0: source 2020-01-15, practice 2021-04-19, outpatient, Internal Medicine

Chief complaint: presents to the clinic for a weight management consultation

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
She presents to the clinic for a weight management consultation

HISTORY OF PRESENT ILLNESS:
She reports a 5-year history of unsuccessful weight loss attempts, including dietary changes, exercise, and the use of weight-loss medications.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Morbid Obesity (Class III)
- Obstructive Sleep Apnea
- Glucocorticoid-Remediable Aldosteronism (Familial Hyperaldosteronism Type I)
- Age-related Macular Degeneration
- Seasonal Allergic Rhinitis

MEDICATIONS:
Her current medications include metformin, lisinopril, and continuous positive airway pressure therapy at night.

Home Medications:
- Atorvastatin 40 mg PO nightly
- Dexamethasone 0.5 mg PO daily
- Continuous Positive Airway Pressure (CPAP) 10 cm H2O nightly

ALLERGIES:
Sulfonamide antibiotics (e.g., trimethoprim-sulfamethoxazole) - Toxic Epidermal Necrolysis, Pollen (seasonal)

FAMILY HISTORY:
Her family history is significant for obesity and early-onset coronary artery disease.

SOCIAL HISTORY:
She does not smoke or consume alcohol.

VITAL SIGNS:
On examination, her blood pressure is 142/88 mmHg, heart rate is 78 bpm, and her BMI is calculated to be 37 kg/m^2.

LABORATORY DATA:
Laboratory results are as follows:

- Hemoglobin A1c: 7.8%
- Fasting plasma glucose: 172 mg/dL
- Total cholesterol: 220 mg/dL
- HDL cholesterol: 35 mg/dL
- LDL cholesterol: 145 mg/dL
- Triglycerides: 250 mg/dL

## Encounter 1: source 2020-04-15, practice 2021-07-29, outpatient, Endocrinology

Chief complaint: muscle weakness, frequent urination, and difficulty concentrating

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Ahmed Hassan, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Muscle weakness, frequent urination, and difficulty concentrating

HISTORY OF PRESENT ILLNESS:
The patient is a 44-year-old female with Glucocorticoid-Remediable Aldosteronism, Type 2 Diabetes Mellitus, and morbid obesity who returns for follow-up following her weight management consultation in January. She presents with muscle weakness, frequent urination, and difficulty concentrating for the past month.

PAST MEDICAL HISTORY:
Active Problem List:
- Morbid obesity (diagnosed 2020-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Morbid Obesity (Class III)
- Obstructive Sleep Apnea
- Glucocorticoid-Remediable Aldosteronism (Familial Hyperaldosteronism Type I)
- Age-related Macular Degeneration
- Seasonal Allergic Rhinitis

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO nightly
- Dexamethasone 0.5 mg PO daily
- Continuous Positive Airway Pressure (CPAP) 10 cm H2O nightly

ALLERGIES:
Sulfonamide antibiotics (e.g., trimethoprim-sulfamethoxazole) - Toxic Epidermal Necrolysis, Pollen (seasonal)

FAMILY HISTORY:
Her sister and mother also have hypertension.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative assistant.

VITAL SIGNS:
Blood pressure is 146/92 mmHg

LABORATORY DATA:
Serum sodium of 140 mEq/L, potassium of 2.9 mEq/L, bicarbonate of 33 mEq/L, and chloride of 95 mEq/L. Arterial pH is 7.51.

## Encounter 2: source 2020-10-15, practice 2022-01-20, ed, Emergency Medicine

Chief complaint: presents to the emergency department with rapidly worsening skin pain and widespread areas of peeling over the past 2 days

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. James Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
She presents to the emergency department with rapidly worsening skin pain and widespread areas of peeling over the past 2 days

HISTORY OF PRESENT ILLNESS:
The patient is a 44-year-old female with a history of glucocorticoid-remediable aldosteronism diagnosed in April 2020 and morbid obesity evaluated in January 2020 who presents to the emergency department with rapidly worsening skin pain and widespread areas of peeling over the past 2 days. She reports that she initially developed a low-grade fever and sore throat approximately one week ago and was prescribed an oral medication at an urgent care clinic. Three days after starting the medication, she developed a diffuse, erythematous rash across her trunk and face that progressed to involve her upper extremities. Over the last 24 hours, she has had increasing pain in her mouth, eyes, and genital area, with difficulty eating, photophobia, and burning on urination. She also reports malaise, chills, and generalized body aches. She denies recent travel, new foods, or known environmental exposures.

PAST MEDICAL HISTORY:
Active Problem List:
- Morbid obesity (diagnosed 2020-01-15)
- Glucocorticoid-Remediable Aldosteronism (Familial Hyperaldosteronism Type I) (diagnosed 2020-04-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Morbid Obesity (Class III)
- Obstructive Sleep Apnea
- Age-related Macular Degeneration
- Seasonal Allergic Rhinitis

Her past medical history includes well-controlled hypertension and seasonal allergic rhinitis.

MEDICATIONS:
Medications include hydrochlorothiazide and a recently prescribed antibiotic.

Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO nightly
- Dexamethasone 0.5 mg PO daily
- Continuous Positive Airway Pressure (CPAP) 10 cm H2O nightly

ALLERGIES:
Sulfonamide antibiotics (e.g., trimethoprim-sulfamethoxazole) - Toxic Epidermal Necrolysis, Pollen (seasonal)

FAMILY HISTORY:
- Familial hyperaldosteronism type I (autosomal dominant) in father and paternal grandmother
- Premature coronary artery disease (father, myocardial infarction at age 48)
- Type 2 diabetes mellitus (mother)
- Age-related macular degeneration (maternal grandmother)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative assistant.

VITAL SIGNS:
On examination, her temperature is 38.9°C (102.0°F), blood pressure is 102/66 mm Hg, pulse is 112/min, and respirations are 20/min.

PHYSICAL EXAMINATION:
Her face appears erythematous and swollen, with crusting at the lips and multiple erosions inside the oral mucosa. There is bilateral conjunctival injection without discharge. Her chest and back reveal coalescing erythematous macules with flaccid bullae and areas of skin sloughing on gentle pressure. Similar findings are noted on the arms and upper thighs. The genital mucosa appears erythematous and denuded in some areas. There is no lymphadenopathy.

LABORATORY DATA:
Laboratory studies reveal mild leukopenia and elevated transaminases. Blood cultures and viral PCR panels are pending.

IMAGING:
A chest radiograph shows no pulmonary infiltrates.

## Encounter 3: source 2021-07-15, practice 2022-10-27, outpatient, Ophthalmology

Chief complaint: 6-month history of progressive blurring of vision, particularly when reading or using a computer, and difficulty seeing in dim light

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Maria Gonzalez, MD
Department: Ophthalmology
Visit Type: outpatient

CHIEF COMPLAINT:
6-month history of progressive blurring of vision, particularly when reading or using a computer, and difficulty seeing in dim light

HISTORY OF PRESENT ILLNESS:
The patient is a 45-year-old female with glucocorticoid-remediable aldosteronism and history of toxic epidermal necrolysis who presents with progressive visual symptoms. Over six months, she has noted progressive blurring, particularly when reading or using a computer, with difficulty in dim light. She also reports noticing a dark spot in the center of her vision that seems to move as she looks around.

PAST MEDICAL HISTORY:
Active Problem List:
- Morbid obesity (diagnosed 2020-01-15)
- Glucocorticoid-Remediable Aldosteronism (Familial Hyperaldosteronism Type I) (diagnosed 2020-04-15)
- Toxic epidermal necrolysis (diagnosed 2020-10-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Morbid Obesity (Class III)
- Obstructive Sleep Apnea
- Age-related Macular Degeneration
- Seasonal Allergic Rhinitis

Her past medical history includes hypertension, which has been poorly controlled, and hyperlipidemia.

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO nightly
- Dexamethasone 0.5 mg PO daily
- Continuous Positive Airway Pressure (CPAP) 10 cm H2O nightly

ALLERGIES:
Sulfonamide antibiotics (e.g., trimethoprim-sulfamethoxazole) - Toxic Epidermal Necrolysis, Pollen (seasonal)

FAMILY HISTORY:
- Familial hyperaldosteronism type I (autosomal dominant) in father and paternal grandmother
- Premature coronary artery disease (father, myocardial infarction at age 48)
- Type 2 diabetes mellitus (mother)
- Age-related macular degeneration (maternal grandmother)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative assistant.

PHYSICAL EXAMINATION:
On physical examination, her visual acuity is 20/40 in both eyes, and there is no afferent pupillary defect. She sees wavy lines on an Amsler grid test. Fundoscopic examination reveals exudates and microaneurysms in the macula, with hard drusen and a subretinal hemorrhage.

IMAGING:
Fluorescein angiography shows patchy areas of hypofluorescence.

## Encounter 4: source 2022-07-15, practice 2023-10-13, ed, Emergency Medicine

Chief complaint: Severe right upper quadrant pain

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Aisha Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe right upper quadrant pain

HISTORY OF PRESENT ILLNESS:
Returning to the emergency department, this 46-year-old female with morbid obesity and type 2 diabetes mellitus presents with a 4-hour history of severe right upper quadrant pain that started after eating a large, fatty meal. She describes the pain as sharp and constant, radiating to her right shoulder. She denies any changes in bowel habits or urine color.

PAST MEDICAL HISTORY:
Active Problem List:
- Morbid obesity (diagnosed 2020-01-15)
- Glucocorticoid-Remediable Aldosteronism (Familial Hyperaldosteronism Type I) (diagnosed 2020-04-15)
- Toxic epidermal necrolysis (diagnosed 2020-10-15)
- Age-related macular degeneration (diagnosed 2021-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Morbid Obesity (Class III)
- Obstructive Sleep Apnea
- Seasonal Allergic Rhinitis

Her past medical history is significant for hyperlipidemia and obesity.

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO nightly
- Dexamethasone 0.5 mg PO daily
- Continuous Positive Airway Pressure (CPAP) 10 cm H2O nightly

ALLERGIES:
Sulfonamide antibiotics (e.g., trimethoprim-sulfamethoxazole) - Toxic Epidermal Necrolysis, Pollen (seasonal)

FAMILY HISTORY:
- Familial hyperaldosteronism type I (autosomal dominant) in father and paternal grandmother
- Premature coronary artery disease (father, myocardial infarction at age 48)
- Type 2 diabetes mellitus (mother)
- Age-related macular degeneration (maternal grandmother)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative assistant.

VITAL SIGNS:
Her vital signs are stable

PHYSICAL EXAMINATION:
There is marked tenderness in the right upper quadrant without rebound or guarding. Murphy's sign is negative.

LABORATORY DATA:
White blood cell count: 7,000/uL
- Hemoglobin: 13.5 g/dL
- Platelets: 250,000/uL
- Aspartate aminotransferase (AST): 30 U/L
- Alanine aminotransferase (ALT): 35 U/L
- Alkaline phosphatase: 90 U/L
- Total bilirubin: 0.8 mg/dL

IMAGING:
An abdominal ultrasound is performed, which reveals multiple gallstones in the gallbladder, with no evidence of cholecystitis or choledocholithiasis.

## Encounter 5: source 2023-01-15, practice 2024-04-22, ed, Emergency Medicine

Chief complaint: comes to the emergency department with 2 days of worsening abdominal pain, nausea, and vomiting

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
She comes to the emergency department with 2 days of worsening abdominal pain, nausea, and vomiting

HISTORY OF PRESENT ILLNESS:
This patient with glucocorticoid-remediable aldosteronism on dexamethasone, morbid obesity, and prior choledocholithiasis presents with abdominal pain. She says the pain began gradually after a heavy meal and has since become constant and more intense. She reports mild subjective fevers and chills but no changes in bowel habits. She has no history of recent travel, alcohol abuse, or liver disease.

PAST MEDICAL HISTORY:
Active Problem List:
- Morbid obesity (diagnosed 2020-01-15)
- Glucocorticoid-Remediable Aldosteronism (Familial Hyperaldosteronism Type I) (diagnosed 2020-04-15)
- Toxic epidermal necrolysis (diagnosed 2020-10-15)
- Age-related macular degeneration (diagnosed 2021-07-15)
- Choledocholithiasis (diagnosed 2022-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Morbid Obesity (Class III)
- Obstructive Sleep Apnea
- Seasonal Allergic Rhinitis

Her past medical history includes obesity and hyperlipidemia. She has had three prior pregnancies without complications.

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO nightly
- Dexamethasone 0.5 mg PO daily
- Continuous Positive Airway Pressure (CPAP) 10 cm H2O nightly

ALLERGIES:
Sulfonamide antibiotics (e.g., trimethoprim-sulfamethoxazole) - Toxic Epidermal Necrolysis, Pollen (seasonal)

FAMILY HISTORY:
- Familial hyperaldosteronism type I (autosomal dominant) in father and paternal grandmother
- Premature coronary artery disease (father, myocardial infarction at age 48)
- Type 2 diabetes mellitus (mother)
- Age-related macular degeneration (maternal grandmother)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative assistant.

VITAL SIGNS:
Temperature is 38.3°C (100.9°F), blood pressure is 128/78 mm Hg, pulse is 102/min, and respiratory rate is 18/min. BMI is 33 kg/m²

PHYSICAL EXAMINATION:
Abdominal examination reveals moderate tenderness without rebound or guarding, and bowel sounds are normal

LABORATORY DATA:
Laboratory studies show leukocytosis with neutrophilic predominance; liver enzymes and serum amylase are within normal limits

IMAGING:
Abdominal ultrasound demonstrates gallstones and gallbladder wall thickening with pericholecystic fluid

## Encounter 6: source 2024-01-15, practice 2025-04-20, ed, Emergency Medicine

Chief complaint: sudden-onset severe upper abdominal pain that began 6 hours ago

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Christopher Lee, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden-onset severe upper abdominal pain that began 6 hours ago

HISTORY OF PRESENT ILLNESS:
The patient, with prior choledocholithiasis and cholecystitis, presents with sudden-onset severe upper abdominal pain. The pain is constant, radiates to the back, and is associated with nausea and two episodes of non-bloody, non-bilious vomiting. She denies recent trauma, alcohol intake, or new medications.

PAST MEDICAL HISTORY:
Active Problem List:
- Morbid obesity (diagnosed 2020-01-15)
- Glucocorticoid-Remediable Aldosteronism (Familial Hyperaldosteronism Type I) (diagnosed 2020-04-15)
- Toxic epidermal necrolysis (diagnosed 2020-10-15)
- Age-related macular degeneration (diagnosed 2021-07-15)
- Choledocholithiasis (diagnosed 2022-07-15)
- Acute cholecystitis (diagnosed 2023-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Morbid Obesity (Class III)
- Obstructive Sleep Apnea
- Seasonal Allergic Rhinitis

PAST SURGICAL HISTORY:
She has no prior abdominal surgeries

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO nightly
- Dexamethasone 0.5 mg PO daily
- Continuous Positive Airway Pressure (CPAP) 10 cm H2O nightly

ALLERGIES:
Sulfonamide antibiotics (e.g., trimethoprim-sulfamethoxazole) - Toxic Epidermal Necrolysis, Pollen (seasonal)

FAMILY HISTORY:
Family history is notable for gallstones in her mother.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative assistant.

VITAL SIGNS:
Afebrile with a blood pressure of 128/78 mm Hg, pulse of 96/min, and respiratory rate of 20/min

PHYSICAL EXAMINATION:
She appears uncomfortable and is lying still on the stretcher. Abdominal examination reveals epigastric tenderness to deep palpation without guarding or rebound; bowel sounds are decreased. Murphy's sign is negative.

LABORATORY DATA:
Amylase 1,120 U/L
Lipase 2,600 U/L
Alanine aminotransferase 172 U/L
Aspartate aminotransferase 156 U/L
Total bilirubin 2.1 mg/dL

IMAGING:
Abdominal ultrasound reveals multiple gallstones within the gallbladder and common bile duct dilation to 9 mm.

## Encounter 7: source 2025-01-15, practice 2026-04-18, ed, Emergency Medicine

Chief complaint: acute onset of chest pain radiating to her left arm, associated with shortness of breath and diaphoresis

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. David Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset of chest pain radiating to her left arm, associated with shortness of breath and diaphoresis

PAST MEDICAL HISTORY:
Active Problem List:
- Morbid obesity (diagnosed 2020-01-15)
- Glucocorticoid-Remediable Aldosteronism (Familial Hyperaldosteronism Type I) (diagnosed 2020-04-15)
- Toxic epidermal necrolysis (diagnosed 2020-10-15)
- Age-related macular degeneration (diagnosed 2021-07-15)
- Choledocholithiasis (diagnosed 2022-07-15)
- Acute cholecystitis (diagnosed 2023-01-15)
- Choledocholithiasis with cholangitis (diagnosed 2024-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Morbid Obesity (Class III)
- Obstructive Sleep Apnea
- Seasonal Allergic Rhinitis

She has a history of hypertension and hyperlipidemia.

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO nightly
- Dexamethasone 0.5 mg PO daily
- Continuous Positive Airway Pressure (CPAP) 10 cm H2O nightly

ALLERGIES:
Sulfonamide antibiotics (e.g., trimethoprim-sulfamethoxazole) - Toxic Epidermal Necrolysis, Pollen (seasonal)

FAMILY HISTORY:
Her father had a myocardial infarction at age 50

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative assistant.

VITAL SIGNS:
Her blood pressure is 160/90 mmHg, heart rate is 98 bpm

PHYSICAL EXAMINATION:
She is in moderate distress

LABORATORY DATA:
Troponin I level is elevated at 0.5 ng/mL (normal <0.04 ng/mL)

OTHER STUDIES:
An ECG shows ST-segment elevations in leads II, III, and aVF

ASSESSMENT AND PLAN:
Acute myocardial infarction
