# synthetic_hospital patient 2571

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 24 at first encounter, sex M, Non-Hispanic White, Private insurance (employer-sponsored).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 1 | 2020-07-15 | 2026-02-07 (Sat) | ed | Emergency Medicine |
| 2 | 2020-07-15 | 2026-02-15 (Sun) | icu | Neurosurgery |
| 3 | 2020-07-15 | 2026-02-15 (Sun) | icu | Neurosurgery |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private insurance (employer-sponsored)",
 "occupation": "Construction laborer",
 "alcohol_use": "Social alcohol use (2-3 beers on weekends)",
 "family_history": [
  "Father: Hypertension, hyperlipidemia",
  "Mother: Major depressive disorder",
  "Older brother: Attention-deficit/hyperactivity disorder"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker; quit 18 months ago (1/2 pack per day history)",
 "home_medications": [
  {
   "dose": "20 mg PO every morning",
   "name": "Methylphenidate extended-release"
  },
  {
   "dose": "10 mg PO daily as needed for allergic rhinitis",
   "name": "Cetirizine"
  }
 ],
 "surgical_history": [
  "Tonsillectomy and adenoidectomy (age 8)",
  "Open reduction internal fixation of right distal radius fracture (age 16)"
 ],
 "chronic_conditions": [
  "Attention-deficit/hyperactivity disorder (ADHD), combined type",
  "Seasonal allergic rhinitis"
 ],
 "age_at_first_encounter": 24
}
```

Primary diagnoses: ["Diffuse axonal injury", "Paroxysmal sympathetic hyperactivity", "Traumatic cerebral edema with elevated intracranial pressure"]
Comorbidities: ["Coma", "Dependence on mechanical ventilation"]

## Encounter 1: source 2020-07-15, practice 2026-02-07, ed, Emergency Medicine

Chief complaint: Brought to the emergency department by ambulance following high-speed motor vehicle collision

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Brought to the emergency department by ambulance following high-speed motor vehicle collision

HISTORY OF PRESENT ILLNESS:
This 24-year-old male with attention-deficit/hyperactivity disorder and seasonal allergic rhinitis was the unrestrained driver in an MVC, reportedly striking a tree head-on. On arrival, he is intubated for airway protection and has a Glasgow Coma Scale score of 4 (E1, V1, M2).

PAST MEDICAL HISTORY:
Active Problem List:
- Attention-deficit/hyperactivity disorder (ADHD), combined type
- Seasonal allergic rhinitis

PAST SURGICAL HISTORY:
- Tonsillectomy and adenoidectomy (age 8)
- Open reduction internal fixation of right distal radius fracture (age 16)

MEDICATIONS:
Home Medications:
- Methylphenidate extended-release 20 mg PO every morning
- Cetirizine 10 mg PO daily as needed for allergic rhinitis

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, hyperlipidemia
- Mother: Major depressive disorder
- Older brother: Attention-deficit/hyperactivity disorder

SOCIAL HISTORY:
Smoking: Former smoker; quit 18 months ago (1/2 pack per day history). Alcohol: Social alcohol use (2-3 beers on weekends). Occupation: Construction laborer.

VITAL SIGNS:
His blood pressure is 122/78 mmHg, heart rate 86/min, and oxygen saturation 98% on mechanical ventilation.

PHYSICAL EXAMINATION:
There is no evidence of external head trauma or skull fracture. Pupils are equal and reactive, and there is no lateralizing neurologic deficit.

LABORATORY DATA:
Initial labs are unremarkable, and toxicology screen is negative.

IMAGING:
A non-contrast CT scan of the head is obtained and is shown below.
MRI is pending.

OTHER STUDIES:
EEG shows preserved background activity but no signs of seizure.

ASSESSMENT AND PLAN:
The trauma team notes no signs of intracranial hemorrhage or expanding mass.
Over the next 48 hours, the patient remains comatose despite normalization of vitals, normal intracranial pressure, and absence of sedation.

Neurosurgery is consulted for evaluation.

## Encounter 2: source 2020-07-15, practice 2026-02-15, icu, Neurosurgery

Chief complaint: Severe traumatic brain injury with cerebral edema and elevated intracranial pressure

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Priya Patel, MD
Department: Neurosurgery
Visit Type: icu

CHIEF COMPLAINT:
Severe traumatic brain injury with cerebral edema and elevated intracranial pressure

HISTORY OF PRESENT ILLNESS:
This 24-year-old male with attention-deficit/hyperactivity disorder and seasonal allergic rhinitis was initially evaluated in the emergency department earlier today and is transferred to the intensive care unit. He was the unrestrained driver, and he struck a tree at high speed. On arrival, he is intubated and sedated.

PAST MEDICAL HISTORY:
Active Problem List:
- Diffuse axonal injury (diagnosed 2020-07-15)
- Attention-deficit/hyperactivity disorder (ADHD), combined type
- Seasonal allergic rhinitis

PAST SURGICAL HISTORY:
- Tonsillectomy and adenoidectomy (age 8)
- Open reduction internal fixation of right distal radius fracture (age 16)

MEDICATIONS:
Home Medications:
- Methylphenidate extended-release 20 mg PO every morning
- Cetirizine 10 mg PO daily as needed for allergic rhinitis

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, hyperlipidemia
- Mother: Major depressive disorder
- Older brother: Attention-deficit/hyperactivity disorder

SOCIAL HISTORY:
Smoking: Former smoker; quit 18 months ago (1/2 pack per day history). Alcohol: Social alcohol use (2-3 beers on weekends). Occupation: Construction laborer.

VITAL SIGNS:
His blood pressure is 110/70 mmHg, heart rate is 92 beats per minute, respiratory rate is 14 breaths per minute with mechanical ventilation, and oxygen saturation is 98% on 50% FiO2.

PHYSICAL EXAMINATION:
His Glasgow Coma Scale score is 6. Pupils are equal and reactive. There is no evidence of external head trauma.

LABORATORY DATA:
His serum sodium is 138 mEq/L, potassium is 4.1 mEq/L, chloride is 100 mEq/L, bicarbonate is 24 mEq/L, BUN is 18 mg/dL, creatinine is 0.9 mg/dL, and glucose is 110 mg/dL.

IMAGING:
A CT scan of the head reveals diffuse cerebral edema without any focal lesion.

## Encounter 3: source 2020-07-15, practice 2026-02-15, icu, Neurosurgery

Chief complaint: Episodes of tachycardia, hypertension, and diaphoresis 2 days after admission

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Priya Patel, MD
Department: Neurosurgery
Visit Type: icu

CHIEF COMPLAINT:
Episodes of tachycardia, hypertension, and diaphoresis 2 days after admission

HISTORY OF PRESENT ILLNESS:
This 24-year-old male with Attention-deficit/hyperactivity disorder, combined type, and seasonal allergic rhinitis on methylphenidate extended-release and cetirizine, who was admitted to the ICU two days prior as the unrestrained driver in a car that hit a tree at high speed, presents with new episodes of tachycardia with heart rates up to 130 beats per minute, blood pressure spikes to 190/110 mmHg, and respiratory rate increases to 30 breaths per minute. He remains intubated and sedated due to a decreased level of consciousness. These episodes are accompanied by fever up to 38.9°C and profuse diaphoresis.

PAST MEDICAL HISTORY:
Active Problem List:
- Diffuse axonal injury (diagnosed 2020-07-15)
- Traumatic cerebral edema with elevated intracranial pressure (diagnosed 2020-07-15)
- Attention-deficit/hyperactivity disorder (ADHD), combined type
- Seasonal allergic rhinitis

PAST SURGICAL HISTORY:
- Tonsillectomy and adenoidectomy (age 8)
- Open reduction internal fixation of right distal radius fracture (age 16)

MEDICATIONS:
His current medications include propofol for sedation and fentanyl for analgesia.

Home Medications:
- Methylphenidate extended-release 20 mg PO every morning
- Cetirizine 10 mg PO daily as needed for allergic rhinitis

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension, hyperlipidemia
- Mother: Major depressive disorder
- Older brother: Attention-deficit/hyperactivity disorder

SOCIAL HISTORY:
Smoking: Former smoker; quit 18 months ago (1/2 pack per day history). Alcohol: Social alcohol use (2-3 beers on weekends). Occupation: Construction laborer.

PHYSICAL EXAMINATION:
His Glasgow Coma Scale score is 7. Physical examination during an episode reveals dilated pupils, increased muscle tone, and extensor posturing.

IMAGING:
A CT scan of the head reveals diffuse axonal injury with no evidence of hemorrhage.
