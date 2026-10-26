# synthetic_hospital patient 2763

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 40 at first encounter, sex F, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-06-03 (Mon) | outpatient | Gastroenterology |
| 1 | 2021-07-15 | 2025-11-24 (Mon) | outpatient | Obstetrics & Gynecology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Financial analyst",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Mother: Colorectal cancer diagnosed at age 45",
  "Maternal aunt: Endometrial cancer diagnosed at age 50"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "325 mg daily",
   "name": "Ferrous sulfate"
  },
  {
   "dose": "200 mg as needed for dysmenorrhea",
   "name": "Ibuprofen"
  }
 ],
 "surgical_history": [
  "Vaginal delivery x2 (last complicated by prolonged second stage of labor)"
 ],
 "chronic_conditions": [
  "Irregular menstrual cycles"
 ],
 "age_at_first_encounter": 40
}
```

Primary diagnoses: ["MLH1-deficient colorectal cancer (Lynch Syndrome)", "Stress urinary incontinence"]
Comorbidities: ["Sexually active with inconsistent contraceptive use"]

## Encounter 0: source 2020-01-15, practice 2024-06-03, outpatient, Gastroenterology

Chief complaint: Concerned about her risk of cancer because her mother was diagnosed with colorectal cancer at age 45 and her maternal aunt with endometrial cancer at age 50. She reports irregular menstrual cycles and occasional intermenstrual bleeding for the past 6 months.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Concerned about her risk of cancer because her mother was diagnosed with colorectal cancer at age 45 and her maternal aunt with endometrial cancer at age 50. She reports irregular menstrual cycles and occasional intermenstrual bleeding for the past 6 months.

HISTORY OF PRESENT ILLNESS:
She is concerned about her risk of cancer because her mother was diagnosed with colorectal cancer at age 45 and her maternal aunt with endometrial cancer at age 50. She has no personal history of cancer but reports irregular menstrual cycles and occasional intermenstrual bleeding for the past 6 months. She has no children and expresses a desire to maintain fertility.

PAST MEDICAL HISTORY:
Active Problem List:
- Irregular menstrual cycles

PAST SURGICAL HISTORY:
- Vaginal delivery x2 (last complicated by prolonged second stage of labor)

MEDICATIONS:
Home Medications:
- Ferrous sulfate 325 mg daily
- Ibuprofen 200 mg as needed for dysmenorrhea

ALLERGIES:
NKDA

FAMILY HISTORY:
Her mother was diagnosed with colorectal cancer at age 45 and her maternal aunt with endometrial cancer at age 50

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Financial analyst.

PHYSICAL EXAMINATION:
Her physical examination is unremarkable, and her pelvic exam does not reveal any abnormalities. Her body mass index is 26 kg/m^2.

LABORATORY DATA:
- Complete blood count: Normal
- CA-125: Within normal limits
- Endometrial biopsy: Simple hyperplasia without atypia

## Encounter 1: source 2021-07-15, practice 2025-11-24, outpatient, Obstetrics & Gynecology

Chief complaint: Urinary incontinence for the past six months. She leaks urine when she coughs, laughs, or exercises, especially during weekly jogging sessions, and has not improved with home pelvic floor exercises.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Maria Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Urinary incontinence for the past six months. She leaks urine when she coughs, laughs, or exercises, especially during weekly jogging sessions, and has not improved with home pelvic floor exercises.

HISTORY OF PRESENT ILLNESS:
The patient with MLH1-deficient colorectal cancer (Lynch Syndrome) and irregular menstrual cycles presents to Gynecology with urinary complaints. She reports urine leakage with cough, laugh, or exercise, especially during weekly jogging sessions. She has tried pelvic floor exercises at home with no significant improvement.

PAST MEDICAL HISTORY:
Active Problem List:
- MLH1-deficient colorectal cancer (Lynch Syndrome) (diagnosed 2020-01-15)
- Irregular menstrual cycles

Her past medical history is significant for two vaginal deliveries. Her last delivery was complicated by a prolonged second stage of labor.

PAST SURGICAL HISTORY:
- Vaginal delivery x2 (last complicated by prolonged second stage of labor)

MEDICATIONS:
Home Medications:
- Ferrous sulfate 325 mg daily
- Ibuprofen 200 mg as needed for dysmenorrhea

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Colorectal cancer diagnosed at age 45
- Maternal aunt: Endometrial cancer diagnosed at age 50

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Financial analyst.

REVIEW OF SYSTEMS:
She denies dysuria, urgency, or nocturia.

PHYSICAL EXAMINATION:
On physical examination, there is no suprapubic tenderness, and the pelvic exam reveals urethral hypermobility with a positive cough stress test.

LABORATORY DATA:
Her urinalysis is normal.
