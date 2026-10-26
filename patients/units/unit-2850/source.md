# synthetic_hospital patient 2850

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 64 at first encounter, sex F, Non-Hispanic White, Medicare due to disability, supplemental private insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-06-26 (Mon) | outpatient | Rheumatology |
| 1 | 2020-07-15 | 2023-12-26 (Tue) | outpatient | Hematology & Oncology |
| 2 | 2021-07-15 | 2024-12-30 (Mon) | outpatient | Internal Medicine |
| 3 | 2022-07-15 | 2025-12-30 (Tue) | outpatient | Pulmonology |
| 4 | 2022-10-15 | 2026-03-22 (Sun) | ed | Emergency Medicine |
| 5 | 2022-11-15 | 2026-05-06 (Wed) | ed | Emergency Medicine |
| 6 | 2022-12-15 | 2026-06-05 (Fri) | ed | Emergency Medicine |
| 7 | 2023-01-15 | 2026-06-20 (Sat) | icu | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare due to disability, supplemental private insurance",
 "occupation": "Retired registered nurse",
 "alcohol_use": "History of heavy alcohol use, in recovery for 18 months",
 "family_history": [
  "Father: Myocardial infarction at age 68, hypertension",
  "Mother: Hypertension, osteoarthritis",
  "Older brother: Lung cancer",
  "Paternal uncle: Lymphoma"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 40 pack-years, quit 5 years ago",
 "home_medications": [
  {
   "dose": "5 mg daily (dose adjusted per INR)",
   "name": "Warfarin"
  },
  {
   "dose": "25 mg oral daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "10 mg oral daily",
   "name": "Amlodipine"
  },
  {
   "dose": "90 mcg, 2 inhalations every 4-6 hours as needed for shortness of breath",
   "name": "Albuterol HFA inhaler"
  },
  {
   "dose": "500 mg oral as needed for pain",
   "name": "Acetaminophen"
  }
 ],
 "surgical_history": [
  "Mechanical mitral valve replacement (10 years prior)",
  "Right upper lobe bronchoscopy with biopsy (recent)"
 ],
 "chronic_conditions": [
  "Hypertension, secondary to renal artery stenosis",
  "Rheumatic heart disease, status post mechanical mitral valve replacement",
  "Chronic obstructive pulmonary disease (COPD), moderate severity",
  "Osteoarthritis, bilateral hands",
  "Follicular lymphoma, indolent, diagnosed 2 years prior",
  "Non-small cell lung cancer, Stage IIIB, recently diagnosed"
 ],
 "age_at_first_encounter": 64
}
```

Primary diagnoses: ["Renal artery stenosis", "Wernicke encephalopathy", "Follicular lymphoma", "Prosthetic mitral valve thrombosis", "Ramsay Hunt syndrome (Herpes zoster oticus)", "Stage IIIB Non-Small Cell Lung Cancer", "Osteoarthritis of hand", "Hypochloremic metabolic alkalosis secondary to vomiting with hypovolemia"]
Comorbidities: ["Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2023-06-26, outpatient, Rheumatology

Chief complaint: 6-month history of pain and stiffness in her hands

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
6-month history of pain and stiffness in her hands

HISTORY OF PRESENT ILLNESS:
She reports that the symptoms are worse in the morning and after periods of inactivity, but they tend to improve with movement throughout the day. She denies any recent trauma or systemic symptoms such as fever or weight loss.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension, secondary to renal artery stenosis
- Rheumatic heart disease, status post mechanical mitral valve replacement
- Chronic obstructive pulmonary disease (COPD), moderate severity
- Osteoarthritis, bilateral hands
- Follicular lymphoma, indolent, diagnosed 2 years prior
- Non-small cell lung cancer, Stage IIIB, recently diagnosed

Her medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- Mechanical mitral valve replacement (10 years prior)
- Right upper lobe bronchoscopy with biopsy (recent)

MEDICATIONS:
Hydrochlorothiazide

Home Medications:
- Warfarin 5 mg daily (dose adjusted per INR)
- Amlodipine 10 mg oral daily
- Albuterol HFA inhaler 90 mcg, 2 inhalations every 4-6 hours as needed for shortness of breath
- Acetaminophen 500 mg oral as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 68, hypertension
- Mother: Hypertension, osteoarthritis
- Older brother: Lung cancer
- Paternal uncle: Lymphoma

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 5 years ago. Alcohol: History of heavy alcohol use, in recovery for 18 months. Occupation: Retired registered nurse.

PHYSICAL EXAMINATION:
On physical examination, you note bony enlargements at the distal interphalangeal joints of both hands, which are tender to palpation. There is also similar, but less pronounced, enlargement at the proximal interphalangeal joints. No erythema, warmth, or rashes are observed. The rest of the physical examination is unremarkable.

## Encounter 1: source 2020-07-15, practice 2023-12-26, outpatient, Hematology & Oncology

Chief complaint: Intermittent, painless swelling in her neck for 2 years

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent, painless swelling in her neck for 2 years

HISTORY OF PRESENT ILLNESS:
Following her January 2020 Rheumatology visit, the patient, who has follicular lymphoma and non-small cell lung cancer, presents to Hematology & Oncology with a 2-year history of intermittent, painless swelling in her neck. She reports that the swellings vary in size and occasionally decrease without any intervention. She denies fevers, night sweats, or unintentional weight loss.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis of hand (diagnosed 2020-01-15)
- Hypertension, secondary to renal artery stenosis
- Rheumatic heart disease, status post mechanical mitral valve replacement
- Chronic obstructive pulmonary disease (COPD), moderate severity
- Osteoarthritis, bilateral hands
- Follicular lymphoma, indolent, diagnosed 2 years prior
- Non-small cell lung cancer, Stage IIIB, recently diagnosed

PAST SURGICAL HISTORY:
- Mechanical mitral valve replacement (10 years prior)
- Right upper lobe bronchoscopy with biopsy (recent)

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Warfarin 5 mg daily (dose adjusted per INR)
- Amlodipine 10 mg oral daily
- Albuterol HFA inhaler 90 mcg, 2 inhalations every 4-6 hours as needed for shortness of breath
- Acetaminophen 500 mg oral as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 68, hypertension
- Mother: Hypertension, osteoarthritis
- Older brother: Lung cancer
- Paternal uncle: Lymphoma

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 5 years ago. Alcohol: History of heavy alcohol use, in recovery for 18 months. Occupation: Retired registered nurse.

PHYSICAL EXAMINATION:
On physical examination, you note multiple rubbery, non-tender lymph nodes in the cervical and axillary regions. The largest cervical node is approximately 3 cm in diameter. The spleen is palpable 2 cm below the left costal margin.

LABORATORY DATA:
Hemoglobin: 13.2 g/dL (normal: 12-16 g/dL)
- White blood cell count: 7,800/mm³ (normal: 4,500-11,000/mm³)
- Platelet count: 245,000/mm³ (normal: 150,000-450,000/mm³)
- Lactate dehydrogenase (LDH): 190 U/L (normal: 140-280 U/L)
- Beta-2 microglobulin: Elevated

IMAGING:
A chest X-ray reveals no mediastinal or hilar lymphadenopathy.

## Encounter 2: source 2021-07-15, practice 2024-12-30, outpatient, Internal Medicine

Chief complaint: Follow-up appointment for worsening hypertension control

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Jennifer Okafor, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up appointment for worsening hypertension control

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of her hypertension in the setting of her known follicular lymphoma and osteoarthritis. She has a 30-year history of hypertension well-controlled with multiple medications until approximately 6 months ago, in which it was recorded at 150/90. At an office visit 4 months ago, it was measured at 160/95. Today it is 180/100.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis of hand (diagnosed 2020-01-15)
- Follicular lymphoma (diagnosed 2020-07-15)
- Hypertension, secondary to renal artery stenosis
- Rheumatic heart disease, status post mechanical mitral valve replacement
- Chronic obstructive pulmonary disease (COPD), moderate severity
- Osteoarthritis, bilateral hands
- Follicular lymphoma, indolent, diagnosed 2 years prior
- Non-small cell lung cancer, Stage IIIB, recently diagnosed

PAST SURGICAL HISTORY:
- Mechanical mitral valve replacement (10 years prior)
- Right upper lobe bronchoscopy with biopsy (recent)

MEDICATIONS:
Home Medications:
- Warfarin 5 mg daily (dose adjusted per INR)
- Hydrochlorothiazide 25 mg oral daily
- Amlodipine 10 mg oral daily
- Albuterol HFA inhaler 90 mcg, 2 inhalations every 4-6 hours as needed for shortness of breath
- Acetaminophen 500 mg oral as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 68, hypertension
- Mother: Hypertension, osteoarthritis
- Older brother: Lung cancer
- Paternal uncle: Lymphoma

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 5 years ago. Alcohol: History of heavy alcohol use, in recovery for 18 months. Occupation: Retired registered nurse.

PHYSICAL EXAMINATION:
Cardiopulmonary examination is normal. On the left side of her neck there is auscultation of a common carotid bruit. JVD is not elevated.

LABORATORY DATA:
Creatinine is 1.0 mg/dL. Blood urea nitrogen is 15 mg/dL.

## Encounter 3: source 2022-07-15, practice 2025-12-30, outpatient, Pulmonology

Chief complaint: Progressive fatigue and unintentional weight loss over 3 months

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. David Kim, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue and unintentional weight loss over 3 months

HISTORY OF PRESENT ILLNESS:
The patient with mechanical mitral valve replacement, follicular lymphoma, and renal artery stenosis presents to the clinic with a 3-month history of progressive fatigue and a 10-pound unintentional weight loss. She denies fever, night sweats, or changes in appetite.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis of hand (diagnosed 2020-01-15)
- Follicular lymphoma (diagnosed 2020-07-15)
- Renal artery stenosis (diagnosed 2021-07-15)
- Hypertension, secondary to renal artery stenosis
- Rheumatic heart disease, status post mechanical mitral valve replacement
- Chronic obstructive pulmonary disease (COPD), moderate severity
- Osteoarthritis, bilateral hands
- Follicular lymphoma, indolent, diagnosed 2 years prior
- Non-small cell lung cancer, Stage IIIB, recently diagnosed

Her past medical history is significant for hypertension, controlled with hydrochlorothiazide, and a 40-pack-year smoking history.

PAST SURGICAL HISTORY:
- Mechanical mitral valve replacement (10 years prior)
- Right upper lobe bronchoscopy with biopsy (recent)

MEDICATIONS:
Home Medications:
- Warfarin 5 mg daily (dose adjusted per INR)
- Hydrochlorothiazide 25 mg oral daily
- Amlodipine 10 mg oral daily
- Albuterol HFA inhaler 90 mcg, 2 inhalations every 4-6 hours as needed for shortness of breath
- Acetaminophen 500 mg oral as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 68, hypertension
- Mother: Hypertension, osteoarthritis
- Older brother: Lung cancer
- Paternal uncle: Lymphoma

SOCIAL HISTORY:
She quit smoking 5 years ago. The patient is accompanied by her adult daughter. Before discussing the biopsy results, the physician asks the patient if she would prefer to have the conversation in private or with her daughter present. The patient opts to have her daughter stay.

PHYSICAL EXAMINATION:
On physical examination, she appears cachectic. Lung auscultation reveals decreased breath sounds in the right lower lobe. No lymphadenopathy is palpable.

IMAGING:
A chest X-ray shows a 4 cm mass in the right lower lobe of the lung.

PATHOLOGY:
Pathology confirms non-small cell lung cancer.

OTHER STUDIES:
A CT-guided biopsy of the mass is performed

## Encounter 4: source 2022-10-15, practice 2026-03-22, ed, Emergency Medicine

Chief complaint: Severe left ear pain, rash, and acute facial droop

ENCOUNTER NOTE
Date: 2022-10-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe left ear pain, rash, and acute facial droop

HISTORY OF PRESENT ILLNESS:
The patient is a 66-year-old female with a history of follicular lymphoma and Stage IIIB non-small cell lung cancer diagnosed three months prior who presents to the emergency department with a 3-day history of severe pain in her left ear and a rash that has developed around her left auricle and external auditory canal. She also complains of difficulty closing her left eye and drooping of the left side of her mouth that started a day after the ear pain began.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis of hand (diagnosed 2020-01-15)
- Follicular lymphoma (diagnosed 2020-07-15)
- Renal artery stenosis (diagnosed 2021-07-15)
- Stage IIIB Non-Small Cell Lung Cancer (diagnosed 2022-07-15)
- Hypertension, secondary to renal artery stenosis
- Rheumatic heart disease, status post mechanical mitral valve replacement
- Chronic obstructive pulmonary disease (COPD), moderate severity
- Osteoarthritis, bilateral hands
- Follicular lymphoma, indolent, diagnosed 2 years prior
- Non-small cell lung cancer, Stage IIIB, recently diagnosed

PAST SURGICAL HISTORY:
- Mechanical mitral valve replacement (10 years prior)
- Right upper lobe bronchoscopy with biopsy (recent)

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Warfarin 5 mg daily (dose adjusted per INR)
- Amlodipine 10 mg oral daily
- Albuterol HFA inhaler 90 mcg, 2 inhalations every 4-6 hours as needed for shortness of breath
- Acetaminophen 500 mg oral as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 68, hypertension
- Mother: Hypertension, osteoarthritis
- Older brother: Lung cancer
- Paternal uncle: Lymphoma

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 5 years ago. Alcohol: History of heavy alcohol use, in recovery for 18 months. Occupation: Retired registered nurse.

PHYSICAL EXAMINATION:
Erythematous vesicular rash on the left auricle and in the external auditory canal. There is also left-sided facial weakness with loss of forehead wrinkling, inability to close the left eye, and drooping of the left corner of the mouth.

OTHER STUDIES:
Audiometry shows mild sensorineural hearing loss on the left side.

## Encounter 5: source 2022-11-15, practice 2026-05-06, ed, Emergency Medicine

Chief complaint: Acute confusion, unsteadiness, and memory loss over several days

ENCOUNTER NOTE
Date: 2022-11-15
Provider: Dr. James Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute confusion, unsteadiness, and memory loss over several days

HISTORY OF PRESENT ILLNESS:
The patient is a 66-year-old female with a history of Stage IIIB non-small cell lung cancer, follicular lymphoma, and Ramsay Hunt syndrome diagnosed one month prior, who presents with acute confusion, unsteadiness, and memory loss evolving over several days. According to her son, she has been confabulating stories to explain recent events that she clearly does not remember and has become "clumsy," frequently bumping into furniture. He additionally reports that she recently stated she was preparing to attend a medical seminar abroad, although no such plans exist.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis of hand (diagnosed 2020-01-15)
- Follicular lymphoma (diagnosed 2020-07-15)
- Renal artery stenosis (diagnosed 2021-07-15)
- Stage IIIB Non-Small Cell Lung Cancer (diagnosed 2022-07-15)
- Ramsay Hunt syndrome (Herpes zoster oticus) (diagnosed 2022-10-15)
- Hypertension, secondary to renal artery stenosis
- Rheumatic heart disease, status post mechanical mitral valve replacement
- Chronic obstructive pulmonary disease (COPD), moderate severity
- Osteoarthritis, bilateral hands
- Follicular lymphoma, indolent, diagnosed 2 years prior
- Non-small cell lung cancer, Stage IIIB, recently diagnosed

She has a history of hypertension.

PAST SURGICAL HISTORY:
She underwent bariatric surgery several years ago for obesity.

MEDICATIONS:
Home Medications:
- Warfarin 5 mg daily (dose adjusted per INR)
- Hydrochlorothiazide 25 mg oral daily
- Amlodipine 10 mg oral daily
- Albuterol HFA inhaler 90 mcg, 2 inhalations every 4-6 hours as needed for shortness of breath
- Acetaminophen 500 mg oral as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 68, hypertension
- Mother: Hypertension, osteoarthritis
- Older brother: Lung cancer
- Paternal uncle: Lymphoma

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 5 years ago. Alcohol: History of heavy alcohol use, in recovery for 18 months. Occupation: Retired registered nurse.

PHYSICAL EXAMINATION:
She appears thin, unkempt, and distracted. She has difficulty maintaining eye contact and struggles to follow moving objects during the exam. Her gait is unstable, and she requires support to walk. She is pleasant but cannot accurately state the date or current location.

LABORATORY DATA:
Her labs, including alcohol and tox screens, are within normal limits.

IMAGING:
MRI of the brain is most likely to reveal atrophy of

## Encounter 6: source 2022-12-15, practice 2026-06-05, ed, Emergency Medicine

Chief complaint: Progressive weakness and vomiting for two days

ENCOUNTER NOTE
Date: 2022-12-15
Provider: Dr. Aisha Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive weakness and vomiting for two days

HISTORY OF PRESENT ILLNESS:
The patient returns one month after Wernicke encephalopathy with progressive weakness and vomiting. She reports using her albuterol inhaler more frequently due to a recent exacerbation of her COPD.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis of hand (diagnosed 2020-01-15)
- Follicular lymphoma (diagnosed 2020-07-15)
- Renal artery stenosis (diagnosed 2021-07-15)
- Stage IIIB Non-Small Cell Lung Cancer (diagnosed 2022-07-15)
- Ramsay Hunt syndrome (Herpes zoster oticus) (diagnosed 2022-10-15)
- Wernicke encephalopathy (diagnosed 2022-11-15)
- Hypertension, secondary to renal artery stenosis
- Rheumatic heart disease, status post mechanical mitral valve replacement
- Chronic obstructive pulmonary disease (COPD), moderate severity
- Osteoarthritis, bilateral hands
- Follicular lymphoma, indolent, diagnosed 2 years prior
- Non-small cell lung cancer, Stage IIIB, recently diagnosed

PAST SURGICAL HISTORY:
- Mechanical mitral valve replacement (10 years prior)
- Right upper lobe bronchoscopy with biopsy (recent)

MEDICATIONS:
Home Medications:
- Warfarin 5 mg daily (dose adjusted per INR)
- Hydrochlorothiazide 25 mg oral daily
- Amlodipine 10 mg oral daily
- Albuterol HFA inhaler 90 mcg, 2 inhalations every 4-6 hours as needed for shortness of breath
- Acetaminophen 500 mg oral as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 68, hypertension
- Mother: Hypertension, osteoarthritis
- Older brother: Lung cancer
- Paternal uncle: Lymphoma

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 5 years ago. Alcohol: History of heavy alcohol use, in recovery for 18 months. Occupation: Retired registered nurse.

VITAL SIGNS:
Her blood pressure is 150/90 mmHg, heart rate is 102 beats per minute, and respiratory rate is 22 breaths per minute with deep, regular breaths.

PHYSICAL EXAMINATION:
On examination, she is alert but appears dehydrated. Her skin turgor is decreased, and mucous membranes are dry.

LABORATORY DATA:
Laboratory results are as follows:

- Serum sodium: 142 mEq/L
- Serum potassium: 3.0 mEq/L
- Serum chloride: 95 mEq/L
- Serum bicarbonate: 35 mEq/L
- Blood pH: 7.55
- Urine chloride: 10 mEq/L

## Encounter 7: source 2023-01-15, practice 2026-06-20, icu, Emergency Medicine

Chief complaint: Acute onset of dyspnea and palpitations

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Robert Liu, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Acute onset of dyspnea and palpitations

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis of hand (diagnosed 2020-01-15)
- Follicular lymphoma (diagnosed 2020-07-15)
- Renal artery stenosis (diagnosed 2021-07-15)
- Stage IIIB Non-Small Cell Lung Cancer (diagnosed 2022-07-15)
- Ramsay Hunt syndrome (Herpes zoster oticus) (diagnosed 2022-10-15)
- Wernicke encephalopathy (diagnosed 2022-11-15)
- Hypochloremic metabolic alkalosis secondary to vomiting with hypovolemia (diagnosed 2022-12-15)
- Hypertension, secondary to renal artery stenosis
- Rheumatic heart disease, status post mechanical mitral valve replacement
- Chronic obstructive pulmonary disease (COPD), moderate severity
- Osteoarthritis, bilateral hands
- Follicular lymphoma, indolent, diagnosed 2 years prior
- Non-small cell lung cancer, Stage IIIB, recently diagnosed

She has a history of rheumatic heart disease.

PAST SURGICAL HISTORY:
She underwent mitral valve replacement with a mechanical valve 5 years ago.

MEDICATIONS:
She reports adherence to her anticoagulation therapy with warfarin.

Home Medications:
- Hydrochlorothiazide 25 mg oral daily
- Amlodipine 10 mg oral daily
- Albuterol HFA inhaler 90 mcg, 2 inhalations every 4-6 hours as needed for shortness of breath
- Acetaminophen 500 mg oral as needed for pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 68, hypertension
- Mother: Hypertension, osteoarthritis
- Older brother: Lung cancer
- Paternal uncle: Lymphoma

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 5 years ago. Alcohol: History of heavy alcohol use, in recovery for 18 months. Occupation: Retired registered nurse.

VITAL SIGNS:
On examination, her blood pressure is 145/90 mmHg, heart rate is 112 beats per minute, and oxygen saturation is 92% on room air.

PHYSICAL EXAMINATION:
A new holosystolic murmur is heard at the apex, radiating to the axilla. She has bilateral rales halfway up both lung fields and 2+ pitting edema in her lower extremities.

LABORATORY DATA:
Laboratory values are as follows:

- Hemoglobin: 13.2 g/dL
- White blood cell count: 7.8 x 10^3/uL
- Platelet count: 250 x 10^3/uL
- International normalized ratio (INR): 1.8
- Brain natriuretic peptide (BNP): 580 pg/mL

IMAGING:
An urgent transthoracic echocardiogram reveals a hyperechoic mass on the mitral valve with restricted leaflet motion.

OTHER STUDIES:
An electrocardiogram shows atrial fibrillation with rapid ventricular response.
