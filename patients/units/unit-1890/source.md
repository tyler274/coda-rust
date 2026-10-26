# synthetic_hospital patient 1890

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 25 at first encounter, sex F, Non-Hispanic White, Private commercial insurance (parent's plan).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-04-30 (Wed) | ed | Emergency Medicine |
| 1 | 2020-08-15 | 2025-11-16 (Sun) | ed | Emergency Medicine |
| 2 | 2021-03-15 | 2026-06-30 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Peanuts",
  "Tree nuts (almond, walnut)",
  "Kiwi",
  "Banana",
  "Avocado"
 ],
 "insurance": "Private commercial insurance (parent's plan)",
 "occupation": "Graduate student",
 "alcohol_use": "Occasional social use (1-2 drinks per week)",
 "family_history": [
  "Mother: Allergic rhinitis and eczema",
  "Father: Asthma",
  "Younger brother: Peanut allergy"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "90 mcg per actuation, 2 inhalations every 4-6 hours as needed for wheezing or shortness of breath",
   "name": "Albuterol HFA inhaler"
  },
  {
   "dose": "0.3 mg intramuscular, inject into thigh at onset of anaphylaxis symptoms; carry two pens at all times",
   "name": "Epinephrine auto-injector"
  },
  {
   "dose": "10 mg orally daily as needed for allergic rhinitis symptoms",
   "name": "Cetirizine"
  },
  {
   "dose": "2 sprays per nostril once daily during allergy season",
   "name": "Fluticasone propionate nasal spray"
  },
  {
   "dose": "apply thin layer to affected skin areas twice daily for eczema flare-ups",
   "name": "Triamcinolone acetonide cream 0.1%"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Atopic dermatitis",
  "Allergic rhinitis",
  "Mild persistent asthma"
 ],
 "age_at_first_encounter": 25
}
```

Primary diagnoses: ["Anaphylaxis due to peanut allergy", "Anaphylaxis due to food"]
Comorbidities: ["Allergy to peanuts", "Atopic dermatitis"]

## Encounter 0: source 2020-01-15, practice 2025-04-30, ed, Emergency Medicine

Chief complaint: Sudden onset of difficulty breathing, facial swelling, and diffuse itchy rash after consuming cake at a birthday party

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of difficulty breathing, facial swelling, and diffuse itchy rash after consuming cake at a birthday party

HISTORY OF PRESENT ILLNESS:
She reports that she was at a friend's birthday party where she consumed a slice of cake about 30 minutes prior to symptom onset. She has a known allergy to peanuts and is concerned that the cake may have contained nuts.

PAST MEDICAL HISTORY:
Active Problem List:
- Atopic dermatitis
- Allergic rhinitis
- Mild persistent asthma

Her medical history is significant for mild persistent asthma, for which she uses an albuterol inhaler as needed.

MEDICATIONS:
She takes albuterol inhaler as needed.

Home Medications:
- Albuterol HFA inhaler 90 mcg per actuation, 2 inhalations every 4-6 hours as needed for wheezing or shortness of breath
- Epinephrine auto-injector 0.3 mg intramuscular, inject into thigh at onset of anaphylaxis symptoms; carry two pens at all times
- Cetirizine 10 mg orally daily as needed for allergic rhinitis symptoms
- Fluticasone propionate nasal spray 2 sprays per nostril once daily during allergy season
- Triamcinolone acetonide cream 0.1% apply thin layer to affected skin areas twice daily for eczema flare-ups

ALLERGIES:
She has a known allergy to peanuts

FAMILY HISTORY:
- Mother: Allergic rhinitis and eczema
- Father: Asthma
- Younger brother: Peanut allergy

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per week). Occupation: Graduate student.

REVIEW OF SYSTEMS:
She denies any gastrointestinal or neurologic symptoms.

VITAL SIGNS:
On examination, her blood pressure is 90/60 mmHg, heart rate is 120 beats per minute, respiratory rate is 28 breaths per minute, and oxygen saturation is 92% on room air.

PHYSICAL EXAMINATION:
She appears anxious and has diffuse urticaria, angioedema of the lips, and wheezing on auscultation.

LABORATORY DATA:
Serum tryptase: Elevated
Complete blood count: Within normal limits
Serum electrolytes: Within normal limits

## Encounter 1: source 2020-08-15, practice 2025-11-16, ed, Emergency Medicine

Chief complaint: Sudden onset of difficulty breathing, facial swelling, and diffuse itchy rash after eating mixed fruit salad

ENCOUNTER NOTE
Date: 2020-08-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of difficulty breathing, facial swelling, and diffuse itchy rash after eating mixed fruit salad

HISTORY OF PRESENT ILLNESS:
The patient with chronic atopic conditions and a January 2020 peanut reaction presents with recurrent symptoms. She reports that at a friend's birthday party she consumed a mixed fruit salad, and within minutes she felt her throat tightening and her lips swelling.

PAST MEDICAL HISTORY:
Active Problem List:
- Anaphylaxis due to peanut allergy (diagnosed 2020-01-15)
- Atopic dermatitis
- Allergic rhinitis
- Mild persistent asthma

Her past medical history is significant for mild atopic dermatitis.

MEDICATIONS:
Home Medications:
- Albuterol HFA inhaler 90 mcg per actuation, 2 inhalations every 4-6 hours as needed for wheezing or shortness of breath
- Epinephrine auto-injector 0.3 mg intramuscular, inject into thigh at onset of anaphylaxis symptoms; carry two pens at all times
- Cetirizine 10 mg orally daily as needed for allergic rhinitis symptoms
- Fluticasone propionate nasal spray 2 sprays per nostril once daily during allergy season
- Triamcinolone acetonide cream 0.1% apply thin layer to affected skin areas twice daily for eczema flare-ups

ALLERGIES:
Peanuts, Tree nuts (almond, walnut), Kiwi, Banana, Avocado

FAMILY HISTORY:
- Mother: Allergic rhinitis and eczema
- Father: Asthma
- Younger brother: Peanut allergy

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per week). Occupation: Graduate student.

VITAL SIGNS:
On examination, her blood pressure is 85/50 mmHg, heart rate is 120 beats per minute, respiratory rate is 28 breaths per minute, and oxygen saturation is 92% on room air.

PHYSICAL EXAMINATION:
She appears anxious, has diffuse urticaria, and her voice is hoarse. Lung auscultation reveals bilateral wheezes.

LABORATORY DATA:
Laboratory Values:
- Serum tryptase: Elevated
- Complete blood count: Within normal limits
- Serum electrolytes: Within normal limits

PLAN:
Intravenous access is established, and fluids are initiated.

## Encounter 2: source 2021-03-15, practice 2026-06-30, ed, Emergency Medicine

Chief complaint: Sudden onset of difficulty breathing, facial swelling, and diffuse itchy rash after consuming cake potentially containing peanut oil

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of difficulty breathing, facial swelling, and diffuse itchy rash after consuming cake potentially containing peanut oil

HISTORY OF PRESENT ILLNESS:
The patient returns for her third presentation with suspected anaphylaxis following similar episodes in 2020. She reports that she was at a friend's birthday party where she consumed a slice of cake about 30 minutes prior to symptom onset and is concerned that the cake may have contained peanut oil.

PAST MEDICAL HISTORY:
Active Problem List:
- Anaphylaxis due to peanut allergy (diagnosed 2020-01-15)
- Anaphylaxis due to food (diagnosed 2020-08-15)
- Atopic dermatitis
- Allergic rhinitis
- Mild persistent asthma

Her medical history is significant for eczema and allergic rhinitis.

MEDICATIONS:
Home Medications:
- Albuterol HFA inhaler 90 mcg per actuation, 2 inhalations every 4-6 hours as needed for wheezing or shortness of breath
- Epinephrine auto-injector 0.3 mg intramuscular, inject into thigh at onset of anaphylaxis symptoms; carry two pens at all times
- Cetirizine 10 mg orally daily as needed for allergic rhinitis symptoms
- Fluticasone propionate nasal spray 2 sprays per nostril once daily during allergy season
- Triamcinolone acetonide cream 0.1% apply thin layer to affected skin areas twice daily for eczema flare-ups

ALLERGIES:
She has a known allergy to peanuts

FAMILY HISTORY:
- Mother: Allergic rhinitis and eczema
- Father: Asthma
- Younger brother: Peanut allergy

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per week). Occupation: Graduate student.

VITAL SIGNS:
On examination, her blood pressure is 90/60 mmHg, heart rate is 110 beats per minute, respiratory rate is 25 breaths per minute, and oxygen saturation is 92% on room air.

PHYSICAL EXAMINATION:
She has bilateral wheezes on lung auscultation, angioedema of the lips, and urticaria covering her trunk and arms.

LABORATORY DATA:
Laboratory values are as follows:
- Serum tryptase: Elevated
- Total IgE: Elevated
- Complete blood count: Within normal limits
- Basic metabolic panel: Within normal limits

PLAN:
Intramuscular epinephrine is administered immediately upon arrival.
