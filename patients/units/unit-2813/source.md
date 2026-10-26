# synthetic_hospital patient 2813

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 41 at first encounter, sex F, Non-Hispanic White, Commercial PPO (employer-sponsored).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-03-17 (Thu) | ed | Emergency Medicine |
| 1 | 2024-01-15 | 2026-03-20 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Warfarin (skin necrosis/hemorrhagic infarction)",
  "Sulfa drugs (rash)"
 ],
 "insurance": "Commercial PPO (employer-sponsored)",
 "occupation": "Secondary school teacher",
 "alcohol_use": "Social (1-2 glasses of wine weekly)",
 "family_history": [
  "Mother: deep vein thrombosis (age 54), heterozygous Factor V Leiden carrier",
  "Maternal grandmother: multiple late pregnancy losses",
  "Father: type 2 diabetes mellitus",
  "Sister: healthy"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "5 mg PO twice daily",
   "name": "Apixaban"
  },
  {
   "dose": "1 tablet PO daily",
   "name": "Prenatal vitamin (methylfolate formulation)"
  },
  {
   "dose": "2000 units PO daily",
   "name": "Vitamin D3"
  }
 ],
 "surgical_history": [
  "Dilation and curettage (2019, 2021)",
  "Laparoscopic appendectomy (age 16)"
 ],
 "chronic_conditions": [
  "Heterozygous Factor V Leiden thrombophilia",
  "History of recurrent pregnancy loss (3 first-trimester spontaneous abortions)",
  "History of provoked deep vein thrombosis (left lower extremity)"
 ],
 "age_at_first_encounter": 41
}
```

Primary diagnoses: ["Anticoagulation-related hemorrhage", "Factor V Leiden thrombophilia"]
Comorbidities: ["Recurrent pregnancy loss"]

## Encounter 0: source 2020-01-15, practice 2022-03-17, ed, Emergency Medicine

Chief complaint: Sudden onset of right calf pain and swelling

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of right calf pain and swelling

HISTORY OF PRESENT ILLNESS:
That began earlier in the day. She denies any recent trauma, surgery, or prolonged immobilization.

PAST MEDICAL HISTORY:
Active Problem List:
- Heterozygous Factor V Leiden thrombophilia
- History of recurrent pregnancy loss (3 first-trimester spontaneous abortions)
- History of provoked deep vein thrombosis (left lower extremity)

PAST SURGICAL HISTORY:
- Dilation and curettage (2019, 2021)
- Laparoscopic appendectomy (age 16)

MEDICATIONS:
Home Medications:
- Apixaban 5 mg PO twice daily
- Prenatal vitamin (methylfolate formulation) 1 tablet PO daily
- Vitamin D3 2000 units PO daily

ALLERGIES:
Warfarin (skin necrosis/hemorrhagic infarction), Sulfa drugs (rash)

FAMILY HISTORY:
Her family history is significant for her father having had a deep vein thrombosis (DVT) at the age of 40.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine weekly). Occupation: Secondary school teacher.

VITAL SIGNS:
Her vital signs are stable

PHYSICAL EXAMINATION:
On physical examination, her right calf is warm, erythematous, and tender to palpation. There is mild pitting edema compared to the left leg. The Homans' sign is positive. The rest of the physical examination is unremarkable.

LABORATORY DATA:
Laboratory values are as follows:

- Complete Blood Count (CBC): Within normal limits
- Prothrombin Time (PT): 12 seconds (control 11-13.5 seconds)
- Activated Partial Thromboplastin Time (aPTT): 30 seconds (control 25-35 seconds)
- D-dimer: Elevated

IMAGING:
A Doppler ultrasound of the right leg confirms the presence of a thrombus in the popliteal vein.

## Encounter 1: source 2024-01-15, practice 2026-03-20, ed, Emergency Medicine

Chief complaint: Painful, erythematous, and purpuric lesions on breasts and thighs

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. James Okafor, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Painful, erythematous, and purpuric lesions on breasts and thighs

HISTORY OF PRESENT ILLNESS:
This 45-year-old female with heterozygous Factor V Leiden thrombophilia diagnosed in 2020 and history of recurrent pregnancy loss and provoked deep vein thrombosis returns to the emergency department with a 2-day history of painful, erythematous, and purpuric lesions on her breasts and thighs. She reports that she started taking warfarin 5 days ago for a newly diagnosed deep vein thrombosis.

PAST MEDICAL HISTORY:
Active Problem List:
- Factor V Leiden thrombophilia (diagnosed 2020-01-15)
- Heterozygous Factor V Leiden thrombophilia
- History of recurrent pregnancy loss (3 first-trimester spontaneous abortions)
- History of provoked deep vein thrombosis (left lower extremity)

Her medical history is significant for recurrent miscarriages.

PAST SURGICAL HISTORY:
- Dilation and curettage (2019, 2021)
- Laparoscopic appendectomy (age 16)

MEDICATIONS:
Home Medications:
- Apixaban 5 mg PO twice daily
- Prenatal vitamin (methylfolate formulation) 1 tablet PO daily
- Vitamin D3 2000 units PO daily

ALLERGIES:
Warfarin (skin necrosis/hemorrhagic infarction), Sulfa drugs (rash)

FAMILY HISTORY:
- Mother: deep vein thrombosis (age 54), heterozygous Factor V Leiden carrier
- Maternal grandmother: multiple late pregnancy losses
- Father: type 2 diabetes mellitus
- Sister: healthy

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine weekly). Occupation: Secondary school teacher.

VITAL SIGNS:
She is afebrile, and her vital signs are stable.

PHYSICAL EXAMINATION:
On examination, the lesions are sharply demarcated, with a central area of necrosis.

LABORATORY DATA:
Laboratory values are as follows:

- INR: 4.5 (normal range: 0.8-1.2)
- PT: 42 seconds (normal range: 12-15 seconds)
- aPTT: 30 seconds (normal range: 30-40 seconds)
- Platelet count: 250,000/µL (normal range: 150,000-400,000/µL)
- Protein C activity: Pending

ASSESSMENT AND PLAN:
Based on the clinical presentation and the patient's recent initiation of warfarin therapy,
