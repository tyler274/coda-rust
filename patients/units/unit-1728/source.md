# synthetic_hospital patient 1728

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex F, Non-Hispanic White, Commercial PPO insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-03-02 (Wed) | outpatient | Neurology |
| 1 | 2020-04-15 | 2022-05-26 (Thu) | outpatient | Obstetrics & Gynecology |
| 2 | 2021-01-15 | 2023-02-20 (Mon) | outpatient | Obstetrics & Gynecology |
| 3 | 2021-07-15 | 2023-08-22 (Tue) | outpatient | Obstetrics & Gynecology |
| 4 | 2022-01-15 | 2024-02-15 (Thu) | outpatient | Obstetrics & Gynecology |
| 5 | 2022-03-15 | 2024-04-24 (Wed) | ed | Emergency Medicine |
| 6 | 2022-05-15 | 2024-06-20 (Thu) | ed | Emergency Medicine |
| 7 | 2024-01-15 | 2026-02-16 (Mon) | outpatient | Obstetrics & Gynecology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Commercial PPO insurance",
 "occupation": "Project manager at a consulting firm",
 "alcohol_use": "Occasional (1-2 drinks per month)",
 "family_history": [
  "Von Hippel-Lindau disease in father (diagnosed age 35)",
  "Pheochromocytoma in paternal aunt",
  "Chronic hypertension in mother"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "10 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "90 mcg (1-2 puffs) as needed for asthma symptoms",
   "name": "Albuterol inhaler"
  },
  {
   "dose": "50 mg as needed at onset of migraine (max 2 doses per 24 hours)",
   "name": "Sumatriptan"
  }
 ],
 "surgical_history": [
  "Laparoscopic resection of androgen-secreting ovarian neoplasm (right ovary)",
  "Vaginal delivery at 34 weeks gestation (pregnancy complicated by preeclampsia)",
  "Vaginal delivery at 39 weeks gestation (uncomplicated)"
 ],
 "chronic_conditions": [
  "Essential hypertension",
  "Von Hippel-Lindau disease",
  "Asthma",
  "Migraine with aura"
 ],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Chronic hypertension with superimposed preeclampsia", "Benign neoplasm of ovary", "Von Hippel-Lindau disease", "High risk pregnancy for pre-eclampsia", "Essential hypertension", "Copper-containing intrauterine device", "Gestational diabetes mellitus", "Precipitous labor"]
Comorbidities: ["Pregnancy (28 weeks gestation)", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2022-03-02, outpatient, Neurology

Chief complaint: Sudden onset headaches and episodes of palpitations, sweating, and anxiety over the past 3 months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Patel, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Sudden onset headaches and episodes of palpitations, sweating, and anxiety over the past 3 months

HISTORY OF PRESENT ILLNESS:
She presents to the clinic with a complaint of sudden onset headaches and episodes of palpitations, sweating, and anxiety over the past 3 months.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension
- Von Hippel-Lindau disease
- Asthma
- Migraine with aura

PAST SURGICAL HISTORY:
- Laparoscopic resection of androgen-secreting ovarian neoplasm (right ovary)
- Vaginal delivery at 34 weeks gestation (pregnancy complicated by preeclampsia)
- Vaginal delivery at 39 weeks gestation (uncomplicated)

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg daily
- Albuterol inhaler 90 mcg (1-2 puffs) as needed for asthma symptoms
- Sumatriptan 50 mg as needed at onset of migraine (max 2 doses per 24 hours)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
She mentions that her father had a similar condition and underwent surgery for a brain tumor in his thirties.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per month). Occupation: Project manager at a consulting firm.

VITAL SIGNS:
Her blood pressure is 158/96 mmHg.

PHYSICAL EXAMINATION:
On fundoscopic examination, multiple retinal angiomas are noted.

LABORATORY DATA:
Laboratory results are as follows:

- Serum glucose: 90 mg/dL
- Serum sodium: 140 mEq/L
- Serum potassium: 3.9 mEq/L
- Serum calcium: 9.8 mg/dL
- Plasma metanephrines: Elevated
- Plasma normetanephrines: Elevated

IMAGING:
An MRI of the brain reveals a cerebellar mass consistent with a hemangioblastoma. Abdominal CT scan shows a 3 cm left adrenal mass.

