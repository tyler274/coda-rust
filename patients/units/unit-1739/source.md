# synthetic_hospital patient 1739

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 28 at first encounter, sex F, Hispanic, Commercial.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-09-12 (Mon) | outpatient | Rheumatology |
| 1 | 2020-04-15 | 2022-11-24 (Thu) | outpatient | Rheumatology |
| 2 | 2020-07-15 | 2023-03-06 (Mon) | ed | Emergency Medicine |
| 3 | 2021-01-15 | 2023-08-29 (Tue) | outpatient | Gastroenterology |
| 4 | 2021-07-15 | 2024-03-09 (Sat) | icu | Hematology/Oncology |
| 5 | 2022-07-15 | 2025-02-21 (Fri) | ed | Emergency Medicine |
| 6 | 2023-07-15 | 2026-03-09 (Mon) | outpatient | Obstetrics & Gynecology |
| 7 | 2023-09-15 | 2026-04-23 (Thu) | inpatient | General Surgery |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Commercial",
 "occupation": "Public health researcher",
 "alcohol_use": "Occasional social use (\u22641 drink per week)",
 "family_history": [
  "Mother: Systemic lupus erythematosus",
  "Maternal aunt: Rheumatoid arthritis",
  "Father: Hypertension",
  "Paternal grandfather: Colon cancer (age 62)"
 ],
 "race_ethnicity": "Hispanic",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "200 mg PO daily",
   "name": "Hydroxychloroquine"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Prednisone"
  },
  {
   "dose": "75 mg PO daily",
   "name": "Azathioprine"
  }
 ],
 "surgical_history": [
  "Laparoscopic appendectomy during second trimester of pregnancy"
 ],
 "chronic_conditions": [
  "Systemic lupus erythematosus",
  "Crohn's disease",
  "Raynaud's phenomenon",
  "History of acute promyelocytic leukemia (in remission)"
 ],
 "age_at_first_encounter": 28
}
```

Primary diagnoses: ["Systemic lupus erythematosus", "Systemic lupus erythematosus with arthropathy", "Crohn Disease", "Fetal Atrioventricular Block", "Acute appendicitis", "Acute Promyelocytic Leukemia with Disseminated Intravascular Coagulation", "Chloroquine-sensitive malaria prophylaxis"]
Comorbidities: ["Pregnancy", "Anemia", "Splenomegaly", "Leukocytosis", "Thrombocytopenia"]

## Encounter 0: source 2020-01-15, practice 2022-09-12, outpatient, Rheumatology

Chief complaint: 3-month history of fatigue, joint pain, and photosensitive rash with occasional fevers and unintentional weight loss

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Elena Rodriguez, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
3-month history of fatigue, joint pain, and photosensitive rash with occasional fevers and unintentional weight loss

HISTORY OF PRESENT ILLNESS:
She reports experiencing occasional fevers and unintentional weight loss over the last few months.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus
- Crohn's disease
- Raynaud's phenomenon
- History of acute promyelocytic leukemia (in remission)

Her medical history is significant for Raynaud's phenomenon and a miscarriage one year ago.

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy during second trimester of pregnancy

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg PO daily
- Prednisone 5 mg PO daily
- Azathioprine 75 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Systemic lupus erythematosus
- Maternal aunt: Rheumatoid arthritis
- Father: Hypertension
- Paternal grandfather: Colon cancer (age 62)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (≤1 drink per week). Occupation: Public health researcher.

PHYSICAL EXAMINATION:
On physical examination, you note a malar rash and mild splenomegaly. No lymphadenopathy is present.

LABORATORY DATA:
Laboratory studies reveal:

- Hemoglobin: 10.2 g/dL (Normal: 12-16 g/dL)
- Leukocyte count: 2.9 x 10^9/L (Normal: 4-11 x 10^9/L)
- Platelet count: 95 x 10^9/L (Normal: 150-400 x 10^9/L)
- Erythrocyte sedimentation rate (ESR): 50 mm/hr (Normal: 0-20 mm/hr)
- Antinuclear antibodies (ANA): Positive
- Anti-double-stranded DNA (anti-dsDNA): Positive
- Anti-Smith (anti-Sm) antibodies: Positive
- Complement levels (C3, C4): Low

## Encounter 1: source 2020-04-15, practice 2022-11-24, outpatient, Rheumatology

Chief complaint: Aching wrists and ankles for 10 days with mild morning stiffness and new painless oral sore

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Elena Rodriguez, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Aching wrists and ankles for 10 days with mild morning stiffness and new painless oral sore

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of systemic lupus erythematosus diagnosed three months prior. She describes mild morning stiffness and difficulty opening jars. She notes that she recently developed a painless sore inside her cheek.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus with arthropathy (diagnosed 2020-01-15)
- Systemic lupus erythematosus
- Crohn's disease
- Raynaud's phenomenon
- History of acute promyelocytic leukemia (in remission)

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy during second trimester of pregnancy

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg PO daily
- Prednisone 5 mg PO daily
- Azathioprine 75 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
She has no family history of similar symptoms, but her mother has hypothyroidism.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (≤1 drink per week). Occupation: Public health researcher.

PHYSICAL EXAMINATION:
On physical examination, there is mild swelling of several metacarpophalangeal and proximal interphalangeal joints, as well as a shallow ulcer on the buccal mucosa.

LABORATORY DATA:
Laboratory studies reveal a hemoglobin of 11.2 g/dL, hematocrit of 33.6%, white blood cell count of 4,200/µL, and platelet count of 55,000/µL.

## Encounter 2: source 2020-07-15, practice 2023-03-06, ed, Emergency Medicine

Chief complaint: 8 hours of progressive abdominal pain migrating from umbilicus to right lower quadrant with emesis

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
8 hours of progressive abdominal pain migrating from umbilicus to right lower quadrant with emesis

HISTORY OF PRESENT ILLNESS:
The patient is a 28-year-old female with systemic lupus erythematosus under active rheumatology management since January 2020, Crohn's disease, and maintenance immunosuppressive therapy who presents with acute abdominal pain. The pain began as vague discomfort around the umbilicus and has now localized to the right lower quadrant. She has experienced two episodes of non-bloody, non-bilious emesis and reports subjective fever and chills. She has had no vaginal bleeding, leakage of fluid, or dysuria. She has had regular prenatal care with no complications until now.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus with arthropathy (diagnosed 2020-01-15)
- Systemic lupus erythematosus (diagnosed 2020-04-15)
- Crohn's disease
- Raynaud's phenomenon
- History of acute promyelocytic leukemia (in remission)

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy during second trimester of pregnancy

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg PO daily
- Prednisone 5 mg PO daily
- Azathioprine 75 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Systemic lupus erythematosus
- Maternal aunt: Rheumatoid arthritis
- Father: Hypertension
- Paternal grandfather: Colon cancer (age 62)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (≤1 drink per week). Occupation: Public health researcher.

VITAL SIGNS:
Her temperature is 38.4°C (101.1°F), pulse is 114/min, and blood pressure is 98/64 mm Hg.

PHYSICAL EXAMINATION:
Abdominal examination reveals focal tenderness in the right lower quadrant with voluntary guarding. A speculum exam shows a closed cervix with no discharge. The fetal heart rate is 170/min with moderate variability, no decelerations, and intermittent contractions are noted on external tocometry.

LABORATORY DATA:
Her white blood cell count is 17,900/mm³, and urinalysis is normal.

## Encounter 3: source 2021-01-15, practice 2023-08-29, outpatient, Gastroenterology

Chief complaint: 6-month history of intermittent abdominal pain, bloody diarrhea, and unintentional weight loss

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Anita Kapoor, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
6-month history of intermittent abdominal pain, bloody diarrhea, and unintentional weight loss

HISTORY OF PRESENT ILLNESS:
The patient is a 29-year-old female with a history of systemic lupus erythematosus diagnosed in January 2020 and acute appendicitis treated in July 2020 who presents to the clinic with a 6-month history of intermittent abdominal pain, diarrhea, and unintentional weight loss. She reports that her bowel movements are sometimes bloody and often accompanied by a feeling of incomplete evacuation.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus with arthropathy (diagnosed 2020-01-15)
- Systemic lupus erythematosus (diagnosed 2020-04-15)
- Acute appendicitis (diagnosed 2020-07-15)
- Crohn's disease
- Raynaud's phenomenon
- History of acute promyelocytic leukemia (in remission)

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy during second trimester of pregnancy

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg PO daily
- Prednisone 5 mg PO daily
- Azathioprine 75 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
She denies any family history of gastrointestinal diseases.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (≤1 drink per week). Occupation: Public health researcher.

PHYSICAL EXAMINATION:
On examination, she appears underweight and has mild tenderness in the right lower quadrant. Perianal inspection reveals skin tags and a small fissure.

LABORATORY DATA:
Laboratory tests show:

- Hemoglobin: 11.2 g/dL (Normal: 12-16 g/dL)
- White blood cell count: 9,500 /µL (Normal: 4,500-11,000 /µL)
- C-reactive protein: 24 mg/L (Normal: <5 mg/L)
- Erythrocyte sedimentation rate: 40 mm/hr (Normal: 0-20 mm/hr)

PATHOLOGY:
Biopsy of the affected areas shows transmural inflammation and non-caseating granulomas.

OTHER STUDIES:
Colonoscopy demonstrates segmental areas of inflamed mucosa with linear ulcers and cobblestoning.

## Encounter 4: source 2021-07-15, practice 2024-03-09, icu, Hematology/Oncology

Chief complaint: Persistent oozing from venipuncture sites and spontaneous bruising

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. David Kim, MD
Department: Hematology/Oncology
Visit Type: icu

CHIEF COMPLAINT:
Persistent oozing from venipuncture sites and spontaneous bruising

HISTORY OF PRESENT ILLNESS:
The patient is a 29-year-old female with systemic lupus erythematosus, Crohn's disease, and prior acute promyelocytic leukemia in remission who presents with bleeding symptoms. She reports that over the past day, nurses have needed to change her dressings more frequently due to steady blood seepage. She has also noticed bruising developing on her arms and legs without any associated trauma. She denies recent nosebleeds, heavy menstrual bleeding, or gastrointestinal symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus with arthropathy (diagnosed 2020-01-15)
- Systemic lupus erythematosus (diagnosed 2020-04-15)
- Acute appendicitis (diagnosed 2020-07-15)
- Crohn Disease (diagnosed 2021-01-15)
- Crohn's disease
- Raynaud's phenomenon
- History of acute promyelocytic leukemia (in remission)

Cytogenetic studies confirmed a t(15;17) translocation, and her oncologist had expressed concern about her risk for coagulopathy related to treatment.

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy during second trimester of pregnancy

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg PO daily
- Prednisone 5 mg PO daily
- Azathioprine 75 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Systemic lupus erythematosus
- Maternal aunt: Rheumatoid arthritis
- Father: Hypertension
- Paternal grandfather: Colon cancer (age 62)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (≤1 drink per week). Occupation: Public health researcher.

VITAL SIGNS:
Her blood pressure is normal, and she is afebrile.

PHYSICAL EXAMINATION:
She is alert and oriented but appears pale and mildly tachycardic. Physical examination reveals scattered petechiae on her lower extremities and non-tender ecchymoses overlying her forearms. There is no hepatosplenomegaly, lymphadenopathy, or focal neurologic deficit. Her central line site and several IV lines have persistent oozing despite local pressure.

LABORATORY DATA:
Initial labs reveal anemia and thrombocytopenia. Laboratory studies and a blood smear are shown:
Bleeding time 12 minutes
Prothrombin time 22 seconds
Partial thromboplastin time 46 seconds

## Encounter 5: source 2022-07-15, practice 2025-02-21, ed, Emergency Medicine

Chief complaint: Intermittent fevers, chills, drenching sweats, and myalgias after returning from sub-Saharan Africa

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Intermittent fevers, chills, drenching sweats, and myalgias after returning from sub-Saharan Africa

HISTORY OF PRESENT ILLNESS:
The patient is a 30-year-old female with systemic lupus erythematosus, Crohn's disease, and acute promyelocytic leukemia in remission on hydroxychloroquine, prednisone, and azathioprine, who returns from a 3-week volunteer trip to a rural farming region in sub-Saharan Africa. She reports that while abroad she took a daily tablet prescribed for prevention of a parasitic illness, but over the last 5 days she has developed intermittent fevers, chills, drenching sweats, malaise, and diffuse myalgias.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus with arthropathy (diagnosed 2020-01-15)
- Systemic lupus erythematosus (diagnosed 2020-04-15)
- Acute appendicitis (diagnosed 2020-07-15)
- Crohn Disease (diagnosed 2021-01-15)
- Acute Promyelocytic Leukemia with Disseminated Intravascular Coagulation (diagnosed 2021-07-15)
- Crohn's disease
- Raynaud's phenomenon
- History of acute promyelocytic leukemia (in remission)

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy during second trimester of pregnancy

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg PO daily
- Prednisone 5 mg PO daily
- Azathioprine 75 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Systemic lupus erythematosus
- Maternal aunt: Rheumatoid arthritis
- Father: Hypertension
- Paternal grandfather: Colon cancer (age 62)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (≤1 drink per week). Occupation: Public health researcher.

REVIEW OF SYSTEMS:
She denies respiratory or urinary symptoms.

VITAL SIGNS:
Temperature is 39.5 °C (103.1 °F), pulse is 110/min, blood pressure is 104/68 mmHg, and respirations are 20/min.

PHYSICAL EXAMINATION:
On examination, she appears fatigued and diaphoretic. The sclerae are anicteric, lungs are clear, and the abdomen is soft but mildly enlarged in the left upper quadrant.

LABORATORY DATA:
Laboratory testing reveals normocytic anemia and thrombocytopenia. A peripheral blood smear demonstrates intracellular ring-shaped inclusions within red blood cells.

## Encounter 6: source 2023-07-15, practice 2026-03-09, outpatient, Obstetrics & Gynecology

Chief complaint: Routine prenatal visit with concern for decreased fetal movements

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Jennifer Walsh, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine prenatal visit with concern for decreased fetal movements

HISTORY OF PRESENT ILLNESS:
She returns for prenatal care with systemic lupus erythematosus, Crohn disease, and leukemia. She reports no rash, joint pain, or other lupus-related symptoms during pregnancy. However, she is concerned because she feels the baby's movements have decreased over the past week.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus with arthropathy (diagnosed 2020-01-15)
- Systemic lupus erythematosus (diagnosed 2020-04-15)
- Acute appendicitis (diagnosed 2020-07-15)
- Crohn Disease (diagnosed 2021-01-15)
- Acute Promyelocytic Leukemia with Disseminated Intravascular Coagulation (diagnosed 2021-07-15)
- Chloroquine-sensitive malaria prophylaxis (diagnosed 2022-07-15)
- Crohn's disease
- Raynaud's phenomenon
- History of acute promyelocytic leukemia (in remission)

She has a history of systemic lupus erythematosus (SLE). Her obstetric history is significant for one previous miscarriage at 10 weeks gestation.

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy during second trimester of pregnancy

MEDICATIONS:
She has been stable on hydroxychloroquine and low-dose prednisone.

Home Medications:
- Azathioprine 75 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Systemic lupus erythematosus
- Maternal aunt: Rheumatoid arthritis
- Father: Hypertension
- Paternal grandfather: Colon cancer (age 62)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (≤1 drink per week). Occupation: Public health researcher.

VITAL SIGNS:
On examination, her blood pressure is 120/78 mmHg, heart rate is 88 bpm, and temperature is 98.6°F (37°C)

PHYSICAL EXAMINATION:
Fetal heart rate monitoring shows a persistent fetal heart rate of 90 bpm. There are no signs of fetal distress or uterine contractions.

LABORATORY DATA:
Laboratory results are as follows:
- Anti-SSA (Ro) antibodies: Positive
- Anti-SSB (La) antibodies: Positive
- Complete blood count: Within normal limits
- Urinalysis: No proteinuria or active sediment

## Encounter 7: source 2023-09-15, practice 2026-04-23, inpatient, General Surgery

Chief complaint: 24-hour history of right-sided abdominal pain migrating from umbilicus to right upper quadrant with nausea

ENCOUNTER NOTE
Date: 2023-09-15
Provider: Dr. Robert Chang, MD
Department: General Surgery
Visit Type: inpatient

CHIEF COMPLAINT:
24-hour history of right-sided abdominal pain migrating from umbilicus to right upper quadrant with nausea

HISTORY OF PRESENT ILLNESS:
This 31-year-old female with systemic lupus erythematosus, Crohn's disease, and a history of acute appendicitis in 2020 presents to the emergency department with a 24-hour history of right-sided abdominal pain. She describes the pain as sharp and constant, initially located around the umbilicus but now localized to the right mid-to-upper quadrant. She also reports nausea but no vomiting, fever, or changes in bowel habits.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus with arthropathy (diagnosed 2020-01-15)
- Systemic lupus erythematosus (diagnosed 2020-04-15)
- Acute appendicitis (diagnosed 2020-07-15)
- Crohn Disease (diagnosed 2021-01-15)
- Acute Promyelocytic Leukemia with Disseminated Intravascular Coagulation (diagnosed 2021-07-15)
- Chloroquine-sensitive malaria prophylaxis (diagnosed 2022-07-15)
- Fetal Atrioventricular Block (diagnosed 2023-07-15)
- Crohn's disease
- Raynaud's phenomenon
- History of acute promyelocytic leukemia (in remission)

She has had an uncomplicated pregnancy thus far.

PAST SURGICAL HISTORY:
- Laparoscopic appendectomy during second trimester of pregnancy

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg PO daily
- Prednisone 5 mg PO daily
- Azathioprine 75 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Systemic lupus erythematosus
- Maternal aunt: Rheumatoid arthritis
- Father: Hypertension
- Paternal grandfather: Colon cancer (age 62)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (≤1 drink per week). Occupation: Public health researcher.

VITAL SIGNS:
On examination, her temperature is 37.8°C (100°F), heart rate is 98 bpm, and blood pressure is 125/75 mmHg.

PHYSICAL EXAMINATION:
Abdominal examination reveals tenderness and guarding in the right mid-to-upper quadrant with a negative Murphy's sign.

LABORATORY DATA:
Laboratory tests show a white blood cell count of 12,000/µL with a left shift. Urinalysis is normal.

IMAGING:
An obstetric ultrasound confirms a viable intrauterine pregnancy and no evidence of cholecystitis or pyelonephritis.
