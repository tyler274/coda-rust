# synthetic_hospital patient 2070

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex M, White, Uninsured.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-07-12 (Mon) | outpatient | Dermatology |
| 1 | 2020-07-15 | 2022-01-06 (Thu) | outpatient | Hematology & Oncology |
| 2 | 2021-01-15 | 2022-07-05 (Tue) | outpatient | Endocrinology |
| 3 | 2021-07-15 | 2023-01-09 (Mon) | outpatient | Dermatology |
| 4 | 2022-01-15 | 2023-07-10 (Mon) | outpatient | Internal Medicine |
| 5 | 2023-01-15 | 2024-07-04 (Thu) | outpatient | Hematology & Oncology |
| 6 | 2025-01-15 | 2026-07-21 (Tue) | icu | Critical Care Medicine |
| 7 | 2025-01-15 | 2026-07-21 (Tue) | inpatient | Psychiatry |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Uninsured",
 "occupation": "Construction laborer and part-time groundskeeper",
 "alcohol_use": "Heavy daily use for 25 years (approximately 1 bottle of vodka or equivalent daily), with increased consumption following recent divorce; last drink 3 days prior to hospitalization",
 "family_history": [
  "Father with alcohol use disorder and liver cirrhosis",
  "Mother with type 2 diabetes",
  "No family history of hematologic malignancies or porphyria"
 ],
 "race_ethnicity": "White",
 "smoking_status": "Never",
 "home_medications": [
  {
   "dose": "250 mg PO BID (prophylaxis for asplenia - patient non-adherent)",
   "name": "Penicillin VK"
  },
  {
   "dose": "1 mg PO daily",
   "name": "Folic acid"
  },
  {
   "dose": "100 mg PO daily",
   "name": "Thiamine"
  },
  {
   "dose": "2000 units PO daily",
   "name": "Cholecalciferol (Vitamin D3)"
  },
  {
   "dose": "200 mg PO twice weekly (for PCT - prescribed but not started)",
   "name": "Hydroxychloroquine"
  }
 ],
 "surgical_history": [
  "Splenectomy (10 years ago following motor vehicle accident)"
 ],
 "chronic_conditions": [
  "Alcohol use disorder, severe",
  "Chronic hepatitis C infection (untreated)",
  "Status post splenectomy with asplenia",
  "Alcoholic liver disease with cirrhosis",
  "Porphyria cutanea tarda",
  "Severe malnutrition with multiple micronutrient deficiencies"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Folate deficiency anemia", "Alcohol withdrawal delirium", "Overwhelming post-splenectomy infection (OPSI) risk", "Refractory hypoxemia in ARDS", "Porphyria cutanea tarda", "Alcoholic liver disease with vitamin D deficiency and secondary hyperparathyroidism"]
Comorbidities: ["Chronic hepatitis C", "Alcohol use disorder"]

## Encounter 0: source 2020-01-15, practice 2021-07-12, outpatient, Dermatology

Chief complaint: Patient presents to the clinic with painful blistering skin lesions on the backs of his hands and forearms that have developed over the past few months following sun exposure.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Patient presents to the clinic with painful blistering skin lesions on the backs of his hands and forearms that have developed over the past few months following sun exposure.

HISTORY OF PRESENT ILLNESS:
He reports that the lesions often appear after minor trauma or sun exposure and are particularly prevalent on the backs of his hands and forearms. He describes them as painful blisters that heal slowly, leaving behind fragile skin and areas of hyperpigmentation.

PAST MEDICAL HISTORY:
Active Problem List:
- Alcohol use disorder, severe
- Chronic hepatitis C infection (untreated)
- Status post splenectomy with asplenia
- Alcoholic liver disease with cirrhosis
- Porphyria cutanea tarda
- Severe malnutrition with multiple micronutrient deficiencies

The patient has a history of chronic hepatitis C, for which he has not been receiving treatment.

PAST SURGICAL HISTORY:
- Splenectomy (10 years ago following motor vehicle accident)

MEDICATIONS:
Home Medications:
- Penicillin VK 250 mg PO BID (prophylaxis for asplenia - patient non-adherent)
- Folic acid 1 mg PO daily
- Thiamine 100 mg PO daily
- Cholecalciferol (Vitamin D3) 2000 units PO daily
- Hydroxychloroquine 200 mg PO twice weekly (for PCT - prescribed but not started)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with alcohol use disorder and liver cirrhosis
- Mother with type 2 diabetes
- No family history of hematologic malignancies or porphyria

SOCIAL HISTORY:
He also admits to heavy alcohol use.

PHYSICAL EXAMINATION:
On physical examination, you note several vesicles and bullae on sun-exposed areas, as well as areas of scarring and hyperpigmentation. His liver is palpable 2 cm below the right costal margin.

LABORATORY DATA:
Laboratory tests reveal the following:

- Serum aminotransferases (AST/ALT): Elevated
- Serum ferritin: Elevated
- Hepatitis C virus RNA: Detected
- Urine porphyrins: Elevated

## Encounter 1: source 2020-07-15, practice 2022-01-06, outpatient, Hematology & Oncology

Chief complaint: Patient reports a 2-month history of worsening fatigue, shortness of breath on exertion, and a sore tongue.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Patient reports a 2-month history of worsening fatigue, shortness of breath on exertion, and a sore tongue.

PAST MEDICAL HISTORY:
Active Problem List:
- Porphyria cutanea tarda (diagnosed 2020-01-15)
- Alcohol use disorder, severe
- Chronic hepatitis C infection (untreated)
- Status post splenectomy with asplenia
- Alcoholic liver disease with cirrhosis
- Severe malnutrition with multiple micronutrient deficiencies

PAST SURGICAL HISTORY:
- Splenectomy (10 years ago following motor vehicle accident)

MEDICATIONS:
He does not take any medications

Home Medications:
- Penicillin VK 250 mg PO BID (prophylaxis for asplenia - patient non-adherent)
- Folic acid 1 mg PO daily
- Thiamine 100 mg PO daily
- Cholecalciferol (Vitamin D3) 2000 units PO daily
- Hydroxychloroquine 200 mg PO twice weekly (for PCT - prescribed but not started)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with alcohol use disorder and liver cirrhosis
- Mother with type 2 diabetes
- No family history of hematologic malignancies or porphyria

SOCIAL HISTORY:
He reports a significant increase in alcohol consumption over the past year following a divorce. He denies use of tobacco or illicit drugs.

PHYSICAL EXAMINATION:
On physical examination, he appears pale with mild jaundice. His tongue is erythematous and smooth. Cardiovascular examination reveals a systolic flow murmur.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 9.2 g/dL (Normal: 13.8-17.2 g/dL)
- Mean corpuscular volume (MCV): 110 fL (Normal: 80-96 fL)
- White blood cell count: 3.8 x 10^9/L (Normal: 4.5-11 x 10^9/L)
- Platelet count: 150 x 10^9/L (Normal: 150-450 x 10^9/L)
- Serum folate: 2 ng/mL (Normal: 3-17 ng/mL)
- Serum vitamin B12: 400 pg/mL (Normal: 200-900 pg/mL)
- Liver function tests: Mildly elevated AST and ALT

## Encounter 2: source 2021-01-15, practice 2022-07-05, outpatient, Endocrinology

Chief complaint: Patient complains of persistent fatigue, difficulty concentrating, and frequent muscle cramps in his legs despite consuming calcium-rich foods.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Priya Patel, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Patient complains of persistent fatigue, difficulty concentrating, and frequent muscle cramps in his legs despite consuming calcium-rich foods.

HISTORY OF PRESENT ILLNESS:
The patient with alcoholic cirrhosis, prior porphyria cutanea tarda, and folate deficiency anemia returns with complaints of persistent fatigue, difficulty concentrating, and frequent leg muscle cramps. He consumes large quantities of canned salmon, sardines, and fortified dairy products.

PAST MEDICAL HISTORY:
Active Problem List:
- Porphyria cutanea tarda (diagnosed 2020-01-15)
- Folate deficiency anemia (diagnosed 2020-07-15)
- Alcohol use disorder, severe
- Chronic hepatitis C infection (untreated)
- Status post splenectomy with asplenia
- Alcoholic liver disease with cirrhosis
- Severe malnutrition with multiple micronutrient deficiencies

His past medical history is significant for chronic alcoholism. He has not seen a physician in over a decade.

PAST SURGICAL HISTORY:
- Splenectomy (10 years ago following motor vehicle accident)

MEDICATIONS:
Home Medications:
- Penicillin VK 250 mg PO BID (prophylaxis for asplenia - patient non-adherent)
- Folic acid 1 mg PO daily
- Thiamine 100 mg PO daily
- Cholecalciferol (Vitamin D3) 2000 units PO daily
- Hydroxychloroquine 200 mg PO twice weekly (for PCT - prescribed but not started)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with alcohol use disorder and liver cirrhosis
- Mother with type 2 diabetes
- No family history of hematologic malignancies or porphyria

SOCIAL HISTORY:
He works as a construction laborer in southern Texas and reports being outdoors nearly every day without sunblock. He reports drinking a bottle of vodka daily for the past 8 years.

REVIEW OF SYSTEMS:
He denies any recent illness, infection, or trauma.

PHYSICAL EXAMINATION:
On physical exam, his muscle strength is slightly reduced in the proximal lower limbs. Neurologic exam is otherwise normal. The remainder of the exam is unremarkable.

## Encounter 3: source 2021-07-15, practice 2023-01-09, outpatient, Dermatology

Chief complaint: Patient returns with worsening blistering skin lesions on the backs of his hands and reports dark-colored urine for the past several weeks.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Sarah Chen, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Patient returns with worsening blistering skin lesions on the backs of his hands and reports dark-colored urine for the past several weeks.

HISTORY OF PRESENT ILLNESS:
Patient returns for follow-up of porphyria cutanea tarda diagnosed January 2020, noting hand lesions have worsened over the past several months. He works as a groundskeeper and spends long hours outdoors. He reports dark urine but denies abdominal pain or neurologic symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Porphyria cutanea tarda (diagnosed 2020-01-15)
- Folate deficiency anemia (diagnosed 2020-07-15)
- Alcoholic liver disease with vitamin D deficiency and secondary hyperparathyroidism (diagnosed 2021-01-15)
- Alcohol use disorder, severe
- Chronic hepatitis C infection (untreated)
- Status post splenectomy with asplenia
- Alcoholic liver disease with cirrhosis
- Severe malnutrition with multiple micronutrient deficiencies

Medical history is significant for chronic alcohol use and untreated hepatitis C.

PAST SURGICAL HISTORY:
- Splenectomy (10 years ago following motor vehicle accident)

MEDICATIONS:
Home Medications:
- Penicillin VK 250 mg PO BID (prophylaxis for asplenia - patient non-adherent)
- Folic acid 1 mg PO daily
- Thiamine 100 mg PO daily
- Cholecalciferol (Vitamin D3) 2000 units PO daily
- Hydroxychloroquine 200 mg PO twice weekly (for PCT - prescribed but not started)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with alcohol use disorder and liver cirrhosis
- Mother with type 2 diabetes
- No family history of hematologic malignancies or porphyria

SOCIAL HISTORY:
Smoking: Never. Alcohol: Heavy daily use for 25 years (approximately 1 bottle of vodka or equivalent daily), with increased consumption following recent divorce; last drink 3 days prior to hospitalization. Occupation: Construction laborer and part-time groundskeeper.

PHYSICAL EXAMINATION:
Physical examination shows hyperpigmentation and fragile skin with erosions on sun-exposed areas.

LABORATORY DATA:
Labs show elevated liver enzymes.

IMAGING:
The image below shows his lesions.

## Encounter 4: source 2022-01-15, practice 2023-07-10, outpatient, Internal Medicine

Chief complaint: Patient presents with progressive fatigue, intermittent lightheadedness, and palpitations with mild exertion over the past two months.

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. James O'Brien, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Patient presents with progressive fatigue, intermittent lightheadedness, and palpitations with mild exertion over the past two months.

HISTORY OF PRESENT ILLNESS:
The patient with severe malnutrition and prior similar complaints in 2020 and 2021 returns with progressive fatigue and decreased exercise tolerance over the past two months. He also reports intermittent lightheadedness and palpitations with mild exertion. He denies any blood loss, melena, or recent illness.

PAST MEDICAL HISTORY:
Active Problem List:
- Porphyria cutanea tarda (diagnosed 2020-01-15)
- Folate deficiency anemia (diagnosed 2020-07-15)
- Alcoholic liver disease with vitamin D deficiency and secondary hyperparathyroidism (diagnosed 2021-01-15)
- Alcohol use disorder, severe
- Chronic hepatitis C infection (untreated)
- Status post splenectomy with asplenia
- Alcoholic liver disease with cirrhosis
- Severe malnutrition with multiple micronutrient deficiencies

PAST SURGICAL HISTORY:
- Splenectomy (10 years ago following motor vehicle accident)

MEDICATIONS:
Home Medications:
- Penicillin VK 250 mg PO BID (prophylaxis for asplenia - patient non-adherent)
- Folic acid 1 mg PO daily
- Thiamine 100 mg PO daily
- Cholecalciferol (Vitamin D3) 2000 units PO daily
- Hydroxychloroquine 200 mg PO twice weekly (for PCT - prescribed but not started)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with alcohol use disorder and liver cirrhosis
- Mother with type 2 diabetes
- No family history of hematologic malignancies or porphyria

SOCIAL HISTORY:
He does not follow a specific diet but admits to drinking several cans of beer daily for the past 15 years.

PHYSICAL EXAMINATION:
On physical exam, the patient appears pale. There is no jaundice, hepatosplenomegaly, or lymphadenopathy. Neurologic exam shows no deficits.

LABORATORY DATA:
Hemoglobin 9.1 g/dL Mean corpuscular volume 108 fL Reticulocyte count 0.4%

OTHER STUDIES:
A peripheral blood smear is obtained and shown.

## Encounter 5: source 2023-01-15, practice 2024-07-04, outpatient, Hematology & Oncology

Chief complaint: Patient reports recurrent infections, easy bruising, and significant unintentional weight loss of 10 kg over the past six months.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Aisha Williams, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Patient reports recurrent infections, easy bruising, and significant unintentional weight loss of 10 kg over the past six months.

HISTORY OF PRESENT ILLNESS:
The patient with asplenia status post splenectomy and a history of recurrent folate deficiency anemia managed in this clinic returns for evaluation of recurrent infections and easy bruising. He reports a significant unintentional weight loss of 10 kg during this period.

PAST MEDICAL HISTORY:
Active Problem List:
- Porphyria cutanea tarda (diagnosed 2020-01-15)
- Folate deficiency anemia (diagnosed 2020-07-15)
- Alcoholic liver disease with vitamin D deficiency and secondary hyperparathyroidism (diagnosed 2021-01-15)
- Alcohol use disorder, severe
- Chronic hepatitis C infection (untreated)
- Status post splenectomy with asplenia
- Alcoholic liver disease with cirrhosis
- Severe malnutrition with multiple micronutrient deficiencies

His past medical history is significant for a splenectomy following a motor vehicle accident 10 years ago.

PAST SURGICAL HISTORY:
- Splenectomy (10 years ago following motor vehicle accident)

MEDICATIONS:
Home Medications:
- Penicillin VK 250 mg PO BID (prophylaxis for asplenia - patient non-adherent)
- Folic acid 1 mg PO daily
- Thiamine 100 mg PO daily
- Cholecalciferol (Vitamin D3) 2000 units PO daily
- Hydroxychloroquine 200 mg PO twice weekly (for PCT - prescribed but not started)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with alcohol use disorder and liver cirrhosis
- Mother with type 2 diabetes
- No family history of hematologic malignancies or porphyria

SOCIAL HISTORY:
With a history of chronic alcoholism

PHYSICAL EXAMINATION:
On physical examination, he is afebrile, pale, and has multiple ecchymoses on his arms and legs. His abdominal examination is unremarkable with no hepatosplenomegaly.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 9.8 g/dL (Normal: 13.8-17.2 g/dL)
- Mean corpuscular volume (MCV): 92 fL (Normal: 80-100 fL)
- White blood cell count: 3.2 x 10^9/L (Normal: 4.5-11.0 x 10^9/L)
- Platelet count: 120 x 10^9/L (Normal: 150-450 x 10^9/L)
- Peripheral blood smear: Presence of Howell-Jolly bodies

## Encounter 6: source 2025-01-15, practice 2026-07-21, icu, Critical Care Medicine

Chief complaint: Patient is transferred from the emergency department with severe shortness of breath and refractory hypoxemia following an episode of severe upper abdominal pain radiating to the back and repeated vomiting.

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Robert Kim, MD
Department: Critical Care Medicine
Visit Type: icu

CHIEF COMPLAINT:
Patient is transferred from the emergency department with severe shortness of breath and refractory hypoxemia following an episode of severe upper abdominal pain radiating to the back and repeated vomiting.

HISTORY OF PRESENT ILLNESS:
The patient is a 50-year-old male with severe alcohol use disorder, cirrhosis, asplenia, and prior encounters for porphyria cutanea tarda, nutritional deficiencies, and infection risk who presents with respiratory decompensation and was recently complaining of severe upper abdominal pain radiating to the back. He had been vomiting repeatedly over the last two days and refused to eat or drink much. The patient is placed on supplemental oxygen, but there is minimal improvement in his hypoxia. Despite aggressive fluid resuscitation and supportive care, the patient's oxygenation does not improve, and mechanical ventilation is initiated.

PAST MEDICAL HISTORY:
Active Problem List:
- Porphyria cutanea tarda (diagnosed 2020-01-15)
- Folate deficiency anemia (diagnosed 2020-07-15)
- Alcoholic liver disease with vitamin D deficiency and secondary hyperparathyroidism (diagnosed 2021-01-15)
- Overwhelming post-splenectomy infection (OPSI) risk (diagnosed 2023-01-15)
- Alcohol use disorder, severe
- Chronic hepatitis C infection (untreated)
- Status post splenectomy with asplenia
- Alcoholic liver disease with cirrhosis
- Severe malnutrition with multiple micronutrient deficiencies

PAST SURGICAL HISTORY:
- Splenectomy (10 years ago following motor vehicle accident)

MEDICATIONS:
Home Medications:
- Penicillin VK 250 mg PO BID (prophylaxis for asplenia - patient non-adherent)
- Folic acid 1 mg PO daily
- Thiamine 100 mg PO daily
- Cholecalciferol (Vitamin D3) 2000 units PO daily
- Hydroxychloroquine 200 mg PO twice weekly (for PCT - prescribed but not started)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with alcohol use disorder and liver cirrhosis
- Mother with type 2 diabetes
- No family history of hematologic malignancies or porphyria

SOCIAL HISTORY:
The patient has a history of chronic alcohol use

VITAL SIGNS:
His temperature is 38.3°C (100.9°F), pulse is 112/min, respiratory rate is 32/min, and blood pressure is 100/62 mm Hg. Oxygen saturation is 83% on room air.

PHYSICAL EXAMINATION:
On arrival, he is tachypneic and appears visibly distressed, using accessory muscles for breathing.
Chest auscultation reveals diffuse crackles bilaterally.
Abdominal examination reveals epigastric tenderness with guarding.

LABORATORY DATA:
Laboratory testing reveals an elevated serum glucose level and decreased serum calcium. Arterial blood gas reveals a PaO₂ of 55 mm Hg despite a FiO₂ of 0.60.

IMAGING:
A chest radiograph shows bilateral patchy opacities in the lung fields.
and bedside cardiac ultrasound reveals normal left ventricular function and no pericardial effusion.

OTHER STUDIES:
ECG is unremarkable,

ASSESSMENT AND PLAN:
The team rules out cardiac causes for the pulmonary findings and concludes that the patient’s current respiratory condition is secondary to an underlying systemic process.

## Encounter 7: source 2025-01-15, practice 2026-07-21, inpatient, Psychiatry

Chief complaint: Patient develops severe confusion, agitation, tremors, and visual hallucinations three days into hospitalization for acute pancreatitis.

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Maria Rodriguez, MD
Department: Psychiatry
Visit Type: inpatient

CHIEF COMPLAINT:
Patient develops severe confusion, agitation, tremors, and visual hallucinations three days into hospitalization for acute pancreatitis.

HISTORY OF PRESENT ILLNESS:
The patient is a 50-year-old male with severe alcohol use disorder, cirrhosis, chronic hepatitis C, asplenia, and prior complications including porphyria cutanea tarda and nutritional deficiencies, currently hospitalized for acute pancreatitis, who is brought to the emergency department by his wife due to severe confusion, agitation, and tremors. He was hospitalized 3 days ago for treatment of acute pancreatitis. He has become increasingly restless and irritable over the past 24 hours and is now hallucinating, claiming there are "snakes on the floor."

PAST MEDICAL HISTORY:
Active Problem List:
- Porphyria cutanea tarda (diagnosed 2020-01-15)
- Folate deficiency anemia (diagnosed 2020-07-15)
- Alcoholic liver disease with vitamin D deficiency and secondary hyperparathyroidism (diagnosed 2021-01-15)
- Overwhelming post-splenectomy infection (OPSI) risk (diagnosed 2023-01-15)
- Refractory hypoxemia in ARDS (diagnosed 2025-01-15)
- Alcohol use disorder, severe
- Chronic hepatitis C infection (untreated)
- Status post splenectomy with asplenia
- Alcoholic liver disease with cirrhosis
- Severe malnutrition with multiple micronutrient deficiencies

PAST SURGICAL HISTORY:
- Splenectomy (10 years ago following motor vehicle accident)

MEDICATIONS:
Home Medications:
- Penicillin VK 250 mg PO BID (prophylaxis for asplenia - patient non-adherent)
- Folic acid 1 mg PO daily
- Thiamine 100 mg PO daily
- Cholecalciferol (Vitamin D3) 2000 units PO daily
- Hydroxychloroquine 200 mg PO twice weekly (for PCT - prescribed but not started)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with alcohol use disorder and liver cirrhosis
- Mother with type 2 diabetes
- No family history of hematologic malignancies or porphyria

SOCIAL HISTORY:
His wife reports that he has a 25-year history of heavy alcohol use and had not consumed any alcohol since being admitted to the hospital.

VITAL SIGNS:
Temperature is 38.2°C (100.8°F), blood pressure is 162/98 mm Hg, pulse is 126/min, and respirations are 24/min.

PHYSICAL EXAMINATION:
On examination, he is disoriented to time and place, appears frightened, and attempts to pull out his IV lines.
Diaphoresis and fine tremor are noted. Cardiopulmonary and neurologic exams are otherwise unremarkable.
