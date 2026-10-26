# synthetic_hospital patient 1790

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 28 at first encounter, sex F, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2019-05-15 (Wed) | outpatient | Endocrinology |
| 1 | 2020-05-15 | 2019-09-10 (Tue) | outpatient | Internal Medicine |
| 2 | 2021-07-15 | 2020-11-17 (Tue) | outpatient | Obstetrics & Gynecology |
| 3 | 2022-03-15 | 2021-07-23 (Fri) | outpatient | Obstetrics & Gynecology |
| 4 | 2024-01-15 | 2023-05-11 (Thu) | icu | Emergency Medicine |
| 5 | 2025-01-15 | 2024-05-22 (Wed) | ed | Emergency Medicine |
| 6 | 2027-01-15 | 2026-05-25 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Sulfonamides (anaphylaxis)",
  "Latex (contact dermatitis)"
 ],
 "insurance": "Medicaid",
 "occupation": "Graphic designer",
 "alcohol_use": "Occasional social use",
 "family_history": [
  "Father: Epilepsy",
  "Mother: Rheumatoid arthritis, type 2 diabetes mellitus",
  "Sister: Major depressive disorder",
  "Maternal aunt: Breast cancer (age 45)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "300 mg PO at bedtime",
   "name": "Phenytoin"
  },
  {
   "dose": "0.1 mg PO twice daily",
   "name": "Desmopressin acetate"
  },
  {
   "dose": "1 tablet PO daily",
   "name": "Biktarvy (bictegravir/tenofovir alafenamide/emtricitabine)"
  },
  {
   "dose": "100 mg PO daily",
   "name": "Sertraline"
  },
  {
   "dose": "1 mg PO daily",
   "name": "Folic acid"
  },
  {
   "dose": "1 tablet PO daily",
   "name": "Prenatal vitamin with iron"
  }
 ],
 "surgical_history": [
  "Laparoscopic salpingectomy for ruptured ectopic pregnancy (age 26)",
  "Dilation and curettage (age 27)"
 ],
 "chronic_conditions": [
  "Epilepsy (childhood onset, generalized seizure disorder)",
  "Central diabetes insipidus",
  "HIV infection (advanced, CDC Stage 3, diagnosed 3 years prior)",
  "Major depressive disorder (recurrent, severe)",
  "History of methotrexate-induced folate deficiency"
 ],
 "age_at_first_encounter": 28
}
```

Primary diagnoses: ["Central diabetes insipidus (inadequately controlled)", "Epilepsy", "Methotrexate-induced folate deficiency", "Diabetes insipidus in pregnancy (gestational exacerbation of central diabetes insipidus)", "Major depressive disorder with psychotic features", "Cerebral toxoplasmosis"]
Comorbidities: ["Medication noncompliance", "Epilepsy"]

## Encounter 0: source 2020-01-15, practice 2019-05-15, outpatient, Endocrinology

Chief complaint: 28-year-old woman presents with 3-month history of excessive urination and persistent thirst, drinking up to 6 liters of water daily

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
28-year-old woman presents with 3-month history of excessive urination and persistent thirst, drinking up to 6 liters of water daily

HISTORY OF PRESENT ILLNESS:
A 28-year-old woman presents to the clinic with a 3-month history of excessive urination and persistent thirst. She reports drinking up to 6 liters of water daily and waking up multiple times at night to urinate. She denies any recent head trauma or surgery.

PAST MEDICAL HISTORY:
Active Problem List:
- Epilepsy (childhood onset, generalized seizure disorder)
- Central diabetes insipidus
- HIV infection (advanced, CDC Stage 3, diagnosed 3 years prior)
- Major depressive disorder (recurrent, severe)
- History of methotrexate-induced folate deficiency

PAST SURGICAL HISTORY:
- Laparoscopic salpingectomy for ruptured ectopic pregnancy (age 26)
- Dilation and curettage (age 27)

MEDICATIONS:
She is.

Home Medications:
- Phenytoin 300 mg PO at bedtime
- Desmopressin acetate 0.1 mg PO twice daily
- Biktarvy (bictegravir/tenofovir alafenamide/emtricitabine) 1 tablet PO daily
- Sertraline 100 mg PO daily
- Folic acid 1 mg PO daily
- Prenatal vitamin with iron 1 tablet PO daily

ALLERGIES:
Sulfonamides (anaphylaxis), Latex (contact dermatitis)

FAMILY HISTORY:
- Father: Epilepsy
- Mother: Rheumatoid arthritis, type 2 diabetes mellitus
- Sister: Major depressive disorder
- Maternal aunt: Breast cancer (age 45)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use. Occupation: Graphic designer.

PHYSICAL EXAMINATION:
Physical examination is normal.

LABORATORY DATA:
Laboratory tests reveal:

- Serum sodium: 148 mEq/L
- Serum osmolality: 300 mOsm/kg
- Urine osmolality: 150 mOsm/kg

ASSESSMENT AND PLAN:
Based on the patient's presentation and laboratory findings,

## Encounter 1: source 2020-05-15, practice 2019-09-10, outpatient, Internal Medicine

Chief complaint: Routine follow-up for fatigue and gum swelling/bleeding in the setting of methotrexate-induced folate deficiency

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. Michael Rodriguez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for fatigue and gum swelling/bleeding in the setting of methotrexate-induced folate deficiency

HISTORY OF PRESENT ILLNESS:
The patient with epilepsy, HIV, and methotrexate-induced folate deficiency returns for follow-up, seen in Endocrinology four months ago. She reports feeling fatigued for the past two months and has noticed her gums appear swollen and bleed easily when brushing her teeth. She denies any changes in diet, bowel habits, or weight loss. Her menstrual cycles are regular.

PAST MEDICAL HISTORY:
Active Problem List:
- Central diabetes insipidus (inadequately controlled) (diagnosed 2020-01-15)
- Epilepsy (childhood onset, generalized seizure disorder)
- Central diabetes insipidus
- HIV infection (advanced, CDC Stage 3, diagnosed 3 years prior)
- Major depressive disorder (recurrent, severe)
- History of methotrexate-induced folate deficiency

PAST SURGICAL HISTORY:
- Laparoscopic salpingectomy for ruptured ectopic pregnancy (age 26)
- Dilation and curettage (age 27)

MEDICATIONS:
Home Medications:
- Phenytoin 300 mg PO at bedtime
- Desmopressin acetate 0.1 mg PO twice daily
- Biktarvy (bictegravir/tenofovir alafenamide/emtricitabine) 1 tablet PO daily
- Sertraline 100 mg PO daily
- Folic acid 1 mg PO daily
- Prenatal vitamin with iron 1 tablet PO daily

ALLERGIES:
Sulfonamides (anaphylaxis), Latex (contact dermatitis)

FAMILY HISTORY:
- Father: Epilepsy
- Mother: Rheumatoid arthritis, type 2 diabetes mellitus
- Sister: Major depressive disorder
- Maternal aunt: Breast cancer (age 45)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use. Occupation: Graphic designer.

PHYSICAL EXAMINATION:
On physical examination, her gums are erythematous and hyperplastic.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 10.2 g/dL (Normal: 12-16 g/dL)
- Mean corpuscular volume (MCV): 102 fL (Normal: 80-96 fL)
- Serum folate: 3 ng/mL (Normal: 5-20 ng/mL)
- Serum calcium: 9.2 mg/dL (Normal: 8.5-10.2 mg/dL)
- 25-hydroxyvitamin D: 30 ng/mL (Normal: 30-100 ng/mL)

## Encounter 2: source 2021-07-15, practice 2020-11-17, outpatient, Obstetrics & Gynecology

Chief complaint: Preconception counseling regarding epilepsy medication effects on future pregnancy

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Aisha Patel, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Preconception counseling regarding epilepsy medication effects on future pregnancy

HISTORY OF PRESENT ILLNESS:
The 29-year-old patient with epilepsy, central diabetes insipidus, and methotrexate-induced folate deficiency returns for preconception counseling after care for polyuria and fatigue. She expresses desire to start a family and is concerned about medication effects on future pregnancy. She denies recent illnesses, medication changes, or missed doses of phenytoin.

PAST MEDICAL HISTORY:
Active Problem List:
- Central diabetes insipidus (inadequately controlled) (diagnosed 2020-01-15)
- Methotrexate-induced folate deficiency (diagnosed 2020-05-15)
- Epilepsy (childhood onset, generalized seizure disorder)
- Central diabetes insipidus
- HIV infection (advanced, CDC Stage 3, diagnosed 3 years prior)
- Major depressive disorder (recurrent, severe)

She has a history of epilepsy, well-controlled on phenytoin for the past 5 years, with her last seizure occurring over 3 years ago.

PAST SURGICAL HISTORY:
- Laparoscopic salpingectomy for ruptured ectopic pregnancy (age 26)
- Dilation and curettage (age 27)

MEDICATIONS:
Her current medications include phenytoin and a multivitamin.

Home Medications:
- Desmopressin acetate 0.1 mg PO twice daily
- Biktarvy (bictegravir/tenofovir alafenamide/emtricitabine) 1 tablet PO daily
- Sertraline 100 mg PO daily
- Folic acid 1 mg PO daily
- Prenatal vitamin with iron 1 tablet PO daily

ALLERGIES:
Sulfonamides (anaphylaxis), Latex (contact dermatitis)

FAMILY HISTORY:
- Father: Epilepsy
- Mother: Rheumatoid arthritis, type 2 diabetes mellitus
- Sister: Major depressive disorder
- Maternal aunt: Breast cancer (age 45)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use. Occupation: Graphic designer.

PHYSICAL EXAMINATION:
Her physical examination is unremarkable.

LABORATORY DATA:
Her most recent laboratory tests are as follows:

- Complete blood count: within normal limits
- Liver function tests: within normal limits
- Serum phenytoin level: within therapeutic range

ASSESSMENT AND PLAN:
Given her medical history and current desire to conceive,

## Encounter 3: source 2022-03-15, practice 2021-07-23, outpatient, Obstetrics & Gynecology

Chief complaint: Increased thirst and nocturia (5-6 times nightly) during established pregnancy

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. Aisha Patel, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Increased thirst and nocturia (5-6 times nightly) during established pregnancy

HISTORY OF PRESENT ILLNESS:
The patient returns following preconception counseling with central diabetes insipidus. She reports having to wake up five to six times at night to urinate and feels that she cannot drink enough water to quench her thirst.

PAST MEDICAL HISTORY:
Active Problem List:
- Central diabetes insipidus (inadequately controlled) (diagnosed 2020-01-15)
- Methotrexate-induced folate deficiency (diagnosed 2020-05-15)
- Epilepsy (diagnosed 2021-07-15)
- Epilepsy (childhood onset, generalized seizure disorder)
- Central diabetes insipidus
- HIV infection (advanced, CDC Stage 3, diagnosed 3 years prior)
- Major depressive disorder (recurrent, severe)

Her past medical history is significant for mild diabetes insipidus, which was well controlled with desmopressin until recently.

PAST SURGICAL HISTORY:
- Laparoscopic salpingectomy for ruptured ectopic pregnancy (age 26)
- Dilation and curettage (age 27)

MEDICATIONS:
She denies any use of medications other than prenatal vitamins

Home Medications:
- Phenytoin 300 mg PO at bedtime
- Desmopressin acetate 0.1 mg PO twice daily
- Biktarvy (bictegravir/tenofovir alafenamide/emtricitabine) 1 tablet PO daily
- Sertraline 100 mg PO daily
- Folic acid 1 mg PO daily
- Prenatal vitamin with iron 1 tablet PO daily

ALLERGIES:
She has no known drug allergies.

FAMILY HISTORY:
- Father: Epilepsy
- Mother: Rheumatoid arthritis, type 2 diabetes mellitus
- Sister: Major depressive disorder
- Maternal aunt: Breast cancer (age 45)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use. Occupation: Graphic designer.

PHYSICAL EXAMINATION:
On physical examination, her vital signs are within normal limits, and no abnormalities are noted.

LABORATORY DATA:
Laboratory tests reveal the following:

- Serum sodium: 140 mEq/L (normal range: 135-145 mEq/L)
- Serum osmolality: 280 mOsm/kg (normal range: 275-295 mOsm/kg)
- Urine specific gravity: 1.005 (normal range: 1.010-1.030)
- 24-hour urine volume: 4.5 L

## Encounter 4: source 2024-01-15, practice 2023-05-11, icu, Emergency Medicine

Chief complaint: Brought to ED after falling from second-story window during witnessed seizure, intubated for airway protection

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Brought to ED after falling from second-story window during witnessed seizure, intubated for airway protection

HISTORY OF PRESENT ILLNESS:
This 32-year-old female with childhood-onset generalized epilepsy and central diabetes insipidus previously complicated by gestational exacerbation during pregnancy in 2022 presents to the emergency department following a witnessed seizure and fall. On arrival, she is obtunded and postictal. Her roommate reports that the patient has had epilepsy since childhood and recently stopped taking her medications. She is intubated for airway protection. Over the next two hours, she receives 120 mL of isotonic saline but continues to have profuse urine output, totaling 1.4 liters.

PAST MEDICAL HISTORY:
Active Problem List:
- Central diabetes insipidus (inadequately controlled) (diagnosed 2020-01-15)
- Methotrexate-induced folate deficiency (diagnosed 2020-05-15)
- Epilepsy (diagnosed 2021-07-15)
- Diabetes insipidus in pregnancy (gestational exacerbation of central diabetes insipidus) (diagnosed 2022-03-15)
- Epilepsy (childhood onset, generalized seizure disorder)
- Central diabetes insipidus
- HIV infection (advanced, CDC Stage 3, diagnosed 3 years prior)
- Major depressive disorder (recurrent, severe)

PAST SURGICAL HISTORY:
- Laparoscopic salpingectomy for ruptured ectopic pregnancy (age 26)
- Dilation and curettage (age 27)

MEDICATIONS:
Home Medications:
- Phenytoin 300 mg PO at bedtime
- Desmopressin acetate 0.1 mg PO twice daily
- Biktarvy (bictegravir/tenofovir alafenamide/emtricitabine) 1 tablet PO daily
- Sertraline 100 mg PO daily
- Folic acid 1 mg PO daily
- Prenatal vitamin with iron 1 tablet PO daily

ALLERGIES:
Sulfonamides (anaphylaxis), Latex (contact dermatitis)

FAMILY HISTORY:
- Father: Epilepsy
- Mother: Rheumatoid arthritis, type 2 diabetes mellitus
- Sister: Major depressive disorder
- Maternal aunt: Breast cancer (age 45)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use. Occupation: Graphic designer.

VITAL SIGNS:
Her temperature is 36.5°C (97.7°F), pulse is 108/min, and blood pressure is 138/90 mm Hg.

PHYSICAL EXAMINATION:
Physical examination reveals irregular respirations and sluggishly reactive pupils. There is no purposeful movement in response to painful stimuli, and she exhibits decerebrate posturing. Deep tendon reflexes are brisk.

LABORATORY DATA:
Laboratory studies are as follows:
Serum sodium                          151 mEq/L
Serum osmolality                      302 mOsm/kg
Urine specific gravity                1.002
Urine osmolality                        68 mOsm/kg
Glucose                                    110 mg/dL

IMAGING:
Head CT shows bilateral cerebral contusions and a small intraventricular hemorrhage.

## Encounter 5: source 2025-01-15, practice 2024-05-22, ed, Emergency Medicine

Chief complaint: Several days of worsening occipital headache and new-onset confusion with lethargy

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Fatima Al-Zahra, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Several days of worsening occipital headache and new-onset confusion with lethargy

HISTORY OF PRESENT ILLNESS:
This 33-year-old with advanced HIV, epilepsy, and central diabetes insipidus presents with headache and confusion. The pain is described as dull and located in the occipital region. She has had difficulty maintaining attention and is increasingly lethargic, per her roommate.

PAST MEDICAL HISTORY:
Active Problem List:
- Central diabetes insipidus (inadequately controlled) (diagnosed 2020-01-15)
- Methotrexate-induced folate deficiency (diagnosed 2020-05-15)
- Epilepsy (diagnosed 2021-07-15)
- Diabetes insipidus in pregnancy (gestational exacerbation of central diabetes insipidus) (diagnosed 2022-03-15)
- Epilepsy (childhood onset, generalized seizure disorder)
- Central diabetes insipidus
- HIV infection (advanced, CDC Stage 3, diagnosed 3 years prior)
- Major depressive disorder (recurrent, severe)

She was diagnosed with HIV 3 years ago but has been poorly adherent to antiretroviral therapy.

PAST SURGICAL HISTORY:
- Laparoscopic salpingectomy for ruptured ectopic pregnancy (age 26)
- Dilation and curettage (age 27)

MEDICATIONS:
Home Medications:
- Phenytoin 300 mg PO at bedtime
- Desmopressin acetate 0.1 mg PO twice daily
- Biktarvy (bictegravir/tenofovir alafenamide/emtricitabine) 1 tablet PO daily
- Sertraline 100 mg PO daily
- Folic acid 1 mg PO daily
- Prenatal vitamin with iron 1 tablet PO daily

ALLERGIES:
Sulfonamides (anaphylaxis), Latex (contact dermatitis)

FAMILY HISTORY:
- Father: Epilepsy
- Mother: Rheumatoid arthritis, type 2 diabetes mellitus
- Sister: Major depressive disorder
- Maternal aunt: Breast cancer (age 45)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use. Occupation: Graphic designer.

VITAL SIGNS:
She is afebrile, blood pressure is 110/65 mmHg, pulse is 94/min, and respiratory rate is 16/min.

PHYSICAL EXAMINATION:
On examination, she is oriented only to person. Fundoscopy reveals no papilledema.

LABORATORY DATA:
Her current CD4+ T cell count is 50/µL.

IMAGING:
A non-contrast CT scan of the head reveals four ring-enhancing lesions in the basal ganglia and subcortical white matter.

## Encounter 6: source 2027-01-15, practice 2026-05-25, ed, Emergency Medicine

Chief complaint: Brought by roommate after discovery of suicide note with 6-month history of worsening depression and weight loss

ENCOUNTER NOTE
Date: 2027-01-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Brought by roommate after discovery of suicide note with 6-month history of worsening depression and weight loss

HISTORY OF PRESENT ILLNESS:
The patient, who has childhood-onset epilepsy, central diabetes insipidus, advanced HIV, and recurrent severe major depressive disorder, presents after her roommate discovered a suicide note, with prior toxoplasmosis and seizures. The patient admits to writing the note and expresses a desire to end her life due to overwhelming feelings of worthlessness and hopelessness. She reports a 6-month history of depressed mood, anhedonia, significant weight loss, insomnia, and decreased concentration. She reveals that she has a detailed plan to overdose on her roommate's prescription medications and has already collected the pills.

PAST MEDICAL HISTORY:
Active Problem List:
- Central diabetes insipidus (inadequately controlled) (diagnosed 2020-01-15)
- Methotrexate-induced folate deficiency (diagnosed 2020-05-15)
- Epilepsy (diagnosed 2021-07-15)
- Diabetes insipidus in pregnancy (gestational exacerbation of central diabetes insipidus) (diagnosed 2022-03-15)
- Cerebral toxoplasmosis (diagnosed 2025-01-15)
- Epilepsy (childhood onset, generalized seizure disorder)
- Central diabetes insipidus
- HIV infection (advanced, CDC Stage 3, diagnosed 3 years prior)
- Major depressive disorder (recurrent, severe)

She has a history of major depressive disorder.

PAST SURGICAL HISTORY:
- Laparoscopic salpingectomy for ruptured ectopic pregnancy (age 26)
- Dilation and curettage (age 27)

MEDICATIONS:
She was previously on sertraline, which she stopped taking 3 months ago.

Home Medications:
- Phenytoin 300 mg PO at bedtime
- Desmopressin acetate 0.1 mg PO twice daily
- Biktarvy (bictegravir/tenofovir alafenamide/emtricitabine) 1 tablet PO daily
- Folic acid 1 mg PO daily
- Prenatal vitamin with iron 1 tablet PO daily

ALLERGIES:
Sulfonamides (anaphylaxis), Latex (contact dermatitis)

FAMILY HISTORY:
- Father: Epilepsy
- Mother: Rheumatoid arthritis, type 2 diabetes mellitus
- Sister: Major depressive disorder
- Maternal aunt: Breast cancer (age 45)

SOCIAL HISTORY:
She denies any substance abuse.

VITAL SIGNS:
Her vital signs are stable,

PHYSICAL EXAMINATION:
On examination, she is tearful, has poor eye contact, and speaks in a low, monotone voice. and physical examination is unremarkable.

LABORATORY DATA:
Laboratory tests, including a complete blood count, comprehensive metabolic panel, and thyroid function tests, are within normal limits.
