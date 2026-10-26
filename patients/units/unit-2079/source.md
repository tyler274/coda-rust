# synthetic_hospital patient 2079

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 56 at first encounter, sex M, Non-Hispanic White, Employer-sponsored commercial insurance (Blue Cross Blue Shield).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-03-17 (Fri) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2023-09-04 (Mon) | outpatient | Pulmonology |
| 2 | 2020-10-15 | 2023-12-11 (Mon) | outpatient | Hematology & Oncology |
| 3 | 2021-03-15 | 2024-05-17 (Fri) | inpatient | Hematology & Oncology |
| 4 | 2022-03-15 | 2025-05-06 (Tue) | ed | Emergency Medicine |
| 5 | 2023-03-15 | 2026-05-07 (Thu) | icu | Neurology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Employer-sponsored commercial insurance (Blue Cross Blue Shield)",
 "occupation": "Long-haul truck driver",
 "alcohol_use": "Occasional (1-2 beers on weekends)",
 "family_history": [
  "Father: lung cancer (diagnosed age 62, deceased)",
  "Mother: diverticulosis",
  "Maternal uncle: Guillain-Barr\u00e9 syndrome (post-infectious)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 40 pack-year history (approximately 1.5 packs per day since age 20); two unsuccessful quit attempts in the past year using nicotine patches and group counseling",
 "home_medications": [
  {
   "dose": "1 packet (3.4g) in 8oz water daily",
   "name": "Psyllium husk (Metamucil)"
  },
  {
   "dose": "400 mg PO every 6-8 hours as needed for knee pain (chronic use)",
   "name": "Ibuprofen"
  },
  {
   "dose": "2 puffs (90 mcg/puff) every 4-6 hours as needed for wheezing",
   "name": "Albuterol HFA inhaler"
  },
  {
   "dose": "17g PO daily as needed for constipation",
   "name": "Polyethylene glycol 3350 (MiraLAX)"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Chronic constipation",
  "Osteoarthritis (bilateral knees)",
  "Chronic bronchitis",
  "Tobacco use disorder",
  "Iron deficiency anemia",
  "Stage IIIB Non-Small Cell Lung Cancer (adenocarcinoma)"
 ],
 "age_at_first_encounter": 56
}
```

Primary diagnoses: ["Acute diverticulitis of sigmoid colon", "Guillain-Barr\u00e9 syndrome", "Stage IIIB Non-Small Cell Lung Cancer", "Transfusion-related acute lung injury", "Tobacco use disorder", "Suspected malignant neoplasm of lung"]
Comorbidities: ["NSAID use", "Tobacco use disorder"]

## Encounter 0: source 2020-01-15, practice 2023-03-17, outpatient, Internal Medicine

Chief complaint: Routine evaluation for smoking cessation. Patient reports two failed quit attempts in the past year using nicotine patches and group counseling due to work-related stress.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Maria Rodriguez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine evaluation for smoking cessation. Patient reports two failed quit attempts in the past year using nicotine patches and group counseling due to work-related stress.

HISTORY OF PRESENT ILLNESS:
Over the past year, he has attempted to quit smoking twice—once by using nicotine patches and once through group counseling at a community health center. Both attempts lasted several weeks, but he ultimately relapsed due to cravings and stress at work. He says he knows smoking is bad for him, especially given his breathing problems, but at today’s visit, he says, “I’m just not ready to try again right now. Maybe someday.”

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic constipation
- Osteoarthritis (bilateral knees)
- Chronic bronchitis
- Tobacco use disorder
- Iron deficiency anemia
- Stage IIIB Non-Small Cell Lung Cancer (adenocarcinoma)

MEDICATIONS:
Home Medications:
- Psyllium husk (Metamucil) 1 packet (3.4g) in 8oz water daily
- Ibuprofen 400 mg PO every 6-8 hours as needed for knee pain (chronic use)
- Albuterol HFA inhaler 2 puffs (90 mcg/puff) every 4-6 hours as needed for wheezing
- Polyethylene glycol 3350 (MiraLAX) 17g PO daily as needed for constipation

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: lung cancer (diagnosed age 62, deceased)
- Mother: diverticulosis
- Maternal uncle: Guillain-Barré syndrome (post-infectious)

SOCIAL HISTORY:
30-pack-year smoking history

PLAN:
The physician wants to respond in a way that supports the patient’s autonomy, explores his values, and enhances his internal motivation for change.

## Encounter 1: source 2020-07-15, practice 2023-09-04, outpatient, Pulmonology

Chief complaint: Chronic cough and unintentional weight loss over several months. Patient initially attributed symptoms to chronic bronchitis but presents due to persistent severity.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. David Kim, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Chronic cough and unintentional weight loss over several months. Patient initially attributed symptoms to chronic bronchitis but presents due to persistent severity.

PAST MEDICAL HISTORY:
Active Problem List:
- Tobacco use disorder (diagnosed 2020-01-15)
- Chronic constipation
- Osteoarthritis (bilateral knees)
- Chronic bronchitis
- Iron deficiency anemia
- Stage IIIB Non-Small Cell Lung Cancer (adenocarcinoma)

MEDICATIONS:
Home Medications:
- Psyllium husk (Metamucil) 1 packet (3.4g) in 8oz water daily
- Ibuprofen 400 mg PO every 6-8 hours as needed for knee pain (chronic use)
- Albuterol HFA inhaler 2 puffs (90 mcg/puff) every 4-6 hours as needed for wheezing
- Polyethylene glycol 3350 (MiraLAX) 17g PO daily as needed for constipation

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: lung cancer (diagnosed age 62, deceased)
- Mother: diverticulosis
- Maternal uncle: Guillain-Barré syndrome (post-infectious)

SOCIAL HISTORY:
With a history of smoking

IMAGING:
He undergoes a chest CT scan, and the result is positive.

ASSESSMENT AND PLAN:
His physician estimates his pre-test probability of lung cancer is 30% based on clinical risk factors.
The CT scan's sensitivity is reported as 90%, and specificity as 85%. The positive predictive value (PPV) of the CT scan in the hospital population is 70%. His physician wants to know how best to use the test result to inform this individual patient's likelihood of having lung cancer. She mentions that likelihood ratios (LR) can be used to convert pre-test probability to post-test probability for an individual. She also notes that predictive values like PPV and NPV depend on disease prevalence in the population.

## Encounter 2: source 2020-10-15, practice 2023-12-11, outpatient, Hematology & Oncology

Chief complaint: Worsening dyspnea, persistent cough with occasional hemoptysis, and unexplained weight loss of 10 kg over 3 months despite treatment for chronic bronchitis.

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Sarah Johnson, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening dyspnea, persistent cough with occasional hemoptysis, and unexplained weight loss of 10 kg over 3 months despite treatment for chronic bronchitis.

HISTORY OF PRESENT ILLNESS:
The patient with a history of tobacco use disorder returns following his July 2020 Pulmonology evaluation. He presents with a 3-month history of worsening dyspnea, a persistent cough with occasional hemoptysis, and unexplained weight loss of 10 kg. He denies any recent travel or sick contacts.

PAST MEDICAL HISTORY:
Active Problem List:
- Tobacco use disorder (diagnosed 2020-01-15)
- Suspected malignant neoplasm of lung (diagnosed 2020-07-15)
- Chronic constipation
- Osteoarthritis (bilateral knees)
- Chronic bronchitis
- Iron deficiency anemia
- Stage IIIB Non-Small Cell Lung Cancer (adenocarcinoma)

MEDICATIONS:
Home Medications:
- Psyllium husk (Metamucil) 1 packet (3.4g) in 8oz water daily
- Ibuprofen 400 mg PO every 6-8 hours as needed for knee pain (chronic use)
- Albuterol HFA inhaler 2 puffs (90 mcg/puff) every 4-6 hours as needed for wheezing
- Polyethylene glycol 3350 (MiraLAX) 17g PO daily as needed for constipation

ALLERGIES:
NKDA

FAMILY HISTORY:
His family history is significant for lung cancer in his father.

SOCIAL HISTORY:
40-pack-year smoking history

PHYSICAL EXAMINATION:
On physical examination, you note hypertrophic changes of the skin and nails, including digital clubbing. His respiratory examination reveals decreased breath sounds and dullness to percussion at the right lung base.

LABORATORY DATA:
Laboratory tests reveal:

- Hemoglobin: 13.5 g/dL (normal: 13.8-17.2 g/dL)
- White blood cell count: 7,500/mm³ (normal: 4,500-11,000/mm³)
- Platelet count: 250,000/mm³ (normal: 150,000-450,000/mm³)
- Sodium: 140 mEq/L (normal: 135-145 mEq/L)
- Potassium: 4.2 mEq/L (normal: 3.5-5.1 mEq/L)
- Calcium: 10.2 mg/dL (normal: 8.5-10.2 mg/dL)
- Alkaline phosphatase: 120 U/L (normal: 40-130 U/L)
- Carcinoembryonic antigen (CEA): Elevated

IMAGING:
A chest X-ray shows a mass in the right lower lobe, and subsequent CT scan confirms a 4 cm mass with associated lymphadenopathy.

## Encounter 3: source 2021-03-15, practice 2024-05-17, inpatient, Hematology & Oncology

Chief complaint: Severe fatigue and progressive shortness of breath requiring hospital admission for evaluation of worsening anemia during chemotherapy treatment.

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Sarah Johnson, MD
Department: Hematology & Oncology
Visit Type: inpatient

CHIEF COMPLAINT:
Severe fatigue and progressive shortness of breath requiring hospital admission for evaluation of worsening anemia during chemotherapy treatment.

HISTORY OF PRESENT ILLNESS:
The patient is a 57-year-old male with Stage IIIB non-small cell lung cancer diagnosed five months prior who presents during active chemotherapy treatment. He is admitted to the hospital for evaluation of fatigue and shortness of breath over the past month. After stabilization with intravenous fluids, he is transfused with packed red blood cells. Approximately two hours after the transfusion is started, he begins to feel chills and discomfort.

PAST MEDICAL HISTORY:
Active Problem List:
- Tobacco use disorder (diagnosed 2020-01-15)
- Suspected malignant neoplasm of lung (diagnosed 2020-07-15)
- Stage IIIB Non-Small Cell Lung Cancer (diagnosed 2020-10-15)
- Chronic constipation
- Osteoarthritis (bilateral knees)
- Chronic bronchitis
- Iron deficiency anemia
- Stage IIIB Non-Small Cell Lung Cancer (adenocarcinoma)

MEDICATIONS:
He takes chronic NSAID use.

Home Medications:
- Psyllium husk (Metamucil) 1 packet (3.4g) in 8oz water daily
- Ibuprofen 400 mg PO every 6-8 hours as needed for knee pain (chronic use)
- Albuterol HFA inhaler 2 puffs (90 mcg/puff) every 4-6 hours as needed for wheezing
- Polyethylene glycol 3350 (MiraLAX) 17g PO daily as needed for constipation

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: lung cancer (diagnosed age 62, deceased)
- Mother: diverticulosis
- Maternal uncle: Guillain-Barré syndrome (post-infectious)

SOCIAL HISTORY:
Smoking: Current smoker, 40 pack-year history (approximately 1.5 packs per day since age 20); two unsuccessful quit attempts in the past year using nicotine patches and group counseling. Alcohol: Occasional (1-2 beers on weekends). Occupation: Long-haul truck driver.

REVIEW OF SYSTEMS:
He denies back pain, dark urine, rash, or difficulty breathing.

VITAL SIGNS:
His temperature is measured at 38.5°C (101.3°F), pulse is 102/min, blood pressure is 124/78 mmHg, and oxygen saturation is 98% on room air.

PHYSICAL EXAMINATION:
Pale conjunctiva and mild tachycardia

LABORATORY DATA:
Hemoglobin of 7.4 g/dL and MCV of 78 fL. A Coombs test is negative.

PLAN:
The transfusion is temporarily stopped, and the blood bank is notified for evaluation.

## Encounter 4: source 2022-03-15, practice 2025-05-06, ed, Emergency Medicine

Chief complaint: Acute onset of constant sharp left lower quadrant abdominal pain for 2 days, worsened with movement and associated with low-grade fever.

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. James Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset of constant sharp left lower quadrant abdominal pain for 2 days, worsened with movement and associated with low-grade fever.

HISTORY OF PRESENT ILLNESS:
This 58-year-old male with Stage IIIB lung cancer s/p TRALI presents with a 2-day history of left lower quadrant abdominal pain. The pain is constant, sharp, and worsened with movement.

PAST MEDICAL HISTORY:
Active Problem List:
- Tobacco use disorder (diagnosed 2020-01-15)
- Suspected malignant neoplasm of lung (diagnosed 2020-07-15)
- Stage IIIB Non-Small Cell Lung Cancer (diagnosed 2020-10-15)
- Transfusion-related acute lung injury (diagnosed 2021-03-15)
- Chronic constipation
- Osteoarthritis (bilateral knees)
- Chronic bronchitis
- Iron deficiency anemia
- Stage IIIB Non-Small Cell Lung Cancer (adenocarcinoma)

MEDICATIONS:
He takes over-the-counter fiber supplements; occasional use of ibuprofen for knee pain.

Home Medications:
- Psyllium husk (Metamucil) 1 packet (3.4g) in 8oz water daily
- Albuterol HFA inhaler 2 puffs (90 mcg/puff) every 4-6 hours as needed for wheezing
- Polyethylene glycol 3350 (MiraLAX) 17g PO daily as needed for constipation

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: lung cancer (diagnosed age 62, deceased)
- Mother: diverticulosis
- Maternal uncle: Guillain-Barré syndrome (post-infectious)

SOCIAL HISTORY:
Diet is high in red meats and low in fruits and vegetables; 30-pack-year smoking history

VITAL SIGNS:
BMI of 32 kg/m^2; temperature is 38.3°C (100.9°F), heart rate is 102 beats per minute, and blood pressure is 135/85 mmHg

PHYSICAL EXAMINATION:
Abdominal examination reveals tenderness and guarding in the left lower quadrant without rebound tenderness. There is no rectal bleeding.

LABORATORY DATA:
White blood cell count: 14,000/mm^3; Hemoglobin: 13.8 g/dL; C-reactive protein: 24 mg/L (normal <5 mg/L)

## Encounter 5: source 2023-03-15, practice 2026-05-07, icu, Neurology

Chief complaint: Progressive bilateral lower extremity weakness, ascending paresthesias, and mild respiratory difficulty over 3 days following a recent respiratory infection.

ENCOUNTER NOTE
Date: 2023-03-15
Provider: Dr. Aisha Patel, MD
Department: Neurology
Visit Type: icu

CHIEF COMPLAINT:
Progressive bilateral lower extremity weakness, ascending paresthesias, and mild respiratory difficulty over 3 days following a recent respiratory infection.

HISTORY OF PRESENT ILLNESS:
The patient is a 59-year-old male with Stage IIIB non-small cell lung cancer and chronic bronchitis, last seen in 2022 for acute diverticulitis, who presents with new neurological complaints. Over the past three days, he has developed tingling in his feet, which has progressed to bilateral lower extremity weakness. He reports mild shortness of breath and says his hands now feel weak as well. He was recently treated for a respiratory infection.

PAST MEDICAL HISTORY:
Active Problem List:
- Tobacco use disorder (diagnosed 2020-01-15)
- Suspected malignant neoplasm of lung (diagnosed 2020-07-15)
- Stage IIIB Non-Small Cell Lung Cancer (diagnosed 2020-10-15)
- Transfusion-related acute lung injury (diagnosed 2021-03-15)
- Acute diverticulitis of sigmoid colon (diagnosed 2022-03-15)
- Chronic constipation
- Osteoarthritis (bilateral knees)
- Chronic bronchitis
- Iron deficiency anemia
- Stage IIIB Non-Small Cell Lung Cancer (adenocarcinoma)

MEDICATIONS:
Home Medications:
- Psyllium husk (Metamucil) 1 packet (3.4g) in 8oz water daily
- Ibuprofen 400 mg PO every 6-8 hours as needed for knee pain (chronic use)
- Albuterol HFA inhaler 2 puffs (90 mcg/puff) every 4-6 hours as needed for wheezing
- Polyethylene glycol 3350 (MiraLAX) 17g PO daily as needed for constipation

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: lung cancer (diagnosed age 62, deceased)
- Mother: diverticulosis
- Maternal uncle: Guillain-Barré syndrome (post-infectious)

SOCIAL HISTORY:
He has a 30-pack-year smoking history

VITAL SIGNS:
Vital signs show a blood pressure of 122/78 mm Hg, heart rate of 96/min, respiratory rate of 20/min, and oxygen saturation of 94% on room air.

PHYSICAL EXAMINATION:
Neurologic examination reveals diminished reflexes in all extremities and reduced strength in the lower limbs. There is no muscle tenderness or fasciculations. The remainder of the physical exam is unremarkable.
