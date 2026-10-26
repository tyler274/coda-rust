# synthetic_hospital patient 1716

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex F, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-12-27 (Mon) | outpatient | Pulmonology |
| 1 | 2021-07-15 | 2023-06-30 (Fri) | outpatient | Gastroenterology |
| 2 | 2022-01-15 | 2023-12-29 (Fri) | ed | Emergency Medicine |
| 3 | 2023-04-15 | 2025-03-29 (Sat) | ed | Emergency Medicine |
| 4 | 2023-07-15 | 2025-06-19 (Thu) | outpatient | Obstetrics & Gynecology |
| 5 | 2023-08-15 | 2025-08-04 (Mon) | outpatient | Obstetrics & Gynecology |
| 6 | 2024-01-15 | 2025-12-28 (Sun) | ed | Emergency Medicine |
| 7 | 2024-07-15 | 2026-07-01 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Elementary school teacher",
 "alcohol_use": "Social (1-2 drinks per month)",
 "family_history": [
  "Father: Hereditary hemorrhagic telangiectasia",
  "Mother: Systemic lupus erythematosus",
  "Maternal aunt: Rheumatoid arthritis"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1.2 g PO three times daily",
   "name": "Mesalamine"
  },
  {
   "dose": "200 mg PO daily",
   "name": "Hydroxychloroquine"
  },
  {
   "dose": "10 mg PO daily (chronic low-dose with intermittent tapers for flares)",
   "name": "Prednisone"
  },
  {
   "dose": "90 mcg inhaler, 2 puffs every 4-6 hours PRN for wheezing",
   "name": "Albuterol"
  },
  {
   "dose": "325 mg PO daily",
   "name": "Ferrous sulfate"
  }
 ],
 "surgical_history": [
  "Dilatation and curettage for spontaneous abortion at 10 weeks gestation (age 32)",
  "Emergency cesarean delivery for fetal distress (age 32)",
  "Spontaneous vaginal delivery (age 32)"
 ],
 "chronic_conditions": [
  "Crohn's disease (diagnosed age 24)",
  "Systemic lupus erythematosus",
  "Hereditary hemorrhagic telangiectasia (Osler-Weber-Rendu syndrome)",
  "Mild intermittent asthma"
 ],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Choriocarcinoma", "Hereditary Hemorrhagic Telangiectasia (Osler-Weber-Rendu Syndrome)", "Rectovaginal fistula", "Systemic Lupus Erythematosus with Central Nervous System Involvement (Neuropsychiatric SLE)", "Superficial surgical site infection (cellulitis) following cesarean delivery", "Herpetic whitlow or viral lymphangitis", "Breast engorgement (postpartum)"]
Comorbidities: ["Exclusive breastfeeding", "Anemia"]

## Encounter 0: source 2020-01-15, practice 2021-12-27, outpatient, Pulmonology

Chief complaint: Recurrent epistaxis requiring increased medical attention over the past year

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Recurrent epistaxis requiring increased medical attention over the past year

HISTORY OF PRESENT ILLNESS:
History of recurrent nosebleeds since childhood. She reports that the nosebleeds have increased in frequency over the past year, often occurring several times a week and sometimes requiring medical attention to control. She also mentions occasional blood-tinged sputum but denies any cough or shortness of breath.

PAST MEDICAL HISTORY:
Active Problem List:
- Crohn's disease (diagnosed age 24)
- Systemic lupus erythematosus
- Hereditary hemorrhagic telangiectasia (Osler-Weber-Rendu syndrome)
- Mild intermittent asthma

PAST SURGICAL HISTORY:
- Dilatation and curettage for spontaneous abortion at 10 weeks gestation (age 32)
- Emergency cesarean delivery for fetal distress (age 32)
- Spontaneous vaginal delivery (age 32)

