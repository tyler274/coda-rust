# synthetic_hospital patient 2010

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 55 at first encounter, sex M, Non-Hispanic White, Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2019-08-01 (Thu) | outpatient | Gastroenterology |
| 1 | 2021-03-15 | 2020-10-13 (Tue) | outpatient | Rheumatology |
| 2 | 2022-09-15 | 2022-04-12 (Tue) | ed | Emergency Medicine |
| 3 | 2024-09-15 | 2024-04-15 (Mon) | ed | Emergency Medicine |
| 4 | 2026-07-15 | 2026-01-31 (Sat) | inpatient | Pulmonology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield PPO",
 "occupation": "Construction site supervisor",
 "alcohol_use": "Occasional (1-2 drinks per week)",
 "family_history": [
  "Rheumatoid arthritis in mother",
  "Hypertension in father",
  "Atrial fibrillation in paternal grandfather",
  "Coronary artery disease in father"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "15 mg weekly",
   "name": "Methotrexate"
  },
  {
   "dose": "5 mg daily",
   "name": "Prednisone"
  },
  {
   "dose": "5 mg daily (adjusted per INR)",
   "name": "Warfarin"
  },
  {
   "dose": "20 mg daily",
   "name": "Lisinopril"
  },
  {
   "dose": "5 mg daily",
   "name": "Amlodipine"
  }
 ],
 "surgical_history": [
  "Repair of right quadriceps tendon rupture"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Seropositive rheumatoid arthritis",
  "Paroxysmal atrial fibrillation",
  "History of ventricular tachycardia",
  "Seasonal allergic rhinitis",
  "History of procainamide-induced systemic lupus erythematosus",
  "History of Helicobacter pylori infection"
 ],
 "age_at_first_encounter": 55
}
```

Primary diagnoses: ["Helicobacter pylori infection", "Amiodarone-induced pulmonary toxicity", "Quadriceps tendon rupture", "Acute deep vein thrombosis of left lower extremity", "Drug-induced systemic lupus erythematosus"]
Comorbidities: ["Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2019-08-01, outpatient, Gastroenterology

Chief complaint: Epigastric burning discomfort that persists for several months, worse between meals and at night

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Epigastric burning discomfort that persists for several months, worse between meals and at night

HISTORY OF PRESENT ILLNESS:
That has persisted for several months. He describes the pain as a burning sensation that worsens between meals and occasionally wakes him up at night. He has tried over-the-counter antacids with minimal relief. He denies any weight loss, vomiting, or blood in the stool but notes occasional nausea and early satiety.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Seropositive rheumatoid arthritis
- Paroxysmal atrial fibrillation
- History of ventricular tachycardia
- Seasonal allergic rhinitis
- History of procainamide-induced systemic lupus erythematosus
- History of Helicobacter pylori infection

His medical history is significant only for hypertension, well controlled on amlodipine.

PAST SURGICAL HISTORY:
- Repair of right quadriceps tendon rupture

MEDICATIONS:
He takes amlodipine.

Home Medications:
- Methotrexate 15 mg weekly
- Prednisone 5 mg daily
- Warfarin 5 mg daily (adjusted per INR)
- Lisinopril 20 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Rheumatoid arthritis in mother
- Hypertension in father
- Atrial fibrillation in paternal grandfather
- Coronary artery disease in father

SOCIAL HISTORY:
He has no history of NSAID use, alcohol abuse, or smoking.

PHYSICAL EXAMINATION:
Physical examination is unremarkable except for mild epigastric tenderness without guarding or rebound. No hepatosplenomegaly or abdominal masses are noted.

LABORATORY DATA:
His stool is guaiac negative.

PATHOLOGY:
A light microscopy from a biopsy specimen is shown.

OTHER STUDIES:
An upper endoscopy is performed and reveals a shallow ulcer in the proximal duodenum.

## Encounter 1: source 2021-03-15, practice 2020-10-13, outpatient, Rheumatology

Chief complaint: Symmetric joint pain, fatigue, low-grade fevers, and photosensitive malar rash for 3 weeks

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Marcus Johnson, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Symmetric joint pain, fatigue, low-grade fevers, and photosensitive malar rash for 3 weeks

HISTORY OF PRESENT ILLNESS:
The patient with seropositive rheumatoid arthritis and history of procainamide-induced systemic lupus erythematosus presents with three weeks of progressive symptoms. He reports symmetric stiffness in his wrists, elbows, and knees, especially in the mornings, which improves throughout the day. He also notes intermittent muscle aches and a photosensitive rash across his cheeks and nose that appeared after a recent fishing trip. He denies prior history of autoimmune disease.

PAST MEDICAL HISTORY:
Active Problem List:
- Helicobacter pylori infection (diagnosed 2020-01-15)
- Hypertension
- Seropositive rheumatoid arthritis
- Paroxysmal atrial fibrillation
- History of ventricular tachycardia
- Seasonal allergic rhinitis
- History of procainamide-induced systemic lupus erythematosus

His past medical history is notable for hypertension and a history of ventricular arrhythmias,.

PAST SURGICAL HISTORY:
- Repair of right quadriceps tendon rupture

MEDICATIONS:
He takes for which he has been taking procainamide for the past 5 months. He takes no other medications, does not use over-the-counter supplements,.

Home Medications:
- Methotrexate 15 mg weekly
- Prednisone 5 mg daily
- Warfarin 5 mg daily (adjusted per INR)
- Lisinopril 20 mg daily
- Amlodipine 5 mg daily

ALLERGIES:
He has no known allergies.

FAMILY HISTORY:
- Rheumatoid arthritis in mother
- Hypertension in father
- Atrial fibrillation in paternal grandfather
- Coronary artery disease in father

SOCIAL HISTORY:
He is a non-smoker and drinks alcohol occasionally.

PHYSICAL EXAMINATION:
On physical exam, he has mild swelling and tenderness of the wrists and metacarpophalangeal joints bilaterally. There is an erythematous maculopapular rash across the malar region that spares the nasolabial folds. Cardiopulmonary and neurologic exams are normal.

LABORATORY DATA:
Laboratory testing reveals a positive antinuclear antibody (ANA) titer and mildly elevated ESR.

PLAN:
Additional testing is ordered to evaluate the suspected autoimmune etiology.

## Encounter 2: source 2022-09-15, practice 2022-04-12, ed, Emergency Medicine

Chief complaint: Right knee injury after falling from ladder with audible pop, severe pain, swelling, and inability to bear weight

ENCOUNTER NOTE
Date: 2022-09-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Right knee injury after falling from ladder with audible pop, severe pain, swelling, and inability to bear weight

HISTORY OF PRESENT ILLNESS:
This 57-year-old male with seropositive RA and history of procainamide-induced lupus presents. He reports hearing a "pop" in his right knee followed by severe pain, swelling, and inability to stand or walk.

PAST MEDICAL HISTORY:
Active Problem List:
- Helicobacter pylori infection (diagnosed 2020-01-15)
- Drug-induced systemic lupus erythematosus (diagnosed 2021-03-15)
- Hypertension
- Seropositive rheumatoid arthritis
- Paroxysmal atrial fibrillation
- History of ventricular tachycardia
- Seasonal allergic rhinitis
- History of procainamide-induced systemic lupus erythematosus

His medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- Repair of right quadriceps tendon rupture

MEDICATIONS:
He takes for which he takes lisinopril.

Home Medications:
- Methotrexate 15 mg weekly
- Prednisone 5 mg daily
- Warfarin 5 mg daily (adjusted per INR)
- Amlodipine 5 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Rheumatoid arthritis in mother
- Hypertension in father
- Atrial fibrillation in paternal grandfather
- Coronary artery disease in father

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per week). Occupation: Construction site supervisor.

PHYSICAL EXAMINATION:
On physical examination, his right knee is swollen and ecchymotic. Palpation reveals a gap above the patella, and the patella appears to be lower than on the unaffected side. He is unable to extend his right knee against gravity.

IMAGING:
An X-ray of the knee shows no bone fractures but reveals an abnormal position of the patella.

## Encounter 3: source 2024-09-15, practice 2024-04-15, ed, Emergency Medicine

Chief complaint: Acute onset of left calf swelling and mild pain over the past 2 days

ENCOUNTER NOTE
Date: 2024-09-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset of left calf swelling and mild pain over the past 2 days

HISTORY OF PRESENT ILLNESS:
This 59-year-old with paroxysmal atrial fibrillation on warfarin, procainamide-induced lupus, and quadriceps tendon rupture presents to emergency department after noticing swelling, mild pain in left calf for past 2 days.

PAST MEDICAL HISTORY:
Active Problem List:
- Helicobacter pylori infection (diagnosed 2020-01-15)
- Drug-induced systemic lupus erythematosus (diagnosed 2021-03-15)
- Quadriceps tendon rupture (diagnosed 2022-09-15)
- Hypertension
- Seropositive rheumatoid arthritis
- Paroxysmal atrial fibrillation
- History of ventricular tachycardia
- Seasonal allergic rhinitis
- History of procainamide-induced systemic lupus erythematosus

He has a history of hypertension and was diagnosed with atrial fibrillation 10 days ago,.

PAST SURGICAL HISTORY:
- Repair of right quadriceps tendon rupture

MEDICATIONS:
He takes for which his physician prescribed warfarin. He takes no herbal supplements,.

Home Medications:
- Methotrexate 15 mg weekly
- Prednisone 5 mg daily
- Lisinopril 20 mg daily
- Amlodipine 5 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Rheumatoid arthritis in mother
- Hypertension in father
- Atrial fibrillation in paternal grandfather
- Coronary artery disease in father

SOCIAL HISTORY:
Drinks alcohol occasionally, and does not smoke.

REVIEW OF SYSTEMS:
He denies chest pain, dyspnea, or recent trauma.

VITAL SIGNS:
Vital signs are within normal limits.

PHYSICAL EXAMINATION:
On examination, the left lower extremity is warm, erythematous, and tender along the posterior calf with mild pitting edema.

LABORATORY DATA:
Laboratory testing reveals an INR of 3.1.

IMAGING:
Duplex ultrasonography confirms acute deep vein thrombosis.

PLAN:
The patient is advised to continue his current medication with routine follow-up.

## Encounter 4: source 2026-07-15, practice 2026-01-31, inpatient, Pulmonology

Chief complaint: Progressive shortness of breath and dry cough developing over 2 weeks

ENCOUNTER NOTE
Date: 2026-07-15
Provider: Dr. Emily Rodriguez, MD
Department: Pulmonology
Visit Type: inpatient

CHIEF COMPLAINT:
Progressive shortness of breath and dry cough developing over 2 weeks

PAST MEDICAL HISTORY:
Active Problem List:
- Helicobacter pylori infection (diagnosed 2020-01-15)
- Drug-induced systemic lupus erythematosus (diagnosed 2021-03-15)
- Quadriceps tendon rupture (diagnosed 2022-09-15)
- Acute deep vein thrombosis of left lower extremity (diagnosed 2024-09-15)
- Hypertension
- Seropositive rheumatoid arthritis
- Paroxysmal atrial fibrillation
- History of ventricular tachycardia
- Seasonal allergic rhinitis
- History of procainamide-induced systemic lupus erythematosus

Medical history is significant for seropositive rheumatoid arthritis, seasonal allergies and newly diagnosed hypertension.

PAST SURGICAL HISTORY:
- Repair of right quadriceps tendon rupture

MEDICATIONS:
He has been taking methotrexate and low-dose prednisone for the past year, and was started a month ago on propranolol and hydrochlorothiazide.

Home Medications:
- Warfarin 5 mg daily (adjusted per INR)
- Lisinopril 20 mg daily
- Amlodipine 5 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Rheumatoid arthritis in mother
- Hypertension in father
- Atrial fibrillation in paternal grandfather
- Coronary artery disease in father

SOCIAL HISTORY:
He does not smoke

PHYSICAL EXAMINATION:
Physical examination reveals end-inspiratory crackles at the lung bases.

IMAGING:
Chest x-ray reveals bilateral reticular opacities.
