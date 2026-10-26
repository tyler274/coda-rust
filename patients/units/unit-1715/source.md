# synthetic_hospital patient 1715

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex F, Non-Hispanic White, Private PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-02-25 (Thu) | outpatient | Hematology |
| 1 | 2020-07-15 | 2021-08-16 (Mon) | outpatient | Obstetrics & Gynecology |
| 2 | 2021-07-15 | 2022-08-22 (Mon) | outpatient | Hematology |
| 3 | 2022-01-15 | 2023-02-27 (Mon) | outpatient | Obstetrics & Gynecology |
| 4 | 2022-07-15 | 2023-08-22 (Tue) | outpatient | Rheumatology |
| 5 | 2023-07-15 | 2024-08-26 (Mon) | outpatient | Maternal-Fetal Medicine |
| 6 | 2023-09-15 | 2024-10-23 (Wed) | ed | Emergency Medicine |
| 7 | 2024-09-15 | 2025-10-20 (Mon) | outpatient | Obstetrics & Gynecology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Private PPO",
 "occupation": "Schoolteacher",
 "alcohol_use": "None",
 "family_history": [
  "Maternal grandmother with systemic lupus erythematosus",
  "Paternal uncle with history of pulmonary embolism (Factor V Leiden carrier)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "200 mg PO BID",
   "name": "Hydroxychloroquine"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Prednisone"
  },
  {
   "dose": "5 mg PO daily (therapeutic anticoagulation)",
   "name": "Warfarin"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  },
  {
   "dose": "1 tablet PO daily",
   "name": "Prenatal vitamin"
  }
 ],
 "surgical_history": [
  "Dilation and curettage (recurrent pregnancy losses)",
  "Hysteroscopic myomectomy"
 ],
 "chronic_conditions": [
  "Systemic lupus erythematosus",
  "Secondary antiphospholipid syndrome",
  "Factor V Leiden thrombophilia",
  "History of deep vein thrombosis",
  "Migraine with aura",
  "Submucosal uterine leiomyoma",
  "Cervical insufficiency"
 ],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Factor V Leiden thrombophilia", "Antiphospholipid syndrome", "Encounter for initial prescription of implantable subdermal contraceptive", "Submucosal uterine leiomyoma", "Cervical insufficiency", "Partial hydatidiform mole"]
Comorbidities: ["History of deep vein thrombosis", "Personal History of Cholelithiasis", "Recurrent pregnancy loss", "Systemic lupus erythematosus"]

## Encounter 0: source 2020-01-15, practice 2021-02-25, outpatient, Hematology

Chief complaint: Chronic fatigue and vague left calf discomfort with prolonged standing

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Hematology
Visit Type: outpatient

CHIEF COMPLAINT:
Chronic fatigue and vague left calf discomfort with prolonged standing

HISTORY OF PRESENT ILLNESS:
She works as a schoolteacher and reports that she has had difficulty standing for long periods due to vague aching in her left calf that comes and goes. She denies chest pain, dyspnea, or palpitations.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus
- Secondary antiphospholipid syndrome
- Factor V Leiden thrombophilia
- History of deep vein thrombosis
- Migraine with aura
- Submucosal uterine leiomyoma
- Cervical insufficiency

Over the past five years, she has had three pregnancies—two of which ended in spontaneous losses during the second trimester. She is otherwise healthy.

PAST SURGICAL HISTORY:
- Dilation and curettage (recurrent pregnancy losses)
- Hysteroscopic myomectomy

MEDICATIONS:
She takes only prenatal vitamins.

Home Medications:
- Hydroxychloroquine 200 mg PO BID
- Prednisone 5 mg PO daily
- Warfarin 5 mg PO daily (therapeutic anticoagulation)
- Aspirin 81 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
Family history reveals that her father had a pulmonary embolism in his 40s.

SOCIAL HISTORY:
She does not smoke, drink alcohol, or use illicit drugs.

REVIEW OF SYSTEMS:
Her menses are regular.

VITAL SIGNS:
Her vital signs are normal.

