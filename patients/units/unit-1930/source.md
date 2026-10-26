# synthetic_hospital patient 1930

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 35 at first encounter, sex F, Uzbek-American, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-04-01 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-05-15 | 2024-08-06 (Tue) | outpatient | Internal Medicine |
| 2 | 2021-01-15 | 2025-04-08 (Tue) | outpatient | Dermatology |
| 3 | 2021-07-15 | 2025-10-03 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Nickel"
 ],
 "insurance": "Medicaid",
 "occupation": "Jewelry maker (recently transitioned from farm work)",
 "alcohol_use": "None",
 "family_history": [
  "No family history of liver disease or atopic conditions"
 ],
 "race_ethnicity": "Uzbek-American",
 "smoking_status": "Never",
 "home_medications": [],
 "surgical_history": [],
 "chronic_conditions": [],
 "age_at_first_encounter": 35
}
```

Primary diagnoses: ["Hepatic hydatid cyst", "Cutaneous larva migrans", "Allergic contact dermatitis due to nickel"]
Comorbidities: ["Occupational mechanical strain", "Eosinophilia"]

## Encounter 0: source 2020-01-15, practice 2024-04-01, outpatient, Internal Medicine

Chief complaint: Right upper quadrant abdominal pain and occasional nausea

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Elena Vasquez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Right upper quadrant abdominal pain and occasional nausea

HISTORY OF PRESENT ILLNESS:
2-month history of right upper quadrant abdominal pain and occasional nausea. She reports no vomiting, changes in bowel habits, or weight loss. She has not experienced fever or jaundice.

PAST MEDICAL HISTORY:
Her past medical history is unremarkable

MEDICATIONS:
She takes no medications

ALLERGIES:
Nickel

FAMILY HISTORY:
- No family history of liver disease or atopic conditions

SOCIAL HISTORY:
She recently returned from a 6-month humanitarian trip to a rural area in South America where she worked with livestock.

VITAL SIGNS:
On examination, her vital signs are within normal limits.

PHYSICAL EXAMINATION:
Abdominal examination reveals mild tenderness in the right upper quadrant without rebound or guarding.

LABORATORY DATA:
Laboratory tests show normal liver function tests and a mild eosinophilia. Serologic testing for Echinococcus is positive.

IMAGING:
An abdominal ultrasound reveals a 5 cm cystic lesion in the right lobe of the liver with an "eggshell" calcification.

## Encounter 1: source 2020-05-15, practice 2024-08-06, outpatient, Internal Medicine

Chief complaint: Intermittent right upper quadrant pain, nausea, and occasional vomiting

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. Elena Vasquez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent right upper quadrant pain, nausea, and occasional vomiting

HISTORY OF PRESENT ILLNESS:
The patient is a 35-year-old female with a history of hepatic hydatid cyst who returns to the Internal Medicine clinic for follow-up, having previously presented on January 15, 2020, with similar complaints. She presents with a 6-month history of intermittent right upper quadrant pain, nausea, and occasional vomiting. She reports that the pain is dull and does not radiate. She has not noticed any change in bowel habits or weight loss.

PAST MEDICAL HISTORY:
Active Problem List:
- Hepatic hydatid cyst (diagnosed 2020-01-15)

ALLERGIES:
Nickel

FAMILY HISTORY:
- No family history of liver disease or atopic conditions

SOCIAL HISTORY:
The patient immigrated from a rural area in Central Asia two years ago and works on a farm where she is in regular contact with dogs.

PHYSICAL EXAMINATION:
On physical examination, she has mild hepatomegaly without jaundice.

LABORATORY DATA:
Laboratory studies show:

- White blood cell count: 7,000/uL
- Hemoglobin: 13.5 g/dL
- Platelets: 250,000/uL
- Alanine aminotransferase (ALT): 45 U/L
- Aspartate aminotransferase (AST): 40 U/L
- Alkaline phosphatase: 120 U/L
- Total bilirubin: 0.8 mg/dL

IMAGING:
Abdominal ultrasound reveals a 10 cm anechoic cystic structure in the right lobe of the liver with internal septations. No free fluid is noted in the abdomen.

## Encounter 2: source 2021-01-15, practice 2025-04-08, outpatient, Dermatology

Chief complaint: 2-day history of an itchy rash on her wrists and hands

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Kwame Osei, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
2-day history of an itchy rash on her wrists and hands

HISTORY OF PRESENT ILLNESS:
The patient is a 36-year-old female with a history of hepatic hydatid cyst. She reports recently starting a new job as a jewelry maker and has been handling various metals, and denies new soaps, lotions, or cosmetics.

PAST MEDICAL HISTORY:
Active Problem List:
- Hepatic hydatid cyst (diagnosed 2020-01-15)

MEDICATIONS:
She takes no medications

ALLERGIES:
She has no known drug allergies

FAMILY HISTORY:
- No family history of liver disease or atopic conditions

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Jewelry maker (recently transitioned from farm work).

REVIEW OF SYSTEMS:
She has no respiratory symptoms
She denies fever, joint pain, or recent travel

VITAL SIGNS:
Vital signs:
- Temperature: 98.6°F (37°C)
- Blood pressure: 120/78 mmHg
- Heart rate: 78 bpm
- Respiratory rate: 16 breaths/min

PHYSICAL EXAMINATION:
On examination, there is erythema, edema, and small vesicles on the dorsal aspects of both wrists and the interdigital spaces of her hands.
Physical examination:
- Skin: Erythematous, edematous plaques with overlying vesicles on the wrists and hands; no mucosal involvement

## Encounter 3: source 2021-07-15, practice 2025-10-03, ed, Emergency Medicine

Chief complaint: 2-week history of an itchy rash on her right foot

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Mei-Lin Zhou, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
2-week history of an itchy rash on her right foot

HISTORY OF PRESENT ILLNESS:
The patient presents with new foot rash following abdominal symptoms in 2020 and wrist dermatitis in January 2021. She recently returned from beach vacation in Caribbean where she frequently walked barefoot on sand. She reports the rash began as a small red bump but progressed to form a winding, raised track that extends several centimeters.

PAST MEDICAL HISTORY:
Active Problem List:
- Hepatic hydatid cyst (diagnosed 2020-01-15)
- Allergic contact dermatitis due to nickel (diagnosed 2021-01-15)

MEDICATIONS:
She takes no medications

ALLERGIES:
Nickel

FAMILY HISTORY:
- No family history of liver disease or atopic conditions

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Jewelry maker (recently transitioned from farm work).

REVIEW OF SYSTEMS:
She denies any fever, chills, or systemic symptoms.

PHYSICAL EXAMINATION:
On physical examination, there is a serpiginous, erythematous, and slightly raised track on the lateral aspect of her right foot. There are no signs of secondary infection. The remainder of the examination is unremarkable.

LABORATORY DATA:
Laboratory values:
- Complete blood count: Within normal limits
- Eosinophil count: Slightly elevated
- Stool ova and parasites: Negative
