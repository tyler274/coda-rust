# synthetic_hospital patient 2574

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 19 at first encounter, sex M, African American, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2019-06-10 (Mon) | outpatient | Hematology & Oncology |
| 1 | 2020-04-15 | 2019-09-16 (Mon) | ed | Emergency Medicine |
| 2 | 2021-01-15 | 2020-06-15 (Mon) | outpatient | Orthopedics |
| 3 | 2022-07-15 | 2021-11-29 (Mon) | outpatient | Internal Medicine |
| 4 | 2026-01-15 | 2025-06-13 (Fri) | ed | Emergency Medicine |
| 5 | 2026-07-15 | 2025-12-12 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicaid",
 "occupation": "Part-time library assistant, community college student",
 "alcohol_use": "Rare (special occasions only), avoids due to dehydration risk",
 "family_history": [
  "Mother: Sickle cell trait",
  "Father: Sickle cell trait",
  "Younger brother: Sickle cell disease"
 ],
 "race_ethnicity": "African American",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1 mg oral daily",
   "name": "Folic acid"
  },
  {
   "dose": "250 mg oral twice daily",
   "name": "Penicillin VK"
  },
  {
   "dose": "1000 mg oral daily",
   "name": "Hydroxyurea"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Sickle cell anemia (HbSS genotype)",
  "Avascular necrosis of right femoral head",
  "Functional asplenia",
  "History of recurrent acute chest syndrome"
 ],
 "age_at_first_encounter": 19
}
```

Primary diagnoses: ["Sickle cell anemia without crisis", "Sickle cell disease with vaso-occlusive crisis", "Folate deficiency anemia", "Splenic laceration", "Osteonecrosis of right femoral head (avascular necrosis)", "Delayed hemolytic transfusion reaction"]
Comorbidities: ["Sickle cell anemia without crisis", "Leukocytosis", "Hemolytic anemia", "Splenomegaly"]

## Encounter 0: source 2020-01-15, practice 2019-06-10, outpatient, Hematology & Oncology

Chief complaint: Routine follow-up for sickle cell disease with recent frequent hospitalizations

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Amina Patel, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for sickle cell disease with recent frequent hospitalizations

HISTORY OF PRESENT ILLNESS:
He reports having experienced four painful vaso-occlusive crises requiring hospitalization in the past year. His last hospitalization was two months ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Sickle cell anemia (HbSS genotype)
- Avascular necrosis of right femoral head
- Functional asplenia
- History of recurrent acute chest syndrome

MEDICATIONS:
He is currently taking folic acid and penicillin prophylaxis.

Home Medications:
- Penicillin VK 250 mg oral twice daily
- Hydroxyurea 1000 mg oral daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Sickle cell trait
- Father: Sickle cell trait
- Younger brother: Sickle cell disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (special occasions only), avoids due to dehydration risk. Occupation: Part-time library assistant, community college student.

VITAL SIGNS:
His temperature is 37.2°C (99°F), blood pressure is 128/76 mmHg, pulse is 88/min, and respiratory rate is 16/min.

PHYSICAL EXAMINATION:
On examination, he appears well. His spleen is non-palpable, and there are no signs of acute distress.

LABORATORY DATA:
Laboratory values are as follows:
- Hemoglobin: 8.2 g/dL
- Hematocrit: 24.5%
- White blood cell count: 12,000/mm³
- Platelet count: 350,000/mm³
- Reticulocyte count: 10% (elevated)
- Fetal hemoglobin (HbF): 5%

ASSESSMENT AND PLAN:
Given the patient's history and current management,

## Encounter 1: source 2020-04-15, practice 2019-09-16, ed, Emergency Medicine

Chief complaint: Severe deep bone pain in extremities triggered by cold weather

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. James Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe deep bone pain in extremities triggered by cold weather

HISTORY OF PRESENT ILLNESS:
The patient is a 19-year-old male with HbSS sickle cell disease, functional asplenia, and avascular necrosis of the right femoral head who presents to the emergency department three months following his January 2020 hematology outpatient visit for routine follow-up. He reports a history of recurrent painful episodes in his extremities, which he describes as "deep bone pain." He states that these episodes have been occurring since childhood and are often triggered by cold weather or dehydration.

PAST MEDICAL HISTORY:
Active Problem List:
- Sickle cell anemia without crisis (diagnosed 2020-01-15)
- Sickle cell anemia (HbSS genotype)
- Avascular necrosis of right femoral head
- Functional asplenia
- History of recurrent acute chest syndrome

His past medical history is significant for sickle cell anemia, and he has been hospitalized twice in the past year for acute chest syndrome.

MEDICATIONS:
Home Medications:
- Folic acid 1 mg oral daily
- Penicillin VK 250 mg oral twice daily
- Hydroxyurea 1000 mg oral daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Sickle cell trait
- Father: Sickle cell trait
- Younger brother: Sickle cell disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (special occasions only), avoids due to dehydration risk. Occupation: Part-time library assistant, community college student.

VITAL SIGNS:
His temperature is 37.2°C, blood pressure is 128/76 mmHg, pulse is 88 beats per minute, and respiratory rate is 18 breaths per minute. His oxygen saturation is 97% on room air.

PHYSICAL EXAMINATION:
On examination, he appears to be in mild distress.

LABORATORY DATA:
Laboratory tests reveal the following:

- Hemoglobin: 8.2 g/dL
- Hematocrit: 24.5%
- Reticulocyte count: 4.5% (elevated)
- Lactate dehydrogenase (LDH): 350 U/L (elevated)
- Indirect bilirubin: 2.1 mg/dL (elevated)
- Peripheral smear: Sickled erythrocytes present

## Encounter 2: source 2021-01-15, practice 2020-06-15, outpatient, Orthopedics

Chief complaint: Persistent right groin pain with ambulation worsening over six months

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Kwame Okafor, MD
Department: Orthopedics
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent right groin pain with ambulation worsening over six months

HISTORY OF PRESENT ILLNESS:
This 20-year-old male with HbSS sickle cell anemia and right femoral head avascular necrosis returns for orthopedic follow-up after Hematology and ED encounters. He presents with a 6-month history of persistent right groin pain, describing it as a dull ache worsening with walking and somewhat relieved by rest. He denies any recent trauma, fever, or weight loss.

PAST MEDICAL HISTORY:
Active Problem List:
- Sickle cell anemia without crisis (diagnosed 2020-01-15)
- Sickle cell disease with vaso-occlusive crisis (diagnosed 2020-04-15)
- Sickle cell anemia (HbSS genotype)
- Avascular necrosis of right femoral head
- Functional asplenia
- History of recurrent acute chest syndrome

His past medical history is significant for sickle cell disease, for which he has been hospitalized twice in the past year for vaso-occlusive crises.

MEDICATIONS:
Home Medications:
- Folic acid 1 mg oral daily
- Penicillin VK 250 mg oral twice daily
- Hydroxyurea 1000 mg oral daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Sickle cell trait
- Father: Sickle cell trait
- Younger brother: Sickle cell disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (special occasions only), avoids due to dehydration risk. Occupation: Part-time library assistant, community college student.

PHYSICAL EXAMINATION:
Physical examination reveals a full range of motion in the right hip, but the patient reports pain with internal rotation. There is no warmth, erythema, or swelling of the joint.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 8.5 g/dL (Normal: 13.8-17.2 g/dL)
- Leukocyte count: 12,000/mm³ (Normal: 4,500-11,000/mm³)
- Platelet count: 450,000/mm³ (Normal: 150,000-450,000/mm³)
- Erythrocyte sedimentation rate (ESR): 20 mm/hr (Normal: 0-22 mm/hr)
- C-reactive protein (CRP): 2.0 mg/L (Normal: <10.0 mg/L)

IMAGING:
Radiographs of the right hip show irregularities in the femoral head's contour with subchondral sclerosis but no evidence of fracture or dislocation.

## Encounter 3: source 2022-07-15, practice 2021-11-29, outpatient, Internal Medicine

Chief complaint: Progressive fatigue and shortness of breath

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Sarah Kim, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue and shortness of breath

HISTORY OF PRESENT ILLNESS:
The patient with HbSS and prior right hip avascular necrosis returns with progressive fatigue and shortness of breath. He reports no sickle cell crisis in the past six months, denying any recent infections, bleeding, or dietary changes.

PAST MEDICAL HISTORY:
Active Problem List:
- Sickle cell anemia without crisis (diagnosed 2020-01-15)
- Sickle cell disease with vaso-occlusive crisis (diagnosed 2020-04-15)
- Osteonecrosis of right femoral head (avascular necrosis) (diagnosed 2021-01-15)
- Sickle cell anemia (HbSS genotype)
- Avascular necrosis of right femoral head
- Functional asplenia
- History of recurrent acute chest syndrome

He has a known history of sickle cell disease.

MEDICATIONS:
Home Medications:
- Folic acid 1 mg oral daily
- Penicillin VK 250 mg oral twice daily
- Hydroxyurea 1000 mg oral daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Sickle cell trait
- Father: Sickle cell trait
- Younger brother: Sickle cell disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (special occasions only), avoids due to dehydration risk. Occupation: Part-time library assistant, community college student.

PHYSICAL EXAMINATION:
On examination, he appears pale and jaundiced. His spleen is palpable 2 cm below the left costal margin.

LABORATORY DATA:
Hemoglobin: 7.8 g/dL (Normal: 13.8-17.2 g/dL)
- Mean corpuscular volume (MCV): 102 fL (Normal: 80-96 fL)
- Reticulocyte count: 1% (Normal: 0.5-1.5%)
- Serum folate: 2 ng/mL (Normal: 3-17 ng/mL)
- Serum lactate dehydrogenase (LDH): Elevated
- Indirect bilirubin: Elevated
- Haptoglobin: Decreased

## Encounter 4: source 2026-01-15, practice 2025-06-13, ed, Emergency Medicine

Chief complaint: Sudden severe left upper quadrant abdominal pain radiating to left shoulder

ENCOUNTER NOTE
Date: 2026-01-15
Provider: Dr. Roberto Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden severe left upper quadrant abdominal pain radiating to left shoulder

HISTORY OF PRESENT ILLNESS:
Patient with HbSS and functional asplenia presents with left upper quadrant pain. He reports having had a sore throat, fever, and fatigue for the past week, which improved with rest and hydration. He denies any recent trauma or injury.

PAST MEDICAL HISTORY:
Active Problem List:
- Sickle cell anemia without crisis (diagnosed 2020-01-15)
- Sickle cell disease with vaso-occlusive crisis (diagnosed 2020-04-15)
- Osteonecrosis of right femoral head (avascular necrosis) (diagnosed 2021-01-15)
- Folate deficiency anemia (diagnosed 2022-07-15)
- Sickle cell anemia (HbSS genotype)
- Avascular necrosis of right femoral head
- Functional asplenia
- History of recurrent acute chest syndrome

MEDICATIONS:
Home Medications:
- Folic acid 1 mg oral daily
- Penicillin VK 250 mg oral twice daily
- Hydroxyurea 1000 mg oral daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Sickle cell trait
- Father: Sickle cell trait
- Younger brother: Sickle cell disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (special occasions only), avoids due to dehydration risk. Occupation: Part-time library assistant, community college student.

VITAL SIGNS:
His vital signs are: blood pressure 90/60 mmHg, heart rate 120 beats per minute, respiratory rate 22 breaths per minute, and temperature 37.8°C (100°F).

PHYSICAL EXAMINATION:
On examination, he appears pale and diaphoretic. Abdominal examination reveals tenderness in the left upper quadrant with guarding. Bowel sounds are normal. There is no rebound tenderness or rigidity.

LABORATORY DATA:
Laboratory tests reveal:

- Hemoglobin: 9.8 g/dL (Normal: 13.8-17.2 g/dL)
- White blood cell count: 15,000 /µL (Normal: 4,500-11,000 /µL)
- Platelets: 250,000 /µL (Normal: 150,000-450,000 /µL)
- AST: 45 U/L (Normal: 10-40 U/L)
- ALT: 50 U/L (Normal: 7-56 U/L)
- Monospot test: Positive

IMAGING:
An abdominal ultrasound shows an enlarged spleen with a hypoechoic area suggestive of a subcapsular hematoma.

## Encounter 5: source 2026-07-15, practice 2025-12-12, ed, Emergency Medicine

Chief complaint: Worsening fatigue, jaundice, and dark urine following recent transfusion

ENCOUNTER NOTE
Date: 2026-07-15
Provider: Dr. Emily Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening fatigue, jaundice, and dark urine following recent transfusion

HISTORY OF PRESENT ILLNESS:
Returning six months after splenic laceration, the patient with HbSS sickle cell disease and functional asplenia presents to the emergency department with a 2-day history of increasing fatigue, jaundice, and dark urine. He reports receiving a red blood cell transfusion 10 days ago due to a sickle cell crisis.

PAST MEDICAL HISTORY:
Active Problem List:
- Sickle cell anemia without crisis (diagnosed 2020-01-15)
- Sickle cell disease with vaso-occlusive crisis (diagnosed 2020-04-15)
- Osteonecrosis of right femoral head (avascular necrosis) (diagnosed 2021-01-15)
- Folate deficiency anemia (diagnosed 2022-07-15)
- Splenic laceration (diagnosed 2026-01-15)
- Sickle cell anemia (HbSS genotype)
- Avascular necrosis of right femoral head
- Functional asplenia
- History of recurrent acute chest syndrome

MEDICATIONS:
Home Medications:
- Folic acid 1 mg oral daily
- Penicillin VK 250 mg oral twice daily
- Hydroxyurea 1000 mg oral daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Sickle cell trait
- Father: Sickle cell trait
- Younger brother: Sickle cell disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (special occasions only), avoids due to dehydration risk. Occupation: Part-time library assistant, community college student.

VITAL SIGNS:
His temperature is 37.8°C (100°F), blood pressure is 130/80 mmHg, pulse is 105/min, and respiratory rate is 18/min.

PHYSICAL EXAMINATION:
Physical examination reveals scleral icterus and mild splenomegaly. There is no rash or lymphadenopathy.

LABORATORY DATA:
Laboratory values are as follows:

- Hemoglobin: 7 g/dL (baseline 9 g/dL)
- Total bilirubin: 3.5 mg/dL (direct bilirubin 0.8 mg/dL)
- Lactate dehydrogenase (LDH): 600 U/L
- Haptoglobin: <10 mg/dL
- Reticulocyte count: 15% (elevated)
- Direct Coombs test: Positive (previously negative)
