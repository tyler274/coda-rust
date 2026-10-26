# synthetic_hospital patient 2779

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex F, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-09-25 (Mon) | outpatient | Gastroenterology |
| 1 | 2020-09-15 | 2024-05-27 (Mon) | outpatient | Neurology |
| 2 | 2021-11-15 | 2025-07-24 (Thu) | ed | Emergency Medicine |
| 3 | 2022-11-15 | 2026-07-15 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Elementary school teacher",
 "alcohol_use": "Social (1-2 drinks per month)",
 "family_history": [
  "Crohn's disease in maternal uncle",
  "Cholelithiasis requiring cholecystectomy in mother",
  "Colon cancer in maternal grandfather (age 68)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "5 mg/kg IV infusion every 8 weeks",
   "name": "Infliximab"
  },
  {
   "dose": "50 mg PO daily",
   "name": "Azathioprine"
  },
  {
   "dose": "1 mg PO daily",
   "name": "Folic acid"
  },
  {
   "dose": "2000 IU PO daily",
   "name": "Vitamin D3"
  }
 ],
 "surgical_history": [
  "Open cholecystectomy (age 35)",
  "Ileal resection (50 cm, age 28)",
  "Ileal resection (terminal ileum, age 32)",
  "Small bowel stricturoplasty (age 39)"
 ],
 "chronic_conditions": [
  "Crohn's disease with ileal involvement",
  "Vitamin B12 deficiency",
  "Malabsorption syndrome",
  "Status post cholecystectomy"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Vitamin B12 deficiency", "Sigmoid volvulus", "Sphincter of Oddi dysfunction"]
Comorbidities: ["Status post right hemicolectomy"]

## Encounter 0: source 2020-01-15, practice 2023-09-25, outpatient, Gastroenterology

Chief complaint: Worsening fatigue over the past 6 months with difficulty concentrating

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Jennifer Walsh, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening fatigue over the past 6 months with difficulty concentrating

HISTORY OF PRESENT ILLNESS:
Worsening fatigue over the past 6 months. She reports feeling tired even after a full night’s sleep and has had increasing difficulty concentrating at work. She denies any blood loss, heavy periods, or recent illness.

PAST MEDICAL HISTORY:
Active Problem List:
- Crohn's disease with ileal involvement
- Vitamin B12 deficiency
- Malabsorption syndrome
- Status post cholecystectomy

PAST SURGICAL HISTORY:
- Open cholecystectomy (age 35)
- Ileal resection (50 cm, age 28)
- Ileal resection (terminal ileum, age 32)
- Small bowel stricturoplasty (age 39)

MEDICATIONS:
She takes she does not take any supplements or medications.

Home Medications:
- Infliximab 5 mg/kg IV infusion every 8 weeks
- Azathioprine 50 mg PO daily
- Folic acid 1 mg PO daily
- Vitamin D3 2000 IU PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Crohn's disease in maternal uncle
- Cholelithiasis requiring cholecystectomy in mother
- Colon cancer in maternal grandfather (age 68)

SOCIAL HISTORY:
Her diet is mostly vegetarian

PHYSICAL EXAMINATION:
On physical examination, she appears pale but is in no acute distress. Neurologic examination reveals decreased vibratory sensation and impaired proprioception in the lower extremities. Reflexes are slightly diminished at the ankles.

LABORATORY DATA:
Hemoglobin is 9 g/dL and MCV 105 fL. Peripheral smear shows hypersegmented neutrophils.

## Encounter 1: source 2020-09-15, practice 2024-05-27, outpatient, Neurology

Chief complaint: 3-month history of fatigue, weakness, and tingling sensations in hands and feet with balance difficulty

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Marcus Johnson, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
3-month history of fatigue, weakness, and tingling sensations in hands and feet with balance difficulty

HISTORY OF PRESENT ILLNESS:
Returning for B12 deficiency follow-up with ileal Crohn's disease, she reports a recent unintentional weight loss of 10 pounds and has had difficulty maintaining balance when walking.

PAST MEDICAL HISTORY:
Active Problem List:
- Vitamin B12 deficiency (diagnosed 2020-01-15)
- Crohn's disease with ileal involvement
- Malabsorption syndrome
- Status post cholecystectomy

Her Crohn's disease has been complicated by multiple small bowel resections.

PAST SURGICAL HISTORY:
- Open cholecystectomy (age 35)
- Ileal resection (50 cm, age 28)
- Ileal resection (terminal ileum, age 32)
- Small bowel stricturoplasty (age 39)

MEDICATIONS:
Home Medications:
- Infliximab 5 mg/kg IV infusion every 8 weeks
- Azathioprine 50 mg PO daily
- Folic acid 1 mg PO daily
- Vitamin D3 2000 IU PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Crohn's disease in maternal uncle
- Cholelithiasis requiring cholecystectomy in mother
- Colon cancer in maternal grandfather (age 68)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
On physical examination, she appears pale, and her neurological examination reveals decreased vibration sense in her lower extremities and a positive Romberg sign.

LABORATORY DATA:
Her complete blood count shows a hemoglobin of 9.8 g/dL, mean corpuscular volume (MCV) of 115 fL, and a normal white blood cell and platelet count. Serum vitamin B12 level is measured at 220 pg/mL (reference range: 200-900 pg/mL).

Laboratory Values:
- Hemoglobin: 9.8 g/dL
- MCV: 115 fL
- Vitamin B12: 220 pg/mL

## Encounter 2: source 2021-11-15, practice 2025-07-24, ed, Emergency Medicine

Chief complaint: Severe, intermittent right upper quadrant abdominal pain radiating to the back after fatty meals

ENCOUNTER NOTE
Date: 2021-11-15
Provider: Dr. Elena Vasquez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe, intermittent right upper quadrant abdominal pain radiating to the back after fatty meals

HISTORY OF PRESENT ILLNESS:
A 46-year-old female with Crohn's disease with ileal involvement, s/p cholecystectomy, and vitamin B12 deficiency with prior gastroenterology and neurology assessments, presents to the emergency department with a 6-hour history of severe, intermittent pain in the right upper quadrant of the abdomen that radiates to the back. She describes the pain as sharp and cramp-like, worsening after meals, especially those high in fat. She denies any fever, chills, or changes in bowel habits.

PAST MEDICAL HISTORY:
Active Problem List:
- Vitamin B12 deficiency (diagnosed 2020-01-15)
- Crohn's disease with ileal involvement
- Malabsorption syndrome
- Status post cholecystectomy

Her past medical history is significant for cholecystectomy performed two years ago.

PAST SURGICAL HISTORY:
- Open cholecystectomy (age 35)
- Ileal resection (50 cm, age 28)
- Ileal resection (terminal ileum, age 32)
- Small bowel stricturoplasty (age 39)

MEDICATIONS:
Home Medications:
- Infliximab 5 mg/kg IV infusion every 8 weeks
- Azathioprine 50 mg PO daily
- Folic acid 1 mg PO daily
- Vitamin D3 2000 IU PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Crohn's disease in maternal uncle
- Cholelithiasis requiring cholecystectomy in mother
- Colon cancer in maternal grandfather (age 68)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

VITAL SIGNS:
Her vital signs are stable

PHYSICAL EXAMINATION:
On physical examination, there is tenderness to palpation in the right upper quadrant without rebound or guarding. Murphy's sign is negative.

LABORATORY DATA:
Laboratory tests reveal:

- Serum aminotransferases (AST/ALT): Slightly elevated
- Alkaline phosphatase: Elevated
- Total bilirubin: Normal
- Lipase: Normal
- Complete blood count: Normal

IMAGING:
An abdominal ultrasound shows no evidence of gallstones or biliary ductal dilation.

## Encounter 3: source 2022-11-15, practice 2026-07-15, ed, Emergency Medicine

Chief complaint: 12-hour history of progressively worsening abdominal pain with vomiting and inability to pass flatus

ENCOUNTER NOTE
Date: 2022-11-15
Provider: Dr. David Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
12-hour history of progressively worsening abdominal pain with vomiting and inability to pass flatus

HISTORY OF PRESENT ILLNESS:
This 47-year-old female with Crohn's disease with ileal involvement, s/p cholecystectomy, and vitamin B12 deficiency previously evaluated in gastroenterology and neurology, with prior emergency department presentation in 2021 for abdominal pain, returns with a 12-hour history of progressively worsening abdominal pain. The pain is initially colicky and located in the right lower quadrant but has become constant and diffuse. She reports nausea and one episode of non-bilious vomiting. She has not passed flatus or had a bowel movement since the pain started.

PAST MEDICAL HISTORY:
Active Problem List:
- Vitamin B12 deficiency (diagnosed 2020-01-15)
- Sphincter of Oddi dysfunction (diagnosed 2021-11-15)
- Crohn's disease with ileal involvement
- Malabsorption syndrome
- Status post cholecystectomy

Her past medical history is significant for an open cholecystectomy 10 years ago.

PAST SURGICAL HISTORY:
- Open cholecystectomy (age 35)
- Ileal resection (50 cm, age 28)
- Ileal resection (terminal ileum, age 32)
- Small bowel stricturoplasty (age 39)

MEDICATIONS:
Home Medications:
- Infliximab 5 mg/kg IV infusion every 8 weeks
- Azathioprine 50 mg PO daily
- Folic acid 1 mg PO daily
- Vitamin D3 2000 IU PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Crohn's disease in maternal uncle
- Cholelithiasis requiring cholecystectomy in mother
- Colon cancer in maternal grandfather (age 68)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
On examination, her abdomen is distended and tympanic to percussion with tenderness in the right lower quadrant. There is no rebound tenderness or guarding. Bowel sounds are high-pitched and hyperactive.

LABORATORY DATA:
Her laboratory values are as follows:

- White blood cell count: 12,000 /µL
- Hemoglobin: 13.5 g/dL
- Platelet count: 250,000 /µL
- Sodium: 138 mEq/L
- Potassium: 3.9 mEq/L
- Chloride: 100 mEq/L
- Bicarbonate: 24 mEq/L
- Blood urea nitrogen: 18 mg/dL
- Creatinine: 0.9 mg/dL

IMAGING:
An abdominal x-ray reveals a large, dilated loop of colon in the right upper quadrant with a "coffee bean" sign.
