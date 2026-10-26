# synthetic_hospital patient 1974

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 56 at first encounter, sex M, Non-Hispanic White, Private PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-05-08 (Fri) | outpatient | Gastroenterology |
| 1 | 2020-05-15 | 2020-09-10 (Thu) | outpatient | Cardiology |
| 2 | 2020-11-15 | 2021-03-01 (Mon) | outpatient | Cardiology |
| 3 | 2021-07-15 | 2021-10-25 (Mon) | outpatient | Rheumatology |
| 4 | 2022-07-15 | 2022-11-02 (Wed) | ed | Emergency Medicine |
| 5 | 2023-07-15 | 2023-11-01 (Wed) | ed | Emergency Medicine |
| 6 | 2024-07-15 | 2024-10-31 (Thu) | ed | Emergency Medicine |
| 7 | 2025-07-15 | 2025-11-05 (Wed) | icu | Gastroenterology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private PPO",
 "occupation": "Office administrator",
 "alcohol_use": "Social (2-3 drinks on weekends)",
 "family_history": [
  "Father: Myocardial infarction at age 62",
  "Mother: Type 2 diabetes mellitus",
  "Brother: Hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1000 mg PO BID",
   "name": "Metformin"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "50 mg PO daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "400 mg PO PRN for joint pain",
   "name": "Ibuprofen"
  }
 ],
 "surgical_history": [
  "Diagnostic cardiac catheterization"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Type 2 Diabetes Mellitus",
  "Coronary Artery Disease",
  "Hyperlipidemia",
  "Whipple Disease",
  "Crystal-induced arthropathy (Gout)"
 ],
 "age_at_first_encounter": 56
}
```

Primary diagnoses: ["Acute liver failure", "Stable Angina Pectoris (Medical Management)", "Hemodynamically unstable wide-complex tachycardia", "Stable Angina Pectoris due to Coronary Artery Disease", "Acute right-sided subdural hematoma with subfalcine herniation", "Whipple disease", "Crystal-induced arthropathy (Gout or Pseudogout)", "Acute mitral regurgitation"]
Comorbidities: ["Cardiogenic Shock", "Acute myocardial infarction", "Hyperlipidemia", "Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2020-05-08, outpatient, Gastroenterology

Chief complaint: Intermittent abdominal pain, chronic diarrhea, and 15-pound weight loss over 6 months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent abdominal pain, chronic diarrhea, and 15-pound weight loss over 6 months

HISTORY OF PRESENT ILLNESS:
He presents to the clinic with a 6-month history of intermittent abdominal pain, chronic diarrhea, and a weight loss of 15 pounds. He mentions that the diarrhea is bulky, greasy, and foul-smelling. He also complains of joint pain that migrates between his knees, wrists, and shoulders, without any noticeable swelling or redness.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Type 2 Diabetes Mellitus
- Coronary Artery Disease
- Hyperlipidemia
- Whipple Disease
- Crystal-induced arthropathy (Gout)

He has a past medical history of hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Diagnostic cardiac catheterization

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Ibuprofen 400 mg PO PRN for joint pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 diabetes mellitus
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (2-3 drinks on weekends). Occupation: Office administrator.

VITAL SIGNS:
Afebrile, with a blood pressure of 130/85 mmHg

PHYSICAL EXAMINATION:
Generalized non-tender lymphadenopathy.

LABORATORY DATA:
Hemoglobin: 12.1 g/dL (Normal: 13.8-17.2 g/dL)
- White blood cell count: 7,000 /µL (Normal: 4,500-11,000 /µL)
- Platelet count: 250,000 /µL (Normal: 150,000-450,000 /µL)
- Fasting glucose: 160 mg/dL (Normal: 70-99 mg/dL)
- Serum albumin: 3.2 g/dL (Normal: 3.5-5.0 g/dL)
- Erythrocyte sedimentation rate (ESR): 50 mm/hr (Normal: 0-22 mm/hr)
- Anti-transglutaminase antibodies: Negative

PATHOLOGY:
Biopsy of the small intestine shows periodic acid-Schiff (PAS) positive macrophages in the lamina propria.

OTHER STUDIES:
An upper endoscopy is performed,

## Encounter 1: source 2020-05-15, practice 2020-09-10, outpatient, Cardiology

Chief complaint: Progressive exertional chest pain over the past 3 months

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. Michael Rodriguez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive exertional chest pain over the past 3 months

HISTORY OF PRESENT ILLNESS:
The patient with coronary artery disease and Whipple disease presents four months after his Gastroenterology encounter, reporting a 3-month history of progressive exertional chest pain. He describes the pain as a pressure-like sensation in the center of his chest that occurs with walking up hills and resolves with rest.

PAST MEDICAL HISTORY:
Active Problem List:
- Whipple disease (diagnosed 2020-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Coronary Artery Disease
- Hyperlipidemia
- Crystal-induced arthropathy (Gout)

PAST SURGICAL HISTORY:
- Diagnostic cardiac catheterization

MEDICATIONS:
His current medications include metformin, lisinopril, and atorvastatin.

Home Medications:
- Metoprolol succinate 50 mg PO daily
- Ibuprofen 400 mg PO PRN for joint pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 diabetes mellitus
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (2-3 drinks on weekends). Occupation: Office administrator.

VITAL SIGNS:
His blood pressure is 138/86 mmHg, and his pulse is 78 beats per minute.

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

IMAGING:
He is referred for a coronary angiography, which reveals a 70% stenosis in the left anterior descending artery.

OTHER STUDIES:
An exercise stress test shows ST-segment depression in the anterior leads at a high level of exertion.

ASSESSMENT AND PLAN:
The patient is hesitant about undergoing revascularization and asks about the possibility of managing his condition with medications alone. You recall a recent study that compared medical therapy alone to percutaneous coronary intervention (PCI) plus medical therapy in patients with stable coronary artery disease. The study concluded that there was no significant difference in the primary outcome of death, myocardial infarction, or hospitalization for unstable angina. However, you are concerned about the possibility of a type II error due to the small sample size of the study.

## Encounter 2: source 2020-11-15, practice 2021-03-01, outpatient, Cardiology

Chief complaint: Follow-up appointment for persistent angina despite medical therapy

ENCOUNTER NOTE
Date: 2020-11-15
Provider: Dr. Michael Rodriguez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up appointment for persistent angina despite medical therapy

HISTORY OF PRESENT ILLNESS:
Returning for follow-up of chest pain assessed in May 2020, the patient with coronary artery disease and Whipple disease reports compliance with his medication regimen, which includes metformin, lisinopril, and atorvastatin. However, he has been experiencing persistent angina despite maximal tolerated doses of metoprolol. His current symptoms limit his ability to perform moderate physical activity.

PAST MEDICAL HISTORY:
Active Problem List:
- Whipple disease (diagnosed 2020-01-15)
- Stable Angina Pectoris due to Coronary Artery Disease (diagnosed 2020-05-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Coronary Artery Disease
- Hyperlipidemia
- Crystal-induced arthropathy (Gout)

PAST SURGICAL HISTORY:
- Diagnostic cardiac catheterization

MEDICATIONS:
He takes metformin, lisinopril, and atorvastatin; metoprolol.

Home Medications:
- Metoprolol succinate 50 mg PO daily
- Ibuprofen 400 mg PO PRN for joint pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 diabetes mellitus
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (2-3 drinks on weekends). Occupation: Office administrator.

VITAL SIGNS:
Blood pressure is 132/84 mmHg, heart rate is 78 bpm

PHYSICAL EXAMINATION:
No signs of heart failure

OTHER STUDIES:
An electrocardiogram shows no new changes since his MI

ASSESSMENT AND PLAN:
The physician believes that ranolazine is appropriate for this patient due to his refractory angina and lack of other contraindications

His cardiologist recommends adding ranolazine to his regimen, but his insurance plan has denied coverage for this medication

## Encounter 3: source 2021-07-15, practice 2021-10-25, outpatient, Rheumatology

Chief complaint: Severe pain and swelling over right knee for 2 days

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Aisha Johnson, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Severe pain and swelling over right knee for 2 days

HISTORY OF PRESENT ILLNESS:
The patient, currently managed for Whipple disease and coronary artery disease, returns for evaluation of his known crystal-induced arthropathy. He reports that the pain worsens with movement and is associated with redness and warmth over the affected area. He denies any recent trauma or insect bites. He has a history of similar, less severe episodes over the past year, which resolved with NSAIDs.

PAST MEDICAL HISTORY:
Active Problem List:
- Whipple disease (diagnosed 2020-01-15)
- Stable Angina Pectoris due to Coronary Artery Disease (diagnosed 2020-05-15)
- Stable Angina Pectoris (Medical Management) (diagnosed 2020-11-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Coronary Artery Disease
- Hyperlipidemia
- Crystal-induced arthropathy (Gout)

His medical history is significant for hypertension, hyperlipidemia, and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Diagnostic cardiac catheterization

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Ibuprofen 400 mg PO PRN for joint pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 diabetes mellitus
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (2-3 drinks on weekends). Occupation: Office administrator.

PHYSICAL EXAMINATION:
On physical examination, there is a swollen, erythematous area over the prepatellar region of the right knee. The area is tender to palpation and there is limited range of motion due to pain. Aspiration of the bursa yields a yellowish fluid with white flecks.

LABORATORY DATA:
Laboratory values are as follows:

- Serum uric acid: 9.2 mg/dL (normal: 3.4-7.0 mg/dL)
- Erythrocyte sedimentation rate (ESR): 50 mm/hr (normal: 0-22 mm/hr)
- C-reactive protein (CRP): 25 mg/L (normal: <10 mg/L)
- White blood cell count (WBC) in bursal fluid: 18,000 cells/µL with 90% polymorphonuclear cells

IMAGING:
Radiographs of the right knee show punched-out erosions with overhanging edges of cortical bone.

## Encounter 4: source 2022-07-15, practice 2022-11-02, ed, Emergency Medicine

Chief complaint: Sudden onset palpitations, chest discomfort, dizziness, and shortness of breath

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset palpitations, chest discomfort, dizziness, and shortness of breath

HISTORY OF PRESENT ILLNESS:
This patient with coronary artery disease (with prior cardiology follow-up for angina) and Whipple disease describes the palpitations as sudden in onset, associated with dizziness and shortness of breath. His chest discomfort is substernal, pressure-like, and does not radiate. He denies any previous cardiac history.

PAST MEDICAL HISTORY:
Active Problem List:
- Whipple disease (diagnosed 2020-01-15)
- Stable Angina Pectoris due to Coronary Artery Disease (diagnosed 2020-05-15)
- Stable Angina Pectoris (Medical Management) (diagnosed 2020-11-15)
- Crystal-induced arthropathy (Gout or Pseudogout) (diagnosed 2021-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Coronary Artery Disease
- Hyperlipidemia
- Crystal-induced arthropathy (Gout)

PAST SURGICAL HISTORY:
- Diagnostic cardiac catheterization

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Ibuprofen 400 mg PO PRN for joint pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 diabetes mellitus
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (2-3 drinks on weekends). Occupation: Office administrator.

VITAL SIGNS:
On examination, his blood pressure is 85/50 mmHg, heart rate is 180 beats per minute, respiratory rate is 22 breaths per minute, and oxygen saturation is 92% on room air

PHYSICAL EXAMINATION:
His jugular venous pressure is elevated, and lung auscultation reveals bibasilar crackles.

LABORATORY DATA:
Sodium: 140 mEq/L
Potassium: 4.2 mEq/L
Creatinine: 1.1 mg/dL
Troponin I: Elevated

OTHER STUDIES:
The ECG shows a wide complex tachycardia without discernible P waves.

PLAN:
Intravenous access and oxygen supplementation have been initiated.

## Encounter 5: source 2023-07-15, practice 2023-11-01, ed, Emergency Medicine

Chief complaint: Sudden-onset shortness of breath and chest discomfort while walking

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Emily Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden-onset shortness of breath and chest discomfort while walking

HISTORY OF PRESENT ILLNESS:
The patient is a 59-year-old male with coronary artery disease and 2022 wide-complex tachycardia presenting acutely. He reports his breathing became difficult while walking to the grocery store and has worsened rapidly over the past hour. He now feels lightheaded and cannot lie flat without gasping for air.

PAST MEDICAL HISTORY:
Active Problem List:
- Whipple disease (diagnosed 2020-01-15)
- Stable Angina Pectoris due to Coronary Artery Disease (diagnosed 2020-05-15)
- Stable Angina Pectoris (Medical Management) (diagnosed 2020-11-15)
- Crystal-induced arthropathy (Gout or Pseudogout) (diagnosed 2021-07-15)
- Hemodynamically unstable wide-complex tachycardia (diagnosed 2022-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Coronary Artery Disease
- Hyperlipidemia
- Crystal-induced arthropathy (Gout)

He has a history of hypertension and coronary artery disease but no known valvular disease.

PAST SURGICAL HISTORY:
- Diagnostic cardiac catheterization

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Ibuprofen 400 mg PO PRN for joint pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 diabetes mellitus
- Brother: Hypertension

SOCIAL HISTORY:
He does not smoke and drinks alcohol socially.

VITAL SIGNS:
His blood pressure is 98/60 mm Hg, heart rate is 112/min, and respiratory rate is 28/min.

PHYSICAL EXAMINATION:
On examination, he is diaphoretic and appears anxious. Jugular venous distension is noted to the angle of the mandible while upright. Cardiac auscultation reveals a new, soft, blowing holosystolic murmur best heard at the apex and radiating to the axilla. Bibasilar crackles are present on lung auscultation. There is no peripheral edema.

IMAGING:
Chest x-ray reveals bilateral pulmonary infiltrates with an enlarged left atrium.

OTHER STUDIES:
ECG shows sinus tachycardia with no ST-segment elevations or depressions.

PLAN:
He is placed on supplemental oxygen, and furosemide is administered. Transthoracic echocardiogram is pending. A Swan-Ganz catheter is inserted for hemodynamic monitoring.

## Encounter 6: source 2024-07-15, practice 2024-10-31, ed, Emergency Medicine

Chief complaint: Sudden onset severe headache and left-sided weakness

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. James Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset severe headache and left-sided weakness

HISTORY OF PRESENT ILLNESS:
This 60-year-old male with CAD, HTN, and prior ED visits for cardiac events presents with headache. He reports the headache is severe, throbbing, and localized to the right side. His wife notes he has been increasingly confused over the past hour.

PAST MEDICAL HISTORY:
Active Problem List:
- Whipple disease (diagnosed 2020-01-15)
- Stable Angina Pectoris due to Coronary Artery Disease (diagnosed 2020-05-15)
- Stable Angina Pectoris (Medical Management) (diagnosed 2020-11-15)
- Crystal-induced arthropathy (Gout or Pseudogout) (diagnosed 2021-07-15)
- Hemodynamically unstable wide-complex tachycardia (diagnosed 2022-07-15)
- Acute mitral regurgitation (diagnosed 2023-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Coronary Artery Disease
- Hyperlipidemia
- Crystal-induced arthropathy (Gout)

PAST SURGICAL HISTORY:
- Diagnostic cardiac catheterization

MEDICATIONS:
His medications include metformin, lisinopril, and atorvastatin.

Home Medications:
- Metoprolol succinate 50 mg PO daily
- Ibuprofen 400 mg PO PRN for joint pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 diabetes mellitus
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (2-3 drinks on weekends). Occupation: Office administrator.

VITAL SIGNS:
His blood pressure is 190/110 mmHg, heart rate is 88 beats per minute, and he is afebrile

PHYSICAL EXAMINATION:
Neurological examination reveals decreased strength in the left lower extremity, with preserved strength in the left upper extremity. There is no facial asymmetry, and his pupils are equal and reactive to light.

IMAGING:
A non-contrast head CT scan shows a right-sided subdural hematoma with midline shift.

## Encounter 7: source 2025-07-15, practice 2025-11-05, icu, Gastroenterology

Chief complaint: Progressive weakness and confusion over 2 days following syncope

ENCOUNTER NOTE
Date: 2025-07-15
Provider: Dr. Maria Gonzalez, MD
Department: Gastroenterology
Visit Type: icu

CHIEF COMPLAINT:
Progressive weakness and confusion over 2 days following syncope

HISTORY OF PRESENT ILLNESS:
This 61-year-old male with Whipple disease and CAD presents with progressive weakness and confusion. His wife reports that he had an episode of severe chest pain and "passed out" at home 3 days ago but refused to seek medical attention at that time.

PAST MEDICAL HISTORY:
Active Problem List:
- Whipple disease (diagnosed 2020-01-15)
- Stable Angina Pectoris due to Coronary Artery Disease (diagnosed 2020-05-15)
- Stable Angina Pectoris (Medical Management) (diagnosed 2020-11-15)
- Crystal-induced arthropathy (Gout or Pseudogout) (diagnosed 2021-07-15)
- Hemodynamically unstable wide-complex tachycardia (diagnosed 2022-07-15)
- Acute mitral regurgitation (diagnosed 2023-07-15)
- Acute right-sided subdural hematoma with subfalcine herniation (diagnosed 2024-07-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Coronary Artery Disease
- Hyperlipidemia
- Crystal-induced arthropathy (Gout)

PAST SURGICAL HISTORY:
- Diagnostic cardiac catheterization

MEDICATIONS:
He takes metformin and lisinopril.

Home Medications:
- Atorvastatin 40 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Ibuprofen 400 mg PO PRN for joint pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 62
- Mother: Type 2 diabetes mellitus
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (2-3 drinks on weekends). Occupation: Office administrator.

VITAL SIGNS:
Afebrile, blood pressure is 90/60 mmHg, heart rate is 110 beats per minute

PHYSICAL EXAMINATION:
Disoriented to time and place. His skin is jaundiced, and his abdomen is soft with mild right upper quadrant tenderness without rebound or guarding.

LABORATORY DATA:
- Hemoglobin: 13.2 g/dL
- White blood cell count: 11,000/mm^3
- Platelets: 250,000/mm^3
- Serum creatinine: 1.9 mg/dL (elevated from baseline of 1.2 mg/dL)
- Aspartate aminotransferase (AST): 2,500 U/L
- Alanine aminotransferase (ALT): 2,200 U/L
- Total bilirubin: 3.5 mg/dL
- Alkaline phosphatase: 120 U/L

OTHER STUDIES:
An electrocardiogram shows nonspecific ST and T wave changes.
