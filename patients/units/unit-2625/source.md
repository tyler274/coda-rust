# synthetic_hospital patient 2625

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex M, Non-Hispanic White, Private PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-10-14 (Mon) | outpatient | Gastroenterology |
| 1 | 2021-01-15 | 2025-10-06 (Mon) | outpatient | Gastroenterology |
| 2 | 2021-07-15 | 2026-04-20 (Mon) | outpatient | Gastroenterology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private PPO",
 "occupation": "Financial analyst",
 "alcohol_use": "None",
 "family_history": [
  "Maternal grandmother with ulcerative colitis",
  "Father with primary sclerosing cholangitis",
  "Mother with autoimmune hypothyroidism"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1.2 g oral twice daily",
   "name": "Mesalamine"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Ulcerative colitis"
 ],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Primary sclerosing cholangitis", "Ulcerative colitis"]
Comorbidities: ["Ulcerative colitis"]

## Encounter 0: source 2020-01-15, practice 2024-10-14, outpatient, Gastroenterology

Chief complaint: Routine follow-up for ulcerative colitis

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine follow-up for ulcerative colitis

HISTORY OF PRESENT ILLNESS:
He reports compliance with his medication and has not had a flare in the past year. He denies any recent bloody stools, abdominal pain, or weight loss.

PAST MEDICAL HISTORY:
Active Problem List:
- Ulcerative colitis

He was diagnosed with ulcerative colitis 9 years ago.

MEDICATIONS:
He has been on mesalamine since then

ALLERGIES:
NKDA

FAMILY HISTORY:
His family history is significant for colorectal cancer in his father, who was diagnosed at age 50.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Financial analyst.

VITAL SIGNS:
His vital signs are stable

PHYSICAL EXAMINATION:
Abdominal examination reveals no tenderness or masses

LABORATORY DATA:
Laboratory tests show normal complete blood count and basic metabolic panel.

ASSESSMENT AND PLAN:
Given the patient's history and current status,

## Encounter 1: source 2021-01-15, practice 2025-10-06, outpatient, Gastroenterology

Chief complaint: Fatigue and pruritus

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue and pruritus

HISTORY OF PRESENT ILLNESS:
The patient with ulcerative colitis returns for follow-up with fatigue and pruritus for the past 6 months. He mentions that he has had intermittent episodes of bloody diarrhea for the past year, which he attributed to hemorrhoids.

PAST MEDICAL HISTORY:
Active Problem List:
- Ulcerative colitis (diagnosed 2020-01-15)

MEDICATIONS:
Home Medications:
- Mesalamine 1.2 g oral twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
His family history is notable for ulcerative colitis in his mother.

SOCIAL HISTORY:
He denies any recent travel, alcohol use, or risky sexual behaviors.

PHYSICAL EXAMINATION:
On physical examination, he appears well-nourished and jaundiced. His abdomen is soft, non-distended, with no tenderness on palpation. There is no hepatosplenomegaly or ascites.

LABORATORY DATA:
Total bilirubin: 2.1 mg/dL (normal: 0.3-1.2 mg/dL)
- Direct bilirubin: 1.5 mg/dL (normal: 0.0-0.3 mg/dL)
- Alkaline phosphatase: 420 U/L (normal: 40-130 U/L)
- γ-glutamyl transpeptidase (GGT): 210 U/L (normal: 9-48 U/L)
- Aspartate aminotransferase (AST): 50 U/L (normal: 10-40 U/L)
- Alanine aminotransferase (ALT): 55 U/L (normal: 7-56 U/L)

IMAGING:
Magnetic resonance cholangiopancreatography (MRCP) shows multifocal intrahepatic and extrahepatic biliary strictures with segmental dilations.

## Encounter 2: source 2021-07-15, practice 2026-04-20, outpatient, Gastroenterology

Chief complaint: 6-month history of fatigue, pruritus, and intermittent jaundice

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
6-month history of fatigue, pruritus, and intermittent jaundice

HISTORY OF PRESENT ILLNESS:
The patient with ulcerative colitis and known primary sclerosing cholangitis returns for follow-up six months after initial evaluation for fatigue and pruritus. He reports having pale stools and dark urine.

PAST MEDICAL HISTORY:
Active Problem List:
- Ulcerative colitis (diagnosed 2020-01-15)
- Primary sclerosing cholangitis (diagnosed 2021-01-15)

His past medical history is significant for ulcerative colitis, which has been well-controlled on mesalamine.

MEDICATIONS:
Home Medications:
- Mesalamine 1.2 g oral twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal grandmother with ulcerative colitis
- Father with primary sclerosing cholangitis
- Mother with autoimmune hypothyroidism

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Financial analyst.

PHYSICAL EXAMINATION:
On physical examination, he is afebrile, and his sclerae are icteric. His abdomen is soft, non-distended, with no palpable masses, but there is mild right upper quadrant tenderness.

LABORATORY DATA:
Laboratory studies reveal:

- Total bilirubin: 3.2 mg/dL (normal: 0.3-1.2 mg/dL)
- Direct bilirubin: 2.5 mg/dL (normal: 0.0-0.3 mg/dL)
- Alkaline phosphatase: 420 U/L (normal: 40-130 U/L)
- Aspartate aminotransferase (AST): 80 U/L (normal: 10-40 U/L)
- Alanine aminotransferase (ALT): 75 U/L (normal: 7-56 U/L)

IMAGING:
Abdominal ultrasound shows beading and irregularity of the intrahepatic bile ducts.
