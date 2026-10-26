# synthetic_hospital patient 2044

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex M, Asian, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2018-11-28 (Wed) | outpatient | Otolaryngology |
| 1 | 2021-01-15 | 2019-11-29 (Fri) | outpatient | Hematology |
| 2 | 2025-01-15 | 2023-11-27 (Mon) | ed | Emergency Medicine |
| 3 | 2026-01-15 | 2024-12-07 (Sat) | ed | Emergency Medicine |
| 4 | 2027-01-15 | 2025-12-06 (Sat) | icu | Cardiology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Office administrator",
 "alcohol_use": "No alcohol use",
 "family_history": [
  "Father with hypertension",
  "Paternal uncle with nasopharyngeal carcinoma",
  "Mother with type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Asian",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "10 mg orally once daily",
   "name": "lisinopril"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Hypertension"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Nasopharyngeal Carcinoma", "Chronic myeloid leukemia, BCR/ABL-positive", "Infective Endocarditis (Aortic Valve) with Complications", "Purulent pericarditis"]
Comorbidities: ["Anemia", "Intravenous Drug Use (Substance Use Disorder)"]

## Encounter 0: source 2020-01-15, practice 2018-11-28, outpatient, Otolaryngology

Chief complaint: Progressive fatigue and a painless lump on his neck

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Otolaryngology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue and a painless lump on his neck

HISTORY OF PRESENT ILLNESS:
He presents to the clinic with a 2-week history of progressive fatigue and a painless lump on his neck. He denies fever, night sweats, or weight loss.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension

His past medical history is significant for hypertension, controlled with lisinopril.

MEDICATIONS:
Lisinopril

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with hypertension
- Paternal uncle with nasopharyngeal carcinoma
- Mother with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Office administrator.

PHYSICAL EXAMINATION:
On physical examination, a 3 cm non-tender, firm lymph node is palpable in the left supraclavicular area. The remainder of the examination is unremarkable.

LABORATORY DATA:
Laboratory tests reveal:

- Hemoglobin: 13.5 g/dL (normal: 13.8-17.2 g/dL)
- White blood cell count: 11,000/uL (normal: 4,500-11,000/uL)
- Platelets: 250,000/uL (normal: 150,000-450,000/uL)
- Lactate dehydrogenase (LDH): Elevated

IMAGING:
A chest X-ray shows no mediastinal widening or lung pathology.

PLAN:
Given the patient's demographic and clinical presentation, the physician decides to perform a test for a specific cancer marker known to have a high prevalence in this population.

## Encounter 1: source 2021-01-15, practice 2019-11-29, outpatient, Hematology

Chief complaint: Persistent fatigue and early satiety over the past few months

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Michael Rodriguez, MD
Department: Hematology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent fatigue and early satiety over the past few months

HISTORY OF PRESENT ILLNESS:
The patient returns for evaluation of persistent fatigue and early satiety, one year following prior Otolaryngology assessment for neck mass. He reports no recent infections or weight loss.

PAST MEDICAL HISTORY:
Active Problem List:
- Nasopharyngeal Carcinoma (diagnosed 2020-01-15)
- Hypertension

MEDICATIONS:
The patient does not take any medications,

Home Medications:
- lisinopril 10 mg orally once daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with hypertension
- Paternal uncle with nasopharyngeal carcinoma
- Mother with type 2 diabetes mellitus

SOCIAL HISTORY:
There is no history of toxin exposure.

PHYSICAL EXAMINATION:
On examination, the spleen tip is palpable. There is no lymphadenopathy or hepatomegaly.

LABORATORY DATA:
Peripheral smear shows increased granulocytic precursors without significant dysplasia. Cytogenetic analysis confirms a chromosomal abnormality. Laboratory results:
WBC                                        110,000 /mm³
Hemoglobin                              11.4 g/dL
Platelets                                    570,000 /mm³
Myelocytes                               10%
Metamyelocytes                        8%
Promyelocytes                          5%

## Encounter 2: source 2025-01-15, practice 2023-11-27, ed, Emergency Medicine

Chief complaint: Fever, night sweats, and a 10-pound weight loss; lower back pain

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fever, night sweats, and a 10-pound weight loss; lower back pain

HISTORY OF PRESENT ILLNESS:
This 50-year-old male with hypertension, nasopharyngeal carcinoma (diagnosed 2020), and chronic myeloid leukemia (diagnosed 2021) presents to the emergency department with a 3-week history of fever, night sweats, and a 10-pound weight loss. He reports a new onset of lower back pain that is worse at night and does not improve with over-the-counter pain medications.

PAST MEDICAL HISTORY:
Active Problem List:
- Nasopharyngeal Carcinoma (diagnosed 2020-01-15)
- Chronic myeloid leukemia, BCR/ABL-positive (diagnosed 2021-01-15)
- Hypertension

