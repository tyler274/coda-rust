# synthetic_hospital patient 2608

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 28 at first encounter, sex M, White, non-Hispanic, Private (Employer-sponsored PPO).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-01-01 (Wed) | ed | Emergency Medicine |
| 1 | 2020-07-15 | 2025-07-01 (Tue) | ed | Emergency Medicine |
| 2 | 2021-01-15 | 2025-12-27 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Cat dander (known trigger)",
  "Tree pollen",
  "Grass pollen",
  "Dust mites"
 ],
 "insurance": "Private (Employer-sponsored PPO)",
 "occupation": "Software Engineer",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Father: Allergic rhinitis and asthma",
  "Mother: Seasonal allergies",
  "Sister: Atopic dermatitis"
 ],
 "race_ethnicity": "White, non-Hispanic",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1 spray (50 mcg) per nostril daily",
   "name": "Fluticasone propionate nasal spray"
  },
  {
   "dose": "10 mg PO PRN for allergic symptoms",
   "name": "Cetirizine"
  },
  {
   "dose": "90 mcg, 2 puffs PRN for wheezing/shortness of breath",
   "name": "Albuterol HFA inhaler"
  }
 ],
 "surgical_history": [
  "Tonsillectomy and adenoidectomy (age 8)"
 ],
 "chronic_conditions": [
  "Allergic rhinitis (seasonal and perennial)",
  "Mild intermittent allergic asthma"
 ],
 "age_at_first_encounter": 28
}
```

Primary diagnoses: ["Allergic reaction treated with systemic corticosteroids", "Anaphylaxis due to food", "Anaphylaxis"]
Comorbidities: ["Allergy to peanuts", "Allergic rhinitis", "Acute urticaria"]

## Encounter 0: source 2020-01-15, practice 2025-01-01, ed, Emergency Medicine

Chief complaint: Widespread itching, nasal congestion, watery eyes, and shortness of breath beginning minutes after exposure to cat dander at a friend's home.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Widespread itching, nasal congestion, watery eyes, and shortness of breath beginning minutes after exposure to cat dander at a friend's home.

HISTORY OF PRESENT ILLNESS:
He presents to the emergency department shortly after developing widespread itching, nasal congestion, watery eyes, and shortness of breath. He reports that his symptoms began within minutes of visiting a friend's home where a cat was present. He recalls similar but milder symptoms during previous exposures.

PAST MEDICAL HISTORY:
Active Problem List:
- Allergic rhinitis (seasonal and perennial)
- Mild intermittent allergic asthma

PAST SURGICAL HISTORY:
- Tonsillectomy and adenoidectomy (age 8)

MEDICATIONS:
Home Medications:
- Fluticasone propionate nasal spray 1 spray (50 mcg) per nostril daily
- Cetirizine 10 mg PO PRN for allergic symptoms
- Albuterol HFA inhaler 90 mcg, 2 puffs PRN for wheezing/shortness of breath

ALLERGIES:
Cat dander (known trigger), Tree pollen, Grass pollen, Dust mites

FAMILY HISTORY:
- Father: Allergic rhinitis and asthma
- Mother: Seasonal allergies
- Sister: Atopic dermatitis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Software Engineer.

VITAL SIGNS:
His temperature is 36.9°C (98.4°F), blood pressure is 110/68 mm Hg, pulse is 104/min, and respirations are 22/min

PHYSICAL EXAMINATION:
Skin examination shows erythematous, raised wheals over both arms and neck.

LABORATORY DATA:
Laboratory evaluation shows no eosinophilia or leukocytosis.

## Encounter 1: source 2020-07-15, practice 2025-07-01, ed, Emergency Medicine

Chief complaint: Sudden onset difficulty breathing, swelling of the lips and face, and widespread hives after consuming shellfish at a restaurant, associated with lightheadedness.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset difficulty breathing, swelling of the lips and face, and widespread hives after consuming shellfish at a restaurant, associated with lightheadedness.

HISTORY OF PRESENT ILLNESS:
This 28-year-old man with allergic rhinitis, intermittent asthma, and a prior ED visit for allergic reaction to cat dander in January 2020, returns to the emergency department with sudden onset of difficulty breathing, swelling of the lips and face, and widespread hives after eating at a seafood restaurant. He reports feeling lightheaded and dizzy shortly after consuming shellfish.

PAST MEDICAL HISTORY:
Active Problem List:
- Allergic reaction treated with systemic corticosteroids (diagnosed 2020-01-15)
- Allergic rhinitis (seasonal and perennial)
- Mild intermittent allergic asthma

He denies any prior history of similar reactions but has a history of seasonal allergies.

PAST SURGICAL HISTORY:
- Tonsillectomy and adenoidectomy (age 8)

MEDICATIONS:
Home Medications:
- Fluticasone propionate nasal spray 1 spray (50 mcg) per nostril daily
- Cetirizine 10 mg PO PRN for allergic symptoms
- Albuterol HFA inhaler 90 mcg, 2 puffs PRN for wheezing/shortness of breath

ALLERGIES:
Cat dander (known trigger), Tree pollen, Grass pollen, Dust mites

FAMILY HISTORY:
- Father: Allergic rhinitis and asthma
- Mother: Seasonal allergies
- Sister: Atopic dermatitis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Software Engineer.

VITAL SIGNS:
His vital signs reveal a pulse of 135/min, respiratory rate of 30/min, and blood pressure of 78/50 mm Hg.

PHYSICAL EXAMINATION:
On examination, diffuse wheezing is heard throughout the lungs, and there is noticeable periorbital edema.

LABORATORY DATA:
Laboratory studies show elevated serum tryptase levels.

ASSESSMENT AND PLAN:
The rapid progression of symptoms following allergen exposure suggests an acute hypersensitivity reaction.

## Encounter 2: source 2021-01-15, practice 2025-12-27, ed, Emergency Medicine

Chief complaint: Difficulty breathing, diffuse itching, and swelling of the lips and tongue rapidly developing after being stung by a wasp while hiking.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Aisha Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Difficulty breathing, diffuse itching, and swelling of the lips and tongue rapidly developing after being stung by a wasp while hiking.

HISTORY OF PRESENT ILLNESS:
The patient with allergic rhinitis, asthma, and prior allergic reactions is brought to the emergency department shortly after. He rapidly developed difficulty breathing, diffuse itching, and swelling of the lips and tongue.

PAST MEDICAL HISTORY:
Active Problem List:
- Allergic reaction treated with systemic corticosteroids (diagnosed 2020-01-15)
- Anaphylaxis due to food (diagnosed 2020-07-15)
- Allergic rhinitis (seasonal and perennial)
- Mild intermittent allergic asthma

PAST SURGICAL HISTORY:
- Tonsillectomy and adenoidectomy (age 8)

MEDICATIONS:
Home Medications:
- Fluticasone propionate nasal spray 1 spray (50 mcg) per nostril daily
- Cetirizine 10 mg PO PRN for allergic symptoms
- Albuterol HFA inhaler 90 mcg, 2 puffs PRN for wheezing/shortness of breath

ALLERGIES:
Cat dander (known trigger), Tree pollen, Grass pollen, Dust mites

FAMILY HISTORY:
- Father: Allergic rhinitis and asthma
- Mother: Seasonal allergies
- Sister: Atopic dermatitis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Software Engineer.

VITAL SIGNS:
On arrival, he is hypotensive with a blood pressure of 76/44 mm Hg and tachycardic with a heart rate of 130/min.

PHYSICAL EXAMINATION:
Physical exam reveals widespread urticaria and inspiratory stridor.

ASSESSMENT AND PLAN:
Although epinephrine has multiple physiologic effects, the most important mechanism by which it restores blood pressure in this setting is related to vascular tone.

Intramuscular epinephrine is administered immediately.
