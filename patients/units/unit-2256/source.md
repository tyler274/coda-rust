# synthetic_hospital patient 2256

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 67 at first encounter, sex M, Non-Hispanic African American, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-05-18 (Mon) | outpatient | Ophthalmology |
| 1 | 2020-09-15 | 2021-01-08 (Fri) | outpatient | Neurology |
| 2 | 2022-01-15 | 2022-05-26 (Thu) | ed | Emergency Medicine |
| 3 | 2022-01-15 | 2022-05-26 (Thu) | ed | Emergency Medicine |
| 4 | 2023-01-15 | 2023-05-18 (Thu) | ed | Emergency Medicine |
| 5 | 2024-01-15 | 2024-05-22 (Wed) | ed | Emergency Medicine |
| 6 | 2025-01-15 | 2025-05-22 (Thu) | outpatient | Cardiology |
| 7 | 2026-01-15 | 2026-05-27 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired postal service supervisor",
 "alcohol_use": "Social, approximately 2 drinks per week",
 "family_history": [
  "Father: Myocardial infarction at age 61, deceased",
  "Mother: Alzheimer's disease, onset age 72",
  "Sister: Primary open-angle glaucoma"
 ],
 "race_ethnicity": "Non-Hispanic African American",
 "smoking_status": "Former smoker, quit 8 years ago, 25 pack-year history",
 "home_medications": [
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Amlodipine"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  },
  {
   "dose": "50 mg PO daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "1 tablet PO daily",
   "name": "Antiretroviral therapy (biktarvy)"
  },
  {
   "dose": "1 drop OU at bedtime",
   "name": "Latanoprost ophthalmic solution"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Donepezil"
  }
 ],
 "surgical_history": [
  "Percutaneous coronary intervention with drug-eluting stent placement (inferior wall)",
  "Status post ventricular septal defect repair",
  "Cataract extraction with intraocular lens placement, right eye (year prior)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Hyperlipidemia",
  "HIV infection, well-controlled on antiretroviral therapy",
  "Alzheimer's disease",
  "Primary open-angle glaucoma",
  "Coronary artery disease, status post inferior wall myocardial infarction",
  "History of cerebrovascular accident (anterior cerebral artery territory)"
 ],
 "age_at_first_encounter": 67
}
```

Primary diagnoses: ["Alzheimer's disease", "Ventricular septal defect", "Acute angle-closure glaucoma", "Primary open-angle glaucoma", "Acute decompensated heart failure", "Anterior cerebral artery territory infarction", "ST-segment elevation myocardial infarction, inferior wall", "Concussion (Mild Traumatic Brain Injury)"]
Comorbidities: ["Hyperlipidemia", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2020-05-18, outpatient, Ophthalmology

Chief complaint: Progressive difficulty perceiving objects outside his direct line of sight over the past 8 months, frequently bumping into furniture and having trouble seeing pedestrians while driving.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Ophthalmology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive difficulty perceiving objects outside his direct line of sight over the past 8 months, frequently bumping into furniture and having trouble seeing pedestrians while driving.

HISTORY OF PRESENT ILLNESS:
Progressive difficulty perceiving objects outside his direct line of sight over the past 8 months. He notices that he often bumps into furniture and has trouble seeing pedestrians while driving unless they are directly in front of him. He also reports intermittent mild eye discomfort, particularly in dim lighting. There are no complaints of acute vision loss, flashes, floaters, or photopsia.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Hyperlipidemia
- HIV infection, well-controlled on antiretroviral therapy
- Alzheimer's disease
- Primary open-angle glaucoma
- Coronary artery disease, status post inferior wall myocardial infarction
- History of cerebrovascular accident (anterior cerebral artery territory)

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (inferior wall)
- Status post ventricular septal defect repair
- Cataract extraction with intraocular lens placement, right eye (year prior)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Antiretroviral therapy (biktarvy) 1 tablet PO daily
- Latanoprost ophthalmic solution 1 drop OU at bedtime
- Donepezil 10 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 61, deceased
- Mother: Alzheimer's disease, onset age 72
- Sister: Primary open-angle glaucoma

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago, 25 pack-year history. Alcohol: Social, approximately 2 drinks per week. Occupation: Retired postal service supervisor.

PHYSICAL EXAMINATION:
On ophthalmologic examination, his intraocular pressure is 27 mm Hg (reference range: 10-22 mm Hg), and funduscopy reveals bilateral optic disc cupping without flame hemorrhages or exudates. Central vision is intact

OTHER STUDIES:
Formal visual field testing demonstrates marked peripheral constriction

## Encounter 1: source 2020-09-15, practice 2021-01-08, outpatient, Neurology

Chief complaint: Progressive memory loss over the past year with increasing difficulty in problem-solving and disorientation within his own home, reported by his daughter.

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. James Washington, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive memory loss over the past year with increasing difficulty in problem-solving and disorientation within his own home, reported by his daughter.

HISTORY OF PRESENT ILLNESS:
This 67-year-old male with hypertension, hyperlipidemia, well-controlled HIV, coronary artery disease, and primary open-angle glaucoma evaluated in ophthalmology earlier this year returns to clinic with his daughter, who reports that her father has been experiencing progressive memory loss over the past year. She also notes that he has been having difficulty with problem-solving and has become increasingly disoriented, even within his own home. The patient has been adherent to his ART regimen and has had an undetectable viral load for the past 10 years.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary open-angle glaucoma (diagnosed 2020-01-15)
- Hypertension
- Hyperlipidemia
- HIV infection, well-controlled on antiretroviral therapy
- Alzheimer's disease
- Coronary artery disease, status post inferior wall myocardial infarction
- History of cerebrovascular accident (anterior cerebral artery territory)

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (inferior wall)
- Status post ventricular septal defect repair
- Cataract extraction with intraocular lens placement, right eye (year prior)

MEDICATIONS:
He takes antiretroviral therapy (ART), lisinopril and atorvastatin.

Home Medications:
- Amlodipine 5 mg PO daily
- Aspirin 81 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Antiretroviral therapy (biktarvy) 1 tablet PO daily
- Latanoprost ophthalmic solution 1 drop OU at bedtime
- Donepezil 10 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 61, deceased
- Mother: Alzheimer's disease, onset age 72
- Sister: Primary open-angle glaucoma

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago, 25 pack-year history. Alcohol: Social, approximately 2 drinks per week. Occupation: Retired postal service supervisor.

PHYSICAL EXAMINATION:
On physical examination, the patient is alert but appears apathetic. He has difficulty recalling recent events and struggles to perform serial 7s. His gait is normal, and there are no signs of motor weakness. A Mini-Mental State Examination (MMSE) score is 20/30.

LABORATORY DATA:
His most recent laboratory results are as follows:

- CD4 count: 600 cells/mm³
- HIV viral load: <20 copies/mL
- Complete blood count: within normal limits
- Fasting glucose: 98 mg/dL
- Serum creatinine: 1.0 mg/dL
- Liver function tests: within normal limits

## Encounter 2: source 2022-01-15, practice 2022-05-26, ed, Emergency Medicine

Chief complaint: Acute onset of shortness of breath, orthopnea, and palpitations that began 2 hours ago while mowing the lawn, without chest pain.

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Maria Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset of shortness of breath, orthopnea, and palpitations that began 2 hours ago while mowing the lawn, without chest pain.

HISTORY OF PRESENT ILLNESS:
The patient with coronary artery disease, prior inferior wall myocardial infarction, and primary open-angle glaucoma presents with acute onset of shortness of breath, orthopnea, and palpitations that began 2 hours ago while he was mowing the lawn. He denies chest pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary open-angle glaucoma (diagnosed 2020-01-15)
- Alzheimer's disease (diagnosed 2020-09-15)
- Hypertension
- Hyperlipidemia
- HIV infection, well-controlled on antiretroviral therapy
- Coronary artery disease, status post inferior wall myocardial infarction
- History of cerebrovascular accident (anterior cerebral artery territory)

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (inferior wall)
- Status post ventricular septal defect repair
- Cataract extraction with intraocular lens placement, right eye (year prior)

MEDICATIONS:
His medications include lisinopril and atorvastatin.

Home Medications:
- Amlodipine 5 mg PO daily
- Aspirin 81 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Antiretroviral therapy (biktarvy) 1 tablet PO daily
- Latanoprost ophthalmic solution 1 drop OU at bedtime
- Donepezil 10 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 61, deceased
- Mother: Alzheimer's disease, onset age 72
- Sister: Primary open-angle glaucoma

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago, 25 pack-year history. Alcohol: Social, approximately 2 drinks per week. Occupation: Retired postal service supervisor.

VITAL SIGNS:
On examination, his blood pressure is 150/90 mmHg, heart rate is 110 bpm, respiratory rate is 28 breaths/min, and oxygen saturation is 88% on room air.

PHYSICAL EXAMINATION:
He appears in respiratory distress with bilateral rales halfway up both lung fields and has jugular venous distension. Cardiac auscultation reveals a new holosystolic murmur at the apex radiating to the axilla.

LABORATORY DATA:
Troponin I level is elevated at 10 ng/mL (normal <0.04 ng/mL).

IMAGING:
Chest X-ray demonstrates pulmonary edema.

OTHER STUDIES:
Electrocardiogram shows ST-segment elevations in leads II, III, and aVF.

## Encounter 3: source 2022-01-15, practice 2022-05-26, ed, Emergency Medicine

Chief complaint: Acute onset of shortness of breath, chest pain, and palpitations that began suddenly while at rest, occurring five days after recent myocardial infarction admission.

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset of shortness of breath, chest pain, and palpitations that began suddenly while at rest, occurring five days after recent myocardial infarction admission.

HISTORY OF PRESENT ILLNESS:
Patient with CAD and prior CVA returns five days following MI admission. He reports shortness of breath, chest pain, and palpitations that began suddenly while at rest, discharged two days ago on aspirin, metoprolol, and atorvastatin.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary open-angle glaucoma (diagnosed 2020-01-15)
- Alzheimer's disease (diagnosed 2020-09-15)
- ST-segment elevation myocardial infarction, inferior wall (diagnosed 2022-01-15)
- Hypertension
- Hyperlipidemia
- HIV infection, well-controlled on antiretroviral therapy
- Coronary artery disease, status post inferior wall myocardial infarction
- History of cerebrovascular accident (anterior cerebral artery territory)

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (inferior wall)
- Status post ventricular septal defect repair
- Cataract extraction with intraocular lens placement, right eye (year prior)

MEDICATIONS:
He takes aspirin, metoprolol, and atorvastatin.

Home Medications:
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Antiretroviral therapy (biktarvy) 1 tablet PO daily
- Latanoprost ophthalmic solution 1 drop OU at bedtime
- Donepezil 10 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 61, deceased
- Mother: Alzheimer's disease, onset age 72
- Sister: Primary open-angle glaucoma

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago, 25 pack-year history. Alcohol: Social, approximately 2 drinks per week. Occupation: Retired postal service supervisor.

VITAL SIGNS:
Blood pressure is 85/50 mmHg, heart rate is 122 bpm, and respiratory rate is 28 breaths/min

PHYSICAL EXAMINATION:
He appears diaphoretic and in distress. Auscultation of the chest reveals rales at the lung bases and a new harsh holosystolic murmur with a palpable thrill at the left sternal border. Jugular venous pressure is elevated.

LABORATORY DATA:
Laboratory Values:
- Troponin I: Elevated
- B-type natriuretic peptide (BNP): Significantly elevated
- Creatinine: 1.2 mg/dL (baseline 1.0 mg/dL)
- Hemoglobin: 13.2 g/dL
- White Blood Cell Count: 12,000 /μL

IMAGING:
A bedside echocardiogram reveals left ventricular dilation and an abnormal flow of blood between the ventricles.

OTHER STUDIES:
The electrocardiogram shows Q waves in the anterior leads that were present on discharge.

## Encounter 4: source 2023-01-15, practice 2023-05-18, ed, Emergency Medicine

Chief complaint: Dragging his left foot while walking with subjective inability to lift the left leg well, noticed acutely by his wife.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. David Okafor, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Dragging his left foot while walking with subjective inability to lift the left leg well, noticed acutely by his wife.

HISTORY OF PRESENT ILLNESS:
The patient with history of ACA infarction and Alzheimer's disease presents with acute gait changes. He is brought to the emergency department after his wife noticed he was dragging his left foot while walking. He denies pain but says he feels like he "can't lift his leg well."

PAST MEDICAL HISTORY:
Active Problem List:
- Primary open-angle glaucoma (diagnosed 2020-01-15)
- Alzheimer's disease (diagnosed 2020-09-15)
- ST-segment elevation myocardial infarction, inferior wall (diagnosed 2022-01-15)
- Ventricular septal defect (diagnosed 2022-01-15)
- Hypertension
- Hyperlipidemia
- HIV infection, well-controlled on antiretroviral therapy
- Coronary artery disease, status post inferior wall myocardial infarction
- History of cerebrovascular accident (anterior cerebral artery territory)

He has a history of hypertension and hyperlipidemia. He has no prior history of stroke or TIA.

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (inferior wall)
- Status post ventricular septal defect repair
- Cataract extraction with intraocular lens placement, right eye (year prior)

MEDICATIONS:
He takes he takes lisinopril and atorvastatin.

Home Medications:
- Amlodipine 5 mg PO daily
- Aspirin 81 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Antiretroviral therapy (biktarvy) 1 tablet PO daily
- Latanoprost ophthalmic solution 1 drop OU at bedtime
- Donepezil 10 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 61, deceased
- Mother: Alzheimer's disease, onset age 72
- Sister: Primary open-angle glaucoma

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago, 25 pack-year history. Alcohol: Social, approximately 2 drinks per week. Occupation: Retired postal service supervisor.

PHYSICAL EXAMINATION:
On examination, he is alert and oriented. Cranial nerves are intact, and strength is 5/5 in the upper extremities and right leg. Strength is 3/5 in the left lower extremity with slightly increased tone. There is no sensory loss, and coordination and reflexes are normal in the arms.

IMAGING:
Non-contrast CT of the head confirms the diagnosis.

## Encounter 5: source 2024-01-15, practice 2024-05-22, ed, Emergency Medicine

Chief complaint: Sudden severe left eye pain with frontal headache, vomiting, blurred vision, and multicolored halos around lights that began two hours ago.

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Michael Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden severe left eye pain with frontal headache, vomiting, blurred vision, and multicolored halos around lights that began two hours ago.

HISTORY OF PRESENT ILLNESS:
The patient is a 71-year-old male with a history of primary open-angle glaucoma who presents with acute ocular symptoms. He reports that these symptoms began two hours ago. He notes that his vision in the left eye has become blurred and describes seeing multicolored halos around lights. He denies recent trauma, fever, or discharge. He reports that light makes his symptoms worse, and he is unable to keep his left eye open.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary open-angle glaucoma (diagnosed 2020-01-15)
- Alzheimer's disease (diagnosed 2020-09-15)
- ST-segment elevation myocardial infarction, inferior wall (diagnosed 2022-01-15)
- Ventricular septal defect (diagnosed 2022-01-15)
- Anterior cerebral artery territory infarction (diagnosed 2023-01-15)
- Hypertension
- Hyperlipidemia
- HIV infection, well-controlled on antiretroviral therapy
- Coronary artery disease, status post inferior wall myocardial infarction
- History of cerebrovascular accident (anterior cerebral artery territory)

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (inferior wall)
- Status post ventricular septal defect repair
- Cataract extraction with intraocular lens placement, right eye (year prior)

MEDICATIONS:
He takes regular use of amlodipine and atorvastatin.

Home Medications:
- Lisinopril 20 mg PO daily
- Aspirin 81 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Antiretroviral therapy (biktarvy) 1 tablet PO daily
- Latanoprost ophthalmic solution 1 drop OU at bedtime
- Donepezil 10 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 61, deceased
- Mother: Alzheimer's disease, onset age 72
- Sister: Primary open-angle glaucoma

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago, 25 pack-year history. Alcohol: Social, approximately 2 drinks per week. Occupation: Retired postal service supervisor.

PHYSICAL EXAMINATION:
On exam, the left eye is erythematous, with marked conjunctival injection, excessive tearing, and a pupil that appears fixed and mid-dilated. The right eye is normal. The cornea of the left eye is cloudy, and the anterior chamber appears shallow.

OTHER STUDIES:
Tonometry reveals an intraocular pressure of 38 mm Hg (reference range: 10–22 mm Hg).

## Encounter 6: source 2025-01-15, practice 2025-05-22, outpatient, Cardiology

Chief complaint: Follow-up clinic visit for secondary prevention of myocardial infarction as part of a clinical trial with new medication initiation.

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Jennifer Liu, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up clinic visit for secondary prevention of myocardial infarction as part of a clinical trial with new medication initiation.

HISTORY OF PRESENT ILLNESS:
The patient is a 72-year-old male with coronary artery disease status post inferior wall myocardial infarction complicated by ventricular septal defect who returns for clinical trial follow-up for secondary prevention. He was recently started on a new medication as part of a clinical trial for secondary prevention of myocardial infarction. The patient reports adherence to all medications and a healthy diet. He denies any chest pain, shortness of breath, or other new symptoms. The patient is interested in understanding the effectiveness of his new medication.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary open-angle glaucoma (diagnosed 2020-01-15)
- Alzheimer's disease (diagnosed 2020-09-15)
- ST-segment elevation myocardial infarction, inferior wall (diagnosed 2022-01-15)
- Ventricular septal defect (diagnosed 2022-01-15)
- Anterior cerebral artery territory infarction (diagnosed 2023-01-15)
- Acute angle-closure glaucoma (diagnosed 2024-01-15)
- Hypertension
- Hyperlipidemia
- HIV infection, well-controlled on antiretroviral therapy
- Coronary artery disease, status post inferior wall myocardial infarction
- History of cerebrovascular accident (anterior cerebral artery territory)

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (inferior wall)
- Status post ventricular septal defect repair
- Cataract extraction with intraocular lens placement, right eye (year prior)

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Antiretroviral therapy (biktarvy) 1 tablet PO daily
- Latanoprost ophthalmic solution 1 drop OU at bedtime
- Donepezil 10 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 61, deceased
- Mother: Alzheimer's disease, onset age 72
- Sister: Primary open-angle glaucoma

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago, 25 pack-year history. Alcohol: Social, approximately 2 drinks per week. Occupation: Retired postal service supervisor.

VITAL SIGNS:
His blood pressure is 132/84 mmHg, and his heart rate is 78 bpm.

PHYSICAL EXAMINATION:
His physical examination is unremarkable.

LABORATORY DATA:
Laboratory values are as follows:

- Total cholesterol: 190 mg/dL
- LDL cholesterol: 110 mg/dL
- HDL cholesterol: 45 mg/dL
- Triglycerides: 150 mg/dL

ASSESSMENT AND PLAN:
You review the clinical trial data and note the following hazard ratios for various outcomes:

- All-cause mortality: 0.85
- Nonfatal myocardial infarction: 0.90
- Stroke: 0.95
- Hospitalization for heart failure: 0.80

Based on the hazard ratios provided,

## Encounter 7: source 2026-01-15, practice 2026-05-27, ed, Emergency Medicine

Chief complaint: Slipping on ice and hitting the back of his head with brief loss of consciousness, vomiting twice, and persistent dull headache.

ENCOUNTER NOTE
Date: 2026-01-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Slipping on ice and hitting the back of his head with brief loss of consciousness, vomiting twice, and persistent dull headache.

HISTORY OF PRESENT ILLNESS:
The patient is a 73-year-old male with Alzheimer's disease and prior cerebrovascular accident who presents to the Emergency Department. He briefly lost consciousness but was alert and responsive within minutes. Emergency personnel report that he vomited twice en route to the hospital. In the ED, he is mildly confused but oriented and reports a lingering dull headache. His symptoms have gradually improved over the last hour.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary open-angle glaucoma (diagnosed 2020-01-15)
- Alzheimer's disease (diagnosed 2020-09-15)
- ST-segment elevation myocardial infarction, inferior wall (diagnosed 2022-01-15)
- Ventricular septal defect (diagnosed 2022-01-15)
- Anterior cerebral artery territory infarction (diagnosed 2023-01-15)
- Acute angle-closure glaucoma (diagnosed 2024-01-15)
- Acute decompensated heart failure (diagnosed 2025-01-15)
- Hypertension
- Hyperlipidemia
- HIV infection, well-controlled on antiretroviral therapy
- Coronary artery disease, status post inferior wall myocardial infarction
- History of cerebrovascular accident (anterior cerebral artery territory)

His past medical history includes hypertension and hyperlipidemia.

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (inferior wall)
- Status post ventricular septal defect repair
- Cataract extraction with intraocular lens placement, right eye (year prior)

MEDICATIONS:
He is not taking any anticoagulant medications.

Home Medications:
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Antiretroviral therapy (biktarvy) 1 tablet PO daily
- Latanoprost ophthalmic solution 1 drop OU at bedtime
- Donepezil 10 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 61, deceased
- Mother: Alzheimer's disease, onset age 72
- Sister: Primary open-angle glaucoma

SOCIAL HISTORY:
Smoking: Former smoker, quit 8 years ago, 25 pack-year history. Alcohol: Social, approximately 2 drinks per week. Occupation: Retired postal service supervisor.

REVIEW OF SYSTEMS:
He denies visual changes, seizures, or weakness.

PHYSICAL EXAMINATION:
There are no focal neurological deficits on exam, and he is speaking coherently. After a brief period of observation, he is awake, cooperative, and ambulatory, though still mildly fatigued.
