# synthetic_hospital patient 2263

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 67 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-08-29 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-04-15 | 2022-12-07 (Wed) | outpatient | Cardiology |
| 2 | 2020-07-15 | 2023-02-28 (Tue) | follow_up | Cardiology |
| 3 | 2020-08-15 | 2023-03-22 (Wed) | ed | Emergency Medicine |
| 4 | 2021-07-15 | 2024-03-01 (Fri) | ed | Emergency Medicine |
| 5 | 2022-07-15 | 2025-02-26 (Wed) | ed | Emergency Medicine |
| 6 | 2023-01-15 | 2025-09-04 (Thu) | ed | Emergency Medicine |
| 7 | 2023-07-15 | 2026-02-19 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Handyman (self-employed)",
 "alcohol_use": "Social (1-2 drinks per month)",
 "family_history": [
  "Father: Myocardial infarction at age 58",
  "Mother: Osteoarthritis",
  "Brother: Hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 10 years ago (20 pack-year history)",
 "home_medications": [
  {
   "dose": "81 mg oral daily",
   "name": "Aspirin"
  },
  {
   "dose": "75 mg oral daily",
   "name": "Clopidogrel"
  },
  {
   "dose": "20 mg oral daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg oral daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "50 mg oral daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "40 mg oral daily",
   "name": "Furosemide"
  },
  {
   "dose": "40 mg oral daily",
   "name": "Pantoprazole"
  },
  {
   "dose": "25 mg oral three times daily as needed for joint pain",
   "name": "Indomethacin"
  }
 ],
 "surgical_history": [
  "Percutaneous coronary intervention with drug-eluting stent placement (proximal LAD)",
  "Diagnostic cardiac catheterization"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Coronary artery disease",
  "Heart failure with reduced ejection fraction",
  "Left ventricular aneurysm (status post myocardial infarction)",
  "Osteoarthritis (shoulders, elbows, cervical spine)",
  "Cervical spondylosis",
  "History of duodenal ulcer disease (H. pylori eradicated)"
 ],
 "age_at_first_encounter": 67
}
```

Primary diagnoses: ["Heart failure with reduced ejection fraction", "Left ventricular aneurysm", "Acute pulmonary edema due to left heart failure (increased pulmonary capillary hydrostatic pressure)", "Dressler syndrome", "Osteoarthritis", "Bleeding duodenal ulcer", "Neuropathic pain syndrome", "Status post percutaneous coronary intervention"]
Comorbidities: ["Essential hypertension", "Old myocardial infarction"]

## Encounter 0: source 2020-01-15, practice 2022-08-29, outpatient, Internal Medicine

Chief complaint: Gradually worsening discomfort in both shoulders and elbows over the past 2 years.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Lisa Thompson, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Gradually worsening discomfort in both shoulders and elbows over the past 2 years.

HISTORY OF PRESENT ILLNESS:
The pain is worse in the evenings after his work as a handyman and improves somewhat with rest. He reports mild stiffness in the morning that lasts only a few minutes before improving. He denies fevers, rash, weight loss, or recent infections. There is no history of joint trauma or inflammatory arthritis.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Coronary artery disease
- Heart failure with reduced ejection fraction
- Left ventricular aneurysm (status post myocardial infarction)
- Osteoarthritis (shoulders, elbows, cervical spine)
- Cervical spondylosis
- History of duodenal ulcer disease (H. pylori eradicated)

His medical history includes controlled hypertension.

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (proximal LAD)
- Diagnostic cardiac catheterization

MEDICATIONS:
Home Medications:
- Aspirin 81 mg oral daily
- Clopidogrel 75 mg oral daily
- Lisinopril 20 mg oral daily
- Atorvastatin 40 mg oral daily
- Metoprolol succinate 50 mg oral daily
- Furosemide 40 mg oral daily
- Pantoprazole 40 mg oral daily
- Indomethacin 25 mg oral three times daily as needed for joint pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 58
- Mother: Osteoarthritis
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago (20 pack-year history). Alcohol: Social (1-2 drinks per month). Occupation: Handyman (self-employed).

PHYSICAL EXAMINATION:
Examination shows crepitus with passive movement of the shoulders and elbows, without warmth or significant swelling. There is mild limitation in the range of motion due to discomfort. Strength is preserved, and sensation is intact. No nodules or skin lesions are present.

LABORATORY DATA:
Laboratory studies, including ESR and CRP, are within normal limits.

ASSESSMENT AND PLAN:
The involved joint is most likely to show

## Encounter 1: source 2020-04-15, practice 2022-12-07, outpatient, Cardiology

Chief complaint: Worsening shortness of breath and fatigue with persistent cough and leg swelling.

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening shortness of breath and fatigue with persistent cough and leg swelling.

HISTORY OF PRESENT ILLNESS:
Returning three months after his prior visit, he reports a persistent cough, especially at night, and has noticed swelling in his legs.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis (diagnosed 2020-01-15)
- Hypertension
- Coronary artery disease
- Heart failure with reduced ejection fraction
- Left ventricular aneurysm (status post myocardial infarction)
- Osteoarthritis (shoulders, elbows, cervical spine)
- Cervical spondylosis
- History of duodenal ulcer disease (H. pylori eradicated)

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (proximal LAD)
- Diagnostic cardiac catheterization

MEDICATIONS:
He takes aspirin, lisinopril, and atorvastatin.

Home Medications:
- Clopidogrel 75 mg oral daily
- Metoprolol succinate 50 mg oral daily
- Furosemide 40 mg oral daily
- Pantoprazole 40 mg oral daily
- Indomethacin 25 mg oral three times daily as needed for joint pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 58
- Mother: Osteoarthritis
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago (20 pack-year history). Alcohol: Social (1-2 drinks per month). Occupation: Handyman (self-employed).

VITAL SIGNS:
Blood pressure is 130/85 mmHg, heart rate is 88 beats per minute, and respiratory rate is 18 breaths per minute

PHYSICAL EXAMINATION:
Cardiac auscultation reveals a holosystolic murmur at the apex radiating to the axilla. Lung auscultation is notable for bibasilar crackles. Extremities show 2+ pitting edema.

IMAGING:
An echocardiogram demonstrates a dilated left ventricle with a dyskinetic area in the anterior wall.

OTHER STUDIES:
An ECG shows persistent ST-segment elevation and deep Q waves in the anterior leads.

## Encounter 2: source 2020-07-15, practice 2023-02-28, follow_up, Cardiology

Chief complaint: Follow-up visit 6 weeks after percutaneous coronary intervention with drug-eluting stent placement.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Cardiology
Visit Type: follow_up

CHIEF COMPLAINT:
Follow-up visit 6 weeks after percutaneous coronary intervention with drug-eluting stent placement.

HISTORY OF PRESENT ILLNESS:
This 67-year-old male with coronary artery disease, heart failure with reduced ejection fraction, and left ventricular aneurysm returns for follow-up six weeks after percutaneous coronary intervention with placement of a drug-eluting stent in the proximal left anterior descending artery for unstable angina, following his April 2020 evaluation for worsening heart failure symptoms. He reports no chest pain or dyspnea and is adherent to his dual antiplatelet therapy. His blood pressure and heart rate are well controlled. Review of the stent placement report notes that the device was coated with paclitaxel.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis (diagnosed 2020-01-15)
- Left ventricular aneurysm (diagnosed 2020-04-15)
- Hypertension
- Coronary artery disease
- Heart failure with reduced ejection fraction
- Left ventricular aneurysm (status post myocardial infarction)
- Osteoarthritis (shoulders, elbows, cervical spine)
- Cervical spondylosis
- History of duodenal ulcer disease (H. pylori eradicated)

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (proximal LAD)
- Diagnostic cardiac catheterization

MEDICATIONS:
Home Medications:
- Aspirin 81 mg oral daily
- Clopidogrel 75 mg oral daily
- Lisinopril 20 mg oral daily
- Atorvastatin 40 mg oral daily
- Metoprolol succinate 50 mg oral daily
- Furosemide 40 mg oral daily
- Pantoprazole 40 mg oral daily
- Indomethacin 25 mg oral three times daily as needed for joint pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 58
- Mother: Osteoarthritis
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago (20 pack-year history). Alcohol: Social (1-2 drinks per month). Occupation: Handyman (self-employed).

## Encounter 3: source 2020-08-15, practice 2023-03-22, ed, Emergency Medicine

Chief complaint: Pleuritic chest pain and low-grade fevers for 2 weeks.

ENCOUNTER NOTE
Date: 2020-08-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Pleuritic chest pain and low-grade fevers for 2 weeks.

HISTORY OF PRESENT ILLNESS:
This 67-year-old male with coronary artery disease and left ventricular aneurysm presents to the emergency department ten weeks after percutaneous coronary intervention with a 2-week history of pleuritic chest pain and low-grade fevers. He reports that the pain is sharp, worsens with deep breaths, and is relieved by leaning forward.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis (diagnosed 2020-01-15)
- Left ventricular aneurysm (diagnosed 2020-04-15)
- Status post percutaneous coronary intervention (diagnosed 2020-07-15)
- Hypertension
- Coronary artery disease
- Heart failure with reduced ejection fraction
- Left ventricular aneurysm (status post myocardial infarction)
- Osteoarthritis (shoulders, elbows, cervical spine)
- Cervical spondylosis
- History of duodenal ulcer disease (H. pylori eradicated)

His medical history is significant for a myocardial infarction 6 weeks ago, for which he underwent percutaneous coronary intervention.

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (proximal LAD)
- Diagnostic cardiac catheterization

MEDICATIONS:
Home Medications:
- Aspirin 81 mg oral daily
- Clopidogrel 75 mg oral daily
- Lisinopril 20 mg oral daily
- Atorvastatin 40 mg oral daily
- Metoprolol succinate 50 mg oral daily
- Furosemide 40 mg oral daily
- Pantoprazole 40 mg oral daily
- Indomethacin 25 mg oral three times daily as needed for joint pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 58
- Mother: Osteoarthritis
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago (20 pack-year history). Alcohol: Social (1-2 drinks per month). Occupation: Handyman (self-employed).

VITAL SIGNS:
His vital signs are stable, and there are no signs of hemodynamic instability.

PHYSICAL EXAMINATION:
On physical examination, a pericardial friction rub is auscultated.

LABORATORY DATA:
Laboratory findings are as follows:

- White blood cell count: 12,000 /µL
- Hemoglobin: 14.2 g/dL
- Erythrocyte sedimentation rate (ESR): 55 mm/hr
- C-reactive protein (CRP): 25 mg/L
- Troponin I: <0.04 ng/mL (normal)

IMAGING:
Echocardiography reveals a small pericardial effusion without evidence of tamponade.

OTHER STUDIES:
An ECG shows diffuse ST-segment elevations and PR depressions.

## Encounter 4: source 2021-07-15, practice 2024-03-01, ed, Emergency Medicine

Chief complaint: Neck pain and hand weakness after rear-end collision.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. James Washington, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Neck pain and hand weakness after rear-end collision.

HISTORY OF PRESENT ILLNESS:
The patient is a 68-year-old male with cervical spondylosis and coronary artery disease who presents to the emergency department after being involved in a rear-end collision while driving. He denies loss of consciousness or lower extremity weakness.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis (diagnosed 2020-01-15)
- Left ventricular aneurysm (diagnosed 2020-04-15)
- Status post percutaneous coronary intervention (diagnosed 2020-07-15)
- Dressler syndrome (diagnosed 2020-08-15)
- Hypertension
- Coronary artery disease
- Heart failure with reduced ejection fraction
- Left ventricular aneurysm (status post myocardial infarction)
- Osteoarthritis (shoulders, elbows, cervical spine)
- Cervical spondylosis
- History of duodenal ulcer disease (H. pylori eradicated)

His past medical history is significant for hypertension and cervical spondylosis.

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (proximal LAD)
- Diagnostic cardiac catheterization

MEDICATIONS:
Home Medications:
- Aspirin 81 mg oral daily
- Clopidogrel 75 mg oral daily
- Lisinopril 20 mg oral daily
- Atorvastatin 40 mg oral daily
- Metoprolol succinate 50 mg oral daily
- Furosemide 40 mg oral daily
- Pantoprazole 40 mg oral daily
- Indomethacin 25 mg oral three times daily as needed for joint pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 58
- Mother: Osteoarthritis
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago (20 pack-year history). Alcohol: Social (1-2 drinks per month). Occupation: Handyman (self-employed).

PHYSICAL EXAMINATION:
On physical examination, the patient has a reduced range of motion in his neck due to pain. Neurological examination reveals decreased grip strength and impaired fine motor skills in both hands. Sensation to light touch and pinprick is diminished in the upper extremities, more pronounced in the hands. Reflexes are diminished in the biceps and brachioradialis but are normal in the lower extremities. There is no bowel or bladder incontinence.

IMAGING:
An MRI of the cervical spine shows no evidence of acute fracture but reveals chronic degenerative changes.

## Encounter 5: source 2022-07-15, practice 2025-02-26, ed, Emergency Medicine

Chief complaint: Progressive shortness of breath over 2 weeks with orthopnea and paroxysmal nocturnal dyspnea.

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive shortness of breath over 2 weeks with orthopnea and paroxysmal nocturnal dyspnea.

HISTORY OF PRESENT ILLNESS:
This 69-year-old male with heart failure with reduced ejection fraction and left ventricular aneurysm status post myocardial infarction presents with progressive cardiopulmonary symptoms. He states that over the past two weeks, he has had increasing difficulty climbing stairs or walking short distances without becoming breathless. He also reports waking up in the middle of the night gasping for air and now needs to sleep propped up on three pillows. He denies chest pain, fever, or recent illness.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis (diagnosed 2020-01-15)
- Left ventricular aneurysm (diagnosed 2020-04-15)
- Status post percutaneous coronary intervention (diagnosed 2020-07-15)
- Dressler syndrome (diagnosed 2020-08-15)
- Neuropathic pain syndrome (diagnosed 2021-07-15)
- Hypertension
- Coronary artery disease
- Heart failure with reduced ejection fraction
- Left ventricular aneurysm (status post myocardial infarction)
- Osteoarthritis (shoulders, elbows, cervical spine)
- Cervical spondylosis
- History of duodenal ulcer disease (H. pylori eradicated)

His past medical history includes hypertension and a recent anterior wall myocardial infarction 3 months ago, which was treated medically.

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (proximal LAD)
- Diagnostic cardiac catheterization

MEDICATIONS:
Home Medications:
- Aspirin 81 mg oral daily
- Clopidogrel 75 mg oral daily
- Lisinopril 20 mg oral daily
- Atorvastatin 40 mg oral daily
- Metoprolol succinate 50 mg oral daily
- Furosemide 40 mg oral daily
- Pantoprazole 40 mg oral daily
- Indomethacin 25 mg oral three times daily as needed for joint pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 58
- Mother: Osteoarthritis
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago (20 pack-year history). Alcohol: Social (1-2 drinks per month). Occupation: Handyman (self-employed).

VITAL SIGNS:
His temperature is 36.9°C (98.4°F), blood pressure 102/66 mm Hg, heart rate 108/min, respiratory rate 24/min, and oxygen saturation is 89% on room air.

PHYSICAL EXAMINATION:
On exam, he appears mildly distressed and is using accessory muscles of respiration. Lung auscultation reveals bilateral late inspiratory crackles at the bases, and there is no wheezing. Cardiac examination reveals an S3 gallop. There is mild bilateral lower extremity pitting edema.

IMAGING:
Chest x-ray shows bilateral perihilar opacities consistent with pulmonary edema.

OTHER STUDIES:
A Swan-Ganz catheterization reveals an elevated pulmonary capillary wedge pressure.

## Encounter 6: source 2023-01-15, practice 2025-09-04, ed, Emergency Medicine

Chief complaint: Progressive shortness of breath with reduced appetite and leg swelling.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Jennifer Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive shortness of breath with reduced appetite and leg swelling.

HISTORY OF PRESENT ILLNESS:
This 70-year-old male with known heart failure with reduced ejection fraction and left ventricular aneurysm, previously hospitalized in July 2022 for similar symptoms, presents to the emergency department with progressive shortness of breath that has worsened over the past several days. He reports waking up at night gasping for air and needing to sleep propped up on several pillows. He denies chest pain, cough, or fever but mentions reduced appetite and mild leg swelling.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis (diagnosed 2020-01-15)
- Left ventricular aneurysm (diagnosed 2020-04-15)
- Status post percutaneous coronary intervention (diagnosed 2020-07-15)
- Dressler syndrome (diagnosed 2020-08-15)
- Neuropathic pain syndrome (diagnosed 2021-07-15)
- Heart failure with reduced ejection fraction (diagnosed 2022-07-15)
- Hypertension
- Coronary artery disease
- Left ventricular aneurysm (status post myocardial infarction)
- Osteoarthritis (shoulders, elbows, cervical spine)
- Cervical spondylosis
- History of duodenal ulcer disease (H. pylori eradicated)

His medical history includes long-standing hypertension and a prior anterior wall myocardial infarction several years ago.

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (proximal LAD)
- Diagnostic cardiac catheterization

MEDICATIONS:
Home Medications:
- Aspirin 81 mg oral daily
- Clopidogrel 75 mg oral daily
- Lisinopril 20 mg oral daily
- Atorvastatin 40 mg oral daily
- Metoprolol succinate 50 mg oral daily
- Furosemide 40 mg oral daily
- Pantoprazole 40 mg oral daily
- Indomethacin 25 mg oral three times daily as needed for joint pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 58
- Mother: Osteoarthritis
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago (20 pack-year history). Alcohol: Social (1-2 drinks per month). Occupation: Handyman (self-employed).

PHYSICAL EXAMINATION:
On examination, he appears fatigued and dyspneic at rest. Cardiac auscultation reveals a third heart sound, and pulmonary examination reveals bibasilar crackles without wheezing. Jugular venous pressure is elevated while peripheral pulses are weak.

IMAGING:
A chest radiograph shows increased vascular markings and small bilateral pleural effusions.

OTHER STUDIES:
A pulmonary artery catheterization is performed for further hemodynamic monitoring.

## Encounter 7: source 2023-07-15, practice 2026-02-19, ed, Emergency Medicine

Chief complaint: Progressive fatigue, lightheadedness, and melena over 2 weeks.

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Maria Garcia, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive fatigue, lightheadedness, and melena over 2 weeks.

HISTORY OF PRESENT ILLNESS:
The patient is a 70-year-old male with coronary artery disease, heart failure with reduced ejection fraction, and prior duodenal ulcer disease presenting with progressive fatigue, lightheadedness, and melena over two weeks. He had acute pulmonary edema six months ago. He reports getting winded while climbing stairs and feels dizzy when rising from a chair. He has also noticed that his stools have turned dark and sticky, which he describes as "tarry." He denies vomiting, abdominal pain, or visible blood in the stool. He has no fever, chills, weight loss, or recent changes in appetite, though he has felt mildly nauseated and has been eating less than usual. He attributes his symptoms to "old age and arthritis slowing him down."

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis (diagnosed 2020-01-15)
- Left ventricular aneurysm (diagnosed 2020-04-15)
- Status post percutaneous coronary intervention (diagnosed 2020-07-15)
- Dressler syndrome (diagnosed 2020-08-15)
- Neuropathic pain syndrome (diagnosed 2021-07-15)
- Heart failure with reduced ejection fraction (diagnosed 2022-07-15)
- Acute pulmonary edema due to left heart failure (increased pulmonary capillary hydrostatic pressure) (diagnosed 2023-01-15)
- Hypertension
- Coronary artery disease
- Left ventricular aneurysm (status post myocardial infarction)
- Osteoarthritis (shoulders, elbows, cervical spine)
- Cervical spondylosis
- History of duodenal ulcer disease (H. pylori eradicated)

His medical history includes hypertension, osteoarthritis, and a duodenal ulcer diagnosed several years ago during an evaluation for melena. He was treated for Helicobacter pylori infection and completed a course of acid suppression therapy. Since then, he has not had any recurrence of ulcer symptoms.

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (proximal LAD)
- Diagnostic cardiac catheterization

MEDICATIONS:
His medications include lisinopril and indomethacin for chronic joint pain. He manages his medications himself and does not routinely review them with his doctor.

Home Medications:
- Aspirin 81 mg oral daily
- Clopidogrel 75 mg oral daily
- Atorvastatin 40 mg oral daily
- Metoprolol succinate 50 mg oral daily
- Furosemide 40 mg oral daily
- Pantoprazole 40 mg oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 58
- Mother: Osteoarthritis
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 10 years ago (20 pack-year history). Alcohol: Social (1-2 drinks per month). Occupation: Handyman (self-employed).

VITAL SIGNS:
His blood pressure is 108/66 mm Hg, pulse is 98/min, and respiratory rate is 16/min.

PHYSICAL EXAMINATION:
On examination, the patient appears tired and pale. Cardiopulmonary examination is unremarkable. Abdominal examination reveals mild epigastric tenderness without rebound or guarding. Rectal examination reveals black, tarry stool that is positive for occult blood. He has no abdominal masses, hepatosplenomegaly, or signs of chronic liver disease.

LABORATORY DATA:
Laboratory studies show:
Hemoglobin                                 8.2 g/dL
MCV                                           82 fL
BUN                                           38 mg/dL
Creatinine                                  1.1 mg/dL
Platelets                                     240,000/mm³
INR                                           1.0
