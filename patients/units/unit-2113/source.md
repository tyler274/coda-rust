# synthetic_hospital patient 2113

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 43 at first encounter, sex M, Southeast Asian, Commercial insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-07-10 (Mon) | outpatient | Oral and Maxillofacial Surgery |
| 1 | 2020-11-15 | 2024-05-27 (Mon) | outpatient | Dermatology |
| 2 | 2022-05-15 | 2025-11-14 (Fri) | inpatient | Internal Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial insurance",
 "occupation": "Lifeguard",
 "alcohol_use": "None",
 "family_history": [
  "Father: Type 2 diabetes mellitus",
  "Mother: History of pulmonary tuberculosis (treated)",
  "Paternal uncle: Melanoma"
 ],
 "race_ethnicity": "Southeast Asian",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1000 mg twice daily",
   "name": "Metformin"
  },
  {
   "dose": "24 units subcutaneously at bedtime",
   "name": "Insulin glargine"
  }
 ],
 "surgical_history": [
  "Dental extraction (recent)"
 ],
 "chronic_conditions": [
  "Type 2 Diabetes Mellitus (poorly controlled)"
 ],
 "age_at_first_encounter": 43
}
```

Primary diagnoses: ["Cervicofacial actinomycosis", "Tuberculosis", "Nodular melanoma"]
Comorbidities: ["Poorly controlled diabetes mellitus", "Delayed management of life-threatening respiratory failure"]

## Encounter 0: source 2020-01-15, practice 2023-07-10, outpatient, Oral and Maxillofacial Surgery

Chief complaint: 3-month history of a painless, slowly enlarging mass on the right side of his jaw with occasional drainage of cheesy material

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Leila Farhad, MD
Department: Oral and Maxillofacial Surgery
Visit Type: outpatient

CHIEF COMPLAINT:
3-month history of a painless, slowly enlarging mass on the right side of his jaw with occasional drainage of cheesy material

HISTORY OF PRESENT ILLNESS:
He reports occasional drainage of a "cheesy" material from a small opening over the mass.

PAST MEDICAL HISTORY:
Active Problem List:
- Type 2 Diabetes Mellitus (poorly controlled)

His medical history is significant for poorly controlled diabetes mellitus and a recent dental extraction due to a tooth abscess.

PAST SURGICAL HISTORY:
- Dental extraction (recent)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg twice daily
- Insulin glargine 24 units subcutaneously at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus
- Mother: History of pulmonary tuberculosis (treated)
- Paternal uncle: Melanoma

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Lifeguard.

PHYSICAL EXAMINATION:
On examination, there is a 4 cm, firm, non-tender mass on the right mandible with a small sinus tract exuding purulent material. No lymphadenopathy is noted. Oral examination reveals poor dental hygiene with multiple carious teeth.

LABORATORY DATA:
A sample of the drained fluid is sent for laboratory analysis, which reveals the presence of sulfur granules.

## Encounter 1: source 2020-11-15, practice 2024-05-27, outpatient, Dermatology

Chief complaint: Evaluation of a growing, pruritic lesion on the left upper back first noticed several months ago

ENCOUNTER NOTE
Date: 2020-11-15
Provider: Dr. James Chen, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Evaluation of a growing, pruritic lesion on the left upper back first noticed several months ago

HISTORY OF PRESENT ILLNESS:
The patient with poorly controlled diabetes and prior cervicofacial actinomycosis presents with a back lesion that he noticed several months ago. He reports the lesion has increased in size and has recently started to itch, denying trauma, pain, or bleeding from the area.

PAST MEDICAL HISTORY:
Active Problem List:
- Cervicofacial actinomycosis (diagnosed 2020-01-15)
- Type 2 Diabetes Mellitus (poorly controlled)

PAST SURGICAL HISTORY:
- Dental extraction (recent)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg twice daily
- Insulin glargine 24 units subcutaneously at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Type 2 diabetes mellitus
- Mother: History of pulmonary tuberculosis (treated)
- Paternal uncle: Melanoma

SOCIAL HISTORY:
He works as a lifeguard and has had multiple sunburns in the past.

PHYSICAL EXAMINATION:
The image of the lesion is shown below. No lymphadenopathy is appreciated.

## Encounter 2: source 2022-05-15, practice 2025-11-14, inpatient, Internal Medicine

Chief complaint: 3-month history of progressive fatigue, productive cough with hemoptysis, intermittent fevers, night sweats, and 8kg unintentional weight loss

ENCOUNTER NOTE
Date: 2022-05-15
Provider: Dr. Aisha Washington, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
3-month history of progressive fatigue, productive cough with hemoptysis, intermittent fevers, night sweats, and 8kg unintentional weight loss

HISTORY OF PRESENT ILLNESS:
This 45-year-old male with poorly controlled Type 2 Diabetes Mellitus, prior cervicofacial actinomycosis (2020), and nodular melanoma (2020) presents with a three-month history of progressive symptoms. He reports progressive fatigue, productive cough with hemoptysis, intermittent fevers, night sweats, and an unintentional 8-kg (17.6-lb) weight loss over this period.

PAST MEDICAL HISTORY:
Active Problem List:
- Cervicofacial actinomycosis (diagnosed 2020-01-15)
- Nodular melanoma (diagnosed 2020-11-15)
- Type 2 Diabetes Mellitus (poorly controlled)

His medical history is significant for poorly controlled type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Dental extraction (recent)

MEDICATIONS:
Home Medications:
- Metformin 1000 mg twice daily
- Insulin glargine 24 units subcutaneously at bedtime

ALLERGIES:
NKDA

FAMILY HISTORY:
Recalls that several relatives overseas had “lung disease” with prolonged cough.

SOCIAL HISTORY:
He immigrated from rural Southeast Asia 5 years ago and recently returned from visiting family there 4 months ago. He denies alcohol, illicit drug use, or recent sick contacts in the United States

VITAL SIGNS:
On examination, his temperature is 37.8°C (100°F), blood pressure is 118/70 mm Hg, pulse is 92/min, and respiratory rate is 18/min.

PHYSICAL EXAMINATION:
Pulmonary auscultation reveals scattered crackles and bronchial breath sounds over the right upper lung field. There is mild digital clubbing but no cyanosis.

LABORATORY DATA:
Three sputum samples are obtained on consecutive mornings. Ziehl–Neelsen staining shows slender, red, beaded rods consistent with acid-fast bacilli. The sputum is inoculated onto Lowenstein–Jensen culture medium for definitive diagnosis.

IMAGING:
A chest x-ray demonstrates a thick-walled cavitary lesion in the right upper lobe with adjacent patchy consolidations.
