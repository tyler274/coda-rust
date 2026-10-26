# synthetic_hospital patient 2774

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 52 at first encounter, sex F, South Asian (Indian), Private employer-sponsored health insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-05-02 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2022-11-14 (Mon) | outpatient | Cardiology |
| 2 | 2022-07-15 | 2024-11-01 (Fri) | outpatient | Cardiology |
| 3 | 2023-07-15 | 2025-10-27 (Mon) | outpatient | Cardiology |
| 4 | 2024-01-15 | 2026-04-26 (Sun) | icu | Cardiology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Private employer-sponsored health insurance",
 "occupation": "Administrative assistant",
 "alcohol_use": "None",
 "family_history": [
  "Father: Hypertension, cerebrovascular accident at age 70",
  "Mother: Type 2 diabetes mellitus, hypothyroidism",
  "No family history of congenital heart disease or rheumatic fever"
 ],
 "race_ethnicity": "South Asian (Indian)",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "5 mg orally daily (INR goal 2.0-3.0)",
   "name": "Warfarin"
  },
  {
   "dose": "40 mg orally daily",
   "name": "Furosemide"
  },
  {
   "dose": "50 mg orally daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "20 mEq orally daily",
   "name": "Potassium chloride"
  }
 ],
 "surgical_history": [
  "Tonsillectomy (age 11)",
  "Wisdom teeth extraction (age 22)"
 ],
 "chronic_conditions": [
  "Rheumatic heart disease (severe mitral stenosis)",
  "Pulmonary hypertension (secondary to valvular disease)",
  "Chronic heart failure (New York Heart Association Class III)",
  "Paroxysmal atrial fibrillation",
  "History of acute rheumatic fever (childhood, age 8)"
 ],
 "age_at_first_encounter": 52
}
```

Primary diagnoses: ["Rheumatic mitral stenosis", "Pulmonary Hypertension", "Stable Chronic Heart Failure"]
Comorbidities: ["Atrial fibrillation", "Rheumatic mitral stenosis", "History of rheumatic fever", "Left ventricular hypertrophy"]

## Encounter 0: source 2020-01-15, practice 2022-05-02, outpatient, Internal Medicine

Chief complaint: Progressive dyspnea on exertion, orthopnea, and paroxysmal nocturnal dyspnea for 6 months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. James Okonkwo, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive dyspnea on exertion, orthopnea, and paroxysmal nocturnal dyspnea for 6 months

HISTORY OF PRESENT ILLNESS:
She presents to the clinic with a 6-month history of progressive dyspnea on exertion, orthopnea, and paroxysmal nocturnal dyspnea. She reports a mild, persistent cough, especially when lying down, but denies any chest pain, palpitations, or lower extremity edema.

PAST MEDICAL HISTORY:
Active Problem List:
- Rheumatic heart disease (severe mitral stenosis)
- Pulmonary hypertension (secondary to valvular disease)
- Chronic heart failure (New York Heart Association Class III)
- Paroxysmal atrial fibrillation
- History of acute rheumatic fever (childhood, age 8)

Her past medical history is significant for rheumatic fever during childhood.

PAST SURGICAL HISTORY:
- Tonsillectomy (age 11)
- Wisdom teeth extraction (age 22)

MEDICATIONS:
Home Medications:
- Warfarin 5 mg orally daily (INR goal 2.0-3.0)
- Furosemide 40 mg orally daily
- Metoprolol succinate 50 mg orally daily
- Potassium chloride 20 mEq orally daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertension, cerebrovascular accident at age 70
- Mother: Type 2 diabetes mellitus, hypothyroidism
- No family history of congenital heart disease or rheumatic fever

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative assistant.

PHYSICAL EXAMINATION:
On physical examination, you note a diastolic rumbling murmur best heard at the apex with the patient in the left lateral decubitus position.

LABORATORY DATA:
Laboratory Values:
- Hemoglobin: 13.2 g/dL
- B-type natriuretic peptide (BNP): 150 pg/mL (normal <100 pg/mL)
- Serum creatinine: 0.9 mg/dL
- Brain natriuretic peptide (BNP): Elevated

IMAGING:
A transthoracic echocardiogram reveals a thickened mitral valve with a narrowed orifice and doming of the anterior mitral leaflet during diastole. Pulmonary artery systolic pressure is estimated to be 45 mmHg, while left ventricular end-diastolic pressure is within normal limits.

OTHER STUDIES:
The patient&#x27;s electrocardiogram shows signs of left atrial enlargement.

## Encounter 1: source 2020-07-15, practice 2022-11-14, outpatient, Cardiology

Chief complaint: Progressive shortness of breath and reduced exercise tolerance over the past year following recent hospitalization

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Priya Sharma, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive shortness of breath and reduced exercise tolerance over the past year following recent hospitalization

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of severe rheumatic mitral stenosis following January 2020 presentation. She presents with progressive shortness of breath and reduced exercise tolerance over the past year. She reports occasional episodes of palpitations and had a recent hospital visit for acute-onset atrial fibrillation.

PAST MEDICAL HISTORY:
Active Problem List:
- Rheumatic mitral stenosis (diagnosed 2020-01-15)
- Rheumatic heart disease (severe mitral stenosis)
- Pulmonary hypertension (secondary to valvular disease)
- Chronic heart failure (New York Heart Association Class III)
- Paroxysmal atrial fibrillation
- History of acute rheumatic fever (childhood, age 8)

PAST SURGICAL HISTORY:
- Tonsillectomy (age 11)
- Wisdom teeth extraction (age 22)

MEDICATIONS:
Home Medications:
- Warfarin 5 mg orally daily (INR goal 2.0-3.0)
- Furosemide 40 mg orally daily
- Metoprolol succinate 50 mg orally daily
- Potassium chloride 20 mEq orally daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertension, cerebrovascular accident at age 70
- Mother: Type 2 diabetes mellitus, hypothyroidism
- No family history of congenital heart disease or rheumatic fever

SOCIAL HISTORY:
She emigrated from South Asia 10 years ago

VITAL SIGNS:
Vital signs are stable.

PHYSICAL EXAMINATION:
On cardiac auscultation, a low-pitched, rumbling diastolic murmur is best heard at the apex in the left lateral decubitus position, with an opening snap following S2. Lung auscultation reveals bibasilar crackles.

IMAGING:
Echocardiogram demonstrates restricted mitral valve opening and an enlarged left atrium.

OTHER STUDIES:
ECG shows an irregularly irregular rhythm.

## Encounter 2: source 2022-07-15, practice 2024-11-01, outpatient, Cardiology

Chief complaint: Worsening dyspnea on exertion progressing to shortness of breath at rest with ankle swelling

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Michael Chen, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening dyspnea on exertion progressing to shortness of breath at rest with ankle swelling

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of rheumatic mitral stenosis, last evaluated in 2020, with progression of symptoms. She reports reduced exercise tolerance, difficulty climbing stairs, and waking up at night gasping for air. She also describes intermittent dry cough and recent swelling in her ankles. She denies chest pain, palpitations, syncope, or orthopnea.

PAST MEDICAL HISTORY:
Active Problem List:
- Rheumatic mitral stenosis (diagnosed 2020-01-15)
- Rheumatic heart disease (severe mitral stenosis)
- Pulmonary hypertension (secondary to valvular disease)
- Chronic heart failure (New York Heart Association Class III)
- Paroxysmal atrial fibrillation
- History of acute rheumatic fever (childhood, age 8)

Her past medical history is significant for acute rheumatic fever in childhood, which required hospitalization but was not followed by long-term prophylaxis. She has no known coronary artery disease or lung disease.

PAST SURGICAL HISTORY:
- Tonsillectomy (age 11)
- Wisdom teeth extraction (age 22)

MEDICATIONS:
She is.

Home Medications:
- Warfarin 5 mg orally daily (INR goal 2.0-3.0)
- Furosemide 40 mg orally daily
- Metoprolol succinate 50 mg orally daily
- Potassium chloride 20 mEq orally daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertension, cerebrovascular accident at age 70
- Mother: Type 2 diabetes mellitus, hypothyroidism
- No family history of congenital heart disease or rheumatic fever

SOCIAL HISTORY:
She has no history of alcohol or drug use

VITAL SIGNS:
Her temperature is 36.8°C (98.2°F), blood pressure is 112/76 mm Hg, pulse is 84/min and regular, and respirations are 20/min.

PHYSICAL EXAMINATION:
On examination, she appears mildly dyspneic at rest. Jugular venous pressure is elevated. Lung auscultation reveals bibasilar crackles without wheezes. Cardiac examination reveals a loud S₂ and a grade 4/6 low-pitched, rumbling diastolic murmur best heard at the apex in the left lateral decubitus position, accentuated by handgrip. The murmur is associated with a palpable thrill but no opening snap. There is mild pitting edema of both ankles.

IMAGING:
A transthoracic echocardiogram shows a thickened, immobile mitral valve with restricted opening and elevated left atrial pressures. There is no significant mitral regurgitation or aortic valve abnormality.

OTHER STUDIES:
ECG shows left atrial enlargement and right ventricular hypertrophy.

## Encounter 3: source 2023-07-15, practice 2025-10-27, outpatient, Cardiology

Chief complaint: Progressive orthopnea, paroxysmal nocturnal dyspnea, and episode of hemoptysis

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Priya Sharma, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive orthopnea, paroxysmal nocturnal dyspnea, and episode of hemoptysis

HISTORY OF PRESENT ILLNESS:
She returns for follow-up of rheumatic mitral stenosis with progressive symptoms. She also reports a recent episode of coughing up a small amount of blood. She denies chest pain, fever, or weight loss.

PAST MEDICAL HISTORY:
Active Problem List:
- Rheumatic mitral stenosis (diagnosed 2020-01-15)
- Pulmonary Hypertension (diagnosed 2022-07-15)
- Rheumatic heart disease (severe mitral stenosis)
- Pulmonary hypertension (secondary to valvular disease)
- Chronic heart failure (New York Heart Association Class III)
- Paroxysmal atrial fibrillation
- History of acute rheumatic fever (childhood, age 8)

PAST SURGICAL HISTORY:
- Tonsillectomy (age 11)
- Wisdom teeth extraction (age 22)

MEDICATIONS:
Home Medications:
- Warfarin 5 mg orally daily (INR goal 2.0-3.0)
- Furosemide 40 mg orally daily
- Metoprolol succinate 50 mg orally daily
- Potassium chloride 20 mEq orally daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertension, cerebrovascular accident at age 70
- Mother: Type 2 diabetes mellitus, hypothyroidism
- No family history of congenital heart disease or rheumatic fever

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative assistant.

VITAL SIGNS:
On examination, her blood pressure is 130/85 mmHg, heart rate is 92 bpm and irregularly irregular, and oxygen saturation is 96% on room air.

PHYSICAL EXAMINATION:
Lung auscultation reveals bibasilar crackles. Cardiac examination shows a diastolic rumbling murmur best heard at the apex with the patient in the left lateral decubitus position.

IMAGING:
A transthoracic echocardiogram reveals a thickened mitral valve with a narrowed orifice and left atrial enlargement.

OTHER STUDIES:
An electrocardiogram shows atrial fibrillation with no ST changes or T wave inversions.

## Encounter 4: source 2024-01-15, practice 2026-04-26, icu, Cardiology

Chief complaint: Acute onset of shortness of breath, palpitations, and cough productive of pink, frothy sputum

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Maria Gonzalez, MD
Department: Cardiology
Visit Type: icu

CHIEF COMPLAINT:
Acute onset of shortness of breath, palpitations, and cough productive of pink, frothy sputum

PAST MEDICAL HISTORY:
Active Problem List:
- Rheumatic mitral stenosis (diagnosed 2020-01-15)
- Pulmonary Hypertension (diagnosed 2022-07-15)
- Rheumatic heart disease (severe mitral stenosis)
- Pulmonary hypertension (secondary to valvular disease)
- Chronic heart failure (New York Heart Association Class III)
- Paroxysmal atrial fibrillation
- History of acute rheumatic fever (childhood, age 8)

She has a history of rheumatic fever in childhood.

PAST SURGICAL HISTORY:
- Tonsillectomy (age 11)
- Wisdom teeth extraction (age 22)

MEDICATIONS:
Home Medications:
- Warfarin 5 mg orally daily (INR goal 2.0-3.0)
- Furosemide 40 mg orally daily
- Metoprolol succinate 50 mg orally daily
- Potassium chloride 20 mEq orally daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertension, cerebrovascular accident at age 70
- Mother: Type 2 diabetes mellitus, hypothyroidism
- No family history of congenital heart disease or rheumatic fever

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative assistant.

VITAL SIGNS:
Blood pressure is 130/80 mmHg, heart rate is irregularly irregular at 120 bpm, and oxygen saturation is 89% on room air

PHYSICAL EXAMINATION:
Lung auscultation reveals bibasilar crackles. Cardiac examination shows a diastolic rumbling murmur best heard at the apex with an opening snap after S2.

IMAGING:
Chest X-ray demonstrates pulmonary edema and an enlarged left atrium.

OTHER STUDIES:
An electrocardiogram shows atrial fibrillation with no ST changes.
