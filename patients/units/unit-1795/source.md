# synthetic_hospital patient 1795

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 22 at first encounter, sex F, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-11-05 (Tue) | ed | Obstetrics & Gynecology |
| 1 | 2020-11-15 | 2025-09-07 (Sun) | ed | Emergency Medicine |
| 2 | 2021-07-15 | 2026-04-25 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Part-time retail associate",
 "alcohol_use": "None",
 "family_history": [
  "Autoimmune thyroid disease (mother)",
  "Systemic lupus erythematosus (maternal aunt)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never",
 "home_medications": [],
 "surgical_history": [],
 "chronic_conditions": [],
 "age_at_first_encounter": 22
}
```

Primary diagnoses: ["Acute Salicylate Toxicity (Aspirin Overdose)", "Anti-NMDA receptor encephalitis", "Septic abortion"]
Comorbidities: ["Sinus tachycardia secondary to burns", "Hypotension"]

## Encounter 0: source 2020-01-15, practice 2024-11-05, ed, Obstetrics & Gynecology

Chief complaint: Fever, lower abdominal pain, and heavy vaginal bleeding for 2 days following elective abortion at an outside clinic.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Obstetrics & Gynecology
Visit Type: ed

CHIEF COMPLAINT:
Fever, lower abdominal pain, and heavy vaginal bleeding for 2 days following elective abortion at an outside clinic.

HISTORY OF PRESENT ILLNESS:
She presents to the emergency department with a 2-day history of fever, lower abdominal pain, and heavy vaginal bleeding. She reports that she underwent an elective abortion at a clinic she found online 1 week ago. Since the procedure, she has felt increasingly unwell.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Autoimmune thyroid disease (mother)
- Systemic lupus erythematosus (maternal aunt)

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Part-time retail associate.

VITAL SIGNS:
Her temperature is 38.9°C (102°F), blood pressure is 100/60 mmHg, and pulse is 110/min.

PHYSICAL EXAMINATION:
On pelvic examination, there is purulent discharge from the cervical os and the uterus is tender to palpation.

LABORATORY DATA:
Laboratory findings are as follows:

- White blood cell count: 18,000/mm³
- Hemoglobin: 10.2 g/dL
- Platelets: 250,000/mm³
- C-reactive protein: 24 mg/L (normal <5 mg/L)
- Blood cultures: Pending

IMAGING:
Transvaginal ultrasound shows retained products of conception with no evidence of intrauterine gestation.

## Encounter 1: source 2020-11-15, practice 2025-09-07, ed, Emergency Medicine

Chief complaint: Brought by friends due to erratic behavior over the past week including severe mood swings, paranoia, and auditory hallucinations.

ENCOUNTER NOTE
Date: 2020-11-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Brought by friends due to erratic behavior over the past week including severe mood swings, paranoia, and auditory hallucinations.

HISTORY OF PRESENT ILLNESS:
The patient is a 22-year-old female with a prior Emergency Department encounter in January 2020 following elective abortion who now presents with acute behavioral disturbances. They report that she has been experiencing severe mood swings, paranoia, and auditory hallucinations.

PAST MEDICAL HISTORY:
Active Problem List:
- Septic abortion (diagnosed 2020-01-15)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Autoimmune thyroid disease (mother)
- Systemic lupus erythematosus (maternal aunt)

SOCIAL HISTORY:
She denies the use of drugs or alcohol.

VITAL SIGNS:
She has a low-grade fever, and her vitals are as follows: BP 130/85 mmHg, HR 102 bpm, RR 18 breaths/min, and temperature 37.8°C (100°F).

PHYSICAL EXAMINATION:
On examination, she is agitated, disoriented to time and place, and exhibits orofacial dyskinesias.
A neurological examination reveals no focal deficits, but she exhibits increased muscle tone and intermittent dystonic postures of her arms.

LABORATORY DATA:
CSF analysis shows:
- WBC: 22/mm³ (normal: 0-5/mm³)
- RBC: 4/mm³ (normal: 0-2/mm³)
- Protein: 56 mg/dL (normal: 15-45 mg/dL)
- Glucose: 60 mg/dL (normal: 45-80 mg/dL)
- No organisms seen on Gram stain

IMAGING:
A CT scan of the pelvis reveals a 3 cm mass in the right ovary.

OTHER STUDIES:
An EEG shows diffuse slow-wave activity without epileptiform discharges.

## Encounter 2: source 2021-07-15, practice 2026-04-25, ed, Emergency Medicine

Chief complaint: Found lying on bathroom floor confused and lethargic after reportedly ingesting an entire bottle of over-the-counter pills following an argument.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Found lying on bathroom floor confused and lethargic after reportedly ingesting an entire bottle of over-the-counter pills following an argument.

HISTORY OF PRESENT ILLNESS:
The patient returns to the emergency department with history of anti-NMDA receptor encephalitis and septic abortion. One hour earlier, the patient reportedly ingested an entire bottle of over-the-counter pills after an argument with her partner. The roommate says she later complained of nausea, ringing in her ears, and dizziness.

PAST MEDICAL HISTORY:
Active Problem List:
- Septic abortion (diagnosed 2020-01-15)
- Anti-NMDA receptor encephalitis (diagnosed 2020-11-15)

MEDICATIONS:
She takes no regular medications

ALLERGIES:
NKDA

FAMILY HISTORY:
- Autoimmune thyroid disease (mother)
- Systemic lupus erythematosus (maternal aunt)

SOCIAL HISTORY:
She denies drug or alcohol use, and this is her first known overdose.

VITAL SIGNS:
Her blood pressure is 102/58 mm Hg, heart rate is 115/min, respiratory rate is 28/min, and oxygen saturation is 98% on room air.

PHYSICAL EXAMINATION:
On examination, she is somnolent but arousable, breathing rapidly and deeply. Pupils are equal and reactive; neurologic exam is non-focal. Her mucous membranes are dry and capillary refill is slightly delayed. Lungs are clear to auscultation, and there are no signs of trauma or track marks.

LABORATORY DATA:
Urinalysis is pending. A serum drug screen is sent, and blood gases are drawn for evaluation of her acid-base status.

OTHER STUDIES:
An ECG shows sinus tachycardia.

ASSESSMENT AND PLAN:
The clinical team suspects the ingestion of a toxic metabolic agent and begins planning treatment.