MEDICATIONS:
Home Medications:
- Mesalamine 1.2 g PO three times daily
- Hydroxychloroquine 200 mg PO daily
- Prednisone 10 mg PO daily (chronic low-dose with intermittent tapers for flares)
- Albuterol 90 mcg inhaler, 2 puffs every 4-6 hours PRN for wheezing
- Ferrous sulfate 325 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
Her family history is significant for similar symptoms in her father and a cousin.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
On examination, you note several small red-to-purple lesions on her lips and oral mucosa. Auscultation of the chest reveals no abnormalities.

LABORATORY DATA:
Hemoglobin: 11.2 g/dL (Normal: 12-16 g/dL)
Platelet count: 250,000/mm³ (Normal: 150,000-400,000/mm³)
International Normalized Ratio (INR): 1.0 (Normal: 0.8-1.2)
Partial thromboplastin time (PTT): 30 seconds (Normal: 25-35 seconds)

## Encounter 1: source 2021-07-15, practice 2023-06-30, outpatient, Gastroenterology

Chief complaint: Six-month history of intermittent abdominal pain, diarrhea, and recent onset of malodorous vaginal discharge with passage of flatus per vagina

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of intermittent abdominal pain, diarrhea, and recent onset of malodorous vaginal discharge with passage of flatus per vagina

HISTORY OF PRESENT ILLNESS:
The patient with Crohn's disease, SLE, and HHT returns following January 2020 Pulmonology encounter with a 6-month history of intermittent abdominal pain, diarrhea, and unintentional weight loss. She also describes recent onset of malodorous, tan vaginal discharge with occasional passage of flatus per vagina.

PAST MEDICAL HISTORY:
Active Problem List:
- Hereditary Hemorrhagic Telangiectasia (Osler-Weber-Rendu Syndrome) (diagnosed 2020-01-15)
- Crohn's disease (diagnosed age 24)
- Systemic lupus erythematosus
- Mild intermittent asthma

Crohn's disease diagnosed 8 years ago, which has been managed with mesalamine and intermittent corticosteroids.

PAST SURGICAL HISTORY:
- Dilatation and curettage for spontaneous abortion at 10 weeks gestation (age 32)
- Emergency cesarean delivery for fetal distress (age 32)
- Spontaneous vaginal delivery (age 32)

MEDICATIONS:
Home Medications:
- Mesalamine 1.2 g PO three times daily
- Hydroxychloroquine 200 mg PO daily
- Prednisone 10 mg PO daily (chronic low-dose with intermittent tapers for flares)
- Albuterol 90 mcg inhaler, 2 puffs every 4-6 hours PRN for wheezing
- Ferrous sulfate 325 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hereditary hemorrhagic telangiectasia
- Mother: Systemic lupus erythematosus
- Maternal aunt: Rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
On physical examination, she is afebrile, and her abdominal exam reveals tenderness in the right lower quadrant without rebound or guarding. Pelvic examination shows a small lesion on the posterior vaginal wall.

LABORATORY DATA:
- Hemoglobin: 12.1 g/dL
- White blood cell count: 9,000/uL
- C-reactive protein: 24 mg/L (normal <5 mg/L)
- Stool cultures: No pathogenic bacteria, ova, or parasites detected

## Encounter 2: source 2022-01-15, practice 2023-12-29, ed, Emergency Medicine

Chief complaint: Generalized tonic-clonic seizure with several months of preceding headaches and photosensitive malar rash

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Emily Watson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Generalized tonic-clonic seizure with several months of preceding headaches and photosensitive malar rash

HISTORY OF PRESENT ILLNESS:
The patient with systemic lupus erythematosus, Crohn's disease, and recent gastroenterology follow-up presents to the emergency department. Her boyfriend reports intermittent headaches, fatigue, and a malar rash that worsens with sun exposure over the past few months.

PAST MEDICAL HISTORY:
Active Problem List:
- Hereditary Hemorrhagic Telangiectasia (Osler-Weber-Rendu Syndrome) (diagnosed 2020-01-15)
- Rectovaginal fistula (diagnosed 2021-07-15)
- Crohn's disease (diagnosed age 24)
- Systemic lupus erythematosus
- Mild intermittent asthma

