# synthetic_hospital patient 2732

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 43 at first encounter, sex F, Non-Hispanic White, Commercial PPO (Blue Cross Blue Shield).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-04-11 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-03-15 | 2022-06-20 (Mon) | outpatient | Cardiology |
| 2 | 2020-07-15 | 2022-09-30 (Fri) | outpatient | Orthopedics & Sports Medicine |
| 3 | 2021-01-15 | 2023-04-10 (Mon) | outpatient | Obstetrics & Gynecology |
| 4 | 2021-07-15 | 2023-10-13 (Fri) | outpatient | General Surgery |
| 5 | 2022-05-15 | 2024-08-01 (Thu) | outpatient | Gastroenterology |
| 6 | 2023-01-15 | 2025-04-04 (Fri) | outpatient | Gastroenterology |
| 7 | 2023-07-15 | 2025-10-14 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO (Blue Cross Blue Shield)",
 "occupation": "Elementary school teacher",
 "alcohol_use": "None",
 "family_history": [
  "Father: deceased at age 68 (cardiac disease)",
  "Mother: type 2 diabetes mellitus, hypothyroidism",
  "Sister: obesity, smoker"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 1.5 packs per day for 23 years (34.5 pack-years), with intermittent periods of reduced consumption",
 "home_medications": [
  {
   "dose": "100 mcg PO daily",
   "name": "Levothyroxine"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Omeprazole"
  },
  {
   "dose": "1000 mg PO BID",
   "name": "Metformin"
  }
 ],
 "surgical_history": [
  "Laparoscopic cholecystectomy (5 years prior)",
  "Right femoral hernia repair with mesh (recent)"
 ],
 "chronic_conditions": [
  "Obesity (Class II, BMI 36 kg/m\u00b2)",
  "Type 2 Diabetes Mellitus (poorly controlled, diagnosed 2 years prior)",
  "Hypothyroidism (acquired)",
  "Gastroesophageal reflux disease (GERD)",
  "Osteoarthritis (bilateral knees)",
  "Tobacco use disorder"
 ],
 "age_at_first_encounter": 43
}
```

Primary diagnoses: ["Gastroesophageal reflux disease", "Acute pancreatitis", "Achalasia", "Osteoarthritis of knee", "Femoral hernia, right side, incarcerated", "Malignancy-associated hypercoagulable state", "Postcholecystectomy syndrome", "Pes anserinus pain syndrome"]
Comorbidities: ["Tobacco use disorder", "Obesity"]

## Encounter 0: source 2020-01-15, practice 2022-04-11, outpatient, Internal Medicine

Chief complaint: Bilateral knee pain and stiffness developing gradually over 6 weeks, worse with stairs and prolonged sitting; patient seeks counseling on reducing long-term joint damage risk.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Bilateral knee pain and stiffness developing gradually over 6 weeks, worse with stairs and prolonged sitting; patient seeks counseling on reducing long-term joint damage risk.

HISTORY OF PRESENT ILLNESS:
She describes aching in both knees, especially after a long day of teaching, and notes that the pain is worse when going up or down stairs. The stiffness tends to occur after prolonged sitting and improves somewhat with movement. She reports no redness, warmth, or swelling in the joints. She admits to minimal exercise lately but had recently attempted to rejoin a weekend hiking group. After a few hikes, she found that her knee pain significantly worsened, which discouraged her from continuing. She has no morning stiffness, no involvement of her hands or other joints,

PAST MEDICAL HISTORY:
Active Problem List:
- Obesity (Class II, BMI 36 kg/m²)
- Type 2 Diabetes Mellitus (poorly controlled, diagnosed 2 years prior)
- Hypothyroidism (acquired)
- Gastroesophageal reflux disease (GERD)
- Osteoarthritis (bilateral knees)
- Tobacco use disorder

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (5 years prior)
- Right femoral hernia repair with mesh (recent)

MEDICATIONS:
Home Medications:
- Levothyroxine 100 mcg PO daily
- Omeprazole 20 mg PO daily
- Metformin 1000 mg PO BID

ALLERGIES:
She has no allergies.

FAMILY HISTORY:
Her mother underwent knee replacement surgery in her 60s.

SOCIAL HISTORY:
Over the past year, she has gained about 9 kg (20 lb) due to emotional eating after her father’s passing. She smokes about half a pack of cigarettes daily and has done so for nearly two decades. Her diet is irregular, and she often eats takeout or processed foods.

VITAL SIGNS:
On exam, her height is 168 cm (5 ft 6 in) and weight is 91 kg (200 lb), giving her a BMI of 32.2 kg/m².

PHYSICAL EXAMINATION:
Knee examination shows full range of motion, no effusion, and mild crepitus during flexion and extension. No tenderness is present over the joint lines or ligaments, and there is no erythema. Strength, sensation, and reflexes are intact in both lower extremities.

## Encounter 1: source 2020-03-15, practice 2022-06-20, outpatient, Cardiology

Chief complaint: Six-month history of intermittent burning retrosternal chest pain occurring after meals and lasting hours, with sensation of food sticking; denies exertional component.

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Michael Rodriguez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of intermittent burning retrosternal chest pain occurring after meals and lasting hours, with sensation of food sticking; denies exertional component.

HISTORY OF PRESENT ILLNESS:
She presents to the Cardiology clinic with a six-month history of intermittent chest pain, two months following her prior evaluation for bilateral knee osteoarthritis, with known history of obesity, poorly controlled type 2 diabetes mellitus, and gastroesophageal reflux disease. The pain is described as a burning sensation located retrosternally, which often occurs after meals and lasts for several hours. She denies any exertional component to the pain. She also reports a sensation of food sticking in her chest and frequent heartburn, especially when lying down. She has tried over-the-counter antacids with partial relief of her symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis of knee (diagnosed 2020-01-15)
- Obesity (Class II, BMI 36 kg/m²)
- Type 2 Diabetes Mellitus (poorly controlled, diagnosed 2 years prior)
- Hypothyroidism (acquired)
- Gastroesophageal reflux disease (GERD)
- Osteoarthritis (bilateral knees)
- Tobacco use disorder

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (5 years prior)
- Right femoral hernia repair with mesh (recent)

MEDICATIONS:
Home Medications:
- Levothyroxine 100 mcg PO daily
- Omeprazole 20 mg PO daily
- Metformin 1000 mg PO BID

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: deceased at age 68 (cardiac disease)
- Mother: type 2 diabetes mellitus, hypothyroidism
- Sister: obesity, smoker

SOCIAL HISTORY:
Smoking: Current smoker, 1.5 packs per day for 23 years (34.5 pack-years), with intermittent periods of reduced consumption. Alcohol: None. Occupation: Elementary school teacher.

REVIEW OF SYSTEMS:
She denies cough, wheezing, or shortness of breath.

VITAL SIGNS:
Her vital signs are within normal limits

PHYSICAL EXAMINATION:
Cardiovascular examination is unremarkable

OTHER STUDIES:
An electrocardiogram shows normal sinus rhythm with no ischemic changes. Upper endoscopy reveals mild erythema of the distal esophagus, but no ulcers or masses.

## Encounter 2: source 2020-07-15, practice 2022-09-30, outpatient, Orthopedics & Sports Medicine

Chief complaint: Two-month history of pain and swelling over the upper medial aspect of her right tibia that worsens with jogging and improves with rest; recently increased running distance for half marathon training.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Jennifer Park, MD
Department: Orthopedics & Sports Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Two-month history of pain and swelling over the upper medial aspect of her right tibia that worsens with jogging and improves with rest; recently increased running distance for half marathon training.

HISTORY OF PRESENT ILLNESS:
The patient returns with a new complaint, evaluated in January 2020 for knee pain. She reports that the pain worsens with activity, especially after jogging, and improves with rest. She has recently increased her running distance in preparation for a half marathon.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis of knee (diagnosed 2020-01-15)
- Gastroesophageal reflux disease (diagnosed 2020-03-15)
- Obesity (Class II, BMI 36 kg/m²)
- Type 2 Diabetes Mellitus (poorly controlled, diagnosed 2 years prior)
- Hypothyroidism (acquired)
- Gastroesophageal reflux disease (GERD)
- Osteoarthritis (bilateral knees)
- Tobacco use disorder

She has a history of osteoarthritis.

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (5 years prior)
- Right femoral hernia repair with mesh (recent)

MEDICATIONS:
Home Medications:
- Levothyroxine 100 mcg PO daily
- Omeprazole 20 mg PO daily
- Metformin 1000 mg PO BID

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: deceased at age 68 (cardiac disease)
- Mother: type 2 diabetes mellitus, hypothyroidism
- Sister: obesity, smoker

SOCIAL HISTORY:
Smoking: Current smoker, 1.5 packs per day for 23 years (34.5 pack-years), with intermittent periods of reduced consumption. Alcohol: None. Occupation: Elementary school teacher.

VITAL SIGNS:
The patient's BMI is 32 kg/m^2

PHYSICAL EXAMINATION:
On physical examination, there is tenderness to palpation over the medial knee, just below the joint line. There is no erythema or warmth, but a small, nonfluctuant swelling is noted. Knee range of motion is full, but pain is elicited with resisted knee flexion.

IMAGING:
Radiographs of the knee show no fractures or significant joint space narrowing.

## Encounter 3: source 2021-01-15, practice 2023-04-10, outpatient, Obstetrics & Gynecology

Chief complaint: Desires contraceptive counseling specifically regarding initiating the contraceptive patch; has regular monthly periods and is not currently using birth control.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Amara Okafor, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Desires contraceptive counseling specifically regarding initiating the contraceptive patch; has regular monthly periods and is not currently using birth control.

HISTORY OF PRESENT ILLNESS:
The patient is a 44-year-old female with Type 2 Diabetes Mellitus and Obesity presenting for gynecologic consultation following recent orthopedic care. She has regular monthly periods and is not currently on any birth control. She recently attended a bachelorette party where several of her friends were discussing how easy and effective the contraceptive patch is, and now she is interested in starting it. She wants something that doesn't require daily attention and expresses concern about gaining weight with injectable contraception.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis of knee (diagnosed 2020-01-15)
- Gastroesophageal reflux disease (diagnosed 2020-03-15)
- Pes anserinus pain syndrome (diagnosed 2020-07-15)
- Obesity (Class II, BMI 36 kg/m²)
- Type 2 Diabetes Mellitus (poorly controlled, diagnosed 2 years prior)
- Hypothyroidism (acquired)
- Gastroesophageal reflux disease (GERD)
- Osteoarthritis (bilateral knees)
- Tobacco use disorder

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (5 years prior)
- Right femoral hernia repair with mesh (recent)

MEDICATIONS:
Home Medications:
- Levothyroxine 100 mcg PO daily
- Omeprazole 20 mg PO daily
- Metformin 1000 mg PO BID

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: deceased at age 68 (cardiac disease)
- Mother: type 2 diabetes mellitus, hypothyroidism
- Sister: obesity, smoker

SOCIAL HISTORY:
She is in a monogamous relationship and would prefer not to use condoms. However, she has smoked approximately 1.5 packs of cigarettes per day since the age of 20. She reports that her job in event coordination is highly stressful and that she often smokes more when under pressure.

VITAL SIGNS:
On examination, her blood pressure is 124/82 mm Hg, pulse is 74/min, and BMI is 25.

PHYSICAL EXAMINATION:
She has no signs of hirsutism, acne, or other hyperandrogenic features.

## Encounter 4: source 2021-07-15, practice 2023-10-13, outpatient, General Surgery

Chief complaint: Three-month history of a bulge in her right groin that gradually increases in size, more prominent with coughing or standing and reduces when lying down; denies associated pain.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. James Sullivan, MD
Department: General Surgery
Visit Type: outpatient

CHIEF COMPLAINT:
Three-month history of a bulge in her right groin that gradually increases in size, more prominent with coughing or standing and reduces when lying down; denies associated pain.

HISTORY OF PRESENT ILLNESS:
This 44-year-old female with obesity, type 2 diabetes mellitus, and osteoarthritis presents with a groin bulge. She reports that the bulge becomes more prominent when she coughs or stands for long periods and reduces in size when she lies down. She denies any pain associated with the bulge.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis of knee (diagnosed 2020-01-15)
- Gastroesophageal reflux disease (diagnosed 2020-03-15)
- Pes anserinus pain syndrome (diagnosed 2020-07-15)
- Malignancy-associated hypercoagulable state (diagnosed 2021-01-15)
- Obesity (Class II, BMI 36 kg/m²)
- Type 2 Diabetes Mellitus (poorly controlled, diagnosed 2 years prior)
- Hypothyroidism (acquired)
- Gastroesophageal reflux disease (GERD)
- Osteoarthritis (bilateral knees)
- Tobacco use disorder

Her past medical history is significant for obesity and hypothyroidism, which is well-controlled.

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (5 years prior)
- Right femoral hernia repair with mesh (recent)

MEDICATIONS:
She takes on levothyroxine.

Home Medications:
- Omeprazole 20 mg PO daily
- Metformin 1000 mg PO BID

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: deceased at age 68 (cardiac disease)
- Mother: type 2 diabetes mellitus, hypothyroidism
- Sister: obesity, smoker

SOCIAL HISTORY:
Smoking: Current smoker, 1.5 packs per day for 23 years (34.5 pack-years), with intermittent periods of reduced consumption. Alcohol: None. Occupation: Elementary school teacher.

VITAL SIGNS:
The patient&#x27;s vital signs are within normal limits.

PHYSICAL EXAMINATION:
On physical examination, a non-tender, non-pulsatile mass is palpated below the inguinal ligament on the right side. The mass is not reducible. Valsalva maneuver increases the size of the mass. Abdominal examination is otherwise unremarkable. No bowel sounds are heard over the mass.

LABORATORY DATA:
Laboratory values:
- Hemoglobin: 13.2 g/dL
- White blood cell count: 7,500/mm^3
- Platelets: 250,000/mm^3

## Encounter 5: source 2022-05-15, practice 2024-08-01, outpatient, Gastroenterology

Chief complaint: Intermittent right upper quadrant abdominal pain radiating to her back for 6 months; describes pain as crampy and occasionally associated with meals.

ENCOUNTER NOTE
Date: 2022-05-15
Provider: Dr. Priya Patel, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent right upper quadrant abdominal pain radiating to her back for 6 months; describes pain as crampy and occasionally associated with meals.

HISTORY OF PRESENT ILLNESS:
This 45-year-old female with obesity, type 2 diabetes mellitus, hypothyroidism, and prior gastroenterology evaluation for GERD presents with a 6-month history of intermittent right upper quadrant abdominal pain that radiates to her back. She describes the pain as crampy and occasionally associated with meals.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis of knee (diagnosed 2020-01-15)
- Gastroesophageal reflux disease (diagnosed 2020-03-15)
- Pes anserinus pain syndrome (diagnosed 2020-07-15)
- Malignancy-associated hypercoagulable state (diagnosed 2021-01-15)
- Femoral hernia, right side, incarcerated (diagnosed 2021-07-15)
- Obesity (Class II, BMI 36 kg/m²)
- Type 2 Diabetes Mellitus (poorly controlled, diagnosed 2 years prior)
- Hypothyroidism (acquired)
- Gastroesophageal reflux disease (GERD)
- Osteoarthritis (bilateral knees)
- Tobacco use disorder

Her past medical history is significant for gastroesophageal reflux disease.

PAST SURGICAL HISTORY:
She underwent a laparoscopic cholecystectomy 18 months ago.

MEDICATIONS:
She takes omeprazole.

Home Medications:
- Levothyroxine 100 mcg PO daily
- Metformin 1000 mg PO BID

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: deceased at age 68 (cardiac disease)
- Mother: type 2 diabetes mellitus, hypothyroidism
- Sister: obesity, smoker

SOCIAL HISTORY:
Smoking: Current smoker, 1.5 packs per day for 23 years (34.5 pack-years), with intermittent periods of reduced consumption. Alcohol: None. Occupation: Elementary school teacher.

VITAL SIGNS:
Vital signs are within normal limits

PHYSICAL EXAMINATION:
Abdominal examination reveals mild tenderness to palpation in the right upper quadrant without rebound or guarding.

LABORATORY DATA:
Laboratory tests show normal liver function tests and lipase.

IMAGING:
An abdominal ultrasound reveals no gallstones or biliary ductal dilatation.

## Encounter 6: source 2023-01-15, practice 2025-04-04, outpatient, Gastroenterology

Chief complaint: Worsening difficulty swallowing over the past 6 months affecting both solids and liquids with nocturnal regurgitation of undigested food; unintentional weight loss of 4.5 kg.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. David Kim, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening difficulty swallowing over the past 6 months affecting both solids and liquids with nocturnal regurgitation of undigested food; unintentional weight loss of 4.5 kg.

HISTORY OF PRESENT ILLNESS:
The patient, a 46-year-old female with obesity, type 2 diabetes mellitus, and history of postcholecystectomy syndrome, returns with symptoms. She reports difficulty with both solids and liquids and often regurgitates undigested food, especially at night. She has unintentionally lost 4.5 kg (10 lb) over the past several months. She has no history of heartburn, choking, or coughing spells.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis of knee (diagnosed 2020-01-15)
- Gastroesophageal reflux disease (diagnosed 2020-03-15)
- Pes anserinus pain syndrome (diagnosed 2020-07-15)
- Malignancy-associated hypercoagulable state (diagnosed 2021-01-15)
- Femoral hernia, right side, incarcerated (diagnosed 2021-07-15)
- Postcholecystectomy syndrome (diagnosed 2022-05-15)
- Obesity (Class II, BMI 36 kg/m²)
- Type 2 Diabetes Mellitus (poorly controlled, diagnosed 2 years prior)
- Hypothyroidism (acquired)
- Gastroesophageal reflux disease (GERD)
- Osteoarthritis (bilateral knees)
- Tobacco use disorder

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (5 years prior)
- Right femoral hernia repair with mesh (recent)

MEDICATIONS:
Home Medications:
- Levothyroxine 100 mcg PO daily
- Omeprazole 20 mg PO daily
- Metformin 1000 mg PO BID

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: deceased at age 68 (cardiac disease)
- Mother: type 2 diabetes mellitus, hypothyroidism
- Sister: obesity, smoker

SOCIAL HISTORY:
She has smoked one pack of cigarettes daily for the past 10 years.

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

IMAGING:
Barium swallow results are shown.

## Encounter 7: source 2023-07-15, practice 2025-10-14, ed, Emergency Medicine

Chief complaint: Severe epigastric pain radiating to her back with nausea and vomiting for 24 hours; pain is constant and worsens after eating.

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Emily Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe epigastric pain radiating to her back with nausea and vomiting for 24 hours; pain is constant and worsens after eating.

HISTORY OF PRESENT ILLNESS:
This 46-year-old female with obesity, poorly controlled type 2 diabetes mellitus, and prior postcholecystectomy syndrome presents to the emergency department with a 24-hour history of severe epigastric pain radiating to her back, nausea, and vomiting. She reports that the pain is constant and worsens after eating.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis of knee (diagnosed 2020-01-15)
- Gastroesophageal reflux disease (diagnosed 2020-03-15)
- Pes anserinus pain syndrome (diagnosed 2020-07-15)
- Malignancy-associated hypercoagulable state (diagnosed 2021-01-15)
- Femoral hernia, right side, incarcerated (diagnosed 2021-07-15)
- Postcholecystectomy syndrome (diagnosed 2022-05-15)
- Achalasia (diagnosed 2023-01-15)
- Obesity (Class II, BMI 36 kg/m²)
- Type 2 Diabetes Mellitus (poorly controlled, diagnosed 2 years prior)
- Hypothyroidism (acquired)
- Gastroesophageal reflux disease (GERD)
- Osteoarthritis (bilateral knees)
- Tobacco use disorder

She has a history of poorly controlled type 2 diabetes mellitus and obesity.

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (5 years prior)
- Right femoral hernia repair with mesh (recent)

MEDICATIONS:
Home Medications:
- Levothyroxine 100 mcg PO daily
- Omeprazole 20 mg PO daily
- Metformin 1000 mg PO BID

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: deceased at age 68 (cardiac disease)
- Mother: type 2 diabetes mellitus, hypothyroidism
- Sister: obesity, smoker

SOCIAL HISTORY:
She denies alcohol use but mentions that her diet is high in fats and carbohydrates.

VITAL SIGNS:
On physical examination, her temperature is 37.8°C (100°F), blood pressure is 135/85 mmHg, and heart rate is 102 beats per minute. Her BMI is 35 kg/m^2.

PHYSICAL EXAMINATION:
Abdominal examination reveals epigastric tenderness without rebound or guarding. There is no jaundice or scleral icterus. A skin examination shows multiple, small, yellowish papules over the extensor surfaces of her arms and legs.

LABORATORY DATA:
Laboratory tests reveal:
- Serum amylase: 450 U/L (Normal: 30-110 U/L)
- Serum lipase: 900 U/L (Normal: 0-160 U/L)
- Fasting serum lipid profile:
- Total cholesterol: 320 mg/dL (Normal: 40 mg/dL)
- LDL cholesterol: 190 mg/dL (Normal: <100 mg/dL)
