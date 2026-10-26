# synthetic_hospital patient 1762

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex F, African American, Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-02-19 (Wed) | outpatient | Rheumatology |
| 1 | 2020-04-15 | 2020-05-20 (Wed) | outpatient | Obstetrics & Gynecology |
| 2 | 2020-09-15 | 2020-10-15 (Thu) | outpatient | Internal Medicine |
| 3 | 2021-03-15 | 2021-04-19 (Mon) | outpatient | Orthopedics |
| 4 | 2022-01-15 | 2022-02-04 (Fri) | outpatient | Obstetrics & Gynecology |
| 5 | 2022-02-15 | 2022-03-14 (Mon) | outpatient | Obstetrics & Gynecology |
| 6 | 2025-01-15 | 2025-02-11 (Tue) | ed | Emergency Medicine |
| 7 | 2026-01-15 | 2026-02-07 (Sat) | icu | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Blue Cross Blue Shield PPO",
 "occupation": "High school mathematics teacher",
 "alcohol_use": "Social, 1-2 drinks monthly",
 "family_history": [
  "Mother: Rheumatoid arthritis",
  "Maternal aunt: Systemic lupus erythematosus",
  "Father: Hypertension"
 ],
 "race_ethnicity": "African American",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "200 mg by mouth twice daily",
   "name": "Hydroxychloroquine"
  },
  {
   "dose": "5 mg by mouth daily",
   "name": "Prednisone"
  },
  {
   "dose": "81 mg by mouth daily",
   "name": "Aspirin"
  }
 ],
 "surgical_history": [
  "Dilation and curettage for incomplete spontaneous abortion (\u00d73)",
  "Core decompression of right femoral head (avascular necrosis)"
 ],
 "chronic_conditions": [
  "Systemic lupus erythematosus (diagnosed age 27)",
  "Secondary antiphospholipid syndrome",
  "Avascular necrosis of right femoral head",
  "History of transient ischemic attack",
  "History of recurrent pregnancy loss (three first-trimester miscarriages, one second-trimester fetal loss)"
 ],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Antiphospholipid syndrome", "Avascular necrosis of femoral head", "Tricyclic Antidepressant Overdose", "Torsades de Pointes"]
Comorbidities: ["False positive syphilis serology", "Recurrent pregnancy loss", "Systemic lupus erythematosus"]

## Encounter 0: source 2020-01-15, practice 2020-02-19, outpatient, Rheumatology

Chief complaint: Persistent fatigue and intermittent joint stiffness in fingers and wrists

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent fatigue and intermittent joint stiffness in fingers and wrists

HISTORY OF PRESENT ILLNESS:
That has developed gradually over the past few months. She notes that her fingers and wrists feel tight in the morning, but the symptoms improve throughout the day. She denies any rash, photosensitivity, oral ulcers, or Raynaud symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus (diagnosed age 27)
- Secondary antiphospholipid syndrome
- Avascular necrosis of right femoral head
- History of transient ischemic attack
- History of recurrent pregnancy loss (three first-trimester miscarriages, one second-trimester fetal loss)

She recalls being told a year ago that her syphilis screening test (RPR) was positive, but the confirmatory treponemal-specific test was negative.

PAST SURGICAL HISTORY:
- Dilation and curettage for incomplete spontaneous abortion (×3)
- Core decompression of right femoral head (avascular necrosis)

MEDICATIONS:
She takes she does not take any daily medications.

Home Medications:
- Hydroxychloroquine 200 mg by mouth twice daily
- Prednisone 5 mg by mouth daily
- Aspirin 81 mg by mouth daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Rheumatoid arthritis
- Maternal aunt: Systemic lupus erythematosus
- Father: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks monthly. Occupation: High school mathematics teacher.

PHYSICAL EXAMINATION:
Physical examination is normal.

LABORATORY DATA:
Current laboratory results reveal a prolonged activated partial thromboplastin time (aPTT) with normal prothrombin time (PT). Complete blood count and kidney function are within normal limits.

## Encounter 1: source 2020-04-15, practice 2020-05-20, outpatient, Obstetrics & Gynecology

Chief complaint: Requesting contraception advice

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Maria Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Requesting contraception advice

HISTORY OF PRESENT ILLNESS:
The patient with systemic lupus erythematosus, secondary antiphospholipid syndrome, and recurrent pregnancy loss returns for gynecologic consultation following her January rheumatology visit. Her menstrual cycles are regular, and she has no history of sexually transmitted infections. She has never used any form of contraception before and is currently in a monogamous relationship.

