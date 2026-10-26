# synthetic_hospital patient 2028

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 57 at first encounter, sex M, Non-Hispanic White, Private insurance (Blue Cross Blue Shield).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-04-28 (Wed) | outpatient | Gastroenterology |
| 1 | 2020-04-15 | 2021-07-28 (Wed) | outpatient | Internal Medicine |
| 2 | 2021-04-15 | 2022-07-20 (Wed) | outpatient | Cardiology |
| 3 | 2022-01-15 | 2023-04-25 (Tue) | outpatient | Ear Nose & Throat |
| 4 | 2022-07-15 | 2023-10-18 (Wed) | outpatient | Ear Nose & Throat |
| 5 | 2023-01-15 | 2024-04-15 (Mon) | outpatient | Ear Nose & Throat |
| 6 | 2024-01-15 | 2025-04-25 (Fri) | outpatient | Hematology & Oncology |
| 7 | 2024-07-15 | 2025-10-13 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private insurance (Blue Cross Blue Shield)",
 "occupation": "Retired construction foreman",
 "alcohol_use": "Heavy daily alcohol consumption (6-8 beers per day)",
 "family_history": [
  "Father: Lung cancer (deceased age 64)",
  "Mother: Type 2 Diabetes Mellitus",
  "Paternal uncle: Myocardial infarction"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 40 pack-year history (approximately 1 pack per day since age 17)",
 "home_medications": [
  {
   "dose": "10 mg daily",
   "name": "Amlodipine"
  },
  {
   "dose": "20 mg daily",
   "name": "Lisinopril"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Chronic Pancreatitis"
 ],
 "age_at_first_encounter": 57
}
```

Primary diagnoses: ["Spinal epidural hematoma", "Lung cancer screening candidate (high-risk smoker)", "Leriche syndrome (Aortoiliac occlusive disease)", "Oral cavity squamous cell carcinoma", "Pancreatic adenocarcinoma (mucin-producing) with chronic disseminated intravascular coagulation", "Squamous cell carcinoma of the tonsil", "Chronic pancreatitis (dietary management)", "Head and neck squamous cell carcinoma"]
Comorbidities: ["Alcohol use disorder", "Tobacco use disorder", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2021-04-28, outpatient, Gastroenterology

Chief complaint: Chronic epigastric discomfort and unintentional weight loss

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Chronic epigastric discomfort and unintentional weight loss

HISTORY OF PRESENT ILLNESS:
He describes the pain as dull and non-radiating, worsening after meals, and often accompanied by bloating and greasy, foul-smelling stools. He has no history of blood in stool, vomiting, or fevers.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Chronic Pancreatitis

His past medical history is notable for multiple prior hospitalizations for acute pancreatitis, most recently eight months ago.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He is not taking any regular medications and denies NSAID use.

Home Medications:
- Amlodipine 10 mg daily
- Lisinopril 20 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (deceased age 64)
- Mother: Type 2 Diabetes Mellitus
- Paternal uncle: Myocardial infarction

SOCIAL HISTORY:
He drinks six to eight beers daily and smokes one pack of cigarettes per day.

PHYSICAL EXAMINATION:
On exam, he appears thin but in no distress. BMI is 18.1 kg/m². There is mild epigastric tenderness without rebound or guarding.

LABORATORY DATA:
Laboratory testing reveals a normal serum calcium and lipase. Stool analysis is positive for fat and undigested muscle fibers. D-xylose test results are within normal limits.

IMAGING:
A CT scan of the abdomen is performed and shown below.

## Encounter 1: source 2020-04-15, practice 2021-07-28, outpatient, Internal Medicine

Chief complaint: Routine check-up and lung cancer screening inquiry

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine check-up and lung cancer screening inquiry

HISTORY OF PRESENT ILLNESS:
This 57-year-old male with hypertension and chronic pancreatitis returns following his Gastroenterology evaluation. He has no complaints but is concerned about his risk for lung cancer due to his 30-pack-year smoking history. He inquires about lung cancer screening.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic pancreatitis (dietary management) (diagnosed 2020-01-15)
- Hypertension
- Chronic Pancreatitis

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Amlodipine 10 mg daily
- Lisinopril 20 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (deceased age 64)
- Mother: Type 2 Diabetes Mellitus
- Paternal uncle: Myocardial infarction

SOCIAL HISTORY:
History of smoking. 30-pack-year smoking history.

VITAL SIGNS:
Blood pressure 142/88 mmHg

PHYSICAL EXAMINATION:
Unremarkable

PLAN:
Discussed the option of low-dose computed tomography (LDCT) and explained the importance of understanding the trade-offs between sensitivity and specificity when interpreting screening results. Mentioned a new blood biomarker test for lung cancer that is under consideration. The receiver operating characteristic (ROC) curve for this test shows that at a cutoff value of 50 units, the sensitivity is 90% and the specificity is 70%. If the cutoff value is increased to 70 units, the sensitivity decreases to 75%, but the specificity increases to 85%. The patient asks which cutoff value would be more appropriate for him, considering his risk factors.

## Encounter 2: source 2021-04-15, practice 2022-07-20, outpatient, Cardiology

Chief complaint: Progressive difficulty walking with calf and buttock pain

ENCOUNTER NOTE
Date: 2021-04-15
Provider: Dr. Priya Patel, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive difficulty walking with calf and buttock pain

HISTORY OF PRESENT ILLNESS:
The patient is a 58-year-old male with hypertension and chronic pancreatitis previously seen in Gastroenterology and Internal Medicine who presents with progressive difficulty walking over the past year. He reports that after walking about 200 meters, he develops cramping pain in his calves and buttocks that resolves with rest. Recently, he has also noticed erectile dysfunction that has become persistent.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic pancreatitis (dietary management) (diagnosed 2020-01-15)
- Lung cancer screening candidate (high-risk smoker) (diagnosed 2020-04-15)
- Hypertension
- Chronic Pancreatitis

He has a history of poorly controlled hypertension.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Amlodipine 10 mg daily
- Lisinopril 20 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (deceased age 64)
- Mother: Type 2 Diabetes Mellitus
- Paternal uncle: Myocardial infarction

SOCIAL HISTORY:
Heavy tobacco use

REVIEW OF SYSTEMS:
He denies trauma, fever, or recent infection.

PHYSICAL EXAMINATION:
On examination, the patient has diminished femoral pulses bilaterally, and dorsalis pedis pulses are not palpable. The lower extremities are cool to touch with sparse hair distribution. Neurologic exam shows intact strength and sensation in both legs. Cardiac exam reveals no murmurs, and lung auscultation is clear.

LABORATORY DATA:
Laboratory studies show normal renal function and hemoglobin A1c.

OTHER STUDIES:
Ankle-brachial index is markedly reduced.

## Encounter 3: source 2022-01-15, practice 2023-04-25, outpatient, Ear Nose & Throat

Chief complaint: Persistent left-sided ear pain for 6 weeks

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. James O'Brien, MD
Department: Ear Nose & Throat
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent left-sided ear pain for 6 weeks

HISTORY OF PRESENT ILLNESS:
This patient with hypertension, chronic pancreatitis, and Leriche syndrome presents with ear pain. He reports that the pain is dull and constant, and it occasionally radiates to the throat. He denies hearing loss, tinnitus, or discharge from the ear.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic pancreatitis (dietary management) (diagnosed 2020-01-15)
- Lung cancer screening candidate (high-risk smoker) (diagnosed 2020-04-15)
- Leriche syndrome (Aortoiliac occlusive disease) (diagnosed 2021-04-15)
- Hypertension
- Chronic Pancreatitis

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Amlodipine 10 mg daily
- Lisinopril 20 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (deceased age 64)
- Mother: Type 2 Diabetes Mellitus
- Paternal uncle: Myocardial infarction

SOCIAL HISTORY:
The patient has a 40-pack-year smoking history and consumes alcohol daily.

VITAL SIGNS:
His vital signs are within normal limits.

PHYSICAL EXAMINATION:
On examination, the left ear appears normal with an intact tympanic membrane and no signs of inflammation or infection. However, palpation of the neck reveals a firm, non-tender lymph node in the left cervical chain. The patient&#x27;s voice is hoarse, which he attributes to "just a bad habit of clearing my throat." He has no respiratory distress or stridor.

LABORATORY DATA:
Laboratory values:
- Complete blood count: Within normal limits
- Erythrocyte sedimentation rate: 20 mm/hr (normal < 20 mm/hr)
- Basic metabolic panel: Within normal limits

## Encounter 4: source 2022-07-15, practice 2023-10-18, outpatient, Ear Nose & Throat

Chief complaint: Persistent sore throat and difficulty swallowing for 2 months

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. James O'Brien, MD
Department: Ear Nose & Throat
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent sore throat and difficulty swallowing for 2 months

HISTORY OF PRESENT ILLNESS:
The patient is a 59-year-old male with hypertension and chronic pancreatitis who returns to the Ear Nose and Throat clinic following his prior evaluation for head and neck squamous cell carcinoma. He denies any recent weight loss, fever, or night sweats.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic pancreatitis (dietary management) (diagnosed 2020-01-15)
- Lung cancer screening candidate (high-risk smoker) (diagnosed 2020-04-15)
- Leriche syndrome (Aortoiliac occlusive disease) (diagnosed 2021-04-15)
- Head and neck squamous cell carcinoma (diagnosed 2022-01-15)
- Hypertension
- Chronic Pancreatitis

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Amlodipine 10 mg daily
- Lisinopril 20 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (deceased age 64)
- Mother: Type 2 Diabetes Mellitus
- Paternal uncle: Myocardial infarction

SOCIAL HISTORY:
40-pack-year smoking history and chronic alcohol use

PHYSICAL EXAMINATION:
On examination, you note a white, thickened plaque on the lateral border of his tongue that does not scrape off with a tongue depressor. The area is firm to palpation and there is a small area of ulceration at the center of the plaque. There are no palpable cervical lymph nodes. The rest of the oral cavity examination is unremarkable. Given the patient's history and physical examination findings,

## Encounter 5: source 2023-01-15, practice 2024-04-15, outpatient, Ear Nose & Throat

Chief complaint: Persistent sore throat with 10-pound unintentional weight loss

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Aisha Washington, MD
Department: Ear Nose & Throat
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent sore throat with 10-pound unintentional weight loss

HISTORY OF PRESENT ILLNESS:
The patient with chronic pancreatitis and prior ENT cancers returns with sore throat for the past 8 weeks, denying recent upper respiratory infections but reporting 10-pound unintentional weight loss over the last two months.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic pancreatitis (dietary management) (diagnosed 2020-01-15)
- Lung cancer screening candidate (high-risk smoker) (diagnosed 2020-04-15)
- Leriche syndrome (Aortoiliac occlusive disease) (diagnosed 2021-04-15)
- Head and neck squamous cell carcinoma (diagnosed 2022-01-15)
- Oral cavity squamous cell carcinoma (diagnosed 2022-07-15)
- Hypertension
- Chronic Pancreatitis

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Amlodipine 10 mg daily
- Lisinopril 20 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (deceased age 64)
- Mother: Type 2 Diabetes Mellitus
- Paternal uncle: Myocardial infarction

SOCIAL HISTORY:
40-pack-year smoking history. He has no history of alcohol abuse or exposure to human papillomavirus.

VITAL SIGNS:
His vital signs are within normal limits.

PHYSICAL EXAMINATION:
On examination, you note a 2 cm ulcerative lesion on the right tonsil with ipsilateral cervical lymphadenopathy. The patient has a hoarse voice and no apparent lesions on the oral mucosa or tongue.

LABORATORY DATA:
Laboratory values:
- Complete Blood Count (CBC): Within normal limits
- Erythrocyte Sedimentation Rate (ESR): 20 mm/hr (normal: 0-22 mm/hr)
- Human Immunodeficiency Virus (HIV) test: Negative

OTHER STUDIES:
A biopsy of the tonsillar lesion is performed.

## Encounter 6: source 2024-01-15, practice 2025-04-25, outpatient, Hematology & Oncology

Chief complaint: Epigastric pain radiating to back with new-onset diabetes and weight loss

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. David Kim, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Epigastric pain radiating to back with new-onset diabetes and weight loss

HISTORY OF PRESENT ILLNESS:
The patient, with a known history of chronic pancreatitis and prior head and neck malignancies, presents to the clinic with a 3-month history of epigastric pain radiating to the back, unexplained weight loss, and new-onset diabetes mellitus. He denies any recent trauma or bleeding episodes.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic pancreatitis (dietary management) (diagnosed 2020-01-15)
- Lung cancer screening candidate (high-risk smoker) (diagnosed 2020-04-15)
- Leriche syndrome (Aortoiliac occlusive disease) (diagnosed 2021-04-15)
- Head and neck squamous cell carcinoma (diagnosed 2022-01-15)
- Oral cavity squamous cell carcinoma (diagnosed 2022-07-15)
- Squamous cell carcinoma of the tonsil (diagnosed 2023-01-15)
- Hypertension
- Chronic Pancreatitis

His past medical history is significant for chronic pancreatitis and a 20-pack-year smoking history.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Amlodipine 10 mg daily
- Lisinopril 20 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (deceased age 64)
- Mother: Type 2 Diabetes Mellitus
- Paternal uncle: Myocardial infarction

SOCIAL HISTORY:
Smoking: Current smoker, 40 pack-year history (approximately 1 pack per day since age 17). Alcohol: Heavy daily alcohol consumption (6-8 beers per day). Occupation: Retired construction foreman.

PHYSICAL EXAMINATION:
On physical examination, he appears jaundiced with scleral icterus. An abdominal examination reveals a palpable, nontender gallbladder.

LABORATORY DATA:
Laboratory tests reveal the following:

- Hemoglobin: 12.5 g/dL (normal: 13.8-17.2 g/dL)
- Platelet count: 250,000/µL (normal: 150,000-450,000/µL)
- Prothrombin time (PT): 12 seconds (normal: 9.5-13.5 seconds)
- Activated partial thromboplastin time (aPTT): 30 seconds (normal: 25-35 seconds)
- Fibrinogen: 400 mg/dL (normal: 200-400 mg/dL)
- D-dimer: Elevated

IMAGING:
An abdominal CT scan shows a mass in the head of the pancreas with liver metastases.

## Encounter 7: source 2024-07-15, practice 2025-10-13, ed, Emergency Medicine

Chief complaint: Sudden onset severe tearing back pain with leg weakness and difficulty urinating

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Emily Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset severe tearing back pain with leg weakness and difficulty urinating

HISTORY OF PRESENT ILLNESS:
This patient with hypertension, chronic pancreatitis, and pancreatic cancer presents with back pain that began several hours ago. The pain is described as tearing and radiates to his abdomen. He also reports difficulty urinating and weakness in both legs.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic pancreatitis (dietary management) (diagnosed 2020-01-15)
- Lung cancer screening candidate (high-risk smoker) (diagnosed 2020-04-15)
- Leriche syndrome (Aortoiliac occlusive disease) (diagnosed 2021-04-15)
- Head and neck squamous cell carcinoma (diagnosed 2022-01-15)
- Oral cavity squamous cell carcinoma (diagnosed 2022-07-15)
- Squamous cell carcinoma of the tonsil (diagnosed 2023-01-15)
- Pancreatic adenocarcinoma (mucin-producing) with chronic disseminated intravascular coagulation (diagnosed 2024-01-15)
- Hypertension
- Chronic Pancreatitis

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Amlodipine 10 mg daily
- Lisinopril 20 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (deceased age 64)
- Mother: Type 2 Diabetes Mellitus
- Paternal uncle: Myocardial infarction

SOCIAL HISTORY:
Smoking

VITAL SIGNS:
On physical examination, his blood pressure is 180/95 mmHg, heart rate is 88 beats per minute, and he is afebrile.

PHYSICAL EXAMINATION:
Neurological examination reveals decreased strength in the lower extremities, more pronounced in the flexor groups, and loss of pain and temperature sensation over the trunk and legs. Light touch and proprioception are intact. The patient has a flaccid bladder and decreased rectal tone.

IMAGING:
An urgent CT angiography of the chest and abdomen is performed, revealing a descending thoracic aortic dissection extending to the level of T10.