Her medical history is significant for mild intermittent asthma and one episode of unexplained thrombocytopenia last year.

PAST SURGICAL HISTORY:
- Dilatation and curettage for spontaneous abortion at 10 weeks gestation (age 32)
- Emergency cesarean delivery for fetal distress (age 32)
- Spontaneous vaginal delivery (age 32)

MEDICATIONS:
Home Medications:
- Mesalamine 1.2 g PO three times daily
- Hydroxychloroquine 200 mg PO daily
- Prednisone 10 mg PO daily (chronic low-dose with intermittent tapers for flares)
- Albuterol 90 mcg inhaler, 2 puffs every 4-6 hours PRN for wheezing
- Ferrous sulfate 325 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hereditary hemorrhagic telangiectasia
- Mother: Systemic lupus erythematosus
- Maternal aunt: Rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

VITAL SIGNS:
Her temperature is 37.8°C (100°F), blood pressure is 135/85 mmHg, and heart rate is 88/min.

PHYSICAL EXAMINATION:
On examination, she is postictal but arousable. A malar rash is noted across her cheeks and bridge of her nose. No oral ulcers are observed. Neurological examination is nonfocal.

LABORATORY DATA:
Laboratory studies are as follows:

- Hemoglobin: 11.2 g/dL
- Leukocyte count: 3,800/uL
- Platelet count: 95,000/uL
- Serum creatinine: 1.1 mg/dL
- Urinalysis: 2+ protein, 5-10 red blood cells/high power field
- Antinuclear antibodies (ANA): Positive
- Anti-double-stranded DNA antibodies (anti-dsDNA): Positive
- Complement levels (C3, C4): Low

## Encounter 3: source 2023-04-15, practice 2025-03-29, ed, Emergency Medicine

Chief complaint: Shortness of breath and heavy intermittent vaginal bleeding three months after spontaneous abortion

ENCOUNTER NOTE
Date: 2023-04-15
Provider: Dr. James Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Shortness of breath and heavy intermittent vaginal bleeding three months after spontaneous abortion

HISTORY OF PRESENT ILLNESS:
The patient is a 35-year-old female with Crohn's disease, systemic lupus erythematosus, and hereditary hemorrhagic telangiectasia, previously evaluated for rectovaginal fistula and neuropsychiatric SLE. She presents to the emergency department with shortness of breath and intermittent vaginal bleeding for the past two weeks. She reports that the bleeding is irregular and heavier than her normal menstrual periods. She had a spontaneous abortion 3 months ago at 10 weeks gestation, which was followed by a dilation and curettage procedure.

PAST MEDICAL HISTORY:
Active Problem List:
- Hereditary Hemorrhagic Telangiectasia (Osler-Weber-Rendu Syndrome) (diagnosed 2020-01-15)
- Rectovaginal fistula (diagnosed 2021-07-15)
- Systemic Lupus Erythematosus with Central Nervous System Involvement (Neuropsychiatric SLE) (diagnosed 2022-01-15)
- Crohn's disease (diagnosed age 24)
- Systemic lupus erythematosus
- Mild intermittent asthma

PAST SURGICAL HISTORY:
- Dilatation and curettage for spontaneous abortion at 10 weeks gestation (age 32)
- Emergency cesarean delivery for fetal distress (age 32)
- Spontaneous vaginal delivery (age 32)

MEDICATIONS:
Home Medications:
- Mesalamine 1.2 g PO three times daily
- Hydroxychloroquine 200 mg PO daily
- Prednisone 10 mg PO daily (chronic low-dose with intermittent tapers for flares)
- Albuterol 90 mcg inhaler, 2 puffs every 4-6 hours PRN for wheezing
- Ferrous sulfate 325 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hereditary hemorrhagic telangiectasia
- Mother: Systemic lupus erythematosus
- Maternal aunt: Rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

