# synthetic_hospital patient 1768

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 27 at first encounter, sex F, Ashkenazi Jewish, Preferred Provider Organization (PPO).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-06-03 (Thu) | outpatient | Gastroenterology |
| 1 | 2020-07-15 | 2021-11-22 (Mon) | outpatient | Internal Medicine |
| 2 | 2021-07-15 | 2022-11-29 (Tue) | outpatient | Gastroenterology |
| 3 | 2022-01-15 | 2023-06-05 (Mon) | outpatient | Dermatology |
| 4 | 2022-07-15 | 2023-11-29 (Wed) | outpatient | Gastroenterology |
| 5 | 2023-01-15 | 2024-05-17 (Fri) | outpatient | Dermatology |
| 6 | 2024-01-15 | 2025-05-22 (Thu) | inpatient | Emergency Medicine |
| 7 | 2024-07-15 | 2025-11-17 (Mon) | follow_up | Gastroenterology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Preferred Provider Organization (PPO)",
 "occupation": "Elementary school art teacher",
 "alcohol_use": "Social (1-2 drinks per month)",
 "family_history": [
  "Father: Crohn disease",
  "Mother: Rheumatoid arthritis",
  "Paternal aunt: Hypothyroidism"
 ],
 "race_ethnicity": "Ashkenazi Jewish",
 "smoking_status": "Former smoker (quit 6 months ago; previously 1 pack per day for 10 years)",
 "home_medications": [
  {
   "dose": "800 mg PO TID",
   "name": "Mesalamine delayed-release"
  },
  {
   "dose": "325 mg (65 mg elemental iron) PO daily",
   "name": "Ferrous sulfate"
  },
  {
   "dose": "1000 mcg PO daily",
   "name": "Cyanocobalamin"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Crohn disease (diagnosed age 22; initially classified as ulcerative colitis)",
  "Iron deficiency anemia",
  "Vitamin B12 deficiency (dietary)"
 ],
 "age_at_first_encounter": 27
}
```

Primary diagnoses: ["Crohn Disease", "Small bowel obstruction due to fibrotic stricture secondary to Crohn's disease", "Ulcerative colitis", "Erythema nodosum (isolated)", "Iron Deficiency Anemia"]
Comorbidities: ["Iron Deficiency Anemia", "Crohn Disease"]

## Encounter 0: source 2020-01-15, practice 2021-06-03, outpatient, Gastroenterology

Chief complaint: Intermittent bloody diarrhea, abdominal pain, and weight loss over 6 months with up to six daily bowel movements and nocturnal stools

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent bloody diarrhea, abdominal pain, and weight loss over 6 months with up to six daily bowel movements and nocturnal stools

HISTORY OF PRESENT ILLNESS:
A 23-year-old woman presents to the clinic with a 6-month history of intermittent bloody diarrhea, abdominal pain, and weight loss. She reports having up to six bowel movements daily, often with nocturnal stools.

PAST MEDICAL HISTORY:
Active Problem List:
- Crohn disease (diagnosed age 22; initially classified as ulcerative colitis)
- Iron deficiency anemia
- Vitamin B12 deficiency (dietary)

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Mesalamine delayed-release 800 mg PO TID
- Ferrous sulfate 325 mg (65 mg elemental iron) PO daily
- Cyanocobalamin 1000 mcg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
She denies any family history of gastrointestinal diseases.

SOCIAL HISTORY:
Smoking: Former smoker (quit 6 months ago; previously 1 pack per day for 10 years). Alcohol: Social (1-2 drinks per month). Occupation: Elementary school art teacher.

PHYSICAL EXAMINATION:
On physical examination, she has mild tenderness in the left lower quadrant without rebound or guarding.

LABORATORY DATA:
Laboratory tests reveal:

- Hemoglobin: 10.2 g/dL (Normal: 12-16 g/dL)
- Erythrocyte sedimentation rate (ESR): 45 mm/hr (Normal: 0-20 mm/hr)
- C-reactive protein (CRP): 25 mg/L (Normal: <5 mg/L)
- Albumin: 3.2 g/dL (Normal: 3.5-5.0 g/dL)

Stool cultures and Clostridium difficile toxin assays are negative.

IMAGING:
Colonoscopy shows continuous mucosal inflammation and ulceration starting from the rectum and extending proximally to the splenic flexure. There are also several areas of swollen mucosa that appear as pseudopolyps.

PATHOLOGY:
Biopsies from the affected areas show chronic inflammation limited to the mucosa and submucosa.

## Encounter 1: source 2020-07-15, practice 2021-11-22, outpatient, Internal Medicine

Chief complaint: Fatigue, shortness of breath on exertion, and frequent dizziness for the past two months

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. James O'Brien, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue, shortness of breath on exertion, and frequent dizziness for the past two months

HISTORY OF PRESENT ILLNESS:
She returns with fatigue, dyspnea, dizziness with ulcerative colitis history. She has been following a strict vegan diet for the past year and does not take any supplements.

PAST MEDICAL HISTORY:
Active Problem List:
- Ulcerative colitis (diagnosed 2020-01-15)
- Crohn disease (diagnosed age 22; initially classified as ulcerative colitis)
- Iron deficiency anemia
- Vitamin B12 deficiency (dietary)

She denies any other medical history.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Mesalamine delayed-release 800 mg PO TID
- Ferrous sulfate 325 mg (65 mg elemental iron) PO daily
- Cyanocobalamin 1000 mcg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Crohn disease
- Mother: Rheumatoid arthritis
- Paternal aunt: Hypothyroidism

SOCIAL HISTORY:
Smoking: Former smoker (quit 6 months ago; previously 1 pack per day for 10 years). Alcohol: Social (1-2 drinks per month). Occupation: Elementary school art teacher.

PHYSICAL EXAMINATION:
On physical examination, her skin appears pale, and her conjunctivae are pale as well. Cardiovascular examination reveals a systolic murmur.

LABORATORY DATA:
Laboratory tests reveal:

- Hemoglobin: 9.2 g/dL (Normal: 12-15.5 g/dL)
- Mean corpuscular volume (MCV): 74 fL (Normal: 80-100 fL)
- Serum iron: 30 µg/dL (Normal: 50-170 µg/dL)
- Total iron-binding capacity (TIBC): 600 µg/dL (Normal: 250-450 µg/dL)
- Ferritin: 8 ng/mL (Normal: 15-200 ng/mL)
- Vitamin B12: Normal
- Folate: Normal

## Encounter 2: source 2021-07-15, practice 2022-11-29, outpatient, Gastroenterology

Chief complaint: Intermittent abdominal pain, diarrhea, and progressive weight loss over 6 months with up to six loose stools daily, occasionally bloody

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent abdominal pain, diarrhea, and progressive weight loss over 6 months with up to six loose stools daily, occasionally bloody

HISTORY OF PRESENT ILLNESS:
The patient returns to the gastroenterology clinic following prior encounters in January 2020 and July 2020, with established Crohn disease and iron deficiency anemia. She presents with a 6-month history of intermittent abdominal pain, diarrhea, and weight loss. She reports that her symptoms have progressively worsened over the last two months. She has experienced up to six loose stools per day, occasionally with blood.

PAST MEDICAL HISTORY:
Active Problem List:
- Ulcerative colitis (diagnosed 2020-01-15)
- Iron Deficiency Anemia (diagnosed 2020-07-15)
- Crohn disease (diagnosed age 22; initially classified as ulcerative colitis)
- Vitamin B12 deficiency (dietary)

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Mesalamine delayed-release 800 mg PO TID
- Ferrous sulfate 325 mg (65 mg elemental iron) PO daily
- Cyanocobalamin 1000 mcg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
She denies any family history of gastrointestinal diseases.

SOCIAL HISTORY:
Smoking: Former smoker (quit 6 months ago; previously 1 pack per day for 10 years). Alcohol: Social (1-2 drinks per month). Occupation: Elementary school art teacher.

PHYSICAL EXAMINATION:
On physical examination, she is afebrile, and her abdominal exam reveals tenderness in the right lower quadrant without rebound or guarding. A perianal examination shows a draining fistula.

LABORATORY DATA:
Laboratory tests reveal:

- Hemoglobin: 11.2 g/dL (normal: 12-16 g/dL)
- White blood cell count: 9,500 /µL (normal: 4,500-11,000 /µL)
- C-reactive protein: 24 mg/L (normal: <5 mg/L)
- Erythrocyte sedimentation rate: 40 mm/hr (normal: 0-20 mm/hr)

PATHOLOGY:
Biopsies reveal noncaseating granulomas.

OTHER STUDIES:
Colonoscopy shows discontinuous areas of inflamed mucosa with intervening segments of normal-appearing bowel,

## Encounter 3: source 2022-01-15, practice 2023-06-05, outpatient, Dermatology

Chief complaint: Painful red bumps on her shins for 2 weeks, appeared suddenly and increased in number

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Maria Garcia, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Painful red bumps on her shins for 2 weeks, appeared suddenly and increased in number

HISTORY OF PRESENT ILLNESS:
The patient is a 29-year-old female with Crohn disease and iron deficiency anemia who presents with a 2-week history of painful red bumps on her shins. She reports that the bumps appeared suddenly and have increased in number. She denies any recent travel, sick contacts, or new sexual partners.

PAST MEDICAL HISTORY:
Active Problem List:
- Ulcerative colitis (diagnosed 2020-01-15)
- Iron Deficiency Anemia (diagnosed 2020-07-15)
- Crohn Disease (diagnosed 2021-07-15)
- Vitamin B12 deficiency (dietary)

Her past medical history is significant for ulcerative colitis, which is currently well-controlled on mesalamine.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
She takes no other medications.

Home Medications:
- Mesalamine delayed-release 800 mg PO TID
- Ferrous sulfate 325 mg (65 mg elemental iron) PO daily
- Cyanocobalamin 1000 mcg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Crohn disease
- Mother: Rheumatoid arthritis
- Paternal aunt: Hypothyroidism

SOCIAL HISTORY:
She does not smoke

VITAL SIGNS:
Her vital signs are within normal limits.

PHYSICAL EXAMINATION:
On physical examination, there are multiple 2- to 3-cm tender, erythematous nodules on the anterior aspects of both lower legs. There is no joint swelling or abdominal tenderness.

LABORATORY DATA:
White blood cell count: 9,000/uL (normal range: 4,500-11,000/uL)
- Hemoglobin: 13.2 g/dL (normal range: 12-16 g/dL)
- Erythrocyte sedimentation rate (ESR): 40 mm/hr (normal range: 0-20 mm/hr)
- C-reactive protein (CRP): 2.5 mg/dL (normal range: 0.0-1.0 mg/dL)
- Antistreptolysin O (ASO) titer: Negative

IMAGING:
Chest X-ray: Normal

## Encounter 4: source 2022-07-15, practice 2023-11-29, outpatient, Gastroenterology

Chief complaint: Painful red bumps on legs for 3 weeks accompanied by 2-month history of intermittent abdominal pain, diarrhea, and 5 kg unintentional weight loss

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Painful red bumps on legs for 3 weeks accompanied by 2-month history of intermittent abdominal pain, diarrhea, and 5 kg unintentional weight loss

HISTORY OF PRESENT ILLNESS:
The patient with Crohn disease presents with recurrent bumps on her legs and gastrointestinal symptoms. She reports that the bumps appeared suddenly and have increased in number. She also mentions a 2-month history of intermittent abdominal pain, diarrhea, and unintentional weight loss of 5 kg.

PAST MEDICAL HISTORY:
Active Problem List:
- Ulcerative colitis (diagnosed 2020-01-15)
- Iron Deficiency Anemia (diagnosed 2020-07-15)
- Crohn Disease (diagnosed 2021-07-15)
- Erythema nodosum (isolated) (diagnosed 2022-01-15)
- Vitamin B12 deficiency (dietary)

Her past medical history is significant for iron deficiency anemia.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Mesalamine delayed-release 800 mg PO TID
- Ferrous sulfate 325 mg (65 mg elemental iron) PO daily
- Cyanocobalamin 1000 mcg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Crohn disease
- Mother: Rheumatoid arthritis
- Paternal aunt: Hypothyroidism

SOCIAL HISTORY:
Smoking: Former smoker (quit 6 months ago; previously 1 pack per day for 10 years). Alcohol: Social (1-2 drinks per month). Occupation: Elementary school art teacher.

PHYSICAL EXAMINATION:
On physical examination, there are multiple tender, erythematous nodules measuring 2-3 cm on her anterior shins. Her abdominal examination reveals tenderness in the right lower quadrant without rebound or guarding.

LABORATORY DATA:
Laboratory results are as follows:

- Hemoglobin: 10.2 g/dL (Normal: 12-16 g/dL)
- Mean corpuscular volume (MCV): 72 fL (Normal: 80-100 fL)
- C-reactive protein (CRP): 24 mg/L (Normal: <5 mg/L)
- Erythrocyte sedimentation rate (ESR): 45 mm/hr (Normal: 0-20 mm/hr)
- Fecal calprotectin: 250 µg/g (Normal: <50 µg/g)

Stool cultures and Clostridium difficile toxin assay are negative.

## Encounter 5: source 2023-01-15, practice 2024-05-17, outpatient, Dermatology

Chief complaint: Painful, deep nodules on lower legs for 6 weeks accompanied by low-grade fevers, joint aches, and occasional fatigue

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Maria Garcia, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Painful, deep nodules on lower legs for 6 weeks accompanied by low-grade fevers, joint aches, and occasional fatigue

HISTORY OF PRESENT ILLNESS:
The patient is a 30-year-old female with Crohn disease and prior erythema nodosum episodes in 2022 who returns with painful deep nodules on her lower legs accompanied by low-grade fevers, joint aches, and occasional fatigue. She states the nodules appeared suddenly and are tender to the touch, sometimes making it uncomfortable to wear jeans or walk long distances. She denies recent insect bites, medication changes, or new skincare products, but reports a sore throat with cough approximately one month ago that resolved without treatment.

PAST MEDICAL HISTORY:
Active Problem List:
- Ulcerative colitis (diagnosed 2020-01-15)
- Iron Deficiency Anemia (diagnosed 2020-07-15)
- Crohn Disease (diagnosed 2021-07-15)
- Erythema nodosum (isolated) (diagnosed 2022-01-15)
- Vitamin B12 deficiency (dietary)

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Mesalamine delayed-release 800 mg PO TID
- Ferrous sulfate 325 mg (65 mg elemental iron) PO daily
- Cyanocobalamin 1000 mcg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Crohn disease
- Mother: Rheumatoid arthritis
- Paternal aunt: Hypothyroidism

SOCIAL HISTORY:
Smoking: Former smoker (quit 6 months ago; previously 1 pack per day for 10 years). Alcohol: Social (1-2 drinks per month). Occupation: Elementary school art teacher.

VITAL SIGNS:
Her BMI is 29

PHYSICAL EXAMINATION:
Physical examination reveals multiple poorly demarcated, warm, erythematous plaques and subcutaneous nodules along the anterior shins; some are beginning to fade with a yellowish hue. No mucosal involvement or active joint swelling is noted.

LABORATORY DATA:
A rapid streptococcal antigen test is now negative.

## Encounter 6: source 2024-01-15, practice 2025-05-22, inpatient, Emergency Medicine

Chief complaint: Worsening abdominal pain, nausea, and vomiting for 2 days with greenish vomitus and obstipation

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Kwame Asante, MD
Department: Emergency Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Worsening abdominal pain, nausea, and vomiting for 2 days with greenish vomitus and obstipation

HISTORY OF PRESENT ILLNESS:
The patient is a 31-year-old female with Crohn disease initially classified as ulcerative colitis at age 22 who presents with a 2-day history of worsening abdominal pain, nausea, and vomiting. She notes greenish vomitus and has not passed flatus or had a bowel movement since the symptoms started.

PAST MEDICAL HISTORY:
Active Problem List:
- Ulcerative colitis (diagnosed 2020-01-15)
- Iron Deficiency Anemia (diagnosed 2020-07-15)
- Crohn Disease (diagnosed 2021-07-15)
- Erythema nodosum (isolated) (diagnosed 2022-01-15)
- Vitamin B12 deficiency (dietary)

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Mesalamine delayed-release 800 mg PO TID
- Ferrous sulfate 325 mg (65 mg elemental iron) PO daily
- Cyanocobalamin 1000 mcg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Crohn disease
- Mother: Rheumatoid arthritis
- Paternal aunt: Hypothyroidism

SOCIAL HISTORY:
She mentions that she has been smoking a pack of cigarettes per day for the past 10 years

VITAL SIGNS:
She has a low-grade fever of 37.8°C (100°F)

PHYSICAL EXAMINATION:
On examination, her abdomen is distended and tender to palpation, with hyperactive bowel sounds

LABORATORY DATA:
Laboratory tests reveal a white blood cell count of 12,000/uL, hemoglobin of 13.2 g/dL, and a C-reactive protein of 25 mg/L

IMAGING:
An abdominal X-ray shows dilated loops of small bowel with air-fluid levels

## Encounter 7: source 2024-07-15, practice 2025-11-17, follow_up, Gastroenterology

Chief complaint: Follow-up visit to assess disease control after previous flares and hospitalization

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: follow_up

CHIEF COMPLAINT:
Follow-up visit to assess disease control after previous flares and hospitalization

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of her Crohn disease following recent hospitalization. She previously experienced frequent flares requiring corticosteroids and biologic therapy but reports significant clinical improvement over the past year, with normalization of bowel movements and no abdominal pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Ulcerative colitis (diagnosed 2020-01-15)
- Iron Deficiency Anemia (diagnosed 2020-07-15)
- Crohn Disease (diagnosed 2021-07-15)
- Erythema nodosum (isolated) (diagnosed 2022-01-15)
- Small bowel obstruction due to fibrotic stricture secondary to Crohn's disease (diagnosed 2024-01-15)
- Vitamin B12 deficiency (dietary)

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Mesalamine delayed-release 800 mg PO TID
- Ferrous sulfate 325 mg (65 mg elemental iron) PO daily
- Cyanocobalamin 1000 mcg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Crohn disease
- Mother: Rheumatoid arthritis
- Paternal aunt: Hypothyroidism

SOCIAL HISTORY:
Smoking: Former smoker (quit 6 months ago; previously 1 pack per day for 10 years). Alcohol: Social (1-2 drinks per month). Occupation: Elementary school art teacher.

PATHOLOGY:
Biopsy reveals decreased lamina propria lymphocyte infiltration and increased expression of a cytokine secreted by regulatory T cells that inhibits the synthesis of IFN-γ and TNF-α by effector T cells.

OTHER STUDIES:
Colonoscopy now shows reduced mucosal inflammation compared to prior studies.

ASSESSMENT AND PLAN:
This cytokine is thought to contribute to sustained mucosal healing and long-term remission in inflammatory bowel disease.
