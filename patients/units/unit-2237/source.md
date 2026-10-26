# synthetic_hospital patient 2237

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 68 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-07-07 (Mon) | outpatient | Cardiology |
| 1 | 2020-04-15 | 2025-10-11 (Sat) | icu | Emergency Medicine |
| 2 | 2021-01-15 | 2026-07-10 (Fri) | icu | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired postal service supervisor",
 "alcohol_use": "None",
 "family_history": [
  "Father: Myocardial infarction at age 58, deceased",
  "Mother: Hypertension and hyperlipidemia, age 82",
  "Brother: Coronary artery disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 40 pack-years, quit 5 years ago",
 "home_medications": [
  {
   "dose": "40 mg oral daily",
   "name": "Furosemide"
  },
  {
   "dose": "20 mg oral daily",
   "name": "Lisinopril"
  },
  {
   "dose": "12.5 mg oral twice daily",
   "name": "Carvedilol"
  },
  {
   "dose": "40 mg oral daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "81 mg oral daily",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "Percutaneous coronary intervention with drug-eluting stent (5 years prior)",
  "Percutaneous coronary intervention (10 years prior)"
 ],
 "chronic_conditions": [
  "Coronary artery disease",
  "Chronic systolic heart failure with reduced ejection fraction (LVEF 30%)",
  "Hypertension",
  "Hyperlipidemia",
  "History of myocardial infarction"
 ],
 "age_at_first_encounter": 68
}
```

Primary diagnoses: ["Cardiogenic Shock", "Acute coronary syndrome"]
Comorbidities: ["Medication noncompliance", "Old myocardial infarction", "Hyperlipidemia", "Essential hypertension", "Heart failure with reduced ejection fraction"]

## Encounter 0: source 2020-01-15, practice 2025-07-07, outpatient, Cardiology

Chief complaint: Progressive substernal chest pressure radiating to the left arm over two months, now occurring at rest while reading the newspaper.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive substernal chest pressure radiating to the left arm over two months, now occurring at rest while reading the newspaper.

HISTORY OF PRESENT ILLNESS:
He reports that for the past two months, he has experienced episodes of substernal pressure radiating to his left arm whenever he climbs stairs or walks briskly after meals. Over the past week, the pain has become more frequent, sometimes occurring even while he reads his morning newspaper. This chest discomfort is often accompanied by mild diaphoresis, shortness of breath, and a sense of impending doom. He denies nausea, vomiting, or palpitations, but admits that he has been limiting his activity for fear of triggering the pain. Despite these health issues, the patient maintains a very structured morning routine that he has followed for years. He wakes at 6 a.m., prepares two slices of buttered toast and scrambled eggs, and enjoys a cup of strong black coffee. He always spreads out the daily newspaper across his kitchen table and reads every section carefully, starting with international affairs before moving to local news and the arts. After breakfast, he sometimes takes a short walk to a nearby cafe where he sits outside, sipping another coffee while watching passersby. He takes great pleasure in photographing the small details of street life—shadows cast by bicycles, the expressions of cafe patrons, and architectural patterns. His passion is black-and-white photography, and he has built an extensive collection of prints over the years. In recent months, however, his routine has been disrupted. He has skipped his cafe visits because walking even two blocks provokes angina. He also finds himself cutting breakfast short when chest pressure arises during reading. His wife notes that he seems increasingly anxious and withdrawn, worried about the unpredictability of his symptoms. Despite this, he has resisted coming to the doctor until now, believing he could manage on his own.

PAST MEDICAL HISTORY:
Active Problem List:
- Coronary artery disease
- Chronic systolic heart failure with reduced ejection fraction (LVEF 30%)
- Hypertension
- Hyperlipidemia
- History of myocardial infarction

The patient describes a long history of cardiovascular disease. He was first diagnosed with hypertension 20 years ago, for which he has been intermittently compliant with medication. Ten years ago, he was found to have hyperlipidemia, but he did not take his statin consistently due to concerns about muscle pain. Five years ago, he suffered a myocardial infarction and underwent balloon angioplasty without stent placement. Since then, he has been on aspirin but admits to occasionally forgetting his daily dose.

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent (5 years prior)
- Percutaneous coronary intervention (10 years prior)

MEDICATIONS:
Home Medications:
- Furosemide 40 mg oral daily
- Lisinopril 20 mg oral daily
- Carvedilol 12.5 mg oral twice daily
- Atorvastatin 40 mg oral daily
- Aspirin 81 mg oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
His father died of a heart attack at age 59, and his older brother had a coronary artery bypass at age 62.

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 5 years ago. Alcohol: None. Occupation: Retired postal service supervisor.

VITAL SIGNS:
On examination, his blood pressure is 148/90 mm Hg, heart rate 84/min, and BMI 29.

PHYSICAL EXAMINATION:
Cardiac auscultation reveals a normal S1 and S2 without murmurs, rubs, or gallops. His lungs are clear, and there is no peripheral edema.

IMAGING:
Coronary angiography demonstrates multivessel coronary artery disease with a tight lesion in the left anterior descending artery.

OTHER STUDIES:
An electrocardiogram shows nonspecific ST-T wave changes. Stress testing is positive for inducible ischemia at low exertion,

PLAN:
Given his unstable symptoms, the decision is made to proceed with percutaneous coronary intervention and stent placement. In preparation for the procedure, he is started on aspirin and another drug that prevents platelet aggregation by blocking the glycoprotein IIb/IIIa receptor, thereby inhibiting platelet interaction with fibrinogen. The patient is counseled about the increased risk of bleeding but agrees to therapy.

## Encounter 1: source 2020-04-15, practice 2025-10-11, icu, Emergency Medicine

Chief complaint: Found collapsed at home after three days of progressive shortness of breath, orthopnea, and bilateral leg swelling.

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Found collapsed at home after three days of progressive shortness of breath, orthopnea, and bilateral leg swelling.

HISTORY OF PRESENT ILLNESS:
This patient returns three months after acute coronary syndrome evaluation. His wife reports that over the past 3 days he has had progressive shortness of breath, orthopnea, and swelling in his legs.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute coronary syndrome (diagnosed 2020-01-15)
- Coronary artery disease
- Chronic systolic heart failure with reduced ejection fraction (LVEF 30%)
- Hypertension
- Hyperlipidemia
- History of myocardial infarction

He has a history of coronary artery disease with a myocardial infarction 5 years ago, chronic systolic heart failure (LVEF 30%), hypertension, and hyperlipidemia.

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent (5 years prior)
- Percutaneous coronary intervention (10 years prior)

MEDICATIONS:
His medications include furosemide, lisinopril, carvedilol, and atorvastatin, but he missed several doses recently due to nausea.

Home Medications:
- Aspirin 81 mg oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 58, deceased
- Mother: Hypertension and hyperlipidemia, age 82
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 5 years ago. Alcohol: None. Occupation: Retired postal service supervisor.

VITAL SIGNS:
Blood pressure is 82/54 mm Hg, heart rate 112/min, respiratory rate 26/min, and oxygen saturation 88% on room air.

PHYSICAL EXAMINATION:
On arrival, the patient appears diaphoretic and anxious. Jugular venous distension is noted to the angle of the jaw at 45 degrees. Cardiac auscultation reveals an S3 gallop; lung auscultation reveals bibasilar crackles. Extremities are cool to the touch with delayed capillary refill, and pitting edema is present to the knees bilaterally.

LABORATORY DATA:
Laboratory studies reveal elevated BNP and mildly elevated troponin.

IMAGING:
An urgent chest x-ray reveals the following.

OTHER STUDIES:
ECG shows sinus tachycardia with nonspecific ST-T changes.

PLAN:
The patient is placed on supplemental oxygen and undergoes right heart catheterization for further hemodynamic assessment.

## Encounter 2: source 2021-01-15, practice 2026-07-10, icu, Emergency Medicine

Chief complaint: Sudden onset of severe shortness of breath and acute confusion that began this morning.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Sudden onset of severe shortness of breath and acute confusion that began this morning.

HISTORY OF PRESENT ILLNESS:
This 69-year-old male with coronary artery disease, chronic systolic heart failure with reduced ejection fraction, and prior ICU admission for cardiogenic shock is brought to the emergency department with severe shortness of breath and confusion that began suddenly this morning.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute coronary syndrome (diagnosed 2020-01-15)
- Cardiogenic Shock (diagnosed 2020-04-15)
- Coronary artery disease
- Chronic systolic heart failure with reduced ejection fraction (LVEF 30%)
- Hypertension
- Hyperlipidemia
- History of myocardial infarction

He has a history of prior myocardial infarction and chronic heart failure with reduced ejection fraction.

PAST SURGICAL HISTORY:
- Percutaneous coronary intervention with drug-eluting stent (5 years prior)
- Percutaneous coronary intervention (10 years prior)

MEDICATIONS:
Home Medications:
- Furosemide 40 mg oral daily
- Lisinopril 20 mg oral daily
- Carvedilol 12.5 mg oral twice daily
- Atorvastatin 40 mg oral daily
- Aspirin 81 mg oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 58, deceased
- Mother: Hypertension and hyperlipidemia, age 82
- Brother: Coronary artery disease

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-years, quit 5 years ago. Alcohol: None. Occupation: Retired postal service supervisor.

VITAL SIGNS:
On arrival, he is hypotensive with a blood pressure of 78/55 mm Hg, heart rate 120/min, and respiratory rate 28/min.

PHYSICAL EXAMINATION:
Jugular venous distension is present, and auscultation reveals diffuse crackles bilaterally. Extremities are cool and clammy.

IMAGING:
A bedside echocardiogram shows severely decreased left ventricular contractility.
