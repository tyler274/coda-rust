# synthetic_hospital patient 1789

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 22 at first encounter, sex F, Hispanic, Private insurance through parental plan.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-06-29 (Tue) | outpatient | Dermatology |
| 1 | 2020-04-15 | 2021-09-23 (Thu) | outpatient | Allergy & Immunology |
| 2 | 2020-10-15 | 2022-03-29 (Tue) | outpatient | Hematology |
| 3 | 2021-09-15 | 2023-02-24 (Fri) | outpatient | Dermatology |
| 4 | 2022-09-15 | 2024-02-24 (Sat) | inpatient | Nephrology |
| 5 | 2023-09-15 | 2025-03-10 (Mon) | ed | Emergency Medicine |
| 6 | 2024-09-15 | 2026-03-05 (Thu) | inpatient | Infectious Disease |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private insurance through parental plan",
 "occupation": "Veterinary technician (part-time), lives on family ranch",
 "alcohol_use": "No alcohol use",
 "family_history": [
  "Father: Hereditary Angioedema (confirmed C1 inhibitor deficiency)",
  "Mother: Systemic Lupus Erythematosus, hypothyroidism",
  "Maternal aunt: Rheumatoid arthritis",
  "Paternal grandmother: 'Swelling episodes' (suspected undiagnosed HAE)"
 ],
 "race_ethnicity": "Hispanic",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "200 mg PO BID",
   "name": "Hydroxychloroquine"
  },
  {
   "dose": "1000 mg PO BID",
   "name": "Mycophenolate mofetil"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Prednisone"
  },
  {
   "dose": "200 mg PO TID (for HAE long-term prophylaxis)",
   "name": "Danazol"
  },
  {
   "dose": "1 mg PO daily",
   "name": "Folic acid"
  },
  {
   "dose": "2000 IU PO daily",
   "name": "Cholecalciferol (Vitamin D3)"
  }
 ],
 "surgical_history": [
  "Tracheal intubation (prior airway angioedema episode)",
  "Renal biopsy (diagnostic, prior admission)"
 ],
 "chronic_conditions": [
  "Systemic Lupus Erythematosus",
  "Hereditary Angioedema Type I (C1 Esterase Inhibitor Deficiency)",
  "Lupus Nephritis Class IV (diffuse proliferative, in remission)",
  "Secondary Cold Autoimmune Hemolytic Anemia",
  "History of spontaneous abortion (10 weeks gestation)"
 ],
 "age_at_first_encounter": 22
}
```

Primary diagnoses: ["Hereditary Angioedema (C1 Inhibitor Deficiency)", "Bubonic plague (Yersinia pestis infection)", "Systemic lupus erythematosus", "Lupus nephritis flare", "Cold autoimmune hemolytic anemia"]
Comorbidities: ["Personal History of Cholelithiasis", "Leukocytosis", "Thrombocytopenia", "Systemic lupus erythematosus"]

## Encounter 0: source 2020-01-15, practice 2021-06-29, outpatient, Dermatology

Chief complaint: Joint pain, fatigue, and facial rash with photosensitivity and oral ulcers.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Elena Martinez, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Joint pain, fatigue, and facial rash with photosensitivity and oral ulcers.

HISTORY OF PRESENT ILLNESS:
She reports episodes of photosensitivity and oral ulcers.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic Lupus Erythematosus
- Hereditary Angioedema Type I (C1 Esterase Inhibitor Deficiency)
- Lupus Nephritis Class IV (diffuse proliferative, in remission)
- Secondary Cold Autoimmune Hemolytic Anemia
- History of spontaneous abortion (10 weeks gestation)

Her medical history is significant for one spontaneous abortion at 10 weeks.

PAST SURGICAL HISTORY:
- Tracheal intubation (prior airway angioedema episode)
- Renal biopsy (diagnostic, prior admission)

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg PO BID
- Mycophenolate mofetil 1000 mg PO BID
- Prednisone 5 mg PO daily
- Danazol 200 mg PO TID (for HAE long-term prophylaxis)
- Folic acid 1 mg PO daily
- Cholecalciferol (Vitamin D3) 2000 IU PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hereditary Angioedema (confirmed C1 inhibitor deficiency)
- Mother: Systemic Lupus Erythematosus, hypothyroidism
- Maternal aunt: Rheumatoid arthritis
- Paternal grandmother: 'Swelling episodes' (suspected undiagnosed HAE)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Veterinary technician (part-time), lives on family ranch.

PHYSICAL EXAMINATION:
Physical examination reveals a malar rash and synovitis in both hands.

LABORATORY DATA:
Laboratory results show positive ANA, anti-dsDNA, and anti-Smith antibodies. Her aPTT is elevated, and VDRL is positive, but she denies any sexual history. CBC shows mild leukopenia and anemia. ESR is elevated, and complement C3 is decreased.

IMAGING:
Chest X-ray shows no acute pathology.

## Encounter 1: source 2020-04-15, practice 2021-09-23, outpatient, Allergy & Immunology

Chief complaint: Four-day history of generalized arthritis and facial rash.

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. James Chen, MD
Department: Allergy & Immunology
Visit Type: outpatient

CHIEF COMPLAINT:
Four-day history of generalized arthritis and facial rash.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus (diagnosed 2020-01-15)
- Hereditary Angioedema Type I (C1 Esterase Inhibitor Deficiency)
- Lupus Nephritis Class IV (diffuse proliferative, in remission)
- Secondary Cold Autoimmune Hemolytic Anemia
- History of spontaneous abortion (10 weeks gestation)

PAST SURGICAL HISTORY:
- Tracheal intubation (prior airway angioedema episode)
- Renal biopsy (diagnostic, prior admission)

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg PO BID
- Mycophenolate mofetil 1000 mg PO BID
- Prednisone 5 mg PO daily
- Danazol 200 mg PO TID (for HAE long-term prophylaxis)
- Folic acid 1 mg PO daily
- Cholecalciferol (Vitamin D3) 2000 IU PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hereditary Angioedema (confirmed C1 inhibitor deficiency)
- Mother: Systemic Lupus Erythematosus, hypothyroidism
- Maternal aunt: Rheumatoid arthritis
- Paternal grandmother: 'Swelling episodes' (suspected undiagnosed HAE)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Veterinary technician (part-time), lives on family ranch.

PHYSICAL EXAMINATION:
Physical examination shows a rash that appears relatively symmetric across the cheeks and bridge of the nose.

LABORATORY DATA:
Laboratory studies show a platelet count of 100,000/μL.

ASSESSMENT AND PLAN:
A genetic defect in

## Encounter 2: source 2020-10-15, practice 2022-03-29, outpatient, Hematology

Chief complaint: Progressive fatigue, exertional dyspnea, dizziness, and dark urine over two weeks.

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Sarah Okafor, MD
Department: Hematology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue, exertional dyspnea, dizziness, and dark urine over two weeks.

HISTORY OF PRESENT ILLNESS:
The patient returns to Hematology following January Dermatology and April Allergy evaluations for systemic lupus erythematosus and hereditary angioedema. She reports recent arthralgia and increased sensitivity to sunlight. She also notes intermittent low-grade fevers and a history of photosensitive rash, denying recent infections or NSAID use.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus (diagnosed 2020-01-15)
- Hereditary Angioedema (C1 Inhibitor Deficiency) (diagnosed 2020-04-15)
- Hereditary Angioedema Type I (C1 Esterase Inhibitor Deficiency)
- Lupus Nephritis Class IV (diffuse proliferative, in remission)
- Secondary Cold Autoimmune Hemolytic Anemia
- History of spontaneous abortion (10 weeks gestation)

PAST SURGICAL HISTORY:
- Tracheal intubation (prior airway angioedema episode)
- Renal biopsy (diagnostic, prior admission)

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg PO BID
- Mycophenolate mofetil 1000 mg PO BID
- Prednisone 5 mg PO daily
- Danazol 200 mg PO TID (for HAE long-term prophylaxis)
- Folic acid 1 mg PO daily
- Cholecalciferol (Vitamin D3) 2000 IU PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hereditary Angioedema (confirmed C1 inhibitor deficiency)
- Mother: Systemic Lupus Erythematosus, hypothyroidism
- Maternal aunt: Rheumatoid arthritis
- Paternal grandmother: 'Swelling episodes' (suspected undiagnosed HAE)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Veterinary technician (part-time), lives on family ranch.

PHYSICAL EXAMINATION:
She is pale and mildly tachycardic with no jaundice or lymphadenopathy

LABORATORY DATA:
Hemoglobin 7.9 g/dL, reticulocyte count of 8%, elevated lactate dehydrogenase, low haptoglobin, and indirect hyperbilirubinemia. Peripheral smear shows spherocytes. A direct Coombs test is positive.

## Encounter 3: source 2021-09-15, practice 2023-02-24, outpatient, Dermatology

Chief complaint: Joint pain, fatigue, and facial rash with photosensitivity and oral ulcers.

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. Elena Martinez, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Joint pain, fatigue, and facial rash with photosensitivity and oral ulcers.

HISTORY OF PRESENT ILLNESS:
The SLE patient returns. She reports episodes of photosensitivity and oral ulcers.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus (diagnosed 2020-01-15)
- Hereditary Angioedema (C1 Inhibitor Deficiency) (diagnosed 2020-04-15)
- Cold autoimmune hemolytic anemia (diagnosed 2020-10-15)
- Hereditary Angioedema Type I (C1 Esterase Inhibitor Deficiency)
- Lupus Nephritis Class IV (diffuse proliferative, in remission)
- Secondary Cold Autoimmune Hemolytic Anemia
- History of spontaneous abortion (10 weeks gestation)

Her medical history is significant for one spontaneous abortion at 10 weeks.

PAST SURGICAL HISTORY:
- Tracheal intubation (prior airway angioedema episode)
- Renal biopsy (diagnostic, prior admission)

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg PO BID
- Mycophenolate mofetil 1000 mg PO BID
- Prednisone 5 mg PO daily
- Danazol 200 mg PO TID (for HAE long-term prophylaxis)
- Folic acid 1 mg PO daily
- Cholecalciferol (Vitamin D3) 2000 IU PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hereditary Angioedema (confirmed C1 inhibitor deficiency)
- Mother: Systemic Lupus Erythematosus, hypothyroidism
- Maternal aunt: Rheumatoid arthritis
- Paternal grandmother: 'Swelling episodes' (suspected undiagnosed HAE)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Veterinary technician (part-time), lives on family ranch.

PHYSICAL EXAMINATION:
Physical examination reveals a malar rash and synovitis in both hands.

LABORATORY DATA:
Laboratory results show positive ANA, anti-dsDNA, and anti-Smith antibodies. Her aPTT is elevated, and VDRL is positive, but she denies any sexual history. CBC shows mild leukopenia and anemia. ESR is elevated, and complement C3 is decreased.

IMAGING:
Chest X-ray shows no acute pathology.

## Encounter 4: source 2022-09-15, practice 2024-02-24, inpatient, Nephrology

Chief complaint: Increasing fatigue, joint pain, reddish urine, and facial puffiness over two days.

ENCOUNTER NOTE
Date: 2022-09-15
Provider: Dr. Michael Thompson, MD
Department: Nephrology
Visit Type: inpatient

CHIEF COMPLAINT:
Increasing fatigue, joint pain, reddish urine, and facial puffiness over two days.

HISTORY OF PRESENT ILLNESS:
The patient with SLE and class IV lupus nephritis previously in remission presents with morning stiffness in her fingers and knees lasting about an hour and facial puffiness.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus (diagnosed 2020-01-15)
- Hereditary Angioedema (C1 Inhibitor Deficiency) (diagnosed 2020-04-15)
- Cold autoimmune hemolytic anemia (diagnosed 2020-10-15)
- Hereditary Angioedema Type I (C1 Esterase Inhibitor Deficiency)
- Lupus Nephritis Class IV (diffuse proliferative, in remission)
- Secondary Cold Autoimmune Hemolytic Anemia
- History of spontaneous abortion (10 weeks gestation)

Past medical history is significant for photosensitivity and a prior episode of pleuritic chest pain.

PAST SURGICAL HISTORY:
- Tracheal intubation (prior airway angioedema episode)
- Renal biopsy (diagnostic, prior admission)

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg PO BID
- Mycophenolate mofetil 1000 mg PO BID
- Prednisone 5 mg PO daily
- Danazol 200 mg PO TID (for HAE long-term prophylaxis)
- Folic acid 1 mg PO daily
- Cholecalciferol (Vitamin D3) 2000 IU PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hereditary Angioedema (confirmed C1 inhibitor deficiency)
- Mother: Systemic Lupus Erythematosus, hypothyroidism
- Maternal aunt: Rheumatoid arthritis
- Paternal grandmother: 'Swelling episodes' (suspected undiagnosed HAE)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Veterinary technician (part-time), lives on family ranch.

VITAL SIGNS:
Vital signs reveal a blood pressure of 148/94 mm Hg.

PHYSICAL EXAMINATION:
Physical examination shows mild periorbital edema and tenderness of the small joints of both hands.

LABORATORY DATA:
Laboratory studies reveal a serum creatinine of 2.0 mg/dL, elevated anti-dsDNA antibody titers, low complement levels, and urinalysis shows dysmorphic red blood cells and red cell casts.

PLAN:
A renal biopsy is scheduled.

## Encounter 5: source 2023-09-15, practice 2025-03-10, ed, Emergency Medicine

Chief complaint: Two-day progressive swelling of face and extremities without urticaria or pruritus.

ENCOUNTER NOTE
Date: 2023-09-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-day progressive swelling of face and extremities without urticaria or pruritus.

HISTORY OF PRESENT ILLNESS:
The patient with established Hereditary Angioedema Type I returns with recurrent swelling. She reports that these episodes have occurred sporadically over the past 5 years, often resolving spontaneously without intervention. She denies any urticaria or pruritus.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus (diagnosed 2020-01-15)
- Hereditary Angioedema (C1 Inhibitor Deficiency) (diagnosed 2020-04-15)
- Cold autoimmune hemolytic anemia (diagnosed 2020-10-15)
- Lupus nephritis flare (diagnosed 2022-09-15)
- Hereditary Angioedema Type I (C1 Esterase Inhibitor Deficiency)
- Lupus Nephritis Class IV (diffuse proliferative, in remission)
- Secondary Cold Autoimmune Hemolytic Anemia
- History of spontaneous abortion (10 weeks gestation)

Her medical history is significant for a similar episode one year ago that required intubation due to laryngeal edema.

PAST SURGICAL HISTORY:
- Tracheal intubation (prior airway angioedema episode)
- Renal biopsy (diagnostic, prior admission)

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg PO BID
- Mycophenolate mofetil 1000 mg PO BID
- Prednisone 5 mg PO daily
- Danazol 200 mg PO TID (for HAE long-term prophylaxis)
- Folic acid 1 mg PO daily
- Cholecalciferol (Vitamin D3) 2000 IU PO daily

ALLERGIES:
She has no known allergies

FAMILY HISTORY:
Her mother has a history of similar symptoms.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: No alcohol use. Occupation: Veterinary technician (part-time), lives on family ranch.

VITAL SIGNS:
On examination, her vital signs are stable,

PHYSICAL EXAMINATION:
She has marked, non-pitting edema of her lips and eyelids, without erythema or warmth. Abdominal examination reveals mild diffuse tenderness without rebound or guarding.

LABORATORY DATA:
Laboratory tests reveal the following:

- Complete blood count: Normal
- Basic metabolic panel: Normal
- C4 complement level: Low
- C1 inhibitor level: Pending
- IgE level: Normal

## Encounter 6: source 2024-09-15, practice 2026-03-05, inpatient, Infectious Disease

Chief complaint: Fever, difficulty breathing, and leg pain following flea exposure at ranch.

ENCOUNTER NOTE
Date: 2024-09-15
Provider: Dr. Robert Kim, MD
Department: Infectious Disease
Visit Type: inpatient

CHIEF COMPLAINT:
Fever, difficulty breathing, and leg pain following flea exposure at ranch.

HISTORY OF PRESENT ILLNESS:
This 26-year-old female with lupus, hereditary angioedema, and cold autoimmune hemolytic anemia (hospitalized 2022, 2023) presents with acute symptoms. Over the past 4 days, she has developed progressive fatigue, chills, dry cough that became blood-streaked, and pain in her left groin. She also noticed a painful skin lesion on her ankle that appeared shortly after she removed a flea from her sock.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus (diagnosed 2020-01-15)
- Hereditary Angioedema (C1 Inhibitor Deficiency) (diagnosed 2020-04-15)
- Cold autoimmune hemolytic anemia (diagnosed 2020-10-15)
- Lupus nephritis flare (diagnosed 2022-09-15)
- Hereditary Angioedema Type I (C1 Esterase Inhibitor Deficiency)
- Lupus Nephritis Class IV (diffuse proliferative, in remission)
- Secondary Cold Autoimmune Hemolytic Anemia
- History of spontaneous abortion (10 weeks gestation)

PAST SURGICAL HISTORY:
- Tracheal intubation (prior airway angioedema episode)
- Renal biopsy (diagnostic, prior admission)

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg PO BID
- Mycophenolate mofetil 1000 mg PO BID
- Prednisone 5 mg PO daily
- Danazol 200 mg PO TID (for HAE long-term prophylaxis)
- Folic acid 1 mg PO daily
- Cholecalciferol (Vitamin D3) 2000 IU PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Hereditary Angioedema (confirmed C1 inhibitor deficiency)
- Mother: Systemic Lupus Erythematosus, hypothyroidism
- Maternal aunt: Rheumatoid arthritis
- Paternal grandmother: 'Swelling episodes' (suspected undiagnosed HAE)

SOCIAL HISTORY:
She lives with her parents on a ranch in northern Arizona and has frequent exposure to livestock and stray cats.

VITAL SIGNS:
On arrival, she is febrile and dyspneic. Temperature is 39.2°C (102.6°F), pulse is 110/min, respirations are 30/min, and blood pressure is 92/58 mm Hg.

PHYSICAL EXAMINATION:
On physical exam, there is a 1.5-cm necrotic ulcer on her lateral ankle and multiple erythematous macules on the lower extremities. A tender, fluctuant 4-cm mass is palpated in the left inguinal region. Pulmonary examination reveals crackles bilaterally.

LABORATORY DATA:
Laboratory testing shows leukocytosis and thrombocytopenia.

OTHER STUDIES:
Aspiration of the groin mass yields purulent material containing many neutrophils and a few gram-negative rods that grow well at 4 degrees Celsius.