VITAL SIGNS:
Her vital signs are as follows: blood pressure 130/85 mmHg, heart rate 102 beats per minute, respiratory rate 22 breaths per minute, and oxygen saturation 94% on room air.

PHYSICAL EXAMINATION:
On physical examination, her uterus is palpably enlarged. Auscultation of the lungs reveals diffuse rales.

LABORATORY DATA:
Laboratory tests reveal the following:
- Hemoglobin: 11.2 g/dL
- White blood cell count: 7,800 cells/mm^3
- Platelets: 250,000 cells/mm^3
- β-hCG: 225,000 mIU/mL

IMAGING:
A chest x-ray shows multiple bilateral pulmonary infiltrates.

## Encounter 4: source 2023-07-15, practice 2025-06-19, outpatient, Obstetrics & Gynecology

Chief complaint: Bilateral breast pain and swelling four days after spontaneous vaginal delivery with infant feeding difficulties

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Maria Gonzalez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Bilateral breast pain and swelling four days after spontaneous vaginal delivery with infant feeding difficulties

HISTORY OF PRESENT ILLNESS:
This 35-year-old patient with Crohn's disease, systemic lupus erythematosus, hereditary hemorrhagic telangiectasia, and recent history of choriocarcinoma presents to the obstetrics clinic four days following spontaneous vaginal delivery with complaints of bilateral breast pain and swelling. She reports that she has been trying to breastfeed her newborn, but the baby seems to have difficulty latching on, and she is not sure if the baby is getting enough milk. She denies any fever, nipple discharge, or red streaks on her breasts. She is eager to continue breastfeeding and asks for advice on how to alleviate her discomfort.

PAST MEDICAL HISTORY:
Active Problem List:
- Hereditary Hemorrhagic Telangiectasia (Osler-Weber-Rendu Syndrome) (diagnosed 2020-01-15)
- Rectovaginal fistula (diagnosed 2021-07-15)
- Systemic Lupus Erythematosus with Central Nervous System Involvement (Neuropsychiatric SLE) (diagnosed 2022-01-15)
- Choriocarcinoma (diagnosed 2023-04-15)
- Crohn's disease (diagnosed age 24)
- Systemic lupus erythematosus
- Mild intermittent asthma

PAST SURGICAL HISTORY:
- Dilatation and curettage for spontaneous abortion at 10 weeks gestation (age 32)
- Emergency cesarean delivery for fetal distress (age 32)
- Spontaneous vaginal delivery (age 32)

MEDICATIONS:
Home Medications:
- Mesalamine 1.2 g PO three times daily
- Hydroxychloroquine 200 mg PO daily
- Prednisone 10 mg PO daily (chronic low-dose with intermittent tapers for flares)
- Albuterol 90 mcg inhaler, 2 puffs every 4-6 hours PRN for wheezing
- Ferrous sulfate 325 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hereditary hemorrhagic telangiectasia
- Mother: Systemic lupus erythematosus
- Maternal aunt: Rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

VITAL SIGNS:
Her vital signs are within normal limits.

PHYSICAL EXAMINATION:
On examination, her breasts are symmetrically enlarged, firm, and tender to palpation. There is no skin discoloration, and the nipples are intact without cracks or fissures. Axillary lymph nodes are non-palpable.

## Encounter 5: source 2023-08-15, practice 2025-08-04, outpatient, Obstetrics & Gynecology

Chief complaint: Painful vesicular lesions on right breast developing during exclusive breastfeeding four weeks postpartum

ENCOUNTER NOTE
Date: 2023-08-15
Provider: Dr. David Kim, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Painful vesicular lesions on right breast developing during exclusive breastfeeding four weeks postpartum

