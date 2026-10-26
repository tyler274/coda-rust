# synthetic_hospital patient 2593

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 34 at first encounter, sex M, White, non-Hispanic, Aetna PPO (employer-sponsored).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-10-08 (Wed) | ed | Emergency Medicine |
| 1 | 2020-07-15 | 2026-04-08 (Wed) | inpatient | Orthopedic Surgery |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Aetna PPO (employer-sponsored)",
 "occupation": "Field service technician",
 "alcohol_use": "Social (2-3 drinks on weekends)",
 "family_history": [
  "Father: Hypertension (diagnosed age 55)",
  "Mother: Hypothyroidism"
 ],
 "race_ethnicity": "White, non-Hispanic",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [],
 "chronic_conditions": [],
 "age_at_first_encounter": 34
}
```

Primary diagnoses: ["Traumatic rupture of spleen with hemoperitoneum", "Ketamine anesthesia"]
Comorbidities: ["Traumatic hemorrhagic shock with acute coagulopathy"]

## Encounter 0: source 2020-01-15, practice 2025-10-08, ed, Emergency Medicine

Chief complaint: Severe left-sided abdominal pain, difficulty breathing, and left shoulder pain following motor vehicle collision

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe left-sided abdominal pain, difficulty breathing, and left shoulder pain following motor vehicle collision

HISTORY OF PRESENT ILLNESS:
He presents to the emergency department following a motor vehicle collision. He complains of severe left-sided abdominal pain and has noted some difficulty breathing. He also mentions a peculiar pain in his left shoulder.

PAST MEDICAL HISTORY:
His past medical history is unremarkable

MEDICATIONS:
He takes no medications

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension (diagnosed age 55)
- Mother: Hypothyroidism

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (2-3 drinks on weekends). Occupation: Field service technician.

VITAL SIGNS:
On examination, his vital signs are: blood pressure 100/60 mmHg, heart rate 110 beats per minute, respiratory rate 22 breaths per minute, and oxygen saturation 94% on room air.

PHYSICAL EXAMINATION:
His abdomen is tender to palpation in the left upper quadrant with guarding. Chest examination reveals decreased breath sounds at the left lower lung field.

IMAGING:
Chest X-ray shows fractures of the 9th and 10th ribs on the left side. Focused Assessment with Sonography for Trauma (FAST) exam shows free fluid in the abdomen.

## Encounter 1: source 2020-07-15, practice 2026-04-08, inpatient, Orthopedic Surgery

Chief complaint: High-speed car accident with multiple lacerations and fractured tibia

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. James Park, MD
Department: Orthopedic Surgery
Visit Type: inpatient

CHIEF COMPLAINT:
High-speed car accident with multiple lacerations and fractured tibia

HISTORY OF PRESENT ILLNESS:
The patient, who sustained traumatic splenic rupture in a motor vehicle collision six months prior, returns following a second high-speed accident. He is brought to the emergency department after a high-speed car accident in which he sustained multiple lacerations and a fractured tibia. He is agitated and in significant pain but remains hemodynamically unstable despite fluid resuscitation. After the surgery, he describes vivid dreams and a sensation that his body was floating above the operating table.

PAST MEDICAL HISTORY:
Active Problem List:
- Traumatic rupture of spleen with hemoperitoneum (diagnosed 2020-01-15)

MEDICATIONS:
The anesthesiologist selects an intravenous agent that provides profound analgesia and amnesia while maintaining spontaneous ventilation and airway reflexes.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hypertension (diagnosed age 55)
- Mother: Hypothyroidism

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (2-3 drinks on weekends). Occupation: Field service technician.

VITAL SIGNS:
Shortly after administration, the patient develops mild hypertension and tachycardia, with preserved oxygen saturation.

PHYSICAL EXAMINATION:
His eyes remain open with horizontal nystagmus, and he appears to be staring into space without reacting to his surroundings.

PLAN:
The orthopedic surgery team decides to proceed with urgent debridement of contaminated wounds under general anesthesia.