ASSESSMENT AND PLAN:
Based on the clinical presentation and test results,

## Encounter 1: source 2020-04-15, practice 2022-05-26, outpatient, Obstetrics & Gynecology

Chief complaint: 6-month history of hirsutism, deepening of her voice, and male-pattern baldness with irregular menstrual cycles

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Maria Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
6-month history of hirsutism, deepening of her voice, and male-pattern baldness with irregular menstrual cycles

HISTORY OF PRESENT ILLNESS:
The patient diagnosed with Von Hippel-Lindau disease in January 2020 presents for evaluation. She reports a 6-month history of hirsutism, deepening of her voice, and male-pattern baldness. Her menstrual cycles have become irregular and she has not had a period for the last 3 months.

PAST MEDICAL HISTORY:
Active Problem List:
- Von Hippel-Lindau disease (diagnosed 2020-01-15)
- Essential hypertension
- Asthma
- Migraine with aura

PAST SURGICAL HISTORY:
- Laparoscopic resection of androgen-secreting ovarian neoplasm (right ovary)
- Vaginal delivery at 34 weeks gestation (pregnancy complicated by preeclampsia)
- Vaginal delivery at 39 weeks gestation (uncomplicated)

MEDICATIONS:
She denies any use of medications, supplements, or performance-enhancing drugs.

Home Medications:
- Lisinopril 10 mg daily
- Albuterol inhaler 90 mcg (1-2 puffs) as needed for asthma symptoms
- Sumatriptan 50 mg as needed at onset of migraine (max 2 doses per 24 hours)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Von Hippel-Lindau disease in father (diagnosed age 35)
- Pheochromocytoma in paternal aunt
- Chronic hypertension in mother

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per month). Occupation: Project manager at a consulting firm.

VITAL SIGNS:
Her blood pressure is 130/85 mmHg.

PHYSICAL EXAMINATION:
On physical examination, she has increased muscle mass, clitoromegaly, and acne.

LABORATORY DATA:
Laboratory tests reveal the following:

- Serum testosterone: 250 ng/dL (normal range: 15-70 ng/dL)
- DHEAS: 180 µg/dL (normal range: 35-430 µg/dL)
- LH: 7 mIU/mL (normal range: 2-12 mIU/mL)
- FSH: 8 mIU/mL (normal range: 3-20 mIU/mL)
- 17-hydroxyprogesterone: 200 ng/dL (normal range: 20-200 ng/dL)

## Encounter 2: source 2021-01-15, practice 2023-02-20, outpatient, Obstetrics & Gynecology

Chief complaint: Requesting contraception advice for reliable birth control without daily maintenance

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Jennifer Chen, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Requesting contraception advice for reliable birth control without daily maintenance

HISTORY OF PRESENT ILLNESS:
The patient is a 33-year-old female with Von Hippel-Lindau disease, essential hypertension, asthma, and migraine with aura who returns following her prior gynecologic evaluation for virilization and irregular menses. She is in a monogamous relationship and desires a reliable form of contraception without daily maintenance. Her menstrual cycles are regular, with her last menstrual period starting 10 days ago. She denies any recent sexually transmitted infections or pelvic inflammatory disease.

PAST MEDICAL HISTORY:
Active Problem List:
- Von Hippel-Lindau disease (diagnosed 2020-01-15)
- Benign neoplasm of ovary (diagnosed 2020-04-15)
- Essential hypertension
- Asthma
- Migraine with aura

She has a history of chronic hypertension, well-controlled on lisinopril, and migraines with aura.

PAST SURGICAL HISTORY:
- Laparoscopic resection of androgen-secreting ovarian neoplasm (right ovary)
- Vaginal delivery at 34 weeks gestation (pregnancy complicated by preeclampsia)
- Vaginal delivery at 39 weeks gestation (uncomplicated)

MEDICATIONS:
She takes her only medication is lisinopril.

Home Medications:
- Albuterol inhaler 90 mcg (1-2 puffs) as needed for asthma symptoms
- Sumatriptan 50 mg as needed at onset of migraine (max 2 doses per 24 hours)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Von Hippel-Lindau disease in father (diagnosed age 35)
- Pheochromocytoma in paternal aunt
- Chronic hypertension in mother

