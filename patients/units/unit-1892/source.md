# synthetic_hospital patient 1892

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 27 at first encounter, sex F, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-12-22 (Mon) | outpatient | Obstetrics & Gynecology |
| 1 | 2020-07-15 | 2026-06-05 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicaid",
 "occupation": "Retail sales associate (currently on leave)",
 "alcohol_use": "History of heavy alcohol use with documented use during current pregnancy; patient denies current use in clinical encounters",
 "family_history": [
  "Father: Ulcerative colitis",
  "Maternal grandfather: Colon cancer (diagnosed age 62)",
  "Brother: Opioid use disorder"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 6 months prior to conception; no current tobacco use",
 "home_medications": [
  {
   "dose": "800 mg PO three times daily",
   "name": "Mesalamine delayed-release"
  },
  {
   "dose": "1 tablet PO daily",
   "name": "Prenatal vitamin with 1 mg folic acid"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Ulcerative colitis (limited to proctitis on recent colonoscopy)",
  "Opioid use disorder (heroin, severe, with multiple relapses)",
  "Alcohol use disorder",
  "Cannabis use disorder",
  "Pregnancy (G1P0)"
 ],
 "age_at_first_encounter": 27
}
```

Primary diagnoses: ["Ulcerative colitis, acute exacerbation with rectal bleeding", "History of alcohol and cannabis use in pregnancy"]
Comorbidities: ["Pregnancy (16 weeks gestation)"]

## Encounter 0: source 2020-01-15, practice 2025-12-22, outpatient, Obstetrics & Gynecology

Chief complaint: Concerns about her opioid use during pregnancy; patient seeks safest treatment options to quit heroin for the health of her unborn child

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Sharma, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Concerns about her opioid use during pregnancy; patient seeks safest treatment options to quit heroin for the health of her unborn child

HISTORY OF PRESENT ILLNESS:
She has a 5-year history of heroin use and has been in and out of rehabilitation programs. She reports a recent relapse and expresses a strong desire to quit for the health of her unborn child. She denies any other substance use. She inquires about the safest treatment option for her and her baby during the pregnancy.

PAST MEDICAL HISTORY:
Active Problem List:
- Ulcerative colitis (limited to proctitis on recent colonoscopy)
- Opioid use disorder (heroin, severe, with multiple relapses)
- Alcohol use disorder
- Cannabis use disorder
- Pregnancy (G1P0)

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Mesalamine delayed-release 800 mg PO three times daily
- Prenatal vitamin with 1 mg folic acid 1 tablet PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Ulcerative colitis
- Maternal grandfather: Colon cancer (diagnosed age 62)
- Brother: Opioid use disorder

SOCIAL HISTORY:
Smoking: Former smoker, quit 6 months prior to conception; no current tobacco use. Alcohol: History of heavy alcohol use with documented use during current pregnancy; patient denies current use in clinical encounters. Occupation: Retail sales associate (currently on leave).

VITAL SIGNS:
Her vital signs are stable

PHYSICAL EXAMINATION:
Physical examination is unremarkable except for track marks on her forearms

LABORATORY DATA:
Urine toxicology confirms the presence of opioids

## Encounter 1: source 2020-07-15, practice 2026-06-05, ed, Emergency Medicine

Chief complaint: 1-week history of bloody diarrhea and abdominal pain

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
1-week history of bloody diarrhea and abdominal pain

PAST MEDICAL HISTORY:
Active Problem List:
- History of alcohol and cannabis use in pregnancy (diagnosed 2020-01-15)
- Ulcerative colitis (limited to proctitis on recent colonoscopy)
- Opioid use disorder (heroin, severe, with multiple relapses)
- Alcohol use disorder
- Cannabis use disorder
- Pregnancy (G1P0)

She has a known history of ulcerative colitis (UC) but has been in remission for the past year.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
She has been compliant with her mesalamine therapy.

Home Medications:
- Mesalamine delayed-release 800 mg PO three times daily
- Prenatal vitamin with 1 mg folic acid 1 tablet PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Ulcerative colitis
- Maternal grandfather: Colon cancer (diagnosed age 62)
- Brother: Opioid use disorder

SOCIAL HISTORY:
Smoking: Former smoker, quit 6 months prior to conception; no current tobacco use. Alcohol: History of heavy alcohol use with documented use during current pregnancy; patient denies current use in clinical encounters. Occupation: Retail sales associate (currently on leave).

VITAL SIGNS:
Her vital signs are stable

PHYSICAL EXAMINATION:
She has mild tenderness on palpation of the lower abdomen without rebound or guarding

LABORATORY DATA:
Laboratory tests reveal:

- Hemoglobin: 10.8 g/dL (Normal: 12-16 g/dL)
- White blood cell count: 12,000 /µL (Normal: 4,500-11,000 /µL)
- C-reactive protein: 25 mg/L (Normal: <5 mg/L)
- Stool culture: No pathogens isolated
