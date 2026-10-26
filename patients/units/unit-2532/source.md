# synthetic_hospital patient 2532

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 33 at first encounter, sex M, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-04-19 (Mon) | ed | Emergency Medicine |
| 1 | 2020-04-15 | 2021-07-19 (Mon) | outpatient | Gastroenterology |
| 2 | 2021-03-15 | 2022-06-22 (Wed) | inpatient | General Surgery |
| 3 | 2022-03-15 | 2023-06-17 (Sat) | icu | Infectious Disease |
| 4 | 2023-03-15 | 2024-06-06 (Thu) | inpatient | Neurology |
| 5 | 2024-01-15 | 2025-04-23 (Wed) | inpatient | Pulmonary & Critical Care |
| 6 | 2025-01-15 | 2026-04-09 (Thu) | inpatient | Internal Medicine |
| 7 | 2025-02-15 | 2026-05-22 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Food delivery driver",
 "alcohol_use": "Social (2-3 drinks per week)",
 "family_history": [
  "Father with Crohn's disease",
  "Maternal uncle with epilepsy",
  "Mother with type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 1 pack per day",
 "home_medications": [
  {
   "dose": "1 tablet daily",
   "name": "Biktarvy (bictegravir/tenofovir alafenamide/emtricitabine)"
  },
  {
   "dose": "1 double-strength tablet daily (PCP prophylaxis)",
   "name": "Trimethoprim-sulfamethoxazole"
  },
  {
   "dose": "1.2 g orally twice daily",
   "name": "Mesalamine"
  },
  {
   "dose": "500 mg orally twice daily",
   "name": "Levetiracetam"
  },
  {
   "dose": "1 tablet daily",
   "name": "Multivitamin with zinc"
  }
 ],
 "surgical_history": [
  "Colonoscopy with biopsy (age 28, diagnostic for IBD)",
  "Closed reduction of posterior right shoulder dislocation (age 32, secondary to seizure)"
 ],
 "chronic_conditions": [
  "HIV/AIDS (diagnosed age 27, AIDS-defining illness, intermittent adherence to antiretroviral therapy)",
  "Inflammatory bowel disease (indeterminate colitis with features of both Crohn's disease and ulcerative colitis)",
  "Epilepsy (generalized tonic-clonic seizure disorder)",
  "History of intravenous drug use disorder (heroin)",
  "History of intranasal cocaine use",
  "Tobacco use disorder"
 ],
 "age_at_first_encounter": 33
}
```

Primary diagnoses: ["Cerebral toxoplasmosis", "Pneumococcal pneumonia", "Cryptococcal meningitis", "Zinc deficiency", "Posterior dislocation of right shoulder", "Toxic megacolon secondary to Clostridioides difficile infection", "Heroin overdose"]
Comorbidities: ["HIV-related opportunistic skin infection", "Tobacco use disorder", "Medication noncompliance"]

## Encounter 0: source 2020-01-15, practice 2021-04-19, ed, Emergency Medicine

Chief complaint: Generalized tonic-clonic seizure with posterior right shoulder dislocation

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Generalized tonic-clonic seizure with posterior right shoulder dislocation

HISTORY OF PRESENT ILLNESS:
A 32-year-old male is brought to the emergency department after experiencing a generalized tonic-clonic seizure at home. His wife reports that he has a history of epilepsy and has been non-compliant with his medication regimen. The patient is postictal and unable to provide a coherent history.

PAST MEDICAL HISTORY:
Active Problem List:
- HIV/AIDS (diagnosed age 27, AIDS-defining illness, intermittent adherence to antiretroviral therapy)
- Inflammatory bowel disease (indeterminate colitis with features of both Crohn's disease and ulcerative colitis)
- Epilepsy (generalized tonic-clonic seizure disorder)
- History of intravenous drug use disorder (heroin)
- History of intranasal cocaine use
- Tobacco use disorder

PAST SURGICAL HISTORY:
- Colonoscopy with biopsy (age 28, diagnostic for IBD)
- Closed reduction of posterior right shoulder dislocation (age 32, secondary to seizure)

MEDICATIONS:
Home Medications:
- Biktarvy (bictegravir/tenofovir alafenamide/emtricitabine) 1 tablet daily
- Trimethoprim-sulfamethoxazole 1 double-strength tablet daily (PCP prophylaxis)
- Mesalamine 1.2 g orally twice daily
- Levetiracetam 500 mg orally twice daily
- Multivitamin with zinc 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with Crohn's disease
- Maternal uncle with epilepsy
- Mother with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day. Alcohol: Social (2-3 drinks per week). Occupation: Food delivery driver.

PHYSICAL EXAMINATION:
On examination, the patient's right arm is held in adduction and internal rotation, and there is a noticeable flattening of the anterior aspect of the shoulder. He winces in pain when an attempt is made to externally rotate the shoulder.

IMAGING:
An X-ray of the right shoulder is obtained and shows the humeral head internally rotated and positioned posterior to the glenoid.

## Encounter 1: source 2020-04-15, practice 2021-07-19, outpatient, Gastroenterology

Chief complaint: Persistent perioral rash, taste disturbance, and hair loss with chronic diarrhea

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Priya Patel, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent perioral rash, taste disturbance, and hair loss with chronic diarrhea

HISTORY OF PRESENT ILLNESS:
The patient with HIV returns following shoulder dislocation. He reports that he has been experiencing chronic diarrhea and weight loss for the past year.

PAST MEDICAL HISTORY:
Active Problem List:
- Posterior dislocation of right shoulder (diagnosed 2020-01-15)
- HIV/AIDS (diagnosed age 27, AIDS-defining illness, intermittent adherence to antiretroviral therapy)
- Inflammatory bowel disease (indeterminate colitis with features of both Crohn's disease and ulcerative colitis)
- Epilepsy (generalized tonic-clonic seizure disorder)
- History of intravenous drug use disorder (heroin)
- History of intranasal cocaine use
- Tobacco use disorder

His medical history is significant for Crohn&#x27;s disease.

PAST SURGICAL HISTORY:
He underwent an ileal resection two years ago

MEDICATIONS:
He admits to not adhering to his prescribed vitamin and mineral supplements.

Home Medications:
- Biktarvy (bictegravir/tenofovir alafenamide/emtricitabine) 1 tablet daily
- Trimethoprim-sulfamethoxazole 1 double-strength tablet daily (PCP prophylaxis)
- Mesalamine 1.2 g orally twice daily
- Levetiracetam 500 mg orally twice daily
- Multivitamin with zinc 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with Crohn's disease
- Maternal uncle with epilepsy
- Mother with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day. Alcohol: Social (2-3 drinks per week). Occupation: Food delivery driver.

PHYSICAL EXAMINATION:
Physical examination reveals a perioral pustular rash, alopecia, and pale conjunctivae.

LABORATORY DATA:
Laboratory results are as follows:

- Hemoglobin: 10.2 g/dL (Normal: 13.8-17.2 g/dL)
- Albumin: 3.2 g/dL (Normal: 3.5-5.0 g/dL)
- Alkaline phosphatase: 70 U/L (Normal: 40-130 U/L)
- Zinc: 50 mcg/dL (Normal: 70-120 mcg/dL)

## Encounter 2: source 2021-03-15, practice 2022-06-22, inpatient, General Surgery

Chief complaint: Worsening abdominal pain, distension, and bloody diarrhea

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Marcus Johnson, MD
Department: General Surgery
Visit Type: inpatient

CHIEF COMPLAINT:
Worsening abdominal pain, distension, and bloody diarrhea

HISTORY OF PRESENT ILLNESS:
This 34-year-old male with indeterminate colitis and HIV/AIDS was previously evaluated in Gastroenterology clinic for chronic diarrhea and zinc deficiency. He presents to the emergency department with a two-day history of worsening abdominal pain, distension, and bloody diarrhea. He reports having six to eight bloody bowel movements per day. He has not been compliant with his mesalamine therapy.

PAST MEDICAL HISTORY:
Active Problem List:
- Posterior dislocation of right shoulder (diagnosed 2020-01-15)
- Zinc deficiency (diagnosed 2020-04-15)
- HIV/AIDS (diagnosed age 27, AIDS-defining illness, intermittent adherence to antiretroviral therapy)
- Inflammatory bowel disease (indeterminate colitis with features of both Crohn's disease and ulcerative colitis)
- Epilepsy (generalized tonic-clonic seizure disorder)
- History of intravenous drug use disorder (heroin)
- History of intranasal cocaine use
- Tobacco use disorder

PAST SURGICAL HISTORY:
- Colonoscopy with biopsy (age 28, diagnostic for IBD)
- Closed reduction of posterior right shoulder dislocation (age 32, secondary to seizure)

MEDICATIONS:
He takes mesalamine.

Home Medications:
- Biktarvy (bictegravir/tenofovir alafenamide/emtricitabine) 1 tablet daily
- Trimethoprim-sulfamethoxazole 1 double-strength tablet daily (PCP prophylaxis)
- Levetiracetam 500 mg orally twice daily
- Multivitamin with zinc 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with Crohn's disease
- Maternal uncle with epilepsy
- Mother with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day. Alcohol: Social (2-3 drinks per week). Occupation: Food delivery driver.

VITAL SIGNS:
On examination, his temperature is 38.7°C (101.7°F), heart rate is 120 bpm, and blood pressure is 95/60 mmHg.

PHYSICAL EXAMINATION:
His abdomen is distended, with tenderness to palpation diffusely, but no rebound or guarding. Bowel sounds are hypoactive.

LABORATORY DATA:
Laboratory findings are as follows:

- White blood cell count: 18,000/mm³
- Hemoglobin: 12.5 g/dL
- Platelet count: 300,000/mm³
- Sodium: 135 mEq/L
- Potassium: 3.9 mEq/L
- Chloride: 100 mEq/L
- Bicarbonate: 22 mEq/L
- Blood urea nitrogen: 28 mg/dL
- Creatinine: 1.2 mg/dL

IMAGING:
An abdominal x-ray shows colonic dilation with a transverse colon diameter of 7 cm.

## Encounter 3: source 2022-03-15, practice 2023-06-17, icu, Infectious Disease

Chief complaint: Progressive headaches, nausea, confusion, and diplopia

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. Elena Rodriguez, MD
Department: Infectious Disease
Visit Type: icu

CHIEF COMPLAINT:
Progressive headaches, nausea, confusion, and diplopia

HISTORY OF PRESENT ILLNESS:
Patient with HIV/AIDS with intermittent adherence, indeterminate colitis, and epilepsy, prior toxic megacolon and zinc deficiency, presents to the emergency department with 2-week history of progressive headaches, nausea, and confusion. He also complains of double vision, which worsens with lateral gaze.

PAST MEDICAL HISTORY:
Active Problem List:
- Posterior dislocation of right shoulder (diagnosed 2020-01-15)
- Zinc deficiency (diagnosed 2020-04-15)
- Toxic megacolon secondary to Clostridioides difficile infection (diagnosed 2021-03-15)
- HIV/AIDS (diagnosed age 27, AIDS-defining illness, intermittent adherence to antiretroviral therapy)
- Inflammatory bowel disease (indeterminate colitis with features of both Crohn's disease and ulcerative colitis)
- Epilepsy (generalized tonic-clonic seizure disorder)
- History of intravenous drug use disorder (heroin)
- History of intranasal cocaine use
- Tobacco use disorder

PAST SURGICAL HISTORY:
- Colonoscopy with biopsy (age 28, diagnostic for IBD)
- Closed reduction of posterior right shoulder dislocation (age 32, secondary to seizure)

MEDICATIONS:
Home Medications:
- Biktarvy (bictegravir/tenofovir alafenamide/emtricitabine) 1 tablet daily
- Trimethoprim-sulfamethoxazole 1 double-strength tablet daily (PCP prophylaxis)
- Mesalamine 1.2 g orally twice daily
- Levetiracetam 500 mg orally twice daily
- Multivitamin with zinc 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with Crohn's disease
- Maternal uncle with epilepsy
- Mother with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day. Alcohol: Social (2-3 drinks per week). Occupation: Food delivery driver.

VITAL SIGNS:
His temperature is 37.8°C (100°F), blood pressure is 120/75 mmHg, pulse is 88/min, and respiratory rate is 16/min

PHYSICAL EXAMINATION:
On examination, the patient appears lethargic. Neurological examination reveals bilateral lateral gaze palsy. Fundoscopy shows no papilledema.

LABORATORY DATA:
An HIV test is positive, and his CD4 count is 45 cells/mm³.

OTHER STUDIES:
A lumbar puncture is performed, and the opening pressure is noted to be elevated. CSF analysis reveals the following:

- WBC: 20 cells/mm³ (80% lymphocytes)
- Protein: 60 mg/dL
- Glucose: 25 mg/dL (serum glucose 90 mg/dL)
- India ink stain: Positive for encapsulated yeasts
- Cryptococcal antigen: Positive

## Encounter 4: source 2023-03-15, practice 2024-06-06, inpatient, Neurology

Chief complaint: Two-week history of headaches, fever, and confusion

ENCOUNTER NOTE
Date: 2023-03-15
Provider: Dr. James O'Brien, MD
Department: Neurology
Visit Type: inpatient

CHIEF COMPLAINT:
Two-week history of headaches, fever, and confusion

PAST MEDICAL HISTORY:
Active Problem List:
- Posterior dislocation of right shoulder (diagnosed 2020-01-15)
- Zinc deficiency (diagnosed 2020-04-15)
- Toxic megacolon secondary to Clostridioides difficile infection (diagnosed 2021-03-15)
- Cryptococcal meningitis (diagnosed 2022-03-15)
- HIV/AIDS (diagnosed age 27, AIDS-defining illness, intermittent adherence to antiretroviral therapy)
- Inflammatory bowel disease (indeterminate colitis with features of both Crohn's disease and ulcerative colitis)
- Epilepsy (generalized tonic-clonic seizure disorder)
- History of intravenous drug use disorder (heroin)
- History of intranasal cocaine use
- Tobacco use disorder

His medical history is significant for non-compliance with antiretroviral therapy.

PAST SURGICAL HISTORY:
- Colonoscopy with biopsy (age 28, diagnostic for IBD)
- Closed reduction of posterior right shoulder dislocation (age 32, secondary to seizure)

MEDICATIONS:
Home Medications:
- Biktarvy (bictegravir/tenofovir alafenamide/emtricitabine) 1 tablet daily
- Trimethoprim-sulfamethoxazole 1 double-strength tablet daily (PCP prophylaxis)
- Mesalamine 1.2 g orally twice daily
- Levetiracetam 500 mg orally twice daily
- Multivitamin with zinc 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with Crohn's disease
- Maternal uncle with epilepsy
- Mother with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day. Alcohol: Social (2-3 drinks per week). Occupation: Food delivery driver.

VITAL SIGNS:
Temperature of 38.7°C

PHYSICAL EXAMINATION:
On physical examination, he is febrile, appears lethargic, and has a Glasgow Coma Scale score of 14. Neurological examination reveals no focal deficits.

LABORATORY DATA:
His most recent CD4 count, taken 1 month ago, was 80/mm3. Laboratory results are as follows: Hemoglobin: 12.2 g/dL; White blood cell count: 4,500/mm3; Platelets: 250,000/mm3; Toxoplasma gondii IgG: Positive

IMAGING:
A contrast-enhanced MRI of the brain shows multiple ring-enhancing lesions in the cerebral cortex and basal ganglia.

## Encounter 5: source 2024-01-15, practice 2025-04-23, inpatient, Pulmonary & Critical Care

Chief complaint: Four-day history of fever, chills, productive cough with hemoptysis, and pleuritic chest pain

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. David Kim, MD
Department: Pulmonary & Critical Care
Visit Type: inpatient

CHIEF COMPLAINT:
Four-day history of fever, chills, productive cough with hemoptysis, and pleuritic chest pain

HISTORY OF PRESENT ILLNESS:
This 37-year-old male with HIV/AIDS complicated by prior opportunistic infections including cryptococcal meningitis and cerebral toxoplasmosis in the setting of intermittent antiretroviral therapy adherence reports that his sputum is thick, yellow, and occasionally streaked with blood. He also endorses left-sided chest pain that worsens with deep breaths and coughing, as well as mild shortness of breath during exertion. He has had similar symptoms once before, about a year ago, when he was hospitalized and treated for “a lung infection” with antibiotics and improved within a week.

PAST MEDICAL HISTORY:
Active Problem List:
- Posterior dislocation of right shoulder (diagnosed 2020-01-15)
- Zinc deficiency (diagnosed 2020-04-15)
- Toxic megacolon secondary to Clostridioides difficile infection (diagnosed 2021-03-15)
- Cryptococcal meningitis (diagnosed 2022-03-15)
- Cerebral toxoplasmosis (diagnosed 2023-03-15)
- HIV/AIDS (diagnosed age 27, AIDS-defining illness, intermittent adherence to antiretroviral therapy)
- Inflammatory bowel disease (indeterminate colitis with features of both Crohn's disease and ulcerative colitis)
- Epilepsy (generalized tonic-clonic seizure disorder)
- History of intravenous drug use disorder (heroin)
- History of intranasal cocaine use
- Tobacco use disorder

He was diagnosed with HIV 4 years ago but was not engaged in regular medical care until recently. His most recent CD4+ count was 110/µL.

PAST SURGICAL HISTORY:
- Colonoscopy with biopsy (age 28, diagnostic for IBD)
- Closed reduction of posterior right shoulder dislocation (age 32, secondary to seizure)

MEDICATIONS:
He began antiretroviral therapy 3 months ago and has been taking his medications consistently since then. He is also taking trimethoprim-sulfamethoxazole.

Home Medications:
- Biktarvy (bictegravir/tenofovir alafenamide/emtricitabine) 1 tablet daily
- Mesalamine 1.2 g orally twice daily
- Levetiracetam 500 mg orally twice daily
- Multivitamin with zinc 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with Crohn's disease
- Maternal uncle with epilepsy
- Mother with type 2 diabetes mellitus

SOCIAL HISTORY:
He smokes one pack of cigarettes daily and drinks alcohol socially. He works as a food delivery driver and recently returned to work after recovering from a respiratory illness 5 weeks ago. He lives alone and has not traveled recently.

REVIEW OF SYSTEMS:
He denies orthopnea, paroxysmal nocturnal dyspnea, leg swelling, or weight loss.

VITAL SIGNS:
Temperature is 39.5°C (103.1°F), blood pressure is 118/62 mm Hg, pulse is 108/min, respirations are 22/min, and oxygen saturation is 96% on room air.

PHYSICAL EXAMINATION:
On examination, he appears fatigued and slightly diaphoretic but is alert and oriented. Pulmonary examination reveals decreased breath sounds, dullness to percussion, and coarse crackles at the left lower lung field. The remainder of the exam is unremarkable. There is no lymphadenopathy or hepatosplenomegaly.

IMAGING:
Chest x-ray reveals a dense left lower lobe consolidation with air bronchograms and no cavitation or effusion.

## Encounter 6: source 2025-01-15, practice 2026-04-09, inpatient, Internal Medicine

Chief complaint: Four days of worsening shortness of breath, productive cough, fever, and right-sided pleuritic chest pain

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Aisha Washington, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Four days of worsening shortness of breath, productive cough, fever, and right-sided pleuritic chest pain

HISTORY OF PRESENT ILLNESS:
This 38-year-old male with HIV/AIDS with intermittent antiretroviral therapy adherence, history of cryptococcal meningitis, cerebral toxoplasmosis, and prior pneumococcal pneumonia one year prior, presents with 4 days of worsening shortness of breath, non-bloody productive cough, fever, and right-sided pleuritic chest pain. He reports generalized fatigue, decreased appetite, and chills. He has not seen a physician since his last hospitalization 12 months ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Posterior dislocation of right shoulder (diagnosed 2020-01-15)
- Zinc deficiency (diagnosed 2020-04-15)
- Toxic megacolon secondary to Clostridioides difficile infection (diagnosed 2021-03-15)
- Cryptococcal meningitis (diagnosed 2022-03-15)
- Cerebral toxoplasmosis (diagnosed 2023-03-15)
- Pneumococcal pneumonia (diagnosed 2024-01-15)
- HIV/AIDS (diagnosed age 27, AIDS-defining illness, intermittent adherence to antiretroviral therapy)
- Inflammatory bowel disease (indeterminate colitis with features of both Crohn's disease and ulcerative colitis)
- Epilepsy (generalized tonic-clonic seizure disorder)
- History of intravenous drug use disorder (heroin)
- History of intranasal cocaine use
- Tobacco use disorder

His past medical history is notable for HIV, diagnosed 6 years ago, but he has been off antiretroviral therapy for over 3 years.

PAST SURGICAL HISTORY:
- Colonoscopy with biopsy (age 28, diagnostic for IBD)
- Closed reduction of posterior right shoulder dislocation (age 32, secondary to seizure)

MEDICATIONS:
Home Medications:
- Biktarvy (bictegravir/tenofovir alafenamide/emtricitabine) 1 tablet daily
- Trimethoprim-sulfamethoxazole 1 double-strength tablet daily (PCP prophylaxis)
- Mesalamine 1.2 g orally twice daily
- Levetiracetam 500 mg orally twice daily
- Multivitamin with zinc 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with Crohn's disease
- Maternal uncle with epilepsy
- Mother with type 2 diabetes mellitus

SOCIAL HISTORY:
He lives alone and denies any recent travel, occupational exposures, or sick contacts. He smokes a pack of cigarettes daily and occasionally uses intranasal cocaine.

VITAL SIGNS:
Temperature is 38.9°C (102°F), blood pressure is 108/62 mmHg, heart rate is 112/min, respiratory rate is 26/min, and oxygen saturation is 93% on room air.

PHYSICAL EXAMINATION:
On examination, the patient is ill-appearing and mildly diaphoretic. Pulmonary exam reveals dullness to percussion, decreased breath sounds, and crackles over the right posterior lung field. There are no oral lesions or skin rashes.

LABORATORY DATA:
Laboratory studies show a WBC count of 13,800/mm³ and a CD4 count of 72/µL.

IMAGING:
A chest X-ray is obtained and shown below.

## Encounter 7: source 2025-02-15, practice 2026-05-22, ed, Emergency Medicine

Chief complaint: Altered mental status concerning for heroin overdose

ENCOUNTER NOTE
Date: 2025-02-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Altered mental status concerning for heroin overdose

HISTORY OF PRESENT ILLNESS:
The 38-year-old male with a complex history of HIV/AIDS with intermittent antiretroviral adherence, prior opportunistic infections including cryptococcal meningitis and cerebral toxoplasmosis, and recurrent pneumococcal pneumonia most recently one month ago, presents with altered mental status. The patient has a known history of intravenous drug use and is accompanied by friends who report heroin use with concern for overdose. They further note that he was recently released from jail and has not been taking any prescribed medications. Naloxone is administered with improvement in his respiratory rate and mental status. However, the patient becomes agitated and refuses further treatment, attempting to leave the emergency department.

PAST MEDICAL HISTORY:
Active Problem List:
- Posterior dislocation of right shoulder (diagnosed 2020-01-15)
- Zinc deficiency (diagnosed 2020-04-15)
- Toxic megacolon secondary to Clostridioides difficile infection (diagnosed 2021-03-15)
- Cryptococcal meningitis (diagnosed 2022-03-15)
- Cerebral toxoplasmosis (diagnosed 2023-03-15)
- Pneumococcal pneumonia (diagnosed 2024-01-15)
- HIV/AIDS (diagnosed age 27, AIDS-defining illness, intermittent adherence to antiretroviral therapy)
- Inflammatory bowel disease (indeterminate colitis with features of both Crohn's disease and ulcerative colitis)
- Epilepsy (generalized tonic-clonic seizure disorder)
- History of intravenous drug use disorder (heroin)
- History of intranasal cocaine use
- Tobacco use disorder

PAST SURGICAL HISTORY:
- Colonoscopy with biopsy (age 28, diagnostic for IBD)
- Closed reduction of posterior right shoulder dislocation (age 32, secondary to seizure)

MEDICATIONS:
Home Medications:
- Biktarvy (bictegravir/tenofovir alafenamide/emtricitabine) 1 tablet daily
- Trimethoprim-sulfamethoxazole 1 double-strength tablet daily (PCP prophylaxis)
- Mesalamine 1.2 g orally twice daily
- Levetiracetam 500 mg orally twice daily
- Multivitamin with zinc 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with Crohn's disease
- Maternal uncle with epilepsy
- Mother with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Current smoker, 1 pack per day. Alcohol: Social (2-3 drinks per week). Occupation: Food delivery driver.

VITAL SIGNS:
Respiratory rate is 6 breaths per minute, heart rate is 55 beats per minute, blood pressure is 90/50 mmHg, and temperature is 36.6°C (97.9°F). The patient's oxygen saturation is 89% on room air.

PHYSICAL EXAMINATION:
On examination, the patient is unresponsive to verbal commands. His pupils are pinpoint, and there are track marks on his arms.

LABORATORY DATA:
A fingerstick glucose level is 110 mg/dL.
