# synthetic_hospital patient 2072

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 41 at first encounter, sex M, Non-Hispanic Black, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-10-06 (Mon) | ed | Emergency Medicine |
| 1 | 2020-05-15 | 2026-01-27 (Tue) | ed | Emergency Medicine |
| 2 | 2020-10-15 | 2026-07-11 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Unemployed, disabled",
 "alcohol_use": "History of alcohol use disorder, currently abstinent for 6 months",
 "family_history": [
  "Mother with schizophrenia",
  "Father with opioid use disorder",
  "No known family history of hematologic or immunodeficiency disorders"
 ],
 "race_ethnicity": "Non-Hispanic Black",
 "smoking_status": "Current smoker, 1 pack per day for 20 years",
 "home_medications": [
  {
   "dose": "50/25/200 mg orally daily, poorly adherent",
   "name": "Bictegravir/Tenofovir alafenamide/Emtricitabine (Biktarvy)"
  },
  {
   "dose": "300 mg orally BID",
   "name": "Clozapine"
  },
  {
   "dose": "1200 mg orally weekly for MAC prophylaxis, poorly adherent",
   "name": "Azithromycin"
  }
 ],
 "surgical_history": [
  "Incision and drainage of left arm abscess, 2019",
  "No other known surgeries"
 ],
 "chronic_conditions": [
  "Human Immunodeficiency Virus (HIV) infection with AIDS (CD4 count <50 cells/mm\u00b3)",
  "Schizophrenia, treatment-refractory",
  "Major depressive disorder, recurrent",
  "Generalized anxiety disorder",
  "Opioid use disorder, severe, in unstable remission",
  "History of invasive fungal pneumonia (Pneumocystis jirovecii)",
  "History of disseminated Mycobacterium avium complex infection"
 ],
 "age_at_first_encounter": 41
}
```

Primary diagnoses: ["Cytomegalovirus esophagitis", "Agranulocytosis"]
Comorbidities: ["Major depressive disorder", "Intravenous Drug Use (Substance Use Disorder)", "Inadequate Health Insurance", "HIV-related opportunistic skin infection"]

## Encounter 0: source 2020-01-15, practice 2025-10-06, ed, Emergency Medicine

Chief complaint: Two-week history of worsening blurry vision, floaters, and flashes in the left eye

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-week history of worsening blurry vision, floaters, and flashes in the left eye

HISTORY OF PRESENT ILLNESS:
He says the problem began as “little specks floating around” that have gradually grown into large dark patches obscuring his sight. He also reports occasional flashes of light and describes his vision as “looking through a fogged-up window.” He has mild discomfort but no severe eye pain. Today, he says his main concern is that he is afraid of going completely blind. He explains that his vision problems are much worse in his left eye but are beginning to affect the right eye as well.

PAST MEDICAL HISTORY:
Active Problem List:
- Human Immunodeficiency Virus (HIV) infection with AIDS (CD4 count <50 cells/mm³)
- Schizophrenia, treatment-refractory
- Major depressive disorder, recurrent
- Generalized anxiety disorder
- Opioid use disorder, severe, in unstable remission
- History of invasive fungal pneumonia (Pneumocystis jirovecii)
- History of disseminated Mycobacterium avium complex infection

The patient has a long history of HIV infection diagnosed 12 years ago. He was initially prescribed antiretroviral therapy but has been inconsistently adherent, often stopping his medications during times of instability. He admits that he has not taken his HIV medicines in over a year. His last clinic visit was more than 18 months ago.

PAST SURGICAL HISTORY:
- Incision and drainage of left arm abscess, 2019
- No other known surgeries

MEDICATIONS:
Home Medications:
- Bictegravir/Tenofovir alafenamide/Emtricitabine (Biktarvy) 50/25/200 mg orally daily, poorly adherent
- Clozapine 300 mg orally BID
- Azithromycin 1200 mg orally weekly for MAC prophylaxis, poorly adherent

ALLERGIES:
NKDA

FAMILY HISTORY:
He was once married, but his wife passed away 10 years ago from complications of breast cancer. He became estranged from his only brother after a series of family disputes, and both of his parents died in his twenties.

SOCIAL HISTORY:
He has struggled with depression since these losses. He began using intravenous heroin in his thirties, which worsened his social isolation. Over the past decade, he has been intermittently homeless, often sleeping in shelters or outdoors. For the past several winters, he has been staying under a bridge near Harvard Square in Cambridge, Massachusetts, where he would light small fires at night to stay warm. He sometimes traded belongings for food and occasionally worked day labor jobs. Despite these hardships, he has had a few passions in life. He enjoys writing short stories, especially about his experiences traveling across New England. He once told a physician that he loved the sense of freedom he felt when hitchhiking between small towns. He also has a deep interest in jazz and would occasionally busk with a harmonica outside small cafes in Boston. The patient admits to past alcohol and cocaine use but says that heroin was his main struggle. He has been on and off methadone treatment but relapsed frequently. He has not used intravenous drugs in the last year, though he continues to smoke cigarettes daily.

REVIEW OF SYSTEMS:
He denies recent headaches, focal weakness, or seizures. He has had no recent fevers, chills, or weight gain but admits to losing 15 pounds over the past 6 months.

VITAL SIGNS:
His vital signs are stable.

PHYSICAL EXAMINATION:
On examination, he is thin, with temporal wasting. He appears disheveled but cooperative. Ocular exam reveals cotton-wool exudates and hemorrhagic lesions along the retinal periphery, resembling a “pizza pie” or “flame-shaped” pattern. Pupillary light reflexes are intact. The remainder of the neurological exam is nonfocal.

LABORATORY DATA:
Laboratory studies from today show a CD4 count of 35/mm³.

## Encounter 1: source 2020-05-15, practice 2026-01-27, ed, Emergency Medicine

Chief complaint: Fever, chills, and severe sore throat preventing oral intake

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. Michael Okonkwo, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fever, chills, and severe sore throat preventing oral intake

HISTORY OF PRESENT ILLNESS:
The patient is a 41-year-old male with HIV infection with AIDS (CD4 count <50 cells/mm³), treatment-refractory schizophrenia, and a history of CMV esophagitis diagnosed in January 2020, who is poorly adherent to antiretroviral therapy and MAC prophylaxis. He is brought to the emergency department by a community outreach volunteer because of fever, chills, and sore throat. He says that the throat pain started 3 days ago and has been getting progressively worse. He also reports fatigue, malaise, and painful swallowing that prevents him from eating much solid food. He denies cough, chest pain, or shortness of breath. Over the past week, however, he has felt progressively more fatigued. He developed subjective fevers, chills, and oral discomfort. Two days ago he noticed painful ulcerations on his gums and tongue. Today, he feels too weak to walk long distances and accepted the help of a volunteer who insisted he be brought to the hospital.

PAST MEDICAL HISTORY:
Active Problem List:
- Cytomegalovirus esophagitis (diagnosed 2020-01-15)
- Human Immunodeficiency Virus (HIV) infection with AIDS (CD4 count <50 cells/mm³)
- Schizophrenia, treatment-refractory
- Major depressive disorder, recurrent
- Generalized anxiety disorder
- Opioid use disorder, severe, in unstable remission
- History of invasive fungal pneumonia (Pneumocystis jirovecii)
- History of disseminated Mycobacterium avium complex infection

His past psychiatric history is extensive, including diagnoses of schizophrenia, depression, and generalized anxiety disorder. He has been hospitalized multiple times in psychiatric wards since his late teens, often due to auditory hallucinations and disorganized behavior.

PAST SURGICAL HISTORY:
- Incision and drainage of left arm abscess, 2019
- No other known surgeries

MEDICATIONS:
Over the years, he has been tried on multiple medications including haloperidol, risperidone, quetiapine, fluoxetine, and lithium. Due to persistent refractory psychosis, his care team eventually initiated clozapine therapy, which he has been taking for the past year. He is also prescribed valproic acid as a mood stabilizer, lorazepam as needed for agitation, and sertraline for comorbid depressive symptoms. He admits that his lifestyle makes it difficult to keep up with medical appointments, and he often skips his medications depending on availability. Despite these challenges, he has continued clozapine almost daily because, in his words, "it's the only one that really makes the voices quiet."

Home Medications:
- Bictegravir/Tenofovir alafenamide/Emtricitabine (Biktarvy) 50/25/200 mg orally daily, poorly adherent
- Azithromycin 1200 mg orally weekly for MAC prophylaxis, poorly adherent

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with schizophrenia
- Father with opioid use disorder
- No known family history of hematologic or immunodeficiency disorders

SOCIAL HISTORY:
He lost touch with his family years ago after repeated conflicts and strained relationships, and he currently has no fixed residence. He usually sleeps outside near a convenience store in Brickell, Miami. He describes this as "the safest spot I could find" and says that store clerks occasionally give him food. On days when he does not stay in Brickell, he sometimes travels to Wynwood and sleeps near an abandoned building, where he feels somewhat shielded from the rain. He frequently wanders shopping streets in both neighborhoods, asking strangers for spare change. He does not smoke cigarettes but occasionally drinks alcohol when he has access to it. He does not use illicit drugs currently but has a remote history of marijuana and cocaine use.

REVIEW OF SYSTEMS:
On review of systems, he denies chest pain, hemoptysis, abdominal pain, diarrhea, or skin rashes.

VITAL SIGNS:
His temperature is 38.9 °C (102 °F), blood pressure 108/65 mm Hg, pulse 110/min, and respiratory rate 18/min. Oxygen saturation is 97% on room air.

PHYSICAL EXAMINATION:
On physical examination, he appears tired and somewhat disheveled. He has multiple tender oral ulcers and pharyngeal erythema. No cervical lymphadenopathy is appreciated. Cardiovascular and pulmonary exams are unremarkable. Abdominal exam reveals no hepatosplenomegaly. There are no skin rashes or petechiae.

LABORATORY DATA:
Further questioning reveals that he has not had a complete blood count performed in several months.

ASSESSMENT AND PLAN:
His care team is concerned about a medication-induced complication.

## Encounter 2: source 2020-10-15, practice 2026-07-11, ed, Emergency Medicine

Chief complaint: Two-day history of bright red blood per rectum

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Emily Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-day history of bright red blood per rectum

HISTORY OF PRESENT ILLNESS:
This 41-year-old male with HIV/AIDS (CD4 <50), treatment-refractory schizophrenia, and poor medication adherence presents with a two-day history of bright red blood per rectum, following prior encounters for cytomegalovirus esophagitis and agranulocytosis. He is afebrile.

PAST MEDICAL HISTORY:
Active Problem List:
- Cytomegalovirus esophagitis (diagnosed 2020-01-15)
- Agranulocytosis (diagnosed 2020-05-15)
- Human Immunodeficiency Virus (HIV) infection with AIDS (CD4 count <50 cells/mm³)
- Schizophrenia, treatment-refractory
- Major depressive disorder, recurrent
- Generalized anxiety disorder
- Opioid use disorder, severe, in unstable remission
- History of invasive fungal pneumonia (Pneumocystis jirovecii)
- History of disseminated Mycobacterium avium complex infection

He was treated for a fungal pneumonia last year and a separate non-Tuberculous Mycobacterium pulmonary infection 6 months ago.

PAST SURGICAL HISTORY:
- Incision and drainage of left arm abscess, 2019
- No other known surgeries

MEDICATIONS:
Home Medications:
- Bictegravir/Tenofovir alafenamide/Emtricitabine (Biktarvy) 50/25/200 mg orally daily, poorly adherent
- Clozapine 300 mg orally BID
- Azithromycin 1200 mg orally weekly for MAC prophylaxis, poorly adherent

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with schizophrenia
- Father with opioid use disorder
- No known family history of hematologic or immunodeficiency disorders

SOCIAL HISTORY:
He has an history of being in and out of different homeless shelters and emergency room visits for drug overdoses.

VITAL SIGNS:
Vitals are normal.

PATHOLOGY:
Biopsy of ulcerated mucosa shows intranuclear inclusions.

OTHER STUDIES:
Sigmoidoscopy shows confluent ulcers.
