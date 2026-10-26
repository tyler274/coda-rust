# synthetic_hospital patient 2191

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 65 at first encounter, sex M, Non-Hispanic White, Medicare with supplemental private insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2017-12-22 (Fri) | inpatient | Internal Medicine |
| 1 | 2020-07-15 | 2018-06-22 (Fri) | outpatient | Nephrology |
| 2 | 2021-01-15 | 2018-12-24 (Mon) | outpatient | Hematology |
| 3 | 2022-01-15 | 2020-01-07 (Tue) | outpatient | Nephrology |
| 4 | 2023-01-15 | 2020-12-28 (Mon) | outpatient | Podiatry |
| 5 | 2025-01-15 | 2022-12-23 (Fri) | ed | Emergency Medicine |
| 6 | 2027-01-15 | 2024-12-20 (Fri) | ed | Emergency Medicine |
| 7 | 2028-01-15 | 2025-12-23 (Tue) | outpatient | Urology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare with supplemental private insurance",
 "occupation": "Retired construction foreman",
 "alcohol_use": "Social drinker (1-2 drinks per week)",
 "family_history": [
  "Father: Myocardial infarction at age 62",
  "Mother: Type 2 Diabetes Mellitus, Chronic Kidney Disease",
  "Paternal uncle: Lung cancer"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 40 pack-year history",
 "home_medications": [
  {
   "dose": "24 units subcutaneously at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "10 mg orally daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg orally daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "800 mg orally three times daily with meals",
   "name": "Sevelamer carbonate"
  },
  {
   "dose": "0.25 mcg orally daily",
   "name": "Calcitriol"
  },
  {
   "dose": "650 mg orally every 6 hours as needed for pain",
   "name": "Acetaminophen"
  }
 ],
 "surgical_history": [
  "Left knee arthroscopy (2015)"
 ],
 "chronic_conditions": [
  "Type 2 Diabetes Mellitus (20 years duration)",
  "Hypertension",
  "Coronary Artery Disease",
  "Chronic Kidney Disease Stage 4 secondary to diabetic nephropathy",
  "Hyperlipidemia",
  "Osteoarthritis"
 ],
 "age_at_first_encounter": 65
}
```

Primary diagnoses: ["Anemia of chronic kidney disease", "Heart failure with reduced ejection fraction", "Tertiary hyperparathyroidism", "Osteomyelitis of the foot", "Type 2 diabetes mellitus", "Acute ST-segment elevation myocardial infarction (STEMI) of inferior wall", "Renal Cell Carcinoma"]
Comorbidities: ["Tobacco use disorder", "Type 2 diabetes mellitus", "Chronic kidney disease stage 4", "Essential hypertension", "Diabetic motor neuropathy", "Coronary Artery Disease (Antiplatelet Therapy)"]

## Encounter 0: source 2020-01-15, practice 2017-12-22, inpatient, Internal Medicine

Chief complaint: Fever, productive cough, and dyspnea requiring hospitalization for community-acquired pneumonia

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Fever, productive cough, and dyspnea requiring hospitalization for community-acquired pneumonia

HISTORY OF PRESENT ILLNESS:
He has been febrile and mildly tachypneic for two days but is hemodynamically stable and receiving ceftriaxone and azithromycin. During hospitalization, his glucose levels are monitored every 6 hours, and he is transitioned from his outpatient oral medications to a subcutaneous insulin regimen. On hospital day 3, he receives his scheduled dose of insulin glargine in the morning. A few hours later, a nurse notes that his serum glucose has dropped from 228 mg/dL to 112 mg/dL without any change in IV fluids or food intake. The patient remains alert and asymptomatic.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 Diabetes Mellitus (20 years duration)
- Hypertension
- Coronary Artery Disease
- Chronic Kidney Disease Stage 4 secondary to diabetic nephropathy
- Hyperlipidemia
- Osteoarthritis

PAST SURGICAL HISTORY:
- Left knee arthroscopy (2015)

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneously at bedtime
- Lisinopril 10 mg orally daily
- Atorvastatin 40 mg orally daily
- Sevelamer carbonate 800 mg orally three times daily with meals
- Calcitriol 0.25 mcg orally daily
- Acetaminophen 650 mg orally every 6 hours as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 Diabetes Mellitus, Chronic Kidney Disease
- Paternal uncle: Lung cancer

SOCIAL HISTORY:
Smoking: Current smoker, 40 pack-year history. Alcohol: Social drinker (1-2 drinks per week). Occupation: Retired construction foreman.

PHYSICAL EXAMINATION:
Physical exam is unremarkable, and there is no evidence of lipodystrophy or insulin overdosage.

ASSESSMENT AND PLAN:
The team suspects this reflects a pharmacologic effect of the insulin dose.

## Encounter 1: source 2020-07-15, practice 2018-06-22, outpatient, Nephrology

Chief complaint: Generalized weakness and bone pain during routine CKD follow-up

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Raj Patel, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
Generalized weakness and bone pain during routine CKD follow-up

HISTORY OF PRESENT ILLNESS:
The patient returns for nephrology follow-up since January. He reports generalized weakness and bone pain for the past few months. He denies any recent trauma or falls.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus (20 years duration)
- Hypertension
- Coronary Artery Disease
- Chronic Kidney Disease Stage 4 secondary to diabetic nephropathy
- Hyperlipidemia
- Osteoarthritis

PAST SURGICAL HISTORY:
- Left knee arthroscopy (2015)

MEDICATIONS:
His medications include insulin glargine, sevelamer, and calcitriol.

Home Medications:
- Lisinopril 10 mg orally daily
- Atorvastatin 40 mg orally daily
- Sevelamer carbonate 800 mg orally three times daily with meals
- Acetaminophen 650 mg orally every 6 hours as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 Diabetes Mellitus, Chronic Kidney Disease
- Paternal uncle: Lung cancer

SOCIAL HISTORY:
Smoking: Current smoker, 40 pack-year history. Alcohol: Social drinker (1-2 drinks per week). Occupation: Retired construction foreman.

PHYSICAL EXAMINATION:
On physical examination, his blood pressure is 145/90 mmHg, and he has a palpable, non-tender mass in the neck.

LABORATORY DATA:
Laboratory results are as follows:

- Serum calcium: 11.2 mg/dL (Normal: 8.5-10.2 mg/dL)
- Serum phosphorus: 5.8 mg/dL (Normal: 2.5-4.5 mg/dL)
- Serum creatinine: 4.9 mg/dL (Normal: 0.6-1.2 mg/dL)
- Estimated GFR: 18 mL/min/1.73m^2
- Serum PTH: 800 pg/mL (Normal: 15-65 pg/mL)

## Encounter 2: source 2021-01-15, practice 2018-12-24, outpatient, Hematology

Chief complaint: Fatigue, poor exercise tolerance, dizziness, and metallic taste

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Emily Rodriguez, MD
Department: Hematology
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue, poor exercise tolerance, dizziness, and metallic taste

HISTORY OF PRESENT ILLNESS:
The patient is a 66-year-old male with long-standing diabetic nephropathy and history of tertiary hyperparathyroidism who presents with fatigue, poor exercise tolerance, and occasional dizziness over the past few months. He also notes loss of appetite and a metallic taste, and stopped NSAIDs recently due to worsening kidney function.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Tertiary hyperparathyroidism (diagnosed 2020-07-15)
- Type 2 Diabetes Mellitus (20 years duration)
- Hypertension
- Coronary Artery Disease
- Chronic Kidney Disease Stage 4 secondary to diabetic nephropathy
- Hyperlipidemia
- Osteoarthritis

He has a history of diabetes, hypertension, coronary artery disease, and osteoarthritis.

PAST SURGICAL HISTORY:
- Left knee arthroscopy (2015)

MEDICATIONS:
Medications include insulin, lisinopril, and acetaminophen.

Home Medications:
- Insulin glargine 24 units subcutaneously at bedtime
- Atorvastatin 40 mg orally daily
- Sevelamer carbonate 800 mg orally three times daily with meals
- Calcitriol 0.25 mcg orally daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 Diabetes Mellitus, Chronic Kidney Disease
- Paternal uncle: Lung cancer

SOCIAL HISTORY:
He smokes and drinks socially.

PHYSICAL EXAMINATION:
Exam shows pale conjunctivae and decreased vibratory sensation in the feet.

LABORATORY DATA:
Labs reveal hemoglobin 9.2 g/dL, MCV 89 fL, reticulocyte count 0.4%, creatinine 5.1 mg/dL, and GFR 14 mL/min/1.73 m². Ferritin, transferrin saturation, TSH, B12, and folate are normal. Stool guaiac is negative. Peripheral smear shows normocytic RBCs without abnormalities.

## Encounter 3: source 2022-01-15, practice 2020-01-07, outpatient, Nephrology

Chief complaint: Worsening fatigue and reduced exercise tolerance

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Raj Patel, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening fatigue and reduced exercise tolerance

HISTORY OF PRESENT ILLNESS:
This 67-year-old male with chronic kidney disease Stage 4 secondary to diabetic nephropathy returns for evaluation of worsening fatigue and reduced exercise tolerance, symptoms previously assessed by Hematology. He reports no bleeding, melena, or changes in diet.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Tertiary hyperparathyroidism (diagnosed 2020-07-15)
- Anemia of chronic kidney disease (diagnosed 2021-01-15)
- Type 2 Diabetes Mellitus (20 years duration)
- Hypertension
- Coronary Artery Disease
- Chronic Kidney Disease Stage 4 secondary to diabetic nephropathy
- Hyperlipidemia
- Osteoarthritis

He has a history of hypertension, type 2 diabetes mellitus, and chronic kidney disease stage 4.

PAST SURGICAL HISTORY:
- Left knee arthroscopy (2015)

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneously at bedtime
- Lisinopril 10 mg orally daily
- Atorvastatin 40 mg orally daily
- Sevelamer carbonate 800 mg orally three times daily with meals
- Calcitriol 0.25 mcg orally daily
- Acetaminophen 650 mg orally every 6 hours as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 Diabetes Mellitus, Chronic Kidney Disease
- Paternal uncle: Lung cancer

SOCIAL HISTORY:
Smoking: Current smoker, 40 pack-year history. Alcohol: Social drinker (1-2 drinks per week). Occupation: Retired construction foreman.

VITAL SIGNS:
Blood pressure is 150/92 mmHg.

PHYSICAL EXAMINATION:
Physical examination shows pale conjunctivae but is otherwise normal. Neurologic exam is unremarkable.

LABORATORY DATA:
Recent labs reveal stable kidney function with a creatinine of 3.0 mg/dL. Laboratory results:
Hemoglobin 9.8 g/dL
Hematocrit 30%
MCV 85 fL
Serum ferritin 230 ng/mL
Serum iron 40 µg/dL

## Encounter 4: source 2023-01-15, practice 2020-12-28, outpatient, Podiatry

Chief complaint: Non-healing ulcer on plantar surface of right foot for 3 months

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Michael Thompson, MD
Department: Podiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Non-healing ulcer on plantar surface of right foot for 3 months

HISTORY OF PRESENT ILLNESS:
The patient with Type 2 Diabetes Mellitus, Stage 4 CKD, and prior tertiary hyperparathyroidism returns for podiatry evaluation. He reports that the ulcer has been present for 3 months and has gradually increased in size. He denies any fever or chills but mentions a dull ache in the area. His diabetes has been poorly controlled with a recent HbA1c of 9.2%.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Tertiary hyperparathyroidism (diagnosed 2020-07-15)
- Anemia of chronic kidney disease (diagnosed 2021-01-15)
- Type 2 Diabetes Mellitus (20 years duration)
- Hypertension
- Coronary Artery Disease
- Chronic Kidney Disease Stage 4 secondary to diabetic nephropathy
- Hyperlipidemia
- Osteoarthritis

PAST SURGICAL HISTORY:
- Left knee arthroscopy (2015)

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneously at bedtime
- Lisinopril 10 mg orally daily
- Atorvastatin 40 mg orally daily
- Sevelamer carbonate 800 mg orally three times daily with meals
- Calcitriol 0.25 mcg orally daily
- Acetaminophen 650 mg orally every 6 hours as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 Diabetes Mellitus, Chronic Kidney Disease
- Paternal uncle: Lung cancer

SOCIAL HISTORY:
Smoking: Current smoker, 40 pack-year history. Alcohol: Social drinker (1-2 drinks per week). Occupation: Retired construction foreman.

PHYSICAL EXAMINATION:
On physical examination, the ulcer measures 3 cm in diameter, with a calloused edge and minimal surrounding erythema. There is no purulent discharge. Sensation is decreased in a stocking-glove distribution, and dorsalis pedis pulses are diminished bilaterally.

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 7,500 /µL
- Hemoglobin: 13.2 g/dL
- Erythrocyte sedimentation rate (ESR): 70 mm/hr
- C-reactive protein (CRP): 25 mg/L

## Encounter 5: source 2025-01-15, practice 2022-12-23, ed, Emergency Medicine

Chief complaint: Sudden onset severe chest pain radiating to left arm

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Jennifer Walsh, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset severe chest pain radiating to left arm

HISTORY OF PRESENT ILLNESS:
This 70-year-old male with type 2 diabetes, CKD 4, and CAD presents with severe chest pain radiating to the left arm which started 30 minutes ago while at rest. He stopped medications one week ago due to feeling well and cost concerns.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Tertiary hyperparathyroidism (diagnosed 2020-07-15)
- Anemia of chronic kidney disease (diagnosed 2021-01-15)
- Osteomyelitis of the foot (diagnosed 2023-01-15)
- Type 2 Diabetes Mellitus (20 years duration)
- Hypertension
- Coronary Artery Disease
- Chronic Kidney Disease Stage 4 secondary to diabetic nephropathy
- Hyperlipidemia
- Osteoarthritis

PAST SURGICAL HISTORY:
Drug-eluting stent placement in his left anterior descending artery 4 months ago

MEDICATIONS:
He takes metformin, lisinopril, and atorvastatin.

Home Medications:
- Insulin glargine 24 units subcutaneously at bedtime
- Sevelamer carbonate 800 mg orally three times daily with meals
- Calcitriol 0.25 mcg orally daily
- Acetaminophen 650 mg orally every 6 hours as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 Diabetes Mellitus, Chronic Kidney Disease
- Paternal uncle: Lung cancer

SOCIAL HISTORY:
Smoking: Current smoker, 40 pack-year history. Alcohol: Social drinker (1-2 drinks per week). Occupation: Retired construction foreman.

VITAL SIGNS:
Blood pressure is 160/90 mmHg

PHYSICAL EXAMINATION:
Diaphoretic, in distress

LABORATORY DATA:
Troponin I: 12 ng/mL (normal <0.04 ng/mL)
- Creatine kinase-MB: 80 U/L (normal 0-24 U/L)
- Myoglobin: 1500 ng/mL (normal 28-72 ng/mL)

OTHER STUDIES:
An electrocardiogram shows ST-segment elevation in the anterior leads.

## Encounter 6: source 2027-01-15, practice 2024-12-20, ed, Emergency Medicine

Chief complaint: Increasing shortness of breath, orthopnea, and paroxysmal nocturnal dyspnea

ENCOUNTER NOTE
Date: 2027-01-15
Provider: Dr. James Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Increasing shortness of breath, orthopnea, and paroxysmal nocturnal dyspnea

HISTORY OF PRESENT ILLNESS:
The patient is a 72-year-old man with a known history of coronary artery disease status post inferior wall STEMI in 2025, chronic kidney disease stage 4 with prior tertiary hyperparathyroidism and anemia of chronic disease, and long-standing type 2 diabetes mellitus complicated by prior osteomyelitis of the foot, who is brought to the emergency department by his daughter due to increasing shortness of breath over the past week. He reports difficulty breathing when lying flat and now sleeps upright in a recliner. He also notes waking up at night gasping for air and a recent reduction in his ability to walk short distances without becoming winded. He denies fever, cough, or chest pain. He has not had any recent medication changes and does not use alcohol, tobacco, or illicit drugs. His daughter notes that he has appeared increasingly fatigued and has had swelling in his feet and ankles that worsens throughout the day. He has also complained of needing to urinate more frequently at night. His appetite is reduced, but he has not experienced weight loss, nausea, or vomiting.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Tertiary hyperparathyroidism (diagnosed 2020-07-15)
- Anemia of chronic kidney disease (diagnosed 2021-01-15)
- Osteomyelitis of the foot (diagnosed 2023-01-15)
- Acute ST-segment elevation myocardial infarction (STEMI) of inferior wall (diagnosed 2025-01-15)
- Type 2 Diabetes Mellitus (20 years duration)
- Hypertension
- Coronary Artery Disease
- Chronic Kidney Disease Stage 4 secondary to diabetic nephropathy
- Hyperlipidemia
- Osteoarthritis

His past medical history includes coronary artery disease, type 2 diabetes mellitus, and long-standing hypertension.

PAST SURGICAL HISTORY:
- Left knee arthroscopy (2015)

MEDICATIONS:
Home Medications:
- Insulin glargine 24 units subcutaneously at bedtime
- Lisinopril 10 mg orally daily
- Atorvastatin 40 mg orally daily
- Sevelamer carbonate 800 mg orally three times daily with meals
- Calcitriol 0.25 mcg orally daily
- Acetaminophen 650 mg orally every 6 hours as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 Diabetes Mellitus, Chronic Kidney Disease
- Paternal uncle: Lung cancer

SOCIAL HISTORY:
He has not had any recent medication changes and does not use alcohol, tobacco, or illicit drugs.

VITAL SIGNS:
His heart rate is 104/min and regular. Blood pressure is 138/82 mm Hg.

PHYSICAL EXAMINATION:
On physical examination, he appears dyspneic and uses accessory muscles to breathe. Auscultation reveals end-expiratory crackles at the bases of both lungs. There is no wheezing or rhonchi. His heart rate is 104/min and regular, with a displaced apical impulse and soft S1. A third heart sound (S3) is audible at the apex. Jugular venous distension is observed with the head of the bed elevated to 45 degrees. There is bilateral 2+ pitting edema of the lower extremities extending to the mid-shin. The abdomen is soft and non-tender, and there is no hepatosplenomegaly. Neurologic exam is unremarkable.

LABORATORY DATA:
Laboratory testing is pending.

IMAGING:
Chest x-ray shows bilateral perihilar opacities with a "bat-wing" distribution and Kerley B lines.

OTHER STUDIES:
ECG shows normal sinus rhythm with left ventricular hypertrophy.

## Encounter 7: source 2028-01-15, practice 2025-12-23, outpatient, Urology

Chief complaint: Unintentional weight loss, intermittent flank pain, and hematuria

ENCOUNTER NOTE
Date: 2028-01-15
Provider: Dr. Robert Kim, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
Unintentional weight loss, intermittent flank pain, and hematuria

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus (diagnosed 2020-01-15)
- Tertiary hyperparathyroidism (diagnosed 2020-07-15)
- Anemia of chronic kidney disease (diagnosed 2021-01-15)
- Osteomyelitis of the foot (diagnosed 2023-01-15)
- Acute ST-segment elevation myocardial infarction (STEMI) of inferior wall (diagnosed 2025-01-15)
- Heart failure with reduced ejection fraction (diagnosed 2027-01-15)
- Type 2 Diabetes Mellitus (20 years duration)
- Hypertension
- Coronary Artery Disease
- Chronic Kidney Disease Stage 4 secondary to diabetic nephropathy
- Hyperlipidemia
- Osteoarthritis

His past medical history is significant for hypertension, controlled with lisinopril, and type 2 diabetes mellitus, managed with metformin.

PAST SURGICAL HISTORY:
- Left knee arthroscopy (2015)

MEDICATIONS:
He takes lisinopril, metformin.

Home Medications:
- Insulin glargine 24 units subcutaneously at bedtime
- Atorvastatin 40 mg orally daily
- Sevelamer carbonate 800 mg orally three times daily with meals
- Calcitriol 0.25 mcg orally daily
- Acetaminophen 650 mg orally every 6 hours as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 Diabetes Mellitus, Chronic Kidney Disease
- Paternal uncle: Lung cancer

SOCIAL HISTORY:
40-pack-year smoking history

PHYSICAL EXAMINATION:
On physical examination, a firm, nontender mass is palpable in the left flank. No lymphadenopathy is noted.

LABORATORY DATA:
- Hemoglobin: 12.5 g/dL (normal: 13.8-17.2 g/dL)
- White blood cell count: 7,500 /μL (normal: 4,500-11,000 /μL)
- Platelets: 250,000 /μL (normal: 150,000-450,000 /μL)
- Creatinine: 1.2 mg/dL (normal: 0.6-1.3 mg/dL)
- Calcium: 10.8 mg/dL (normal: 8.5-10.2 mg/dL)
- Urinalysis: Red blood cells present

IMAGING:
A contrast-enhanced abdominal CT scan reveals a 7 cm mass in the left kidney with no evidence of metastasis.
