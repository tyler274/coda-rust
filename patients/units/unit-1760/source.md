# synthetic_hospital patient 1760

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex F, Non-Hispanic White, Employer-sponsored PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-10-21 (Thu) | outpatient | Obstetrics & Gynecology |
| 1 | 2020-07-15 | 2022-04-20 (Wed) | outpatient | Reproductive Endocrinology |
| 2 | 2021-01-15 | 2022-10-24 (Mon) | outpatient | Obstetrics & Gynecology |
| 3 | 2021-07-15 | 2023-04-13 (Thu) | outpatient | Obstetrics & Gynecology |
| 4 | 2024-01-15 | 2025-10-19 (Sun) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Employer-sponsored PPO",
 "occupation": "Elementary school teacher",
 "alcohol_use": "Social (1-2 glasses of wine per week)",
 "family_history": [
  "Mother with endometriosis requiring hysterectomy at age 42",
  "Maternal aunt with unexplained infertility",
  "Father with hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "600 mg PO every 6-8 hours PRN for dysmenorrhea and pelvic pain",
   "name": "Ibuprofen"
  },
  {
   "dose": "1 tablet PO daily continuously for endometriosis suppression",
   "name": "Ethinyl estradiol/norethindrone acetate"
  },
  {
   "dose": "1 tablet PO daily",
   "name": "Prenatal vitamin with folic acid"
  }
 ],
 "surgical_history": [
  "Laparoscopic right salpingectomy for ruptured tubal ectopic pregnancy",
  "Diagnostic laparoscopy with lysis of pelvic adhesions and excision of endometriosis implants"
 ],
 "chronic_conditions": [
  "Endometriosis with associated chronic pelvic pain",
  "Tubal factor infertility secondary to recurrent pelvic inflammatory disease",
  "History of recurrent Chlamydia trachomatis infection"
 ],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Tubal factor infertility", "Ruptured tubal ectopic pregnancy", "Endometriosis"]
Comorbidities: ["Female infertility", "Pelvic inflammatory disease (suspected)", "History of intravenous drug use"]

## Encounter 0: source 2020-01-15, practice 2021-10-21, outpatient, Obstetrics & Gynecology

Chief complaint: Evaluation of primary infertility

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Evaluation of primary infertility

HISTORY OF PRESENT ILLNESS:
She and her husband have been trying to conceive for over one year. Her menstrual cycles are regular, and she has no history of abnormal uterine bleeding, dysmenorrhea, or pelvic pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Endometriosis with associated chronic pelvic pain
- Tubal factor infertility secondary to recurrent pelvic inflammatory disease
- History of recurrent Chlamydia trachomatis infection

PAST SURGICAL HISTORY:
She has no significant past surgical history.

MEDICATIONS:
Home Medications:
- Ibuprofen 600 mg PO every 6-8 hours PRN for dysmenorrhea and pelvic pain
- Ethinyl estradiol/norethindrone acetate 1 tablet PO daily continuously for endometriosis suppression
- Prenatal vitamin with folic acid 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with endometriosis requiring hysterectomy at age 42
- Maternal aunt with unexplained infertility
- Father with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per week). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

IMAGING:
A hysterosalpingogram is shown.

## Encounter 1: source 2020-07-15, practice 2022-04-20, outpatient, Reproductive Endocrinology

Chief complaint: Inability to conceive after 14 months of regular, unprotected intercourse

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. James Park, MD
Department: Reproductive Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Inability to conceive after 14 months of regular, unprotected intercourse

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of infertility with endometriosis, initially assessed in January 2020. She reports a regular menstrual cycle with a 28-day interval. Her husband's semen analysis is within normal limits.

PAST MEDICAL HISTORY:
Active Problem List:
- Tubal factor infertility (diagnosed 2020-01-15)
- Endometriosis with associated chronic pelvic pain
- Tubal factor infertility secondary to recurrent pelvic inflammatory disease
- History of recurrent Chlamydia trachomatis infection

Her past medical history is significant for a severe pelvic inflammatory disease (PID) two years ago, which required hospitalization and intravenous antibiotics.

PAST SURGICAL HISTORY:
- Laparoscopic right salpingectomy for ruptured tubal ectopic pregnancy
- Diagnostic laparoscopy with lysis of pelvic adhesions and excision of endometriosis implants

MEDICATIONS:
Home Medications:
- Ibuprofen 600 mg PO every 6-8 hours PRN for dysmenorrhea and pelvic pain
- Ethinyl estradiol/norethindrone acetate 1 tablet PO daily continuously for endometriosis suppression
- Prenatal vitamin with folic acid 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with endometriosis requiring hysterectomy at age 42
- Maternal aunt with unexplained infertility
- Father with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per week). Occupation: Elementary school teacher.

VITAL SIGNS:
Her vital signs are stable

PHYSICAL EXAMINATION:
Her pelvic examination is unremarkable

IMAGING:
A hysterosalpingogram is performed, which shows bilateral tubal occlusion.

## Encounter 2: source 2021-01-15, practice 2022-10-24, outpatient, Obstetrics & Gynecology

Chief complaint: Chronic pelvic pain

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Sarah Chen, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Chronic pelvic pain

HISTORY OF PRESENT ILLNESS:
The patient returns to the gynecology clinic for management of chronic pelvic pain in the context of known endometriosis and prior evaluation for tubal factor infertility. Following her infertility workup in 2020, she now presents with a 6-month history of chronic pelvic pain that worsens during her menstrual periods. She also reports deep dyspareunia and non-bloody diarrhea that seems to correlate with her menstrual cycle. She has been using ibuprofen and oral contraceptive pills for the last three months with minimal relief of her symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Tubal factor infertility (diagnosed 2020-01-15)
- Endometriosis with associated chronic pelvic pain
- Tubal factor infertility secondary to recurrent pelvic inflammatory disease
- History of recurrent Chlamydia trachomatis infection

