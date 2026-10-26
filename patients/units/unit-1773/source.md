# synthetic_hospital patient 1773

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex F, White, Employer-sponsored private insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-01-15 (Fri) | outpatient | Neurology |
| 1 | 2020-04-15 | 2021-04-13 (Tue) | outpatient | Internal Medicine |
| 2 | 2020-07-15 | 2021-07-08 (Thu) | outpatient | Obstetrics & Gynecology |
| 3 | 2022-01-15 | 2023-01-09 (Mon) | outpatient | Endocrinology |
| 4 | 2023-01-15 | 2024-01-08 (Mon) | outpatient | Obstetrics & Gynecology |
| 5 | 2023-07-15 | 2024-07-11 (Thu) | outpatient | Neurology |
| 6 | 2024-07-15 | 2025-07-01 (Tue) | outpatient | Neurology |
| 7 | 2025-07-15 | 2026-07-16 (Thu) | outpatient | Neurology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Employer-sponsored private insurance",
 "occupation": "Graphic designer",
 "alcohol_use": "Social",
 "family_history": [
  "Migraine headaches in mother",
  "Breast cancer in maternal aunt (premenopausal)",
  "Hypertension in father"
 ],
 "race_ethnicity": "White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "10 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "25 mg daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "50 mg PRN",
   "name": "Sumatriptan"
  },
  {
   "dose": "500 mg PRN",
   "name": "Acetaminophen"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Migraine with aura",
  "Essential hypertension",
  "Estrogen receptor-positive breast cancer",
  "Adrenal Cushing's syndrome"
 ],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Migraine with aura", "Malignant neoplasm of male breast", "Essential hypertension", "Adrenal Cushing's syndrome"]
Comorbidities: ["Essential hypertension", "Obesity"]

## Encounter 0: source 2020-01-15, practice 2021-01-15, outpatient, Neurology

Chief complaint: Recurring headaches with visual disturbances

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Recurring headaches with visual disturbances

HISTORY OF PRESENT ILLNESS:
She presents to the clinic with a 5-month history of recurring headaches. She describes the pain as a throbbing sensation behind her right eye that builds over 30–60 minutes and can last for up to 8 hours. The episodes occur 2–3 times per month and are sometimes preceded by flashes of light in her peripheral vision. During attacks, she prefers to lie in a dark room and avoid sound. She denies fever, neck stiffness, trauma, or recent illness.

PAST MEDICAL HISTORY:
Active Problem List:
- Migraine with aura
- Essential hypertension
- Estrogen receptor-positive breast cancer
- Adrenal Cushing's syndrome

Her past medical history includes well-controlled hypertension.

MEDICATIONS:
She takes she is currently taking lisinopril.

Home Medications:
- Hydrochlorothiazide 25 mg daily
- Sumatriptan 50 mg PRN
- Acetaminophen 500 mg PRN

ALLERGIES:
NKDA

FAMILY HISTORY:
- Migraine headaches in mother
- Breast cancer in maternal aunt (premenopausal)
- Hypertension in father

SOCIAL HISTORY:
She is not pregnant and currently takes no contraceptives

PHYSICAL EXAMINATION:
Neurologic exam is normal, and there are no focal deficits

## Encounter 1: source 2020-04-15, practice 2021-04-13, outpatient, Internal Medicine

Chief complaint: Routine hypertension management and contraception interest

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Marcus Johnson, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine hypertension management and contraception interest

HISTORY OF PRESENT ILLNESS:
The patient with migraine with aura presents for hypertension management and contraception counseling. She reports no current symptoms and is interested in starting a birth control method. She is concerned about the risk of cancer associated with hormonal contraceptives.

PAST MEDICAL HISTORY:
Active Problem List:
- Migraine with aura (diagnosed 2020-01-15)
- Essential hypertension
- Estrogen receptor-positive breast cancer
- Adrenal Cushing's syndrome

She has been on hydrochlorothiazide for the past year with good control of her blood pressure.

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Lisinopril 10 mg daily
- Sumatriptan 50 mg PRN
- Acetaminophen 500 mg PRN

ALLERGIES:
NKDA

FAMILY HISTORY:
Family history is unremarkable

SOCIAL HISTORY:
No history of smoking

VITAL SIGNS:
Blood pressure today is 128/82 mmHg

PHYSICAL EXAMINATION:
Her physical examination is within normal limits.

OTHER STUDIES:
Her most recent Pap smear was normal.

## Encounter 2: source 2020-07-15, practice 2021-07-08, outpatient, Obstetrics & Gynecology

Chief complaint: Contraception counseling regarding stroke risk

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Priya Patel, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Contraception counseling regarding stroke risk

HISTORY OF PRESENT ILLNESS:
The patient returns for contraceptive counseling following her April 2020 encounter where birth control was discussed during hypertension management. She has recently married and is looking for a reliable method of birth control but is concerned about the risk of thrombosis with hormonal contraception due to her migraines with aura.

