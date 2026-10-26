# synthetic_hospital patient 2584

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 30 at first encounter, sex M, Non-Hispanic White, Blue Cross Blue Shield (Commercial).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-07-10 (Mon) | outpatient | Pulmonology |
| 1 | 2020-10-15 | 2024-04-03 (Wed) | ed | Emergency Medicine |
| 2 | 2022-01-15 | 2025-06-30 (Mon) | ed | Emergency Medicine |
| 3 | 2023-01-15 | 2026-07-05 (Sun) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield (Commercial)",
 "occupation": "Schoolteacher",
 "alcohol_use": "Heavy episodic binge use (weekends)",
 "family_history": [
  "Father: Early-onset emphysema diagnosed age 42, former smoker",
  "Mother: Cryptogenic cirrhosis",
  "Sister: Alpha-1 antitrypsin deficiency carrier (MZ phenotype)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never",
 "home_medications": [
  {
   "dose": "60 mg/kg IV infusion weekly",
   "name": "Alpha-1 antitrypsin augmentation therapy (Prolastin-C)"
  },
  {
   "dose": "18 mcg inhaled daily",
   "name": "Tiotropium bromide"
  },
  {
   "dose": "90 mcg inhaler, 2 puffs every 4-6 hours PRN for dyspnea",
   "name": "Albuterol"
  },
  {
   "dose": "400 mg PO PRN for arm pain",
   "name": "Ibuprofen"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Alpha-1 antitrypsin deficiency (PiZZ phenotype) with panacinar emphysema",
  "Alcohol use disorder",
  "Cocaine use disorder (denies use to providers)",
  "Exercise-induced chest discomfort",
  "Radial tunnel syndrome, right upper extremity"
 ],
 "age_at_first_encounter": 30
}
```

Primary diagnoses: ["Cocaine intoxication", "Alpha-1 antitrypsin deficiency", "Radial tunnel syndrome"]
Comorbidities: ["Acute alcohol intoxication", "Family History of Hereditary Hemorrhagic Telangiectasia"]

## Encounter 0: source 2020-01-15, practice 2023-07-10, outpatient, Pulmonology

Chief complaint: Progressive shortness of breath over the past year with increasing difficulty performing aerobic activities and cycling to work

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive shortness of breath over the past year with increasing difficulty performing aerobic activities and cycling to work

HISTORY OF PRESENT ILLNESS:
He reports that over the last year, he has had increasing difficulty with aerobic activity and recently had to stop cycling to work due to fatigue and dyspnea.

PAST MEDICAL HISTORY:
Active Problem List:
- Alpha-1 antitrypsin deficiency (PiZZ phenotype) with panacinar emphysema
- Alcohol use disorder
- Cocaine use disorder (denies use to providers)
- Exercise-induced chest discomfort
- Radial tunnel syndrome, right upper extremity

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Alpha-1 antitrypsin augmentation therapy (Prolastin-C) 60 mg/kg IV infusion weekly
- Tiotropium bromide 18 mcg inhaled daily
- Albuterol 90 mcg inhaler, 2 puffs every 4-6 hours PRN for dyspnea
- Ibuprofen 400 mg PO PRN for arm pain

ALLERGIES:
NKDA

FAMILY HISTORY:
His mother has asthma, and his father died of cirrhosis in his 40s.

SOCIAL HISTORY:
He does not smoke or use recreational drugs. He works as a schoolteacher and denies exposure to dust, fumes, or industrial chemicals.

REVIEW OF SYSTEMS:
The patient has no chronic cough, wheezing, or sputum production.

PHYSICAL EXAMINATION:
On physical examination, he is thin and mildly tachypneic. Lungs are clear to auscultation with decreased breath sounds and prolonged expiration. His chest is hyperexpanded with an increased anteroposterior diameter.

IMAGING:
Chest radiograph shows bilateral hyperlucency of the lower lobes with flattened diaphragms.

OTHER STUDIES:
Pulmonary function testing shows increased total lung capacity, increased residual volume, and a markedly decreased FEV₁/FVC ratio. A lung biopsy is obtained to confirm the diagnosis and exclude other pathology.

## Encounter 1: source 2020-10-15, practice 2024-04-03, ed, Emergency Medicine

Chief complaint: Acute inability to extend right wrist and fingers after being found unconscious on a park bench with arm draped over a backrest

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute inability to extend right wrist and fingers after being found unconscious on a park bench with arm draped over a backrest

HISTORY OF PRESENT ILLNESS:
The patient with alpha-1 antitrypsin deficiency seen in prior pulmonology care is brought to the emergency department after being found unconscious on a park bench. He had been drinking heavily and was lying with his right arm draped over the backrest for several hours.

PAST MEDICAL HISTORY:
Active Problem List:
- Alpha-1 antitrypsin deficiency (diagnosed 2020-01-15)
- Alpha-1 antitrypsin deficiency (PiZZ phenotype) with panacinar emphysema
- Alcohol use disorder
- Cocaine use disorder (denies use to providers)
- Exercise-induced chest discomfort
- Radial tunnel syndrome, right upper extremity

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Alpha-1 antitrypsin augmentation therapy (Prolastin-C) 60 mg/kg IV infusion weekly
- Tiotropium bromide 18 mcg inhaled daily
- Albuterol 90 mcg inhaler, 2 puffs every 4-6 hours PRN for dyspnea
- Ibuprofen 400 mg PO PRN for arm pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Early-onset emphysema diagnosed age 42, former smoker
- Mother: Cryptogenic cirrhosis
- Sister: Alpha-1 antitrypsin deficiency carrier (MZ phenotype)

SOCIAL HISTORY:
Smoking: Never. Alcohol: Heavy episodic binge use (weekends). Occupation: Schoolteacher.

PHYSICAL EXAMINATION:
On examination, there is weakness of wrist and finger extension, and the hand hangs limply. Sensation is diminished over the dorsal surface of the forearm and the dorsum of the hand, particularly the first web space. Triceps reflex is preserved. There is no swelling, trauma, or tenderness of the elbow or wrist.

## Encounter 2: source 2022-01-15, practice 2025-06-30, ed, Emergency Medicine

Chief complaint: Sudden onset of chest discomfort, severe agitation, and anxiety at a party after using an unknown substance

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Maria Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of chest discomfort, severe agitation, and anxiety at a party after using an unknown substance

HISTORY OF PRESENT ILLNESS:
This 32-year-old male with alpha-1 antitrypsin deficiency (PiZZ phenotype) and cocaine use disorder, evaluated for radial tunnel syndrome after suspected substance use, presents with chest discomfort, agitation, and anxiety. They report that he had been drinking alcohol and using an unknown substance earlier in the evening, after which he became increasingly restless, talkative, and euphoric. Over the past hour he has complained of feeling "on edge" and has had episodes of intense anxiety alternating with bursts of energy. He denies hallucinations but has been noted to pace continuously and speak rapidly.

PAST MEDICAL HISTORY:
Active Problem List:
- Alpha-1 antitrypsin deficiency (diagnosed 2020-01-15)
- Radial tunnel syndrome (diagnosed 2020-10-15)
- Alpha-1 antitrypsin deficiency (PiZZ phenotype) with panacinar emphysema
- Alcohol use disorder
- Cocaine use disorder (denies use to providers)
- Exercise-induced chest discomfort
- Radial tunnel syndrome, right upper extremity

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Alpha-1 antitrypsin augmentation therapy (Prolastin-C) 60 mg/kg IV infusion weekly
- Tiotropium bromide 18 mcg inhaled daily
- Albuterol 90 mcg inhaler, 2 puffs every 4-6 hours PRN for dyspnea
- Ibuprofen 400 mg PO PRN for arm pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Early-onset emphysema diagnosed age 42, former smoker
- Mother: Cryptogenic cirrhosis
- Sister: Alpha-1 antitrypsin deficiency carrier (MZ phenotype)

SOCIAL HISTORY:
Smoking: Never. Alcohol: Heavy episodic binge use (weekends). Occupation: Schoolteacher.

VITAL SIGNS:
His temperature is 38.1°C (100.6°F). Blood pressure is 168/96 mmHg, pulse is 128/min, and respirations are 24/min.

PHYSICAL EXAMINATION:
On examination, he is diaphoretic, pupils are markedly dilated. Cardiac exam reveals tachycardia without murmurs, and lungs are clear.

OTHER STUDIES:
An ECG shows sinus tachycardia without acute ischemic changes.

## Encounter 3: source 2023-01-15, practice 2026-07-05, ed, Emergency Medicine

Chief complaint: Collapse at nightclub with altered mental status, hyperactivity, and rapid speech witnessed by friends

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. David Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Collapse at nightclub with altered mental status, hyperactivity, and rapid speech witnessed by friends

HISTORY OF PRESENT ILLNESS:
This 33-year-old male with Alpha-1 antitrypsin deficiency (PiZZ) and panacinar emphysema returns with presentation similar to 2022. Friends state that he had been drinking alcohol, appeared unusually energetic, and had been speaking rapidly without pause for several hours. They add that he repeatedly left for the restroom and returned with a "refreshed" demeanor.

PAST MEDICAL HISTORY:
Active Problem List:
- Alpha-1 antitrypsin deficiency (diagnosed 2020-01-15)
- Radial tunnel syndrome (diagnosed 2020-10-15)
- Cocaine intoxication (diagnosed 2022-01-15)
- Alpha-1 antitrypsin deficiency (PiZZ phenotype) with panacinar emphysema
- Alcohol use disorder
- Cocaine use disorder (denies use to providers)
- Exercise-induced chest discomfort
- Radial tunnel syndrome, right upper extremity

He has no documented medical history, though his family mentions occasional chest discomfort during exercise.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Alpha-1 antitrypsin augmentation therapy (Prolastin-C) 60 mg/kg IV infusion weekly
- Tiotropium bromide 18 mcg inhaled daily
- Albuterol 90 mcg inhaler, 2 puffs every 4-6 hours PRN for dyspnea
- Ibuprofen 400 mg PO PRN for arm pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Early-onset emphysema diagnosed age 42, former smoker
- Mother: Cryptogenic cirrhosis
- Sister: Alpha-1 antitrypsin deficiency carrier (MZ phenotype)

SOCIAL HISTORY:
Smoking: Never. Alcohol: Heavy episodic binge use (weekends). Occupation: Schoolteacher.

VITAL SIGNS:
Blood pressure is 188/102 mmHg, pulse is 132/min, and respirations are 24/min.

PHYSICAL EXAMINATION:
On arrival, he is diaphoretic, extremely agitated, and combative. His pupils are widely dilated, and his mucous membranes are dry.
Neurological examination is limited due to agitation but shows no focal deficits.

OTHER STUDIES:
On cardiac monitoring, frequent premature ventricular contractions are noted, and the patient clutches his chest, reporting severe substernal pain.

PLAN:
Intravenous benzodiazepines are prepared to manage his behavior and prevent further complications.
