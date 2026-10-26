# synthetic_hospital patient 2591

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 25 at first encounter, sex M, White, non-Hispanic, Private insurance (parental plan, eligible until age 26).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-05-27 (Tue) | ed | Emergency Medicine |
| 1 | 2020-07-15 | 2025-11-23 (Sun) | inpatient | Neurology |
| 2 | 2021-01-15 | 2026-05-30 (Sat) | icu | Neurology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private insurance (parental plan, eligible until age 26)",
 "occupation": "Graduate student (MBA program)",
 "alcohol_use": "Social use (2-3 drinks per weekend)",
 "family_history": [
  "Father: Hypertension, diagnosed age 45",
  "Mother: Hypothyroidism",
  "No family history of bleeding disorders or congenital ocular diseases"
 ],
 "race_ethnicity": "White, non-Hispanic",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "90 mcg/puff, 2 puffs every 4-6 hours as needed for wheezing",
   "name": "Albuterol HFA inhaler"
  }
 ],
 "surgical_history": [
  "Tonsillectomy (age 8)",
  "Wisdom teeth extraction (age 18)"
 ],
 "chronic_conditions": [
  "Asthma, well-controlled"
 ],
 "age_at_first_encounter": 25
}
```

Primary diagnoses: ["Traumatic optic neuropathy (retrobulbar), right eye", "Auricular hematoma", "Blunt head trauma"]
Comorbidities: ["Blunt head trauma"]

## Encounter 0: source 2020-01-15, practice 2025-05-27, ed, Emergency Medicine

Chief complaint: Left ear pain and swelling after being struck on the side of the head during a rugby match earlier in the day

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Left ear pain and swelling after being struck on the side of the head during a rugby match earlier in the day

HISTORY OF PRESENT ILLNESS:
After being struck on the side of the head during a rugby match earlier in the day. He denies loss of consciousness, nausea, vomiting, or dizziness.

PAST MEDICAL HISTORY:
Active Problem List:
- Asthma, well-controlled

PAST SURGICAL HISTORY:
- Tonsillectomy (age 8)
- Wisdom teeth extraction (age 18)

MEDICATIONS:
Home Medications:
- Albuterol HFA inhaler 90 mcg/puff, 2 puffs every 4-6 hours as needed for wheezing

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, diagnosed age 45
- Mother: Hypothyroidism
- No family history of bleeding disorders or congenital ocular diseases

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social use (2-3 drinks per weekend). Occupation: Graduate student (MBA program).

VITAL SIGNS:
On examination, his vital signs are stable

PHYSICAL EXAMINATION:
There is no evidence of facial fractures or cervical spine tenderness. Otoscopic examination of the left ear reveals an intact tympanic membrane, but the external ear is swollen, tender, and has a fluctuant mass on the antihelical fold. There is no overlying skin laceration or abrasion. The patient&#x27;s hearing is unaffected. The rest of the physical examination is unremarkable.

## Encounter 1: source 2020-07-15, practice 2025-11-23, inpatient, Neurology

Chief complaint: Severe headache and two episodes of vomiting after falling off bicycle at high speed and hitting head on curb despite helmet use

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Rodriguez, MD
Department: Neurology
Visit Type: inpatient

CHIEF COMPLAINT:
Severe headache and two episodes of vomiting after falling off bicycle at high speed and hitting head on curb despite helmet use

HISTORY OF PRESENT ILLNESS:
The patient has well-controlled asthma and prior auricular hematoma, presenting after a bicycle accident. He wore a helmet but hit his head on the curb, with severe headache and two episodes of vomiting since the accident.

PAST MEDICAL HISTORY:
Active Problem List:
- Auricular hematoma (diagnosed 2020-01-15)
- Asthma, well-controlled

His past medical history is significant for well-controlled asthma,.

PAST SURGICAL HISTORY:
- Tonsillectomy (age 8)
- Wisdom teeth extraction (age 18)

MEDICATIONS:
Home Medications:
- Albuterol HFA inhaler 90 mcg/puff, 2 puffs every 4-6 hours as needed for wheezing

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, diagnosed age 45
- Mother: Hypothyroidism
- No family history of bleeding disorders or congenital ocular diseases

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social use (2-3 drinks per weekend). Occupation: Graduate student (MBA program).

PHYSICAL EXAMINATION:
On examination, he is alert and oriented but appears agitated. His Glasgow Coma Scale score is 15. He has a laceration on his scalp, but there is no evidence of cerebrospinal fluid otorrhea or raccoon eyes. His pupils are equal and reactive to light, and there is no neck stiffness. The rest of the physical examination is unremarkable.

ASSESSMENT AND PLAN:
Given the patient's presentation and high-risk features for intracranial injury,

## Encounter 2: source 2021-01-15, practice 2026-05-30, icu, Neurology

Chief complaint: Sudden loss of vision in right eye with severe headache and nausea after motor vehicle collision with blunt trauma to forehead

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Marcus Rodriguez, MD
Department: Neurology
Visit Type: icu

CHIEF COMPLAINT:
Sudden loss of vision in right eye with severe headache and nausea after motor vehicle collision with blunt trauma to forehead

HISTORY OF PRESENT ILLNESS:
The patient is a 26-year-old male with well-controlled asthma and prior head trauma in 2020 including auricular hematoma and blunt head trauma. He presents to the emergency department with sudden loss of vision in his right eye after being involved in a motor vehicle collision where he sustained blunt trauma to forehead, complaining of severe headache and nausea.

PAST MEDICAL HISTORY:
Active Problem List:
- Auricular hematoma (diagnosed 2020-01-15)
- Blunt head trauma (diagnosed 2020-07-15)
- Asthma, well-controlled

PAST SURGICAL HISTORY:
- Tonsillectomy (age 8)
- Wisdom teeth extraction (age 18)

MEDICATIONS:
Home Medications:
- Albuterol HFA inhaler 90 mcg/puff, 2 puffs every 4-6 hours as needed for wheezing

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, diagnosed age 45
- Mother: Hypothyroidism
- No family history of bleeding disorders or congenital ocular diseases

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social use (2-3 drinks per weekend). Occupation: Graduate student (MBA program).

VITAL SIGNS:
- Blood pressure: 130/85 mmHg
- Heart rate: 88 bpm
- Respiratory rate: 16 breaths/min
- Temperature: 98.6°F (37°C)

PHYSICAL EXAMINATION:
On examination, his right pupil is dilated and does not constrict in response to light, but the left pupil reacts normally. Visual acuity is 20/200 in the right eye and 20/20 in the left eye. Fundoscopic examination reveals a normal appearing retina and optic disc in both eyes. No other injuries are noted on physical examination.
