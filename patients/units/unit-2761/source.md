# synthetic_hospital patient 2761

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 39 at first encounter, sex F, Non-Hispanic White, Private PPO (Blue Cross Blue Shield).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-05-05 (Wed) | outpatient | Internal Medicine |
| 1 | 2020-02-15 | 2021-06-10 (Thu) | inpatient | Obstetrics & Gynecology |
| 2 | 2020-07-15 | 2021-11-16 (Tue) | ed | Emergency Medicine |
| 3 | 2024-01-15 | 2025-05-04 (Sun) | ed | Emergency Medicine |
| 4 | 2024-07-15 | 2025-11-14 (Fri) | outpatient | Obstetrics & Gynecology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private PPO (Blue Cross Blue Shield)",
 "occupation": "International Business Consultant",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Mother: Deep vein thrombosis (age 45)",
  "Maternal aunt: Pulmonary embolism",
  "Paternal grandmother: Endometrial cancer (age 62)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never",
 "home_medications": [
  {
   "dose": "5 mg twice daily",
   "name": "Apixaban"
  },
  {
   "dose": "10 mg daily",
   "name": "Medroxyprogesterone acetate"
  },
  {
   "dose": "2000 units daily",
   "name": "Cholecalciferol (Vitamin D3)"
  },
  {
   "dose": "N/A",
   "name": "Ethinyl estradiol/norgestimate (discontinued 3 months prior)"
  }
 ],
 "surgical_history": [
  "Cesarean delivery (age 38)",
  "Postpartum pulmonary embolism (age 38)"
 ],
 "chronic_conditions": [
  "Chronic thromboembolic pulmonary hypertension",
  "History of recurrent venous thromboembolism",
  "Endometrial hyperplasia",
  "Vitamin D deficiency"
 ],
 "age_at_first_encounter": 39
}
```

Primary diagnoses: ["Pulmonary embolism", "Chronic thromboembolic pulmonary hypertension", "Endometrial Hyperplasia", "Vitamin D deficiency"]
Comorbidities: ["Delayed management of life-threatening respiratory failure", "Long-term oral contraceptive use"]

## Encounter 0: source 2020-01-15, practice 2021-05-05, outpatient, Internal Medicine

Chief complaint: Evaluation of fatigue and generalized body aches

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Evaluation of fatigue and generalized body aches

HISTORY OF PRESENT ILLNESS:
She works full-time in an office and reports spending little to no time outside during the day. She lives in a northern U.S. state and says she usually wears long sleeves, pants, and sunscreen when outdoors.
On further questioning, she notes a healthy appetite and consumes a diet rich in fatty fish, which she eats multiple times a week.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic thromboembolic pulmonary hypertension
- History of recurrent venous thromboembolism
- Endometrial hyperplasia
- Vitamin D deficiency

PAST SURGICAL HISTORY:
- Cesarean delivery (age 38)
- Postpartum pulmonary embolism (age 38)

MEDICATIONS:
She does not take any medications or supplements.

Home Medications:
- Apixaban 5 mg twice daily
- Medroxyprogesterone acetate 10 mg daily
- Cholecalciferol (Vitamin D3) 2000 units daily
- Ethinyl estradiol/norgestimate (discontinued 3 months prior) N/A

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Deep vein thrombosis (age 45)
- Maternal aunt: Pulmonary embolism
- Paternal grandmother: Endometrial cancer (age 62)

SOCIAL HISTORY:
Smoking: Never. Alcohol: Social (1-2 drinks per week). Occupation: International Business Consultant.

VITAL SIGNS:
Her BMI is 22 kg/m².

PHYSICAL EXAMINATION:
Physical examination reveals diffuse bone tenderness, especially over the shins and sternum, but no joint swelling or erythema. Neurologic exam is normal.

LABORATORY DATA:
Her liver and kidney function tests are within normal limits.

IMAGING:
Radiographic imaging of the long bones shows decreased bone density and cortical thinning.

PATHOLOGY:
A bone biopsy reveals unmineralized osteoid with normal collagen matrix.

PLAN:
She is started on a supplementation regimen and referred to endocrinology for further metabolic work-up.

## Encounter 1: source 2020-02-15, practice 2021-06-10, inpatient, Obstetrics & Gynecology

Chief complaint: New-onset left leg discomfort and sudden shortness of breath following cesarean delivery

ENCOUNTER NOTE
Date: 2020-02-15
Provider: Dr. Maria Gonzalez, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
New-onset left leg discomfort and sudden shortness of breath following cesarean delivery

HISTORY OF PRESENT ILLNESS:
The patient is a 38-year-old woman with chronic thromboembolic pulmonary hypertension, recurrent venous thromboembolism, and endometrial hyperplasia who was evaluated one month prior for fatigue and vitamin D deficiency. She is recovering in the hospital following an uncomplicated cesarean delivery performed two days ago. She had been ambulating slowly with assistance but remained largely bedridden since the procedure. On morning rounds, she complains of new-onset left leg discomfort, describing it as a sore, cramping sensation in the calf. A few hours later, she develops sudden-onset shortness of breath while attempting to use the bathroom. She is found sitting on the toilet, gasping for air and holding her chest. Nurses report that she was previously alert and oriented but now appears pale and mildly confused.

PAST MEDICAL HISTORY:
Active Problem List:
- Vitamin D deficiency (diagnosed 2020-01-15)
- Chronic thromboembolic pulmonary hypertension
- History of recurrent venous thromboembolism
- Endometrial hyperplasia

PAST SURGICAL HISTORY:
- Cesarean delivery (age 38)
- Postpartum pulmonary embolism (age 38)

MEDICATIONS:
Home Medications:
- Apixaban 5 mg twice daily
- Medroxyprogesterone acetate 10 mg daily
- Cholecalciferol (Vitamin D3) 2000 units daily
- Ethinyl estradiol/norgestimate (discontinued 3 months prior) N/A

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Deep vein thrombosis (age 45)
- Maternal aunt: Pulmonary embolism
- Paternal grandmother: Endometrial cancer (age 62)

SOCIAL HISTORY:
Smoking: Never. Alcohol: Social (1-2 drinks per week). Occupation: International Business Consultant.

VITAL SIGNS:
Vitals reveal a heart rate of 118/min, respiratory rate of 30/min, blood pressure of 106/72 mm Hg, and oxygen saturation of 91% on room air. Temperature is 37.1°C (98.8°F).

PHYSICAL EXAMINATION:
On exam, she is tachypneic, anxious, and diaphoretic. Her left leg appears mildly swollen and tender to palpation compared to the right. Jugular venous pressure is normal, and there are no rales or wheezing on auscultation. There is no chest wall tenderness, and heart sounds are rapid but regular.

LABORATORY DATA:
ABG is performed to evaluate her respiratory status.

IMAGING:
Chest x-ray is unremarkable.

OTHER STUDIES:
ECG shows sinus tachycardia.

## Encounter 2: source 2020-07-15, practice 2021-11-16, ed, Emergency Medicine

Chief complaint: Sudden-onset shortness of breath and sharp pleuritic chest pain

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. James Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden-onset shortness of breath and sharp pleuritic chest pain

HISTORY OF PRESENT ILLNESS:
The patient with chronic thromboembolic pulmonary hypertension presents five months after her prior pulmonary embolism with sudden-onset shortness of breath. She also reports sharp chest pain worsening when taking a deep breath. She recently returned from a transatlantic flight and has been using oral contraceptive pills for the past year. She denies cough, fever, or recent illness.

PAST MEDICAL HISTORY:
Active Problem List:
- Vitamin D deficiency (diagnosed 2020-01-15)
- Pulmonary embolism (diagnosed 2020-02-15)
- Chronic thromboembolic pulmonary hypertension
- History of recurrent venous thromboembolism
- Endometrial hyperplasia

PAST SURGICAL HISTORY:
- Cesarean delivery (age 38)
- Postpartum pulmonary embolism (age 38)

MEDICATIONS:
Home Medications:
- Apixaban 5 mg twice daily
- Medroxyprogesterone acetate 10 mg daily
- Cholecalciferol (Vitamin D3) 2000 units daily
- Ethinyl estradiol/norgestimate (discontinued 3 months prior) N/A

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Deep vein thrombosis (age 45)
- Maternal aunt: Pulmonary embolism
- Paternal grandmother: Endometrial cancer (age 62)

SOCIAL HISTORY:
Smoking: Never. Alcohol: Social (1-2 drinks per week). Occupation: International Business Consultant.

VITAL SIGNS:
Respiratory rate is 24/min, heart rate is 110/min, and blood pressure is 108/72 mm Hg. Oxygen saturation is 93% on room air

PHYSICAL EXAMINATION:
On examination, she appears mildly anxious, Cardiopulmonary examination reveals clear breath sounds and no murmurs. There is mild tenderness in the right calf without swelling.

IMAGING:
Chest x-ray is unremarkable.

## Encounter 3: source 2024-01-15, practice 2025-05-04, ed, Emergency Medicine

Chief complaint: Sudden shortness of breath and chest discomfort worsening with inspiration

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Emily Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden shortness of breath and chest discomfort worsening with inspiration

HISTORY OF PRESENT ILLNESS:
This 43-year-old female with chronic thromboembolic pulmonary hypertension and recurrent venous thromboembolism presents with sudden shortness of breath and chest discomfort worsening with inspiration. She returned from an international flight two days ago and has been using oral contraceptive pills.

PAST MEDICAL HISTORY:
Active Problem List:
- Vitamin D deficiency (diagnosed 2020-01-15)
- Pulmonary embolism (diagnosed 2020-02-15)
- Chronic thromboembolic pulmonary hypertension (diagnosed 2020-07-15)
- History of recurrent venous thromboembolism
- Endometrial hyperplasia

PAST SURGICAL HISTORY:
- Cesarean delivery (age 38)
- Postpartum pulmonary embolism (age 38)

MEDICATIONS:
Home Medications:
- Apixaban 5 mg twice daily
- Medroxyprogesterone acetate 10 mg daily
- Cholecalciferol (Vitamin D3) 2000 units daily
- Ethinyl estradiol/norgestimate (discontinued 3 months prior) N/A

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Deep vein thrombosis (age 45)
- Maternal aunt: Pulmonary embolism
- Paternal grandmother: Endometrial cancer (age 62)

SOCIAL HISTORY:
Smoking: Never. Alcohol: Social (1-2 drinks per week). Occupation: International Business Consultant.

VITAL SIGNS:
On examination, her temperature is 37.0°C (98.6°F), pulse is 112/min, blood pressure is 108/72 mmHg, respiratory rate is 24/min, and oxygen saturation is 93% on room air.

PHYSICAL EXAMINATION:
She appears mildly anxious and is breathing rapidly. Cardiopulmonary examination is normal.

OTHER STUDIES:
An ECG shows sinus tachycardia.

## Encounter 4: source 2024-07-15, practice 2025-11-14, outpatient, Obstetrics & Gynecology

Chief complaint: Intermittent light vaginal bleeding over the past six weeks

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Aisha Patel, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent light vaginal bleeding over the past six weeks

HISTORY OF PRESENT ILLNESS:
The patient is a 43-year-old female with chronic thromboembolic pulmonary hypertension and recurrent venous thromboembolism who returns for evaluation of intermittent light vaginal bleeding. She reports that her menstrual cycles had been regular while using a hormonal contraceptive for several years, but she decided to stop part of her regimen about three months ago due to perceived side effects. Since then, her periods have become unpredictable, with occasional mid-cycle spotting. She denies pelvic pain, discharge, or fever.

PAST MEDICAL HISTORY:
Active Problem List:
- Vitamin D deficiency (diagnosed 2020-01-15)
- Pulmonary embolism (diagnosed 2020-02-15)
- Chronic thromboembolic pulmonary hypertension (diagnosed 2020-07-15)
- History of recurrent venous thromboembolism
- Endometrial hyperplasia

PAST SURGICAL HISTORY:
- Cesarean delivery (age 38)
- Postpartum pulmonary embolism (age 38)

MEDICATIONS:
Home Medications:
- Apixaban 5 mg twice daily
- Medroxyprogesterone acetate 10 mg daily
- Cholecalciferol (Vitamin D3) 2000 units daily
- Ethinyl estradiol/norgestimate (discontinued 3 months prior) N/A

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Deep vein thrombosis (age 45)
- Maternal aunt: Pulmonary embolism
- Paternal grandmother: Endometrial cancer (age 62)

SOCIAL HISTORY:
Smoking: Never. Alcohol: Social (1-2 drinks per week). Occupation: International Business Consultant.

VITAL SIGNS:
Her BMI is 23. Vital signs are normal.

PHYSICAL EXAMINATION:
A pelvic examination is unremarkable.

IMAGING:
Transvaginal ultrasound reveals an endometrial stripe measuring 13 mm.

PATHOLOGY:
Endometrial sampling is obtained and shows glandular proliferation without atypia.
