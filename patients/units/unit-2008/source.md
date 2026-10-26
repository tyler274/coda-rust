# synthetic_hospital patient 2008

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 55 at first encounter, sex M, White, Eastern European (Polish), Medicare with supplemental Blue Cross.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-11-19 (Fri) | outpatient | Dermatology |
| 1 | 2020-04-15 | 2022-02-09 (Wed) | outpatient | Gastroenterology |
| 2 | 2020-10-15 | 2022-08-03 (Wed) | outpatient | Gastroenterology |
| 3 | 2021-01-15 | 2022-11-07 (Mon) | outpatient | Neurology |
| 4 | 2022-01-15 | 2023-11-08 (Wed) | icu | Otolaryngology |
| 5 | 2023-01-15 | 2024-11-11 (Mon) | outpatient | Urology |
| 6 | 2023-07-15 | 2025-05-04 (Sun) | ed | Emergency Medicine |
| 7 | 2024-01-15 | 2025-11-13 (Thu) | outpatient | Ophthalmology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare with supplemental Blue Cross",
 "occupation": "Former administrative clerk, currently on disability",
 "alcohol_use": "History of chronic alcoholism; currently reports social drinking",
 "family_history": [
  "Father with Parkinson disease",
  "Mother with gastric adenocarcinoma",
  "Paternal uncle with renal cell carcinoma"
 ],
 "race_ethnicity": "White, Eastern European (Polish)",
 "smoking_status": "30 pack-year history; former smoker",
 "home_medications": [
  {
   "dose": "20 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "25/100 mg three times daily",
   "name": "Carbidopa-levodopa"
  },
  {
   "dose": "50 mg twice daily",
   "name": "Riluzole"
  },
  {
   "dose": "25 mg daily (discontinued)",
   "name": "Hydrochlorothiazide"
  }
 ],
 "surgical_history": [
  "Total thyroidectomy for multinodular goiter"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Parkinson disease",
  "Chronic alcoholism",
  "Amyotrophic lateral sclerosis",
  "Gastric adenocarcinoma",
  "Clear cell renal cell carcinoma",
  "Seborrheic dermatitis"
 ],
 "age_at_first_encounter": 55
}
```

Primary diagnoses: ["Subconjunctival hemorrhage", "Postoperative hemorrhage following inguinal hernia repair", "Seborrheic dermatitis", "Cerebellar hemorrhage", "Gastric adenocarcinoma", "Amyotrophic lateral sclerosis", "Clear cell renal cell carcinoma"]
Comorbidities: ["Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2021-11-19, outpatient, Dermatology

Chief complaint: Itchy, flaky skin on scalp and face for 3 months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Itchy, flaky skin on scalp and face for 3 months

HISTORY OF PRESENT ILLNESS:
He reports that the condition worsens with stress and during the winter months. He has tried over-the-counter dandruff shampoos without significant improvement.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Parkinson disease
- Chronic alcoholism
- Amyotrophic lateral sclerosis
- Gastric adenocarcinoma
- Clear cell renal cell carcinoma
- Seborrheic dermatitis

His past medical history is significant for hypertension and he was diagnosed with Parkinson disease two years ago.

PAST SURGICAL HISTORY:
- Total thyroidectomy for multinodular goiter

MEDICATIONS:
He takes he takes hydrochlorothiazide.

Home Medications:
- Lisinopril 20 mg daily
- Carbidopa-levodopa 25/100 mg three times daily
- Riluzole 50 mg twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with Parkinson disease
- Mother with gastric adenocarcinoma
- Paternal uncle with renal cell carcinoma

SOCIAL HISTORY:
Smoking: 30 pack-year history; former smoker. Alcohol: History of chronic alcoholism; currently reports social drinking. Occupation: Former administrative clerk, currently on disability.

PHYSICAL EXAMINATION:
On physical examination, you note erythematous plaques with a greasy scale on the scalp, eyebrows, nasolabial folds, and behind the ears. There is no lymphadenopathy, and the rest of the examination is unremarkable.

## Encounter 1: source 2020-04-15, practice 2022-02-09, outpatient, Gastroenterology

Chief complaint: Weight loss and chronic mid-epigastric pain for 3 months

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Patel, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Weight loss and chronic mid-epigastric pain for 3 months

HISTORY OF PRESENT ILLNESS:
This 55-year-old male with seborrheic dermatitis presents with weight loss and epigastric pain. He reports that the pain worsens after meals, especially when consuming his traditional Eastern European diet, which includes frequent consumption of smoked meats and pickled vegetables. He denies any recent travel or sick contacts.

PAST MEDICAL HISTORY:
Active Problem List:
- Seborrheic dermatitis (diagnosed 2020-01-15)
- Hypertension
- Parkinson disease
- Chronic alcoholism
- Amyotrophic lateral sclerosis
- Gastric adenocarcinoma
- Clear cell renal cell carcinoma

PAST SURGICAL HISTORY:
- Total thyroidectomy for multinodular goiter

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Carbidopa-levodopa 25/100 mg three times daily
- Riluzole 50 mg twice daily
- Hydrochlorothiazide 25 mg daily (discontinued)

ALLERGIES:
NKDA

FAMILY HISTORY:
His family history is significant for gastric cancer in his father.

SOCIAL HISTORY:
Smoking: 30 pack-year history; former smoker. Alcohol: History of chronic alcoholism; currently reports social drinking. Occupation: Former administrative clerk, currently on disability.

PHYSICAL EXAMINATION:
On physical examination, he appears cachectic and pale. Abdominal examination reveals mild tenderness in the epigastric region without rebound or guarding.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 10.2 g/dL (Normal: 13.8-17.2 g/dL)
- Mean corpuscular volume (MCV): 92 fL (Normal: 80-100 fL)
- Serum ferritin: 30 ng/mL (Normal: 24-336 ng/mL)
- Fecal occult blood test: Positive

OTHER STUDIES:
Esophagogastroduodenoscopy is performed, revealing a 3 cm ulcerated mass in the antrum of the stomach. Biopsy of the mass is taken.

## Encounter 2: source 2020-10-15, practice 2022-08-03, outpatient, Gastroenterology

Chief complaint: Progressive epigastric pain and 10-pound weight loss over 6 months

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Michael Patel, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive epigastric pain and 10-pound weight loss over 6 months

HISTORY OF PRESENT ILLNESS:
The patient with Parkinson disease returns for Gastroenterology follow-up since his April 2020 evaluation. He presents with a 6-month history of progressive epigastric pain and a 10-pound weight loss. He denies any changes in bowel habits or dysphagia.

PAST MEDICAL HISTORY:
Active Problem List:
- Seborrheic dermatitis (diagnosed 2020-01-15)
- Gastric adenocarcinoma (diagnosed 2020-04-15)
- Hypertension
- Parkinson disease
- Chronic alcoholism
- Amyotrophic lateral sclerosis
- Clear cell renal cell carcinoma

His past medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- Total thyroidectomy for multinodular goiter

MEDICATIONS:
He takes controlled with hydrochlorothiazide.

Home Medications:
- Lisinopril 20 mg daily
- Carbidopa-levodopa 25/100 mg three times daily
- Riluzole 50 mg twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with Parkinson disease
- Mother with gastric adenocarcinoma
- Paternal uncle with renal cell carcinoma

SOCIAL HISTORY:
He has a 30-pack-year smoking history and drinks alcohol socially.

PHYSICAL EXAMINATION:
On physical examination, he appears pale and has mild epigastric tenderness on palpation. There is no rebound or guarding.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 9.8 g/dL (Normal: 13.8-17.2 g/dL)
- Mean corpuscular volume (MCV): 74 fL (Normal: 80-96 fL)
- Serum iron: 30 µg/dL (Normal: 65-176 µg/dL)
- Total iron-binding capacity (TIBC): 400 µg/dL (Normal: 250-450 µg/dL)
- Transferrin saturation: 8% (Normal: 20-50%)
- Liver function tests:
- Aspartate aminotransferase (AST): 85 U/L (Normal: 10-40 U/L)
- Alanine aminotransferase (ALT): 78 U/L (Normal: 7-56 U/L)
- Alkaline phosphatase (ALP): 220 U/L (Normal: 40-130 U/L)

IMAGING:
Abdominal ultrasonography reveals hepatomegaly with a heterogeneous echo texture.

## Encounter 3: source 2021-01-15, practice 2022-11-07, outpatient, Neurology

Chief complaint: Progressive hand weakness and difficulty walking for 6 months

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Aisha Washington, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive hand weakness and difficulty walking for 6 months

HISTORY OF PRESENT ILLNESS:
The patient returns for neurology evaluation of progressive motor symptoms in the context of known Parkinson disease and recent gastric adenocarcinoma management. He reports that he frequently trips and has noticed a decrease in his grip strength, which has made it challenging to button his shirts and hold utensils. He also mentions occasional muscle twitches in his arms and legs.

PAST MEDICAL HISTORY:
Active Problem List:
- Seborrheic dermatitis (diagnosed 2020-01-15)
- Gastric adenocarcinoma (diagnosed 2020-04-15)
- Hypertension
- Parkinson disease
- Chronic alcoholism
- Amyotrophic lateral sclerosis
- Clear cell renal cell carcinoma

His past medical history is significant for hypertension, controlled with.

PAST SURGICAL HISTORY:
- Total thyroidectomy for multinodular goiter

MEDICATIONS:
He takes lisinopril.

Home Medications:
- Carbidopa-levodopa 25/100 mg three times daily
- Riluzole 50 mg twice daily
- Hydrochlorothiazide 25 mg daily (discontinued)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with Parkinson disease
- Mother with gastric adenocarcinoma
- Paternal uncle with renal cell carcinoma

SOCIAL HISTORY:
Smoking: 30 pack-year history; former smoker. Alcohol: History of chronic alcoholism; currently reports social drinking. Occupation: Former administrative clerk, currently on disability.

PHYSICAL EXAMINATION:
On physical examination, you note atrophy of the intrinsic hand muscles and fasciculations in the upper extremities. His neurological examination reveals hyperreflexia in the lower extremities and a positive Babinski sign.

LABORATORY DATA:
Routine blood work is unremarkable.

## Encounter 4: source 2022-01-15, practice 2023-11-08, icu, Otolaryngology

Chief complaint: Difficulty breathing and tightness in neck following thyroid surgery

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Robert Kim, MD
Department: Otolaryngology
Visit Type: icu

CHIEF COMPLAINT:
Difficulty breathing and tightness in neck following thyroid surgery

HISTORY OF PRESENT ILLNESS:
The patient is a 57-year-old male with amyotrophic lateral sclerosis and gastric adenocarcinoma who presents to the intensive care unit. He underwent a total thyroidectomy 6 hours ago for a multinodular goiter.

PAST MEDICAL HISTORY:
Active Problem List:
- Seborrheic dermatitis (diagnosed 2020-01-15)
- Gastric adenocarcinoma (diagnosed 2020-04-15)
- Amyotrophic lateral sclerosis (diagnosed 2021-01-15)
- Hypertension
- Parkinson disease
- Chronic alcoholism
- Clear cell renal cell carcinoma

PAST SURGICAL HISTORY:
- Total thyroidectomy for multinodular goiter

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Carbidopa-levodopa 25/100 mg three times daily
- Riluzole 50 mg twice daily
- Hydrochlorothiazide 25 mg daily (discontinued)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with Parkinson disease
- Mother with gastric adenocarcinoma
- Paternal uncle with renal cell carcinoma

SOCIAL HISTORY:
Smoking: 30 pack-year history; former smoker. Alcohol: History of chronic alcoholism; currently reports social drinking. Occupation: Former administrative clerk, currently on disability.

VITAL SIGNS:
His vital signs are as follows: BP 145/90 mmHg, HR 110 bpm, RR 28 breaths/min, Temp 37.1°C (98.8°F), and O2 Sat 92% on room air

PHYSICAL EXAMINATION:
On examination, his voice is hoarse, and he appears anxious. He is using accessory muscles to breathe and has stridor. His neck is swollen and tender to palpation. The surgical site is clean, but there is noticeable swelling and ecchymosis extending from the incision site.

## Encounter 5: source 2023-01-15, practice 2024-11-11, outpatient, Urology

Chief complaint: Fatigue, headaches, and left flank fullness for 3 months

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. James O'Brien, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue, headaches, and left flank fullness for 3 months

HISTORY OF PRESENT ILLNESS:
This 58-year-old male with a history of gastric adenocarcinoma and amyotrophic lateral sclerosis, previously hospitalized in 2022 for postoperative complications, returns with a 3-month history of fatigue, headaches, and an uncomfortable sensation of fullness in his left flank. He denies any respiratory symptoms, chest pain, or history of smoking.

PAST MEDICAL HISTORY:
Active Problem List:
- Seborrheic dermatitis (diagnosed 2020-01-15)
- Gastric adenocarcinoma (diagnosed 2020-04-15)
- Amyotrophic lateral sclerosis (diagnosed 2021-01-15)
- Postoperative hemorrhage following inguinal hernia repair (diagnosed 2022-01-15)
- Hypertension
- Parkinson disease
- Chronic alcoholism
- Clear cell renal cell carcinoma

His past medical history is significant for hypertension, which is well-controlled on lisinopril.

PAST SURGICAL HISTORY:
- Total thyroidectomy for multinodular goiter

MEDICATIONS:
He takes lisinopril.

Home Medications:
- Carbidopa-levodopa 25/100 mg three times daily
- Riluzole 50 mg twice daily
- Hydrochlorothiazide 25 mg daily (discontinued)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with Parkinson disease
- Mother with gastric adenocarcinoma
- Paternal uncle with renal cell carcinoma

SOCIAL HISTORY:
Smoking: 30 pack-year history; former smoker. Alcohol: History of chronic alcoholism; currently reports social drinking. Occupation: Former administrative clerk, currently on disability.

VITAL SIGNS:
On physical examination, his blood pressure is 135/85 mmHg, heart rate is 78 bpm, and oxygen saturation is 98% on room air.

PHYSICAL EXAMINATION:
His skin appears ruddy, and his spleen is palpable 3 cm below the left costal margin.

LABORATORY DATA:
Laboratory findings are as follows:
- Hemoglobin: 18.5 g/dL (Normal: 13.8-17.2 g/dL)
- Hematocrit: 55% (Normal: 40.7-50.3%)
- White blood cells: 7,000/uL (Normal: 4,500-11,000/uL)
- Platelets: 250,000/uL (Normal: 150,000-450,000/uL)
- Serum erythropoietin: Elevated

## Encounter 6: source 2023-07-15, practice 2025-05-04, ed, Emergency Medicine

Chief complaint: Severe headache, nausea, vomiting, and difficulty walking for 2 days

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. David Smith, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe headache, nausea, vomiting, and difficulty walking for 2 days

HISTORY OF PRESENT ILLNESS:
This 58-year-old male with a history of Parkinson disease, amyotrophic lateral sclerosis, and clear cell renal cell carcinoma presents to the emergency department with a 2-day history of progressive headache, nausea, and vomiting. He also complains of difficulty walking, which he attributes to a feeling of imbalance.

PAST MEDICAL HISTORY:
Active Problem List:
- Seborrheic dermatitis (diagnosed 2020-01-15)
- Gastric adenocarcinoma (diagnosed 2020-04-15)
- Amyotrophic lateral sclerosis (diagnosed 2021-01-15)
- Postoperative hemorrhage following inguinal hernia repair (diagnosed 2022-01-15)
- Clear cell renal cell carcinoma (diagnosed 2023-01-15)
- Hypertension
- Parkinson disease
- Chronic alcoholism

PAST SURGICAL HISTORY:
- Total thyroidectomy for multinodular goiter

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg daily
- Carbidopa-levodopa 25/100 mg three times daily
- Riluzole 50 mg twice daily
- Hydrochlorothiazide 25 mg daily (discontinued)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with Parkinson disease
- Mother with gastric adenocarcinoma
- Paternal uncle with renal cell carcinoma

SOCIAL HISTORY:
Smoking: 30 pack-year history; former smoker. Alcohol: History of chronic alcoholism; currently reports social drinking. Occupation: Former administrative clerk, currently on disability.

VITAL SIGNS:
Blood pressure of 180/95 mmHg

PHYSICAL EXAMINATION:
On examination, he is alert but anxious. Neurological examination reveals slurred speech, nystagmus on lateral gaze, and a wide-based gait. He tends to fall to the left when attempting to walk.

IMAGING:
A non-contrast head CT scan shows a 4 cm hyperdensity in the posterior fossa with surrounding edema. There is evidence of effacement of the fourth ventricle and upward transtentorial herniation.

## Encounter 7: source 2024-01-15, practice 2025-11-13, outpatient, Ophthalmology

Chief complaint: Acute painless red left eye noticed upon waking

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Maria Gonzalez, MD
Department: Ophthalmology
Visit Type: outpatient

CHIEF COMPLAINT:
Acute painless red left eye noticed upon waking

HISTORY OF PRESENT ILLNESS:
The patient with gastric cancer reports a red eye that he noticed upon waking up this morning. He denies any pain, vision changes, discharge, or recent trauma to the eye.

PAST MEDICAL HISTORY:
Active Problem List:
- Seborrheic dermatitis (diagnosed 2020-01-15)
- Gastric adenocarcinoma (diagnosed 2020-04-15)
- Amyotrophic lateral sclerosis (diagnosed 2021-01-15)
- Postoperative hemorrhage following inguinal hernia repair (diagnosed 2022-01-15)
- Clear cell renal cell carcinoma (diagnosed 2023-01-15)
- Cerebellar hemorrhage (diagnosed 2023-07-15)
- Hypertension
- Parkinson disease
- Chronic alcoholism

He has a history of hypertension. He does not have a history of diabetes or coagulopathy and has not been on anticoagulant therapy.

PAST SURGICAL HISTORY:
- Total thyroidectomy for multinodular goiter

MEDICATIONS:
He takes for which he takes hydrochlorothiazide.

Home Medications:
- Lisinopril 20 mg daily
- Carbidopa-levodopa 25/100 mg three times daily
- Riluzole 50 mg twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with Parkinson disease
- Mother with gastric adenocarcinoma
- Paternal uncle with renal cell carcinoma

SOCIAL HISTORY:
Smoking: 30 pack-year history; former smoker. Alcohol: History of chronic alcoholism; currently reports social drinking. Occupation: Former administrative clerk, currently on disability.

PHYSICAL EXAMINATION:
On physical examination, his visual acuity is 20/20 in both eyes, and intraocular pressures are normal. There is no corneal abrasion or foreign body noted. The right eye appears normal, but the left eye has a bright red patch on the sclera. There is no conjunctival injection or purulent discharge. The anterior chamber is clear, and the fundoscopic examination is normal. The remainder of the physical examination is unremarkable.
