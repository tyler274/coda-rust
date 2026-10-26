# synthetic_hospital patient 2758

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 42 at first encounter, sex F, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-03-17 (Wed) | outpatient | Pulmonology |
| 1 | 2021-01-15 | 2022-03-14 (Mon) | outpatient | Neurology |
| 2 | 2023-01-15 | 2024-03-12 (Tue) | outpatient | Gastroenterology |
| 3 | 2023-05-15 | 2024-07-01 (Mon) | outpatient | Psychiatry |
| 4 | 2024-01-15 | 2025-03-03 (Mon) | outpatient | Internal Medicine |
| 5 | 2024-07-15 | 2025-08-29 (Fri) | ed | Emergency Medicine |
| 6 | 2024-09-15 | 2025-11-03 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Seasonal allergens (tree pollen, dust mites)",
  "Shellfish (pruritus, urticaria)"
 ],
 "insurance": "Commercial PPO",
 "occupation": "High school mathematics teacher",
 "alcohol_use": "Social (1-2 glasses of wine per week)",
 "family_history": [
  "Father: Hereditary Hemorrhagic Telangiectasia (recurrent epistaxis, pulmonary arteriovenous malformations)",
  "Mother: Hypothyroidism, generalized anxiety disorder",
  "Son: ADHD (age 17, estranged following domestic conflict)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "25-100 mg PO three times daily",
   "name": "Carbidopa-Levodopa"
  },
  {
   "dose": "2 mg PO at bedtime",
   "name": "Alprazolam"
  },
  {
   "dose": "10 mg PO daily as needed",
   "name": "Loratadine"
  },
  {
   "dose": "325 mg PO daily with vitamin C",
   "name": "Ferrous sulfate"
  }
 ],
 "surgical_history": [
  "Nasal cauterization for recurrent epistaxis (age 25)",
  "Esophagogastroduodenoscopy with small bowel biopsy (age 44)"
 ],
 "chronic_conditions": [
  "Hereditary Hemorrhagic Telangiectasia (Osler-Weber-Rendu syndrome)",
  "Parkinson disease",
  "Generalized anxiety disorder",
  "Allergic rhinitis",
  "Iron deficiency anemia secondary to menorrhagia",
  "Functional neurological disorder (conversion disorder)"
 ],
 "age_at_first_encounter": 42
}
```

Primary diagnoses: ["Parkinson disease with psychosis", "Iron Deficiency Anemia", "Acute hemolytic transfusion reaction", "Ancylostomiasis (Hookworm infection)", "Hereditary Hemorrhagic Telangiectasia (Osler-Weber-Rendu Syndrome)", "Functional neurological disorder", "Benzodiazepine withdrawal syndrome"]
Comorbidities: ["Menorrhagia", "Chronic insomnia", "Iron Deficiency Anemia"]

## Encounter 0: source 2020-01-15, practice 2021-03-17, outpatient, Pulmonology

Chief complaint: Worsening dyspnea on exertion and frequent nosebleeds

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening dyspnea on exertion and frequent nosebleeds

HISTORY OF PRESENT ILLNESS:
A 38-year-old woman is evaluated for worsening dyspnea on exertion and frequent nosebleeds that began during adolescence.

PAST MEDICAL HISTORY:
Active Problem List:
- Hereditary Hemorrhagic Telangiectasia (Osler-Weber-Rendu syndrome)
- Parkinson disease
- Generalized anxiety disorder
- Allergic rhinitis
- Iron deficiency anemia secondary to menorrhagia
- Functional neurological disorder (conversion disorder)

PAST SURGICAL HISTORY:
- Nasal cauterization for recurrent epistaxis (age 25)
- Esophagogastroduodenoscopy with small bowel biopsy (age 44)

MEDICATIONS:
Home Medications:
- Carbidopa-Levodopa 25-100 mg PO three times daily
- Alprazolam 2 mg PO at bedtime
- Loratadine 10 mg PO daily as needed
- Ferrous sulfate 325 mg PO daily with vitamin C

ALLERGIES:
Seasonal allergens (tree pollen, dust mites), Shellfish (pruritus, urticaria)

FAMILY HISTORY:
She reports a family history of similar symptoms in her father and paternal aunt, but her husband's family is unaffected.
The couple is concerned about the possibility of their children inheriting the condition.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per week). Occupation: High school mathematics teacher.

PHYSICAL EXAMINATION:
Physical examination is shown in below photograph.

LABORATORY DATA:
Further evaluation reveals iron deficiency anemia.

IMAGING:
Echocardiography demonstrates a high-normal ejection fraction of 72% without valvular abnormalities.

ASSESSMENT AND PLAN:
The physician suspects a vascular disorder with autosomal dominant inheritance involving TGF-β signaling.

## Encounter 1: source 2021-01-15, practice 2022-03-14, outpatient, Neurology

Chief complaint: Chronic sleep disturbance and urge to move legs at night

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Michael Okonkwo, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Chronic sleep disturbance and urge to move legs at night

HISTORY OF PRESENT ILLNESS:
The patient is a 43-year-old female with a history of Parkinson disease and Hereditary Hemorrhagic Telangiectasia (Osler-Weber-Rendu syndrome) who returns to the neurology clinic for evaluation of chronic sleep disturbance. She was last evaluated in pulmonology approximately one year prior for epistaxis and dyspnea. For the past year, she has experienced an urge to move her legs whenever she lies down at night, especially during periods of inactivity. She describes a deep, unpleasant sensation in both calves that is temporarily relieved by movement. She often walks around her bedroom multiple times before falling asleep. The symptoms are confined to the evening and nighttime and do not occur while standing, sitting, or during the day. She reports daily fatigue due to poor sleep but has no history of anxiety, depression, or other psychiatric issues.

PAST MEDICAL HISTORY:
Active Problem List:
- Hereditary Hemorrhagic Telangiectasia (Osler-Weber-Rendu Syndrome) (diagnosed 2020-01-15)
- Parkinson disease
- Generalized anxiety disorder
- Allergic rhinitis
- Iron deficiency anemia secondary to menorrhagia
- Functional neurological disorder (conversion disorder)

PAST SURGICAL HISTORY:
- Nasal cauterization for recurrent epistaxis (age 25)
- Esophagogastroduodenoscopy with small bowel biopsy (age 44)

MEDICATIONS:
Home Medications:
- Carbidopa-Levodopa 25-100 mg PO three times daily
- Alprazolam 2 mg PO at bedtime
- Loratadine 10 mg PO daily as needed
- Ferrous sulfate 325 mg PO daily with vitamin C

ALLERGIES:
Seasonal allergens (tree pollen, dust mites), Shellfish (pruritus, urticaria)

FAMILY HISTORY:
- Father: Hereditary Hemorrhagic Telangiectasia (recurrent epistaxis, pulmonary arteriovenous malformations)
- Mother: Hypothyroidism, generalized anxiety disorder
- Son: ADHD (age 17, estranged following domestic conflict)

SOCIAL HISTORY:
She follows a vegetarian diet and reports that her menstrual periods are regular but heavy.

PHYSICAL EXAMINATION:
Neurologic examination is normal.

ASSESSMENT AND PLAN:
A diagnosis is made, and the physician discusses both symptom management and long-term implications of the condition.

## Encounter 2: source 2023-01-15, practice 2024-03-12, outpatient, Gastroenterology

Chief complaint: Difficulty swallowing solid foods and sore tongue

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Emily Rodriguez, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Difficulty swallowing solid foods and sore tongue

HISTORY OF PRESENT ILLNESS:
The patient is a 45-year-old woman with established Hereditary Hemorrhagic Telangiectasia and Parkinson disease, previously evaluated in Pulmonology and Neurology, who returns for ongoing care. She presents with complaints of difficulty swallowing solid foods that has been gradually worsening over the past year. She reports intermittent fatigue and recently noticed her tongue feels sore when eating spicy foods. She denies chest pain, reflux, weight loss, or changes in appetite.

PAST MEDICAL HISTORY:
Active Problem List:
- Hereditary Hemorrhagic Telangiectasia (Osler-Weber-Rendu Syndrome) (diagnosed 2020-01-15)
- Parkinson disease with psychosis (diagnosed 2021-01-15)
- Parkinson disease
- Generalized anxiety disorder
- Allergic rhinitis
- Iron deficiency anemia secondary to menorrhagia
- Functional neurological disorder (conversion disorder)

PAST SURGICAL HISTORY:
- Nasal cauterization for recurrent epistaxis (age 25)
- Esophagogastroduodenoscopy with small bowel biopsy (age 44)

MEDICATIONS:
Home Medications:
- Carbidopa-Levodopa 25-100 mg PO three times daily
- Alprazolam 2 mg PO at bedtime
- Loratadine 10 mg PO daily as needed
- Ferrous sulfate 325 mg PO daily with vitamin C

ALLERGIES:
Seasonal allergens (tree pollen, dust mites), Shellfish (pruritus, urticaria)

FAMILY HISTORY:
- Father: Hereditary Hemorrhagic Telangiectasia (recurrent epistaxis, pulmonary arteriovenous malformations)
- Mother: Hypothyroidism, generalized anxiety disorder
- Son: ADHD (age 17, estranged following domestic conflict)

SOCIAL HISTORY:
Her diet consists mainly of vegetables and grains, and she avoids red meat due to personal preferences.

VITAL SIGNS:
Vitals are: temperature 36.7°C (98.1°F), blood pressure 116/72 mm Hg, pulse 84/min, respiratory rate 16/min, and BMI 22.6 kg/m².

PHYSICAL EXAMINATION:
On physical exam, she appears pale and has dry, brittle nails that appear spoon-shaped. Cardiac and abdominal exams are unremarkable. Neurological examination is normal.

## Encounter 3: source 2023-05-15, practice 2024-07-01, outpatient, Psychiatry

Chief complaint: Progressive difficulty speaking over the past 3 months

ENCOUNTER NOTE
Date: 2023-05-15
Provider: Dr. Aisha Patel, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive difficulty speaking over the past 3 months

HISTORY OF PRESENT ILLNESS:
The patient is a 45-year-old female with a history of Hereditary Hemorrhagic Telangiectasia, Parkinson disease, iron deficiency anemia, and functional neurological disorder who presents to the Psychiatry clinic with progressive difficulty speaking. She was most recently evaluated by Gastroenterology four months prior for dysphagia and sore tongue in the setting of iron deficiency anemia, with ongoing management of her chronic conditions. She reports that her voice began sounding strained after a heated argument with her teenage son, during which she told him she wished she had never raised him. Two days later, he abruptly left home to stay with his father in another state. Since then, she has been unable to project her voice and often resorts to whispering or writing to communicate. She denies sore throat, cough, or recent upper respiratory infection. The patient reports feeling persistently sad and often reflects on the incident, feeling intense guilt for her words. She has difficulty initiating sleep and wakes frequently during the night, sometimes replaying the argument in her mind. She continues to work as an administrative assistant but avoids talking on the phone. Coworkers have noticed her withdrawn demeanor and lack of participation in meetings. She has not lost interest in activities she enjoys, but she spends less time socializing because of her voice problem. She denies flashbacks, intrusive images, or nightmares about the event. There is no history of hallucinations or delusions. She has not experienced significant weight changes or appetite disturbance.

PAST MEDICAL HISTORY:
Active Problem List:
- Hereditary Hemorrhagic Telangiectasia (Osler-Weber-Rendu Syndrome) (diagnosed 2020-01-15)
- Parkinson disease with psychosis (diagnosed 2021-01-15)
- Iron Deficiency Anemia (diagnosed 2023-01-15)
- Parkinson disease
- Generalized anxiety disorder
- Allergic rhinitis
- Iron deficiency anemia secondary to menorrhagia
- Functional neurological disorder (conversion disorder)

PAST SURGICAL HISTORY:
- Nasal cauterization for recurrent epistaxis (age 25)
- Esophagogastroduodenoscopy with small bowel biopsy (age 44)

MEDICATIONS:
Home Medications:
- Carbidopa-Levodopa 25-100 mg PO three times daily
- Alprazolam 2 mg PO at bedtime
- Loratadine 10 mg PO daily as needed
- Ferrous sulfate 325 mg PO daily with vitamin C

ALLERGIES:
Seasonal allergens (tree pollen, dust mites), Shellfish (pruritus, urticaria)

FAMILY HISTORY:
- Father: Hereditary Hemorrhagic Telangiectasia (recurrent epistaxis, pulmonary arteriovenous malformations)
- Mother: Hypothyroidism, generalized anxiety disorder
- Son: ADHD (age 17, estranged following domestic conflict)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per week). Occupation: High school mathematics teacher.

REVIEW OF SYSTEMS:
The patient denies any recent fevers, neck masses, or difficulty swallowing.

PHYSICAL EXAMINATION:
On physical examination, she communicates in a whisper, but the oropharynx appears normal and there are no cranial nerve deficits. Mental status examination shows a constricted affect, with anxiety noted when the argument is discussed.

## Encounter 4: source 2024-01-15, practice 2025-03-03, outpatient, Internal Medicine

Chief complaint: Progressive fatigue, lightheadedness, and decreased exercise tolerance

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. David Kim, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive fatigue, lightheadedness, and decreased exercise tolerance

HISTORY OF PRESENT ILLNESS:
The patient is a 46-year-old female with hereditary hemorrhagic telangiectasia, Parkinson disease, and a history of iron deficiency anemia who presents with progressive fatigue, lightheadedness, and decreased exercise tolerance. She denies any recent illness, fever, weight loss, or significant gastrointestinal symptoms such as diarrhea or abdominal pain. Six months ago, she spent several weeks in a rural farming village in northeastern Brazil as part of a volunteer agricultural program, where she often worked barefoot in the soil and lacked access to clean water and adequate sanitation facilities. She has not traveled since and denies heavy menstrual bleeding.

PAST MEDICAL HISTORY:
Active Problem List:
- Hereditary Hemorrhagic Telangiectasia (Osler-Weber-Rendu Syndrome) (diagnosed 2020-01-15)
- Parkinson disease with psychosis (diagnosed 2021-01-15)
- Iron Deficiency Anemia (diagnosed 2023-01-15)
- Functional neurological disorder (diagnosed 2023-05-15)
- Parkinson disease
- Generalized anxiety disorder
- Allergic rhinitis
- Iron deficiency anemia secondary to menorrhagia
- Functional neurological disorder (conversion disorder)

PAST SURGICAL HISTORY:
- Nasal cauterization for recurrent epistaxis (age 25)
- Esophagogastroduodenoscopy with small bowel biopsy (age 44)

MEDICATIONS:
She takes use of medications.

Home Medications:
- Carbidopa-Levodopa 25-100 mg PO three times daily
- Alprazolam 2 mg PO at bedtime
- Loratadine 10 mg PO daily as needed
- Ferrous sulfate 325 mg PO daily with vitamin C

ALLERGIES:
Seasonal allergens (tree pollen, dust mites), Shellfish (pruritus, urticaria)

FAMILY HISTORY:
- Father: Hereditary Hemorrhagic Telangiectasia (recurrent epistaxis, pulmonary arteriovenous malformations)
- Mother: Hypothyroidism, generalized anxiety disorder
- Son: ADHD (age 17, estranged following domestic conflict)

SOCIAL HISTORY:
Her diet is relatively balanced,

PHYSICAL EXAMINATION:
On physical examination, she appears pale but is hemodynamically stable. There is no jaundice, lymphadenopathy, or hepatosplenomegaly. Cardiopulmonary and abdominal exams are unremarkable.

LABORATORY DATA:
Laboratory studies reveal a hemoglobin level of 7.2 g/dL and hematocrit of 23%, with a mean corpuscular volume of 68 fL.

OTHER STUDIES:
Stool microscopy demonstrates numerous oval eggs with thin shells.

## Encounter 5: source 2024-07-15, practice 2025-08-29, ed, Emergency Medicine

Chief complaint: Increasing anxiety, tremors, and palpitations

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. James Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Increasing anxiety, tremors, and palpitations

HISTORY OF PRESENT ILLNESS:
This 46-year-old female with Hereditary Hemorrhagic Telangiectasia, Parkinson disease, functional neurological disorder, and generalized anxiety disorder was last seen in January 2024 for hookworm infection and now presents to the emergency department with a 24-hour history of increasing anxiety, tremors, and palpitations. She reports a history of insomnia for which she was taking alprazolam 2 mg at bedtime for the past 6 months. She abruptly stopped taking the medication 3 days ago after her prescription ran out.

PAST MEDICAL HISTORY:
Active Problem List:
- Hereditary Hemorrhagic Telangiectasia (Osler-Weber-Rendu Syndrome) (diagnosed 2020-01-15)
- Parkinson disease with psychosis (diagnosed 2021-01-15)
- Iron Deficiency Anemia (diagnosed 2023-01-15)
- Functional neurological disorder (diagnosed 2023-05-15)
- Ancylostomiasis (Hookworm infection) (diagnosed 2024-01-15)
- Parkinson disease
- Generalized anxiety disorder
- Allergic rhinitis
- Iron deficiency anemia secondary to menorrhagia
- Functional neurological disorder (conversion disorder)

Her past medical history is significant for generalized anxiety disorder.

PAST SURGICAL HISTORY:
- Nasal cauterization for recurrent epistaxis (age 25)
- Esophagogastroduodenoscopy with small bowel biopsy (age 44)

MEDICATIONS:
Home Medications:
- Carbidopa-Levodopa 25-100 mg PO three times daily
- Alprazolam 2 mg PO at bedtime
- Loratadine 10 mg PO daily as needed
- Ferrous sulfate 325 mg PO daily with vitamin C

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Hereditary Hemorrhagic Telangiectasia (recurrent epistaxis, pulmonary arteriovenous malformations)
- Mother: Hypothyroidism, generalized anxiety disorder
- Son: ADHD (age 17, estranged following domestic conflict)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per week). Occupation: High school mathematics teacher.

VITAL SIGNS:
Her vital signs are as follows: blood pressure 160/95 mmHg, heart rate 110 beats per minute, respiratory rate 22 breaths per minute, and temperature 37.8°C (100°F)

PHYSICAL EXAMINATION:
On examination, she is agitated, diaphoretic, and tremulous. Neurological examination reveals increased muscle tone and hyperreflexia.

LABORATORY DATA:
Laboratory studies show:
- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Chloride: 102 mEq/L
- Bicarbonate: 24 mEq/L
- Blood urea nitrogen: 18 mg/dL
- Creatinine: 0.9 mg/dL
- Glucose: 98 mg/dL
- Calcium: 9.2 mg/dL
- Magnesium: 2.0 mEq/L
- Complete blood count: within normal limits

OTHER STUDIES:
An electrocardiogram (ECG) shows sinus tachycardia with no other abnormalities.

## Encounter 6: source 2024-09-15, practice 2025-11-03, ed, Emergency Medicine

Chief complaint: Itching and raised erythematous wheals during blood transfusion

ENCOUNTER NOTE
Date: 2024-09-15
Provider: Dr. Lisa Wong, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Itching and raised erythematous wheals during blood transfusion

HISTORY OF PRESENT ILLNESS:
This 46-year-old female with hereditary hemorrhagic telangiectasia and iron deficiency anemia is undergoing a packed red blood cell transfusion in the emergency department. Fifteen minutes into the transfusion, she complains of itching and develops raised, erythematous wheals on her trunk and arms. She denies any difficulty breathing, changes in voice, or feeling of throat closure.

PAST MEDICAL HISTORY:
Active Problem List:
- Hereditary Hemorrhagic Telangiectasia (Osler-Weber-Rendu Syndrome) (diagnosed 2020-01-15)
- Parkinson disease with psychosis (diagnosed 2021-01-15)
- Iron Deficiency Anemia (diagnosed 2023-01-15)
- Functional neurological disorder (diagnosed 2023-05-15)
- Ancylostomiasis (Hookworm infection) (diagnosed 2024-01-15)
- Benzodiazepine withdrawal syndrome (diagnosed 2024-07-15)
- Parkinson disease
- Generalized anxiety disorder
- Allergic rhinitis
- Iron deficiency anemia secondary to menorrhagia
- Functional neurological disorder (conversion disorder)

Her past medical history is significant for allergic rhinitis.

PAST SURGICAL HISTORY:
- Nasal cauterization for recurrent epistaxis (age 25)
- Esophagogastroduodenoscopy with small bowel biopsy (age 44)

MEDICATIONS:
She takes for which she takes loratadine as needed.

Home Medications:
- Carbidopa-Levodopa 25-100 mg PO three times daily
- Alprazolam 2 mg PO at bedtime
- Ferrous sulfate 325 mg PO daily with vitamin C

ALLERGIES:
She has no known drug allergies.

FAMILY HISTORY:
- Father: Hereditary Hemorrhagic Telangiectasia (recurrent epistaxis, pulmonary arteriovenous malformations)
- Mother: Hypothyroidism, generalized anxiety disorder
- Son: ADHD (age 17, estranged following domestic conflict)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per week). Occupation: High school mathematics teacher.

VITAL SIGNS:
Her blood pressure is 125/80 mmHg, heart rate is 88 beats per minute, and respiratory rate is 16 breaths per minute.

PHYSICAL EXAMINATION:
On examination, she has no wheezing, stridor, or angioedema.

PLAN:
The transfusion is stopped immediately.
