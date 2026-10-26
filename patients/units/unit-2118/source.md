# synthetic_hospital patient 2118

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 42 at first encounter, sex M, Non-Hispanic White, Employer-sponsored PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-03-04 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-05-15 | 2024-06-19 (Wed) | ed | Emergency Medicine |
| 2 | 2021-05-15 | 2025-06-23 (Mon) | ed | Emergency Medicine |
| 3 | 2022-05-15 | 2026-06-20 (Sat) | icu | Infectious Disease |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Employer-sponsored PPO",
 "occupation": "International Business Development Manager",
 "alcohol_use": "Social drinker (2-3 drinks per week)",
 "family_history": [
  "Father with bipolar disorder",
  "Mother with hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "300 mg twice daily",
   "name": "Lithium carbonate"
  }
 ],
 "surgical_history": [
  "Appendectomy at age 18"
 ],
 "chronic_conditions": [
  "Chronic kidney disease",
  "Bipolar disorder"
 ],
 "age_at_first_encounter": 42
}
```

Primary diagnoses: ["Hyperchloremic metabolic acidosis due to excess normal saline infusion", "Nephrogenic Diabetes Insipidus", "Ebola virus disease", "Ciguatera poisoning"]
Comorbidities: ["Dehydration"]

## Encounter 0: source 2020-01-15, practice 2024-03-04, outpatient, Internal Medicine

Chief complaint: persistent excessive thirst and frequent urination

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent excessive thirst and frequent urination

HISTORY OF PRESENT ILLNESS:
He comes to the clinic due to persistent excessive thirst and frequent urination that has worsened over the past several months. He reports getting up multiple times during the night to urinate and feeling constantly fatigued despite sleeping adequately. He drinks approximately 4-5 liters of water daily and carries a bottle with him at work to avoid dehydration symptoms such as dizziness and dry mouth. He denies recent travel, vomiting, or diarrhea.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic kidney disease
- Bipolar disorder

His past medical history is significant for bipolar disorder,.

PAST SURGICAL HISTORY:
- Appendectomy at age 18

MEDICATIONS:
He takes for which he has been taking lithium for the past five years with stable mood control.

Home Medications:
- Lithium carbonate 300 mg twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with bipolar disorder
- Mother with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social drinker (2-3 drinks per week). Occupation: International Business Development Manager.

VITAL SIGNS:
On examination, temperature is 36.7°C (98.1°F), blood pressure is 124/78 mmHg, pulse is 76/min, and BMI is 23 kg/m².

PHYSICAL EXAMINATION:
He does not have any mental status changes.
Physical examination is otherwise unremarkable except for mild dry mucous membranes.

LABORATORY DATA:
Further evaluation reveals that despite elevated serum osmolality, his urine remains inappropriately dilute.

OTHER STUDIES:
A trial of desmopressin produces minimal change in his urinary concentration.

## Encounter 1: source 2020-05-15, practice 2024-06-19, ed, Emergency Medicine

Chief complaint: nausea, vomiting, and diarrhea with pins and needles sensation

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Nausea, vomiting, and diarrhea with pins and needles sensation

HISTORY OF PRESENT ILLNESS:
The patient is a 42-year-old male with chronic kidney disease and bipolar disorder maintained on lithium carbonate who was previously evaluated in January 2020 for persistent excessive thirst and frequent urination, now presenting with nausea, vomiting, and diarrhea that began 3 hours after eating grilled fish at a beachside restaurant while vacationing in the Bahamas. He states the gastrointestinal symptoms were initially most prominent, but over the past few hours he has also developed a strange sensation in his hands and feet, describing it as "pins and needles." He adds that touching cold water feels like it is burning hot, and hot objects feel cold to the touch. He denies fever or recent travel outside the Caribbean. His travel companion ate a different meal and remains asymptomatic.

PAST MEDICAL HISTORY:
Active Problem List:
- Nephrogenic Diabetes Insipidus (diagnosed 2020-01-15)
- Chronic kidney disease
- Bipolar disorder

PAST SURGICAL HISTORY:
- Appendectomy at age 18

MEDICATIONS:
Home Medications:
- Lithium carbonate 300 mg twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with bipolar disorder
- Mother with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social drinker (2-3 drinks per week). Occupation: International Business Development Manager.

VITAL SIGNS:
His vital signs are stable.

PHYSICAL EXAMINATION:
On examination, he appears mildly dehydrated but is alert and oriented. Neurologic exam reveals intact cranial nerves and normal motor strength, but decreased sensation to light touch in a stocking-glove distribution. There is no rash or muscle rigidity.

LABORATORY DATA:
Basic labs, including CBC and electrolytes, are within normal limits.

## Encounter 2: source 2021-05-15, practice 2025-06-23, ed, Emergency Medicine

Chief complaint: weakness and fatigue for the past two days

ENCOUNTER NOTE
Date: 2021-05-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Weakness and fatigue for the past two days

HISTORY OF PRESENT ILLNESS:
This patient with chronic kidney disease and bipolar disorder on lithium and nephrogenic diabetes insipidus presents to the emergency department with complaints of weakness and fatigue for two days. He has been receiving intravenous fluids at home for dehydration due to gastroenteritis.

PAST MEDICAL HISTORY:
Active Problem List:
- Nephrogenic Diabetes Insipidus (diagnosed 2020-01-15)
- Ciguatera poisoning (diagnosed 2020-05-15)
- Chronic kidney disease
- Bipolar disorder

PAST SURGICAL HISTORY:
- Appendectomy at age 18

MEDICATIONS:
Home Medications:
- Lithium carbonate 300 mg twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with bipolar disorder
- Mother with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social drinker (2-3 drinks per week). Occupation: International Business Development Manager.

VITAL SIGNS:
On examination, his blood pressure is 130/85 mmHg, heart rate is 88 beats per minute, and he is afebrile

PHYSICAL EXAMINATION:
His mucous membranes are moist, and skin turgor is normal.

LABORATORY DATA:
Laboratory results are as follows:

- Sodium (Na): 140 mEq/L
- Potassium (K): 3.9 mEq/L
- Chloride (Cl): 115 mEq/L
- Bicarbonate (HCO3): 18 mEq/L
- Blood urea nitrogen (BUN): 29 mg/dL
- Creatinine: 2.1 mg/dL
- Glucose: 98 mg/dL
- Arterial blood gas (ABG): pH 7.32, pCO2 40 mmHg, pO2 98 mmHg

## Encounter 3: source 2022-05-15, practice 2026-06-20, icu, Infectious Disease

Chief complaint: fever, diffuse abdominal pain, and multiple episodes of non-bloody vomiting and watery diarrhea

ENCOUNTER NOTE
Date: 2022-05-15
Provider: Dr. Robert Kim, MD
Department: Infectious Disease
Visit Type: icu

CHIEF COMPLAINT:
Fever, diffuse abdominal pain, and multiple episodes of non-bloody vomiting and watery diarrhea

HISTORY OF PRESENT ILLNESS:
This 44-year-old male with chronic kidney disease and bipolar disorder maintained on lithium carbonate, with a history of nephrogenic diabetes insipidus, ciguatera poisoning, and hyperchloremic metabolic acidosis, presents with acute symptoms. He also reports muscle aches, malaise, and a sore throat. He returned to the United States one week ago after a month-long business trip to Guinea in West Africa, during which he visited both rural and urban areas. While abroad, he recalls attending a local funeral where he had close contact with family members of the deceased and helped prepare the body. He denies consuming undercooked meat or exposure to freshwater lakes.

PAST MEDICAL HISTORY:
Active Problem List:
- Nephrogenic Diabetes Insipidus (diagnosed 2020-01-15)
- Ciguatera poisoning (diagnosed 2020-05-15)
- Hyperchloremic metabolic acidosis due to excess normal saline infusion (diagnosed 2021-05-15)
- Chronic kidney disease
- Bipolar disorder

PAST SURGICAL HISTORY:
- Appendectomy at age 18

MEDICATIONS:
Home Medications:
- Lithium carbonate 300 mg twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with bipolar disorder
- Mother with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social drinker (2-3 drinks per week). Occupation: International Business Development Manager.

VITAL SIGNS:
His temperature is 39.2°C (102.6°F), heart rate is 110/min, blood pressure is 96/64 mm Hg, and respiratory rate is 22/min.

PHYSICAL EXAMINATION:
On examination, the patient appears acutely ill and dehydrated. Conjunctival injection and mild pharyngeal erythema are noted. The abdomen is diffusely tender without rebound or guarding. No rash or bleeding is observed.

LABORATORY DATA:
Laboratory studies reveal thrombocytopenia, elevated liver enzymes, and mild leukopenia.

ASSESSMENT AND PLAN:
The clinical team suspects a viral hemorrhagic fever and initiates supportive management while awaiting confirmation.
