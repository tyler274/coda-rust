# synthetic_hospital patient 1912

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 25 at first encounter, sex F, Non-Hispanic White, Private PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-06-06 (Fri) | outpatient | Otolaryngology |
| 1 | 2020-03-15 | 2025-08-17 (Sun) | ed | Emergency Medicine |
| 2 | 2020-09-15 | 2026-02-02 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Shellfish (shrimp) - anaphylaxis",
  "Dust mites",
  "Pollen (seasonal)"
 ],
 "insurance": "Private PPO",
 "occupation": "Swim instructor",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Mother: Asthma, allergic rhinitis",
  "Father: Atopic dermatitis",
  "Maternal uncle: Shellfish allergy"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "Apply thin layer to affected areas twice daily as needed for eczema flare",
   "name": "Triamcinolone acetonide cream 0.1%"
  },
  {
   "dose": "Apply liberally to dry skin areas daily for maintenance",
   "name": "Petroleum jelly (Vaseline)"
  }
 ],
 "surgical_history": [
  "Wisdom teeth extraction (age 18)"
 ],
 "chronic_conditions": [
  "Atopic dermatitis (mild, intermittent)"
 ],
 "age_at_first_encounter": 25
}
```

Primary diagnoses: ["Anaphylaxis due to shellfish", "Acute diffuse otitis externa"]
Comorbidities: ["Atopic dermatitis"]

## Encounter 0: source 2020-01-15, practice 2025-06-06, outpatient, Otolaryngology

Chief complaint: Left ear pain with sensation of fullness and decreased hearing

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Otolaryngology
Visit Type: outpatient

CHIEF COMPLAINT:
Left ear pain with sensation of fullness and decreased hearing

HISTORY OF PRESENT ILLNESS:
She presents to the clinic in July with a 3-day history of left ear pain that worsens when she pulls on her earlobe. She also reports a sensation of fullness and decreased hearing in the affected ear. She denies any fever, upper respiratory symptoms, or recent illness. and admits to frequently swimming and diving during her job.

PAST MEDICAL HISTORY:
Active Problem List:
- Atopic dermatitis (mild, intermittent)

She has a history of eczema.

PAST SURGICAL HISTORY:
- Wisdom teeth extraction (age 18)

MEDICATIONS:
Home Medications:
- Triamcinolone acetonide cream 0.1% Apply thin layer to affected areas twice daily as needed for eczema flare
- Petroleum jelly (Vaseline) Apply liberally to dry skin areas daily for maintenance

ALLERGIES:
Shellfish (shrimp) - anaphylaxis, Dust mites, Pollen (seasonal)

FAMILY HISTORY:
- Mother: Asthma, allergic rhinitis
- Father: Atopic dermatitis
- Maternal uncle: Shellfish allergy

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Swim instructor.

VITAL SIGNS:
The patient's vital signs are within normal limits.

PHYSICAL EXAMINATION:
On examination, the external auditory canal is erythematous and edematous, with greenish discharge noted. The tympanic membrane is not clearly visible due to the swelling. There is tenderness when the tragus is palpated. Otoscopic examination is painful for the patient.

## Encounter 1: source 2020-03-15, practice 2025-08-17, ed, Emergency Medicine

Chief complaint: Sudden onset difficulty breathing, facial swelling, and diffuse itchy rash after eating shrimp

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset difficulty breathing, facial swelling, and diffuse itchy rash after eating shrimp

HISTORY OF PRESENT ILLNESS:
The patient is a 25-year-old female with a history of mild intermittent atopic dermatitis managed with topical triamcinolone and petroleum jelly who was evaluated two months prior in Otolaryngology for left ear pain and fullness, now presenting to the emergency department. She reports that she was at a friend's house for dinner 30 minutes ago where she ate shrimp, which she has never had a problem with before. She denies any other recent medication use or insect bites.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute diffuse otitis externa (diagnosed 2020-01-15)
- Atopic dermatitis (mild, intermittent)

Her past medical history is significant for mild eczema, which is well-controlled with topical steroids.

PAST SURGICAL HISTORY:
- Wisdom teeth extraction (age 18)

MEDICATIONS:
Home Medications:
- Triamcinolone acetonide cream 0.1% Apply thin layer to affected areas twice daily as needed for eczema flare
- Petroleum jelly (Vaseline) Apply liberally to dry skin areas daily for maintenance

ALLERGIES:
Shellfish (shrimp) - anaphylaxis, Dust mites, Pollen (seasonal)

FAMILY HISTORY:
- Mother: Asthma, allergic rhinitis
- Father: Atopic dermatitis
- Maternal uncle: Shellfish allergy

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Swim instructor.

VITAL SIGNS:
On examination, her blood pressure is 90/50 mmHg, heart rate is 110 beats per minute, respiratory rate is 25 breaths per minute, and oxygen saturation is 92% on room air.

PHYSICAL EXAMINATION:
She appears anxious and has audible wheezing on auscultation. Her face is notably swollen, and she has urticaria on her trunk and arms.

LABORATORY DATA:
Laboratory Values:
- Serum tryptase: Elevated
- Complete blood count: Within normal limits
- Serum electrolytes: Within normal limits

## Encounter 2: source 2020-09-15, practice 2026-02-02, ed, Emergency Medicine

Chief complaint: Sudden-onset facial swelling, shortness of breath, and diffuse itchy rash

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden-onset facial swelling, shortness of breath, and diffuse itchy rash

HISTORY OF PRESENT ILLNESS:
The patient is a 25-year-old female with a history of mild intermittent atopic dermatitis who returns to the emergency department with recurrent symptoms similar to her March 2020 presentation. She is brought by ambulance shortly after developing sudden-onset facial swelling, shortness of breath, and a diffuse itchy rash. She had been dining at a seafood restaurant with friends and developed symptoms approximately 15 minutes after eating shrimp. She denies insect stings, recent medication use, or new personal care products.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute diffuse otitis externa (diagnosed 2020-01-15)
- Anaphylaxis due to shellfish (diagnosed 2020-03-15)
- Atopic dermatitis (mild, intermittent)

PAST SURGICAL HISTORY:
- Wisdom teeth extraction (age 18)

MEDICATIONS:
Home Medications:
- Triamcinolone acetonide cream 0.1% Apply thin layer to affected areas twice daily as needed for eczema flare
- Petroleum jelly (Vaseline) Apply liberally to dry skin areas daily for maintenance

ALLERGIES:
Shellfish (shrimp) - anaphylaxis, Dust mites, Pollen (seasonal)

FAMILY HISTORY:
- Mother: Asthma, allergic rhinitis
- Father: Atopic dermatitis
- Maternal uncle: Shellfish allergy

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Swim instructor.

VITAL SIGNS:
Her vital signs are: temperature 37.0°C, blood pressure 92/60 mmHg, heart rate 118/min, respiratory rate 24/min, and oxygen saturation 93% on room air.

PHYSICAL EXAMINATION:
Physical examination reveals periorbital edema, urticaria across the chest and arms, inspiratory stridor, and wheezing bilaterally. She is alert but appears anxious and dyspneic.

PLAN:
An IV line is placed, and she is given oxygen via facemask.
