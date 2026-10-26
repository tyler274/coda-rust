# synthetic_hospital patient 2100

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 43 at first encounter, sex M, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-07-13 (Tue) | outpatient | Gastroenterology |
| 1 | 2020-07-15 | 2022-01-17 (Mon) | outpatient | Allergy & Immunology |
| 2 | 2022-07-15 | 2024-01-08 (Mon) | outpatient | Gastroenterology |
| 3 | 2024-01-15 | 2025-07-15 (Tue) | outpatient | Gastroenterology |
| 4 | 2024-09-15 | 2026-03-28 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Grass pollen",
  "Ragweed pollen",
  "Dust mites"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Landscape designer",
 "alcohol_use": "Occasional (currently abstinent during antiviral therapy)",
 "family_history": [
  "Father: Myocardial infarction at age 66, hyperlipidemia",
  "Mother: Seasonal allergic rhinitis",
  "Sister: Asthma"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never",
 "home_medications": [
  {
   "dose": "20 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "180 mcg, 2 inhalations twice daily",
   "name": "Budesonide inhaler"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Omeprazole"
  },
  {
   "dose": "400 mg PO daily (week 8 of 12-week course)",
   "name": "Sofosbuvir"
  },
  {
   "dose": "1000 mg PO daily in 2 divided doses (week 8 of 12-week course)",
   "name": "Ribavirin"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Gastroesophageal reflux disease",
  "Mild persistent asthma",
  "Seasonal allergic rhinitis",
  "Hyperlipidemia",
  "Chronic hepatitis C virus infection"
 ],
 "age_at_first_encounter": 43
}
```

Primary diagnoses: ["Gastroesophageal reflux disease", "Allergic rhinitis", "Microangiopathic hemolytic anemia"]
Comorbidities: ["Allergic rhinitis"]

## Encounter 0: source 2020-01-15, practice 2021-07-13, outpatient, Gastroenterology

Chief complaint: Burning discomfort in the chest occurring several times per week, particularly after large evening meals

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Anita Kapoor, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Burning discomfort in the chest occurring several times per week, particularly after large evening meals

HISTORY OF PRESENT ILLNESS:
He presents to the clinic with a 9-month history of burning discomfort in the chest that occurs several times per week, particularly after large evening meals. He says the pain sometimes radiates to his throat and is accompanied by a sour, bitter taste in the back of his mouth. Lying down after dinner or bending forward makes the discomfort worse, while standing or walking after meals seems to help. He has started sleeping with two pillows at night to reduce the symptoms. Over the last month, the pain has occasionally awakened him from sleep. He reports that spicy foods, fried meals, tomato-based sauces, chocolate, and coffee all aggravate his symptoms. He drinks 2–3 cups of coffee daily, has wine with dinner several nights per week, and eats late at night due to his work schedule.

PAST MEDICAL HISTORY:
Active Problem List:
- Gastroesophageal reflux disease
- Mild persistent asthma
- Seasonal allergic rhinitis
- Hyperlipidemia
- Chronic hepatitis C virus infection

Past medical history is significant only for seasonal allergic rhinitis.

MEDICATIONS:
Medications include occasional over-the-counter antacids, which provide partial but temporary relief.

Home Medications:
- Atorvastatin 20 mg PO daily
- Budesonide inhaler 180 mcg, 2 inhalations twice daily
- Omeprazole 20 mg PO daily
- Sofosbuvir 400 mg PO daily (week 8 of 12-week course)
- Ribavirin 1000 mg PO daily in 2 divided doses (week 8 of 12-week course)

ALLERGIES:
Grass pollen, Ragweed pollen, Dust mites

FAMILY HISTORY:
- Father: Myocardial infarction at age 66, hyperlipidemia
- Mother: Seasonal allergic rhinitis
- Sister: Asthma

SOCIAL HISTORY:
Smoking: Never. Alcohol: Occasional (currently abstinent during antiviral therapy). Occupation: Landscape designer.

REVIEW OF SYSTEMS:
He denies dysphagia, odynophagia, hematemesis, or weight loss.

VITAL SIGNS:
Vital signs are normal.

PHYSICAL EXAMINATION:
On physical examination, he is afebrile and appears comfortable.
Cardiac, pulmonary, and abdominal examinations are unremarkable. Oropharyngeal mucosa is normal.

## Encounter 1: source 2020-07-15, practice 2022-01-17, outpatient, Allergy & Immunology

Chief complaint: Intermittent eye redness, itching, and watery discharge for 3 weeks during early summer season

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Chen, MD
Department: Allergy & Immunology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent eye redness, itching, and watery discharge for 3 weeks during early summer season

HISTORY OF PRESENT ILLNESS:
The patient presents for evaluation of recurrent allergic symptoms in the context of known seasonal rhinitis and prior gastroenterology assessment for GERD in January 2020. He also reports frequent sneezing and a clear nasal discharge, especially when he is outdoors or has the windows open. These symptoms occur every year around the same time and have been present since his early 30s. His symptoms are most bothersome in the mornings and when mowing lawns or trimming hedges. He states that he usually feels better after taking over-the-counter allergy medications such as loratadine or fexofenadine. These medications relieve his symptoms within an hour.

PAST MEDICAL HISTORY:
Active Problem List:
- Gastroesophageal reflux disease (diagnosed 2020-01-15)
- Mild persistent asthma
- Seasonal allergic rhinitis
- Hyperlipidemia
- Chronic hepatitis C virus infection

MEDICATIONS:
Home Medications:
- Atorvastatin 20 mg PO daily
- Budesonide inhaler 180 mcg, 2 inhalations twice daily
- Omeprazole 20 mg PO daily
- Sofosbuvir 400 mg PO daily (week 8 of 12-week course)
- Ribavirin 1000 mg PO daily in 2 divided doses (week 8 of 12-week course)

ALLERGIES:
Grass pollen, Ragweed pollen, Dust mites

FAMILY HISTORY:
- Father: Myocardial infarction at age 66, hyperlipidemia
- Mother: Seasonal allergic rhinitis
- Sister: Asthma

SOCIAL HISTORY:
He works as a landscape designer and spends most of the day outside.

PHYSICAL EXAMINATION:
Physical examination shows conjunctival injection and periorbital edema, but no fever, rash, or purulent discharge.

## Encounter 2: source 2022-07-15, practice 2024-01-08, outpatient, Gastroenterology

Chief complaint: Long-standing upper gastrointestinal symptoms with burning sensation behind breastbone

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Anita Kapoor, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Long-standing upper gastrointestinal symptoms with burning sensation behind breastbone

HISTORY OF PRESENT ILLNESS:
The patient returns to the Gastroenterology clinic for ongoing management of long-standing upper gastrointestinal symptoms, with similar presentations dating back to January 2020. He is currently in week 8 of a 12-week course of antiviral therapy for chronic hepatitis C. He reports experiencing a burning sensation behind his breastbone that worsens after meals and when lying down. These symptoms have been present for several years and occur multiple times a week, particularly after large meals or consuming coffee or fried foods. He notes occasional regurgitation of sour-tasting fluid into his mouth but denies dysphagia, odynophagia, vomiting, early satiety, or unintentional weight loss. He has not had any recent illness or changes in appetite.

PAST MEDICAL HISTORY:
Active Problem List:
- Gastroesophageal reflux disease (diagnosed 2020-01-15)
- Allergic rhinitis (diagnosed 2020-07-15)
- Mild persistent asthma
- Seasonal allergic rhinitis
- Hyperlipidemia
- Chronic hepatitis C virus infection

His medical history is otherwise unremarkable.

MEDICATIONS:
He takes he does not take any medications.

Home Medications:
- Atorvastatin 20 mg PO daily
- Budesonide inhaler 180 mcg, 2 inhalations twice daily
- Omeprazole 20 mg PO daily
- Sofosbuvir 400 mg PO daily (week 8 of 12-week course)
- Ribavirin 1000 mg PO daily in 2 divided doses (week 8 of 12-week course)

ALLERGIES:
Grass pollen, Ragweed pollen, Dust mites

FAMILY HISTORY:
- Father: Myocardial infarction at age 66, hyperlipidemia
- Mother: Seasonal allergic rhinitis
- Sister: Asthma

SOCIAL HISTORY:
Smoking: Never. Alcohol: Occasional (currently abstinent during antiviral therapy). Occupation: Landscape designer.

PHYSICAL EXAMINATION:
Physical examination is within normal limits

IMAGING:
Upper endoscopy is performed and the distal esophagus appears as shown in the photograph

## Encounter 3: source 2024-01-15, practice 2025-07-15, outpatient, Gastroenterology

Chief complaint: Worsening symptoms of heartburn and regurgitation over the past several weeks despite twice-daily PPI therapy

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Anita Kapoor, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening symptoms of heartburn and regurgitation over the past several weeks despite twice-daily PPI therapy

HISTORY OF PRESENT ILLNESS:
The patient returns to the Gastroenterology clinic for follow-up of long-standing gastroesophageal reflux disease, previously evaluated in January 2020 and July 2022 for burning chest discomfort and upper gastrointestinal symptoms. He reports a several-year history of GERD and has been taking omeprazole twice daily for the last 2 years. He previously had good symptom control but states that his symptoms flare up during specific times, particularly around holidays or social gatherings when he "eats more than usual." The patient recently attended a multi-day festival where meals were large and frequent, and he noticed that his usual PPI regimen provided little relief. He asks if his medications should be changed or increased further.

PAST MEDICAL HISTORY:
Active Problem List:
- Gastroesophageal reflux disease (diagnosed 2020-01-15)
- Allergic rhinitis (diagnosed 2020-07-15)
- Mild persistent asthma
- Seasonal allergic rhinitis
- Hyperlipidemia
- Chronic hepatitis C virus infection

Medical history includes mild asthma, seasonal allergies, and hyperlipidemia managed with atorvastatin.

MEDICATIONS:
Home Medications:
- Atorvastatin 20 mg PO daily
- Budesonide inhaler 180 mcg, 2 inhalations twice daily
- Omeprazole 20 mg PO daily
- Sofosbuvir 400 mg PO daily (week 8 of 12-week course)
- Ribavirin 1000 mg PO daily in 2 divided doses (week 8 of 12-week course)

ALLERGIES:
Grass pollen, Ragweed pollen, Dust mites

FAMILY HISTORY:
- Father: Myocardial infarction at age 66, hyperlipidemia
- Mother: Seasonal allergic rhinitis
- Sister: Asthma

SOCIAL HISTORY:
He does not smoke and drinks alcohol occasionally.

REVIEW OF SYSTEMS:
He denies dysphagia, weight loss, hematemesis, or melena.

VITAL SIGNS:
Vital signs are within normal limits.

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

## Encounter 4: source 2024-09-15, practice 2026-03-28, ed, Emergency Medicine

Chief complaint: Worsening fatigue and reduced exercise tolerance over the past 3 weeks with associated shortness of breath and lightheadedness

ENCOUNTER NOTE
Date: 2024-09-15
Provider: Dr. James Okonkwo, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening fatigue and reduced exercise tolerance over the past 3 weeks with associated shortness of breath and lightheadedness

HISTORY OF PRESENT ILLNESS:
The patient is a 47-year-old male with gastroesophageal reflux disease (previously evaluated in 2020, 2022, and 2024) who presents to the emergency department. He has been undergoing treatment for chronic hepatitis C virus (HCV) infection and started a new combination therapy 2 months ago, developing increasing shortness of breath when climbing stairs, lightheadedness, and occasional palpitations.

PAST MEDICAL HISTORY:
Active Problem List:
- Gastroesophageal reflux disease (diagnosed 2020-01-15)
- Allergic rhinitis (diagnosed 2020-07-15)
- Mild persistent asthma
- Seasonal allergic rhinitis
- Hyperlipidemia
- Chronic hepatitis C virus infection

His past medical history includes mild asthma, seasonal allergies.

MEDICATIONS:
His current medications include inhaled budesonide and combination therapy with sofosbuvir and ribavirin.

Home Medications:
- Atorvastatin 20 mg PO daily
- Budesonide inhaler 180 mcg, 2 inhalations twice daily
- Omeprazole 20 mg PO daily

ALLERGIES:
He has no known drug allergies

FAMILY HISTORY:
- Father: Myocardial infarction at age 66, hyperlipidemia
- Mother: Seasonal allergic rhinitis
- Sister: Asthma

SOCIAL HISTORY:
He does not smoke or drink alcohol

REVIEW OF SYSTEMS:
He denies fever, night sweats, chest pain, or gastrointestinal bleeding. His appetite is normal, and his weight is stable.

PHYSICAL EXAMINATION:
On exam, he appears pale and mildly tachycardic. Scleral icterus is noted, but there is no hepatosplenomegaly. Cardiopulmonary exam reveals no murmurs or crackles.

LABORATORY DATA:
Laboratory studies show a drop in hemoglobin from 13.6 g/dL at baseline to 9.1 g/dL. Reticulocyte count is elevated. Lactate dehydrogenase is elevated, and haptoglobin is low. A peripheral smear reveals polychromasia and fragmented erythrocytes. Direct antiglobulin (Coombs) test is negative.