PHYSICAL EXAMINATION:
Physical exam reveals mild tenderness to deep palpation of the left calf without erythema, edema, or skin changes. Cardiopulmonary and neurologic exams are unremarkable.

LABORATORY DATA:
Initial labs show:
Hemoglobin                                                13.2 g/dL
Platelet count                                            245,000/mm³
Prothrombin time (PT)                                     12.1 seconds
Partial thromboplastin time (PTT)                         30 seconds

OTHER STUDIES:
Further evaluation includes a PTT assay repeated after the addition of activated protein C, which shows no change in clotting time.

## Encounter 1: source 2020-07-15, practice 2021-08-16, outpatient, Obstetrics & Gynecology

Chief complaint: Request for long-term contraception due to heavy menstrual bleeding and medication adherence issues

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Maria Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Request for long-term contraception due to heavy menstrual bleeding and medication adherence issues

HISTORY OF PRESENT ILLNESS:
The patient is a 32-year-old female with systemic lupus erythematosus, secondary antiphospholipid syndrome, and Factor V Leiden thrombophilia diagnosed during her hematology evaluation earlier this year, who presents for long-term contraceptive management. She reports a history of heavy menstrual bleeding and dysmenorrhea, which has caused her to miss work on several occasions. She has tried multiple oral contraceptives but often forgets to take them. She is not currently sexually active but wishes to have a reliable form of contraception in place. The patient expresses concern about the need for daily medication adherence and is interested in a method that requires less frequent attention.

PAST MEDICAL HISTORY:
Active Problem List:
- Factor V Leiden thrombophilia (diagnosed 2020-01-15)
- Systemic lupus erythematosus
- Secondary antiphospholipid syndrome
- History of deep vein thrombosis
- Migraine with aura
- Submucosal uterine leiomyoma
- Cervical insufficiency

Her medical history is significant for migraines with aura.

PAST SURGICAL HISTORY:
- Dilation and curettage (recurrent pregnancy losses)
- Hysteroscopic myomectomy

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg PO BID
- Prednisone 5 mg PO daily
- Warfarin 5 mg PO daily (therapeutic anticoagulation)
- Aspirin 81 mg PO daily
- Prenatal vitamin 1 tablet PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
A family history of venous thromboembolism

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Schoolteacher.

VITAL SIGNS:
Her blood pressure is 122/78 mmHg, and her BMI is 26 kg/m^2.

PHYSICAL EXAMINATION:
On pelvic examination, the uterus is normal in size, and there are no adnexal masses.

## Encounter 2: source 2021-07-15, practice 2022-08-22, outpatient, Hematology

Chief complaint: Evaluation of three first-trimester miscarriages in past two years

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. James Patterson, MD
Department: Hematology
Visit Type: outpatient

CHIEF COMPLAINT:
Evaluation of three first-trimester miscarriages in past two years

HISTORY OF PRESENT ILLNESS:
The patient with SLE and secondary antiphospholipid syndrome returns for evaluation of recurrent pregnancy loss, following prior encounters for thrombophilia and contraception. She has had three miscarriages in the past two years, all occurring before the 10th week of gestation. She denies any recent infections, drug use, or exposure to environmental toxins.

PAST MEDICAL HISTORY:
Active Problem List:
- Factor V Leiden thrombophilia (diagnosed 2020-01-15)
- Encounter for initial prescription of implantable subdermal contraceptive (diagnosed 2020-07-15)
- Systemic lupus erythematosus
- Secondary antiphospholipid syndrome
- History of deep vein thrombosis
- Migraine with aura
- Submucosal uterine leiomyoma
- Cervical insufficiency

PAST SURGICAL HISTORY:
- Dilation and curettage (recurrent pregnancy losses)
- Hysteroscopic myomectomy

MEDICATIONS:
Her medications include hydroxychloroquine and low-dose aspirin.

Home Medications:
- Prednisone 5 mg PO daily
- Warfarin 5 mg PO daily (therapeutic anticoagulation)
- Prenatal vitamin 1 tablet PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Maternal grandmother with systemic lupus erythematosus
- Paternal uncle with history of pulmonary embolism (Factor V Leiden carrier)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Schoolteacher.