PAST MEDICAL HISTORY:
Active Problem List:
- Migraine with aura (diagnosed 2020-01-15)
- Essential hypertension (diagnosed 2020-04-15)
- Estrogen receptor-positive breast cancer
- Adrenal Cushing's syndrome

Her migraines are typically unilateral, pulsating, and associated with photophobia, phonophobia, and visual auras. She has no personal history of venous thromboembolism (VTE).

MEDICATIONS:
Her only medication is sumatriptan, which she uses as needed for migraine attacks.

Home Medications:
- Lisinopril 10 mg daily
- Hydrochlorothiazide 25 mg daily
- Acetaminophen 500 mg PRN

ALLERGIES:
NKDA

FAMILY HISTORY:
Family history of venous thromboembolism (VTE),

SOCIAL HISTORY:
She reports no smoking history

VITAL SIGNS:
She has a BMI of 24 kg/m^2. Her blood pressure is 125/78 mmHg,

PHYSICAL EXAMINATION:
Her physical examination is unremarkable.

## Encounter 3: source 2022-01-15, practice 2023-01-09, outpatient, Endocrinology

Chief complaint: Progressive weight gain, fatigue, and menstrual irregularities

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. David Kim, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive weight gain, fatigue, and menstrual irregularities

HISTORY OF PRESENT ILLNESS:
The patient presents to endocrinology with progressive weight gain, fatigue, and menstrual irregularities in the context of her history of migraine with aura and essential hypertension. She reports increased fat around her face and upper back, easy bruising, skin thinning, irregular menses, and purple abdominal striae.

PAST MEDICAL HISTORY:
Active Problem List:
- Migraine with aura (diagnosed 2020-01-15)
- Essential hypertension (diagnosed 2020-04-15)
- Estrogen receptor-positive breast cancer
- Adrenal Cushing's syndrome

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg daily
- Hydrochlorothiazide 25 mg daily
- Sumatriptan 50 mg PRN
- Acetaminophen 500 mg PRN

ALLERGIES:
NKDA

FAMILY HISTORY:
- Migraine headaches in mother
- Breast cancer in maternal aunt (premenopausal)
- Hypertension in father

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social. Occupation: Graphic designer.

VITAL SIGNS:
Blood pressure is 150/95 mmHg.

PHYSICAL EXAMINATION:
Physical exam reveals central obesity, a rounded face, and dorsocervical fat pad.

IMAGING:
CT imaging of the abdomen reveals a 3.5-cm adrenal mass.

OTHER STUDIES:
Low-dose dexamethasone fails to suppress serum cortisol.

## Encounter 4: source 2023-01-15, practice 2024-01-08, outpatient, Obstetrics & Gynecology

Chief complaint: Contraceptive counseling after recent breast cancer diagnosis

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Maria Gonzalez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Contraceptive counseling after recent breast cancer diagnosis

HISTORY OF PRESENT ILLNESS:
Patient returns for contraceptive counseling with migraine with aura, hypertension, Cushing's syndrome, and breast cancer, evaluated in 2020 for stroke risk. She was recently diagnosed with estrogen receptor-positive breast cancer. She is interested in a reliable form of contraception. She does not desire pregnancy in the near future. Her menstrual cycles are regular, with last menstrual period 1 week ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Migraine with aura (diagnosed 2020-01-15)
- Essential hypertension (diagnosed 2020-04-15)
- Adrenal Cushing's syndrome (diagnosed 2022-01-15)
- Estrogen receptor-positive breast cancer

Her medical history is significant for migraines with aura.

PAST SURGICAL HISTORY:
She underwent a lumpectomy with sentinel lymph node biopsy.

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg daily
- Hydrochlorothiazide 25 mg daily
- Sumatriptan 50 mg PRN
- Acetaminophen 500 mg PRN

ALLERGIES:
NKDA

FAMILY HISTORY:
- Migraine headaches in mother
- Breast cancer in maternal aunt (premenopausal)
- Hypertension in father

SOCIAL HISTORY:
She is currently in a monogamous relationship

VITAL SIGNS:
Her blood pressure is 125/78 mmHg, and her BMI is 26 kg/m^2

PHYSICAL EXAMINATION:
Her breast examination is unremarkable post-surgery, and the rest of her physical examination is normal.

## Encounter 5: source 2023-07-15, practice 2024-07-11, outpatient, Neurology

Chief complaint: Increased frequency of severe recurrent headaches

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Increased frequency of severe recurrent headaches

HISTORY OF PRESENT ILLNESS:
The patient returns to Neurology with history of migraine, Cushing's and breast cancer, noting increased headache frequency over the past 6 months. She describes the headaches as unilateral, pulsating, and severe, occurring about twice a week and lasting for several hours. The headaches are often accompanied by nausea and sensitivity to light. She has tried over-the-counter ibuprofen and acetaminophen without significant relief.

