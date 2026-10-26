# synthetic_hospital patient 2700

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 57 at first encounter, sex F, Non-Hispanic White, Commercial insurance (Blue Cross Blue Shield).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-10-27 (Fri) | outpatient | Internal Medicine |
| 1 | 2020-04-15 | 2024-02-05 (Mon) | outpatient | Urology |
| 2 | 2020-07-15 | 2024-05-03 (Fri) | outpatient | Ear Nose & Throat |
| 3 | 2021-01-15 | 2024-11-06 (Wed) | outpatient | Urology |
| 4 | 2021-04-15 | 2025-01-20 (Mon) | outpatient | Pulmonology |
| 5 | 2021-07-15 | 2025-04-25 (Fri) | inpatient | Emergency Medicine |
| 6 | 2022-01-15 | 2025-10-24 (Fri) | outpatient | Dermatology |
| 7 | 2022-07-15 | 2026-05-01 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Trimethoprim-sulfamethoxazole (diffuse alveolar hemorrhage)",
  "Lisinopril (dry cough)"
 ],
 "insurance": "Commercial insurance (Blue Cross Blue Shield)",
 "occupation": "Retired administrative assistant",
 "alcohol_use": "None",
 "family_history": [
  "Type 2 diabetes mellitus (mother)",
  "Hypertension (father)",
  "Cardiovascular disease (paternal grandfather)"
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
   "dose": "12.5 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Atorvastatin"
  }
 ],
 "surgical_history": [
  "Multiple dental extractions (partial edentulism)"
 ],
 "chronic_conditions": [
  "Type 2 diabetes mellitus",
  "Hypertension",
  "Postmenopausal status (menopause at age 52, no hormone replacement therapy)",
  "Urinary incontinence (urgency predominant)",
  "History of varicella zoster virus infection (chickenpox)",
  "Torus palatinus"
 ],
 "age_at_first_encounter": 57
}
```

Primary diagnoses: ["Urinary incontinence", "Torus palatinus", "Diffuse alveolar hemorrhage", "Herpes Zoster (Shingles), T8 Dermatome", "Systemic menopausal syndrome", "Lisinopril adverse effect"]
Comorbidities: ["Postmenopausal status", "Urinary tract infection", "Essential hypertension", "Type 2 diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2023-10-27, outpatient, Internal Medicine

Chief complaint: Increased urinary frequency, urgency, and burning sensation during urination

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Increased urinary frequency, urgency, and burning sensation during urination

HISTORY OF PRESENT ILLNESS:
She presents to the clinic complaining of a 6-month history of increased urinary frequency, urgency, and a burning sensation during urination. She also reports experiencing recurrent urinary tract infections and mild urinary incontinence that has worsened over the past few months. She has tried over-the-counter cranberry supplements and increased water intake with no improvement in symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 diabetes mellitus
- Hypertension
- Postmenopausal status (menopause at age 52, no hormone replacement therapy)
- Urinary incontinence (urgency predominant)
- History of varicella zoster virus infection (chickenpox)
- Torus palatinus

Her past medical history is significant for hypertension, which is well-controlled with hydrochlorothiazide. She underwent menopause at the age of 52 and has not used hormone replacement therapy.

PAST SURGICAL HISTORY:
- Multiple dental extractions (partial edentulism)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Atorvastatin 20 mg PO daily

ALLERGIES:
Trimethoprim-sulfamethoxazole (diffuse alveolar hemorrhage), Lisinopril (dry cough)

FAMILY HISTORY:
- Type 2 diabetes mellitus (mother)
- Hypertension (father)
- Cardiovascular disease (paternal grandfather)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired administrative assistant.

PHYSICAL EXAMINATION:
On physical examination, the external genitalia appear atrophic with pale, thin vulvar skin. Vaginal mucosa is dry and lacks rugae. There is no vaginal discharge or bleeding, and bimanual examination is unremarkable.

LABORATORY DATA:
Urinalysis shows no evidence of infection.

## Encounter 1: source 2020-04-15, practice 2024-02-05, outpatient, Urology

Chief complaint: 6-month history of sudden, overwhelming urges to urinate resulting in occasional incontinence

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
6-month history of sudden, overwhelming urges to urinate resulting in occasional incontinence

HISTORY OF PRESENT ILLNESS:
The patient presents to Urology for further evaluation of urinary symptoms previously assessed in Internal Medicine three months prior. She reports that these symptoms are affecting her quality of life, as she is now hesitant to leave her house for extended periods. She has tried to manage her symptoms by reducing her caffeine intake and scheduling regular bathroom breaks every 2 hours, but she has not noticed any improvement.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic menopausal syndrome (diagnosed 2020-01-15)
- Type 2 diabetes mellitus
- Hypertension
- Postmenopausal status (menopause at age 52, no hormone replacement therapy)
- Urinary incontinence (urgency predominant)
- History of varicella zoster virus infection (chickenpox)
- Torus palatinus

Her past medical history is significant for type 2 diabetes mellitus, well-controlled on metformin, and hypertension, managed with lisinopril.

PAST SURGICAL HISTORY:
- Multiple dental extractions (partial edentulism)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Atorvastatin 20 mg PO daily

ALLERGIES:
Trimethoprim-sulfamethoxazole (diffuse alveolar hemorrhage), Lisinopril (dry cough)

FAMILY HISTORY:
- Type 2 diabetes mellitus (mother)
- Hypertension (father)
- Cardiovascular disease (paternal grandfather)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired administrative assistant.

PHYSICAL EXAMINATION:
On physical examination, her abdomen is soft and non-tender, with no palpable masses. A pelvic examination reveals no prolapse or noticeable abnormalities.

LABORATORY DATA:
Urinalysis is negative for infection, glucose, or protein.

OTHER STUDIES:
Post-void residual volume is within normal limits.

## Encounter 2: source 2020-07-15, practice 2024-05-03, outpatient, Ear Nose & Throat

Chief complaint: Discomfort in mouth with difficulty eating and occasional speech interference

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Jennifer Park, MD
Department: Ear Nose & Throat
Visit Type: outpatient

CHIEF COMPLAINT:
Discomfort in mouth with difficulty eating and occasional speech interference

HISTORY OF PRESENT ILLNESS:
The patient presents for ENT evaluation of oral discomfort that has been progressively worsening over the past year, continuing longitudinal care for diabetes and hypertension following recent urology and internal medicine encounters. She reports difficulty with eating and occasional speech interference. She mentions that she has been considering getting dentures due to recent tooth loss but is concerned about the fit because of this oral mass.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic menopausal syndrome (diagnosed 2020-01-15)
- Urinary incontinence (diagnosed 2020-04-15)
- Type 2 diabetes mellitus
- Hypertension
- Postmenopausal status (menopause at age 52, no hormone replacement therapy)
- Urinary incontinence (urgency predominant)
- History of varicella zoster virus infection (chickenpox)
- Torus palatinus

She has a history of type 2 diabetes mellitus and hypertension, which are well-controlled.

PAST SURGICAL HISTORY:
- Multiple dental extractions (partial edentulism)

MEDICATIONS:
She takes metformin and lisinopril, respectively.

Home Medications:
- Hydrochlorothiazide 12.5 mg PO daily
- Atorvastatin 20 mg PO daily

ALLERGIES:
Trimethoprim-sulfamethoxazole (diffuse alveolar hemorrhage), Lisinopril (dry cough)

FAMILY HISTORY:
- Type 2 diabetes mellitus (mother)
- Hypertension (father)
- Cardiovascular disease (paternal grandfather)

SOCIAL HISTORY:
She is a non-smoker and does not consume alcohol.

PHYSICAL EXAMINATION:
On examination, you note a hard, midline mass on the roof of her mouth, which is non-tender to palpation. The mass does not blanch with pressure and the overlying mucosa is intact without ulceration. No cervical lymphadenopathy is palpable. The rest of the oral cavity examination is unremarkable.

## Encounter 3: source 2021-01-15, practice 2024-11-06, outpatient, Urology

Chief complaint: Ongoing issues with urine leakage gradually worsened over the past eight months

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Michael Rodriguez, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
Ongoing issues with urine leakage gradually worsened over the past eight months

HISTORY OF PRESENT ILLNESS:
The patient is a 58-year-old postmenopausal female who returns to the urology clinic for further evaluation of progressive urinary urgency and involuntary leakage. Following her initial presentation to Internal Medicine in January 2020 and subsequent assessment in this department in April 2020 for similar symptoms, she now reports worsening over the intervening eight months. She reports that the episodes typically begin with a sudden, intense sensation of needing to urinate, which she describes as coming "out of nowhere." Often, she is unable to make it to the bathroom in time, resulting in involuntary leakage of moderate amounts of urine. These events have become increasingly disruptive, occurring several times during the day and occasionally at night. She has tried to cut back on fluids, but the urgency remains unchanged. She does not experience leakage when sneezing, laughing, or exercising, and she denies any sensation of incomplete bladder emptying or dribbling after voiding. There is no associated dysuria, hematuria, or pelvic discomfort.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic menopausal syndrome (diagnosed 2020-01-15)
- Urinary incontinence (diagnosed 2020-04-15)
- Torus palatinus (diagnosed 2020-07-15)
- Type 2 diabetes mellitus
- Hypertension
- Postmenopausal status (menopause at age 52, no hormone replacement therapy)
- Urinary incontinence (urgency predominant)
- History of varicella zoster virus infection (chickenpox)

Her medical history includes mild hypertension. She is postmenopausal.

PAST SURGICAL HISTORY:
She has never undergone pelvic surgery.

MEDICATIONS:
She takes she takes a low-dose thiazide diuretic.

Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Atorvastatin 20 mg PO daily

ALLERGIES:
Trimethoprim-sulfamethoxazole (diffuse alveolar hemorrhage), Lisinopril (dry cough)

FAMILY HISTORY:
- Type 2 diabetes mellitus (mother)
- Hypertension (father)
- Cardiovascular disease (paternal grandfather)

SOCIAL HISTORY:
She lives independently and remains active in her community, although she has started to avoid outings due to fear of accidents.

PHYSICAL EXAMINATION:
Pelvic examination shows normal tissue tone and no pelvic organ prolapse. Neurologic examination is normal.

LABORATORY DATA:
Urinalysis is negative for leukocyte esterase, nitrites, blood, or glucose.

## Encounter 4: source 2021-04-15, practice 2025-01-20, outpatient, Pulmonology

Chief complaint: 3-month history of persistent dry cough

ENCOUNTER NOTE
Date: 2021-04-15
Provider: Dr. David Kim, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
3-month history of persistent dry cough

HISTORY OF PRESENT ILLNESS:
Patient presents with persistent cough during urologic care. She reports that the cough is nonproductive and is not associated with dyspnea, wheezing, or chest pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic menopausal syndrome (diagnosed 2020-01-15)
- Urinary incontinence (diagnosed 2020-04-15)
- Torus palatinus (diagnosed 2020-07-15)
- Type 2 diabetes mellitus
- Hypertension
- Postmenopausal status (menopause at age 52, no hormone replacement therapy)
- Urinary incontinence (urgency predominant)
- History of varicella zoster virus infection (chickenpox)

She has a past medical history of hypertension and type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Multiple dental extractions (partial edentulism)

MEDICATIONS:
Her medications include lisinopril, metformin, and atorvastatin.

Home Medications:
- Hydrochlorothiazide 12.5 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Type 2 diabetes mellitus (mother)
- Hypertension (father)
- Cardiovascular disease (paternal grandfather)

SOCIAL HISTORY:
She has never smoked

VITAL SIGNS:
On physical examination, her blood pressure is 135/85 mmHg, heart rate is 78 beats per minute, and respiratory rate is 16 breaths per minute.

PHYSICAL EXAMINATION:
Lung auscultation reveals no wheezes, rales, or rhonchi. The rest of the examination is unremarkable.

LABORATORY DATA:
Laboratory values:
- Hemoglobin A1c: 7.2%
- Serum creatinine: 1.0 mg/dL
- Serum potassium: 4.5 mEq/L

## Encounter 5: source 2021-07-15, practice 2025-04-25, inpatient, Emergency Medicine

Chief complaint: 3-day history of fever, shortness of breath, and dry cough

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Lisa Thompson, MD
Department: Emergency Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
3-day history of fever, shortness of breath, and dry cough

HISTORY OF PRESENT ILLNESS:
Patient with incontinence and prior pulmonary evaluation returning with acute symptoms. She reports that her symptoms began a week after she started taking a new medication for a urinary tract infection. She denies any recent travel or sick contacts.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic menopausal syndrome (diagnosed 2020-01-15)
- Urinary incontinence (diagnosed 2020-04-15)
- Torus palatinus (diagnosed 2020-07-15)
- Lisinopril adverse effect (diagnosed 2021-04-15)
- Type 2 diabetes mellitus
- Hypertension
- Postmenopausal status (menopause at age 52, no hormone replacement therapy)
- Urinary incontinence (urgency predominant)
- History of varicella zoster virus infection (chickenpox)

Her past medical history is significant for type 2 diabetes mellitus and hypertension.

PAST SURGICAL HISTORY:
- Multiple dental extractions (partial edentulism)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO BID
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Atorvastatin 20 mg PO daily

ALLERGIES:
Trimethoprim-sulfamethoxazole (diffuse alveolar hemorrhage), Lisinopril (dry cough)

FAMILY HISTORY:
- Type 2 diabetes mellitus (mother)
- Hypertension (father)
- Cardiovascular disease (paternal grandfather)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired administrative assistant.

VITAL SIGNS:
On examination, she is febrile with a temperature of 38.5°C (101.3°F), her heart rate is 102 beats per minute, and her respiratory rate is 22 breaths per minute. Oxygen saturation is 92% on room air.

PHYSICAL EXAMINATION:
Lung auscultation reveals bilateral basilar crackles. A diffuse erythematous rash is noted on her trunk and upper extremities.

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 12,000/mm³ with a normal differential
- Hemoglobin: 13.2 g/dL
- Platelet count: 250,000/mm³
- Serum creatinine: 1.0 mg/dL
- Blood urea nitrogen: 14 mg/dL
- C-reactive protein: 50 mg/L (normal <10 mg/L)

IMAGING:
Chest X-ray shows bilateral basilar opacities and small pleural effusions.

## Encounter 6: source 2022-01-15, practice 2025-10-24, outpatient, Dermatology

Chief complaint: Painful rash on left flank present for three days

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Aisha Patel, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Painful rash on left flank present for three days

HISTORY OF PRESENT ILLNESS:
She has a history of varicella zoster virus infection. She describes the pain as a constant, burning sensation that began a few days before the rash appeared.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic menopausal syndrome (diagnosed 2020-01-15)
- Urinary incontinence (diagnosed 2020-04-15)
- Torus palatinus (diagnosed 2020-07-15)
- Lisinopril adverse effect (diagnosed 2021-04-15)
- Diffuse alveolar hemorrhage (diagnosed 2021-07-15)
- Type 2 diabetes mellitus
- Hypertension
- Postmenopausal status (menopause at age 52, no hormone replacement therapy)
- Urinary incontinence (urgency predominant)
- History of varicella zoster virus infection (chickenpox)

Her medical history is significant for type 2 diabetes mellitus and hypertension, which are well-controlled.

PAST SURGICAL HISTORY:
- Multiple dental extractions (partial edentulism)

MEDICATIONS:
She takes with metformin and lisinopril, respectively.

Home Medications:
- Hydrochlorothiazide 12.5 mg PO daily
- Atorvastatin 20 mg PO daily

ALLERGIES:
Trimethoprim-sulfamethoxazole (diffuse alveolar hemorrhage), Lisinopril (dry cough)

FAMILY HISTORY:
- Type 2 diabetes mellitus (mother)
- Hypertension (father)
- Cardiovascular disease (paternal grandfather)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired administrative assistant.

VITAL SIGNS:
Her temperature is 37.2°C (99°F), blood pressure is 132/84 mmHg, and heart rate is 78 beats per minute

PHYSICAL EXAMINATION:
On physical examination, you note a cluster of clear vesicles on an erythematous base, distributed in a band-like pattern along the T8 dermatome. There are no vesicles crossing the midline. The remainder of the examination is unremarkable.

LABORATORY DATA:
Laboratory values:
- Complete blood count: within normal limits
- Fasting blood glucose: 146 mg/dL
- Hemoglobin A1c: 7.2%

## Encounter 7: source 2022-07-15, practice 2026-05-01, ed, Emergency Medicine

Chief complaint: Severe burning pain on the left side of her chest for three days

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Robert Singh, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe burning pain on the left side of her chest for three days

HISTORY OF PRESENT ILLNESS:
The patient returns six months after her prior zoster episode with a history of varicella zoster virus infection. She describes the pain as a constant, burning sensation that has been present for the past three days. She denies any recent trauma or injury to the area.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic menopausal syndrome (diagnosed 2020-01-15)
- Urinary incontinence (diagnosed 2020-04-15)
- Torus palatinus (diagnosed 2020-07-15)
- Lisinopril adverse effect (diagnosed 2021-04-15)
- Diffuse alveolar hemorrhage (diagnosed 2021-07-15)
- Herpes Zoster (Shingles), T8 Dermatome (diagnosed 2022-01-15)
- Type 2 diabetes mellitus
- Hypertension
- Postmenopausal status (menopause at age 52, no hormone replacement therapy)
- Urinary incontinence (urgency predominant)
- History of varicella zoster virus infection (chickenpox)

Her medical history is significant for hypertension, and type 2 diabetes mellitus. She had chickenpox as a child.

PAST SURGICAL HISTORY:
- Multiple dental extractions (partial edentulism)

MEDICATIONS:
She takes for which she takes hydrochlorothiazide, and type 2 diabetes mellitus controlled with metformin.

Home Medications:
- Lisinopril 20 mg PO daily
- Atorvastatin 20 mg PO daily

ALLERGIES:
Trimethoprim-sulfamethoxazole (diffuse alveolar hemorrhage), Lisinopril (dry cough)

FAMILY HISTORY:
- Type 2 diabetes mellitus (mother)
- Hypertension (father)
- Cardiovascular disease (paternal grandfather)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired administrative assistant.

VITAL SIGNS:
Her vital signs are within normal limits.

PHYSICAL EXAMINATION:
On physical examination, you note hyperesthesia over the affected area but no rash is evident.
