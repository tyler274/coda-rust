# synthetic_hospital patient 2252

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 63 at first encounter, sex M, Non-Hispanic White, Blue Cross Blue Shield (Commercial PPO).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-10-28 (Thu) | outpatient | Internal Medicine |
| 1 | 2021-01-15 | 2022-11-04 (Fri) | ed | Emergency Medicine |
| 2 | 2023-01-15 | 2024-10-24 (Thu) | outpatient | Oncology |
| 3 | 2023-07-15 | 2025-04-28 (Mon) | outpatient | Internal Medicine |
| 4 | 2024-01-15 | 2025-11-06 (Thu) | outpatient | Cardiology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield (Commercial PPO)",
 "occupation": "Retired construction foreman",
 "alcohol_use": "Social, 2-3 drinks per week",
 "family_history": [
  "Father: Myocardial infarction at age 68",
  "Mother: Ischemic stroke at age 72",
  "Paternal uncle: Thoracic aortic aneurysm"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 40 pack-years, quit 18 months ago",
 "home_medications": [
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "1 tablet PO daily",
   "name": "Multivitamin"
  }
 ],
 "surgical_history": [
  "Ascending aortic graft replacement",
  "Pericardial window drainage",
  "Port-a-cath placement for chemotherapy"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Hyperlipidemia",
  "Stage IIIB Non-Small Cell Lung Cancer (adenocarcinoma)",
  "Status post Type A aortic dissection with cardiac tamponade",
  "First-degree atrioventricular block"
 ],
 "age_at_first_encounter": 63
}
```

Primary diagnoses: ["Mobitz type I atrioventricular block (Wenckebach phenomenon)", "Cardiac tamponade", "Hyperlipidemia", "Infective Endocarditis (Aortic Valve) with Complications"]
Comorbidities: ["Hyperlipidemia", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2021-10-28, outpatient, Internal Medicine

Chief complaint: Routine follow-up for chronic medical conditions and medication optimization

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for chronic medical conditions and medication optimization

HISTORY OF PRESENT ILLNESS:
He has been adherent to his medication regimen, which includes lisinopril and atorvastatin. He reports no new symptoms and feels generally well. The patient inquires about the benefits of adding a new medication to further lower his risk of myocardial infarction (MI).

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Hyperlipidemia
- Stage IIIB Non-Small Cell Lung Cancer (adenocarcinoma)
- Status post Type A aortic dissection with cardiac tamponade
- First-degree atrioventricular block

PAST SURGICAL HISTORY:
- Ascending aortic graft replacement
- Pericardial window drainage
- Port-a-cath placement for chemotherapy

MEDICATIONS:
He takes lisinopril and atorvastatin.

Home Medications:
- Multivitamin 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Ischemic stroke at age 72
- Paternal uncle: Thoracic aortic aneurysm

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 18 months ago. Alcohol: Social, 2-3 drinks per week. Occupation: Retired construction foreman.

VITAL SIGNS:
His blood pressure today is 132/84 mmHg, and his heart rate is 78 bpm.

PHYSICAL EXAMINATION:
His physical examination is unremarkable.

LABORATORY DATA:
His most recent lipid panel shows an LDL of 98 mg/dL, HDL of 45 mg/dL, and triglycerides of 150 mg/dL.

ASSESSMENT AND PLAN:
A recent large randomized controlled trial showed that adding ezetimibe to statin therapy in patients like him further reduced the risk of MI by 2% over a 5-year period.

## Encounter 1: source 2021-01-15, practice 2022-11-04, ed, Emergency Medicine

Chief complaint: Sudden onset of severe, sharp, tearing chest pain radiating to the back between the scapulae

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of severe, sharp, tearing chest pain radiating to the back between the scapulae

HISTORY OF PRESENT ILLNESS:
This patient with prior Type A dissection presents with pain that began 2 hours ago. The pain is described as tearing in nature and radiates to the back between the scapulae.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperlipidemia (diagnosed 2020-01-15)
- Hypertension
- Stage IIIB Non-Small Cell Lung Cancer (adenocarcinoma)
- Status post Type A aortic dissection with cardiac tamponade
- First-degree atrioventricular block

PAST SURGICAL HISTORY:
- Ascending aortic graft replacement
- Pericardial window drainage
- Port-a-cath placement for chemotherapy

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Multivitamin 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Ischemic stroke at age 72
- Paternal uncle: Thoracic aortic aneurysm

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 18 months ago. Alcohol: Social, 2-3 drinks per week. Occupation: Retired construction foreman.

VITAL SIGNS:
His blood pressure is 90/60 mmHg, heart rate is 118 bpm, respiratory rate is 22 breaths per minute, and oxygen saturation is 94% on room air.

PHYSICAL EXAMINATION:
The patient appears diaphoretic and anxious.
Physical examination reveals distended neck veins and muffled heart sounds. The patient's blood pressure drops to 85/55 mmHg upon inspiration.

IMAGING:
Chest X-ray demonstrates an enlarged cardiac silhouette.

OTHER STUDIES:
An electrocardiogram shows low voltage QRS complexes and electrical alternans.

## Encounter 2: source 2023-01-15, practice 2024-10-24, outpatient, Oncology

Chief complaint: New onset shortness of breath and dry cough following first cycle of chemotherapy

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Priya Patel, MD
Department: Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
New onset shortness of breath and dry cough following first cycle of chemotherapy

HISTORY OF PRESENT ILLNESS:
The patient is a 66-year-old male with hypertension, hyperlipidemia, first-degree atrioventricular block, and status post Type A aortic dissection with cardiac tamponade (presenting in 2021 with sudden severe tearing chest pain radiating to the back between the scapulae), who returns to the oncology clinic for follow-up evaluation. He was recently diagnosed with stage IIIB non-small cell lung cancer (NSCLC) and completed his first cycle of chemotherapy three weeks ago. He reports a new onset of shortness of breath and a dry cough. The patient is interested in the prognosis and treatment options for his condition. He inquires about a recent clinical trial he read about that tested a new targeted therapy for NSCLC. The trial reported a hazard ratio of 0.75 (95% CI 0.59-0.96) for mortality with the new therapy compared to standard chemotherapy.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperlipidemia (diagnosed 2020-01-15)
- Cardiac tamponade (diagnosed 2021-01-15)
- Hypertension
- Stage IIIB Non-Small Cell Lung Cancer (adenocarcinoma)
- Status post Type A aortic dissection with cardiac tamponade
- First-degree atrioventricular block

PAST SURGICAL HISTORY:
- Ascending aortic graft replacement
- Pericardial window drainage
- Port-a-cath placement for chemotherapy

MEDICATIONS:
Lisinopril, atorvastatin, and a multivitamin

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Ischemic stroke at age 72
- Paternal uncle: Thoracic aortic aneurysm

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 18 months ago. Alcohol: Social, 2-3 drinks per week. Occupation: Retired construction foreman.

REVIEW OF SYSTEMS:
He denies chest pain, fever, or weight loss.

VITAL SIGNS:
Blood pressure is 138/86 mmHg, heart rate is 92 beats per minute, respiratory rate is 18 breaths per minute, and oxygen saturation is 94% on room air

PHYSICAL EXAMINATION:
Lung auscultation reveals decreased breath sounds in the right lower lobe without wheezes or crackles. The remainder of the examination is unremarkable.

IMAGING:
A chest X-ray shows a right lower lobe opacity consistent with a pleural effusion.

OTHER STUDIES:
Thoracentesis is performed, and the pleural fluid analysis is negative for malignancy.

## Encounter 3: source 2023-07-15, practice 2025-04-28, outpatient, Internal Medicine

Chief complaint: Three weeks of low-grade fevers, fatigue, unintentional weight loss, and dull lower abdominal discomfort

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Robert Kim, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Three weeks of low-grade fevers, fatigue, unintentional weight loss, and dull lower abdominal discomfort

HISTORY OF PRESENT ILLNESS:
Patient with Stage IIIB lung cancer and Type A aortic dissection also reports dull lower abdominal discomfort and intermittent constipation over the past few months.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperlipidemia (diagnosed 2020-01-15)
- Cardiac tamponade (diagnosed 2021-01-15)
- Hypertension
- Stage IIIB Non-Small Cell Lung Cancer (adenocarcinoma)
- Status post Type A aortic dissection with cardiac tamponade
- First-degree atrioventricular block

He has a history of hypertension and hyperlipidemia and is not up to date on his age-appropriate cancer screenings.

PAST SURGICAL HISTORY:
- Ascending aortic graft replacement
- Pericardial window drainage
- Port-a-cath placement for chemotherapy

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Multivitamin 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Ischemic stroke at age 72
- Paternal uncle: Thoracic aortic aneurysm

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 18 months ago. Alcohol: Social, 2-3 drinks per week. Occupation: Retired construction foreman.

PHYSICAL EXAMINATION:
On physical exam, he has a new holosystolic murmur best heard at the apex. There are no peripheral stigmata of endocarditis.

LABORATORY DATA:
Blood cultures are drawn and grow Gram-positive cocci in chains after 48 hours.

IMAGING:
Transesophageal echocardiography reveals a mobile vegetation on the mitral valve.

OTHER STUDIES:
Given the identity of the cultured organism, colonoscopy is performed and reveals a partially obstructing mass in the descending colon.

PLAN:
The patient is started on IV antibiotics.

## Encounter 4: source 2024-01-15, practice 2025-11-06, outpatient, Cardiology

Chief complaint: Intermittent dizziness and palpitations for the past two weeks

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Elena Vasquez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent dizziness and palpitations for the past two weeks

HISTORY OF PRESENT ILLNESS:
The patient is a 67-year-old male with a history of Type A aortic dissection complicated by cardiac tamponade and recent infective endocarditis involving the aortic valve, returning for evaluation of intermittent dizziness and palpitations over the past two weeks. He denies chest pain, dyspnea, or syncope.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperlipidemia (diagnosed 2020-01-15)
- Cardiac tamponade (diagnosed 2021-01-15)
- Infective Endocarditis (Aortic Valve) with Complications (diagnosed 2023-07-15)
- Hypertension
- Stage IIIB Non-Small Cell Lung Cancer (adenocarcinoma)
- Status post Type A aortic dissection with cardiac tamponade
- First-degree atrioventricular block

PAST SURGICAL HISTORY:
- Ascending aortic graft replacement
- Pericardial window drainage
- Port-a-cath placement for chemotherapy

MEDICATIONS:
He takes lisinopril and atorvastatin.

Home Medications:
- Multivitamin 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Ischemic stroke at age 72
- Paternal uncle: Thoracic aortic aneurysm

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 18 months ago. Alcohol: Social, 2-3 drinks per week. Occupation: Retired construction foreman.

VITAL SIGNS:
On examination, his blood pressure is 130/80 mmHg, heart rate is irregular at 68 bpm, and respiratory rate is 16 breaths per minute

PHYSICAL EXAMINATION:
Cardiovascular examination reveals an irregularly irregular rhythm with no murmurs, rubs, or gallops.

OTHER STUDIES:
An electrocardiogram (ECG) is performed and shows varying PR intervals with a pattern of progressive PR interval lengthening until a P wave is not followed by a QRS complex. Subsequent beats have a shorter PR interval that again begins to lengthen.
