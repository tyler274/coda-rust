# synthetic_hospital patient 2580

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 27 at first encounter, sex M, Non-Hispanic White, University student health insurance with TRICARE Reserve Select.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-08-09 (Tue) | outpatient | Gastroenterology |
| 1 | 2020-09-15 | 2023-04-06 (Thu) | outpatient | Dermatology |
| 2 | 2022-09-15 | 2025-04-03 (Thu) | ed | Emergency Medicine |
| 3 | 2023-03-15 | 2025-10-07 (Tue) | inpatient | Internal Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "University student health insurance with TRICARE Reserve Select",
 "occupation": "Graduate student and U.S. Army Reserve member",
 "alcohol_use": "Rare (1-2 drinks per month) due to underlying hepatobiliary condition",
 "family_history": [
  "Mother with rheumatoid arthritis",
  "Maternal uncle with ulcerative colitis"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "300 mg PO three times daily",
   "name": "Ursodiol"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Primary sclerosing cholangitis"
 ],
 "age_at_first_encounter": 27
}
```

Primary diagnoses: ["Histoplasmosis", "Visceral leishmaniasis", "Pemphigus Vulgaris", "Extensive ulcerative colitis (pancolitis)"]
Comorbidities: ["Anemia"]

## Encounter 0: source 2020-01-15, practice 2022-08-09, outpatient, Gastroenterology

Chief complaint: Bloody diarrhea and intermittent abdominal cramping for 6 months with 10-pound weight loss

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Patel, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Bloody diarrhea and intermittent abdominal cramping for 6 months with 10-pound weight loss

HISTORY OF PRESENT ILLNESS:
He presents to the clinic with a 6-month history of bloody diarrhea and intermittent abdominal cramping. He reports a 10-pound weight loss and low-grade fevers over the past two months.

PAST MEDICAL HISTORY:
Active Problem List:
- Primary sclerosing cholangitis

His medical history is significant for primary sclerosing cholangitis.

MEDICATIONS:
Home Medications:
- Ursodiol 300 mg PO three times daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with rheumatoid arthritis
- Maternal uncle with ulcerative colitis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (1-2 drinks per month) due to underlying hepatobiliary condition. Occupation: Graduate student and U.S. Army Reserve member.

PHYSICAL EXAMINATION:
On physical examination, he appears fatigued, and his abdomen is tender to palpation in the left lower quadrant without rebound or guarding. Rectal examination reveals blood-streaked stool.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 11.2 g/dL (normal: 13.8-17.2 g/dL)
- Mean corpuscular volume (MCV): 86 fL (normal: 80-100 fL)
- C-reactive protein (CRP): 24 mg/L (normal: <5 mg/L)
- Erythrocyte sedimentation rate (ESR): 45 mm/hr (normal: 0-22 mm/hr)

PATHOLOGY:
Biopsies from the colon reveal mucosal inflammation and crypt abscesses.

OTHER STUDIES:
Colonoscopy shows continuous mucosal inflammation and ulceration starting from the rectum and extending proximally to the entire colon.

## Encounter 1: source 2020-09-15, practice 2023-04-06, outpatient, Dermatology

Chief complaint: Painful sores in mouth and blistering skin lesions on chest and back for two weeks

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. James Okafor, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Painful sores in mouth and blistering skin lesions on chest and back for two weeks

HISTORY OF PRESENT ILLNESS:
Given his primary sclerosing cholangitis and ulcerative colitis diagnosed in January, he has noticed several areas of skin irritation and blistering on his chest and back that have worsened despite avoiding new soaps or detergents.

PAST MEDICAL HISTORY:
Active Problem List:
- Extensive ulcerative colitis (pancolitis) (diagnosed 2020-01-15)
- Primary sclerosing cholangitis

MEDICATIONS:
Home Medications:
- Ursodiol 300 mg PO three times daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with rheumatoid arthritis
- Maternal uncle with ulcerative colitis

SOCIAL HISTORY:
He works as a graduate student

REVIEW OF SYSTEMS:
He denies fever, joint pain, or recent travel.

PHYSICAL EXAMINATION:
On examination, there are multiple flaccid blisters and erosions with surrounding erythema on his anterior chest. Gentle traction on the skin near the lesions causes extension and new erosions. Oral mucosal examination reveals shallow erosions along the buccal mucosa and soft palate.

LABORATORY DATA:
Laboratory evaluation is unremarkable except for mild anemia.

## Encounter 2: source 2022-09-15, practice 2025-04-03, ed, Emergency Medicine

Chief complaint: Persistent fever, cough, and malaise for 10 days following cave exploration in Mississippi River valley

ENCOUNTER NOTE
Date: 2022-09-15
Provider: Dr. Sarah Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Persistent fever, cough, and malaise for 10 days following cave exploration in Mississippi River valley

HISTORY OF PRESENT ILLNESS:
This 29-year-old male with primary sclerosing cholangitis on ursodiol, ulcerative colitis, and pemphigus vulgaris presents with progressive symptoms. He returned 2 weeks ago from a camping trip along the Mississippi River valley, where he explored caves and old barns. Since then, he has developed progressive fatigue, mild weight loss, and occasional chest discomfort. He denies hemoptysis, recent sick contacts, or prior lung disease.

PAST MEDICAL HISTORY:
Active Problem List:
- Extensive ulcerative colitis (pancolitis) (diagnosed 2020-01-15)
- Pemphigus Vulgaris (diagnosed 2020-09-15)
- Primary sclerosing cholangitis

MEDICATIONS:
Home Medications:
- Ursodiol 300 mg PO three times daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with rheumatoid arthritis
- Maternal uncle with ulcerative colitis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (1-2 drinks per month) due to underlying hepatobiliary condition. Occupation: Graduate student and U.S. Army Reserve member.

PHYSICAL EXAMINATION:
Examination shows scattered oral ulcers and tender cervical lymphadenopathy. Breath sounds are decreased with fine bibasilar crackles.

LABORATORY DATA:
Laboratory studies show mild normocytic anemia and elevated lactate dehydrogenase.

IMAGING:
A chest radiograph reveals bilateral hilar lymphadenopathy with patchy reticulonodular infiltrates.

OTHER STUDIES:
A sputum sample is sent for fungal culture.

## Encounter 3: source 2023-03-15, practice 2025-10-07, inpatient, Internal Medicine

Chief complaint: Three weeks of fatigue, subjective fevers, progressive malaise, and non-healing cutaneous lesion

ENCOUNTER NOTE
Date: 2023-03-15
Provider: Dr. Carlos Mendez, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Three weeks of fatigue, subjective fevers, progressive malaise, and non-healing cutaneous lesion

HISTORY OF PRESENT ILLNESS:
Patient with PSC, UC, pemphigus, and histoplasmosis has three weeks of symptoms. He denies weight loss but reports decreased appetite and generalized weakness. Lesion remains unhealed despite local wound care.

PAST MEDICAL HISTORY:
Active Problem List:
- Extensive ulcerative colitis (pancolitis) (diagnosed 2020-01-15)
- Pemphigus Vulgaris (diagnosed 2020-09-15)
- Histoplasmosis (diagnosed 2022-09-15)
- Primary sclerosing cholangitis

MEDICATIONS:
Home Medications:
- Ursodiol 300 mg PO three times daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with rheumatoid arthritis
- Maternal uncle with ulcerative colitis

SOCIAL HISTORY:
He recently returned from a military assignment in the Middle East, where he spent several months conducting patrols in remote desert areas. He reports being bitten by insects frequently but denies any recent animal exposures or freshwater swimming.

PHYSICAL EXAMINATION:
On physical examination, he appears tired, with mild hepatosplenomegaly and a 1.5-cm ulcerated lesion with raised borders on his right forearm.

LABORATORY DATA:
Initial laboratory testing reveals a hemoglobin level of 9.1 g/dL, WBC count of 2,200/mm³, and platelet count of 85,000/mm³. Liver enzymes are mildly elevated. Blood cultures are negative. Rapid malaria testing is negative.

PATHOLOGY:
A skin punch biopsy of the lesion shows intracellular round and oval structures within histiocytes on Giemsa stain.