VITAL SIGNS:
Her blood pressure is 120/78 mmHg

PHYSICAL EXAMINATION:
On examination, she appears well. and her physical examination is unremarkable.

LABORATORY DATA:
Laboratory studies are ordered, and the results are as follows:

- Complete blood count: within normal limits
- Prothrombin time (PT): 12 seconds (control 11-13.5 seconds)
- Activated partial thromboplastin time (aPTT): 45 seconds (control 25-35 seconds)
- aPTT (after 1:1 dilution with normal plasma): 45 seconds
- Anticardiolipin antibody: Positive
- β2-glycoprotein-I antibody: Positive
- Lupus anticoagulant: Positive

ASSESSMENT AND PLAN:
Based on the patient's history and laboratory findings,

## Encounter 3: source 2022-01-15, practice 2023-02-27, outpatient, Obstetrics & Gynecology

Chief complaint: Three consecutive first-trimester pregnancy losses and desire to conceive

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Aisha Johnson, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Three consecutive first-trimester pregnancy losses and desire to conceive

HISTORY OF PRESENT ILLNESS:
The patient returns to obstetric care for evaluation of recurrent first-trimester pregnancy loss and desire to conceive following prior hematology assessment. She reports that each pregnancy ended spontaneously around 8 to 10 weeks of gestation. Her menstrual cycles are regular, with moderate bleeding lasting 5 days. The patient expresses a strong desire to conceive again.

PAST MEDICAL HISTORY:
Active Problem List:
- Factor V Leiden thrombophilia (diagnosed 2020-01-15)
- Encounter for initial prescription of implantable subdermal contraceptive (diagnosed 2020-07-15)
- Antiphospholipid syndrome (diagnosed 2021-07-15)
- Systemic lupus erythematosus
- Secondary antiphospholipid syndrome
- History of deep vein thrombosis
- Migraine with aura
- Submucosal uterine leiomyoma
- Cervical insufficiency

PAST SURGICAL HISTORY:
- Dilation and curettage (recurrent pregnancy losses)
- Hysteroscopic myomectomy

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg PO BID
- Prednisone 5 mg PO daily
- Warfarin 5 mg PO daily (therapeutic anticoagulation)
- Aspirin 81 mg PO daily
- Prenatal vitamin 1 tablet PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Maternal grandmother with systemic lupus erythematosus
- Paternal uncle with history of pulmonary embolism (Factor V Leiden carrier)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Schoolteacher.

PHYSICAL EXAMINATION:
On pelvic examination, the uterus is normal in size, and there is no adnexal tenderness.

IMAGING:
A transvaginal ultrasound reveals a 2 cm submucosal fibroid impinging on the uterine cavity.

## Encounter 4: source 2022-07-15, practice 2023-08-22, outpatient, Rheumatology

Chief complaint: Intermittent headaches and recent transient episode of left-sided weakness

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. David Kim, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent headaches and recent transient episode of left-sided weakness

HISTORY OF PRESENT ILLNESS:
The patient returns with intermittent headaches and transient left-sided weakness, reporting two first-trimester miscarriages in the past year. Her SLE has been well-controlled on hydroxychloroquine and low-dose prednisone.

PAST MEDICAL HISTORY:
Active Problem List:
- Factor V Leiden thrombophilia (diagnosed 2020-01-15)
- Encounter for initial prescription of implantable subdermal contraceptive (diagnosed 2020-07-15)
- Antiphospholipid syndrome (diagnosed 2021-07-15)
- Submucosal uterine leiomyoma (diagnosed 2022-01-15)
- Systemic lupus erythematosus
- Secondary antiphospholipid syndrome
- History of deep vein thrombosis
- Migraine with aura
- Cervical insufficiency

PAST SURGICAL HISTORY:
- Dilation and curettage (recurrent pregnancy losses)
- Hysteroscopic myomectomy

MEDICATIONS:
She takes hydroxychloroquine and low-dose prednisone.

