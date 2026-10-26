# synthetic_hospital patient 1774

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 29 at first encounter, sex F, White, non-Hispanic, Private PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-01-28 (Thu) | outpatient | Dermatology |
| 1 | 2020-04-15 | 2021-04-26 (Mon) | outpatient | Gastroenterology |
| 2 | 2020-11-15 | 2021-12-03 (Fri) | outpatient | Dermatology |
| 3 | 2021-09-15 | 2022-09-26 (Mon) | outpatient | Rheumatology |
| 4 | 2022-09-15 | 2023-10-02 (Mon) | outpatient | Internal Medicine |
| 5 | 2023-09-15 | 2024-10-06 (Sun) | ed | Emergency Medicine |
| 6 | 2024-09-15 | 2025-09-28 (Sun) | inpatient | Hematology/Oncology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private PPO",
 "occupation": "Graphic designer",
 "alcohol_use": "Occasional, 1-2 drinks per month",
 "family_history": [
  "Mother: Systemic lupus erythematosus",
  "Maternal grandmother: Rheumatoid arthritis",
  "Paternal aunt: Celiac disease",
  "Father: Type 2 diabetes mellitus"
 ],
 "race_ethnicity": "White, non-Hispanic",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "200 mg PO twice daily",
   "name": "Hydroxychloroquine"
  },
  {
   "dose": "325 mg PO three times daily",
   "name": "Ferrous sulfate"
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
  "None"
 ],
 "chronic_conditions": [
  "Systemic lupus erythematosus",
  "Celiac disease",
  "Iron deficiency anemia",
  "Recurrent aphthous stomatitis"
 ],
 "age_at_first_encounter": 29
}
```

Primary diagnoses: ["Systemic lupus erythematosus", "Celiac disease", "Dermatitis herpetiformis", "Cold autoimmune hemolytic anemia", "Warm autoimmune hemolytic anemia"]
Comorbidities: ["Recurrent Oral Aphthae", "Iron Deficiency Anemia", "Systemic lupus erythematosus"]

## Encounter 0: source 2020-01-15, practice 2021-01-28, outpatient, Dermatology

Chief complaint: 2-week history of facial rash and joint pain affecting hands and wrists with increased sensitivity to sunlight

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
2-week history of facial rash and joint pain affecting hands and wrists with increased sensitivity to sunlight

HISTORY OF PRESENT ILLNESS:
A 28-year-old woman presents with a 2-week history of facial rash and joint pain affecting her hands and wrists. She reports morning stiffness lasting about an hour and increased sensitivity to sunlight.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus
- Celiac disease
- Iron deficiency anemia
- Recurrent aphthous stomatitis

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg PO twice daily
- Ferrous sulfate 325 mg PO three times daily
- Folic acid 1 mg PO daily
- Vitamin D3 2000 IU PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
Her mother has rheumatoid arthritis.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional, 1-2 drinks per month. Occupation: Graphic designer.

VITAL SIGNS:
Her temperature is 37.4°C (99.3°F), blood pressure is 118/74 mmHg, pulse is 78/min, respiratory rate is 16/min, and oxygen saturation is 98% on room air.

PHYSICAL EXAMINATION:
Physical examination findings are shown below.

LABORATORY DATA:
Laboratory tests reveal positive antinuclear antibodies (ANA), positive anti–double-stranded DNA antibodies, and low complement C3 and C4.

## Encounter 1: source 2020-04-15, practice 2021-04-26, outpatient, Gastroenterology

Chief complaint: 6-month history of chronic diarrhea, bloating, and 10-pound weight loss with symptoms worsening after consuming bread and pasta

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Okafor, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
6-month history of chronic diarrhea, bloating, and 10-pound weight loss with symptoms worsening after consuming bread and pasta

HISTORY OF PRESENT ILLNESS:
The patient, recently diagnosed with systemic lupus erythematosus in Dermatology, presents with gastrointestinal complaints. She reports that her symptoms seem to worsen after consuming bread and pasta. She denies any blood in her stool.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus (diagnosed 2020-01-15)
- Celiac disease
- Iron deficiency anemia
- Recurrent aphthous stomatitis

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg PO twice daily
- Ferrous sulfate 325 mg PO three times daily
- Folic acid 1 mg PO daily
- Vitamin D3 2000 IU PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Systemic lupus erythematosus
- Maternal grandmother: Rheumatoid arthritis
- Paternal aunt: Celiac disease
- Father: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional, 1-2 drinks per month. Occupation: Graphic designer.

PHYSICAL EXAMINATION:
A slim woman appearing her stated age. Her abdomen is soft, non-distended with mild diffuse tenderness but no rebound or guarding. Bowel sounds are normal.

LABORATORY DATA:
Hemoglobin: 10.2 g/dL (Normal: 12-16 g/dL)
- Mean corpuscular volume (MCV): 74 fL (Normal: 80-100 fL)
- Serum iron: 30 µg/dL (Normal: 50-170 µg/dL)
- Total iron-binding capacity (TIBC): 450 µg/dL (Normal: 250-450 µg/dL)
- Serum ferritin: 15 ng/mL (Normal: 15-200 ng/mL)
- Anti-tissue transglutaminase IgA: Positive
- Endomysial antibodies (EMA): Positive

ASSESSMENT AND PLAN:
Given the patient's symptoms, medical history, and laboratory findings,

## Encounter 2: source 2020-11-15, practice 2021-12-03, outpatient, Dermatology

Chief complaint: 6-month history of intensely pruritic skin lesions that began on elbows and spread to knees, lower back, and buttocks

ENCOUNTER NOTE
Date: 2020-11-15
Provider: Dr. Sarah Chen, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
6-month history of intensely pruritic skin lesions that began on elbows and spread to knees, lower back, and buttocks

HISTORY OF PRESENT ILLNESS:
The patient with systemic lupus erythematosus and celiac disease returns with a new complaint. She reports that the lesions began on her elbows and have since spread to her knees, lower back, and buttocks. She denies any recent travel, new medications, or changes in her detergents or soaps.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus (diagnosed 2020-01-15)
- Celiac disease (diagnosed 2020-04-15)
- Iron deficiency anemia
- Recurrent aphthous stomatitis

Her past medical history is significant for iron-deficiency anemia and recurrent oral ulcers.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg PO twice daily
- Ferrous sulfate 325 mg PO three times daily
- Folic acid 1 mg PO daily
- Vitamin D3 2000 IU PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Systemic lupus erythematosus
- Maternal grandmother: Rheumatoid arthritis
- Paternal aunt: Celiac disease
- Father: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional, 1-2 drinks per month. Occupation: Graphic designer.

REVIEW OF SYSTEMS:
She has no joint pain, abdominal pain, or diarrhea.

PHYSICAL EXAMINATION:
Physical examination reveals clusters of erythematous papules and vesicles over her extensor surfaces. A few lesions appear excoriated.

LABORATORY DATA:
Laboratory studies show:

- Hemoglobin: 10.2 g/dL (Normal: 12-16 g/dL)
- Mean corpuscular volume (MCV): 74 fL (Normal: 80-100 fL)
- Tissue transglutaminase IgA: Positive

PATHOLOGY:
Dermatopathology (skin biopsy): Subepidermal vesicles with neutrophilic infiltrates

## Encounter 3: source 2021-09-15, practice 2022-09-26, outpatient, Rheumatology

Chief complaint: Joint pain and fatigue for several months with morning stiffness lasting one hour and occasional painless mouth sores

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. Priya Sharma, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Joint pain and fatigue for several months with morning stiffness lasting one hour and occasional painless mouth sores

HISTORY OF PRESENT ILLNESS:
The patient with systemic lupus erythematosus and celiac disease returns for rheumatology follow-up. She reports morning stiffness lasting about an hour that improves slightly with movement. She also mentions having occasional painless mouth sores and increased hair loss recently. She denies fever, cough, or recent infections.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus (diagnosed 2020-01-15)
- Celiac disease (diagnosed 2020-04-15)
- Dermatitis herpetiformis (diagnosed 2020-11-15)
- Iron deficiency anemia
- Recurrent aphthous stomatitis

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg PO twice daily
- Ferrous sulfate 325 mg PO three times daily
- Folic acid 1 mg PO daily
- Vitamin D3 2000 IU PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Systemic lupus erythematosus
- Maternal grandmother: Rheumatoid arthritis
- Paternal aunt: Celiac disease
- Father: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional, 1-2 drinks per month. Occupation: Graphic designer.

VITAL SIGNS:
On examination, temperature is 37.2°C (99°F), blood pressure is 132/78 mmHg, pulse is 76/min, and respirations are 14/min.

PHYSICAL EXAMINATION:
There is mild tenderness of both wrists without swelling or erythema. Oral examination shows shallow ulcers on the buccal mucosa. Cardiopulmonary and neurologic examinations are normal.

LABORATORY DATA:
Laboratory studies show:
Hemoglobin 11.8 g/dL
WBC count 3,200 /mm³
Serum creatinine 0.9 mg/dL
Urinalysis Normal

## Encounter 4: source 2022-09-15, practice 2023-10-02, outpatient, Internal Medicine

Chief complaint: 3-month history of intermittent joint pains throughout body, notably worse in hands, with recent painful mouth ulcer

ENCOUNTER NOTE
Date: 2022-09-15
Provider: Dr. James Wilson, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
3-month history of intermittent joint pains throughout body, notably worse in hands, with recent painful mouth ulcer

HISTORY OF PRESENT ILLNESS:
The patient is a 31-year-old female with systemic lupus erythematosus and celiac disease who returns for follow-up evaluation. She reports intermittent joint pains throughout her body, but she says they're worse in her hands on most occasions. Today her fingers are notably sore. After further questioning, she acknowledges that she had a painful mouth ulcer a few months ago that self-resolved after about a week and a half.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus (diagnosed 2020-01-15)
- Celiac disease (diagnosed 2020-04-15)
- Dermatitis herpetiformis (diagnosed 2020-11-15)
- Iron deficiency anemia
- Recurrent aphthous stomatitis

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg PO twice daily
- Ferrous sulfate 325 mg PO three times daily
- Folic acid 1 mg PO daily
- Vitamin D3 2000 IU PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
Her father had an episode of deQuervain thyroiditis several years ago.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional, 1-2 drinks per month. Occupation: Graphic designer.

VITAL SIGNS:
Vitals are normal.

LABORATORY DATA:
Laboratory studies show antibodies against the nuclear Smith antigen.

## Encounter 5: source 2023-09-15, practice 2024-10-06, ed, Emergency Medicine

Chief complaint: Progressive fatigue, shortness of breath, dark urine, and mild yellowing of eyes over the past 3 weeks

ENCOUNTER NOTE
Date: 2023-09-15
Provider: Dr. Elena Vasquez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive fatigue, shortness of breath, dark urine, and mild yellowing of eyes over the past 3 weeks

HISTORY OF PRESENT ILLNESS:
The patient with established SLE and celiac disease presents with progressive fatigue and shortness of breath over the past 3 weeks. She also notes dark urine and mild yellowing of her eyes.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus (diagnosed 2020-01-15)
- Celiac disease (diagnosed 2020-04-15)
- Dermatitis herpetiformis (diagnosed 2020-11-15)
- Iron deficiency anemia
- Recurrent aphthous stomatitis

She has a history of systemic lupus erythematosus (SLE) but is currently not on immunosuppressive therapy.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg PO twice daily
- Ferrous sulfate 325 mg PO three times daily
- Folic acid 1 mg PO daily
- Vitamin D3 2000 IU PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Systemic lupus erythematosus
- Maternal grandmother: Rheumatoid arthritis
- Paternal aunt: Celiac disease
- Father: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional, 1-2 drinks per month. Occupation: Graphic designer.

PHYSICAL EXAMINATION:
On examination, she has scleral icterus and mild splenomegaly.

LABORATORY DATA:
Laboratory results are as follows:
Hemoglobin 8.2 g/dL
Reticulocyte count 8.1%
Total bilirubin 4.5 mg/dL (indirect predominance)
LDH Elevated
Haptoglobin Undetectable
Direct Coombs test                                              Positive
Peripheral smear                                                Spherocytes present

## Encounter 6: source 2024-09-15, practice 2025-09-28, inpatient, Hematology/Oncology

Chief complaint: Progressive fatigue and mild jaundice with acute worsening requiring hospital admission for transfusion support

ENCOUNTER NOTE
Date: 2024-09-15
Provider: Dr. Robert Kim, MD
Department: Hematology/Oncology
Visit Type: inpatient

CHIEF COMPLAINT:
Progressive fatigue and mild jaundice with acute worsening requiring hospital admission for transfusion support

HISTORY OF PRESENT ILLNESS:
Patient with SLE, prior AIHA. Presents with progressive fatigue and mild jaundice over the past month.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus (diagnosed 2020-01-15)
- Celiac disease (diagnosed 2020-04-15)
- Dermatitis herpetiformis (diagnosed 2020-11-15)
- Cold autoimmune hemolytic anemia (diagnosed 2023-09-15)
- Iron deficiency anemia
- Recurrent aphthous stomatitis

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
She takes currently on hydroxychloroquine.

Home Medications:
- Ferrous sulfate 325 mg PO three times daily
- Folic acid 1 mg PO daily
- Vitamin D3 2000 IU PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Systemic lupus erythematosus
- Maternal grandmother: Rheumatoid arthritis
- Paternal aunt: Celiac disease
- Father: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional, 1-2 drinks per month. Occupation: Graphic designer.

PHYSICAL EXAMINATION:
On examination, she has mild scleral icterus but no hepatosplenomegaly

LABORATORY DATA:
Laboratory studies reveal hemoglobin of 8.2 g/dL, reticulocyte count of 8%, and elevated indirect bilirubin. Her direct Coombs test is positive, and urinalysis is negative for hemoglobin. Peripheral smear shows spherocytes without schistocytes.
