# synthetic_hospital patient 2183

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 64 at first encounter, sex M, Non-Hispanic White, Medicare (disability) and Blue Cross Blue Shield supplemental.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2019-12-11 (Wed) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2020-06-08 (Mon) | outpatient | Internal Medicine |
| 2 | 2021-01-15 | 2020-12-14 (Mon) | outpatient | Cardiology |
| 3 | 2022-01-15 | 2021-12-13 (Mon) | outpatient | Podiatry |
| 4 | 2023-01-15 | 2022-12-02 (Fri) | outpatient | Cardiology |
| 5 | 2024-07-15 | 2024-05-29 (Wed) | follow_up | Cardiothoracic Surgery |
| 6 | 2024-08-15 | 2024-07-11 (Thu) | outpatient | Psychiatry |
| 7 | 2026-01-15 | 2025-12-11 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare (disability) and Blue Cross Blue Shield supplemental",
 "occupation": "Retired construction foreman",
 "alcohol_use": "Former heavy alcohol use (2-3 drinks nightly), currently abstinent for the past 6 months",
 "family_history": [
  "Father: Premature coronary artery disease (myocardial infarction at age 58)",
  "Mother: Type 2 diabetes mellitus",
  "Sister: Hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 40 pack-year history, quit 5 years ago",
 "home_medications": [
  {
   "dose": "81 mg oral daily",
   "name": "Aspirin"
  },
  {
   "dose": "50 mg oral daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "40 mg oral daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "20 mg oral daily",
   "name": "Lisinopril"
  },
  {
   "dose": "1000 mg oral twice daily",
   "name": "Metformin"
  },
  {
   "dose": "24 units subcutaneously at bedtime",
   "name": "Insulin glargine"
  },
  {
   "dose": "500 mg oral twice daily",
   "name": "Valproic acid"
  },
  {
   "dose": "90 mcg inhaled, 2 puffs every 4 hours as needed for shortness of breath",
   "name": "Albuterol HFA"
  }
 ],
 "surgical_history": [
  "Coronary artery bypass grafting (CABG)",
  "Cardiac catheterization with percutaneous coronary intervention (prior)"
 ],
 "chronic_conditions": [
  "Coronary artery disease (myocardial infarctions 5 years and 1 year prior)",
  "Type 2 diabetes mellitus with peripheral neuropathy",
  "Hypertension",
  "Hyperlipidemia",
  "Chronic obstructive pulmonary disease",
  "Heart failure with reduced ejection fraction",
  "Major depressive disorder",
  "History of complex partial seizures (well-controlled)"
 ],
 "age_at_first_encounter": 64
}
```

Primary diagnoses: ["Major depressive disorder", "Hyperlipidemia", "Right-sided pleural effusion", "Lateral plantar nerve", "Stable Angina Pectoris (Medical Management)", "Statin-associated muscle symptoms", "Heart failure with reduced ejection fraction", "Fluoroquinolone-induced Achilles tendinopathy with suspected rupture"]
Comorbidities: ["Old myocardial infarction", "Tobacco use disorder", "Type 2 diabetes mellitus", "Hyperlipidemia", "Essential hypertension", "Coronary Artery Disease (Antiplatelet Therapy)"]

## Encounter 0: source 2020-01-15, practice 2019-12-11, outpatient, Internal Medicine

Chief complaint: Returns for routine follow-up evaluation of chronic hyperlipidemia and dietary adherence

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Returns for routine follow-up evaluation of chronic hyperlipidemia and dietary adherence

HISTORY OF PRESENT ILLNESS:
He has been adherent to dietary recommendations. He occasionally experiences mild upper abdominal fullness

PAST MEDICAL HISTORY:
Active Problem List:
- Coronary artery disease (myocardial infarctions 5 years and 1 year prior)
- Type 2 diabetes mellitus with peripheral neuropathy
- Hypertension
- Hyperlipidemia
- Chronic obstructive pulmonary disease
- Heart failure with reduced ejection fraction
- Major depressive disorder
- History of complex partial seizures (well-controlled)

PAST SURGICAL HISTORY:
- Coronary artery bypass grafting (CABG)
- Cardiac catheterization with percutaneous coronary intervention (prior)

MEDICATIONS:
He is already receiving high-intensity statin therapy along with antihypertensive medications.

Home Medications:
- Aspirin 81 mg oral daily
- Metoprolol succinate 50 mg oral daily
- Atorvastatin 40 mg oral daily
- Lisinopril 20 mg oral daily
- Metformin 1000 mg oral twice daily
- Insulin glargine 24 units subcutaneously at bedtime
- Valproic acid 500 mg oral twice daily
- Albuterol HFA 90 mcg inhaled, 2 puffs every 4 hours as needed for shortness of breath

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Premature coronary artery disease (myocardial infarction at age 58)
- Mother: Type 2 diabetes mellitus
- Sister: Hypertension

SOCIAL HISTORY:
He denies alcohol use

REVIEW OF SYSTEMS:
Jaundice, or recent weight changes

PHYSICAL EXAMINATION:
On examination, his body mass index is 31 kg/m², blood pressure is 136/78 mmHg, and the remainder of the exam is unremarkable

LABORATORY DATA:
His most recent lipid profile demonstrates persistently elevated fasting triglyceride levels, while his low-density lipoprotein (LDL) cholesterol is well controlled

PLAN:
Because of his residual hypertriglyceridemia, his physician prescribes fenofibrate as an adjunct to ongoing therapy

## Encounter 1: source 2020-07-15, practice 2020-06-08, outpatient, Internal Medicine

Chief complaint: Muscle cramps and weakness beginning two weeks after medication adjustment

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Muscle cramps and weakness beginning two weeks after medication adjustment

HISTORY OF PRESENT ILLNESS:
This patient with coronary artery disease returns following his January hyperlipidemia visit. Symptoms started two weeks after initiating atorvastatin 80 mg daily. He reports that these symptoms are interfering with his daily activities.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperlipidemia (diagnosed 2020-01-15)
- Coronary artery disease (myocardial infarctions 5 years and 1 year prior)
- Type 2 diabetes mellitus with peripheral neuropathy
- Hypertension
- Chronic obstructive pulmonary disease
- Heart failure with reduced ejection fraction
- Major depressive disorder
- History of complex partial seizures (well-controlled)

His past medical history is significant for a myocardial infarction one year ago. He also has type 2 diabetes mellitus and hypertension.

PAST SURGICAL HISTORY:
Percutaneous coronary intervention

MEDICATIONS:
Atorvastatin 80 mg daily. His current medications include aspirin, metoprolol, lisinopril, and metformin.

Home Medications:
- Metoprolol succinate 50 mg oral daily
- Insulin glargine 24 units subcutaneously at bedtime
- Valproic acid 500 mg oral twice daily
- Albuterol HFA 90 mcg inhaled, 2 puffs every 4 hours as needed for shortness of breath

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Premature coronary artery disease (myocardial infarction at age 58)
- Mother: Type 2 diabetes mellitus
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-year history, quit 5 years ago. Alcohol: Former heavy alcohol use (2-3 drinks nightly), currently abstinent for the past 6 months. Occupation: Retired construction foreman.

PHYSICAL EXAMINATION:
Physical examination is unremarkable except for tenderness on palpation of the proximal muscles of the lower extremities.

LABORATORY DATA:
Creatine kinase level is mildly elevated. Liver function tests are within normal limits.

## Encounter 2: source 2021-01-15, practice 2020-12-14, outpatient, Cardiology

Chief complaint: Worsening chest tightness occurring when walking uphill or climbing stairs

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Michael Rodriguez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening chest tightness occurring when walking uphill or climbing stairs

HISTORY OF PRESENT ILLNESS:
The patient with CAD and statin intolerance reports pain. The pain is described as a squeezing sensation in the center of his chest and resolves with rest.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperlipidemia (diagnosed 2020-01-15)
- Statin-associated muscle symptoms (diagnosed 2020-07-15)
- Coronary artery disease (myocardial infarctions 5 years and 1 year prior)
- Type 2 diabetes mellitus with peripheral neuropathy
- Hypertension
- Chronic obstructive pulmonary disease
- Heart failure with reduced ejection fraction
- Major depressive disorder
- History of complex partial seizures (well-controlled)

PAST SURGICAL HISTORY:
- Coronary artery bypass grafting (CABG)
- Cardiac catheterization with percutaneous coronary intervention (prior)

MEDICATIONS:
He takes atorvastatin and hydrochlorothiazide.

Home Medications:
- Aspirin 81 mg oral daily
- Metoprolol succinate 50 mg oral daily
- Lisinopril 20 mg oral daily
- Metformin 1000 mg oral twice daily
- Insulin glargine 24 units subcutaneously at bedtime
- Valproic acid 500 mg oral twice daily
- Albuterol HFA 90 mcg inhaled, 2 puffs every 4 hours as needed for shortness of breath

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Premature coronary artery disease (myocardial infarction at age 58)
- Mother: Type 2 diabetes mellitus
- Sister: Hypertension

SOCIAL HISTORY:
25-pack-year smoking history

VITAL SIGNS:
His vital signs are stable

PHYSICAL EXAMINATION:
Physical examination is unremarkable

OTHER STUDIES:
He undergoes stress testing, which reproduces his symptoms and reveals signs of inducible ischemia

PLAN:
To reduce his anginal episodes, his physician initiates a medication that improves oxygen delivery to the myocardium primarily through vascular smooth muscle relaxation. Within minutes of taking the medication sublingually during an episode of chest pain, the patient reports symptom relief. The drug does not affect cardiac contractility or significantly dilate coronary arteries at lower doses. Instead, it works by reducing myocardial oxygen demand by decreasing preload. The patient is advised to sit down before taking the medication to avoid dizziness. On follow-up, the frequency of anginal episodes has declined, and the patient tolerates the treatment well. He is instructed not to take the medication with phosphodiesterase-5 inhibitors. The prescribing physician explains that the agent functions by activating an intracellular signaling cascade that leads to dephosphorylation of myosin light chains in vascular smooth muscle.

## Encounter 3: source 2022-01-15, practice 2021-12-13, outpatient, Podiatry

Chief complaint: Evaluation of a chronic wound on the bottom of his right foot first noticed three weeks ago

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Emily Johnson, MD
Department: Podiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Evaluation of a chronic wound on the bottom of his right foot first noticed three weeks ago

HISTORY OF PRESENT ILLNESS:
The patient is a 66-year-old male with type 2 diabetes mellitus with peripheral neuropathy and coronary artery disease who presents for evaluation of a chronic wound on his right foot. He has established longitudinal care in internal medicine and cardiology, most recently evaluated one year prior for stable angina. He reports first noticing the lesion approximately three weeks ago when his wife pointed out blood on his sock, although he himself had not felt any pain. He reports that for the past few years, he has had a "tingly" or "numb" feeling in both feet, especially at night when lying in bed. Occasionally, this sensation disrupts his sleep. He denies fever, chills, or drainage from the wound. The lesion does not interfere with his ability to walk, but he mentions that over the last few months, he has had difficulty feeling the gas pedal while driving. He has no back pain or radiating leg symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperlipidemia (diagnosed 2020-01-15)
- Statin-associated muscle symptoms (diagnosed 2020-07-15)
- Stable Angina Pectoris (Medical Management) (diagnosed 2021-01-15)
- Coronary artery disease (myocardial infarctions 5 years and 1 year prior)
- Type 2 diabetes mellitus with peripheral neuropathy
- Hypertension
- Chronic obstructive pulmonary disease
- Heart failure with reduced ejection fraction
- Major depressive disorder
- History of complex partial seizures (well-controlled)

His medical history includes type 2 diabetes mellitus for 18 years, hypertension, dyslipidemia, coronary artery disease, and peripheral vascular disease.

PAST SURGICAL HISTORY:
He has never had lower extremity amputations.

MEDICATIONS:
His current medications include metformin, insulin glargine, lisinopril, atorvastatin, and aspirin.

Home Medications:
- Metoprolol succinate 50 mg oral daily
- Valproic acid 500 mg oral twice daily
- Albuterol HFA 90 mcg inhaled, 2 puffs every 4 hours as needed for shortness of breath

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Premature coronary artery disease (myocardial infarction at age 58)
- Mother: Type 2 diabetes mellitus
- Sister: Hypertension

SOCIAL HISTORY:
He has a 30-pack-year smoking history but quit five years ago. On further questioning, the patient notes that he is not consistent with checking his feet and has not seen a podiatrist in years. He wears the same pair of shoes daily and admits they are quite worn.

REVIEW OF SYSTEMS:
On review of systems, he denies visual changes, recent falls, bowel or bladder incontinence, and upper extremity numbness. He has not noticed any weight loss or fatigue.

VITAL SIGNS:
On physical examination, he is afebrile

PHYSICAL EXAMINATION:
There is a 2-cm shallow ulcer with a clean base and surrounding callus on the plantar surface beneath the first metatarsal head of the right foot. There is no erythema, swelling, or fluctuance. A probe-to-bone test is negative. Monofilament testing shows loss of protective sensation over the plantar surface of both feet, including the toes and heels. There is reduced vibration sense at the great toe bilaterally. Capillary refill is delayed, and dorsalis pedis and posterior tibial pulses are diminished on the right.

LABORATORY DATA:
His A1c at the last visit was 8.9%, and

OTHER STUDIES:
The ankle-brachial index is 0.62 on the affected side.

PLAN:
The patient is scheduled for foot imaging and vascular consultation.

## Encounter 4: source 2023-01-15, practice 2022-12-02, outpatient, Cardiology

Chief complaint: Three-month history of progressive exertional dyspnea and bilateral ankle swelling

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Michael Rodriguez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Three-month history of progressive exertional dyspnea and bilateral ankle swelling

HISTORY OF PRESENT ILLNESS:
The patient with coronary artery disease returns to cardiology clinic. He presents with a 3-month history of progressive exertional dyspnea and bilateral ankle swelling.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperlipidemia (diagnosed 2020-01-15)
- Statin-associated muscle symptoms (diagnosed 2020-07-15)
- Stable Angina Pectoris (Medical Management) (diagnosed 2021-01-15)
- Lateral plantar nerve (diagnosed 2022-01-15)
- Coronary artery disease (myocardial infarctions 5 years and 1 year prior)
- Type 2 diabetes mellitus with peripheral neuropathy
- Hypertension
- Chronic obstructive pulmonary disease
- Heart failure with reduced ejection fraction
- Major depressive disorder
- History of complex partial seizures (well-controlled)

He has a past medical history of hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Coronary artery bypass grafting (CABG)
- Cardiac catheterization with percutaneous coronary intervention (prior)

MEDICATIONS:
His medications include metformin, lisinopril, and atorvastatin.

Home Medications:
- Aspirin 81 mg oral daily
- Metoprolol succinate 50 mg oral daily
- Insulin glargine 24 units subcutaneously at bedtime
- Valproic acid 500 mg oral twice daily
- Albuterol HFA 90 mcg inhaled, 2 puffs every 4 hours as needed for shortness of breath

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Premature coronary artery disease (myocardial infarction at age 58)
- Mother: Type 2 diabetes mellitus
- Sister: Hypertension

SOCIAL HISTORY:
30-pack-year smoking history

REVIEW OF SYSTEMS:
He denies chest pain, palpitations, or syncope.

VITAL SIGNS:
On physical examination, his blood pressure is 145/90 mmHg, heart rate is 88 beats per minute, respiratory rate is 18 breaths per minute, and oxygen saturation is 96% on room air.

PHYSICAL EXAMINATION:
Cardiac examination reveals an S3 gallop and jugular venous distension. Lung fields are clear to auscultation. There is 2+ pitting edema in both ankles.

LABORATORY DATA:
Serum sodium: 138 mEq/L
Serum potassium: 4.5 mEq/L
Serum creatinine: 1.2 mg/dL
Fasting blood glucose: 160 mg/dL
B-type natriuretic peptide (BNP): 900 pg/mL

IMAGING:
Chest X-ray: Cardiomegaly with no pulmonary infiltrates

## Encounter 5: source 2024-07-15, practice 2024-05-29, follow_up, Cardiothoracic Surgery

Chief complaint: Follow-up visit three days after hospital discharge following coronary artery bypass grafting

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. David Park, MD
Department: Cardiothoracic Surgery
Visit Type: follow_up

CHIEF COMPLAINT:
Follow-up visit three days after hospital discharge following coronary artery bypass grafting

HISTORY OF PRESENT ILLNESS:
Patient returns three days post-discharge after coronary artery bypass grafting with history of MIs, heart failure, and diabetic neuropathy. He reports no chest pain but mentions mild shortness of breath on exertion not experienced before surgery. He denies fever, chills, or cough.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperlipidemia (diagnosed 2020-01-15)
- Statin-associated muscle symptoms (diagnosed 2020-07-15)
- Stable Angina Pectoris (Medical Management) (diagnosed 2021-01-15)
- Lateral plantar nerve (diagnosed 2022-01-15)
- Heart failure with reduced ejection fraction (diagnosed 2023-01-15)
- Coronary artery disease (myocardial infarctions 5 years and 1 year prior)
- Type 2 diabetes mellitus with peripheral neuropathy
- Hypertension
- Chronic obstructive pulmonary disease
- Major depressive disorder
- History of complex partial seizures (well-controlled)

His past medical history is significant for hypertension, hyperlipidemia, and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Coronary artery bypass grafting (CABG)
- Cardiac catheterization with percutaneous coronary intervention (prior)

MEDICATIONS:
Home Medications:
- Aspirin 81 mg oral daily
- Metoprolol succinate 50 mg oral daily
- Atorvastatin 40 mg oral daily
- Lisinopril 20 mg oral daily
- Metformin 1000 mg oral twice daily
- Insulin glargine 24 units subcutaneously at bedtime
- Valproic acid 500 mg oral twice daily
- Albuterol HFA 90 mcg inhaled, 2 puffs every 4 hours as needed for shortness of breath

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Premature coronary artery disease (myocardial infarction at age 58)
- Mother: Type 2 diabetes mellitus
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-year history, quit 5 years ago. Alcohol: Former heavy alcohol use (2-3 drinks nightly), currently abstinent for the past 6 months. Occupation: Retired construction foreman.

VITAL SIGNS:
On physical examination, his blood pressure is 135/85 mmHg, heart rate is 78 beats per minute, respiratory rate is 16 breaths per minute, and oxygen saturation is 96% on room air.

PHYSICAL EXAMINATION:
Auscultation of the lungs reveals decreased breath sounds at the left lung base without wheezes or crackles. The remainder of the examination is unremarkable.

IMAGING:
A chest X-ray shows a small left-sided pleural effusion.

## Encounter 6: source 2024-08-15, practice 2024-07-11, outpatient, Psychiatry

Chief complaint: Six-week follow-up visit after hospitalization for acute ST-elevation myocardial infarction

ENCOUNTER NOTE
Date: 2024-08-15
Provider: Dr. Aisha Patel, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Six-week follow-up visit after hospitalization for acute ST-elevation myocardial infarction

HISTORY OF PRESENT ILLNESS:
The patient is a 68-year-old male with coronary artery disease including prior myocardial infarctions five years and one year prior, heart failure with reduced ejection fraction, and recent coronary artery bypass grafting who returns for psychiatric follow-up six weeks after hospitalization for acute ST-elevation myocardial infarction. He completed a course of reperfusion therapy and was discharged on aspirin, metoprolol, atorvastatin, and lisinopril. He reports adherence to his medications and cardiac rehabilitation program. However, he describes a persistent feeling of sadness, loss of interest in activities he once enjoyed, and difficulty sleeping. He also mentions a 5-pound weight loss and fatigue. He denies any suicidal ideation.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperlipidemia (diagnosed 2020-01-15)
- Statin-associated muscle symptoms (diagnosed 2020-07-15)
- Stable Angina Pectoris (Medical Management) (diagnosed 2021-01-15)
- Lateral plantar nerve (diagnosed 2022-01-15)
- Heart failure with reduced ejection fraction (diagnosed 2023-01-15)
- Right-sided pleural effusion (diagnosed 2024-07-15)
- Coronary artery disease (myocardial infarctions 5 years and 1 year prior)
- Type 2 diabetes mellitus with peripheral neuropathy
- Hypertension
- Chronic obstructive pulmonary disease
- Major depressive disorder
- History of complex partial seizures (well-controlled)

PAST SURGICAL HISTORY:
- Coronary artery bypass grafting (CABG)
- Cardiac catheterization with percutaneous coronary intervention (prior)

MEDICATIONS:
Home Medications:
- Aspirin 81 mg oral daily
- Metoprolol succinate 50 mg oral daily
- Atorvastatin 40 mg oral daily
- Lisinopril 20 mg oral daily
- Metformin 1000 mg oral twice daily
- Insulin glargine 24 units subcutaneously at bedtime
- Valproic acid 500 mg oral twice daily
- Albuterol HFA 90 mcg inhaled, 2 puffs every 4 hours as needed for shortness of breath

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Premature coronary artery disease (myocardial infarction at age 58)
- Mother: Type 2 diabetes mellitus
- Sister: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-year history, quit 5 years ago. Alcohol: Former heavy alcohol use (2-3 drinks nightly), currently abstinent for the past 6 months. Occupation: Retired construction foreman.

VITAL SIGNS:
His vital signs are within normal limits.

PHYSICAL EXAMINATION:
His physical examination is unremarkable, and

LABORATORY DATA:
His recent laboratory tests are as follows:

- Serum sodium: 140 mEq/L
- Serum potassium: 4.2 mEq/L
- Serum creatinine: 1.0 mg/dL
- Fasting glucose: 98 mg/dL
- Total cholesterol: 175 mg/dL
- LDL cholesterol: 70 mg/dL
- HDL cholesterol: 45 mg/dL

ASSESSMENT AND PLAN:
Given the patient's history and current symptoms,

## Encounter 7: source 2026-01-15, practice 2025-12-11, ed, Emergency Medicine

Chief complaint: Three-day history of sudden sharp pain in the back of his right ankle with clicking sensation and inability to bear weight

ENCOUNTER NOTE
Date: 2026-01-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Three-day history of sudden sharp pain in the back of his right ankle with clicking sensation and inability to bear weight

HISTORY OF PRESENT ILLNESS:
The patient is a 70-year-old male with coronary artery disease, heart failure with reduced ejection fraction, type 2 diabetes mellitus with peripheral neuropathy, and major depressive disorder who presents to the Emergency Department. He follows with Psychiatry for management of depressive symptoms following his recent hospitalization for acute ST-elevation myocardial infarction and has a history of right foot complications previously evaluated in Podiatry. He describes a sharp, stabbing pain that began while walking down a flight of stairs and reports worsening with any movement. He denies trauma, fever, or swelling elsewhere. He reports difficulty bearing weight on the right foot and notes a "clicking" sensation near the heel. He also reports that his mood has worsened significantly over the past month. He describes feeling increasingly fatigued, has lost interest in his usual routines, and is no longer regularly attending his community senior center, which he attributes to his mother's recent diagnosis of metastatic small cell lung carcinoma, leaving him feeling "helpless and guilty." He denies suicidal ideation but reports sleeping more during the day and skipping meals. He recently requested refills for all of his medications during a telehealth check-in but failed to mention his ankle symptoms at that time. He expresses worry that the added stress may be affecting his sleep and appetite and acknowledges being less consistent with his medications lately.

PAST MEDICAL HISTORY:
Active Problem List:
- Hyperlipidemia (diagnosed 2020-01-15)
- Statin-associated muscle symptoms (diagnosed 2020-07-15)
- Stable Angina Pectoris (Medical Management) (diagnosed 2021-01-15)
- Lateral plantar nerve (diagnosed 2022-01-15)
- Heart failure with reduced ejection fraction (diagnosed 2023-01-15)
- Right-sided pleural effusion (diagnosed 2024-07-15)
- Major depressive disorder (diagnosed 2024-08-15)
- Coronary artery disease (myocardial infarctions 5 years and 1 year prior)
- Type 2 diabetes mellitus with peripheral neuropathy
- Hypertension
- Chronic obstructive pulmonary disease
- History of complex partial seizures (well-controlled)

His past medical history includes coronary artery disease with a myocardial infarction 5 years ago, hypertension, hyperlipidemia, type 2 diabetes mellitus, and chronic obstructive pulmonary disease (COPD). He is also being treated for bipolar I disorder, which has been well-controlled on mood stabilizers.

PAST SURGICAL HISTORY:
- Coronary artery bypass grafting (CABG)
- Cardiac catheterization with percutaneous coronary intervention (prior)

MEDICATIONS:
His current medications include metoprolol, atorvastatin, lisinopril, aspirin, metformin, valproic acid, albuterol inhaler as needed, and ciprofloxacin, which was started 5 days ago for a urinary tract infection diagnosed at an urgent care clinic. He is not on corticosteroids.

Home Medications:
- Metoprolol succinate 50 mg oral daily
- Insulin glargine 24 units subcutaneously at bedtime
- Albuterol HFA 90 mcg inhaled, 2 puffs every 4 hours as needed for shortness of breath

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
His mother's recent diagnosis of metastatic small cell lung carcinoma

SOCIAL HISTORY:
He has a 50-pack-year smoking history and drinks 2–3 glasses of whiskey nightly. He also expresses frustration about ongoing arguments with his sister regarding their mother's estate planning. He says she has been repeatedly pressuring him to sign legal documents concerning inheritance distribution, despite their mother still being alive and receiving chemotherapy. He reports that these conversations make him feel overwhelmed and are straining their relationship. He says he tries to avoid phone calls from her and has even considered changing his number.

VITAL SIGNS:
His temperature is 36.9°C (98.4°F), pulse is 72/min, respiratory rate is 14/min, and blood pressure is 136/82 mm Hg.

PHYSICAL EXAMINATION:
On physical examination, he appears older than his stated age but is in no acute distress. He walks with an antalgic gait favoring the left side. There is mild swelling and point tenderness over the right posterior ankle just proximal to the heel. Thompson test reveals diminished plantar flexion when the calf is squeezed. Passive dorsiflexion elicits sharp discomfort. There is no redness, crepitus, or overlying warmth. His Achilles tendon appears thickened compared to the contralateral side. Strength is decreased on plantarflexion, but sensation and pulses are intact.

LABORATORY DATA:
Laboratory values show normal white blood cell count, ESR, and CRP.

IMAGING:
Radiographs of the ankle show mild degenerative changes but no fracture or calcification.
