# synthetic_hospital patient 2076

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 42 at first encounter, sex M, Non-Hispanic White, Private insurance through employer.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-05-16 (Mon) | outpatient | Neurology |
| 1 | 2020-03-15 | 2022-07-21 (Thu) | outpatient | General Surgery |
| 2 | 2020-07-15 | 2022-11-14 (Mon) | outpatient | Nephrology |
| 3 | 2021-01-15 | 2023-05-11 (Thu) | outpatient | Dermatology |
| 4 | 2022-01-15 | 2024-05-20 (Mon) | outpatient | Orthopedic Surgery |
| 5 | 2022-07-15 | 2024-11-15 (Fri) | ed | Emergency Medicine |
| 6 | 2024-01-15 | 2026-05-23 (Sat) | icu | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private insurance through employer",
 "occupation": "Warehouse supervisor",
 "alcohol_use": "Social, 1-2 drinks per week",
 "family_history": [
  "Father: Type 2 Diabetes Mellitus with diabetic nephropathy",
  "Mother: Hypertension",
  "Sister: Autoimmune thyroid disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 3 years ago",
 "home_medications": [
  {
   "dose": "1000 mg PO twice daily",
   "name": "Metformin"
  },
  {
   "dose": "300 mg PO three times daily",
   "name": "Gabapentin"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Furosemide"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Atorvastatin"
  }
 ],
 "surgical_history": [
  "Split-thickness skin grafting for thermal burns involving chest and upper extremities",
  "Incision and drainage of perianal abscess"
 ],
 "chronic_conditions": [
  "Type 2 Diabetes Mellitus",
  "Chronic inflammatory demyelinating polyneuropathy",
  "Lumbar disc herniation with radiculopathy",
  "Anorectal fistula",
  "Nephrotic syndrome",
  "Recurrent dermatophyte infections (tinea pedis, corporis)"
 ],
 "age_at_first_encounter": 42
}
```

Primary diagnoses: ["Soft tissue infection", "Third-degree thermal burns involving 36% total body surface area", "Anorectal fistula", "Lumbar radiculopathy due to disc herniation", "Chronic inflammatory demyelinating polyneuropathy", "Tinea corporis", "Nephrotic syndrome (edema management)"]
Comorbidities: ["Type 2 diabetes mellitus", "Tinea pedis"]

## Encounter 0: source 2020-01-15, practice 2022-05-16, outpatient, Neurology

Chief complaint: Progressive symmetric weakness in arms and legs with difficulty climbing stairs and lifting objects

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive symmetric weakness in arms and legs with difficulty climbing stairs and lifting objects

HISTORY OF PRESENT ILLNESS:
He presents to the clinic with a 6-month history of progressive weakness in both his arms and legs. He reports difficulty climbing stairs and lifting objects above his head. He also mentions occasional tingling in his hands and feet.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 Diabetes Mellitus
- Chronic inflammatory demyelinating polyneuropathy
- Lumbar disc herniation with radiculopathy
- Anorectal fistula
- Nephrotic syndrome
- Recurrent dermatophyte infections (tinea pedis, corporis)

His past medical history is significant for type 2 diabetes mellitus,.

PAST SURGICAL HISTORY:
- Split-thickness skin grafting for thermal burns involving chest and upper extremities
- Incision and drainage of perianal abscess

MEDICATIONS:
He takes which is well-controlled with metformin.

Home Medications:
- Gabapentin 300 mg PO three times daily
- Lisinopril 10 mg PO daily
- Furosemide 40 mg PO daily
- Atorvastatin 20 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus with diabetic nephropathy
- Mother: Hypertension
- Sister: Autoimmune thyroid disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 3 years ago. Alcohol: Social, 1-2 drinks per week. Occupation: Warehouse supervisor.

PHYSICAL EXAMINATION:
On physical examination, you note symmetric weakness in both proximal and distal muscles, decreased reflexes in all extremities, and mild sensory loss in a glove-and-stocking distribution.

LABORATORY DATA:
Cerebrospinal fluid analysis reveals elevated protein with a normal cell count.

OTHER STUDIES:
Nerve conduction studies show slowed conduction velocities and prolonged distal latencies.

## Encounter 1: source 2020-03-15, practice 2022-07-21, outpatient, General Surgery

Chief complaint: Six-month history of intermittent perianal pain worsening with defecation and chronic foul-smelling discharge

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Michael Rodriguez, MD
Department: General Surgery
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of intermittent perianal pain worsening with defecation and chronic foul-smelling discharge

HISTORY OF PRESENT ILLNESS:
The patient with Type 2 Diabetes Mellitus and recently diagnosed Chronic inflammatory demyelinating polyneuropathy presents for surgical evaluation. He also reports a chronic, foul-smelling discharge from the same area.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic inflammatory demyelinating polyneuropathy (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus
- Lumbar disc herniation with radiculopathy
- Anorectal fistula
- Nephrotic syndrome
- Recurrent dermatophyte infections (tinea pedis, corporis)

His past medical history is significant for type 2 diabetes mellitus, which is controlled with metformin.

PAST SURGICAL HISTORY:
- Split-thickness skin grafting for thermal burns involving chest and upper extremities
- Incision and drainage of perianal abscess

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Gabapentin 300 mg PO three times daily
- Lisinopril 10 mg PO daily
- Furosemide 40 mg PO daily
- Atorvastatin 20 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus with diabetic nephropathy
- Mother: Hypertension
- Sister: Autoimmune thyroid disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 3 years ago. Alcohol: Social, 1-2 drinks per week. Occupation: Warehouse supervisor.

PHYSICAL EXAMINATION:
On physical examination, there is erythema and tenderness to palpation around the anus, and a small opening is noted on the skin adjacent to the anal verge. Digital rectal examination is painful for the patient and reveals induration in the perianal region. No rectal masses are palpable, and the prostate is unremarkable.

LABORATORY DATA:
Laboratory values are as follows:

- White blood cell count: 9,000 /µL
- Hemoglobin: 13.5 g/dL
- Fasting blood glucose: 160 mg/dL
- C-reactive protein: 2.8 mg/L

## Encounter 2: source 2020-07-15, practice 2022-11-14, outpatient, Nephrology

Chief complaint: Progressive bilateral leg swelling with unintentional weight gain and left flank discomfort

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Priya Patel, MD
Department: Nephrology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive bilateral leg swelling with unintentional weight gain and left flank discomfort

HISTORY OF PRESENT ILLNESS:
Patient returns for nephrology follow-up with CIDP and anorectal fistula. He has also noticed progressive swelling in both legs over the past few weeks, along with an unintentional weight gain.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic inflammatory demyelinating polyneuropathy (diagnosed 2020-01-15)
- Anorectal fistula (diagnosed 2020-03-15)
- Type 2 Diabetes Mellitus
- Lumbar disc herniation with radiculopathy
- Nephrotic syndrome
- Recurrent dermatophyte infections (tinea pedis, corporis)

His medical history includes type 2 diabetes mellitus, managed with oral agents.

PAST SURGICAL HISTORY:
- Split-thickness skin grafting for thermal burns involving chest and upper extremities
- Incision and drainage of perianal abscess

MEDICATIONS:
He does not take any anticoagulants or antiplatelet medications.

Home Medications:
- Metformin 1000 mg PO twice daily
- Gabapentin 300 mg PO three times daily
- Lisinopril 10 mg PO daily
- Furosemide 40 mg PO daily
- Atorvastatin 20 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus with diabetic nephropathy
- Mother: Hypertension
- Sister: Autoimmune thyroid disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 3 years ago. Alcohol: Social, 1-2 drinks per week. Occupation: Warehouse supervisor.

VITAL SIGNS:
Vital signs are stable.

PHYSICAL EXAMINATION:
On physical examination, there is bilateral lower extremity pitting edema and a soft, non-tender abdomen. When standing, a palpable fullness is noted above the left testicle.

LABORATORY DATA:
Urinalysis reveals marked proteinuria. Further workup shows low serum albumin and preserved kidney function.

IMAGING:
Imaging of the abdomen is ordered and reveals a left renal vein thrombosis.

## Encounter 3: source 2021-01-15, practice 2023-05-11, outpatient, Dermatology

Chief complaint: Pruritic, scaly rash on trunk with central clearing and similar lesions in groin and between toes

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. James O'Brien, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Pruritic, scaly rash on trunk with central clearing and similar lesions in groin and between toes

HISTORY OF PRESENT ILLNESS:
The patient with a history of recurrent dermatophyte infections and ongoing management for nephrotic syndrome presents with new skin complaints. He reports that the rash began as a small red patch that has slowly expanded outward with central clearing. He also complains of similar smaller lesions on his groin and between his toes. He denies any recent travel or sick contacts.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic inflammatory demyelinating polyneuropathy (diagnosed 2020-01-15)
- Anorectal fistula (diagnosed 2020-03-15)
- Nephrotic syndrome (edema management) (diagnosed 2020-07-15)
- Type 2 Diabetes Mellitus
- Lumbar disc herniation with radiculopathy
- Nephrotic syndrome
- Recurrent dermatophyte infections (tinea pedis, corporis)

PAST SURGICAL HISTORY:
- Split-thickness skin grafting for thermal burns involving chest and upper extremities
- Incision and drainage of perianal abscess

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Gabapentin 300 mg PO three times daily
- Lisinopril 10 mg PO daily
- Furosemide 40 mg PO daily
- Atorvastatin 20 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus with diabetic nephropathy
- Mother: Hypertension
- Sister: Autoimmune thyroid disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 3 years ago. Alcohol: Social, 1-2 drinks per week. Occupation: Warehouse supervisor.

PHYSICAL EXAMINATION:
On physical examination, the patient has well-demarcated, erythematous, annular plaques with raised borders and central clearing on his trunk. Similar lesions are noted in his groin and interdigital spaces of his feet. His toenails are thickened and discolored.

LABORATORY DATA:
His recent laboratory values are as follows:

- Fasting blood glucose: 210 mg/dL
- Hemoglobin A1c: 9.2%
- Complete blood count: Within normal limits
- HIV test: Negative

## Encounter 4: source 2022-01-15, practice 2024-05-20, outpatient, Orthopedic Surgery

Chief complaint: Low back pain radiating down right leg with tingling and difficulty lifting right foot

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Emily Johnson, MD
Department: Orthopedic Surgery
Visit Type: outpatient

CHIEF COMPLAINT:
Low back pain radiating down right leg with tingling and difficulty lifting right foot

HISTORY OF PRESENT ILLNESS:
The patient returns for evaluation of lower extremity symptoms in the context of known lumbar disc herniation and prior diagnosis of chronic inflammatory demyelinating polyneuropathy. He reports the pain radiates down his right leg and is associated with a tingling sensation over the lateral aspect of his thigh and calf. He also mentions difficulty when trying to lift his right foot off the ground.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic inflammatory demyelinating polyneuropathy (diagnosed 2020-01-15)
- Anorectal fistula (diagnosed 2020-03-15)
- Nephrotic syndrome (edema management) (diagnosed 2020-07-15)
- Tinea corporis (diagnosed 2021-01-15)
- Type 2 Diabetes Mellitus
- Lumbar disc herniation with radiculopathy
- Nephrotic syndrome
- Recurrent dermatophyte infections (tinea pedis, corporis)

His medical history is significant for type 2 diabetes mellitus, which is controlled with metformin.

PAST SURGICAL HISTORY:
- Split-thickness skin grafting for thermal burns involving chest and upper extremities
- Incision and drainage of perianal abscess

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Gabapentin 300 mg PO three times daily
- Lisinopril 10 mg PO daily
- Furosemide 40 mg PO daily
- Atorvastatin 20 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus with diabetic nephropathy
- Mother: Hypertension
- Sister: Autoimmune thyroid disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 3 years ago. Alcohol: Social, 1-2 drinks per week. Occupation: Warehouse supervisor.

PHYSICAL EXAMINATION:
On physical examination, there is decreased sensation over the lateral thigh, calf, and dorsal foot. The patient has weakness during great toe extension and difficulty with foot dorsiflexion, inversion, and eversion. Straight leg raise test is positive on the right side at 30 degrees. Ankle reflex is diminished on the affected side.

IMAGING:
MRI of the lumbar spine reveals a herniated disc at the L4-L5 level with nerve root compression.

## Encounter 5: source 2022-07-15, practice 2024-11-15, ed, Emergency Medicine

Chief complaint: Painful, red, and swollen left lower leg with fever and warmth

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Painful, red, and swollen left lower leg with fever and warmth

HISTORY OF PRESENT ILLNESS:
This 44-year-old male with Type 2 Diabetes Mellitus, nephrotic syndrome, and recurrent dermatophyte infections previously evaluated in Dermatology presents to the Emergency Department with a 3-day history of a painful, red, and swollen left lower leg. He reports a sensation of warmth over the affected area and has had a low-grade fever at home. He denies any trauma to the leg but mentions that he has been dealing with a persistent foot rash for several months, which he has not treated.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic inflammatory demyelinating polyneuropathy (diagnosed 2020-01-15)
- Anorectal fistula (diagnosed 2020-03-15)
- Nephrotic syndrome (edema management) (diagnosed 2020-07-15)
- Tinea corporis (diagnosed 2021-01-15)
- Lumbar radiculopathy due to disc herniation (diagnosed 2022-01-15)
- Type 2 Diabetes Mellitus
- Lumbar disc herniation with radiculopathy
- Nephrotic syndrome
- Recurrent dermatophyte infections (tinea pedis, corporis)

His medical history is significant for poorly controlled type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Split-thickness skin grafting for thermal burns involving chest and upper extremities
- Incision and drainage of perianal abscess

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Gabapentin 300 mg PO three times daily
- Lisinopril 10 mg PO daily
- Furosemide 40 mg PO daily
- Atorvastatin 20 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus with diabetic nephropathy
- Mother: Hypertension
- Sister: Autoimmune thyroid disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 3 years ago. Alcohol: Social, 1-2 drinks per week. Occupation: Warehouse supervisor.

PHYSICAL EXAMINATION:
On physical examination, the left lower leg is erythematous, edematous, and tender to palpation. There is no crepitus, and pulses are intact. The interdigital spaces of his left foot show scaling and maceration.

LABORATORY DATA:
- White blood cell count: 14,000 /µL
- Neutrophils: 80%
- Hemoglobin A1c: 9.2%
- C-reactive protein: 18 mg/L (normal <5 mg/L)

## Encounter 6: source 2024-01-15, practice 2026-05-23, icu, Emergency Medicine

Chief complaint: Sustained third-degree thermal burns involving chest, abdomen, and upper extremities in house fire

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Aisha Washington, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Sustained third-degree thermal burns involving chest, abdomen, and upper extremities in house fire

HISTORY OF PRESENT ILLNESS:
The patient, with established chronic inflammatory demyelinating polyneuropathy and nephrotic syndrome, presents following a house fire. He has second and third-degree burns covering his entire chest, abdomen, and both upper extremities.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic inflammatory demyelinating polyneuropathy (diagnosed 2020-01-15)
- Anorectal fistula (diagnosed 2020-03-15)
- Nephrotic syndrome (edema management) (diagnosed 2020-07-15)
- Tinea corporis (diagnosed 2021-01-15)
- Lumbar radiculopathy due to disc herniation (diagnosed 2022-01-15)
- Soft tissue infection (diagnosed 2022-07-15)
- Type 2 Diabetes Mellitus
- Lumbar disc herniation with radiculopathy
- Nephrotic syndrome
- Recurrent dermatophyte infections (tinea pedis, corporis)

His past medical history is significant for type 2 diabetes mellitus, well-controlled on metformin.

PAST SURGICAL HISTORY:
- Split-thickness skin grafting for thermal burns involving chest and upper extremities
- Incision and drainage of perianal abscess

MEDICATIONS:
Home Medications:
- Metformin 1000 mg PO twice daily
- Gabapentin 300 mg PO three times daily
- Lisinopril 10 mg PO daily
- Furosemide 40 mg PO daily
- Atorvastatin 20 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 Diabetes Mellitus with diabetic nephropathy
- Mother: Hypertension
- Sister: Autoimmune thyroid disease

SOCIAL HISTORY:
Smoking: Former smoker, quit 3 years ago. Alcohol: Social, 1-2 drinks per week. Occupation: Warehouse supervisor.

VITAL SIGNS:
His vital signs are as follows: blood pressure 110/70 mmHg, heart rate 120 beats per minute, respiratory rate 22 breaths per minute, and temperature 37.8°C.

PHYSICAL EXAMINATION:
On examination, the burned areas appear dry and leathery with a white and charred surface. There are no blisters present, and the patient reports minimal pain in the most severely affected areas.

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 16,000 /μL
- Hemoglobin: 14.5 g/dL
- Serum sodium: 145 mEq/L
- Serum potassium: 4.2 mEq/L
- Serum creatinine: 1.1 mg/dL
- Random blood glucose: 200 mg/dL
