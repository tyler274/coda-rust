# synthetic_hospital patient 1820

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 28 at first encounter, sex F, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-01-31 (Fri) | outpatient | Obstetrics & Gynecology |
| 1 | 2020-07-15 | 2020-07-18 (Sat) | inpatient | Obstetrics & Gynecology |
| 2 | 2022-07-15 | 2022-07-24 (Sun) | icu | Obstetrics & Gynecology |
| 3 | 2024-07-15 | 2024-07-20 (Sat) | icu | Obstetrics & Gynecology |
| 4 | 2026-07-15 | 2026-07-28 (Tue) | inpatient | Obstetrics & Gynecology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicaid",
 "occupation": "Administrative coordinator",
 "alcohol_use": "None",
 "family_history": [
  "Mother with type 2 diabetes mellitus",
  "Paternal grandmother with breast cancer"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 6 years ago",
 "home_medications": [
  {
   "dose": "1 tablet daily",
   "name": "Prenatal vitamin"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Chronic hepatitis C infection",
  "History of intravenous drug use disorder in sustained remission"
 ],
 "age_at_first_encounter": 28
}
```

Primary diagnoses: ["Fetal distress (Category III fetal heart rate tracing)", "Placental abruption", "Susceptibility to hepatitis B virus infection", "Preterm premature rupture of membranes (PPROM)", "Acute fatty liver of pregnancy"]
Comorbidities: ["Pregnancy at 32 weeks gestation"]

## Encounter 0: source 2020-01-15, practice 2020-01-31, outpatient, Obstetrics & Gynecology

Chief complaint: Routine prenatal visit at 10 weeks gestation

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine prenatal visit at 10 weeks gestation

HISTORY OF PRESENT ILLNESS:
She reports compliance with her prenatal vitamins and has had an uneventful pregnancy thus far.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic hepatitis C infection
- History of intravenous drug use disorder in sustained remission

She has a known history of chronic hepatitis C infection, which was diagnosed 5 years ago during a routine health screening. She has no known history of hepatitis A or B infection and is unsure of her immunization status.

MEDICATIONS:
She reports compliance with her prenatal vitamins

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with type 2 diabetes mellitus
- Paternal grandmother with breast cancer

SOCIAL HISTORY:
Her past medical history is significant for intravenous drug use, which she discontinued 6 years ago.

REVIEW OF SYSTEMS:
She denies any history of jaundice, pruritus, or abdominal pain.

VITAL SIGNS:
Her vital signs are within normal limits

PHYSICAL EXAMINATION:
Her physical examination is unremarkable

LABORATORY DATA:
Laboratory studies from her first trimester are as follows:

- Hepatitis C virus RNA, quantitative: 1,500,000 IU/mL
- Hepatitis B surface antigen: Negative
- Hepatitis B core antibody, total: Negative
- Hepatitis B surface antibody: Negative
- Hepatitis A IgG: Negative

## Encounter 1: source 2020-07-15, practice 2020-07-18, inpatient, Obstetrics & Gynecology

Chief complaint: Clear fluid leaking from vagina for 2 hours at 32 weeks gestation

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Maria Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
Clear fluid leaking from vagina for 2 hours at 32 weeks gestation

HISTORY OF PRESENT ILLNESS:
Patient with chronic hepatitis C and IVDU in remission presents at 32 weeks after 10-week visit. She denies contractions, vaginal bleeding, or recent sexual intercourse. Pregnancy has been uncomplicated with appropriate prenatal care.

PAST MEDICAL HISTORY:
Active Problem List:
- Susceptibility to hepatitis B virus infection (diagnosed 2020-01-15)
- Chronic hepatitis C infection
- History of intravenous drug use disorder in sustained remission

MEDICATIONS:
Home Medications:
- Prenatal vitamin 1 tablet daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with type 2 diabetes mellitus
- Paternal grandmother with breast cancer

SOCIAL HISTORY:
Smoking: Former smoker, quit 6 years ago. Alcohol: None. Occupation: Administrative coordinator.

VITAL SIGNS:
On examination, her vital signs are stable: blood pressure 120/75 mmHg, heart rate 80 bpm, respiratory rate 16 breaths/min, and temperature 37.0°C (98.6°F).

PHYSICAL EXAMINATION:
Speculum examination confirms pooling of clear fluid in the posterior fornix. Nitrazine test is positive, and ferning is observed under the microscope. Fetal heart tracing is reassuring with a baseline of 140 bpm, moderate variability, no decelerations, and no contractions.

LABORATORY DATA:
Complete blood count shows:

- Hemoglobin: 12.5 g/dL
- Leukocytes: 9,000/mm³
- Platelets: 250,000/mm³

C-reactive protein is within normal limits.

PLAN:
The patient is admitted for observation.

## Encounter 2: source 2022-07-15, practice 2022-07-24, icu, Obstetrics & Gynecology

Chief complaint: Severe itching of palms and soles for 2 weeks at 35 weeks gestation

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. James O'Brien, MD
Department: Obstetrics & Gynecology
Visit Type: icu

CHIEF COMPLAINT:
Severe itching of palms and soles for 2 weeks at 35 weeks gestation

HISTORY OF PRESENT ILLNESS:
This 30-year-old with chronic hepatitis C, intravenous drug use in remission, and prior PPROM presents at 35 weeks with pruritus. She reports the itching is most intense on the palms of her hands and soles of her feet and occurs mostly at night. She denies rash, jaundice, nausea, vomiting, or change in bowel habits.

PAST MEDICAL HISTORY:
Active Problem List:
- Susceptibility to hepatitis B virus infection (diagnosed 2020-01-15)
- Preterm premature rupture of membranes (PPROM) (diagnosed 2020-07-15)
- Chronic hepatitis C infection
- History of intravenous drug use disorder in sustained remission

Her pregnancy has been uncomplicated until now.

MEDICATIONS:
Home Medications:
- Prenatal vitamin 1 tablet daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with type 2 diabetes mellitus
- Paternal grandmother with breast cancer

SOCIAL HISTORY:
Smoking: Former smoker, quit 6 years ago. Alcohol: None. Occupation: Administrative coordinator.

PHYSICAL EXAMINATION:
On examination, there are no skin lesions or rashes noted, and her abdomen is appropriately sized for gestational age with normal fetal heart tones.

LABORATORY DATA:
Laboratory tests reveal the following:

- Total bile acids: 45 µmol/L (normal <10 µmol/L)
- Alanine aminotransferase (ALT): 70 U/L (normal 7-56 U/L)
- Aspartate aminotransferase (AST): 68 U/L (normal 10-40 U/L)
- Alkaline phosphatase: 120 U/L (normal 30-120 U/L)
- Total bilirubin: 1.2 mg/dL (normal 0.1-1.2 mg/dL)

## Encounter 3: source 2024-07-15, practice 2024-07-20, icu, Obstetrics & Gynecology

Chief complaint: Sudden severe abdominal pain and vaginal bleeding at 34 weeks gestation

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Aisha Patel, MD
Department: Obstetrics & Gynecology
Visit Type: icu

CHIEF COMPLAINT:
Sudden severe abdominal pain and vaginal bleeding at 34 weeks gestation

HISTORY OF PRESENT ILLNESS:
The patient with chronic hepatitis C, intravenous drug use disorder in sustained remission, and prior preterm premature rupture of membranes and acute fatty liver of pregnancy presents with sharp constant pain rated 8 of 10 severity. She denies trauma or recent intercourse. Prenatal course was complicated by severe polyhydramnios.

PAST MEDICAL HISTORY:
Active Problem List:
- Susceptibility to hepatitis B virus infection (diagnosed 2020-01-15)
- Preterm premature rupture of membranes (PPROM) (diagnosed 2020-07-15)
- Acute fatty liver of pregnancy (diagnosed 2022-07-15)
- Chronic hepatitis C infection
- History of intravenous drug use disorder in sustained remission

MEDICATIONS:
Home Medications:
- Prenatal vitamin 1 tablet daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with type 2 diabetes mellitus
- Paternal grandmother with breast cancer

SOCIAL HISTORY:
Smoking: Former smoker, quit 6 years ago. Alcohol: None. Occupation: Administrative coordinator.

VITAL SIGNS:
Blood pressure is 140/90 mmHg, heart rate is 110 bpm

PHYSICAL EXAMINATION:
On examination, the uterus is tender to palpation with frequent contractions. Fetal heart rate monitoring shows a repetitive late deceleration pattern.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 10.2 g/dL (baseline 12.5 g/dL)
- Platelet count: 210,000/mm³
- Prothrombin time (PT): 12 seconds (control 11-13.5 seconds)
- Activated partial thromboplastin time (aPTT): 30 seconds (control 25-35 seconds)
- Fibrinogen: 400 mg/dL (normal 200-400 mg/dL)

## Encounter 4: source 2026-07-15, practice 2026-07-28, inpatient, Obstetrics & Gynecology

Chief complaint: Sudden severe abdominal pain, vaginal bleeding, and decreased fetal movement at 39 weeks gestation

ENCOUNTER NOTE
Date: 2026-07-15
Provider: Dr. David Kim, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
Sudden severe abdominal pain, vaginal bleeding, and decreased fetal movement at 39 weeks gestation

HISTORY OF PRESENT ILLNESS:
This patient with chronic hepatitis C, remote IVDU, and prior abruption presents to the labor and delivery unit with 1-hour history of sudden onset of severe abdominal pain and vaginal bleeding. She reports that the baby has been moving less than usual.

PAST MEDICAL HISTORY:
Active Problem List:
- Susceptibility to hepatitis B virus infection (diagnosed 2020-01-15)
- Preterm premature rupture of membranes (PPROM) (diagnosed 2020-07-15)
- Acute fatty liver of pregnancy (diagnosed 2022-07-15)
- Placental abruption (diagnosed 2024-07-15)
- Chronic hepatitis C infection
- History of intravenous drug use disorder in sustained remission

Her pregnancy has been uncomplicated until now, and she has had appropriate prenatal care.

MEDICATIONS:
Home Medications:
- Prenatal vitamin 1 tablet daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with type 2 diabetes mellitus
- Paternal grandmother with breast cancer

SOCIAL HISTORY:
Smoking: Former smoker, quit 6 years ago. Alcohol: None. Occupation: Administrative coordinator.

VITAL SIGNS:
Her blood pressure is 120/75 mmHg, heart rate is 88 bpm, and temperature is 37.0°C (98.6°F).

PHYSICAL EXAMINATION:
On examination, her abdomen is tender to palpation in the uterine area, and the uterus is hypertonic.

IMAGING:
There are no signs of placental abruption on ultrasound.

OTHER STUDIES:
Fetal heart monitoring shows recurrent late decelerations, minimal variability, and no accelerations after a 10-minute observation period.

PLAN:
The patient is given oxygen by face mask, IV fluids are started, and she is placed in the left lateral decubitus position. Despite these measures, the fetal heart tracing does not improve.
