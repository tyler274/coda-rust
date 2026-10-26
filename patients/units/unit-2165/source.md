# synthetic_hospital patient 2165

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 43 at first encounter, sex M, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-07-08 (Mon) | outpatient | Gastroenterology |
| 1 | 2022-01-15 | 2026-07-16 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Database Administrator",
 "alcohol_use": "Lifelong abstinence",
 "family_history": [
  "Father with ulcerative colitis",
  "Maternal cousin with Crohn disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1.2 g orally twice daily",
   "name": "Mesalamine"
  },
  {
   "dose": "300 mg orally three times daily",
   "name": "Ursodiol"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Primary sclerosing cholangitis",
  "Ulcerative colitis"
 ],
 "age_at_first_encounter": 43
}
```

Primary diagnoses: ["Obstructive jaundice", "Primary sclerosing cholangitis"]
Comorbidities: ["Ulcerative colitis"]

## Encounter 0: source 2020-01-15, practice 2024-07-08, outpatient, Gastroenterology

Chief complaint: Fatigue and generalized itching for the past two months

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue and generalized itching for the past two months

HISTORY OF PRESENT ILLNESS:
He presents to the clinic complaining of fatigue and generalized itching for the past two months. He denies any jaundice, dark urine, or pale stools. His ulcerative colitis has been well-controlled on mesalamine.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary sclerosing cholangitis
- Ulcerative colitis

MEDICATIONS:
He takes mesalamine.

Home Medications:
- Ursodiol 300 mg orally three times daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with ulcerative colitis
- Maternal cousin with Crohn disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Lifelong abstinence. Occupation: Database Administrator.

PHYSICAL EXAMINATION:
On examination, he appears well but is scratching his arms and back. His abdomen is soft, non-tender, and without hepatosplenomegaly.

LABORATORY DATA:
Total bilirubin: 1.2 mg/dL (normal 0.3-1.2 mg/dL)
- Direct bilirubin: 0.4 mg/dL (normal 0.0-0.3 mg/dL)
- Alkaline phosphatase: 420 U/L (normal 40-130 U/L)
- Aspartate aminotransferase (AST): 50 U/L (normal 10-40 U/L)
- Alanine aminotransferase (ALT): 55 U/L (normal 7-56 U/L)

ASSESSMENT AND PLAN:
Given the patient's history and presentation,

## Encounter 1: source 2022-01-15, practice 2026-07-16, ed, Emergency Medicine

Chief complaint: 3-month history of subacute right upper quadrant pain and unexplained weight loss with recent onset of pruritus and dark urine

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
3-month history of subacute right upper quadrant pain and unexplained weight loss with recent onset of pruritus and dark urine

HISTORY OF PRESENT ILLNESS:
History includes PSC from 2020. He reports recent onset of pruritus and dark urine.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary sclerosing cholangitis (diagnosed 2020-01-15)
- Ulcerative colitis

His past medical history is significant for primary sclerosing cholangitis diagnosed two years ago.

MEDICATIONS:
Home Medications:
- Mesalamine 1.2 g orally twice daily
- Ursodiol 300 mg orally three times daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with ulcerative colitis
- Maternal cousin with Crohn disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Lifelong abstinence. Occupation: Database Administrator.

PHYSICAL EXAMINATION:
He appears cachectic and jaundiced. His liver is palpable 3 cm below the right costal margin, and the spleen is not palpable. Murphy&#x27;s sign is negative.

LABORATORY DATA:
- Total bilirubin: 5.2 mg/dL (normal: 0.3-1.2 mg/dL)
- Direct bilirubin: 3.8 mg/dL (normal: 0.0-0.3 mg/dL)
- Alkaline phosphatase: 420 U/L (normal: 40-130 U/L)
- Aspartate aminotransferase (AST): 80 U/L (normal: 10-40 U/L)
- Alanine aminotransferase (ALT): 75 U/L (normal: 7-56 U/L)
- CA 19-9: Elevated

IMAGING:
Abdominal ultrasound shows dilation of the intrahepatic and common bile ducts without evidence of gallstones. Magnetic resonance cholangiopancreatography (MRCP) reveals an irregular stricture of the common bile duct.