PAST MEDICAL HISTORY:
Active Problem List:
- Antiphospholipid syndrome (diagnosed 2020-01-15)
- Systemic lupus erythematosus (diagnosed age 27)
- Secondary antiphospholipid syndrome
- Avascular necrosis of right femoral head
- History of transient ischemic attack
- History of recurrent pregnancy loss (three first-trimester miscarriages, one second-trimester fetal loss)

She has a history of recurrent miscarriages and recently tested positive for antiphospholipid antibodies.

PAST SURGICAL HISTORY:
- Dilation and curettage for incomplete spontaneous abortion (×3)
- Core decompression of right femoral head (avascular necrosis)

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg by mouth twice daily
- Prednisone 5 mg by mouth daily
- Aspirin 81 mg by mouth daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
She has no family history of thromboembolic events

SOCIAL HISTORY:
She does not smoke

VITAL SIGNS:
Her blood pressure is 120/78 mmHg, and her BMI is 24 kg/m^2

PHYSICAL EXAMINATION:
On examination, her physical findings are unremarkable

## Encounter 2: source 2020-09-15, practice 2020-10-15, outpatient, Internal Medicine

Chief complaint: Evaluation of recurrent pregnancy loss history and recent transient neurologic episode

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. James Wilson, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Evaluation of recurrent pregnancy loss history and recent transient neurologic episode

HISTORY OF PRESENT ILLNESS:
The patient with secondary antiphospholipid syndrome and recurrent pregnancy loss returns for follow-up. She reports a transient episode of right-sided weakness and aphasia last month, which resolved within 24 hours.

PAST MEDICAL HISTORY:
Active Problem List:
- Antiphospholipid syndrome (diagnosed 2020-01-15)
- Systemic lupus erythematosus (diagnosed age 27)
- Secondary antiphospholipid syndrome
- Avascular necrosis of right femoral head
- History of transient ischemic attack
- History of recurrent pregnancy loss (three first-trimester miscarriages, one second-trimester fetal loss)

PAST SURGICAL HISTORY:
- Dilation and curettage for incomplete spontaneous abortion (×3)
- Core decompression of right femoral head (avascular necrosis)

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg by mouth twice daily
- Prednisone 5 mg by mouth daily
- Aspirin 81 mg by mouth daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
Her family history is notable for a maternal aunt with systemic lupus erythematosus.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks monthly. Occupation: High school mathematics teacher.

VITAL SIGNS:
Blood pressure is 120/78 mmHg, heart rate is 72 beats per minute

PHYSICAL EXAMINATION:
The rest of the physical examination is unremarkable.

LABORATORY DATA:
Laboratory studies reveal:

- Hemoglobin: 13.2 g/dL
- Platelet count: 210,000/mm^3
- Prothrombin time (PT): 12 seconds (control 11-13.5 seconds)
- Activated partial thromboplastin time (aPTT): 38 seconds (control 25-35 seconds)
- Anticardiolipin antibodies (IgG and IgM): Elevated
- Lupus anticoagulant: Positive
- Beta-2 glycoprotein I antibodies: Positive

## Encounter 3: source 2021-03-15, practice 2021-04-19, outpatient, Orthopedics

Chief complaint: Progressive right groin pain exacerbated by walking and climbing stairs

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Robert Kim, MD
Department: Orthopedics
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive right groin pain exacerbated by walking and climbing stairs

HISTORY OF PRESENT ILLNESS:
This 33-year-old female with systemic lupus erythematosus, secondary antiphospholipid syndrome, and history of recurrent pregnancy loss and transient ischemic attack presents with progressive right groin pain. She reports that the pain is exacerbated by walking and climbing stairs.

PAST MEDICAL HISTORY:
Active Problem List:
- Antiphospholipid syndrome (diagnosed 2020-01-15)
- Systemic lupus erythematosus (diagnosed age 27)
- Secondary antiphospholipid syndrome
- Avascular necrosis of right femoral head
- History of transient ischemic attack
- History of recurrent pregnancy loss (three first-trimester miscarriages, one second-trimester fetal loss)

PAST SURGICAL HISTORY:
- Dilation and curettage for incomplete spontaneous abortion (×3)
- Core decompression of right femoral head (avascular necrosis)

MEDICATIONS:
Her SLE has been managed with hydroxychloroquine and chronic prednisone therapy.

Home Medications:
- Aspirin 81 mg by mouth daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Rheumatoid arthritis
- Maternal aunt: Systemic lupus erythematosus
- Father: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks monthly. Occupation: High school mathematics teacher.

PHYSICAL EXAMINATION:
On physical examination, she has a full range of motion in her right hip, but there is pain with internal rotation.