MEDICATIONS:
Home Medications:
- lisinopril 10 mg orally once daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with hypertension
- Paternal uncle with nasopharyngeal carcinoma
- Mother with type 2 diabetes mellitus

SOCIAL HISTORY:
History of intravenous drug use

VITAL SIGNS:
Temperature of 38.7°C (101.7°F)

PHYSICAL EXAMINATION:
Patient appears ill; A new heart murmur is noted; there is tenderness to palpation over the lumbar spine

LABORATORY DATA:
- White blood cell count: 12,000/mm³ (normal 4,500-11,000/mm³)
- Hemoglobin: 10.2 g/dL (normal 13.8-17.2 g/dL)
- Erythrocyte sedimentation rate (ESR): 85 mm/hr (normal 0-22 mm/hr)
- Blood cultures: Pending

IMAGING:
A transthoracic echocardiogram (TTE) is performed and shows no vegetations.

## Encounter 3: source 2026-01-15, practice 2024-12-07, ed, Emergency Medicine

Chief complaint: Fever, night sweats, and a 10-pound weight loss with new shortness of breath and orthopnea

ENCOUNTER NOTE
Date: 2026-01-15
Provider: Dr. Aisha Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fever, night sweats, and a 10-pound weight loss with new shortness of breath and orthopnea

HISTORY OF PRESENT ILLNESS:
The patient returns to the emergency department one year after his prior presentation with similar symptoms, with histories of nasopharyngeal carcinoma and chronic myeloid leukemia. He reports a new onset of shortness of breath and orthopnea over the past few days.

PAST MEDICAL HISTORY:
Active Problem List:
- Nasopharyngeal Carcinoma (diagnosed 2020-01-15)
- Chronic myeloid leukemia, BCR/ABL-positive (diagnosed 2021-01-15)
- Infective Endocarditis (Aortic Valve) with Complications (diagnosed 2025-01-15)
- Hypertension

MEDICATIONS:
Home Medications:
- lisinopril 10 mg orally once daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with hypertension
- Paternal uncle with nasopharyngeal carcinoma
- Mother with type 2 diabetes mellitus

SOCIAL HISTORY:
With a history of intravenous drug use

VITAL SIGNS:
On examination, his temperature is 38.6°C, blood pressure is 110/70 mmHg, heart rate is 115 bpm, and respiratory rate is 22 breaths per minute.

PHYSICAL EXAMINATION:
A grade III/VI diastolic murmur is heard at the right sternal border. His lungs are clear to auscultation.

LABORATORY DATA:
Laboratory findings are as follows:

- White blood cell count: 14,000 /µL
- Hemoglobin: 10.2 g/dL
- Platelet count: 150,000 /µL
- Blood cultures: Positive for methicillin-sensitive Staphylococcus aureus (MSSA)

IMAGING:
- Echocardiogram: Large vegetation on the aortic valve, moderate aortic regurgitation, and evidence of an aortic root abscess

ASSESSMENT AND PLAN:
Given the patient&#x27;s presentation and test results,

## Encounter 4: source 2027-01-15, practice 2025-12-06, icu, Cardiology

Chief complaint: Sharp, central chest pain with high-grade fever and productive cough

ENCOUNTER NOTE
Date: 2027-01-15
Provider: Dr. David Kim, MD
Department: Cardiology
Visit Type: icu

CHIEF COMPLAINT:
Sharp, central chest pain with high-grade fever and productive cough

HISTORY OF PRESENT ILLNESS:
The patient with nasopharyngeal carcinoma, chronic myeloid leukemia, and recurrent endocarditis presents with a 2-day history of sharp, central chest pain that worsens with inspiration and lying flat. He also reports a high-grade fever, chills, and a productive cough with yellowish sputum.

PAST MEDICAL HISTORY:
Active Problem List:
- Nasopharyngeal Carcinoma (diagnosed 2020-01-15)
- Chronic myeloid leukemia, BCR/ABL-positive (diagnosed 2021-01-15)
- Infective Endocarditis (Aortic Valve) with Complications (diagnosed 2025-01-15)
- Hypertension

MEDICATIONS:
Home Medications:
- lisinopril 10 mg orally once daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with hypertension
- Paternal uncle with nasopharyngeal carcinoma
- Mother with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Office administrator.

VITAL SIGNS:
Temperature is 39.2°C (102.5°F), blood pressure is 100/60 mmHg, heart rate is 120 beats per minute, and respiratory rate is 22 breaths per minute

PHYSICAL EXAMINATION:
A friction rub is heard on cardiac auscultation. Jugular venous pressure is elevated.

LABORATORY DATA:
- White blood cell count: 18,000/mm³ (normal 4,500-11,000)
- Hemoglobin: 13.5 g/dL (normal 13.8-17.2)
- C-reactive protein: 90 mg/L (normal <10)
- Blood cultures: Pending

IMAGING:
An urgent echocardiogram shows a large pericardial effusion with evidence of cardiac tamponade.
