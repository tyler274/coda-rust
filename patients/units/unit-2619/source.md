# synthetic_hospital patient 2619

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 31 at first encounter, sex M, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-03-28 (Tue) | icu | Emergency Medicine |
| 1 | 2020-09-15 | 2023-11-19 (Sun) | ed | Emergency Medicine |
| 2 | 2021-11-15 | 2025-01-12 (Sun) | ed | Emergency Medicine |
| 3 | 2023-05-15 | 2026-07-24 (Fri) | icu | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Penicillin (anaphylaxis)"
 ],
 "insurance": "Medicaid",
 "occupation": "Construction laborer",
 "alcohol_use": "Heavy daily use, approximately 6-8 beers per day; history of binge drinking, frequent intoxication during recreational activities including motorcycle riding",
 "family_history": [
  "Father: Alcohol use disorder, deceased age 54 from cirrhosis",
  "Mother: Type 2 diabetes mellitus, hypothyroidism",
  "Brother: Major depressive disorder"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 1 pack per day for 12 years",
 "home_medications": [
  {
   "dose": "10 mcg in each nostril twice daily (frequently missed doses)",
   "name": "Desmopressin acetate nasal spray"
  },
  {
   "dose": "750 mg orally twice daily",
   "name": "Levetiracetam"
  },
  {
   "dose": "100 mg orally daily",
   "name": "Thiamine"
  },
  {
   "dose": "1 mg orally daily",
   "name": "Folic acid"
  },
  {
   "dose": "5 mg orally every 6 hours as needed for pain",
   "name": "Oxycodone"
  }
 ],
 "surgical_history": [
  "Craniotomy with evacuation of subdural hematoma (age 28)",
  "Open reduction and internal fixation of right femoral shaft fracture (age 28)"
 ],
 "chronic_conditions": [
  "Post-traumatic central diabetes insipidus",
  "Post-traumatic epilepsy",
  "Alcohol use disorder, severe",
  "Chronic pain syndrome"
 ],
 "age_at_first_encounter": 31
}
```

Primary diagnoses: ["Traumatic brain injury", "Central diabetes insipidus (inadequately controlled)", "Acute Epidural Hematoma", "Basilar skull fracture with brainstem herniation"]
Comorbidities: ["Traumatic brain injury", "Brain herniation"]

## Encounter 0: source 2020-01-15, practice 2023-03-28, icu, Emergency Medicine

Chief complaint: Found unconscious at the scene of a motorcycle accident without helmet use.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Found unconscious at the scene of a motorcycle accident without helmet use.

HISTORY OF PRESENT ILLNESS:
He was not wearing a helmet. On arrival, his Glasgow Coma Scale score is 4, and he is immediately intubated and admitted to the intensive care unit. Over the next 3 hours, the patient is given 1 liter of normal saline for suspected hypovolemia. However, he produces over 1500 mL of urine during that period. A Foley catheter remains in place, and fluid output continues at >300 mL/hr. The patient is receiving no diuretics or osmotic agents.

PAST MEDICAL HISTORY:
Active Problem List:
- Post-traumatic central diabetes insipidus
- Post-traumatic epilepsy
- Alcohol use disorder, severe
- Chronic pain syndrome

PAST SURGICAL HISTORY:
- Craniotomy with evacuation of subdural hematoma (age 28)
- Open reduction and internal fixation of right femoral shaft fracture (age 28)

MEDICATIONS:
Home Medications:
- Desmopressin acetate nasal spray 10 mcg in each nostril twice daily (frequently missed doses)
- Levetiracetam 750 mg orally twice daily
- Thiamine 100 mg orally daily
- Folic acid 1 mg orally daily
- Oxycodone 5 mg orally every 6 hours as needed for pain

ALLERGIES:
Penicillin (anaphylaxis)

FAMILY HISTORY:
- Father: Alcohol use disorder, deceased age 54 from cirrhosis
- Mother: Type 2 diabetes mellitus, hypothyroidism
- Brother: Major depressive disorder

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day for 12 years. Alcohol: Heavy daily use, approximately 6-8 beers per day; history of binge drinking, frequent intoxication during recreational activities including motorcycle riding. Occupation: Construction laborer.

PHYSICAL EXAMINATION:
Pupillary and corneal reflexes are intact.

LABORATORY DATA:
Repeat laboratory evaluation shows the following:
Serum sodium 152 mEq/L
Serum osmolality 308 mOsm/kg
Urine osmolality 64 mOsm/kg
Urine specific gravity 1.002
Serum glucose 112 mg/dL
Serum creatinine 0.9 mg/dL

IMAGING:
Initial CT imaging reveals a small subarachnoid hemorrhage and diffuse cerebral edema.

## Encounter 1: source 2020-09-15, practice 2023-11-19, ed, Emergency Medicine

Chief complaint: Motor vehicle collision with head strike against windshield while unrestrained.

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Motor vehicle collision with head strike against windshield while unrestrained.

HISTORY OF PRESENT ILLNESS:
This patient with post-traumatic central diabetes insipidus from a January 2020 motorcycle accident is brought to the emergency department after being involved in a motor vehicle collision. He was the unrestrained driver, and his head struck the windshield. On arrival, he is alert and oriented but appears drowsy.

PAST MEDICAL HISTORY:
Active Problem List:
- Central diabetes insipidus (inadequately controlled) (diagnosed 2020-01-15)
- Post-traumatic central diabetes insipidus
- Post-traumatic epilepsy
- Alcohol use disorder, severe
- Chronic pain syndrome

PAST SURGICAL HISTORY:
- Craniotomy with evacuation of subdural hematoma (age 28)
- Open reduction and internal fixation of right femoral shaft fracture (age 28)

MEDICATIONS:
Home Medications:
- Desmopressin acetate nasal spray 10 mcg in each nostril twice daily (frequently missed doses)
- Levetiracetam 750 mg orally twice daily
- Thiamine 100 mg orally daily
- Folic acid 1 mg orally daily
- Oxycodone 5 mg orally every 6 hours as needed for pain

ALLERGIES:
Penicillin (anaphylaxis)

FAMILY HISTORY:
- Father: Alcohol use disorder, deceased age 54 from cirrhosis
- Mother: Type 2 diabetes mellitus, hypothyroidism
- Brother: Major depressive disorder

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day for 12 years. Alcohol: Heavy daily use, approximately 6-8 beers per day; history of binge drinking, frequent intoxication during recreational activities including motorcycle riding. Occupation: Construction laborer.

VITAL SIGNS:
His vital signs are as follows: blood pressure 140/90 mmHg, heart rate 88 beats per minute, respiratory rate 16 breaths per minute, and temperature 37.0°C (98.6°F)

PHYSICAL EXAMINATION:
Physical examination reveals a laceration on his forehead and bruising around his left eye. His pupils are asymmetric, with the left pupil measuring 5 mm and fixed, while the right pupil is 3 mm and reactive to light. Motor examination shows 4/5 strength in the right upper and lower extremities and 5/5 strength in the left extremities.

IMAGING:
A head CT scan is performed urgently and reveals a biconvex, hyperdense collection along the left temporal region.

## Encounter 2: source 2021-11-15, practice 2025-01-12, ed, Emergency Medicine

Chief complaint: Rollover motor vehicle collision with progressive somnolence and emesis.

ENCOUNTER NOTE
Date: 2021-11-15
Provider: Dr. Emily Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Rollover motor vehicle collision with progressive somnolence and emesis.

HISTORY OF PRESENT ILLNESS:
The patient with central diabetes insipidus and traumatic brain injury from 2020 presents following a rollover collision. He was extricated after several minutes, and bystanders report he was briefly combative and disoriented. In the trauma bay, he becomes progressively more somnolent and less responsive to questions. He vomited once during transport.

PAST MEDICAL HISTORY:
Active Problem List:
- Central diabetes insipidus (inadequately controlled) (diagnosed 2020-01-15)
- Acute Epidural Hematoma (diagnosed 2020-09-15)
- Post-traumatic central diabetes insipidus
- Post-traumatic epilepsy
- Alcohol use disorder, severe
- Chronic pain syndrome

PAST SURGICAL HISTORY:
- Craniotomy with evacuation of subdural hematoma (age 28)
- Open reduction and internal fixation of right femoral shaft fracture (age 28)

MEDICATIONS:
Home Medications:
- Desmopressin acetate nasal spray 10 mcg in each nostril twice daily (frequently missed doses)
- Levetiracetam 750 mg orally twice daily
- Thiamine 100 mg orally daily
- Folic acid 1 mg orally daily
- Oxycodone 5 mg orally every 6 hours as needed for pain

ALLERGIES:
Penicillin (anaphylaxis)

FAMILY HISTORY:
- Father: Alcohol use disorder, deceased age 54 from cirrhosis
- Mother: Type 2 diabetes mellitus, hypothyroidism
- Brother: Major depressive disorder

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day for 12 years. Alcohol: Heavy daily use, approximately 6-8 beers per day; history of binge drinking, frequent intoxication during recreational activities including motorcycle riding. Occupation: Construction laborer.

VITAL SIGNS:
Blood pressure is elevated and pulse is slow. Pulse oximetry dips during episodes of hypoventilation.

PHYSICAL EXAMINATION:
Examination reveals a right temporal scalp contusion with an underlying step-off. His right pupil appears larger than the left and reacts sluggishly to bright light. He exhibits weaker spontaneous movement of the left arm and leg compared with the right. His breathing pattern is irregular, punctuated by short apneic spells. A rigid cervical collar is in place and breath sounds are clear bilaterally.

PLAN:
Rapid sequence endotracheal intubation is performed. The team initiates controlled hyperventilation per protocol. The head of the bed is elevated and sedation is started. Shortly afterward, the pupillary asymmetry appears slightly less pronounced. Neurosurgery is en route for emergent intervention. While preparing for imaging, the senior resident reviews why the ventilator strategy can transiently lower intracranial pressure. She asks the medical student to explain the key physiologic effect of lowering arterial carbon dioxide in this setting.

## Encounter 3: source 2023-05-15, practice 2026-07-24, icu, Emergency Medicine

Chief complaint: Fall from 10-foot height onto concrete surface with altered mental status.

ENCOUNTER NOTE
Date: 2023-05-15
Provider: Dr. James Park, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Fall from 10-foot height onto concrete surface with altered mental status.

HISTORY OF PRESENT ILLNESS:
This 34-year-old male with a history of post-traumatic central diabetes insipidus and epilepsy secondary to multiple prior traumatic brain injuries presents after a fall. On arrival, he is confused but able to answer simple questions.

PAST MEDICAL HISTORY:
Active Problem List:
- Central diabetes insipidus (inadequately controlled) (diagnosed 2020-01-15)
- Acute Epidural Hematoma (diagnosed 2020-09-15)
- Traumatic brain injury (diagnosed 2021-11-15)
- Post-traumatic central diabetes insipidus
- Post-traumatic epilepsy
- Alcohol use disorder, severe
- Chronic pain syndrome

PAST SURGICAL HISTORY:
- Craniotomy with evacuation of subdural hematoma (age 28)
- Open reduction and internal fixation of right femoral shaft fracture (age 28)

MEDICATIONS:
Home Medications:
- Desmopressin acetate nasal spray 10 mcg in each nostril twice daily (frequently missed doses)
- Levetiracetam 750 mg orally twice daily
- Thiamine 100 mg orally daily
- Folic acid 1 mg orally daily
- Oxycodone 5 mg orally every 6 hours as needed for pain

ALLERGIES:
Penicillin (anaphylaxis)

FAMILY HISTORY:
- Father: Alcohol use disorder, deceased age 54 from cirrhosis
- Mother: Type 2 diabetes mellitus, hypothyroidism
- Brother: Major depressive disorder

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day for 12 years. Alcohol: Heavy daily use, approximately 6-8 beers per day; history of binge drinking, frequent intoxication during recreational activities including motorcycle riding. Occupation: Construction laborer.

PHYSICAL EXAMINATION:
Physical examination reveals periorbital ecchymoses, clear fluid leaking from the nose, and ecchymosis over the mastoid process. He opens his eyes to voice, follows commands inconsistently, and withdraws to pain.

IMAGING:
Head CT is ordered to assess for intracranial injury.