HISTORY OF PRESENT ILLNESS:
The patient is a 32-year-old woman with Crohn's disease, systemic lupus erythematosus, hereditary hemorrhagic telangiectasia, and mild intermittent asthma who returns for a routine postpartum visit with her 4-week-old infant four weeks following spontaneous vaginal delivery and recent choriocarcinoma. She presents following her recent evaluation for postpartum breast engorgement. She reports that she has been exclusively breastfeeding and is concerned because she has developed painful vesicular lesions on her right breast over the past two days. She denies any fever, chills, or flu-like symptoms. The infant appears well and is gaining weight appropriately. She has not received any recent vaccinations or been exposed to any sick contacts.

PAST MEDICAL HISTORY:
Active Problem List:
- Hereditary Hemorrhagic Telangiectasia (Osler-Weber-Rendu Syndrome) (diagnosed 2020-01-15)
- Rectovaginal fistula (diagnosed 2021-07-15)
- Systemic Lupus Erythematosus with Central Nervous System Involvement (Neuropsychiatric SLE) (diagnosed 2022-01-15)
- Choriocarcinoma (diagnosed 2023-04-15)
- Breast engorgement (postpartum) (diagnosed 2023-07-15)
- Crohn's disease (diagnosed age 24)
- Systemic lupus erythematosus
- Mild intermittent asthma

PAST SURGICAL HISTORY:
- Dilatation and curettage for spontaneous abortion at 10 weeks gestation (age 32)
- Emergency cesarean delivery for fetal distress (age 32)
- Spontaneous vaginal delivery (age 32)

MEDICATIONS:
Home Medications:
- Mesalamine 1.2 g PO three times daily
- Hydroxychloroquine 200 mg PO daily
- Prednisone 10 mg PO daily (chronic low-dose with intermittent tapers for flares)
- Albuterol 90 mcg inhaler, 2 puffs every 4-6 hours PRN for wheezing
- Ferrous sulfate 325 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hereditary hemorrhagic telangiectasia
- Mother: Systemic lupus erythematosus
- Maternal aunt: Rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

VITAL SIGNS:
Her vital signs are within normal limits.

PHYSICAL EXAMINATION:
On examination, the mother has multiple clustered vesicles on an erythematous base localized to the areola of the right breast. There are no similar lesions elsewhere on her body

LABORATORY DATA:
HIV antibody test: Negative
Complete blood count: Within normal limits
Rapid plasma reagin (RPR): Non-reactive

## Encounter 6: source 2024-01-15, practice 2025-12-28, ed, Emergency Medicine

Chief complaint: Fever of 38.5°C with incisional pain and erythema on postoperative day three following emergency cesarean delivery

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Lisa Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fever of 38.5°C with incisional pain and erythema on postoperative day three following emergency cesarean delivery

HISTORY OF PRESENT ILLNESS:
The patient with Crohn's disease and systemic lupus erythematosus presents three days after emergency cesarean delivery for fetal distress with pain and redness around her surgical incision.

PAST MEDICAL HISTORY:
Active Problem List:
- Hereditary Hemorrhagic Telangiectasia (Osler-Weber-Rendu Syndrome) (diagnosed 2020-01-15)
- Rectovaginal fistula (diagnosed 2021-07-15)
- Systemic Lupus Erythematosus with Central Nervous System Involvement (Neuropsychiatric SLE) (diagnosed 2022-01-15)
- Choriocarcinoma (diagnosed 2023-04-15)
- Breast engorgement (postpartum) (diagnosed 2023-07-15)
- Herpetic whitlow or viral lymphangitis (diagnosed 2023-08-15)
- Crohn's disease (diagnosed age 24)
- Systemic lupus erythematosus
- Mild intermittent asthma

PAST SURGICAL HISTORY:
- Dilatation and curettage for spontaneous abortion at 10 weeks gestation (age 32)
- Emergency cesarean delivery for fetal distress (age 32)
- Spontaneous vaginal delivery (age 32)

