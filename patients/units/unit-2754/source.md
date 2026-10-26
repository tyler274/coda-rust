# synthetic_hospital patient 2754

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 46 at first encounter, sex F, Non-Hispanic White, Private insurance through employer.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-06-09 (Thu) | outpatient | Dermatology |
| 1 | 2020-04-15 | 2022-09-05 (Mon) | outpatient | Dermatology |
| 2 | 2022-01-15 | 2024-06-17 (Mon) | outpatient | Neurology |
| 3 | 2024-01-15 | 2026-06-15 (Mon) | outpatient | Hematology/Oncology |
| 4 | 2024-02-15 | 2026-07-08 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Rituximab (infusion-related reaction: fever, chills, hypotension)"
 ],
 "insurance": "Private insurance through employer",
 "occupation": "Landscaping supervisor",
 "alcohol_use": "Former heavy use (6-10 drinks/week), quit 3 months ago upon lymphoma diagnosis",
 "family_history": [
  "Mother: Hypothyroidism",
  "Father: Hypertension, type 2 diabetes",
  "No family history of autoimmune disease or hematologic malignancies"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never",
 "home_medications": [
  {
   "dose": "400 IU PO twice daily",
   "name": "Vitamin E (alpha-tocopherol)"
  },
  {
   "dose": "36,000 units lipase PO with meals and 18,000 units with snacks",
   "name": "Pancrelipase"
  },
  {
   "dose": "325 mg PO daily",
   "name": "Ferrous sulfate"
  },
  {
   "dose": "8 mg PO every 8 hours PRN nausea",
   "name": "Ondansetron"
  }
 ],
 "surgical_history": [
  "Percutaneous liver biopsy (prior staging for hepatitis C)",
  "Bone marrow biopsy and aspiration (diagnostic for lymphoma)"
 ],
 "chronic_conditions": [
  "Chronic hepatitis C infection",
  "Mixed cryoglobulinemia (Type II/III)",
  "Porphyria cutanea tarda",
  "Malabsorption syndrome with chronic steatorrhea",
  "Vitamin E deficiency secondary to fat malabsorption",
  "Chronic iron deficiency anemia secondary to menorrhagia",
  "Diffuse large B-cell lymphoma (currently in first-line chemotherapy)"
 ],
 "age_at_first_encounter": 46
}
```

Primary diagnoses: ["Cryoglobulinemia", "Diffuse large B-cell lymphoma", "Venous Air Embolism", "Porphyria cutanea tarda", "Ataxia with vitamin E deficiency"]
Comorbidities: ["Chronic hepatitis C", "Anemia", "Peripheral neuropathy"]

## Encounter 0: source 2020-01-15, practice 2022-06-09, outpatient, Dermatology

Chief complaint: Blistering skin lesions on the backs of her hands that worsen with sun exposure

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Blistering skin lesions on the backs of her hands that worsen with sun exposure

HISTORY OF PRESENT ILLNESS:
She says the blisters take weeks to heal and often leave behind dark scars.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic hepatitis C infection
- Mixed cryoglobulinemia (Type II/III)
- Porphyria cutanea tarda
- Malabsorption syndrome with chronic steatorrhea
- Vitamin E deficiency secondary to fat malabsorption
- Chronic iron deficiency anemia secondary to menorrhagia
- Diffuse large B-cell lymphoma (currently in first-line chemotherapy)

She has a history of hepatitis C infection.

PAST SURGICAL HISTORY:
- Percutaneous liver biopsy (prior staging for hepatitis C)
- Bone marrow biopsy and aspiration (diagnostic for lymphoma)

MEDICATIONS:
Home Medications:
- Vitamin E (alpha-tocopherol) 400 IU PO twice daily
- Pancrelipase 36,000 units lipase PO with meals and 18,000 units with snacks
- Ferrous sulfate 325 mg PO daily
- Ondansetron 8 mg PO every 8 hours PRN nausea

ALLERGIES:
Rituximab (infusion-related reaction: fever, chills, hypotension)

FAMILY HISTORY:
- Mother: Hypothyroidism
- Father: Hypertension, type 2 diabetes
- No family history of autoimmune disease or hematologic malignancies

SOCIAL HISTORY:
She works as a landscaping supervisor and spends hours outdoors. She has a history of chronic alcohol use.

PHYSICAL EXAMINATION:
Physical examination shows hyperpigmentation and fragile skin on the dorsal hands with several healing blisters.

LABORATORY DATA:
Laboratory studies reveal elevated liver enzymes and increased total plasma porphyrins.

## Encounter 1: source 2020-04-15, practice 2022-09-05, outpatient, Dermatology

Chief complaint: Persistent fatigue, joint stiffness, and a tingling sensation in both feet for the past month

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent fatigue, joint stiffness, and a tingling sensation in both feet for the past month

HISTORY OF PRESENT ILLNESS:
She returns for cryoglobulinemia follow-up after January's blistering. She also reports several episodes of nosebleeds and occasional dark urine in the morning. She denies recent infections or travel.

PAST MEDICAL HISTORY:
Active Problem List:
- Porphyria cutanea tarda (diagnosed 2020-01-15)
- Chronic hepatitis C infection
- Mixed cryoglobulinemia (Type II/III)
- Malabsorption syndrome with chronic steatorrhea
- Vitamin E deficiency secondary to fat malabsorption
- Chronic iron deficiency anemia secondary to menorrhagia
- Diffuse large B-cell lymphoma (currently in first-line chemotherapy)

She has a long-standing history of a "liver issue" for which she was treated several years ago but cannot recall the exact diagnosis.

PAST SURGICAL HISTORY:
- Percutaneous liver biopsy (prior staging for hepatitis C)
- Bone marrow biopsy and aspiration (diagnostic for lymphoma)

MEDICATIONS:
Home Medications:
- Vitamin E (alpha-tocopherol) 400 IU PO twice daily
- Pancrelipase 36,000 units lipase PO with meals and 18,000 units with snacks
- Ferrous sulfate 325 mg PO daily
- Ondansetron 8 mg PO every 8 hours PRN nausea

ALLERGIES:
Rituximab (infusion-related reaction: fever, chills, hypotension)

FAMILY HISTORY:
- Mother: Hypothyroidism
- Father: Hypertension, type 2 diabetes
- No family history of autoimmune disease or hematologic malignancies

SOCIAL HISTORY:
Smoking: Never. Alcohol: Former heavy use (6-10 drinks/week), quit 3 months ago upon lymphoma diagnosis. Occupation: Landscaping supervisor.

VITAL SIGNS:
Vital signs are normal.

PHYSICAL EXAMINATION:
Physical examination reveals scattered petechiae on both legs and decreased pinprick sensation over the feet. Cardiopulmonary and abdominal exams are unremarkable. Neurologic exam reveals mild ankle dorsiflexion weakness bilaterally.

LABORATORY DATA:
Lab findings show:
Hemoglobin 10.5 g/dL
Platelets 145,000/mm³
Creatinine 1.7 mg/dL
AST/ALT Mildly elevated
Urinalysis 2+ protein, red cell casts
ESR Elevated
C3 Normal
C4 Decreased
Hepatitis C antibodies Positive

PATHOLOGY:
A skin biopsy from a leg lesion shows leukocytoclastic vasculitis.

## Encounter 2: source 2022-01-15, practice 2024-06-17, outpatient, Neurology

Chief complaint: Progressive difficulty walking

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Priya Patel, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive difficulty walking

HISTORY OF PRESENT ILLNESS:
The patient returns for evaluation of gait instability in the context of vitamin E deficiency secondary to malabsorption and cryoglobulinemia with peripheral neuropathy. She presents with a 4-month history of progressive difficulty walking. She reports numbness and tingling in both her hands and feet that have gradually worsened. She also describes frequent stumbling and trouble with fine hand movements such as buttoning clothes.

PAST MEDICAL HISTORY:
Active Problem List:
- Porphyria cutanea tarda (diagnosed 2020-01-15)
- Cryoglobulinemia (diagnosed 2020-04-15)
- Chronic hepatitis C infection
- Mixed cryoglobulinemia (Type II/III)
- Malabsorption syndrome with chronic steatorrhea
- Vitamin E deficiency secondary to fat malabsorption
- Chronic iron deficiency anemia secondary to menorrhagia
- Diffuse large B-cell lymphoma (currently in first-line chemotherapy)

She has a history of chronic steatorrhea and intermittent bloating for several years but has not sought medical evaluation.

PAST SURGICAL HISTORY:
- Percutaneous liver biopsy (prior staging for hepatitis C)
- Bone marrow biopsy and aspiration (diagnostic for lymphoma)

MEDICATIONS:
Home Medications:
- Vitamin E (alpha-tocopherol) 400 IU PO twice daily
- Pancrelipase 36,000 units lipase PO with meals and 18,000 units with snacks
- Ferrous sulfate 325 mg PO daily
- Ondansetron 8 mg PO every 8 hours PRN nausea

ALLERGIES:
Rituximab (infusion-related reaction: fever, chills, hypotension)

FAMILY HISTORY:
- Mother: Hypothyroidism
- Father: Hypertension, type 2 diabetes
- No family history of autoimmune disease or hematologic malignancies

SOCIAL HISTORY:
Smoking: Never. Alcohol: Former heavy use (6-10 drinks/week), quit 3 months ago upon lymphoma diagnosis. Occupation: Landscaping supervisor.

REVIEW OF SYSTEMS:
She denies any visual changes, dysphagia, or recent illness. Her appetite is normal, and she has not experienced significant weight loss.

PHYSICAL EXAMINATION:
On examination, she has decreased vibratory sensation and proprioception in all extremities. Muscle strength is slightly reduced diffusely, and she demonstrates a wide-based gait with a positive Romberg sign. Reflexes are diminished in the lower limbs.

LABORATORY DATA:
Routine blood work shows normal hemoglobin, electrolytes, and glucose levels.

## Encounter 3: source 2024-01-15, practice 2026-06-15, outpatient, Hematology/Oncology

Chief complaint: Comes to the hematology clinic for follow-up after completing her first cycle of chemotherapy for a recently diagnosed lymphoid malignancy

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. James O'Brien, MD
Department: Hematology/Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Comes to the hematology clinic for follow-up after completing her first cycle of chemotherapy for a recently diagnosed lymphoid malignancy

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up after her first chemotherapy cycle with history of hepatitis C, cryoglobulinemia, porphyria cutanea tarda, and malabsorption complicated by vitamin E deficiency and ataxia. Six months ago, she began experiencing fatigue, drenching night sweats, and an unintentional 7-kg (15-lb) weight loss. She also noted swelling in her neck and underarms that gradually increased in size. After the first infusion of the monoclonal antibody, she experienced transient fever, chills, and mild hypotension, which resolved with supportive care.

PAST MEDICAL HISTORY:
Active Problem List:
- Porphyria cutanea tarda (diagnosed 2020-01-15)
- Cryoglobulinemia (diagnosed 2020-04-15)
- Ataxia with vitamin E deficiency (diagnosed 2022-01-15)
- Chronic hepatitis C infection
- Mixed cryoglobulinemia (Type II/III)
- Malabsorption syndrome with chronic steatorrhea
- Vitamin E deficiency secondary to fat malabsorption
- Chronic iron deficiency anemia secondary to menorrhagia
- Diffuse large B-cell lymphoma (currently in first-line chemotherapy)

PAST SURGICAL HISTORY:
- Percutaneous liver biopsy (prior staging for hepatitis C)
- Bone marrow biopsy and aspiration (diagnostic for lymphoma)

MEDICATIONS:
Based on these findings, she was started on combination chemotherapy including an alkylating agent, an anthracycline, a vinca alkaloid, and rituximab.

Home Medications:
- Vitamin E (alpha-tocopherol) 400 IU PO twice daily
- Pancrelipase 36,000 units lipase PO with meals and 18,000 units with snacks
- Ferrous sulfate 325 mg PO daily
- Ondansetron 8 mg PO every 8 hours PRN nausea

ALLERGIES:
Rituximab (infusion-related reaction: fever, chills, hypotension)

FAMILY HISTORY:
There is no family history of cancer.

SOCIAL HISTORY:
She does not smoke, drink alcohol, or use illicit drugs.

PHYSICAL EXAMINATION:
On physical examination at diagnosis, she had multiple firm, nontender cervical and axillary lymph nodes. Hepatosplenomegaly was present without abdominal tenderness.

LABORATORY DATA:
Laboratory studies revealed mild normocytic anemia and an elevated lactate dehydrogenase level. Peripheral blood smear showed no abnormal cells.

IMAGING:
CT scans demonstrated diffuse lymphadenopathy above and below the diaphragm.

PATHOLOGY:
An excisional biopsy of a cervical lymph node revealed sheets of atypical lymphoid cells with irregular nuclear contours and scant cytoplasm. Immunohistochemistry was positive for CD19 and CD20, but negative for CD3 and CD15.

PLAN:
She was counseled that these infusion reactions are common and usually diminish with subsequent doses.

## Encounter 4: source 2024-02-15, practice 2026-07-08, ed, Emergency Medicine

Chief complaint: Presents to the emergency department with acute onset of dyspnea and hypoxemia

ENCOUNTER NOTE
Date: 2024-02-15
Provider: Dr. Aisha Washington, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Presents to the emergency department with acute onset of dyspnea and hypoxemia

HISTORY OF PRESENT ILLNESS:
The patient with DLBCL on chemotherapy presents with dyspnea and hypoxemia 1 hour after packed red blood cell transfusion. She reports no chest pain or leg swelling.

PAST MEDICAL HISTORY:
Active Problem List:
- Porphyria cutanea tarda (diagnosed 2020-01-15)
- Cryoglobulinemia (diagnosed 2020-04-15)
- Ataxia with vitamin E deficiency (diagnosed 2022-01-15)
- Diffuse large B-cell lymphoma (diagnosed 2024-01-15)
- Chronic hepatitis C infection
- Mixed cryoglobulinemia (Type II/III)
- Malabsorption syndrome with chronic steatorrhea
- Vitamin E deficiency secondary to fat malabsorption
- Chronic iron deficiency anemia secondary to menorrhagia
- Diffuse large B-cell lymphoma (currently in first-line chemotherapy)

PAST SURGICAL HISTORY:
- Percutaneous liver biopsy (prior staging for hepatitis C)
- Bone marrow biopsy and aspiration (diagnostic for lymphoma)

MEDICATIONS:
Home Medications:
- Vitamin E (alpha-tocopherol) 400 IU PO twice daily
- Pancrelipase 36,000 units lipase PO with meals and 18,000 units with snacks
- Ferrous sulfate 325 mg PO daily
- Ondansetron 8 mg PO every 8 hours PRN nausea

ALLERGIES:
Rituximab (infusion-related reaction: fever, chills, hypotension)

FAMILY HISTORY:
- Mother: Hypothyroidism
- Father: Hypertension, type 2 diabetes
- No family history of autoimmune disease or hematologic malignancies

SOCIAL HISTORY:
Smoking: Never. Alcohol: Former heavy use (6-10 drinks/week), quit 3 months ago upon lymphoma diagnosis. Occupation: Landscaping supervisor.

VITAL SIGNS:
Her temperature is 37.8°C (100°F), blood pressure is 135/85 mmHg, heart rate is 110 beats/min, respiratory rate is 28 breaths/min, and oxygen saturation is 88% on room air.

PHYSICAL EXAMINATION:
Physical examination reveals bilateral crackles on lung auscultation, but no jugular venous distension or peripheral edema.

LABORATORY DATA:
Arterial blood gas on 2 liters of oxygen via nasal cannula shows a PaO2 of 60 mmHg. Brain natriuretic peptide (BNP) level is within normal limits.

IMAGING:
Chest X-ray shows bilateral pulmonary infiltrates.
