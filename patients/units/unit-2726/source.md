# synthetic_hospital patient 2726

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 44 at first encounter, sex F, African American, Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-02-08 (Mon) | outpatient | Rheumatology |
| 1 | 2020-04-15 | 2021-05-11 (Tue) | outpatient | Internal Medicine |
| 2 | 2021-01-15 | 2022-01-31 (Mon) | outpatient | Orthopedic Surgery |
| 3 | 2022-01-15 | 2023-01-27 (Fri) | ed | Emergency Medicine |
| 4 | 2022-03-15 | 2023-03-28 (Tue) | outpatient | Psychiatry |
| 5 | 2023-03-15 | 2024-03-29 (Fri) | ed | Emergency Medicine |
| 6 | 2024-03-15 | 2025-04-09 (Wed) | outpatient | Rheumatology |
| 7 | 2025-03-15 | 2026-04-05 (Sun) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Hydralazine (drug-induced hemolysis)",
  "Sulfonamides (severe rash)"
 ],
 "insurance": "Blue Cross Blue Shield PPO",
 "occupation": "Administrative Assistant",
 "alcohol_use": "None",
 "family_history": [
  "Mother: Systemic lupus erythematosus",
  "Father: Hypertension, type 2 diabetes mellitus",
  "Sister: Hashimoto thyroiditis"
 ],
 "race_ethnicity": "African American",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "200 mg PO BID",
   "name": "Hydroxychloroquine"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Prednisone"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Amlodipine"
  },
  {
   "dose": "25 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "2000 units PO daily",
   "name": "Cholecalciferol"
  },
  {
   "dose": "600 mg PO BID",
   "name": "Calcium carbonate"
  }
 ],
 "surgical_history": [
  "Splenectomy (atraumatic rupture)",
  "Right hip core decompression",
  "Stereotactic brain biopsy"
 ],
 "chronic_conditions": [
  "Systemic lupus erythematosus",
  "Hypertension",
  "Avascular necrosis of right femoral head",
  "History of primary central nervous system lymphoma",
  "Status post splenectomy",
  "Osteopenia"
 ],
 "age_at_first_encounter": 44
}
```

Primary diagnoses: ["Senile purpura", "Glucocorticoid-induced neuropsychiatric symptoms", "Avascular necrosis of femoral head", "Immunosuppressed state", "Atraumatic splenic rupture with hemoperitoneum", "Secondary Hyperaldosteronism due to Dietary Sodium Restriction", "Primary central nervous system lymphoma", "Leukocytosis"]
Comorbidities: ["Long-term glucocorticoid therapy", "Essential hypertension", "Systemic lupus erythematosus"]

## Encounter 0: source 2020-01-15, practice 2021-02-08, outpatient, Rheumatology

Chief complaint: persistent fatigue, diffuse arthralgia, and intermittent low-grade fevers with photosensitive skin changes and recurrent oral ulcers

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent fatigue, diffuse arthralgia, and intermittent low-grade fevers with photosensitive skin changes and recurrent oral ulcers

HISTORY OF PRESENT ILLNESS:
She reports a several-month history of photosensitive skin changes and recurrent oral ulcers.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus
- Hypertension
- Avascular necrosis of right femoral head
- History of primary central nervous system lymphoma
- Status post splenectomy
- Osteopenia

PAST SURGICAL HISTORY:
- Splenectomy (atraumatic rupture)
- Right hip core decompression
- Stereotactic brain biopsy

MEDICATIONS:
She takes she takes only occasional over-the-counter analgesics.

Home Medications:
- Hydroxychloroquine 200 mg PO BID
- Prednisone 10 mg PO daily
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Cholecalciferol 2000 units PO daily
- Calcium carbonate 600 mg PO BID

ALLERGIES:
Hydralazine (drug-induced hemolysis), Sulfonamides (severe rash)

FAMILY HISTORY:
- Mother: Systemic lupus erythematosus
- Father: Hypertension, type 2 diabetes mellitus
- Sister: Hashimoto thyroiditis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative Assistant.

PHYSICAL EXAMINATION:
On examination, she appears tired but in no acute distress. There are faint erythematous patches across the malar region, mild synovial fullness at the wrists, and a few shallow ulcerations on the buccal mucosa. Cardiopulmonary findings are normal.

LABORATORY DATA:
Laboratory studies reveal mild normocytic anemia and the presence of antinuclear antibodies, but renal function and urinalysis are normal at this time.

OTHER STUDIES:
During a research protocol, her peripheral blood mononuclear cells are isolated and exposed in vitro to a foreign peptide antigen. Despite adequate antigen presentation by MHC class II molecules, the T lymphocytes fail to proliferate or produce interleukin-2.

## Encounter 1: source 2020-04-15, practice 2021-05-11, outpatient, Internal Medicine

Chief complaint: difficult-to-control hypertension despite adherence to three-drug regimen

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Difficult-to-control hypertension despite adherence to three-drug regimen

HISTORY OF PRESENT ILLNESS:
Patient with SLE returns for hypertension follow-up after Rheumatology evaluation three months ago for fatigue and arthralgia, reporting adherence to a three-drug regimen including a calcium channel blocker, an ACE inhibitor, and a thiazide diuretic while denying headache, chest pain, or palpitations.

PAST MEDICAL HISTORY:
Active Problem List:
- Immunosuppressed state (diagnosed 2020-01-15)
- Systemic lupus erythematosus
- Hypertension
- Avascular necrosis of right femoral head
- History of primary central nervous system lymphoma
- Status post splenectomy
- Osteopenia

PAST SURGICAL HISTORY:
- Splenectomy (atraumatic rupture)
- Right hip core decompression
- Stereotactic brain biopsy

MEDICATIONS:
She takes a three-drug regimen including a calcium channel blocker, an ACE inhibitor, and a thiazide diuretic.

Home Medications:
- Hydroxychloroquine 200 mg PO BID
- Prednisone 10 mg PO daily
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Cholecalciferol 2000 units PO daily
- Calcium carbonate 600 mg PO BID

ALLERGIES:
Hydralazine (drug-induced hemolysis), Sulfonamides (severe rash)

FAMILY HISTORY:
Her family history is non-contributory.

SOCIAL HISTORY:
She does not smoke, drink alcohol, or use illicit drugs.

VITAL SIGNS:
Her blood pressure is 154/96 mmHg, heart rate is 78 beats per minute

PHYSICAL EXAMINATION:
On physical examination, she appears well-hydrated. No edema is noted.

LABORATORY DATA:
Laboratory results are as follows:

- Sodium: 146 mEq/L (Normal: 135-145 mEq/L)
- Potassium: 3.1 mEq/L (Normal: 3.5-5.1 mEq/L)
- Chloride: 108 mEq/L (Normal: 98-107 mEq/L)
- Bicarbonate: 32 mEq/L (Normal: 22-29 mEq/L)
- Creatinine: 0.9 mg/dL (Normal: 0.6-1.1 mg/dL)
- Plasma renin activity: Low

## Encounter 2: source 2021-01-15, practice 2022-01-31, outpatient, Orthopedic Surgery

Chief complaint: 3-month history of progressive right hip pain worse with weight-bearing

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Aisha Johnson, MD
Department: Orthopedic Surgery
Visit Type: outpatient

CHIEF COMPLAINT:
3-month history of progressive right hip pain worse with weight-bearing

HISTORY OF PRESENT ILLNESS:
The patient returns with progressive right hip pain from known avascular necrosis. She describes the pain as a dull ache that worsens with weight-bearing activities and improves with rest. She denies any recent trauma or injury.

PAST MEDICAL HISTORY:
Active Problem List:
- Immunosuppressed state (diagnosed 2020-01-15)
- Secondary Hyperaldosteronism due to Dietary Sodium Restriction (diagnosed 2020-04-15)
- Systemic lupus erythematosus
- Hypertension
- Avascular necrosis of right femoral head
- History of primary central nervous system lymphoma
- Status post splenectomy
- Osteopenia

Her medical history is significant for systemic lupus erythematosus (SLE).

PAST SURGICAL HISTORY:
- Splenectomy (atraumatic rupture)
- Right hip core decompression
- Stereotactic brain biopsy

MEDICATIONS:
She takes prednisone 20 mg daily for the past 6 months.

Home Medications:
- Hydroxychloroquine 200 mg PO BID
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Cholecalciferol 2000 units PO daily
- Calcium carbonate 600 mg PO BID

ALLERGIES:
Hydralazine (drug-induced hemolysis), Sulfonamides (severe rash)

FAMILY HISTORY:
- Mother: Systemic lupus erythematosus
- Father: Hypertension, type 2 diabetes mellitus
- Sister: Hashimoto thyroiditis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative Assistant.

PHYSICAL EXAMINATION:
On physical examination, she has a reduced range of motion in the right hip, particularly with internal rotation. There is no erythema, warmth, or swelling over the joint.

IMAGING:
An X-ray of the hip shows no fractures or dislocations.
MRI of the hip reveals a crescent sign consistent with subchondral fracture and areas of bone marrow edema.

PLAN:
Given her history and physical examination findings, you order an MRI of the hip.

## Encounter 3: source 2022-01-15, practice 2023-01-27, ed, Emergency Medicine

Chief complaint: sudden onset of sharp, left upper quadrant pain radiating to left shoulder

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of sharp, left upper quadrant pain radiating to left shoulder

HISTORY OF PRESENT ILLNESS:
This 46-year-old female with systemic lupus erythematosus presents with acute abdominal pain. She describes the pain as constant and radiating to her left shoulder. She denies any trauma or previous similar episodes.

PAST MEDICAL HISTORY:
Active Problem List:
- Immunosuppressed state (diagnosed 2020-01-15)
- Secondary Hyperaldosteronism due to Dietary Sodium Restriction (diagnosed 2020-04-15)
- Avascular necrosis of femoral head (diagnosed 2021-01-15)
- Systemic lupus erythematosus
- Hypertension
- Avascular necrosis of right femoral head
- History of primary central nervous system lymphoma
- Status post splenectomy
- Osteopenia

PAST SURGICAL HISTORY:
- Splenectomy (atraumatic rupture)
- Right hip core decompression
- Stereotactic brain biopsy

MEDICATIONS:
Her medications include hydroxychloroquine and prednisone.

Home Medications:
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Cholecalciferol 2000 units PO daily
- Calcium carbonate 600 mg PO BID

ALLERGIES:
Hydralazine (drug-induced hemolysis), Sulfonamides (severe rash)

FAMILY HISTORY:
- Mother: Systemic lupus erythematosus
- Father: Hypertension, type 2 diabetes mellitus
- Sister: Hashimoto thyroiditis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative Assistant.

VITAL SIGNS:
On examination, her temperature is 37.8°C (100°F), blood pressure is 130/85 mmHg, pulse is 95/min, and respiratory rate is 18/min.

PHYSICAL EXAMINATION:
Abdominal examination reveals tenderness to palpation in the left upper quadrant with a positive splenic percussion sign.

LABORATORY DATA:
Laboratory tests reveal the following:

- Hemoglobin: 12.1 g/dL
- White blood cell count: 9,000/mm³
- Platelet count: 250,000/mm³
- INR: 1.0
- Lactate dehydrogenase (LDH): Elevated
- Peripheral blood smear: No schistocytes

IMAGING:
An abdominal ultrasound shows a wedge-shaped, hypoechoic area in the spleen.

## Encounter 4: source 2022-03-15, practice 2023-03-28, outpatient, Psychiatry

Chief complaint: husband reports patient has been acting strangely with irritability and insomnia

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. Lisa Patel, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Husband reports patient has been acting strangely with irritability and insomnia

HISTORY OF PRESENT ILLNESS:
This 46-year-old female with systemic lupus erythematosus, history of primary central nervous system lymphoma, avascular necrosis of the right femoral head, and status post splenectomy two months prior for atraumatic rupture, presents to the Psychiatry clinic with her husband, who reports that she has been "acting strangely" for the past two weeks. She was started on high-dose prednisone therapy for a severe lupus flare 1 month ago. The husband describes her as being increasingly irritable, having difficulty sleeping, and expressing grandiose ideas. She has also been spending large amounts of money online, which is uncharacteristic of her usual behavior. She denies any history of similar symptoms in the past.

PAST MEDICAL HISTORY:
Active Problem List:
- Immunosuppressed state (diagnosed 2020-01-15)
- Secondary Hyperaldosteronism due to Dietary Sodium Restriction (diagnosed 2020-04-15)
- Avascular necrosis of femoral head (diagnosed 2021-01-15)
- Atraumatic splenic rupture with hemoperitoneum (diagnosed 2022-01-15)
- Systemic lupus erythematosus
- Hypertension
- Avascular necrosis of right femoral head
- History of primary central nervous system lymphoma
- Status post splenectomy
- Osteopenia

PAST SURGICAL HISTORY:
- Splenectomy (atraumatic rupture)
- Right hip core decompression
- Stereotactic brain biopsy

MEDICATIONS:
Her current medications include prednisone, hydroxychloroquine, and lisinopril.

Home Medications:
- Amlodipine 5 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Cholecalciferol 2000 units PO daily
- Calcium carbonate 600 mg PO BID

ALLERGIES:
Hydralazine (drug-induced hemolysis), Sulfonamides (severe rash)

FAMILY HISTORY:
- Mother: Systemic lupus erythematosus
- Father: Hypertension, type 2 diabetes mellitus
- Sister: Hashimoto thyroiditis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative Assistant.

VITAL SIGNS:
Vital signs are within normal limits.

PHYSICAL EXAMINATION:
On examination, the patient is agitated, easily distracted, and her speech is pressured.

LABORATORY DATA:
Laboratory values are as follows:

- Hemoglobin: 13.2 g/dL
- White blood cell count: 7,800/mm^3
- Platelet count: 250,000/mm^3
- Serum creatinine: 0.9 mg/dL
- Urinalysis: Normal
- Thyroid-stimulating hormone (TSH): 2.5 mIU/L

## Encounter 5: source 2023-03-15, practice 2024-03-29, ed, Emergency Medicine

Chief complaint: new-onset seizures, headache, progressive confusion and visual disturbances

ENCOUNTER NOTE
Date: 2023-03-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
New-onset seizures, headache, progressive confusion and visual disturbances

HISTORY OF PRESENT ILLNESS:
The patient is a 47-year-old female with systemic lupus erythematosus on chronic immunosuppressive therapy, status post splenectomy, and prior primary central nervous system lymphoma. She presents with progressive confusion and visual disturbances over the past week.

PAST MEDICAL HISTORY:
Active Problem List:
- Immunosuppressed state (diagnosed 2020-01-15)
- Secondary Hyperaldosteronism due to Dietary Sodium Restriction (diagnosed 2020-04-15)
- Avascular necrosis of femoral head (diagnosed 2021-01-15)
- Atraumatic splenic rupture with hemoperitoneum (diagnosed 2022-01-15)
- Glucocorticoid-induced neuropsychiatric symptoms (diagnosed 2022-03-15)
- Systemic lupus erythematosus
- Hypertension
- Avascular necrosis of right femoral head
- History of primary central nervous system lymphoma
- Status post splenectomy
- Osteopenia

PAST SURGICAL HISTORY:
- Splenectomy (atraumatic rupture)
- Right hip core decompression
- Stereotactic brain biopsy

MEDICATIONS:
She takes hydroxychloroquine and low-dose prednisone.

Home Medications:
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Cholecalciferol 2000 units PO daily
- Calcium carbonate 600 mg PO BID

ALLERGIES:
Hydralazine (drug-induced hemolysis), Sulfonamides (severe rash)

FAMILY HISTORY:
- Mother: Systemic lupus erythematosus
- Father: Hypertension, type 2 diabetes mellitus
- Sister: Hashimoto thyroiditis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative Assistant.

PHYSICAL EXAMINATION:
Neurological examination shows left-sided weakness and difficulty with coordination. Fundoscopic examination is normal. There is no fever or neck stiffness.

LABORATORY DATA:
HIV testing is negative

IMAGING:
Brain MRI shows a single, well-defined ring-enhancing lesion in the right parietal lobe with minimal surrounding edema

OTHER STUDIES:
Lumbar puncture is unremarkable

## Encounter 6: source 2024-03-15, practice 2025-04-09, outpatient, Rheumatology

Chief complaint: increasing fatigue and painful malar rash following prednisone taper

ENCOUNTER NOTE
Date: 2024-03-15
Provider: Dr. Sarah Chen, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Increasing fatigue and painful malar rash following prednisone taper

HISTORY OF PRESENT ILLNESS:
The 48-year-old with systemic lupus erythematosus, history of primary CNS lymphoma, and status post splenectomy returns for follow-up. She presents to the clinic with a two-week history of increasing fatigue and a painful rash over her cheeks. She reports symptom onset shortly after finishing a 10-day course of oral prednisone for an SLE flare.

PAST MEDICAL HISTORY:
Active Problem List:
- Immunosuppressed state (diagnosed 2020-01-15)
- Secondary Hyperaldosteronism due to Dietary Sodium Restriction (diagnosed 2020-04-15)
- Avascular necrosis of femoral head (diagnosed 2021-01-15)
- Atraumatic splenic rupture with hemoperitoneum (diagnosed 2022-01-15)
- Glucocorticoid-induced neuropsychiatric symptoms (diagnosed 2022-03-15)
- Primary central nervous system lymphoma (diagnosed 2023-03-15)
- Systemic lupus erythematosus
- Hypertension
- Avascular necrosis of right femoral head
- Status post splenectomy
- Osteopenia

PAST SURGICAL HISTORY:
- Splenectomy (atraumatic rupture)
- Right hip core decompression
- Stereotactic brain biopsy

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg PO BID
- Prednisone 10 mg PO daily
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Cholecalciferol 2000 units PO daily
- Calcium carbonate 600 mg PO BID

ALLERGIES:
Hydralazine (drug-induced hemolysis), Sulfonamides (severe rash)

FAMILY HISTORY:
- Mother: Systemic lupus erythematosus
- Father: Hypertension, type 2 diabetes mellitus
- Sister: Hashimoto thyroiditis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative Assistant.

VITAL SIGNS:
Her temperature is 37.2°C (99°F), blood pressure is 130/85 mmHg, pulse is 78/min, and respiratory rate is 16/min.

PHYSICAL EXAMINATION:
Physical examination reveals malar erythema with mild edema.

LABORATORY DATA:
Laboratory studies are as follows:

- White blood cell count: 14,000/µL (Normal: 4,500-11,000/µL)
- Neutrophils: 80% (Normal: 40-60%)
- Lymphocytes: 10% (Normal: 20-40%)
- Hemoglobin: 12.2 g/dL (Normal: 12-16 g/dL)
- Platelets: 220,000/µL (Normal: 150,000-450,000/µL)
- Erythrocyte sedimentation rate (ESR): 50 mm/hr (Normal: 0-20 mm/hr)
- C-reactive protein (CRP): 4.5 mg/L (Normal: <3.0 mg/L)

## Encounter 7: source 2025-03-15, practice 2026-04-05, ed, Emergency Medicine

Chief complaint: generalized weakness, dark urine, and new petechial rash on lower extremities

ENCOUNTER NOTE
Date: 2025-03-15
Provider: Dr. Robert Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Generalized weakness, dark urine, and new petechial rash on lower extremities

HISTORY OF PRESENT ILLNESS:
This patient with SLE, prior splenectomy, and primary CNS lymphoma presents to the emergency department with a 3-day history of generalized weakness, dark urine, and a new petechial rash on her lower extremities.

PAST MEDICAL HISTORY:
Active Problem List:
- Immunosuppressed state (diagnosed 2020-01-15)
- Secondary Hyperaldosteronism due to Dietary Sodium Restriction (diagnosed 2020-04-15)
- Avascular necrosis of femoral head (diagnosed 2021-01-15)
- Atraumatic splenic rupture with hemoperitoneum (diagnosed 2022-01-15)
- Glucocorticoid-induced neuropsychiatric symptoms (diagnosed 2022-03-15)
- Primary central nervous system lymphoma (diagnosed 2023-03-15)
- Leukocytosis (diagnosed 2024-03-15)
- Systemic lupus erythematosus
- Hypertension
- Avascular necrosis of right femoral head
- Status post splenectomy
- Osteopenia

She has a past medical history of systemic lupus erythematosus.

PAST SURGICAL HISTORY:
- Splenectomy (atraumatic rupture)
- Right hip core decompression
- Stereotactic brain biopsy

MEDICATIONS:
She was recently started on hydralazine for hypertension.

Home Medications:
- Hydroxychloroquine 200 mg PO BID
- Prednisone 10 mg PO daily
- Lisinopril 20 mg PO daily
- Amlodipine 5 mg PO daily
- Hydrochlorothiazide 25 mg PO daily
- Cholecalciferol 2000 units PO daily
- Calcium carbonate 600 mg PO BID

ALLERGIES:
Hydralazine (drug-induced hemolysis), Sulfonamides (severe rash)

FAMILY HISTORY:
- Mother: Systemic lupus erythematosus
- Father: Hypertension, type 2 diabetes mellitus
- Sister: Hashimoto thyroiditis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Administrative Assistant.

VITAL SIGNS:
Her temperature is 37.8°C (100°F), blood pressure is 150/90 mmHg, pulse is 110/min, and respiratory rate is 20/min.

PHYSICAL EXAMINATION:
Physical examination reveals jaundice and splenomegaly.

LABORATORY DATA:
Hemoglobin: 8 g/dL (Normal: 12-16 g/dL)
- Platelet count: 40,000/µL (Normal: 150,000-400,000/µL)
- Lactate dehydrogenase (LDH): 700 U/L (Normal: 140-280 U/L)
- Total bilirubin: 3.2 mg/dL (Normal: 0.3-1.2 mg/dL)
- Direct bilirubin: 0.8 mg/dL (Normal: 0.0-0.3 mg/dL)
- Haptoglobin: <10 mg/dL (Normal: 30-200 mg/dL)
- Peripheral smear: Schistocytes present
