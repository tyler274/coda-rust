# synthetic_hospital patient 2317

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 72 at first encounter, sex M, African American, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2017-12-14 (Thu) | outpatient | Neurology |
| 1 | 2021-01-15 | 2018-12-13 (Thu) | outpatient | Internal Medicine |
| 2 | 2022-01-15 | 2019-12-19 (Thu) | outpatient | Gastroenterology |
| 3 | 2026-01-15 | 2023-12-12 (Tue) | ed | Emergency Medicine |
| 4 | 2028-01-15 | 2025-12-13 (Sat) | inpatient | Nephrology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Sulfamethoxazole (anaphylaxis)",
  "Latex (contact dermatitis)"
 ],
 "insurance": "Medicare",
 "occupation": "Retired accountant",
 "alcohol_use": "One 5-oz glass of wine daily with dinner",
 "family_history": [
  "Father: Essential tremor, atrial fibrillation",
  "Mother: Osteoporosis, hypertension",
  "Sister: Systemic lupus erythematosus",
  "Paternal uncle: Colon cancer (age 68)"
 ],
 "race_ethnicity": "African American",
 "smoking_status": "Former smoker, quit 20 years ago (15 pack-year history)",
 "home_medications": [
  {
   "dose": "5 mg daily",
   "name": "Prednisone"
  },
  {
   "dose": "500 mg twice daily",
   "name": "Mycophenolate mofetil"
  },
  {
   "dose": "20 mg daily at bedtime",
   "name": "Atorvastatin"
  },
  {
   "dose": "400 mg three times daily with meals",
   "name": "Ibuprofen"
  },
  {
   "dose": "20 mg daily (OTC)",
   "name": "Omeprazole"
  },
  {
   "dose": "1000 mcg sublingual daily",
   "name": "Cyanocobalamin"
  },
  {
   "dose": "325 mg (65 mg elemental) daily",
   "name": "Ferrous sulfate"
  },
  {
   "dose": "600 mg/400 units twice daily",
   "name": "Calcium carbonate with Vitamin D3"
  },
  {
   "dose": "70 mg weekly",
   "name": "Alendronate"
  }
 ],
 "surgical_history": [
  "Right total knee arthroplasty (2018)",
  "Diagnostic laparoscopy (1995)"
 ],
 "chronic_conditions": [
  "Systemic lupus erythematosus (diagnosed age 26)",
  "Atrial fibrillation",
  "Osteoarthritis",
  "Gastroesophageal reflux disease",
  "Essential tremor",
  "Vitamin B12 deficiency",
  "Iron deficiency anemia",
  "Osteoporosis (steroid-induced and disuse)",
  "Hyperlipidemia"
 ],
 "age_at_first_encounter": 72
}
```

Primary diagnoses: ["Acute mesenteric ischemia due to arterial embolism", "Disuse osteoporosis (Immobilization)", "Vitamin B12 deficiency", "Essential tremor", "Iron deficiency anemia"]
Comorbidities: ["Long term (current) use of immunosuppressants"]

## Encounter 0: source 2020-01-15, practice 2017-12-14, outpatient, Neurology

Chief complaint: Difficulty writing and using utensils over the past year with gradual hand shakiness

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Difficulty writing and using utensils over the past year with gradual hand shakiness

HISTORY OF PRESENT ILLNESS:
He reports a gradual onset of hand shakiness that worsens when he drinks coffee, tries to hold a spoon, or signs his name. He drinks a glass of wine in the evening daily, which he says improves the shakiness. He also reports that the symptoms disappear when he is lying down.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus (diagnosed age 26)
- Atrial fibrillation
- Osteoarthritis
- Gastroesophageal reflux disease
- Essential tremor
- Vitamin B12 deficiency
- Iron deficiency anemia
- Osteoporosis (steroid-induced and disuse)
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Right total knee arthroplasty (2018)
- Diagnostic laparoscopy (1995)

MEDICATIONS:
He takes only atorvastatin.

Home Medications:
- Prednisone 5 mg daily
- Mycophenolate mofetil 500 mg twice daily
- Ibuprofen 400 mg three times daily with meals
- Omeprazole 20 mg daily (OTC)
- Cyanocobalamin 1000 mcg sublingual daily
- Ferrous sulfate 325 mg (65 mg elemental) daily
- Calcium carbonate with Vitamin D3 600 mg/400 units twice daily
- Alendronate 70 mg weekly

ALLERGIES:
Sulfamethoxazole (anaphylaxis), Latex (contact dermatitis)

FAMILY HISTORY:
His father had similar symptoms beginning in his 60s.

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago (15 pack-year history). Alcohol: One 5-oz glass of wine daily with dinner. Occupation: Retired accountant.

REVIEW OF SYSTEMS:
He denies weakness, numbness, gait instability, or changes in speech.

VITAL SIGNS:
Vital signs are: temperature 36.8°C (98.2°F), blood pressure 124/74 mm Hg, pulse 68/min, and respiratory rate 14/min.

PHYSICAL EXAMINATION:
On exam, he has a fine, bilateral tremor of the hands that is most pronounced when his arms are extended or during finger-to-nose testing. His gait and muscle strength are normal with no rigidity. Deep tendon reflexes are 2+ throughout. Examination of other systems is unremarkable.

## Encounter 1: source 2021-01-15, practice 2018-12-13, outpatient, Internal Medicine

Chief complaint: Progressive numbness and tingling in feet for 6 months with difficulty walking and recent falls

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. James Okafor, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive numbness and tingling in feet for 6 months with difficulty walking and recent falls

HISTORY OF PRESENT ILLNESS:
This 73-year-old male with longstanding systemic lupus erythematosus, previously evaluated in Neurology one year ago for hand tremor, now presents with a 6-month history of progressive numbness and tingling in his feet. He reports difficulty walking, especially in the dark, and has fallen twice in the past month.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential tremor (diagnosed 2020-01-15)
- Systemic lupus erythematosus (diagnosed age 26)
- Atrial fibrillation
- Osteoarthritis
- Gastroesophageal reflux disease
- Vitamin B12 deficiency
- Iron deficiency anemia
- Osteoporosis (steroid-induced and disuse)
- Hyperlipidemia

His past medical history is significant for gastroesophageal reflux disease (GERD).

PAST SURGICAL HISTORY:
- Right total knee arthroplasty (2018)
- Diagnostic laparoscopy (1995)

MEDICATIONS:
He takes he has been taking over-the-counter antacids regularly for several years.

Home Medications:
- Prednisone 5 mg daily
- Mycophenolate mofetil 500 mg twice daily
- Atorvastatin 20 mg daily at bedtime
- Ibuprofen 400 mg three times daily with meals
- Omeprazole 20 mg daily (OTC)
- Cyanocobalamin 1000 mcg sublingual daily
- Ferrous sulfate 325 mg (65 mg elemental) daily
- Calcium carbonate with Vitamin D3 600 mg/400 units twice daily
- Alendronate 70 mg weekly

ALLERGIES:
Sulfamethoxazole (anaphylaxis), Latex (contact dermatitis)

FAMILY HISTORY:
- Father: Essential tremor, atrial fibrillation
- Mother: Osteoporosis, hypertension
- Sister: Systemic lupus erythematosus
- Paternal uncle: Colon cancer (age 68)

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago (15 pack-year history). Alcohol: One 5-oz glass of wine daily with dinner. Occupation: Retired accountant.

PHYSICAL EXAMINATION:
Physical examination reveals decreased vibratory sensation in both lower extremities and diminished proprioception at the toes. Ankle reflexes are absent bilaterally.

LABORATORY DATA:
Routine blood work shows:

- Hemoglobin: 11.2 g/dL (normal: 13.8-17.2 g/dL)
- Mean corpuscular volume (MCV): 104 fL (normal: 80-96 fL)
- Serum vitamin B12: 190 pg/mL (normal: 200-900 pg/mL)
- Methylmalonic acid: Elevated

## Encounter 2: source 2022-01-15, practice 2019-12-19, outpatient, Gastroenterology

Chief complaint: Progressive fatigue and shortness of breath on exertion over 3 months with unintentional weight loss

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Maria Rodriguez, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue and shortness of breath on exertion over 3 months with unintentional weight loss

HISTORY OF PRESENT ILLNESS:
This 74-year-old male with longstanding systemic lupus erythematosus on chronic immunosuppression presents for evaluation of progressive constitutional symptoms. One year following treatment for vitamin B12 deficiency, he returns with a 3-month history of progressive fatigue and shortness of breath on exertion. He denies any chest pain, palpitations, or syncope. He also reports a 10-pound unintentional weight loss over the last 6 months but denies any changes in diet or appetite.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential tremor (diagnosed 2020-01-15)
- Vitamin B12 deficiency (diagnosed 2021-01-15)
- Systemic lupus erythematosus (diagnosed age 26)
- Atrial fibrillation
- Osteoarthritis
- Gastroesophageal reflux disease
- Iron deficiency anemia
- Osteoporosis (steroid-induced and disuse)
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Right total knee arthroplasty (2018)
- Diagnostic laparoscopy (1995)

MEDICATIONS:
He takes ibuprofen 400 mg three times daily.

Home Medications:
- Prednisone 5 mg daily
- Mycophenolate mofetil 500 mg twice daily
- Atorvastatin 20 mg daily at bedtime
- Omeprazole 20 mg daily (OTC)
- Cyanocobalamin 1000 mcg sublingual daily
- Ferrous sulfate 325 mg (65 mg elemental) daily
- Calcium carbonate with Vitamin D3 600 mg/400 units twice daily
- Alendronate 70 mg weekly

ALLERGIES:
Sulfamethoxazole (anaphylaxis), Latex (contact dermatitis)

FAMILY HISTORY:
Non-contributory

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago (15 pack-year history). Alcohol: One 5-oz glass of wine daily with dinner. Occupation: Retired accountant.

VITAL SIGNS:
Blood pressure is 130/80 mmHg, heart rate is 92 beats per minute

PHYSICAL EXAMINATION:
He appears pale. His cardiac examination is normal, and there is no lymphadenopathy or hepatosplenomegaly. Rectal examination reveals no masses, but there is melena on the glove.

LABORATORY DATA:
Hemoglobin: 9.2 g/dL (Normal: 13.8-17.2 g/dL)
- Mean corpuscular volume (MCV): 74 fL (Normal: 80-96 fL)
- Serum iron: 30 µg/dL (Normal: 65-176 µg/dL)
- Total iron-binding capacity (TIBC): 450 µg/dL (Normal: 250-450 µg/dL)
- Ferritin: 15 ng/mL (Normal: 24-336 ng/mL)

## Encounter 3: source 2026-01-15, practice 2023-12-12, ed, Emergency Medicine

Chief complaint: Sudden onset of severe diffuse abdominal pain beginning 6 hours ago associated with nausea and vomiting

ENCOUNTER NOTE
Date: 2026-01-15
Provider: Dr. David Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of severe diffuse abdominal pain beginning 6 hours ago associated with nausea and vomiting

HISTORY OF PRESENT ILLNESS:
The 78-year-old male with SLE, AFib, and prior essential tremor, B12 deficiency, and anemia presents with sudden abdominal pain. The pain is diffuse, 10/10 in intensity, and associated with nausea and vomiting. He denies any recent changes in bowel habits or rectal bleeding.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential tremor (diagnosed 2020-01-15)
- Vitamin B12 deficiency (diagnosed 2021-01-15)
- Iron deficiency anemia (diagnosed 2022-01-15)
- Systemic lupus erythematosus (diagnosed age 26)
- Atrial fibrillation
- Osteoarthritis
- Gastroesophageal reflux disease
- Osteoporosis (steroid-induced and disuse)
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Right total knee arthroplasty (2018)
- Diagnostic laparoscopy (1995)

MEDICATIONS:
Home Medications:
- Prednisone 5 mg daily
- Mycophenolate mofetil 500 mg twice daily
- Atorvastatin 20 mg daily at bedtime
- Ibuprofen 400 mg three times daily with meals
- Omeprazole 20 mg daily (OTC)
- Cyanocobalamin 1000 mcg sublingual daily
- Ferrous sulfate 325 mg (65 mg elemental) daily
- Calcium carbonate with Vitamin D3 600 mg/400 units twice daily
- Alendronate 70 mg weekly

ALLERGIES:
Sulfamethoxazole (anaphylaxis), Latex (contact dermatitis)

FAMILY HISTORY:
- Father: Essential tremor, atrial fibrillation
- Mother: Osteoporosis, hypertension
- Sister: Systemic lupus erythematosus
- Paternal uncle: Colon cancer (age 68)

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago (15 pack-year history). Alcohol: One 5-oz glass of wine daily with dinner. Occupation: Retired accountant.

VITAL SIGNS:
His vital signs are as follows: blood pressure 150/90 mmHg, heart rate 110 bpm, respiratory rate 22 breaths/min, and temperature 37.8°C (100°F).

PHYSICAL EXAMINATION:
On examination, his abdomen is distended, with diffuse tenderness and guarding. Bowel sounds are hypoactive.

LABORATORY DATA:
Laboratory studies reveal:

- White blood cell count: 18,000/mm³
- Hemoglobin: 16.5 g/dL
- Amylase: 250 U/L (Normal: 30-110 U/L)
- Arterial blood gas: pH 7.28, PaCO2 32 mmHg, HCO3- 16 mmol/L

OTHER STUDIES:
An electrocardiogram shows atrial fibrillation with a rapid ventricular response.

## Encounter 4: source 2028-01-15, practice 2025-12-13, inpatient, Nephrology

Chief complaint: Advanced lupus nephritis with complications of immobilization and disuse osteoporosis; patient expires

ENCOUNTER NOTE
Date: 2028-01-15
Provider: Dr. Aisha Johnson, MD
Department: Nephrology
Visit Type: inpatient

CHIEF COMPLAINT:
Advanced lupus nephritis with complications of immobilization and disuse osteoporosis; patient expires

PAST MEDICAL HISTORY:
Active Problem List:
- Essential tremor (diagnosed 2020-01-15)
- Vitamin B12 deficiency (diagnosed 2021-01-15)
- Iron deficiency anemia (diagnosed 2022-01-15)
- Acute mesenteric ischemia due to arterial embolism (diagnosed 2026-01-15)
- Systemic lupus erythematosus (diagnosed age 26)
- Atrial fibrillation
- Osteoarthritis
- Gastroesophageal reflux disease
- Osteoporosis (steroid-induced and disuse)
- Hyperlipidemia

PAST SURGICAL HISTORY:
- Right total knee arthroplasty (2018)
- Diagnostic laparoscopy (1995)

MEDICATIONS:
He takes managed chronically with prednisone and immunosuppressants.

Home Medications:
- Mycophenolate mofetil 500 mg twice daily
- Atorvastatin 20 mg daily at bedtime
- Ibuprofen 400 mg three times daily with meals
- Omeprazole 20 mg daily (OTC)
- Cyanocobalamin 1000 mcg sublingual daily
- Ferrous sulfate 325 mg (65 mg elemental) daily
- Calcium carbonate with Vitamin D3 600 mg/400 units twice daily
- Alendronate 70 mg weekly

ALLERGIES:
Sulfamethoxazole (anaphylaxis), Latex (contact dermatitis)

FAMILY HISTORY:
- Father: Essential tremor, atrial fibrillation
- Mother: Osteoporosis, hypertension
- Sister: Systemic lupus erythematosus
- Paternal uncle: Colon cancer (age 68)

SOCIAL HISTORY:
Smoking: Former smoker, quit 20 years ago (15 pack-year history). Alcohol: One 5-oz glass of wine daily with dinner. Occupation: Retired accountant.

PATHOLOGY:
A gross pathologic cross-section of his vertebrae are shown.
