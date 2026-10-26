# synthetic_hospital patient 2775

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 42 at first encounter, sex F, White, non-Hispanic, Private PPO (Blue Cross Blue Shield).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-03-24 (Wed) | outpatient | Hematology & Oncology |
| 1 | 2020-01-15 | 2021-04-01 (Thu) | follow_up | Hematology & Oncology |
| 2 | 2020-07-15 | 2021-10-05 (Tue) | outpatient | Hematology & Oncology |
| 3 | 2021-01-15 | 2022-03-27 (Sun) | ed | Emergency Medicine |
| 4 | 2021-07-15 | 2022-10-03 (Mon) | inpatient | Internal Medicine |
| 5 | 2022-07-15 | 2023-09-21 (Thu) | outpatient | Neurology |
| 6 | 2023-01-15 | 2024-03-28 (Thu) | ed | Emergency Medicine |
| 7 | 2025-01-15 | 2026-03-31 (Tue) | outpatient | Neurology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private PPO (Blue Cross Blue Shield)",
 "occupation": "Elementary school teacher",
 "alcohol_use": "Social (1-2 drinks per month)",
 "family_history": [
  "No family history of bleeding disorders",
  "Mother: Hypothyroidism",
  "Father: Hypertension"
 ],
 "race_ethnicity": "White, non-Hispanic",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "10 mg daily",
   "name": "Prednisone"
  },
  {
   "dose": "600 mg three times daily",
   "name": "Calcium carbonate"
  },
  {
   "dose": "2000 IU daily",
   "name": "Vitamin D3"
  }
 ],
 "surgical_history": [
  "Tonsillectomy and adenoidectomy, age 12",
  "Wisdom teeth extraction, age 20"
 ],
 "chronic_conditions": [
  "Chronic immune thrombocytopenic purpura (ITP)"
 ],
 "age_at_first_encounter": 42
}
```

Primary diagnoses: ["Idiopathic thrombocytopenic purpura", "Bell's palsy", "Vestibular neuritis"]
Comorbidities: ["Upper respiratory tract infection"]

## Encounter 0: source 2020-01-15, practice 2021-03-24, outpatient, Hematology & Oncology

Chief complaint: Easy bruising and frequent nosebleeds for the past 18 months.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Easy bruising and frequent nosebleeds for the past 18 months.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic immune thrombocytopenic purpura (ITP)

She has a history of chronic immune thrombocytopenia and has been treated with multiple courses of glucocorticoids and intravenous immunoglobulin with temporary improvement in her platelet counts.

PAST SURGICAL HISTORY:
- Tonsillectomy and adenoidectomy, age 12
- Wisdom teeth extraction, age 20

MEDICATIONS:
Her current medications include prednisone 10 mg daily.

Home Medications:
- Calcium carbonate 600 mg three times daily
- Vitamin D3 2000 IU daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- No family history of bleeding disorders
- Mother: Hypothyroidism
- Father: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
On examination, she has scattered petechiae on her lower extremities and several ecchymoses of varying ages on her arms and legs. There is no hepatosplenomegaly.

LABORATORY DATA:
Laboratory tests reveal the following:

- Hemoglobin: 13.2 g/dL
- White blood cell count: 7,200/mm3
- Platelets: 28,000/mm3
- Prothrombin time (PT): 12 seconds (control 11-13.5 seconds)
- Activated partial thromboplastin time (aPTT): 30 seconds (control 25-35 seconds)

## Encounter 1: source 2020-01-15, practice 2021-04-01, follow_up, Hematology & Oncology

Chief complaint: Worsening bruising, new episodes of epistaxis, and increasing gum bleeding 10 days after initial ITP diagnosis.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Hematology & Oncology
Visit Type: follow_up

CHIEF COMPLAINT:
Worsening bruising, new episodes of epistaxis, and increasing gum bleeding 10 days after initial ITP diagnosis.

HISTORY OF PRESENT ILLNESS:
The patient is a 42-year-old female with immune thrombocytopenic purpura (ITP) who returns to the clinic for follow-up evaluation approximately 10 days following her initial diagnosis. She initially presented with spontaneous bruising and bleeding gums, and laboratory evaluation at that time demonstrated isolated thrombocytopenia with a platelet count of 11,000/mm³. Peripheral smear was normal aside from large platelets, and prothrombin time (PT) and activated partial thromboplastin time (aPTT) were within normal limits. She was started on oral prednisone at the prior visit. She currently denies hematuria, melena, or changes in neurologic status, and reports no history of trauma. The patient has been adherent to prednisone and reports no side effects.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic thrombocytopenic purpura (diagnosed 2020-01-15)
- Chronic immune thrombocytopenic purpura (ITP)

PAST SURGICAL HISTORY:
- Tonsillectomy and adenoidectomy, age 12
- Wisdom teeth extraction, age 20

MEDICATIONS:
Home Medications:
- Prednisone 10 mg daily
- Calcium carbonate 600 mg three times daily
- Vitamin D3 2000 IU daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- No family history of bleeding disorders
- Mother: Hypothyroidism
- Father: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

VITAL SIGNS:
Vital signs are stable.

PHYSICAL EXAMINATION:
Examination reveals multiple new petechiae on the lower extremities and ecchymoses on the forearms and thighs. There is dried blood at the gumline but no active bleeding. No hepatosplenomegaly or lymphadenopathy is noted.

LABORATORY DATA:
Repeat labs show a hemoglobin level of 12.3 g/dL, leukocyte count of 6,900/mm³, and platelet count of 9,000/mm³. Coagulation studies remain normal.

## Encounter 2: source 2020-07-15, practice 2021-10-05, outpatient, Hematology & Oncology

Chief complaint: Increased bruising over the past two weeks with occasional nosebleeds.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Sarah Chen, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Increased bruising over the past two weeks with occasional nosebleeds.

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of chronic ITP diagnosed January 2020. She reports noticing easy bruising on her arms and legs without significant trauma. She also has occasional nosebleeds that are difficult to stop.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic thrombocytopenic purpura (diagnosed 2020-01-15)
- Chronic immune thrombocytopenic purpura (ITP)

PAST SURGICAL HISTORY:
- Tonsillectomy and adenoidectomy, age 12
- Wisdom teeth extraction, age 20

MEDICATIONS:
Home Medications:
- Prednisone 10 mg daily
- Calcium carbonate 600 mg three times daily
- Vitamin D3 2000 IU daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- No family history of bleeding disorders
- Mother: Hypothyroidism
- Father: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
Physical examination reveals multiple petechiae and ecchymoses on the extremities. There is no hepatosplenomegaly or lymphadenopathy. Neurologic examination is normal.

LABORATORY DATA:
Hemoglobin 13.0 g/dL
Hematocrit 39%
MCV 86 fL
Platelet count 18,000/µL
White blood cell count 6,800/µL
Bleeding time Prolonged
PT 12 sec
PTT 30 sec

## Encounter 3: source 2021-01-15, practice 2022-03-27, ed, Emergency Medicine

Chief complaint: Easy bruising and increased bleeding from gums over the past 5 days, with prolonged nosebleed lasting 30 minutes.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. James Wilson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Easy bruising and increased bleeding from gums over the past 5 days, with prolonged nosebleed lasting 30 minutes.

HISTORY OF PRESENT ILLNESS:
The patient is a 43-year-old female with a known history of chronic immune thrombocytopenic purpura diagnosed in January 2020 who returns to the emergency department with recurrent bleeding symptoms. She presents approximately six months following her last outpatient hematology evaluation with similar complaints. She says she first noticed purplish discoloration on her thighs and upper arms that appeared spontaneously. Over the past two days, she has experienced bleeding after brushing her teeth and a prolonged nosebleed lasting nearly 30 minutes. She denies hematuria, melena, or heavy menstrual bleeding. She has not had recent dental work or trauma. She recalls having a mild upper respiratory infection about two weeks ago, which resolved without treatment.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic thrombocytopenic purpura (diagnosed 2020-01-15)
- Chronic immune thrombocytopenic purpura (ITP)

She has no personal or family history of bleeding disorders.

PAST SURGICAL HISTORY:
- Tonsillectomy and adenoidectomy, age 12
- Wisdom teeth extraction, age 20

MEDICATIONS:
She is not taking any anticoagulants or NSAIDs.

Home Medications:
- Prednisone 10 mg daily
- Calcium carbonate 600 mg three times daily
- Vitamin D3 2000 IU daily

ALLERGIES:
NKDA

FAMILY HISTORY:
She has no personal or family history of bleeding disorders

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

VITAL SIGNS:
On exam, her vital signs are normal.

PHYSICAL EXAMINATION:
She appears well and in no distress. There are several ecchymoses over her arms and thighs, and petechiae are noted on both lower extremities. Her gums are mildly erythematous with residual dried blood, but there is no active bleeding. There is no hepatosplenomegaly or lymphadenopathy. Neurologic and cardiopulmonary exams are normal. Pelvic exam is unremarkable.

LABORATORY DATA:
Laboratory testing reveals a hemoglobin level of 12.9 g/dL, leukocyte count of 6,700/mm³, and platelet count of 14,000/mm³.

## Encounter 4: source 2021-07-15, practice 2022-10-03, inpatient, Internal Medicine

Chief complaint: Easy bruising and increased bleeding from gums over the past 5 days with spontaneous purpuric discoloration.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Lisa Rodriguez, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Easy bruising and increased bleeding from gums over the past 5 days with spontaneous purpuric discoloration.

HISTORY OF PRESENT ILLNESS:
The patient with chronic immune thrombocytopenic purpura initially diagnosed in January 2020 returns with recurrent bleeding symptoms, having presented to the emergency department six months prior with similar complaints. She says she first noticed purplish discoloration on her thighs and upper arms that appeared spontaneously. Over the past two days, she has experienced bleeding after brushing her teeth and a prolonged nosebleed lasting nearly 30 minutes. She denies hematuria, melena, or heavy menstrual bleeding. She has not had recent dental work or trauma. She recalls having a mild upper respiratory infection about two weeks ago, which resolved without treatment.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic thrombocytopenic purpura (diagnosed 2020-01-15)
- Chronic immune thrombocytopenic purpura (ITP)

She has no personal or family history of bleeding disorders.

PAST SURGICAL HISTORY:
- Tonsillectomy and adenoidectomy, age 12
- Wisdom teeth extraction, age 20

MEDICATIONS:
She is not taking any anticoagulants or NSAIDs.

Home Medications:
- Prednisone 10 mg daily
- Calcium carbonate 600 mg three times daily
- Vitamin D3 2000 IU daily

ALLERGIES:
NKDA

FAMILY HISTORY:
Family history of bleeding disorders

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

VITAL SIGNS:
On exam, her vital signs are normal.

PHYSICAL EXAMINATION:
She appears well and in no distress. There are several ecchymoses over her arms and thighs, and petechiae are noted on both lower extremities. Her gums are mildly erythematous with residual dried blood, but there is no active bleeding. There is no hepatosplenomegaly or lymphadenopathy. Neurologic and cardiopulmonary exams are normal. Pelvic exam is unremarkable.

LABORATORY DATA:
Laboratory testing reveals a hemoglobin level of 12.9 g/dL, leukocyte count of 6,700/mm³, and platelet count of 14,000/mm³.

## Encounter 5: source 2022-07-15, practice 2023-09-21, outpatient, Neurology

Chief complaint: Sudden-onset weakness of the right side of her face following recent upper respiratory illness.

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. David Kim, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Sudden-onset weakness of the right side of her face following recent upper respiratory illness.

HISTORY OF PRESENT ILLNESS:
This 44-year-old female with chronic ITP and recurrent bleeding complications presents to Neurology. She is unable to close her right eye, raise her right eyebrow, or smile symmetrically. She reports recent recovery from an upper respiratory illness. She denies rash, ear pain, and tick exposure.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic thrombocytopenic purpura (diagnosed 2020-01-15)
- Chronic immune thrombocytopenic purpura (ITP)

PAST SURGICAL HISTORY:
- Tonsillectomy and adenoidectomy, age 12
- Wisdom teeth extraction, age 20

MEDICATIONS:
Home Medications:
- Prednisone 10 mg daily
- Calcium carbonate 600 mg three times daily
- Vitamin D3 2000 IU daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- No family history of bleeding disorders
- Mother: Hypothyroidism
- Father: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
Neurologic examination reveals complete right-sided facial paralysis. Strength and sensation in the limbs are normal.

## Encounter 6: source 2023-01-15, practice 2024-03-28, ed, Emergency Medicine

Chief complaint: 2-day history of severe vertigo, nausea, and vomiting preventing daily activities.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Amanda Foster, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
2-day history of severe vertigo, nausea, and vomiting preventing daily activities.

HISTORY OF PRESENT ILLNESS:
This 45-year-old female with chronic immune thrombocytopenic purpura maintained on prednisone and history of Bell's palsy reports that symptoms started suddenly and have persisted, preventing her from performing daily activities. She also notes a recent upper respiratory tract infection that resolved a week ago. The patient denies any hearing loss or tinnitus.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic thrombocytopenic purpura (diagnosed 2020-01-15)
- Bell's palsy (diagnosed 2022-07-15)
- Chronic immune thrombocytopenic purpura (ITP)

PAST SURGICAL HISTORY:
- Tonsillectomy and adenoidectomy, age 12
- Wisdom teeth extraction, age 20

MEDICATIONS:
Home Medications:
- Prednisone 10 mg daily
- Calcium carbonate 600 mg three times daily
- Vitamin D3 2000 IU daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- No family history of bleeding disorders
- Mother: Hypothyroidism
- Father: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

VITAL SIGNS:
Vital signs are within normal limits

PHYSICAL EXAMINATION:
On examination, she has spontaneous nystagmus that increases when she looks to the left and a positive Romberg sign. She is unable to perform the Dix-Hallpike maneuver due to severe nausea. Neurological examination is otherwise non-focal.

## Encounter 7: source 2025-01-15, practice 2026-03-31, outpatient, Neurology

Chief complaint: Sudden-onset weakness of the right side of her face with inability to close the right eye.

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. David Kim, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Sudden-onset weakness of the right side of her face with inability to close the right eye.

HISTORY OF PRESENT ILLNESS:
She has chronic ITP and presents with recurrent right facial weakness similar to her 2022 episode. She is unable to close her right eye, raise her right eyebrow, or smile symmetrically. She reports recent recovery from an upper respiratory illness. She denies rash, ear pain, and tick exposure.

PAST MEDICAL HISTORY:
Active Problem List:
- Idiopathic thrombocytopenic purpura (diagnosed 2020-01-15)
- Bell's palsy (diagnosed 2022-07-15)
- Vestibular neuritis (diagnosed 2023-01-15)
- Chronic immune thrombocytopenic purpura (ITP)

PAST SURGICAL HISTORY:
- Tonsillectomy and adenoidectomy, age 12
- Wisdom teeth extraction, age 20

MEDICATIONS:
Home Medications:
- Prednisone 10 mg daily
- Calcium carbonate 600 mg three times daily
- Vitamin D3 2000 IU daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- No family history of bleeding disorders
- Mother: Hypothyroidism
- Father: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
Neurologic examination reveals complete right-sided facial paralysis. Strength and sensation in the limbs are normal.
