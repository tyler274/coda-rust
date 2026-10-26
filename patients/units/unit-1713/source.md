# synthetic_hospital patient 1713

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 27 at first encounter, sex F, Non-Hispanic White, Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-08-11 (Wed) | outpatient | Dermatology |
| 1 | 2020-03-15 | 2021-10-05 (Tue) | outpatient | Psychiatry |
| 2 | 2020-07-15 | 2022-02-10 (Thu) | outpatient | Psychiatry |
| 3 | 2022-07-15 | 2024-01-31 (Wed) | outpatient | Dermatology |
| 4 | 2023-01-15 | 2024-08-05 (Mon) | outpatient | Psychiatry |
| 5 | 2023-07-15 | 2025-01-27 (Mon) | outpatient | Dermatology |
| 6 | 2024-01-15 | 2025-07-29 (Tue) | ed | Emergency Medicine |
| 7 | 2024-03-15 | 2025-10-01 (Wed) | procedure | Dermatology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield PPO",
 "occupation": "Administrative Assistant (currently on medical leave)",
 "alcohol_use": "None (abstinent due to medical history)",
 "family_history": [
  "Acute intermittent porphyria (father)",
  "Major depressive disorder (mother)",
  "Hidradenitis suppurativa (sister)",
  "Type 2 diabetes mellitus (maternal grandmother)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker (quit 2 years ago; 0.5 pack-year)",
 "home_medications": [
  {
   "dose": "100 mg oral daily",
   "name": "Sertraline"
  },
  {
   "dose": "100 mg oral twice daily",
   "name": "Doxycycline"
  },
  {
   "dose": "400 mg oral every 6 hours as needed for pain",
   "name": "Ibuprofen"
  }
 ],
 "surgical_history": [
  "Multiple dermatologic laser treatments and chemical peels (facial)",
  "Incision and drainage of right axillary abscess",
  "Tonsillectomy (age 7)"
 ],
 "chronic_conditions": [
  "Body dysmorphic disorder",
  "Binge eating disorder",
  "Hidradenitis suppurativa",
  "Acute intermittent porphyria",
  "Major depressive disorder, recurrent",
  "Generalized anxiety disorder",
  "Obesity (BMI 33)"
 ],
 "age_at_first_encounter": 27
}
```

Primary diagnoses: ["Binge eating disorder", "Body dysmorphic disorder", "Acute intermittent porphyria", "Hidradenitis suppurativa"]
Comorbidities: ["Situational anxiety", "Major depressive disorder", "History of recent pneumonia", "Obesity", "Prediabetes"]

## Encounter 0: source 2020-01-15, practice 2021-08-11, outpatient, Dermatology

Chief complaint: 6-month history of painful nodules in bilateral axillae and groin with intermittent rupture and purulent drainage

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
6-month history of painful nodules in bilateral axillae and groin with intermittent rupture and purulent drainage

HISTORY OF PRESENT ILLNESS:
She reports that the nodules often rupture, discharging a foul-smelling fluid. She has tried over-the-counter topical antibiotics with no improvement.

PAST MEDICAL HISTORY:
Active Problem List:
- Body dysmorphic disorder
- Binge eating disorder
- Hidradenitis suppurativa
- Acute intermittent porphyria
- Major depressive disorder, recurrent
- Generalized anxiety disorder
- Obesity (BMI 33)

Her medical history is significant for obesity and severe acne during adolescence.

PAST SURGICAL HISTORY:
- Multiple dermatologic laser treatments and chemical peels (facial)
- Incision and drainage of right axillary abscess
- Tonsillectomy (age 7)

MEDICATIONS:
Home Medications:
- Sertraline 100 mg oral daily
- Doxycycline 100 mg oral twice daily
- Ibuprofen 400 mg oral every 6 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Acute intermittent porphyria (father)
- Major depressive disorder (mother)
- Hidradenitis suppurativa (sister)
- Type 2 diabetes mellitus (maternal grandmother)

SOCIAL HISTORY:
Smoking: Former smoker (quit 2 years ago; 0.5 pack-year). Alcohol: None (abstinent due to medical history). Occupation: Administrative Assistant (currently on medical leave).

VITAL SIGNS:
She has no fever, and vital signs are within normal limits.

PHYSICAL EXAMINATION:
On physical examination, you note multiple inflamed nodules in both axillae and the groin, some of which have formed sinus tracts. There is also scarring from previous lesions.

LABORATORY DATA:
Laboratory tests reveal:
- Hemoglobin: 13.2 g/dL
- White blood cell count: 11,000 /µL
- C-reactive protein: 24 mg/L (normal <10 mg/L)
- HbA1c: 5.9%

## Encounter 1: source 2020-03-15, practice 2021-10-05, outpatient, Psychiatry

Chief complaint: Recurrent episodes of excessive food consumption with loss of control over the past 6 months and 15-pound weight gain

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Michael Rodriguez, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Recurrent episodes of excessive food consumption with loss of control over the past 6 months and 15-pound weight gain

HISTORY OF PRESENT ILLNESS:
The patient returns to outpatient psychiatry with ongoing eating concerns, following recent dermatology evaluation for hidradenitis suppurativa and within the context of her established psychiatric history. She reports that during these episodes, which occur at least once a week, she feels a loss of control and cannot stop eating. She often consumes large quantities of sweets and high-calorie foods in a short period. She feels extremely distressed and guilty afterward. Her weight has increased by 15 pounds over the last six months. She denies purging behaviors, excessive exercise, or use of laxatives.

PAST MEDICAL HISTORY:
Active Problem List:
- Hidradenitis suppurativa (diagnosed 2020-01-15)
- Body dysmorphic disorder
- Binge eating disorder
- Acute intermittent porphyria
- Major depressive disorder, recurrent
- Generalized anxiety disorder
- Obesity (BMI 33)

PAST SURGICAL HISTORY:
- Multiple dermatologic laser treatments and chemical peels (facial)
- Incision and drainage of right axillary abscess
- Tonsillectomy (age 7)

MEDICATIONS:
She takes sertraline.

Home Medications:
- Doxycycline 100 mg oral twice daily
- Ibuprofen 400 mg oral every 6 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Acute intermittent porphyria (father)
- Major depressive disorder (mother)
- Hidradenitis suppurativa (sister)
- Type 2 diabetes mellitus (maternal grandmother)

SOCIAL HISTORY:
Smoking: Former smoker (quit 2 years ago; 0.5 pack-year). Alcohol: None (abstinent due to medical history). Occupation: Administrative Assistant (currently on medical leave).

VITAL SIGNS:
Her vital signs are within normal limits

PHYSICAL EXAMINATION:
Her physical examination is unremarkable except for moderate obesity

LABORATORY DATA:
- Fasting glucose: 98 mg/dL
- Thyroid-stimulating hormone (TSH): 2.5 mIU/L
- Complete blood count (CBC): within normal limits
- Lipid profile: within normal limits

## Encounter 2: source 2020-07-15, practice 2022-02-10, outpatient, Psychiatry

Chief complaint: Preoccupation with perceived severe acne resulting in social isolation and medical leave from employment

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Preoccupation with perceived severe acne resulting in social isolation and medical leave from employment

HISTORY OF PRESENT ILLNESS:
The patient is a 27-year-old female with a history of binge eating disorder and hidradenitis suppurativa who returns for ongoing psychiatric care following previous encounters in Psychiatry and Dermatology. She presents with continued preoccupation regarding perceived severe acne. She reports spending several hours a day examining her skin in the mirror, convinced that she has severe acne, although her friends and family reassure her that her skin is clear. She admits to avoiding social situations and has taken leave from her job due to her belief that her coworkers are fixated on her skin imperfections. She becomes tearful when discussing her appearance and expresses frustration that previous dermatologists have not taken her concerns seriously.

PAST MEDICAL HISTORY:
Active Problem List:
- Hidradenitis suppurativa (diagnosed 2020-01-15)
- Binge eating disorder (diagnosed 2020-03-15)
- Body dysmorphic disorder
- Acute intermittent porphyria
- Major depressive disorder, recurrent
- Generalized anxiety disorder
- Obesity (BMI 33)

She has a history of depression and anxiety. She has no other significant medical history.

PAST SURGICAL HISTORY:
- Multiple dermatologic laser treatments and chemical peels (facial)
- Incision and drainage of right axillary abscess
- Tonsillectomy (age 7)

MEDICATIONS:
She takes taking sertraline.

Home Medications:
- Doxycycline 100 mg oral twice daily
- Ibuprofen 400 mg oral every 6 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Acute intermittent porphyria (father)
- Major depressive disorder (mother)
- Hidradenitis suppurativa (sister)
- Type 2 diabetes mellitus (maternal grandmother)

SOCIAL HISTORY:
Smoking: Former smoker (quit 2 years ago; 0.5 pack-year). Alcohol: None (abstinent due to medical history). Occupation: Administrative Assistant (currently on medical leave).

VITAL SIGNS:
Vital signs are within normal limits

PHYSICAL EXAMINATION:
On examination, her skin appears clear with no signs of active acne or scarring. She has slight erythema on her cheeks, which she attributes to her rigorous skin care routine.

## Encounter 3: source 2022-07-15, practice 2024-01-31, outpatient, Dermatology

Chief complaint: Follow-up for recurrent painful nodules in axillae and groin, now with 3-year history and inadequate response to topical therapies

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Sarah Chen, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up for recurrent painful nodules in axillae and groin, now with 3-year history and inadequate response to topical therapies

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of hidradenitis suppurativa, with known history since January 2020 and psychiatric comorbidities. She reports a 3-year history of recurrent, painful nodules in her axillae and groin. These nodules often rupture, discharging a foul-smelling fluid. She has tried multiple courses of oral antibiotics and topical treatments with temporary relief.

PAST MEDICAL HISTORY:
Active Problem List:
- Hidradenitis suppurativa (diagnosed 2020-01-15)
- Binge eating disorder (diagnosed 2020-03-15)
- Body dysmorphic disorder (diagnosed 2020-07-15)
- Acute intermittent porphyria
- Major depressive disorder, recurrent
- Generalized anxiety disorder
- Obesity (BMI 33)

Her medical history is significant for obesity and severe acne during adolescence.

PAST SURGICAL HISTORY:
- Multiple dermatologic laser treatments and chemical peels (facial)
- Incision and drainage of right axillary abscess
- Tonsillectomy (age 7)

MEDICATIONS:
Home Medications:
- Sertraline 100 mg oral daily
- Doxycycline 100 mg oral twice daily
- Ibuprofen 400 mg oral every 6 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Acute intermittent porphyria (father)
- Major depressive disorder (mother)
- Hidradenitis suppurativa (sister)
- Type 2 diabetes mellitus (maternal grandmother)

SOCIAL HISTORY:
Smoking: Former smoker (quit 2 years ago; 0.5 pack-year). Alcohol: None (abstinent due to medical history). Occupation: Administrative Assistant (currently on medical leave).

REVIEW OF SYSTEMS:
No fever or systemic symptoms are present.

PHYSICAL EXAMINATION:
On physical examination, you note multiple, tender, erythematous nodules in both axillae and the groin, some of which have formed interconnected sinus tracts. There are also areas of scarring and rope-like hypertrophic scars in the same regions.

LABORATORY DATA:
Hemoglobin: 13.2 g/dL
White blood cell count: 11,000/uL
C-reactive protein: 24 mg/L (elevated)
HbA1c: 6.1%

## Encounter 4: source 2023-01-15, practice 2024-08-05, outpatient, Psychiatry

Chief complaint: Request for plastic surgery referral following multiple unsuccessful dermatologic treatments for perceived facial asymmetry

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Michael Rodriguez, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Request for plastic surgery referral following multiple unsuccessful dermatologic treatments for perceived facial asymmetry

HISTORY OF PRESENT ILLNESS:
The patient returns to the psychiatry clinic for ongoing management of body dysmorphic disorder initially diagnosed in July 2020, following recent dermatology evaluation in July 2022 for hidradenitis suppurativa. She reports spending several hours a day examining her skin in the mirror and has undergone multiple dermatological treatments for what she describes as "severe facial asymmetry." She states that the treatments have not been successful and is requesting referral for plastic surgery.

PAST MEDICAL HISTORY:
Active Problem List:
- Hidradenitis suppurativa (diagnosed 2020-01-15)
- Binge eating disorder (diagnosed 2020-03-15)
- Body dysmorphic disorder (diagnosed 2020-07-15)
- Acute intermittent porphyria
- Major depressive disorder, recurrent
- Generalized anxiety disorder
- Obesity (BMI 33)

She has a history of depression and anxiety, for which she is not currently receiving treatment.

PAST SURGICAL HISTORY:
- Multiple dermatologic laser treatments and chemical peels (facial)
- Incision and drainage of right axillary abscess
- Tonsillectomy (age 7)

MEDICATIONS:
Home Medications:
- Sertraline 100 mg oral daily
- Doxycycline 100 mg oral twice daily
- Ibuprofen 400 mg oral every 6 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Acute intermittent porphyria (father)
- Major depressive disorder (mother)
- Hidradenitis suppurativa (sister)
- Type 2 diabetes mellitus (maternal grandmother)

SOCIAL HISTORY:
Smoking: Former smoker (quit 2 years ago; 0.5 pack-year). Alcohol: None (abstinent due to medical history). Occupation: Administrative Assistant (currently on medical leave).

PHYSICAL EXAMINATION:
On examination, her skin appears clear with no obvious asymmetry. She becomes agitated when the physician suggests that her skin looks normal.

## Encounter 5: source 2023-07-15, practice 2025-01-27, outpatient, Dermatology

Chief complaint: Recurrent painful nodules in axillae and groin with foul-smelling drainage despite multiple courses of oral antibiotics

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Sarah Chen, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Recurrent painful nodules in axillae and groin with foul-smelling drainage despite multiple courses of oral antibiotics

HISTORY OF PRESENT ILLNESS:
Returning for continued management of recurrent painful nodules previously evaluated in 2020 and 2022, the patient reports a 3-year history of recurrent, painful nodules in her axillae and groin that often rupture, discharging foul-smelling fluid. Multiple courses of oral antibiotics have provided only temporary relief.

PAST MEDICAL HISTORY:
Active Problem List:
- Hidradenitis suppurativa (diagnosed 2020-01-15)
- Binge eating disorder (diagnosed 2020-03-15)
- Body dysmorphic disorder (diagnosed 2020-07-15)
- Acute intermittent porphyria
- Major depressive disorder, recurrent
- Generalized anxiety disorder
- Obesity (BMI 33)

Her medical history is significant for obesity and severe acne during adolescence.

PAST SURGICAL HISTORY:
- Multiple dermatologic laser treatments and chemical peels (facial)
- Incision and drainage of right axillary abscess
- Tonsillectomy (age 7)

MEDICATIONS:
Home Medications:
- Sertraline 100 mg oral daily
- Doxycycline 100 mg oral twice daily
- Ibuprofen 400 mg oral every 6 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Acute intermittent porphyria (father)
- Major depressive disorder (mother)
- Hidradenitis suppurativa (sister)
- Type 2 diabetes mellitus (maternal grandmother)

SOCIAL HISTORY:
Smoking: Former smoker (quit 2 years ago; 0.5 pack-year). Alcohol: None (abstinent due to medical history). Occupation: Administrative Assistant (currently on medical leave).

VITAL SIGNS:
She is afebrile and vital signs are within normal limits.

PHYSICAL EXAMINATION:
On physical examination, you note several deep-seated nodules in both axillae and the groin, some of which have formed interconnected tracts. There are also multiple hypertrophic scars and open comedones in the same areas.

LABORATORY DATA:
Laboratory studies show:
- Hemoglobin: 13.2 g/dL
- White blood cell count: 9,000/uL
- C-reactive protein: 24 mg/L (normal <5 mg/L)
- HbA1c: 6.1%

## Encounter 6: source 2024-01-15, practice 2025-07-29, ed, Emergency Medicine

Chief complaint: Severe, diffuse abdominal pain with nausea and constipation over the past 2 days

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Jennifer Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe, diffuse abdominal pain with nausea and constipation over the past 2 days

HISTORY OF PRESENT ILLNESS:
This 31-year-old female with acute intermittent porphyria and longitudinal histories of hidradenitis suppurativa and body dysmorphic disorder presents to the emergency department with a 2-day history of severe, diffuse abdominal pain. She describes the pain as cramping and worsening with time. She also reports experiencing nausea and constipation but denies vomiting or diarrhea.

PAST MEDICAL HISTORY:
Active Problem List:
- Hidradenitis suppurativa (diagnosed 2020-01-15)
- Binge eating disorder (diagnosed 2020-03-15)
- Body dysmorphic disorder (diagnosed 2020-07-15)
- Acute intermittent porphyria
- Major depressive disorder, recurrent
- Generalized anxiety disorder
- Obesity (BMI 33)

PAST SURGICAL HISTORY:
- Multiple dermatologic laser treatments and chemical peels (facial)
- Incision and drainage of right axillary abscess
- Tonsillectomy (age 7)

MEDICATIONS:
She takes sertraline.

Home Medications:
- Doxycycline 100 mg oral twice daily
- Ibuprofen 400 mg oral every 6 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
Her mother had similar episodes of abdominal pain in the past but never received a definitive diagnosis

SOCIAL HISTORY:
Smoking: Former smoker (quit 2 years ago; 0.5 pack-year). Alcohol: None (abstinent due to medical history). Occupation: Administrative Assistant (currently on medical leave).

PHYSICAL EXAMINATION:
On examination, her abdomen is soft but diffusely tender without rebound or guarding. Neurological examination reveals mild weakness in her upper extremities and decreased sensation in a "stocking-glove" distribution.

LABORATORY DATA:
Initial laboratory tests reveal:

- Sodium: 140 mEq/L
- Potassium: 3.9 mEq/L
- Chloride: 102 mEq/L
- Bicarbonate: 24 mEq/L
- Blood urea nitrogen: 18 mg/dL
- Creatinine: 0.9 mg/dL
- Calcium: 9.2 mg/dL
- Complete blood count: Normal
- Urinalysis: Normal

OTHER STUDIES:
Given her symptoms and family history, you order a urine porphobilinogen test, which returns elevated.

## Encounter 7: source 2024-03-15, practice 2025-10-01, procedure, Dermatology

Chief complaint: Scheduled removal of benign forearm skin lesion with expressed concerns regarding local anesthetic injection pain

ENCOUNTER NOTE
Date: 2024-03-15
Provider: Dr. Sarah Chen, MD
Department: Dermatology
Visit Type: procedure

CHIEF COMPLAINT:
Scheduled removal of benign forearm skin lesion with expressed concerns regarding local anesthetic injection pain

HISTORY OF PRESENT ILLNESS:
The patient with chronic Hidradenitis suppurativa and anxiety returns for excision. She expresses concern about the pain of the injection for local anesthesia.

PAST MEDICAL HISTORY:
Active Problem List:
- Hidradenitis suppurativa (diagnosed 2020-01-15)
- Binge eating disorder (diagnosed 2020-03-15)
- Body dysmorphic disorder (diagnosed 2020-07-15)
- Acute intermittent porphyria (diagnosed 2024-01-15)
- Major depressive disorder, recurrent
- Generalized anxiety disorder
- Obesity (BMI 33)

PAST SURGICAL HISTORY:
- Multiple dermatologic laser treatments and chemical peels (facial)
- Incision and drainage of right axillary abscess
- Tonsillectomy (age 7)

MEDICATIONS:
Home Medications:
- Sertraline 100 mg oral daily
- Doxycycline 100 mg oral twice daily
- Ibuprofen 400 mg oral every 6 hours as needed for pain

ALLERGIES:
No known allergies or contraindications to local anesthetics

FAMILY HISTORY:
- Acute intermittent porphyria (father)
- Major depressive disorder (mother)
- Hidradenitis suppurativa (sister)
- Type 2 diabetes mellitus (maternal grandmother)

SOCIAL HISTORY:
Smoking: Former smoker (quit 2 years ago; 0.5 pack-year). Alcohol: None (abstinent due to medical history). Occupation: Administrative Assistant (currently on medical leave).

VITAL SIGNS:
Vital signs are within normal limits

PHYSICAL EXAMINATION:
Physical examination is unremarkable except for the 1 cm skin lesion
