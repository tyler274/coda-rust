# synthetic_hospital patient 2295

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 67 at first encounter, sex M, Non-Hispanic White, Medicare with supplemental commercial insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-11-07 (Fri) | outpatient | Hematology/Oncology |
| 1 | 2020-04-15 | 2026-01-30 (Fri) | inpatient | Neurology |
| 2 | 2020-07-15 | 2026-05-04 (Mon) | icu | Pulmonary & Critical Care |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicare with supplemental commercial insurance",
 "occupation": "Retired construction foreman",
 "alcohol_use": "Occasional social use, 1-2 drinks per month",
 "family_history": [
  "Father: Deceased at age 71 from lung cancer, heavy smoker",
  "Mother: History of ischemic stroke at age 74",
  "Older brother: Chronic obstructive pulmonary disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 42 pack-years, quit 18 months ago",
 "home_medications": [
  {
   "dose": "81 mg oral daily",
   "name": "Aspirin"
  },
  {
   "dose": "40 mg oral nightly",
   "name": "Atorvastatin"
  },
  {
   "dose": "2 mg oral twice daily (therapeutic drug monitoring required)",
   "name": "Tacrolimus"
  },
  {
   "dose": "100/25 mcg inhaled once daily",
   "name": "Fluticasone furoate/vilanterol"
  },
  {
   "dose": "18 mcg inhaled daily",
   "name": "Tiotropium bromide"
  },
  {
   "dose": "5 mg oral daily",
   "name": "Amlodipine"
  },
  {
   "dose": "160/800 mg oral three times weekly (Pneumocystis prophylaxis)",
   "name": "Trimethoprim-sulfamethoxazole"
  }
 ],
 "surgical_history": [
  "Implantable central venous port placement (right chest) for chemotherapy administration",
  "Emergency endotracheal intubation with mechanical ventilation"
 ],
 "chronic_conditions": [
  "Chronic obstructive pulmonary disease (COPD), Global Initiative for Chronic Obstructive Lung Disease (GOLD) Stage III",
  "Acute myeloid leukemia, newly diagnosed, undergoing induction chemotherapy protocol with planned allogeneic stem cell transplantation",
  "Ischemic cerebrovascular disease with residual dysphagia",
  "Hypertension",
  "Dyslipidemia"
 ],
 "age_at_first_encounter": 67
}
```

Primary diagnoses: ["Severe Clostridioides difficile infection without toxic megacolon", "Ventilator-associated pneumonia", "Calcineurin inhibitor nephrotoxicity"]
Comorbidities: ["Dehydration", "Acute ischemic stroke"]

## Encounter 0: source 2020-01-15, practice 2025-11-07, outpatient, Hematology/Oncology

Chief complaint: Follow-up 10 days after starting treatment for newly diagnosed AML with low energy, dizziness, oral mucositis, and hair thinning.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Patel, MD
Department: Hematology/Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up 10 days after starting treatment for newly diagnosed AML with low energy, dizziness, oral mucositis, and hair thinning.

HISTORY OF PRESENT ILLNESS:
He was recently hospitalized for unintentional weight loss and fatigue. Today he reports ongoing low energy, mild dizziness when standing, difficulty eating due to a sore mouth, and some thinning of his hair that he recently noticed in the mirror. His appetite is reduced, and he has had intermittent loose stools over the past several days.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (COPD), Global Initiative for Chronic Obstructive Lung Disease (GOLD) Stage III
- Acute myeloid leukemia, newly diagnosed, undergoing induction chemotherapy protocol with planned allogeneic stem cell transplantation
- Ischemic cerebrovascular disease with residual dysphagia
- Hypertension
- Dyslipidemia

PAST SURGICAL HISTORY:
- Implantable central venous port placement (right chest) for chemotherapy administration
- Emergency endotracheal intubation with mechanical ventilation

MEDICATIONS:
He was discharged on a multidrug regimen administered through a port.

Home Medications:
- Aspirin 81 mg oral daily
- Atorvastatin 40 mg oral nightly
- Tacrolimus 2 mg oral twice daily (therapeutic drug monitoring required)
- Fluticasone furoate/vilanterol 100/25 mcg inhaled once daily
- Tiotropium bromide 18 mcg inhaled daily
- Amlodipine 5 mg oral daily
- Trimethoprim-sulfamethoxazole 160/800 mg oral three times weekly (Pneumocystis prophylaxis)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Deceased at age 71 from lung cancer, heavy smoker
- Mother: History of ischemic stroke at age 74
- Older brother: Chronic obstructive pulmonary disease

SOCIAL HISTORY:
Smoking: Former smoker, 42 pack-years, quit 18 months ago. Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Retired construction foreman.

VITAL SIGNS:
Vital signs are within normal limits.

PHYSICAL EXAMINATION:
Physical examination shows pale conjunctivae and dry oral mucosa with scattered erosions along the gingiva. No lymphadenopathy is present. Rectal exam is heme-negative.

LABORATORY DATA:
Laboratory testing reveals:
Hemoglobin 8.9 g/dL
Mean corpuscular volume 91 fL
Platelets 110,000/mm³
White blood cells 2,300/mm³
Absolute neutrophil count 850/mm³

IMAGING:
Imaging revealed multiple hepatic lesions and a mass in the sigmoid colon

PATHOLOGY:
A biopsy confirmed a diagnosis of adenocarcinoma

## Encounter 1: source 2020-04-15, practice 2026-01-30, inpatient, Neurology

Chief complaint: Worsening diarrhea during hospitalization for acute ischemic stroke.

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. James O'Brien, MD
Department: Neurology
Visit Type: inpatient

CHIEF COMPLAINT:
Worsening diarrhea during hospitalization for acute ischemic stroke.

HISTORY OF PRESENT ILLNESS:
This 67-year-old male with GOLD Stage III COPD, ischemic cerebrovascular disease with residual dysphagia, and newly diagnosed acute myeloid leukemia undergoing induction chemotherapy was admitted nine days ago for an acute ischemic stroke. He presents with worsening diarrhea during this hospitalization, having been evaluated approximately three months prior for calcineurin inhibitor nephrotoxicity. He has been receiving supportive care, including aspirin, atorvastatin, and physical therapy. Due to concern for aspiration pneumonia early in his admission, he was empirically started on intravenous ceftriaxone and oral azithromycin, which were discontinued two days ago. Over the past 48 hours, nursing staff report that the patient has been having frequent loose, watery stools — approximately 5 to 6 episodes per day — which are nonbloody and foul-smelling. He has developed mild diffuse abdominal discomfort, reduced appetite, and occasional dizziness when ambulating.

PAST MEDICAL HISTORY:
Active Problem List:
- Calcineurin inhibitor nephrotoxicity (diagnosed 2020-01-15)
- Chronic obstructive pulmonary disease (COPD), Global Initiative for Chronic Obstructive Lung Disease (GOLD) Stage III
- Acute myeloid leukemia, newly diagnosed, undergoing induction chemotherapy protocol with planned allogeneic stem cell transplantation
- Ischemic cerebrovascular disease with residual dysphagia
- Hypertension
- Dyslipidemia

PAST SURGICAL HISTORY:
- Implantable central venous port placement (right chest) for chemotherapy administration
- Emergency endotracheal intubation with mechanical ventilation

MEDICATIONS:
He takes aspirin, atorvastatin, intravenous ceftriaxone, and oral azithromycin.

Home Medications:
- Tacrolimus 2 mg oral twice daily (therapeutic drug monitoring required)
- Fluticasone furoate/vilanterol 100/25 mcg inhaled once daily
- Tiotropium bromide 18 mcg inhaled daily
- Amlodipine 5 mg oral daily
- Trimethoprim-sulfamethoxazole 160/800 mg oral three times weekly (Pneumocystis prophylaxis)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Deceased at age 71 from lung cancer, heavy smoker
- Mother: History of ischemic stroke at age 74
- Older brother: Chronic obstructive pulmonary disease

SOCIAL HISTORY:
Smoking: Former smoker, 42 pack-years, quit 18 months ago. Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Retired construction foreman.

VITAL SIGNS:
He is afebrile, hemodynamically stable

PHYSICAL EXAMINATION:
Fully oriented, though he appears tired. On physical exam, his mucous membranes are dry, and the abdomen is soft and nondistended with mild generalized tenderness but no rebound or guarding. Bowel sounds are present and hyperactive.

LABORATORY DATA:
Labs reveal a leukocyte count of 14,600/mm³.

OTHER STUDIES:
Stool studies are obtained.

PLAN:
He is placed on contact isolation, and IV fluids are started.

## Encounter 2: source 2020-07-15, practice 2026-05-04, icu, Pulmonary & Critical Care

Chief complaint: New fever of 38.7°C with increased purulent secretions and increased suctioning requirements five days after intubation.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Maria Gonzalez, MD
Department: Pulmonary & Critical Care
Visit Type: icu

CHIEF COMPLAINT:
New fever of 38.7°C with increased purulent secretions and increased suctioning requirements five days after intubation.

HISTORY OF PRESENT ILLNESS:
The patient with COPD and AML, recently managed for nephrotoxicity and stroke, is admitted to the ICU. He was intubated five days ago and has been sedated since then. His oxygen requirements have increased, and he is now on 60% FiO2 to maintain an oxygen saturation of 92%.

PAST MEDICAL HISTORY:
Active Problem List:
- Calcineurin inhibitor nephrotoxicity (diagnosed 2020-01-15)
- Severe Clostridioides difficile infection without toxic megacolon (diagnosed 2020-04-15)
- Chronic obstructive pulmonary disease (COPD), Global Initiative for Chronic Obstructive Lung Disease (GOLD) Stage III
- Acute myeloid leukemia, newly diagnosed, undergoing induction chemotherapy protocol with planned allogeneic stem cell transplantation
- Ischemic cerebrovascular disease with residual dysphagia
- Hypertension
- Dyslipidemia

PAST SURGICAL HISTORY:
- Implantable central venous port placement (right chest) for chemotherapy administration
- Emergency endotracheal intubation with mechanical ventilation

MEDICATIONS:
Home Medications:
- Aspirin 81 mg oral daily
- Atorvastatin 40 mg oral nightly
- Tacrolimus 2 mg oral twice daily (therapeutic drug monitoring required)
- Fluticasone furoate/vilanterol 100/25 mcg inhaled once daily
- Tiotropium bromide 18 mcg inhaled daily
- Amlodipine 5 mg oral daily
- Trimethoprim-sulfamethoxazole 160/800 mg oral three times weekly (Pneumocystis prophylaxis)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Deceased at age 71 from lung cancer, heavy smoker
- Mother: History of ischemic stroke at age 74
- Older brother: Chronic obstructive pulmonary disease

SOCIAL HISTORY:
Smoking: Former smoker, 42 pack-years, quit 18 months ago. Alcohol: Occasional social use, 1-2 drinks per month. Occupation: Retired construction foreman.

PHYSICAL EXAMINATION:
The head of the bed is noted to be flat during the examination.

LABORATORY DATA:
His white blood cell count is elevated at 16,000 cells/µL

IMAGING:
A chest X-ray shows new infiltrates in the right lower lobe.
