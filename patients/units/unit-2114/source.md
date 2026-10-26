# synthetic_hospital patient 2114

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 55 at first encounter, sex M, Non-Hispanic White, Private (Blue Cross Blue Shield).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-09-20 (Mon) | outpatient | Urology |
| 1 | 2020-04-15 | 2021-12-06 (Mon) | outpatient | Otolaryngology |
| 2 | 2021-01-15 | 2022-09-23 (Fri) | icu | Neurosurgery |
| 3 | 2021-07-15 | 2023-03-09 (Thu) | inpatient | General Surgery |
| 4 | 2022-01-15 | 2023-09-11 (Mon) | icu | Pulmonary and Critical Care |
| 5 | 2023-01-15 | 2024-09-22 (Sun) | icu | Pulmonary and Critical Care |
| 6 | 2024-01-15 | 2025-09-13 (Sat) | icu | General Surgery |
| 7 | 2024-03-15 | 2025-11-17 (Mon) | follow_up | General Surgery |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Private (Blue Cross Blue Shield)",
 "occupation": "Office administrator",
 "alcohol_use": "Social (2-3 drinks per week)",
 "family_history": [
  "Father: Colorectal cancer (diagnosed age 62)",
  "Mother: Type 2 diabetes mellitus",
  "Sister: Breast cancer"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 20 pack-years, quit 3 years ago",
 "home_medications": [
  {
   "dose": "0.4 mg PO daily",
   "name": "Tamsulosin"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  },
  {
   "dose": "500 mg PO PRN pain",
   "name": "Acetaminophen"
  }
 ],
 "surgical_history": [
  "Open reduction and internal fixation of complex pelvic fracture (age 53)",
  "Right hemicolectomy with primary anastomosis for Stage II colorectal cancer",
  "Exploratory laparotomy with sigmoid colectomy and end colostomy for perforated diverticulitis",
  "Parathyroidectomy for primary hyperparathyroidism",
  "Decompressive craniectomy for traumatic cerebral edema"
 ],
 "chronic_conditions": [
  "Urethral stricture secondary to traumatic pelvic fracture",
  "History of colorectal adenocarcinoma, status post resection",
  "Diverticulosis",
  "History of severe acute pancreatitis"
 ],
 "age_at_first_encounter": 55
}
```

Primary diagnoses: ["Post-obstructive atelectasis", "Retained surgical foreign body (gossypiboma)", "Euthyroid sick syndrome", "Urethral stricture", "Traumatic cerebral edema with elevated intracranial pressure", "Refractory hypoxemia in ARDS", "Injury of external branch of superior laryngeal nerve"]
Comorbidities: ["Status post laparoscopic cholecystectomy", "Dependence on mechanical ventilation"]

## Encounter 0: source 2020-01-15, practice 2021-09-20, outpatient, Urology

Chief complaint: Difficulty initiating urination and a weak urine stream for the past 6 months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
Difficulty initiating urination and a weak urine stream for the past 6 months

HISTORY OF PRESENT ILLNESS:
He presents to the clinic with a 6-month history of difficulty initiating urination and a weak urine stream. He reports frequent dribbling post-urination and a sensation of incomplete bladder emptying.

PAST MEDICAL HISTORY:
Active Problem List:
- Urethral stricture secondary to traumatic pelvic fracture
- History of colorectal adenocarcinoma, status post resection
- Diverticulosis
- History of severe acute pancreatitis

His past medical history is significant for a pelvic fracture due to a motor vehicle accident two years ago, which required surgical intervention.

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of complex pelvic fracture (age 53)
- Right hemicolectomy with primary anastomosis for Stage II colorectal cancer
- Exploratory laparotomy with sigmoid colectomy and end colostomy for perforated diverticulitis
- Parathyroidectomy for primary hyperparathyroidism
- Decompressive craniectomy for traumatic cerebral edema

MEDICATIONS:
Home Medications:
- Tamsulosin 0.4 mg PO daily
- Aspirin 81 mg PO daily
- Acetaminophen 500 mg PO PRN pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Colorectal cancer (diagnosed age 62)
- Mother: Type 2 diabetes mellitus
- Sister: Breast cancer

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 3 years ago. Alcohol: Social (2-3 drinks per week). Occupation: Office administrator.

PHYSICAL EXAMINATION:
On physical examination, his bladder is palpable and distended. Digital rectal examination reveals no abnormalities of the prostate.

LABORATORY DATA:
Urinalysis is unremarkable.

OTHER STUDIES:
Postvoid residual volume measured by bladder ultrasound is 150 mL.

## Encounter 1: source 2020-04-15, practice 2021-12-06, outpatient, Otolaryngology

Chief complaint: Persistent throat discomfort and changes in voice following anterior neck surgery

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Okafor, MD
Department: Otolaryngology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent throat discomfort and changes in voice following anterior neck surgery

HISTORY OF PRESENT ILLNESS:
The 55-year-old male with colorectal cancer s/p resection, urethral stricture, diverticulosis, and pancreatitis, last seen in Urology on 2020-01-15, returns with voice changes that began shortly after anterior neck surgery. Since the procedure, he reports difficulty reaching high-pitched tones when speaking and experiences mild choking when attempting to swallow liquids. He denies hoarseness, aspiration of solids, or shortness of breath.

PAST MEDICAL HISTORY:
Active Problem List:
- Urethral stricture (diagnosed 2020-01-15)
- Urethral stricture secondary to traumatic pelvic fracture
- History of colorectal adenocarcinoma, status post resection
- Diverticulosis
- History of severe acute pancreatitis

PAST SURGICAL HISTORY:
He underwent the operation two months ago to address a multinodular thyroid enlargement that was causing mild dysphagia.

MEDICATIONS:
Home Medications:
- Tamsulosin 0.4 mg PO daily
- Aspirin 81 mg PO daily
- Acetaminophen 500 mg PO PRN pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Colorectal cancer (diagnosed age 62)
- Mother: Type 2 diabetes mellitus
- Sister: Breast cancer

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 3 years ago. Alcohol: Social (2-3 drinks per week). Occupation: Office administrator.

PHYSICAL EXAMINATION:
Physical examination reveals a well-healed anterior cervical scar and symmetric elevation of the palate. His speech is clear but notably weak and breathy during attempts to project his voice.

OTHER STUDIES:
Flexible laryngoscopy shows normal vocal cord motion but poor tensioning of the ipsilateral vocal fold during phonation. Sensation above the vocal cords is diminished on the affected side.

## Encounter 2: source 2021-01-15, practice 2022-09-23, icu, Neurosurgery

Chief complaint: Admitted to the intensive care unit following a motor vehicle collision with multiple head injuries

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Jennifer Patel, MD
Department: Neurosurgery
Visit Type: icu

CHIEF COMPLAINT:
Admitted to the intensive care unit following a motor vehicle collision with multiple head injuries

HISTORY OF PRESENT ILLNESS:
The patient is a 56-year-old male with urethral stricture and colorectal adenocarcinoma admitted after trauma. He has been intubated and mechanically ventilated. He is currently being hyperventilated to reduce intracranial pressure. Despite this, his intracranial hypertension persists, and the medical team decides to initiate pharmacologic therapy.

PAST MEDICAL HISTORY:
Active Problem List:
- Urethral stricture (diagnosed 2020-01-15)
- Injury of external branch of superior laryngeal nerve (diagnosed 2020-04-15)
- Urethral stricture secondary to traumatic pelvic fracture
- History of colorectal adenocarcinoma, status post resection
- Diverticulosis
- History of severe acute pancreatitis

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of complex pelvic fracture (age 53)
- Right hemicolectomy with primary anastomosis for Stage II colorectal cancer
- Exploratory laparotomy with sigmoid colectomy and end colostomy for perforated diverticulitis
- Parathyroidectomy for primary hyperparathyroidism
- Decompressive craniectomy for traumatic cerebral edema

MEDICATIONS:
Home Medications:
- Tamsulosin 0.4 mg PO daily
- Aspirin 81 mg PO daily
- Acetaminophen 500 mg PO PRN pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Colorectal cancer (diagnosed age 62)
- Mother: Type 2 diabetes mellitus
- Sister: Breast cancer

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 3 years ago. Alcohol: Social (2-3 drinks per week). Occupation: Office administrator.

PHYSICAL EXAMINATION:
Neurologic evaluation reveals decreased level of consciousness with fixed and dilated pupils.

LABORATORY DATA:
Soon after administration, laboratory analysis reveals changes in serum parameters and urine output.

IMAGING:
CT scan of the head shows evidence of diffuse cerebral edema with midline shift.

## Encounter 3: source 2021-07-15, practice 2023-03-09, inpatient, General Surgery

Chief complaint: Has not passed stool or flatus and reports abdominal distension following recent right hemicolectomy

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Robert Kim, MD
Department: General Surgery
Visit Type: inpatient

CHIEF COMPLAINT:
Has not passed stool or flatus and reports abdominal distension following recent right hemicolectomy

HISTORY OF PRESENT ILLNESS:
This 56-year-old male with colorectal cancer and urethral stricture presents following recent right hemicolectomy. Since the procedure, he has not passed stool or flatus, and his abdomen feels distended. He has no nausea or vomiting at present, but he reports discomfort due to abdominal bloating.

PAST MEDICAL HISTORY:
Active Problem List:
- Urethral stricture (diagnosed 2020-01-15)
- Injury of external branch of superior laryngeal nerve (diagnosed 2020-04-15)
- Traumatic cerebral edema with elevated intracranial pressure (diagnosed 2021-01-15)
- Urethral stricture secondary to traumatic pelvic fracture
- History of colorectal adenocarcinoma, status post resection
- Diverticulosis
- History of severe acute pancreatitis

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of complex pelvic fracture (age 53)
- Right hemicolectomy with primary anastomosis for Stage II colorectal cancer
- Exploratory laparotomy with sigmoid colectomy and end colostomy for perforated diverticulitis
- Parathyroidectomy for primary hyperparathyroidism
- Decompressive craniectomy for traumatic cerebral edema

MEDICATIONS:
Home Medications:
- Tamsulosin 0.4 mg PO daily
- Aspirin 81 mg PO daily
- Acetaminophen 500 mg PO PRN pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Colorectal cancer (diagnosed age 62)
- Mother: Type 2 diabetes mellitus
- Sister: Breast cancer

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 3 years ago. Alcohol: Social (2-3 drinks per week). Occupation: Office administrator.

PHYSICAL EXAMINATION:
On exam, bowel sounds are hypoactive, and there is mild diffuse tenderness without rebound or guarding.

PLAN:
He is started on bethanechol.

## Encounter 4: source 2022-01-15, practice 2023-09-11, icu, Pulmonary and Critical Care

Chief complaint: Acute respiratory distress syndrome developing in the setting of severe acute pancreatitis

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Emily Rodriguez, MD
Department: Pulmonary and Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Acute respiratory distress syndrome developing in the setting of severe acute pancreatitis

HISTORY OF PRESENT ILLNESS:
The patient with a history of colorectal adenocarcinoma s/p hemicolectomy and severe acute pancreatitis is recovering in the intensive care unit after developing acute respiratory distress syndrome (ARDS) due to severe pancreatitis. He was intubated for 6 days and is currently being weaned from mechanical ventilation. He is receiving supportive care only and is otherwise hemodynamically stable.

PAST MEDICAL HISTORY:
Active Problem List:
- Urethral stricture (diagnosed 2020-01-15)
- Injury of external branch of superior laryngeal nerve (diagnosed 2020-04-15)
- Traumatic cerebral edema with elevated intracranial pressure (diagnosed 2021-01-15)
- Urethral stricture secondary to traumatic pelvic fracture
- History of colorectal adenocarcinoma, status post resection
- Diverticulosis
- History of severe acute pancreatitis

He has no prior history of thyroid disease.

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of complex pelvic fracture (age 53)
- Right hemicolectomy with primary anastomosis for Stage II colorectal cancer
- Exploratory laparotomy with sigmoid colectomy and end colostomy for perforated diverticulitis
- Parathyroidectomy for primary hyperparathyroidism
- Decompressive craniectomy for traumatic cerebral edema

MEDICATIONS:
Home Medications:
- Tamsulosin 0.4 mg PO daily
- Aspirin 81 mg PO daily
- Acetaminophen 500 mg PO PRN pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Colorectal cancer (diagnosed age 62)
- Mother: Type 2 diabetes mellitus
- Sister: Breast cancer

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 3 years ago. Alcohol: Social (2-3 drinks per week). Occupation: Office administrator.

REVIEW OF SYSTEMS:
No new symptoms are reported.

VITAL SIGNS:
He is afebrile. Vital signs are within normal limits,

PHYSICAL EXAMINATION:
Alert, and responsive but fatigued. Physical examination is unremarkable.

PLAN:
The medical team orders a thyroid panel as part of routine follow-up labs.

## Encounter 5: source 2023-01-15, practice 2024-09-22, icu, Pulmonary and Critical Care

Chief complaint: Admitted after near-drowning incident with progressive dyspnea and refractory hypoxemia

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. David Park, MD
Department: Pulmonary and Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Admitted after near-drowning incident with progressive dyspnea and refractory hypoxemia

HISTORY OF PRESENT ILLNESS:
He is a 58-year-old male with history of colorectal adenocarcinoma, urethral stricture, and prior ARDS presenting following a near-drowning incident. He was intubated at the scene and placed on mechanical ventilation with a high inspired oxygen fraction (FiO₂ 1.0) to correct severe hypoxemia. During the following 72 hours, his oxygenation initially improves, but he later develops progressive dyspnea and diffuse rales.

PAST MEDICAL HISTORY:
Active Problem List:
- Urethral stricture (diagnosed 2020-01-15)
- Injury of external branch of superior laryngeal nerve (diagnosed 2020-04-15)
- Traumatic cerebral edema with elevated intracranial pressure (diagnosed 2021-01-15)
- Euthyroid sick syndrome (diagnosed 2022-01-15)
- Urethral stricture secondary to traumatic pelvic fracture
- History of colorectal adenocarcinoma, status post resection
- Diverticulosis
- History of severe acute pancreatitis

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of complex pelvic fracture (age 53)
- Right hemicolectomy with primary anastomosis for Stage II colorectal cancer
- Exploratory laparotomy with sigmoid colectomy and end colostomy for perforated diverticulitis
- Parathyroidectomy for primary hyperparathyroidism
- Decompressive craniectomy for traumatic cerebral edema

MEDICATIONS:
Home Medications:
- Tamsulosin 0.4 mg PO daily
- Aspirin 81 mg PO daily
- Acetaminophen 500 mg PO PRN pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Colorectal cancer (diagnosed age 62)
- Mother: Type 2 diabetes mellitus
- Sister: Breast cancer

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 3 years ago. Alcohol: Social (2-3 drinks per week). Occupation: Office administrator.

PHYSICAL EXAMINATION:
Physical examination shows increased work of breathing, and his oxygen saturation declines despite continued ventilation.

IMAGING:
Chest imaging reveals bilateral patchy opacities consistent with noncardiogenic pulmonary edema.

PATHOLOGY:
Microscopic examination of a lung biopsy shows necrosis of alveolar epithelium, interstitial inflammation, and hyaline membrane formation.

## Encounter 6: source 2024-01-15, practice 2025-09-13, icu, General Surgery

Chief complaint: Oxygen saturation gradually decreasing over several hours despite stable ventilator settings following emergency abdominal surgery

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Lisa Wong, MD
Department: General Surgery
Visit Type: icu

CHIEF COMPLAINT:
Oxygen saturation gradually decreasing over several hours despite stable ventilator settings following emergency abdominal surgery

HISTORY OF PRESENT ILLNESS:
This 59-year-old with history of colorectal resection, prior ARDS from pancreatitis, and traumatic pelvic fracture presents to ICU following emergency abdominal surgery for perforated diverticulitis. He is intubated and mechanically ventilated in the ICU. He has been sedated and stable on ventilator for past 2 days. He remains afebrile, hemodynamically stable, and has no signs of infection.

PAST MEDICAL HISTORY:
Active Problem List:
- Urethral stricture (diagnosed 2020-01-15)
- Injury of external branch of superior laryngeal nerve (diagnosed 2020-04-15)
- Traumatic cerebral edema with elevated intracranial pressure (diagnosed 2021-01-15)
- Euthyroid sick syndrome (diagnosed 2022-01-15)
- Refractory hypoxemia in ARDS (diagnosed 2023-01-15)
- Urethral stricture secondary to traumatic pelvic fracture
- History of colorectal adenocarcinoma, status post resection
- Diverticulosis
- History of severe acute pancreatitis

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of complex pelvic fracture (age 53)
- Right hemicolectomy with primary anastomosis for Stage II colorectal cancer
- Exploratory laparotomy with sigmoid colectomy and end colostomy for perforated diverticulitis
- Parathyroidectomy for primary hyperparathyroidism
- Decompressive craniectomy for traumatic cerebral edema

MEDICATIONS:
Home Medications:
- Tamsulosin 0.4 mg PO daily
- Aspirin 81 mg PO daily
- Acetaminophen 500 mg PO PRN pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Colorectal cancer (diagnosed age 62)
- Mother: Type 2 diabetes mellitus
- Sister: Breast cancer

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 3 years ago. Alcohol: Social (2-3 drinks per week). Occupation: Office administrator.

VITAL SIGNS:
The ventilator is delivering 100% FiO₂.

PHYSICAL EXAMINATION:
Chest auscultation reveals decreased breath sounds in the right lower lung field. The endotracheal tube is confirmed to be in good position.

LABORATORY DATA:
Arterial blood gas shows mild hypoxemia with a normal A–a gradient.

IMAGING:
Chest x-ray reveals a dense, homogenous opacity in the right lower lobe with associated loss of volume and slight tracheal deviation toward the right.

## Encounter 7: source 2024-03-15, practice 2025-11-17, follow_up, General Surgery

Chief complaint: Constant dull abdominal pain and distension worsening since recent surgery for perforated diverticulitis

ENCOUNTER NOTE
Date: 2024-03-15
Provider: Dr. Lisa Wong, MD
Department: General Surgery
Visit Type: follow_up

CHIEF COMPLAINT:
Constant dull abdominal pain and distension worsening since recent surgery for perforated diverticulitis

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up following recent surgery for perforated diverticulitis in the context of known diverticulosis. He reports the pain is constant, dull, and has been worsening since surgery. He denies nausea, vomiting, or changes in bowel habits.

PAST MEDICAL HISTORY:
Active Problem List:
- Urethral stricture (diagnosed 2020-01-15)
- Injury of external branch of superior laryngeal nerve (diagnosed 2020-04-15)
- Traumatic cerebral edema with elevated intracranial pressure (diagnosed 2021-01-15)
- Euthyroid sick syndrome (diagnosed 2022-01-15)
- Refractory hypoxemia in ARDS (diagnosed 2023-01-15)
- Post-obstructive atelectasis (diagnosed 2024-01-15)
- Urethral stricture secondary to traumatic pelvic fracture
- History of colorectal adenocarcinoma, status post resection
- Diverticulosis
- History of severe acute pancreatitis

PAST SURGICAL HISTORY:
Status post hemicolectomy three days ago

MEDICATIONS:
Home Medications:
- Tamsulosin 0.4 mg PO daily
- Aspirin 81 mg PO daily
- Acetaminophen 500 mg PO PRN pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Colorectal cancer (diagnosed age 62)
- Mother: Type 2 diabetes mellitus
- Sister: Breast cancer

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-years, quit 3 years ago. Alcohol: Social (2-3 drinks per week). Occupation: Office administrator.

VITAL SIGNS:
His temperature is 37.8°C (100°F), blood pressure is 135/85 mmHg, pulse is 98/min, and respiratory rate is 20/min.

PHYSICAL EXAMINATION:
On physical examination, the abdomen is distended, with diffuse tenderness to palpation, but no rebound tenderness or guarding. Bowel sounds are diminished.

LABORATORY DATA:
Laboratory values are as follows:

- White blood cell count: 12,000/mm³
- Hemoglobin: 13.2 g/dL
- Platelet count: 250,000/mm³
- Serum creatinine: 1.0 mg/dL
- Blood urea nitrogen: 14 mg/dL

IMAGING:
A postoperative abdominal X-ray shows a radio-opaque object in the right lower quadrant.