LABORATORY DATA:
Her recent laboratory tests are as follows:

- Erythrocyte sedimentation rate (ESR): 20 mm/hr
- C-reactive protein (CRP): 2.0 mg/L
- Antinuclear antibody (ANA) titer: 1:640 (homogeneous pattern)
- Anti-double-stranded DNA (anti-dsDNA): Positive
- Complete blood count (CBC): Within normal limits

IMAGING:
Plain radiographs of the pelvis and right hip are unremarkable.

ASSESSMENT AND PLAN:
Given the patient's history and symptoms,

## Encounter 4: source 2022-01-15, practice 2022-02-04, outpatient, Obstetrics & Gynecology

Chief complaint: First prenatal visit for high-risk pregnancy

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Lisa Johnson, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
First prenatal visit for high-risk pregnancy

HISTORY OF PRESENT ILLNESS:
Returning for prenatal care after prior pregnancy losses, she expresses concern about carrying the pregnancy to term because she has had three prior first-trimester miscarriages.

PAST MEDICAL HISTORY:
Active Problem List:
- Antiphospholipid syndrome (diagnosed 2020-01-15)
- Avascular necrosis of femoral head (diagnosed 2021-03-15)
- Systemic lupus erythematosus (diagnosed age 27)
- Secondary antiphospholipid syndrome
- Avascular necrosis of right femoral head
- History of transient ischemic attack
- History of recurrent pregnancy loss (three first-trimester miscarriages, one second-trimester fetal loss)

She has a history of systemic lupus erythematosus (SLE) diagnosed 5 years ago.

PAST SURGICAL HISTORY:
- Dilation and curettage for incomplete spontaneous abortion (×3)
- Core decompression of right femoral head (avascular necrosis)

MEDICATIONS:
She is currently maintained on hydroxychloroquine.

Home Medications:
- Prednisone 5 mg by mouth daily
- Aspirin 81 mg by mouth daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Rheumatoid arthritis
- Maternal aunt: Systemic lupus erythematosus
- Father: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks monthly. Occupation: High school mathematics teacher.

VITAL SIGNS:
Her vital signs are normal

PHYSICAL EXAMINATION:
Physical exam is unremarkable

LABORATORY DATA:
Laboratory tests show a positive ANA and mildly elevated aPTT

PLAN:
A coagulation panel is ordered, and further testing is planned to evaluate the cause of pregnancy loss

## Encounter 5: source 2022-02-15, practice 2022-03-14, outpatient, Obstetrics & Gynecology

Chief complaint: Evaluation of current pregnancy at 10 weeks gestation

ENCOUNTER NOTE
Date: 2022-02-15
Provider: Dr. Lisa Johnson, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Evaluation of current pregnancy at 10 weeks gestation

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up evaluation at 10 weeks gestation following her initial prenatal visit last month, presenting with a history of systemic lupus erythematosus, secondary antiphospholipid syndrome, recurrent pregnancy loss, transient ischemic attack, and avascular necrosis of the right femoral head. Her current pregnancy was achieved naturally.

PAST MEDICAL HISTORY:
Active Problem List:
- Antiphospholipid syndrome (diagnosed 2020-01-15)
- Avascular necrosis of femoral head (diagnosed 2021-03-15)
- Systemic lupus erythematosus (diagnosed age 27)
- Secondary antiphospholipid syndrome
- Avascular necrosis of right femoral head
- History of transient ischemic attack
- History of recurrent pregnancy loss (three first-trimester miscarriages, one second-trimester fetal loss)

She has a history of three prior spontaneous miscarriages, all of which occurred in the first trimester. She reports no abdominal pain, vaginal bleeding, or infections during those pregnancies. She has a known autoimmune condition diagnosed several years ago.

PAST SURGICAL HISTORY:
- Dilation and curettage for incomplete spontaneous abortion (×3)
- Core decompression of right femoral head (avascular necrosis)

MEDICATIONS:
She takes she is taking prenatal vitamins and low-dose aspirin. for which she takes hydroxychloroquine.

Home Medications:
- Prednisone 5 mg by mouth daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Rheumatoid arthritis
- Maternal aunt: Systemic lupus erythematosus
- Father: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks monthly. Occupation: High school mathematics teacher.

VITAL SIGNS:
Her vital signs are normal

PHYSICAL EXAMINATION:
Fetal heart tones are present

LABORATORY DATA:
Routine laboratory testing reveals a positive VDRL screening test, but the confirmatory FTA-ABS test is negative. Repeat VDRL testing 12 weeks later remains positive. Additional labs show no evidence of infection, and metabolic and hematologic studies are within normal limits.

