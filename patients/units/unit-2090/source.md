# synthetic_hospital patient 2090

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 38 at first encounter, sex M, White, non-Hispanic, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-12-17 (Thu) | outpatient | Infectious Diseases |
| 1 | 2020-09-15 | 2021-09-01 (Wed) | outpatient | Otolaryngology |
| 2 | 2021-05-15 | 2022-05-02 (Mon) | outpatient | Dermatology |
| 3 | 2022-03-15 | 2023-02-27 (Mon) | outpatient | Gastroenterology |
| 4 | 2023-01-15 | 2024-01-03 (Wed) | outpatient | Oncology |
| 5 | 2023-09-15 | 2024-08-22 (Thu) | ed | Emergency Medicine |
| 6 | 2024-09-15 | 2025-08-20 (Wed) | ed | Emergency Medicine |
| 7 | 2025-05-15 | 2026-05-01 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicaid",
 "occupation": "Construction laborer, intermittently employed",
 "alcohol_use": "Occasional, 1-2 drinks per month",
 "family_history": [
  "Father with hypertension",
  "Mother with type 2 diabetes mellitus",
  "No family history of hematologic malignancies or primary immunodeficiency disorders"
 ],
 "race_ethnicity": "White, non-Hispanic",
 "smoking_status": "Former smoker, quit 3 years ago, 10 pack-year history",
 "home_medications": [
  {
   "dose": "50/25/200 mg tablet, one daily",
   "name": "Bictegravir/Tenofovir alafenamide/Emtricitabine"
  },
  {
   "dose": "double strength tablet, three times weekly (for PCP prophylaxis)",
   "name": "Trimethoprim-sulfamethoxazole"
  },
  {
   "dose": "100 mg tablet, daily as needed for thrush",
   "name": "Fluconazole"
  },
  {
   "dose": "500 mg tablet, as needed for pain or fever",
   "name": "Acetaminophen"
  }
 ],
 "surgical_history": [
  "Tonsillectomy, age 12"
 ],
 "chronic_conditions": [
  "Human immunodeficiency virus (HIV) infection, diagnosed 10 years prior, with intermittent antiretroviral therapy adherence",
  "History of Pneumocystis jirovecii pneumonia",
  "History of oropharyngeal candidiasis",
  "Polysubstance use disorder (cocaine and methamphetamine), previous intranasal and smoked use, last use 6 months ago",
  "Unstable housing status, currently in transitional housing"
 ],
 "age_at_first_encounter": 38
}
```

Primary diagnoses: ["Non-Hodgkin lymphoma", "Kaposi sarcoma (AIDS-related)", "Candidal esophagitis", "HIV-related opportunistic skin infection", "Oral hairy leukoplakia", "Cryptosporidiosis", "History of Pneumocystis jirovecii pneumonia"]
Comorbidities: ["HIV-related opportunistic skin infection", "Medication noncompliance", "Oral candidiasis"]

## Encounter 0: source 2020-01-15, practice 2020-12-17, outpatient, Infectious Diseases

Chief complaint: Presents to the clinic for a routine follow-up with no acute complaints.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Infectious Diseases
Visit Type: outpatient

CHIEF COMPLAINT:
Presents to the clinic for a routine follow-up with no acute complaints.

HISTORY OF PRESENT ILLNESS:
He reports adherence to his antiretroviral therapy and has no complaints. His last CD4 count was 350/mm3 six months ago. He is interested in receiving any vaccinations that he may need.

PAST MEDICAL HISTORY:
Active Problem List:
- Human immunodeficiency virus (HIV) infection, diagnosed 10 years prior, with intermittent antiretroviral therapy adherence
- History of Pneumocystis jirovecii pneumonia
- History of oropharyngeal candidiasis
- Polysubstance use disorder (cocaine and methamphetamine), previous intranasal and smoked use, last use 6 months ago
- Unstable housing status, currently in transitional housing

PAST SURGICAL HISTORY:
- Tonsillectomy, age 12

MEDICATIONS:
Home Medications:
- Bictegravir/Tenofovir alafenamide/Emtricitabine 50/25/200 mg tablet, one daily
- Trimethoprim-sulfamethoxazole double strength tablet, three times weekly (for PCP prophylaxis)
- Fluconazole 100 mg tablet, daily as needed for thrush
- Acetaminophen 500 mg tablet, as needed for pain or fever

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with hypertension
- Mother with type 2 diabetes mellitus
- No family history of hematologic malignancies or primary immunodeficiency disorders

SOCIAL HISTORY:
He is sexually active with one partner and uses condoms consistently.

PHYSICAL EXAMINATION:
His physical examination is unremarkable.

LABORATORY DATA:
Today's laboratory results are as follows:

- CD4 count: 220/mm3
- HIV viral load: Undetectable
- Varicella-zoster virus (VZV) IgG: Negative

## Encounter 1: source 2020-09-15, practice 2021-09-01, outpatient, Otolaryngology

Chief complaint: Painless white lesion on the lateral border of the tongue persisting for two weeks.

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Michael Rodriguez, MD
Department: Otolaryngology
Visit Type: outpatient

CHIEF COMPLAINT:
Painless white lesion on the lateral border of the tongue persisting for two weeks.

HISTORY OF PRESENT ILLNESS:
This 38-year-old male with ten years of HIV and intermittent ART adherence, last evaluated eight months ago for an opportunistic skin infection, presents to the clinic with a painless white lesion on the lateral border of his tongue persisting for two weeks. He reports no recent illness but endorses intermittent fatigue and unintentional weight loss over the past several months.

PAST MEDICAL HISTORY:
Active Problem List:
- HIV-related opportunistic skin infection (diagnosed 2020-01-15)
- Human immunodeficiency virus (HIV) infection, diagnosed 10 years prior, with intermittent antiretroviral therapy adherence
- History of Pneumocystis jirovecii pneumonia
- History of oropharyngeal candidiasis
- Polysubstance use disorder (cocaine and methamphetamine), previous intranasal and smoked use, last use 6 months ago
- Unstable housing status, currently in transitional housing

He is not an asthmatic.

PAST SURGICAL HISTORY:
- Tonsillectomy, age 12

MEDICATIONS:
Home Medications:
- Bictegravir/Tenofovir alafenamide/Emtricitabine 50/25/200 mg tablet, one daily
- Trimethoprim-sulfamethoxazole double strength tablet, three times weekly (for PCP prophylaxis)
- Fluconazole 100 mg tablet, daily as needed for thrush
- Acetaminophen 500 mg tablet, as needed for pain or fever

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with hypertension
- Mother with type 2 diabetes mellitus
- No family history of hematologic malignancies or primary immunodeficiency disorders

SOCIAL HISTORY:
He has a history of recreational drug use and reports multiple sexual partners.

PHYSICAL EXAMINATION:
On physical exam, the white lesion appears corrugated and firmly adherent; it cannot be scraped off. No cervical lymphadenopathy is noted.

PLAN:
An HIV test is ordered.

## Encounter 2: source 2021-05-15, practice 2022-05-02, outpatient, Dermatology

Chief complaint: Two-month history of fatigue, intermittent fever, and unintentional weight loss of 10 kg.

ENCOUNTER NOTE
Date: 2021-05-15
Provider: Dr. Aisha Patel, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Two-month history of fatigue, intermittent fever, and unintentional weight loss of 10 kg.

PAST MEDICAL HISTORY:
Active Problem List:
- HIV-related opportunistic skin infection (diagnosed 2020-01-15)
- Oral hairy leukoplakia (diagnosed 2020-09-15)
- Human immunodeficiency virus (HIV) infection, diagnosed 10 years prior, with intermittent antiretroviral therapy adherence
- History of Pneumocystis jirovecii pneumonia
- History of oropharyngeal candidiasis
- Polysubstance use disorder (cocaine and methamphetamine), previous intranasal and smoked use, last use 6 months ago
- Unstable housing status, currently in transitional housing

PAST SURGICAL HISTORY:
- Tonsillectomy, age 12

MEDICATIONS:
Home Medications:
- Bictegravir/Tenofovir alafenamide/Emtricitabine 50/25/200 mg tablet, one daily
- Trimethoprim-sulfamethoxazole double strength tablet, three times weekly (for PCP prophylaxis)
- Fluconazole 100 mg tablet, daily as needed for thrush
- Acetaminophen 500 mg tablet, as needed for pain or fever

ALLERGIES:
He has no known drug allergies.

FAMILY HISTORY:
- Father with hypertension
- Mother with type 2 diabetes mellitus
- No family history of hematologic malignancies or primary immunodeficiency disorders

SOCIAL HISTORY:
He is sexually active with multiple partners and does not consistently use protection.

REVIEW OF SYSTEMS:
He reports no cough, diarrhea, or night sweats.

VITAL SIGNS:
On examination, his temperature is 38.2°C, blood pressure is 120/75 mmHg, pulse is 88 beats per minute, and BMI is 21 kg/m^2.

PHYSICAL EXAMINATION:
Oral examination reveals multiple white, non-removable plaques with a corrugated surface on the lateral aspects of his tongue. There are no other oral lesions. The remainder of the physical examination is unremarkable.

LABORATORY DATA:
Laboratory tests reveal:

- CD4+ T-cell count: 180 cells/µL
- HIV antibody/antigen test: Pending
- Rapid plasma reagin (RPR): Non-reactive
- Hepatitis B surface antigen (HBsAg): Negative
- Hepatitis C antibody: Negative

## Encounter 3: source 2022-03-15, practice 2023-02-27, outpatient, Gastroenterology

Chief complaint: Difficulty swallowing and painful swallowing for the past two weeks with sensation of food sticking in mid-chest.

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. James Thompson, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Difficulty swallowing and painful swallowing for the past two weeks with sensation of food sticking in mid-chest.

HISTORY OF PRESENT ILLNESS:
The patient, with a ten-year history of HIV infection complicated by prior opportunistic infections including oral hairy leukoplakia and intermittent antiretroviral therapy adherence, presents with new gastrointestinal symptoms. He reports a sensation of food sticking in his mid-chest area. He also mentions a recent onset of oral white patches that are slightly painful. His HIV has been poorly controlled, and he has not been compliant with his antiretroviral therapy.

PAST MEDICAL HISTORY:
Active Problem List:
- HIV-related opportunistic skin infection (diagnosed 2020-01-15)
- Oral hairy leukoplakia (diagnosed 2020-09-15)
- Human immunodeficiency virus (HIV) infection, diagnosed 10 years prior, with intermittent antiretroviral therapy adherence
- History of Pneumocystis jirovecii pneumonia
- History of oropharyngeal candidiasis
- Polysubstance use disorder (cocaine and methamphetamine), previous intranasal and smoked use, last use 6 months ago
- Unstable housing status, currently in transitional housing

PAST SURGICAL HISTORY:
- Tonsillectomy, age 12

MEDICATIONS:
Home Medications:
- Bictegravir/Tenofovir alafenamide/Emtricitabine 50/25/200 mg tablet, one daily
- Trimethoprim-sulfamethoxazole double strength tablet, three times weekly (for PCP prophylaxis)
- Fluconazole 100 mg tablet, daily as needed for thrush
- Acetaminophen 500 mg tablet, as needed for pain or fever

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with hypertension
- Mother with type 2 diabetes mellitus
- No family history of hematologic malignancies or primary immunodeficiency disorders

SOCIAL HISTORY:
Smoking: Former smoker, quit 3 years ago, 10 pack-year history. Alcohol: Occasional, 1-2 drinks per month. Occupation: Construction laborer, intermittently employed.

VITAL SIGNS:
Temperature is 37.8°C (100°F), blood pressure is 125/78 mmHg, pulse is 88/min, and respiratory rate is 16/min

PHYSICAL EXAMINATION:
On physical examination, oral thrush is noted

LABORATORY DATA:
- CD4+ count: 50 cells/mm³
- HIV viral load: 120,000 copies/mL
- Complete blood count: within normal limits
- Basic metabolic panel: within normal limits

OTHER STUDIES:
Upper gastrointestinal endoscopy is performed, which shows linear yellow-white plaques along the esophagus. Biopsy specimens are taken.

## Encounter 4: source 2023-01-15, practice 2024-01-03, outpatient, Oncology

Chief complaint: Several weeks of fatigue and appearance of new violaceous skin lesions on lower legs and palate with 5-kg weight loss.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Emily Watson, MD
Department: Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Several weeks of fatigue and appearance of new violaceous skin lesions on lower legs and palate with 5-kg weight loss.

HISTORY OF PRESENT ILLNESS:
This 41-year-old male with HIV diagnosed 10 years ago, intermittent ART adherence, and prior Pneumocystis pneumonia and candidal esophagitis presents with several weeks of fatigue and skin findings. He reports noticing multiple painless, violaceous patches and plaques on his lower legs and the roof of his mouth. He has also experienced a 5-kg unintentional weight loss over the past 2 months and occasional early satiety. He denies fever, night sweats, or cough. He has not seen an infectious disease specialist in over a year.

PAST MEDICAL HISTORY:
Active Problem List:
- HIV-related opportunistic skin infection (diagnosed 2020-01-15)
- Oral hairy leukoplakia (diagnosed 2020-09-15)
- Candidal esophagitis (diagnosed 2022-03-15)
- Human immunodeficiency virus (HIV) infection, diagnosed 10 years prior, with intermittent antiretroviral therapy adherence
- History of Pneumocystis jirovecii pneumonia
- History of oropharyngeal candidiasis
- Polysubstance use disorder (cocaine and methamphetamine), previous intranasal and smoked use, last use 6 months ago
- Unstable housing status, currently in transitional housing

His medical history is significant for HIV diagnosed 4 years ago. He has a history of Pneumocystis pneumonia 2 years ago and oral thrush last year.

PAST SURGICAL HISTORY:
- Tonsillectomy, age 12

MEDICATIONS:
He takes he has been inconsistently taking his antiretroviral therapy. He is not on any prophylactic antibiotics.

Home Medications:
- Bictegravir/Tenofovir alafenamide/Emtricitabine 50/25/200 mg tablet, one daily
- Trimethoprim-sulfamethoxazole double strength tablet, three times weekly (for PCP prophylaxis)
- Fluconazole 100 mg tablet, daily as needed for thrush
- Acetaminophen 500 mg tablet, as needed for pain or fever

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with hypertension
- Mother with type 2 diabetes mellitus
- No family history of hematologic malignancies or primary immunodeficiency disorders

SOCIAL HISTORY:
Housing instability

PHYSICAL EXAMINATION:
On physical examination, he appears thin and pale. Multiple violaceous, non-blanching plaques are seen on the bilateral lower extremities and the hard palate. No lymphadenopathy is appreciated. Abdominal examination is soft and non-tender with no hepatosplenomegaly.

LABORATORY DATA:
Hemoglobin is 11 g/dL.

## Encounter 5: source 2023-09-15, practice 2024-08-22, ed, Emergency Medicine

Chief complaint: Worsening shortness of breath, dry cough, and low-grade fevers over the past 10 days with progressive fatigue.

ENCOUNTER NOTE
Date: 2023-09-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening shortness of breath, dry cough, and low-grade fevers over the past 10 days with progressive fatigue.

HISTORY OF PRESENT ILLNESS:
This 41-year-old male with HIV infection, intermittent ART adherence, and prior Pneumocystis jirovecii pneumonia and Kaposi sarcoma presents with respiratory symptoms. He is brought to the emergency department by his roommate. He reports feeling progressively fatigued and notes difficulty climbing stairs or walking short distances without becoming winded. His roommate mentions the patient has lost a significant amount of weight over the past 3 months and has had occasional white plaques on his tongue. The patient denies sputum production, chest pain, or hemoptysis.

PAST MEDICAL HISTORY:
Active Problem List:
- HIV-related opportunistic skin infection (diagnosed 2020-01-15)
- Oral hairy leukoplakia (diagnosed 2020-09-15)
- Candidal esophagitis (diagnosed 2022-03-15)
- Kaposi sarcoma (AIDS-related) (diagnosed 2023-01-15)
- Human immunodeficiency virus (HIV) infection, diagnosed 10 years prior, with intermittent antiretroviral therapy adherence
- History of Pneumocystis jirovecii pneumonia
- History of oropharyngeal candidiasis
- Polysubstance use disorder (cocaine and methamphetamine), previous intranasal and smoked use, last use 6 months ago
- Unstable housing status, currently in transitional housing

He has not seen a physician in several years and is unaware of any chronic medical conditions.

PAST SURGICAL HISTORY:
- Tonsillectomy, age 12

MEDICATIONS:
Home Medications:
- Bictegravir/Tenofovir alafenamide/Emtricitabine 50/25/200 mg tablet, one daily
- Trimethoprim-sulfamethoxazole double strength tablet, three times weekly (for PCP prophylaxis)
- Fluconazole 100 mg tablet, daily as needed for thrush
- Acetaminophen 500 mg tablet, as needed for pain or fever

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with hypertension
- Mother with type 2 diabetes mellitus
- No family history of hematologic malignancies or primary immunodeficiency disorders

SOCIAL HISTORY:
Smoking: Former smoker, quit 3 years ago, 10 pack-year history. Alcohol: Occasional, 1-2 drinks per month. Occupation: Construction laborer, intermittently employed.

VITAL SIGNS:
Oxygen saturation of 89% on room air

PHYSICAL EXAMINATION:
On examination, he appears thin and mildly tachypneic. Oral thrush is noted, and auscultation reveals faint bilateral crackles.

LABORATORY DATA:
Initial labs show:
WBC 4,300/mm³
CD4 count 62/mm³
HIV-1 viral load >500,000 copies/mL
ABG on room air PaO₂ 63 mmHg

IMAGING:
Chest x-ray is shown.

## Encounter 6: source 2024-09-15, practice 2025-08-20, ed, Emergency Medicine

Chief complaint: Brought to emergency department by partner after experiencing a generalized seizure.

ENCOUNTER NOTE
Date: 2024-09-15
Provider: Dr. Lisa Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Brought to emergency department by partner after experiencing a generalized seizure.

HISTORY OF PRESENT ILLNESS:
The patient with longstanding HIV and prior Pneumocystis jirovecii pneumonia and Kaposi sarcoma presents after a generalized seizure. He has not been compliant with his antiretroviral therapy for over a year because of the side-effects but has been compliant with his trimethoprim-sulfamethoxazole prophylaxis.

PAST MEDICAL HISTORY:
Active Problem List:
- HIV-related opportunistic skin infection (diagnosed 2020-01-15)
- Oral hairy leukoplakia (diagnosed 2020-09-15)
- Candidal esophagitis (diagnosed 2022-03-15)
- Kaposi sarcoma (AIDS-related) (diagnosed 2023-01-15)
- History of Pneumocystis jirovecii pneumonia (diagnosed 2023-09-15)
- Human immunodeficiency virus (HIV) infection, diagnosed 10 years prior, with intermittent antiretroviral therapy adherence
- History of oropharyngeal candidiasis
- Polysubstance use disorder (cocaine and methamphetamine), previous intranasal and smoked use, last use 6 months ago
- Unstable housing status, currently in transitional housing

PAST SURGICAL HISTORY:
- Tonsillectomy, age 12

MEDICATIONS:
Home Medications:
- Bictegravir/Tenofovir alafenamide/Emtricitabine 50/25/200 mg tablet, one daily
- Trimethoprim-sulfamethoxazole double strength tablet, three times weekly (for PCP prophylaxis)
- Fluconazole 100 mg tablet, daily as needed for thrush
- Acetaminophen 500 mg tablet, as needed for pain or fever

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with hypertension
- Mother with type 2 diabetes mellitus
- No family history of hematologic malignancies or primary immunodeficiency disorders

SOCIAL HISTORY:
Smoking: Former smoker, quit 3 years ago, 10 pack-year history. Alcohol: Occasional, 1-2 drinks per month. Occupation: Construction laborer, intermittently employed.

LABORATORY DATA:
CD4 count is 35/μL.

IMAGING:
A CT of the head shows a ring-enhancing lesion.

## Encounter 7: source 2025-05-15, practice 2026-05-01, ed, Emergency Medicine

Chief complaint: Six days of profuse watery diarrhea, abdominal cramping, and fatigue with 4-kg weight loss.

ENCOUNTER NOTE
Date: 2025-05-15
Provider: Dr. David Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Six days of profuse watery diarrhea, abdominal cramping, and fatigue with 4-kg weight loss.

HISTORY OF PRESENT ILLNESS:
This 43-year-old male with longstanding HIV infection, prior Pneumocystis jirovecii pneumonia, and non-Hodgkin lymphoma presents with six days of profuse watery diarrhea, abdominal cramping, and fatigue. He denies blood or mucus in the stool and has lost 4 kg (9 lb) over the past week. He recently attended an outdoor picnic where he ate raw vegetables but has not traveled internationally or taken antibiotics. He drinks bottled water at home.

PAST MEDICAL HISTORY:
Active Problem List:
- HIV-related opportunistic skin infection (diagnosed 2020-01-15)
- Oral hairy leukoplakia (diagnosed 2020-09-15)
- Candidal esophagitis (diagnosed 2022-03-15)
- Kaposi sarcoma (AIDS-related) (diagnosed 2023-01-15)
- History of Pneumocystis jirovecii pneumonia (diagnosed 2023-09-15)
- Non-Hodgkin lymphoma (diagnosed 2024-09-15)
- Human immunodeficiency virus (HIV) infection, diagnosed 10 years prior, with intermittent antiretroviral therapy adherence
- History of oropharyngeal candidiasis
- Polysubstance use disorder (cocaine and methamphetamine), previous intranasal and smoked use, last use 6 months ago
- Unstable housing status, currently in transitional housing

PAST SURGICAL HISTORY:
- Tonsillectomy, age 12

MEDICATIONS:
Home Medications:
- Bictegravir/Tenofovir alafenamide/Emtricitabine 50/25/200 mg tablet, one daily
- Trimethoprim-sulfamethoxazole double strength tablet, three times weekly (for PCP prophylaxis)
- Fluconazole 100 mg tablet, daily as needed for thrush
- Acetaminophen 500 mg tablet, as needed for pain or fever

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with hypertension
- Mother with type 2 diabetes mellitus
- No family history of hematologic malignancies or primary immunodeficiency disorders

SOCIAL HISTORY:
Smoking: Former smoker, quit 3 years ago, 10 pack-year history. Alcohol: Occasional, 1-2 drinks per month. Occupation: Construction laborer, intermittently employed.

VITAL SIGNS:
Febrile at 38 C

PHYSICAL EXAMINATION:
Mildly dehydrated, and has no abdominal tenderness

LABORATORY DATA:
Modified acid-fast staining of the stool reveals round oocysts measuring 4–6 microns

PLAN:
He is started on antiretroviral therapy and supportive care
