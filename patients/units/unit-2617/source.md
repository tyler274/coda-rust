# synthetic_hospital patient 2617

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 28 at first encounter, sex M, Caucasian, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-10-07 (Tue) | follow_up | Physical Medicine & Rehabilitation |
| 1 | 2020-06-15 | 2026-02-26 (Thu) | outpatient | Ophthalmology |
| 2 | 2020-09-15 | 2026-06-08 (Mon) | outpatient | Gastroenterology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Construction worker",
 "alcohol_use": "Denies alcohol use",
 "family_history": [
  "Mother with hypothyroidism",
  "Father with hypertension",
  "No family history of autoimmune disease or liver disease"
 ],
 "race_ethnicity": "Caucasian",
 "smoking_status": "Former smoker, quit 2 years ago",
 "home_medications": [],
 "surgical_history": [
  "Primary repair of penetrating globe injury, right eye (3 months ago)",
  "Open reduction internal fixation of left femur and pelvic fractures (6 weeks ago)",
  "Dental extractions and root canal therapy performed in Thailand (2 months ago)"
 ],
 "chronic_conditions": [],
 "age_at_first_encounter": 28
}
```

Primary diagnoses: ["Disuse atrophy", "Sympathetic ophthalmia", "Acute hepatitis B virus infection"]
Comorbidities: ["Status post laparoscopic cholecystectomy"]

## Encounter 0: source 2020-01-15, practice 2025-10-07, follow_up, Physical Medicine & Rehabilitation

Chief complaint: Persistent weakness in both lower limbs following surgical stabilization of traumatic fractures

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. James Park, MD
Department: Physical Medicine & Rehabilitation
Visit Type: follow_up

CHIEF COMPLAINT:
Persistent weakness in both lower limbs following surgical stabilization of traumatic fractures

HISTORY OF PRESENT ILLNESS:
He is brought to a rehabilitation facility after being discharged from the hospital, where he had undergone surgical stabilization for a traumatic injury several weeks ago. He initially presented after a fall from a ladder, which resulted in multiple fractures and prolonged immobilization. After one month of limited mobility, a follow-up evaluation is requested to assess the current state of the affected muscles.

PAST MEDICAL HISTORY:
He has no prior history of neuromuscular disorders or systemic autoimmune disease.

PAST SURGICAL HISTORY:
- Primary repair of penetrating globe injury, right eye (3 months ago)
- Open reduction internal fixation of left femur and pelvic fractures (6 weeks ago)
- Dental extractions and root canal therapy performed in Thailand (2 months ago)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with hypothyroidism
- Father with hypertension
- No family history of autoimmune disease or liver disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 2 years ago. Alcohol: Denies alcohol use. Occupation: Construction worker.

PHYSICAL EXAMINATION:
On physical exam, he is alert and cooperative, with stable vital signs and no signs of systemic infection. Neurologic evaluation shows minimal voluntary movement in both legs with decreased tone. Deep tendon reflexes in the lower extremities are difficult to elicit. His upper extremity strength is preserved, and sensation is intact in the upper body. There is no evidence of acute thrombosis or signs of compartment syndrome.

## Encounter 1: source 2020-06-15, practice 2026-02-26, outpatient, Ophthalmology

Chief complaint: Progressive vision loss, eye pain, and photophobia in left eye

ENCOUNTER NOTE
Date: 2020-06-15
Provider: Dr. Aisha Patel, MD
Department: Ophthalmology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive vision loss, eye pain, and photophobia in left eye

HISTORY OF PRESENT ILLNESS:
The patient is a 28-year-old male with a history of persistent lower limb weakness following surgical stabilization of traumatic fractures previously evaluated by Physical Medicine and Rehabilitation. He now presents to the ophthalmology clinic with a 2-week history of progressive vision loss, eye pain, and photophobia in his left eye. He reports a penetrating injury to his right eye 3 months ago, which required surgical repair. His right eye has since recovered well, but he is now concerned about his left eye.

PAST MEDICAL HISTORY:
Active Problem List:
- Disuse atrophy (diagnosed 2020-01-15)

PAST SURGICAL HISTORY:
- Primary repair of penetrating globe injury, right eye (3 months ago)
- Open reduction internal fixation of left femur and pelvic fractures (6 weeks ago)
- Dental extractions and root canal therapy performed in Thailand (2 months ago)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with hypothyroidism
- Father with hypertension
- No family history of autoimmune disease or liver disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 2 years ago. Alcohol: Denies alcohol use. Occupation: Construction worker.

PHYSICAL EXAMINATION:
On examination, his visual acuity is 20/40 in the right eye and 20/200 in the left eye. Slit-lamp examination of the left eye reveals anterior chamber cells and flare, keratic precipitates, and posterior synechiae. Fundoscopy shows diffuse choroidal thickening and multifocal serous retinal detachments. There is no evidence of infection, and intraocular pressures are within normal limits.

LABORATORY DATA:
Laboratory Values:
- ESR: 25 mm/hr (Normal: 0-22 mm/hr)
- CRP: 10 mg/L (Normal: <5 mg/L)
- ANA, RF, anti-DS DNA: Negative

IMAGING:
Chest X-ray: Normal

## Encounter 2: source 2020-09-15, practice 2026-06-08, outpatient, Gastroenterology

Chief complaint: One-week history of fatigue, low-grade fever, nausea, and dark-colored urine

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Robert Kim, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
One-week history of fatigue, low-grade fever, nausea, and dark-colored urine

HISTORY OF PRESENT ILLNESS:
He has sympathetic ophthalmia and disuse atrophy and reports fatigue, low-grade fever, nausea, and dark-colored urine. He recently returned from Southeast Asia after dental infection treatment. He denies alcohol use, acetaminophen overdose, or prior liver disease.

PAST MEDICAL HISTORY:
Active Problem List:
- Disuse atrophy (diagnosed 2020-01-15)
- Sympathetic ophthalmia (diagnosed 2020-06-15)

PAST SURGICAL HISTORY:
- Primary repair of penetrating globe injury, right eye (3 months ago)
- Open reduction internal fixation of left femur and pelvic fractures (6 weeks ago)
- Dental extractions and root canal therapy performed in Thailand (2 months ago)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with hypothyroidism
- Father with hypertension
- No family history of autoimmune disease or liver disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 2 years ago. Alcohol: Denies alcohol use. Occupation: Construction worker.

PHYSICAL EXAMINATION:
Physical examination reveals mild right upper quadrant tenderness.

LABORATORY DATA:
Laboratory studies show elevated AST and ALT in the thousands, mildly elevated total bilirubin, and positive HBsAg.

PATHOLOGY:
A liver biopsy shows hepatocyte ballooning degeneration and Councilman bodies.