SOCIAL HISTORY:
She has no history of smoking.

VITAL SIGNS:
Her blood pressure today is 128/82 mmHg

PHYSICAL EXAMINATION:
On pelvic examination, her uterus is anteverted and non-tender, with no adnexal masses or tenderness.

## Encounter 3: source 2021-07-15, practice 2023-08-22, outpatient, Obstetrics & Gynecology

Chief complaint: Preconception consultation regarding safety of current antihypertensive medications during pregnancy

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. David Kim, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Preconception consultation regarding safety of current antihypertensive medications during pregnancy

HISTORY OF PRESENT ILLNESS:
The patient returns for preconception counseling with known Von Hippel-Lindau disease and history of benign ovarian neoplasm and copper IUD placement. She reports a regular menstrual cycle and is currently using condoms for contraception. She expresses a desire to conceive within the next few months and inquires about the safety of her current medications during pregnancy.

PAST MEDICAL HISTORY:
Active Problem List:
- Von Hippel-Lindau disease (diagnosed 2020-01-15)
- Benign neoplasm of ovary (diagnosed 2020-04-15)
- Copper-containing intrauterine device (diagnosed 2021-01-15)
- Essential hypertension
- Asthma
- Migraine with aura

PAST SURGICAL HISTORY:
- Laparoscopic resection of androgen-secreting ovarian neoplasm (right ovary)
- Vaginal delivery at 34 weeks gestation (pregnancy complicated by preeclampsia)
- Vaginal delivery at 39 weeks gestation (uncomplicated)

MEDICATIONS:
She has been well-controlled on extended-release nifedipine 60 mg daily for the past year.

Home Medications:
- Lisinopril 10 mg daily
- Albuterol inhaler 90 mcg (1-2 puffs) as needed for asthma symptoms
- Sumatriptan 50 mg as needed at onset of migraine (max 2 doses per 24 hours)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Von Hippel-Lindau disease in father (diagnosed age 35)
- Pheochromocytoma in paternal aunt
- Chronic hypertension in mother

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per month). Occupation: Project manager at a consulting firm.

VITAL SIGNS:
Her blood pressure today is 128/78 mmHg.

PHYSICAL EXAMINATION:
Her physical examination is unremarkable

LABORATORY DATA:
Her laboratory tests reveal normal renal function and electrolytes

## Encounter 4: source 2022-01-15, practice 2024-02-15, outpatient, Obstetrics & Gynecology

Chief complaint: Routine prenatal visit for high-risk pregnancy monitoring with no current complaints

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. David Kim, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine prenatal visit for high-risk pregnancy monitoring with no current complaints

HISTORY OF PRESENT ILLNESS:
The patient returns for routine prenatal monitoring, having previously presented in July 2021 for preconception counseling regarding antihypertensive medication management. She carries a history of Von Hippel-Lindau disease, essential hypertension, asthma, and migraine with aura. She has no complaints.

PAST MEDICAL HISTORY:
Active Problem List:
- Von Hippel-Lindau disease (diagnosed 2020-01-15)
- Benign neoplasm of ovary (diagnosed 2020-04-15)
- Copper-containing intrauterine device (diagnosed 2021-01-15)
- Essential hypertension (diagnosed 2021-07-15)
- Asthma
- Migraine with aura

She has a history of chronic hypertension and her first pregnancy was complicated by preeclampsia, which required delivery at 34 weeks.

PAST SURGICAL HISTORY:
- Laparoscopic resection of androgen-secreting ovarian neoplasm (right ovary)
- Vaginal delivery at 34 weeks gestation (pregnancy complicated by preeclampsia)
- Vaginal delivery at 39 weeks gestation (uncomplicated)

MEDICATIONS:
Her current medications include prenatal vitamins and labetalol.

Home Medications:
- Lisinopril 10 mg daily
- Albuterol inhaler 90 mcg (1-2 puffs) as needed for asthma symptoms
- Sumatriptan 50 mg as needed at onset of migraine (max 2 doses per 24 hours)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Von Hippel-Lindau disease in father (diagnosed age 35)
- Pheochromocytoma in paternal aunt
- Chronic hypertension in mother

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per month). Occupation: Project manager at a consulting firm.

