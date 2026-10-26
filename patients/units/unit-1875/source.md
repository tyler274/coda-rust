# synthetic_hospital patient 1875

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 25 at first encounter, sex F, Hispanic/Latina, Employer-sponsored health insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-12-16 (Tue) | outpatient | Obstetrics & Gynecology |
| 1 | 2020-03-15 | 2026-02-23 (Mon) | outpatient | Rheumatology |
| 2 | 2020-07-15 | 2026-06-22 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Dust mites",
  "Ragweed pollen"
 ],
 "insurance": "Employer-sponsored health insurance",
 "occupation": "Elementary school teacher",
 "alcohol_use": "None",
 "family_history": [
  "Mother: Asthma",
  "Father: Seasonal allergies",
  "Maternal aunt: Eczema"
 ],
 "race_ethnicity": "Hispanic/Latina",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "110 mcg, 2 puffs twice daily",
   "name": "Fluticasone propionate inhaler"
  },
  {
   "dose": "90 mcg, 2 puffs every 4-6 hours as needed for wheezing",
   "name": "Albuterol sulfate inhaler"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Asthma"
 ],
 "age_at_first_encounter": 25
}
```

Primary diagnoses: ["Anesthetic medication reaction", "Acute asthma exacerbation"]
Comorbidities: ["Asthma"]

## Encounter 0: source 2020-01-15, practice 2025-12-16, outpatient, Obstetrics & Gynecology

Chief complaint: 2-month history of progressively worsening shortness of breath and wheezing during pregnancy

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Carmen Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
2-month history of progressively worsening shortness of breath and wheezing during pregnancy

HISTORY OF PRESENT ILLNESS:
She has a known history of asthma, which had been well-controlled on inhaled corticosteroids and occasional albuterol for the past 5 years. She denies any recent respiratory infections, allergen exposures, or changes in her environment. She reports that she has been using her albuterol inhaler more frequently, about 4-5 times per week, but has not been using her inhaled corticosteroids regularly since she found out she was pregnant due to concerns about the safety of medications during pregnancy.

PAST MEDICAL HISTORY:
Active Problem List:
- Asthma

MEDICATIONS:
Home Medications:
- Fluticasone propionate inhaler 110 mcg, 2 puffs twice daily
- Albuterol sulfate inhaler 90 mcg, 2 puffs every 4-6 hours as needed for wheezing

ALLERGIES:
Dust mites, Ragweed pollen

FAMILY HISTORY:
- Mother: Asthma
- Father: Seasonal allergies
- Maternal aunt: Eczema

SOCIAL HISTORY:
She also denies tobacco, alcohol, or drug use.

VITAL SIGNS:
Oxygen saturation is 95% on room air.

PHYSICAL EXAMINATION:
On physical examination, she is in mild respiratory distress. Lung auscultation reveals bilateral expiratory wheezes.

## Encounter 1: source 2020-03-15, practice 2026-02-23, outpatient, Rheumatology

Chief complaint: Fever, widespread itchy rash, and painful, swollen joints

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Jamal Washington, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Fever, widespread itchy rash, and painful, swollen joints

HISTORY OF PRESENT ILLNESS:
The patient is a 25-year-old female with asthma maintained on fluticasone and albuterol who was evaluated two months prior for worsening shortness of breath during pregnancy. She presents to the clinic with a 3-day history of fever, widespread itchy rash, and painful, swollen joints, reporting that symptoms began approximately 10 days after initiating amoxicillin for a sinus infection.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute asthma exacerbation (diagnosed 2020-01-15)
- Asthma

MEDICATIONS:
Home Medications:
- Fluticasone propionate inhaler 110 mcg, 2 puffs twice daily
- Albuterol sulfate inhaler 90 mcg, 2 puffs every 4-6 hours as needed for wheezing

ALLERGIES:
She has no known drug allergies.

FAMILY HISTORY:
- Mother: Asthma
- Father: Seasonal allergies
- Maternal aunt: Eczema

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Elementary school teacher.

VITAL SIGNS:
A temperature of 38.5°C (101.3°F)

PHYSICAL EXAMINATION:
On examination, she has an urticarial rash on her trunk and limbs, and tender, swollen knees and ankles. There is also noticeable cervical lymphadenopathy.

LABORATORY DATA:
Laboratory tests reveal the following:

- White blood cell count: 11,000/mm³ (normal 4,500-11,000/mm³)
- Erythrocyte sedimentation rate (ESR): 45 mm/hr (normal <20 mm/hr)
- C-reactive protein (CRP): 30 mg/L (normal <10 mg/L)
- Serum complement levels (C3, C4): Normal
- Rheumatoid factor: Negative
- Antinuclear antibody (ANA): Negative

## Encounter 2: source 2020-07-15, practice 2026-06-22, ed, Emergency Medicine

Chief complaint: Sudden onset difficulty breathing, facial swelling, and diffuse itchy rash

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Priya Sharma, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset difficulty breathing, facial swelling, and diffuse itchy rash

HISTORY OF PRESENT ILLNESS:
The patient with asthma and prior anesthetic reaction presents with difficulty breathing, facial swelling, and rash that developed within minutes after taking ibuprofen for menstrual cramps. She reports previous ibuprofen use without issues.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute asthma exacerbation (diagnosed 2020-01-15)
- Anesthetic medication reaction (diagnosed 2020-03-15)
- Asthma

MEDICATIONS:
Home Medications:
- Fluticasone propionate inhaler 110 mcg, 2 puffs twice daily
- Albuterol sulfate inhaler 90 mcg, 2 puffs every 4-6 hours as needed for wheezing

ALLERGIES:
Dust mites, Ragweed pollen

FAMILY HISTORY:
- Mother: Asthma
- Father: Seasonal allergies
- Maternal aunt: Eczema

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Elementary school teacher.

VITAL SIGNS:
On examination, her blood pressure is 90/50 mmHg, heart rate is 120 beats per minute, respiratory rate is 28 breaths per minute, and oxygen saturation is 88% on room air.

PHYSICAL EXAMINATION:
She has diffuse wheezing on auscultation, facial angioedema, and urticaria.

LABORATORY DATA:
Her complete blood count and basic metabolic panel are within normal limits.

PLAN:
Intravenous access is established, and she is given supplemental oxygen.
