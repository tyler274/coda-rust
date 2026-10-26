# synthetic_hospital patient 1752

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 28 at first encounter, sex F, Italian descent (Mediterranean), Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-11-09 (Mon) | outpatient | Hematology & Oncology |
| 1 | 2020-04-15 | 2021-02-08 (Mon) | outpatient | Psychiatry |
| 2 | 2021-01-15 | 2021-11-03 (Wed) | outpatient | Obstetrics & Gynecology |
| 3 | 2022-01-15 | 2022-11-01 (Tue) | outpatient | General Surgery |
| 4 | 2023-01-15 | 2023-11-16 (Thu) | ed | Emergency Medicine |
| 5 | 2023-07-15 | 2024-05-02 (Thu) | ed | Emergency Medicine |
| 6 | 2024-07-15 | 2025-05-15 (Thu) | inpatient | Hematology & Oncology |
| 7 | 2025-01-15 | 2025-10-30 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Software developer",
 "alcohol_use": "Social drinker (1-2 drinks/week); currently abstinent for preconception",
 "family_history": [
  "Father with beta-thalassemia trait",
  "Mother with generalized anxiety disorder",
  "Paternal aunt with fibrocystic breast disease"
 ],
 "race_ethnicity": "Italian descent (Mediterranean)",
 "smoking_status": "Former occasional smoker; quit 3 months ago",
 "home_medications": [
  {
   "dose": "50 mg daily",
   "name": "Sertraline"
  },
  {
   "dose": "1 mg daily",
   "name": "Folic acid"
  },
  {
   "dose": "5 mg orally at bedtime PRN insomnia (limited to 10 tablets/month)",
   "name": "Zolpidem"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Generalized anxiety disorder",
  "Beta-thalassemia minor",
  "Fibrocystic breast changes",
  "Chronic insomnia disorder"
 ],
 "age_at_first_encounter": 28
}
```

Primary diagnoses: ["Complex breast cyst requiring surveillance", "Pancytopenia", "Thalassemia minor", "Insomnia - indication for pharmacologic therapy with Z-drugs", "Hyperventilation Syndrome", "Susceptibility to hepatitis A virus infection", "Benzodiazepine overdose"]
Comorbidities: ["Situational anxiety", "History of recent pneumonia"]

## Encounter 0: source 2020-01-15, practice 2020-11-09, outpatient, Hematology & Oncology

Chief complaint: Six-month history of progressive fatigue, shortness of breath on exertion, and heavy menstrual periods.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of progressive fatigue, shortness of breath on exertion, and heavy menstrual periods.

HISTORY OF PRESENT ILLNESS:
She presents to the clinic with complaints of fatigue and shortness of breath on exertion for the past 6 months. She reports having heavy menstrual periods.

PAST MEDICAL HISTORY:
Active Problem List:
- Generalized anxiety disorder
- Beta-thalassemia minor
- Fibrocystic breast changes
- Chronic insomnia disorder

Her past medical history is significant for mild jaundice during childhood, which resolved without intervention.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Sertraline 50 mg daily
- Folic acid 1 mg daily
- Zolpidem 5 mg orally at bedtime PRN insomnia (limited to 10 tablets/month)

ALLERGIES:
NKDA

FAMILY HISTORY:
Her family history is notable for her mother having a "blood disorder" that was never clearly diagnosed.

SOCIAL HISTORY:
Smoking: Former occasional smoker; quit 3 months ago. Alcohol: Social drinker (1-2 drinks/week); currently abstinent for preconception. Occupation: Software developer.

PHYSICAL EXAMINATION:
On physical examination, she appears pale. No hepatosplenomegaly is noted.

LABORATORY DATA:
Her complete blood count reveals a low erythrocyte count with microcytosis and reactive thrombocytosis. Iron studies are ordered and the results are as follows:

- Serum iron: 60 mcg/dL (normal: 50-170 mcg/dL)
- Total iron-binding capacity (TIBC): 300 mcg/dL (normal: 250-450 mcg/dL)
- Ferritin: 30 ng/mL (normal: 15-200 ng/mL)
- Transferrin saturation: 20% (normal: 15-50%)

## Encounter 1: source 2020-04-15, practice 2021-02-08, outpatient, Psychiatry

Chief complaint: Difficulty falling asleep and frequent nighttime awakenings causing daytime fatigue and impaired work concentration.

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Difficulty falling asleep and frequent nighttime awakenings causing daytime fatigue and impaired work concentration.

HISTORY OF PRESENT ILLNESS:
This patient with beta-thalassemia minor presents with insomnia following January evaluation for fatigue. She reports feeling tired during the day and trouble concentrating at work. She denies snoring or periods of apnea reported by her partner.

PAST MEDICAL HISTORY:
Active Problem List:
- Thalassemia minor (diagnosed 2020-01-15)
- Generalized anxiety disorder
- Beta-thalassemia minor
- Fibrocystic breast changes
- Chronic insomnia disorder

The patient has a history of anxiety.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
She takes she takes sertraline.

Home Medications:
- Folic acid 1 mg daily
- Zolpidem 5 mg orally at bedtime PRN insomnia (limited to 10 tablets/month)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with beta-thalassemia trait
- Mother with generalized anxiety disorder
- Paternal aunt with fibrocystic breast disease

SOCIAL HISTORY:
She works as a software developer and often works late into the night on her computer. She drinks two cups of coffee in the morning and occasionally has a glass of wine with dinner. She smokes a few cigarettes on the weekends.

PHYSICAL EXAMINATION:
Physical examination is unremarkable. Her BMI is 23 kg/m^2.

LABORATORY DATA:
Thyroid-stimulating hormone (TSH) and complete blood count (CBC) are within normal limits.

## Encounter 2: source 2021-01-15, practice 2021-11-03, outpatient, Obstetrics & Gynecology

Chief complaint: Preconception counseling regarding genetic risks and pregnancy planning.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Sarah Chen, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Preconception counseling regarding genetic risks and pregnancy planning.

PAST MEDICAL HISTORY:
Active Problem List:
- Thalassemia minor (diagnosed 2020-01-15)
- Insomnia - indication for pharmacologic therapy with Z-drugs (diagnosed 2020-04-15)
- Generalized anxiety disorder
- Beta-thalassemia minor
- Fibrocystic breast changes
- Chronic insomnia disorder

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Sertraline 50 mg daily
- Folic acid 1 mg daily
- Zolpidem 5 mg orally at bedtime PRN insomnia (limited to 10 tablets/month)

ALLERGIES:
NKDA

FAMILY HISTORY:
She reports a family history of anemia

SOCIAL HISTORY:
She does not smoke or drink alcohol. Her partner is of Southeast Asian descent and has no known medical conditions.

REVIEW OF SYSTEMS:
She denies any symptoms of fatigue, dyspnea, or palpitations.

VITAL SIGNS:
She has a normal BMI

PHYSICAL EXAMINATION:
Her physical examination is unremarkable.

LABORATORY DATA:
Laboratory studies reveal:

- Hemoglobin: 12.1 g/dL (normal: 12-16 g/dL)
- Mean corpuscular volume (MCV): 72 fL (normal: 80-100 fL)
- Iron: 110 µg/dL (normal: 50-170 µg/dL)
- Total iron-binding capacity: 300 µg/dL (normal: 250-450 µg/dL)
- Ferritin: 30 ng/mL (normal: 15-200 ng/mL)

Hemoglobin electrophoresis is performed and shows an abnormal hemoglobin pattern.

## Encounter 3: source 2022-01-15, practice 2022-11-01, outpatient, General Surgery

Chief complaint: New tender breast lump discovered on self-examination two weeks ago with cyclical lump history.

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Emily Watson, MD
Department: General Surgery
Visit Type: outpatient

CHIEF COMPLAINT:
New tender breast lump discovered on self-examination two weeks ago with cyclical lump history.

HISTORY OF PRESENT ILLNESS:
The patient is a 30-year-old female with established generalized anxiety disorder, beta-thalassemia minor, fibrocystic breast changes, and chronic insomnia disorder who presents with a new breast concern. She returns to the outpatient setting approximately one year following her preconception counseling encounter, reporting symptoms in the context of her known fibrocystic breast changes. She reports that she first noticed the lump approximately 2 weeks ago, and it has gradually become more tender over the past several days. She also indicates that she frequently develops smaller breast lumps that seem to appear and disappear in relation to her menstrual cycle. She notes that in the past, these lumps have caused discomfort before her period and then diminished or resolved once menstruation began. She recalls that during her last two cycles she noticed similar nodules, which worried her initially, but they went away as soon as her period started. Her menses occur at regular 28-day intervals. Her last menstrual period was approximately 3 weeks ago. She describes her breasts as feeling "lumpy and achy" every month, especially in the days leading up to her period. She often needs to wear a more supportive bra during that time to help with discomfort. She states that she has become increasingly anxious because this new lump feels larger than the ones she has felt before. She indicates that she has read online that breast pain and nodules can sometimes be signs of cancer, which has caused her significant worry.

PAST MEDICAL HISTORY:
Active Problem List:
- Thalassemia minor (diagnosed 2020-01-15)
- Insomnia - indication for pharmacologic therapy with Z-drugs (diagnosed 2020-04-15)
- Generalized anxiety disorder
- Beta-thalassemia minor
- Fibrocystic breast changes
- Chronic insomnia disorder

She has never had breast trauma or prior breast surgery.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
She does not take medications

Home Medications:
- Sertraline 50 mg daily
- Folic acid 1 mg daily
- Zolpidem 5 mg orally at bedtime PRN insomnia (limited to 10 tablets/month)

ALLERGIES:
NKDA

FAMILY HISTORY:
She has no family history of breast or ovarian cancer.

SOCIAL HISTORY:
She does not smoke, drinks alcohol only occasionally, and exercises several times a week. She eats a balanced diet, has a normal body mass index.

REVIEW OF SYSTEMS:
She denies nipple discharge, skin dimpling, fever, or systemic symptoms.

VITAL SIGNS:
On examination, her vital signs are normal.

PHYSICAL EXAMINATION:
There is no lymphadenopathy in the cervical, supraclavicular, or axillary regions. The breasts are symmetric with no overlying skin changes such as dimpling, erythema, or peau d’orange. Palpation of the right breast reveals multiple small, smooth, soft, mobile nodules clustered in the upper outer quadrant, the largest measuring about 2 cm. The nodules are mildly tender to palpation but not associated with overlying warmth. The left breast also contains several smaller, mobile nodules with similar characteristics. Both breasts are diffusely tender to deep palpation, particularly in the upper outer quadrants. There is no nipple retraction or abnormal discharge. The masses are mobile, not fixed, and there is no evidence of overlying erythema, warmth, or fluctuation. No discrete dominant mass is palpated, but the tissue feels more nodular than expected for her age.

## Encounter 4: source 2023-01-15, practice 2023-11-16, ed, Emergency Medicine

Chief complaint: Acute onset progressive shortness of breath, palpitations, and perioral and extremity tingling.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. James Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset progressive shortness of breath, palpitations, and perioral and extremity tingling.

HISTORY OF PRESENT ILLNESS:
The patient is a 31-year-old female with generalized anxiety disorder and beta-thalassemia minor. She presents with a 2-day history of progressive shortness of breath, palpitations, and tingling in her extremities.

PAST MEDICAL HISTORY:
Active Problem List:
- Thalassemia minor (diagnosed 2020-01-15)
- Insomnia - indication for pharmacologic therapy with Z-drugs (diagnosed 2020-04-15)
- Complex breast cyst requiring surveillance (diagnosed 2022-01-15)
- Generalized anxiety disorder
- Beta-thalassemia minor
- Fibrocystic breast changes
- Chronic insomnia disorder

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Sertraline 50 mg daily
- Folic acid 1 mg daily
- Zolpidem 5 mg orally at bedtime PRN insomnia (limited to 10 tablets/month)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with beta-thalassemia trait
- Mother with generalized anxiety disorder
- Paternal aunt with fibrocystic breast disease

SOCIAL HISTORY:
Smoking: Former occasional smoker; quit 3 months ago. Alcohol: Social drinker (1-2 drinks/week); currently abstinent for preconception. Occupation: Software developer.

VITAL SIGNS:
Her respiratory rate is 28 breaths per minute, heart rate is 110 beats per minute, blood pressure is 130/85 mmHg, and oxygen saturation is 99% on room air

PHYSICAL EXAMINATION:
On examination, she is alert and oriented but appears anxious. Lung auscultation reveals no abnormalities.

LABORATORY DATA:
Arterial blood gas analysis shows a pH of 7.49, PaCO2 of 30 mmHg, and HCO3- of 22 mEq/L. Urinalysis is notable for a pH of 7.5.

## Encounter 5: source 2023-07-15, practice 2024-05-02, ed, Emergency Medicine

Chief complaint: Unprovoked raccoon bite to lower extremity while hiking with concern for rabies and zoonotic disease exposure.

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Unprovoked raccoon bite to lower extremity while hiking with concern for rabies and zoonotic disease exposure.

HISTORY OF PRESENT ILLNESS:
The patient is a 31-year-old female with beta-thalassemia minor, generalized anxiety disorder, chronic insomnia, and fibrocystic breast changes who presents to the Emergency Department following a raccoon bite sustained while hiking in a wooded area. She reports that the raccoon appeared healthy and unprovoked before the attack. The patient is anxious and concerned about the risk of rabies. She is unsure if the raccoon can be captured for testing.

PAST MEDICAL HISTORY:
Active Problem List:
- Thalassemia minor (diagnosed 2020-01-15)
- Insomnia - indication for pharmacologic therapy with Z-drugs (diagnosed 2020-04-15)
- Complex breast cyst requiring surveillance (diagnosed 2022-01-15)
- Hyperventilation Syndrome (diagnosed 2023-01-15)
- Generalized anxiety disorder
- Beta-thalassemia minor
- Fibrocystic breast changes
- Chronic insomnia disorder

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Sertraline 50 mg daily
- Folic acid 1 mg daily
- Zolpidem 5 mg orally at bedtime PRN insomnia (limited to 10 tablets/month)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with beta-thalassemia trait
- Mother with generalized anxiety disorder
- Paternal aunt with fibrocystic breast disease

SOCIAL HISTORY:
Smoking: Former occasional smoker; quit 3 months ago. Alcohol: Social drinker (1-2 drinks/week); currently abstinent for preconception. Occupation: Software developer.

VITAL SIGNS:
Her vital signs are stable, and she has no signs of systemic infection.

PHYSICAL EXAMINATION:
On examination, there is a 3 cm laceration on her left calf with mild erythema and edema. There is no evidence of neurovascular compromise, and the wound is not actively bleeding.

## Encounter 6: source 2024-07-15, practice 2025-05-15, inpatient, Hematology & Oncology

Chief complaint: Two-month history of worsening fatigue, frequent epistaxis, and fever despite recent antibiotic course.

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Sarah Chen, MD
Department: Hematology & Oncology
Visit Type: inpatient

CHIEF COMPLAINT:
Two-month history of worsening fatigue, frequent epistaxis, and fever despite recent antibiotic course.

HISTORY OF PRESENT ILLNESS:
The patient with beta-thalassemia minor followed since 2020 presents with a 2-month history of increasing fatigue, frequent nosebleeds, and a recent episode of fever. She reports a recent course of antibiotics for a urinary tract infection.

PAST MEDICAL HISTORY:
Active Problem List:
- Thalassemia minor (diagnosed 2020-01-15)
- Insomnia - indication for pharmacologic therapy with Z-drugs (diagnosed 2020-04-15)
- Complex breast cyst requiring surveillance (diagnosed 2022-01-15)
- Hyperventilation Syndrome (diagnosed 2023-01-15)
- Susceptibility to hepatitis A virus infection (diagnosed 2023-07-15)
- Generalized anxiety disorder
- Beta-thalassemia minor
- Fibrocystic breast changes
- Chronic insomnia disorder

She denies any significant past medical history.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Sertraline 50 mg daily
- Folic acid 1 mg daily
- Zolpidem 5 mg orally at bedtime PRN insomnia (limited to 10 tablets/month)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with beta-thalassemia trait
- Mother with generalized anxiety disorder
- Paternal aunt with fibrocystic breast disease

SOCIAL HISTORY:
Smoking: Former occasional smoker; quit 3 months ago. Alcohol: Social drinker (1-2 drinks/week); currently abstinent for preconception. Occupation: Software developer.

PHYSICAL EXAMINATION:
On examination, she appears pale and has several ecchymoses on her arms and legs. There is no lymphadenopathy or splenomegaly.

LABORATORY DATA:
- Hemoglobin: 7.8 g/dL (Normal: 12-16 g/dL)
- White blood cell count: 2.1 x 10^9/L (Normal: 4.5-11 x 10^9/L)
- Platelets: 20 x 10^9/L (Normal: 150-400 x 10^9/L)
- Reticulocyte count: 0.5% (Normal: 0.5-2.5%)

PATHOLOGY:
- Bone marrow biopsy: Hypocellular with normal cell morphology

## Encounter 7: source 2025-01-15, practice 2025-10-30, ed, Emergency Medicine

Chief complaint: Found increasingly drowsy and difficult to arouse by friends after period of work-related stress.

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Found increasingly drowsy and difficult to arouse by friends after period of work-related stress.

HISTORY OF PRESENT ILLNESS:
The patient is a 33-year-old female with generalized anxiety disorder, beta-thalassemia minor, and chronic insomnia who presents to the emergency department following a recent admission for pancytopenia. She is brought by her friends after they found her to be increasingly drowsy and difficult to arouse. They report that she has been under significant stress at work and has a history of anxiety for which she occasionally takes medication, but they are unsure of the name.

PAST MEDICAL HISTORY:
Active Problem List:
- Thalassemia minor (diagnosed 2020-01-15)
- Insomnia - indication for pharmacologic therapy with Z-drugs (diagnosed 2020-04-15)
- Complex breast cyst requiring surveillance (diagnosed 2022-01-15)
- Hyperventilation Syndrome (diagnosed 2023-01-15)
- Susceptibility to hepatitis A virus infection (diagnosed 2023-07-15)
- Pancytopenia (diagnosed 2024-07-15)
- Generalized anxiety disorder
- Beta-thalassemia minor
- Fibrocystic breast changes
- Chronic insomnia disorder

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Sertraline 50 mg daily
- Folic acid 1 mg daily
- Zolpidem 5 mg orally at bedtime PRN insomnia (limited to 10 tablets/month)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with beta-thalassemia trait
- Mother with generalized anxiety disorder
- Paternal aunt with fibrocystic breast disease

SOCIAL HISTORY:
Smoking: Former occasional smoker; quit 3 months ago. Alcohol: Social drinker (1-2 drinks/week); currently abstinent for preconception. Occupation: Software developer.

VITAL SIGNS:
Her vital signs are as follows: blood pressure 130/85 mmHg, heart rate 88 beats per minute, respiratory rate 14 breaths per minute, and oxygen saturation 98% on room air.

PHYSICAL EXAMINATION:
On examination, the patient has slurred speech, an unsteady gait, and seems drowsy but is arousable. Pupils are 4 mm and reactive to light. There is no evidence of trauma.

LABORATORY DATA:
A urine toxicology screen is positive for benzodiazepines. No other substances are detected.
