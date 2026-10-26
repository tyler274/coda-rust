# synthetic_hospital patient 2596

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 25 at first encounter, sex M, African American, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-02-23 (Thu) | outpatient | Internal Medicine |
| 1 | 2020-04-15 | 2023-05-23 (Tue) | outpatient | Allergy & Immunology |
| 2 | 2020-10-15 | 2023-11-24 (Fri) | outpatient | Urology |
| 3 | 2021-04-15 | 2024-05-28 (Tue) | outpatient | Allergy & Immunology |
| 4 | 2021-10-15 | 2024-11-25 (Mon) | outpatient | Hematology & Oncology |
| 5 | 2022-05-15 | 2025-07-01 (Tue) | ed | Emergency Medicine |
| 6 | 2022-10-15 | 2025-11-22 (Sat) | ed | Emergency Medicine |
| 7 | 2023-01-15 | 2026-03-03 (Tue) | icu | Anesthesiology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Succinylcholine (anaphylactoid reaction - malignant hyperthermia trigger)",
  "Volatile anesthetics (malignant hyperthermia trigger)"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Graduate student",
 "alcohol_use": "None",
 "family_history": [
  "Testicular germ cell tumor in older brother",
  "Suspected familial malignant hyperthermia susceptibility"
 ],
 "race_ethnicity": "African American",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "200 mg intramuscular every 2 weeks",
   "name": "Testosterone cypionate"
  },
  {
   "dose": "Monthly infusion per immunology protocol",
   "name": "Immune globulin (IVIG)"
  },
  {
   "dose": "250/50 mcg inhaled twice daily",
   "name": "Fluticasone/salmeterol"
  },
  {
   "dose": "90 mcg inhaled as needed for bronchospasm",
   "name": "Albuterol sulfate"
  },
  {
   "dose": "0.5 mg orally as needed for anxiety",
   "name": "Alprazolam"
  }
 ],
 "surgical_history": [
  "Right orchiopexy at age 15 months",
  "Open reduction and internal fixation of right tibial fracture"
 ],
 "chronic_conditions": [
  "Common variable immunodeficiency",
  "Asthma",
  "Anxiety disorder",
  "Gender dysphoria",
  "Sickle cell trait",
  "Malignant hyperthermia susceptibility"
 ],
 "age_at_first_encounter": 25
}
```

Primary diagnoses: ["Common variable immunodeficiency", "Brain abscess", "Testicular germ cell tumor screening in cryptorchidism", "Gender Dysphoria", "Bronchial obstruction (mucus plug/foreign body)", "Sickle cell trait", "Malignant hyperthermia"]
Comorbidities: ["Opportunistic pulmonary infection requiring diagnostic sampling", "Chronic Sinusitis", "Asthma", "Acute Bacterial Sinusitis", "Bronchiectasis"]

## Encounter 0: source 2020-01-15, practice 2023-02-23, outpatient, Internal Medicine

Chief complaint: Routine health check-up for gender-affirming care maintenance

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine health check-up for gender-affirming care maintenance

HISTORY OF PRESENT ILLNESS:
He has been on testosterone therapy for the past 2 years and reports general well-being. He mentions experiencing anxiety when interacting with healthcare professionals due to past experiences of being misgendered and called by his deadname. The patient's electronic health record displays his preferred name and gender, but the printed labels on his laboratory requisition forms show his legal name, which he no longer identifies with.

PAST MEDICAL HISTORY:
Active Problem List:
- Common variable immunodeficiency
- Asthma
- Anxiety disorder
- Gender dysphoria
- Sickle cell trait
- Malignant hyperthermia susceptibility

His medical history is significant for well-controlled asthma and anxiety.

PAST SURGICAL HISTORY:
He has no surgical history.

MEDICATIONS:
He is currently taking fluticasone/salmeterol and alprazolam as needed.

Home Medications:
- Testosterone cypionate 200 mg intramuscular every 2 weeks
- Immune globulin (IVIG) Monthly infusion per immunology protocol
- Albuterol sulfate 90 mcg inhaled as needed for bronchospasm

ALLERGIES:
Succinylcholine (anaphylactoid reaction - malignant hyperthermia trigger), Volatile anesthetics (malignant hyperthermia trigger)

FAMILY HISTORY:
- Testicular germ cell tumor in older brother
- Suspected familial malignant hyperthermia susceptibility

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Graduate student.

VITAL SIGNS:
On examination, his blood pressure is 128/76 mmHg, heart rate is 72 beats per minute, and BMI is 24 kg/m^2.

PHYSICAL EXAMINATION:
His physical examination is unremarkable.

LABORATORY DATA:
Laboratory results are as follows:

- Hemoglobin: 15.2 g/dL
- Hematocrit: 45%
- Fasting glucose: 88 mg/dL
- Total cholesterol: 190 mg/dL
- HDL cholesterol: 40 mg/dL
- LDL cholesterol: 120 mg/dL
- Triglycerides: 150 mg/dL

## Encounter 1: source 2020-04-15, practice 2023-05-23, outpatient, Allergy & Immunology

Chief complaint: Recurrent sinusitis and bronchitis follow-up

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. James Rodriguez, MD
Department: Allergy & Immunology
Visit Type: outpatient

CHIEF COMPLAINT:
Recurrent sinusitis and bronchitis follow-up

HISTORY OF PRESENT ILLNESS:
The patient, who has common variable immunodeficiency and is on IVIG, returns for follow-up of recurrent sinusitis and bronchitis following his January visit for gender-affirming care. He presents with history of recurrent sinusitis and bronchitis since adolescence. He reports four episodes of sinusitis in the past year, each requiring antibiotic therapy. He also mentions a recent episode of pneumonia that was slow to resolve with standard treatment.

PAST MEDICAL HISTORY:
Active Problem List:
- Gender Dysphoria (diagnosed 2020-01-15)
- Common variable immunodeficiency
- Asthma
- Anxiety disorder
- Sickle cell trait
- Malignant hyperthermia susceptibility

His past medical history is significant for an episode of acute otitis media and gastrointestinal giardiasis during his college years.

PAST SURGICAL HISTORY:
- Right orchiopexy at age 15 months
- Open reduction and internal fixation of right tibial fracture

MEDICATIONS:
Home Medications:
- Testosterone cypionate 200 mg intramuscular every 2 weeks
- Immune globulin (IVIG) Monthly infusion per immunology protocol
- Fluticasone/salmeterol 250/50 mcg inhaled twice daily
- Albuterol sulfate 90 mcg inhaled as needed for bronchospasm
- Alprazolam 0.5 mg orally as needed for anxiety

ALLERGIES:
Succinylcholine (anaphylactoid reaction - malignant hyperthermia trigger), Volatile anesthetics (malignant hyperthermia trigger)

FAMILY HISTORY:
His family history is noncontributory.

SOCIAL HISTORY:
He denies smoking, alcohol, or drug use.

VITAL SIGNS:
His vital signs are within normal limits.

PHYSICAL EXAMINATION:
On physical examination, the patient appears well-developed and well-nourished. Examination of the head and neck reveals mild mucosal edema without purulent nasal discharge. Lung auscultation is clear, and the remainder of the exam is unremarkable.

LABORATORY DATA:
Laboratory studies are ordered, and the results are as follows:

- Hemoglobin: 14.2 g/dL
- White blood cell count: 6,800 cells/µL
- Platelets: 250,000 cells/µL
- Serum IgG: 200 mg/dL (Low)
- Serum IgA: <7 mg/dL (Low)
- Serum IgM: 40 mg/dL (Low)

IMAGING:
Chest X-ray: No acute disease

## Encounter 2: source 2020-10-15, practice 2023-11-24, outpatient, Urology

Chief complaint: Routine health examination with concerns about testicular cancer risk

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Priya Patel, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine health examination with concerns about testicular cancer risk

HISTORY OF PRESENT ILLNESS:
The patient presents in the context of his CVID and gender-affirming care, following recent encounters for immunology and gender dysphoria management. He reports a history of right cryptorchidism that was surgically corrected with orchiopexy at the age of 15 months. He has no complaints but is concerned about his future fertility and the risk of cancer, as his older brother was recently diagnosed with testicular cancer.

PAST MEDICAL HISTORY:
Active Problem List:
- Gender Dysphoria (diagnosed 2020-01-15)
- Common variable immunodeficiency (diagnosed 2020-04-15)
- Asthma
- Anxiety disorder
- Sickle cell trait
- Malignant hyperthermia susceptibility

His medical history is significant for asthma.

PAST SURGICAL HISTORY:
Significant past surgical history.

MEDICATIONS:
He takes for which he uses an albuterol inhaler as needed.

Home Medications:
- Testosterone cypionate 200 mg intramuscular every 2 weeks
- Immune globulin (IVIG) Monthly infusion per immunology protocol
- Fluticasone/salmeterol 250/50 mcg inhaled twice daily
- Albuterol sulfate 90 mcg inhaled as needed for bronchospasm
- Alprazolam 0.5 mg orally as needed for anxiety

ALLERGIES:
Succinylcholine (anaphylactoid reaction - malignant hyperthermia trigger), Volatile anesthetics (malignant hyperthermia trigger)

FAMILY HISTORY:
His family history is notable for his brother's testicular cancer and a paternal grandfather with prostate cancer.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Graduate student.

VITAL SIGNS:
His vital signs are within normal limits.

PHYSICAL EXAMINATION:
On examination, both testes are palpable in the scrotum, with the right testis slightly smaller than the left. There are no masses palpable, and the examination is otherwise unremarkable.

ASSESSMENT AND PLAN:
Given his history and concerns,

## Encounter 3: source 2021-04-15, practice 2024-05-28, outpatient, Allergy & Immunology

Chief complaint: Follow-up for recurrent sinus infections and pneumonia

ENCOUNTER NOTE
Date: 2021-04-15
Provider: Dr. James Rodriguez, MD
Department: Allergy & Immunology
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up for recurrent sinus infections and pneumonia

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of his common variable immunodeficiency, last evaluated in April 2020. He reports experiencing four episodes of sinusitis and three episodes of pneumonia during this period, each requiring antibiotic therapy. He also mentions a persistent productive cough and occasional shortness of breath.

PAST MEDICAL HISTORY:
Active Problem List:
- Gender Dysphoria (diagnosed 2020-01-15)
- Common variable immunodeficiency (diagnosed 2020-04-15)
- Testicular germ cell tumor screening in cryptorchidism (diagnosed 2020-10-15)
- Asthma
- Anxiety disorder
- Sickle cell trait
- Malignant hyperthermia susceptibility

His past medical history is significant for childhood asthma, which he states was well-controlled until recent years. His immunization history is up to date.

PAST SURGICAL HISTORY:
- Right orchiopexy at age 15 months
- Open reduction and internal fixation of right tibial fracture

MEDICATIONS:
Home Medications:
- Testosterone cypionate 200 mg intramuscular every 2 weeks
- Immune globulin (IVIG) Monthly infusion per immunology protocol
- Fluticasone/salmeterol 250/50 mcg inhaled twice daily
- Albuterol sulfate 90 mcg inhaled as needed for bronchospasm
- Alprazolam 0.5 mg orally as needed for anxiety

ALLERGIES:
Succinylcholine (anaphylactoid reaction - malignant hyperthermia trigger), Volatile anesthetics (malignant hyperthermia trigger)

FAMILY HISTORY:
- Testicular germ cell tumor in older brother
- Suspected familial malignant hyperthermia susceptibility

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Graduate student.

PHYSICAL EXAMINATION:
On physical examination, you note crackles at the lung bases bilaterally.

LABORATORY DATA:
Laboratory tests reveal the following:

- White blood cell count: 7,000/uL (normal range: 4,500-11,000/uL)
- Hemoglobin: 14 g/dL (normal range: 13.8-17.2 g/dL)
- Platelets: 250,000/uL (normal range: 150,000-450,000/uL)
- Serum immunoglobulin G (IgG): 200 mg/dL (normal range: 700-1600 mg/dL)
- Serum immunoglobulin A (IgA): 20 mg/dL (normal range: 70-400 mg/dL)
- Serum immunoglobulin M (IgM): 40 mg/dL (normal range: 40-230 mg/dL)

IMAGING:
Chest X-ray: Evidence of bronchiectasis and scattered fibrotic changes

## Encounter 4: source 2021-10-15, practice 2024-11-25, outpatient, Hematology & Oncology

Chief complaint: Painless gross hematuria for 2 days

ENCOUNTER NOTE
Date: 2021-10-15
Provider: Dr. Kwame Osei, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Painless gross hematuria for 2 days

HISTORY OF PRESENT ILLNESS:
This 26-year-old male with sickle cell trait and CVID on IVIG, prior Urology evaluation, presents with a 2-day history of painless gross hematuria. He denies any recent trauma, fever, dysuria, or flank pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Gender Dysphoria (diagnosed 2020-01-15)
- Common variable immunodeficiency (diagnosed 2020-04-15)
- Testicular germ cell tumor screening in cryptorchidism (diagnosed 2020-10-15)
- Asthma
- Anxiety disorder
- Sickle cell trait
- Malignant hyperthermia susceptibility

His past medical history is significant for mild asthma, and he mentions that he was told he has a "blood trait" during a newborn screening, but he has never experienced any related symptoms.

PAST SURGICAL HISTORY:
- Right orchiopexy at age 15 months
- Open reduction and internal fixation of right tibial fracture

MEDICATIONS:
Home Medications:
- Testosterone cypionate 200 mg intramuscular every 2 weeks
- Immune globulin (IVIG) Monthly infusion per immunology protocol
- Fluticasone/salmeterol 250/50 mcg inhaled twice daily
- Albuterol sulfate 90 mcg inhaled as needed for bronchospasm
- Alprazolam 0.5 mg orally as needed for anxiety

ALLERGIES:
Succinylcholine (anaphylactoid reaction - malignant hyperthermia trigger), Volatile anesthetics (malignant hyperthermia trigger)

FAMILY HISTORY:
His family history is non-contributory.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Graduate student.

VITAL SIGNS:
On physical examination, his blood pressure is 125/78 mmHg, heart rate is 72 beats per minute, and temperature is 98.6°F (37°C).

PHYSICAL EXAMINATION:
Abdominal examination is unremarkable with no costovertebral angle tenderness.

LABORATORY DATA:
Urinalysis reveals red blood cells but no casts, white blood cells, or bacteria. Hemoglobin electrophoresis shows:

 - HbA: 55%
 - HbS: 43%
 - HbF: 1.5%
 - HbA2: 2.5%

## Encounter 5: source 2022-05-15, practice 2025-07-01, ed, Emergency Medicine

Chief complaint: Acute worsening of chronic cough with copious sputum production

ENCOUNTER NOTE
Date: 2022-05-15
Provider: Dr. Emily Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute worsening of chronic cough with copious sputum production

HISTORY OF PRESENT ILLNESS:
This 27-year-old male with common variable immunodeficiency and asthma, previously evaluated for recurrent sinus infections and pneumonia, presents with acute worsening of chronic cough and copious sputum production. He reports that these symptoms have been present since childhood but have worsened over the past year. He also mentions experiencing frequent ear infections during his childhood.

PAST MEDICAL HISTORY:
Active Problem List:
- Gender Dysphoria (diagnosed 2020-01-15)
- Common variable immunodeficiency (diagnosed 2020-04-15)
- Testicular germ cell tumor screening in cryptorchidism (diagnosed 2020-10-15)
- Sickle cell trait (diagnosed 2021-10-15)
- Asthma
- Anxiety disorder
- Malignant hyperthermia susceptibility

His past medical history is significant for several episodes of pneumonia.

PAST SURGICAL HISTORY:
- Right orchiopexy at age 15 months
- Open reduction and internal fixation of right tibial fracture

MEDICATIONS:
Home Medications:
- Testosterone cypionate 200 mg intramuscular every 2 weeks
- Immune globulin (IVIG) Monthly infusion per immunology protocol
- Fluticasone/salmeterol 250/50 mcg inhaled twice daily
- Albuterol sulfate 90 mcg inhaled as needed for bronchospasm
- Alprazolam 0.5 mg orally as needed for anxiety

ALLERGIES:
Succinylcholine (anaphylactoid reaction - malignant hyperthermia trigger), Volatile anesthetics (malignant hyperthermia trigger)

FAMILY HISTORY:
- Testicular germ cell tumor in older brother
- Suspected familial malignant hyperthermia susceptibility

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Graduate student.

PHYSICAL EXAMINATION:
On physical examination, you note digital clubbing and auscultate crackles at the lung bases bilaterally.

IMAGING:
A chest X-ray reveals dextrocardia. High-resolution CT scan of the chest shows bronchiectasis.

## Encounter 6: source 2022-10-15, practice 2025-11-22, ed, Emergency Medicine

Chief complaint: Progressive right-sided headache with early-morning vomiting

ENCOUNTER NOTE
Date: 2022-10-15
Provider: Dr. Michael Chang, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive right-sided headache with early-morning vomiting

HISTORY OF PRESENT ILLNESS:
The patient, who carries a diagnosis of common variable immunodeficiency on monthly IVIG replacement and was evaluated in this emergency department five months prior for bronchial obstruction, presents to the emergency department with a 2-week history of progressive right-sided headache, which has recently become more severe and localized. He also reports early-morning vomiting for the past three days.

PAST MEDICAL HISTORY:
Active Problem List:
- Gender Dysphoria (diagnosed 2020-01-15)
- Common variable immunodeficiency (diagnosed 2020-04-15)
- Testicular germ cell tumor screening in cryptorchidism (diagnosed 2020-10-15)
- Sickle cell trait (diagnosed 2021-10-15)
- Bronchial obstruction (mucus plug/foreign body) (diagnosed 2022-05-15)
- Asthma
- Anxiety disorder
- Malignant hyperthermia susceptibility

His past medical history is significant for recurrent sinus infections, the most recent of which occurred one month ago and was not treated with antibiotics.

PAST SURGICAL HISTORY:
- Right orchiopexy at age 15 months
- Open reduction and internal fixation of right tibial fracture

MEDICATIONS:
Home Medications:
- Testosterone cypionate 200 mg intramuscular every 2 weeks
- Immune globulin (IVIG) Monthly infusion per immunology protocol
- Fluticasone/salmeterol 250/50 mcg inhaled twice daily
- Albuterol sulfate 90 mcg inhaled as needed for bronchospasm
- Alprazolam 0.5 mg orally as needed for anxiety

ALLERGIES:
Succinylcholine (anaphylactoid reaction - malignant hyperthermia trigger), Volatile anesthetics (malignant hyperthermia trigger)

FAMILY HISTORY:
- Testicular germ cell tumor in older brother
- Suspected familial malignant hyperthermia susceptibility

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Graduate student.

VITAL SIGNS:
His temperature is 38.6°C, heart rate is 102 bpm, and blood pressure is 130/75 mmHg.

PHYSICAL EXAMINATION:
On examination, the patient appears lethargic.
Neurological examination reveals right-sided weakness and a positive right Babinski sign.

LABORATORY DATA:
Laboratory values are as follows:

- White blood cell count: 15,000 /µL
- Hemoglobin: 13.5 g/dL
- Platelet count: 250,000 /µL
- C-reactive protein: 24 mg/L (normal <5 mg/L)
- Erythrocyte sedimentation rate: 55 mm/hr (normal 0-20 mm/hr)

IMAGING:
A CT scan of the head without contrast shows a ring-enhancing lesion in the left frontal lobe.

## Encounter 7: source 2023-01-15, practice 2026-03-03, icu, Anesthesiology

Chief complaint: Intraoperative crisis during tibia fracture repair

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. David Kim, MD
Department: Anesthesiology
Visit Type: icu

CHIEF COMPLAINT:
Intraoperative crisis during tibia fracture repair

HISTORY OF PRESENT ILLNESS:
The patient, with known malignant hyperthermia susceptibility and common variable immunodeficiency, is undergoing a repair of a fractured tibia under general anesthesia. Approximately 45 minutes after induction with isoflurane and succinylcholine, the anesthesiologist notes that the patient's end-tidal CO2 is rising despite increased ventilation.

PAST MEDICAL HISTORY:
Active Problem List:
- Gender Dysphoria (diagnosed 2020-01-15)
- Common variable immunodeficiency (diagnosed 2020-04-15)
- Testicular germ cell tumor screening in cryptorchidism (diagnosed 2020-10-15)
- Sickle cell trait (diagnosed 2021-10-15)
- Bronchial obstruction (mucus plug/foreign body) (diagnosed 2022-05-15)
- Brain abscess (diagnosed 2022-10-15)
- Asthma
- Anxiety disorder
- Malignant hyperthermia susceptibility

PAST SURGICAL HISTORY:
- Right orchiopexy at age 15 months
- Open reduction and internal fixation of right tibial fracture

MEDICATIONS:
Home Medications:
- Testosterone cypionate 200 mg intramuscular every 2 weeks
- Immune globulin (IVIG) Monthly infusion per immunology protocol
- Fluticasone/salmeterol 250/50 mcg inhaled twice daily
- Albuterol sulfate 90 mcg inhaled as needed for bronchospasm
- Alprazolam 0.5 mg orally as needed for anxiety

ALLERGIES:
Succinylcholine (anaphylactoid reaction - malignant hyperthermia trigger), Volatile anesthetics (malignant hyperthermia trigger)

FAMILY HISTORY:
- Testicular germ cell tumor in older brother
- Suspected familial malignant hyperthermia susceptibility

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Graduate student.

VITAL SIGNS:
The patient's heart rate has increased to 110 bpm from a baseline of 70 bpm, and his temperature is now 39.2°C, up from 37.0°C at the start of the procedure.

PHYSICAL EXAMINATION:
On examination, the patient's jaw is noted to be rigid, and his limbs are stiff. His skin is warm and flushed.

ASSESSMENT AND PLAN:
The anesthesiologist suspects a serious complication related to the anesthetic agents.
