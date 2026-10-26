# synthetic_hospital patient 1918

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 24 at first encounter, sex F, Non-Hispanic White, Parental private insurance (PPO).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-12-12 (Fri) | ed | Emergency Medicine |
| 1 | 2020-07-15 | 2026-06-04 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Peanuts (anaphylaxis)",
  "Tree nuts (anaphylaxis)"
 ],
 "insurance": "Parental private insurance (PPO)",
 "occupation": "Graduate student",
 "alcohol_use": "Social (1-2 drinks on weekends)",
 "family_history": [
  "Mother: Asthma, allergic rhinitis",
  "Father: Atopic dermatitis",
  "Younger brother: Peanut allergy"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "110 mcg, 2 puffs twice daily",
   "name": "Fluticasone propionate HFA inhaler"
  },
  {
   "dose": "90 mcg, 2 puffs every 4-6 hours as needed for wheezing or shortness of breath",
   "name": "Albuterol sulfate HFA inhaler"
  }
 ],
 "surgical_history": [
  "Tonsillectomy (age 7)",
  "Wisdom teeth extraction (age 20)"
 ],
 "chronic_conditions": [
  "Mild persistent asthma"
 ],
 "age_at_first_encounter": 24
}
```

Primary diagnoses: ["Anaphylaxis due to food", "Anaphylaxis"]
Comorbidities: ["Allergy to peanuts"]

## Encounter 0: source 2020-01-15, practice 2025-12-12, ed, Emergency Medicine

Chief complaint: Sudden onset of difficulty breathing, facial swelling, and diffuse itchy rash after consuming cake at a birthday party

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of difficulty breathing, facial swelling, and diffuse itchy rash after consuming cake at a birthday party

HISTORY OF PRESENT ILLNESS:
She reports that she was at a friend's birthday party where she consumed a slice of cake 30 minutes prior to symptom onset. She is concerned that the cake may have contained peanut oil.

PAST MEDICAL HISTORY:
Active Problem List:
- Mild persistent asthma

Her medical history is significant for mild persistent asthma,.

PAST SURGICAL HISTORY:
- Tonsillectomy (age 7)
- Wisdom teeth extraction (age 20)

MEDICATIONS:
She takes for which she uses an inhaled corticosteroid and occasionally a short-acting beta-agonist.

Home Medications:
- Fluticasone propionate HFA inhaler 110 mcg, 2 puffs twice daily
- Albuterol sulfate HFA inhaler 90 mcg, 2 puffs every 4-6 hours as needed for wheezing or shortness of breath

ALLERGIES:
She has a known allergy to peanuts.

FAMILY HISTORY:
- Mother: Asthma, allergic rhinitis
- Father: Atopic dermatitis
- Younger brother: Peanut allergy

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks on weekends). Occupation: Graduate student.

VITAL SIGNS:
On examination, her blood pressure is 90/60 mmHg, heart rate is 110 beats per minute, respiratory rate is 25 breaths per minute, and oxygen saturation is 92% on room air.

PHYSICAL EXAMINATION:
She has diffuse urticaria, facial angioedema, and wheezing on auscultation.

PLAN:
Intramuscular epinephrine is administered immediately.

## Encounter 1: source 2020-07-15, practice 2026-06-04, ed, Emergency Medicine

Chief complaint: Collapse and loss of consciousness at outdoor gathering shortly after eating nut-containing dessert, preceded by flushing and gasping

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Okafor, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Collapse and loss of consciousness at outdoor gathering shortly after eating nut-containing dessert, preceded by flushing and gasping

HISTORY OF PRESENT ILLNESS:
The patient is a 24-year-old female with mild persistent asthma and a history of difficulty breathing, facial swelling, and diffuse rash following food ingestion in January 2020, who presents with recurrent symptoms. Moments earlier, she developed sudden shortness of breath, dizziness, and generalized itching shortly after eating a dessert containing nuts. Her friends report that she became flushed and started gasping for air before losing consciousness.

PAST MEDICAL HISTORY:
Active Problem List:
- Anaphylaxis due to food (diagnosed 2020-01-15)
- Mild persistent asthma

PAST SURGICAL HISTORY:
- Tonsillectomy (age 7)
- Wisdom teeth extraction (age 20)

MEDICATIONS:
Home Medications:
- Fluticasone propionate HFA inhaler 110 mcg, 2 puffs twice daily
- Albuterol sulfate HFA inhaler 90 mcg, 2 puffs every 4-6 hours as needed for wheezing or shortness of breath

ALLERGIES:
Peanuts (anaphylaxis), Tree nuts (anaphylaxis)

FAMILY HISTORY:
- Mother: Asthma, allergic rhinitis
- Father: Atopic dermatitis
- Younger brother: Peanut allergy

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks on weekends). Occupation: Graduate student.

VITAL SIGNS:
On arrival, her temperature is 36.8°C (98.2°F), blood pressure is 68/42 mm Hg, pulse is 128/min, and respirations are 28/min with audible wheezing.

PHYSICAL EXAMINATION:
Physical examination reveals diffuse urticaria, periorbital swelling, and use of accessory respiratory muscles.

ASSESSMENT AND PLAN:
The administered drug exerts its life-saving effect primarily through

Intramuscular medication is administered immediately, followed by intravenous fluids and supplemental oxygen. Within minutes, her airway obstruction and hypotension improve significantly.
