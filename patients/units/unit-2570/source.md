# synthetic_hospital patient 2570

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 22 at first encounter, sex M, Hispanic (Mexican-American), Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-04-06 (Sun) | ed | Emergency Medicine |
| 1 | 2020-09-15 | 2025-12-04 (Thu) | ed | Emergency Medicine |
| 2 | 2021-05-15 | 2026-08-02 (Sun) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Apprentice electrician",
 "alcohol_use": "Heavy episodic drinking (5-6 standard drinks per occasion), 2-3 times weekly; prior DUI citation at age 20",
 "family_history": [
  "Father: Alcohol use disorder, hypertension",
  "Mother: Major depressive disorder, hypothyroidism",
  "Older brother: ADHD, learning disability"
 ],
 "race_ethnicity": "Hispanic (Mexican-American)",
 "smoking_status": "Current daily smoker, 1 pack per day for 4 years",
 "home_medications": [
  {
   "dose": "36 mg PO every morning",
   "name": "Methylphenidate ER"
  },
  {
   "dose": "50 mg PO daily",
   "name": "Sertraline"
  },
  {
   "dose": "600 mg PO every 6 hours PRN pain",
   "name": "Ibuprofen"
  }
 ],
 "surgical_history": [
  "Open reduction and internal fixation of left distal radius fracture (age 19)",
  "Tonsillectomy and adenoidectomy (age 8)"
 ],
 "chronic_conditions": [
  "Attention-deficit/hyperactivity disorder (ADHD), combined presentation",
  "History of mild traumatic brain injury with residual post-concussive syndrome (age 19)"
 ],
 "age_at_first_encounter": 22
}
```

Primary diagnoses: ["Posterior hip dislocation", "Diffuse axonal injury"]
Comorbidities: ["Motor vehicle accident", "Delayed management of life-threatening respiratory failure"]

## Encounter 0: source 2020-01-15, practice 2025-04-06, ed, Emergency Medicine

Chief complaint: Severe pain in the right hip

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Maria Santos, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe pain in the right hip

HISTORY OF PRESENT ILLNESS:
He is brought to the emergency department by ambulance after being involved in a high-speed motor vehicle collision. He was the front-seat passenger and was not wearing a seatbelt. According to EMS, the car struck a guardrail and spun into a ditch. The patient was alert at the scene but complained of severe pain in the right hip and was unable to bear weight. He denies head trauma, loss of consciousness, or abdominal pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Attention-deficit/hyperactivity disorder (ADHD), combined presentation
- History of mild traumatic brain injury with residual post-concussive syndrome (age 19)

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of left distal radius fracture (age 19)
- Tonsillectomy and adenoidectomy (age 8)

MEDICATIONS:
Home Medications:
- Methylphenidate ER 36 mg PO every morning
- Sertraline 50 mg PO daily
- Ibuprofen 600 mg PO every 6 hours PRN pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder, hypertension
- Mother: Major depressive disorder, hypothyroidism
- Older brother: ADHD, learning disability

SOCIAL HISTORY:
Smoking: Current daily smoker, 1 pack per day for 4 years. Alcohol: Heavy episodic drinking (5-6 standard drinks per occasion), 2-3 times weekly; prior DUI citation at age 20. Occupation: Apprentice electrician.

PHYSICAL EXAMINATION:
There is no bleeding, open wound, or deformity of the thigh. On physical examination, the right leg appears shortened and held in a position of internal rotation at the hip. Passive range of motion is limited by pain. There is no sensory loss or distal neurovascular compromise.

IMAGING:
Radiographs are pending

PLAN:
The orthopedic resident is consulted for evaluation

## Encounter 1: source 2020-09-15, practice 2025-12-04, ed, Emergency Medicine

Chief complaint: Brought to the emergency department after being involved in a high-speed motor vehicle collision

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. James Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Brought to the emergency department after being involved in a high-speed motor vehicle collision

HISTORY OF PRESENT ILLNESS:
This 22-year-old male with Attention-deficit/hyperactivity disorder, combined presentation, history of mild traumatic brain injury with residual post-concussive syndrome sustained at age 19, and prior emergency department evaluation in January 2020 for posterior hip dislocation presents following a high-speed motor vehicle collision. He was the unrestrained driver, and he lost consciousness at the scene.

PAST MEDICAL HISTORY:
Active Problem List:
- Posterior hip dislocation (diagnosed 2020-01-15)
- Attention-deficit/hyperactivity disorder (ADHD), combined presentation
- History of mild traumatic brain injury with residual post-concussive syndrome (age 19)

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of left distal radius fracture (age 19)
- Tonsillectomy and adenoidectomy (age 8)

MEDICATIONS:
Home Medications:
- Methylphenidate ER 36 mg PO every morning
- Sertraline 50 mg PO daily
- Ibuprofen 600 mg PO every 6 hours PRN pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder, hypertension
- Mother: Major depressive disorder, hypothyroidism
- Older brother: ADHD, learning disability

SOCIAL HISTORY:
Smoking: Current daily smoker, 1 pack per day for 4 years. Alcohol: Heavy episodic drinking (5-6 standard drinks per occasion), 2-3 times weekly; prior DUI citation at age 20. Occupation: Apprentice electrician.

VITAL SIGNS:
BP 130/85 mmHg, HR 102 bpm, RR 18 breaths/min, and temperature 37.1°C

PHYSICAL EXAMINATION:
On arrival, he is disoriented and agitated. His Glasgow Coma Scale score is 13. Physical examination reveals an abrasion over his forehead, but no other external signs of trauma. Pupils are equal and reactive to light. No focal neurological deficits are noted.

IMAGING:
A CT scan of the head shows numerous minute punctate hemorrhages with blurring of the gray-white interface. There is no evidence of skull fractures or large intracranial hematomas.

## Encounter 2: source 2021-05-15, practice 2026-08-02, ed, Emergency Medicine

Chief complaint: Brought to the emergency department after being involved in a high-speed motor vehicle collision

ENCOUNTER NOTE
Date: 2021-05-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Brought to the emergency department after being involved in a high-speed motor vehicle collision

HISTORY OF PRESENT ILLNESS:
The patient with ADHD and prior diffuse axonal injury presents following a motor vehicle collision. He was the unrestrained driver, and there was significant damage to the driver's side of the vehicle. On arrival, he is unconscious with a Glasgow Coma Scale score of 6.

PAST MEDICAL HISTORY:
Active Problem List:
- Posterior hip dislocation (diagnosed 2020-01-15)
- Diffuse axonal injury (diagnosed 2020-09-15)
- Attention-deficit/hyperactivity disorder (ADHD), combined presentation
- History of mild traumatic brain injury with residual post-concussive syndrome (age 19)

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of left distal radius fracture (age 19)
- Tonsillectomy and adenoidectomy (age 8)

MEDICATIONS:
Home Medications:
- Methylphenidate ER 36 mg PO every morning
- Sertraline 50 mg PO daily
- Ibuprofen 600 mg PO every 6 hours PRN pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder, hypertension
- Mother: Major depressive disorder, hypothyroidism
- Older brother: ADHD, learning disability

SOCIAL HISTORY:
Smoking: Current daily smoker, 1 pack per day for 4 years. Alcohol: Heavy episodic drinking (5-6 standard drinks per occasion), 2-3 times weekly; prior DUI citation at age 20. Occupation: Apprentice electrician.

PHYSICAL EXAMINATION:
His pupils are equal and reactive, and there is no evidence of skull fractures or focal neurologic deficits on examination.

IMAGING:
Initial CT scan of the head shows no evidence of skull fractures, intracranial hemorrhage, or cerebral edema.

ASSESSMENT AND PLAN:
Despite the lack of significant findings on the CT scan, the patient remains comatose with intermittent decerebrate posturing.
