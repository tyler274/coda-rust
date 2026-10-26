# synthetic_hospital patient 2104

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 44 at first encounter, sex M, Asian, UnitedHealthcare PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-05-04 (Wed) | outpatient | Otolaryngology |
| 1 | 2020-03-15 | 2022-06-16 (Thu) | inpatient | Hematology/Oncology |
| 2 | 2023-01-15 | 2025-05-05 (Mon) | outpatient | Rheumatology |
| 3 | 2023-07-15 | 2025-10-27 (Mon) | outpatient | Pulmonology |
| 4 | 2024-01-15 | 2026-05-05 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "UnitedHealthcare PPO",
 "occupation": "Financial analyst",
 "alcohol_use": "None",
 "family_history": [
  "Father with nasopharyngeal carcinoma",
  "Mother with type 2 diabetes mellitus",
  "Sister with autoimmune thyroid disease"
 ],
 "race_ethnicity": "Asian",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [
  "Bilateral tympanostomy tube placement (age 8)"
 ],
 "chronic_conditions": [
  "Chronic otitis media",
  "Chronic kidney disease"
 ],
 "age_at_first_encounter": 44
}
```

Primary diagnoses: ["Granulomatosis with polyangiitis", "Tumor lysis syndrome", "Nasopharyngeal Carcinoma"]
Comorbidities: ["Chronic Sinusitis", "Acute kidney injury", "Recurrent otitis media"]

## Encounter 0: source 2020-01-15, practice 2022-05-04, outpatient, Otolaryngology

Chief complaint: 3-month history of nasal congestion, intermittent epistaxis, persistent headache, and ear fullness

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Otolaryngology
Visit Type: outpatient

CHIEF COMPLAINT:
3-month history of nasal congestion, intermittent epistaxis, persistent headache, and ear fullness

HISTORY OF PRESENT ILLNESS:
He also complains of a persistent headache and a sensation of fullness in his ears.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic otitis media
- Chronic kidney disease

He has a history of chronic otitis media, which has not improved with standard antibiotic treatment.

PAST SURGICAL HISTORY:
- Bilateral tympanostomy tube placement (age 8)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with nasopharyngeal carcinoma
- Mother with type 2 diabetes mellitus
- Sister with autoimmune thyroid disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Financial analyst.

PHYSICAL EXAMINATION:
On physical examination, you note a firm, non-tender mass in the left side of his nasopharynx and bilateral cervical lymphadenopathy. Cranial nerve examination reveals a reduced gag reflex and decreased sensation in the distribution of the trigeminal nerve.

LABORATORY DATA:
An Epstein-Barr virus (EBV) serology test shows elevated viral capsid antigen IgG and early antigen IgG.

## Encounter 1: source 2020-03-15, practice 2022-06-16, inpatient, Hematology/Oncology

Chief complaint: Admitted for first cycle of chemotherapy for nasopharyngeal carcinoma, develops severe nausea, flank pain, and decreased urine output within 24 hours

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Michael Rodriguez, MD
Department: Hematology/Oncology
Visit Type: inpatient

CHIEF COMPLAINT:
Admitted for first cycle of chemotherapy for nasopharyngeal carcinoma, develops severe nausea, flank pain, and decreased urine output within 24 hours

HISTORY OF PRESENT ILLNESS:
He returns for chemotherapy for his nasopharyngeal carcinoma. Chemotherapy is initiated, but within 24 hours, he develops severe nausea, flank pain, and decreased urine output.

PAST MEDICAL HISTORY:
Active Problem List:
- Nasopharyngeal Carcinoma (diagnosed 2020-01-15)
- Chronic otitis media
- Chronic kidney disease

PAST SURGICAL HISTORY:
- Bilateral tympanostomy tube placement (age 8)

MEDICATIONS:
He takes no regular medications

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with nasopharyngeal carcinoma
- Mother with type 2 diabetes mellitus
- Sister with autoimmune thyroid disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Financial analyst.

LABORATORY DATA:
Baseline labs are notable for WBC 85,000/µL, potassium 4.8 mEq/L, calcium 9.2 mg/dL, phosphate 4.6 mg/dL, uric acid 8.5 mg/dL, and creatinine 0.9 mg/dL.
Repeat labs show potassium 6.7 mEq/L, phosphate 8.2 mg/dL, calcium 6.3 mg/dL, uric acid 15.4 mg/dL, and creatinine 2.3 mg/dL.

OTHER STUDIES:
Urinalysis reveals uric acid crystals.

## Encounter 2: source 2023-01-15, practice 2025-05-05, outpatient, Rheumatology

Chief complaint: Persistent fatigue, nasal congestion, joint discomfort, and rust-colored urine over the past two days

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Aisha Patel, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent fatigue, nasal congestion, joint discomfort, and rust-colored urine over the past two days

HISTORY OF PRESENT ILLNESS:
The patient with chronic otitis media, chronic kidney disease, and prior nasopharyngeal carcinoma complicated by tumor lysis syndrome returns for evaluation. He reports feeling “off” for several weeks with recurrent sinus pressure and nasal crusting despite multiple antibiotic courses. Recently, he developed a dry cough and exertional dyspnea with rust-colored urine noted over the past two days.

PAST MEDICAL HISTORY:
Active Problem List:
- Nasopharyngeal Carcinoma (diagnosed 2020-01-15)
- Tumor lysis syndrome (diagnosed 2020-03-15)
- Chronic otitis media
- Chronic kidney disease

PAST SURGICAL HISTORY:
- Bilateral tympanostomy tube placement (age 8)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with nasopharyngeal carcinoma
- Mother with type 2 diabetes mellitus
- Sister with autoimmune thyroid disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Financial analyst.

PHYSICAL EXAMINATION:
On physical exam, there is tenderness over the maxillary sinuses, trace pedal edema, and faint bibasilar crackles.

LABORATORY DATA:
Urinalysis shows +3 blood, proteinuria, and dysmorphic RBCs. Serum creatinine is 2.4 mg/dL. Laboratory testing is positive for cytoplasmic anti-neutrophil cytoplasmic antibodies.

IMAGING:
Chest imaging reveals multiple bilateral pulmonary nodules, some with central clearing.

## Encounter 3: source 2023-07-15, practice 2025-10-27, outpatient, Pulmonology

Chief complaint: 2-month history of persistent sinusitis unresponsive to antibiotics, productive cough with occasional hemoptysis, and 10-pound unintentional weight loss

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. James Wilson, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
2-month history of persistent sinusitis unresponsive to antibiotics, productive cough with occasional hemoptysis, and 10-pound unintentional weight loss

HISTORY OF PRESENT ILLNESS:
The patient returns six months after his rheumatology evaluation for granulomatosis with polyangiitis, with history of nasopharyngeal carcinoma and tumor lysis syndrome. He presents with two-month history of persistent sinusitis unresponsive to antibiotics, productive cough with occasional hemoptysis, and ten-pound unintentional weight loss. He also reports generalized malaise and arthralgias.

PAST MEDICAL HISTORY:
Active Problem List:
- Nasopharyngeal Carcinoma (diagnosed 2020-01-15)
- Tumor lysis syndrome (diagnosed 2020-03-15)
- Granulomatosis with polyangiitis (diagnosed 2023-01-15)
- Chronic otitis media
- Chronic kidney disease

His past medical history is significant for chronic otitis media and a recent diagnosis of unexplained acute kidney injury.

PAST SURGICAL HISTORY:
- Bilateral tympanostomy tube placement (age 8)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with nasopharyngeal carcinoma
- Mother with type 2 diabetes mellitus
- Sister with autoimmune thyroid disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Financial analyst.

PHYSICAL EXAMINATION:
On physical examination, you note nasal crusting and tenderness over his maxillary sinuses. Auscultation of the lungs reveals scattered crackles, but no wheezes.

LABORATORY DATA:
Urinalysis shows red blood cell casts. Laboratory results are as follows: - Creatinine: 2.8 mg/dL (elevated) - Erythrocyte sedimentation rate (ESR): 80 mm/hr (elevated) - C-ANCA: Positive - Urine protein: 2+ (elevated)

## Encounter 4: source 2024-01-15, practice 2026-05-05, ed, Emergency Medicine

Chief complaint: Worsening nasal congestion, occasional nosebleeds, and acute worsening shortness of breath on exertion with occasional hemoptysis

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Emily Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening nasal congestion, occasional nosebleeds, and acute worsening shortness of breath on exertion with occasional hemoptysis

HISTORY OF PRESENT ILLNESS:
This 48-year-old male with chronic otitis media, chronic kidney disease, prior nasopharyngeal carcinoma and tumor lysis syndrome, and recent sinonasal and pulmonary evaluations, presents with a 4-month history of worsening nasal congestion, occasional nosebleeds, and fatigue. He also reports unintentional weight loss and intermittent fevers during this period. Over the past 2 weeks, he has developed increasing shortness of breath on exertion and occasional hemoptysis.

PAST MEDICAL HISTORY:
Active Problem List:
- Nasopharyngeal Carcinoma (diagnosed 2020-01-15)
- Tumor lysis syndrome (diagnosed 2020-03-15)
- Granulomatosis with polyangiitis (diagnosed 2023-01-15)
- Chronic otitis media
- Chronic kidney disease

His medical history is otherwise unremarkable.

PAST SURGICAL HISTORY:
- Bilateral tympanostomy tube placement (age 8)

MEDICATIONS:
He takes no regular medications

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with nasopharyngeal carcinoma
- Mother with type 2 diabetes mellitus
- Sister with autoimmune thyroid disease

SOCIAL HISTORY:
He does not smoke, drink alcohol, or use illicit drugs.

PHYSICAL EXAMINATION:
On examination, he appears pale with crusting and ulceration of the nasal mucosa. There is bilateral rhonchi on lung auscultation, and the skin over both shins shows several palpable purpuric lesions.

LABORATORY DATA:
Laboratory studies reveal hemoglobin 9.8 g/dL, creatinine 2.1 mg/dL, and urinalysis showing red blood cell casts.

IMAGING:
Chest x-ray demonstrates multiple bilateral pulmonary nodules with some areas of cavitation.

ASSESSMENT AND PLAN:
These autoantibodies are most likely directed against antigens present in
