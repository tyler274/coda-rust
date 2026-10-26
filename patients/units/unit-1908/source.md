# synthetic_hospital patient 1908

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 34 at first encounter, sex F, Non-Hispanic White, Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-10-21 (Mon) | outpatient | Family Medicine |
| 1 | 2020-04-15 | 2025-01-16 (Thu) | procedure | Allergy & Immunology |
| 2 | 2021-01-15 | 2025-10-14 (Tue) | ed | Ophthalmology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Latex (anaphylaxis)",
  "Dust mites (rhinitis)",
  "Tree pollen (seasonal rhinitis)"
 ],
 "insurance": "Blue Cross Blue Shield PPO",
 "occupation": "Elementary school teacher",
 "alcohol_use": "Occasional (\u22642 drinks per month)",
 "family_history": [
  "Mother: Asthma, allergic rhinitis",
  "Father: Atopic dermatitis",
  "Younger sister: Recurrent herpes simplex labialis"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "Apply thin layer to affected areas twice daily as needed for eczema",
   "name": "Mometasone furoate 0.1% ointment"
  },
  {
   "dose": "10 mg orally once daily as needed for allergic rhinitis",
   "name": "Cetirizine"
  },
  {
   "dose": "1 spray in each nostril once daily",
   "name": "Fluticasone propionate nasal spray"
  },
  {
   "dose": "1 gram orally at first sign of prodrome, then 1 gram 12 hours later (episodic therapy for cold sores)",
   "name": "Valacyclovir"
  }
 ],
 "surgical_history": [
  "Tonsillectomy and adenoidectomy at age 8 complicated by intraoperative anaphylaxis",
  "Excisional biopsy of skin nevus at age 34 complicated by intraoperative anaphylaxis"
 ],
 "chronic_conditions": [
  "Atopic dermatitis (moderate, well-controlled)",
  "Allergic rhinitis (seasonal and perennial)",
  "Recurrent herpes simplex labialis (2-3 episodes annually)",
  "History of anaphylaxis (previous intraoperative episode)"
 ],
 "age_at_first_encounter": 34
}
```

Primary diagnoses: ["Herpes Simplex Virus Infection", "Anaphylaxis", "Herpes simplex keratitis"]
Comorbidities: ["Atopic dermatitis"]

## Encounter 0: source 2020-01-15, practice 2024-10-21, outpatient, Family Medicine

Chief complaint: Painful sores on her lips and inside her mouth that have been present for 3 days.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Family Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Painful sores on her lips and inside her mouth that have been present for 3 days.

HISTORY OF PRESENT ILLNESS:
She reports that she has had similar episodes in the past, usually occurring 2-3 times per year, but they seem to be less frequent now. She denies any recent sexual activity or new partners.

PAST MEDICAL HISTORY:
Active Problem List:
- Atopic dermatitis (moderate, well-controlled)
- Allergic rhinitis (seasonal and perennial)
- Recurrent herpes simplex labialis (2-3 episodes annually)
- History of anaphylaxis (previous intraoperative episode)

Her medical history is significant for eczema, which is well-controlled with topical steroids.

PAST SURGICAL HISTORY:
- Tonsillectomy and adenoidectomy at age 8 complicated by intraoperative anaphylaxis
- Excisional biopsy of skin nevus at age 34 complicated by intraoperative anaphylaxis

MEDICATIONS:
Home Medications:
- Mometasone furoate 0.1% ointment Apply thin layer to affected areas twice daily as needed for eczema
- Cetirizine 10 mg orally once daily as needed for allergic rhinitis
- Fluticasone propionate nasal spray 1 spray in each nostril once daily
- Valacyclovir 1 gram orally at first sign of prodrome, then 1 gram 12 hours later (episodic therapy for cold sores)

ALLERGIES:
Latex (anaphylaxis), Dust mites (rhinitis), Tree pollen (seasonal rhinitis)

FAMILY HISTORY:
- Mother: Asthma, allergic rhinitis
- Father: Atopic dermatitis
- Younger sister: Recurrent herpes simplex labialis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (≤2 drinks per month). Occupation: Elementary school teacher.

VITAL SIGNS:
Her vital signs are within normal limits.

PHYSICAL EXAMINATION:
On examination, you note several small vesicles on an erythematous base on the vermillion border of her lips and a few similar lesions on her buccal mucosa. No cervical lymphadenopathy is palpable. The rest of the examination is unremarkable.

LABORATORY DATA:
Laboratory values:
- Complete blood count: Within normal limits
- HSV type-specific serology: HSV-1 IgG positive, HSV-2 IgG negative

## Encounter 1: source 2020-04-15, practice 2025-01-16, procedure, Allergy & Immunology

Chief complaint: Acute onset of facial swelling, difficulty breathing, and widespread itchy rash during minor surgical procedure.

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Patel, MD
Department: Allergy & Immunology
Visit Type: procedure

CHIEF COMPLAINT:
Acute onset of facial swelling, difficulty breathing, and widespread itchy rash during minor surgical procedure.

HISTORY OF PRESENT ILLNESS:
The patient with anaphylaxis history returns for a minor surgical procedure following treatment for herpes labialis. She reports that she was undergoing the procedure when she suddenly felt her throat closing up. She mentions that she has had a similar, though less severe, reaction during a previous surgery.

PAST MEDICAL HISTORY:
Active Problem List:
- Herpes Simplex Virus Infection (diagnosed 2020-01-15)
- Atopic dermatitis (moderate, well-controlled)
- Allergic rhinitis (seasonal and perennial)
- Recurrent herpes simplex labialis (2-3 episodes annually)
- History of anaphylaxis (previous intraoperative episode)

The patient has a history of atopic dermatitis and allergic rhinitis.

PAST SURGICAL HISTORY:
- Tonsillectomy and adenoidectomy at age 8 complicated by intraoperative anaphylaxis
- Excisional biopsy of skin nevus at age 34 complicated by intraoperative anaphylaxis

MEDICATIONS:
Home Medications:
- Mometasone furoate 0.1% ointment Apply thin layer to affected areas twice daily as needed for eczema
- Cetirizine 10 mg orally once daily as needed for allergic rhinitis
- Fluticasone propionate nasal spray 1 spray in each nostril once daily
- Valacyclovir 1 gram orally at first sign of prodrome, then 1 gram 12 hours later (episodic therapy for cold sores)

ALLERGIES:
Latex (anaphylaxis), Dust mites (rhinitis), Tree pollen (seasonal rhinitis)

FAMILY HISTORY:
- Mother: Asthma, allergic rhinitis
- Father: Atopic dermatitis
- Younger sister: Recurrent herpes simplex labialis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (≤2 drinks per month). Occupation: Elementary school teacher.

VITAL SIGNS:
Her blood pressure is 85/50 mmHg, heart rate is 120 beats per minute, respiratory rate is 28 breaths per minute, and oxygen saturation is 88% on room air.

PHYSICAL EXAMINATION:
Physical examination reveals bilateral periorbital edema, diffuse urticaria, and wheezing on auscultation. There is no jugular venous distention or peripheral edema.

PLAN:
Intravenous access is established, and fluids are initiated.

## Encounter 2: source 2021-01-15, practice 2025-10-14, ed, Ophthalmology

Chief complaint: Severe pain, redness, and blurred vision in her right eye for 3 days.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. James O'Brien, MD
Department: Ophthalmology
Visit Type: ed

CHIEF COMPLAINT:
Severe pain, redness, and blurred vision in her right eye for 3 days.

HISTORY OF PRESENT ILLNESS:
This 35-year-old female with recurrent herpes simplex labialis (January 2020) and anaphylaxis (April 2020) presents to the clinic with a 3-day history of severe pain, redness, and blurred vision in her right eye. She reports that the symptoms started with a sensation of something being in her eye, followed by tearing and photophobia.

PAST MEDICAL HISTORY:
Active Problem List:
- Herpes Simplex Virus Infection (diagnosed 2020-01-15)
- Anaphylaxis (diagnosed 2020-04-15)
- Atopic dermatitis (moderate, well-controlled)
- Allergic rhinitis (seasonal and perennial)
- Recurrent herpes simplex labialis (2-3 episodes annually)
- History of anaphylaxis (previous intraoperative episode)

Her medical history is significant for recurrent cold sores.

PAST SURGICAL HISTORY:
- Tonsillectomy and adenoidectomy at age 8 complicated by intraoperative anaphylaxis
- Excisional biopsy of skin nevus at age 34 complicated by intraoperative anaphylaxis

MEDICATIONS:
Home Medications:
- Mometasone furoate 0.1% ointment Apply thin layer to affected areas twice daily as needed for eczema
- Cetirizine 10 mg orally once daily as needed for allergic rhinitis
- Fluticasone propionate nasal spray 1 spray in each nostril once daily
- Valacyclovir 1 gram orally at first sign of prodrome, then 1 gram 12 hours later (episodic therapy for cold sores)

ALLERGIES:
Latex (anaphylaxis), Dust mites (rhinitis), Tree pollen (seasonal rhinitis)

FAMILY HISTORY:
- Mother: Asthma, allergic rhinitis
- Father: Atopic dermatitis
- Younger sister: Recurrent herpes simplex labialis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (≤2 drinks per month). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
On examination, her visual acuity is decreased in the affected eye. Slit-lamp examination reveals multiple small vesicles on the cornea and a branching dendritic ulcer that stains with fluorescein. There is no purulent discharge. The left eye examination is unremarkable.
