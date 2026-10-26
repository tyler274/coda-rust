# synthetic_hospital patient 2251

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 67 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-11-28 (Thu) | ed | Emergency Medicine |
| 1 | 2020-02-15 | 2025-01-04 (Sat) | inpatient | Urology |
| 2 | 2021-01-15 | 2025-12-12 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired High School Mathematics Teacher",
 "alcohol_use": "Occasional social use (1-2 drinks per month)",
 "family_history": [
  "Father with hypertension and coronary artery disease",
  "Mother with diverticular disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 12 years ago (15 pack-year history)",
 "home_medications": [
  {
   "dose": "5 mg oral daily",
   "name": "Amlodipine"
  },
  {
   "dose": "10 mg oral daily",
   "name": "Lisinopril"
  },
  {
   "dose": "0.4 mg oral daily",
   "name": "Tamsulosin"
  }
 ],
 "surgical_history": [
  "Cystoscopy with bladder stone removal",
  "Abdominal surgery for intestinal obstruction"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Benign Prostatic Hyperplasia",
  "Diverticulosis"
 ],
 "age_at_first_encounter": 67
}
```

Primary diagnoses: ["Infective Endocarditis (Aortic Valve) with Complications", "Intestinal obstruction requiring cross-sectional imaging", "Urethral injury"]
Comorbidities: ["Essential hypertension", "Status post laparoscopic cholecystectomy", "Urinary retention"]

## Encounter 0: source 2020-01-15, practice 2024-11-28, ed, Emergency Medicine

Chief complaint: severe, crampy abdominal pain, vomiting, and inability to pass stool or gas

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe, crampy abdominal pain, vomiting, and inability to pass stool or gas

HISTORY OF PRESENT ILLNESS:
He presents to the emergency department with a 2-day history of severe, crampy abdominal pain, vomiting, and inability to pass stool or gas. He denies any fever or chills.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Benign Prostatic Hyperplasia
- Diverticulosis

PAST SURGICAL HISTORY:
Remote history of an open cholecystectomy

MEDICATIONS:
Home Medications:
- Amlodipine 5 mg oral daily
- Lisinopril 10 mg oral daily
- Tamsulosin 0.4 mg oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with hypertension and coronary artery disease
- Mother with diverticular disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 12 years ago (15 pack-year history). Alcohol: Occasional social use (1-2 drinks per month). Occupation: Retired High School Mathematics Teacher.

VITAL SIGNS:
His vital signs are as follows: BP 150/90 mmHg, HR 110 bpm, RR 22 breaths/min, and temperature 37.2°C.

PHYSICAL EXAMINATION:
On examination, his abdomen is distended, tympanic to percussion, and diffusely tender without rebound or guarding. Bowel sounds are high-pitched and hyperactive. Rectal examination reveals an empty rectal vault.

LABORATORY DATA:
Laboratory tests show a leukocytosis of 14,000/uL.

IMAGING:
An upright abdominal x-ray reveals dilated loops of small bowel with multiple air-fluid levels and no air in the rectum.

## Encounter 1: source 2020-02-15, practice 2025-01-04, inpatient, Urology

Chief complaint: Hospitalized for management of postoperative urinary retention following abdominal surgery

ENCOUNTER NOTE
Date: 2020-02-15
Provider: Dr. Michael Rodriguez, MD
Department: Urology
Visit Type: inpatient

CHIEF COMPLAINT:
Hospitalized for management of postoperative urinary retention following abdominal surgery

HISTORY OF PRESENT ILLNESS:
This 67-year-old male with benign prostatic hyperplasia is hospitalized status post recent abdominal surgery after a prior admission for intestinal obstruction. A nurse attempts to insert a Foley catheter but encounters mild resistance after passing several centimeters. The patient suddenly reports perineal discomfort, and a small amount of blood appears at the urethral meatus. The catheter is immediately withdrawn, and urology is consulted.

PAST MEDICAL HISTORY:
Active Problem List:
- Intestinal obstruction requiring cross-sectional imaging (diagnosed 2020-01-15)
- Hypertension
- Benign Prostatic Hyperplasia
- Diverticulosis

PAST SURGICAL HISTORY:
- Cystoscopy with bladder stone removal
- Abdominal surgery for intestinal obstruction

MEDICATIONS:
Home Medications:
- Amlodipine 5 mg oral daily
- Lisinopril 10 mg oral daily
- Tamsulosin 0.4 mg oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with hypertension and coronary artery disease
- Mother with diverticular disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 12 years ago (15 pack-year history). Alcohol: Occasional social use (1-2 drinks per month). Occupation: Retired High School Mathematics Teacher.

PHYSICAL EXAMINATION:
Physical examination reveals a distended bladder and mild perineal tenderness without visible swelling or scrotal hematoma.

PLAN:
Retrograde urethrogram is ordered to assess for injury.

## Encounter 2: source 2021-01-15, practice 2025-12-12, ed, Emergency Medicine

Chief complaint: 6 days of intermittent fevers, malaise, and decreased appetite

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Amara Okafor, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
6 days of intermittent fevers, malaise, and decreased appetite

HISTORY OF PRESENT ILLNESS:
The patient is a 68-year-old male with a history of hypertension, benign prostatic hyperplasia, and diverticulosis, previously hospitalized for intestinal obstruction and urethral injury, who presents with six days of intermittent fevers, malaise, and decreased appetite. He underwent cystoscopy and bladder stone removal 2 weeks ago due to chronic urinary retention from benign prostatic hyperplasia.

PAST MEDICAL HISTORY:
Active Problem List:
- Intestinal obstruction requiring cross-sectional imaging (diagnosed 2020-01-15)
- Urethral injury (diagnosed 2020-02-15)
- Hypertension
- Benign Prostatic Hyperplasia
- Diverticulosis

His past medical history includes hypertension and diverticulosis. He has no prosthetic valves or indwelling lines.

PAST SURGICAL HISTORY:
- Cystoscopy with bladder stone removal
- Abdominal surgery for intestinal obstruction

MEDICATIONS:
Home Medications:
- Amlodipine 5 mg oral daily
- Lisinopril 10 mg oral daily
- Tamsulosin 0.4 mg oral daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with hypertension and coronary artery disease
- Mother with diverticular disease

SOCIAL HISTORY:
He does not use any IV drugs

VITAL SIGNS:
Temperature of 38.3°C (101°F)

PHYSICAL EXAMINATION:
On physical exam, he is febrile. Cardiac auscultation reveals a new holosystolic murmur best heard at the lower left sternal border. There are no Janeway lesions or Osler nodes.

LABORATORY DATA:
Three separate sets of blood cultures all grow Gram-positive cocci in pairs and chains. The microbiology report identifies the organism as Enterococcus species.

IMAGING:
Transthoracic echocardiogram reveals a mobile mass on the tricuspid valve.
