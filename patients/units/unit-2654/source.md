# synthetic_hospital patient 2654

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 33 at first encounter, sex M, Non-Hispanic White, Commercial PPO (Blue Cross).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-07-28 (Mon) | ed | Emergency Medicine |
| 1 | 2020-09-15 | 2026-04-07 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO (Blue Cross)",
 "occupation": "Geographic Information Systems (GIS) Specialist",
 "alcohol_use": "Social (2-3 drinks per week)",
 "family_history": [
  "Maternal migraines",
  "Father with essential hypertension (controlled)",
  "No family history of premature stroke or clotting disorders"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [
  "Wisdom teeth extraction (age 21)"
 ],
 "chronic_conditions": [],
 "age_at_first_encounter": 33
}
```

Primary diagnoses: ["High-altitude pulmonary edema", "Decompression Sickness"]
Comorbidities: ["Exposure to high altitude"]

## Encounter 0: source 2020-01-15, practice 2025-07-28, ed, Emergency Medicine

Chief complaint: Progressive shortness of breath and dry cough for 2 days following rapid ascent to 10,000 feet during hiking trip

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive shortness of breath and dry cough for 2 days following rapid ascent to 10,000 feet during hiking trip

HISTORY OF PRESENT ILLNESS:
He reports that he recently traveled to a mountainous region at an altitude of 10,000 feet for a hiking trip. His symptoms started shortly after his ascent. He denies any fevers, chills, or sick contacts.

PAST MEDICAL HISTORY:
His past medical history is unremarkable

PAST SURGICAL HISTORY:
- Wisdom teeth extraction (age 21)

MEDICATIONS:
He takes no medications

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal migraines
- Father with essential hypertension (controlled)
- No family history of premature stroke or clotting disorders

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (2-3 drinks per week). Occupation: Geographic Information Systems (GIS) Specialist.

VITAL SIGNS:
On examination, his temperature is 37.2°C (99°F), heart rate is 110 bpm, respiratory rate is 26 breaths/min, blood pressure is 130/80 mmHg, and oxygen saturation is 85% on room air

PHYSICAL EXAMINATION:
Lung auscultation reveals bilateral crackles, but no wheezes or rhonchi. The remainder of the examination is unremarkable.

LABORATORY DATA:
Laboratory tests reveal:

- White blood cell count: 9,000/mm3
- Hemoglobin: 15 g/dL
- Platelet count: 250,000/mm3
- Arterial blood gas on room air: pH 7.45, PaCO2 30 mmHg, PaO2 55 mmHg

IMAGING:
Chest radiography shows patchy infiltrates in both lung fields without lobar consolidation.

PLAN:
The patient is placed on 2 liters of supplemental oxygen via nasal cannula, and his oxygen saturation improves to 92%.

## Encounter 1: source 2020-09-15, practice 2026-04-07, ed, Emergency Medicine

Chief complaint: Sudden onset of confusion, difficulty speaking, and left-sided weakness immediately following rapid ascent from scuba dive

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of confusion, difficulty speaking, and left-sided weakness immediately following rapid ascent from scuba dive

HISTORY OF PRESENT ILLNESS:
This 33-year-old male with a history of high-altitude pulmonary edema following rapid ascent in January presents following a scuba dive. He reports that he was scuba diving at a depth of 30 meters and surfaced rapidly because he was running out of air. His dive buddy noticed that he was not acting normally once they were back on the boat.

PAST MEDICAL HISTORY:
Active Problem List:
- High-altitude pulmonary edema (diagnosed 2020-01-15)

PAST SURGICAL HISTORY:
- Wisdom teeth extraction (age 21)

MEDICATIONS:
He takes no medications

ALLERGIES:
NKDA

FAMILY HISTORY:
- Maternal migraines
- Father with essential hypertension (controlled)
- No family history of premature stroke or clotting disorders

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (2-3 drinks per week). Occupation: Geographic Information Systems (GIS) Specialist.

VITAL SIGNS:
Temperature is 37.0°C (98.6°F), blood pressure is 130/85 mmHg, pulse is 100 beats/min, and respiratory rate is 22 breaths/min

PHYSICAL EXAMINATION:
He appears anxious and has cyanotic lips. Neurological examination reveals dysarthria, left-sided hemiparesis, and gait ataxia. Chest auscultation reveals bilateral crackles.

LABORATORY DATA:
An arterial blood gas shows:

- pH: 7.45
- PaCO2: 35 mmHg
- PaO2: 60 mmHg
- HCO3-: 24 mEq/L
