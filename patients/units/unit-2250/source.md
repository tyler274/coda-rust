# synthetic_hospital patient 2250

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 64 at first encounter, sex M, White, non-Hispanic, Private insurance through former employer, Medicare-eligible.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-06-23 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-04-15 | 2025-09-15 (Mon) | outpatient | Cardiology |
| 2 | 2020-09-15 | 2026-02-18 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Niacin (intolerance - severe flushing)",
  "No known drug allergies"
 ],
 "insurance": "Private insurance through former employer, Medicare-eligible",
 "occupation": "Retired certified public accountant",
 "alcohol_use": "Occasional social use (1-2 drinks per week)",
 "family_history": [
  "Father: Myocardial infarction at age 58",
  "Mother: Hypertension and hyperlipidemia",
  "Brother: Atrial fibrillation diagnosed age 62"
 ],
 "race_ethnicity": "White, non-Hispanic",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "200 mg by mouth daily",
   "name": "Amiodarone"
  },
  {
   "dose": "20 mg by mouth daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg by mouth daily",
   "name": "Atorvastatin"
  }
 ],
 "surgical_history": [
  "No prior surgeries"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Hyperlipidemia",
  "Atrial fibrillation",
  "Coronary artery disease"
 ],
 "age_at_first_encounter": 64
}
```

Primary diagnoses: ["Flecainide adverse effect", "Niacin-induced flushing (prostaglandin-mediated)", "Non-ST elevation myocardial infarction (NSTEMI)"]
Comorbidities: ["Hyperlipidemia", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2025-06-23, outpatient, Internal Medicine

Chief complaint: Intense facial flushing and burning sensation on face and chest after starting new cholesterol medication.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Sharma, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Intense facial flushing and burning sensation on face and chest after starting new cholesterol medication.

HISTORY OF PRESENT ILLNESS:
He is prescribed niacin for secondary prevention of cardiovascular disease. At a follow-up visit one week later, he reports that he stopped taking the medication after two doses due to intense facial flushing and warmth. He says he took the pill in the evening with food, as directed. The patient is interested in retrying niacin if the side effects can be reduced.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Hyperlipidemia
- Atrial fibrillation
- Coronary artery disease

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
His current medications include atorvastatin, aspirin, and metoprolol.

Home Medications:
- Amiodarone 200 mg by mouth daily
- Lisinopril 20 mg by mouth daily

ALLERGIES:
Niacin (intolerance - severe flushing), No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 58
- Mother: Hypertension and hyperlipidemia
- Brother: Atrial fibrillation diagnosed age 62

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per week). Occupation: Retired certified public accountant.

REVIEW OF SYSTEMS:
He denies rash, shortness of breath, or dizziness. He did not have hives or throat swelling.

VITAL SIGNS:
His blood pressure and heart rate are normal.

PHYSICAL EXAMINATION:
Physical examination is unremarkable, and there are no signs of angioedema or urticaria.

ASSESSMENT AND PLAN:
The physician explains that the flushing is a known side effect of niacin, not an allergy, and can often be managed with premedication.

The physician recommends taking the next dose with a low-dose nonsteroidal anti-inflammatory drug.

## Encounter 1: source 2020-04-15, practice 2025-09-15, outpatient, Cardiology

Chief complaint: Progressive dyspnea on exertion and nonproductive cough over the past 2 months.

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. James Okafor, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive dyspnea on exertion and nonproductive cough over the past 2 months.

HISTORY OF PRESENT ILLNESS:
This patient with hyperlipidemia and atrial fibrillation returns three months after niacin-induced flushing. He presents to clinic with a 2-month history of progressive dyspnea on exertion and nonproductive cough. He denies fever, chills, or chest pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Niacin-induced flushing (prostaglandin-mediated) (diagnosed 2020-01-15)
- Hypertension
- Hyperlipidemia
- Atrial fibrillation
- Coronary artery disease

His past medical history is significant for hypertension and hyperlipidemia.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
His medications include amiodarone, lisinopril, and atorvastatin.

ALLERGIES:
Niacin (intolerance - severe flushing), No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 58
- Mother: Hypertension and hyperlipidemia
- Brother: Atrial fibrillation diagnosed age 62

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per week). Occupation: Retired certified public accountant.

VITAL SIGNS:
Oxygen saturation is 92% on room air

PHYSICAL EXAMINATION:
Auscultation of the lungs reveals bilateral crackles at the bases

LABORATORY DATA:
Laboratory Values:
- White Blood Cell Count: 7,000/uL
- Hemoglobin: 14 g/dL
- Platelet Count: 250,000/uL
- BNP: 90 pg/mL (normal <100 pg/mL)
- Serum Creatinine: 1.0 mg/dL

IMAGING:
A chest radiograph shows diffuse interstitial infiltrates

OTHER STUDIES:
Pulmonary function tests reveal a restrictive pattern with reduced diffusion capacity for carbon monoxide (DLCO)

## Encounter 2: source 2020-09-15, practice 2026-02-18, ed, Emergency Medicine

Chief complaint: Acute chest discomfort described as pressure-like sensation in the middle of the chest radiating to the left side, beginning 3 hours ago while gardening.

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Maria Gonzalez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute chest discomfort described as pressure-like sensation in the middle of the chest radiating to the left side, beginning 3 hours ago while gardening.

HISTORY OF PRESENT ILLNESS:
The patient is a 64-year-old male with hypertension, hyperlipidemia, atrial fibrillation, and coronary artery disease who presents with acute chest discomfort, having previously experienced niacin-induced flushing and flecainide-associated dyspnea requiring medication changes. He is brought to the emergency department by his wife because of chest discomfort that began 3 hours ago. The pain started while he was working in his backyard garden, planting tomato seedlings. He describes it as a pressure-like sensation located in the middle of his chest that radiates to his left shoulder and jaw. The discomfort was initially mild but has progressively intensified despite resting indoors. He has associated shortness of breath, mild nausea, and lightheadedness, but no vomiting, syncope, or palpitations.

PAST MEDICAL HISTORY:
Active Problem List:
- Niacin-induced flushing (prostaglandin-mediated) (diagnosed 2020-01-15)
- Flecainide adverse effect (diagnosed 2020-04-15)
- Hypertension
- Hyperlipidemia
- Atrial fibrillation
- Coronary artery disease

The patient has a 15-year history of hypertension and hyperlipidemia, for which he takes lisinopril and atorvastatin.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
He takes lisinopril and atorvastatin.

Home Medications:
- Amiodarone 200 mg by mouth daily

ALLERGIES:
Niacin (intolerance - severe flushing), No known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 58
- Mother: Hypertension and hyperlipidemia
- Brother: Atrial fibrillation diagnosed age 62

SOCIAL HISTORY:
He was a certified public accountant before retiring last year and now spends much of his time gardening, playing chess with friends at the local community center, and doing crossword puzzles. He does not smoke and drinks a glass of wine occasionally with dinner.

VITAL SIGNS:
His temperature is 36.9°C (98.4°F), blood pressure is 146/88 mm Hg, pulse is 92/min, and respirations are 18/min. Oxygen saturation is 97% on room air.

PHYSICAL EXAMINATION:
On arrival, he appears mildly anxious but is conversing appropriately. Cardiac examination reveals a regular rhythm without murmurs, rubs, or gallops. Lungs are clear to auscultation. The abdomen is soft and nontender, and extremities show no edema.

OTHER STUDIES:
An electrocardiogram reveals ST-segment depressions in leads II, III, and aVF.

PLAN:
The patient is started on supplemental oxygen, aspirin, and intravenous unfractionated heparin as part of acute management.
