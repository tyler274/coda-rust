# synthetic_hospital patient 2117

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 46 at first encounter, sex M, Non-Hispanic White, Private PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-09-15 (Thu) | outpatient | Internal Medicine |
| 1 | 2021-07-15 | 2024-03-28 (Thu) | outpatient | Gastroenterology |
| 2 | 2022-07-15 | 2025-03-31 (Mon) | outpatient | Pulmonology |
| 3 | 2023-07-15 | 2026-03-16 (Mon) | outpatient | Internal Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Private PPO",
 "occupation": "Software engineer",
 "alcohol_use": "Occasional social use (1-2 drinks per week)",
 "family_history": [
  "Father with Barrett esophagus",
  "Mother with autoimmune hypothyroidism"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "10 mg PO daily",
   "name": "Prednisone"
  },
  {
   "dose": "1.2 g PO twice daily",
   "name": "Mesalamine"
  },
  {
   "dose": "50 mg PO daily",
   "name": "Azathioprine"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Omeprazole"
  },
  {
   "dose": "500 mg chewable tablet PRN for heartburn",
   "name": "Calcium carbonate (Tums)"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Ulcerative colitis",
  "Gastroesophageal reflux disease",
  "Barrett esophagus",
  "Azathioprine-induced interstitial lung disease"
 ],
 "age_at_first_encounter": 46
}
```

Primary diagnoses: ["Interstitial lung disease", "Exogenous glucocorticoid use", "Gastroesophageal reflux disease", "Barrett esophagus"]
Comorbidities: ["Family history of diabetes mellitus", "Gastroesophageal reflux disease"]

## Encounter 0: source 2020-01-15, practice 2022-09-15, outpatient, Internal Medicine

Chief complaint: Recurring uncomfortable sensation in the chest, particularly after eating or lying down, with sour taste in mouth disturbing sleep

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Recurring uncomfortable sensation in the chest, particularly after eating or lying down, with sour taste in mouth disturbing sleep

HISTORY OF PRESENT ILLNESS:
He presents to the clinic with a 3-month history of a recurring uncomfortable sensation in the chest, particularly after eating or lying down. He describes a sour taste in his mouth that seems to rise from his upper abdomen intermittently, often disturbing his sleep. He denies any weight loss, dysphagia, or vomiting of blood.
A trial of an oral medication provided significant relief within one week.

PAST MEDICAL HISTORY:
Active Problem List:
- Ulcerative colitis
- Gastroesophageal reflux disease
- Barrett esophagus
- Azathioprine-induced interstitial lung disease

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He is not on any chronic medications

Home Medications:
- Prednisone 10 mg PO daily
- Mesalamine 1.2 g PO twice daily
- Azathioprine 50 mg PO daily
- Omeprazole 40 mg PO daily
- Calcium carbonate (Tums) 500 mg chewable tablet PRN for heartburn

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with Barrett esophagus
- Mother with autoimmune hypothyroidism

SOCIAL HISTORY:
His diet includes frequent late-night meals, and he reports consuming multiple cups of coffee daily. He works long shifts and often skips breakfast.
He does not smoke, but occasionally drinks alcohol socially.

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

OTHER STUDIES:
Cardiac and pulmonary evaluations were previously normal.

PLAN:
The physician explains that the drug works by targeting a key transporter involved in gastric acid secretion.

## Encounter 1: source 2021-07-15, practice 2024-03-28, outpatient, Gastroenterology

Chief complaint: Worsening heartburn and difficulty swallowing over the past few months with burning chest pain after meals

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. James Rodriguez, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening heartburn and difficulty swallowing over the past few months with burning chest pain after meals

HISTORY OF PRESENT ILLNESS:
The patient returns to the Gastroenterology clinic for follow-up of gastroesophageal reflux disease previously evaluated in January 2020. He presents with worsening heartburn and difficulty swallowing over the past few months. He reports burning chest pain after meals and occasional regurgitation of sour-tasting fluid. He has no history of weight loss, hematemesis, or melena. He takes over-the-counter antacids with minimal relief.

PAST MEDICAL HISTORY:
Active Problem List:
- Gastroesophageal reflux disease (diagnosed 2020-01-15)
- Ulcerative colitis
- Barrett esophagus
- Azathioprine-induced interstitial lung disease

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Prednisone 10 mg PO daily
- Mesalamine 1.2 g PO twice daily
- Azathioprine 50 mg PO daily
- Omeprazole 40 mg PO daily
- Calcium carbonate (Tums) 500 mg chewable tablet PRN for heartburn

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with Barrett esophagus
- Mother with autoimmune hypothyroidism

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per week). Occupation: Software engineer.

PATHOLOGY:
, and biopsy reveals non-ciliated, mucin-producing columnar epithelial cells replacing the normal squamous epithelium in the distal esophagus.

OTHER STUDIES:
Upper endoscopy is performed

## Encounter 2: source 2022-07-15, practice 2025-03-31, outpatient, Pulmonology

Chief complaint: Progressive shortness of breath and fatigue over the past 6 months with dry persistent cough

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Aisha Patel, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive shortness of breath and fatigue over the past 6 months with dry persistent cough

HISTORY OF PRESENT ILLNESS:
The patient with GERD and Barrett esophagus presents with progressive shortness of breath and fatigue over the past 6 months. He also reports a dry, persistent cough that occasionally worsens at night.

PAST MEDICAL HISTORY:
Active Problem List:
- Gastroesophageal reflux disease (diagnosed 2020-01-15)
- Barrett esophagus (diagnosed 2021-07-15)
- Ulcerative colitis
- Azathioprine-induced interstitial lung disease

His medical history is notable for episodic heartburn.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He takes he occasionally uses over-the-counter antacids.

Home Medications:
- Prednisone 10 mg PO daily
- Mesalamine 1.2 g PO twice daily
- Azathioprine 50 mg PO daily
- Omeprazole 40 mg PO daily
- Calcium carbonate (Tums) 500 mg chewable tablet PRN for heartburn

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
His father had rheumatoid arthritis, and his sister was recently diagnosed with an autoimmune thyroid disorder.

SOCIAL HISTORY:
He works as a software engineer and has no history of smoking or significant environmental exposures.

REVIEW OF SYSTEMS:
He denies fever, chills, or chest pain.

PHYSICAL EXAMINATION:
On examination, he has clubbing of the fingers and mild end-inspiratory crackles at the lung bases. No joint swelling or rashes are observed.

IMAGING:
Chest X-ray shows bilateral reticular opacities in the lower lobes. High-resolution CT of the chest reveals basilar-predominant ground-glass opacities without honeycombing.

PATHOLOGY:
Lung biopsy shows uniform thickening of the alveolar septa with dense collagen deposition and smooth muscle hyperplasia. There is no evidence of granulomas or fibroblastic foci.

## Encounter 3: source 2023-07-15, practice 2026-03-16, outpatient, Internal Medicine

Chief complaint: New skin changes including gradual thinning and easy bruising without trauma

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
New skin changes including gradual thinning and easy bruising without trauma

HISTORY OF PRESENT ILLNESS:
The patient with ulcerative colitis returns for follow-up. He reports gradual thinning of his skin and easy bruising, without trauma.

PAST MEDICAL HISTORY:
Active Problem List:
- Gastroesophageal reflux disease (diagnosed 2020-01-15)
- Barrett esophagus (diagnosed 2021-07-15)
- Interstitial lung disease (diagnosed 2022-07-15)
- Ulcerative colitis
- Azathioprine-induced interstitial lung disease

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He takes prednisone, mesalamine, and azathioprine.

Home Medications:
- Omeprazole 40 mg PO daily
- Calcium carbonate (Tums) 500 mg chewable tablet PRN for heartburn

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
His father was diagnosed with type 2 diabetes mellitus, and his mother was recently diagnosed with cancer.

SOCIAL HISTORY:
He drinks alcohol occasionally and does not smoke.

VITAL SIGNS:
Vital signs are normal.

PHYSICAL EXAMINATION:
Physical examination findings are shown in the image below.

LABORATORY DATA:
Laboratory studies show low serum cortisol and suppressed ACTH.
