# synthetic_hospital patient 2524

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 35 at first encounter, sex M, Hispanic/Latino, Private PPO (employer-sponsored).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-02-28 (Mon) | outpatient | Gastroenterology |
| 1 | 2020-07-15 | 2022-08-27 (Sat) | ed | Emergency Medicine |
| 2 | 2021-01-15 | 2023-03-02 (Thu) | icu | Surgery |
| 3 | 2021-09-15 | 2023-10-30 (Mon) | inpatient | Infectious Diseases |
| 4 | 2022-03-15 | 2024-04-25 (Thu) | ed | Emergency Medicine |
| 5 | 2022-09-15 | 2024-10-18 (Fri) | outpatient | Neurology |
| 6 | 2023-03-15 | 2025-04-17 (Thu) | inpatient | Gastroenterology |
| 7 | 2023-09-15 | 2025-10-24 (Fri) | inpatient | Gastroenterology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Private PPO (employer-sponsored)",
 "occupation": "Software engineer",
 "alcohol_use": "None",
 "family_history": [
  "Father: Hypertension",
  "Mother: Type 2 diabetes mellitus",
  "Paternal uncle: Ulcerative colitis",
  "No known family history of immunodeficiency or primary neurological disorders"
 ],
 "race_ethnicity": "Hispanic/Latino",
 "smoking_status": "Never",
 "home_medications": [
  {
   "dose": "50mg/25mg/200mg PO daily (prescribed; patient reports intermittent adherence)",
   "name": "Bictegravir/Tenofovir alafenamide/Emtricitabine (Biktarvy)"
  },
  {
   "dose": "1 tablet PO daily",
   "name": "Multivitamin"
  }
 ],
 "surgical_history": [
  "Split-thickness skin grafting for thermal burn wound coverage",
  "Tracheostomy (placed during burn admission, subsequently decannulated)"
 ],
 "chronic_conditions": [
  "HIV infection with AIDS (CDC Stage 3, poorly controlled due to non-adherence)",
  "Ulcerative colitis",
  "History of progressive multifocal leukoencephalopathy (PML)",
  "History of cryptococcal meningitis",
  "History of miliary tuberculosis",
  "History of severe thermal burns (40% total body surface area)"
 ],
 "age_at_first_encounter": 35
}
```

Primary diagnoses: ["Cytomegalovirus colitis", "Miliary tuberculosis", "Neurocysticercosis", "Cryptococcal meningitis", "Burn wound infection with sepsis", "Ulcerative colitis", "Progressive multifocal leukoencephalopathy", "Cytomegalovirus esophagitis"]
Comorbidities: ["Medication noncompliance", "Sinus tachycardia secondary to burns", "HIV-related opportunistic skin infection", "Fever", "Cancer-related constitutional symptoms", "Anemia", "Immunosuppressed state"]

## Encounter 0: source 2020-01-15, practice 2022-02-28, outpatient, Gastroenterology

Chief complaint: Six-month history of progressively worsening diarrhea with blood and mucus, occurring 6-8 times daily with nocturnal symptoms, associated with 5 kg weight loss and fatigue.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Elena Rodriguez, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of progressively worsening diarrhea with blood and mucus, occurring 6-8 times daily with nocturnal symptoms, associated with 5 kg weight loss and fatigue.

HISTORY OF PRESENT ILLNESS:
He reports that his bowel movements are loose, occur 6–8 times daily, and are often accompanied by blood and mucus. The diarrhea sometimes wakes him at night. Over the past 3 months, he has lost 5 kg (11 lb) without trying. He has also noticed occasional low-grade fevers, fatigue, and decreased appetite. He denies recent travel, antibiotic use, or contact with anyone with similar symptoms. He has no history of camping, consumption of untreated water, or recent foreign travel.

PAST MEDICAL HISTORY:
Active Problem List:
- HIV infection with AIDS (CDC Stage 3, poorly controlled due to non-adherence)
- Ulcerative colitis
- History of progressive multifocal leukoencephalopathy (PML)
- History of cryptococcal meningitis
- History of miliary tuberculosis
- History of severe thermal burns (40% total body surface area)

PAST SURGICAL HISTORY:
- Split-thickness skin grafting for thermal burn wound coverage
- Tracheostomy (placed during burn admission, subsequently decannulated)

MEDICATIONS:
His only medication is a daily multivitamin.

Home Medications:
- Bictegravir/Tenofovir alafenamide/Emtricitabine (Biktarvy) 50mg/25mg/200mg PO daily (prescribed; patient reports intermittent adherence)

ALLERGIES:
He has no known drug allergies.

FAMILY HISTORY:
His father has a history of an “intestinal problem” that required surgery in his 40s, but the details are unclear.

SOCIAL HISTORY:
He does not smoke or drink alcohol and works as a software engineer.

REVIEW OF SYSTEMS:
On review of systems, the patient reports occasional joint pain in his knees and ankles, especially in the morning, which improves throughout the day. He also describes redness and mild discomfort in his eyes over the past few weeks. He denies skin rashes, oral ulcers, or chronic back pain.

VITAL SIGNS:
Temperature is 37.9°C (100.2°F), blood pressure 118/74 mm Hg, pulse 84/min, respirations 14/min.

PHYSICAL EXAMINATION:
On examination, the patient appears mildly pale but is in no acute distress. His abdomen is soft with mild tenderness in the lower quadrants, without rebound or guarding. There is no palpable hepatosplenomegaly or abdominal masses. Bowel sounds are present and normoactive. Rectal examination reveals gross blood mixed with stool. No perianal fistulas or abscesses are noted.

LABORATORY DATA:
Initial laboratory studies show mild normocytic anemia and elevated erythrocyte sedimentation rate. Stool cultures and ova/parasite studies are negative.

## Encounter 1: source 2020-07-15, practice 2022-08-27, ed, Emergency Medicine

Chief complaint: Three-week history of progressively worsening headaches, low-grade fevers, confusion, and generalized muscle pain following return from humanitarian trip to rural Guatemala.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Three-week history of progressively worsening headaches, low-grade fevers, confusion, and generalized muscle pain following return from humanitarian trip to rural Guatemala.

HISTORY OF PRESENT ILLNESS:
The patient with HIV/AIDS and ulcerative colitis presents. He also complains of generalized muscle pain. He recently returned from a humanitarian trip to rural Guatemala.

PAST MEDICAL HISTORY:
Active Problem List:
- Ulcerative colitis (diagnosed 2020-01-15)
- HIV infection with AIDS (CDC Stage 3, poorly controlled due to non-adherence)
- History of progressive multifocal leukoencephalopathy (PML)
- History of cryptococcal meningitis
- History of miliary tuberculosis
- History of severe thermal burns (40% total body surface area)

PAST SURGICAL HISTORY:
- Split-thickness skin grafting for thermal burn wound coverage
- Tracheostomy (placed during burn admission, subsequently decannulated)

MEDICATIONS:
Home Medications:
- Bictegravir/Tenofovir alafenamide/Emtricitabine (Biktarvy) 50mg/25mg/200mg PO daily (prescribed; patient reports intermittent adherence)
- Multivitamin 1 tablet PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertension
- Mother: Type 2 diabetes mellitus
- Paternal uncle: Ulcerative colitis
- No known family history of immunodeficiency or primary neurological disorders

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Software engineer.

PHYSICAL EXAMINATION:
On examination, he is oriented to person only, with a fever of 38.3°C (100.9°F) and mild right-sided weakness. Fundoscopy reveals no papilledema.

LABORATORY DATA:
Serologic testing for IgM and IgG antibodies to protozoa are negative

IMAGING:
MRI of the brain shows multiple enhancing lesions at the corticomedullary junction.

PLAN:
A brain biopsy is deferred

## Encounter 2: source 2021-01-15, practice 2023-03-02, icu, Surgery

Chief complaint: Second and third-degree burns covering 40% total body surface area sustained in house fire six hours ago, with inhalational injury requiring intubation and mechanical ventilation.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Sarah Chen, MD
Department: Surgery
Visit Type: icu

CHIEF COMPLAINT:
Second and third-degree burns covering 40% total body surface area sustained in house fire six hours ago, with inhalational injury requiring intubation and mechanical ventilation.

HISTORY OF PRESENT ILLNESS:
The patient, with HIV/AIDS, ulcerative colitis, and opportunistic infections, sustained burns in a house fire 6 hours ago and is currently intubated due to inhalational injury and on mechanical ventilation.

PAST MEDICAL HISTORY:
Active Problem List:
- Ulcerative colitis (diagnosed 2020-01-15)
- Neurocysticercosis (diagnosed 2020-07-15)
- HIV infection with AIDS (CDC Stage 3, poorly controlled due to non-adherence)
- History of progressive multifocal leukoencephalopathy (PML)
- History of cryptococcal meningitis
- History of miliary tuberculosis
- History of severe thermal burns (40% total body surface area)

PAST SURGICAL HISTORY:
- Split-thickness skin grafting for thermal burn wound coverage
- Tracheostomy (placed during burn admission, subsequently decannulated)

MEDICATIONS:
Home Medications:
- Bictegravir/Tenofovir alafenamide/Emtricitabine (Biktarvy) 50mg/25mg/200mg PO daily (prescribed; patient reports intermittent adherence)
- Multivitamin 1 tablet PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertension
- Mother: Type 2 diabetes mellitus
- Paternal uncle: Ulcerative colitis
- No known family history of immunodeficiency or primary neurological disorders

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Software engineer.

VITAL SIGNS:
On examination, his blood pressure is 95/60 mmHg, heart rate is 120 beats per minute, and temperature is 38.2°C.

PHYSICAL EXAMINATION:
His abdomen is soft and non-distended.

LABORATORY DATA:
Laboratory results are as follows:

- Hemoglobin: 14.5 g/dL
- White blood cell count: 16,000/mm³
- Platelets: 250,000/mm³
- Sodium: 145 mEq/L
- Potassium: 4.0 mEq/L
- Blood urea nitrogen: 18 mg/dL
- Creatinine: 1.0 mg/dL
- Glucose: 160 mg/dL
- Albumin: 3.5 g/dL

ASSESSMENT AND PLAN:
The patient is hemodynamically stable on fluid resuscitation.

## Encounter 3: source 2021-09-15, practice 2023-10-30, inpatient, Infectious Diseases

Chief complaint: Three-week history of fever, night sweats, non-productive cough, and 5 kg weight loss; CD4 count was 180 cells/mm³ six months ago with poor ART adherence.

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. Robert Kim, MD
Department: Infectious Diseases
Visit Type: inpatient

CHIEF COMPLAINT:
Three-week history of fever, night sweats, non-productive cough, and 5 kg weight loss; CD4 count was 180 cells/mm³ six months ago with poor ART adherence.

HISTORY OF PRESENT ILLNESS:
The patient with HIV/AIDS, history of miliary tuberculosis, and severe thermal burns presents with immunosuppression. He has lost 5 kg over the past month. His CD4 count six months ago was 180 cells/mm³, but he has not been compliant with his antiretroviral therapy.

PAST MEDICAL HISTORY:
Active Problem List:
- Ulcerative colitis (diagnosed 2020-01-15)
- Neurocysticercosis (diagnosed 2020-07-15)
- Burn wound infection with sepsis (diagnosed 2021-01-15)
- HIV infection with AIDS (CDC Stage 3, poorly controlled due to non-adherence)
- History of progressive multifocal leukoencephalopathy (PML)
- History of cryptococcal meningitis
- History of miliary tuberculosis
- History of severe thermal burns (40% total body surface area)

PAST SURGICAL HISTORY:
- Split-thickness skin grafting for thermal burn wound coverage
- Tracheostomy (placed during burn admission, subsequently decannulated)

MEDICATIONS:
Home Medications:
- Bictegravir/Tenofovir alafenamide/Emtricitabine (Biktarvy) 50mg/25mg/200mg PO daily (prescribed; patient reports intermittent adherence)
- Multivitamin 1 tablet PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertension
- Mother: Type 2 diabetes mellitus
- Paternal uncle: Ulcerative colitis
- No known family history of immunodeficiency or primary neurological disorders

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Software engineer.

VITAL SIGNS:
Temperature of 38.9°C, heart rate of 110 bpm

PHYSICAL EXAMINATION:
Diffuse crackles on lung auscultation

LABORATORY DATA:
Hemoglobin: 10.2 g/dL
White blood cell count: 4,500 cells/mm³
Platelets: 250,000 cells/mm³
Sodium: 135 mEq/L
Potassium: 4.2 mEq/L
Creatinine: 1.1 mg/dL
Alanine aminotransferase (ALT): 45 U/L
Aspartate aminotransferase (AST): 50 U/L
Alkaline phosphatase: 120 U/L
HIV viral load: 120,000 copies/mL
CD4 count: 50 cells/mm³

IMAGING:
Chest X-ray reveals numerous 1-2 mm nodules throughout both lung fields.

OTHER STUDIES:
A tuberculin skin test is performed and shows no induration after 48 hours.

## Encounter 4: source 2022-03-15, practice 2024-04-25, ed, Emergency Medicine

Chief complaint: Two-week progressively worsening headache, fever, and neck stiffness unresponsive to acetaminophen.

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-week progressively worsening headache, fever, and neck stiffness unresponsive to acetaminophen.

HISTORY OF PRESENT ILLNESS:
The patient with CDC Stage 3 HIV/AIDS and history of cryptococcal meningitis presents to the emergency department with a 2-week history of headache, fever, and neck stiffness. He reports that his symptoms have progressively worsened despite taking acetaminophen.

PAST MEDICAL HISTORY:
Active Problem List:
- Ulcerative colitis (diagnosed 2020-01-15)
- Neurocysticercosis (diagnosed 2020-07-15)
- Burn wound infection with sepsis (diagnosed 2021-01-15)
- Miliary tuberculosis (diagnosed 2021-09-15)
- HIV infection with AIDS (CDC Stage 3, poorly controlled due to non-adherence)
- History of progressive multifocal leukoencephalopathy (PML)
- History of cryptococcal meningitis
- History of severe thermal burns (40% total body surface area)

His medical history is significant for non-adherence to antiretroviral therapy.

PAST SURGICAL HISTORY:
- Split-thickness skin grafting for thermal burn wound coverage
- Tracheostomy (placed during burn admission, subsequently decannulated)

MEDICATIONS:
Home Medications:
- Bictegravir/Tenofovir alafenamide/Emtricitabine (Biktarvy) 50mg/25mg/200mg PO daily (prescribed; patient reports intermittent adherence)
- Multivitamin 1 tablet PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertension
- Mother: Type 2 diabetes mellitus
- Paternal uncle: Ulcerative colitis
- No known family history of immunodeficiency or primary neurological disorders

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Software engineer.

VITAL SIGNS:
On examination, he is febrile with a temperature of 38.9°C

PHYSICAL EXAMINATION:
Neck flexion elicits resistance and pain. Fundoscopic examination reveals no papilledema.

LABORATORY DATA:
His most recent CD4 count, taken 1 month ago, was 50/mm3.

OTHER STUDIES:
Lumbar puncture is performed, and the cerebrospinal fluid (CSF) analysis reveals:

- Opening pressure: 250 mm H2O
- WBC: 20/mm3 (90% lymphocytes)
- Protein: 100 mg/dL
- Glucose: 20 mg/dL (serum glucose 90 mg/dL)
- India ink stain: Positive for encapsulated yeasts
- CSF culture: Pending

ASSESSMENT AND PLAN:
Based on the patient's presentation and CSF findings,

## Encounter 5: source 2022-09-15, practice 2024-10-18, outpatient, Neurology

Chief complaint: Two-week progressive weakness in left arm, difficulty walking, mild headache, and personality changes noted by partner; denies fever or neck stiffness.

ENCOUNTER NOTE
Date: 2022-09-15
Provider: Dr. David Liu, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Two-week progressive weakness in left arm, difficulty walking, mild headache, and personality changes noted by partner; denies fever or neck stiffness.

HISTORY OF PRESENT ILLNESS:
This 37-year-old male with HIV/AIDS (CDC Stage 3, poorly controlled), prior progressive multifocal leukoencephalopathy, and cryptococcal meningitis presents to the clinic with a 2-week history of progressive weakness in his left arm and difficulty walking. He reports a mild headache but denies fever, neck stiffness, or photophobia. His partner mentions that he has been increasingly forgetful and had a personality change. His antiretroviral therapy has been irregular due to social circumstances.

PAST MEDICAL HISTORY:
Active Problem List:
- Ulcerative colitis (diagnosed 2020-01-15)
- Neurocysticercosis (diagnosed 2020-07-15)
- Burn wound infection with sepsis (diagnosed 2021-01-15)
- Miliary tuberculosis (diagnosed 2021-09-15)
- Cryptococcal meningitis (diagnosed 2022-03-15)
- HIV infection with AIDS (CDC Stage 3, poorly controlled due to non-adherence)
- History of progressive multifocal leukoencephalopathy (PML)
- History of severe thermal burns (40% total body surface area)

PAST SURGICAL HISTORY:
- Split-thickness skin grafting for thermal burn wound coverage
- Tracheostomy (placed during burn admission, subsequently decannulated)

MEDICATIONS:
Home Medications:
- Bictegravir/Tenofovir alafenamide/Emtricitabine (Biktarvy) 50mg/25mg/200mg PO daily (prescribed; patient reports intermittent adherence)
- Multivitamin 1 tablet PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertension
- Mother: Type 2 diabetes mellitus
- Paternal uncle: Ulcerative colitis
- No known family history of immunodeficiency or primary neurological disorders

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Software engineer.

PHYSICAL EXAMINATION:
On examination, he has a left-sided hemiparesis and ataxic gait.

LABORATORY DATA:
His CD4 count is 150/mm3 and viral load is elevated.

IMAGING:
MRI of the brain shows multiple nonenhancing white matter lesions without mass effect or edema.

## Encounter 6: source 2023-03-15, practice 2025-04-17, inpatient, Gastroenterology

Chief complaint: Two-week odynophagia, dysphagia, and retrosternal chest pain with 5 kg weight loss; failed one-week fluconazole trial for presumed candidiasis.

ENCOUNTER NOTE
Date: 2023-03-15
Provider: Dr. Elena Rodriguez, MD
Department: Gastroenterology
Visit Type: inpatient

CHIEF COMPLAINT:
Two-week odynophagia, dysphagia, and retrosternal chest pain with 5 kg weight loss; failed one-week fluconazole trial for presumed candidiasis.

HISTORY OF PRESENT ILLNESS:
This 38-year-old male with HIV infection with AIDS, ulcerative colitis, and history of progressive multifocal leukoencephalopathy, cryptococcal meningitis, and miliary tuberculosis, presents with persistent symptoms. His antiretroviral therapy has been irregular due to social circumstances. He reports a recent weight loss of 5 kg and has a low-grade fever. Despite treatment with oral fluconazole for one week, his symptoms have not improved. He denies any vesicular lesions in the oropharynx.

PAST MEDICAL HISTORY:
Active Problem List:
- Ulcerative colitis (diagnosed 2020-01-15)
- Neurocysticercosis (diagnosed 2020-07-15)
- Burn wound infection with sepsis (diagnosed 2021-01-15)
- Miliary tuberculosis (diagnosed 2021-09-15)
- Cryptococcal meningitis (diagnosed 2022-03-15)
- Progressive multifocal leukoencephalopathy (diagnosed 2022-09-15)
- HIV infection with AIDS (CDC Stage 3, poorly controlled due to non-adherence)
- History of progressive multifocal leukoencephalopathy (PML)
- History of severe thermal burns (40% total body surface area)

PAST SURGICAL HISTORY:
- Split-thickness skin grafting for thermal burn wound coverage
- Tracheostomy (placed during burn admission, subsequently decannulated)

MEDICATIONS:
Home Medications:
- Bictegravir/Tenofovir alafenamide/Emtricitabine (Biktarvy) 50mg/25mg/200mg PO daily (prescribed; patient reports intermittent adherence)
- Multivitamin 1 tablet PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertension
- Mother: Type 2 diabetes mellitus
- Paternal uncle: Ulcerative colitis
- No known family history of immunodeficiency or primary neurological disorders

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Software engineer.

PHYSICAL EXAMINATION:
Oral thrush

LABORATORY DATA:
CD4 count is 50 cells/mm³

## Encounter 7: source 2023-09-15, practice 2025-10-24, inpatient, Gastroenterology

Chief complaint: Two-week history of abdominal pain, frequent small-volume bloody stools, and crampy lower abdominal pain.

ENCOUNTER NOTE
Date: 2023-09-15
Provider: Dr. Michael Thompson, MD
Department: Gastroenterology
Visit Type: inpatient

CHIEF COMPLAINT:
Two-week history of abdominal pain, frequent small-volume bloody stools, and crampy lower abdominal pain.

HISTORY OF PRESENT ILLNESS:
Patient with HIV/AIDS, ulcerative colitis, and CMV esophagitis reports 2-week history of abdominal pain and diarrhea. He reports frequent, small-volume, bloody stools and crampy lower abdominal pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Ulcerative colitis (diagnosed 2020-01-15)
- Neurocysticercosis (diagnosed 2020-07-15)
- Burn wound infection with sepsis (diagnosed 2021-01-15)
- Miliary tuberculosis (diagnosed 2021-09-15)
- Cryptococcal meningitis (diagnosed 2022-03-15)
- Progressive multifocal leukoencephalopathy (diagnosed 2022-09-15)
- Cytomegalovirus esophagitis (diagnosed 2023-03-15)
- HIV infection with AIDS (CDC Stage 3, poorly controlled due to non-adherence)
- History of progressive multifocal leukoencephalopathy (PML)
- History of severe thermal burns (40% total body surface area)

His medical history is significant for non-compliance with antiretroviral therapy.

PAST SURGICAL HISTORY:
- Split-thickness skin grafting for thermal burn wound coverage
- Tracheostomy (placed during burn admission, subsequently decannulated)

MEDICATIONS:
Home Medications:
- Bictegravir/Tenofovir alafenamide/Emtricitabine (Biktarvy) 50mg/25mg/200mg PO daily (prescribed; patient reports intermittent adherence)
- Multivitamin 1 tablet PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hypertension
- Mother: Type 2 diabetes mellitus
- Paternal uncle: Ulcerative colitis
- No known family history of immunodeficiency or primary neurological disorders

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Software engineer.

VITAL SIGNS:
His temperature is 38.2°C (100.8°F), blood pressure is 100/60 mmHg, and heart rate is 110 beats per minute.

PHYSICAL EXAMINATION:
On physical examination, he appears cachectic and has diffuse abdominal tenderness without rebound or guarding.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 10.2 g/dL
- White blood cell count: 4,500/mm3
- Platelets: 150,000/mm3
- CD4 count: 40/mm3
- HIV viral load: 120,000 copies/mL

OTHER STUDIES:
A colonoscopy is performed, revealing multiple ulcerations in the sigmoid colon. Biopsies are taken

PLAN:
The patient is admitted for further management