Home Medications:
- Warfarin 5 mg PO daily (therapeutic anticoagulation)
- Aspirin 81 mg PO daily
- Prenatal vitamin 1 tablet PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Maternal grandmother with systemic lupus erythematosus
- Paternal uncle with history of pulmonary embolism (Factor V Leiden carrier)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Schoolteacher.

REVIEW OF SYSTEMS:
She denies any recent infections, trauma, or new medications.

VITAL SIGNS:
Afebrile, blood pressure is 130/85 mmHg, and pulse is 78 beats per minute

PHYSICAL EXAMINATION:
Neurological examination is nonfocal

LABORATORY DATA:
Hemoglobin: 12.5 g/dL
Platelet count: 180,000/µL
Prothrombin time (PT): 12 seconds (control 11-13.5 seconds)
Activated partial thromboplastin time (aPTT): 42 seconds (control 25-35 seconds)
Anticardiolipin antibody: Positive
Anti-β2-glycoprotein-I antibody: Positive
Lupus anticoagulant: Positive

## Encounter 5: source 2023-07-15, practice 2024-08-26, outpatient, Maternal-Fetal Medicine

Chief complaint: Prenatal visit at 10 weeks gestation with history of recurrent miscarriage and second-trimester loss

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Emily Watson, MD
Department: Maternal-Fetal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Prenatal visit at 10 weeks gestation with history of recurrent miscarriage and second-trimester loss

HISTORY OF PRESENT ILLNESS:
The patient with antiphospholipid syndrome and recurrent pregnancy loss reports that she is currently 10 weeks pregnant and is concerned about the possibility of another miscarriage.

PAST MEDICAL HISTORY:
Active Problem List:
- Factor V Leiden thrombophilia (diagnosed 2020-01-15)
- Encounter for initial prescription of implantable subdermal contraceptive (diagnosed 2020-07-15)
- Antiphospholipid syndrome (diagnosed 2021-07-15)
- Submucosal uterine leiomyoma (diagnosed 2022-01-15)
- Systemic lupus erythematosus
- Secondary antiphospholipid syndrome
- History of deep vein thrombosis
- Migraine with aura
- Cervical insufficiency

PAST SURGICAL HISTORY:
- Dilation and curettage (recurrent pregnancy losses)
- Hysteroscopic myomectomy

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg PO BID
- Prednisone 5 mg PO daily
- Warfarin 5 mg PO daily (therapeutic anticoagulation)
- Aspirin 81 mg PO daily
- Prenatal vitamin 1 tablet PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
No family history of clotting disorders

SOCIAL HISTORY:
She does not smoke

VITAL SIGNS:
Blood pressure is 120/78 mmHg, heart rate is 88 beats per minute

PHYSICAL EXAMINATION:
Physical examination is unremarkable

LABORATORY DATA:
Laboratory tests reveal the following:

- Hemoglobin: 13.2 g/dL
- Platelet count: 110 x 10^9/L
- Activated partial thromboplastin time (aPTT): 42 seconds (control 30 seconds)
- Lupus anticoagulant: Positive
- Anticardiolipin antibodies: Positive
- Anti-β2 glycoprotein I antibodies: Positive

## Encounter 6: source 2023-09-15, practice 2024-10-23, ed, Emergency Medicine

Chief complaint: Acute vaginal bleeding and shortness of breath at 12 weeks gestation

ENCOUNTER NOTE
Date: 2023-09-15
Provider: Dr. Robert Liu, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute vaginal bleeding and shortness of breath at 12 weeks gestation

HISTORY OF PRESENT ILLNESS:
The patient with SLE, secondary APS, and recurrent pregnancy loss presents to the emergency department at 12 weeks gestation following recent MFM evaluation. She reports that the bleeding started 2 weeks ago and has progressively worsened. She has not had prenatal care for this pregnancy. Her last menstrual period was 12 weeks ago, but she notes that she has felt unusually fatigued and has experienced palpitations and heat intolerance.

