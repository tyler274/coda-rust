# synthetic_hospital patient 2647

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 26 at first encounter, sex M, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-01-08 (Mon) | outpatient | Internal Medicine |
| 1 | 2021-01-15 | 2024-12-23 (Mon) | outpatient | Hematology |
| 2 | 2022-03-15 | 2026-02-24 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Farm laborer",
 "alcohol_use": "None",
 "family_history": [
  "Mother with lifelong history of recurrent epistaxis",
  "Maternal grandfather with frequent nosebleeds and unexplained iron deficiency anemia"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [],
 "chronic_conditions": [],
 "age_at_first_encounter": 26
}
```

Primary diagnoses: ["Ancylostomiasis (Hookworm infection)", "Necator americanus infection", "Hereditary Hemorrhagic Telangiectasia (Osler-Weber-Rendu Syndrome)"]
Comorbidities: ["Iron Deficiency Anemia"]

## Encounter 0: source 2020-01-15, practice 2024-01-08, outpatient, Internal Medicine

Chief complaint: Recurrent spontaneous nosebleeds over the past year with increasing fatigue

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Sharma, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Recurrent spontaneous nosebleeds over the past year with increasing fatigue

HISTORY OF PRESENT ILLNESS:
He reports several spontaneous episodes over the past year, which usually resolve with pressure but occasionally require packing. He also describes increasing fatigue and shortness of breath when climbing stairs.

PAST MEDICAL HISTORY:
He has no history of bleeding disorders.

ALLERGIES:
NKDA

FAMILY HISTORY:
Family history is notable for similar symptoms in his father.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Farm laborer.

PHYSICAL EXAMINATION:
On exam, conjunctivae are pale, and there are numerous pinpoint red lesions on the lips, tongue, and oral mucosa, as shown in the image.

LABORATORY DATA:
Laboratory testing reveals hemoglobin of 9.1 g/dL and mean corpuscular volume of 72 fL.

IMAGING:
A chest CT scan reveals multiple vascular malformations in the lungs.

## Encounter 1: source 2021-01-15, practice 2024-12-23, outpatient, Hematology

Chief complaint: Worsening fatigue and decreased exercise tolerance over the past 4 months

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. James Okafor, MD
Department: Hematology
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening fatigue and decreased exercise tolerance over the past 4 months

HISTORY OF PRESENT ILLNESS:
Patient with Hereditary Hemorrhagic Telangiectasia returns for evaluation. He works as a farm laborer in a rural area and spends most of his time outdoors.

PAST MEDICAL HISTORY:
Active Problem List:
- Hereditary Hemorrhagic Telangiectasia (Osler-Weber-Rendu Syndrome) (diagnosed 2020-01-15)

He has not seen a doctor in several years.

MEDICATIONS:
He takes no medications

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with lifelong history of recurrent epistaxis
- Maternal grandfather with frequent nosebleeds and unexplained iron deficiency anemia

SOCIAL HISTORY:
He does not smoke, drink alcohol, or use illicit drugs.

REVIEW OF SYSTEMS:
He denies overt bleeding, easy bruising, night sweats, or recent illness. He has had occasional mild abdominal discomfort and intermittent loose stools but no nausea, vomiting, or jaundice.

VITAL SIGNS:
Vital signs are within normal limits.

PHYSICAL EXAMINATION:
On examination, he appears pale and thin. There is mild conjunctival pallor, spoon-shaped nails, and a grade 2/6 systolic flow murmur at the left sternal border. Cardiopulmonary examination is otherwise unremarkable. Abdominal examination reveals mild diffuse tenderness without organomegaly.

LABORATORY DATA:
Laboratory studies show:
Hemoglobin 9.3 g/dL
Mean corpuscular volume 70 fL
Ferritin 7 ng/mL
Leukocyte count 7,800/mm³
Eosinophils 8%

OTHER STUDIES:
Stool microscopy reveals oval, thin-shelled eggs.

## Encounter 2: source 2022-03-15, practice 2026-02-24, ed, Emergency Medicine

Chief complaint: Acute onset severe shortness of breath on exertion and profound fatigue after recent return from Southeast Asia

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. Elena Vasquez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset severe shortness of breath on exertion and profound fatigue after recent return from Southeast Asia

HISTORY OF PRESENT ILLNESS:
The patient is a 28-year-old male with established Hereditary Hemorrhagic Telangiectasia diagnosed in 2020 and prior Necator americanus infection managed in 2021 who presents with acute onset severe shortness of breath on exertion and profound fatigue. He recently returned to the United States after spending 6 months volunteering on rural agricultural development projects in Southeast Asia. While abroad, he often walked barefoot in the fields and recalls having a mildly pruritic rash on the soles of his feet that resolved spontaneously. He denies any overt bleeding, weight loss, or dietary changes.

PAST MEDICAL HISTORY:
Active Problem List:
- Hereditary Hemorrhagic Telangiectasia (Osler-Weber-Rendu Syndrome) (diagnosed 2020-01-15)
- Necator americanus infection (diagnosed 2021-01-15)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with lifelong history of recurrent epistaxis
- Maternal grandfather with frequent nosebleeds and unexplained iron deficiency anemia

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Farm laborer.

REVIEW OF SYSTEMS:
Review of systems is negative for fever, melena, or hematochezia.

PHYSICAL EXAMINATION:
On physical exam, he appears pale and slightly tachycardic. His conjunctivae are pale, and mild systolic flow murmur is heard on auscultation. Abdominal exam is unremarkable, and there is no lymphadenopathy.

LABORATORY DATA:
Laboratory studies reveal:
Hemoglobin 9.6 g/dL
MCV 72 fL
Ferritin Low
Reticulocyte count Mildly elevated

PLAN:
A stool ova and parasite exam is ordered.
