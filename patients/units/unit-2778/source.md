# synthetic_hospital patient 2778

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 42 at first encounter, sex F, Non-Hispanic White, Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-10-07 (Wed) | outpatient | Psychiatry |
| 1 | 2020-07-15 | 2021-04-14 (Wed) | outpatient | Rheumatology |
| 2 | 2021-07-15 | 2022-04-18 (Mon) | outpatient | Gastroenterology |
| 3 | 2023-01-15 | 2023-10-23 (Mon) | outpatient | Cardiology |
| 4 | 2025-01-15 | 2025-10-13 (Mon) | outpatient | Endocrinology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield PPO",
 "occupation": "Administrative assistant",
 "alcohol_use": "None",
 "family_history": [
  "Bipolar disorder in mother",
  "Migraine headaches in sister",
  "Hypothyroidism in maternal grandmother"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "300 mg orally twice daily",
   "name": "Lithium carbonate"
  },
  {
   "dose": "100 mg orally as needed for migraine headache (maximum 200 mg/day)",
   "name": "Sumatriptan"
  },
  {
   "dose": "5 mg orally daily",
   "name": "Amlodipine"
  },
  {
   "dose": "400 mg orally at bedtime",
   "name": "Magnesium oxide"
  }
 ],
 "surgical_history": [
  "Tonsillectomy as a child"
 ],
 "chronic_conditions": [
  "Bipolar disorder",
  "Migraine disorder",
  "Raynaud phenomenon"
 ],
 "age_at_first_encounter": 42
}
```

Primary diagnoses: ["Chronic vasospastic disorder (Raynaud's phenomenon/Buerger's disease)", "Vasospastic angina", "Lithium-induced thyroiditis with nephrogenic diabetes insipidus", "Magnesium-induced diarrhea", "Drug-induced hypothyroidism"]
Comorbidities: ["Benign headache or migraine", "Long term (current) use of immunosuppressants", "Bipolar Disorder"]

## Encounter 0: source 2020-01-15, practice 2020-10-07, outpatient, Psychiatry

Chief complaint: Routine follow-up for bipolar disorder with stable mood symptoms

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for bipolar disorder with stable mood symptoms

HISTORY OF PRESENT ILLNESS:
She has been stable on lithium therapy for the past two years with good control of her symptoms. She reports adherence to her medication regimen and denies any recent manic or depressive episodes. She has no complaints today, but she mentions experiencing increased thirst and urination over the past few weeks.

PAST MEDICAL HISTORY:
Active Problem List:
- Bipolar disorder
- Migraine disorder
- Raynaud phenomenon

PAST SURGICAL HISTORY:
- Tonsillectomy as a child

MEDICATIONS:
Home Medications:
- Lithium carbonate 300 mg orally twice daily
- Sumatriptan 100 mg orally as needed for migraine headache (maximum 200 mg/day)
- Amlodipine 5 mg orally daily
- Magnesium oxide 400 mg orally at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Bipolar disorder in mother
- Migraine headaches in sister
- Hypothyroidism in maternal grandmother

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative assistant.

PHYSICAL EXAMINATION:
Her physical examination is unremarkable.

LABORATORY DATA:
Her current laboratory values are as follows:

- Serum sodium: 142 mEq/L (normal: 135-145 mEq/L)
- Serum potassium: 4.1 mEq/L (normal: 3.5-5.1 mEq/L)
- Serum creatinine: 1.0 mg/dL (normal: 0.6-1.1 mg/dL)
- Serum calcium: 10.2 mg/dL (normal: 8.5-10.2 mg/dL)
- Serum lithium: 1.2 mEq/L (therapeutic range: 0.6-1.2 mEq/L)
- TSH: 5.5 μIU/mL (normal: 0.4-4.0 μIU/mL)

## Encounter 1: source 2020-07-15, practice 2021-04-14, outpatient, Rheumatology

Chief complaint: Follow-up for Raynaud phenomenon with new palpitations and flushing

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up for Raynaud phenomenon with new palpitations and flushing

HISTORY OF PRESENT ILLNESS:
This 42-year-old female with bipolar disorder and Raynaud phenomenon returns for continued rheumatology follow-up. Her most recent psychiatry encounter in January 2020 was notable for management of drug-induced hypothyroidism. She was started on a new medication 5 days ago, which she reports has helped reduce the frequency of her vasospastic episodes. However, she has noticed increased awareness of her heartbeat, even at rest, and occasional flushing of the face. She denies chest pain, dizziness, or leg swelling.

PAST MEDICAL HISTORY:
Active Problem List:
- Drug-induced hypothyroidism (diagnosed 2020-01-15)
- Bipolar disorder
- Migraine disorder
- Raynaud phenomenon

Her medical history is notable for migraine headaches, but she has not had a recent episode.

PAST SURGICAL HISTORY:
- Tonsillectomy as a child

MEDICATIONS:
She does not take any other medications.

Home Medications:
- Lithium carbonate 300 mg orally twice daily
- Sumatriptan 100 mg orally as needed for migraine headache (maximum 200 mg/day)
- Amlodipine 5 mg orally daily
- Magnesium oxide 400 mg orally at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Bipolar disorder in mother
- Migraine headaches in sister
- Hypothyroidism in maternal grandmother

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative assistant.

VITAL SIGNS:
Vital signs are stable.

PHYSICAL EXAMINATION:
Cardiopulmonary exam reveals no murmurs, rubs, or gallops. No crackles or wheezing are noted on lung auscultation. Extremities are warm with good peripheral pulses. Capillary refill is brisk. There is no peripheral edema.

OTHER STUDIES:
ECG performed as part of her baseline evaluation is unremarkable.

ASSESSMENT AND PLAN:
The physician explains that her symptoms are expected with this class of medication

She do not warrant discontinuation.

## Encounter 2: source 2021-07-15, practice 2022-04-18, outpatient, Gastroenterology

Chief complaint: Persistent loose stools and watery diarrhea for two weeks

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Aisha Patel, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent loose stools and watery diarrhea for two weeks

HISTORY OF PRESENT ILLNESS:
The patient is a 43-year-old female with bipolar disorder, migraine disorder, and Raynaud phenomenon who presents to Gastroenterology with persistent loose stools. She describes having 4 to 5 watery bowel movements daily without blood or mucus. She denies abdominal pain, weight loss, recent travel, sick contacts, food allergies, and recent dietary changes.

PAST MEDICAL HISTORY:
Active Problem List:
- Drug-induced hypothyroidism (diagnosed 2020-01-15)
- Chronic vasospastic disorder (Raynaud's phenomenon/Buerger's disease) (diagnosed 2020-07-15)
- Bipolar disorder
- Migraine disorder
- Raynaud phenomenon

She has a history of migraines, for which she recently began over-the-counter vitamin supplementation after reading about natural remedies. She is otherwise healthy and takes no prescription medications.

PAST SURGICAL HISTORY:
- Tonsillectomy as a child

MEDICATIONS:
When asked about supplements, the patient reports she has been taking a magnesium oxide capsule each night before bed.

Home Medications:
- Lithium carbonate 300 mg orally twice daily
- Sumatriptan 100 mg orally as needed for migraine headache (maximum 200 mg/day)
- Amlodipine 5 mg orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Bipolar disorder in mother
- Migraine headaches in sister
- Hypothyroidism in maternal grandmother

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative assistant.

VITAL SIGNS:
Vital signs are within normal limits.

PHYSICAL EXAMINATION:
Physical examination reveals a soft, non-tender abdomen with increased bowel sounds.

LABORATORY DATA:
Laboratory testing shows normal complete blood count, serum electrolytes, and thyroid-stimulating hormone. Stool cultures, ova and parasites, and fecal leukocyte testing are negative.

PLAN:
Colonoscopy is deferred due to her age and absence of red flags.

## Encounter 3: source 2023-01-15, practice 2023-10-23, outpatient, Cardiology

Chief complaint: Episodic chest pain at rest with constricting sensation

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. James O'Brien, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Episodic chest pain at rest with constricting sensation

HISTORY OF PRESENT ILLNESS:
The patient with a history of chronic vasospastic disorder evaluated in Rheumatology presents reporting chest pain that occurs at rest, usually in the early morning hours. She describes the pain as a constricting sensation that lasts for about 15 minutes and resolves spontaneously. She also reports episodes of her fingers turning white and then blue when exposed to cold temperatures or during emotional stress.

PAST MEDICAL HISTORY:
Active Problem List:
- Drug-induced hypothyroidism (diagnosed 2020-01-15)
- Chronic vasospastic disorder (Raynaud's phenomenon/Buerger's disease) (diagnosed 2020-07-15)
- Magnesium-induced diarrhea (diagnosed 2021-07-15)
- Bipolar disorder
- Migraine disorder
- Raynaud phenomenon

Her past medical history is significant for migraine headaches.

PAST SURGICAL HISTORY:
- Tonsillectomy as a child

MEDICATIONS:
Her medications include sumatriptan as needed for migraines.

Home Medications:
- Lithium carbonate 300 mg orally twice daily
- Amlodipine 5 mg orally daily
- Magnesium oxide 400 mg orally at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Bipolar disorder in mother
- Migraine headaches in sister
- Hypothyroidism in maternal grandmother

SOCIAL HISTORY:
She does not smoke

VITAL SIGNS:
On physical examination, her blood pressure is 130/85 mmHg, heart rate is 72 beats per minute, and respiratory rate is 16 breaths per minute.

PHYSICAL EXAMINATION:
Cardiac examination is unremarkable with no murmurs, rubs, or gallops. There is no peripheral edema.

LABORATORY DATA:
Cardiac biomarkers are within normal limits.

Laboratory Values:
- Troponin I: 50 mg/dL)
- Triglycerides: 100 mg/dL (normal <150 mg/dL)

OTHER STUDIES:
An electrocardiogram (ECG) performed during an episode of chest pain shows transient ST-segment elevation.

## Encounter 4: source 2025-01-15, practice 2025-10-13, outpatient, Endocrinology

Chief complaint: Persistent thirst, heat intolerance, and nocturia with weight loss

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Priya Sharma, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent thirst, heat intolerance, and nocturia with weight loss

HISTORY OF PRESENT ILLNESS:
47-year-old female with bipolar disorder on chronic lithium therapy and prior drug-induced hypothyroidism returns with concerns. She drinks about 4–5 liters of water per day and often wakes up at night to urinate. She also reports difficulty sleeping and a recent 5-pound weight loss without changes in appetite.

PAST MEDICAL HISTORY:
Active Problem List:
- Drug-induced hypothyroidism (diagnosed 2020-01-15)
- Chronic vasospastic disorder (Raynaud's phenomenon/Buerger's disease) (diagnosed 2020-07-15)
- Magnesium-induced diarrhea (diagnosed 2021-07-15)
- Vasospastic angina (diagnosed 2023-01-15)
- Bipolar disorder
- Migraine disorder
- Raynaud phenomenon

She has a history of bipolar disorder.

PAST SURGICAL HISTORY:
- Tonsillectomy as a child

MEDICATIONS:
She has been taking medication for the past several years.

Home Medications:
- Lithium carbonate 300 mg orally twice daily
- Sumatriptan 100 mg orally as needed for migraine headache (maximum 200 mg/day)
- Amlodipine 5 mg orally daily
- Magnesium oxide 400 mg orally at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Bipolar disorder in mother
- Migraine headaches in sister
- Hypothyroidism in maternal grandmother

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative assistant.

VITAL SIGNS:
Vital signs are within normal limits. BMI is 23 kg/m².

PHYSICAL EXAMINATION:
Physical examination reveals a fine tremor of the hands and mild lid lag, but no thyroid tenderness or enlargement.
