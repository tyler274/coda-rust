# synthetic_hospital patient 1749

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 31 at first encounter, sex F, Non-Hispanic White, Blue Cross PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-01-08 (Mon) | inpatient | Obstetrics & Gynecology |
| 1 | 2021-01-15 | 2025-01-01 (Wed) | inpatient | Obstetrics & Gynecology |
| 2 | 2022-01-15 | 2026-01-01 (Thu) | icu | Obstetrics & Gynecology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross PPO",
 "occupation": "High school mathematics teacher",
 "alcohol_use": "Social (1-2 glasses of wine per month)",
 "family_history": [
  "Mother with history of cesarean delivery",
  "Sister with recurrent HSV-2 infection",
  "Maternal aunt with osteoporosis"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "500 mg PO twice daily for 3 days during prodromal symptoms (not taken during current pregnancy)",
   "name": "Valacyclovir"
  }
 ],
 "surgical_history": [
  "Low transverse cesarean delivery (age 28)",
  "Uncomplicated spontaneous vaginal delivery (age 24)"
 ],
 "chronic_conditions": [
  "Recurrent genital herpes simplex virus infection"
 ],
 "age_at_first_encounter": 31
}
```

Primary diagnoses: ["Uterine rupture", "Active genital herpes simplex virus infection at term", "External anal sphincter injury"]
Comorbidities: ["Term pregnancy"]

## Encounter 0: source 2020-01-15, practice 2024-01-08, inpatient, Obstetrics & Gynecology

Chief complaint: Admitted in spontaneous labor at 39 weeks gestation with prolonged second stage requiring operative vaginal assistance

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Amara Patel, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
Admitted in spontaneous labor at 39 weeks gestation with prolonged second stage requiring operative vaginal assistance

HISTORY OF PRESENT ILLNESS:
Her prenatal course was uncomplicated. Labor progresses slowly but steadily, and after several hours of pushing, the fetal head remains low and engaged. Due to concerns about maternal exhaustion and soft tissue resistance, a small posterior incision is made at the time of crowning to assist delivery. The newborn is delivered without instrumentation and has Apgar scores of 8 and 9 at 1 and 5 minutes, respectively. A few hours later, the patient reports difficulty sensing the urge to defecate and has an episode of stool leakage.

PAST MEDICAL HISTORY:
Active Problem List:
- Recurrent genital herpes simplex virus infection

PAST SURGICAL HISTORY:
No relevant surgical history

MEDICATIONS:
Home Medications:
- Valacyclovir 500 mg PO twice daily for 3 days during prodromal symptoms (not taken during current pregnancy)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with history of cesarean delivery
- Sister with recurrent HSV-2 infection
- Maternal aunt with osteoporosis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per month). Occupation: High school mathematics teacher.

PHYSICAL EXAMINATION:
Pelvic exam reveals the incision site extending slightly beyond the planned limit, but no evidence of rectal mucosa disruption.

## Encounter 1: source 2021-01-15, practice 2025-01-01, inpatient, Obstetrics & Gynecology

Chief complaint: Regular contractions every 5 minutes for 2 hours with painful vulvar vesicular lesions noted at 39 weeks gestation

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Sarah Chen, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
Regular contractions every 5 minutes for 2 hours with painful vulvar vesicular lesions noted at 39 weeks gestation

HISTORY OF PRESENT ILLNESS:
This 32-year-old with recurrent genital HSV and prior sphincter injury presents at 39 weeks. She reports that her first child was delivered vaginally. Prenatal course was unremarkable until one week ago when she developed painful vesicular lesions on her vulva.

PAST MEDICAL HISTORY:
Active Problem List:
- External anal sphincter injury (diagnosed 2020-01-15)
- Recurrent genital herpes simplex virus infection

She has a history of genital herpes but has not had an outbreak for over a year and did not receive antiviral prophylaxis during this pregnancy.

PAST SURGICAL HISTORY:
- Low transverse cesarean delivery (age 28)
- Uncomplicated spontaneous vaginal delivery (age 24)

MEDICATIONS:
Home Medications:
- Valacyclovir 500 mg PO twice daily for 3 days during prodromal symptoms (not taken during current pregnancy)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with history of cesarean delivery
- Sister with recurrent HSV-2 infection
- Maternal aunt with osteoporosis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per month). Occupation: High school mathematics teacher.

VITAL SIGNS:
On examination, her temperature is 37.2°C (99°F), blood pressure is 120/78 mmHg, and fetal heart rate tracing is reassuring.

PHYSICAL EXAMINATION:
Physical examination reveals multiple clustered vesicles and shallow ulcers on the labia majora. No lesions are noted on the cervix on speculum examination.

## Encounter 2: source 2022-01-15, practice 2026-01-01, icu, Obstetrics & Gynecology

Chief complaint: Acute onset of severe lower abdominal pain during active labor with sudden prolonged fetal bradycardia to 60 bpm

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Michael Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: icu

CHIEF COMPLAINT:
Acute onset of severe lower abdominal pain during active labor with sudden prolonged fetal bradycardia to 60 bpm

HISTORY OF PRESENT ILLNESS:
The patient, who has recurrent genital HSV and prior deliveries complicated by sphincter injury and active genital HSV at term, presents with regular contractions every 2–3 minutes. Her current pregnancy has been uncomplicated, and she is attempting a trial of labor after cesarean (TOLAC). Suddenly, the fetal heart rate drops from 140/min to 60/min and stays low for over 3 minutes. The patient reports sharp lower abdominal pain.

PAST MEDICAL HISTORY:
Active Problem List:
- External anal sphincter injury (diagnosed 2020-01-15)
- Active genital herpes simplex virus infection at term (diagnosed 2021-01-15)
- Recurrent genital herpes simplex virus infection

Her past medical history is notable for a low transverse cesarean delivery 3 years ago for nonreassuring fetal status.

PAST SURGICAL HISTORY:
- Low transverse cesarean delivery (age 28)
- Uncomplicated spontaneous vaginal delivery (age 24)

MEDICATIONS:
Home Medications:
- Valacyclovir 500 mg PO twice daily for 3 days during prodromal symptoms (not taken during current pregnancy)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with history of cesarean delivery
- Sister with recurrent HSV-2 infection
- Maternal aunt with osteoporosis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per month). Occupation: High school mathematics teacher.

PHYSICAL EXAMINATION:
She has been at 10 cm dilation for the past hour, and the fetal head is at +1 station. exhibits a tense, tender uterus on palpation. Vaginal bleeding is minimal.