PAST MEDICAL HISTORY:
Active Problem List:
- Migraine with aura (diagnosed 2020-01-15)
- Essential hypertension (diagnosed 2020-04-15)
- Adrenal Cushing's syndrome (diagnosed 2022-01-15)
- Malignant neoplasm of male breast (diagnosed 2023-01-15)
- Estrogen receptor-positive breast cancer

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg daily
- Hydrochlorothiazide 25 mg daily
- Sumatriptan 50 mg PRN
- Acetaminophen 500 mg PRN

ALLERGIES:
NKDA

FAMILY HISTORY:
- Migraine headaches in mother
- Breast cancer in maternal aunt (premenopausal)
- Hypertension in father

SOCIAL HISTORY:
She works as a graphic designer and reports that these headaches are affecting her ability to concentrate at work.

REVIEW OF SYSTEMS:
She denies any recent head trauma or changes in vision.

VITAL SIGNS:
Her blood pressure is 120/78 mmHg, heart rate is 72 beats per minute. Her BMI is 26 kg/m^2

PHYSICAL EXAMINATION:
Neurological examination is nonfocal. There is no neck stiffness or rash.

## Encounter 6: source 2024-07-15, practice 2025-07-01, outpatient, Neurology

Chief complaint: Menstrual-associated headaches with nausea

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Aisha Johnson, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Menstrual-associated headaches with nausea

HISTORY OF PRESENT ILLNESS:
The patient with migraine with aura, hypertension, Cushing's syndrome, and breast cancer returns to Neurology for headache follow-up, last seen here in July 2023. She describes the headaches as severe, unilateral, and pulsatile, often accompanied by nausea and sensitivity to light and sound. She notes that the headaches tend to occur around her menstrual periods and are sometimes preceded by visual disturbances such as flashing lights. She denies any recent head trauma, fever, or changes in vision outside of her headache episodes.

PAST MEDICAL HISTORY:
Active Problem List:
- Migraine with aura (diagnosed 2020-01-15)
- Essential hypertension (diagnosed 2020-04-15)
- Adrenal Cushing's syndrome (diagnosed 2022-01-15)
- Malignant neoplasm of male breast (diagnosed 2023-01-15)
- Estrogen receptor-positive breast cancer

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg daily
- Hydrochlorothiazide 25 mg daily
- Sumatriptan 50 mg PRN
- Acetaminophen 500 mg PRN

ALLERGIES:
NKDA

FAMILY HISTORY:
- Migraine headaches in mother
- Breast cancer in maternal aunt (premenopausal)
- Hypertension in father

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social. Occupation: Graphic designer.

VITAL SIGNS:
On physical examination, her blood pressure is 120/78 mmHg, heart rate is 72 beats per minute, and temperature is 98.6°F (37°C)

PHYSICAL EXAMINATION:
Neurologic examination, including cranial nerves, motor, sensory, coordination, and gait, is normal. Fundoscopic examination is also normal.

## Encounter 7: source 2025-07-15, practice 2026-07-16, outpatient, Neurology

Chief complaint: Recurrent headaches with photophobia and phonophobia

ENCOUNTER NOTE
Date: 2025-07-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Recurrent headaches with photophobia and phonophobia

HISTORY OF PRESENT ILLNESS:
The patient is a 37-year-old female with a known history of migraine with aura dating back to 2020 and recent follow-up visits in this clinic in 2023 and 2024, who returns for continued evaluation of recurrent headaches. The headaches are described as moderate to severe, pulsating in nature, unilateral, and often last from 6 to 24 hours if untreated. She reports that her headaches are frequently preceded by flashes of light in her visual field and are accompanied by nausea, photophobia, and phonophobia. These episodes tend to occur around her menstrual cycle and sometimes follow the consumption of red wine or cheese. She prefers to lie down in a quiet, dark room during attacks. There is no history of trauma, fever, neck stiffness, or neurological deficits.

PAST MEDICAL HISTORY:
Active Problem List:
- Migraine with aura (diagnosed 2020-01-15)
- Essential hypertension (diagnosed 2020-04-15)
- Adrenal Cushing's syndrome (diagnosed 2022-01-15)
- Malignant neoplasm of male breast (diagnosed 2023-01-15)
- Estrogen receptor-positive breast cancer

MEDICATIONS:
She does not take any medications other than occasional acetaminophen for headache relief.

Home Medications:
- Lisinopril 10 mg daily
- Hydrochlorothiazide 25 mg daily
- Sumatriptan 50 mg PRN

ALLERGIES:
NKDA

FAMILY HISTORY:
- Migraine headaches in mother
- Breast cancer in maternal aunt (premenopausal)
- Hypertension in father

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social. Occupation: Graphic designer.

VITAL SIGNS:
Vital signs are within normal limits.

PHYSICAL EXAMINATION:
Neurological examination is normal
