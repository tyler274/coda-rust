# synthetic_hospital patient 2225

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 70 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-02-07 (Tue) | outpatient | Urology |
| 1 | 2020-04-15 | 2023-05-10 (Wed) | outpatient | Internal Medicine |
| 2 | 2020-07-15 | 2023-08-01 (Tue) | outpatient | Gastroenterology |
| 3 | 2021-01-15 | 2024-02-16 (Fri) | outpatient | Cardiology |
| 4 | 2021-03-15 | 2024-04-07 (Sun) | ed | Emergency Medicine |
| 5 | 2022-01-15 | 2025-02-14 (Fri) | ed | Emergency Medicine |
| 6 | 2022-07-15 | 2025-08-01 (Fri) | ed | Emergency Medicine |
| 7 | 2023-01-15 | 2026-02-05 (Thu) | icu | Critical Care |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired high school mathematics teacher",
 "alcohol_use": "Occasional social use (1-2 drinks per week)",
 "family_history": [
  "Father: Myocardial infarction at age 68, hypertension",
  "Mother: Ischemic stroke at age 75, hyperlipidemia",
  "Maternal uncle: Abdominal aortic aneurysm"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "81 mg daily",
   "name": "Aspirin"
  },
  {
   "dose": "40 mg daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "20 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "5 mg daily",
   "name": "Amlodipine"
  },
  {
   "dose": "10 mg daily",
   "name": "Prednisone"
  },
  {
   "dose": "0.4 mg nightly",
   "name": "Tamsulosin"
  }
 ],
 "surgical_history": [
  "Appendectomy (age 45)",
  "Cardiac catheterization with coronary angiography (2 weeks prior to initial encounter)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Hyperlipidemia",
  "Coronary artery disease (stable angina)",
  "Giant cell arteritis",
  "Benign prostatic hyperplasia"
 ],
 "age_at_first_encounter": 70
}
```

Primary diagnoses: ["Cholesterol embolism", "Arteriovenous fistula", "Acute mesenteric ischemia", "Intestinal obstruction requiring cross-sectional imaging", "Long term (current) use of immunosuppressants", "Ischemic colitis", "Aortic dissection", "Recurrent benign prostatic hyperplasia"]
Comorbidities: ["Lactic acidosis", "Acute kidney injury", "Hyperlipidemia", "Essential hypertension", "Status post laparoscopic cholecystectomy"]

## Encounter 0: source 2020-01-15, practice 2023-02-07, outpatient, Urology

Chief complaint: Progressive urinary frequency, urgency, and weak stream over the past several months with nocturia

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive urinary frequency, urgency, and weak stream over the past several months with nocturia

HISTORY OF PRESENT ILLNESS:
He describes increased frequency and urgency, especially at night, requiring him to wake up multiple times to urinate. He also notes a weak urinary stream and a sensation of incomplete bladder emptying. The patient denies any dysuria, hematuria, fever, or flank pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Hyperlipidemia
- Coronary artery disease (stable angina)
- Giant cell arteritis
- Benign prostatic hyperplasia

He has a history of hypertension and hyperlipidemia, both well-controlled with medications.

PAST SURGICAL HISTORY:
- Appendectomy (age 45)
- Cardiac catheterization with coronary angiography (2 weeks prior to initial encounter)

MEDICATIONS:
Home Medications:
- Aspirin 81 mg daily
- Atorvastatin 40 mg daily
- Lisinopril 20 mg daily
- Amlodipine 5 mg daily
- Prednisone 10 mg daily
- Tamsulosin 0.4 mg nightly

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68, hypertension
- Mother: Ischemic stroke at age 75, hyperlipidemia
- Maternal uncle: Abdominal aortic aneurysm

SOCIAL HISTORY:
He does not smoke and drinks alcohol occasionally.

VITAL SIGNS:
On examination, his vital signs are within normal limits.

PHYSICAL EXAMINATION:
Digital rectal examination reveals a uniformly enlarged, smooth, and nontender prostate.

LABORATORY DATA:
Urinalysis is unremarkable, and serum creatinine is within the normal range.

OTHER STUDIES:
A postvoid residual volume is mildly elevated.

PLAN:
He is started on medical management and counseled about follow-up.

## Encounter 1: source 2020-04-15, practice 2023-05-10, outpatient, Internal Medicine

Chief complaint: Routine follow-up for giant cell arteritis monitoring

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for giant cell arteritis monitoring

HISTORY OF PRESENT ILLNESS:
The patient returns for giant cell arteritis monitoring. He feels well overall, has no complaints, and denies dizziness, fatigue, or leg swelling.

PAST MEDICAL HISTORY:
Active Problem List:
- Recurrent benign prostatic hyperplasia (diagnosed 2020-01-15)
- Hypertension
- Hyperlipidemia
- Coronary artery disease (stable angina)
- Giant cell arteritis
- Benign prostatic hyperplasia

PAST SURGICAL HISTORY:
- Appendectomy (age 45)
- Cardiac catheterization with coronary angiography (2 weeks prior to initial encounter)

MEDICATIONS:
He is started on lisinopril for.

Home Medications:
- Aspirin 81 mg daily
- Atorvastatin 40 mg daily
- Amlodipine 5 mg daily
- Prednisone 10 mg daily
- Tamsulosin 0.4 mg nightly

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68, hypertension
- Mother: Ischemic stroke at age 75, hyperlipidemia
- Maternal uncle: Abdominal aortic aneurysm

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per week). Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
His blood pressure has improved to 132/78 mm Hg

LABORATORY DATA:
Basic metabolic panel shows a potassium level of 5.0 mEq/L (up from 4.2 two weeks ago). Serum creatinine has mildly increased from 1.0 to 1.2 mg/dL.

ASSESSMENT AND PLAN:
The physician anticipates that further evaluation will show the drug’s effects on components of the renin-angiotensin-aldosterone system (RAAS).

## Encounter 2: source 2020-07-15, practice 2023-08-01, outpatient, Gastroenterology

Chief complaint: Six-month history of postprandial abdominal pain and 8kg unintentional weight loss

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Emily Johnson, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of postprandial abdominal pain and 8kg unintentional weight loss

HISTORY OF PRESENT ILLNESS:
This 70-year-old male with coronary artery disease and giant cell arteritis presents with new abdominal complaints following recent Urology and Internal Medicine encounters. He reports that the pain typically begins about 30 minutes after eating and lasts for several hours. Over the past several months, he has intentionally reduced his food intake to avoid these episodes and has lost 8 kg. The pain is crampy, diffuse, and not associated with bowel movements.

PAST MEDICAL HISTORY:
Active Problem List:
- Recurrent benign prostatic hyperplasia (diagnosed 2020-01-15)
- Long term (current) use of immunosuppressants (diagnosed 2020-04-15)
- Hypertension
- Hyperlipidemia
- Coronary artery disease (stable angina)
- Giant cell arteritis
- Benign prostatic hyperplasia

His medical history is notable for long-standing hypertension and hyperlipidemia.

PAST SURGICAL HISTORY:
- Appendectomy (age 45)
- Cardiac catheterization with coronary angiography (2 weeks prior to initial encounter)

MEDICATIONS:
He takes amlodipine and atorvastatin daily.

Home Medications:
- Aspirin 81 mg daily
- Lisinopril 20 mg daily
- Prednisone 10 mg daily
- Tamsulosin 0.4 mg nightly

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68, hypertension
- Mother: Ischemic stroke at age 75, hyperlipidemia
- Maternal uncle: Abdominal aortic aneurysm

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per week). Occupation: Retired high school mathematics teacher.

REVIEW OF SYSTEMS:
He denies fever, vomiting, or hematochezia.

PHYSICAL EXAMINATION:
Physical examination shows a thin elderly man in no acute distress; abdominal exam reveals mild diffuse tenderness without guarding or rebound. There is no abdominal mass, and bowel sounds are present.

LABORATORY DATA:
Laboratory studies, including complete blood count and liver enzymes, are within normal limits.

ASSESSMENT AND PLAN:
The most likely cause of this patient’s symptoms is stenosis of

## Encounter 3: source 2021-01-15, practice 2024-02-16, outpatient, Cardiology

Chief complaint: Two-week history of mild pain and swelling in left groin following recent cardiac catheterization

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. David Kim, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Two-week history of mild pain and swelling in left groin following recent cardiac catheterization

HISTORY OF PRESENT ILLNESS:
The patient with coronary artery disease on chronic immunosuppression presents. He reports that the symptoms began a few days after he underwent a cardiac catheterization for the evaluation of chest pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Recurrent benign prostatic hyperplasia (diagnosed 2020-01-15)
- Long term (current) use of immunosuppressants (diagnosed 2020-04-15)
- Ischemic colitis (diagnosed 2020-07-15)
- Hypertension
- Hyperlipidemia
- Coronary artery disease (stable angina)
- Giant cell arteritis
- Benign prostatic hyperplasia

His medical history is significant for hypertension, hyperlipidemia, and stable angina.

PAST SURGICAL HISTORY:
- Appendectomy (age 45)
- Cardiac catheterization with coronary angiography (2 weeks prior to initial encounter)

MEDICATIONS:
Home Medications:
- Aspirin 81 mg daily
- Atorvastatin 40 mg daily
- Lisinopril 20 mg daily
- Amlodipine 5 mg daily
- Prednisone 10 mg daily
- Tamsulosin 0.4 mg nightly

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68, hypertension
- Mother: Ischemic stroke at age 75, hyperlipidemia
- Maternal uncle: Abdominal aortic aneurysm

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per week). Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
His vital signs are within normal limits.

PHYSICAL EXAMINATION:
On physical examination, you note a small, tender mass in the left groin with a palpable thrill. Auscultation over the mass reveals a continuous bruit.
The remainder of the examination is unremarkable.

LABORATORY DATA:
Laboratory values:
- Hemoglobin: 14.2 g/dL
- White blood cell count: 7,500/mm^3
- Platelet count: 250,000/mm^3
- Serum creatinine: 1.0 mg/dL
- Blood urea nitrogen: 18 mg/dL

## Encounter 4: source 2021-03-15, practice 2024-04-07, ed, Emergency Medicine

Chief complaint: Two-week progressive malaise, abdominal pain, and painful purple rash on lower extremities with transient visual disturbances

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-week progressive malaise, abdominal pain, and painful purple rash on lower extremities with transient visual disturbances

HISTORY OF PRESENT ILLNESS:
This 71-year-old with giant cell arteritis on immunosuppression and recent left groin arteriovenous fistula presents with two-week malaise, abdominal pain, lower extremity purple rash, and transient visual disturbances.

PAST MEDICAL HISTORY:
Active Problem List:
- Recurrent benign prostatic hyperplasia (diagnosed 2020-01-15)
- Long term (current) use of immunosuppressants (diagnosed 2020-04-15)
- Ischemic colitis (diagnosed 2020-07-15)
- Arteriovenous fistula (diagnosed 2021-01-15)
- Hypertension
- Hyperlipidemia
- Coronary artery disease (stable angina)
- Giant cell arteritis
- Benign prostatic hyperplasia

PAST SURGICAL HISTORY:
- Appendectomy (age 45)
- Cardiac catheterization with coronary angiography (2 weeks prior to initial encounter)

MEDICATIONS:
His medications include aspirin, atorvastatin, and lisinopril.

Home Medications:
- Amlodipine 5 mg daily
- Prednisone 10 mg daily
- Tamsulosin 0.4 mg nightly

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68, hypertension
- Mother: Ischemic stroke at age 75, hyperlipidemia
- Maternal uncle: Abdominal aortic aneurysm

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per week). Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
His blood pressure is 155/90 mmHg

PHYSICAL EXAMINATION:
On physical examination, he has a diffuse, blanching erythematous rash with areas of livedo reticularis on his legs. A few of his toes appear cyanotic. Fundoscopic examination reveals bright, refractile plaques in the retinal arterioles.

LABORATORY DATA:
Laboratory studies show:

- Creatinine: 2.8 mg/dL (baseline 1.2 mg/dL)
- Blood urea nitrogen (BUN): 50 mg/dL
- C-reactive protein (CRP): 15 mg/L
- Eosinophils: 8% (absolute count 700/µL)
- Complement levels: Normal
- Urinalysis: No proteinuria, but microscopic hematuria is present

ASSESSMENT AND PLAN:
Based on the clinical presentation and recent medical history,

## Encounter 5: source 2022-01-15, practice 2025-02-14, ed, Emergency Medicine

Chief complaint: Twelve-hour history of severe crampy abdominal pain, vomiting, and obstipation

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. James Smith, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Twelve-hour history of severe crampy abdominal pain, vomiting, and obstipation

HISTORY OF PRESENT ILLNESS:
This 72-year-old male with hypertension, hyperlipidemia, coronary artery disease, giant cell arteritis on chronic prednisone therapy, benign prostatic hyperplasia, and a remote appendectomy presents to the emergency department with a 12-hour history of severe, crampy abdominal pain. He has prior gastrointestinal presentations including ischemic colitis and cholesterol embolism. He reports multiple episodes of vomiting and has not passed flatus or had a bowel movement since the pain started. The patient's condition deteriorates over the next hour, with increasing tachycardia and hypotension despite fluid resuscitation.

PAST MEDICAL HISTORY:
Active Problem List:
- Recurrent benign prostatic hyperplasia (diagnosed 2020-01-15)
- Long term (current) use of immunosuppressants (diagnosed 2020-04-15)
- Ischemic colitis (diagnosed 2020-07-15)
- Arteriovenous fistula (diagnosed 2021-01-15)
- Cholesterol embolism (diagnosed 2021-03-15)
- Hypertension
- Hyperlipidemia
- Coronary artery disease (stable angina)
- Giant cell arteritis
- Benign prostatic hyperplasia

PAST SURGICAL HISTORY:
Remote history of appendectomy

MEDICATIONS:
Home Medications:
- Aspirin 81 mg daily
- Atorvastatin 40 mg daily
- Lisinopril 20 mg daily
- Amlodipine 5 mg daily
- Prednisone 10 mg daily
- Tamsulosin 0.4 mg nightly

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68, hypertension
- Mother: Ischemic stroke at age 75, hyperlipidemia
- Maternal uncle: Abdominal aortic aneurysm

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per week). Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
He is tachycardic with a heart rate of 110 bpm, blood pressure is 100/60 mmHg, and he is febrile with a temperature of 38.3°C (101°F).

PHYSICAL EXAMINATION:
His abdomen is distended and tympanic on percussion. Bowel sounds are high-pitched and hyperactive in all quadrants.

LABORATORY DATA:
White blood cell count: 18,000 /µL
Hemoglobin: 13.2 g/dL
Platelet count: 250,000 /µL
Serum lactate: 4.5 mmol/L
Serum creatinine: 1.2 mg/dL
Serum electrolytes are within normal limits

IMAGING:
An abdominal X-ray shows dilated loops of small bowel with air-fluid levels. There is no free air under the diaphragm.

## Encounter 6: source 2022-07-15, practice 2025-08-01, ed, Emergency Medicine

Chief complaint: Sudden onset severe periumbilical abdominal pain for 12 hours without relief

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Maria Garcia, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset severe periumbilical abdominal pain for 12 hours without relief

HISTORY OF PRESENT ILLNESS:
The patient with prior ischemic colitis and cholesterol embolism presents with pain that started 12 hours ago. The pain is constant, located in the periumbilical region, and is not relieved by any position.

PAST MEDICAL HISTORY:
Active Problem List:
- Recurrent benign prostatic hyperplasia (diagnosed 2020-01-15)
- Long term (current) use of immunosuppressants (diagnosed 2020-04-15)
- Ischemic colitis (diagnosed 2020-07-15)
- Arteriovenous fistula (diagnosed 2021-01-15)
- Cholesterol embolism (diagnosed 2021-03-15)
- Intestinal obstruction requiring cross-sectional imaging (diagnosed 2022-01-15)
- Hypertension
- Hyperlipidemia
- Coronary artery disease (stable angina)
- Giant cell arteritis
- Benign prostatic hyperplasia

PAST SURGICAL HISTORY:
Recent endovascular repair of an abdominal aortic aneurysm (AAA) one week ago

MEDICATIONS:
He takes aspirin, atorvastatin, and lisinopril.

Home Medications:
- Amlodipine 5 mg daily
- Prednisone 10 mg daily
- Tamsulosin 0.4 mg nightly

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68, hypertension
- Mother: Ischemic stroke at age 75, hyperlipidemia
- Maternal uncle: Abdominal aortic aneurysm

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per week). Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
Blood pressure 150/90 mmHg, heart rate 105 bpm, respiratory rate 22 breaths/min, and temperature 37.8°C (100°F)

PHYSICAL EXAMINATION:
Abdomen is distended, with diffuse tenderness and guarding. Bowel sounds are hypoactive. Rectal examination reveals no blood.

LABORATORY DATA:
White blood cell count: 16,000/mm³
Hemoglobin: 12 g/dL
Lactate: 5 mmol/L
Creatinine: 1.8 mg/dL (baseline 1.2 mg/dL)
Arterial blood gas: pH 7.32, PaCO2 32 mmHg, HCO3- 18 mmol/L

IMAGING:
An abdominal CT scan with contrast shows no evidence of AAA leakage or rupture, but there is bowel wall thickening and pneumatosis intestinalis.

## Encounter 7: source 2023-01-15, practice 2026-02-05, icu, Critical Care

Chief complaint: Sudden severe tearing chest pain radiating to back with shortness of breath

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Robert Brown, MD
Department: Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Sudden severe tearing chest pain radiating to back with shortness of breath

HISTORY OF PRESENT ILLNESS:
The patient is a 73-year-old male with coronary artery disease, giant cell arteritis on chronic immunosuppression, and prior ischemic events including mesenteric ischemia and cholesterol embolism, who presents with acute chest pain. The pain is described as tearing and has been persistent for the past two hours.

PAST MEDICAL HISTORY:
Active Problem List:
- Recurrent benign prostatic hyperplasia (diagnosed 2020-01-15)
- Long term (current) use of immunosuppressants (diagnosed 2020-04-15)
- Ischemic colitis (diagnosed 2020-07-15)
- Arteriovenous fistula (diagnosed 2021-01-15)
- Cholesterol embolism (diagnosed 2021-03-15)
- Intestinal obstruction requiring cross-sectional imaging (diagnosed 2022-01-15)
- Acute mesenteric ischemia (diagnosed 2022-07-15)
- Hypertension
- Hyperlipidemia
- Coronary artery disease (stable angina)
- Giant cell arteritis
- Benign prostatic hyperplasia

PAST SURGICAL HISTORY:
- Appendectomy (age 45)
- Cardiac catheterization with coronary angiography (2 weeks prior to initial encounter)

MEDICATIONS:
Home Medications:
- Aspirin 81 mg daily
- Atorvastatin 40 mg daily
- Lisinopril 20 mg daily
- Amlodipine 5 mg daily
- Prednisone 10 mg daily
- Tamsulosin 0.4 mg nightly

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Myocardial infarction at age 68, hypertension
- Mother: Ischemic stroke at age 75, hyperlipidemia
- Maternal uncle: Abdominal aortic aneurysm

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per week). Occupation: Retired high school mathematics teacher.

VITAL SIGNS:
His blood pressure is 180/100 mmHg in the right arm and 160/90 mmHg in the left arm.

PHYSICAL EXAMINATION:
Physical examination reveals a diastolic murmur best heard at the right second intercostal space and a weak pulse in the femoral arteries. The patient appears diaphoretic and anxious.

LABORATORY DATA:
Laboratory values are as follows:

- Hemoglobin: 14 g/dL
- White blood cell count: 12,000/mm^3
- Platelets: 250,000/mm^3
- Serum creatinine: 1.2 mg/dL
- Troponin I: <0.04 ng/mL (normal)

IMAGING:
Chest X-ray demonstrates a widened mediastinum.

OTHER STUDIES:
An electrocardiogram shows no ST changes or Q waves.
