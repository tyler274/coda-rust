# synthetic_hospital patient 1732

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 31 at first encounter, sex F, Non-Hispanic White, Employer-sponsored commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2026-01-19 (Mon) | outpatient | Obstetrics & Gynecology |
| 1 | 2020-02-15 | 2026-02-13 (Fri) | ed | Emergency Medicine |
| 2 | 2020-03-15 | 2026-03-13 (Fri) | outpatient | Obstetrics & Gynecology |
| 3 | 2020-03-15 | 2026-03-13 (Fri) | outpatient | Internal Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Employer-sponsored commercial PPO",
 "occupation": "Elementary school teacher",
 "alcohol_use": "Social (1-2 drinks per month), abstained during pregnancy and breastfeeding",
 "family_history": [
  "Mother: postpartum depression after second pregnancy",
  "Maternal aunt: bipolar disorder type I",
  "Father: type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1 tablet daily",
   "name": "Prenatal vitamin with iron"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [],
 "age_at_first_encounter": 31
}
```

Primary diagnoses: ["Postpartum psychosis", "Supervision of normal first pregnancy, third trimester", "Postpartum depression", "Lactational mastitis, right breast"]
Comorbidities: ["Exclusive breastfeeding", "Primigravida status"]

## Encounter 0: source 2020-01-15, practice 2026-01-19, outpatient, Obstetrics & Gynecology

Chief complaint: Routine prenatal visit in third trimester with decreased fetal movements over the past 24 hours

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine prenatal visit in third trimester with decreased fetal movements over the past 24 hours

HISTORY OF PRESENT ILLNESS:
Her pregnancy has been uncomplicated,

PAST MEDICAL HISTORY:
She has no significant past medical history.

MEDICATIONS:
Home Medications:
- Prenatal vitamin with iron 1 tablet daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: postpartum depression after second pregnancy
- Maternal aunt: bipolar disorder type I
- Father: type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month), abstained during pregnancy and breastfeeding. Occupation: Elementary school teacher.

VITAL SIGNS:
Her vital signs are within normal limits,

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

OTHER STUDIES:
A nonstress test is performed, which shows less than 2 fetal heart rate accelerations over 40 minutes. A subsequent biophysical profile (BPP) is performed, yielding the following scores: fetal breathing movements (2/2), fetal movement (2/2), fetal tone (2/2), amniotic fluid volume (2/2), and nonstress test (0/2), for a total score of 8/10.

## Encounter 1: source 2020-02-15, practice 2026-02-13, ed, Emergency Medicine

Chief complaint: Brought to emergency department by husband 10 days after delivering first child for insomnia, paranoid delusions, and agitation

ENCOUNTER NOTE
Date: 2020-02-15
Provider: Dr. Michael Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Brought to emergency department by husband 10 days after delivering first child for insomnia, paranoid delusions, and agitation

HISTORY OF PRESENT ILLNESS:
The 31-year-old gravida 1, ten days postpartum and seen last month prenatally, is brought by her husband reporting she has not been sleeping and has been talking to herself. She has been expressing fears that someone is trying to harm their baby and has become increasingly agitated.

PAST MEDICAL HISTORY:
Active Problem List:
- Supervision of normal first pregnancy, third trimester (diagnosed 2020-01-15)

She had an uncomplicated pregnancy and vaginal delivery.

MEDICATIONS:
Home Medications:
- Prenatal vitamin with iron 1 tablet daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: postpartum depression after second pregnancy
- Maternal aunt: bipolar disorder type I
- Father: type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month), abstained during pregnancy and breastfeeding. Occupation: Elementary school teacher.

VITAL SIGNS:
Her vital signs are within normal limits.

PHYSICAL EXAMINATION:
On examination, she is disheveled, appears to be responding to internal stimuli, and is not oriented to time or place.

LABORATORY DATA:
Laboratory studies show:

- Hemoglobin: 12.5 g/dL
- White blood cell count: 7,000/mm^3
- Platelets: 250,000/mm^3
- Thyroid-stimulating hormone: 2.5 mIU/L
- Urinalysis: Normal

## Encounter 2: source 2020-03-15, practice 2026-03-13, outpatient, Obstetrics & Gynecology

Chief complaint: Two-day history of right breast pain, swelling, redness, and fever at 3 weeks postpartum

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Sarah Chen, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Two-day history of right breast pain, swelling, redness, and fever at 3 weeks postpartum

HISTORY OF PRESENT ILLNESS:
This 31-year-old postpartum patient returns to the Obstetrics & Gynecology clinic three weeks following spontaneous vaginal delivery of a healthy infant, having recently been evaluated in the emergency department at ten days postpartum for postpartum psychosis. She has been exclusively breastfeeding and notes worsening pain in the upper outer quadrant of the right breast, along with swelling, redness, and chills. The infant recently started biting during nursing, and she has noticed a cracked nipple.

PAST MEDICAL HISTORY:
Active Problem List:
- Supervision of normal first pregnancy, third trimester (diagnosed 2020-01-15)
- Postpartum psychosis (diagnosed 2020-02-15)

MEDICATIONS:
Home Medications:
- Prenatal vitamin with iron 1 tablet daily

ALLERGIES:
She has no allergies

FAMILY HISTORY:
- Mother: postpartum depression after second pregnancy
- Maternal aunt: bipolar disorder type I
- Father: type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month), abstained during pregnancy and breastfeeding. Occupation: Elementary school teacher.

VITAL SIGNS:
Her temperature is 38.7°C (101.7°F), and her heart rate is 96/min

PHYSICAL EXAMINATION:
On examination, the right breast is erythematous, indurated, and tender to palpation. No fluctuance is appreciated, and there is no nipple discharge.

## Encounter 3: source 2020-03-15, practice 2026-03-13, outpatient, Internal Medicine

Chief complaint: Six-week postpartum follow-up visit with persistent low mood, anhedonia, guilt, and fatigue

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Priya Patel, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Six-week postpartum follow-up visit with persistent low mood, anhedonia, guilt, and fatigue

HISTORY OF PRESENT ILLNESS:
The patient returns for her six-week postpartum follow-up visit following an emergency department presentation ten days postpartum for insomnia, paranoid delusions, and agitation, and a recent evaluation for right breast pain, swelling, redness, and fever. She reports feeling "down" and having trouble sleeping, even when her baby is asleep. She has little interest in activities she used to enjoy and feels guilty for not being a "good enough" mother. She also mentions difficulty concentrating and feeling fatigued most of the day. Her appetite has decreased, and she has lost 5 pounds since delivery. She denies any thoughts of harming herself or her baby.

PAST MEDICAL HISTORY:
Active Problem List:
- Supervision of normal first pregnancy, third trimester (diagnosed 2020-01-15)
- Postpartum psychosis (diagnosed 2020-02-15)
- Lactational mastitis, right breast (diagnosed 2020-03-15)

Her pregnancy was uncomplicated.

MEDICATIONS:
Home Medications:
- Prenatal vitamin with iron 1 tablet daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: postpartum depression after second pregnancy
- Maternal aunt: bipolar disorder type I
- Father: type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month), abstained during pregnancy and breastfeeding. Occupation: Elementary school teacher.

VITAL SIGNS:
On examination, her vital signs are within normal limits

PHYSICAL EXAMINATION:
There are no abnormal findings.

LABORATORY DATA:
Her thyroid-stimulating hormone (TSH) level is 2.5 mIU/L (normal range: 0.4-4.0 mIU/L).
