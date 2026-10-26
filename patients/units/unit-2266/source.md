# synthetic_hospital patient 2266

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 66 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-03-09 (Mon) | outpatient | Orthopedics |
| 1 | 2020-05-15 | 2020-06-25 (Thu) | outpatient | Gastroenterology |
| 2 | 2021-09-15 | 2021-10-28 (Thu) | procedure | Urology |
| 3 | 2022-09-15 | 2022-11-07 (Mon) | outpatient | Urology |
| 4 | 2023-09-15 | 2023-11-06 (Mon) | inpatient | Vascular Surgery |
| 5 | 2024-09-15 | 2024-10-27 (Sun) | ed | Emergency Medicine |
| 6 | 2025-09-15 | 2025-10-31 (Fri) | ed | Emergency Medicine |
| 7 | 2026-03-15 | 2026-05-06 (Wed) | inpatient | Oncology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Sulfa medications (hives)",
  "Iodinated contrast media (premedication required)"
 ],
 "insurance": "Medicare",
 "occupation": "Retired long-haul truck driver",
 "alcohol_use": "Occasional (1-2 drinks per week)",
 "family_history": [
  "Father: Abdominal aortic aneurysm (ruptured, age 72)",
  "Mother: Hypertension, breast cancer (age 68)",
  "Brother: Lung cancer",
  "Paternal uncle: Pancreatic cancer"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 35 pack-years, quit 18 months ago",
 "home_medications": [
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Furosemide"
  },
  {
   "dose": "50 mg PO daily",
   "name": "Metoprolol succinate"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  },
  {
   "dose": "1 mg PO daily",
   "name": "Folic acid"
  },
  {
   "dose": "5 mg PO q6h PRN pain",
   "name": "Oxycodone"
  }
 ],
 "surgical_history": [
  "Right radical nephrectomy (robotic-assisted)",
  "Pancreaticoduodenectomy (Whipple procedure)",
  "Thoracic aortic aneurysm repair with graft placement",
  "Endovascular abdominal aortic aneurysm repair (EVAR)",
  "Open sigmoid colectomy with primary anastomosis"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Congestive heart failure with reduced ejection fraction",
  "Cervical spondylosis",
  "Diverticulosis",
  "Stage IV colorectal adenocarcinoma (diagnosed 6 months prior)",
  "History of pancreatic adenocarcinoma (s/p pancreaticoduodenectomy)",
  "History of right-sided renal cell carcinoma (s/p right radical nephrectomy)",
  "History of thoracic aortic aneurysm (s/p open repair)",
  "History of abdominal aortic aneurysm (s/p endovascular repair)",
  "Status post central venous catheter placement for chemotherapy",
  "History of tobacco use disorder"
 ],
 "age_at_first_encounter": 66
}
```

Primary diagnoses: ["Spinal epidural hematoma", "Cervical spondylosis", "Uncomplicated parapneumonic effusion", "Prerenal acute kidney injury secondary to hypovolemia", "Colovesical fistula", "Catheter-related bloodstream infection due to Staphylococcus epidermidis with secondary pneumonia", "Pancreatic adenocarcinoma (mucin-producing) with chronic disseminated intravascular coagulation", "Renal Cell Carcinoma"]
Comorbidities: ["Essential hypertension", "Presence of peripherally inserted central catheter (PICC)"]

## Encounter 0: source 2020-01-15, practice 2020-03-09, outpatient, Orthopedics

Chief complaint: Several months of gradually worsening neck pain with occasional morning stiffness

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Orthopedics
Visit Type: outpatient

CHIEF COMPLAINT:
Several months of gradually worsening neck pain with occasional morning stiffness

HISTORY OF PRESENT ILLNESS:
He describes the discomfort as a dull, aching sensation in the posterior neck that sometimes radiates to the shoulders, particularly after long periods of desk work or driving. He denies any trauma, recent infections, or constitutional symptoms. He also reports occasional stiffness in the morning lasting about 10–15 minutes but says it improves throughout the day. There is no history of numbness, weakness, or bowel/bladder dysfunction.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Congestive heart failure with reduced ejection fraction
- Cervical spondylosis
- Diverticulosis
- Stage IV colorectal adenocarcinoma (diagnosed 6 months prior)
- History of pancreatic adenocarcinoma (s/p pancreaticoduodenectomy)
- History of right-sided renal cell carcinoma (s/p right radical nephrectomy)
- History of thoracic aortic aneurysm (s/p open repair)
- History of abdominal aortic aneurysm (s/p endovascular repair)
- Status post central venous catheter placement for chemotherapy
- History of tobacco use disorder

PAST SURGICAL HISTORY:
- Right radical nephrectomy (robotic-assisted)
- Pancreaticoduodenectomy (Whipple procedure)
- Thoracic aortic aneurysm repair with graft placement
- Endovascular abdominal aortic aneurysm repair (EVAR)
- Open sigmoid colectomy with primary anastomosis

MEDICATIONS:
He takes lisinopril.

Home Medications:
- Furosemide 40 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily
- Folic acid 1 mg PO daily
- Oxycodone 5 mg PO q6h PRN pain

ALLERGIES:
Sulfa medications (hives), Iodinated contrast media (premedication required)

FAMILY HISTORY:
- Father: Abdominal aortic aneurysm (ruptured, age 72)
- Mother: Hypertension, breast cancer (age 68)
- Brother: Lung cancer
- Paternal uncle: Pancreatic cancer

SOCIAL HISTORY:
Smoking: Former smoker, 35 pack-years, quit 18 months ago. Alcohol: Occasional (1-2 drinks per week). Occupation: Retired long-haul truck driver.

PHYSICAL EXAMINATION:
Neurological examination is normal, with preserved strength and sensation in all extremities. Reflexes are symmetric.

IMAGING:
MRI of the cervical spine reveals degenerative changes involving the vertebral bodies and intervertebral discs, with osteophyte formation and mild narrowing of the neural foramina at multiple levels.

## Encounter 1: source 2020-05-15, practice 2020-06-25, outpatient, Gastroenterology

Chief complaint: Progressive fatigue, pruritus, dark urine, and 10-lb unintentional weight loss over 2 months

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. Michael Rodriguez, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue, pruritus, dark urine, and 10-lb unintentional weight loss over 2 months

HISTORY OF PRESENT ILLNESS:
The patient is a 66-year-old male with a history of pancreatic adenocarcinoma s/p pancreaticoduodenectomy and Stage IV colorectal adenocarcinoma presenting to Gastroenterology clinic with a 2-month history of progressive fatigue, pruritus, and dark urine. He also reports a 4.5 kg (10 lb) unintentional weight loss over the same period. He denies abdominal pain or changes in bowel habits.

PAST MEDICAL HISTORY:
Active Problem List:
- Cervical spondylosis (diagnosed 2020-01-15)
- Hypertension
- Congestive heart failure with reduced ejection fraction
- Diverticulosis
- Stage IV colorectal adenocarcinoma (diagnosed 6 months prior)
- History of pancreatic adenocarcinoma (s/p pancreaticoduodenectomy)
- History of right-sided renal cell carcinoma (s/p right radical nephrectomy)
- History of thoracic aortic aneurysm (s/p open repair)
- History of abdominal aortic aneurysm (s/p endovascular repair)
- Status post central venous catheter placement for chemotherapy
- History of tobacco use disorder

His past medical history is significant for hypertension, controlled with hydrochlorothiazide.

PAST SURGICAL HISTORY:
- Right radical nephrectomy (robotic-assisted)
- Pancreaticoduodenectomy (Whipple procedure)
- Thoracic aortic aneurysm repair with graft placement
- Endovascular abdominal aortic aneurysm repair (EVAR)
- Open sigmoid colectomy with primary anastomosis

MEDICATIONS:
He takes hydrochlorothiazide.

Home Medications:
- Lisinopril 20 mg PO daily
- Furosemide 40 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily
- Folic acid 1 mg PO daily
- Oxycodone 5 mg PO q6h PRN pain

ALLERGIES:
Sulfa medications (hives), Iodinated contrast media (premedication required)

FAMILY HISTORY:
- Father: Abdominal aortic aneurysm (ruptured, age 72)
- Mother: Hypertension, breast cancer (age 68)
- Brother: Lung cancer
- Paternal uncle: Pancreatic cancer

SOCIAL HISTORY:
Smoking: Former smoker, 35 pack-years, quit 18 months ago. Alcohol: Occasional (1-2 drinks per week). Occupation: Retired long-haul truck driver.

PHYSICAL EXAMINATION:
On physical examination, the patient appears jaundiced with scleral icterus. His abdomen is soft, non-tender, without palpable masses, but there is a palpable, non-tender gallbladder.

LABORATORY DATA:
Laboratory values are as follows:

- Total bilirubin: 6.2 mg/dL (normal: 0.3-1.2 mg/dL)
- Direct bilirubin: 4.8 mg/dL (normal: 0.0-0.3 mg/dL)
- Alkaline phosphatase: 420 U/L (normal: 40-130 U/L)
- Aspartate aminotransferase (AST): 68 U/L (normal: 10-40 U/L)
- Alanine aminotransferase (ALT): 55 U/L (normal: 7-56 U/L)
- CA 19-9: 150 U/mL (normal: <37 U/mL)

IMAGING:
An abdominal ultrasound shows dilated intrahepatic bile ducts but no gallstones or biliary sludge. The common bile duct is dilated at 12 mm. The pancreas is not well visualized due to overlying bowel gas.

## Encounter 2: source 2021-09-15, practice 2021-10-28, procedure, Urology

Chief complaint: Scheduled for right radical nephrectomy for renal cell carcinoma

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. Emily Watson, MD
Department: Urology
Visit Type: procedure

CHIEF COMPLAINT:
Scheduled for right radical nephrectomy for renal cell carcinoma

HISTORY OF PRESENT ILLNESS:
This 67-year-old male with Stage IV colorectal adenocarcinoma diagnosed six months prior presents for scheduled right radical nephrectomy, having previously undergone evaluation for cervical spondylosis in January 2020 and pancreatic adenocarcinoma in May 2020. On the day of surgery, the patient is brought to the operating room and placed under general anesthesia. The surgical team performs a time-out procedure. The circulating nurse confirms the patient's identity using the wristband and the medical record, and the surgeon confirms the procedure and the operative site. The anesthesiologist reviews the patient's allergies and medication list. Just before the first incision, the scrub nurse initiates a final time-out.

PAST MEDICAL HISTORY:
Active Problem List:
- Cervical spondylosis (diagnosed 2020-01-15)
- Pancreatic adenocarcinoma (mucin-producing) with chronic disseminated intravascular coagulation (diagnosed 2020-05-15)
- Hypertension
- Congestive heart failure with reduced ejection fraction
- Diverticulosis
- Stage IV colorectal adenocarcinoma (diagnosed 6 months prior)
- History of pancreatic adenocarcinoma (s/p pancreaticoduodenectomy)
- History of right-sided renal cell carcinoma (s/p right radical nephrectomy)
- History of thoracic aortic aneurysm (s/p open repair)
- History of abdominal aortic aneurysm (s/p endovascular repair)
- Status post central venous catheter placement for chemotherapy
- History of tobacco use disorder

PAST SURGICAL HISTORY:
- Right radical nephrectomy (robotic-assisted)
- Pancreaticoduodenectomy (Whipple procedure)
- Thoracic aortic aneurysm repair with graft placement
- Endovascular abdominal aortic aneurysm repair (EVAR)
- Open sigmoid colectomy with primary anastomosis

MEDICATIONS:
He takes lisinopril for hypertension.

Home Medications:
- Furosemide 40 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily
- Folic acid 1 mg PO daily
- Oxycodone 5 mg PO q6h PRN pain

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Abdominal aortic aneurysm (ruptured, age 72)
- Mother: Hypertension, breast cancer (age 68)
- Brother: Lung cancer
- Paternal uncle: Pancreatic cancer

SOCIAL HISTORY:
Smoking: Former smoker, 35 pack-years, quit 18 months ago. Alcohol: Occasional (1-2 drinks per week). Occupation: Retired long-haul truck driver.

## Encounter 3: source 2022-09-15, practice 2022-11-07, outpatient, Urology

Chief complaint: 2-month history of recurrent urinary tract infections with passage of gas and occasional fecal material through urethra

ENCOUNTER NOTE
Date: 2022-09-15
Provider: Dr. James Park, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
2-month history of recurrent urinary tract infections with passage of gas and occasional fecal material through urethra

HISTORY OF PRESENT ILLNESS:
The patient with Stage IV colorectal adenocarcinoma presents with recurrent urinary tract infections. He reports experiencing passage of gas and occasional fecal material through the urethra. The patient denies any recent trauma or instrumentation.

PAST MEDICAL HISTORY:
Active Problem List:
- Cervical spondylosis (diagnosed 2020-01-15)
- Pancreatic adenocarcinoma (mucin-producing) with chronic disseminated intravascular coagulation (diagnosed 2020-05-15)
- Renal Cell Carcinoma (diagnosed 2021-09-15)
- Hypertension
- Congestive heart failure with reduced ejection fraction
- Diverticulosis
- Stage IV colorectal adenocarcinoma (diagnosed 6 months prior)
- History of pancreatic adenocarcinoma (s/p pancreaticoduodenectomy)
- History of right-sided renal cell carcinoma (s/p right radical nephrectomy)
- History of thoracic aortic aneurysm (s/p open repair)
- History of abdominal aortic aneurysm (s/p endovascular repair)
- Status post central venous catheter placement for chemotherapy
- History of tobacco use disorder

His past medical history is significant for hypertension and diverticulosis.

PAST SURGICAL HISTORY:
- Right radical nephrectomy (robotic-assisted)
- Pancreaticoduodenectomy (Whipple procedure)
- Thoracic aortic aneurysm repair with graft placement
- Endovascular abdominal aortic aneurysm repair (EVAR)
- Open sigmoid colectomy with primary anastomosis

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Furosemide 40 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily
- Folic acid 1 mg PO daily
- Oxycodone 5 mg PO q6h PRN pain

ALLERGIES:
Sulfa medications (hives), Iodinated contrast media (premedication required)

FAMILY HISTORY:
- Father: Abdominal aortic aneurysm (ruptured, age 72)
- Mother: Hypertension, breast cancer (age 68)
- Brother: Lung cancer
- Paternal uncle: Pancreatic cancer

SOCIAL HISTORY:
Smoking: Former smoker, 35 pack-years, quit 18 months ago. Alcohol: Occasional (1-2 drinks per week). Occupation: Retired long-haul truck driver.

PHYSICAL EXAMINATION:
On physical examination, there is mild suprapubic tenderness but no rebound or guarding.

LABORATORY DATA:
Urinalysis reveals pyuria and bacteriuria. No hematuria is noted.

## Encounter 4: source 2023-09-15, practice 2023-11-06, inpatient, Vascular Surgery

Chief complaint: Progressive lethargy and mild shortness of breath 48 hours after endovascular abdominal aortic aneurysm repair

ENCOUNTER NOTE
Date: 2023-09-15
Provider: Dr. Priya Patel, MD
Department: Vascular Surgery
Visit Type: inpatient

CHIEF COMPLAINT:
Progressive lethargy and mild shortness of breath 48 hours after endovascular abdominal aortic aneurysm repair

HISTORY OF PRESENT ILLNESS:
The patient is a 69-year-old male with a history of pancreatic adenocarcinoma status post pancreaticoduodenectomy, renal cell carcinoma status post right radical nephrectomy, and Stage IV colorectal adenocarcinoma who presents 48 hours after undergoing abdominal aortic aneurysm repair. During the procedure, he has a significant drop in blood pressure that requires vasopressor support for stabilization. Postoperatively, he is initially stable but develops progressive lethargy and mild shortness of breath over the next 48 hours. His blood pressure remains low-normal despite fluid administration, and his skin appears cool. Nursing staff note that his urine output has steadily decreased over the past day.

PAST MEDICAL HISTORY:
Active Problem List:
- Cervical spondylosis (diagnosed 2020-01-15)
- Pancreatic adenocarcinoma (mucin-producing) with chronic disseminated intravascular coagulation (diagnosed 2020-05-15)
- Renal Cell Carcinoma (diagnosed 2021-09-15)
- Colovesical fistula (diagnosed 2022-09-15)
- Hypertension
- Congestive heart failure with reduced ejection fraction
- Diverticulosis
- Stage IV colorectal adenocarcinoma (diagnosed 6 months prior)
- History of pancreatic adenocarcinoma (s/p pancreaticoduodenectomy)
- History of right-sided renal cell carcinoma (s/p right radical nephrectomy)
- History of thoracic aortic aneurysm (s/p open repair)
- History of abdominal aortic aneurysm (s/p endovascular repair)
- Status post central venous catheter placement for chemotherapy
- History of tobacco use disorder

PAST SURGICAL HISTORY:
- Right radical nephrectomy (robotic-assisted)
- Pancreaticoduodenectomy (Whipple procedure)
- Thoracic aortic aneurysm repair with graft placement
- Endovascular abdominal aortic aneurysm repair (EVAR)
- Open sigmoid colectomy with primary anastomosis

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Furosemide 40 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily
- Folic acid 1 mg PO daily
- Oxycodone 5 mg PO q6h PRN pain

ALLERGIES:
Sulfa medications (hives), Iodinated contrast media (premedication required)

FAMILY HISTORY:
- Father: Abdominal aortic aneurysm (ruptured, age 72)
- Mother: Hypertension, breast cancer (age 68)
- Brother: Lung cancer
- Paternal uncle: Pancreatic cancer

SOCIAL HISTORY:
Smoking: Former smoker, 35 pack-years, quit 18 months ago. Alcohol: Occasional (1-2 drinks per week). Occupation: Retired long-haul truck driver.

VITAL SIGNS:
His blood pressure remains low-normal despite fluid administration. Urine output is 15 mL/h.

PHYSICAL EXAMINATION:
His skin appears cool. Physical examination reveals diffuse crackles in both lung bases without peripheral edema.

LABORATORY DATA:
Serum creatinine concentration is 4.5 mg/dL (preoperative value 1 mg/dL).

## Encounter 5: source 2024-09-15, practice 2024-10-27, ed, Emergency Medicine

Chief complaint: Acute onset of lower extremity weakness, numbness, and urinary retention 12 hours after thoracic aortic aneurysm repair

ENCOUNTER NOTE
Date: 2024-09-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset of lower extremity weakness, numbness, and urinary retention 12 hours after thoracic aortic aneurysm repair

HISTORY OF PRESENT ILLNESS:
The patient is a 70-year-old male with extensive vascular history including prior endovascular abdominal aortic aneurysm repair and cervical spondylosis who presents to the emergency department 12 hours following thoracic aortic aneurysm repair. He reports that he was initially able to move his legs postoperatively, but over the past few hours, he has lost the ability to move them. He also complains of numbness in his legs and an inability to feel when his bladder is full.

PAST MEDICAL HISTORY:
Active Problem List:
- Cervical spondylosis (diagnosed 2020-01-15)
- Pancreatic adenocarcinoma (mucin-producing) with chronic disseminated intravascular coagulation (diagnosed 2020-05-15)
- Renal Cell Carcinoma (diagnosed 2021-09-15)
- Colovesical fistula (diagnosed 2022-09-15)
- Prerenal acute kidney injury secondary to hypovolemia (diagnosed 2023-09-15)
- Hypertension
- Congestive heart failure with reduced ejection fraction
- Diverticulosis
- Stage IV colorectal adenocarcinoma (diagnosed 6 months prior)
- History of pancreatic adenocarcinoma (s/p pancreaticoduodenectomy)
- History of right-sided renal cell carcinoma (s/p right radical nephrectomy)
- History of thoracic aortic aneurysm (s/p open repair)
- History of abdominal aortic aneurysm (s/p endovascular repair)
- Status post central venous catheter placement for chemotherapy
- History of tobacco use disorder

PAST SURGICAL HISTORY:
- Right radical nephrectomy (robotic-assisted)
- Pancreaticoduodenectomy (Whipple procedure)
- Thoracic aortic aneurysm repair with graft placement
- Endovascular abdominal aortic aneurysm repair (EVAR)
- Open sigmoid colectomy with primary anastomosis

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Furosemide 40 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily
- Folic acid 1 mg PO daily
- Oxycodone 5 mg PO q6h PRN pain

ALLERGIES:
Sulfa medications (hives), Iodinated contrast media (premedication required)

FAMILY HISTORY:
- Father: Abdominal aortic aneurysm (ruptured, age 72)
- Mother: Hypertension, breast cancer (age 68)
- Brother: Lung cancer
- Paternal uncle: Pancreatic cancer

SOCIAL HISTORY:
Smoking

VITAL SIGNS:
The patient's vital signs are stable

PHYSICAL EXAMINATION:
On physical examination, the patient has bilateral flaccid paralysis of the lower extremities, preserved proprioception and vibration sense, and loss of pain and temperature sensation below the level of the umbilicus. Rectal tone is decreased.
and there are no signs of external bleeding or hematoma.

LABORATORY DATA:
His postoperative hemoglobin level is stable compared to preoperative values.

## Encounter 6: source 2025-09-15, practice 2025-10-31, ed, Emergency Medicine

Chief complaint: Progressive shortness of breath, nonproductive cough, and generalized fatigue over 2 weeks

ENCOUNTER NOTE
Date: 2025-09-15
Provider: Dr. Lisa Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive shortness of breath, nonproductive cough, and generalized fatigue over 2 weeks

HISTORY OF PRESENT ILLNESS:
The patient, with Stage IV colorectal adenocarcinoma on chemotherapy via central venous catheter and heart failure with reduced ejection fraction, presents to the emergency department with progressive shortness of breath over the past 2 weeks. He also reports a nonproductive cough and generalized fatigue that has been worsening. He denies chest pain, fever, night sweats, weight loss, hemoptysis, or recent travel.

PAST MEDICAL HISTORY:
Active Problem List:
- Cervical spondylosis (diagnosed 2020-01-15)
- Pancreatic adenocarcinoma (mucin-producing) with chronic disseminated intravascular coagulation (diagnosed 2020-05-15)
- Renal Cell Carcinoma (diagnosed 2021-09-15)
- Colovesical fistula (diagnosed 2022-09-15)
- Prerenal acute kidney injury secondary to hypovolemia (diagnosed 2023-09-15)
- Spinal epidural hematoma (diagnosed 2024-09-15)
- Hypertension
- Congestive heart failure with reduced ejection fraction
- Diverticulosis
- Stage IV colorectal adenocarcinoma (diagnosed 6 months prior)
- History of pancreatic adenocarcinoma (s/p pancreaticoduodenectomy)
- History of right-sided renal cell carcinoma (s/p right radical nephrectomy)
- History of thoracic aortic aneurysm (s/p open repair)
- History of abdominal aortic aneurysm (s/p endovascular repair)
- Status post central venous catheter placement for chemotherapy
- History of tobacco use disorder

His past medical history includes hypertension and chronic congestive heart failure.

PAST SURGICAL HISTORY:
- Right radical nephrectomy (robotic-assisted)
- Pancreaticoduodenectomy (Whipple procedure)
- Thoracic aortic aneurysm repair with graft placement
- Endovascular abdominal aortic aneurysm repair (EVAR)
- Open sigmoid colectomy with primary anastomosis

MEDICATIONS:
Current medications include lisinopril and furosemide.

Home Medications:
- Metoprolol succinate 50 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily
- Folic acid 1 mg PO daily
- Oxycodone 5 mg PO q6h PRN pain

ALLERGIES:
Sulfa medications (hives), Iodinated contrast media (premedication required)

FAMILY HISTORY:
- Father: Abdominal aortic aneurysm (ruptured, age 72)
- Mother: Hypertension, breast cancer (age 68)
- Brother: Lung cancer
- Paternal uncle: Pancreatic cancer

SOCIAL HISTORY:
He does not smoke and drinks alcohol only occasionally.

VITAL SIGNS:
On examination, temperature is 37.1°C (98.8°F), blood pressure is 142/88 mm Hg, pulse is 96/min, respiratory rate is 20/min, and oxygen saturation is 90% on room air

PHYSICAL EXAMINATION:
Pulmonary exam reveals decreased tactile fremitus and dullness to percussion over the right lower lung field with markedly reduced breath sounds in that area, while the left lung has normal tactile fremitus, percussion note, and breath sounds. There is no jugular venous distention, peripheral edema, or tracheal deviation.

## Encounter 7: source 2026-03-15, practice 2026-05-06, inpatient, Oncology

Chief complaint: Persistent fever, productive cough with yellow sputum, and pleuritic chest pain during palliative chemotherapy

ENCOUNTER NOTE
Date: 2026-03-15
Provider: Dr. Robert Singh, MD
Department: Oncology
Visit Type: inpatient

CHIEF COMPLAINT:
Persistent fever, productive cough with yellow sputum, and pleuritic chest pain during palliative chemotherapy

HISTORY OF PRESENT ILLNESS:
The patient is a 72-year-old male with Stage IV colorectal adenocarcinoma, history of pancreatic adenocarcinoma status post pancreaticoduodenectomy, and prior parapneumonic effusion who returns with new respiratory symptoms. He is currently undergoing palliative chemotherapy via a central venous catheter placed in the right subclavian vein. Over the past 3 days, he has experienced increasing fatigue, productive cough with yellow sputum, and pleuritic chest pain. He denies recent travel, sick contacts, or hospitalizations in the past month. He is afebrile on admission but developed a temperature of 38.8°C (101.8°F) 12 hours later.

PAST MEDICAL HISTORY:
Active Problem List:
- Cervical spondylosis (diagnosed 2020-01-15)
- Pancreatic adenocarcinoma (mucin-producing) with chronic disseminated intravascular coagulation (diagnosed 2020-05-15)
- Renal Cell Carcinoma (diagnosed 2021-09-15)
- Colovesical fistula (diagnosed 2022-09-15)
- Prerenal acute kidney injury secondary to hypovolemia (diagnosed 2023-09-15)
- Spinal epidural hematoma (diagnosed 2024-09-15)
- Uncomplicated parapneumonic effusion (diagnosed 2025-09-15)
- Hypertension
- Congestive heart failure with reduced ejection fraction
- Diverticulosis
- Stage IV colorectal adenocarcinoma (diagnosed 6 months prior)
- History of pancreatic adenocarcinoma (s/p pancreaticoduodenectomy)
- History of right-sided renal cell carcinoma (s/p right radical nephrectomy)
- History of thoracic aortic aneurysm (s/p open repair)
- History of abdominal aortic aneurysm (s/p endovascular repair)
- Status post central venous catheter placement for chemotherapy
- History of tobacco use disorder

He was diagnosed with stage IV colorectal cancer six months ago.

PAST SURGICAL HISTORY:
- Right radical nephrectomy (robotic-assisted)
- Pancreaticoduodenectomy (Whipple procedure)
- Thoracic aortic aneurysm repair with graft placement
- Endovascular abdominal aortic aneurysm repair (EVAR)
- Open sigmoid colectomy with primary anastomosis

MEDICATIONS:
Home Medications:
- Lisinopril 20 mg PO daily
- Furosemide 40 mg PO daily
- Metoprolol succinate 50 mg PO daily
- Atorvastatin 40 mg PO daily
- Aspirin 81 mg PO daily
- Folic acid 1 mg PO daily
- Oxycodone 5 mg PO q6h PRN pain

ALLERGIES:
Sulfa medications (hives), Iodinated contrast media (premedication required)

FAMILY HISTORY:
- Father: Abdominal aortic aneurysm (ruptured, age 72)
- Mother: Hypertension, breast cancer (age 68)
- Brother: Lung cancer
- Paternal uncle: Pancreatic cancer

SOCIAL HISTORY:
Smoking: Former smoker, 35 pack-years, quit 18 months ago. Alcohol: Occasional (1-2 drinks per week). Occupation: Retired long-haul truck driver.

VITAL SIGNS:
Blood pressure is 102/66 mm Hg, heart rate 104/min, respiratory rate 22/min, and oxygen saturation is 91% on room air.

PHYSICAL EXAMINATION:
On exam, bronchial breath sounds and increased tactile fremitus are noted over the right upper lung field. There is no jugular venous distension or peripheral edema. His central catheter site appears clean with no discharge or erythema.

LABORATORY DATA:
Laboratory testing reveals leukocytosis with a left shift.

IMAGING:
Chest X-ray shows a patchy consolidation in the right upper lobe.

OTHER STUDIES:
A sputum culture is obtained.
