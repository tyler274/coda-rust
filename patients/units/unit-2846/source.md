# synthetic_hospital patient 2846

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 70 at first encounter, sex F, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-10-31 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-04-15 | 2023-01-31 (Tue) | outpatient | Rheumatology |
| 2 | 2020-10-15 | 2023-08-01 (Tue) | outpatient | Gastroenterology |
| 3 | 2021-04-15 | 2024-01-18 (Thu) | outpatient | Ophthalmology |
| 4 | 2021-10-15 | 2024-08-06 (Tue) | outpatient | Rheumatology |
| 5 | 2022-04-15 | 2025-01-27 (Mon) | ed | Emergency Medicine |
| 6 | 2022-07-15 | 2025-05-02 (Fri) | icu | Cardiothoracic Surgery |
| 7 | 2023-01-15 | 2025-10-21 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired elementary school teacher",
 "alcohol_use": "None",
 "family_history": [
  "Mother with giant cell arteritis",
  "Father with hypertension and aortic aneurysm",
  "Sister with primary open-angle glaucoma"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "10 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "12.5 mg daily",
   "name": "Hydrochlorothiazide"
  }
 ],
 "surgical_history": [
  "Ascending aortic graft replacement (Type A dissection repair)",
  "Laser peripheral iridotomy, right eye"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Polymyalgia rheumatica",
  "Giant cell arteritis",
  "Hiatal hernia",
  "History of pulmonary tuberculosis",
  "Chronic angle-closure glaucoma"
 ],
 "age_at_first_encounter": 70
}
```

Primary diagnoses: ["Type A Aortic Dissection", "Giant cell arteritis or autoimmune inflammatory condition", "Polymyalgia rheumatica", "Hiatal hernia", "Osteoporosis screening in malabsorptive disorder", "Ethambutol-induced optic neuropathy", "Acute angle-closure glaucoma"]
Comorbidities: ["Polymyalgia rheumatica", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2022-10-31, outpatient, Internal Medicine

Chief complaint: Routine health maintenance visit with concern about osteoporosis risk

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine health maintenance visit with concern about osteoporosis risk

HISTORY OF PRESENT ILLNESS:
The patient is concerned about her risk of osteoporosis because a close friend was recently diagnosed with it.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Polymyalgia rheumatica
- Giant cell arteritis
- Hiatal hernia
- History of pulmonary tuberculosis
- Chronic angle-closure glaucoma

She has a history of hypertension, which is well-controlled with hydrochlorothiazide. She has not had any fractures or falls in the past.

PAST SURGICAL HISTORY:
- Ascending aortic graft replacement (Type A dissection repair)
- Laser peripheral iridotomy, right eye

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Lisinopril 10 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
Her mother had a hip fracture at the age of 74.

SOCIAL HISTORY:
She does not smoke and consumes alcohol occasionally.
She exercises regularly and has a balanced diet.

PHYSICAL EXAMINATION:
On physical examination, her BMI is 22 kg/m^2, and there are no abnormalities noted.

LABORATORY DATA:
Her most recent blood work from six months ago showed normal calcium, vitamin D, and thyroid-stimulating hormone levels.

## Encounter 1: source 2020-04-15, practice 2023-01-31, outpatient, Rheumatology

Chief complaint: Pain and stiffness in shoulders and hips with morning stiffness

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Marcus Johnson, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Pain and stiffness in shoulders and hips with morning stiffness

HISTORY OF PRESENT ILLNESS:
Patient is a 70-year-old female with established polymyalgia rheumatica and giant cell arteritis who returns for rheumatologic evaluation following her routine health maintenance visit three months prior. She presents with a 2-month history of pain and stiffness in her shoulders and hips, which is worse in the mornings and improves with activity throughout the day. She also reports experiencing mild fatigue and a low-grade fever.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoporosis screening in malabsorptive disorder (diagnosed 2020-01-15)
- Hypertension
- Polymyalgia rheumatica
- Giant cell arteritis
- Hiatal hernia
- History of pulmonary tuberculosis
- Chronic angle-closure glaucoma

PAST SURGICAL HISTORY:
- Ascending aortic graft replacement (Type A dissection repair)
- Laser peripheral iridotomy, right eye

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Lisinopril 10 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with giant cell arteritis
- Father with hypertension and aortic aneurysm
- Sister with primary open-angle glaucoma

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired elementary school teacher.

PHYSICAL EXAMINATION:
Bilateral shoulder and hip girdle tenderness without joint swelling. Range of motion is limited due to pain.

LABORATORY DATA:
- Erythrocyte sedimentation rate (ESR): 70 mm/hr (Normal: 0-20 mm/hr)
- C-reactive protein (CRP): 25 mg/L (Normal: <10 mg/L)
- Complete blood count (CBC): Normal
- Rheumatoid factor (RF): Negative
- Antinuclear antibodies (ANA): Negative

## Encounter 2: source 2020-10-15, practice 2023-08-01, outpatient, Gastroenterology

Chief complaint: Intermittent chest discomfort and early satiety

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Priya Patel, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent chest discomfort and early satiety

HISTORY OF PRESENT ILLNESS:
The patient is a 70-year-old female with a history of polymyalgia rheumatica diagnosed in April 2020 and known hiatal hernia who presents for evaluation of recurrent upper gastrointestinal symptoms. She reports occasional shortness of breath after large dinners and states that she sometimes feels pressure in the middle of her chest when lying down. The discomfort is not related to exertion and is not relieved by antacids. She denies dysphagia to solids or liquids but notes that she occasionally feels bloated after eating and has mild early satiety.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoporosis screening in malabsorptive disorder (diagnosed 2020-01-15)
- Polymyalgia rheumatica (diagnosed 2020-04-15)
- Hypertension
- Giant cell arteritis
- Hiatal hernia
- History of pulmonary tuberculosis
- Chronic angle-closure glaucoma

PAST SURGICAL HISTORY:
- Ascending aortic graft replacement (Type A dissection repair)
- Laser peripheral iridotomy, right eye

MEDICATIONS:
She takes lisinopril for hypertension and no other medications.

Home Medications:
- Hydrochlorothiazide 12.5 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with giant cell arteritis
- Father with hypertension and aortic aneurysm
- Sister with primary open-angle glaucoma

SOCIAL HISTORY:
She does not smoke, drink alcohol, or use illicit drugs.

VITAL SIGNS:
The patient’s BMI is 27 kg/m². Her temperature is 37.1°C (98.8°F), blood pressure is 138/80 mm Hg, and pulse is 76/min.

PHYSICAL EXAMINATION:
Cardiopulmonary examination is unremarkable. Abdominal examination reveals no tenderness, organomegaly, or masses. Bowel sounds are normal.

IMAGING:
A chest x-ray obtained for unrelated reasons last month showed an air-fluid level in the posterior mediastinum. Upper gastrointestinal barium swallow performed today demonstrates a portion of the gastric fundus located above the diaphragm, with the gastroesophageal junction in its normal subdiaphragmatic position.

## Encounter 3: source 2021-04-15, practice 2024-01-18, outpatient, Ophthalmology

Chief complaint: Progressive vision changes with color desaturation over 10 days

ENCOUNTER NOTE
Date: 2021-04-15
Provider: Dr. David Kim, MD
Department: Ophthalmology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive vision changes with color desaturation over 10 days

HISTORY OF PRESENT ILLNESS:
This 71-year-old with chronic angle-closure glaucoma and pulmonary tuberculosis presents with vision changes. She reports that her vision has become "washed out" and that colors appear faded, especially reds and greens. She also has increasing difficulty seeing details directly in front of her.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoporosis screening in malabsorptive disorder (diagnosed 2020-01-15)
- Polymyalgia rheumatica (diagnosed 2020-04-15)
- Hiatal hernia (diagnosed 2020-10-15)
- Hypertension
- Giant cell arteritis
- History of pulmonary tuberculosis
- Chronic angle-closure glaucoma

Two months ago, she was diagnosed with active pulmonary tuberculosis, confirmed by sputum culture, and started on standard multi-drug therapy.

PAST SURGICAL HISTORY:
- Ascending aortic graft replacement (Type A dissection repair)
- Laser peripheral iridotomy, right eye

MEDICATIONS:
Her only other medication is lisinopril for hypertension.

Home Medications:
- Hydrochlorothiazide 12.5 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with giant cell arteritis
- Father with hypertension and aortic aneurysm
- Sister with primary open-angle glaucoma

SOCIAL HISTORY:
She does not smoke or drink alcohol.

PHYSICAL EXAMINATION:
Visual acuity is 20/60 bilaterally. Fundoscopic exam is normal. Visual field testing reveals central scotomas and red-green color discrimination impairment. Peripheral vision is intact.

## Encounter 4: source 2021-10-15, practice 2024-08-06, outpatient, Rheumatology

Chief complaint: New-onset fatigue and difficulty raising arms above head

ENCOUNTER NOTE
Date: 2021-10-15
Provider: Dr. Marcus Johnson, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
New-onset fatigue and difficulty raising arms above head

HISTORY OF PRESENT ILLNESS:
The patient returns to rheumatology with recurrent symptoms following her initial polymyalgia rheumatica diagnosis and recent ethambutol-induced optic neuropathy. She says her shoulders and hips feel stiff in the morning, especially when she gets out of bed, and the stiffness improves throughout the day. She has had no visual symptoms, headaches, or jaw discomfort.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoporosis screening in malabsorptive disorder (diagnosed 2020-01-15)
- Polymyalgia rheumatica (diagnosed 2020-04-15)
- Hiatal hernia (diagnosed 2020-10-15)
- Ethambutol-induced optic neuropathy (diagnosed 2021-04-15)
- Hypertension
- Giant cell arteritis
- History of pulmonary tuberculosis
- Chronic angle-closure glaucoma

PAST SURGICAL HISTORY:
- Ascending aortic graft replacement (Type A dissection repair)
- Laser peripheral iridotomy, right eye

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg daily
- Hydrochlorothiazide 12.5 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with giant cell arteritis
- Father with hypertension and aortic aneurysm
- Sister with primary open-angle glaucoma

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired elementary school teacher.

REVIEW OF SYSTEMS:
She denies weight loss, fevers, or recent illness.

PHYSICAL EXAMINATION:
Physical examination reveals reduced active range of motion in both shoulders, but no synovitis or joint swelling. There is no temporal artery tenderness.

LABORATORY DATA:
Laboratory testing shows an elevated erythrocyte sedimentation rate. Rheumatoid factor and ANA are negative.

## Encounter 5: source 2022-04-15, practice 2025-01-27, ed, Emergency Medicine

Chief complaint: Three-week history of persistent right-sided head pain with jaw claudication

ENCOUNTER NOTE
Date: 2022-04-15
Provider: Dr. Emily Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Three-week history of persistent right-sided head pain with jaw claudication

HISTORY OF PRESENT ILLNESS:
The patient with polymyalgia rheumatica and prior rheumatology visits for shoulder stiffness presents to the emergency department. She says the pain is worse in the evenings and describes it as “tight and uncomfortable.” She also notes feeling more tired than usual and having difficulty lifting her arms to brush her hair due to shoulder discomfort. Her appetite is decreased, and she has unintentionally lost 3 lb (1.4 kg).

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoporosis screening in malabsorptive disorder (diagnosed 2020-01-15)
- Polymyalgia rheumatica (diagnosed 2020-04-15)
- Hiatal hernia (diagnosed 2020-10-15)
- Ethambutol-induced optic neuropathy (diagnosed 2021-04-15)
- Hypertension
- Giant cell arteritis
- History of pulmonary tuberculosis
- Chronic angle-closure glaucoma

PAST SURGICAL HISTORY:
- Ascending aortic graft replacement (Type A dissection repair)
- Laser peripheral iridotomy, right eye

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg daily
- Hydrochlorothiazide 12.5 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with giant cell arteritis
- Father with hypertension and aortic aneurysm
- Sister with primary open-angle glaucoma

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired elementary school teacher.

VITAL SIGNS:
Temperature is 37.7°C (99.9°F)

PHYSICAL EXAMINATION:
Physical examination shows tenderness to palpation over the right temple. Strength testing is normal.

LABORATORY DATA:
Lab studies show:
Hemoglobin 10.8 g/dL
ESR 92 mm/hr
Creatine kinase                        Normal
TSH Normal

## Encounter 6: source 2022-07-15, practice 2025-05-02, icu, Cardiothoracic Surgery

Chief complaint: Sudden onset severe tearing chest pain radiating to back

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. James O'Brien, MD
Department: Cardiothoracic Surgery
Visit Type: icu

CHIEF COMPLAINT:
Sudden onset severe tearing chest pain radiating to back

HISTORY OF PRESENT ILLNESS:
Patient with polymyalgia rheumatica, giant cell arteritis presents with chest pain. She describes the pain as "tearing" in nature. She also reports a 3-month history of headaches, jaw claudication, and intermittent blurred vision.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoporosis screening in malabsorptive disorder (diagnosed 2020-01-15)
- Polymyalgia rheumatica (diagnosed 2020-04-15)
- Hiatal hernia (diagnosed 2020-10-15)
- Ethambutol-induced optic neuropathy (diagnosed 2021-04-15)
- Giant cell arteritis or autoimmune inflammatory condition (diagnosed 2022-04-15)
- Hypertension
- Giant cell arteritis
- History of pulmonary tuberculosis
- Chronic angle-closure glaucoma

PAST SURGICAL HISTORY:
- Ascending aortic graft replacement (Type A dissection repair)
- Laser peripheral iridotomy, right eye

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg daily
- Hydrochlorothiazide 12.5 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with giant cell arteritis
- Father with hypertension and aortic aneurysm
- Sister with primary open-angle glaucoma

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired elementary school teacher.

VITAL SIGNS:
On physical examination, her blood pressure is 170/90 mmHg in the right arm and 160/88 mmHg in the left arm.

PHYSICAL EXAMINATION:
Her temporal arteries are tender to palpation and have a decreased pulse.

LABORATORY DATA:
- Hemoglobin: 12.1 g/dL
- White blood cell count: 9,000/uL
- Platelet count: 450,000/uL
- Erythrocyte sedimentation rate (ESR): 85 mm/hr
- C-reactive protein (CRP): 25 mg/L

IMAGING:
A chest X-ray reveals a widened mediastinum.

## Encounter 7: source 2023-01-15, practice 2025-10-21, ed, Emergency Medicine

Chief complaint: Severe right eye pain, blurred vision, and halos around lights

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Aisha Washington, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe right eye pain, blurred vision, and halos around lights

HISTORY OF PRESENT ILLNESS:
The patient is a 73-year-old female with chronic angle-closure glaucoma and prior ethambutol-induced optic neuropathy who presents with sudden onset of severe pain in her right eye, blurred vision, and seeing halos around lights that started a few hours ago after attending a movie premiere. She also reports a headache and nausea.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoporosis screening in malabsorptive disorder (diagnosed 2020-01-15)
- Polymyalgia rheumatica (diagnosed 2020-04-15)
- Hiatal hernia (diagnosed 2020-10-15)
- Ethambutol-induced optic neuropathy (diagnosed 2021-04-15)
- Giant cell arteritis or autoimmune inflammatory condition (diagnosed 2022-04-15)
- Type A Aortic Dissection (diagnosed 2022-07-15)
- Hypertension
- Giant cell arteritis
- History of pulmonary tuberculosis
- Chronic angle-closure glaucoma

Her medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- Ascending aortic graft replacement (Type A dissection repair)
- Laser peripheral iridotomy, right eye

MEDICATIONS:
She takes hydrochlorothiazide.

Home Medications:
- Lisinopril 10 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with giant cell arteritis
- Father with hypertension and aortic aneurysm
- Sister with primary open-angle glaucoma

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired elementary school teacher.

PHYSICAL EXAMINATION:
On physical examination, her right eye is red, the cornea appears cloudy, and the pupil is mid-dilated and non-reactive to light. Intraocular pressure (IOP) in the right eye is 52 mmHg. The left eye examination is unremarkable with an IOP of 16 mmHg. Visual acuity is decreased in the right eye. Fundoscopic examination is difficult due to corneal edema.
