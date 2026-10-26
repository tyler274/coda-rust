# synthetic_hospital patient 1871

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 28 at first encounter, sex F, Filipino, University student health insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-02-14 (Mon) | outpatient | Obstetrics & Gynecology |
| 1 | 2020-05-15 | 2022-06-17 (Fri) | outpatient | Obstetrics & Gynecology |
| 2 | 2020-06-15 | 2022-07-06 (Wed) | ed | Emergency Medicine |
| 3 | 2020-07-15 | 2022-08-10 (Wed) | ed | Emergency Medicine |
| 4 | 2021-01-15 | 2023-02-07 (Tue) | outpatient | Obstetrics & Gynecology |
| 5 | 2023-01-15 | 2025-02-18 (Tue) | ed | Emergency Medicine |
| 6 | 2024-01-15 | 2026-02-05 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Dust mites",
  "Pollen"
 ],
 "insurance": "University student health insurance",
 "occupation": "PhD candidate (Biology) and graduate research assistant",
 "alcohol_use": "Occasional (1-2 drinks/month), currently abstinent",
 "family_history": [
  "Mother with asthma",
  "Father with recurrent kidney stones",
  "Maternal grandmother with type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Filipino",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "90 mcg per actuation, 1-2 puffs every 4-6 hours as needed for wheezing or shortness of breath",
   "name": "Albuterol inhaler"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Mild persistent asthma",
  "Recurrent nephrolithiasis",
  "Seasonal allergic rhinitis"
 ],
 "age_at_first_encounter": 28
}
```

Primary diagnoses: ["Congenital toxoplasmosis", "Acute pyelonephritis in pregnancy", "Pregnancy with risk factors for cervical insufficiency requiring modified exercise regimen", "Mild persistent asthma with acute exacerbation, uncontrolled", "Tuberculosis", "Ureterolithiasis", "Atypical pneumonia (Mycoplasma, Chlamydophila, or Legionella)"]
Comorbidities: ["Recent travel to malaria-endemic region", "Pregnancy second trimester"]

## Encounter 0: source 2020-01-15, practice 2022-02-14, outpatient, Obstetrics & Gynecology

Chief complaint: Routine prenatal visit to discuss safety of continuing daily 5-mile running regimen during pregnancy

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Maria Santos, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine prenatal visit to discuss safety of continuing daily 5-mile running regimen during pregnancy

HISTORY OF PRESENT ILLNESS:
She is an avid runner and inquires about continuing her training regimen, which includes running 5 miles daily at a moderate pace. Her pregnancy has been uncomplicated thus far.

PAST MEDICAL HISTORY:
Active Problem List:
- Mild persistent asthma
- Recurrent nephrolithiasis
- Seasonal allergic rhinitis

MEDICATIONS:
Home Medications:
- Albuterol inhaler 90 mcg per actuation, 1-2 puffs every 4-6 hours as needed for wheezing or shortness of breath

ALLERGIES:
Dust mites, Pollen

FAMILY HISTORY:
However, she mentions that her mother had a "weak cervix" and delivered her brother prematurely.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks/month), currently abstinent. Occupation: PhD candidate (Biology) and graduate research assistant.

REVIEW OF SYSTEMS:
She denies any vaginal bleeding, contractions, or leakage of fluid.

VITAL SIGNS:
Her vital signs are within normal limits

PHYSICAL EXAMINATION:
Her physical examination is unremarkable. Her obstetric examination reveals a fundal height appropriate for gestational age, and fetal heart tones are regular.

## Encounter 1: source 2020-05-15, practice 2022-06-17, outpatient, Obstetrics & Gynecology

Chief complaint: Routine prenatal visit at 24 weeks gestation with mild fatigue

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. Maria Santos, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine prenatal visit at 24 weeks gestation with mild fatigue

HISTORY OF PRESENT ILLNESS:
The patient with a history of mild persistent asthma, recurrent nephrolithiasis, and seasonal allergic rhinitis returns for routine prenatal care at 24 weeks gestation following prior recommendations to modify her exercise regimen. She reports feeling well except for mild fatigue.

PAST MEDICAL HISTORY:
Active Problem List:
- Pregnancy with risk factors for cervical insufficiency requiring modified exercise regimen (diagnosed 2020-01-15)
- Mild persistent asthma
- Recurrent nephrolithiasis
- Seasonal allergic rhinitis

MEDICATIONS:
Home Medications:
- Albuterol inhaler 90 mcg per actuation, 1-2 puffs every 4-6 hours as needed for wheezing or shortness of breath

ALLERGIES:
She has no known drug allergies

FAMILY HISTORY:
- Mother with asthma
- Father with recurrent kidney stones
- Maternal grandmother with type 2 diabetes mellitus

SOCIAL HISTORY:
She has two cats at home and mentions that she recently adopted a stray cat, which she has been caring for, including cleaning its litter box.

VITAL SIGNS:
On examination, her vital signs are within normal limits

PHYSICAL EXAMINATION:
Her physical examination is unremarkable

LABORATORY DATA:
The patient's serology results are as follows:

- Toxoplasma IgG: Positive
- Toxoplasma IgM: Positive
- Rubella IgG: Positive
- Rubella IgM: Negative
- CMV IgG: Positive
- CMV IgM: Negative
- Parvovirus B19 IgG: Positive
- Parvovirus B19 IgM: Negative

IMAGING:
An obstetric ultrasound is performed, which reveals bilateral ventriculomegaly, intracranial calcifications, and mild fetal ascites. The fetus also appears to be small for gestational age.

## Encounter 2: source 2020-06-15, practice 2022-07-06, ed, Emergency Medicine

Chief complaint: Intermittent cough, wheezing, and shortness of breath worsened over past two weeks with minimal relief from albuterol

ENCOUNTER NOTE
Date: 2020-06-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Intermittent cough, wheezing, and shortness of breath worsened over past two weeks with minimal relief from albuterol

HISTORY OF PRESENT ILLNESS:
The patient is a 28-year-old female with mild persistent asthma who presents at approximately 26 weeks gestation following recent obstetric care for congenital toxoplasmosis. Her symptoms have worsened over the past two weeks. She reports that her symptoms are particularly severe at night and during her prenatal yoga classes. She has been using her inhaler more frequently, about 3-4 times per week, but with minimal relief. She expresses concern about the safety of medications during pregnancy but is worried about increasing symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Pregnancy with risk factors for cervical insufficiency requiring modified exercise regimen (diagnosed 2020-01-15)
- Congenital toxoplasmosis (diagnosed 2020-05-15)
- Mild persistent asthma
- Recurrent nephrolithiasis
- Seasonal allergic rhinitis

MEDICATIONS:
Albuterol inhaler as needed

ALLERGIES:
Dust mites, Pollen

FAMILY HISTORY:
- Mother with asthma
- Father with recurrent kidney stones
- Maternal grandmother with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks/month), currently abstinent. Occupation: PhD candidate (Biology) and graduate research assistant.

VITAL SIGNS:
Respiratory rate of 22 breaths per minute. Oxygen saturation is 96% on room air.

PHYSICAL EXAMINATION:
Auscultation of the lungs reveals bilateral expiratory wheezes.

## Encounter 3: source 2020-07-15, practice 2022-08-10, ed, Emergency Medicine

Chief complaint: Fever, chills, and right-sided flank pain with dysuria and increased urinary frequency

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Priya Sharma, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fever, chills, and right-sided flank pain with dysuria and increased urinary frequency

HISTORY OF PRESENT ILLNESS:
28-year-old female with recurrent nephrolithiasis and asthma, evaluated for exacerbation one month ago, presents to the emergency department with a 2-day history of fever, chills, and right-sided flank pain. She reports dysuria and increased urinary frequency.

PAST MEDICAL HISTORY:
Active Problem List:
- Pregnancy with risk factors for cervical insufficiency requiring modified exercise regimen (diagnosed 2020-01-15)
- Congenital toxoplasmosis (diagnosed 2020-05-15)
- Mild persistent asthma with acute exacerbation, uncontrolled (diagnosed 2020-06-15)
- Mild persistent asthma
- Recurrent nephrolithiasis
- Seasonal allergic rhinitis

She has had no complications during her current pregnancy.

MEDICATIONS:
Home Medications:
- Albuterol inhaler 90 mcg per actuation, 1-2 puffs every 4-6 hours as needed for wheezing or shortness of breath

ALLERGIES:
Dust mites, Pollen

FAMILY HISTORY:
- Mother with asthma
- Father with recurrent kidney stones
- Maternal grandmother with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks/month), currently abstinent. Occupation: PhD candidate (Biology) and graduate research assistant.

VITAL SIGNS:
On examination, her temperature is 38.9°C (102°F), blood pressure is 120/78 mmHg, pulse is 110/min, and respiratory rate is 18/min.

PHYSICAL EXAMINATION:
Costovertebral angle tenderness is noted on the right side.

LABORATORY DATA:
Urinalysis shows pyuria, bacteriuria, and positive leukocyte esterase. Urine culture is pending.

PLAN:
She is admitted and started on intravenous ceftriaxone. After 48 hours of treatment, she is afebrile and reports significant symptomatic improvement.

## Encounter 4: source 2021-01-15, practice 2023-02-07, outpatient, Obstetrics & Gynecology

Chief complaint: Persistent cough, night sweats, and 5 kg weight loss over 3 weeks

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Sarah Johnson, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent cough, night sweats, and 5 kg weight loss over 3 weeks

HISTORY OF PRESENT ILLNESS:
The patient with mild persistent asthma returns postpartum. She presents to the clinic with a 3-week history of persistent cough, night sweats, and a weight loss of 5 kg.

PAST MEDICAL HISTORY:
Active Problem List:
- Pregnancy with risk factors for cervical insufficiency requiring modified exercise regimen (diagnosed 2020-01-15)
- Congenital toxoplasmosis (diagnosed 2020-05-15)
- Mild persistent asthma with acute exacerbation, uncontrolled (diagnosed 2020-06-15)
- Acute pyelonephritis in pregnancy (diagnosed 2020-07-15)
- Mild persistent asthma
- Recurrent nephrolithiasis
- Seasonal allergic rhinitis

MEDICATIONS:
Home Medications:
- Albuterol inhaler 90 mcg per actuation, 1-2 puffs every 4-6 hours as needed for wheezing or shortness of breath

ALLERGIES:
Dust mites, Pollen

FAMILY HISTORY:
- Mother with asthma
- Father with recurrent kidney stones
- Maternal grandmother with type 2 diabetes mellitus

SOCIAL HISTORY:
She immigrated from a country with a high prevalence of tuberculosis 5 years ago.

VITAL SIGNS:
Temperature is 38.2°C, respiratory rate is 22 breaths per minute, and oxygen saturation is 97% on room air.

PHYSICAL EXAMINATION:
Lung auscultation reveals bilateral crackles.

LABORATORY DATA:
A complete blood count and basic metabolic panel are within normal limits.

OTHER STUDIES:
A tuberculin skin test shows an induration of 15 mm at 48 hours.

## Encounter 5: source 2023-01-15, practice 2025-02-18, ed, Emergency Medicine

Chief complaint: Acute onset severe right flank pain radiating to right groin associated with nausea

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Robert Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset severe right flank pain radiating to right groin associated with nausea

HISTORY OF PRESENT ILLNESS:
The patient with recurrent nephrolithiasis presents with severe right flank pain that started 6 hours ago and radiates to her right groin. She reports associated nausea but denies vomiting, fever, or urinary symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Pregnancy with risk factors for cervical insufficiency requiring modified exercise regimen (diagnosed 2020-01-15)
- Congenital toxoplasmosis (diagnosed 2020-05-15)
- Mild persistent asthma with acute exacerbation, uncontrolled (diagnosed 2020-06-15)
- Acute pyelonephritis in pregnancy (diagnosed 2020-07-15)
- Tuberculosis (diagnosed 2021-01-15)
- Mild persistent asthma
- Recurrent nephrolithiasis
- Seasonal allergic rhinitis

Her past medical history is significant for two previous episodes of nephrolithiasis.

MEDICATIONS:
Home Medications:
- Albuterol inhaler 90 mcg per actuation, 1-2 puffs every 4-6 hours as needed for wheezing or shortness of breath

ALLERGIES:
Dust mites, Pollen

FAMILY HISTORY:
- Mother with asthma
- Father with recurrent kidney stones
- Maternal grandmother with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks/month), currently abstinent. Occupation: PhD candidate (Biology) and graduate research assistant.

VITAL SIGNS:
On examination, she is afebrile with a blood pressure of 130/85 mmHg, heart rate of 95 beats per minute

PHYSICAL EXAMINATION:
, and is in obvious distress. Abdominal examination reveals right costovertebral angle tenderness.

LABORATORY DATA:
Urinalysis shows:
- Specific gravity: 1.015
- pH: 5.5
- Red blood cells: 10-15/high-power field
- White blood cells: 2-3/high-power field
- No bacteria or nitrites
- No casts or crystals

ASSESSMENT AND PLAN:
Given the patient's pregnancy status and clinical presentation,

## Encounter 6: source 2024-01-15, practice 2026-02-05, ed, Emergency Medicine

Chief complaint: Dry cough, mild fever, and fatigue for 10 days with sore throat onset and recent dormitory exposure

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Dry cough, mild fever, and fatigue for 10 days with sore throat onset and recent dormitory exposure

HISTORY OF PRESENT ILLNESS:
The 32-year-old female with mild persistent asthma and recurrent nephrolithiasis, with prior ED visits for asthma exacerbation and ureterolithiasis, presents to the emergency department with a 10-day history of dry cough, mild fever, and fatigue. She notes a sore throat at the onset and mild headaches over the past week. She denies chest pain, shortness of breath, or hemoptysis. She lives in a dormitory and mentions that several roommates recently had similar symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Pregnancy with risk factors for cervical insufficiency requiring modified exercise regimen (diagnosed 2020-01-15)
- Congenital toxoplasmosis (diagnosed 2020-05-15)
- Mild persistent asthma with acute exacerbation, uncontrolled (diagnosed 2020-06-15)
- Acute pyelonephritis in pregnancy (diagnosed 2020-07-15)
- Tuberculosis (diagnosed 2021-01-15)
- Ureterolithiasis (diagnosed 2023-01-15)
- Mild persistent asthma
- Recurrent nephrolithiasis
- Seasonal allergic rhinitis

MEDICATIONS:
Home Medications:
- Albuterol inhaler 90 mcg per actuation, 1-2 puffs every 4-6 hours as needed for wheezing or shortness of breath

ALLERGIES:
Dust mites, Pollen

FAMILY HISTORY:
- Mother with asthma
- Father with recurrent kidney stones
- Maternal grandmother with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks/month), currently abstinent. Occupation: PhD candidate (Biology) and graduate research assistant.

VITAL SIGNS:
Vital signs show a temperature of 38.1°C (100.6°F), heart rate of 90 bpm, respiratory rate of 18/min, and blood pressure of 120/78 mm Hg.

PHYSICAL EXAMINATION:
Physical exam reveals scattered crackles in the lower lung fields but no wheezing.

LABORATORY DATA:
A rapid influenza test is negative.
Laboratory results show normal leukocyte count with mild lymphocytosis.

IMAGING:
Chest X-ray reveals bilateral patchy interstitial infiltrates.