MEDICATIONS:
Home Medications:
- Mesalamine 1.2 g PO three times daily
- Hydroxychloroquine 200 mg PO daily
- Prednisone 10 mg PO daily (chronic low-dose with intermittent tapers for flares)
- Albuterol 90 mcg inhaler, 2 puffs every 4-6 hours PRN for wheezing
- Ferrous sulfate 325 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hereditary hemorrhagic telangiectasia
- Mother: Systemic lupus erythematosus
- Maternal aunt: Rheumatoid arthritis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

VITAL SIGNS:
Her BMI is 34 kg/m².

PHYSICAL EXAMINATION:
On examination, the lower segment cesarean section incision is erythematous and indurated with mild purulent discharge. There is no foul odor.

LABORATORY DATA:
Her white blood cell count is 14,000/µL, and her C-reactive protein is elevated.

## Encounter 7: source 2024-07-15, practice 2026-07-01, ed, Emergency Medicine

Chief complaint: Sudden-onset right-sided weakness and aphasia with preceding chest discomfort and lightheadedness six months postpartum

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Robert Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden-onset right-sided weakness and aphasia with preceding chest discomfort and lightheadedness six months postpartum

HISTORY OF PRESENT ILLNESS:
The patient with systemic lupus erythematosus and prior choriocarcinoma presents six months postpartum. She has since felt generally well, though she recently noted episodes of lightheadedness and intermittent dull chest discomfort. She has not had follow-up care since delivery.

PAST MEDICAL HISTORY:
Active Problem List:
- Hereditary Hemorrhagic Telangiectasia (Osler-Weber-Rendu Syndrome) (diagnosed 2020-01-15)
- Rectovaginal fistula (diagnosed 2021-07-15)
- Systemic Lupus Erythematosus with Central Nervous System Involvement (Neuropsychiatric SLE) (diagnosed 2022-01-15)
- Choriocarcinoma (diagnosed 2023-04-15)
- Breast engorgement (postpartum) (diagnosed 2023-07-15)
- Herpetic whitlow or viral lymphangitis (diagnosed 2023-08-15)
- Superficial surgical site infection (cellulitis) following cesarean delivery (diagnosed 2024-01-15)
- Crohn's disease (diagnosed age 24)
- Systemic lupus erythematosus
- Mild intermittent asthma

She has no known history of hypertension, diabetes, or stroke. Her only recent medical event was a spontaneous vaginal delivery 3 months ago.

PAST SURGICAL HISTORY:
- Dilatation and curettage for spontaneous abortion at 10 weeks gestation (age 32)
- Emergency cesarean delivery for fetal distress (age 32)
- Spontaneous vaginal delivery (age 32)

MEDICATIONS:
Home Medications:
- Mesalamine 1.2 g PO three times daily
- Hydroxychloroquine 200 mg PO daily
- Prednisone 10 mg PO daily (chronic low-dose with intermittent tapers for flares)
- Albuterol 90 mcg inhaler, 2 puffs every 4-6 hours PRN for wheezing
- Ferrous sulfate 325 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hereditary hemorrhagic telangiectasia
- Mother: Systemic lupus erythematosus
- Maternal aunt: Rheumatoid arthritis

SOCIAL HISTORY:
She denies illicit drug use, recent travel, or trauma.

VITAL SIGNS:
Temperature is 37.1°C (98.8°F), blood pressure is 118/76 mmHg, and pulse is 92/min.

PHYSICAL EXAMINATION:
On examination, she is alert but has expressive aphasia, right upper and lower extremity hemiparesis, and a right facial droop. Cardiopulmonary and abdominal exams are unremarkable.

LABORATORY DATA:
Labs show:
Hemoglobin 10.8 g/dL
Leukocyte count 7,900/mm³
Platelets 242,000/mm³
Serum β-hCG 75,300 mIU/mL

IMAGING:
A CT scan of the head shows a hemorrhagic lesion in the left parietal lobe. CT angiography of the chest reveals multiple bilateral pulmonary nodules.
