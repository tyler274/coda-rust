# synthetic_hospital patient 1689

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 30 at first encounter, sex F, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-02-09 (Wed) | ed | Emergency Medicine |
| 1 | 2021-01-15 | 2023-02-20 (Mon) | ed | Obstetrics & Gynecology |
| 2 | 2022-01-15 | 2024-02-21 (Wed) | ed | Emergency Medicine |
| 3 | 2023-01-15 | 2025-02-04 (Tue) | ed | Emergency Medicine |
| 4 | 2023-07-15 | 2025-08-04 (Mon) | inpatient | Obstetrics & Gynecology |
| 5 | 2023-11-15 | 2025-12-13 (Sat) | ed | Emergency Medicine |
| 6 | 2024-01-15 | 2026-02-17 (Tue) | inpatient | Obstetrics & Gynecology |
| 7 | 2024-01-15 | 2026-02-17 (Tue) | inpatient | Obstetrics & Gynecology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Penicillin - urticaria (hives)"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Human Resources Specialist",
 "alcohol_use": "Social (1-2 drinks per month)",
 "family_history": [
  "Mother: Essential hypertension",
  "Father: Type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "25 mg by mouth daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "1 tablet by mouth daily (continuous use for 10 years)",
   "name": "Oral contraceptive pills (combined ethinyl estradiol/levonorgestrel)"
  }
 ],
 "surgical_history": [
  "Cesarean section x2 (last procedure: classical uterine incision)",
  "Partial hepatectomy for ruptured hepatic mass",
  "Laparoscopic evacuation of hemoperitoneum with right ovarian cystectomy",
  "Blood transfusion (packed red blood cells) following motor vehicle collision"
 ],
 "chronic_conditions": [
  "Essential hypertension",
  "Rh(D)-negative blood type",
  "Irregular menstrual cycles"
 ],
 "age_at_first_encounter": 30
}
```

Primary diagnoses: ["Uterine rupture", "Group B Streptococcus Colonization in Pregnancy with Non-Severe Penicillin Allergy", "Ruptured hepatocellular adenoma", "Ruptured corpus luteum cyst with hemoperitoneum", "Hypovolemic shock", "Threatened preterm labor", "Susceptibility to Rh(D) alloimmunization", "Preterm premature rupture of membranes"]
Comorbidities: ["Hypovolemic shock", "Personal History of Cholelithiasis", "Acute posthemorrhagic anemia"]

## Encounter 0: source 2020-01-15, practice 2022-02-09, ed, Emergency Medicine

Chief complaint: Sudden-onset, severe right upper quadrant pain that started 2 hours ago with light-headedness and nausea

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden-onset, severe right upper quadrant pain that started 2 hours ago with light-headedness and nausea

HISTORY OF PRESENT ILLNESS:
That started 2 hours ago. She describes the pain as sharp and constant. She also reports feeling light-headed and has had several episodes of nausea without vomiting.

PAST MEDICAL HISTORY:
Active Problem List:
- Essential hypertension
- Rh(D)-negative blood type
- Irregular menstrual cycles

PAST SURGICAL HISTORY:
- Cesarean section x2 (last procedure: classical uterine incision)
- Partial hepatectomy for ruptured hepatic mass
- Laparoscopic evacuation of hemoperitoneum with right ovarian cystectomy
- Blood transfusion (packed red blood cells) following motor vehicle collision

MEDICATIONS:
She takes hydrochlorothiazide, and she has been on oral contraceptive pills for the past 10 years.

Home Medications:
- Oral contraceptive pills (combined ethinyl estradiol/levonorgestrel) 1 tablet by mouth daily (continuous use for 10 years)

ALLERGIES:
Penicillin - urticaria (hives)

FAMILY HISTORY:
- Mother: Essential hypertension
- Father: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Human Resources Specialist.

VITAL SIGNS:
Her blood pressure is 90/60 mmHg, heart rate is 120 beats per minute

PHYSICAL EXAMINATION:
She appears pale and diaphoretic. Abdominal examination reveals tenderness to palpation in the right upper quadrant with guarding

LABORATORY DATA:
Hemoglobin: 9.8 g/dL (baseline 13.2 g/dL)
- Hematocrit: 29.4% (baseline 40%)
- Platelets: 250,000 /μL
- INR: 1.0
- AST: 70 U/L
- ALT: 65 U/L
- Alkaline phosphatase: 90 U/L

IMAGING:
An urgent abdominal ultrasound shows a 5 cm hyperechoic lesion in the right lobe of the liver with surrounding free fluid

## Encounter 1: source 2021-01-15, practice 2023-02-20, ed, Obstetrics & Gynecology

Chief complaint: Sudden onset of severe lower abdominal pain after vigorous exercise with nausea

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Michael Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of severe lower abdominal pain after vigorous exercise with nausea

HISTORY OF PRESENT ILLNESS:
The patient is a 31-year-old female with essential hypertension, Rh(D)-negative blood type, and history of ruptured hepatocellular adenoma (January 2020) who returns to the emergency department with sudden onset of severe lower abdominal pain that started 4 hours ago after a session of vigorous exercise. She describes the pain as sharp and constant, with some intermittent cramping. She also reports nausea but no vomiting. Her last menstrual period was 2 weeks ago and was normal.

PAST MEDICAL HISTORY:
Active Problem List:
- Ruptured hepatocellular adenoma (diagnosed 2020-01-15)
- Essential hypertension
- Rh(D)-negative blood type
- Irregular menstrual cycles

PAST SURGICAL HISTORY:
- Cesarean section x2 (last procedure: classical uterine incision)
- Partial hepatectomy for ruptured hepatic mass
- Laparoscopic evacuation of hemoperitoneum with right ovarian cystectomy
- Blood transfusion (packed red blood cells) following motor vehicle collision

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg by mouth daily
- Oral contraceptive pills (combined ethinyl estradiol/levonorgestrel) 1 tablet by mouth daily (continuous use for 10 years)

ALLERGIES:
Penicillin - urticaria (hives)

FAMILY HISTORY:
- Mother: Essential hypertension
- Father: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Human Resources Specialist.

VITAL SIGNS:
Blood pressure 90/60 mmHg, heart rate 120 beats per minute, respiratory rate 22 breaths per minute, and temperature 37.2°C (99°F)

PHYSICAL EXAMINATION:
Abdominal examination reveals diffuse tenderness, marked rigidity, and rebound tenderness. A pelvic examination is deferred due to severe abdominal pain.

LABORATORY DATA:
Laboratory tests show a hemoglobin level of 11.2 g/dL (baseline 13.5 g/dL), white blood cell count of 12,000/uL, and a negative pregnancy test.

IMAGING:
An ultrasound of the pelvis shows free fluid in the cul-de-sac.

ASSESSMENT AND PLAN:
The patient becomes progressively tachycardic and her blood pressure drops to 85/55 mmHg despite fluid resuscitation.

## Encounter 2: source 2022-01-15, practice 2024-02-21, ed, Emergency Medicine

Chief complaint: Dizziness and fatigue following motor vehicle collision with mild abdominal pain

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Emily Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Dizziness and fatigue following motor vehicle collision with mild abdominal pain

HISTORY OF PRESENT ILLNESS:
She presents after a motor vehicle collision with history of ruptured hepatocellular adenoma (2020) and corpus luteum cyst (2021). She reports that she was the restrained driver in a head-on collision at moderate speed. Airbags were deployed, and she did not lose consciousness. She complains of mild abdominal pain but no other specific injuries.

PAST MEDICAL HISTORY:
Active Problem List:
- Ruptured hepatocellular adenoma (diagnosed 2020-01-15)
- Ruptured corpus luteum cyst with hemoperitoneum (diagnosed 2021-01-15)
- Essential hypertension
- Rh(D)-negative blood type
- Irregular menstrual cycles

PAST SURGICAL HISTORY:
- Cesarean section x2 (last procedure: classical uterine incision)
- Partial hepatectomy for ruptured hepatic mass
- Laparoscopic evacuation of hemoperitoneum with right ovarian cystectomy
- Blood transfusion (packed red blood cells) following motor vehicle collision

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg by mouth daily
- Oral contraceptive pills (combined ethinyl estradiol/levonorgestrel) 1 tablet by mouth daily (continuous use for 10 years)

ALLERGIES:
Penicillin - urticaria (hives)

FAMILY HISTORY:
- Mother: Essential hypertension
- Father: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Human Resources Specialist.

VITAL SIGNS:
Vital signs reveal a blood pressure of 90/60 mmHg, heart rate of 120 beats per minute, respiratory rate of 22 breaths per minute, and oxygen saturation of 97% on room air.

PHYSICAL EXAMINATION:
On examination, she is alert but appears pale and anxious. Her skin is cool and clammy. The abdominal examination shows mild tenderness in the left upper quadrant without rebound or guarding. Bowel sounds are normal. Pelvic and rectal examinations are unremarkable. There is no external evidence of trauma.

LABORATORY DATA:
Laboratory tests show:

- Hemoglobin: 9.8 g/dL (normal: 12-16 g/dL)
- Hematocrit: 29% (normal: 36-46%)
- Platelets: 250,000/mm³ (normal: 150,000-400,000/mm³)
- Prothrombin time (PT): 12 seconds (normal: 11-13.5 seconds)
- Partial thromboplastin time (PTT): 30 seconds (normal: 25-35 seconds)
- Lactate: 4.2 mmol/L (normal: 0.5-2.2 mmol/L)

IMAGING:
Focused Assessment with Sonography for Trauma (FAST) exam is negative for free fluid in the abdomen. The chest X-ray is normal.

## Encounter 3: source 2023-01-15, practice 2025-02-04, ed, Emergency Medicine

Chief complaint: Acute onset of lower abdominal pain and vaginal spotting with 6 weeks gestation

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset of lower abdominal pain and vaginal spotting with 6 weeks gestation

HISTORY OF PRESENT ILLNESS:
This 33-year-old Rh(D)-negative female with essential hypertension and prior emergency presentations for ruptured hepatocellular adenoma, ruptured corpus luteum cyst with hemoperitoneum, and hypovolemic shock presents with acute abdominal complaints. She reports that the pain is sharp, located on the left side, and began suddenly 6 hours ago. She also mentions that she has been feeling light-headed and has had some vaginal spotting over the past day. Her last menstrual period was 6 weeks ago, and she has a history of irregular menstrual cycles. She is sexually active with one partner and does not use contraception.

PAST MEDICAL HISTORY:
Active Problem List:
- Ruptured hepatocellular adenoma (diagnosed 2020-01-15)
- Ruptured corpus luteum cyst with hemoperitoneum (diagnosed 2021-01-15)
- Hypovolemic shock (diagnosed 2022-01-15)
- Essential hypertension
- Rh(D)-negative blood type
- Irregular menstrual cycles

Her past medical history is significant for a blood transfusion following a motor vehicle accident 5 years ago, after which she was told she has an Rh(D)-negative blood type.

PAST SURGICAL HISTORY:
- Cesarean section x2 (last procedure: classical uterine incision)
- Partial hepatectomy for ruptured hepatic mass
- Laparoscopic evacuation of hemoperitoneum with right ovarian cystectomy
- Blood transfusion (packed red blood cells) following motor vehicle collision

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg by mouth daily
- Oral contraceptive pills (combined ethinyl estradiol/levonorgestrel) 1 tablet by mouth daily (continuous use for 10 years)

ALLERGIES:
Penicillin - urticaria (hives)

FAMILY HISTORY:
- Mother: Essential hypertension
- Father: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Human Resources Specialist.

VITAL SIGNS:
Blood pressure is 100/60 mmHg, pulse is 110 beats per minute

PHYSICAL EXAMINATION:
On examination, she appears pale. A pelvic examination reveals left adnexal tenderness.

LABORATORY DATA:
A urine pregnancy test is positive. Hemoglobin is 9.8 g/dL, hematocrit is 29.4%, and blood type is confirmed to be O negative.

IMAGING:
Transvaginal ultrasound shows an empty uterine cavity and a left adnexal mass with surrounding free fluid.

## Encounter 4: source 2023-07-15, practice 2025-08-04, inpatient, Obstetrics & Gynecology

Chief complaint: Regular contractions every 5 minutes for 2 hours at 30 weeks gestation without rupture of membranes

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. James Thompson, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
Regular contractions every 5 minutes for 2 hours at 30 weeks gestation without rupture of membranes

HISTORY OF PRESENT ILLNESS:
This 33-year-old Rh(D)-negative female with ruptured hepatic adenoma, ruptured luteum cyst, and first-trimester bleeding presents with regular contractions. She reports no rupture of membranes or vaginal bleeding. Her previous pregnancy was uncomplicated, and she delivered a healthy full-term infant. Her current pregnancy has been uneventful until now. She is currently at 30 weeks gestation.

PAST MEDICAL HISTORY:
Active Problem List:
- Ruptured hepatocellular adenoma (diagnosed 2020-01-15)
- Ruptured corpus luteum cyst with hemoperitoneum (diagnosed 2021-01-15)
- Hypovolemic shock (diagnosed 2022-01-15)
- Susceptibility to Rh(D) alloimmunization (diagnosed 2023-01-15)
- Essential hypertension
- Rh(D)-negative blood type
- Irregular menstrual cycles

PAST SURGICAL HISTORY:
- Cesarean section x2 (last procedure: classical uterine incision)
- Partial hepatectomy for ruptured hepatic mass
- Laparoscopic evacuation of hemoperitoneum with right ovarian cystectomy
- Blood transfusion (packed red blood cells) following motor vehicle collision

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg by mouth daily
- Oral contraceptive pills (combined ethinyl estradiol/levonorgestrel) 1 tablet by mouth daily (continuous use for 10 years)

ALLERGIES:
Penicillin - urticaria (hives)

FAMILY HISTORY:
- Mother: Essential hypertension
- Father: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Human Resources Specialist.

VITAL SIGNS:
Blood pressure is 120/78 mmHg, heart rate is 88 beats per minute, and temperature is 36.8°C (98.2°F)

PHYSICAL EXAMINATION:
Fetal heart tones are 140 beats per minute. A speculum examination reveals a closed cervical os.

LABORATORY DATA:
Fetal fibronectin test returns negative.

IMAGING:
Transabdominal ultrasound shows a normal amount of amniotic fluid with no fetal anomalies.

PLAN:
Given the patient's gestational age and symptoms, the decision is made to initiate tocolysis.

## Encounter 5: source 2023-11-15, practice 2025-12-13, ed, Emergency Medicine

Chief complaint: Sudden gush of clear fluid from the vagina while doing household chores at 34 weeks gestation

ENCOUNTER NOTE
Date: 2023-11-15
Provider: Dr. Lisa Martinez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden gush of clear fluid from the vagina while doing household chores at 34 weeks gestation

HISTORY OF PRESENT ILLNESS:
The patient is a 33-year-old Rh(D)-negative female with a history of threatened preterm labor who returns to the emergency department with a complaint of sudden gush of clear fluid from the vagina while doing household chores. She denies any contractions, vaginal bleeding, or abdominal pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Ruptured hepatocellular adenoma (diagnosed 2020-01-15)
- Ruptured corpus luteum cyst with hemoperitoneum (diagnosed 2021-01-15)
- Hypovolemic shock (diagnosed 2022-01-15)
- Susceptibility to Rh(D) alloimmunization (diagnosed 2023-01-15)
- Threatened preterm labor (diagnosed 2023-07-15)
- Essential hypertension
- Rh(D)-negative blood type
- Irregular menstrual cycles

Her previous pregnancy was uneventful, and she had a vaginal delivery at term.

PAST SURGICAL HISTORY:
- Cesarean section x2 (last procedure: classical uterine incision)
- Partial hepatectomy for ruptured hepatic mass
- Laparoscopic evacuation of hemoperitoneum with right ovarian cystectomy
- Blood transfusion (packed red blood cells) following motor vehicle collision

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg by mouth daily
- Oral contraceptive pills (combined ethinyl estradiol/levonorgestrel) 1 tablet by mouth daily (continuous use for 10 years)

ALLERGIES:
Penicillin - urticaria (hives)

FAMILY HISTORY:
- Mother: Essential hypertension
- Father: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Human Resources Specialist.

VITAL SIGNS:
On examination, her vital signs are stable, and the fetal heart rate is 140 beats per minute.

PHYSICAL EXAMINATION:
Speculum examination confirms pooling of clear fluid in the vaginal vault. Nitrazine paper test turns blue, and ferning is observed under the microscope.

PLAN:
The patient is admitted for inpatient management.

## Encounter 6: source 2024-01-15, practice 2026-02-17, inpatient, Obstetrics & Gynecology

Chief complaint: Regular contractions every 5 minutes with rupture of membranes 30 minutes ago at 36 weeks gestation

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Robert Kim, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
Regular contractions every 5 minutes with rupture of membranes 30 minutes ago at 36 weeks gestation

HISTORY OF PRESENT ILLNESS:
This 34-year-old has a history of threatened preterm labor at 30 weeks and preterm premature rupture of membranes at 34 weeks during this pregnancy. Her prenatal course was unremarkable except for a positive group B Streptococcus (GBS) screen at 36 weeks.

PAST MEDICAL HISTORY:
Active Problem List:
- Ruptured hepatocellular adenoma (diagnosed 2020-01-15)
- Ruptured corpus luteum cyst with hemoperitoneum (diagnosed 2021-01-15)
- Hypovolemic shock (diagnosed 2022-01-15)
- Susceptibility to Rh(D) alloimmunization (diagnosed 2023-01-15)
- Threatened preterm labor (diagnosed 2023-07-15)
- Preterm premature rupture of membranes (diagnosed 2023-11-15)
- Essential hypertension
- Rh(D)-negative blood type
- Irregular menstrual cycles

Her past medical history is significant for a rash following penicillin administration as a child, which she describes as "hives all over the body.".

PAST SURGICAL HISTORY:
- Cesarean section x2 (last procedure: classical uterine incision)
- Partial hepatectomy for ruptured hepatic mass
- Laparoscopic evacuation of hemoperitoneum with right ovarian cystectomy
- Blood transfusion (packed red blood cells) following motor vehicle collision

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg by mouth daily
- Oral contraceptive pills (combined ethinyl estradiol/levonorgestrel) 1 tablet by mouth daily (continuous use for 10 years)

ALLERGIES:
Penicillin - urticaria (hives)

FAMILY HISTORY:
- Mother: Essential hypertension
- Father: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Human Resources Specialist.

REVIEW OF SYSTEMS:
She denies any respiratory distress, facial swelling, or previous anaphylaxis.

VITAL SIGNS:
On examination, her temperature is 37.2°C (99°F), blood pressure is 120/78 mmHg, pulse is 88/min, and respiratory rate is 16/min.

PHYSICAL EXAMINATION:
The fetal heart rate tracing is reassuring. The patient's cervix is dilated to 4 cm.

PLAN:
The obstetric team is preparing for intrapartum management.

## Encounter 7: source 2024-01-15, practice 2026-02-17, inpatient, Obstetrics & Gynecology

Chief complaint: Sudden-onset severe abdominal pain and vaginal bleeding during labor

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. David Nguyen, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
Sudden-onset severe abdominal pain and vaginal bleeding during labor

HISTORY OF PRESENT ILLNESS:
The patient is a 34-year-old Rh(D)-negative female with a history of ruptured hepatocellular adenoma, ruptured corpus luteum cyst, and hypovolemic shock who was admitted earlier today in active labor and now presents with sudden-onset severe abdominal pain and vaginal bleeding during labor. She reports that the pain is sharp and constant.

PAST MEDICAL HISTORY:
Active Problem List:
- Ruptured hepatocellular adenoma (diagnosed 2020-01-15)
- Ruptured corpus luteum cyst with hemoperitoneum (diagnosed 2021-01-15)
- Hypovolemic shock (diagnosed 2022-01-15)
- Susceptibility to Rh(D) alloimmunization (diagnosed 2023-01-15)
- Threatened preterm labor (diagnosed 2023-07-15)
- Preterm premature rupture of membranes (diagnosed 2023-11-15)
- Group B Streptococcus Colonization in Pregnancy with Non-Severe Penicillin Allergy (diagnosed 2024-01-15)
- Essential hypertension
- Rh(D)-negative blood type
- Irregular menstrual cycles

Her past obstetric history is significant for two prior cesarean deliveries, the last of which was a classical cesarean section due to a transverse lie.

PAST SURGICAL HISTORY:
- Cesarean section x2 (last procedure: classical uterine incision)
- Partial hepatectomy for ruptured hepatic mass
- Laparoscopic evacuation of hemoperitoneum with right ovarian cystectomy
- Blood transfusion (packed red blood cells) following motor vehicle collision

MEDICATIONS:
Home Medications:
- Hydrochlorothiazide 25 mg by mouth daily
- Oral contraceptive pills (combined ethinyl estradiol/levonorgestrel) 1 tablet by mouth daily (continuous use for 10 years)

ALLERGIES:
Penicillin - urticaria (hives)

FAMILY HISTORY:
- Mother: Essential hypertension
- Father: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Human Resources Specialist.

VITAL SIGNS:
On examination, her vital signs are: blood pressure 100/60 mmHg, heart rate 110 bpm, respiratory rate 22 breaths/min, and temperature 37.0°C (98.6°F).

PHYSICAL EXAMINATION:
The abdomen is tender to palpation with a palpable irregular mass noted on the right side.

IMAGING:
There is no evidence of placental abruption on ultrasound, and the placenta is noted to be in the posterior position.

OTHER STUDIES:
Fetal heart tones are irregular, with variable decelerations noted on the cardiotocography.
