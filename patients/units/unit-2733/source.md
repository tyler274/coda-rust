# synthetic_hospital patient 2733

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex F, Ashkenazi Jewish, Private PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-10-25 (Mon) | outpatient | Endocrinology |
| 1 | 2020-07-15 | 2022-05-09 (Mon) | outpatient | Endocrinology |
| 2 | 2021-01-15 | 2022-10-25 (Tue) | outpatient | Rheumatology |
| 3 | 2021-07-15 | 2023-04-26 (Wed) | outpatient | Orthopedics & Sports Medicine |
| 4 | 2022-01-15 | 2023-10-25 (Wed) | outpatient | Otolaryngology |
| 5 | 2022-07-15 | 2024-04-23 (Tue) | outpatient | Dermatology |
| 6 | 2023-07-15 | 2025-04-21 (Mon) | outpatient | Pulmonology |
| 7 | 2024-07-15 | 2026-04-27 (Mon) | outpatient | Ophthalmology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Penicillin",
  "Sulfa drugs"
 ],
 "insurance": "Private PPO",
 "occupation": "Administrative assistant",
 "alcohol_use": "Social, 1-2 glasses of wine weekly",
 "family_history": [
  "Mother: Hashimoto thyroiditis",
  "Father: Type 2 diabetes mellitus",
  "Sister: Systemic lupus erythematosus",
  "Maternal aunt: Rheumatoid arthritis"
 ],
 "race_ethnicity": "Ashkenazi Jewish",
 "smoking_status": "Former smoker, 10 pack-years, quit 5 years ago",
 "home_medications": [
  {
   "dose": "125 mcg PO daily",
   "name": "Levothyroxine"
  },
  {
   "dose": "20 mg PO weekly",
   "name": "Methotrexate"
  },
  {
   "dose": "1 mg PO daily",
   "name": "Folic acid"
  },
  {
   "dose": "250/50 mcg inhaled twice daily",
   "name": "Fluticasone propionate/salmeterol inhaler"
  },
  {
   "dose": "90 mcg inhaled every 4-6 hours as needed for wheezing",
   "name": "Albuterol inhaler"
  },
  {
   "dose": "1%, 1 drop in each eye twice daily",
   "name": "Prednisolone acetate ophthalmic suspension"
  },
  {
   "dose": "0.5 mg PO twice weekly",
   "name": "Cabergoline"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Prednisone"
  },
  {
   "dose": "100 mg PO daily",
   "name": "Azathioprine"
  }
 ],
 "surgical_history": [
  "Tonsillectomy and adenoidectomy, age 12",
  "Wisdom teeth extraction, age 22"
 ],
 "chronic_conditions": [
  "Hypothyroidism",
  "Rheumatoid arthritis",
  "Moderate persistent asthma",
  "Chronic anterior uveitis",
  "Prolactinoma",
  "Pemphigus vulgaris",
  "Vestibular schwannoma (right-sided)",
  "Carpal tunnel syndrome",
  "Rheumatoid arthritis-associated interstitial lung disease"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Rheumatoid arthritis", "Steroid-induced open-angle glaucoma", "Prolactinoma", "Hypothyroidism", "Pemphigus Vulgaris", "Acoustic neuroma (Vestibular schwannoma)", "Rheumatoid arthritis-associated interstitial lung disease", "Carpal tunnel syndrome"]
Comorbidities: ["Rheumatoid arthritis", "Hypothyroidism"]

## Encounter 0: source 2020-01-15, practice 2021-10-25, outpatient, Endocrinology

Chief complaint: Bilateral hand numbness and tingling for 6 months, worse at night, with associated fatigue and weight gain.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Bilateral hand numbness and tingling for 6 months, worse at night, with associated fatigue and weight gain.

HISTORY OF PRESENT ILLNESS:
A 42-year-old female presents to the clinic with a 6-month history of bilateral hand numbness and tingling, particularly in the thumb, index, and middle fingers. She reports that her symptoms are worse at night and often awaken her from sleep. She also mentions generalized fatigue, weight gain, and cold intolerance.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypothyroidism
- Rheumatoid arthritis
- Moderate persistent asthma
- Chronic anterior uveitis
- Prolactinoma
- Pemphigus vulgaris
- Vestibular schwannoma (right-sided)
- Carpal tunnel syndrome
- Rheumatoid arthritis-associated interstitial lung disease

Her past medical history is significant for hypothyroidism.

PAST SURGICAL HISTORY:
- Tonsillectomy and adenoidectomy, age 12
- Wisdom teeth extraction, age 22

MEDICATIONS:
She takes for which she takes levothyroxine. However, she admits to being non-compliant with her medication over the past year.

Home Medications:
- Methotrexate 20 mg PO weekly
- Folic acid 1 mg PO daily
- Fluticasone propionate/salmeterol inhaler 250/50 mcg inhaled twice daily
- Albuterol inhaler 90 mcg inhaled every 4-6 hours as needed for wheezing
- Prednisolone acetate ophthalmic suspension 1%, 1 drop in each eye twice daily
- Cabergoline 0.5 mg PO twice weekly
- Prednisone 10 mg PO daily
- Azathioprine 100 mg PO daily

ALLERGIES:
Penicillin, Sulfa drugs

FAMILY HISTORY:
- Mother: Hashimoto thyroiditis
- Father: Type 2 diabetes mellitus
- Sister: Systemic lupus erythematosus
- Maternal aunt: Rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Former smoker, 10 pack-years, quit 5 years ago. Alcohol: Social, 1-2 glasses of wine weekly. Occupation: Administrative assistant.

PHYSICAL EXAMINATION:
On physical examination, you note dry, coarse skin, and delayed relaxation of deep tendon reflexes. A Tinel&#x27;s test elicits tingling in her fingers, and a Phalen&#x27;s test reproduces her symptoms.

LABORATORY DATA:
Her thyroid-stimulating hormone (TSH) level is 12 mIU/L (normal range: 0.4-4.0 mIU/L), and her free thyroxine (T4) level is 0.5 ng/dL (normal range: 0.9-1.7 ng/dL).

Laboratory Values:
- TSH: 12 mIU/L (normal range: 0.4-4.0 mIU/L)
- Free T4: 0.5 ng/dL (normal range: 0.9-1.7 ng/dL)

## Encounter 1: source 2020-07-15, practice 2022-05-09, outpatient, Endocrinology

Chief complaint: Progressively worsening headaches and amenorrhea with bilateral milky nipple discharge over the past 6 months.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressively worsening headaches and amenorrhea with bilateral milky nipple discharge over the past 6 months.

HISTORY OF PRESENT ILLNESS:
The patient returns to the Endocrinology clinic six months following her initial evaluation for hypothyroidism. Her symptoms began approximately 6 months ago, when she also noticed milky discharge from both breasts. She denies visual disturbances, but occasionally has trouble reading fine print. She has no history of pregnancy, and her last menstrual period was over 8 months ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypothyroidism (diagnosed 2020-01-15)
- Rheumatoid arthritis
- Moderate persistent asthma
- Chronic anterior uveitis
- Prolactinoma
- Pemphigus vulgaris
- Vestibular schwannoma (right-sided)
- Carpal tunnel syndrome
- Rheumatoid arthritis-associated interstitial lung disease

Her medical history includes well-controlled hypothyroidism.

PAST SURGICAL HISTORY:
- Tonsillectomy and adenoidectomy, age 12
- Wisdom teeth extraction, age 22

MEDICATIONS:
She takes managed with levothyroxine.

Home Medications:
- Methotrexate 20 mg PO weekly
- Folic acid 1 mg PO daily
- Fluticasone propionate/salmeterol inhaler 250/50 mcg inhaled twice daily
- Albuterol inhaler 90 mcg inhaled every 4-6 hours as needed for wheezing
- Prednisolone acetate ophthalmic suspension 1%, 1 drop in each eye twice daily
- Cabergoline 0.5 mg PO twice weekly
- Prednisone 10 mg PO daily
- Azathioprine 100 mg PO daily

ALLERGIES:
Penicillin, Sulfa drugs

FAMILY HISTORY:
- Mother: Hashimoto thyroiditis
- Father: Type 2 diabetes mellitus
- Sister: Systemic lupus erythematosus
- Maternal aunt: Rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Former smoker, 10 pack-years, quit 5 years ago. Alcohol: Social, 1-2 glasses of wine weekly. Occupation: Administrative assistant.

PHYSICAL EXAMINATION:
Physical examination reveals bitemporal visual field deficits on confrontation testing.

LABORATORY DATA:
Serum prolactin is elevated at 210 ng/mL. TSH and free T4 are within normal limits.

IMAGING:
MRI of the brain reveals a 1.7 cm mass in the sellar region impinging on the optic chiasm.

## Encounter 2: source 2021-01-15, practice 2022-10-25, outpatient, Rheumatology

Chief complaint: Joint pain and morning stiffness lasting over one hour, initially in fingers and progressing to wrists and knees over 6 months.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Elena Rodriguez, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Joint pain and morning stiffness lasting over one hour, initially in fingers and progressing to wrists and knees over 6 months.

HISTORY OF PRESENT ILLNESS:
The patient presents to Rheumatology with joint symptoms in the context of established hypothyroidism and prolactinoma. She reports a 6-month history of joint pain and stiffness, particularly in the mornings lasting more than one hour. She reports that the symptoms initially started in her fingers and have now progressed to involve her wrists and knees.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypothyroidism (diagnosed 2020-01-15)
- Prolactinoma (diagnosed 2020-07-15)
- Rheumatoid arthritis
- Moderate persistent asthma
- Chronic anterior uveitis
- Pemphigus vulgaris
- Vestibular schwannoma (right-sided)
- Carpal tunnel syndrome
- Rheumatoid arthritis-associated interstitial lung disease

PAST SURGICAL HISTORY:
- Tonsillectomy and adenoidectomy, age 12
- Wisdom teeth extraction, age 22

MEDICATIONS:
She takes levothyroxine.

Home Medications:
- Methotrexate 20 mg PO weekly
- Folic acid 1 mg PO daily
- Fluticasone propionate/salmeterol inhaler 250/50 mcg inhaled twice daily
- Albuterol inhaler 90 mcg inhaled every 4-6 hours as needed for wheezing
- Prednisolone acetate ophthalmic suspension 1%, 1 drop in each eye twice daily
- Cabergoline 0.5 mg PO twice weekly
- Prednisone 10 mg PO daily
- Azathioprine 100 mg PO daily

ALLERGIES:
Penicillin, Sulfa drugs

FAMILY HISTORY:
- Mother: Hashimoto thyroiditis
- Father: Type 2 diabetes mellitus
- Sister: Systemic lupus erythematosus
- Maternal aunt: Rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Former smoker, 10 pack-years, quit 5 years ago. Alcohol: Social, 1-2 glasses of wine weekly. Occupation: Administrative assistant.

REVIEW OF SYSTEMS:
She denies any recent infections, diarrhea, or genitourinary symptoms.

PHYSICAL EXAMINATION:
On physical examination, there is bilateral swelling and tenderness of the proximal interphalangeal and metacarpophalangeal joints. There is also mild synovitis in both knees.

LABORATORY DATA:
- Erythrocyte sedimentation rate (ESR): 50 mm/hr (Normal: 0-20 mm/hr)
- C-reactive protein (CRP): 25 mg/L (Normal: <10 mg/L)
- Rheumatoid factor (RF): Negative
- Anti-cyclic citrullinated peptide (anti-CCP) antibodies: Negative
- Antinuclear antibodies (ANA): Negative

## Encounter 3: source 2021-07-15, practice 2023-04-26, outpatient, Orthopedics & Sports Medicine

Chief complaint: Gradually worsening finger stiffness, difficulty opening jars, and persistent dull ache in both wrists for 3 months.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. James O'Brien, MD
Department: Orthopedics & Sports Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Gradually worsening finger stiffness, difficulty opening jars, and persistent dull ache in both wrists for 3 months.

HISTORY OF PRESENT ILLNESS:
The patient with rheumatoid arthritis and prior evaluations for hand numbness and joint pain returns for follow-up. She reports the symptoms are worst in the morning and often make it hard to get dressed, though they improve somewhat over the course of the day. She also notes that her wedding ring feels tighter than usual and she occasionally feels tingling in her palms.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypothyroidism (diagnosed 2020-01-15)
- Prolactinoma (diagnosed 2020-07-15)
- Rheumatoid arthritis (diagnosed 2021-01-15)
- Moderate persistent asthma
- Chronic anterior uveitis
- Pemphigus vulgaris
- Vestibular schwannoma (right-sided)
- Carpal tunnel syndrome
- Rheumatoid arthritis-associated interstitial lung disease

PAST SURGICAL HISTORY:
- Tonsillectomy and adenoidectomy, age 12
- Wisdom teeth extraction, age 22

MEDICATIONS:
Home Medications:
- Levothyroxine 125 mcg PO daily
- Methotrexate 20 mg PO weekly
- Folic acid 1 mg PO daily
- Fluticasone propionate/salmeterol inhaler 250/50 mcg inhaled twice daily
- Albuterol inhaler 90 mcg inhaled every 4-6 hours as needed for wheezing
- Prednisolone acetate ophthalmic suspension 1%, 1 drop in each eye twice daily
- Cabergoline 0.5 mg PO twice weekly
- Prednisone 10 mg PO daily
- Azathioprine 100 mg PO daily

ALLERGIES:
Penicillin, Sulfa drugs

FAMILY HISTORY:
- Mother: Hashimoto thyroiditis
- Father: Type 2 diabetes mellitus
- Sister: Systemic lupus erythematosus
- Maternal aunt: Rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Former smoker, 10 pack-years, quit 5 years ago. Alcohol: Social, 1-2 glasses of wine weekly. Occupation: Administrative assistant.

PHYSICAL EXAMINATION:
Physical examination reveals swelling and tenderness at the second and third metacarpophalangeal joints bilaterally, with restricted wrist flexion due to discomfort. Tinel and Phalen maneuvers reproduce her paresthesias.

LABORATORY DATA:
Laboratory studies show a normocytic anemia and elevated C-reactive protein.

## Encounter 4: source 2022-01-15, practice 2023-10-25, outpatient, Otolaryngology

Chief complaint: Progressive right-sided hearing loss and episodes of vertigo over 6 months with occasional facial numbness.

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Aisha Washington, MD
Department: Otolaryngology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive right-sided hearing loss and episodes of vertigo over 6 months with occasional facial numbness.

HISTORY OF PRESENT ILLNESS:
This 47-year-old female with hypothyroidism, prolactinoma, rheumatoid arthritis, and carpal tunnel syndrome presents with a 6-month history of progressive right-sided hearing loss and episodes of vertigo. She also reports occasional right-sided facial numbness. She denies otalgia, otorrhea, or ototoxic medication use.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypothyroidism (diagnosed 2020-01-15)
- Prolactinoma (diagnosed 2020-07-15)
- Rheumatoid arthritis (diagnosed 2021-01-15)
- Carpal tunnel syndrome (diagnosed 2021-07-15)
- Moderate persistent asthma
- Chronic anterior uveitis
- Pemphigus vulgaris
- Vestibular schwannoma (right-sided)
- Rheumatoid arthritis-associated interstitial lung disease

PAST SURGICAL HISTORY:
- Tonsillectomy and adenoidectomy, age 12
- Wisdom teeth extraction, age 22

MEDICATIONS:
She takes levothyroxine.

Home Medications:
- Methotrexate 20 mg PO weekly
- Folic acid 1 mg PO daily
- Fluticasone propionate/salmeterol inhaler 250/50 mcg inhaled twice daily
- Albuterol inhaler 90 mcg inhaled every 4-6 hours as needed for wheezing
- Prednisolone acetate ophthalmic suspension 1%, 1 drop in each eye twice daily
- Cabergoline 0.5 mg PO twice weekly
- Prednisone 10 mg PO daily
- Azathioprine 100 mg PO daily

ALLERGIES:
Penicillin, Sulfa drugs

FAMILY HISTORY:
- Mother: Hashimoto thyroiditis
- Father: Type 2 diabetes mellitus
- Sister: Systemic lupus erythematosus
- Maternal aunt: Rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Former smoker, 10 pack-years, quit 5 years ago. Alcohol: Social, 1-2 glasses of wine weekly. Occupation: Administrative assistant.

PHYSICAL EXAMINATION:
The Weber test lateralizes to the left ear, and the Rinne test indicates air conduction is greater than bone conduction bilaterally. Cranial nerve examination reveals decreased corneal reflex and facial sensation on the right side.

OTHER STUDIES:
Audiometry shows sensorineural hearing loss in the right ear.

PLAN:
Magnetic resonance imaging (MRI) of the brain with gadolinium contrast is ordered.

## Encounter 5: source 2022-07-15, practice 2024-04-23, outpatient, Dermatology

Chief complaint: Painful oral ulcers for 2 months and recent onset of flaccid skin blisters on chest and back that rupture easily.

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. David Kim, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Painful oral ulcers for 2 months and recent onset of flaccid skin blisters on chest and back that rupture easily.

HISTORY OF PRESENT ILLNESS:
The patient presents for dermatology follow-up in the context of rheumatoid arthritis and endocrine disorders. She reports that the oral ulcers began as small vesicles that rapidly progressed to large erosions, making it difficult to eat. Over the past week, she has noticed similar blisters forming on her chest and back, which easily rupture with minor friction.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypothyroidism (diagnosed 2020-01-15)
- Prolactinoma (diagnosed 2020-07-15)
- Rheumatoid arthritis (diagnosed 2021-01-15)
- Carpal tunnel syndrome (diagnosed 2021-07-15)
- Acoustic neuroma (Vestibular schwannoma) (diagnosed 2022-01-15)
- Moderate persistent asthma
- Chronic anterior uveitis
- Pemphigus vulgaris
- Vestibular schwannoma (right-sided)
- Rheumatoid arthritis-associated interstitial lung disease

Her medical history is significant for hypothyroidism,.

PAST SURGICAL HISTORY:
- Tonsillectomy and adenoidectomy, age 12
- Wisdom teeth extraction, age 22

MEDICATIONS:
She takes for which she takes levothyroxine.

Home Medications:
- Methotrexate 20 mg PO weekly
- Folic acid 1 mg PO daily
- Fluticasone propionate/salmeterol inhaler 250/50 mcg inhaled twice daily
- Albuterol inhaler 90 mcg inhaled every 4-6 hours as needed for wheezing
- Prednisolone acetate ophthalmic suspension 1%, 1 drop in each eye twice daily
- Cabergoline 0.5 mg PO twice weekly
- Prednisone 10 mg PO daily
- Azathioprine 100 mg PO daily

ALLERGIES:
Penicillin, Sulfa drugs

FAMILY HISTORY:
- Mother: Hashimoto thyroiditis
- Father: Type 2 diabetes mellitus
- Sister: Systemic lupus erythematosus
- Maternal aunt: Rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Former smoker, 10 pack-years, quit 5 years ago. Alcohol: Social, 1-2 glasses of wine weekly. Occupation: Administrative assistant.

PHYSICAL EXAMINATION:
On physical examination, flaccid bullae are noted on her trunk, and gentle pressure on the edge of an intact blister causes it to extend into the adjacent normal-appearing skin (positive Nikolsky sign). Several erosions with crusting are also present on her buccal mucosa. No lymphadenopathy is noted.

LABORATORY DATA:
Laboratory Values:
- Complete Blood Count (CBC): Within normal limits
- Thyroid-Stimulating Hormone (TSH): Within normal limits
- Serum Glucose: Within normal limits
- Anti-desmoglein antibodies: Pending

PATHOLOGY:
A skin biopsy is performed, and histopathology reveals intraepidermal cleavage with acantholysis. Direct immunofluorescence of the biopsy specimen shows intercellular deposits of IgG and C3 in the epidermis.

## Encounter 6: source 2023-07-15, practice 2025-04-21, outpatient, Pulmonology

Chief complaint: Progressive shortness of breath with dry cough and increasing fatigue over several months.

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Priya Nasser, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive shortness of breath with dry cough and increasing fatigue over several months.

HISTORY OF PRESENT ILLNESS:
The patient returns for outpatient pulmonology care with a history of rheumatoid arthritis, hypothyroidism, and pemphigus vulgaris. She reports increasing fatigue and dry cough over the past few months. She has been compliant with her methotrexate therapy and has not experienced recent joint flares. She denies chest pain, hemoptysis, or wheezing.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypothyroidism (diagnosed 2020-01-15)
- Prolactinoma (diagnosed 2020-07-15)
- Rheumatoid arthritis (diagnosed 2021-01-15)
- Carpal tunnel syndrome (diagnosed 2021-07-15)
- Acoustic neuroma (Vestibular schwannoma) (diagnosed 2022-01-15)
- Pemphigus Vulgaris (diagnosed 2022-07-15)
- Moderate persistent asthma
- Chronic anterior uveitis
- Vestibular schwannoma (right-sided)
- Rheumatoid arthritis-associated interstitial lung disease

PAST SURGICAL HISTORY:
- Tonsillectomy and adenoidectomy, age 12
- Wisdom teeth extraction, age 22

MEDICATIONS:
She takes methotrexate.

Home Medications:
- Levothyroxine 125 mcg PO daily
- Folic acid 1 mg PO daily
- Fluticasone propionate/salmeterol inhaler 250/50 mcg inhaled twice daily
- Albuterol inhaler 90 mcg inhaled every 4-6 hours as needed for wheezing
- Prednisolone acetate ophthalmic suspension 1%, 1 drop in each eye twice daily
- Cabergoline 0.5 mg PO twice weekly
- Prednisone 10 mg PO daily
- Azathioprine 100 mg PO daily

ALLERGIES:
Penicillin, Sulfa drugs

FAMILY HISTORY:
- Mother: Hashimoto thyroiditis
- Father: Type 2 diabetes mellitus
- Sister: Systemic lupus erythematosus
- Maternal aunt: Rheumatoid arthritis

SOCIAL HISTORY:
10-pack-year smoking history but quit 5 years ago

VITAL SIGNS:
Oxygen saturation is 93% on room air

PHYSICAL EXAMINATION:
Lung exam reveals fine bibasilar crackles

IMAGING:
Chest X-ray reveals reticular opacities at the lung bases

OTHER STUDIES:
Pulmonary function testing demonstrates a restrictive pattern with decreased DLCO

## Encounter 7: source 2024-07-15, practice 2026-04-27, outpatient, Ophthalmology

Chief complaint: Gradually worsening peripheral vision loss in both eyes over 3 months without pain or redness.

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Robert Liu, MD
Department: Ophthalmology
Visit Type: outpatient

CHIEF COMPLAINT:
Gradually worsening peripheral vision loss in both eyes over 3 months without pain or redness.

HISTORY OF PRESENT ILLNESS:
The patient is a 49-year-old female with chronic anterior uveitis maintained on long-term topical corticosteroid therapy who returns with a three-month history of gradually worsening bilateral peripheral vision loss. She reports no eye pain or redness.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypothyroidism (diagnosed 2020-01-15)
- Prolactinoma (diagnosed 2020-07-15)
- Rheumatoid arthritis (diagnosed 2021-01-15)
- Carpal tunnel syndrome (diagnosed 2021-07-15)
- Acoustic neuroma (Vestibular schwannoma) (diagnosed 2022-01-15)
- Pemphigus Vulgaris (diagnosed 2022-07-15)
- Rheumatoid arthritis-associated interstitial lung disease (diagnosed 2023-07-15)
- Moderate persistent asthma
- Chronic anterior uveitis
- Vestibular schwannoma (right-sided)

Her medical history is significant for moderate persistent asthma. She also has rheumatoid arthritis and chronic uveitis.

PAST SURGICAL HISTORY:
- Tonsillectomy and adenoidectomy, age 12
- Wisdom teeth extraction, age 22

MEDICATIONS:
She takes inhaled corticosteroids and occasional oral prednisone during exacerbations. topical glucocorticoid eye drops for the past 6 months.

Home Medications:
- Levothyroxine 125 mcg PO daily
- Methotrexate 20 mg PO weekly
- Folic acid 1 mg PO daily
- Fluticasone propionate/salmeterol inhaler 250/50 mcg inhaled twice daily
- Albuterol inhaler 90 mcg inhaled every 4-6 hours as needed for wheezing
- Prednisolone acetate ophthalmic suspension 1%, 1 drop in each eye twice daily
- Cabergoline 0.5 mg PO twice weekly
- Azathioprine 100 mg PO daily

ALLERGIES:
Penicillin, Sulfa drugs

FAMILY HISTORY:
- Mother: Hashimoto thyroiditis
- Father: Type 2 diabetes mellitus
- Sister: Systemic lupus erythematosus
- Maternal aunt: Rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Former smoker, 10 pack-years, quit 5 years ago. Alcohol: Social, 1-2 glasses of wine weekly. Occupation: Administrative assistant.

PHYSICAL EXAMINATION:
On examination, visual acuity is 20/25 in both eyes, and there is no evidence of cataract or uveitis. Tonometry reveals an intraocular pressure of 28 mmHg in the right eye and 27 mmHg in the left eye. Fundoscopic examination shows increased cup-to-disc ratio bilaterally.

OTHER STUDIES:
Visual field testing demonstrates bilateral peripheral field loss.
