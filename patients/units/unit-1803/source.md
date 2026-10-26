# synthetic_hospital patient 1803

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 25 at first encounter, sex F, Non-Hispanic White, University student health insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-04-17 (Mon) | outpatient | Psychiatry |
| 1 | 2020-07-15 | 2023-10-12 (Thu) | outpatient | Internal Medicine |
| 2 | 2021-07-15 | 2024-10-14 (Mon) | outpatient | Ophthalmology |
| 3 | 2022-07-15 | 2025-10-16 (Thu) | ed | Emergency Medicine |
| 4 | 2023-01-15 | 2026-04-22 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Tree pollen",
  "Grass pollen",
  "Dust mites"
 ],
 "insurance": "University student health insurance",
 "occupation": "Graduate student (Music Performance)",
 "alcohol_use": "Social (1-2 drinks per month)",
 "family_history": [
  "Mother: Asthma",
  "Father: Seasonal allergic rhinitis",
  "No family history of substance use disorder"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "90 mcg, 2 inhalations as needed for asthma symptoms",
   "name": "Albuterol HFA inhaler"
  },
  {
   "dose": "10 mg orally as needed for allergic rhinitis",
   "name": "Cetirizine"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Selective IgA deficiency",
  "Persistent asthma",
  "Seasonal allergic rhinitis",
  "Social anxiety disorder, performance type"
 ],
 "age_at_first_encounter": 25
}
```

Primary diagnoses: ["Selective IgA deficiency", "Allergic conjunctivitis", "Anticholinergic toxicity", "Status asthmaticus with impending respiratory failure", "Social Anxiety Disorder (Performance Type)"]
Comorbidities: ["Allergic rhinitis", "Asthma"]

## Encounter 0: source 2020-01-15, practice 2023-04-17, outpatient, Psychiatry

Chief complaint: Severe nervousness, palpitations, and tremulousness occurring exclusively before musical performances

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Severe nervousness, palpitations, and tremulousness occurring exclusively before musical performances

HISTORY OF PRESENT ILLNESS:
She presents to the clinic with complaints of severe nervousness, palpitations, and tremulousness that occur exclusively before her performances. She reports that these symptoms have been progressively worsening over the past year, and they are now affecting her ability to play. She denies experiencing these symptoms in social settings or during rehearsals. She is seeking treatment to help manage her symptoms during performances.

PAST MEDICAL HISTORY:
Active Problem List:
- Selective IgA deficiency
- Persistent asthma
- Seasonal allergic rhinitis
- Social anxiety disorder, performance type

Her medical history is significant for asthma,.

MEDICATIONS:
She takes for which she uses an albuterol inhaler as needed.

Home Medications:
- Albuterol HFA inhaler 90 mcg, 2 inhalations as needed for asthma symptoms
- Cetirizine 10 mg orally as needed for allergic rhinitis

ALLERGIES:
Tree pollen, Grass pollen, Dust mites

FAMILY HISTORY:
- Mother: Asthma
- Father: Seasonal allergic rhinitis
- No family history of substance use disorder

SOCIAL HISTORY:
She has no personal or family history of substance use disorder.

VITAL SIGNS:
Her vital signs are within normal limits.

PHYSICAL EXAMINATION:
Her physical examination is unremarkable,

## Encounter 1: source 2020-07-15, practice 2023-10-12, outpatient, Internal Medicine

Chief complaint: Facial pressure, nasal congestion, and thick nasal discharge persisting for one week following a cold

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Facial pressure, nasal congestion, and thick nasal discharge persisting for one week following a cold

HISTORY OF PRESENT ILLNESS:
The patient with Selective IgA deficiency and seasonal allergic rhinitis, evaluated in Psychiatry earlier this year, presents with facial pressure, nasal congestion, and thick nasal discharge persisting for the past week. She notes this occurs several times yearly and often follows a cold.

PAST MEDICAL HISTORY:
Active Problem List:
- Social Anxiety Disorder (Performance Type) (diagnosed 2020-01-15)
- Selective IgA deficiency
- Persistent asthma
- Seasonal allergic rhinitis

She has a history of recurrent pneumonias during adolescence, seasonal allergies, and childhood asthma.

MEDICATIONS:
Home Medications:
- Albuterol HFA inhaler 90 mcg, 2 inhalations as needed for asthma symptoms
- Cetirizine 10 mg orally as needed for allergic rhinitis

ALLERGIES:
Tree pollen, Grass pollen, Dust mites

FAMILY HISTORY:
- Mother: Asthma
- Father: Seasonal allergic rhinitis
- No family history of substance use disorder

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Graduate student (Music Performance).

PHYSICAL EXAMINATION:
On exam, there is maxillary sinus tenderness and mild expiratory wheezing.

## Encounter 2: source 2021-07-15, practice 2024-10-14, outpatient, Ophthalmology

Chief complaint: Two-week history of bilateral eye redness, intense itching, and tearing during spring season

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Priya Patel, MD
Department: Ophthalmology
Visit Type: outpatient

CHIEF COMPLAINT:
Two-week history of bilateral eye redness, intense itching, and tearing during spring season

HISTORY OF PRESENT ILLNESS:
Returning for follow-up of allergic rhinitis, she presents to the clinic in the spring. She reports that her symptoms seem to worsen when she is outdoors.

PAST MEDICAL HISTORY:
Active Problem List:
- Social Anxiety Disorder (Performance Type) (diagnosed 2020-01-15)
- Selective IgA deficiency (diagnosed 2020-07-15)
- Persistent asthma
- Seasonal allergic rhinitis

She has a past medical history of seasonal allergic rhinitis.

MEDICATIONS:
She takes for which she takes an over-the-counter cetirizine as needed.

Home Medications:
- Albuterol HFA inhaler 90 mcg, 2 inhalations as needed for asthma symptoms

ALLERGIES:
Tree pollen, Grass pollen, Dust mites

FAMILY HISTORY:
- Mother: Asthma
- Father: Seasonal allergic rhinitis
- No family history of substance use disorder

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Graduate student (Music Performance).

PHYSICAL EXAMINATION:
On physical examination, her visual acuity is normal, and there is no purulent discharge. However, you note bilateral conjunctival hyperemia and eyelid edema. She also has pale, boggy nasal mucosa with clear rhinorrhea.

## Encounter 3: source 2022-07-15, practice 2025-10-16, ed, Emergency Medicine

Chief complaint: Severe difficulty breathing and wheezing unresponsive to frequent albuterol use

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe difficulty breathing and wheezing unresponsive to frequent albuterol use

HISTORY OF PRESENT ILLNESS:
The patient with selective IgA deficiency and asthma presents with dyspnea. She reports that her symptoms began earlier in the day and have progressively worsened despite using her albuterol inhaler every 2 hours.

PAST MEDICAL HISTORY:
Active Problem List:
- Social Anxiety Disorder (Performance Type) (diagnosed 2020-01-15)
- Selective IgA deficiency (diagnosed 2020-07-15)
- Allergic conjunctivitis (diagnosed 2021-07-15)
- Persistent asthma
- Seasonal allergic rhinitis

MEDICATIONS:
Home Medications:
- Albuterol HFA inhaler 90 mcg, 2 inhalations as needed for asthma symptoms
- Cetirizine 10 mg orally as needed for allergic rhinitis

ALLERGIES:
Tree pollen, Grass pollen, Dust mites

FAMILY HISTORY:
- Mother: Asthma
- Father: Seasonal allergic rhinitis
- No family history of substance use disorder

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Graduate student (Music Performance).

VITAL SIGNS:
Her respiratory rate is 30 breaths per minute, heart rate is 110 beats per minute, blood pressure is 135/85 mmHg, and oxygen saturation is 88% on room air.

PHYSICAL EXAMINATION:
On examination, she is using accessory muscles to breathe and has a prolonged expiratory phase with diffuse wheezing on auscultation. She appears fatigued and is becoming drowsy.

LABORATORY DATA:
Arterial blood gas shows a pH of 7.28, PaCO2 of 50 mmHg, PaO2 of 60 mmHg, and bicarbonate of 24 mEq/L.

Laboratory Values:
- pH: 7.28
- PaCO2: 50 mmHg
- PaO2: 60 mmHg
- Bicarbonate: 24 mEq/L

## Encounter 4: source 2023-01-15, practice 2026-04-22, ed, Emergency Medicine

Chief complaint: Altered mental status and confusion during final examination period

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Aisha Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Altered mental status and confusion during final examination period

HISTORY OF PRESENT ILLNESS:
The patient is a 28-year-old female with selective IgA deficiency, asthma, and allergic rhinitis who presents following status asthmaticus and anxiety treatment, and is brought to the emergency department by her friends after they found her at home with altered mental status. She has been under significant stress during her final exams. Her friends mention she had been taking more of her allergy medication than usual.

PAST MEDICAL HISTORY:
Active Problem List:
- Social Anxiety Disorder (Performance Type) (diagnosed 2020-01-15)
- Selective IgA deficiency (diagnosed 2020-07-15)
- Allergic conjunctivitis (diagnosed 2021-07-15)
- Status asthmaticus with impending respiratory failure (diagnosed 2022-07-15)
- Persistent asthma
- Seasonal allergic rhinitis

She has a history of seasonal allergies.

MEDICATIONS:
Home Medications:
- Albuterol HFA inhaler 90 mcg, 2 inhalations as needed for asthma symptoms
- Cetirizine 10 mg orally as needed for allergic rhinitis

ALLERGIES:
Tree pollen, Grass pollen, Dust mites

FAMILY HISTORY:
- Mother: Asthma
- Father: Seasonal allergic rhinitis
- No family history of substance use disorder

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Graduate student (Music Performance).

VITAL SIGNS:
Her vital signs are: temperature 37.2°C (99°F), blood pressure 145/90 mmHg, pulse 110/min, and respiratory rate 18/min

PHYSICAL EXAMINATION:
On examination, the patient is agitated, confused, and has a flushed face. Physical examination reveals dilated pupils, dry oral mucosa, decreased bowel sounds, and urinary retention.

OTHER STUDIES:
An ECG shows sinus tachycardia with normal intervals.