PAST MEDICAL HISTORY:
Active Problem List:
- Factor V Leiden thrombophilia (diagnosed 2020-01-15)
- Encounter for initial prescription of implantable subdermal contraceptive (diagnosed 2020-07-15)
- Antiphospholipid syndrome (diagnosed 2021-07-15)
- Submucosal uterine leiomyoma (diagnosed 2022-01-15)
- Systemic lupus erythematosus
- Secondary antiphospholipid syndrome
- History of deep vein thrombosis
- Migraine with aura
- Cervical insufficiency

PAST SURGICAL HISTORY:
- Dilation and curettage (recurrent pregnancy losses)
- Hysteroscopic myomectomy

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg PO BID
- Prednisone 5 mg PO daily
- Warfarin 5 mg PO daily (therapeutic anticoagulation)
- Aspirin 81 mg PO daily
- Prenatal vitamin 1 tablet PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Maternal grandmother with systemic lupus erythematosus
- Paternal uncle with history of pulmonary embolism (Factor V Leiden carrier)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Schoolteacher.

VITAL SIGNS:
On examination, her blood pressure is 130/85 mmHg, heart rate is 110 bpm, and respiratory rate is 20 breaths per minute.

PHYSICAL EXAMINATION:
Her thyroid gland is diffusely enlarged and non-tender. Pelvic examination reveals a slightly enlarged uterus and small amounts of blood in the vaginal vault.

LABORATORY DATA:
Laboratory results are as follows:

- Human chorionic gonadotropin (hCG): 450,000 mIU/mL (normal range for gestational week 12: 15,000-200,000 mIU/mL)
- Thyroid-stimulating hormone (TSH): <0.01 µIU/mL (normal range: 0.4-4.0 µIU/mL)
- Free thyroxine (T4): 3.2 ng/dL (normal range: 0.8-1.8 ng/dL)

IMAGING:
An ultrasound of the pelvis shows an enlarged uterus with a heterogeneous, multicystic mass occupying the uterine cavity.

## Encounter 7: source 2024-09-15, practice 2025-10-20, outpatient, Obstetrics & Gynecology

Chief complaint: Routine prenatal visit with history of cervical insufficiency

ENCOUNTER NOTE
Date: 2024-09-15
Provider: Dr. Jennifer Patel, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine prenatal visit with history of cervical insufficiency

PAST MEDICAL HISTORY:
Active Problem List:
- Factor V Leiden thrombophilia (diagnosed 2020-01-15)
- Encounter for initial prescription of implantable subdermal contraceptive (diagnosed 2020-07-15)
- Antiphospholipid syndrome (diagnosed 2021-07-15)
- Submucosal uterine leiomyoma (diagnosed 2022-01-15)
- Partial hydatidiform mole (diagnosed 2023-09-15)
- Systemic lupus erythematosus
- Secondary antiphospholipid syndrome
- History of deep vein thrombosis
- Migraine with aura
- Cervical insufficiency

Her first pregnancy ended in a spontaneous miscarriage at 20 weeks, which she describes as a sudden loss of amniotic fluid followed by the delivery of the fetus without any preceding pain or contractions. Her medical history is significant for a high-grade squamous intraepithelial lesion.

PAST SURGICAL HISTORY:
She underwent a cone biopsy 3 years ago.

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg PO BID
- Prednisone 5 mg PO daily
- Warfarin 5 mg PO daily (therapeutic anticoagulation)
- Aspirin 81 mg PO daily
- Prenatal vitamin 1 tablet PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Maternal grandmother with systemic lupus erythematosus
- Paternal uncle with history of pulmonary embolism (Factor V Leiden carrier)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Schoolteacher.

REVIEW OF SYSTEMS:
She denies any vaginal bleeding or discharge.

VITAL SIGNS:
Her vital signs are within normal limits

PHYSICAL EXAMINATION:
The uterine size is appropriate for gestational age. A speculum examination reveals a dilated cervix with visible amniotic membranes bulging through the external os.

IMAGING:
Transvaginal ultrasonography shows a cervical length of 20 mm with funneling of the internal os.
