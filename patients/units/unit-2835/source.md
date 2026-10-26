# synthetic_hospital patient 2835

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 66 at first encounter, sex F, Non-Hispanic White, Medicare with supplemental Medigap policy.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-10-07 (Thu) | outpatient | Internal Medicine |
| 1 | 2020-04-15 | 2022-01-14 (Fri) | outpatient | Psychiatry |
| 2 | 2021-01-15 | 2022-10-10 (Mon) | outpatient | Rheumatology |
| 3 | 2022-01-15 | 2023-10-18 (Wed) | inpatient | General Surgery |
| 4 | 2023-01-15 | 2024-10-08 (Tue) | follow_up | Neurology |
| 5 | 2023-04-15 | 2025-01-13 (Mon) | outpatient | Psychiatry |
| 6 | 2024-01-15 | 2025-10-12 (Sun) | ed | Emergency Medicine |
| 7 | 2024-02-15 | 2025-11-25 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare with supplemental Medigap policy",
 "occupation": "Retired high school teacher",
 "alcohol_use": "None",
 "family_history": [
  "Father: Myocardial infarction at age 70",
  "Mother: Type 2 diabetes mellitus",
  "Sister: Primary open-angle glaucoma"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 1 year ago, 40 pack-year history",
 "home_medications": [
  {
   "dose": "5 mg daily",
   "name": "Amlodipine"
  },
  {
   "dose": "20 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "25 mg daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "1000 mg twice daily",
   "name": "Metformin"
  },
  {
   "dose": "81 mg daily",
   "name": "Aspirin"
  },
  {
   "dose": "15 mg nightly at bedtime",
   "name": "Temazepam"
  }
 ],
 "surgical_history": [
  "Laparoscopic cholecystectomy"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Type 2 Diabetes Mellitus",
  "Hyperlipidemia",
  "Chronic insomnia",
  "Hyperopia",
  "History of recurrent ischemic stroke with residual mild bilateral weakness"
 ],
 "age_at_first_encounter": 66
}
```

Primary diagnoses: ["Lacunar stroke", "Post-stroke depression", "Renal artery stenosis", "Benzodiazepine-induced delirium", "Acute angle-closure glaucoma", "Major depressive disorder", "Polymyalgia rheumatica"]
Comorbidities: ["Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2021-10-07, outpatient, Internal Medicine

Chief complaint: Difficult-to-control hypertension despite three antihypertensive medications

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Difficult-to-control hypertension despite three antihypertensive medications

HISTORY OF PRESENT ILLNESS:
She was diagnosed with high blood pressure 12 years ago and had stable control for many years on a low-dose thiazide. Over the past 8 months, however, her blood pressure has progressively worsened despite the addition of an ACE inhibitor and calcium channel blocker. She now takes 3 antihypertensive medications at maximum doses but continues to have systolic readings >160 mm Hg at home. She also reports cramping pain in both calves that starts after walking 2–3 blocks and resolves within minutes of rest.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Chronic insomnia
- Hyperopia
- History of recurrent ischemic stroke with residual mild bilateral weakness

Her medical history includes type 2 diabetes, hyperlipidemia, and a 40-pack-year smoking history.

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy

MEDICATIONS:
Home Medications:
- Amlodipine 5 mg daily
- Lisinopril 20 mg daily
- Hydrochlorothiazide 25 mg daily
- Metformin 1000 mg twice daily
- Aspirin 81 mg daily
- Temazepam 15 mg nightly at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 70
- Mother: Type 2 diabetes mellitus
- Sister: Primary open-angle glaucoma

SOCIAL HISTORY:
She quit smoking last year after developing transient dizziness while walking uphill. She occasionally drinks licorice tea.

REVIEW OF SYSTEMS:
She denies chest pain, dyspnea, or syncope.

VITAL SIGNS:
On physical exam, her BMI is 23 kg/m² and her blood pressure is 170/100 mm Hg in the right arm and 140/90 mm Hg in the left arm.

PHYSICAL EXAMINATION:
Her radial and brachial pulses are symmetric and full, but femoral and dorsalis pedis pulses are decreased bilaterally. There is no carotid bruit. Auscultation reveals a soft abdominal bruit in the right periumbilical area. Funduscopic exam shows mild arteriolar narrowing but no hemorrhages or exudates. She has trace pretibial edema but no signs of heart failure. Her lower extremities show cool skin and diminished hair growth below the knees. The rest of the exam is unremarkable.

OTHER STUDIES:
Ankle-brachial index is 0.7 bilaterally.

## Encounter 1: source 2020-04-15, practice 2022-01-14, outpatient, Psychiatry

Chief complaint: Persistent sadness, loss of interest in activities, and difficulty sleeping for six months following husband's death

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Okonkwo, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent sadness, loss of interest in activities, and difficulty sleeping for six months following husband's death

HISTORY OF PRESENT ILLNESS:
The patient is a 66-year-old female with hypertension, diabetes, hyperlipidemia, chronic insomnia, and recurrent ischemic stroke who returns to clinic following her recent Internal Medicine evaluation for renal artery stenosis. She reports a 6-month history of persistent sadness, loss of interest in activities, and difficulty sleeping. She reports a 15-pound weight loss during this period. Her husband passed away 8 months ago, and since then, she has felt "empty" and has been isolating herself from friends and family. She denies any suicidal ideation.

PAST MEDICAL HISTORY:
Active Problem List:
- Renal artery stenosis (diagnosed 2020-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Chronic insomnia
- Hyperopia
- History of recurrent ischemic stroke with residual mild bilateral weakness

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Amlodipine 5 mg daily
- Lisinopril 20 mg daily
- Metformin 1000 mg twice daily
- Aspirin 81 mg daily
- Temazepam 15 mg nightly at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 70
- Mother: Type 2 diabetes mellitus
- Sister: Primary open-angle glaucoma

SOCIAL HISTORY:
Smoking: Former smoker, quit 1 year ago, 40 pack-year history. Alcohol: None. Occupation: Retired high school teacher.

VITAL SIGNS:
Within normal limits

PHYSICAL EXAMINATION:
Appears fatigued and has poor eye contact

LABORATORY DATA:
- CBC: Normal
- Thyroid-stimulating hormone (TSH): Normal
- Basic metabolic panel: Normal

## Encounter 2: source 2021-01-15, practice 2022-10-10, outpatient, Rheumatology

Chief complaint: Two months of increasing pain and stiffness in neck, shoulders, and hips, worse in the mornings

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. James Peterson, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Two months of increasing pain and stiffness in neck, shoulders, and hips, worse in the mornings

HISTORY OF PRESENT ILLNESS:
The patient with renal artery stenosis and recurrent stroke presents with two months of neck, shoulder, and hip pain and stiffness. She reports that symptoms are worse in the mornings and after periods of inactivity, but improve slightly with movement. She also mentions experiencing mild fatigue and a 4-pound unintentional weight loss. She has no history of joint swelling.

PAST MEDICAL HISTORY:
Active Problem List:
- Renal artery stenosis (diagnosed 2020-01-15)
- Major depressive disorder (diagnosed 2020-04-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Chronic insomnia
- Hyperopia
- History of recurrent ischemic stroke with residual mild bilateral weakness

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy

MEDICATIONS:
Home Medications:
- Amlodipine 5 mg daily
- Lisinopril 20 mg daily
- Hydrochlorothiazide 25 mg daily
- Metformin 1000 mg twice daily
- Aspirin 81 mg daily
- Temazepam 15 mg nightly at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 70
- Mother: Type 2 diabetes mellitus
- Sister: Primary open-angle glaucoma

SOCIAL HISTORY:
Smoking: Former smoker, quit 1 year ago, 40 pack-year history. Alcohol: None. Occupation: Retired high school teacher.

PHYSICAL EXAMINATION:
On physical examination, there is no joint swelling or tenderness, but there is a decreased range of motion in the shoulders and hips due to pain.

LABORATORY DATA:
Laboratory findings are as follows:

- Erythrocyte sedimentation rate (ESR): 70 mm/hr (Normal: 0-22 mm/hr)
- C-reactive protein (CRP): 25 mg/L (Normal: <10 mg/L)
- Rheumatoid factor (RF): Negative
- Antinuclear antibodies (ANA): Negative
- Complete blood count (CBC): Within normal limits

ASSESSMENT AND PLAN:
Based on the clinical presentation and laboratory findings,

## Encounter 3: source 2022-01-15, practice 2023-10-18, inpatient, General Surgery

Chief complaint: Admitted with acute cholecystitis; surgical evaluation after 48 hours of intravenous antibiotics

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Aisha Patel, MD
Department: General Surgery
Visit Type: inpatient

CHIEF COMPLAINT:
Admitted with acute cholecystitis; surgical evaluation after 48 hours of intravenous antibiotics

HISTORY OF PRESENT ILLNESS:
The patient is a 68-year-old female with history of renal artery stenosis, major depressive disorder, and polymyalgia rheumatica who is admitted for surgical evaluation. After 48 hours of intravenous antibiotics, the surgical team recommends a laparoscopic cholecystectomy.

PAST MEDICAL HISTORY:
Active Problem List:
- Renal artery stenosis (diagnosed 2020-01-15)
- Major depressive disorder (diagnosed 2020-04-15)
- Polymyalgia rheumatica (diagnosed 2021-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Chronic insomnia
- Hyperopia
- History of recurrent ischemic stroke with residual mild bilateral weakness

She has a history of hypertension and well-controlled type 2 diabetes.

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy

MEDICATIONS:
Home Medications:
- Amlodipine 5 mg daily
- Lisinopril 20 mg daily
- Hydrochlorothiazide 25 mg daily
- Metformin 1000 mg twice daily
- Aspirin 81 mg daily
- Temazepam 15 mg nightly at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 70
- Mother: Type 2 diabetes mellitus
- Sister: Primary open-angle glaucoma

SOCIAL HISTORY:
Smoking: Former smoker, quit 1 year ago, 40 pack-year history. Alcohol: None. Occupation: Retired high school teacher.

PHYSICAL EXAMINATION:
The patient is alert, oriented, and speaks fluently.

PLAN:
When the surgeon enters the room to discuss the procedure, the patient’s daughter insists on being present and states, “My mother always gets confused when doctors talk. Just explain everything to me, and I’ll decide for her.” The physician politely asks the daughter to leave so he can speak directly to the patient. The patient agrees and listens carefully. She asks appropriate questions about risks and benefits and ultimately consents to the procedure. The daughter later files a complaint, claiming the physician ignored the family’s right to be involved in decision-making.

## Encounter 4: source 2023-01-15, practice 2024-10-08, follow_up, Neurology

Chief complaint: Follow-up visit after recent hospitalization for lacunar stroke with residual left-sided weakness

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Robert Kim, MD
Department: Neurology
Visit Type: follow_up

CHIEF COMPLAINT:
Follow-up visit after recent hospitalization for lacunar stroke with residual left-sided weakness

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up after stroke hospitalization. She has been experiencing mild left-sided weakness and difficulty with coordination since the event.

PAST MEDICAL HISTORY:
Active Problem List:
- Renal artery stenosis (diagnosed 2020-01-15)
- Major depressive disorder (diagnosed 2020-04-15)
- Polymyalgia rheumatica (diagnosed 2021-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Chronic insomnia
- Hyperopia
- History of recurrent ischemic stroke with residual mild bilateral weakness

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy

MEDICATIONS:
Her current medications include metoprolol, lisinopril, and metformin.

Home Medications:
- Amlodipine 5 mg daily
- Hydrochlorothiazide 25 mg daily
- Aspirin 81 mg daily
- Temazepam 15 mg nightly at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 70
- Mother: Type 2 diabetes mellitus
- Sister: Primary open-angle glaucoma

SOCIAL HISTORY:
Her daughter pulls you aside and requests that you do not disclose any findings that might suggest a progressive or debilitating disease, as she fears this will cause her mother undue stress and despair.

VITAL SIGNS:
On examination, her blood pressure is 132/86 mmHg, heart rate is 78 bpm

PHYSICAL EXAMINATION:
She has a mild left-sided hemiparesis

LABORATORY DATA:
Her laboratory values are as follows:

- Hemoglobin A1c: 7.2%
- Fasting blood glucose: 156 mg/dL
- Total cholesterol: 210 mg/dL
- LDL cholesterol: 130 mg/dL
- HDL cholesterol: 45 mg/dL
- Triglycerides: 240 mg/dL

IMAGING:
The patient's recent MRI showed a small, chronic right lacunar infarct.

## Encounter 5: source 2023-04-15, practice 2025-01-13, outpatient, Psychiatry

Chief complaint: Persistent sadness, lack of interest in activities, and fatigue for three months since the stroke

ENCOUNTER NOTE
Date: 2023-04-15
Provider: Dr. Michael Okonkwo, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent sadness, lack of interest in activities, and fatigue for three months since the stroke

HISTORY OF PRESENT ILLNESS:
The patient with history of recurrent ischemic stroke and prior depression presents to the clinic with her daughter three months following her recent lacunar stroke. The daughter reports that her mother has been experiencing persistent sadness, lack of interest in activities, and fatigue for the past three months. She denies any recent changes in medication, diet, or physical activity.

PAST MEDICAL HISTORY:
Active Problem List:
- Renal artery stenosis (diagnosed 2020-01-15)
- Major depressive disorder (diagnosed 2020-04-15)
- Polymyalgia rheumatica (diagnosed 2021-01-15)
- Lacunar stroke (diagnosed 2023-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Chronic insomnia
- Hyperopia
- History of recurrent ischemic stroke with residual mild bilateral weakness

The patient had an ischemic stroke six months ago, which resulted in mild right-sided weakness. Her medical history is significant for hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy

MEDICATIONS:
Her current medications include lisinopril, metformin, and aspirin.

Home Medications:
- Amlodipine 5 mg daily
- Hydrochlorothiazide 25 mg daily
- Temazepam 15 mg nightly at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 70
- Mother: Type 2 diabetes mellitus
- Sister: Primary open-angle glaucoma

SOCIAL HISTORY:
Smoking: Former smoker, quit 1 year ago, 40 pack-year history. Alcohol: None. Occupation: Retired high school teacher.

VITAL SIGNS:
Her blood pressure is 132/86 mmHg, heart rate is 78 bpm, and BMI is 29 kg/m^2.

PHYSICAL EXAMINATION:
On examination, the patient has a flat affect and admits to feelings of hopelessness and occasional thoughts that life is not worth living. Neurological examination shows mild right-sided hemiparesis.

LABORATORY DATA:
Laboratory values are as follows:

- Hemoglobin A1c: 7.2%
- Fasting glucose: 158 mg/dL
- Total cholesterol: 190 mg/dL
- HDL: 45 mg/dL
- LDL: 120 mg/dL
- Triglycerides: 250 mg/dL
- TSH: 2.5 mIU/L

## Encounter 6: source 2024-01-15, practice 2025-10-12, ed, Emergency Medicine

Chief complaint: Severe right eye pain, headache, nausea, and blurred vision with halos around lights for four hours

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Emily Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe right eye pain, headache, nausea, and blurred vision with halos around lights for four hours

HISTORY OF PRESENT ILLNESS:
This 70-year-old with hyperopia and stroke presents with eye pain. She describes the pain as throbbing and states that it has progressively worsened. She also reports blurred vision and halos around lights.

PAST MEDICAL HISTORY:
Active Problem List:
- Renal artery stenosis (diagnosed 2020-01-15)
- Major depressive disorder (diagnosed 2020-04-15)
- Polymyalgia rheumatica (diagnosed 2021-01-15)
- Lacunar stroke (diagnosed 2023-01-15)
- Post-stroke depression (diagnosed 2023-04-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Chronic insomnia
- Hyperopia
- History of recurrent ischemic stroke with residual mild bilateral weakness

Her past medical history includes hypertension and hyperopia.

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Amlodipine 5 mg daily
- Lisinopril 20 mg daily
- Metformin 1000 mg twice daily
- Aspirin 81 mg daily
- Temazepam 15 mg nightly at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 70
- Mother: Type 2 diabetes mellitus
- Sister: Primary open-angle glaucoma

SOCIAL HISTORY:
Smoking: Former smoker, quit 1 year ago, 40 pack-year history. Alcohol: None. Occupation: Retired high school teacher.

PHYSICAL EXAMINATION:
On physical examination, her right eye is red, the cornea appears cloudy, and the pupil is mid-dilated and non-reactive to light. Visual acuity is decreased in the affected eye. The intraocular pressure (IOP) of the right eye is measured at 48 mmHg, while the left eye has an IOP of 16 mmHg.

OTHER STUDIES:
Gonioscopy is not immediately available.

## Encounter 7: source 2024-02-15, practice 2025-11-25, ed, Emergency Medicine

Chief complaint: Acute onset confusion and unsteadiness on feet over the past two days

ENCOUNTER NOTE
Date: 2024-02-15
Provider: Dr. David Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset confusion and unsteadiness on feet over the past two days

HISTORY OF PRESENT ILLNESS:
This 70-year-old with recurrent stroke and chronic insomnia on temazepam is brought to the emergency department by her daughter because of increasing confusion and unsteadiness on her feet over the past two days.

PAST MEDICAL HISTORY:
Active Problem List:
- Renal artery stenosis (diagnosed 2020-01-15)
- Major depressive disorder (diagnosed 2020-04-15)
- Polymyalgia rheumatica (diagnosed 2021-01-15)
- Lacunar stroke (diagnosed 2023-01-15)
- Post-stroke depression (diagnosed 2023-04-15)
- Acute angle-closure glaucoma (diagnosed 2024-01-15)
- Hypertension
- Type 2 Diabetes Mellitus
- Hyperlipidemia
- Chronic insomnia
- Hyperopia
- History of recurrent ischemic stroke with residual mild bilateral weakness

The patient has a history of hypertension, type 2 diabetes mellitus, and chronic insomnia.

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy

MEDICATIONS:
Her current medications include metformin, lisinopril, and a nightly temazepam that was prescribed 3 weeks ago.

Home Medications:
- Amlodipine 5 mg daily
- Hydrochlorothiazide 25 mg daily
- Aspirin 81 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 70
- Mother: Type 2 diabetes mellitus
- Sister: Primary open-angle glaucoma

SOCIAL HISTORY:
Smoking: Former smoker, quit 1 year ago, 40 pack-year history. Alcohol: None. Occupation: Retired high school teacher.

VITAL SIGNS:
Her blood pressure is 145/85 mmHg, pulse is 78/min, and random blood glucose is 180 mg/dL.

PHYSICAL EXAMINATION:
On examination, the patient is alert but oriented to person only. She has a slow, shuffling gait and uses the wall for support.
There is no evidence of head trauma, and her neurological examination is nonfocal.
