# synthetic_hospital patient 2703

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 52 at first encounter, sex F, Non-Hispanic White, Employer-sponsored preferred provider organization (PPO).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-06-15 (Tue) | outpatient | Rheumatology |
| 1 | 2020-07-15 | 2021-12-10 (Fri) | outpatient | Rheumatology |
| 2 | 2021-07-15 | 2022-12-19 (Mon) | outpatient | Rheumatology |
| 3 | 2023-01-15 | 2024-06-14 (Fri) | ed | Emergency Medicine |
| 4 | 2023-07-15 | 2024-12-17 (Tue) | outpatient | Endocrinology |
| 5 | 2024-01-15 | 2025-06-06 (Fri) | ed | Emergency Medicine |
| 6 | 2024-07-15 | 2025-12-04 (Thu) | outpatient | Obstetrics & Gynecology |
| 7 | 2025-01-15 | 2026-06-06 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Employer-sponsored preferred provider organization (PPO)",
 "occupation": "Administrative assistant",
 "alcohol_use": "None",
 "family_history": [
  "Mother: Rheumatoid arthritis, hypothyroidism",
  "Maternal aunt: Breast cancer (age 58)",
  "Father: Hypertension, myocardial infarction (age 68)",
  "Sister: Systemic lupus erythematosus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 2 packs per day for 20 years",
 "home_medications": [
  {
   "dose": "15 mg orally once weekly",
   "name": "Methotrexate"
  },
  {
   "dose": "25 mg orally daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "20 mg orally daily",
   "name": "Lisinopril"
  },
  {
   "dose": "75 mcg orally daily",
   "name": "Levothyroxine"
  },
  {
   "dose": "90 mcg (2 puffs) inhaled every 4-6 hours as needed for asthma symptoms",
   "name": "Albuterol HFA inhaler"
  },
  {
   "dose": "400 mg orally twice daily",
   "name": "Cimetidine"
  },
  {
   "dose": "81 mg orally occasional",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "Tonsillectomy (age 12)",
  "Appendectomy (age 30)"
 ],
 "chronic_conditions": [
  "Rheumatoid arthritis",
  "Systemic sclerosis",
  "Hypertension",
  "Asthma",
  "Gastroesophageal reflux disease",
  "Hypothyroidism"
 ],
 "age_at_first_encounter": 52
}
```

Primary diagnoses: ["Atlantoaxial subluxation secondary to rheumatoid arthritis", "Theophylline toxicity", "Ruptured posterior communicating artery aneurysm with subarachnoid hemorrhage", "Systemic sclerosis", "Autoimmune sialadenitis", "Secondary Hyperaldosteronism due to Dietary Sodium Restriction", "Dermatomyositis", "Invasive ductal carcinoma of right breast"]
Comorbidities: ["Gastroesophageal reflux disease", "Essential hypertension", "Rheumatoid arthritis"]

## Encounter 0: source 2020-01-15, practice 2021-06-15, outpatient, Rheumatology

Chief complaint: progressive tightening of the skin on her hands and face

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive tightening of the skin on her hands and face

HISTORY OF PRESENT ILLNESS:
A 45-year-old woman presents to the clinic with a 6-month history of progressive tightening of the skin on her hands and face. She reports that her fingers become white and painful when exposed to cold temperatures. She also mentions difficulty swallowing, especially solid foods, and has had episodes of heartburn.

PAST MEDICAL HISTORY:
Active Problem List:
- Rheumatoid arthritis
- Systemic sclerosis
- Hypertension
- Asthma
- Gastroesophageal reflux disease
- Hypothyroidism

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)
- Appendectomy (age 30)

MEDICATIONS:
Home Medications:
- Methotrexate 15 mg orally once weekly
- Hydrochlorothiazide 25 mg orally daily
- Lisinopril 20 mg orally daily
- Levothyroxine 75 mcg orally daily
- Albuterol HFA inhaler 90 mcg (2 puffs) inhaled every 4-6 hours as needed for asthma symptoms
- Cimetidine 400 mg orally twice daily
- Aspirin 81 mg orally occasional

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Rheumatoid arthritis, hypothyroidism
- Maternal aunt: Breast cancer (age 58)
- Father: Hypertension, myocardial infarction (age 68)
- Sister: Systemic lupus erythematosus

SOCIAL HISTORY:
Smoking: Current smoker, 2 packs per day for 20 years. Alcohol: None. Occupation: Administrative assistant.

VITAL SIGNS:
Her blood pressure is 150/95 mmHg.

PHYSICAL EXAMINATION:
On physical examination, you note thickened skin on her fingers extending proximally to the metacarpophalangeal joints and facial skin that appears shiny and taut with loss of expression lines.

LABORATORY DATA:
Laboratory results are as follows:

- Hemoglobin: 13.2 g/dL
- White blood cell count: 7,800 /μL
- Platelets: 250,000 /μL
- Creatinine: 0.9 mg/dL
- Antinuclear antibodies (ANA): Positive
- Anti-topoisomerase I (anti-Scl-70) antibodies: Positive

OTHER STUDIES:
Pulmonary function tests show a reduced diffusing capacity for carbon monoxide (DLCO).

## Encounter 1: source 2020-07-15, practice 2021-12-10, outpatient, Rheumatology

Chief complaint: progressive dry mouth and difficulty swallowing dry foods, gritty sensation in her eyes

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Sarah Chen, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive dry mouth and difficulty swallowing dry foods, gritty sensation in her eyes

HISTORY OF PRESENT ILLNESS:
A 52-year-old female with a history of rheumatoid arthritis and systemic sclerosis diagnosed in January returns to the Rheumatology clinic with a 6-month history of progressive dry mouth and difficulty swallowing dry foods. She also reports a gritty sensation in her eyes and has needed to use artificial tears multiple times a day.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic sclerosis (diagnosed 2020-01-15)
- Rheumatoid arthritis
- Hypertension
- Asthma
- Gastroesophageal reflux disease
- Hypothyroidism

She has a past medical history of hypothyroidism and rheumatoid arthritis.

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)
- Appendectomy (age 30)

MEDICATIONS:
Home Medications:
- Methotrexate 15 mg orally once weekly
- Hydrochlorothiazide 25 mg orally daily
- Lisinopril 20 mg orally daily
- Levothyroxine 75 mcg orally daily
- Albuterol HFA inhaler 90 mcg (2 puffs) inhaled every 4-6 hours as needed for asthma symptoms
- Cimetidine 400 mg orally twice daily
- Aspirin 81 mg orally occasional

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Rheumatoid arthritis, hypothyroidism
- Maternal aunt: Breast cancer (age 58)
- Father: Hypertension, myocardial infarction (age 68)
- Sister: Systemic lupus erythematosus

SOCIAL HISTORY:
Smoking: Current smoker, 2 packs per day for 20 years. Alcohol: None. Occupation: Administrative assistant.

PHYSICAL EXAMINATION:
On physical examination, you note dry oral mucosa and dental caries. Her Schirmer test shows decreased lacrimation with less than 5 mm of wetting after 5 minutes. Her parotid glands are mildly enlarged and non-tender.

LABORATORY DATA:
Laboratory studies are as follows:

- Complete Blood Count (CBC): Normal
- Erythrocyte Sedimentation Rate (ESR): Elevated
- Anti-nuclear Antibody (ANA): Positive
- Rheumatoid Factor (RF): Positive
- Anti-SSA (Ro): Positive
- Anti-SSB (La): Positive

## Encounter 2: source 2021-07-15, practice 2022-12-19, outpatient, Rheumatology

Chief complaint: increasing fatigue, difficulty climbing stairs, and a rash on her face and hands

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Increasing fatigue, difficulty climbing stairs, and a rash on her face and hands

HISTORY OF PRESENT ILLNESS:
The patient returns to the rheumatology clinic for follow-up of systemic sclerosis and autoimmune sialadenitis. She presents with a 3-month history of increasing fatigue, difficulty climbing stairs, and a rash on her face and hands. She reports that the rash worsens with sun exposure.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic sclerosis (diagnosed 2020-01-15)
- Autoimmune sialadenitis (diagnosed 2020-07-15)
- Rheumatoid arthritis
- Hypertension
- Asthma
- Gastroesophageal reflux disease
- Hypothyroidism

Her medical history is significant for hypertension, controlled with hydrochlorothiazide.

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)
- Appendectomy (age 30)

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Methotrexate 15 mg orally once weekly
- Lisinopril 20 mg orally daily
- Levothyroxine 75 mcg orally daily
- Albuterol HFA inhaler 90 mcg (2 puffs) inhaled every 4-6 hours as needed for asthma symptoms
- Cimetidine 400 mg orally twice daily
- Aspirin 81 mg orally occasional

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Rheumatoid arthritis, hypothyroidism
- Maternal aunt: Breast cancer (age 58)
- Father: Hypertension, myocardial infarction (age 68)
- Sister: Systemic lupus erythematosus

SOCIAL HISTORY:
Smoking: Current smoker, 2 packs per day for 20 years. Alcohol: None. Occupation: Administrative assistant.

PHYSICAL EXAMINATION:
On physical examination, you note a heliotrope rash with periorbital edema and erythematous, scaly plaques over the knuckles, known as Gottron's papules. Muscle strength testing reveals symmetrical proximal muscle weakness.

LABORATORY DATA:
Laboratory findings are as follows:

- Creatine kinase (CK): 980 U/L (Normal: 22-198 U/L)
- Aldolase: 23.5 U/L (Normal: 1.0-7.5 U/L)
- Antinuclear antibody (ANA): Positive
- Anti-Jo-1 antibody: Negative
- Erythrocyte sedimentation rate (ESR): 40 mm/hr (Normal: 0-20 mm/hr)

## Encounter 3: source 2023-01-15, practice 2024-06-14, ed, Emergency Medicine

Chief complaint: morning stiffness in her hands and symmetrical joint pain

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. James Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Morning stiffness in her hands and symmetrical joint pain

HISTORY OF PRESENT ILLNESS:
The patient is a 55-year-old female with rheumatoid arthritis, systemic sclerosis, and prior autoimmune sialadenitis and dermatomyositis who presents to the emergency department with a 6-month history of morning stiffness in her hands that lasts for more than an hour and symmetrical joint pain. She reports that her symptoms improve with activity. She also complains of recent onset neck pain and occasional numbness in her extremities.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic sclerosis (diagnosed 2020-01-15)
- Autoimmune sialadenitis (diagnosed 2020-07-15)
- Dermatomyositis (diagnosed 2021-07-15)
- Rheumatoid arthritis
- Hypertension
- Asthma
- Gastroesophageal reflux disease
- Hypothyroidism

Her medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)
- Appendectomy (age 30)

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Methotrexate 15 mg orally once weekly
- Lisinopril 20 mg orally daily
- Levothyroxine 75 mcg orally daily
- Albuterol HFA inhaler 90 mcg (2 puffs) inhaled every 4-6 hours as needed for asthma symptoms
- Cimetidine 400 mg orally twice daily
- Aspirin 81 mg orally occasional

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Rheumatoid arthritis, hypothyroidism
- Maternal aunt: Breast cancer (age 58)
- Father: Hypertension, myocardial infarction (age 68)
- Sister: Systemic lupus erythematosus

SOCIAL HISTORY:
Smoking: Current smoker, 2 packs per day for 20 years. Alcohol: None. Occupation: Administrative assistant.

PHYSICAL EXAMINATION:
On physical examination, there is noticeable swelling and tenderness over her metacarpophalangeal and proximal interphalangeal joints. There is also a limited range of motion in her cervical spine.

LABORATORY DATA:
Laboratory findings are as follows:

- Rheumatoid factor (RF): Positive
- Anti-cyclic citrullinated peptide (anti-CCP) antibodies: Positive
- Erythrocyte sedimentation rate (ESR): Elevated
- C-reactive protein (CRP): Elevated

IMAGING:
Radiographs of her hands show periarticular osteopenia and marginal erosions.

## Encounter 4: source 2023-07-15, practice 2024-12-17, outpatient, Endocrinology

Chief complaint: muscle weakness and fatigue

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Priya Patel, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Muscle weakness and fatigue

HISTORY OF PRESENT ILLNESS:
The patient returns after January ED visit and presents to the clinic with complaints of muscle weakness and fatigue for the past two weeks.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic sclerosis (diagnosed 2020-01-15)
- Autoimmune sialadenitis (diagnosed 2020-07-15)
- Dermatomyositis (diagnosed 2021-07-15)
- Atlantoaxial subluxation secondary to rheumatoid arthritis (diagnosed 2023-01-15)
- Rheumatoid arthritis
- Hypertension
- Asthma
- Gastroesophageal reflux disease
- Hypothyroidism

She has a history of hypertension.

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)
- Appendectomy (age 30)

MEDICATIONS:
She takes which is currently managed with hydrochlorothiazide.

Home Medications:
- Methotrexate 15 mg orally once weekly
- Lisinopril 20 mg orally daily
- Levothyroxine 75 mcg orally daily
- Albuterol HFA inhaler 90 mcg (2 puffs) inhaled every 4-6 hours as needed for asthma symptoms
- Cimetidine 400 mg orally twice daily
- Aspirin 81 mg orally occasional

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Rheumatoid arthritis, hypothyroidism
- Maternal aunt: Breast cancer (age 58)
- Father: Hypertension, myocardial infarction (age 68)
- Sister: Systemic lupus erythematosus

SOCIAL HISTORY:
Smoking: Current smoker, 2 packs per day for 20 years. Alcohol: None. Occupation: Administrative assistant.

REVIEW OF SYSTEMS:
She denies any headaches, palpitations, or excessive sweating.

VITAL SIGNS:
Her blood pressure is 148/92 mmHg.

PHYSICAL EXAMINATION:
Physical examination is unremarkable except for mild muscle tenderness.

LABORATORY DATA:
Laboratory tests reveal the following:

- Serum potassium: 3.1 mEq/L (Normal: 3.5-5.0 mEq/L)
- Serum sodium: 142 mEq/L (Normal: 135-145 mEq/L)
- Serum creatinine: 0.9 mg/dL (Normal: 0.6-1.1 mg/dL)
- Plasma renin activity: Low
- Plasma aldosterone concentration: Elevated

## Encounter 5: source 2024-01-15, practice 2025-06-06, ed, Emergency Medicine

Chief complaint: 2-week history of insomnia, tremors, and palpitations

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
2-week history of insomnia, tremors, and palpitations

HISTORY OF PRESENT ILLNESS:
The patient with systemic sclerosis, dermatomyositis, and atlantoaxial subluxation presents with insomnia, tremors, and palpitations. Her asthma was well-controlled with inhaled corticosteroids and a leukotriene receptor antagonist. A month ago, she experienced an asthma exacerbation, after which her physician added slow-release theophylline.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic sclerosis (diagnosed 2020-01-15)
- Autoimmune sialadenitis (diagnosed 2020-07-15)
- Dermatomyositis (diagnosed 2021-07-15)
- Atlantoaxial subluxation secondary to rheumatoid arthritis (diagnosed 2023-01-15)
- Secondary Hyperaldosteronism due to Dietary Sodium Restriction (diagnosed 2023-07-15)
- Rheumatoid arthritis
- Hypertension
- Asthma
- Gastroesophageal reflux disease
- Hypothyroidism

Medical history is notable for hypertension, gastroesophageal reflux disease and rheumatoid arthritis.

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)
- Appendectomy (age 30)

MEDICATIONS:
Current medications include albuterol, lisinopril, cimetidine, methotrexate and occasional aspirin.

Home Medications:
- Hydrochlorothiazide 25 mg orally daily
- Levothyroxine 75 mcg orally daily
- Albuterol HFA inhaler 90 mcg (2 puffs) inhaled every 4-6 hours as needed for asthma symptoms

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Rheumatoid arthritis, hypothyroidism
- Maternal aunt: Breast cancer (age 58)
- Father: Hypertension, myocardial infarction (age 68)
- Sister: Systemic lupus erythematosus

SOCIAL HISTORY:
Smoking: Current smoker, 2 packs per day for 20 years. Alcohol: None. Occupation: Administrative assistant.

REVIEW OF SYSTEMS:
She denies fever, chest pain, or shortness of breath.

VITAL SIGNS:
Temperature 37.1°C (98.8°F), blood pressure 124/78 mm Hg, pulse 104/min, respiratory rate 16/min

PHYSICAL EXAMINATION:
Physical examination reveals a fine tremor; her lungs are clear to auscultation.

LABORATORY DATA:
Laboratory tests are otherwise unremarkable.

OTHER STUDIES:
ECG shows sinus tachycardia.

## Encounter 6: source 2024-07-15, practice 2025-12-04, outpatient, Obstetrics & Gynecology

Chief complaint: spontaneous bloody discharge from her right nipple

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Jennifer Adams, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Spontaneous bloody discharge from her right nipple

HISTORY OF PRESENT ILLNESS:
The patient is a 56-year-old female with established rheumatoid arthritis, systemic sclerosis, and dermatomyositis who presents to the clinic with a three-month history of spontaneous bloody discharge from her right nipple. She also reports a palpable mass in her right breast that she discovered one month ago. She denies any fever, weight loss, or changes in skin over the breast.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic sclerosis (diagnosed 2020-01-15)
- Autoimmune sialadenitis (diagnosed 2020-07-15)
- Dermatomyositis (diagnosed 2021-07-15)
- Atlantoaxial subluxation secondary to rheumatoid arthritis (diagnosed 2023-01-15)
- Secondary Hyperaldosteronism due to Dietary Sodium Restriction (diagnosed 2023-07-15)
- Theophylline toxicity (diagnosed 2024-01-15)
- Rheumatoid arthritis
- Hypertension
- Asthma
- Gastroesophageal reflux disease
- Hypothyroidism

Her past medical history is significant for hypertension, controlled with hydrochlorothiazide.

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)
- Appendectomy (age 30)

MEDICATIONS:
Home Medications:
- Methotrexate 15 mg orally once weekly
- Hydrochlorothiazide 25 mg orally daily
- Lisinopril 20 mg orally daily
- Levothyroxine 75 mcg orally daily
- Albuterol HFA inhaler 90 mcg (2 puffs) inhaled every 4-6 hours as needed for asthma symptoms
- Cimetidine 400 mg orally twice daily
- Aspirin 81 mg orally occasional

ALLERGIES:
NKDA

FAMILY HISTORY:
She has no family history of breast cancer.

SOCIAL HISTORY:
Smoking: Current smoker, 2 packs per day for 20 years. Alcohol: None. Occupation: Administrative assistant.

PHYSICAL EXAMINATION:
On physical examination, a 2 cm firm, non-tender mass is palpated in the upper outer quadrant of the right breast. There is no axillary lymphadenopathy.

IMAGING:
Mammography shows a 2 cm irregular mass with speculated margins in the right breast. Ultrasound confirms a solid mass with irregular borders and increased vascularity. No other abnormalities are noted.

## Encounter 7: source 2025-01-15, practice 2026-06-06, ed, Emergency Medicine

Chief complaint: suddenly developing an excruciating headache

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Robert Liu, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Suddenly developing an excruciating headache

HISTORY OF PRESENT ILLNESS:
The patient is a 57-year-old female with a history of rheumatoid arthritis, systemic sclerosis, dermatomyositis, and recently diagnosed invasive ductal carcinoma of the right breast who is brought to the emergency department by her coworker after suddenly developing an excruciating headache. She had just finished a long day at the office and was climbing the stairs to the parking lot when she felt a sudden "explosion" inside her head, causing her to fall against the railing. She describes the pain as unlike anything she has ever experienced, calling it the "worst headache of her life." She remembers feeling nauseated immediately after the onset and briefly closing her eyes because the pain was so intense. She does not recall losing consciousness. Since the onset of pain, she has experienced persistent throbbing in her head, worsened by movement and light exposure. She reports blurry vision and double vision, particularly when trying to look to the right. She has also noticed difficulty keeping her left eye open, which her coworker confirms appeared droopy during the car ride to the hospital. She denies prior episodes of similar headache, recent illness, or head trauma.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic sclerosis (diagnosed 2020-01-15)
- Autoimmune sialadenitis (diagnosed 2020-07-15)
- Dermatomyositis (diagnosed 2021-07-15)
- Atlantoaxial subluxation secondary to rheumatoid arthritis (diagnosed 2023-01-15)
- Secondary Hyperaldosteronism due to Dietary Sodium Restriction (diagnosed 2023-07-15)
- Theophylline toxicity (diagnosed 2024-01-15)
- Invasive ductal carcinoma of right breast (diagnosed 2024-07-15)
- Rheumatoid arthritis
- Hypertension
- Asthma
- Gastroesophageal reflux disease
- Hypothyroidism

Her medical history is notable for 15 years of hypertension.

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)
- Appendectomy (age 30)

MEDICATIONS:
She takes for which she takes atenolol.

Home Medications:
- Methotrexate 15 mg orally once weekly
- Hydrochlorothiazide 25 mg orally daily
- Lisinopril 20 mg orally daily
- Levothyroxine 75 mcg orally daily
- Albuterol HFA inhaler 90 mcg (2 puffs) inhaled every 4-6 hours as needed for asthma symptoms
- Cimetidine 400 mg orally twice daily
- Aspirin 81 mg orally occasional

ALLERGIES:
NKDA

FAMILY HISTORY:
She has no known family history of aneurysms or connective tissue disorders, though her mother died suddenly of a stroke at age 60.

SOCIAL HISTORY:
She has smoked two packs of cigarettes daily for the past 20 years but denies alcohol or illicit drug use. She has not had regular health checkups in several years.

VITAL SIGNS:
Her vital signs show a blood pressure of 152/88 mm Hg, pulse 80/min, respiratory rate 16/min, and oxygen saturation of 98% on room air. She is afebrile.

PHYSICAL EXAMINATION:
On arrival, she appears in moderate distress, holding her head and shielding her eyes from the bright overhead lights. On general inspection, she is alert and oriented but repeatedly asks the nurse to dim the lights. Neurologic examination shows left-sided ptosis, with the left eyelid drooping significantly compared to the right. The left pupil is 6.5 mm and sluggishly reactive to light, whereas the right pupil is 4.5 mm and briskly reactive. Upward gaze and adduction of the left eye are limited, while abduction remains intact. The right eye moves normally in all directions. When asked to look to the right, she complains of double vision. Facial sensation is intact, speech is fluent, and motor strength is normal in all extremities. Sensory testing is unremarkable, and reflexes are symmetric. Cardiac exam reveals a regular rate and rhythm without murmurs. Lungs are clear to auscultation. No carotid bruits are present. The remainder of the physical examination is unremarkable.

IMAGING:
A noncontrast CT scan of the head is shown.
