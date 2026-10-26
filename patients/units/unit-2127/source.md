# synthetic_hospital patient 2127

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 37 at first encounter, sex M, White, non-Hispanic, Employer-sponsored PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-03-01 (Sat) | inpatient | Internal Medicine |
| 1 | 2020-07-15 | 2025-08-21 (Thu) | ed | Emergency Medicine |
| 2 | 2021-01-15 | 2026-02-25 (Wed) | icu | Critical Care Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Employer-sponsored PPO",
 "occupation": "Medical Laboratory Technician at St. Luke's Regional Medical Center; part-time operator of family-owned soybean and corn farm",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Father: Hypertension",
  "Mother: Type 2 diabetes mellitus",
  "Paternal grandfather: Myocardial infarction (age 62)"
 ],
 "race_ethnicity": "White, non-Hispanic",
 "smoking_status": "Former smoker (quit 2018; 6 pack-year history)",
 "home_medications": [
  {
   "dose": "10 mg orally daily as needed for allergic rhinitis",
   "name": "Cetirizine"
  }
 ],
 "surgical_history": [
  "Laparoscopic appendectomy (age 16)"
 ],
 "chronic_conditions": [
  "Seasonal allergic rhinitis"
 ],
 "age_at_first_encounter": 37
}
```

Primary diagnoses: ["Organophosphate poisoning", "Methicillin-resistant Staphylococcus aureus (MRSA) skin and soft tissue infection"]
Comorbidities: ["Occupational mechanical strain"]

## Encounter 0: source 2020-01-15, practice 2025-03-01, inpatient, Internal Medicine

Chief complaint: worsening redness, swelling, and pain in right forearm with fever and chills

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Worsening redness, swelling, and pain in right forearm with fever and chills

HISTORY OF PRESENT ILLNESS:
4-day history of worsening redness, swelling, and pain in his right forearm after scraping it while working in his hospital laboratory. He has had fevers and chills for the past 2 days.

PAST MEDICAL HISTORY:
Active Problem List:
- Seasonal allergic rhinitis

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy (age 16)

MEDICATIONS:
Home Medications:
- Cetirizine 10 mg orally daily as needed for allergic rhinitis

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertension
- Mother: Type 2 diabetes mellitus
- Paternal grandfather: Myocardial infarction (age 62)

SOCIAL HISTORY:
Smoking: Former smoker (quit 2018; 6 pack-year history). Alcohol: Social (1-2 drinks per week). Occupation: Medical Laboratory Technician at St. Luke's Regional Medical Center; part-time operator of family-owned soybean and corn farm.

VITAL SIGNS:
Temperature is 38 C

PHYSICAL EXAMINATION:
There is a 4-cm area of erythema, warmth, and purulent drainage with surrounding induration

LABORATORY DATA:
Laboratory testing reveals leukocytosis. Gram stain shows gram-positive cocci in clusters, and the organism is catalase-positive and coagulase-positive. Susceptibility testing shows resistance to amoxicillin and flucloxacillin but preserved sensitivity to vancomycin.

PLAN:
Incision and drainage are performed, and purulent fluid is sent for culture and sensitivity testing.

## Encounter 1: source 2020-07-15, practice 2025-08-21, ed, Emergency Medicine

Chief complaint: difficulty breathing, excessive salivation, and abdominal cramping after pesticide exposure

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. James Okafor, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Difficulty breathing, excessive salivation, and abdominal cramping after pesticide exposure

HISTORY OF PRESENT ILLNESS:
The patient is a 37-year-old male with seasonal allergic rhinitis and history of January hospitalization for right forearm infection, now presenting acutely. He reports spraying pesticides in his field earlier today when his protective mask broke. His wife noted increasing confusion and brought him to the hospital.

PAST MEDICAL HISTORY:
Active Problem List:
- Methicillin-resistant Staphylococcus aureus (MRSA) skin and soft tissue infection (diagnosed 2020-01-15)
- Seasonal allergic rhinitis

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy (age 16)

MEDICATIONS:
Home Medications:
- Cetirizine 10 mg orally daily as needed for allergic rhinitis

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertension
- Mother: Type 2 diabetes mellitus
- Paternal grandfather: Myocardial infarction (age 62)

SOCIAL HISTORY:
Smoking: Former smoker (quit 2018; 6 pack-year history). Alcohol: Social (1-2 drinks per week). Occupation: Medical Laboratory Technician at St. Luke's Regional Medical Center; part-time operator of family-owned soybean and corn farm.

VITAL SIGNS:
His vital signs are as follows: blood pressure 90/60 mmHg, heart rate 55 beats per minute, respiratory rate 22 breaths per minute, and temperature 37.2°C (99.0°F).

PHYSICAL EXAMINATION:
On examination, the patient is diaphoretic, has pinpoint pupils, and is experiencing muscle fasciculations.

LABORATORY DATA:
Arterial blood gas shows a pH of 7.35, PaCO2 of 45 mmHg, PaO2 of 85 mmHg, and HCO3- of 24 mEq/L.

## Encounter 2: source 2021-01-15, practice 2026-02-25, icu, Critical Care Medicine

Chief complaint: collapse and loss of consciousness after pesticide exposure

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Maria Rodriguez, MD
Department: Critical Care Medicine
Visit Type: icu

CHIEF COMPLAINT:
Collapse and loss of consciousness after pesticide exposure

HISTORY OF PRESENT ILLNESS:
The patient with seasonal allergic rhinitis returns six months after a prior pesticide exposure. His coworkers say he was spraying crops earlier in the day. The patient was diaphoretic and drooling prior to losing consciousness. He also vomited once.

PAST MEDICAL HISTORY:
Active Problem List:
- Methicillin-resistant Staphylococcus aureus (MRSA) skin and soft tissue infection (diagnosed 2020-01-15)
- Organophosphate poisoning (diagnosed 2020-07-15)
- Seasonal allergic rhinitis

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy (age 16)

MEDICATIONS:
Home Medications:
- Cetirizine 10 mg orally daily as needed for allergic rhinitis

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertension
- Mother: Type 2 diabetes mellitus
- Paternal grandfather: Myocardial infarction (age 62)

SOCIAL HISTORY:
Smoking: Former smoker (quit 2018; 6 pack-year history). Alcohol: Social (1-2 drinks per week). Occupation: Medical Laboratory Technician at St. Luke's Regional Medical Center; part-time operator of family-owned soybean and corn farm.

VITAL SIGNS:
Vitals: Temperature 36.7°C (98.1°F), blood pressure 108/68 mmHg, heart rate 52/min, respiratory rate 22/min, SpO₂ 94% on room air.

PHYSICAL EXAMINATION:
On physical examination, he is conscious but confused and twitching. His pupils are pinpoint, and he is salivating excessively. Lung auscultation reveals diffuse wheezing and increased bronchial secretions.

ASSESSMENT AND PLAN:
A toxidrome is suspected.

He is stabilized with supportive measures.
