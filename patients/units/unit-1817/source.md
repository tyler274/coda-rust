# synthetic_hospital patient 1817

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 24 at first encounter, sex F, Ashkenazi Jewish, Private PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-05-03 (Tue) | outpatient | Obstetrics & Gynecology |
| 1 | 2021-09-15 | 2023-12-15 (Fri) | outpatient | Obstetrics & Gynecology |
| 2 | 2023-07-15 | 2025-10-22 (Wed) | outpatient | Obstetrics & Gynecology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Latex",
  "Sulfa drugs"
 ],
 "insurance": "Private PPO",
 "occupation": "Ballet instructor",
 "alcohol_use": "None",
 "family_history": [
  "Mother: Breast cancer, age 51",
  "Maternal grandmother: Ovarian cancer, age 47",
  "Sister: BRCA1 gene mutation carrier"
 ],
 "race_ethnicity": "Ashkenazi Jewish",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1 mg/0.5 mg daily",
   "name": "Estradiol/Norethindrone acetate"
  },
  {
   "dose": "50 mg daily",
   "name": "Sertraline"
  },
  {
   "dose": "2000 units daily",
   "name": "Cholecalciferol"
  }
 ],
 "surgical_history": [
  "Vaginal delivery, preterm (prior pregnancy)"
 ],
 "chronic_conditions": [
  "Anorexia nervosa, in remission",
  "Primary ovarian insufficiency",
  "Major depressive disorder"
 ],
 "age_at_first_encounter": 24
}
```

Primary diagnoses: ["Hypergonadotropic hypogonadism", "Intrauterine Growth Restriction", "Serous cystadenocarcinoma of ovary"]
Comorbidities: ["Secondary Amenorrhea", "History of preterm delivery"]

## Encounter 0: source 2020-01-15, practice 2022-05-03, outpatient, Obstetrics & Gynecology

Chief complaint: 5-month history of amenorrhea in a patient with previously regular cycles

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Elena Vasquez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
5-month history of amenorrhea in a patient with previously regular cycles

HISTORY OF PRESENT ILLNESS:
She reports that her menstrual cycles were previously regular. She denies any significant weight change, galactorrhea, or hirsutism. She is not sexually active and has never been pregnant.

PAST MEDICAL HISTORY:
Active Problem List:
- Anorexia nervosa, in remission
- Primary ovarian insufficiency
- Major depressive disorder

PAST SURGICAL HISTORY:
- Vaginal delivery, preterm (prior pregnancy)

MEDICATIONS:
She is.

Home Medications:
- Estradiol/Norethindrone acetate 1 mg/0.5 mg daily
- Sertraline 50 mg daily
- Cholecalciferol 2000 units daily

ALLERGIES:
Latex, Sulfa drugs

FAMILY HISTORY:
- Mother: Breast cancer, age 51
- Maternal grandmother: Ovarian cancer, age 47
- Sister: BRCA1 gene mutation carrier

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Ballet instructor.

VITAL SIGNS:
On physical examination, her BMI is 22 kg/m^2, blood pressure is 120/78 mmHg, and heart rate is 72 bpm

PHYSICAL EXAMINATION:
There is no thyroid enlargement or nodules, and the remainder of the examination is normal.

LABORATORY DATA:
Laboratory tests are ordered, and the results are as follows:

- Pregnancy test: Negative
- Serum prolactin: 18 ng/mL (normal range: 4-23 ng/mL)
- TSH: 2.5 mIU/L (normal range: 0.4-4.0 mIU/L)
- FSH: 85 mIU/mL (normal range for follicular phase: 3.5-12.5 mIU/mL)

## Encounter 1: source 2021-09-15, practice 2023-12-15, outpatient, Obstetrics & Gynecology

Chief complaint: Routine prenatal visit at 28 weeks gestation with concerns about poor maternal weight gain

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. Sarah Chen, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine prenatal visit at 28 weeks gestation with concerns about poor maternal weight gain

HISTORY OF PRESENT ILLNESS:
The patient with prior hypergonadotropic hypogonadism returns for prenatal care at 28 weeks gestation. She has been feeling well throughout the pregnancy, but she is concerned that she has not gained much weight. She has gained 4 kg since the beginning of her pregnancy.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypergonadotropic hypogonadism (diagnosed 2020-01-15)
- Anorexia nervosa, in remission
- Primary ovarian insufficiency
- Major depressive disorder

She has a history of anorexia nervosa, which she reports has been under better control for the past year. Her prepregnancy BMI was 17. Her obstetric history is significant for one previous preterm delivery at 34 weeks.

PAST SURGICAL HISTORY:
- Vaginal delivery, preterm (prior pregnancy)

MEDICATIONS:
Home Medications:
- Estradiol/Norethindrone acetate 1 mg/0.5 mg daily
- Sertraline 50 mg daily
- Cholecalciferol 2000 units daily

ALLERGIES:
Latex, Sulfa drugs

FAMILY HISTORY:
- Mother: Breast cancer, age 51
- Maternal grandmother: Ovarian cancer, age 47
- Sister: BRCA1 gene mutation carrier

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Ballet instructor.

VITAL SIGNS:
On examination, her weight is 50 kg, height is 165 cm, blood pressure is 110/70 mmHg

PHYSICAL EXAMINATION:
Fundal height is measuring 24 cm, which is less than expected for her gestational age. Fetal heart tones are normal.

LABORATORY DATA:
Laboratory results are as follows:

- Hemoglobin: 11.2 g/dL
- Hematocrit: 33%
- White blood cell count: 9,000/mm³
- Platelets: 250,000/mm³
- Urinalysis: Normal

## Encounter 2: source 2023-07-15, practice 2025-10-22, outpatient, Obstetrics & Gynecology

Chief complaint: Gradual abdominal bloating, early satiety, and fatigue worsening over the past month

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Raj Patel, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Gradual abdominal bloating, early satiety, and fatigue worsening over the past month

HISTORY OF PRESENT ILLNESS:
The patient is a 27-year-old female with primary ovarian insufficiency diagnosed in 2020, anorexia nervosa in remission, and a prior pregnancy complicated by intrauterine growth restriction who returns with new abdominal symptoms. She reports that she has noticed worsening over the past month. She mentions that her clothes have started to feel tight around the waist, and she has recently lost interest in eating large meals because she gets full quickly. She has also felt more fatigued than usual but attributes it to work-related stress. Her menstrual history is irregular, with occasional missed periods, but she has not menstruated in nearly two months.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypergonadotropic hypogonadism (diagnosed 2020-01-15)
- Intrauterine Growth Restriction (diagnosed 2021-09-15)
- Anorexia nervosa, in remission
- Primary ovarian insufficiency
- Major depressive disorder

Her past medical and surgical histories are unremarkable.

PAST SURGICAL HISTORY:
Her past medical and surgical histories are unremarkable.

MEDICATIONS:
Home Medications:
- Estradiol/Norethindrone acetate 1 mg/0.5 mg daily
- Sertraline 50 mg daily
- Cholecalciferol 2000 units daily

ALLERGIES:
Latex, Sulfa drugs

FAMILY HISTORY:
- Mother: Breast cancer, age 51
- Maternal grandmother: Ovarian cancer, age 47
- Sister: BRCA1 gene mutation carrier

SOCIAL HISTORY:
She is sexually active but does not use contraception.

REVIEW OF SYSTEMS:
There is no history of weight loss, fever, or gastrointestinal bleeding.

PHYSICAL EXAMINATION:
On examination, her abdomen is distended but soft, with a palpable mass in the lower abdomen that is firm and mobile. Pelvic examination reveals a right adnexal mass.

LABORATORY DATA:
Serum studies show elevated alpha-fetoprotein and beta-hCG.

IMAGING:
Transvaginal ultrasound confirms the presence of a large, heterogeneous mass arising from the ovary with areas of necrosis and hemorrhage.
