# synthetic_hospital patient 2299

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 67 at first encounter, sex M, Non-Hispanic White, Medicare with supplemental private insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-03-29 (Wed) | outpatient | Internal Medicine |
| 1 | 2020-03-15 | 2023-05-30 (Tue) | ed | Emergency Medicine |
| 2 | 2020-07-15 | 2023-09-17 (Sun) | ed | Emergency Medicine |
| 3 | 2021-01-15 | 2024-03-19 (Tue) | ed | Emergency Medicine |
| 4 | 2021-07-15 | 2024-09-20 (Fri) | ed | Emergency Medicine |
| 5 | 2022-01-15 | 2025-03-30 (Sun) | icu | Pulmonary & Critical Care |
| 6 | 2022-07-15 | 2025-09-20 (Sat) | icu | Hematology & Oncology |
| 7 | 2023-01-15 | 2026-04-02 (Thu) | icu | Cardiology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare with supplemental private insurance",
 "occupation": "Retired construction foreman",
 "alcohol_use": "Occasional social use (1-2 drinks per week)",
 "family_history": [
  "Father: Lung cancer",
  "Mother: Chronic obstructive pulmonary disease",
  "Brother: Deep vein thrombosis"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 40 pack-year history, quit 1 year ago",
 "home_medications": [
  {
   "dose": "18 mcg inhaled once daily",
   "name": "Tiotropium"
  },
  {
   "dose": "90 mcg inhaled every 4-6 hours as needed for shortness of breath",
   "name": "Albuterol"
  },
  {
   "dose": "5 mg orally daily (INR goal 2.0-3.0)",
   "name": "Warfarin"
  },
  {
   "dose": "10 mg orally daily",
   "name": "Amlodipine"
  },
  {
   "dose": "5 mg orally daily (chronic maintenance)",
   "name": "Prednisone"
  },
  {
   "dose": "2 L/min via nasal cannula continuous",
   "name": "Oxygen"
  }
 ],
 "surgical_history": [
  "Open cholecystectomy (complicated by deep fascial wound dehiscence)",
  "Open radical prostatectomy",
  "Port-a-cath placement for chemotherapy administration"
 ],
 "chronic_conditions": [
  "Chronic obstructive pulmonary disease (COPD) on long-term home oxygen therapy",
  "Metastatic lung cancer",
  "History of deep vein thrombosis (DVT)",
  "Hypertension",
  "Post-thrombotic syndrome"
 ],
 "age_at_first_encounter": 67
}
```

Primary diagnoses: ["Massive Pulmonary Embolism", "COPD exacerbation with acute hypercapnic respiratory failure without altered mental status", "Cardiac tamponade", "Deep fascial wound dehiscence", "Calcium channel blocker-induced peripheral edema", "Saddle Pulmonary Embolus with Active Gastrointestinal Bleeding", "Pulmonary embolism"]
Comorbidities: ["Chronic obstructive pulmonary disease", "Cardiogenic Shock", "Sinus tachycardia secondary to burns", "Chronic hypoxemic respiratory failure", "Status post laparoscopic cholecystectomy"]

## Encounter 0: source 2020-01-15, practice 2023-03-29, outpatient, Internal Medicine

Chief complaint: Bilateral lower extremity swelling and concern for medication side effect

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Bilateral lower extremity swelling and concern for medication side effect

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (COPD) on long-term home oxygen therapy
- Metastatic lung cancer
- History of deep vein thrombosis (DVT)
- Hypertension
- Post-thrombotic syndrome

PAST SURGICAL HISTORY:
- Open cholecystectomy (complicated by deep fascial wound dehiscence)
- Open radical prostatectomy
- Port-a-cath placement for chemotherapy administration

MEDICATIONS:
He is brought to the operating room, where anesthesia is induced with propofol and fentanyl. To facilitate endotracheal intubation, the anesthesiologist administers a neuromuscular blocking agent that causes no visible muscle fasciculations and provides rapid but reversible paralysis. The procedure is completed without complication, and the patient is extubated in the recovery room after administration of neostigmine.

Home Medications:
- Tiotropium 18 mcg inhaled once daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed for shortness of breath
- Warfarin 5 mg orally daily (INR goal 2.0-3.0)
- Amlodipine 10 mg orally daily
- Prednisone 5 mg orally daily (chronic maintenance)
- Oxygen 2 L/min via nasal cannula continuous

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer
- Mother: Chronic obstructive pulmonary disease
- Brother: Deep vein thrombosis

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-year history, quit 1 year ago. Alcohol: Occasional social use (1-2 drinks per week). Occupation: Retired construction foreman.

PLAN:
Scheduled for elective laparoscopic colectomy due to recurrent diverticulitis

## Encounter 1: source 2020-03-15, practice 2023-05-30, ed, Emergency Medicine

Chief complaint: Sudden onset of severe abdominal pain and sensation of 'something giving way' 7 days after open cholecystectomy

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Michael Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of severe abdominal pain and sensation of 'something giving way' 7 days after open cholecystectomy

HISTORY OF PRESENT ILLNESS:
This 67-year-old male with COPD on home oxygen, metastatic lung cancer, and prior DVT presents to the emergency department following recent evaluation for calcium channel blocker-induced edema. He reports sudden onset of severe abdominal pain and a sensation of "something giving way" in his abdomen 7 days postoperative from an open cholecystectomy. He states the pain started abruptly after he attempted to lift a heavy object.

PAST MEDICAL HISTORY:
Active Problem List:
- Calcium channel blocker-induced peripheral edema (diagnosed 2020-01-15)
- Chronic obstructive pulmonary disease (COPD) on long-term home oxygen therapy
- Metastatic lung cancer
- History of deep vein thrombosis (DVT)
- Hypertension
- Post-thrombotic syndrome

PAST SURGICAL HISTORY:
- Open cholecystectomy (complicated by deep fascial wound dehiscence)
- Open radical prostatectomy
- Port-a-cath placement for chemotherapy administration

MEDICATIONS:
Home Medications:
- Tiotropium 18 mcg inhaled once daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed for shortness of breath
- Warfarin 5 mg orally daily (INR goal 2.0-3.0)
- Amlodipine 10 mg orally daily
- Prednisone 5 mg orally daily (chronic maintenance)
- Oxygen 2 L/min via nasal cannula continuous

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer
- Mother: Chronic obstructive pulmonary disease
- Brother: Deep vein thrombosis

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-year history, quit 1 year ago. Alcohol: Occasional social use (1-2 drinks per week). Occupation: Retired construction foreman.

VITAL SIGNS:
On examination, the patient is afebrile, blood pressure is 130/85 mmHg, heart rate is 110 bpm, and respiratory rate is 22 breaths/min.

PHYSICAL EXAMINATION:
His abdomen is distended with a visible bulge at the previous surgical site. There is no overlying erythema or purulent discharge, but the bulge is tender to palpation and does not reduce with gentle pressure. Bowel sounds are diminished.

LABORATORY DATA:
Laboratory results are as follows:

- Hemoglobin: 13.2 g/dL
- White blood cell count: 11,000 /µL
- Serum creatinine: 1.0 mg/dL
- Blood urea nitrogen: 18 mg/dL
- Serum electrolytes are within normal limits

IMAGING:
An urgent computed tomography (CT) scan of the abdomen shows protrusion of bowel loops through a defect in the fascial layer of the abdominal wall.

## Encounter 2: source 2020-07-15, practice 2023-09-17, ed, Emergency Medicine

Chief complaint: Acute-onset shortness of breath and pleuritic chest pain 6 days after open radical prostatectomy

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Emily Watson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute-onset shortness of breath and pleuritic chest pain 6 days after open radical prostatectomy

HISTORY OF PRESENT ILLNESS:
This patient with history of DVT and prior postoperative wound dehiscence underwent open prostatectomy 6 days ago, was discharged home 3 days ago, and has had limited mobility since surgery.

PAST MEDICAL HISTORY:
Active Problem List:
- Calcium channel blocker-induced peripheral edema (diagnosed 2020-01-15)
- Deep fascial wound dehiscence (diagnosed 2020-03-15)
- Chronic obstructive pulmonary disease (COPD) on long-term home oxygen therapy
- Metastatic lung cancer
- History of deep vein thrombosis (DVT)
- Hypertension
- Post-thrombotic syndrome

PAST SURGICAL HISTORY:
- Open cholecystectomy (complicated by deep fascial wound dehiscence)
- Open radical prostatectomy
- Port-a-cath placement for chemotherapy administration

MEDICATIONS:
Home Medications:
- Tiotropium 18 mcg inhaled once daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed for shortness of breath
- Warfarin 5 mg orally daily (INR goal 2.0-3.0)
- Amlodipine 10 mg orally daily
- Prednisone 5 mg orally daily (chronic maintenance)
- Oxygen 2 L/min via nasal cannula continuous

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer
- Mother: Chronic obstructive pulmonary disease
- Brother: Deep vein thrombosis

SOCIAL HISTORY:
He has a 40-pack-year smoking history.

VITAL SIGNS:
Temperature is 37.2°C (99.0°F), blood pressure is 92/64 mm Hg, pulse is 115/min, and respirations are 28/min. His oxygen saturation is 88% on room air.

PHYSICAL EXAMINATION:
Jugular venous distension is present. Lungs are clear to auscultation, but a loud P2 is appreciated.

LABORATORY DATA:
Arterial blood gas analysis reveals pH 7.47, PaCO₂ 29 mm Hg, and PaO₂ 62 mm Hg.

IMAGING:
CT pulmonary angiography shows multiple bilateral segmental filling defects.

## Encounter 3: source 2021-01-15, practice 2024-03-19, ed, Emergency Medicine

Chief complaint: Increased shortness of breath and productive cough for the past three days with decreased activity tolerance

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. James Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Increased shortness of breath and productive cough for the past three days with decreased activity tolerance

HISTORY OF PRESENT ILLNESS:
The patient with chronic obstructive pulmonary disease on long-term home oxygen and metastatic lung cancer returns to the emergency department, having previously presented for pulmonary embolism and lower extremity edema. He reports a decreased ability to perform his daily activities and has been sleeping more than usual.

PAST MEDICAL HISTORY:
Active Problem List:
- Calcium channel blocker-induced peripheral edema (diagnosed 2020-01-15)
- Deep fascial wound dehiscence (diagnosed 2020-03-15)
- Pulmonary embolism (diagnosed 2020-07-15)
- Chronic obstructive pulmonary disease (COPD) on long-term home oxygen therapy
- Metastatic lung cancer
- History of deep vein thrombosis (DVT)
- Hypertension
- Post-thrombotic syndrome

PAST SURGICAL HISTORY:
- Open cholecystectomy (complicated by deep fascial wound dehiscence)
- Open radical prostatectomy
- Port-a-cath placement for chemotherapy administration

MEDICATIONS:
His home medications include tiotropium, albuterol, and prednisone.

Home Medications:
- Warfarin 5 mg orally daily (INR goal 2.0-3.0)
- Amlodipine 10 mg orally daily
- Oxygen 2 L/min via nasal cannula continuous

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer
- Mother: Chronic obstructive pulmonary disease
- Brother: Deep vein thrombosis

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-year history, quit 1 year ago. Alcohol: Occasional social use (1-2 drinks per week). Occupation: Retired construction foreman.

REVIEW OF SYSTEMS:
He denies chest pain, fever, or chills.

VITAL SIGNS:
With a respiratory rate of 22 breaths per minute, heart rate of 110 bpm, blood pressure of 145/90 mm Hg, and oxygen saturation of 88% on room air

PHYSICAL EXAMINATION:
On examination, he is somnolent but arousable. Auscultation of the lungs reveals diffuse wheezing and decreased breath sounds.

LABORATORY DATA:
Arterial blood gas (ABG) results are as follows: - pH: 7.28 - PaCO2: 82 mm Hg - PaO2: 60 mm Hg - HCO3-: 30 mEq/L

## Encounter 4: source 2021-07-15, practice 2024-09-20, ed, Emergency Medicine

Chief complaint: Worsening shortness of breath, productive cough with yellow-green sputum, and inability to sleep lying flat

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening shortness of breath, productive cough with yellow-green sputum, and inability to sleep lying flat

HISTORY OF PRESENT ILLNESS:
The patient is a 68-year-old male with chronic obstructive pulmonary disease on long-term home oxygen therapy and metastatic lung cancer who returns six months after his prior COPD exacerbation. Three days ago, he developed a low-grade fever and fatigue. Over the past 24 hours, he has become increasingly dyspneic and has been unable to sleep lying flat. His wife notes that he has been coughing more than usual, with yellow-green sputum. He is alert but appears exhausted and is using accessory muscles to breathe.

PAST MEDICAL HISTORY:
Active Problem List:
- Calcium channel blocker-induced peripheral edema (diagnosed 2020-01-15)
- Deep fascial wound dehiscence (diagnosed 2020-03-15)
- Pulmonary embolism (diagnosed 2020-07-15)
- COPD exacerbation with acute hypercapnic respiratory failure without altered mental status (diagnosed 2021-01-15)
- Chronic obstructive pulmonary disease (COPD) on long-term home oxygen therapy
- Metastatic lung cancer
- History of deep vein thrombosis (DVT)
- Hypertension
- Post-thrombotic syndrome

He has a long-standing history of chronic obstructive pulmonary disease (COPD) and uses home oxygen therapy at 2 L/min via nasal cannula.

PAST SURGICAL HISTORY:
- Open cholecystectomy (complicated by deep fascial wound dehiscence)
- Open radical prostatectomy
- Port-a-cath placement for chemotherapy administration

MEDICATIONS:
Home Medications:
- Tiotropium 18 mcg inhaled once daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed for shortness of breath
- Warfarin 5 mg orally daily (INR goal 2.0-3.0)
- Amlodipine 10 mg orally daily
- Prednisone 5 mg orally daily (chronic maintenance)
- Oxygen 2 L/min via nasal cannula continuous

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer
- Mother: Chronic obstructive pulmonary disease
- Brother: Deep vein thrombosis

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-year history, quit 1 year ago. Alcohol: Occasional social use (1-2 drinks per week). Occupation: Retired construction foreman.

VITAL SIGNS:
On examination, his respiratory rate is 32/min, heart rate is 112/min, and temperature is 100.0°F (37.8°C). Pulse oximetry shows oxygen saturation of 85% on room air.

PHYSICAL EXAMINATION:
Physical examination reveals bilateral expiratory wheezing and decreased breath sounds at the bases. There is no jugular venous distension or peripheral edema.

LABORATORY DATA:
Arterial blood gas shows an acid-base disturbance.

IMAGING:
A chest x-ray shows hyperinflated lungs without focal infiltrates.

PLAN:
He is promptly placed on noninvasive positive pressure ventilation.

## Encounter 5: source 2022-01-15, practice 2025-03-30, icu, Pulmonary & Critical Care

Chief complaint: Acute onset of shortness of breath, pleuritic chest pain, and near-syncope after missing warfarin doses

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Robert Kim, MD
Department: Pulmonary & Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Acute onset of shortness of breath, pleuritic chest pain, and near-syncope after missing warfarin doses

HISTORY OF PRESENT ILLNESS:
The patient is a 69-year-old male with COPD on oxygen and prior pulmonary embolism presenting with recurrence. He reports symptoms that started 2 hours ago, feeling lightheaded with a near-syncope episode earlier in the day, having missed warfarin for the past week.

PAST MEDICAL HISTORY:
Active Problem List:
- Calcium channel blocker-induced peripheral edema (diagnosed 2020-01-15)
- Deep fascial wound dehiscence (diagnosed 2020-03-15)
- Pulmonary embolism (diagnosed 2020-07-15)
- COPD exacerbation with acute hypercapnic respiratory failure without altered mental status (diagnosed 2021-01-15)
- Chronic obstructive pulmonary disease (COPD) on long-term home oxygen therapy
- Metastatic lung cancer
- History of deep vein thrombosis (DVT)
- Hypertension
- Post-thrombotic syndrome

PAST SURGICAL HISTORY:
- Open cholecystectomy (complicated by deep fascial wound dehiscence)
- Open radical prostatectomy
- Port-a-cath placement for chemotherapy administration

MEDICATIONS:
His medications include tiotropium, salbutamol, and warfarin.

Home Medications:
- Albuterol 90 mcg inhaled every 4-6 hours as needed for shortness of breath
- Amlodipine 10 mg orally daily
- Prednisone 5 mg orally daily (chronic maintenance)
- Oxygen 2 L/min via nasal cannula continuous

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer
- Mother: Chronic obstructive pulmonary disease
- Brother: Deep vein thrombosis

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-year history, quit 1 year ago. Alcohol: Occasional social use (1-2 drinks per week). Occupation: Retired construction foreman.

VITAL SIGNS:
His blood pressure is 85/50 mmHg, heart rate is 120 beats per minute, respiratory rate is 28 breaths per minute, and oxygen saturation is 88% on room air

PHYSICAL EXAMINATION:
Jugular venous pressure is elevated. Lung auscultation reveals decreased breath sounds without wheezes or crackles.

LABORATORY DATA:
Laboratory Values:
- Troponin I: Elevated
- B-type natriuretic peptide (BNP): Elevated
- D-dimer: Elevated
- INR: 1.2 (subtherapeutic)

IMAGING:
Bedside echocardiography demonstrates right ventricular dilation and hypokinesis.

OTHER STUDIES:
A 12-lead ECG shows sinus tachycardia with a right bundle branch block.

## Encounter 6: source 2022-07-15, practice 2025-09-20, icu, Hematology & Oncology

Chief complaint: Sudden onset shortness of breath, pleuritic chest pain, and hematemesis

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Lisa Johnson, MD
Department: Hematology & Oncology
Visit Type: icu

CHIEF COMPLAINT:
Sudden onset shortness of breath, pleuritic chest pain, and hematemesis

HISTORY OF PRESENT ILLNESS:
The patient with COPD returns six months after massive pulmonary embolism. He reports a productive cough with yellow sputum for the past week, which he attributed to a COPD exacerbation. The patient suddenly develops hematemesis.

PAST MEDICAL HISTORY:
Active Problem List:
- Calcium channel blocker-induced peripheral edema (diagnosed 2020-01-15)
- Deep fascial wound dehiscence (diagnosed 2020-03-15)
- Pulmonary embolism (diagnosed 2020-07-15)
- COPD exacerbation with acute hypercapnic respiratory failure without altered mental status (diagnosed 2021-01-15)
- Massive Pulmonary Embolism (diagnosed 2022-01-15)
- Chronic obstructive pulmonary disease (COPD) on long-term home oxygen therapy
- Metastatic lung cancer
- History of deep vein thrombosis (DVT)
- Hypertension
- Post-thrombotic syndrome

PAST SURGICAL HISTORY:
Recent right hip replacement surgery

MEDICATIONS:
His medications include tiotropium, salbutamol as needed, and he was started on enoxaparin postoperatively.

Home Medications:
- Albuterol 90 mcg inhaled every 4-6 hours as needed for shortness of breath
- Warfarin 5 mg orally daily (INR goal 2.0-3.0)
- Amlodipine 10 mg orally daily
- Prednisone 5 mg orally daily (chronic maintenance)
- Oxygen 2 L/min via nasal cannula continuous

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer
- Mother: Chronic obstructive pulmonary disease
- Brother: Deep vein thrombosis

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-year history, quit 1 year ago. Alcohol: Occasional social use (1-2 drinks per week). Occupation: Retired construction foreman.

VITAL SIGNS:
Tachypneic with a respiratory rate of 28 breaths per minute, oxygen saturation is 89% on room air, heart rate is 110 beats per minute, and blood pressure is 135/85 mmHg

PHYSICAL EXAMINATION:
His right lower extremity is swollen and tender along the distribution of the deep veins.

LABORATORY DATA:
His current laboratory values are as follows:

- Hemoglobin: 7.8 g/dL (Normal: 13.8-17.2 g/dL)
- Hematocrit: 23.4% (Normal: 40.7-50.3%)
- Platelets: 255 x 10^3/µL (Normal: 150-450 x 10^3/µL)
- INR: 1.1 (Normal: 0.8-1.2)
- aPTT: 30 seconds (Normal: 25-35 seconds)

IMAGING:
A chest X-ray shows no evidence of pneumonia or pneumothorax. An ultrasound of the right lower extremity confirms the presence of a proximal deep venous thrombosis. A CT pulmonary angiogram reveals a large saddle pulmonary embolus.

## Encounter 7: source 2023-01-15, practice 2026-04-02, icu, Cardiology

Chief complaint: Collapse at home with drowsiness and altered mental status

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. David Smith, MD
Department: Cardiology
Visit Type: icu

CHIEF COMPLAINT:
Collapse at home with drowsiness and altered mental status

HISTORY OF PRESENT ILLNESS:
This 70-year-old male with metastatic lung cancer, severe COPD on home oxygen therapy, and recent hospitalizations for massive pulmonary embolism and gastrointestinal bleeding presents following collapse at home with altered mental status. On arrival, he is drowsy.

PAST MEDICAL HISTORY:
Active Problem List:
- Calcium channel blocker-induced peripheral edema (diagnosed 2020-01-15)
- Deep fascial wound dehiscence (diagnosed 2020-03-15)
- Pulmonary embolism (diagnosed 2020-07-15)
- COPD exacerbation with acute hypercapnic respiratory failure without altered mental status (diagnosed 2021-01-15)
- Massive Pulmonary Embolism (diagnosed 2022-01-15)
- Saddle Pulmonary Embolus with Active Gastrointestinal Bleeding (diagnosed 2022-07-15)
- Chronic obstructive pulmonary disease (COPD) on long-term home oxygen therapy
- Metastatic lung cancer
- History of deep vein thrombosis (DVT)
- Hypertension
- Post-thrombotic syndrome

He has a history of metastatic lung cancer.

PAST SURGICAL HISTORY:
- Open cholecystectomy (complicated by deep fascial wound dehiscence)
- Open radical prostatectomy
- Port-a-cath placement for chemotherapy administration

MEDICATIONS:
Home Medications:
- Tiotropium 18 mcg inhaled once daily
- Albuterol 90 mcg inhaled every 4-6 hours as needed for shortness of breath
- Warfarin 5 mg orally daily (INR goal 2.0-3.0)
- Amlodipine 10 mg orally daily
- Prednisone 5 mg orally daily (chronic maintenance)
- Oxygen 2 L/min via nasal cannula continuous

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer
- Mother: Chronic obstructive pulmonary disease
- Brother: Deep vein thrombosis

SOCIAL HISTORY:
Smoking: Former smoker, 40 pack-year history, quit 1 year ago. Alcohol: Occasional social use (1-2 drinks per week). Occupation: Retired construction foreman.

VITAL SIGNS:
Vitals are: BP 80/50, HR118, and RR 24

PHYSICAL EXAMINATION:
Physical examination reveals jugular venous distension, muffled heart sounds, and a weak, thready pulse. His extremities are cool, and capillary refill is delayed. On inspiration, there is a noticeable drop in systolic blood pressure by 15 mm Hg. Lungs are clear to auscultation, and there is no peripheral edema.
