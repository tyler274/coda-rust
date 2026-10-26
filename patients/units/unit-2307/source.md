# synthetic_hospital patient 2307

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 63 at first encounter, sex M, Non-Hispanic White, Commercial insurance (early retiree benefits).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2019-12-24 (Tue) | outpatient | Urology |
| 1 | 2020-07-15 | 2020-06-19 (Fri) | outpatient | Pulmonology |
| 2 | 2023-01-15 | 2022-12-26 (Mon) | outpatient | Gastroenterology |
| 3 | 2026-01-15 | 2025-12-15 (Mon) | ed | Emergency Medicine |
| 4 | 2026-02-15 | 2026-01-11 (Sun) | icu | Pulmonary & Critical Care |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial insurance (early retiree benefits)",
 "occupation": "Retired rubber manufacturing plant worker",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Father: Lung cancer (deceased age 68)",
  "Mother: Gastric cancer (deceased age 72)",
  "Brother: Hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 45 pack-years (1 pack per day)",
 "home_medications": [
  {
   "dose": "18 mcg inhaled daily",
   "name": "Tiotropium"
  },
  {
   "dose": "250/50 mcg inhaled twice daily",
   "name": "Fluticasone/Salmeterol"
  },
  {
   "dose": "90 mcg inhaled PRN for shortness of breath",
   "name": "Albuterol"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Omeprazole"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Chronic obstructive pulmonary disease (COPD)",
  "History of Helicobacter pylori infection (treated)",
  "Tobacco use disorder"
 ],
 "age_at_first_encounter": 63
}
```

Primary diagnoses: ["Refractory hypoxemia in ARDS", "Non-small cell lung cancer, right upper lobe", "Gastric adenocarcinoma, diffuse type (signet-ring cell carcinoma)", "Bladder cancer", "Superior vena cava syndrome secondary to lung cancer"]
Comorbidities: ["Tobacco use disorder"]

## Encounter 0: source 2020-01-15, practice 2019-12-24, outpatient, Urology

Chief complaint: 3-month history of painless hematuria, concerned about bladder cancer risk given occupational exposure to aromatic amines in rubber manufacturing

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
3-month history of painless hematuria, concerned about bladder cancer risk given occupational exposure to aromatic amines in rubber manufacturing

HISTORY OF PRESENT ILLNESS:
He presents to the clinic with a 3-month history of painless hematuria. The patient is anxious about his risk of bladder cancer and inquires about regular screening.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (COPD)
- History of Helicobacter pylori infection (treated)
- Tobacco use disorder

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Tiotropium 18 mcg inhaled daily
- Fluticasone/Salmeterol 250/50 mcg inhaled twice daily
- Albuterol 90 mcg inhaled PRN for shortness of breath
- Omeprazole 20 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
His family history is significant for bladder cancer in his father.

SOCIAL HISTORY:
He has a 40-pack-year smoking history and worked in a rubber manufacturing plant until his retirement last year.

PHYSICAL EXAMINATION:
On physical examination, there is no costovertebral angle tenderness, and the abdominal exam is unremarkable.

LABORATORY DATA:
Urinalysis reveals red blood cells but no casts or infection. Urine cytology is negative for malignant cells.

Laboratory Values:
- Urinalysis: Red blood cells present, no casts, no signs of infection
- Urine cytology: Negative for malignant cells

## Encounter 1: source 2020-07-15, practice 2020-06-19, outpatient, Pulmonology

Chief complaint: Persistent cough productive of thick sputum occasionally streaked with blood, accompanied by fatigue and unintentional weight loss

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent cough productive of thick sputum occasionally streaked with blood, accompanied by fatigue and unintentional weight loss

HISTORY OF PRESENT ILLNESS:
The patient with COPD and prior bladder cancer returns reporting fatigue, unintentional weight loss, and mild shortness of breath on exertion.

PAST MEDICAL HISTORY:
Active Problem List:
- Bladder cancer (diagnosed 2020-01-15)
- Chronic obstructive pulmonary disease (COPD)
- History of Helicobacter pylori infection (treated)
- Tobacco use disorder

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Tiotropium 18 mcg inhaled daily
- Fluticasone/Salmeterol 250/50 mcg inhaled twice daily
- Albuterol 90 mcg inhaled PRN for shortness of breath
- Omeprazole 20 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (deceased age 68)
- Mother: Gastric cancer (deceased age 72)
- Brother: Hypertension

SOCIAL HISTORY:
He has a 45-pack-year smoking history and continues to smoke one pack per day.

REVIEW OF SYSTEMS:
He denies fever, night sweats, recent travel, or sick contacts.

VITAL SIGNS:
His temperature is 98.9°F (37.2°C), blood pressure is 134/82 mmHg, heart rate is 88/min, respiratory rate is 16/min, and oxygen saturation is 96% on room air.

PHYSICAL EXAMINATION:
Lung auscultation reveals decreased breath sounds over the right upper lobe.

LABORATORY DATA:
Laboratory studies show a serum calcium level of 14.2 mg/dL with low parathyroid hormone (PTH).

IMAGING:
Chest X-ray shows a hilar mass on the right.

PLAN:
A drug is administered to lower the patient's calcium level.

## Encounter 2: source 2023-01-15, practice 2022-12-26, outpatient, Gastroenterology

Chief complaint: Progressive unintentional weight loss of 8 kg over 3 months with early satiety and intermittent epigastric discomfort

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Aisha Patel, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive unintentional weight loss of 8 kg over 3 months with early satiety and intermittent epigastric discomfort

HISTORY OF PRESENT ILLNESS:
This 66-year-old male with COPD and prior bladder and lung cancer presents. He has also experienced intermittent epigastric discomfort and occasional nausea, but no vomiting or hematemesis. He reports losing approximately 8 kg (18 lb) unintentionally.

PAST MEDICAL HISTORY:
Active Problem List:
- Bladder cancer (diagnosed 2020-01-15)
- Non-small cell lung cancer, right upper lobe (diagnosed 2020-07-15)
- Chronic obstructive pulmonary disease (COPD)
- History of Helicobacter pylori infection (treated)
- Tobacco use disorder

His past medical history includes long-standing smoking and a remote history of Helicobacter pylori infection, which was treated several years ago.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Tiotropium 18 mcg inhaled daily
- Fluticasone/Salmeterol 250/50 mcg inhaled twice daily
- Albuterol 90 mcg inhaled PRN for shortness of breath
- Omeprazole 20 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
His family history is significant for gastric cancer in his mother, who died at age 60.

SOCIAL HISTORY:
He drinks alcohol socially and has a low-fiber, high-salt diet consisting mostly of smoked and pickled foods.

PHYSICAL EXAMINATION:
On examination, he appears thin with pale conjunctiva and mild tenderness in the epigastric region without rebound or guarding. Digital rectal exam reveals positive occult blood.

LABORATORY DATA:
Laboratory studies show a hemoglobin level of 9.8 g/dL.

PATHOLOGY:
Biopsy reveals invasive gland-forming cells with signet-ring morphology and desmoplastic reaction.

OTHER STUDIES:
Esophagogastroduodenoscopy demonstrates a firm, ulcerated mass along the lesser curvature of the stomach

## Encounter 3: source 2026-01-15, practice 2025-12-15, ed, Emergency Medicine

Chief complaint: Progressive facial swelling worse in the mornings and orthopnea over 3 weeks, with persistent cough and 10-pound weight loss

ENCOUNTER NOTE
Date: 2026-01-15
Provider: Dr. James O'Connor, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive facial swelling worse in the mornings and orthopnea over 3 weeks, with persistent cough and 10-pound weight loss

HISTORY OF PRESENT ILLNESS:
The patient is a 69-year-old male with history of bladder cancer, non-small cell lung cancer, and gastric adenocarcinoma who presents to the emergency department with a 3-week history of progressive facial swelling, especially in the mornings, and shortness of breath when lying flat. He also reports a persistent cough and a 10-pound weight loss over the past month.

PAST MEDICAL HISTORY:
Active Problem List:
- Bladder cancer (diagnosed 2020-01-15)
- Non-small cell lung cancer, right upper lobe (diagnosed 2020-07-15)
- Gastric adenocarcinoma, diffuse type (signet-ring cell carcinoma) (diagnosed 2023-01-15)
- Chronic obstructive pulmonary disease (COPD)
- History of Helicobacter pylori infection (treated)
- Tobacco use disorder

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Tiotropium 18 mcg inhaled daily
- Fluticasone/Salmeterol 250/50 mcg inhaled twice daily
- Albuterol 90 mcg inhaled PRN for shortness of breath
- Omeprazole 20 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (deceased age 68)
- Mother: Gastric cancer (deceased age 72)
- Brother: Hypertension

SOCIAL HISTORY:
40-pack-year smoking history

PHYSICAL EXAMINATION:
On physical examination, you note distended neck veins, facial plethora, and upper extremity edema. Auscultation of the lungs reveals decreased breath sounds in the right upper lobe.

LABORATORY DATA:
Laboratory values are as follows:

- Hemoglobin: 14.2 g/dL
- White blood cell count: 7,500/mm^3
- Platelet count: 250,000/mm^3
- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Creatinine: 1.0 mg/dL
- Calcium: 9.8 mg/dL

IMAGING:
A chest x-ray shows a right upper lobe mass and mediastinal widening.

## Encounter 4: source 2026-02-15, practice 2026-01-11, icu, Pulmonary & Critical Care

Chief complaint: Acute respiratory distress requiring emergent intubation and mechanical ventilation due to refractory hypoxemia and hypercapnia

ENCOUNTER NOTE
Date: 2026-02-15
Provider: Dr. Priya Narasimhan, MD
Department: Pulmonary & Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Acute respiratory distress requiring emergent intubation and mechanical ventilation due to refractory hypoxemia and hypercapnia

HISTORY OF PRESENT ILLNESS:
The patient, with history of bladder, lung, and gastric cancer and recent superior vena cava syndrome, is admitted to the intensive care unit for acute respiratory distress. He is intubated and placed on mechanical ventilation due to hypoxemia and hypercapnia. Despite initial settings of volume control ventilation with a tidal volume of 450 mL, respiratory rate of 16 breaths per minute, and FiO2 of 60%, the patient's oxygenation does not improve significantly.

PAST MEDICAL HISTORY:
Active Problem List:
- Bladder cancer (diagnosed 2020-01-15)
- Non-small cell lung cancer, right upper lobe (diagnosed 2020-07-15)
- Gastric adenocarcinoma, diffuse type (signet-ring cell carcinoma) (diagnosed 2023-01-15)
- Superior vena cava syndrome secondary to lung cancer (diagnosed 2026-01-15)
- Chronic obstructive pulmonary disease (COPD)
- History of Helicobacter pylori infection (treated)
- Tobacco use disorder

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Tiotropium 18 mcg inhaled daily
- Fluticasone/Salmeterol 250/50 mcg inhaled twice daily
- Albuterol 90 mcg inhaled PRN for shortness of breath
- Omeprazole 20 mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer (deceased age 68)
- Mother: Gastric cancer (deceased age 72)
- Brother: Hypertension

SOCIAL HISTORY:
Smoking: Current smoker, 45 pack-years (1 pack per day). Alcohol: Social (1-2 drinks per week). Occupation: Retired rubber manufacturing plant worker.

PHYSICAL EXAMINATION:
Diffuse wheezing and decreased breath sounds bilaterally.

LABORATORY DATA:
An arterial blood gas shows pH 7.32, PaCO2 55 mmHg, PaO2 68 mmHg, and HCO3- 24 mEq/L.

OTHER STUDIES:
The ventilator's peak airway pressure is noted to be 35 cm H2O, and after performing an end-inspiratory hold maneuver, the plateau pressure is measured at 30 cm H2O. The PEEP is set at 5 cm H2O.
