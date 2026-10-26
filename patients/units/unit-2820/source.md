# synthetic_hospital patient 2820

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 41 at first encounter, sex F, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-06-14 (Fri) | outpatient | Dermatology |
| 1 | 2021-07-15 | 2025-12-19 (Fri) | outpatient | Occupational Health |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Penicillin (cutaneous rash)",
  "Dust mites"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Biomedical Research Scientist (PhD)",
 "alcohol_use": "Occasional (1-2 drinks per week)",
 "family_history": [
  "Mother: Hypertension",
  "Father: Type 2 diabetes mellitus",
  "Maternal grandmother: Breast cancer (age 68)",
  "No family history of melanoma or skin cancer"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "10 mg PO daily",
   "name": "Cetirizine"
  },
  {
   "dose": "50 mg PO PRN migraine",
   "name": "Sumatriptan"
  }
 ],
 "surgical_history": [
  "Tonsillectomy (age 7)",
  "Wisdom teeth extraction (age 18)"
 ],
 "chronic_conditions": [
  "Seasonal allergic rhinitis",
  "Migraine without aura"
 ],
 "age_at_first_encounter": 41
}
```

Primary diagnoses: ["Malignant melanoma"]
Comorbidities: []

## Encounter 0: source 2020-01-15, practice 2024-06-14, outpatient, Dermatology

Chief complaint: Changing mole on back that has increased in size over 6 months and recently started to itch

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Changing mole on back that has increased in size over 6 months and recently started to itch

HISTORY OF PRESENT ILLNESS:
She reports that the mole has increased in size over the past 6 months and has recently started to itch.

PAST MEDICAL HISTORY:
Active Problem List:
- Seasonal allergic rhinitis
- Migraine without aura

She denies any personal or family history of skin cancer.

PAST SURGICAL HISTORY:
- Tonsillectomy (age 7)
- Wisdom teeth extraction (age 18)

MEDICATIONS:
Home Medications:
- Cetirizine 10 mg PO daily
- Sumatriptan 50 mg PO PRN migraine

ALLERGIES:
Penicillin (cutaneous rash), Dust mites

FAMILY HISTORY:
She denies any personal or family history of skin cancer.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1-2 drinks per week). Occupation: Biomedical Research Scientist (PhD).

PHYSICAL EXAMINATION:
On examination, the mole measures 1.2 cm in diameter, has irregular borders, varying colors including shades of brown and black, and a slightly raised nodular area. The rest of the skin examination is unremarkable. There are no palpable lymph nodes.

ASSESSMENT AND PLAN:
The patient is concerned about the possibility of skin cancer.

## Encounter 1: source 2021-07-15, practice 2025-12-19, outpatient, Occupational Health

Chief complaint: Occupational health consultation for research protocol medical clearance and systematic health assessment

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. James Okonkwo, MD
Department: Occupational Health
Visit Type: outpatient

CHIEF COMPLAINT:
Occupational health consultation for research protocol medical clearance and systematic health assessment

PAST MEDICAL HISTORY:
Active Problem List:
- Malignant melanoma (diagnosed 2020-01-15)
- Seasonal allergic rhinitis
- Migraine without aura

PAST SURGICAL HISTORY:
- Tonsillectomy (age 7)
- Wisdom teeth extraction (age 18)

MEDICATIONS:
Home Medications:
- Cetirizine 10 mg PO daily
- Sumatriptan 50 mg PO PRN migraine

ALLERGIES:
Penicillin (cutaneous rash), Dust mites

FAMILY HISTORY:
- Mother: Hypertension
- Father: Type 2 diabetes mellitus
- Maternal grandmother: Breast cancer (age 68)
- No family history of melanoma or skin cancer

SOCIAL HISTORY:
A 38-year-old female researcher is preparing a comprehensive report on the effectiveness of different immunotherapies for advanced melanoma. She gathers all available randomized controlled trials (RCTs), cohort studies, and case-control studies published over the last decade, focusing on overall survival and adverse events. She and her team critically appraise the quality of each study, extract relevant data, and synthesize the evidence to create a summary that can guide clinical practice. They perform quantitative statistical analyses, including meta-analyses of subgroups of RCTs where the pooled data leads to statistically significant results not seen in some individual trials. The researcher notes that individual RCTs sometimes lack the statistical power to detect differences, but when data are combined across multiple studies, effect sizes become clearer. Their final report includes forest plots, funnel plots for publication bias, and a discussion of heterogeneity. This summary is then submitted for peer review as a reference for oncologists and policymakers.
