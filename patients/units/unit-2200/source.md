# synthetic_hospital patient 2200

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 63 at first encounter, sex M, White, non-Hispanic, Private insurance (Blue Cross Blue Shield).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-08-04 (Fri) | outpatient | Cardiology |
| 1 | 2021-01-15 | 2024-07-30 (Tue) | outpatient | Internal Medicine |
| 2 | 2021-07-15 | 2025-02-10 (Mon) | outpatient | Cardiology |
| 3 | 2022-01-15 | 2025-08-04 (Mon) | outpatient | Cardiology |
| 4 | 2023-01-15 | 2026-07-31 (Fri) | procedure | Orthopedic Surgery |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private insurance (Blue Cross Blue Shield)",
 "occupation": "Retired construction foreman",
 "alcohol_use": "None",
 "family_history": [
  "Father: Myocardial infarction at age 68",
  "Mother: Type 2 diabetes mellitus",
  "Brother: Coronary artery disease"
 ],
 "race_ethnicity": "White, non-Hispanic",
 "smoking_status": "Former smoker, quit 10 years ago (30 pack-year history)",
 "home_medications": [
  {
   "dose": "81 mg daily",
   "name": "Aspirin"
  },
  {
   "dose": "1000 mg twice daily",
   "name": "Metformin"
  },
  {
   "dose": "20 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "50 mg daily",
   "name": "Metoprolol succinate"
  }
 ],
 "surgical_history": [
  "Cardiac catheterization with coronary angiography (recent)",
  "Scheduled: Coronary artery bypass grafting",
  "Scheduled: Total knee arthroplasty"
 ],
 "chronic_conditions": [
  "Type 2 diabetes mellitus",
  "Hypertension",
  "Coronary artery disease (severe triple-vessel)",
  "History of myocardial infarction (7 years prior)",
  "Hyperlipidemia",
  "Peripheral vascular disease",
  "Severe osteoarthritis of the knee"
 ],
 "age_at_first_encounter": 63
}
```

Primary diagnoses: ["Stable Angina Pectoris due to Coronary Artery Disease", "Stable Angina Pectoris (Medical Management)", "Prophylaxis against surgical site infection in total knee arthroplasty", "Aspirin therapy for cardiovascular disease primary prevention"]
Comorbidities: ["Essential hypertension", "Hyperlipidemia", "Obesity", "Type 2 diabetes mellitus", "Peripheral arterial disease"]

## Encounter 0: source 2020-01-15, practice 2023-08-04, outpatient, Cardiology

Chief complaint: Routine follow-up for cardiovascular disease management and medication review.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for cardiovascular disease management and medication review.

HISTORY OF PRESENT ILLNESS:
He reports that he has been taking several medications for many years due to his extensive cardiovascular history. He has no recent hospitalizations.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus
- Hypertension
- Coronary artery disease (severe triple-vessel)
- History of myocardial infarction (7 years prior)
- Hyperlipidemia
- Peripheral vascular disease
- Severe osteoarthritis of the knee

He has also been diagnosed with peripheral vascular disease, which causes him intermittent claudication in his calves after walking a few blocks. He has hypertension and hyperlipidemia, both managed with medication.

PAST SURGICAL HISTORY:
He underwent coronary stent placement at that time.

MEDICATIONS:
He notes that his doctor placed him to take aspirin that he has continued ever since his heart attack. He tolerates it well, though he does experience occasional bruising.

Home Medications:
- Metformin 1000 mg twice daily
- Lisinopril 20 mg daily
- Atorvastatin 40 mg daily
- Metoprolol succinate 50 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Type 2 diabetes mellitus
- Brother: Coronary artery disease

SOCIAL HISTORY:
He quit smoking 10 years ago after having smoked a pack a day for nearly 30 years.

REVIEW OF SYSTEMS:
He denies chest pain, shortness of breath, palpitations, or syncope. He denies gastrointestinal bleeding, melena, or hematemesis.

VITAL SIGNS:
On physical exam, his blood pressure is 138/78 mmHg, heart rate is 72/min, and BMI is 29 kg/m².

PHYSICAL EXAMINATION:
He has diminished dorsalis pedis and posterior tibial pulses bilaterally. His feet are warm, but there is hair loss over the shins. The remainder of his exam is unremarkable.

LABORATORY DATA:
Laboratory studies reveal LDL of 82 mg/dL and HDL of 46 mg/dL. His hemoglobin and platelet counts are within normal limits.

OTHER STUDIES:
An ECG shows normal sinus rhythm.

## Encounter 1: source 2021-01-15, practice 2024-07-30, outpatient, Internal Medicine

Chief complaint: New onset exertional chest pain that resolves with rest.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. James O'Brien, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
New onset exertional chest pain that resolves with rest.

HISTORY OF PRESENT ILLNESS:
This 64-year-old male with severe triple-vessel CAD and prior MI returns after Cardiology follow-up. He reports a new onset of exertional chest pain that resolves with rest. He denies any recent infections or other complaints.

PAST MEDICAL HISTORY:
Active Problem List:
- Aspirin therapy for cardiovascular disease primary prevention (diagnosed 2020-01-15)
- Type 2 diabetes mellitus
- Hypertension
- Coronary artery disease (severe triple-vessel)
- History of myocardial infarction (7 years prior)
- Hyperlipidemia
- Peripheral vascular disease
- Severe osteoarthritis of the knee

PAST SURGICAL HISTORY:
- Cardiac catheterization with coronary angiography (recent)
- Scheduled: Coronary artery bypass grafting
- Scheduled: Total knee arthroplasty

MEDICATIONS:
His current medications include metformin, lisinopril, and atorvastatin.

Home Medications:
- Aspirin 81 mg daily
- Metoprolol succinate 50 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Type 2 diabetes mellitus
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago (30 pack-year history). Alcohol: None. Occupation: Retired construction foreman.

VITAL SIGNS:
On physical examination, his blood pressure is 142/88 mmHg, heart rate is 78 bpm, and BMI is 32 kg/m^2.

PHYSICAL EXAMINATION:
Cardiac examination reveals no murmurs, rubs, or gallops.

LABORATORY DATA:
His recent laboratory results are as follows:

- Hemoglobin A1c: 7.2%
- Total cholesterol: 190 mg/dL
- LDL cholesterol: 100 mg/dL
- HDL cholesterol: 40 mg/dL
- Triglycerides: 150 mg/dL
- Fasting glucose: 158 mg/dL

OTHER STUDIES:
An electrocardiogram shows no acute ST changes or Q waves.

## Encounter 2: source 2021-07-15, practice 2025-02-10, outpatient, Cardiology

Chief complaint: Follow-up for exertional chest pain with new shortness of breath when climbing stairs.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Sarah Chen, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up for exertional chest pain with new shortness of breath when climbing stairs.

HISTORY OF PRESENT ILLNESS:
He returns for follow-up of his severe triple-vessel coronary artery disease and prior exertional chest pain. He reports a new onset of exertional chest pain that resolves with rest. He also mentions shortness of breath when climbing stairs, which was not present during his last visit 6 months ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Aspirin therapy for cardiovascular disease primary prevention (diagnosed 2020-01-15)
- Stable Angina Pectoris (Medical Management) (diagnosed 2021-01-15)
- Type 2 diabetes mellitus
- Hypertension
- Coronary artery disease (severe triple-vessel)
- History of myocardial infarction (7 years prior)
- Hyperlipidemia
- Peripheral vascular disease
- Severe osteoarthritis of the knee

PAST SURGICAL HISTORY:
- Cardiac catheterization with coronary angiography (recent)
- Scheduled: Coronary artery bypass grafting
- Scheduled: Total knee arthroplasty

MEDICATIONS:
His current medications include metformin, lisinopril, and atorvastatin.

Home Medications:
- Aspirin 81 mg daily
- Metoprolol succinate 50 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Type 2 diabetes mellitus
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago (30 pack-year history). Alcohol: None. Occupation: Retired construction foreman.

PHYSICAL EXAMINATION:
On physical examination, his blood pressure is 142/88 mmHg, heart rate is 78 bpm, and BMI is 32 kg/m^2.

LABORATORY DATA:
His recent laboratory values are as follows:

- Hemoglobin A1c: 7.2%
- Total cholesterol: 190 mg/dL
- LDL cholesterol: 100 mg/dL
- HDL cholesterol: 40 mg/dL
- Triglycerides: 150 mg/dL
- Serum creatinine: 1.1 mg/dL

OTHER STUDIES:
An electrocardiogram (ECG) shows no acute ST changes or Q waves.

ASSESSMENT AND PLAN:
Given the patient's symptoms and history,

## Encounter 3: source 2022-01-15, practice 2025-08-04, outpatient, Cardiology

Chief complaint: Pre-operative evaluation for coronary artery bypass grafting following recent cardiac catheterization showing severe triple-vessel disease.

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Michael Patel, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Pre-operative evaluation for coronary artery bypass grafting following recent cardiac catheterization showing severe triple-vessel disease.

HISTORY OF PRESENT ILLNESS:
The patient returns for pre-operative evaluation following recent cardiac catheterization. He has a history of exertional chest pain previously managed with medical therapy. Despite maximal medical therapy with beta-blockers, high-intensity statins, and ACE inhibitors, he continues to experience exertional chest pain and occasional episodes of rest angina. His most recent stress test demonstrated significant ST-segment depression at low workload, and subsequent coronary angiography confirmed the need for surgical revascularization.

PAST MEDICAL HISTORY:
Active Problem List:
- Aspirin therapy for cardiovascular disease primary prevention (diagnosed 2020-01-15)
- Stable Angina Pectoris (Medical Management) (diagnosed 2021-01-15)
- Type 2 diabetes mellitus
- Hypertension
- Coronary artery disease (severe triple-vessel)
- History of myocardial infarction (7 years prior)
- Hyperlipidemia
- Peripheral vascular disease
- Severe osteoarthritis of the knee

He has a long history of poorly controlled hypertension, type 2 diabetes mellitus, and hyperlipidemia.

PAST SURGICAL HISTORY:
- Cardiac catheterization with coronary angiography (recent)
- Scheduled: Coronary artery bypass grafting
- Scheduled: Total knee arthroplasty

MEDICATIONS:
He takes maximal medical therapy with beta-blockers, high-intensity statins, and ACE inhibitors.

Home Medications:
- Aspirin 81 mg daily
- Metformin 1000 mg twice daily
- Lisinopril 20 mg daily
- Atorvastatin 40 mg daily
- Metoprolol succinate 50 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
His father also underwent heart surgery at around the same age, and he remembers visiting him in the hospital when he was a teenager

SOCIAL HISTORY:
He explains that he has always feared surgery but wants to live longer to spend more time with his grandchildren. He describes his daily routine of waking early, making oatmeal with raisins, and reading the local newspaper before walking to a small cafe where he enjoys almond toffee coffee with a modicum of cinnamon and conversation with friends. He also mentions that he has a fondness for chess and that he was his high school chess champion "back in the day."

VITAL SIGNS:
His vital signs are stable. He has a body mass index of 31

PHYSICAL EXAMINATION:
Cardiac auscultation reveals a soft S4 gallop. There is mild pitting edema at both ankles. Peripheral pulses are present but diminished.

LABORATORY DATA:
Mild anemia and elevated HbA1c

IMAGING:
An echocardiogram shows reduced left ventricular ejection fraction of 38% with hypokinesis of the anterior wall.

OTHER STUDIES:
Cardiac catheterization revealed severe triple-vessel coronary artery disease; His most recent stress test demonstrated significant ST-segment depression at low workload, and subsequent coronary angiography confirmed the need for surgical revascularization.

PLAN:
The patient has been counseled extensively about the procedure and agrees to proceed with bypass surgery. The surgeon explains that a conduit vessel will be harvested to create grafts for bypassing the obstructed coronary arteries. While the internal mammary artery is sometimes used, the great saphenous vein of the lower extremity remains the most common vessel for grafting due to its length, accessibility, and caliber. He expresses some anxiety about the operation but is reassured when told that vein harvesting is routine and has well-defined anatomical landmarks. The surgeon emphasizes that careful incision placement helps avoid complications and ensures adequate conduit length for bypass grafting.

## Encounter 4: source 2023-01-15, practice 2026-07-31, procedure, Orthopedic Surgery

Chief complaint: Scheduled for elective total knee arthroplasty due to severe osteoarthritis.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Robert Thompson, MD
Department: Orthopedic Surgery
Visit Type: procedure

CHIEF COMPLAINT:
Scheduled for elective total knee arthroplasty due to severe osteoarthritis.

PAST MEDICAL HISTORY:
Active Problem List:
- Aspirin therapy for cardiovascular disease primary prevention (diagnosed 2020-01-15)
- Stable Angina Pectoris (Medical Management) (diagnosed 2021-01-15)
- Stable Angina Pectoris due to Coronary Artery Disease (diagnosed 2022-01-15)
- Type 2 diabetes mellitus
- Hypertension
- Coronary artery disease (severe triple-vessel)
- History of myocardial infarction (7 years prior)
- Hyperlipidemia
- Peripheral vascular disease
- Severe osteoarthritis of the knee

PAST SURGICAL HISTORY:
- Cardiac catheterization with coronary angiography (recent)
- Scheduled: Coronary artery bypass grafting
- Scheduled: Total knee arthroplasty

MEDICATIONS:
His medications include metformin and lisinopril.

Home Medications:
- Aspirin 81 mg daily
- Atorvastatin 40 mg daily
- Metoprolol succinate 50 mg daily

ALLERGIES:
He has no known drug allergies.

FAMILY HISTORY:
- Father: Myocardial infarction at age 68
- Mother: Type 2 diabetes mellitus
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago (30 pack-year history). Alcohol: None. Occupation: Retired construction foreman.

VITAL SIGNS:
His blood pressure is 135/85 mmHg, heart rate is 78 bpm, and BMI is 32 kg/m^2.

PHYSICAL EXAMINATION:
On physical examination, the knee is non-erythematous with no signs of infection, but there is significant joint space narrowing and crepitus with movement.

LABORATORY DATA:
Preoperative labs are within normal limits.
