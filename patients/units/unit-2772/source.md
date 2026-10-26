# synthetic_hospital patient 2772

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex F, Non-Hispanic White, Blue Cross Blue Shield (employer-provided).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-07-31 (Fri) | outpatient | Endocrinology |
| 1 | 2021-01-15 | 2021-07-26 (Mon) | outpatient | Cardiology |
| 2 | 2023-01-15 | 2023-07-15 (Sat) | ed | Emergency Medicine |
| 3 | 2024-01-15 | 2024-07-31 (Wed) | ed | Emergency Medicine |
| 4 | 2026-01-15 | 2026-07-29 (Wed) | icu | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Ibuprofen (anaphylaxis)",
  "Aspirin (anaphylaxis)",
  "Shellfish (anaphylaxis)",
  "Tree pollen (seasonal allergic rhinitis)"
 ],
 "insurance": "Blue Cross Blue Shield (employer-provided)",
 "occupation": "Elementary school teacher",
 "alcohol_use": "Rare (1-2 drinks per month)",
 "family_history": [
  "Mother: Asthma, allergic rhinitis",
  "Father: Hypertension",
  "Maternal aunt: Rheumatoid arthritis"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "90 mcg, 2 puffs every 4-6 hours as needed for wheezing",
   "name": "Albuterol HFA inhaler"
  },
  {
   "dose": "250/50 mcg, 1 inhalation twice daily",
   "name": "Fluticasone propionate/salmeterol diskus"
  },
  {
   "dose": "2 sprays in each nostril twice daily",
   "name": "Mometasone nasal spray"
  },
  {
   "dose": "10 mg orally once daily at bedtime",
   "name": "Montelukast"
  },
  {
   "dose": "10 mg orally once daily",
   "name": "Prednisone"
  },
  {
   "dose": "30 mg intramuscularly every 4 weeks",
   "name": "Octreotide LAR"
  },
  {
   "dose": "10 mg orally once daily as needed",
   "name": "Cetirizine"
  },
  {
   "dose": "0.3 mg, inject intramuscularly as needed for anaphylaxis",
   "name": "Epinephrine auto-injector"
  }
 ],
 "surgical_history": [
  "Endoscopic sinus surgery (2019)",
  "Appendectomy (2005)"
 ],
 "chronic_conditions": [
  "Severe persistent asthma",
  "Chronic rhinosinusitis with nasal polyposis",
  "Aspirin-exacerbated respiratory disease",
  "Eosinophilic granulomatosis with polyangiitis",
  "Carcinoid syndrome",
  "Atopic dermatitis"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Aspirin-exacerbated respiratory disease", "Acute asthma exacerbation", "Eosinophilic granulomatosis with polyangiitis", "Carcinoid syndrome", "Anaphylaxis due to shellfish"]
Comorbidities: ["Chronic Sinusitis", "Asthma", "Nasal polyps", "Tension-type headache or migraine"]

## Encounter 0: source 2020-01-15, practice 2020-07-31, outpatient, Endocrinology

Chief complaint: Six-month history of episodic facial flushing, watery diarrhea, and wheezing episodes.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of episodic facial flushing, watery diarrhea, and wheezing episodes.

HISTORY OF PRESENT ILLNESS:
She reports that the flushing episodes are intense, last for about 10 minutes, and are sometimes accompanied by a feeling of warmth and palpitations. Her diarrhea is watery, occurs 3-4 times a day, and is not associated with any particular food intake. She also mentions occasional shortness of breath that seems to be unrelated to physical activity.

PAST MEDICAL HISTORY:
Active Problem List:
- Severe persistent asthma
- Chronic rhinosinusitis with nasal polyposis
- Aspirin-exacerbated respiratory disease
- Eosinophilic granulomatosis with polyangiitis
- Carcinoid syndrome
- Atopic dermatitis

She has a past medical history of asthma.

PAST SURGICAL HISTORY:
- Endoscopic sinus surgery (2019)
- Appendectomy (2005)

MEDICATIONS:
She takes for which she uses an albuterol inhaler as needed.

Home Medications:
- Albuterol HFA inhaler 90 mcg, 2 puffs every 4-6 hours as needed for wheezing
- Fluticasone propionate/salmeterol diskus 250/50 mcg, 1 inhalation twice daily
- Mometasone nasal spray 2 sprays in each nostril twice daily
- Montelukast 10 mg orally once daily at bedtime
- Prednisone 10 mg orally once daily
- Octreotide LAR 30 mg intramuscularly every 4 weeks
- Cetirizine 10 mg orally once daily as needed
- Epinephrine auto-injector 0.3 mg, inject intramuscularly as needed for anaphylaxis

ALLERGIES:
Ibuprofen (anaphylaxis), Aspirin (anaphylaxis), Shellfish (anaphylaxis), Tree pollen (seasonal allergic rhinitis)

FAMILY HISTORY:
- Mother: Asthma, allergic rhinitis
- Father: Hypertension
- Maternal aunt: Rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (1-2 drinks per month). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
On physical examination, she appears malnourished with evidence of dermatitis on sun-exposed areas of her skin. Cardiac auscultation reveals a high-pitched, early diastolic murmur best heard along the right sternal border.

LABORATORY DATA:
Laboratory findings are as follows:

- Complete Blood Count (CBC): Within normal limits
- Comprehensive Metabolic Panel (CMP): Within normal limits
- Urinary 5-Hydroxyindoleacetic acid (5-HIAA): Elevated
- Niacin levels: Low
- Serotonin levels: Elevated

## Encounter 1: source 2021-01-15, practice 2021-07-26, outpatient, Cardiology

Chief complaint: Progressive shortness of breath, fatigue, and unintentional weight loss over the past month with new left foot numbness.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Michael Rodriguez, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive shortness of breath, fatigue, and unintentional weight loss over the past month with new left foot numbness.

HISTORY OF PRESENT ILLNESS:
The patient, with severe persistent asthma and eosinophilic granulomatosis with polyangiitis, returns for follow-up one year after her initial Endocrinology presentation. She has a 10-year history of asthma that had previously been well controlled but has recently become more severe, requiring frequent use of her rescue inhaler. She also reports numbness and tingling in her left foot, which began a few weeks ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Carcinoid syndrome (diagnosed 2020-01-15)
- Severe persistent asthma
- Chronic rhinosinusitis with nasal polyposis
- Aspirin-exacerbated respiratory disease
- Eosinophilic granulomatosis with polyangiitis
- Atopic dermatitis

PAST SURGICAL HISTORY:
- Endoscopic sinus surgery (2019)
- Appendectomy (2005)

MEDICATIONS:
Home Medications:
- Albuterol HFA inhaler 90 mcg, 2 puffs every 4-6 hours as needed for wheezing
- Fluticasone propionate/salmeterol diskus 250/50 mcg, 1 inhalation twice daily
- Mometasone nasal spray 2 sprays in each nostril twice daily
- Montelukast 10 mg orally once daily at bedtime
- Prednisone 10 mg orally once daily
- Octreotide LAR 30 mg intramuscularly every 4 weeks
- Cetirizine 10 mg orally once daily as needed
- Epinephrine auto-injector 0.3 mg, inject intramuscularly as needed for anaphylaxis

ALLERGIES:
Ibuprofen (anaphylaxis), Aspirin (anaphylaxis), Shellfish (anaphylaxis), Tree pollen (seasonal allergic rhinitis)

FAMILY HISTORY:
- Mother: Asthma, allergic rhinitis
- Father: Hypertension
- Maternal aunt: Rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (1-2 drinks per month). Occupation: Elementary school teacher.

REVIEW OF SYSTEMS:
Sinus congestion and occasional wheezing

PHYSICAL EXAMINATION:
Scattered expiratory wheezes and decreased sensation over the dorsum of her left foot

LABORATORY DATA:
High eosinophils and an elevated ESR. Anti-myeloperoxidase antibodies are seen. Urinalysis is normal.

PATHOLOGY:
A sural nerve biopsy reveals necrotizing inflammation of small vessels with surrounding eosinophils and granulomas.

## Encounter 2: source 2023-01-15, practice 2023-07-15, ed, Emergency Medicine

Chief complaint: Sudden onset of wheezing, chest tightness, and nasal congestion 30 minutes after ibuprofen ingestion.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of wheezing, chest tightness, and nasal congestion 30 minutes after ibuprofen ingestion.

HISTORY OF PRESENT ILLNESS:
The patient with asthma and prior encounters in 2020 and 2021 presents acutely. She reports that similar episodes have occurred in the past, usually following the ingestion of certain pain relievers, which she cannot recall the names of.

PAST MEDICAL HISTORY:
Active Problem List:
- Carcinoid syndrome (diagnosed 2020-01-15)
- Eosinophilic granulomatosis with polyangiitis (diagnosed 2021-01-15)
- Severe persistent asthma
- Chronic rhinosinusitis with nasal polyposis
- Aspirin-exacerbated respiratory disease
- Atopic dermatitis

PAST SURGICAL HISTORY:
- Endoscopic sinus surgery (2019)
- Appendectomy (2005)

MEDICATIONS:
Home Medications:
- Albuterol HFA inhaler 90 mcg, 2 puffs every 4-6 hours as needed for wheezing
- Fluticasone propionate/salmeterol diskus 250/50 mcg, 1 inhalation twice daily
- Mometasone nasal spray 2 sprays in each nostril twice daily
- Montelukast 10 mg orally once daily at bedtime
- Prednisone 10 mg orally once daily
- Octreotide LAR 30 mg intramuscularly every 4 weeks
- Cetirizine 10 mg orally once daily as needed
- Epinephrine auto-injector 0.3 mg, inject intramuscularly as needed for anaphylaxis

ALLERGIES:
Ibuprofen (anaphylaxis), Aspirin (anaphylaxis), Shellfish (anaphylaxis), Tree pollen (seasonal allergic rhinitis)

FAMILY HISTORY:
- Mother: Asthma, allergic rhinitis
- Father: Hypertension
- Maternal aunt: Rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (1-2 drinks per month). Occupation: Elementary school teacher.

VITAL SIGNS:
Her vital signs are as follows: BP 130/85 mmHg, HR 95 bpm, RR 22 breaths/min, and SpO2 94% on room air.

PHYSICAL EXAMINATION:
On physical examination, she is noted to have bilateral nasal polyps and diffuse wheezing on auscultation.

LABORATORY DATA:
Her complete blood count and basic metabolic panel are within normal limits.

## Encounter 3: source 2024-01-15, practice 2024-07-31, ed, Emergency Medicine

Chief complaint: Acute onset of wheezing and shortness of breath beginning two hours after taking ibuprofen.

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset of wheezing and shortness of breath beginning two hours after taking ibuprofen.

HISTORY OF PRESENT ILLNESS:
The patient with aspirin-exacerbated respiratory disease presents with wheezing that began two hours after taking ibuprofen for tension headache, similar to an episode she reports from one year ago after taking aspirin for menstrual cramps.

PAST MEDICAL HISTORY:
Active Problem List:
- Carcinoid syndrome (diagnosed 2020-01-15)
- Eosinophilic granulomatosis with polyangiitis (diagnosed 2021-01-15)
- Acute asthma exacerbation (diagnosed 2023-01-15)
- Severe persistent asthma
- Chronic rhinosinusitis with nasal polyposis
- Aspirin-exacerbated respiratory disease
- Atopic dermatitis

Medical history is significant for asthma.

PAST SURGICAL HISTORY:
- Endoscopic sinus surgery (2019)
- Appendectomy (2005)

MEDICATIONS:
She takes albuterol inhaler as needed.

Home Medications:
- Albuterol HFA inhaler 90 mcg, 2 puffs every 4-6 hours as needed for wheezing
- Fluticasone propionate/salmeterol diskus 250/50 mcg, 1 inhalation twice daily
- Mometasone nasal spray 2 sprays in each nostril twice daily
- Montelukast 10 mg orally once daily at bedtime
- Prednisone 10 mg orally once daily
- Octreotide LAR 30 mg intramuscularly every 4 weeks
- Cetirizine 10 mg orally once daily as needed
- Epinephrine auto-injector 0.3 mg, inject intramuscularly as needed for anaphylaxis

ALLERGIES:
Ibuprofen (anaphylaxis), Aspirin (anaphylaxis), Shellfish (anaphylaxis), Tree pollen (seasonal allergic rhinitis)

FAMILY HISTORY:
- Mother: Asthma, allergic rhinitis
- Father: Hypertension
- Maternal aunt: Rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (1-2 drinks per month). Occupation: Elementary school teacher.

VITAL SIGNS:
Temperature 37.0°C, blood pressure 130/85 mmHg, heart rate 95 beats per minute, respiratory rate 22 breaths per minute, and oxygen saturation 94% on room air

PHYSICAL EXAMINATION:
Nasal polyps and diffuse wheezing on auscultation of the chest

LABORATORY DATA:
Complete blood count and basic metabolic panel are within normal limits.

## Encounter 4: source 2026-01-15, practice 2026-07-29, icu, Emergency Medicine

Chief complaint: Acute generalized urticaria, severe dyspnea, and lightheadedness immediately after shellfish ingestion with biphasic reaction requiring second epinephrine dose.

ENCOUNTER NOTE
Date: 2026-01-15
Provider: Dr. Aisha Johnson, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Acute generalized urticaria, severe dyspnea, and lightheadedness immediately after shellfish ingestion with biphasic reaction requiring second epinephrine dose.

HISTORY OF PRESENT ILLNESS:
The patient with severe persistent asthma, aspirin-exacerbated respiratory disease, and prior emergency department presentations for NSAID-induced respiratory reactions presents after shellfish ingestion, to which she states she has a known allergy. She is administered intramuscular epinephrine, after which her symptoms improve significantly. However, 1 hour later, her symptoms recur with worsening respiratory distress and hypotension. She receives a second dose of epinephrine with improvement.

PAST MEDICAL HISTORY:
Active Problem List:
- Carcinoid syndrome (diagnosed 2020-01-15)
- Eosinophilic granulomatosis with polyangiitis (diagnosed 2021-01-15)
- Acute asthma exacerbation (diagnosed 2023-01-15)
- Aspirin-exacerbated respiratory disease (diagnosed 2024-01-15)
- Severe persistent asthma
- Chronic rhinosinusitis with nasal polyposis
- Atopic dermatitis

She has a history of asthma and eczema.

PAST SURGICAL HISTORY:
- Endoscopic sinus surgery (2019)
- Appendectomy (2005)

MEDICATIONS:
Home Medications:
- Albuterol HFA inhaler 90 mcg, 2 puffs every 4-6 hours as needed for wheezing
- Fluticasone propionate/salmeterol diskus 250/50 mcg, 1 inhalation twice daily
- Mometasone nasal spray 2 sprays in each nostril twice daily
- Montelukast 10 mg orally once daily at bedtime
- Prednisone 10 mg orally once daily
- Octreotide LAR 30 mg intramuscularly every 4 weeks
- Cetirizine 10 mg orally once daily as needed
- Epinephrine auto-injector 0.3 mg, inject intramuscularly as needed for anaphylaxis

ALLERGIES:
Ibuprofen (anaphylaxis), Aspirin (anaphylaxis), Shellfish (anaphylaxis), Tree pollen (seasonal allergic rhinitis)

FAMILY HISTORY:
- Mother: Asthma, allergic rhinitis
- Father: Hypertension
- Maternal aunt: Rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (1-2 drinks per month). Occupation: Elementary school teacher.

VITAL SIGNS:
On examination, her blood pressure is 85/50 mmHg, heart rate is 120 beats per minute, respiratory rate is 28 breaths per minute, and oxygen saturation is 88% on room air.

PHYSICAL EXAMINATION:
She has diffuse wheezing on auscultation, facial angioedema, and a diffuse, raised, erythematous rash.

LABORATORY DATA:
Laboratory Values:
- Serum tryptase: Elevated
- Complete blood count: Unremarkable
- Basic metabolic panel: Within normal limits
