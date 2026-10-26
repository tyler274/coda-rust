# synthetic_hospital patient 1754

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 35 at first encounter, sex F, Hispanic, Private insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-12-04 (Fri) | outpatient | Pulmonology |
| 1 | 2020-04-15 | 2021-03-17 (Wed) | outpatient | Pulmonology |
| 2 | 2020-10-15 | 2021-09-20 (Mon) | outpatient | Obstetrics & Gynecology |
| 3 | 2021-04-15 | 2022-03-09 (Wed) | outpatient | Internal Medicine |
| 4 | 2021-07-15 | 2022-06-05 (Sun) | ed | Emergency Medicine |
| 5 | 2022-07-15 | 2023-06-22 (Thu) | inpatient | Obstetrics & Gynecology |
| 6 | 2023-11-15 | 2024-10-21 (Mon) | inpatient | Obstetrics & Gynecology |
| 7 | 2024-11-15 | 2025-10-06 (Mon) | icu | Pulmonary & Critical Care |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Private insurance",
 "occupation": "Elementary school teacher",
 "alcohol_use": "Social",
 "family_history": [
  "Mother with asthma",
  "Father with type 2 diabetes mellitus",
  "Maternal history of preeclampsia"
 ],
 "race_ethnicity": "Hispanic",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "100/50 mcg inhaled twice daily",
   "name": "Fluticasone/Salmeterol"
  },
  {
   "dose": "90 mcg inhaled every 4-6 hours as needed",
   "name": "Albuterol"
  },
  {
   "dose": "1000 mg by mouth twice daily",
   "name": "Metformin"
  },
  {
   "dose": "20 mg by mouth once daily",
   "name": "Omeprazole"
  }
 ],
 "surgical_history": [
  "Cesarean section x2",
  "Dilation and curettage for missed abortion"
 ],
 "chronic_conditions": [
  "Asthma",
  "Type 2 diabetes mellitus",
  "Gastroesophageal reflux disease",
  "History of gestational diabetes"
 ],
 "age_at_first_encounter": 35
}
```

Primary diagnoses: ["Hypotension due to neuraxial anesthesia", "Asthma", "Status asthmaticus with impending respiratory failure", "Gastroesophageal reflux disease", "Oral candidiasis", "Placenta accreta", "Cutaneous candidiasis", "Hyponatremia"]
Comorbidities: ["History of previous cesarean delivery", "Asthma", "Gestational diabetes mellitus"]

## Encounter 0: source 2020-01-15, practice 2020-12-04, outpatient, Pulmonology

Chief complaint: Episodic shortness of breath, chest tightness, and wheezing

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Episodic shortness of breath, chest tightness, and wheezing

HISTORY OF PRESENT ILLNESS:
A 35-year-old female presents to the clinic with a 6-month history of episodic shortness of breath, chest tightness, and wheezing. These symptoms often occur at night and sometimes during exercise.

PAST MEDICAL HISTORY:
Active Problem List:
- Asthma
- Type 2 diabetes mellitus
- Gastroesophageal reflux disease
- History of gestational diabetes

PAST SURGICAL HISTORY:
- Cesarean section x2
- Dilation and curettage for missed abortion

MEDICATIONS:
She is.

Home Medications:
- Fluticasone/Salmeterol 100/50 mcg inhaled twice daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed
- Metformin 1000 mg by mouth twice daily
- Omeprazole 20 mg by mouth once daily

ALLERGIES:
She has no known allergies

FAMILY HISTORY:
- Mother with asthma
- Father with type 2 diabetes mellitus
- Maternal history of preeclampsia

SOCIAL HISTORY:
She does not smoke

VITAL SIGNS:
On physical examination, her respiratory rate is 16 breaths per minute, heart rate is 72 beats per minute, blood pressure is 125/78 mmHg, and oxygen saturation is 98% on room air

PHYSICAL EXAMINATION:
Lung auscultation reveals no wheezing or other abnormal sounds at rest.

OTHER STUDIES:
Baseline spirometry is normal with a forced expiratory volume in 1 second (FEV1) of 2.8 L (80% of predicted) and a forced vital capacity (FVC) of 3.5 L (85% of predicted). The patient undergoes a methacholine bronchoprovocation test, which shows a 20% reduction in FEV1 at a concentration of 8 mg/mL of methacholine.

## Encounter 1: source 2020-04-15, practice 2021-03-17, outpatient, Pulmonology

Chief complaint: Worsening shortness of breath, especially at night, with chronic cough

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening shortness of breath, especially at night, with chronic cough

HISTORY OF PRESENT ILLNESS:
The patient returns for pulmonology follow-up three months after her initial evaluation for episodic shortness of breath, chest tightness, and wheezing. She reports a chronic cough often worse when lying down and a burning sensation in her chest after meals. Despite using her albuterol inhaler more frequently, she has not experienced significant relief. She denies any recent upper respiratory infections or changes in her environment.

PAST MEDICAL HISTORY:
Active Problem List:
- Asthma (diagnosed 2020-01-15)
- Type 2 diabetes mellitus
- Gastroesophageal reflux disease
- History of gestational diabetes

PAST SURGICAL HISTORY:
- Cesarean section x2
- Dilation and curettage for missed abortion

MEDICATIONS:
Her asthma medications include a low-dose inhaled corticosteroid and a long-acting beta-agonist.

Home Medications:
- Fluticasone/Salmeterol 100/50 mcg inhaled twice daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed
- Metformin 1000 mg by mouth twice daily
- Omeprazole 20 mg by mouth once daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with asthma
- Father with type 2 diabetes mellitus
- Maternal history of preeclampsia

SOCIAL HISTORY:
She has no history of smoking or occupational exposures.

VITAL SIGNS:
Her BMI is 27 kg/m^2.

PHYSICAL EXAMINATION:
On physical examination, her lung auscultation reveals mild expiratory wheezing bilaterally. Her peak expiratory flow rate is noted to be 15% lower than her personal best.

LABORATORY DATA:
Laboratory Values:
- Complete Blood Count (CBC): Within normal limits
- Spirometry: Mild obstruction without significant reversibility post-bronchodilator
- Esophageal pH monitoring: Not performed

## Encounter 2: source 2020-10-15, practice 2021-09-20, outpatient, Obstetrics & Gynecology

Chief complaint: 2-week history of an itchy rash in her groin area

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Aisha Patel, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
2-week history of an itchy rash in her groin area

HISTORY OF PRESENT ILLNESS:
The patient is a 35-year-old female with asthma, type 2 diabetes mellitus, gastroesophageal reflux disease, and a history of gestational diabetes who presents with a new concern. She has been evaluated in pulmonology earlier this year for asthma and reflux symptoms. She reports that the rash worsened after she started using a new brand of exercise clothing.

PAST MEDICAL HISTORY:
Active Problem List:
- Asthma (diagnosed 2020-01-15)
- Gastroesophageal reflux disease (diagnosed 2020-04-15)
- Type 2 diabetes mellitus
- History of gestational diabetes

She has a past medical history of type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Cesarean section x2
- Dilation and curettage for missed abortion

MEDICATIONS:
She has been on prednisone for the past month for an acute exacerbation of asthma.

Home Medications:
- Fluticasone/Salmeterol 100/50 mcg inhaled twice daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed
- Metformin 1000 mg by mouth twice daily
- Omeprazole 20 mg by mouth once daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with asthma
- Father with type 2 diabetes mellitus
- Maternal history of preeclampsia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social. Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
On physical examination, there are erythematous plaques with sharp borders and satellite papules and pustules in the inguinal folds bilaterally. No lymphadenopathy is noted. The rest of the skin examination is unremarkable.

## Encounter 3: source 2021-04-15, practice 2022-03-09, outpatient, Internal Medicine

Chief complaint: Sore throat and difficulty swallowing for the past week

ENCOUNTER NOTE
Date: 2021-04-15
Provider: Dr. James O'Brien, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Sore throat and difficulty swallowing for the past week

HISTORY OF PRESENT ILLNESS:
She is a 36-year-old with asthma and prior cutaneous candidiasis. She reports a white coating on her tongue that does not scrape off easily. She has not had any recent asthma exacerbations.

PAST MEDICAL HISTORY:
Active Problem List:
- Asthma (diagnosed 2020-01-15)
- Gastroesophageal reflux disease (diagnosed 2020-04-15)
- Cutaneous candidiasis (diagnosed 2020-10-15)
- Type 2 diabetes mellitus
- History of gestational diabetes

PAST SURGICAL HISTORY:
- Cesarean section x2
- Dilation and curettage for missed abortion

MEDICATIONS:
Inhaled corticosteroid twice daily as prescribed for the past six months

Home Medications:
- Fluticasone/Salmeterol 100/50 mcg inhaled twice daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed
- Metformin 1000 mg by mouth twice daily
- Omeprazole 20 mg by mouth once daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with asthma
- Father with type 2 diabetes mellitus
- Maternal history of preeclampsia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social. Occupation: Elementary school teacher.

VITAL SIGNS:
Within normal limits

PHYSICAL EXAMINATION:
Oral examination reveals white plaques on the buccal mucosa and tongue that are adherent and bleed slightly when scraped. Her lungs are clear to auscultation bilaterally. There is no cervical lymphadenopathy.

## Encounter 4: source 2021-07-15, practice 2022-06-05, ed, Emergency Medicine

Chief complaint: Sudden onset of generalized tonic-clonic seizures

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Emily Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of generalized tonic-clonic seizures

HISTORY OF PRESENT ILLNESS:
The patient is a 32-year-old female with asthma, type 2 diabetes mellitus, gastroesophageal reflux disease, and prior gestational diabetes, with a recent history of oral candidiasis, who is brought to the emergency department by her husband due to a sudden onset of generalized tonic-clonic seizures. She delivered her first child 24 hours ago via a prolonged labor that required augmentation with oxytocin. She denies any headache, vision changes, or abdominal pain prior to the seizure. Her husband reports that she has been complaining of nausea and has appeared increasingly confused over the past few hours.

PAST MEDICAL HISTORY:
Active Problem List:
- Asthma (diagnosed 2020-01-15)
- Gastroesophageal reflux disease (diagnosed 2020-04-15)
- Cutaneous candidiasis (diagnosed 2020-10-15)
- Oral candidiasis (diagnosed 2021-04-15)
- Type 2 diabetes mellitus
- History of gestational diabetes

Her pregnancy was complicated by gestational diabetes, which was diet-controlled.

PAST SURGICAL HISTORY:
- Cesarean section x2
- Dilation and curettage for missed abortion

MEDICATIONS:
Home Medications:
- Fluticasone/Salmeterol 100/50 mcg inhaled twice daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed
- Metformin 1000 mg by mouth twice daily
- Omeprazole 20 mg by mouth once daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with asthma
- Father with type 2 diabetes mellitus
- Maternal history of preeclampsia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social. Occupation: Elementary school teacher.

VITAL SIGNS:
On examination, her temperature is 37.1°C (98.8°F), blood pressure is 130/85 mmHg, pulse is 88/min, and respirations are 16/min.

PHYSICAL EXAMINATION:
She is postictal but arousable, with no focal neurological deficits.

LABORATORY DATA:
Laboratory studies are as follows:

- Sodium: 122 mEq/L
- Potassium: 4.2 mEq/L
- Chloride: 98 mEq/L
- Bicarbonate: 24 mEq/L
- Blood urea nitrogen: 18 mg/dL
- Creatinine: 0.9 mg/dL
- Glucose: 140 mg/dL
- Serum osmolality: 260 mOsm/kg
- Urine osmolality: 500 mOsm/kg

## Encounter 5: source 2022-07-15, practice 2023-06-22, inpatient, Obstetrics & Gynecology

Chief complaint: Active labor at 39 weeks gestation

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. David Kim, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
Active labor at 39 weeks gestation

HISTORY OF PRESENT ILLNESS:
The patient is a 37-year-old female with asthma and gastroesophageal reflux disease managed by Pulmonology since 2020, type 2 diabetes mellitus, and prior gestational diabetes. She presents in active labor at 39 weeks gestation. She has had an uncomplicated prenatal course. Her labor progresses normally, and she delivers a healthy male infant with Apgar scores of 8 and 9 at 1 and 5 minutes, respectively. However, the placenta does not deliver spontaneously after 30 minutes, and manual attempts at removal result in significant vaginal bleeding. The bleeding is not controlled with uterine massage and oxytocin administration.

PAST MEDICAL HISTORY:
Active Problem List:
- Asthma (diagnosed 2020-01-15)
- Gastroesophageal reflux disease (diagnosed 2020-04-15)
- Cutaneous candidiasis (diagnosed 2020-10-15)
- Oral candidiasis (diagnosed 2021-04-15)
- Hyponatremia (diagnosed 2021-07-15)
- Type 2 diabetes mellitus
- History of gestational diabetes

PAST SURGICAL HISTORY:
- Cesarean section x2
- Dilation and curettage for missed abortion

MEDICATIONS:
Home Medications:
- Fluticasone/Salmeterol 100/50 mcg inhaled twice daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed
- Metformin 1000 mg by mouth twice daily
- Omeprazole 20 mg by mouth once daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with asthma
- Father with type 2 diabetes mellitus
- Maternal history of preeclampsia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social. Occupation: Elementary school teacher.

VITAL SIGNS:
The patient's vital signs are stable

PHYSICAL EXAMINATION:
On examination, the uterus is firm, and the cervix is dilated.

LABORATORY DATA:
Her hemoglobin has dropped from 12.5 g/dL at admission to 10.2 g/dL

IMAGING:
An ultrasound is performed, which shows an irregular placental bed and placental tissue that appears to be invading the myometrium.

## Encounter 6: source 2023-11-15, practice 2024-10-21, inpatient, Obstetrics & Gynecology

Chief complaint: Induction of labor due to gestational hypertension

ENCOUNTER NOTE
Date: 2023-11-15
Provider: Dr. Maria Santos, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
Induction of labor due to gestational hypertension

HISTORY OF PRESENT ILLNESS:
The 38-year-old G2P1 female with asthma, type 2 diabetes, and prior placenta accreta presents for induction for gestational hypertension. She is admitted to the labor and delivery unit. Her pregnancy has been complicated by gestational diabetes, which is diet-controlled. After discussion with the patient, an epidural anesthesia is placed for pain control. Thirty minutes following the procedure, the patient complains of nausea and dizziness.

PAST MEDICAL HISTORY:
Active Problem List:
- Asthma (diagnosed 2020-01-15)
- Gastroesophageal reflux disease (diagnosed 2020-04-15)
- Cutaneous candidiasis (diagnosed 2020-10-15)
- Oral candidiasis (diagnosed 2021-04-15)
- Hyponatremia (diagnosed 2021-07-15)
- Placenta accreta (diagnosed 2022-07-15)
- Type 2 diabetes mellitus
- History of gestational diabetes

She has a past medical history of asthma and a previous cesarean section.

PAST SURGICAL HISTORY:
- Cesarean section x2
- Dilation and curettage for missed abortion

MEDICATIONS:
Home Medications:
- Fluticasone/Salmeterol 100/50 mcg inhaled twice daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed
- Metformin 1000 mg by mouth twice daily
- Omeprazole 20 mg by mouth once daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with asthma
- Father with type 2 diabetes mellitus
- Maternal history of preeclampsia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social. Occupation: Elementary school teacher.

VITAL SIGNS:
On examination, her blood pressure is 145/95 mmHg, heart rate is 88 beats per minute, and respiratory rate is 16 breaths per minute. She is afebrile. Her blood pressure is now 85/45 mmHg, heart rate is 110 beats per minute, and oxygen saturation is 98% on room air.

PHYSICAL EXAMINATION:
The fetal heart tracing is reassuring. The fetal heart tracing remains reassuring.

## Encounter 7: source 2024-11-15, practice 2025-10-06, icu, Pulmonary & Critical Care

Chief complaint: Severe asthma exacerbation with shortness of breath and wheezing

ENCOUNTER NOTE
Date: 2024-11-15
Provider: Dr. Robert Thompson, MD
Department: Pulmonary & Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Severe asthma exacerbation with shortness of breath and wheezing

HISTORY OF PRESENT ILLNESS:
The patient is a 39-year-old female with asthma and GERD seen in 2020. She reports increasing shortness of breath, wheezing, and cough over the past 3 days not improved with her usual albuterol inhaler.

PAST MEDICAL HISTORY:
Active Problem List:
- Asthma (diagnosed 2020-01-15)
- Gastroesophageal reflux disease (diagnosed 2020-04-15)
- Cutaneous candidiasis (diagnosed 2020-10-15)
- Oral candidiasis (diagnosed 2021-04-15)
- Hyponatremia (diagnosed 2021-07-15)
- Placenta accreta (diagnosed 2022-07-15)
- Hypotension due to neuraxial anesthesia (diagnosed 2023-11-15)
- Type 2 diabetes mellitus
- History of gestational diabetes

PAST SURGICAL HISTORY:
- Cesarean section x2
- Dilation and curettage for missed abortion

MEDICATIONS:
Home Medications:
- Fluticasone/Salmeterol 100/50 mcg inhaled twice daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed
- Metformin 1000 mg by mouth twice daily
- Omeprazole 20 mg by mouth once daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with asthma
- Father with type 2 diabetes mellitus
- Maternal history of preeclampsia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social. Occupation: Elementary school teacher.

VITAL SIGNS:
Her respiratory rate is 28 breaths per minute, heart rate is 110 beats per minute, blood pressure is 135/85 mmHg, and oxygen saturation is 89% on room air.

PHYSICAL EXAMINATION:
On examination, she is using accessory muscles to breathe, and auscultation reveals diffuse expiratory wheezes.

LABORATORY DATA:
Arterial blood gas analysis shows a pH of 7.35, partial pressure of carbon dioxide (PaCO2) of 45 mmHg, partial pressure of oxygen (PaO2) of 60 mmHg, and bicarbonate (HCO3-) of 24 mEq/L.

PLAN:
Despite receiving nebulized albuterol and ipratropium, her respiratory effort has not improved, and repeat arterial blood gas analysis shows a pH of 7.32, PaCO2 of 50 mmHg, and PaO2 of 58 mmHg.