VITAL SIGNS:
Her blood pressure is 138/86 mmHg.

PHYSICAL EXAMINATION:
Her physical examination is unremarkable.

LABORATORY DATA:
Her laboratory tests are as follows:

- Hemoglobin: 12.2 g/dL
- Platelets: 250,000/mm³
- Serum creatinine: 0.8 mg/dL
- Urinalysis: No proteinuria or glucosuria

ASSESSMENT AND PLAN:
Given her history and current pregnancy status,

## Encounter 5: source 2022-03-15, practice 2024-04-24, ed, Emergency Medicine

Chief complaint: Regular contractions every 5 minutes for the past 2 hours at 28 weeks gestation

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. Michael Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Regular contractions every 5 minutes for the past 2 hours at 28 weeks gestation

HISTORY OF PRESENT ILLNESS:
The patient is a 34-year-old female with Von Hippel-Lindau disease and history of benign ovarian neoplasm who presents to the Emergency Department with regular contractions every 5 minutes for the past 2 hours. She has been undergoing high-risk prenatal monitoring following preconception counseling regarding antihypertensive therapy. She reports that her previous pregnancy was uncomplicated and resulted in a full-term vaginal delivery. Her current pregnancy has been unremarkable until now. She denies any vaginal bleeding or leakage of fluid. The patient is currently at 28 weeks of gestation.

PAST MEDICAL HISTORY:
Active Problem List:
- Von Hippel-Lindau disease (diagnosed 2020-01-15)
- Benign neoplasm of ovary (diagnosed 2020-04-15)
- Copper-containing intrauterine device (diagnosed 2021-01-15)
- Essential hypertension (diagnosed 2021-07-15)
- High risk pregnancy for pre-eclampsia (diagnosed 2022-01-15)
- Asthma
- Migraine with aura

Her medical history is significant for well-controlled asthma.

PAST SURGICAL HISTORY:
- Laparoscopic resection of androgen-secreting ovarian neoplasm (right ovary)
- Vaginal delivery at 34 weeks gestation (pregnancy complicated by preeclampsia)
- Vaginal delivery at 39 weeks gestation (uncomplicated)

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg daily
- Albuterol inhaler 90 mcg (1-2 puffs) as needed for asthma symptoms
- Sumatriptan 50 mg as needed at onset of migraine (max 2 doses per 24 hours)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Von Hippel-Lindau disease in father (diagnosed age 35)
- Pheochromocytoma in paternal aunt
- Chronic hypertension in mother

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per month). Occupation: Project manager at a consulting firm.

VITAL SIGNS:
On examination, her vital signs are stable

PHYSICAL EXAMINATION:
Fetal heart tones are regular at 150 beats per minute. A speculum examination reveals a closed cervical os without evidence of ruptured membranes.

LABORATORY DATA:
Fetal fibronectin test returns positive.

IMAGING:
A transvaginal ultrasound estimates the fetal weight at 2100 grams, and the amniotic fluid index is normal.

## Encounter 6: source 2022-05-15, practice 2024-06-20, ed, Emergency Medicine

Chief complaint: Sudden-onset severe abdominal pain and vaginal bleeding at 34 weeks gestation

ENCOUNTER NOTE
Date: 2022-05-15
Provider: Dr. Sarah Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden-onset severe abdominal pain and vaginal bleeding at 34 weeks gestation

HISTORY OF PRESENT ILLNESS:
The 34-year-old female with Von Hippel-Lindau disease, essential hypertension, and a history of precipitous labor at 28 weeks gestation presents to the emergency department at 34 weeks pregnant with a 1-day history of sudden-onset severe abdominal pain and vaginal bleeding. She denies any trauma. Her abdominal pain does not subside with analgesics.

PAST MEDICAL HISTORY:
Active Problem List:
- Von Hippel-Lindau disease (diagnosed 2020-01-15)
- Benign neoplasm of ovary (diagnosed 2020-04-15)
- Copper-containing intrauterine device (diagnosed 2021-01-15)
- Essential hypertension (diagnosed 2021-07-15)
- High risk pregnancy for pre-eclampsia (diagnosed 2022-01-15)
- Precipitous labor (diagnosed 2022-03-15)
- Asthma
- Migraine with aura

