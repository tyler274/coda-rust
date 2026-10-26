# synthetic_hospital patient 2133

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 52 at first encounter, sex M, White, non-Hispanic, Private commercial insurance (Blue Cross Blue Shield).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-12-06 (Mon) | outpatient | Dermatology |
| 1 | 2021-11-15 | 2023-10-01 (Sun) | ed | Emergency Medicine |
| 2 | 2023-11-15 | 2025-10-08 (Wed) | icu | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Private commercial insurance (Blue Cross Blue Shield)",
 "occupation": "Livestock farmer and crop producer",
 "alcohol_use": "Social, 1-2 drinks per week",
 "family_history": [
  "Father with metastatic melanoma at age 68",
  "Mother with type 2 diabetes mellitus",
  "Older brother with basal cell carcinoma",
  "Paternal uncle with hypertension"
 ],
 "race_ethnicity": "White, non-Hispanic",
 "smoking_status": "Former smoker, quit 12 years ago (8 pack-year history)",
 "home_medications": [
  {
   "dose": "500 mg by mouth every 6 hours as needed for knee pain",
   "name": "Acetaminophen"
  },
  {
   "dose": "10 mg by mouth daily as needed during allergy season",
   "name": "Cetirizine"
  }
 ],
 "surgical_history": [
  "Appendectomy at age 22",
  "Arthroscopic meniscectomy, left knee at age 48"
 ],
 "chronic_conditions": [
  "Prediabetes",
  "Osteoarthritis, bilateral knees",
  "Actinic keratosis",
  "Seasonal allergic rhinitis"
 ],
 "age_at_first_encounter": 52
}
```

Primary diagnoses: ["Nodular melanoma", "Bacillus anthracis (Anthrax)", "Organophosphate poisoning"]
Comorbidities: ["Occupational mechanical strain"]

## Encounter 0: source 2020-01-15, practice 2021-12-06, outpatient, Dermatology

Chief complaint: Evaluation of a slowly changing, occasionally itchy pigmented lesion on the upper back first noticed several months ago.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Elena Vasquez, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Evaluation of a slowly changing, occasionally itchy pigmented lesion on the upper back first noticed several months ago.

HISTORY OF PRESENT ILLNESS:
He first noticed it several months ago but says it has been "slowly changing" in appearance, occasionally feeling itchy after sun exposure.

PAST MEDICAL HISTORY:
Active Problem List:
- Prediabetes
- Osteoarthritis, bilateral knees
- Actinic keratosis
- Seasonal allergic rhinitis

He has no personal history of skin cancer.

PAST SURGICAL HISTORY:
- Appendectomy at age 22
- Arthroscopic meniscectomy, left knee at age 48

MEDICATIONS:
Home Medications:
- Acetaminophen 500 mg by mouth every 6 hours as needed for knee pain
- Cetirizine 10 mg by mouth daily as needed during allergy season

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
Recalls that an uncle was diagnosed with a pigmented skin tumor at a young age.

SOCIAL HISTORY:
He has a history of intermittent outdoor work and multiple sunburns during his teenage years.

REVIEW OF SYSTEMS:
Feels otherwise well.

PHYSICAL EXAMINATION:
On physical examination, he appears healthy, but close inspection of the back reveals the lesion demonstrated in the image. There is no palpable lymphadenopathy, and the remainder of the exam is unremarkable.

PATHOLOGY:
A biopsy of the lesion is performed, showing atypical cells with prominent nucleoli and irregular nests at the dermoepidermal junction.

PLAN:
Immunohistochemical staining is ordered to confirm the diagnosis and assist with treatment planning.

## Encounter 1: source 2021-11-15, practice 2023-10-01, ed, Emergency Medicine

Chief complaint: Fever, shortness of breath, and a painless black lesion on the forearm with significant surrounding edema developing 2 days after handling dead livestock.

ENCOUNTER NOTE
Date: 2021-11-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fever, shortness of breath, and a painless black lesion on the forearm with significant surrounding edema developing 2 days after handling dead livestock.

HISTORY OF PRESENT ILLNESS:
The patient is a 53-year-old male with prediabetes, osteoarthritis, and actinic keratosis who was previously evaluated in Dermatology in January 2020 for a changing pigmented lesion on his upper back. He reports developing the lesion 2 days after handling dead livestock on his property.

PAST MEDICAL HISTORY:
Active Problem List:
- Nodular melanoma (diagnosed 2020-01-15)
- Prediabetes
- Osteoarthritis, bilateral knees
- Actinic keratosis
- Seasonal allergic rhinitis

PAST SURGICAL HISTORY:
- Appendectomy at age 22
- Arthroscopic meniscectomy, left knee at age 48

MEDICATIONS:
Home Medications:
- Acetaminophen 500 mg by mouth every 6 hours as needed for knee pain
- Cetirizine 10 mg by mouth daily as needed during allergy season

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with metastatic melanoma at age 68
- Mother with type 2 diabetes mellitus
- Older brother with basal cell carcinoma
- Paternal uncle with hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 12 years ago (8 pack-year history). Alcohol: Social, 1-2 drinks per week. Occupation: Livestock farmer and crop producer.

PHYSICAL EXAMINATION:
On physical examination, the lesion has a black eschar with nonpurulent, edematous margins.

LABORATORY DATA:
Blood cultures grow large, boxcar-shaped gram-positive rods. The organism is known to evade phagocytosis through a poly-D-glutamic acid capsule, and it produces both edema and lethal toxins.

IMAGING:
Chest x-ray shows a widened mediastinum without pulmonary infiltrates.

## Encounter 2: source 2023-11-15, practice 2025-10-08, icu, Emergency Medicine

Chief complaint: Excessive salivation, lacrimation, frequent bowel movements, blurred vision, and difficulty breathing after using a new pesticide.

ENCOUNTER NOTE
Date: 2023-11-15
Provider: Dr. Amara Okafor, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Excessive salivation, lacrimation, frequent bowel movements, blurred vision, and difficulty breathing after using a new pesticide.

HISTORY OF PRESENT ILLNESS:
The patient is a 55-year-old male with a history of nodular melanoma (diagnosed 2020) and prior Bacillus anthracis infection (2021) following livestock exposure, as well as prediabetes and osteoarthritis, who presents to the emergency department with his wife. His wife reports that he has been experiencing excessive salivation, lacrimation, and frequent bowel movements since this morning. He also complains of blurred vision and difficulty breathing. The patient admits to using a new pesticide in his fields without proper protective equipment the day before. His wife is concerned about his increasing confusion and weakness.

PAST MEDICAL HISTORY:
Active Problem List:
- Nodular melanoma (diagnosed 2020-01-15)
- Bacillus anthracis (Anthrax) (diagnosed 2021-11-15)
- Prediabetes
- Osteoarthritis, bilateral knees
- Actinic keratosis
- Seasonal allergic rhinitis

PAST SURGICAL HISTORY:
- Appendectomy at age 22
- Arthroscopic meniscectomy, left knee at age 48

MEDICATIONS:
Home Medications:
- Acetaminophen 500 mg by mouth every 6 hours as needed for knee pain
- Cetirizine 10 mg by mouth daily as needed during allergy season

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with metastatic melanoma at age 68
- Mother with type 2 diabetes mellitus
- Older brother with basal cell carcinoma
- Paternal uncle with hypertension

SOCIAL HISTORY:
Smoking: Former smoker, quit 12 years ago (8 pack-year history). Alcohol: Social, 1-2 drinks per week. Occupation: Livestock farmer and crop producer.

VITAL SIGNS:
His vitals are as follows: BP 150/90 mmHg, HR 55 bpm, RR 22/min, Temp 37.2°C (99.0°F).

PHYSICAL EXAMINATION:
On examination, the patient is anxious, diaphoretic, and has miosis. The chest examination reveals bilateral wheezes, and the abdominal examination shows increased bowel sounds. Neurological examination reveals muscle fasciculations, particularly in the upper extremities.
