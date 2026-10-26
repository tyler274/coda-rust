# synthetic_hospital patient 1733

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 30 at first encounter, sex F, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-05-14 (Thu) | outpatient | Family Medicine |
| 1 | 2020-05-15 | 2020-09-09 (Wed) | outpatient | Obstetrics & Gynecology |
| 2 | 2021-05-15 | 2021-09-06 (Mon) | outpatient | Hematology & Oncology |
| 3 | 2022-09-15 | 2022-12-29 (Thu) | outpatient | Obstetrics & Gynecology |
| 4 | 2023-05-15 | 2023-09-12 (Tue) | outpatient | Obstetrics & Gynecology |
| 5 | 2024-05-15 | 2024-09-05 (Thu) | ed | Emergency Medicine |
| 6 | 2024-11-15 | 2025-03-06 (Thu) | inpatient | Obstetrics & Gynecology |
| 7 | 2025-11-15 | 2026-03-04 (Wed) | inpatient | Neurology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicaid",
 "occupation": "Unemployed",
 "alcohol_use": "Social, 1-2 drinks per month",
 "family_history": [
  "No family history of bleeding disorders",
  "Mother with hypertension",
  "Father with type 2 diabetes"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 5 cigarettes per day",
 "home_medications": [],
 "surgical_history": [
  "No prior surgeries"
 ],
 "chronic_conditions": [
  "History of recurrent chlamydia infection",
  "History of recurrent urinary tract infections",
  "History of intravenous drug use"
 ],
 "age_at_first_encounter": 30
}
```

Primary diagnoses: ["Acute pelvic inflammatory disease with tubo-ovarian abscess", "Idiopathic thrombocytopenic purpura", "Anogenital herpes simplex virus infection", "Asymptomatic Actinomyces colonization of genital tract", "Trichomoniasis", "Acute transverse myelitis", "Chancroid", "History of intravenous drug use"]
Comorbidities: ["High risk sexual behavior requiring counseling", "High risk sexual behavior", "History of intravenous drug use"]

## Encounter 0: source 2020-01-15, practice 2020-05-14, outpatient, Family Medicine

Chief complaint: Routine health examination with no acute complaints.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Family Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine health examination with no acute complaints.

HISTORY OF PRESENT ILLNESS:
She has no complaints and feels generally well.

PAST MEDICAL HISTORY:
Active Problem List:
- History of recurrent chlamydia infection
- History of recurrent urinary tract infections
- History of intravenous drug use

She cannot recall the results. She has never been vaccinated against human papillomavirus (HPV).

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
She takes no medications.

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- No family history of bleeding disorders
- Mother with hypertension
- Father with type 2 diabetes

SOCIAL HISTORY:
She has been sexually active with multiple partners and inconsistently uses condoms.

VITAL SIGNS:
On examination, her vital signs are within normal limits,

PHYSICAL EXAMINATION:
Her physical examination is unremarkable.

PATHOLOGY:
A Pap test is performed, and the results show a high-grade squamous intraepithelial lesion (HSIL).

## Encounter 1: source 2020-05-15, practice 2020-09-09, outpatient, Obstetrics & Gynecology

Chief complaint: Routine prenatal visit at 18 weeks gestation with questions about pet-related infection precautions.

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. Maria Gonzalez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine prenatal visit at 18 weeks gestation with questions about pet-related infection precautions.

HISTORY OF PRESENT ILLNESS:
Patient with IV drug use, recurrent infections, and Actinomyces colonization presents at 18 weeks for prenatal care. She reports feeling well with no complaints but voices concern regarding pet-related infection risks and inquires about necessary precautions.

PAST MEDICAL HISTORY:
Active Problem List:
- Asymptomatic Actinomyces colonization of genital tract (diagnosed 2020-01-15)
- History of recurrent chlamydia infection
- History of recurrent urinary tract infections
- History of intravenous drug use

She has had an uncomplicated pregnancy thus far.

PAST SURGICAL HISTORY:
- No prior surgeries

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- No family history of bleeding disorders
- Mother with hypertension
- Father with type 2 diabetes

SOCIAL HISTORY:
She has a cat at home and mentions that she recently adopted another one from a local shelter.

VITAL SIGNS:
On examination, her vital signs are within normal limits,

PHYSICAL EXAMINATION:
The obstetric examination is consistent with her gestational age. She has no lymphadenopathy or skin lesions.

LABORATORY DATA:
Routine prenatal labs have been normal, including blood type, Rh factor, and antibody screen.
The results return as follows: - Toxoplasma gondii IgG: Positive - Toxoplasma gondii IgM: Negative

ASSESSMENT AND PLAN:
Based on the patient's history and test results,

You decide to order a Toxoplasma gondii IgG and IgM serology given her exposure to cats.

## Encounter 2: source 2021-05-15, practice 2021-09-06, outpatient, Hematology & Oncology

Chief complaint: One-week history of easy bruising and petechiae on lower extremities.

ENCOUNTER NOTE
Date: 2021-05-15
Provider: Dr. James Okafor, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
One-week history of easy bruising and petechiae on lower extremities.

HISTORY OF PRESENT ILLNESS:
The patient is a 31-year-old female with a documented history of intravenous drug use and recurrent genitourinary infections who presents with a one-week history of easy bruising and petechiae on her lower extremities. She denies any recent illnesses, medication changes, or family history of bleeding disorders.

PAST MEDICAL HISTORY:
Active Problem List:
- Asymptomatic Actinomyces colonization of genital tract (diagnosed 2020-01-15)
- History of intravenous drug use (diagnosed 2020-05-15)
- History of recurrent chlamydia infection
- History of recurrent urinary tract infections

PAST SURGICAL HISTORY:
- No prior surgeries

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- No family history of bleeding disorders
- Mother with hypertension
- Father with type 2 diabetes

SOCIAL HISTORY:
She is sexually active with multiple partners and does not consistently use barrier protection.

PHYSICAL EXAMINATION:
On examination, you note several large ecchymoses on her thighs and petechiae scattered over her ankles. There is no lymphadenopathy, hepatosplenomegaly, or signs of joint inflammation.

LABORATORY DATA:
Her complete blood count reveals the following:

- Hemoglobin: 14 g/dL
- White blood cell count: 6,000/uL
- Platelet count: 20,000/uL

The peripheral blood smear shows decreased platelets without clumping and no other abnormalities.

ASSESSMENT AND PLAN:
Given the patient's history and presentation,

## Encounter 3: source 2022-09-15, practice 2022-12-29, outpatient, Obstetrics & Gynecology

Chief complaint: Painful genital lesions that started 3 days ago, initially blisters progressing to painful ulcers, with associated pruritus and dysuria.

ENCOUNTER NOTE
Date: 2022-09-15
Provider: Dr. Aisha Patel, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Painful genital lesions that started 3 days ago, initially blisters progressing to painful ulcers, with associated pruritus and dysuria.

HISTORY OF PRESENT ILLNESS:
Patient returns to OB/GYN with aforementioned history, describing genital lesions that started 3 days ago. She reports that the lesions began as small blisters that quickly evolved into painful ulcers. She also experiences pruritus and dysuria. She denies any vaginal discharge or fever.

PAST MEDICAL HISTORY:
Active Problem List:
- Asymptomatic Actinomyces colonization of genital tract (diagnosed 2020-01-15)
- History of intravenous drug use (diagnosed 2020-05-15)
- Idiopathic thrombocytopenic purpura (diagnosed 2021-05-15)
- History of recurrent chlamydia infection
- History of recurrent urinary tract infections

Her past medical history is significant for recurrent urinary tract infections.

PAST SURGICAL HISTORY:
- No prior surgeries

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- No family history of bleeding disorders
- Mother with hypertension
- Father with type 2 diabetes

SOCIAL HISTORY:
She is sexually active with multiple partners and inconsistently uses condoms.

PHYSICAL EXAMINATION:
On physical examination, multiple small, clustered vesicles and shallow ulcers are noted on the labia majora. There is bilateral tender inguinal lymphadenopathy. The remainder of the examination is unremarkable.

LABORATORY DATA:
A sample from the lesions is obtained for laboratory testing.

Laboratory results:
- Viral culture: Pending
- PCR for herpes simplex virus: Positive

## Encounter 4: source 2023-05-15, practice 2023-09-12, outpatient, Obstetrics & Gynecology

Chief complaint: Foul-smelling vaginal discharge for the past week with slight itching.

ENCOUNTER NOTE
Date: 2023-05-15
Provider: Dr. Robert Kim, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Foul-smelling vaginal discharge for the past week with slight itching.

HISTORY OF PRESENT ILLNESS:
The patient with recurrent chlamydia and prior anogenital HSV infection returns with new symptoms. She reports no pain but mentions a slight itching sensation. Her last menstrual period was 2 weeks ago and was normal. She denies any recent antibiotic use or new personal care products.

PAST MEDICAL HISTORY:
Active Problem List:
- Asymptomatic Actinomyces colonization of genital tract (diagnosed 2020-01-15)
- History of intravenous drug use (diagnosed 2020-05-15)
- Idiopathic thrombocytopenic purpura (diagnosed 2021-05-15)
- Anogenital herpes simplex virus infection (diagnosed 2022-09-15)
- History of recurrent chlamydia infection
- History of recurrent urinary tract infections

PAST SURGICAL HISTORY:
- No prior surgeries

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- No family history of bleeding disorders
- Mother with hypertension
- Father with type 2 diabetes

SOCIAL HISTORY:
She is sexually active with multiple partners and inconsistently uses condoms.

PHYSICAL EXAMINATION:
On physical examination, you note a thin, yellow-green vaginal discharge. The cervix appears erythematous with pinpoint areas of bleeding noted after swabbing for a sample. A wet mount microscopy of the vaginal discharge reveals motile, ovoid-shaped organisms. The patient's vital signs and the remainder of the physical examination are within normal limits.

LABORATORY DATA:
Laboratory Values:
- pH of vaginal secretions: 5.5
- Whiff test: Positive
- Saline microscopy: Motile, ovoid-shaped organisms

## Encounter 5: source 2024-05-15, practice 2024-09-05, ed, Emergency Medicine

Chief complaint: Five-day history of painful genital ulcers that progressed from tender papules to pustules, now with associated groin pain and swelling.

ENCOUNTER NOTE
Date: 2024-05-15
Provider: Dr. David Smith, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Five-day history of painful genital ulcers that progressed from tender papules to pustules, now with associated groin pain and swelling.

HISTORY OF PRESENT ILLNESS:
The patient with history of recurrent chlamydia, intravenous drug use, prior anogenital herpes simplex, and trichomoniasis reports that the lesions began as tender papules that progressed to pustules and now have formed ulcers with erythematous bases and well-demarcated borders. She also complains of pain and swelling in her groin.

PAST MEDICAL HISTORY:
Active Problem List:
- Asymptomatic Actinomyces colonization of genital tract (diagnosed 2020-01-15)
- History of intravenous drug use (diagnosed 2020-05-15)
- Idiopathic thrombocytopenic purpura (diagnosed 2021-05-15)
- Anogenital herpes simplex virus infection (diagnosed 2022-09-15)
- Trichomoniasis (diagnosed 2023-05-15)
- History of recurrent chlamydia infection
- History of recurrent urinary tract infections

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
She is not currently on any medications

ALLERGIES:
She has no known drug allergies

FAMILY HISTORY:
- No family history of bleeding disorders
- Mother with hypertension
- Father with type 2 diabetes

SOCIAL HISTORY:
She has a history of multiple sexual partners and admits to trading sex for drugs. The patient denies any recent travel history.

VITAL SIGNS:
Her vital signs are within normal limits.

PHYSICAL EXAMINATION:
On physical examination, you note several 1-2 cm ulcers on the labia majora with purulent exudate. There is bilateral tender inguinal lymphadenopathy.

LABORATORY DATA:
A rapid plasma reagin (RPR) test is negative. A Gram stain of the exudate shows gram-negative rods.

Laboratory Values:
- White blood cell count: 9,000/uL
- Hemoglobin: 13.2 g/dL
- Platelet count: 250,000/uL
- RPR: Non-reactive

## Encounter 6: source 2024-11-15, practice 2025-03-06, inpatient, Obstetrics & Gynecology

Chief complaint: Three-day history of fever, lower abdominal pain, and vaginal discharge with bilateral pain worsening with movement.

ENCOUNTER NOTE
Date: 2024-11-15
Provider: Dr. Fatima Al-Rashid, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
Three-day history of fever, lower abdominal pain, and vaginal discharge with bilateral pain worsening with movement.

HISTORY OF PRESENT ILLNESS:
The patient is a 34-year-old female with a history of intravenous drug use and recurrent chlamydia and other genital tract infections, including recent chancroid, who presents with acute complaints. She reports that the pain is bilateral and worsens with movement.

PAST MEDICAL HISTORY:
Active Problem List:
- Asymptomatic Actinomyces colonization of genital tract (diagnosed 2020-01-15)
- History of intravenous drug use (diagnosed 2020-05-15)
- Idiopathic thrombocytopenic purpura (diagnosed 2021-05-15)
- Anogenital herpes simplex virus infection (diagnosed 2022-09-15)
- Trichomoniasis (diagnosed 2023-05-15)
- Chancroid (diagnosed 2024-05-15)
- History of recurrent chlamydia infection
- History of recurrent urinary tract infections

Her past medical history is significant for two episodes of chlamydia infection, the most recent one treated 6 months ago.

PAST SURGICAL HISTORY:
- No prior surgeries

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- No family history of bleeding disorders
- Mother with hypertension
- Father with type 2 diabetes

SOCIAL HISTORY:
She is sexually active with multiple partners and does not consistently use condoms.

VITAL SIGNS:
On examination, her temperature is 38.7°C (101.7°F), heart rate is 110 beats per minute, and blood pressure is 100/60 mmHg.

PHYSICAL EXAMINATION:
Pelvic examination reveals cervical motion tenderness, bilateral adnexal tenderness, and a purulent cervical discharge.

LABORATORY DATA:
Laboratory tests reveal:

- White blood cell count: 15,000/µL
- Hemoglobin: 12.2 g/dL
- C-reactive protein: 24 mg/L
- Urine pregnancy test: Negative

IMAGING:
A pelvic ultrasound is performed, which shows a complex multiloculated adnexal mass with thick walls and internal debris.

## Encounter 7: source 2025-11-15, practice 2026-03-04, inpatient, Neurology

Chief complaint: Two-day history of progressive leg weakness and numbness below the navel following a flu-like illness two weeks ago.

ENCOUNTER NOTE
Date: 2025-11-15
Provider: Dr. Michael Johnson, MD
Department: Neurology
Visit Type: inpatient

CHIEF COMPLAINT:
Two-day history of progressive leg weakness and numbness below the navel following a flu-like illness two weeks ago.

HISTORY OF PRESENT ILLNESS:
This 35-year-old with recurrent infections presents with leg weakness. She reports having had a flu-like illness 2 weeks ago, which resolved without treatment. She denies any recent trauma or risky behaviors.

PAST MEDICAL HISTORY:
Active Problem List:
- Asymptomatic Actinomyces colonization of genital tract (diagnosed 2020-01-15)
- History of intravenous drug use (diagnosed 2020-05-15)
- Idiopathic thrombocytopenic purpura (diagnosed 2021-05-15)
- Anogenital herpes simplex virus infection (diagnosed 2022-09-15)
- Trichomoniasis (diagnosed 2023-05-15)
- Chancroid (diagnosed 2024-05-15)
- Acute pelvic inflammatory disease with tubo-ovarian abscess (diagnosed 2024-11-15)
- History of recurrent chlamydia infection
- History of recurrent urinary tract infections

PAST SURGICAL HISTORY:
- No prior surgeries

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- No family history of bleeding disorders
- Mother with hypertension
- Father with type 2 diabetes

SOCIAL HISTORY:
Smoking: Current smoker, 5 cigarettes per day. Alcohol: Social, 1-2 drinks per month. Occupation: Unemployed.

PHYSICAL EXAMINATION:
On examination, she has bilateral lower extremity weakness, graded 3/5, and decreased sensation to light touch and pinprick below the T10 dermatome. Reflexes are brisk in the lower extremities, and Babinski sign is present bilaterally.

LABORATORY DATA:
Laboratory studies show a normal complete blood count, erythrocyte sedimentation rate, and comprehensive metabolic panel.

IMAGING:
An urgent MRI of the spine reveals a focal T2 hyperintensity in the thoracic spinal cord.
