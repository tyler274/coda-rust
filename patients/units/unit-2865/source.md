# synthetic_hospital patient 2865

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 63 at first encounter, sex F, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-02-10 (Wed) | outpatient | Rheumatology |
| 1 | 2020-07-15 | 2021-08-20 (Fri) | outpatient | Gastroenterology |
| 2 | 2022-07-15 | 2023-08-08 (Tue) | outpatient | Internal Medicine |
| 3 | 2024-01-15 | 2025-02-10 (Mon) | outpatient | Psychiatry |
| 4 | 2025-01-15 | 2026-02-06 (Fri) | inpatient | Internal Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Sulfa antibiotics (hives)",
  "NKDA otherwise"
 ],
 "insurance": "Medicare",
 "occupation": "Retired piano instructor",
 "alcohol_use": "None",
 "family_history": [
  "Mother with rheumatoid arthritis",
  "Father with schizophrenia",
  "Sister with autoimmune thyroid disease"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 30 pack-years, quit 5 years ago",
 "home_medications": [
  {
   "dose": "5 mg PO BID",
   "name": "Haloperidol"
  },
  {
   "dose": "20 mg PO every week",
   "name": "Methotrexate"
  },
  {
   "dose": "1 mg PO daily",
   "name": "Folic acid"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Prednisone"
  },
  {
   "dose": "40 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "400 mg PO PRN joint pain",
   "name": "Ibuprofen"
  },
  {
   "dose": "2 mg PO PRN diarrhea",
   "name": "Loperamide"
  }
 ],
 "surgical_history": [
  "Laparoscopic appendectomy (age 12)"
 ],
 "chronic_conditions": [
  "Rheumatoid arthritis",
  "Schizophrenia",
  "Microscopic colitis",
  "Dyslipidemia",
  "Former tobacco use disorder"
 ],
 "age_at_first_encounter": 63
}
```

Primary diagnoses: ["Microscopic colitis", "Severe Clostridioides difficile infection without toxic megacolon", "Rheumatoid arthritis", "Statin-associated muscle symptoms", "Oculogyric crisis due to antipsychotic medication"]
Comorbidities: ["Rheumatoid arthritis", "History of recent pneumonia"]

## Encounter 0: source 2020-01-15, practice 2021-02-10, outpatient, Rheumatology

Chief complaint: Reports 4 months of painful, stiff hands worst in the morning, preventing her from playing piano

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Reports 4 months of painful, stiff hands worst in the morning, preventing her from playing piano

HISTORY OF PRESENT ILLNESS:
That are worst in the morning and improve after an hour of movement. She can no longer open jars and has stopped playing the piano because her fingers "don't bend right."

PAST MEDICAL HISTORY:
Active Problem List:
- Rheumatoid arthritis
- Schizophrenia
- Microscopic colitis
- Dyslipidemia
- Former tobacco use disorder

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy (age 12)

MEDICATIONS:
Home Medications:
- Haloperidol 5 mg PO BID
- Methotrexate 20 mg PO every week
- Folic acid 1 mg PO daily
- Prednisone 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Ibuprofen 400 mg PO PRN joint pain
- Loperamide 2 mg PO PRN diarrhea

ALLERGIES:
Sulfa antibiotics (hives), NKDA otherwise

FAMILY HISTORY:
- Mother with rheumatoid arthritis
- Father with schizophrenia
- Sister with autoimmune thyroid disease

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 5 years ago. Alcohol: None. Occupation: Retired piano instructor.

PHYSICAL EXAMINATION:
Exam shows fusiform swelling and tenderness of the MCP and PIP joints bilaterally with ulnar deviation; the DIPs are spared. Small, firm, nontender nodules are present over the extensor surfaces of both elbows. There is no rash or oral ulceration.

ASSESSMENT AND PLAN:
The clinician explains that her joint inflammation is driven by autoantibodies that form immune complexes in synovial tissue, activating complement and recruiting neutrophils.

## Encounter 1: source 2020-07-15, practice 2021-08-20, outpatient, Gastroenterology

Chief complaint: Chronic watery diarrhea for 6 months, unpredictable episodes including nocturnal symptoms

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Okonkwo, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Chronic watery diarrhea for 6 months, unpredictable episodes including nocturnal symptoms

HISTORY OF PRESENT ILLNESS:
The patient is a 63-year-old female with rheumatoid arthritis who presents to the gastroenterology clinic six months following her initial rheumatology evaluation. She presents with a 6-month history of chronic, watery diarrhea, reporting that episodes occur unpredictably, sometimes during periods of fasting and often at night, with up to five bowel movements per day. She denies any blood or mucus in her stool.

PAST MEDICAL HISTORY:
Active Problem List:
- Rheumatoid arthritis (diagnosed 2020-01-15)
- Schizophrenia
- Microscopic colitis
- Dyslipidemia
- Former tobacco use disorder

Her past medical history is significant for rheumatoid arthritis, for which she takes ibuprofen as needed.

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy (age 12)

MEDICATIONS:
She takes ibuprofen as needed.

Home Medications:
- Haloperidol 5 mg PO BID
- Methotrexate 20 mg PO every week
- Folic acid 1 mg PO daily
- Prednisone 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Loperamide 2 mg PO PRN diarrhea

ALLERGIES:
Sulfa antibiotics (hives), NKDA otherwise

FAMILY HISTORY:
- Mother with rheumatoid arthritis
- Father with schizophrenia
- Sister with autoimmune thyroid disease

SOCIAL HISTORY:
She has a 30-pack-year smoking history but quit 5 years ago.

PHYSICAL EXAMINATION:
On physical examination, her abdomen is soft, non-distended, and non-tender. There is no peripheral edema.

LABORATORY DATA:
Laboratory values are as follows:

- Hemoglobin: 13.2 g/dL
- White blood cell count: 7,800 /μL
- Platelets: 250,000 /μL
- Serum electrolytes: Within normal limits
- C-reactive protein: 2.1 mg/L (normal <5 mg/L)
- Fecal calprotectin: Normal

OTHER STUDIES:
A colonoscopy is performed, which shows a grossly normal-appearing colonic mucosa. Biopsy results are pending.

## Encounter 2: source 2022-07-15, practice 2023-08-08, outpatient, Internal Medicine

Chief complaint: Difficulty climbing stairs and rising from seated position over the past 3 months

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. David Park, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Difficulty climbing stairs and rising from seated position over the past 3 months

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of her rheumatoid arthritis and microscopic colitis with a 3-month history of difficulty climbing stairs and rising from a seated position. She denies any associated joint pain, muscle tenderness, or morning stiffness.

PAST MEDICAL HISTORY:
Active Problem List:
- Rheumatoid arthritis (diagnosed 2020-01-15)
- Microscopic colitis (diagnosed 2020-07-15)
- Schizophrenia
- Dyslipidemia
- Former tobacco use disorder

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy (age 12)

MEDICATIONS:
She takes methotrexate, folic acid, and prednisone 20 mg daily, which she has been taking for the past year.

Home Medications:
- Haloperidol 5 mg PO BID
- Atorvastatin 40 mg PO daily
- Ibuprofen 400 mg PO PRN joint pain
- Loperamide 2 mg PO PRN diarrhea

ALLERGIES:
Sulfa antibiotics (hives), NKDA otherwise

FAMILY HISTORY:
- Mother with rheumatoid arthritis
- Father with schizophrenia
- Sister with autoimmune thyroid disease

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 5 years ago. Alcohol: None. Occupation: Retired piano instructor.

PHYSICAL EXAMINATION:
On physical examination, she has bilateral proximal muscle weakness in the lower extremities. There is no muscle atrophy, skin rash, or joint swelling.

LABORATORY DATA:
Creatine kinase level: 80 U/L (normal range: 38-174 U/L)
- Erythrocyte sedimentation rate: 15 mm/hr (normal range: 0-20 mm/hr)
- C-reactive protein: 2.0 mg/L (normal range: 0.0-10.0 mg/L)
- Complete blood count: within normal limits

## Encounter 3: source 2024-01-15, practice 2025-02-10, outpatient, Psychiatry

Chief complaint: New involuntary movements including lip smacking and tongue chewing noted by daughter

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Fatima Al-Rashid, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
New involuntary movements including lip smacking and tongue chewing noted by daughter

HISTORY OF PRESENT ILLNESS:
This 67-year-old female with rheumatoid arthritis, schizophrenia, and microscopic colitis presents with new involuntary movements. She has a history of statin-associated muscle symptoms. Over the past three months, she has developed involuntary, repetitive movements of her mouth and tongue, which include lip smacking and chewing motions. She also exhibits occasional jerking movements of her hands. Her daughter reports that these symptoms seem to worsen when the patient is at rest and improve with voluntary movements. The patient's schizophrenia symptoms are well-controlled, and there have been no recent changes to her medication regimen.

PAST MEDICAL HISTORY:
Active Problem List:
- Rheumatoid arthritis (diagnosed 2020-01-15)
- Microscopic colitis (diagnosed 2020-07-15)
- Statin-associated muscle symptoms (diagnosed 2022-07-15)
- Schizophrenia
- Dyslipidemia
- Former tobacco use disorder

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy (age 12)

MEDICATIONS:
The patient has been stable on haloperidol for many years.

Home Medications:
- Methotrexate 20 mg PO every week
- Folic acid 1 mg PO daily
- Prednisone 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Ibuprofen 400 mg PO PRN joint pain
- Loperamide 2 mg PO PRN diarrhea

ALLERGIES:
Sulfa antibiotics (hives), NKDA otherwise

FAMILY HISTORY:
- Mother with rheumatoid arthritis
- Father with schizophrenia
- Sister with autoimmune thyroid disease

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 5 years ago. Alcohol: None. Occupation: Retired piano instructor.

PHYSICAL EXAMINATION:
On physical examination, the patient is noted to have rhythmic movements of her trunk and pelvis that she is unable to suppress. Her mental status examination is unchanged from her baseline.

LABORATORY DATA:
Routine laboratory studies, including complete blood count, comprehensive metabolic panel, and thyroid function tests, are within normal limits.

## Encounter 4: source 2025-01-15, practice 2026-02-06, inpatient, Internal Medicine

Chief complaint: 4 days of watery diarrhea, abdominal cramps, and lower abdominal tenderness following recent antibiotic exposure

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Jennifer Walsh, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
4 days of watery diarrhea, abdominal cramps, and lower abdominal tenderness following recent antibiotic exposure

HISTORY OF PRESENT ILLNESS:
The patient is a 68-year-old female with microscopic colitis diagnosed in 2020 who presents with acute diarrheal illness. She was discharged from the hospital 2 weeks ago after being treated for community-acquired pneumonia with a 10-day course of amoxicillin-clavulanate. She has no recent travel, and has not consumed any questionable food. She reports 8–10 loose stools per day but no blood or mucus.

PAST MEDICAL HISTORY:
Active Problem List:
- Rheumatoid arthritis (diagnosed 2020-01-15)
- Microscopic colitis (diagnosed 2020-07-15)
- Statin-associated muscle symptoms (diagnosed 2022-07-15)
- Oculogyric crisis due to antipsychotic medication (diagnosed 2024-01-15)
- Schizophrenia
- Dyslipidemia
- Former tobacco use disorder

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy (age 12)

MEDICATIONS:
Home Medications:
- Haloperidol 5 mg PO BID
- Methotrexate 20 mg PO every week
- Folic acid 1 mg PO daily
- Prednisone 20 mg PO daily
- Atorvastatin 40 mg PO daily
- Ibuprofen 400 mg PO PRN joint pain
- Loperamide 2 mg PO PRN diarrhea

ALLERGIES:
Sulfa antibiotics (hives), NKDA otherwise

FAMILY HISTORY:
- Mother with rheumatoid arthritis
- Father with schizophrenia
- Sister with autoimmune thyroid disease

SOCIAL HISTORY:
Smoking: Former smoker, 30 pack-years, quit 5 years ago. Alcohol: None. Occupation: Retired piano instructor.

REVIEW OF SYSTEMS:
She denies fever, chills, or vomiting.

PHYSICAL EXAMINATION:
Physical examination reveals a mildly distended abdomen with diffuse tenderness to palpation but no rebound or guarding. Bowel sounds are hyperactive.

LABORATORY DATA:
Her white blood cell count is mildly elevated. A stool sample is sent for laboratory analysis and is positive for A/B toxin by enzyme immunoassay.

PLAN:
She is treated with oral vancomycin and reports complete symptom resolution after 10 days.
