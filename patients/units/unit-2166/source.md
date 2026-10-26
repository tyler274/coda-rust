# synthetic_hospital patient 2166

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 41 at first encounter, sex M, Non-Hispanic White, Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-12-06 (Mon) | outpatient | Otolaryngology |
| 1 | 2024-01-15 | 2025-11-30 (Sun) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Blue Cross Blue Shield PPO",
 "occupation": "Software Developer",
 "alcohol_use": "Occasional (\u22641 drink per month)",
 "family_history": [
  "Mother: Generalized anxiety disorder",
  "Maternal grandfather: Meniere disease",
  "Father: Hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "50 mg orally daily",
   "name": "Sertraline"
  },
  {
   "dose": "25 mg orally every 6 hours as needed for anxiety",
   "name": "Hydroxyzine"
  }
 ],
 "surgical_history": [
  "Wisdom teeth extraction, age 18"
 ],
 "chronic_conditions": [
  "Generalized anxiety disorder",
  "History of benzodiazepine use disorder (sustained remission since age 38)"
 ],
 "age_at_first_encounter": 41
}
```

Primary diagnoses: ["Benzodiazepine Withdrawal", "Meniere disease"]
Comorbidities: ["Generalized Anxiety Disorder"]

## Encounter 0: source 2020-01-15, practice 2021-12-06, outpatient, Otolaryngology

Chief complaint: Intermittent episodes of vertigo lasting approximately one hour with associated ear fullness and muffled hearing

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Patel, MD
Department: Otolaryngology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent episodes of vertigo lasting approximately one hour with associated ear fullness and muffled hearing

HISTORY OF PRESENT ILLNESS:
A 45-year-old man comes to the clinic after several months of intermittent episodes in which he feels as though the room is spinning. The spells last for about an hour, occur a few times per month, and are sometimes preceded by a sense of fullness in one ear. During these episodes he notices muffled sounds and low-pitched ringing in the same ear, and he occasionally feels unsteady when walking afterward. Between attacks, he feels well and has no persistent imbalance, but he has become increasingly anxious about when the next episode will occur.

PAST MEDICAL HISTORY:
Active Problem List:
- Generalized anxiety disorder
- History of benzodiazepine use disorder (sustained remission since age 38)

PAST SURGICAL HISTORY:
- Wisdom teeth extraction, age 18

MEDICATIONS:
Home Medications:
- Sertraline 50 mg orally daily
- Hydroxyzine 25 mg orally every 6 hours as needed for anxiety

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Generalized anxiety disorder
- Maternal grandfather: Meniere disease
- Father: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (≤1 drink per month). Occupation: Software Developer.

VITAL SIGNS:
Vital signs are normal

PHYSICAL EXAMINATION:
On examination today, otoscopic findings are unremarkable. Neurologic examination shows normal coordination and gait, and tuning fork testing reveals air conduction greater than bone conduction bilaterally without lateralization.

## Encounter 1: source 2024-01-15, practice 2025-11-30, ed, Emergency Medicine

Chief complaint: Acute agitation, tremors, visual hallucinations of insects, and severe insomnia following abrupt medication cessation

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute agitation, tremors, visual hallucinations of insects, and severe insomnia following abrupt medication cessation

HISTORY OF PRESENT ILLNESS:
The patient is a 45-year-old male with generalized anxiety disorder, benzodiazepine use disorder in sustained remission since age 38, and Meniere disease, maintained on sertraline and hydroxyzine, who presents to the emergency department with a 2-day history of increasing agitation, tremors, and difficulty sleeping. He reports seeing "bugs crawling on the walls" and is convinced that the staff is conspiring against him. He admits to abruptly stopping his medication 3 days ago because he felt it was no longer needed.

PAST MEDICAL HISTORY:
Active Problem List:
- Meniere disease (diagnosed 2020-01-15)
- Generalized anxiety disorder
- History of benzodiazepine use disorder (sustained remission since age 38)

His past medical history is significant for generalized anxiety disorder, for which he has been taking alprazolam for several years.

PAST SURGICAL HISTORY:
- Wisdom teeth extraction, age 18

MEDICATIONS:
He takes alprazolam for several years.

Home Medications:
- Sertraline 50 mg orally daily
- Hydroxyzine 25 mg orally every 6 hours as needed for anxiety

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Generalized anxiety disorder
- Maternal grandfather: Meniere disease
- Father: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (≤1 drink per month). Occupation: Software Developer.

VITAL SIGNS:
Heart rate of 110 bpm and blood pressure of 155/95 mmHg

PHYSICAL EXAMINATION:
On examination, the patient is diaphoretic, with a heart rate of 110 bpm and blood pressure of 155/95 mmHg. Neurological examination reveals bilateral hand tremors and hyperreflexia.

LABORATORY DATA:
Laboratory studies show:
- Sodium: 140 mEq/L
- Potassium: 4.1 mEq/L
- Chloride: 100 mEq/L
- Bicarbonate: 24 mEq/L
- Blood urea nitrogen: 18 mg/dL
- Creatinine: 1.0 mg/dL
- Glucose: 98 mg/dL
- Calcium: 9.2 mg/dL
- Magnesium: 2.0 mg/dL
- Phosphate: 3.5 mg/dL
- Complete blood count: within normal limits
- Urine toxicology screen: Negative