## Encounter 6: source 2025-01-15, practice 2025-02-11, ed, Emergency Medicine

Chief complaint: Two-hour history of palpitations and near-syncope

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. David Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-hour history of palpitations and near-syncope

HISTORY OF PRESENT ILLNESS:
This 37-year-old female with systemic lupus erythematosus, secondary antiphospholipid syndrome, and a history of transient ischemic attack presents with a two-hour history of palpitations and near-syncope. She denies any chest pain, shortness of breath, or previous cardiac history.

PAST MEDICAL HISTORY:
Active Problem List:
- Antiphospholipid syndrome (diagnosed 2020-01-15)
- Avascular necrosis of femoral head (diagnosed 2021-03-15)
- Systemic lupus erythematosus (diagnosed age 27)
- Secondary antiphospholipid syndrome
- Avascular necrosis of right femoral head
- History of transient ischemic attack
- History of recurrent pregnancy loss (three first-trimester miscarriages, one second-trimester fetal loss)

She has a history of systemic lupus erythematosus.

PAST SURGICAL HISTORY:
- Dilation and curettage for incomplete spontaneous abortion (×3)
- Core decompression of right femoral head (avascular necrosis)

MEDICATIONS:
She has been recently started on hydroxychloroquine for joint pain.

Home Medications:
- Prednisone 5 mg by mouth daily
- Aspirin 81 mg by mouth daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Rheumatoid arthritis
- Maternal aunt: Systemic lupus erythematosus
- Father: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks monthly. Occupation: High school mathematics teacher.

VITAL SIGNS:
Her blood pressure is 110/70 mmHg, heart rate is 120 beats per minute

PHYSICAL EXAMINATION:
She is alert and oriented

LABORATORY DATA:
Her serum potassium is 4.2 mEq/L, calcium is 9.1 mg/dL, and magnesium is 1.8 mg/dL

OTHER STUDIES:
An ECG shows a polymorphic ventricular tachycardia with a varying QRS complex morphology and no discernible P waves

## Encounter 7: source 2026-01-15, practice 2026-02-07, icu, Emergency Medicine

Chief complaint: Found unresponsive at home with empty amitriptyline pill bottle

ENCOUNTER NOTE
Date: 2026-01-15
Provider: Dr. Emily Thompson, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Found unresponsive at home with empty amitriptyline pill bottle

HISTORY OF PRESENT ILLNESS:
The patient is a 38-year-old female with systemic lupus erythematosus diagnosed at age 27, secondary antiphospholipid syndrome, avascular necrosis of the right femoral head, and history of transient ischemic attack and recurrent pregnancy loss including three first-trimester miscarriages and one second-trimester fetal loss. She was evaluated in this emergency department for Torsades de Pointes approximately one year prior to presentation and is brought to the emergency department by her friends after they

PAST MEDICAL HISTORY:
Active Problem List:
- Antiphospholipid syndrome (diagnosed 2020-01-15)
- Avascular necrosis of femoral head (diagnosed 2021-03-15)
- Torsades de Pointes (diagnosed 2025-01-15)
- Systemic lupus erythematosus (diagnosed age 27)
- Secondary antiphospholipid syndrome
- Avascular necrosis of right femoral head
- History of transient ischemic attack
- History of recurrent pregnancy loss (three first-trimester miscarriages, one second-trimester fetal loss)

PAST SURGICAL HISTORY:
- Dilation and curettage for incomplete spontaneous abortion (×3)
- Core decompression of right femoral head (avascular necrosis)

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg by mouth twice daily
- Prednisone 5 mg by mouth daily
- Aspirin 81 mg by mouth daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Rheumatoid arthritis
- Maternal aunt: Systemic lupus erythematosus
- Father: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 drinks monthly. Occupation: High school mathematics teacher.

VITAL SIGNS:
On examination, the patient is tachycardic with a heart rate of 120 bpm, hypotensive with a blood pressure of 90/50 mmHg, and febrile with a temperature of 38.5°C (101.3°F).

PHYSICAL EXAMINATION:
She is agitated and disoriented. Her pupils are dilated and poorly responsive to light. The skin is dry and flushed.

LABORATORY DATA:
Laboratory studies are as follows:

- Serum sodium: 140 mEq/L
- Serum potassium: 4.0 mEq/L
- Serum bicarbonate: 22 mEq/L
- Serum creatinine: 1.1 mg/dL
- Serum amitriptyline level: Pending

OTHER STUDIES:
An electrocardiogram (ECG) shows a sinus tachycardia with a QRS duration of 120 msec.
