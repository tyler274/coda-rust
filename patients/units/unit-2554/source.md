# synthetic_hospital patient 2554

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 19 at first encounter, sex M, African American, Private insurance via parental coverage.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-12-10 (Fri) | outpatient | Hematology |
| 1 | 2020-04-15 | 2022-03-04 (Fri) | outpatient | Gastroenterology |
| 2 | 2020-07-15 | 2022-06-21 (Tue) | outpatient | Orthopedics |
| 3 | 2022-01-15 | 2023-12-21 (Thu) | inpatient | Emergency Medicine |
| 4 | 2022-03-15 | 2024-02-13 (Tue) | outpatient | Hematology |
| 5 | 2022-07-15 | 2024-06-12 (Wed) | ed | Emergency Medicine |
| 6 | 2023-01-15 | 2024-12-19 (Thu) | ed | Emergency Medicine |
| 7 | 2024-01-15 | 2025-12-21 (Sun) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private insurance via parental coverage",
 "occupation": "University student (sophomore year)",
 "alcohol_use": "Denies alcohol use",
 "family_history": [
  "Mother with sickle cell trait (HbAS)",
  "Father with sickle cell trait (HbAS)",
  "Maternal grandmother with asthma and environmental allergies",
  "Older brother with sickle cell disease"
 ],
 "race_ethnicity": "African American",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1000 mg orally daily",
   "name": "Hydroxyurea"
  },
  {
   "dose": "1 mg orally daily",
   "name": "Folic acid"
  },
  {
   "dose": "90 mcg, 2 inhalations every 4-6 hours as needed for wheezing or shortness of breath",
   "name": "Albuterol HFA inhaler"
  }
 ],
 "surgical_history": [
  "Laparoscopic splenectomy (complication of splenic sequestration/infarction)",
  "Tonsillectomy and adenoidectomy (age 6)"
 ],
 "chronic_conditions": [
  "Sickle cell anemia (HbSS genotype)",
  "Status post splenectomy (secondary to atraumatic splenic rupture)",
  "Asplenia",
  "Mild intermittent asthma",
  "Seasonal allergic rhinitis",
  "Eosinophilic esophagitis",
  "Avascular necrosis of bilateral femoral heads",
  "Chronic anemia (baseline hemoglobin 7-8 g/dL)",
  "History of acute chest syndrome (multiple episodes)"
 ],
 "age_at_first_encounter": 19
}
```

Primary diagnoses: ["Atraumatic splenic rupture with hemoperitoneum", "Avascular necrosis of the femoral head", "Overwhelming post-splenectomy infection (OPSI) risk", "Ischemic priapism secondary to sickle cell disease", "Eosinophilic esophagitis", "Sickle cell anemia without crisis", "Septic arthritis"]
Comorbidities: ["Anemia", "Sickle Cell Disease", "Sickle cell anemia without crisis", "Asthma"]

## Encounter 0: source 2020-01-15, practice 2021-12-10, outpatient, Hematology

Chief complaint: Evaluation of chronic fatigue and increased susceptibility to respiratory infections.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Hematology
Visit Type: outpatient

CHIEF COMPLAINT:
Evaluation of chronic fatigue and increased susceptibility to respiratory infections.

HISTORY OF PRESENT ILLNESS:
He reports feeling generally well between episodes but describes a history of intermittent joint pain, particularly in the hips and knees, which flares unpredictably and is sometimes associated with low-grade fever. He denies any recent travel, sick contacts, or weight loss. He has been hospitalized several times since childhood for unexplained episodes of severe limb pain and abdominal discomfort, often requiring intravenous fluids and analgesics. He reports that these episodes have become less frequent over the past few years, but he still occasionally experiences milder bouts of bone pain. He has not seen a primary care physician in several years.

PAST MEDICAL HISTORY:
Active Problem List:
- Sickle cell anemia (HbSS genotype)
- Status post splenectomy (secondary to atraumatic splenic rupture)
- Asplenia
- Mild intermittent asthma
- Seasonal allergic rhinitis
- Eosinophilic esophagitis
- Avascular necrosis of bilateral femoral heads
- Chronic anemia (baseline hemoglobin 7-8 g/dL)
- History of acute chest syndrome (multiple episodes)

His past medical history is notable for chronic anemia and a remote history of bacterial meningitis during childhood. Vaccination records are incomplete, but he recalls receiving “extra shots” as a child.

PAST SURGICAL HISTORY:
- Laparoscopic splenectomy (complication of splenic sequestration/infarction)
- Tonsillectomy and adenoidectomy (age 6)

MEDICATIONS:
Home Medications:
- Hydroxyurea 1000 mg orally daily
- Folic acid 1 mg orally daily
- Albuterol HFA inhaler 90 mcg, 2 inhalations every 4-6 hours as needed for wheezing or shortness of breath

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with sickle cell trait (HbAS)
- Father with sickle cell trait (HbAS)
- Maternal grandmother with asthma and environmental allergies
- Older brother with sickle cell disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Denies alcohol use. Occupation: University student (sophomore year).

VITAL SIGNS:
On physical exam, his temperature is 37.0°C (98.6°F), blood pressure is 108/64 mmHg, and pulse is 92/min.

PHYSICAL EXAMINATION:
He appears thin but in no acute distress. Conjunctivae are pale, and sclerae are anicteric. No cervical lymphadenopathy is present. Cardiopulmonary examination is normal. Abdominal examination reveals no hepatosplenomegaly, and the spleen is not palpable.

LABORATORY DATA:
Laboratory studies show:
Hemoglobin 8.4 g/dL
Reticulocyte count 11%
Platelet count 612,000/mm³
Total bilirubin 2.1 mg/dL
Lactate dehydrogenase 482 U/L

OTHER STUDIES:
Peripheral smear is shown.

## Encounter 1: source 2020-04-15, practice 2022-03-04, outpatient, Gastroenterology

Chief complaint: Intermittent difficulty swallowing solid foods and sensation of food getting stuck in chest for 6 months.

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. James Okafor, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent difficulty swallowing solid foods and sensation of food getting stuck in chest for 6 months.

HISTORY OF PRESENT ILLNESS:
This 19-year-old male with sickle cell anemia (HbSS) and asplenia, last evaluated in Hematology three months ago for fatigue, now presents to Gastroenterology with a 6-month history of intermittent difficulty swallowing solid foods and a sensation that food is getting stuck in his chest. He denies any weight loss, gastrointestinal bleeding, or changes in bowel habits.

PAST MEDICAL HISTORY:
Active Problem List:
- Sickle cell anemia without crisis (diagnosed 2020-01-15)
- Sickle cell anemia (HbSS genotype)
- Status post splenectomy (secondary to atraumatic splenic rupture)
- Asplenia
- Mild intermittent asthma
- Seasonal allergic rhinitis
- Eosinophilic esophagitis
- Avascular necrosis of bilateral femoral heads
- Chronic anemia (baseline hemoglobin 7-8 g/dL)
- History of acute chest syndrome (multiple episodes)

PAST SURGICAL HISTORY:
- Laparoscopic splenectomy (complication of splenic sequestration/infarction)
- Tonsillectomy and adenoidectomy (age 6)

MEDICATIONS:
He takes albuterol inhaler as needed.

Home Medications:
- Hydroxyurea 1000 mg orally daily
- Folic acid 1 mg orally daily
- Albuterol HFA inhaler 90 mcg, 2 inhalations every 4-6 hours as needed for wheezing or shortness of breath

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with sickle cell trait (HbAS)
- Father with sickle cell trait (HbAS)
- Maternal grandmother with asthma and environmental allergies
- Older brother with sickle cell disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Denies alcohol use. Occupation: University student (sophomore year).

VITAL SIGNS:
His vital signs are within normal limits

PHYSICAL EXAMINATION:
There is no lymphadenopathy or abdominal tenderness. His skin examination reveals several patches of dry, scaly skin consistent with eczema.

PATHOLOGY:
Biopsy of the esophageal mucosa reveals significant eosinophilic infiltration.

OTHER STUDIES:
An upper endoscopy is performed, which shows multiple concentric rings and linear furrows in the esophagus.

## Encounter 2: source 2020-07-15, practice 2022-06-21, outpatient, Orthopedics

Chief complaint: Progressive left hip pain over 3 weeks, worse with weight-bearing.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Maria Rodriguez, MD
Department: Orthopedics
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive left hip pain over 3 weeks, worse with weight-bearing.

HISTORY OF PRESENT ILLNESS:
The patient with known bilateral femoral head avascular necrosis returns with hip pain. He describes the pain as dull and constant, with occasional sharp exacerbations. The pain worsens with weight-bearing activities and improves with rest. He denies any recent trauma or injury.

PAST MEDICAL HISTORY:
Active Problem List:
- Sickle cell anemia without crisis (diagnosed 2020-01-15)
- Eosinophilic esophagitis (diagnosed 2020-04-15)
- Sickle cell anemia (HbSS genotype)
- Status post splenectomy (secondary to atraumatic splenic rupture)
- Asplenia
- Mild intermittent asthma
- Seasonal allergic rhinitis
- Avascular necrosis of bilateral femoral heads
- Chronic anemia (baseline hemoglobin 7-8 g/dL)
- History of acute chest syndrome (multiple episodes)

His medical history is significant for sickle cell disease, for which he has been hospitalized twice in the past year for vaso-occlusive crises.

PAST SURGICAL HISTORY:
- Laparoscopic splenectomy (complication of splenic sequestration/infarction)
- Tonsillectomy and adenoidectomy (age 6)

MEDICATIONS:
Home Medications:
- Hydroxyurea 1000 mg orally daily
- Folic acid 1 mg orally daily
- Albuterol HFA inhaler 90 mcg, 2 inhalations every 4-6 hours as needed for wheezing or shortness of breath

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with sickle cell trait (HbAS)
- Father with sickle cell trait (HbAS)
- Maternal grandmother with asthma and environmental allergies
- Older brother with sickle cell disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Denies alcohol use. Occupation: University student (sophomore year).

PHYSICAL EXAMINATION:
On physical examination, the patient has a reduced range of motion in the left hip, particularly with internal rotation and abduction. There is no erythema, warmth, or swelling over the joint.

LABORATORY DATA:
Laboratory values are as follows:

- Hemoglobin: 8.5 g/dL (Normal: 13.8-17.2 g/dL)
- Hematocrit: 25.5% (Normal: 40.7-50.3%)
- White blood cell count: 12,000 /µL (Normal: 4,500-11,000 /µL)
- Platelet count: 450,000 /µL (Normal: 150,000-450,000 /µL)
- Reticulocyte count: 10% (Normal: 0.5-1.5%)

IMAGING:
Initial plain radiographs of the hip are unremarkable.

## Encounter 3: source 2022-01-15, practice 2023-12-21, inpatient, Emergency Medicine

Chief complaint: Sudden onset of sharp left upper quadrant abdominal pain radiating to left shoulder after transatlantic flight.

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Sudden onset of sharp left upper quadrant abdominal pain radiating to left shoulder after transatlantic flight.

HISTORY OF PRESENT ILLNESS:
The patient is a 21-year-old male with sickle cell anemia (HbSS genotype) and history of eosinophilic esophagitis and bilateral femoral head avascular necrosis who presents to the emergency department with a sudden onset of sharp, left upper quadrant abdominal pain that started 12 hours ago during a transatlantic flight. He describes the pain as constant and radiating to his left shoulder. He also reports feeling fatigued and having dark urine since the onset of the pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Sickle cell anemia without crisis (diagnosed 2020-01-15)
- Eosinophilic esophagitis (diagnosed 2020-04-15)
- Avascular necrosis of the femoral head (diagnosed 2020-07-15)
- Sickle cell anemia (HbSS genotype)
- Status post splenectomy (secondary to atraumatic splenic rupture)
- Asplenia
- Mild intermittent asthma
- Seasonal allergic rhinitis
- Avascular necrosis of bilateral femoral heads
- Chronic anemia (baseline hemoglobin 7-8 g/dL)
- History of acute chest syndrome (multiple episodes)

His past medical history is significant for mild asthma.

PAST SURGICAL HISTORY:
He denies any recent trauma or surgery.

MEDICATIONS:
Home Medications:
- Hydroxyurea 1000 mg orally daily
- Folic acid 1 mg orally daily
- Albuterol HFA inhaler 90 mcg, 2 inhalations every 4-6 hours as needed for wheezing or shortness of breath

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with sickle cell trait (HbAS)
- Father with sickle cell trait (HbAS)
- Maternal grandmother with asthma and environmental allergies
- Older brother with sickle cell disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Denies alcohol use. Occupation: University student (sophomore year).

VITAL SIGNS:
On examination, his temperature is 37.8°C (100°F), blood pressure is 130/85 mmHg, pulse is 95/min, and respiratory rate is 18/min.

PHYSICAL EXAMINATION:
Abdominal examination reveals tenderness to palpation in the left upper quadrant with no rebound or guarding. There is no splenomegaly.

LABORATORY DATA:
Laboratory studies show:

- Hemoglobin: 11.2 g/dL (normal: 13.8-17.2 g/dL)
- White blood cell count: 9,000/mm³ (normal: 4,500-11,000/mm³)
- Platelet count: 250,000/mm³ (normal: 150,000-450,000/mm³)
- Lactate dehydrogenase (LDH): 450 U/L (normal: 140-280 U/L)
- Haptoglobin: <10 mg/dL (normal: 30-200 mg/dL)
- Peripheral smear: Presence of "bite cells" and "schistocytes"
- Direct antiglobulin test (Coombs test): Negative

## Encounter 4: source 2022-03-15, practice 2024-02-13, outpatient, Hematology

Chief complaint: Fatigue and shortness of breath for two weeks following recent splenectomy.

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. Emily Watson, MD
Department: Hematology
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue and shortness of breath for two weeks following recent splenectomy.

HISTORY OF PRESENT ILLNESS:
The patient is a 21-year-old male with sickle cell anemia (HbSS) and asplenia status post splenectomy two months prior for atraumatic rupture, returning for outpatient hematology follow-up with two weeks of fatigue and shortness of breath. He denies fever, chills, or recent infections.

PAST MEDICAL HISTORY:
Active Problem List:
- Sickle cell anemia without crisis (diagnosed 2020-01-15)
- Eosinophilic esophagitis (diagnosed 2020-04-15)
- Avascular necrosis of the femoral head (diagnosed 2020-07-15)
- Atraumatic splenic rupture with hemoperitoneum (diagnosed 2022-01-15)
- Sickle cell anemia (HbSS genotype)
- Status post splenectomy (secondary to atraumatic splenic rupture)
- Asplenia
- Mild intermittent asthma
- Seasonal allergic rhinitis
- Avascular necrosis of bilateral femoral heads
- Chronic anemia (baseline hemoglobin 7-8 g/dL)
- History of acute chest syndrome (multiple episodes)

He has a history of sickle cell disease and reports several episodes of acute chest syndrome in the past. His vaccinations are up to date.

PAST SURGICAL HISTORY:
- Laparoscopic splenectomy (complication of splenic sequestration/infarction)
- Tonsillectomy and adenoidectomy (age 6)

MEDICATIONS:
Home Medications:
- Hydroxyurea 1000 mg orally daily
- Folic acid 1 mg orally daily
- Albuterol HFA inhaler 90 mcg, 2 inhalations every 4-6 hours as needed for wheezing or shortness of breath

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with sickle cell trait (HbAS)
- Father with sickle cell trait (HbAS)
- Maternal grandmother with asthma and environmental allergies
- Older brother with sickle cell disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Denies alcohol use. Occupation: University student (sophomore year).

VITAL SIGNS:
On physical examination, his temperature is 37.2°C, blood pressure is 128/76 mmHg, heart rate is 88 beats per minute, and respiratory rate is 18 breaths per minute.

PHYSICAL EXAMINATION:
His spleen is non-palpable.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 8.2 g/dL (Normal: 13.8-17.2 g/dL)
- Hematocrit: 24.5% (Normal: 40.7-50.3%)
- White blood cells: 5,000/uL (Normal: 4,500-11,000/uL)
- Platelets: 250,000/uL (Normal: 150,000-450,000/uL)
- Reticulocyte count: 3% (Normal: 0.5-1.5%)
- Peripheral smear: Presence of Howell-Jolly bodies

## Encounter 5: source 2022-07-15, practice 2024-06-12, ed, Emergency Medicine

Chief complaint: Painful erection lasting 6 hours without sexual activity or trauma.

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Robert Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Painful erection lasting 6 hours without sexual activity or trauma.

HISTORY OF PRESENT ILLNESS:
The patient is a 21-year-old male with sickle cell anemia (HbSS genotype) and status post splenectomy six months ago secondary to atraumatic rupture who presents with his current symptoms. He denies any sexual activity, trauma, or illicit drug use prior to the onset of his symptoms. He reports that he has not had any recent sickle cell crises.

PAST MEDICAL HISTORY:
Active Problem List:
- Sickle cell anemia without crisis (diagnosed 2020-01-15)
- Eosinophilic esophagitis (diagnosed 2020-04-15)
- Avascular necrosis of the femoral head (diagnosed 2020-07-15)
- Atraumatic splenic rupture with hemoperitoneum (diagnosed 2022-01-15)
- Overwhelming post-splenectomy infection (OPSI) risk (diagnosed 2022-03-15)
- Sickle cell anemia (HbSS genotype)
- Status post splenectomy (secondary to atraumatic splenic rupture)
- Asplenia
- Mild intermittent asthma
- Seasonal allergic rhinitis
- Avascular necrosis of bilateral femoral heads
- Chronic anemia (baseline hemoglobin 7-8 g/dL)
- History of acute chest syndrome (multiple episodes)

His past medical history is significant for sickle cell anemia.

PAST SURGICAL HISTORY:
- Laparoscopic splenectomy (complication of splenic sequestration/infarction)
- Tonsillectomy and adenoidectomy (age 6)

MEDICATIONS:
He takes hydroxyurea and folic acid.

Home Medications:
- Albuterol HFA inhaler 90 mcg, 2 inhalations every 4-6 hours as needed for wheezing or shortness of breath

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with sickle cell trait (HbAS)
- Father with sickle cell trait (HbAS)
- Maternal grandmother with asthma and environmental allergies
- Older brother with sickle cell disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Denies alcohol use. Occupation: University student (sophomore year).

VITAL SIGNS:
His vital signs are within normal limits.

PHYSICAL EXAMINATION:
On physical examination, the penis is fully erect, tender to palpation, and there is no evidence of trauma. The testes and abdomen are unremarkable on examination.

LABORATORY DATA:
Laboratory studies are as follows:

- Hemoglobin: 8.5 g/dL (Normal: 13.8-17.2 g/dL)
- Hematocrit: 25.5% (Normal: 40.7-50.3%)
- White blood cell count: 12,000 /µL (Normal: 4,500-11,000 /µL)
- Platelet count: 450,000 /µL (Normal: 150,000-450,000 /µL)
- Reticulocyte count: 10% (Normal: 0.5-1.5%)
- Peripheral smear: Sickled erythrocytes present

## Encounter 6: source 2023-01-15, practice 2024-12-19, ed, Emergency Medicine

Chief complaint: Painful erection persisting for 6 hours, similar to prior episode that resolved spontaneously.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Lisa Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Painful erection persisting for 6 hours, similar to prior episode that resolved spontaneously.

HISTORY OF PRESENT ILLNESS:
The patient is a 22-year-old male with sickle cell anemia (HbSS genotype) and history of similar presentation in July 2022 who presents to the emergency department with a painful erection that has persisted for 6 hours. He denies any sexual activity or trauma to the genital area. He reports that he has had similar, but less severe episodes in the past that resolved spontaneously.

PAST MEDICAL HISTORY:
Active Problem List:
- Sickle cell anemia without crisis (diagnosed 2020-01-15)
- Eosinophilic esophagitis (diagnosed 2020-04-15)
- Avascular necrosis of the femoral head (diagnosed 2020-07-15)
- Atraumatic splenic rupture with hemoperitoneum (diagnosed 2022-01-15)
- Overwhelming post-splenectomy infection (OPSI) risk (diagnosed 2022-03-15)
- Ischemic priapism secondary to sickle cell disease (diagnosed 2022-07-15)
- Sickle cell anemia (HbSS genotype)
- Status post splenectomy (secondary to atraumatic splenic rupture)
- Asplenia
- Mild intermittent asthma
- Seasonal allergic rhinitis
- Avascular necrosis of bilateral femoral heads
- Chronic anemia (baseline hemoglobin 7-8 g/dL)
- History of acute chest syndrome (multiple episodes)

His past medical history is significant for sickle cell disease.

PAST SURGICAL HISTORY:
- Laparoscopic splenectomy (complication of splenic sequestration/infarction)
- Tonsillectomy and adenoidectomy (age 6)

MEDICATIONS:
He takes for which he is on hydroxyurea and folic acid.

Home Medications:
- Albuterol HFA inhaler 90 mcg, 2 inhalations every 4-6 hours as needed for wheezing or shortness of breath

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with sickle cell trait (HbAS)
- Father with sickle cell trait (HbAS)
- Maternal grandmother with asthma and environmental allergies
- Older brother with sickle cell disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Denies alcohol use. Occupation: University student (sophomore year).

VITAL SIGNS:
His vital signs are stable

PHYSICAL EXAMINATION:
He appears to be in moderate distress due to pain. On examination, his penis is fully erect, tender to palpation, and there is no evidence of trauma. His abdomen is soft and non-tender, and there are no inguinal lymphadenopathies.

LABORATORY DATA:
Laboratory tests reveal:

- Hemoglobin: 8.5 g/dL (Normal: 13.8-17.2 g/dL)
- Hematocrit: 25.5% (Normal: 40.7-50.3%)
- Reticulocyte count: 10% (Normal: 0.5-1.5%)
- White blood cell count: 12,000 /µL (Normal: 4,500-11,000 /µL)
- Platelet count: 450,000 /µL (Normal: 150,000-450,000 /µL)

## Encounter 7: source 2024-01-15, practice 2025-12-21, ed, Emergency Medicine

Chief complaint: Acute right hip pain, high fever, and inability to bear weight.

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Michael Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute right hip pain, high fever, and inability to bear weight.

HISTORY OF PRESENT ILLNESS:
This 23-year-old male with HbSS sickle cell anemia, status post splenectomy for atraumatic rupture, and known bilateral femoral head avascular necrosis presents with acute right hip pain, fever, and an inability to bear weight. He denies trauma, recent illness, infections, or skin wounds.

PAST MEDICAL HISTORY:
Active Problem List:
- Sickle cell anemia without crisis (diagnosed 2020-01-15)
- Eosinophilic esophagitis (diagnosed 2020-04-15)
- Avascular necrosis of the femoral head (diagnosed 2020-07-15)
- Atraumatic splenic rupture with hemoperitoneum (diagnosed 2022-01-15)
- Overwhelming post-splenectomy infection (OPSI) risk (diagnosed 2022-03-15)
- Ischemic priapism secondary to sickle cell disease (diagnosed 2022-07-15)
- Sickle cell anemia (HbSS genotype)
- Status post splenectomy (secondary to atraumatic splenic rupture)
- Asplenia
- Mild intermittent asthma
- Seasonal allergic rhinitis
- Avascular necrosis of bilateral femoral heads
- Chronic anemia (baseline hemoglobin 7-8 g/dL)
- History of acute chest syndrome (multiple episodes)

PAST SURGICAL HISTORY:
- Laparoscopic splenectomy (complication of splenic sequestration/infarction)
- Tonsillectomy and adenoidectomy (age 6)

MEDICATIONS:
Home Medications:
- Hydroxyurea 1000 mg orally daily
- Folic acid 1 mg orally daily
- Albuterol HFA inhaler 90 mcg, 2 inhalations every 4-6 hours as needed for wheezing or shortness of breath

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with sickle cell trait (HbAS)
- Father with sickle cell trait (HbAS)
- Maternal grandmother with asthma and environmental allergies
- Older brother with sickle cell disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Denies alcohol use. Occupation: University student (sophomore year).

VITAL SIGNS:
His temperature is 38.9°C (102°F)

PHYSICAL EXAMINATION:
On examination, the joint is swollen, erythematous, and tender with severely restricted motion.

LABORATORY DATA:
Laboratory testing reveals a WBC count of 15,200/μL, ESR of 80 mm/hr, and CRP of 115 mg/L. Blood cultures are pending.

OTHER STUDIES:
Joint aspiration yields turbid synovial fluid with 95,000 WBCs/μL (90% neutrophils) and no crystals or organisms on Gram stain.

PLAN:
The patient is admitted and started on empiric IV antibiotics.
