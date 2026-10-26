# synthetic_hospital patient 2525

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 28 at first encounter, sex M, Non-Hispanic White, Commercial health insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-11-27 (Wed) | ed | Emergency Medicine |
| 1 | 2020-02-15 | 2025-01-07 (Tue) | ed | Emergency Medicine |
| 2 | 2021-01-15 | 2025-11-25 (Tue) | icu | Critical Care |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial health insurance",
 "occupation": "Warehouse inventory associate",
 "alcohol_use": "None",
 "family_history": [
  "Mother with Hashimoto thyroiditis",
  "Father with hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "400-600 mg PO PRN for back pain",
   "name": "Ibuprofen"
  },
  {
   "dose": "5 mg PO PRN for muscle spasm",
   "name": "Cyclobenzaprine"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Chronic lower back pain"
 ],
 "age_at_first_encounter": 28
}
```

Primary diagnoses: ["Goodpasture syndrome", "Acute kidney injury"]
Comorbidities: ["Hypoxemia", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2024-11-27, ed, Emergency Medicine

Chief complaint: progressive shortness of breath, fatigue, and intermittent coughing with hemoptysis

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive shortness of breath, fatigue, and intermittent coughing with hemoptysis

HISTORY OF PRESENT ILLNESS:
A 28-year-old man is brought to the emergency department due to progressive shortness of breath, fatigue, and intermittent coughing over the past two weeks. He also reports several episodes of coughing up blood, especially in the mornings, along with dull chest discomfort.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic lower back pain

MEDICATIONS:
Home Medications:
- Ibuprofen 400-600 mg PO PRN for back pain
- Cyclobenzaprine 5 mg PO PRN for muscle spasm

ALLERGIES:
NKDA

FAMILY HISTORY:
Family history is noncontributory.

SOCIAL HISTORY:
He denies recent travel, illicit drug use, or occupational exposures.
He is a non-smoker but works in a warehouse with occasional exposure to dust and fumes.

VITAL SIGNS:
On examination, his temperature is 37.4°C (99.3°F), blood pressure is 142/84 mm Hg, pulse is 88/min, and oxygen saturation is 88% on room air.

PHYSICAL EXAMINATION:
Chest auscultation reveals diffuse bilateral crackles. Cardiac exam is normal. Skin examination is clear, with no rashes or lesions.

LABORATORY DATA:
Urinalysis shows 3+ blood and 2+ protein. Microscopy reveals dysmorphic red blood cells and numerous red blood cell casts. Serum creatinine is 3.6 mg/dL (baseline unknown).

IMAGING:
A chest radiograph demonstrates bilateral patchy opacities.

PATHOLOGY:
Renal biopsy reveals crescentic glomerulonephritis with linear immunofluorescence staining for IgG.

## Encounter 1: source 2020-02-15, practice 2025-01-07, ed, Emergency Medicine

Chief complaint: worsening shortness of breath, recurrent hemoptysis, and decreased urine output

ENCOUNTER NOTE
Date: 2020-02-15
Provider: Dr. James O'Connor, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening shortness of breath, recurrent hemoptysis, and decreased urine output

HISTORY OF PRESENT ILLNESS:
The patient returns to the emergency department approximately one month following his prior presentation with progressive shortness of breath, fatigue, and intermittent hemoptysis. He presents with worsening shortness of breath, recurrent hemoptysis, and decreased urine output. He also reports decreased urine output and generalized malaise.

PAST MEDICAL HISTORY:
Active Problem List:
- Goodpasture syndrome (diagnosed 2020-01-15)
- Chronic lower back pain

MEDICATIONS:
Home Medications:
- Ibuprofen 400-600 mg PO PRN for back pain
- Cyclobenzaprine 5 mg PO PRN for muscle spasm

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with Hashimoto thyroiditis
- Father with hypertension

SOCIAL HISTORY:
He denies any recent travel, sick contacts, or use of illicit drugs.

VITAL SIGNS:
On physical examination, his blood pressure is 150/90 mmHg, heart rate is 88 beats per minute, respiratory rate is 22 breaths per minute, and oxygen saturation is 92% on room air

PHYSICAL EXAMINATION:
Lung auscultation reveals bilateral crackles, and there is mild edema in his lower extremities.

LABORATORY DATA:
- Hemoglobin: 10.2 g/dL (Normal: 13.8-17.2 g/dL)
- Creatinine: 4.5 mg/dL (Normal: 0.6-1.3 mg/dL)
- Blood urea nitrogen (BUN): 36 mg/dL (Normal: 7-20 mg/dL)
- Urinalysis: Dysmorphic red cells, red cell casts

IMAGING:
Chest X-ray: Diffuse bilateral pulmonary infiltrates

## Encounter 2: source 2021-01-15, practice 2025-11-25, icu, Critical Care

Chief complaint: unresponsive

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Maria Rodriguez, MD
Department: Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Unresponsive

HISTORY OF PRESENT ILLNESS:
The patient is a 33-year-old man with Goodpasture syndrome and chronic back pain. He is brought to the emergency department by his roommate, who found him unresponsive on the couch approximately 10 hours ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Goodpasture syndrome (diagnosed 2020-01-15)
- Chronic lower back pain

The roommate reports that the patient has a history of chronic back pain.

MEDICATIONS:
He has taken various medications over the years, although his current regimen is unknown.

Home Medications:
- Ibuprofen 400-600 mg PO PRN for back pain
- Cyclobenzaprine 5 mg PO PRN for muscle spasm

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with Hashimoto thyroiditis
- Father with hypertension

SOCIAL HISTORY:
The patient has also been socially withdrawn lately and recently expressed feeling overwhelmed by work. He does not drink alcohol or smoke, but he has used recreational substances in the past

VITAL SIGNS:
His respiratory rate is 8/min, oxygen saturation is 91% on room air, temperature is 36.2°C (97.2°F), blood pressure is 106/62 mm Hg, and heart rate is 58/min.

PHYSICAL EXAMINATION:
On arrival, the patient is somnolent but arousable to painful stimuli. Pupils are pinpoint and reactive. He has minor abrasions on his elbow and knee but no signs of significant trauma. His breath sounds are clear, heart exam is unremarkable, and there is no edema. His abdomen is soft and non-tender. On examination of the extremities, the left leg is tender with tense muscles and discomfort on passive movement. Capillary refill is slightly delayed. A Foley catheter is placed, and dark brown urine is noted.
