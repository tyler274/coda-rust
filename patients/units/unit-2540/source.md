# synthetic_hospital patient 2540

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 35 at first encounter, sex M, Non-Hispanic White, Uninsured.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-04-20 (Sat) | inpatient | Infectious Disease |
| 1 | 2020-09-15 | 2024-12-12 (Thu) | ed | Emergency Medicine |
| 2 | 2021-09-15 | 2025-12-13 (Sat) | icu | Critical Care Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Uninsured",
 "occupation": "Construction worker (historic building renovation)",
 "alcohol_use": "Social use (1-2 drinks monthly)",
 "family_history": [
  "Father: Coronary artery disease",
  "Mother: Type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker (quit 2 years ago)",
 "home_medications": [],
 "surgical_history": [
  "Appendectomy, age 12"
 ],
 "chronic_conditions": [
  "Human Immunodeficiency Virus (HIV) infection, diagnosed age 30",
  "Acquired Immunodeficiency Syndrome (AIDS) with history of opportunistic infection"
 ],
 "age_at_first_encounter": 35
}
```

Primary diagnoses: ["Progressive disseminated histoplasmosis", "Carbon monoxide poisoning", "Cyanide toxicity"]
Comorbidities: ["Hypoxemia", "Smoke inhalation injury", "Sinus tachycardia secondary to burns", "Tachypnea"]

## Encounter 0: source 2020-01-15, practice 2024-04-20, inpatient, Infectious Disease

Chief complaint: 3-week history of fever, weight loss, and night sweats with nonproductive cough and dyspnea on exertion

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Infectious Disease
Visit Type: inpatient

CHIEF COMPLAINT:
3-week history of fever, weight loss, and night sweats with nonproductive cough and dyspnea on exertion

HISTORY OF PRESENT ILLNESS:
He reports a nonproductive cough and dyspnea on exertion.

PAST MEDICAL HISTORY:
Active Problem List:
- Human Immunodeficiency Virus (HIV) infection, diagnosed age 30
- Acquired Immunodeficiency Syndrome (AIDS) with history of opportunistic infection

PAST SURGICAL HISTORY:
- Appendectomy, age 12

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Coronary artery disease
- Mother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker (quit 2 years ago). Alcohol: Social use (1-2 drinks monthly). Occupation: Construction worker (historic building renovation).

PHYSICAL EXAMINATION:
On physical examination, he is tachypneic with a respiratory rate of 28 breaths per minute, and his oxygen saturation is 92% on room air. Lung auscultation reveals diffuse rales.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 10.2 g/dL
- White blood cell count: 6,000 cells/µL
- Platelets: 250,000 cells/µL
- CD4+ count: 50 cells/µL
- HIV viral load: 120,000 copies/mL
- Serum lactate dehydrogenase (LDH): Elevated
- Urine Histoplasma antigen: Positive

IMAGING:
A chest radiograph shows diffuse reticulonodular infiltrates.

PLAN:
The patient is started on intravenous amphotericin B. After 10 days of treatment, he shows clinical improvement with resolution of fever and improved respiratory symptoms.

## Encounter 1: source 2020-09-15, practice 2024-12-12, ed, Emergency Medicine

Chief complaint: Rescued from house fire, agitated and confused with progressive somnolence since extrication

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Rescued from house fire, agitated and confused with progressive somnolence since extrication

HISTORY OF PRESENT ILLNESS:
The patient is a 35-year-old male with HIV infection, AIDS, and history of progressive disseminated histoplasmosis, who presents following rescue from a house fire. He is agitated and confused. His wife, who accompanies him, reports that he was trapped in the house for approximately 20 minutes before firefighters could reach him. She also mentions that he has been increasingly somnolent since the incident.

PAST MEDICAL HISTORY:
Active Problem List:
- Progressive disseminated histoplasmosis (diagnosed 2020-01-15)
- Human Immunodeficiency Virus (HIV) infection, diagnosed age 30
- Acquired Immunodeficiency Syndrome (AIDS) with history of opportunistic infection

PAST SURGICAL HISTORY:
- Appendectomy, age 12

MEDICATIONS:
He takes no medications

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Coronary artery disease
- Mother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker (quit 2 years ago). Alcohol: Social use (1-2 drinks monthly). Occupation: Construction worker (historic building renovation).

VITAL SIGNS:
On physical examination, his vital signs are as follows: temperature 37.2°C (99.0°F), blood pressure 135/85 mmHg, pulse 110/min, respiratory rate 24/min, and oxygen saturation 92% on room air.

PHYSICAL EXAMINATION:
His skin is soot-covered, but there are no burns or other injuries noted. Auscultation of the lungs reveals mild wheezing bilaterally. Neurological examination is significant for confusion and disorientation to time and place.

LABORATORY DATA:
The patient's carboxyhemoglobin level is pending.

## Encounter 2: source 2021-09-15, practice 2025-12-13, icu, Critical Care Medicine

Chief complaint: Headache, dizziness, and disorientation after house fire rescue with rapid deterioration despite high-flow oxygen therapy

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. Priya Sharma, MD
Department: Critical Care Medicine
Visit Type: icu

CHIEF COMPLAINT:
Headache, dizziness, and disorientation after house fire rescue with rapid deterioration despite high-flow oxygen therapy

HISTORY OF PRESENT ILLNESS:
This 36-year-old with HIV/AIDS and opportunistic infection history presents one year after house fire with carbon monoxide poisoning. He is brought to the emergency department after being rescued from a house fire. He is disoriented and complains of headache and dizziness. The patient's condition deteriorates rapidly with worsening mental status and hypoxia despite high-flow oxygen therapy.

PAST MEDICAL HISTORY:
Active Problem List:
- Progressive disseminated histoplasmosis (diagnosed 2020-01-15)
- Carbon monoxide poisoning (diagnosed 2020-09-15)
- Human Immunodeficiency Virus (HIV) infection, diagnosed age 30
- Acquired Immunodeficiency Syndrome (AIDS) with history of opportunistic infection

PAST SURGICAL HISTORY:
- Appendectomy, age 12

ALLERGIES:
He has no known drug allergies

FAMILY HISTORY:
- Father: Coronary artery disease
- Mother: Type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Former smoker (quit 2 years ago). Alcohol: Social use (1-2 drinks monthly). Occupation: Construction worker (historic building renovation).

VITAL SIGNS:
Blood pressure 110/70 mmHg, heart rate 105 beats/min, respiratory rate 24 breaths/min, and oxygen saturation 92% on room air

PHYSICAL EXAMINATION:
He has soot around his nostrils and in his oropharynx. His lung examination reveals mild wheezing bilaterally.

LABORATORY DATA:
Arterial blood gas shows a PaO2 of 60 mmHg. Carboxyhemoglobin level is 5%. A cyanide level is pending.
