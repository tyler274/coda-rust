# synthetic_hospital patient 1811

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 25 at first encounter, sex F, Non-Hispanic White, University student health insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-09-01 (Mon) | outpatient | Student Health Services |
| 1 | 2020-02-15 | 2025-10-09 (Thu) | ed | Emergency Medicine |
| 2 | 2020-07-15 | 2026-03-08 (Sun) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "University student health insurance",
 "occupation": "Graduate student in Environmental Science",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "No significant family history of early cardiovascular disease, tick-borne illness, or inflammatory bowel disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [],
 "chronic_conditions": [],
 "age_at_first_encounter": 25
}
```

Primary diagnoses: ["Dry bite (non-envenomating snake bite)", "Lyme carditis", "Campylobacter jejuni enteritis (recent infection)"]
Comorbidities: ["Asbestos-related occupational exposure"]

## Encounter 0: source 2020-01-15, practice 2025-09-01, outpatient, Student Health Services

Chief complaint: Noticing a tick on right thigh while showering and requesting evaluation for Lyme disease prophylaxis following hiking in Connecticut two days ago.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Maria Gonzalez, MD
Department: Student Health Services
Visit Type: outpatient

CHIEF COMPLAINT:
Noticing a tick on right thigh while showering and requesting evaluation for Lyme disease prophylaxis following hiking in Connecticut two days ago.

HISTORY OF PRESENT ILLNESS:
She presents to the clinic in June after noticing a tick on her right thigh while showering. She recalls hiking in a wooded area in Connecticut two days ago. The patient is anxious about the risk of Lyme disease and requests treatment.

PAST MEDICAL HISTORY:
Her medical history is unremarkable

MEDICATIONS:
She takes no medications.

ALLERGIES:
NKDA

FAMILY HISTORY:
- No significant family history of early cardiovascular disease, tick-borne illness, or inflammatory bowel disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Graduate student in Environmental Science.

REVIEW OF SYSTEMS:
She denies any fever, headache, joint pains, or skin rashes.

PHYSICAL EXAMINATION:
On examination, the tick appears to be an adult Ixodes scapularis and is firmly attached to the skin, but there is no erythema or induration around the site. The rest of the physical examination is normal.

## Encounter 1: source 2020-02-15, practice 2025-10-09, ed, Emergency Medicine

Chief complaint: Two-day history of palpitations and lightheadedness following recent camping trip in upstate New York; reports prior resolving rash and multiple insect bites.

ENCOUNTER NOTE
Date: 2020-02-15
Provider: Dr. James Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-day history of palpitations and lightheadedness following recent camping trip in upstate New York; reports prior resolving rash and multiple insect bites.

HISTORY OF PRESENT ILLNESS:
Six months following prior assessment for tick bite prophylaxis after hiking in Connecticut, the patient presents to the emergency department in July with a 2-day history of palpitations and lightheadedness. She reports a recent camping trip in upstate New York. She denies any known tick bites but recalls multiple insect bites and a rash that resolved spontaneously.

PAST MEDICAL HISTORY:
Active Problem List:
- Dry bite (non-envenomating snake bite) (diagnosed 2020-01-15)

ALLERGIES:
NKDA

FAMILY HISTORY:
- No significant family history of early cardiovascular disease, tick-borne illness, or inflammatory bowel disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Graduate student in Environmental Science.

VITAL SIGNS:
On examination, her temperature is 37.2°C (99.0°F), blood pressure is 110/70 mmHg, pulse is 45 beats per minute, and respiratory rate is 16 breaths per minute.

PHYSICAL EXAMINATION:
Cardiac examination reveals a regular, bradycardic rhythm without murmurs, rubs, or gallops. Neurological examination is unremarkable.

LABORATORY DATA:
Laboratory tests reveal normal complete blood count and basic metabolic panel. Serologic testing for Lyme disease is pending.

OTHER STUDIES:
An electrocardiogram (ECG) shows a second-degree atrioventricular (AV) block, Mobitz type I (Wenckebach phenomenon).

## Encounter 2: source 2020-07-15, practice 2026-03-08, ed, Emergency Medicine

Chief complaint: Two-day history of fever, severe abdominal cramping, and bloody diarrhea after returning from a camping trip where she consumed grilled chicken.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Fatima Al-Hassan, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-day history of fever, severe abdominal cramping, and bloody diarrhea after returning from a camping trip where she consumed grilled chicken.

HISTORY OF PRESENT ILLNESS:
The patient returns following prior tick exposure and palpitations after camping. She reports that her diarrhea contains mucus and blood. She mentions that she recently returned from a camping trip where she consumed grilled chicken.

PAST MEDICAL HISTORY:
Active Problem List:
- Dry bite (non-envenomating snake bite) (diagnosed 2020-01-15)
- Lyme carditis (diagnosed 2020-02-15)

MEDICATIONS:
She takes no medications

ALLERGIES:
NKDA

FAMILY HISTORY:
- No significant family history of early cardiovascular disease, tick-borne illness, or inflammatory bowel disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Graduate student in Environmental Science.

VITAL SIGNS:
On examination, her temperature is 38.5°C (101.3°F), blood pressure is 120/75 mmHg, pulse is 90/min, and respiratory rate is 16/min.

PHYSICAL EXAMINATION:
Abdominal examination reveals mild tenderness in the right lower quadrant without rebound or guarding.

LABORATORY DATA:
Stool studies show no ova or parasites, and stool culture is pending. Complete blood count reveals a white blood cell count of 11,000/µL with a left shift.
