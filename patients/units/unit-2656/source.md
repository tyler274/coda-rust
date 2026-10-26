# synthetic_hospital patient 2656

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 30 at first encounter, sex M, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-09-02 (Tue) | ed | Emergency Medicine |
| 1 | 2020-09-15 | 2026-05-12 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Unemployed (previous maintenance worker)",
 "alcohol_use": "None",
 "family_history": [
  "Mother: Major depressive disorder",
  "Father: Hypertension",
  "No family history of bleeding disorders or sickle cell disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "50 mg PO at bedtime for sleep",
   "name": "Trazodone"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Major depressive disorder, recurrent, moderate",
  "Insomnia",
  "Medication non-adherence"
 ],
 "age_at_first_encounter": 30
}
```

Primary diagnoses: ["Non-ischemic (arterial) priapism", "Rotenone poisoning"]
Comorbidities: ["Major depressive disorder"]

## Encounter 0: source 2020-01-15, practice 2025-09-02, ed, Emergency Medicine

Chief complaint: Painful erection that has persisted for 6 hours

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Painful erection that has persisted for 6 hours

HISTORY OF PRESENT ILLNESS:
He denies any history of trauma or illicit drug use.

PAST MEDICAL HISTORY:
Active Problem List:
- Major depressive disorder, recurrent, moderate
- Insomnia
- Medication non-adherence

His past medical history is significant for depression. He does not have a history of sickle cell disease or hematologic abnormalities.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He takes trazodone

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Major depressive disorder
- Father: Hypertension
- No family history of bleeding disorders or sickle cell disease

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Unemployed (previous maintenance worker).

VITAL SIGNS:
His vital signs are within normal limits.

PHYSICAL EXAMINATION:
On examination, the penis is fully erect, tender to palpation, and there is no evidence of trauma or injury. The glans penis and corpus spongiosum are not engorged.

LABORATORY DATA:
A complete blood count and metabolic panel are unremarkable. Urinalysis is normal.

## Encounter 1: source 2020-09-15, practice 2026-05-12, ed, Emergency Medicine

Chief complaint: Spontaneous gum bleeding and extensive bruising on arms and legs

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Spontaneous gum bleeding and extensive bruising on arms and legs

HISTORY OF PRESENT ILLNESS:
The patient with a history of major depressive disorder and medication non-adherence returns eight months after his prior ED visit for priapism, presenting with spontaneous gum bleeding and extensive bruising. The roommate reports that the patient has been increasingly reclusive over the past few weeks and may have ingested something harmful, although he denies any specific knowledge. The patient admits to feeling depressed. Further questioning reveals that the patient had been attempting to poison rats in the apartment using over-the-counter rodenticides several days prior.

PAST MEDICAL HISTORY:
Active Problem List:
- Non-ischemic (arterial) priapism (diagnosed 2020-01-15)
- Major depressive disorder, recurrent, moderate
- Insomnia
- Medication non-adherence

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Denies taking any prescription drugs or supplements.

Home Medications:
- Trazodone 50 mg PO at bedtime for sleep

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Major depressive disorder
- Father: Hypertension
- No family history of bleeding disorders or sickle cell disease

SOCIAL HISTORY:
He does not consume alcohol

VITAL SIGNS:
His vitals are within normal limits.

PHYSICAL EXAMINATION:
Physical examination reveals several ecchymoses on the upper extremities, oozing from the gums, and no petechiae. There is no hepatosplenomegaly.

LABORATORY DATA:
A serum toxicology screen is negative for opioids, benzodiazepines, and acetaminophen. Laboratory studies are shown: Platelets 180,000/μL Bleeding time 5 minutes Prothrombin time (PT) 24 seconds Partial thromboplastin time (PTT) 61 seconds
