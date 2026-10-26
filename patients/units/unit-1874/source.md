# synthetic_hospital patient 1874

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 29 at first encounter, sex F, Non-Hispanic White, Private employer-based PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-06-10 (Mon) | outpatient | Gastroenterology |
| 1 | 2020-09-15 | 2025-02-10 (Mon) | outpatient | Gastroenterology |
| 2 | 2022-03-15 | 2026-08-05 (Wed) | outpatient | General Surgery |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private employer-based PPO",
 "occupation": "Elementary school teacher",
 "alcohol_use": "Former social use (1-2 glasses of wine monthly), quit after Crohn disease diagnosis",
 "family_history": [
  "Mother: breast cancer (age 45)",
  "Maternal aunt: ovarian cancer (age 52)",
  "Maternal grandmother: breast cancer (age 68)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "40 mg subcutaneous every 2 weeks",
   "name": "Adalimumab"
  },
  {
   "dose": "2000 units daily",
   "name": "Vitamin D3"
  }
 ],
 "surgical_history": [
  "Left nephrectomy (age 6 for stage II Wilms tumor)",
  "Abdominopelvic radiation therapy (age 6)"
 ],
 "chronic_conditions": [
  "Crohn disease",
  "Hereditary breast and ovarian cancer syndrome (BRCA2 mutation carrier)",
  "History of Wilms tumor (NED since age 6)"
 ],
 "age_at_first_encounter": 29
}
```

Primary diagnoses: ["Crohn Disease", "History of childhood Wilms tumor with abdominopelvic radiation therapy (high risk for secondary colorectal cancer)", "Hereditary breast and ovarian cancer syndrome"]
Comorbidities: ["Family history of breast cancer"]

## Encounter 0: source 2020-01-15, practice 2024-06-10, outpatient, Gastroenterology

Chief complaint: Routine health maintenance visit to discuss additional cancer screening recommendations given her complex oncologic history.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Sharma, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine health maintenance visit to discuss additional cancer screening recommendations given her complex oncologic history.

HISTORY OF PRESENT ILLNESS:
She has no current complaints and is up to date with her vaccinations. She inquires about any additional health screenings she should consider given her medical history.

PAST MEDICAL HISTORY:
Active Problem List:
- Crohn disease
- Hereditary breast and ovarian cancer syndrome (BRCA2 mutation carrier)
- History of Wilms tumor (NED since age 6)

She has a history of stage II Wilms tumor treated with nephrectomy and abdominopelvic radiation at the age of 6.

PAST SURGICAL HISTORY:
- Left nephrectomy (age 6 for stage II Wilms tumor)
- Abdominopelvic radiation therapy (age 6)

MEDICATIONS:
Home Medications:
- Adalimumab 40 mg subcutaneous every 2 weeks
- Vitamin D3 2000 units daily

ALLERGIES:
NKDA

FAMILY HISTORY:
Her family history is significant for breast cancer in her mother at the age of 45.

SOCIAL HISTORY:
She has no history of smoking or alcohol use. Her diet is high in fruits and vegetables, and she exercises regularly.

VITAL SIGNS:
Her vital signs are within normal limits

PHYSICAL EXAMINATION:
Abdominal examination is unremarkable

## Encounter 1: source 2020-09-15, practice 2025-02-10, outpatient, Gastroenterology

Chief complaint: Low-grade fever, fatigue, and intermittent diarrhea persisting for 3 months with 10-lb unintentional weight loss and one episode of hematochezia.

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Priya Sharma, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Low-grade fever, fatigue, and intermittent diarrhea persisting for 3 months with 10-lb unintentional weight loss and one episode of hematochezia.

HISTORY OF PRESENT ILLNESS:
The patient with Crohn disease and prior Wilms tumor returns. She has experienced a 10-lb (4.5-kg) unintentional weight loss and had one episode of hematochezia, denying recent travel or antibiotic use.

PAST MEDICAL HISTORY:
Active Problem List:
- History of childhood Wilms tumor with abdominopelvic radiation therapy (high risk for secondary colorectal cancer) (diagnosed 2020-01-15)
- Crohn disease
- Hereditary breast and ovarian cancer syndrome (BRCA2 mutation carrier)
- History of Wilms tumor (NED since age 6)

PAST SURGICAL HISTORY:
- Left nephrectomy (age 6 for stage II Wilms tumor)
- Abdominopelvic radiation therapy (age 6)

MEDICATIONS:
She does not take any medications

Home Medications:
- Adalimumab 40 mg subcutaneous every 2 weeks
- Vitamin D3 2000 units daily

ALLERGIES:
NKDA

FAMILY HISTORY:
She has a family history of hypertension and irritable bowel syndrome in her older sister.

SOCIAL HISTORY:
She consumes one to two glasses of wine monthly and denies smoking.

VITAL SIGNS:
Her temperature is 37.3°C (99.2°F), pulse is 88/min, blood pressure is 102/60 mm Hg, and respirations are 20/min.

PHYSICAL EXAMINATION:
On physical examination, there is diffuse abdominal tenderness without rebound, increased bowel sounds, and no perianal abnormalities. A digital rectal examination reveals normal tone and positive occult blood.

LABORATORY DATA:
Stool cultures are negative for pathogens.

OTHER STUDIES:
Colonoscopy reveals patchy areas of inflammation with skip lesions.

## Encounter 2: source 2022-03-15, practice 2026-08-05, outpatient, General Surgery

Chief complaint: Evaluation of a firm, non-tender mass detected in the left breast during self-examination.

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. James Chen, MD
Department: General Surgery
Visit Type: outpatient

CHIEF COMPLAINT:
Evaluation of a firm, non-tender mass detected in the left breast during self-examination.

PAST MEDICAL HISTORY:
Active Problem List:
- History of childhood Wilms tumor with abdominopelvic radiation therapy (high risk for secondary colorectal cancer) (diagnosed 2020-01-15)
- Crohn Disease (diagnosed 2020-09-15)
- Hereditary breast and ovarian cancer syndrome (BRCA2 mutation carrier)
- History of Wilms tumor (NED since age 6)

PAST SURGICAL HISTORY:
- Left nephrectomy (age 6 for stage II Wilms tumor)
- Abdominopelvic radiation therapy (age 6)

MEDICATIONS:
Home Medications:
- Adalimumab 40 mg subcutaneous every 2 weeks
- Vitamin D3 2000 units daily

ALLERGIES:
NKDA

FAMILY HISTORY:
Her family history reveals that her older sister was diagnosed with bilateral breast cancer at age 35, and her maternal grandmother died from ovarian cancer in her late 40s.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Former social use (1-2 glasses of wine monthly), quit after Crohn disease diagnosis. Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
Physical examination confirms a 2.4-cm mass in the upper outer quadrant of the breast without overlying skin changes or nipple retraction.

IMAGING:
Mammography reveals a suspicious irregular lesion

OTHER STUDIES:
Core needle biopsy is pending.

ASSESSMENT AND PLAN:
The patient expresses concern about her risk of developing additional cancers and asks about genetic testing. Her physician explains that certain inherited mutations lead to increased susceptibility to both breast and ovarian cancer, often involving defective tumor suppressor genes. These genes are critical in DNA repair mechanisms that maintain genomic stability. Mutations in these genes are inherited in an autosomal dominant fashion with incomplete penetrance.