Her medical history is significant for infertility, having unsuccessfully attempted to conceive for the past year.

PAST SURGICAL HISTORY:
- Laparoscopic right salpingectomy for ruptured tubal ectopic pregnancy
- Diagnostic laparoscopy with lysis of pelvic adhesions and excision of endometriosis implants

MEDICATIONS:
Home Medications:
- Ibuprofen 600 mg PO every 6-8 hours PRN for dysmenorrhea and pelvic pain
- Ethinyl estradiol/norethindrone acetate 1 tablet PO daily continuously for endometriosis suppression
- Prenatal vitamin with folic acid 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with endometriosis requiring hysterectomy at age 42
- Maternal aunt with unexplained infertility
- Father with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per week). Occupation: Elementary school teacher.

VITAL SIGNS:
Her vital signs are within normal limits

PHYSICAL EXAMINATION:
On physical examination, there is tenderness in the posterior cul-de-sac, and the uterus is retroverted.

LABORATORY DATA:
Her complete blood count and inflammatory markers are normal.

IMAGING:
A transvaginal ultrasound reveals no abnormalities.

## Encounter 3: source 2021-07-15, practice 2023-04-13, outpatient, Obstetrics & Gynecology

Chief complaint: Worsening dysmenorrhea and dyspareunia

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Maria Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening dysmenorrhea and dyspareunia

HISTORY OF PRESENT ILLNESS:
The patient with endometriosis and tubal factor infertility returns for follow-up, having previously been evaluated for primary infertility and chronic pelvic pain. She reports the pain has progressively worsened and now affects her quality of life. She and her husband have been trying to conceive for the past 18 months without success. Her menstrual cycles are regular, with a 28-day cycle.

PAST MEDICAL HISTORY:
Active Problem List:
- Tubal factor infertility (diagnosed 2020-01-15)
- Endometriosis (diagnosed 2021-01-15)
- Endometriosis with associated chronic pelvic pain
- Tubal factor infertility secondary to recurrent pelvic inflammatory disease
- History of recurrent Chlamydia trachomatis infection

PAST SURGICAL HISTORY:
- Laparoscopic right salpingectomy for ruptured tubal ectopic pregnancy
- Diagnostic laparoscopy with lysis of pelvic adhesions and excision of endometriosis implants

MEDICATIONS:
Home Medications:
- Ibuprofen 600 mg PO every 6-8 hours PRN for dysmenorrhea and pelvic pain
- Ethinyl estradiol/norethindrone acetate 1 tablet PO daily continuously for endometriosis suppression
- Prenatal vitamin with folic acid 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with endometriosis requiring hysterectomy at age 42
- Maternal aunt with unexplained infertility
- Father with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per week). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
On physical examination, there is tenderness noted in the posterior cul-de-sac, and the uterus appears retroverted.

LABORATORY DATA:
The patient's laboratory values are as follows:

- Hemoglobin: 13.2 g/dL
- White blood cell count: 7,500/mm^3
- Erythrocyte sedimentation rate (ESR): 20 mm/hr
- CA-125: Elevated

IMAGING:
A pelvic ultrasound reveals several small cystic structures in the ovaries, described as "chocolate cysts."

## Encounter 4: source 2024-01-15, practice 2025-10-19, ed, Emergency Medicine

Chief complaint: Sudden onset severe right lower quadrant abdominal pain

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset severe right lower quadrant abdominal pain

HISTORY OF PRESENT ILLNESS:
The patient with endometriosis, chronic pelvic pain, and tubal factor infertility presents acutely. She also reports mild intermittent vaginal spotting over the past two days. Her last menstrual period was 6 weeks ago, and she has a regular 28-day cycle.

PAST MEDICAL HISTORY:
Active Problem List:
- Tubal factor infertility (diagnosed 2020-01-15)
- Endometriosis (diagnosed 2021-01-15)
- Endometriosis with associated chronic pelvic pain
- Tubal factor infertility secondary to recurrent pelvic inflammatory disease
- History of recurrent Chlamydia trachomatis infection

Her past medical history is significant for chlamydia infection treated 1 year ago.

PAST SURGICAL HISTORY:
- Laparoscopic right salpingectomy for ruptured tubal ectopic pregnancy
- Diagnostic laparoscopy with lysis of pelvic adhesions and excision of endometriosis implants

MEDICATIONS:
Home Medications:
- Ibuprofen 600 mg PO every 6-8 hours PRN for dysmenorrhea and pelvic pain
- Ethinyl estradiol/norethindrone acetate 1 tablet PO daily continuously for endometriosis suppression
- Prenatal vitamin with folic acid 1 tablet PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with endometriosis requiring hysterectomy at age 42
- Maternal aunt with unexplained infertility
- Father with hypertension

SOCIAL HISTORY:
She is sexually active with one partner and uses condoms inconsistently.

VITAL SIGNS:
On examination, her blood pressure is 110/70 mmHg, pulse is 98 bpm, and temperature is 37.2°C (99.0°F).

PHYSICAL EXAMINATION:
Abdominal examination reveals tenderness in the right lower quadrant with rebound tenderness. A pelvic exam shows a closed cervical os with no active bleeding, but there is tenderness during movement of the cervix. No adnexal masses are palpable.

LABORATORY DATA:
Urine pregnancy test is positive.

IMAGING:
Transvaginal ultrasound shows an empty uterine cavity and a 3 cm complex adnexal mass on the right side with no blood flow on Doppler.
