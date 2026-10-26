# synthetic_hospital patient 1823

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 28 at first encounter, sex F, Cypriot (Mediterranean), Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-08-06 (Thu) | outpatient | Internal Medicine |
| 1 | 2020-04-15 | 2020-11-16 (Mon) | outpatient | Hematology & Oncology |
| 2 | 2020-09-15 | 2021-04-14 (Wed) | procedure | Obstetrics & Gynecology |
| 3 | 2021-03-15 | 2021-10-18 (Mon) | outpatient | Internal Medicine |
| 4 | 2022-03-15 | 2022-10-17 (Mon) | outpatient | Hematology & Oncology |
| 5 | 2023-03-15 | 2023-10-06 (Fri) | outpatient | Hematology & Oncology |
| 6 | 2024-03-15 | 2024-10-15 (Tue) | outpatient | Hematology & Oncology |
| 7 | 2025-03-15 | 2025-10-14 (Tue) | outpatient | Hematology & Oncology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Elementary school teacher",
 "alcohol_use": "Social (1-2 drinks per month)",
 "family_history": [
  "Father with beta-thalassemia minor",
  "Mother with beta-thalassemia minor",
  "Paternal cousin with beta-thalassemia major",
  "Maternal aunt with endometriosis"
 ],
 "race_ethnicity": "Cypriot (Mediterranean)",
 "smoking_status": "Never",
 "home_medications": [
  {
   "dose": "1 tablet daily",
   "name": "Multivitamin"
  }
 ],
 "surgical_history": [
  "Laparoscopic right ovarian cystectomy for endometrioma"
 ],
 "chronic_conditions": [
  "Beta-thalassemia minor (trait)",
  "Endometriosis",
  "History of iron deficiency anemia"
 ],
 "age_at_first_encounter": 28
}
```

Primary diagnoses: ["Thalassemia minor", "Endometriosis of ovary (Ovarian endometrioma)", "Iron deficiency anemia"]
Comorbidities: ["Menorrhagia", "Family history of hereditary bleeding disorder"]

## Encounter 0: source 2020-01-15, practice 2020-08-06, outpatient, Internal Medicine

Chief complaint: Evaluation of anemia noted during a pre-employment physical for elementary school teaching position.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Evaluation of anemia noted during a pre-employment physical for elementary school teaching position.

HISTORY OF PRESENT ILLNESS:
She feels well, denying fatigue, dyspnea, or dizziness. She reports regular menstrual cycles and no history of heavy bleeding. Her diet includes red meat, leafy greens, and iron-fortified cereals. She is not currently pregnant but plans to start a family within the next year. She is started on oral ferrous sulfate but returns six weeks later with no improvement.

PAST MEDICAL HISTORY:
Active Problem List:
- Beta-thalassemia minor (trait)
- Endometriosis
- History of iron deficiency anemia

PAST SURGICAL HISTORY:
- Laparoscopic right ovarian cystectomy for endometrioma

MEDICATIONS:
She takes a daily multivitamin and does not take any medications or supplements.

ALLERGIES:
NKDA

FAMILY HISTORY:
Her mother and maternal uncle were both told they had "small blood cells" but never required treatment.

SOCIAL HISTORY:
She emigrated from Cyprus six years ago and works as a schoolteacher.

PHYSICAL EXAMINATION:
Physical exam is normal.

LABORATORY DATA:
Laboratory studies show microcytic anemia. Iron panel reveals normal iron and ferritin levels. Hemoglobin electrophoresis is ordered, and results show elevated hemoglobin A2 with mild increase in hemoglobin F.

## Encounter 1: source 2020-04-15, practice 2020-11-16, outpatient, Hematology & Oncology

Chief complaint: Increasing fatigue over the past two months with occasional lightheadedness on standing.

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Increasing fatigue over the past two months with occasional lightheadedness on standing.

HISTORY OF PRESENT ILLNESS:
The patient returns for evaluation of progressive fatigue and lightheadedness, following her initial assessment for anemia detected during a pre-employment physical three months prior, with known Beta-thalassemia minor and Endometriosis. She reports feeling tired during her daily activities and occasionally feeling lightheaded when standing up quickly. Her menstrual cycles are regular but tend to be heavy, requiring frequent pad changes. She denies any unusual cravings or gastrointestinal symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Thalassemia minor (diagnosed 2020-01-15)
- Beta-thalassemia minor (trait)
- Endometriosis
- History of iron deficiency anemia

PAST SURGICAL HISTORY:
- Laparoscopic right ovarian cystectomy for endometrioma

MEDICATIONS:
She does not take any medications

Home Medications:
- Multivitamin 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
There is no family history of blood disorders.

SOCIAL HISTORY:
Follows a balanced diet

PHYSICAL EXAMINATION:
Physical examination shows pale conjunctivae but is otherwise normal.

LABORATORY DATA:
Laboratory results:
Hemoglobin                        9.4 g/dL
MCV                                  72 fL
Serum iron                       24 µg/dL
Ferritin                             9 ng/mL
RDW                               17%

OTHER STUDIES:
A peripheral blood smear is obtained and shown below.

## Encounter 2: source 2020-09-15, practice 2021-04-14, procedure, Obstetrics & Gynecology

Chief complaint: Pelvic pain and evaluation for ovarian cyst.

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Emily Johnson, MD
Department: Obstetrics & Gynecology
Visit Type: procedure

CHIEF COMPLAINT:
Pelvic pain and evaluation for ovarian cyst.

PAST MEDICAL HISTORY:
Active Problem List:
- Thalassemia minor (diagnosed 2020-01-15)
- Iron deficiency anemia (diagnosed 2020-04-15)
- Beta-thalassemia minor (trait)
- Endometriosis

PAST SURGICAL HISTORY:
- Laparoscopic right ovarian cystectomy for endometrioma

MEDICATIONS:
Home Medications:
- Multivitamin 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with beta-thalassemia minor
- Mother with beta-thalassemia minor
- Paternal cousin with beta-thalassemia major
- Maternal aunt with endometriosis

SOCIAL HISTORY:
Smoking: Never. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

## Encounter 3: source 2021-03-15, practice 2021-10-18, outpatient, Internal Medicine

Chief complaint: Routine health examination and follow-up of chronic anemia.

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine health examination and follow-up of chronic anemia.

HISTORY OF PRESENT ILLNESS:
Returning for anemia follow-up with thalassemia minor, iron deficiency, and endometriosis, she has a history of mild fatigue and denies any recent illness, bleeding, or dietary changes.

PAST MEDICAL HISTORY:
Active Problem List:
- Thalassemia minor (diagnosed 2020-01-15)
- Iron deficiency anemia (diagnosed 2020-04-15)
- Endometriosis of ovary (Ovarian endometrioma) (diagnosed 2020-09-15)
- Beta-thalassemia minor (trait)
- Endometriosis

PAST SURGICAL HISTORY:
- Laparoscopic right ovarian cystectomy for endometrioma

MEDICATIONS:
Home Medications:
- Multivitamin 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
Her family history is notable for a sister with a "blood disorder" that was never clearly explained to her.

SOCIAL HISTORY:
Smoking: Never. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
On physical examination, she appears well and has no pallor, jaundice, or organomegaly.

LABORATORY DATA:
Her complete blood count reveals the following:

- Hemoglobin: 11.2 g/dL (Normal: 12-16 g/dL)
- Mean corpuscular volume (MCV): 72 fL (Normal: 80-100 fL)
- Red cell distribution width (RDW): 13.5% (Normal: 11.5-14.5%)
- Reticulocyte count: 3.5% (Normal: 0.5-2.5%)

## Encounter 4: source 2022-03-15, practice 2022-10-17, outpatient, Hematology & Oncology

Chief complaint: Three-month history of worsening fatigue and shortness of breath on exertion.

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. Michael Rodriguez, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Three-month history of worsening fatigue and shortness of breath on exertion.

HISTORY OF PRESENT ILLNESS:
Returning for follow-up of beta-thalassemia minor, iron deficiency anemia, and endometriosis, she reports heavy menses but otherwise feels well.

PAST MEDICAL HISTORY:
Active Problem List:
- Thalassemia minor (diagnosed 2020-01-15)
- Iron deficiency anemia (diagnosed 2020-04-15)
- Endometriosis of ovary (Ovarian endometrioma) (diagnosed 2020-09-15)
- Beta-thalassemia minor (trait)
- Endometriosis

PAST SURGICAL HISTORY:
- Laparoscopic right ovarian cystectomy for endometrioma

MEDICATIONS:
She was started on a daily iron supplement at the time.

Home Medications:
- Multivitamin 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with beta-thalassemia minor
- Mother with beta-thalassemia minor
- Paternal cousin with beta-thalassemia major
- Maternal aunt with endometriosis

SOCIAL HISTORY:
Smoking: Never. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

LABORATORY DATA:
Hemoglobin is 9.2 g/dL, MCV is 72 fL. One month ago, her hemoglobin was 9.0 g/dL and MCV was 73 fL.

## Encounter 5: source 2023-03-15, practice 2023-10-06, outpatient, Hematology & Oncology

Chief complaint: Ongoing complaints of fatigue and mild exertional dyspnea.

ENCOUNTER NOTE
Date: 2023-03-15
Provider: Dr. David Park, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Ongoing complaints of fatigue and mild exertional dyspnea.

PAST MEDICAL HISTORY:
Active Problem List:
- Thalassemia minor (diagnosed 2020-01-15)
- Iron deficiency anemia (diagnosed 2020-04-15)
- Endometriosis of ovary (Ovarian endometrioma) (diagnosed 2020-09-15)
- Beta-thalassemia minor (trait)
- Endometriosis

She denies any significant past medical history.

PAST SURGICAL HISTORY:
- Laparoscopic right ovarian cystectomy for endometrioma

MEDICATIONS:
Home Medications:
- Multivitamin 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
She reports a similar history of anemia in several family members, but she is not aware of any specific diagnosis

SOCIAL HISTORY:
Smoking: Never. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
On physical examination, she appears well with no pallor, jaundice, or organomegaly

LABORATORY DATA:
Her complete blood count reveals:

- Hemoglobin: 11.2 g/dL
- Mean corpuscular volume (MCV): 68 µm^3
- Red blood cell (RBC) count: 6.1 x 10^12/L
- Platelets: 250 x 10^9/L
- White blood cells (WBC): 6.0 x 10^9/L

OTHER STUDIES:
A peripheral blood smear shows target cells.

PLAN:
Serum iron studies and hemoglobin electrophoresis are ordered.

## Encounter 6: source 2024-03-15, practice 2024-10-15, outpatient, Hematology & Oncology

Chief complaint: Evaluation of low hemoglobin discovered during recent blood donation attempt.

ENCOUNTER NOTE
Date: 2024-03-15
Provider: Dr. David Park, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Evaluation of low hemoglobin discovered during recent blood donation attempt.

HISTORY OF PRESENT ILLNESS:
The patient presents for follow-up evaluation of anemia in the context of longstanding beta-thalassemia minor, with prior hematology encounters for similar complaints. She reports symptoms have been present for years and were previously attributed to "low iron," for which she briefly took supplements in the past. She recently donated blood and was informed her hemoglobin was low. She denies gastrointestinal symptoms, heavy menstrual bleeding, or recent infections.

PAST MEDICAL HISTORY:
Active Problem List:
- Thalassemia minor (diagnosed 2020-01-15)
- Iron deficiency anemia (diagnosed 2020-04-15)
- Endometriosis of ovary (Ovarian endometrioma) (diagnosed 2020-09-15)
- Beta-thalassemia minor (trait)
- Endometriosis

She has no chronic diseases.

PAST SURGICAL HISTORY:
- Laparoscopic right ovarian cystectomy for endometrioma

MEDICATIONS:
Home Medications:
- Multivitamin 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
Her family history is significant for a cousin with "some kind of blood disorder."

SOCIAL HISTORY:
Smoking: Never. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
Physical examination is unremarkable, with no pallor, jaundice, or splenomegaly.

LABORATORY DATA:
Laboratory studies show:
Hemoglobin                                                              10.5 g/dL
Hematocrit                                                                31%
Erythrocyte count                                                     5.9 million/mm³
MCV                                                                         70 µm³
RDW                                                                        13.1% (normal: 13–15%)
Reticulocyte count                                                    1.1%
Serum ferritin                                                           118 ng/mL
Serum iron                                                                85 µg/dL
TIBC                                                                         260 µg/dL
Coombs test                                                             Negative

## Encounter 7: source 2025-03-15, practice 2025-10-14, outpatient, Hematology & Oncology

Chief complaint: Asymptomatic participation in hemoglobin-oxygen dissociation research study.

ENCOUNTER NOTE
Date: 2025-03-15
Provider: Dr. Priya Patel, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Asymptomatic participation in hemoglobin-oxygen dissociation research study.

HISTORY OF PRESENT ILLNESS:
The patient with known beta-thalassemia minor returns for annual hematology follow-up. She is evaluated as part of a study for hemoglobin-oxygen dissociation metrics.

PAST MEDICAL HISTORY:
Active Problem List:
- Thalassemia minor (diagnosed 2020-01-15)
- Iron deficiency anemia (diagnosed 2020-04-15)
- Endometriosis of ovary (Ovarian endometrioma) (diagnosed 2020-09-15)
- Beta-thalassemia minor (trait)
- Endometriosis

PAST SURGICAL HISTORY:
- Laparoscopic right ovarian cystectomy for endometrioma

MEDICATIONS:
Home Medications:
- Multivitamin 1 tablet daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with beta-thalassemia minor
- Mother with beta-thalassemia minor
- Paternal cousin with beta-thalassemia major
- Maternal aunt with endometriosis

SOCIAL HISTORY:
Smoking: Never. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

LABORATORY DATA:
Curve is slightly left-shifted, with an increase in HbF and HbA2
