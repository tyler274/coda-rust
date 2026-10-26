# synthetic_hospital patient 2561

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 23 at first encounter, sex M, Non-Hispanic White, Employer-sponsored PPO (Blue Cross Blue Shield).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-06-18 (Fri) | outpatient | Internal Medicine |
| 1 | 2020-03-15 | 2021-08-23 (Mon) | outpatient | Allergy and Immunology |
| 2 | 2021-01-15 | 2022-06-20 (Mon) | outpatient | Pulmonology |
| 3 | 2022-01-15 | 2023-06-19 (Mon) | outpatient | Internal Medicine |
| 4 | 2023-07-15 | 2024-12-07 (Sat) | icu | Emergency Medicine |
| 5 | 2025-01-15 | 2026-06-19 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Employer-sponsored PPO (Blue Cross Blue Shield)",
 "occupation": "Software Engineer",
 "alcohol_use": "Social (weekends, 2-3 drinks)",
 "family_history": [
  "Father: Alpha-1 antitrypsin deficiency (PiZZ) with emphysema",
  "Mother: Hereditary angioedema type I",
  "Paternal uncle: Neurofibromatosis type 1",
  "Maternal grandfather: Hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [
  "Posterior spinal fusion T5-T9 with decompressive laminectomy (complicated by neurogenic shock following motorcycle crash)",
  "Left video-assisted thoracoscopic surgery (VATS) with pulmonary hematoma evacuation and rib fixation (secondary to high-speed motor vehicle collision with pulmonary contusion)",
  "Right radical inguinal orchiectomy (for testicular malignancy)"
 ],
 "chronic_conditions": [
  "Alpha-1 antitrypsin deficiency (PiZZ phenotype) with early-onset panacinar emphysema",
  "Hereditary angioedema type I (C1 esterase inhibitor deficiency)",
  "Neurofibromatosis type 1 (von Recklinghausen disease)",
  "Testicular non-seminomatous germ cell tumor (stage IIB) with paraneoplastic hypercalcemia"
 ],
 "age_at_first_encounter": 23
}
```

Primary diagnoses: ["Alpha-1 antitrypsin deficiency", "Distributive shock (neurogenic or septic)", "Pulmonary contusion with respiratory compromise", "Hereditary Angioedema (C1 Inhibitor Deficiency)", "Neurofibromatosis type 1", "Malignancy-associated hypercalcemia"]
Comorbidities: ["Hypoxemia", "Family History of Hereditary Hemorrhagic Telangiectasia"]

## Encounter 0: source 2020-01-15, practice 2021-06-18, outpatient, Internal Medicine

Chief complaint: presents for a routine exam

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
He presents for a routine exam

PAST MEDICAL HISTORY:
Active Problem List:
- Alpha-1 antitrypsin deficiency (PiZZ phenotype) with early-onset panacinar emphysema
- Hereditary angioedema type I (C1 esterase inhibitor deficiency)
- Neurofibromatosis type 1 (von Recklinghausen disease)
- Testicular non-seminomatous germ cell tumor (stage IIB) with paraneoplastic hypercalcemia

PAST SURGICAL HISTORY:
- Posterior spinal fusion T5-T9 with decompressive laminectomy (complicated by neurogenic shock following motorcycle crash)
- Left video-assisted thoracoscopic surgery (VATS) with pulmonary hematoma evacuation and rib fixation (secondary to high-speed motor vehicle collision with pulmonary contusion)
- Right radical inguinal orchiectomy (for testicular malignancy)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
He reports that his mother and maternal uncle had similar skin findings, but his father does not.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (weekends, 2-3 drinks). Occupation: Software Engineer.

PHYSICAL EXAMINATION:
Physical exam reveals multiple flat, hyperpigmented macules on his trunk and limbs, as well as clusters of small brown lesions in his axillae and groin.
There are no subcutaneous nodules or oral mucosal lesions.
Slit lamp examination is shown below.

## Encounter 1: source 2020-03-15, practice 2021-08-23, outpatient, Allergy and Immunology

Chief complaint: presents to the clinic with a history of multiple episodes over the past year involving localized swelling of his lips, hands, and genitals

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. James Rodriguez, MD
Department: Allergy and Immunology
Visit Type: outpatient

CHIEF COMPLAINT:
He presents to the clinic with a history of multiple episodes over the past year involving localized swelling of his lips, hands, and genitals

HISTORY OF PRESENT ILLNESS:
The patient with known hereditary angioedema type I presents to the allergy and immunology clinic for evaluation of recurrent swelling episodes following a routine internal medicine visit two months ago. Each episode develops over several hours and resolves spontaneously within 2–3 days. He also describes three prior episodes of severe, cramping abdominal pain with bloating and watery diarrhea, during which he was afebrile and had no blood in his stool. He has no history of rash, wheezing, or pruritus. There is no known food allergy or exposure to new medications.

PAST MEDICAL HISTORY:
Active Problem List:
- Neurofibromatosis type 1 (diagnosed 2020-01-15)
- Alpha-1 antitrypsin deficiency (PiZZ phenotype) with early-onset panacinar emphysema
- Hereditary angioedema type I (C1 esterase inhibitor deficiency)
- Neurofibromatosis type 1 (von Recklinghausen disease)
- Testicular non-seminomatous germ cell tumor (stage IIB) with paraneoplastic hypercalcemia

PAST SURGICAL HISTORY:
- Posterior spinal fusion T5-T9 with decompressive laminectomy (complicated by neurogenic shock following motorcycle crash)
- Left video-assisted thoracoscopic surgery (VATS) with pulmonary hematoma evacuation and rib fixation (secondary to high-speed motor vehicle collision with pulmonary contusion)
- Right radical inguinal orchiectomy (for testicular malignancy)

MEDICATIONS:
He takes no daily medications

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
His mother and maternal uncle have a history of unexplained episodes of facial swelling during their youth.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (weekends, 2-3 drinks). Occupation: Software Engineer.

VITAL SIGNS:
Vital signs are stable.

PHYSICAL EXAMINATION:
Physical exam today is unremarkable.

## Encounter 2: source 2021-01-15, practice 2022-06-20, outpatient, Pulmonology

Chief complaint: gradually worsening shortness of breath

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Emily Watson, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Gradually worsening shortness of breath

HISTORY OF PRESENT ILLNESS:
This 24-year-old male with Alpha-1 antitrypsin deficiency, neurofibromatosis type 1, and hereditary angioedema returns to the pulmonology clinic for follow-up. He reports that over the past 2 years, he has noticed decreased exercise tolerance and now becomes winded after walking up a single flight of stairs. He denies wheezing, chest pain, or cough.

PAST MEDICAL HISTORY:
Active Problem List:
- Neurofibromatosis type 1 (diagnosed 2020-01-15)
- Hereditary Angioedema (C1 Inhibitor Deficiency) (diagnosed 2020-03-15)
- Alpha-1 antitrypsin deficiency (PiZZ phenotype) with early-onset panacinar emphysema
- Hereditary angioedema type I (C1 esterase inhibitor deficiency)
- Neurofibromatosis type 1 (von Recklinghausen disease)
- Testicular non-seminomatous germ cell tumor (stage IIB) with paraneoplastic hypercalcemia

PAST SURGICAL HISTORY:
- Posterior spinal fusion T5-T9 with decompressive laminectomy (complicated by neurogenic shock following motorcycle crash)
- Left video-assisted thoracoscopic surgery (VATS) with pulmonary hematoma evacuation and rib fixation (secondary to high-speed motor vehicle collision with pulmonary contusion)
- Right radical inguinal orchiectomy (for testicular malignancy)

MEDICATIONS:
He takes no medications

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
Family history is notable for his father, who died at age 45 from complications of liver failure related to long-term alcohol use.

SOCIAL HISTORY:
He works at a software company and has never smoked or used recreational drugs. He drinks socially on weekends. He has not traveled recently.

VITAL SIGNS:
Pulse oximetry shows 92% saturation on room air.

PHYSICAL EXAMINATION:
On physical examination, the patient appears thin and mildly tachypneic at rest. Lung examination reveals decreased breath sounds and prolonged expiration without wheezing or rales. There is mild hyperresonance to percussion bilaterally, and his chest appears slightly increased in anteroposterior diameter. No digital clubbing is observed. Cardiovascular and abdominal exams are normal, and there is no lower extremity edema.

IMAGING:
A chest x-ray reveals bilateral hyperlucency of the lung fields and several large bullae in the lower lobes.

ASSESSMENT AND PLAN:
His workup is pending, but based on the presentation and imaging findings, his clinician explains that certain measurable parameters of lung physiology are already likely to be abnormal.

The patient is referred for further testing and genetic counseling. He is worried about his long-term prognosis and asks how this condition affects gas exchange.

## Encounter 3: source 2022-01-15, practice 2023-06-19, outpatient, Internal Medicine

Chief complaint: comes to the clinic for routine follow-up after abnormal screening labs

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Michael Park, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
He comes to the clinic for routine follow-up after abnormal screening labs

HISTORY OF PRESENT ILLNESS:
He presents for follow-up of NF1, HAE, and AAT deficiency. He feels well and has no symptoms. He recently underwent biochemical screening as part of a new employee health package.

PAST MEDICAL HISTORY:
Active Problem List:
- Neurofibromatosis type 1 (diagnosed 2020-01-15)
- Hereditary Angioedema (C1 Inhibitor Deficiency) (diagnosed 2020-03-15)
- Alpha-1 antitrypsin deficiency (diagnosed 2021-01-15)
- Alpha-1 antitrypsin deficiency (PiZZ phenotype) with early-onset panacinar emphysema
- Hereditary angioedema type I (C1 esterase inhibitor deficiency)
- Neurofibromatosis type 1 (von Recklinghausen disease)
- Testicular non-seminomatous germ cell tumor (stage IIB) with paraneoplastic hypercalcemia

PAST SURGICAL HISTORY:
- Posterior spinal fusion T5-T9 with decompressive laminectomy (complicated by neurogenic shock following motorcycle crash)
- Left video-assisted thoracoscopic surgery (VATS) with pulmonary hematoma evacuation and rib fixation (secondary to high-speed motor vehicle collision with pulmonary contusion)
- Right radical inguinal orchiectomy (for testicular malignancy)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
He has no significant family history of endocrinopathies, though his father reportedly has "mildly high calcium that doctors have never worried about."

SOCIAL HISTORY:
He exercises regularly

REVIEW OF SYSTEMS:
The patient denies polyuria, polydipsia, nephrolithiasis, or fractures.

LABORATORY DATA:
His creatinine is normal.

OTHER STUDIES:
A genetic test confirms a mutation affecting calcium homeostasis.

## Encounter 4: source 2023-07-15, practice 2024-12-07, icu, Emergency Medicine

Chief complaint: brought to the emergency department after a motorcycle accident

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Jessica Thompson, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Brought to the emergency department after a motorcycle accident

HISTORY OF PRESENT ILLNESS:
With alpha-1 antitrypsin deficiency, hereditary angioedema, and neurofibromatosis, he was not wearing a helmet and was thrown from his motorcycle, landing on his back.

PAST MEDICAL HISTORY:
Active Problem List:
- Neurofibromatosis type 1 (diagnosed 2020-01-15)
- Hereditary Angioedema (C1 Inhibitor Deficiency) (diagnosed 2020-03-15)
- Alpha-1 antitrypsin deficiency (diagnosed 2021-01-15)
- Malignancy-associated hypercalcemia (diagnosed 2022-01-15)
- Alpha-1 antitrypsin deficiency (PiZZ phenotype) with early-onset panacinar emphysema
- Hereditary angioedema type I (C1 esterase inhibitor deficiency)
- Neurofibromatosis type 1 (von Recklinghausen disease)
- Testicular non-seminomatous germ cell tumor (stage IIB) with paraneoplastic hypercalcemia

PAST SURGICAL HISTORY:
- Posterior spinal fusion T5-T9 with decompressive laminectomy (complicated by neurogenic shock following motorcycle crash)
- Left video-assisted thoracoscopic surgery (VATS) with pulmonary hematoma evacuation and rib fixation (secondary to high-speed motor vehicle collision with pulmonary contusion)
- Right radical inguinal orchiectomy (for testicular malignancy)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Alpha-1 antitrypsin deficiency (PiZZ) with emphysema
- Mother: Hereditary angioedema type I
- Paternal uncle: Neurofibromatosis type 1
- Maternal grandfather: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (weekends, 2-3 drinks). Occupation: Software Engineer.

VITAL SIGNS:
On arrival, his vital signs are: blood pressure 85/50 mmHg, heart rate 48 beats per minute, respiratory rate 22 breaths per minute, and temperature 35.6°C. The patient's breathing is shallow, and oxygen saturation is 92% on room air.

PHYSICAL EXAMINATION:
Physical examination reveals flaccid paralysis and loss of sensation below the level of the nipples. The patient is also noted to have a distended bladder. An indwelling urinary catheter is placed, and 800 mL of urine is drained.

IMAGING:
A CT scan of the spine shows a fracture at the T4 vertebral level with evidence of spinal cord injury.

## Encounter 5: source 2025-01-15, practice 2026-06-19, ed, Emergency Medicine

Chief complaint: difficulty breathing following high-speed motor vehicle collision

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. David Kumar, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Difficulty breathing following high-speed motor vehicle collision

HISTORY OF PRESENT ILLNESS:
The patient, with Alpha-1 antitrypsin deficiency, hereditary angioedema type I, neurofibromatosis type 1, and testicular cancer, presents to the emergency department following a motor vehicle collision. He was the restrained driver involved in a high-speed crash. On arrival, he is alert but appears anxious and is complaining of

PAST MEDICAL HISTORY:
Active Problem List:
- Neurofibromatosis type 1 (diagnosed 2020-01-15)
- Hereditary Angioedema (C1 Inhibitor Deficiency) (diagnosed 2020-03-15)
- Alpha-1 antitrypsin deficiency (diagnosed 2021-01-15)
- Malignancy-associated hypercalcemia (diagnosed 2022-01-15)
- Distributive shock (neurogenic or septic) (diagnosed 2023-07-15)
- Alpha-1 antitrypsin deficiency (PiZZ phenotype) with early-onset panacinar emphysema
- Hereditary angioedema type I (C1 esterase inhibitor deficiency)
- Neurofibromatosis type 1 (von Recklinghausen disease)
- Testicular non-seminomatous germ cell tumor (stage IIB) with paraneoplastic hypercalcemia

PAST SURGICAL HISTORY:
- Posterior spinal fusion T5-T9 with decompressive laminectomy (complicated by neurogenic shock following motorcycle crash)
- Left video-assisted thoracoscopic surgery (VATS) with pulmonary hematoma evacuation and rib fixation (secondary to high-speed motor vehicle collision with pulmonary contusion)
- Right radical inguinal orchiectomy (for testicular malignancy)

MEDICATIONS:
He takes no medications

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Alpha-1 antitrypsin deficiency (PiZZ) with emphysema
- Mother: Hereditary angioedema type I
- Paternal uncle: Neurofibromatosis type 1
- Maternal grandfather: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (weekends, 2-3 drinks). Occupation: Software Engineer.

VITAL SIGNS:
Blood pressure 130/85 mmHg, heart rate 110 beats per minute, respiratory rate 28 breaths per minute, and oxygen saturation 89% on room air

PHYSICAL EXAMINATION:
No external signs of chest wall injury, but decreased breath sounds are noted on the left side of the chest

LABORATORY DATA:
Arterial blood gas on room air shows:
- pH: 7.45
- PaCO2: 32 mmHg
- PaO2: 60 mmHg
- HCO3-: 24 mEq/L

IMAGING:
A chest X-ray is performed and shows patchy, irregular alveolar infiltrates in the left lung field without evidence of rib fractures
