# synthetic_hospital patient 2081

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 38 at first encounter, sex M, African American, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-10-11 (Fri) | outpatient | Infectious Diseases |
| 1 | 2020-09-15 | 2025-06-28 (Sat) | ed | Emergency Medicine |
| 2 | 2021-05-15 | 2026-02-10 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Construction worker",
 "alcohol_use": "Social (few beers on weekends)",
 "family_history": [
  "Father with type 2 diabetes mellitus",
  "Mother with essential hypertension"
 ],
 "race_ethnicity": "African American",
 "smoking_status": "Never",
 "home_medications": [
  {
   "dose": "200mg/300mg PO daily",
   "name": "Emtricitabine-Tenofovir DF"
  },
  {
   "dose": "50mg PO daily",
   "name": "Dolutegravir"
  },
  {
   "dose": "160mg/800mg (DS) PO daily",
   "name": "Trimethoprim-Sulfamethoxazole"
  },
  {
   "dose": "100mg PO daily",
   "name": "Fluconazole"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Human Immunodeficiency Virus/Acquired Immunodeficiency Syndrome (HIV/AIDS)",
  "History of Kaposi sarcoma",
  "Recurrent oropharyngeal candidiasis"
 ],
 "age_at_first_encounter": 38
}
```

Primary diagnoses: ["Progressive disseminated histoplasmosis", "Hypertriglyceridemia-induced acute pancreatitis", "Primary central nervous system lymphoma"]
Comorbidities: ["HIV-related opportunistic skin infection", "Oral candidiasis"]

## Encounter 0: source 2020-01-15, practice 2024-10-11, outpatient, Infectious Diseases

Chief complaint: Two-week history of fever, night sweats, and non-productive cough with unintentional weight loss of 10 pounds over the past month.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Maria Rodriguez, MD
Department: Infectious Diseases
Visit Type: outpatient

CHIEF COMPLAINT:
Two-week history of fever, night sweats, and non-productive cough with unintentional weight loss of 10 pounds over the past month.

HISTORY OF PRESENT ILLNESS:
He reports unintentional weight loss of 10 pounds over the last month. His antiretroviral therapy has been irregular due to social circumstances.

PAST MEDICAL HISTORY:
Active Problem List:
- Human Immunodeficiency Virus/Acquired Immunodeficiency Syndrome (HIV/AIDS)
- History of Kaposi sarcoma
- Recurrent oropharyngeal candidiasis

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Emtricitabine-Tenofovir DF 200mg/300mg PO daily
- Dolutegravir 50mg PO daily
- Trimethoprim-Sulfamethoxazole 160mg/800mg (DS) PO daily
- Fluconazole 100mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with type 2 diabetes mellitus
- Mother with essential hypertension

SOCIAL HISTORY:
Smoking: Never. Alcohol: Social (few beers on weekends). Occupation: Construction worker.

PHYSICAL EXAMINATION:
On examination, he appears cachectic, with oral thrush and diffuse lymphadenopathy. His lung examination reveals bilateral crackles.

LABORATORY DATA:
Laboratory tests show:

- CD4 count: 50/mm3
- Hemoglobin: 9.8 g/dL
- White blood cell count: 3,000/mm3
- Platelet count: 150,000/mm3

IMAGING:
- Chest X-ray: diffuse reticulonodular infiltrates

## Encounter 1: source 2020-09-15, practice 2025-06-28, ed, Emergency Medicine

Chief complaint: Severe, persistent upper abdominal pain radiating to the back, worsened by eating, associated with non-bloody vomiting and inability to tolerate solids for two days.

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe, persistent upper abdominal pain radiating to the back, worsened by eating, associated with non-bloody vomiting and inability to tolerate solids for two days.

HISTORY OF PRESENT ILLNESS:
The patient is a 38-year-old male with HIV/AIDS, Kaposi sarcoma, and recurrent oropharyngeal candidiasis, previously treated for disseminated histoplasmosis, presenting with abdominal pain. He describes the pain as constant and worsened by eating. He has had several episodes of non-bloody vomiting and has not eaten solid food in two days. He reports subjective fevers and general malaise. There is no history of trauma, recent ERCP, or significant alcohol use.

PAST MEDICAL HISTORY:
Active Problem List:
- Progressive disseminated histoplasmosis (diagnosed 2020-01-15)
- Human Immunodeficiency Virus/Acquired Immunodeficiency Syndrome (HIV/AIDS)
- History of Kaposi sarcoma
- Recurrent oropharyngeal candidiasis

The patient has a history of AIDS, diagnosed three years ago, and was recently switched to a new antiretroviral regimen due to concerns about resistance.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
His other medications include trimethoprim-sulfamethoxazole for Pneumocystis prophylaxis and fluconazole for oral thrush.

Home Medications:
- Emtricitabine-Tenofovir DF 200mg/300mg PO daily
- Dolutegravir 50mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with type 2 diabetes mellitus
- Mother with essential hypertension

SOCIAL HISTORY:
He drinks alcohol socially, usually a few beers on weekends, and does not use tobacco or recreational drugs.

VITAL SIGNS:
Vital signs are normal except for a mild fever.

PHYSICAL EXAMINATION:
On exam, he appears uncomfortable and is tender to palpation in the epigastric region without rebound or guarding.

LABORATORY DATA:
Laboratory evaluation reveals an elevated amylase and lipase level. Liver function tests are within normal limits.
His most recent CD4 count was 180/mm³.

IMAGING:
Right upper quadrant ultrasound shows a normal gallbladder without gallstones, biliary sludge, or bile duct dilation.

## Encounter 2: source 2021-05-15, practice 2026-02-10, ed, Emergency Medicine

Chief complaint: Two-week history of progressive headache, confusion, and fever with noted poor adherence to antiretroviral therapy.

ENCOUNTER NOTE
Date: 2021-05-15
Provider: Dr. Aisha Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-week history of progressive headache, confusion, and fever with noted poor adherence to antiretroviral therapy.

HISTORY OF PRESENT ILLNESS:
The patient with HIV/AIDS, history of Kaposi sarcoma, and recurrent oropharyngeal candidiasis returns for care following prior encounters for febrile respiratory symptoms and acute abdominal pain. He presents with a two-week history of progressive headache, confusion, and fever, and admits to poor adherence to his antiretroviral therapy.

PAST MEDICAL HISTORY:
Active Problem List:
- Progressive disseminated histoplasmosis (diagnosed 2020-01-15)
- Hypertriglyceridemia-induced acute pancreatitis (diagnosed 2020-09-15)
- Human Immunodeficiency Virus/Acquired Immunodeficiency Syndrome (HIV/AIDS)
- History of Kaposi sarcoma
- Recurrent oropharyngeal candidiasis

Past medical history significant for oral candidiasis and Kaposi sarcoma.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
His current medications include emtricitabine-tenofovir and dolutegravir.

Home Medications:
- Emtricitabine-Tenofovir DF 200mg/300mg PO daily
- Trimethoprim-Sulfamethoxazole 160mg/800mg (DS) PO daily
- Fluconazole 100mg PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with type 2 diabetes mellitus
- Mother with essential hypertension

SOCIAL HISTORY:
Smoking: Never. Alcohol: Social (few beers on weekends). Occupation: Construction worker.

PHYSICAL EXAMINATION:
On examination, he is febrile with a temperature of 38.7°C, and neurological examination reveals mild left-sided weakness.

LABORATORY DATA:
Lumbar puncture is performed, and cerebrospinal fluid analysis is significant for elevated protein, low glucose, and the presence of Epstein-Barr virus DNA.

Laboratory Values:
- CD4+ count: 45 cells/µL
- HIV viral load: 120,000 copies/mL
- CSF protein: 100 mg/dL (normal: 15-45 mg/dL)
- CSF glucose: 30 mg/dL (normal: 45-80 mg/dL)
- CSF Epstein-Barr virus DNA: Positive

IMAGING:
An MRI of the brain demonstrates a solitary, irregular, ring-enhancing mass in the right cerebral hemisphere.
