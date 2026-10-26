# synthetic_hospital patient 1805

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 33 at first encounter, sex F, Non-Hispanic White, Commercial PPO (employer-sponsored).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-12-16 (Wed) | icu | Emergency Medicine |
| 1 | 2020-04-15 | 2021-03-15 (Mon) | outpatient | Allergy & Immunology |
| 2 | 2020-07-15 | 2021-06-16 (Wed) | outpatient | Psychiatry |
| 3 | 2021-01-15 | 2021-12-20 (Mon) | outpatient | Gastroenterology |
| 4 | 2021-07-15 | 2022-06-10 (Fri) | outpatient | Hematology and Oncology |
| 5 | 2023-01-15 | 2023-12-05 (Tue) | outpatient | Obstetrics & Gynecology |
| 6 | 2024-01-15 | 2024-12-17 (Tue) | outpatient | Gastroenterology |
| 7 | 2025-01-15 | 2025-12-09 (Tue) | outpatient | Ear Nose & Throat |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Blood products (IgA deficiency-related anaphylaxis)",
  "Environmental: Tree pollen, dust mites"
 ],
 "insurance": "Commercial PPO (employer-sponsored)",
 "occupation": "Office administrator",
 "alcohol_use": "Occasional (1 drink per month)",
 "family_history": [
  "Sister with celiac disease",
  "Mother with autoimmune hypothyroidism and seasonal allergies",
  "Maternal aunt with uterine fibroids",
  "Father with hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "10 mg oral as needed for allergic rhinitis",
   "name": "Cetirizine"
  }
 ],
 "surgical_history": [
  "Emergency exploratory laparotomy and hemorrhage control following motor vehicle accident (5 years prior)",
  "Blood transfusion complicated by anaphylactic reaction (during above surgery)"
 ],
 "chronic_conditions": [
  "Celiac disease",
  "Selective IgA deficiency",
  "Chronic iron deficiency anemia",
  "Uterine leiomyoma (fibroids)",
  "Generalized anxiety disorder",
  "Dependent personality disorder",
  "Atopic dermatitis (eczema)",
  "Seasonal allergic rhinitis"
 ],
 "age_at_first_encounter": 33
}
```

Primary diagnoses: ["Celiac disease", "Acute Bacterial Sinusitis", "Selective IgA deficiency", "Uterine Leiomyoma (Intramural)", "Dependent personality disorder", "Iron Deficiency Anemia"]
Comorbidities: ["Venous Air Embolism", "Iron Deficiency Anemia"]

## Encounter 0: source 2020-01-15, practice 2020-12-16, icu, Emergency Medicine

Chief complaint: Admitted for emergency surgery following motor vehicle accident with subsequent transfusion reaction

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Admitted for emergency surgery following motor vehicle accident with subsequent transfusion reaction

HISTORY OF PRESENT ILLNESS:
Due to ongoing bleeding, she receives a blood tranfusion in the emergency department. Within an hour, she develops vomiting, diarrhea, hypotension, and shortness of breath. She is given intramuscular epinephrine and stabilized.

PAST MEDICAL HISTORY:
Active Problem List:
- Celiac disease
- Selective IgA deficiency
- Chronic iron deficiency anemia
- Uterine leiomyoma (fibroids)
- Generalized anxiety disorder
- Dependent personality disorder
- Atopic dermatitis (eczema)
- Seasonal allergic rhinitis

PAST SURGICAL HISTORY:
- Emergency exploratory laparotomy and hemorrhage control following motor vehicle accident (5 years prior)
- Blood transfusion complicated by anaphylactic reaction (during above surgery)

MEDICATIONS:
Home Medications:
- Cetirizine 10 mg oral as needed for allergic rhinitis

ALLERGIES:
Blood products (IgA deficiency-related anaphylaxis), Environmental: Tree pollen, dust mites

FAMILY HISTORY:
- Sister with celiac disease
- Mother with autoimmune hypothyroidism and seasonal allergies
- Maternal aunt with uterine fibroids
- Father with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1 drink per month). Occupation: Office administrator.

ASSESSMENT AND PLAN:
The deficient antibody responsible for this patient's condition is most appropriately described as

## Encounter 1: source 2020-04-15, practice 2021-03-15, outpatient, Allergy & Immunology

Chief complaint: Recurrent sinus infections and chronic diarrhea

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. James Rodriguez, MD
Department: Allergy & Immunology
Visit Type: outpatient

CHIEF COMPLAINT:
Recurrent sinus infections and chronic diarrhea

HISTORY OF PRESENT ILLNESS:
The patient with selective IgA deficiency returns for follow-up three months after her ICU admission. She reports having four sinus infections in the past year, each requiring antibiotic therapy. She also describes frequent episodes of bloating and loose stools that seem to worsen with stress.

PAST MEDICAL HISTORY:
Active Problem List:
- Selective IgA deficiency (diagnosed 2020-01-15)
- Celiac disease
- Chronic iron deficiency anemia
- Uterine leiomyoma (fibroids)
- Generalized anxiety disorder
- Dependent personality disorder
- Atopic dermatitis (eczema)
- Seasonal allergic rhinitis

PAST SURGICAL HISTORY:
- Emergency exploratory laparotomy and hemorrhage control following motor vehicle accident (5 years prior)
- Blood transfusion complicated by anaphylactic reaction (during above surgery)

MEDICATIONS:
She does not take any regular medications.

Home Medications:
- Cetirizine 10 mg oral as needed for allergic rhinitis

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Sister with celiac disease
- Mother with autoimmune hypothyroidism and seasonal allergies
- Maternal aunt with uterine fibroids
- Father with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1 drink per month). Occupation: Office administrator.

VITAL SIGNS:
Her vital signs are within normal limits

PHYSICAL EXAMINATION:
Examination of the head and neck reveals mild erythema of the nasal mucosa and clear postnasal drainage. The remainder of the physical examination is unremarkable.

LABORATORY DATA:
- Hemoglobin: 13.2 g/dL
- White blood cell count: 7,000/mm^3
- Platelet count: 250,000/mm^3
- Serum immunoglobulin A (IgA): Undetectable
- Serum immunoglobulin G (IgG): Within normal limits
- Serum immunoglobulin M (IgM): Within normal limits

## Encounter 2: source 2020-07-15, practice 2021-06-16, outpatient, Psychiatry

Chief complaint: Anxiety and difficulty making decisions

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Emily Watson, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Anxiety and difficulty making decisions

HISTORY OF PRESENT ILLNESS:
The patient returns for psychiatric management following recent ICU admission and immunology evaluation. She reports that she feels extremely nervous when her husband is away on business trips, to the point where she has to stay with her parents. She has difficulty expressing disagreement with others for fear of losing their support or approval.

PAST MEDICAL HISTORY:
Active Problem List:
- Selective IgA deficiency (diagnosed 2020-01-15)
- Celiac disease
- Chronic iron deficiency anemia
- Uterine leiomyoma (fibroids)
- Generalized anxiety disorder
- Dependent personality disorder
- Atopic dermatitis (eczema)
- Seasonal allergic rhinitis

Her medical history is significant for chronic iron deficiency anemia and generalized anxiety disorder.

PAST SURGICAL HISTORY:
- Emergency exploratory laparotomy and hemorrhage control following motor vehicle accident (5 years prior)
- Blood transfusion complicated by anaphylactic reaction (during above surgery)

MEDICATIONS:
She is currently.

Home Medications:
- Cetirizine 10 mg oral as needed for allergic rhinitis

ALLERGIES:
Blood products (IgA deficiency-related anaphylaxis), Environmental: Tree pollen, dust mites

FAMILY HISTORY:
- Sister with celiac disease
- Mother with autoimmune hypothyroidism and seasonal allergies
- Maternal aunt with uterine fibroids
- Father with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1 drink per month). Occupation: Office administrator.

PHYSICAL EXAMINATION:
On physical examination, she appears well-groomed but is notably clingy with her mother, who accompanied her to the appointment. She has no abnormal findings on a complete physical examination.

LABORATORY DATA:
Her recent laboratory tests show:

- Hemoglobin: 11.2 g/dL (normal: 12-15.5 g/dL)
- Mean corpuscular volume (MCV): 79 fL (normal: 80-100 fL)
- Serum iron: 45 µg/dL (normal: 50-170 µg/dL)
- Total iron-binding capacity (TIBC): 400 µg/dL (normal: 250-450 µg/dL)
- Ferritin: 15 ng/mL (normal: 15-200 ng/mL)

## Encounter 3: source 2021-01-15, practice 2021-12-20, outpatient, Gastroenterology

Chief complaint: Chronic diarrhea, bloating, and blistering rash on elbows and knees

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Michael Patel, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Chronic diarrhea, bloating, and blistering rash on elbows and knees

HISTORY OF PRESENT ILLNESS:
The patient with selective IgA deficiency presents to Gastroenterology following her April 2020 Allergy and Immunology evaluation, with a 6-month history of chronic diarrhea, bloating, and fatigue. She reports that her symptoms seem to worsen after eating bread and pasta. She has also noticed a non-pruritic blistering rash on her elbows and knees.

PAST MEDICAL HISTORY:
Active Problem List:
- Selective IgA deficiency (diagnosed 2020-01-15)
- Dependent personality disorder (diagnosed 2020-07-15)
- Celiac disease
- Chronic iron deficiency anemia
- Uterine leiomyoma (fibroids)
- Generalized anxiety disorder
- Atopic dermatitis (eczema)
- Seasonal allergic rhinitis

Her past medical history is significant for recurrent sinus infections.

PAST SURGICAL HISTORY:
- Emergency exploratory laparotomy and hemorrhage control following motor vehicle accident (5 years prior)
- Blood transfusion complicated by anaphylactic reaction (during above surgery)

MEDICATIONS:
Home Medications:
- Cetirizine 10 mg oral as needed for allergic rhinitis

ALLERGIES:
Blood products (IgA deficiency-related anaphylaxis), Environmental: Tree pollen, dust mites

FAMILY HISTORY:
- Sister with celiac disease
- Mother with autoimmune hypothyroidism and seasonal allergies
- Maternal aunt with uterine fibroids
- Father with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1 drink per month). Occupation: Office administrator.

PHYSICAL EXAMINATION:
On physical examination, she appears pale and has mild abdominal distension with diffuse tenderness to palpation. Dermatological examination reveals vesicles over her extensor surfaces.

LABORATORY DATA:
Laboratory tests reveal:

- Hemoglobin: 10.2 g/dL (Normal: 12-16 g/dL)
- Mean corpuscular volume (MCV): 74 fL (Normal: 80-100 fL)
- Serum iron: 30 µg/dL (Normal: 50-170 µg/dL)
- Total iron-binding capacity (TIBC): 450 µg/dL (Normal: 250-450 µg/dL)
- Ferritin: 15 ng/mL (Normal: 15-200 ng/mL)
- Serum IgA: 20 mg/dL (Normal: 70-400 mg/dL)

## Encounter 4: source 2021-07-15, practice 2022-06-10, outpatient, Hematology and Oncology

Chief complaint: Fatigue, difficulty concentrating, and pica (chewing ice and corn starch)

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Lisa Johnson, MD
Department: Hematology and Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue, difficulty concentrating, and pica (chewing ice and corn starch)

HISTORY OF PRESENT ILLNESS:
The patient returns to the Hematology and Oncology clinic for management of recurrent iron deficiency anemia in the setting of Celiac disease diagnosed earlier this year and longstanding selective IgA deficiency. She reports fatigue, difficulty concentrating, and pica characterized by chewing ice and corn starch over the past several months. She denies heavy menstrual bleeding, gastrointestinal symptoms, or blood loss.

PAST MEDICAL HISTORY:
Active Problem List:
- Selective IgA deficiency (diagnosed 2020-01-15)
- Dependent personality disorder (diagnosed 2020-07-15)
- Celiac disease (diagnosed 2021-01-15)
- Chronic iron deficiency anemia
- Uterine leiomyoma (fibroids)
- Generalized anxiety disorder
- Atopic dermatitis (eczema)
- Seasonal allergic rhinitis

She has no prior medical conditions.

PAST SURGICAL HISTORY:
- Emergency exploratory laparotomy and hemorrhage control following motor vehicle accident (5 years prior)
- Blood transfusion complicated by anaphylactic reaction (during above surgery)

MEDICATIONS:
Home Medications:
- Cetirizine 10 mg oral as needed for allergic rhinitis

ALLERGIES:
Blood products (IgA deficiency-related anaphylaxis), Environmental: Tree pollen, dust mites

FAMILY HISTORY:
- Sister with celiac disease
- Mother with autoimmune hypothyroidism and seasonal allergies
- Maternal aunt with uterine fibroids
- Father with hypertension

SOCIAL HISTORY:
She works in an office and follows a standard Western diet.

PHYSICAL EXAMINATION:
On physical exam, her conjunctivae are pale, and her tongue appears smooth.

LABORATORY DATA:
Laboratory testing confirms microcytic anemia.

PLAN:
She is started on oral iron supplementation. One week later, repeat blood work is performed to monitor her response.

## Encounter 5: source 2023-01-15, practice 2023-12-05, outpatient, Obstetrics & Gynecology

Chief complaint: Increasingly heavy and painful menstrual periods with pelvic pressure

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Rebecca Martinez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Increasingly heavy and painful menstrual periods with pelvic pressure

HISTORY OF PRESENT ILLNESS:
The patient, with established care for selective IgA deficiency, celiac disease, and chronic iron deficiency anemia previously evaluated in hematology, presents to the clinic with a 6-month history of increasingly heavy and painful menstrual periods. She reports that her menstrual cycle used to be regular, lasting 4-5 days, but now extends up to 7-8 days with significant clotting. She also experiences pelvic pressure and frequent urination.

PAST MEDICAL HISTORY:
Active Problem List:
- Selective IgA deficiency (diagnosed 2020-01-15)
- Dependent personality disorder (diagnosed 2020-07-15)
- Celiac disease (diagnosed 2021-01-15)
- Iron Deficiency Anemia (diagnosed 2021-07-15)
- Uterine leiomyoma (fibroids)
- Generalized anxiety disorder
- Atopic dermatitis (eczema)
- Seasonal allergic rhinitis

She has never been pregnant.

PAST SURGICAL HISTORY:
- Emergency exploratory laparotomy and hemorrhage control following motor vehicle accident (5 years prior)
- Blood transfusion complicated by anaphylactic reaction (during above surgery)

MEDICATIONS:
Home Medications:
- Cetirizine 10 mg oral as needed for allergic rhinitis

ALLERGIES:
Blood products (IgA deficiency-related anaphylaxis), Environmental: Tree pollen, dust mites

FAMILY HISTORY:
- Sister with celiac disease
- Mother with autoimmune hypothyroidism and seasonal allergies
- Maternal aunt with uterine fibroids
- Father with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1 drink per month). Occupation: Office administrator.

PHYSICAL EXAMINATION:
On physical examination, her abdomen is non-tender, but an enlarged, irregularly shaped uterus is palpable.

LABORATORY DATA:
Her laboratory tests are as follows:

- Hemoglobin: 10.2 g/dL (Normal: 12-15.5 g/dL)
- Hematocrit: 30.4% (Normal: 36-46%)
- Platelets: 250,000/mm³ (Normal: 150,000-400,000/mm³)

IMAGING:
A pelvic ultrasound reveals a 5 cm intramural fibroid.

## Encounter 6: source 2024-01-15, practice 2024-12-17, outpatient, Gastroenterology

Chief complaint: Fatigue, bloating, and unintentional weight loss following international travel

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Michael Patel, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue, bloating, and unintentional weight loss following international travel

HISTORY OF PRESENT ILLNESS:
The patient returns to Gastroenterology for follow-up of Celiac disease, Selective IgA deficiency, and iron deficiency anemia. She is unsure what triggers the symptoms, which tend to fluctuate, and denies any fever, vomiting, or abdominal pain. She recently returned from a trip abroad and has unintentionally lost 6.5 kg (14 lb) despite maintaining a normal diet.

PAST MEDICAL HISTORY:
Active Problem List:
- Selective IgA deficiency (diagnosed 2020-01-15)
- Dependent personality disorder (diagnosed 2020-07-15)
- Celiac disease (diagnosed 2021-01-15)
- Iron Deficiency Anemia (diagnosed 2021-07-15)
- Uterine Leiomyoma (Intramural) (diagnosed 2023-01-15)
- Uterine leiomyoma (fibroids)
- Generalized anxiety disorder
- Atopic dermatitis (eczema)
- Seasonal allergic rhinitis

Her medical history is notable for iron deficiency anemia treated 2 years ago.

PAST SURGICAL HISTORY:
- Emergency exploratory laparotomy and hemorrhage control following motor vehicle accident (5 years prior)
- Blood transfusion complicated by anaphylactic reaction (during above surgery)

MEDICATIONS:
Home Medications:
- Cetirizine 10 mg oral as needed for allergic rhinitis

ALLERGIES:
Blood products (IgA deficiency-related anaphylaxis), Environmental: Tree pollen, dust mites

FAMILY HISTORY:
- Sister with celiac disease
- Mother with autoimmune hypothyroidism and seasonal allergies
- Maternal aunt with uterine fibroids
- Father with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1 drink per month). Occupation: Office administrator.

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

LABORATORY DATA:
Laboratory results show a hemoglobin of 10.8 g/dL and mean corpuscular volume of 72 fL. A D-xylose absorption test reveals a serum D-xylose concentration of 6 mg/dL (normal: 25–40) two hours after ingestion of 25 g of D-xylose.

## Encounter 7: source 2025-01-15, practice 2025-12-09, outpatient, Ear Nose & Throat

Chief complaint: Nasal congestion, facial fullness, and cough with double-sickening pattern

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. David Kim, MD
Department: Ear Nose & Throat
Visit Type: outpatient

CHIEF COMPLAINT:
Nasal congestion, facial fullness, and cough with double-sickening pattern

HISTORY OF PRESENT ILLNESS:
This 38-year-old female with selective IgA deficiency and a history of recurrent sinus infections presents with upper respiratory symptoms. She denies having a fever. She reports that her symptoms seemed to improve slightly on the fifth day but have since plateaued.

PAST MEDICAL HISTORY:
Active Problem List:
- Selective IgA deficiency (diagnosed 2020-01-15)
- Dependent personality disorder (diagnosed 2020-07-15)
- Celiac disease (diagnosed 2021-01-15)
- Iron Deficiency Anemia (diagnosed 2021-07-15)
- Uterine Leiomyoma (Intramural) (diagnosed 2023-01-15)
- Uterine leiomyoma (fibroids)
- Generalized anxiety disorder
- Atopic dermatitis (eczema)
- Seasonal allergic rhinitis

PAST SURGICAL HISTORY:
- Emergency exploratory laparotomy and hemorrhage control following motor vehicle accident (5 years prior)
- Blood transfusion complicated by anaphylactic reaction (during above surgery)

MEDICATIONS:
Cetirizine as needed

ALLERGIES:
Blood products (IgA deficiency-related anaphylaxis), Environmental: Tree pollen, dust mites

FAMILY HISTORY:
- Sister with celiac disease
- Mother with autoimmune hypothyroidism and seasonal allergies
- Maternal aunt with uterine fibroids
- Father with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional (1 drink per month). Occupation: Office administrator.

VITAL SIGNS:
Temperature is 37.2°C (99°F)

PHYSICAL EXAMINATION:
Tenderness to palpation over the maxillary sinuses and purulent nasal discharge. There is no dental pain, and transillumination of the sinuses is normal.