She has a known history of chronic hypertension, which has been poorly controlled during pregnancy.

PAST SURGICAL HISTORY:
The patient's obstetric history includes a previous cesarean delivery.

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg daily
- Albuterol inhaler 90 mcg (1-2 puffs) as needed for asthma symptoms
- Sumatriptan 50 mg as needed at onset of migraine (max 2 doses per 24 hours)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Von Hippel-Lindau disease in father (diagnosed age 35)
- Pheochromocytoma in paternal aunt
- Chronic hypertension in mother

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per month). Occupation: Project manager at a consulting firm.

VITAL SIGNS:
On examination, her blood pressure is 160/100 mmHg, heart rate is 88 beats per minute, and temperature is 37.2°C (99°F).

PHYSICAL EXAMINATION:
Abdominal examination reveals a tender uterus with a firm consistency. Fetal heart tones are 140 beats per minute.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 10.2 g/dL (normal: 12-16 g/dL)
- Platelet count: 220,000/mm³ (normal: 150,000-400,000/mm³)
- Serum creatinine: 0.9 mg/dL (normal: 0.6-1.1 mg/dL)
- Aspartate aminotransferase (AST): 70 U/L (normal: 10-40 U/L)
- Alanine aminotransferase (ALT): 50 U/L (normal: 7-56 U/L)
- Urinalysis: 1+ proteinuria

IMAGING:
Fetal ultrasound shows normal anatomy and growth.

## Encounter 7: source 2024-01-15, practice 2026-02-16, outpatient, Obstetrics & Gynecology

Chief complaint: Routine prenatal visit noting abdomen seems larger than at the same stage during previous pregnancy

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Lisa Wang, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine prenatal visit noting abdomen seems larger than at the same stage during previous pregnancy

HISTORY OF PRESENT ILLNESS:
The patient returns for routine prenatal care with Von Hippel-Lindau disease and prior pregnancy with preeclampsia and precipitous labor. She reports feeling well but notes her abdomen seems larger than at the same stage during previous pregnancy. Her current pregnancy has been uncomplicated thus far.

PAST MEDICAL HISTORY:
Active Problem List:
- Von Hippel-Lindau disease (diagnosed 2020-01-15)
- Benign neoplasm of ovary (diagnosed 2020-04-15)
- Copper-containing intrauterine device (diagnosed 2021-01-15)
- Essential hypertension (diagnosed 2021-07-15)
- High risk pregnancy for pre-eclampsia (diagnosed 2022-01-15)
- Precipitous labor (diagnosed 2022-03-15)
- Chronic hypertension with superimposed preeclampsia (diagnosed 2022-05-15)
- Asthma
- Migraine with aura

Her past medical history is significant for gestational diabetes mellitus (GDM) during her first pregnancy, which was diet-controlled.

PAST SURGICAL HISTORY:
- Laparoscopic resection of androgen-secreting ovarian neoplasm (right ovary)
- Vaginal delivery at 34 weeks gestation (pregnancy complicated by preeclampsia)
- Vaginal delivery at 39 weeks gestation (uncomplicated)

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg daily
- Albuterol inhaler 90 mcg (1-2 puffs) as needed for asthma symptoms
- Sumatriptan 50 mg as needed at onset of migraine (max 2 doses per 24 hours)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Von Hippel-Lindau disease in father (diagnosed age 35)
- Pheochromocytoma in paternal aunt
- Chronic hypertension in mother

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per month). Occupation: Project manager at a consulting firm.

VITAL SIGNS:
Her blood pressure is 120/78 mmHg, heart rate is 82 beats per minute

PHYSICAL EXAMINATION:
Her fundal height measures 33 cm

LABORATORY DATA:
Her fasting glucose level this morning was 98 mg/dL, and her HbA1c measured last week was 6.2%.

Laboratory Values:
- Fasting glucose: 98 mg/dL
- HbA1c: 6.2%

IMAGING:
A fetal ultrasound shows a single fetus with an estimated weight above the 90th percentile for gestational age and an amniotic fluid index of 27 cm (normal range: 5-25 cm).
