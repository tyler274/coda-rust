# synthetic_hospital patient 2752

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 40 at first encounter, sex F, Non-Hispanic White, Blue Cross PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-04-14 (Mon) | ed | Emergency Medicine |
| 1 | 2020-07-15 | 2025-10-01 (Wed) | ed | Emergency Medicine |
| 2 | 2021-01-15 | 2026-04-08 (Wed) | inpatient | Internal Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross PPO",
 "occupation": "International travel agent",
 "alcohol_use": "Occasional (1-2 drinks per month)",
 "family_history": [
  "Mother with history of deep vein thrombosis (age 45)",
  "Father with hypertension",
  "Paternal uncle with pulmonary embolism"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 20 pack-year history (1 pack per day)",
 "home_medications": [
  {
   "dose": "1 tablet orally daily",
   "name": "Combined oral contraceptive pill (ethinyl estradiol/norgestimate)"
  }
 ],
 "surgical_history": [
  "No prior surgeries"
 ],
 "chronic_conditions": [
  "Migraine disorder",
  "Mild major depressive disorder",
  "Menstrual irregularity",
  "Tobacco use disorder"
 ],
 "age_at_first_encounter": 40
}
```

Primary diagnoses: ["Pulmonary embolism", "Acute Pulmonary Embolism"]
Comorbidities: ["Irregular menstrual cycles", "Tobacco use disorder", "Delayed management of life-threatening respiratory failure", "Long-term oral contraceptive use"]

## Encounter 0: source 2020-01-15, practice 2025-04-14, ed, Emergency Medicine

Chief complaint: Sudden-onset shortness of breath and lightheadedness beginning while walking from her car to the hospital entrance, accompanied by palpitations and air hunger, following a recent 12-hour flight from Australia

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden-onset shortness of breath and lightheadedness beginning while walking from her car to the hospital entrance, accompanied by palpitations and air hunger, following a recent 12-hour flight from Australia

HISTORY OF PRESENT ILLNESS:
That began while walking from her car to the hospital entrance. She also reports palpitations and a sensation of "air hunger" that developed abruptly and has not improved with rest. She denies fever, cough, or sputum production. She recently returned from a 12-hour flight from Australia.

PAST MEDICAL HISTORY:
Active Problem List:
- Migraine disorder
- Mild major depressive disorder
- Menstrual irregularity
- Tobacco use disorder

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
She takes a combined oral contraceptive pill for irregular menses.

Home Medications:
- Combined oral contraceptive pill (ethinyl estradiol/norgestimate) 1 tablet orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with history of deep vein thrombosis (age 45)
- Father with hypertension
- Paternal uncle with pulmonary embolism

SOCIAL HISTORY:
She has smoked one pack of cigarettes daily for the past 20 years

VITAL SIGNS:
Her temperature is 37.0°C (98.6°F), blood pressure is 106/68 mmHg, heart rate is 114/min, respiratory rate is 28/min, and oxygen saturation is 91% on room air.

PHYSICAL EXAMINATION:
On exam, she is tachypneic and mildly anxious.
Lung auscultation reveals clear breath sounds.

LABORATORY DATA:
Arterial blood gas shows elevated pH and decreased PaCO₂. D-dimer is elevated,

IMAGING:
Chest x-ray is normal,
and CT pulmonary angiography confirms a right segmental pulmonary embolism.

OTHER STUDIES:
ECG shows only sinus tachycardia.

## Encounter 1: source 2020-07-15, practice 2025-10-01, ed, Emergency Medicine

Chief complaint: Sudden-onset shortness of breath and palpitations that began an hour ago while walking up the stairs, with worsening lightheadedness and anxiety

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden-onset shortness of breath and palpitations that began an hour ago while walking up the stairs, with worsening lightheadedness and anxiety

HISTORY OF PRESENT ILLNESS:
The patient is a 40-year-old female with pulmonary embolism in January 2020 on OCPs who returns with similar symptoms that began an hour ago while she was walking up the stairs. She says her symptoms have worsened over the past hour, and she now feels lightheaded and anxious. She denies chest pain, cough, fever, or recent illness.

PAST MEDICAL HISTORY:
Active Problem List:
- Pulmonary embolism (diagnosed 2020-01-15)
- Migraine disorder
- Mild major depressive disorder
- Menstrual irregularity
- Tobacco use disorder

Her medical history is significant for migraines and mild depression.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
She has been taking a combined oral contraceptive pill for the past 8 years to manage her irregular menses

Home Medications:
- Combined oral contraceptive pill (ethinyl estradiol/norgestimate) 1 tablet orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with history of deep vein thrombosis (age 45)
- Father with hypertension
- Paternal uncle with pulmonary embolism

SOCIAL HISTORY:
She has a 20-pack-year smoking history. She works as a travel agent and recently returned from a 10-hour flight from South America.

VITAL SIGNS:
Her respiratory rate is 28/min, heart rate is 112/min, and oxygen saturation is 92% on room air.

PHYSICAL EXAMINATION:
On physical exam, she appears anxious and mildly diaphoretic. She has no jugular venous distension, heart sounds are normal, and lung exam reveals clear breath sounds bilaterally. There is no wheezing, crackles, or rhonchi. Her calves are soft and non-tender, and there is no asymmetry in leg circumference.

LABORATORY DATA:
Arterial blood gas shows an elevated pH. D-dimer testing is elevated.

IMAGING:
A chest x-ray is performed and shows no infiltrates, effusion, or consolidation.

OTHER STUDIES:
ECG shows sinus tachycardia but is otherwise unremarkable.

## Encounter 2: source 2021-01-15, practice 2026-04-08, inpatient, Internal Medicine

Chief complaint: Sudden-onset shortness of breath and chest tightness that worsens with inspiration, beginning acutely while shopping

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Priya Patel, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Sudden-onset shortness of breath and chest tightness that worsens with inspiration, beginning acutely while shopping

HISTORY OF PRESENT ILLNESS:
The patient is a 41-year-old female with a history of pulmonary embolism in January 2020 and July 2020 who presents with recurrent respiratory symptoms. She reports that this presentation mirrors her prior episodes. She states that it feels harder to breathe than usual and denies fever, sputum production, or prior lung problems. She notes a sensation of "tightness" in her chest that worsens with deep breathing. She denies leg swelling but reports mild right calf soreness yesterday that resolved.

PAST MEDICAL HISTORY:
Active Problem List:
- Pulmonary embolism (diagnosed 2020-01-15)
- Migraine disorder
- Mild major depressive disorder
- Menstrual irregularity
- Tobacco use disorder

She has no prior history of asthma or COPD.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Her only medication is a combined oral contraceptive pill, which she started 6 months ago.

Home Medications:
- Combined oral contraceptive pill (ethinyl estradiol/norgestimate) 1 tablet orally daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with history of deep vein thrombosis (age 45)
- Father with hypertension
- Paternal uncle with pulmonary embolism

SOCIAL HISTORY:
She is a smoker with a 15-pack-year history and works a desk job with prolonged sitting. One week ago, she flew from Los Angeles to Tokyo.

VITAL SIGNS:
Her temperature is 36.8°C (98.2°F), blood pressure is 108/72 mmHg, pulse is 116/min, respiratory rate is 30/min, and oxygen saturation is 91% on room air.

PHYSICAL EXAMINATION:
On physical exam, she is anxious, mildly tachypneic, and using accessory muscles of respiration.
Cardiac exam reveals a regular rhythm with a loud second heart sound; lung fields are clear.

LABORATORY DATA:
ABG shows elevated pH and decreased PaCO₂.

IMAGING:
Chest x-ray is unremarkable.
CT pulmonary angiography reveals a wedge-shaped perfusion defect in the right lower lobe.

OTHER STUDIES:
ECG shows sinus tachycardia.
