# synthetic_hospital patient 2826

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 68 at first encounter, sex F, Non-Hispanic White, Medicare Part B, Aetna Supplemental Plan F.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-07-20 (Tue) | outpatient | Endocrinology |
| 1 | 2020-04-15 | 2021-10-25 (Mon) | outpatient | Internal Medicine |
| 2 | 2021-04-15 | 2022-10-13 (Thu) | outpatient | Internal Medicine |
| 3 | 2022-03-15 | 2023-09-26 (Tue) | ed | Emergency Medicine |
| 4 | 2022-05-15 | 2023-11-10 (Fri) | outpatient | Endocrinology |
| 5 | 2023-01-15 | 2024-07-13 (Sat) | ed | Emergency Medicine |
| 6 | 2023-03-15 | 2024-09-11 (Wed) | outpatient | Cardiology |
| 7 | 2024-07-15 | 2026-01-10 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare Part B, Aetna Supplemental Plan F",
 "occupation": "Remote Administrative Consultant (Transitioning to Retirement)",
 "alcohol_use": "Social alcohol use (wine, 1-2 glasses weekly) with intermittent periods of abstinence",
 "family_history": [
  "Mother: Hip fracture secondary to osteoporosis (age 82), hypertension",
  "Father: Atrial fibrillation, essential hypertension",
  "Sister: Graves disease (hyperthyroidism)",
  "Maternal grandmother: Rheumatoid arthritis"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "25 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "10 mg PO daily (alternative to thiazide per cardiology)",
   "name": "Lisinopril"
  },
  {
   "dose": "600 mg PO three times daily with meals",
   "name": "Calcium carbonate"
  },
  {
   "dose": "2000 international units PO daily",
   "name": "Cholecalciferol (Vitamin D3)"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Methimazole"
  },
  {
   "dose": "5 mg PO daily (INR monitored)",
   "name": "Warfarin sodium"
  }
 ],
 "surgical_history": [
  "Open reduction and internal fixation of right proximal humerus fracture",
  "Cataract extraction with intraocular lens placement (left eye)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Osteoporosis",
  "Osteoarthritis involving bilateral knees",
  "Graves disease",
  "Paroxysmal atrial fibrillation"
 ],
 "age_at_first_encounter": 68
}
```

Primary diagnoses: ["Graves disease", "Anterior glenohumeral dislocation with surgical neck fracture of humerus", "Osteoporosis", "Milk-alkali syndrome", "Hypofibrinogenemia", "Long-term anticoagulant therapy"]
Comorbidities: ["Atrial fibrillation", "Postmenopausal status", "Osteoporosis", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2021-07-20, outpatient, Endocrinology

Chief complaint: Palpitations, heat intolerance, and unintentional weight loss

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Palpitations, heat intolerance, and unintentional weight loss

HISTORY OF PRESENT ILLNESS:
6-month history of palpitations, heat intolerance, and unintentional weight loss despite an increased appetite. She also reports frequent bowel movements but denies diarrhea.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Osteoporosis
- Osteoarthritis involving bilateral knees
- Graves disease
- Paroxysmal atrial fibrillation

She has a past medical history of hypertension and osteoporosis.

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of right proximal humerus fracture
- Cataract extraction with intraocular lens placement (left eye)

MEDICATIONS:
Her medications include hydrochlorothiazide and calcium supplements.

Home Medications:
- Lisinopril 10 mg PO daily (alternative to thiazide per cardiology)
- Calcium carbonate 600 mg PO three times daily with meals
- Cholecalciferol (Vitamin D3) 2000 international units PO daily
- Methimazole 10 mg PO daily
- Warfarin sodium 5 mg PO daily (INR monitored)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Hip fracture secondary to osteoporosis (age 82), hypertension
- Father: Atrial fibrillation, essential hypertension
- Sister: Graves disease (hyperthyroidism)
- Maternal grandmother: Rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social alcohol use (wine, 1-2 glasses weekly) with intermittent periods of abstinence. Occupation: Remote Administrative Consultant (Transitioning to Retirement).

VITAL SIGNS:
Heart rate is 102 bpm, blood pressure is 138/86 mmHg

PHYSICAL EXAMINATION:
On physical examination, she has a fine tremor in her outstretched hands. Her skin is warm and moist, and she has a diffusely enlarged, non-tender thyroid gland.

LABORATORY DATA:
Laboratory tests reveal the following:

- Serum TSH: <0.01 mIU/L (normal range: 0.4-4.0 mIU/L)
- Free T4: 3.2 ng/dL (normal range: 0.8-1.8 ng/dL)
- Serum calcium: 10.2 mg/dL (normal range: 8.5-10.2 mg/dL)
- Alkaline phosphatase: 150 U/L (normal range: 30-120 U/L)

OTHER STUDIES:
An EKG shows atrial fibrillation with a rapid ventricular response.

## Encounter 1: source 2020-04-15, practice 2021-10-25, outpatient, Internal Medicine

Chief complaint: Routine follow-up to discuss bone health protection strategies

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Maria Rodriguez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up to discuss bone health protection strategies

HISTORY OF PRESENT ILLNESS:
The patient returns for routine follow-up to discuss bone health protection strategies, presenting three months following her Endocrinology evaluation for Graves disease. She reports no acute symptoms but notes she has become less active over the past several years since retiring. She denies pain at rest but describes feeling stiff when she gets up in the morning and states she walks a little slower. She denies any history of falls.

PAST MEDICAL HISTORY:
Active Problem List:
- Graves disease (diagnosed 2020-01-15)
- Hypertension
- Osteoporosis
- Osteoarthritis involving bilateral knees
- Paroxysmal atrial fibrillation

Her past medical history includes hypertension and mild osteoarthritis of the knees. She has never broken a bone.

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of right proximal humerus fracture
- Cataract extraction with intraocular lens placement (left eye)

MEDICATIONS:
Her current medications are lisinopril and a daily calcium/vitamin D supplement.

Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Calcium carbonate 600 mg PO three times daily with meals
- Cholecalciferol (Vitamin D3) 2000 international units PO daily
- Methimazole 10 mg PO daily
- Warfarin sodium 5 mg PO daily (INR monitored)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Hip fracture secondary to osteoporosis (age 82), hypertension
- Father: Atrial fibrillation, essential hypertension
- Sister: Graves disease (hyperthyroidism)
- Maternal grandmother: Rheumatoid arthritis

SOCIAL HISTORY:
She has never smoked. She drinks wine socially and eats a relatively balanced diet.

REVIEW OF SYSTEMS:
She denies weakness, numbness, or recent weight changes.

VITAL SIGNS:
Her vital signs are normal

PHYSICAL EXAMINATION:
Physical exam is unremarkable. Gait is steady, and strength is preserved.

IMAGING:
Her most recent bone mineral density scan showed a T-score of –2.7 in the femoral neck and –2.8 in the lumbar spine.

ASSESSMENT AND PLAN:
She was diagnosed with osteoporosis but has not yet started pharmacologic therapy due to concerns about side effects and has asked about non-drug strategies first.

## Encounter 2: source 2021-04-15, practice 2022-10-13, outpatient, Internal Medicine

Chief complaint: Progressive fatigue and mild mid-back discomfort over the past year

ENCOUNTER NOTE
Date: 2021-04-15
Provider: Dr. James Patterson, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue and mild mid-back discomfort over the past year

HISTORY OF PRESENT ILLNESS:
The patient is a 69-year-old female with a history of osteoporosis who returns for follow-up approximately one year after her prior bone health evaluation. She reports progressive fatigue and mild mid-back discomfort over the past year but attributes it to aging and poor posture while working from home. She denies focal weakness, paresthesia, or significant joint pain. She remains active with daily walks but has noticed she feels shorter lately and that her pants are fitting differently at the waist.

PAST MEDICAL HISTORY:
Active Problem List:
- Graves disease (diagnosed 2020-01-15)
- Osteoporosis (diagnosed 2020-04-15)
- Hypertension
- Osteoarthritis involving bilateral knees
- Paroxysmal atrial fibrillation

She had early menopause at age 45 and has never taken hormone replacement therapy. Her medical history includes hypertension controlled with hydrochlorothiazide and a remote history of low calcium intake due to lactose intolerance, for which she avoids dairy.

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of right proximal humerus fracture
- Cataract extraction with intraocular lens placement (left eye)

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Lisinopril 10 mg PO daily (alternative to thiazide per cardiology)
- Calcium carbonate 600 mg PO three times daily with meals
- Cholecalciferol (Vitamin D3) 2000 international units PO daily
- Methimazole 10 mg PO daily
- Warfarin sodium 5 mg PO daily (INR monitored)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Hip fracture secondary to osteoporosis (age 82), hypertension
- Father: Atrial fibrillation, essential hypertension
- Sister: Graves disease (hyperthyroidism)
- Maternal grandmother: Rheumatoid arthritis

SOCIAL HISTORY:
She does not smoke and drinks wine socially.

VITAL SIGNS:
On physical examination, blood pressure is 132/78 mmHg, pulse is 76/min, and BMI is 21 kg/m².

PHYSICAL EXAMINATION:
Neurologic examination is normal. Musculoskeletal examination shows mild thoracic kyphosis and decreased height compared to records from five years ago.

## Encounter 3: source 2022-03-15, practice 2023-09-26, ed, Emergency Medicine

Chief complaint: Severe right shoulder pain and inability to move arm after fall

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. Emily Watson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe right shoulder pain and inability to move arm after fall

HISTORY OF PRESENT ILLNESS:
This 70-year-old female with monitored osteoporosis presents to the emergency department after falling onto her outstretched right hand during a morning walk.

PAST MEDICAL HISTORY:
Active Problem List:
- Graves disease (diagnosed 2020-01-15)
- Osteoporosis (diagnosed 2020-04-15)
- Hypertension
- Osteoarthritis involving bilateral knees
- Paroxysmal atrial fibrillation

Her medical history is significant for osteoporosis and hypertension,.

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of right proximal humerus fracture
- Cataract extraction with intraocular lens placement (left eye)

MEDICATIONS:
She takes managed with hydrochlorothiazide and amlodipine.

Home Medications:
- Lisinopril 10 mg PO daily (alternative to thiazide per cardiology)
- Calcium carbonate 600 mg PO three times daily with meals
- Cholecalciferol (Vitamin D3) 2000 international units PO daily
- Methimazole 10 mg PO daily
- Warfarin sodium 5 mg PO daily (INR monitored)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Hip fracture secondary to osteoporosis (age 82), hypertension
- Father: Atrial fibrillation, essential hypertension
- Sister: Graves disease (hyperthyroidism)
- Maternal grandmother: Rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social alcohol use (wine, 1-2 glasses weekly) with intermittent periods of abstinence. Occupation: Remote Administrative Consultant (Transitioning to Retirement).

PHYSICAL EXAMINATION:
On physical examination, her right arm appears shortened and adducted with a prominent acromion. There is decreased sensation over the lateral aspect of her right shoulder.

IMAGING:
Radiographs of the right shoulder show an anterior glenohumeral dislocation with a concomitant fracture of the surgical neck of the humerus.

## Encounter 4: source 2022-05-15, practice 2023-11-10, outpatient, Endocrinology

Chief complaint: Routine follow-up with questions about osteoporosis diagnosis and dietary changes

ENCOUNTER NOTE
Date: 2022-05-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up with questions about osteoporosis diagnosis and dietary changes

HISTORY OF PRESENT ILLNESS:
The patient returns for routine follow-up of osteoporosis, initially diagnosed in 2020 and recently complicated by a fragility fracture of the right shoulder in March 2022. She is concerned about what this means and whether she needs any dietary changes.

PAST MEDICAL HISTORY:
Active Problem List:
- Graves disease (diagnosed 2020-01-15)
- Osteoporosis (diagnosed 2020-04-15)
- Anterior glenohumeral dislocation with surgical neck fracture of humerus (diagnosed 2022-03-15)
- Hypertension
- Osteoarthritis involving bilateral knees
- Paroxysmal atrial fibrillation

Her last colonoscopy and mammogram were normal. She reached menopause at age 50 and never received hormone replacement therapy.

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of right proximal humerus fracture
- Cataract extraction with intraocular lens placement (left eye)

MEDICATIONS:
Her only medication is a thiazide diuretic for hypertension.

Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Lisinopril 10 mg PO daily (alternative to thiazide per cardiology)
- Calcium carbonate 600 mg PO three times daily with meals
- Cholecalciferol (Vitamin D3) 2000 international units PO daily
- Methimazole 10 mg PO daily
- Warfarin sodium 5 mg PO daily (INR monitored)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
There is no family history of kidney stones or endocrine disorders.

SOCIAL HISTORY:
She lives independently, walks daily for exercise, and eats a diet that includes dairy and green vegetables.
She has never smoked or consumed alcohol.

REVIEW OF SYSTEMS:
She denies back pain, joint pain, or difficulty walking.
She has no signs or symptoms suggestive of malignancy, malabsorption, or chronic illness.

VITAL SIGNS:
Vital signs are within normal limits,

PHYSICAL EXAMINATION:
Physical examination shows a thin, well-appearing woman with no spinal tenderness or deformity. Her gait is steady, and neurological exam is unremarkable.

IMAGING:
She recently underwent a dual-energy x-ray absorptiometry (DEXA) scan as part of routine screening. The T-scores for her lumbar spine and left femoral neck were –2.7 and –2.9, respectively.

ASSESSMENT AND PLAN:
The physician reviews her results and explains that no additional lab abnormalities are expected in her case.

The next step is to discuss management options and address her concerns about complications like fracture risk.

## Encounter 5: source 2023-01-15, practice 2024-07-13, ed, Emergency Medicine

Chief complaint: Severe burning skin pain and deep discoloration over lower abdomen and thighs three days after hospital discharge

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe burning skin pain and deep discoloration over lower abdomen and thighs three days after hospital discharge

HISTORY OF PRESENT ILLNESS:
This 71-year-old female with Graves disease, osteoporosis with prior shoulder fracture, and recent new-onset atrial fibrillation returns to the emergency department three days after hospital discharge. She was recently discharged three days ago after a brief hospitalization for new-onset atrial fibrillation and started on oral warfarin for stroke prevention. She now reports burning pain in the affected areas, which began yesterday and has rapidly progressed. She denies fever, trauma, or recent procedures.

PAST MEDICAL HISTORY:
Active Problem List:
- Graves disease (diagnosed 2020-01-15)
- Osteoporosis (diagnosed 2020-04-15)
- Anterior glenohumeral dislocation with surgical neck fracture of humerus (diagnosed 2022-03-15)
- Hypertension
- Osteoarthritis involving bilateral knees
- Paroxysmal atrial fibrillation

Her medical history includes hypertension and osteoarthritis.

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of right proximal humerus fracture
- Cataract extraction with intraocular lens placement (left eye)

MEDICATIONS:
She takes oral warfarin.

Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Lisinopril 10 mg PO daily (alternative to thiazide per cardiology)
- Calcium carbonate 600 mg PO three times daily with meals
- Cholecalciferol (Vitamin D3) 2000 international units PO daily
- Methimazole 10 mg PO daily
- Warfarin sodium 5 mg PO daily (INR monitored)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
She denies any personal or family history of clotting disorders.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social alcohol use (wine, 1-2 glasses weekly) with intermittent periods of abstinence. Occupation: Remote Administrative Consultant (Transitioning to Retirement).

VITAL SIGNS:
Vital signs are stable.

PHYSICAL EXAMINATION:
On physical exam, she has several sharply demarcated, violaceous plaques with overlying bullae and areas of necrosis on the lower abdomen and proximal thighs. No signs of infection or bleeding are present. Peripheral pulses are intact and symmetric.

LABORATORY DATA:
Laboratory results show a normal platelet count and stable hemoglobin; INR is elevated at 3.2.

## Encounter 6: source 2023-03-15, practice 2024-09-11, outpatient, Cardiology

Chief complaint: Routine follow-up for anticoagulation management

ENCOUNTER NOTE
Date: 2023-03-15
Provider: Dr. Michael Chen, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for anticoagulation management

HISTORY OF PRESENT ILLNESS:
This 71-year-old female with paroxysmal atrial fibrillation and recent hypofibrinogenemia returns for anticoagulation management. She was recently started on warfarin therapy for stroke prevention. She is compliant with her medication and denies any recent illness, changes in diet, or use of over-the-counter drugs.

PAST MEDICAL HISTORY:
Active Problem List:
- Graves disease (diagnosed 2020-01-15)
- Osteoporosis (diagnosed 2020-04-15)
- Anterior glenohumeral dislocation with surgical neck fracture of humerus (diagnosed 2022-03-15)
- Hypofibrinogenemia (diagnosed 2023-01-15)
- Hypertension
- Osteoarthritis involving bilateral knees
- Paroxysmal atrial fibrillation

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of right proximal humerus fracture
- Cataract extraction with intraocular lens placement (left eye)

MEDICATIONS:
She takes warfarin therapy.

Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Lisinopril 10 mg PO daily (alternative to thiazide per cardiology)
- Calcium carbonate 600 mg PO three times daily with meals
- Cholecalciferol (Vitamin D3) 2000 international units PO daily
- Methimazole 10 mg PO daily
- Warfarin sodium 5 mg PO daily (INR monitored)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Hip fracture secondary to osteoporosis (age 82), hypertension
- Father: Atrial fibrillation, essential hypertension
- Sister: Graves disease (hyperthyroidism)
- Maternal grandmother: Rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social alcohol use (wine, 1-2 glasses weekly) with intermittent periods of abstinence. Occupation: Remote Administrative Consultant (Transitioning to Retirement).

PHYSICAL EXAMINATION:
She is clinically stable without signs of bleeding or clotting.

LABORATORY DATA:
Her baseline laboratory results, including liver and renal function, were within normal limits. On day 2 of therapy, her INR is 1.1. On day 4, it increases to 2.2.

ASSESSMENT AND PLAN:
Her physician explains that the INR took a few days to reach the target range.

## Encounter 7: source 2024-07-15, practice 2026-01-10, ed, Emergency Medicine

Chief complaint: Confusion, constipation, and generalized weakness

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Lisa Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Confusion, constipation, and generalized weakness

HISTORY OF PRESENT ILLNESS:
This 72-year-old female with osteoporosis on calcium carbonate presents to the emergency department with a two-week history of confusion, constipation, and generalized weakness.

PAST MEDICAL HISTORY:
Active Problem List:
- Graves disease (diagnosed 2020-01-15)
- Osteoporosis (diagnosed 2020-04-15)
- Anterior glenohumeral dislocation with surgical neck fracture of humerus (diagnosed 2022-03-15)
- Hypofibrinogenemia (diagnosed 2023-01-15)
- Long-term anticoagulant therapy (diagnosed 2023-03-15)
- Hypertension
- Osteoarthritis involving bilateral knees
- Paroxysmal atrial fibrillation

She has a past medical history of osteoporosis and hypertension.

PAST SURGICAL HISTORY:
- Open reduction and internal fixation of right proximal humerus fracture
- Cataract extraction with intraocular lens placement (left eye)

MEDICATIONS:
She reports that she has been taking over-the-counter calcium carbonate tablets and drinking milk to strengthen her bones, as well as a prescribed thiazide diuretic for her blood pressure.

Home Medications:
- Hydrochlorothiazide 25 mg PO daily
- Lisinopril 10 mg PO daily (alternative to thiazide per cardiology)
- Cholecalciferol (Vitamin D3) 2000 international units PO daily
- Methimazole 10 mg PO daily
- Warfarin sodium 5 mg PO daily (INR monitored)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Hip fracture secondary to osteoporosis (age 82), hypertension
- Father: Atrial fibrillation, essential hypertension
- Sister: Graves disease (hyperthyroidism)
- Maternal grandmother: Rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social alcohol use (wine, 1-2 glasses weekly) with intermittent periods of abstinence. Occupation: Remote Administrative Consultant (Transitioning to Retirement).

VITAL SIGNS:
Blood pressure is 145/90 mmHg, heart rate is 88 beats per minute

PHYSICAL EXAMINATION:
She appears lethargic. Her abdomen is non-tender, without distension.

LABORATORY DATA:
Laboratory results are as follows:

- Serum calcium: 12.5 mg/dL (normal: 8.5-10.2 mg/dL)
- Serum creatinine: 2.1 mg/dL (normal: 0.6-1.1 mg/dL)
- Blood urea nitrogen (BUN): 35 mg/dL (normal: 7-20 mg/dL)
- Arterial blood gas (ABG):
- pH: 7.52 (normal: 7.35-7.45)
- HCO3-: 32 mEq/L (normal: 22-26 mEq/L)
- PaCO2: 42 mmHg (normal: 35-45 mmHg)
- Urinalysis: No abnormalities detected
