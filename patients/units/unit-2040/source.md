# synthetic_hospital patient 2040

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 44 at first encounter, sex M, Non-Hispanic White, Employer-sponsored private health insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-05-23 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-04-15 | 2022-09-06 (Tue) | outpatient | Psychiatry |
| 2 | 2020-07-15 | 2022-11-23 (Wed) | outpatient | Infectious Disease |
| 3 | 2021-01-15 | 2023-06-12 (Mon) | outpatient | Cardiology |
| 4 | 2022-07-15 | 2024-12-06 (Fri) | ed | Emergency Medicine |
| 5 | 2023-01-15 | 2025-05-23 (Fri) | ed | Emergency Medicine |
| 6 | 2023-07-15 | 2025-12-02 (Tue) | ed | Emergency Medicine |
| 7 | 2024-01-15 | 2026-05-25 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Employer-sponsored private health insurance",
 "occupation": "Accountant",
 "alcohol_use": "None (formerly occasional)",
 "family_history": [
  "Father: premature myocardial infarction at age 55",
  "Mother: hypertension and hyperlipidemia",
  "Sibling: coronary artery disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "10 mg orally daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg orally nightly",
   "name": "Atorvastatin"
  },
  {
   "dose": "81 mg orally daily",
   "name": "Aspirin"
  },
  {
   "dose": "800/100 mg orally daily",
   "name": "Darunavir/Ritonavir"
  },
  {
   "dose": "200/25 mg orally daily",
   "name": "Emtricitabine/Tenofovir alafenamide"
  }
 ],
 "surgical_history": [
  "Percutaneous coronary intervention with drug-eluting stent placement (prior inferior STEMI)",
  "Conservative management of left-sided rib fractures"
 ],
 "chronic_conditions": [
  "Essential hypertension",
  "Hyperlipidemia",
  "HIV infection",
  "Pathological gambling disorder",
  "Coronary artery disease"
 ],
 "age_at_first_encounter": 44
}
```

Primary diagnoses: ["Pathological Gambling", "ST-segment elevation myocardial infarction, inferior wall", "Hypertensive emergency", "HIV Protease Inhibitor-Associated Lipodystrophy Syndrome", "Fracture of two ribs, left side", "Hydrochlorothiazide adverse reaction", "Ureterolithiasis", "Stable Angina Pectoris due to Coronary Artery Disease"]
Comorbidities: ["Hyperlipidemia", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2022-05-23, outpatient, Internal Medicine

Chief complaint: Routine follow-up visit for recently diagnosed essential hypertension with medication side effects

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up visit for recently diagnosed essential hypertension with medication side effects

HISTORY OF PRESENT ILLNESS:
He was diagnosed with essential hypertension three months ago and was started on hydrochlorothiazide at that time. He reports improvement in his home blood pressure readings but has recently noticed increased muscle cramps, occasional lightheadedness when standing, and constipation. He denies chest pain, palpitations, or weight changes.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension
- Hyperlipidemia
- HIV infection
- Pathological gambling disorder
- Coronary artery disease

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (prior inferior STEMI)
- Conservative management of left-sided rib fractures

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg orally daily
- Atorvastatin 40 mg orally nightly
- Aspirin 81 mg orally daily
- Darunavir/Ritonavir 800/100 mg orally daily
- Emtricitabine/Tenofovir alafenamide 200/25 mg orally daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: premature myocardial infarction at age 55
- Mother: hypertension and hyperlipidemia
- Sibling: coronary artery disease

SOCIAL HISTORY:
He follows a low-sodium diet and drinks approximately 2 liters of water daily. He does not smoke and drinks alcohol only occasionally.

VITAL SIGNS:
Blood pressure is 124/76 mm Hg.

PHYSICAL EXAMINATION:
Physical examination shows normal heart sounds, no murmurs, and clear lungs.

LABORATORY DATA:
Urinalysis is negative for protein, glucose, or blood.

OTHER STUDIES:
ECG shows flattening of T waves and the presence of U waves.

## Encounter 1: source 2020-04-15, practice 2022-09-06, outpatient, Psychiatry

Chief complaint: Wife expresses concern about deteriorating financial situation and patient's recent secretive behavior

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Wife expresses concern about deteriorating financial situation and patient's recent secretive behavior

HISTORY OF PRESENT ILLNESS:
The patient is a 44-year-old male with essential hypertension, hyperlipidemia, HIV infection, and coronary artery disease who returns for follow-up evaluation of his gambling behaviors. He admits to spending significant amounts of time at a local casino, often lying about his whereabouts. He reports that he feels a constant urge to gamble, and despite recent substantial financial losses, he believes he can win back the money. He has taken multiple loans and maxed out several credit cards.

PAST MEDICAL HISTORY:
Active Problem List:
- Hydrochlorothiazide adverse reaction (diagnosed 2020-01-15)
- Essential hypertension
- Hyperlipidemia
- HIV infection
- Pathological gambling disorder
- Coronary artery disease

The patient's medical history is notable for hypertension.

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (prior inferior STEMI)
- Conservative management of left-sided rib fractures

MEDICATIONS:
He takes he takes hydrochlorothiazide.

Home Medications:
- Lisinopril 10 mg orally daily
- Atorvastatin 40 mg orally nightly
- Aspirin 81 mg orally daily
- Darunavir/Ritonavir 800/100 mg orally daily
- Emtricitabine/Tenofovir alafenamide 200/25 mg orally daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: premature myocardial infarction at age 55
- Mother: hypertension and hyperlipidemia
- Sibling: coronary artery disease

SOCIAL HISTORY:
He denies any use of alcohol or illicit drugs.

VITAL SIGNS:
His blood pressure is 142/88 mmHg.

PHYSICAL EXAMINATION:
On physical examination, the patient appears restless and anxious. The remainder of the physical examination is unremarkable.

LABORATORY DATA:
Laboratory Values:
- Complete Blood Count (CBC): Within normal limits
- Liver Function Tests (LFTs): Within normal limits
- Urine Toxicology Screen: Negative

## Encounter 2: source 2020-07-15, practice 2022-11-23, outpatient, Infectious Disease

Chief complaint: Follow-up visit four months after HIV diagnosis with body habitus changes

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Priya Patel, MD
Department: Infectious Disease
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up visit four months after HIV diagnosis with body habitus changes

HISTORY OF PRESENT ILLNESS:
The patient is a 44-year-old male with essential hypertension, hyperlipidemia, coronary artery disease, and HIV infection diagnosed four months prior, returning for follow-up evaluation of body habitus changes. He was recently seen in Psychiatry for pathological gambling and has a history of hydrochlorothiazide adverse reaction. He reports that since starting his new medication regimen, he has noticed significant changes in his body. He says his face has become rounder, and he has gained fat in his upper back while losing fat in his arms and legs. He also mentions that his clothes no longer fit properly, and he feels self-conscious about the redistribution of body fat. He has also been more fatigued and sometimes feels unusually thirsty. He has no history of opportunistic infections, and he has been compliant with his therapy.

PAST MEDICAL HISTORY:
Active Problem List:
- Hydrochlorothiazide adverse reaction (diagnosed 2020-01-15)
- Pathological Gambling (diagnosed 2020-04-15)
- Essential hypertension
- Hyperlipidemia
- HIV infection
- Pathological gambling disorder
- Coronary artery disease

His other past medical history is notable only for well-controlled hypertension.

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (prior inferior STEMI)
- Conservative management of left-sided rib fractures

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg orally daily
- Atorvastatin 40 mg orally nightly
- Aspirin 81 mg orally daily
- Darunavir/Ritonavir 800/100 mg orally daily
- Emtricitabine/Tenofovir alafenamide 200/25 mg orally daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: premature myocardial infarction at age 55
- Mother: hypertension and hyperlipidemia
- Sibling: coronary artery disease

SOCIAL HISTORY:
He works as an accountant and does not use alcohol, tobacco, or illicit drugs. He is in a stable relationship and practices safe sex.

VITAL SIGNS:
On exam, he is afebrile, and his vital signs are normal.

PHYSICAL EXAMINATION:
He has a rounded "moon face" and fat accumulation over the posterior neck. His extremities appear thinner compared to central areas. Cardiovascular and pulmonary exams are normal. Abdominal exam reveals no organomegaly. Neurological exam is intact with no focal deficits.

LABORATORY DATA:
Laboratory workup from his last visit noted elevated fasting glucose.

## Encounter 3: source 2021-01-15, practice 2023-06-12, outpatient, Cardiology

Chief complaint: Intermittent chest pain occurring with exertion and resolving with rest

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. James O'Brien, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent chest pain occurring with exertion and resolving with rest

HISTORY OF PRESENT ILLNESS:
Patient with coronary artery disease and HIV infection returns. He reports pain that occurs during exercise and resolves with rest.

PAST MEDICAL HISTORY:
Active Problem List:
- Hydrochlorothiazide adverse reaction (diagnosed 2020-01-15)
- Pathological Gambling (diagnosed 2020-04-15)
- HIV Protease Inhibitor-Associated Lipodystrophy Syndrome (diagnosed 2020-07-15)
- Essential hypertension
- Hyperlipidemia
- HIV infection
- Pathological gambling disorder
- Coronary artery disease

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (prior inferior STEMI)
- Conservative management of left-sided rib fractures

MEDICATIONS:
He takes lisinopril and atorvastatin.

Home Medications:
- Aspirin 81 mg orally daily
- Darunavir/Ritonavir 800/100 mg orally daily
- Emtricitabine/Tenofovir alafenamide 200/25 mg orally daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
No family history of coronary artery disease

SOCIAL HISTORY:
He does not smoke

VITAL SIGNS:
Blood pressure of 135/85 mmHg, heart rate of 78 bpm

PHYSICAL EXAMINATION:
No cardiac murmurs

IMAGING:
Subsequent cardiac catheterization shows a 70% stenosis of the left anterior descending artery

OTHER STUDIES:
An exercise stress test is performed, and the patient develops chest pain and ST-segment depression at a high level of exertion

PLAN:
The patient is enrolled in a clinical trial comparing a new anti-anginal medication to placebo. After 6 months, the trial results show a p-value of 0.03 when comparing the frequency of angina episodes between the new medication and placebo groups. The significance level α was set at 5% prior to the study.

## Encounter 4: source 2022-07-15, practice 2024-12-06, ed, Emergency Medicine

Chief complaint: Sudden onset of severe chest pain radiating to left arm while mowing lawn

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Emily Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of severe chest pain radiating to left arm while mowing lawn

HISTORY OF PRESENT ILLNESS:
This 46-year-old male with coronary artery disease, HIV infection, and essential hypertension is brought to the emergency department by his wife due to sudden onset of severe chest pain radiating to his left arm, which started while he was mowing the lawn.

PAST MEDICAL HISTORY:
Active Problem List:
- Hydrochlorothiazide adverse reaction (diagnosed 2020-01-15)
- Pathological Gambling (diagnosed 2020-04-15)
- HIV Protease Inhibitor-Associated Lipodystrophy Syndrome (diagnosed 2020-07-15)
- Stable Angina Pectoris due to Coronary Artery Disease (diagnosed 2021-01-15)
- Essential hypertension
- Hyperlipidemia
- HIV infection
- Pathological gambling disorder
- Coronary artery disease

The patient has a history of hypertension and hyperlipidemia.

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (prior inferior STEMI)
- Conservative management of left-sided rib fractures

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg orally daily
- Atorvastatin 40 mg orally nightly
- Aspirin 81 mg orally daily
- Darunavir/Ritonavir 800/100 mg orally daily
- Emtricitabine/Tenofovir alafenamide 200/25 mg orally daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: premature myocardial infarction at age 55
- Mother: hypertension and hyperlipidemia
- Sibling: coronary artery disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (formerly occasional). Occupation: Accountant.

VITAL SIGNS:
His blood pressure is 160/90 mmHg, heart rate is 110 bpm, and respiratory rate is 22 breaths/min.

PHYSICAL EXAMINATION:
He appears diaphoretic and in distress.

LABORATORY DATA:
Troponin I level is elevated at 4.2 ng/mL (normal <0.04 ng/mL).

OTHER STUDIES:
An ECG shows ST-segment elevation in leads II, III, and aVF.

PLAN:
The patient is treated with aspirin, clopidogrel, and heparin. He is scheduled for an urgent cardiac catheterization.

## Encounter 5: source 2023-01-15, practice 2025-05-23, ed, Emergency Medicine

Chief complaint: Severe left-sided chest pain following fall from ladder

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe left-sided chest pain following fall from ladder

HISTORY OF PRESENT ILLNESS:
This 47-year-old male with coronary artery disease status post inferior wall myocardial infarction presents with chest pain following a fall from a ladder. He reports that the pain worsens with deep breaths and coughing.

PAST MEDICAL HISTORY:
Active Problem List:
- Hydrochlorothiazide adverse reaction (diagnosed 2020-01-15)
- Pathological Gambling (diagnosed 2020-04-15)
- HIV Protease Inhibitor-Associated Lipodystrophy Syndrome (diagnosed 2020-07-15)
- Stable Angina Pectoris due to Coronary Artery Disease (diagnosed 2021-01-15)
- ST-segment elevation myocardial infarction, inferior wall (diagnosed 2022-07-15)
- Essential hypertension
- Hyperlipidemia
- HIV infection
- Pathological gambling disorder
- Coronary artery disease

His medical history is significant for hypertension, controlled with lisinopril.

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (prior inferior STEMI)
- Conservative management of left-sided rib fractures

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg orally daily
- Atorvastatin 40 mg orally nightly
- Aspirin 81 mg orally daily
- Darunavir/Ritonavir 800/100 mg orally daily
- Emtricitabine/Tenofovir alafenamide 200/25 mg orally daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: premature myocardial infarction at age 55
- Mother: hypertension and hyperlipidemia
- Sibling: coronary artery disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (formerly occasional). Occupation: Accountant.

VITAL SIGNS:
His vital signs are as follows:

- Blood pressure: 145/90 mmHg
- Heart rate: 98 bpm
- Respiratory rate: 22 breaths/min
- Oxygen saturation: 94% on room air

PHYSICAL EXAMINATION:
On physical examination, he exhibits shallow breathing and winces with palpation over the left lateral chest wall. Breath sounds are diminished in the left lower lung field.

IMAGING:
A chest X-ray reveals two fractured ribs on the left side without evidence of pneumothorax.

PLAN:
Given the need to prevent complications such as atelectasis and pneumonia,

## Encounter 6: source 2023-07-15, practice 2025-12-02, ed, Emergency Medicine

Chief complaint: Acute onset of severe left flank pain radiating to the groin with nausea

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Maria Garcia, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset of severe left flank pain radiating to the groin with nausea

HISTORY OF PRESENT ILLNESS:
This 47-year-old male with coronary artery disease, prior myocardial infarction, and HIV infection presents with acute onset of severe left flank pain radiating to the groin, accompanied by nausea and no vomiting. He reports a history of similar, though less severe, episodes in the past.

PAST MEDICAL HISTORY:
Active Problem List:
- Hydrochlorothiazide adverse reaction (diagnosed 2020-01-15)
- Pathological Gambling (diagnosed 2020-04-15)
- HIV Protease Inhibitor-Associated Lipodystrophy Syndrome (diagnosed 2020-07-15)
- Stable Angina Pectoris due to Coronary Artery Disease (diagnosed 2021-01-15)
- ST-segment elevation myocardial infarction, inferior wall (diagnosed 2022-07-15)
- Fracture of two ribs, left side (diagnosed 2023-01-15)
- Essential hypertension
- Hyperlipidemia
- HIV infection
- Pathological gambling disorder
- Coronary artery disease

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (prior inferior STEMI)
- Conservative management of left-sided rib fractures

MEDICATIONS:
He takes lisinopril.

Home Medications:
- Atorvastatin 40 mg orally nightly
- Aspirin 81 mg orally daily
- Darunavir/Ritonavir 800/100 mg orally daily
- Emtricitabine/Tenofovir alafenamide 200/25 mg orally daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: premature myocardial infarction at age 55
- Mother: hypertension and hyperlipidemia
- Sibling: coronary artery disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (formerly occasional). Occupation: Accountant.

VITAL SIGNS:
Afebrile, blood pressure is 140/90 mmHg, and heart rate is 95 bpm

PHYSICAL EXAMINATION:
Abdominal examination reveals left costovertebral angle tenderness.

LABORATORY DATA:
Urinalysis shows hematuria. His serum creatinine is 1.0 mg/dL (normal: 0.6-1.2 mg/dL), and complete blood count is within normal limits.

IMAGING:
A non-contrast CT of the abdomen and pelvis reveals a 7 mm stone in the proximal left ureter without hydronephrosis.

## Encounter 7: source 2024-01-15, practice 2026-05-25, ed, Emergency Medicine

Chief complaint: Found confused and unresponsive at home by family member

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Robert Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Found confused and unresponsive at home by family member

PAST MEDICAL HISTORY:
Active Problem List:
- Hydrochlorothiazide adverse reaction (diagnosed 2020-01-15)
- Pathological Gambling (diagnosed 2020-04-15)
- HIV Protease Inhibitor-Associated Lipodystrophy Syndrome (diagnosed 2020-07-15)
- Stable Angina Pectoris due to Coronary Artery Disease (diagnosed 2021-01-15)
- ST-segment elevation myocardial infarction, inferior wall (diagnosed 2022-07-15)
- Fracture of two ribs, left side (diagnosed 2023-01-15)
- Ureterolithiasis (diagnosed 2023-07-15)
- Essential hypertension
- Hyperlipidemia
- HIV infection
- Pathological gambling disorder
- Coronary artery disease

The patient has a history of poorly controlled hypertension and non-adherence to medication.

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent placement (prior inferior STEMI)
- Conservative management of left-sided rib fractures

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg orally daily
- Atorvastatin 40 mg orally nightly
- Aspirin 81 mg orally daily
- Darunavir/Ritonavir 800/100 mg orally daily
- Emtricitabine/Tenofovir alafenamide 200/25 mg orally daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: premature myocardial infarction at age 55
- Mother: hypertension and hyperlipidemia
- Sibling: coronary artery disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (formerly occasional). Occupation: Accountant.

REVIEW OF SYSTEMS:
There is no evidence of chest pain, dyspnea, or focal motor deficits

VITAL SIGNS:
His blood pressure is measured at 225/130 mm Hg on arrival

PHYSICAL EXAMINATION:
Neurological examination reveals disorientation and bilateral Babinski signs. Funduscopic exam demonstrates flame-shaped hemorrhages and swelling of the optic discs

LABORATORY DATA:
Urinalysis demonstrates 10–18 red blood cells/hpf with mild proteinuria

PLAN:
A rapid-acting intravenous agent is administered, which directly relaxes arteriolar smooth muscle and causes prompt reduction in blood pressure
