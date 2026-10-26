# synthetic_hospital patient 2073

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 50 at first encounter, sex M, South Asian (Indian), Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-01-05 (Thu) | outpatient | Gastroenterology |
| 1 | 2020-07-15 | 2023-07-04 (Tue) | outpatient | Neurology |
| 2 | 2021-01-15 | 2024-01-15 (Mon) | outpatient | Neurology |
| 3 | 2021-09-15 | 2024-09-08 (Sun) | inpatient | Infectious Disease |
| 4 | 2022-05-15 | 2025-05-10 (Sat) | ed | Emergency Medicine |
| 5 | 2022-11-15 | 2025-11-02 (Sun) | ed | Emergency Medicine |
| 6 | 2023-01-15 | 2026-01-11 (Sun) | icu | Emergency Medicine |
| 7 | 2023-07-15 | 2026-07-15 (Wed) | inpatient | Internal Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Construction laborer",
 "alcohol_use": "Chronic severe alcohol use disorder, consuming approximately 6 beers daily (\u226572 g ethanol) for 20 years, with multiple previous withdrawal episodes and intermittent periods of sobriety lasting 1\u20133 months",
 "family_history": [
  "Father: Alcohol use disorder, cirrhosis",
  "Mother: Type 2 diabetes mellitus",
  "Brother: Hypertension"
 ],
 "race_ethnicity": "South Asian (Indian)",
 "smoking_status": "Former smoker, quit 1 year ago, 20 pack-year history",
 "home_medications": [
  {
   "dose": "36,000 units lipase with meals and 18,000 units with snacks",
   "name": "Pancrelipase (Creon)"
  },
  {
   "dose": "100 mg orally daily",
   "name": "Thiamine"
  },
  {
   "dose": "1 mg orally daily",
   "name": "Folic acid"
  },
  {
   "dose": "1 tablet daily",
   "name": "Multivitamin"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Alcohol Use Disorder, Severe",
  "Chronic Pancreatitis with exocrine insufficiency",
  "Alcoholic Cerebellar Degeneration",
  "Alcoholic Liver Disease",
  "Protein-calorie malnutrition",
  "Thiamine deficiency"
 ],
 "age_at_first_encounter": 50
}
```

Primary diagnoses: ["Uncomplicated alcohol withdrawal", "Delirium tremens", "Alcoholic cerebellar degeneration", "Chronic pancreatitis with exocrine insufficiency", "Tuberculous spondylitis (Pott disease)", "Alcoholic hepatitis", "Alcohol Withdrawal Seizures"]
Comorbidities: ["Anemia of chronic disease", "Alcohol use disorder", "Alcoholic liver disease"]

## Encounter 0: source 2020-01-15, practice 2023-01-05, outpatient, Gastroenterology

Chief complaint: Weight loss and chronic diarrhea for the past 6 months with bulky, greasy, foul-smelling stools

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Weight loss and chronic diarrhea for the past 6 months with bulky, greasy, foul-smelling stools

HISTORY OF PRESENT ILLNESS:
He describes his stools as bulky, greasy, and foul-smelling. He also reports frequent episodes of epigastric pain that radiates to his back.

PAST MEDICAL HISTORY:
Active Problem List:
- Alcohol Use Disorder, Severe
- Chronic Pancreatitis with exocrine insufficiency
- Alcoholic Cerebellar Degeneration
- Alcoholic Liver Disease
- Protein-calorie malnutrition
- Thiamine deficiency

MEDICATIONS:
Home Medications:
- Pancrelipase (Creon) 36,000 units lipase with meals and 18,000 units with snacks
- Thiamine 100 mg orally daily
- Folic acid 1 mg orally daily
- Multivitamin 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder, cirrhosis
- Mother: Type 2 diabetes mellitus
- Brother: Hypertension

SOCIAL HISTORY:
With a history of chronic alcohol use
He has a 20-year history of heavy alcohol consumption but has been sober for the last month.

PHYSICAL EXAMINATION:
On physical examination, the patient appears cachectic and mildly jaundiced. His abdomen is tender to palpation in the epigastric region without rebound or guarding. Bowel sounds are normoactive.

LABORATORY DATA:
Laboratory studies reveal:
- Serum amylase: 160 U/L (Normal: 30-110 U/L)
- Serum lipase: 300 U/L (Normal: 0-160 U/L)
- Fecal elastase: <100 mcg/g (Normal: >200 mcg/g)
- Total cholesterol: 145 mg/dL (Normal: 125-200 mg/dL)
- Triglycerides: 88 mg/dL (Normal: <150 mg/dL)

## Encounter 1: source 2020-07-15, practice 2023-07-04, outpatient, Neurology

Chief complaint: 6-month history of difficulty walking and progressive balance problems

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
6-month history of difficulty walking and progressive balance problems

HISTORY OF PRESENT ILLNESS:
The patient with severe alcohol use disorder and chronic pancreatitis returns with progressive gait difficulty. He reports that his balance has progressively worsened, and he now needs to hold onto furniture while moving around his home. He denies any weakness, numbness, or pain in his limbs.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic pancreatitis with exocrine insufficiency (diagnosed 2020-01-15)
- Alcohol Use Disorder, Severe
- Alcoholic Cerebellar Degeneration
- Alcoholic Liver Disease
- Protein-calorie malnutrition
- Thiamine deficiency

His past medical history is significant for chronic alcoholism, although he claims to have been sober for the past two months.

MEDICATIONS:
Home Medications:
- Pancrelipase (Creon) 36,000 units lipase with meals and 18,000 units with snacks
- Thiamine 100 mg orally daily
- Folic acid 1 mg orally daily
- Multivitamin 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder, cirrhosis
- Mother: Type 2 diabetes mellitus
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 1 year ago, 20 pack-year history. Alcohol: Chronic severe alcohol use disorder, consuming approximately 6 beers daily (≥72 g ethanol) for 20 years, with multiple previous withdrawal episodes and intermittent periods of sobriety lasting 1–3 months. Occupation: Construction laborer.

PHYSICAL EXAMINATION:
On physical examination, the patient has a wide-based, unsteady gait and difficulty with heel-to-toe walking. However, he is able to touch his nose with his finger accurately when his eyes are closed. His speech is normal, and there is no nystagmus. The remainder of the neurological examination, including strength, sensation, and reflexes, is unremarkable.

LABORATORY DATA:
Laboratory values:
- Complete blood count: Normal
- Liver function tests: Mildly elevated AST and ALT
- Vitamin B12 level: Normal
- Thyroid-stimulating hormone (TSH): Normal
- Serum thiamine: Low

## Encounter 2: source 2021-01-15, practice 2024-01-15, outpatient, Neurology

Chief complaint: Progressive difficulty walking with frequent falls over past 6 months

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Aisha Patel, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive difficulty walking with frequent falls over past 6 months

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of progressive gait difficulties six months after prior neurology evaluation, with known chronic pancreatitis and alcohol use disorder. He reports that his symptoms have progressively worsened over the past six months. He denies any recent head trauma or loss of consciousness. His family notes that he has been more forgetful and has had trouble performing tasks that require coordination.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic pancreatitis with exocrine insufficiency (diagnosed 2020-01-15)
- Alcoholic cerebellar degeneration (diagnosed 2020-07-15)
- Alcohol Use Disorder, Severe
- Alcoholic Liver Disease
- Protein-calorie malnutrition
- Thiamine deficiency

MEDICATIONS:
Home Medications:
- Pancrelipase (Creon) 36,000 units lipase with meals and 18,000 units with snacks
- Thiamine 100 mg orally daily
- Folic acid 1 mg orally daily
- Multivitamin 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder, cirrhosis
- Mother: Type 2 diabetes mellitus
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 1 year ago, 20 pack-year history. Alcohol: Chronic severe alcohol use disorder, consuming approximately 6 beers daily (≥72 g ethanol) for 20 years, with multiple previous withdrawal episodes and intermittent periods of sobriety lasting 1–3 months. Occupation: Construction laborer.

PHYSICAL EXAMINATION:
On physical examination, the patient has a wide-based, unsteady gait. He is unable to perform rapid alternating movements and demonstrates an intention tremor when reaching for objects. His muscle tone is decreased in all extremities. Deep tendon reflexes are normal. Sensation is intact.

LABORATORY DATA:
The patient's liver function tests are as follows:

- AST: 72 U/L (normal: 10-40 U/L)
- ALT: 55 U/L (normal: 7-56 U/L)
- Alkaline phosphatase: 130 U/L (normal: 40-130 U/L)
- Gamma-glutamyl transferase (GGT): 450 U/L (normal: 9-48 U/L)

## Encounter 3: source 2021-09-15, practice 2024-09-08, inpatient, Infectious Disease

Chief complaint: Chronic mid-back pain with night sweats, fatigue, and 6-kg weight loss

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. James O'Brien, MD
Department: Infectious Disease
Visit Type: inpatient

CHIEF COMPLAINT:
Chronic mid-back pain with night sweats, fatigue, and 6-kg weight loss

HISTORY OF PRESENT ILLNESS:
The patient with severe alcohol use disorder, chronic pancreatitis, and alcoholic cerebellar degeneration presents with mid-back pain. He describes the pain as deep and constant, worse at night, and unresponsive to over-the-counter analgesics. He has also developed night sweats, fatigue, and a 6-kg (13-lb) unintentional weight loss over the last two months.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic pancreatitis with exocrine insufficiency (diagnosed 2020-01-15)
- Alcoholic cerebellar degeneration (diagnosed 2020-07-15)
- Alcohol Use Disorder, Severe
- Alcoholic Liver Disease
- Protein-calorie malnutrition
- Thiamine deficiency

He has not sought medical care recently and has no known HIV diagnosis.

MEDICATIONS:
Home Medications:
- Pancrelipase (Creon) 36,000 units lipase with meals and 18,000 units with snacks
- Thiamine 100 mg orally daily
- Folic acid 1 mg orally daily
- Multivitamin 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder, cirrhosis
- Mother: Type 2 diabetes mellitus
- Brother: Hypertension

SOCIAL HISTORY:
He immigrated to the United States from India 5 years ago and works in construction.

VITAL SIGNS:
His temperature is 37.8°C (100.0°F), blood pressure is 124/70 mm Hg, pulse is 88/min, and respiratory rate is 16/min

PHYSICAL EXAMINATION:
There is moderate tenderness to palpation over the T9–T10 vertebrae. No visible spinal deformity is noted. Neurologic examination is normal.

LABORATORY DATA:
Labs show a white blood cell count of 7,800/µL, hemoglobin of 10.6 g/dL, ESR of 96 mm/hr, and CRP of 8.1 mg/dL

IMAGING:
Chest X-ray reveals apical scarring. An MRI of the spine shows anterior vertebral body destruction at T9 and T10 with intervertebral disc collapse and a prevertebral fluid collection.

## Encounter 4: source 2022-05-15, practice 2025-05-10, ed, Emergency Medicine

Chief complaint: Worsening abdominal pain, jaundice, and decreased appetite over 2 weeks

ENCOUNTER NOTE
Date: 2022-05-15
Provider: Dr. Priya Kumar, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening abdominal pain, jaundice, and decreased appetite over 2 weeks

HISTORY OF PRESENT ILLNESS:
This 52-year-old male has chronic pancreatitis with exocrine insufficiency and alcoholic cerebellar degeneration. He presents with a 2-week history of worsening abdominal pain, jaundice, and decreased appetite.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic pancreatitis with exocrine insufficiency (diagnosed 2020-01-15)
- Alcoholic cerebellar degeneration (diagnosed 2020-07-15)
- Tuberculous spondylitis (Pott disease) (diagnosed 2021-09-15)
- Alcohol Use Disorder, Severe
- Alcoholic Liver Disease
- Protein-calorie malnutrition
- Thiamine deficiency

MEDICATIONS:
Home Medications:
- Pancrelipase (Creon) 36,000 units lipase with meals and 18,000 units with snacks
- Thiamine 100 mg orally daily
- Folic acid 1 mg orally daily
- Multivitamin 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder, cirrhosis
- Mother: Type 2 diabetes mellitus
- Brother: Hypertension

SOCIAL HISTORY:
He reports consuming an average of six beers per day for the past 20 years.

VITAL SIGNS:
Febrile with a temperature of 38.3°C (100.9°F)

PHYSICAL EXAMINATION:
Sclerae are icteric. Abdominal examination reveals a tender, enlarged liver.

LABORATORY DATA:
White blood cell count: 12,000/mm³
Hemoglobin: 13.2 g/dL
Platelet count: 95,000/mm³
Aspartate aminotransferase (AST): 180 U/L
Alanine aminotransferase (ALT): 60 U/L
Total bilirubin: 6.2 mg/dL
Direct bilirubin: 4.1 mg/dL
Alkaline phosphatase: 120 U/L
International normalized ratio (INR): 1.6

## Encounter 5: source 2022-11-15, practice 2025-11-02, ed, Emergency Medicine

Chief complaint: Increasing confusion and agitation after stopping alcohol 2 days ago

ENCOUNTER NOTE
Date: 2022-11-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Increasing confusion and agitation after stopping alcohol 2 days ago

HISTORY OF PRESENT ILLNESS:
This patient with severe alcohol use disorder and prior alcoholic hepatitis presents for evaluation. He is brought to the emergency department by his family. He was last seen drinking heavily 2 days ago and has not consumed alcohol since.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic pancreatitis with exocrine insufficiency (diagnosed 2020-01-15)
- Alcoholic cerebellar degeneration (diagnosed 2020-07-15)
- Tuberculous spondylitis (Pott disease) (diagnosed 2021-09-15)
- Alcoholic hepatitis (diagnosed 2022-05-15)
- Alcohol Use Disorder, Severe
- Alcoholic Liver Disease
- Protein-calorie malnutrition
- Thiamine deficiency

MEDICATIONS:
Home Medications:
- Pancrelipase (Creon) 36,000 units lipase with meals and 18,000 units with snacks
- Thiamine 100 mg orally daily
- Folic acid 1 mg orally daily
- Multivitamin 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder, cirrhosis
- Mother: Type 2 diabetes mellitus
- Brother: Hypertension

SOCIAL HISTORY:
History of chronic alcohol use

VITAL SIGNS:
Blood pressure 160/100 mmHg, heart rate 120 beats per minute, respiratory rate 22 breaths per minute, and temperature 38.2°C

PHYSICAL EXAMINATION:
On examination, the patient is tremulous, diaphoretic, and appears anxious. He is oriented to person but not to place or time. A neurological examination reveals no focal deficits.

LABORATORY DATA:
Serum sodium: 140 mEq/L; Serum potassium: 4.0 mEq/L; Serum chloride: 100 mEq/L; Blood urea nitrogen: 18 mg/dL; Serum creatinine: 1.0 mg/dL; Aspartate aminotransferase (AST): 72 U/L; Alanine aminotransferase (ALT): 50 U/L; Gamma-glutamyl transferase (GGT): 300 U/L; Mean corpuscular volume (MCV): 96 fL

OTHER STUDIES:
An electrocardiogram (ECG) shows sinus tachycardia with no other abnormalities.

## Encounter 6: source 2023-01-15, practice 2026-01-11, icu, Emergency Medicine

Chief complaint: Severe anxiety, tremors, and profuse sweating 12 hours after quitting alcohol cold turkey

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Elena Vasquez, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Severe anxiety, tremors, and profuse sweating 12 hours after quitting alcohol cold turkey

HISTORY OF PRESENT ILLNESS:
This 53-year-old male with severe alcohol use disorder, chronic pancreatitis with exocrine insufficiency, alcoholic cerebellar degeneration, and alcoholic liver disease returns to the emergency department two months after a previous presentation with confusion and agitation following alcohol cessation. He now presents with a 12-hour history of increasing anxiety, hand tremors, and profuse sweating. He reports that he decided to quit drinking alcohol "cold turkey" two days ago after a decade of consuming approximately one pint of vodka daily. His last drink was 36 hours ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic pancreatitis with exocrine insufficiency (diagnosed 2020-01-15)
- Alcoholic cerebellar degeneration (diagnosed 2020-07-15)
- Tuberculous spondylitis (Pott disease) (diagnosed 2021-09-15)
- Alcoholic hepatitis (diagnosed 2022-05-15)
- Uncomplicated alcohol withdrawal (diagnosed 2022-11-15)
- Alcohol Use Disorder, Severe
- Alcoholic Liver Disease
- Protein-calorie malnutrition
- Thiamine deficiency

MEDICATIONS:
Home Medications:
- Pancrelipase (Creon) 36,000 units lipase with meals and 18,000 units with snacks
- Thiamine 100 mg orally daily
- Folic acid 1 mg orally daily
- Multivitamin 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder, cirrhosis
- Mother: Type 2 diabetes mellitus
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 1 year ago, 20 pack-year history. Alcohol: Chronic severe alcohol use disorder, consuming approximately 6 beers daily (≥72 g ethanol) for 20 years, with multiple previous withdrawal episodes and intermittent periods of sobriety lasting 1–3 months. Occupation: Construction laborer.

VITAL SIGNS:
Blood pressure 160/100 mmHg, heart rate 120 beats per minute, respiratory rate 22 breaths per minute, and temperature 37.8°C (100°F)

PHYSICAL EXAMINATION:
On examination, the patient is agitated, diaphoretic, and tremulous. He appears disoriented to time and place. No focal neurological deficits are noted.

LABORATORY DATA:
Serum sodium: 140 mEq/L
Serum potassium: 4.0 mEq/L
Serum chloride: 100 mEq/L
Blood urea nitrogen: 18 mg/dL
Serum creatinine: 1.1 mg/dL
Serum glucose: 90 mg/dL
Serum magnesium: 1.8 mg/dL
Serum calcium: 9.2 mg/dL
Serum aspartate aminotransferase (AST): 72 U/L
Serum alanine aminotransferase (ALT): 50 U/L
Serum alkaline phosphatase: 90 U/L
Complete blood count, including platelets: within normal limits

## Encounter 7: source 2023-07-15, practice 2026-07-15, inpatient, Internal Medicine

Chief complaint: Alcohol withdrawal seizures developing during hospitalization for community-acquired pneumonia

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Robert Thompson, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Alcohol withdrawal seizures developing during hospitalization for community-acquired pneumonia

HISTORY OF PRESENT ILLNESS:
The patient has severe alcohol use disorder with prior withdrawal including delirium tremens, currently hospitalized for pneumonia. He received his last alcoholic drink approximately 24 hours prior to admission. On his second day of hospitalization, the patient becomes increasingly agitated and tremulous. He reports feeling "jittery."

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic pancreatitis with exocrine insufficiency (diagnosed 2020-01-15)
- Alcoholic cerebellar degeneration (diagnosed 2020-07-15)
- Tuberculous spondylitis (Pott disease) (diagnosed 2021-09-15)
- Alcoholic hepatitis (diagnosed 2022-05-15)
- Uncomplicated alcohol withdrawal (diagnosed 2022-11-15)
- Delirium tremens (diagnosed 2023-01-15)
- Alcohol Use Disorder, Severe
- Alcoholic Liver Disease
- Protein-calorie malnutrition
- Thiamine deficiency

MEDICATIONS:
He takes intravenous ceftriaxone and oral azithromycin for pneumonia.

Home Medications:
- Pancrelipase (Creon) 36,000 units lipase with meals and 18,000 units with snacks
- Thiamine 100 mg orally daily
- Folic acid 1 mg orally daily
- Multivitamin 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder, cirrhosis
- Mother: Type 2 diabetes mellitus
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 1 year ago, 20 pack-year history. Alcohol: Chronic severe alcohol use disorder, consuming approximately 6 beers daily (≥72 g ethanol) for 20 years, with multiple previous withdrawal episodes and intermittent periods of sobriety lasting 1–3 months. Occupation: Construction laborer.

REVIEW OF SYSTEMS:
The patient denies any headache, chest pain, or shortness of breath.

VITAL SIGNS:
Temperature 37.8°C, blood pressure 160/100 mmHg, pulse 110 beats per minute, and respiratory rate 18 breaths per minute

PHYSICAL EXAMINATION:
Diaphoretic male with a coarse tremor of the outstretched hands. He appears anxious

LABORATORY DATA:
Normal electrolytes, renal function, and liver enzymes. His blood alcohol level is undetectable.
