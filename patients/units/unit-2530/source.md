# synthetic_hospital patient 2530

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 29 at first encounter, sex M, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-10-21 (Fri) | outpatient | Endocrinology |
| 1 | 2020-04-15 | 2023-01-20 (Fri) | outpatient | Infectious Disease |
| 2 | 2020-05-15 | 2023-02-20 (Mon) | outpatient | Infectious Disease |
| 3 | 2021-01-15 | 2023-10-17 (Tue) | inpatient | Internal Medicine |
| 4 | 2021-07-15 | 2024-04-19 (Fri) | inpatient | Gastroenterology |
| 5 | 2022-01-15 | 2024-10-16 (Wed) | icu | Critical Care Medicine |
| 6 | 2022-07-15 | 2025-05-03 (Sat) | ed | Emergency Medicine |
| 7 | 2023-01-15 | 2025-10-26 (Sun) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Bartender",
 "alcohol_use": "None (despite occupational exposure)",
 "family_history": [
  "Father with Crohn's disease",
  "Maternal cousin with type 1 diabetes insipidus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "0.2 mg orally twice daily (prescribed, adherence inconsistent)",
   "name": "Desmopressin acetate"
  },
  {
   "dose": "300/150 mg orally twice daily (prescribed, currently non-adherent)",
   "name": "Zidovudine-Lamivudine"
  },
  {
   "dose": "800 mg orally three times daily",
   "name": "Mesalamine delayed-release"
  }
 ],
 "surgical_history": [
  "No prior surgeries"
 ],
 "chronic_conditions": [
  "HIV infection",
  "Central diabetes insipidus",
  "Crohn's disease"
 ],
 "age_at_first_encounter": 29
}
```

Primary diagnoses: ["Cryptosporidiosis", "Central diabetes insipidus (inadequately controlled)", "Small bowel obstruction due to fibrotic stricture secondary to Crohn's disease", "HIV-related opportunistic skin infection", "Cholera"]
Comorbidities: ["Traumatic brain injury", "Dehydration", "Orthostatic hypotension", "Reactive lymphadenopathy", "Oral candidiasis"]

## Encounter 0: source 2020-01-15, practice 2022-10-21, outpatient, Endocrinology

Chief complaint: Intense thirst and frequent urination for several weeks, waking multiple times nightly to void and drinking several liters of water daily without relief.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Intense thirst and frequent urination for several weeks, waking multiple times nightly to void and drinking several liters of water daily without relief.

HISTORY OF PRESENT ILLNESS:
A 27-year-old man is brought to the clinic by his roommate due to several weeks of intense thirst and frequent urination. The patient reports waking multiple times at night to urinate and says he often drinks several liters of water each day but still feels dehydrated. He denies any recent illness, fever, or medication changes.

PAST MEDICAL HISTORY:
Active Problem List:
- HIV infection
- Central diabetes insipidus
- Crohn's disease

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Desmopressin acetate 0.2 mg orally twice daily (prescribed, adherence inconsistent)
- Zidovudine-Lamivudine 300/150 mg orally twice daily (prescribed, currently non-adherent)
- Mesalamine delayed-release 800 mg orally three times daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with Crohn's disease
- Maternal cousin with type 1 diabetes insipidus

SOCIAL HISTORY:
He does not use alcohol or drugs

PHYSICAL EXAMINATION:
On physical examination, his mucous membranes appear dry, and he has mild tachycardia.

## Encounter 1: source 2020-04-15, practice 2023-01-20, outpatient, Infectious Disease

Chief complaint: Persistent nightly sweats, 12-pound unintentional weight loss over two months, and painless white plaques on tongue that cannot be scraped off.

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Infectious Disease
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent nightly sweats, 12-pound unintentional weight loss over two months, and painless white plaques on tongue that cannot be scraped off.

HISTORY OF PRESENT ILLNESS:
The patient is a 29-year-old male with HIV, diabetes insipidus, and Crohn's disease last seen in Endocrinology three months ago. He reports experiencing nightly sweats and a 12-pound unintentional weight loss over the past two months. He has also noticed the development of painless white plaques on his tongue that he is unable to scrape off. He denies recent travel or animal exposures.

PAST MEDICAL HISTORY:
Active Problem List:
- Central diabetes insipidus (inadequately controlled) (diagnosed 2020-01-15)
- HIV infection
- Central diabetes insipidus
- Crohn's disease

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
He takes he does not take any medications.

Home Medications:
- Desmopressin acetate 0.2 mg orally twice daily (prescribed, adherence inconsistent)
- Zidovudine-Lamivudine 300/150 mg orally twice daily (prescribed, currently non-adherent)
- Mesalamine delayed-release 800 mg orally three times daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with Crohn's disease
- Maternal cousin with type 1 diabetes insipidus

SOCIAL HISTORY:
He is sexually active with men and does not consistently use barrier protection

PHYSICAL EXAMINATION:
Cervical lymphadenopathy and oropharyngeal candidiasis

LABORATORY DATA:
CD4+ T-cell count of 122/mm³ and a viral load of over 100,000 copies/mL

## Encounter 2: source 2020-05-15, practice 2023-02-20, outpatient, Infectious Disease

Chief complaint: Follow-up evaluation after starting treatment for newly diagnosed opportunistic skin and oral infection with persistent night sweats and weight loss.

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. Michael Rodriguez, MD
Department: Infectious Disease
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up evaluation after starting treatment for newly diagnosed opportunistic skin and oral infection with persistent night sweats and weight loss.

HISTORY OF PRESENT ILLNESS:
The patient is a 29-year-old male with known Crohn's disease, Central diabetes insipidus diagnosed in January 2020, and HIV infection who returns for follow-up evaluation after starting treatment for a newly diagnosed opportunistic skin and oral infection. He was initially evaluated in the Endocrinology clinic for intense thirst and frequent urination, and subsequently presented to Infectious Disease with systemic symptoms and oral plaques. He was first evaluated 3 months ago after reporting persistent night sweats, unintentional weight loss of 6 pounds, and recurrent oral ulcers. At that time, he also described intermittent low-grade fevers and diffuse fatigue that had developed over the course of several months. During his initial evaluation, his physical exam was notable for mild cervical lymphadenopathy and the presence of oral thrush. No skin rashes or neurologic findings were present. Laboratory testing at that time revealed a reduced CD4 T-cell count, and confirmatory testing was positive for an underlying chronic viral infection. The patient was counseled extensively about his diagnosis, the importance of treatment adherence, and strategies to prevent transmission to others. He was started on combination therapy that includes zidovudine. Since beginning treatment, the patient has been taking his medications daily. He reports improved energy levels and resolution of his night sweats. He has had no new opportunistic infections, and his most recent labs show a stable CD4 count. However, today he notes that he has been feeling somewhat more fatigued than usual over the past few weeks.

PAST MEDICAL HISTORY:
Active Problem List:
- Central diabetes insipidus (inadequately controlled) (diagnosed 2020-01-15)
- HIV-related opportunistic skin infection (diagnosed 2020-04-15)
- HIV infection
- Central diabetes insipidus
- Crohn's disease

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
He does not take any daily medications. He was started on combination therapy that includes zidovudine.

Home Medications:
- Desmopressin acetate 0.2 mg orally twice daily (prescribed, adherence inconsistent)
- Zidovudine-Lamivudine 300/150 mg orally twice daily (prescribed, currently non-adherent)
- Mesalamine delayed-release 800 mg orally three times daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with Crohn's disease
- Maternal cousin with type 1 diabetes insipidus

SOCIAL HISTORY:
He works as a bartender and reports occasional use of recreational drugs but denies injection drug use. He has been sexually active with multiple partners, occasionally without barrier protection. He does not smoke cigarettes and drinks alcohol only on weekends.

REVIEW OF SYSTEMS:
He denies shortness of breath, chest pain, abdominal pain, or bleeding. He does not bruise easily and has not noticed any blood in his stool or urine. He reports no neuropathy, nausea, or muscle pain.

VITAL SIGNS:
His vital signs are normal, and his weight is unchanged from his last visit.

PHYSICAL EXAMINATION:
On examination, the patient is afebrile and appears well nourished. He has no lymphadenopathy or hepatosplenomegaly. His cardiovascular, pulmonary, abdominal, and neurologic examinations are all within normal limits.

LABORATORY DATA:
Basic metabolic testing is unremarkable. A complete blood count shows a new macrocytic anemia.

## Encounter 3: source 2021-01-15, practice 2023-10-17, inpatient, Internal Medicine

Chief complaint: Increased thirst and high urine output developing during hospitalization for high-speed motorcycle collision with significant head trauma.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Aisha Johnson, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Increased thirst and high urine output developing during hospitalization for high-speed motorcycle collision with significant head trauma.

HISTORY OF PRESENT ILLNESS:
The patient is a 30-year-old male with a history of central diabetes insipidus diagnosed in January 2020 and inconsistent adherence to desmopressin acetate, as well as HIV infection and Crohn's disease, who has been hospitalized for the past week following a high-speed motorcycle collision that resulted in significant head trauma. His level of consciousness has gradually improved, and he is no longer intubated. However, nursing staff report that he continues to have unusually high urine output, averaging approximately 800 mL/hour over the past 24 hours, despite receiving IV fluids. The patient is alert but complains of increased thirst.

PAST MEDICAL HISTORY:
Active Problem List:
- Central diabetes insipidus (inadequately controlled) (diagnosed 2020-01-15)
- HIV-related opportunistic skin infection (diagnosed 2020-04-15)
- HIV infection
- Central diabetes insipidus
- Crohn's disease

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Desmopressin acetate 0.2 mg orally twice daily (prescribed, adherence inconsistent)
- Zidovudine-Lamivudine 300/150 mg orally twice daily (prescribed, currently non-adherent)
- Mesalamine delayed-release 800 mg orally three times daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with Crohn's disease
- Maternal cousin with type 1 diabetes insipidus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (despite occupational exposure). Occupation: Bartender.

VITAL SIGNS:
His vital signs are otherwise stable.

PHYSICAL EXAMINATION:
On exam, he appears mildly dehydrated, with dry mucous membranes and orthostatic hypotension. There are no focal neurologic deficits.

IMAGING:
A repeat head CT shows stable intracranial findings.

ASSESSMENT AND PLAN:
The team is concerned about an endocrine complication related to his recent trauma.

## Encounter 4: source 2021-07-15, practice 2024-04-19, inpatient, Gastroenterology

Chief complaint: Persistent, watery diarrhea for three weeks with significant weight loss and crampy abdominal pain in a patient not on antiretroviral therapy.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. David Kim, MD
Department: Gastroenterology
Visit Type: inpatient

CHIEF COMPLAINT:
Persistent, watery diarrhea for three weeks with significant weight loss and crampy abdominal pain in a patient not on antiretroviral therapy.

HISTORY OF PRESENT ILLNESS:
The patient with HIV, central diabetes insipidus, and Crohn's disease presents for evaluation. He reports significant weight loss and occasional crampy abdominal pain. He has not been on antiretroviral therapy and has not seen a healthcare provider in over a year.

PAST MEDICAL HISTORY:
Active Problem List:
- Central diabetes insipidus (inadequately controlled) (diagnosed 2020-01-15)
- HIV-related opportunistic skin infection (diagnosed 2020-04-15)
- HIV infection
- Central diabetes insipidus
- Crohn's disease

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Desmopressin acetate 0.2 mg orally twice daily (prescribed, adherence inconsistent)
- Zidovudine-Lamivudine 300/150 mg orally twice daily (prescribed, currently non-adherent)
- Mesalamine delayed-release 800 mg orally three times daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with Crohn's disease
- Maternal cousin with type 1 diabetes insipidus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (despite occupational exposure). Occupation: Bartender.

VITAL SIGNS:
Temperature 37.8°C, blood pressure 100/60 mmHg, heart rate 102 beats per minute, and respiratory rate 18 breaths per minute

PHYSICAL EXAMINATION:
On examination, he appears cachectic and dehydrated. His abdomen is soft with mild diffuse tenderness but no rebound or guarding.

LABORATORY DATA:
- Hemoglobin: 12.1 g/dL
- White blood cell count: 5,200/mm3
- Platelets: 210,000/mm3
- Sodium: 135 mEq/L
- Potassium: 3.5 mEq/L
- Chloride: 100 mEq/L
- Bicarbonate: 24 mEq/L
- Blood urea nitrogen: 18 mg/dL
- Creatinine: 1.0 mg/dL
- Glucose: 88 mg/dL
- CD4 count: 150/mm3

PLAN:
Stool studies including microscopy are ordered.

## Encounter 5: source 2022-01-15, practice 2024-10-16, icu, Critical Care Medicine

Chief complaint: Agitation and disorientation developing two days after high-speed motor vehicle accident, with excessive water drinking and large volumes of clear urine.

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Emily Patel, MD
Department: Critical Care Medicine
Visit Type: icu

CHIEF COMPLAINT:
Agitation and disorientation developing two days after high-speed motor vehicle accident, with excessive water drinking and large volumes of clear urine.

HISTORY OF PRESENT ILLNESS:
The patient is a 31-year-old male with central diabetes insipidus, previously inadequately controlled after head trauma in January 2021, presenting after a high-speed motor vehicle accident. He was the unrestrained driver and lost consciousness at the scene. In the ICU, he is managed supportively. Over the next two days, he becomes increasingly agitated and disoriented. Nurses report he has been drinking water excessively and producing large volumes of clear urine, sometimes exceeding 7 liters per day.

PAST MEDICAL HISTORY:
Active Problem List:
- Central diabetes insipidus (inadequately controlled) (diagnosed 2020-01-15)
- HIV-related opportunistic skin infection (diagnosed 2020-04-15)
- Cryptosporidiosis (diagnosed 2021-07-15)
- HIV infection
- Central diabetes insipidus
- Crohn's disease

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Desmopressin acetate 0.2 mg orally twice daily (prescribed, adherence inconsistent)
- Zidovudine-Lamivudine 300/150 mg orally twice daily (prescribed, currently non-adherent)
- Mesalamine delayed-release 800 mg orally three times daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with Crohn's disease
- Maternal cousin with type 1 diabetes insipidus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (despite occupational exposure). Occupation: Bartender.

VITAL SIGNS:
Vital signs show a temperature of 38.6°C (101.5°F), heart rate of 112/min, and blood pressure of 106/72 mm Hg.

PHYSICAL EXAMINATION:
Physical examination reveals dry mucous membranes and no focal neurologic deficits.

LABORATORY DATA:
Serum studies show: sodium 154 mEq/L, glucose 92 mg/dL, osmolality 318 mOsm/kg. Urine osmolality remains <100 mOsm/kg despite fluid restriction.

OTHER STUDIES:
A trial of desmopressin rapidly increases his urine osmolality and decreases his urine output.

## Encounter 6: source 2022-07-15, practice 2025-05-03, ed, Emergency Medicine

Chief complaint: Profuse rice-water diarrhea starting two days after returning from humanitarian mission in flood-affected region with untreated river water exposure.

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. James Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Profuse rice-water diarrhea starting two days after returning from humanitarian mission in flood-affected region with untreated river water exposure.

HISTORY OF PRESENT ILLNESS:
The patient is a 31-year-old male with HIV infection, inadequately controlled central diabetes insipidus, and Crohn's disease who presents with profuse rice-water diarrhea that started 2 days ago after returning from a humanitarian mission in a flood-affected region. He reports drinking untreated river water during his stay. His stool is described as "rice water" in appearance.

PAST MEDICAL HISTORY:
Active Problem List:
- Central diabetes insipidus (inadequately controlled) (diagnosed 2020-01-15)
- HIV-related opportunistic skin infection (diagnosed 2020-04-15)
- Cryptosporidiosis (diagnosed 2021-07-15)
- HIV infection
- Central diabetes insipidus
- Crohn's disease

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Desmopressin acetate 0.2 mg orally twice daily (prescribed, adherence inconsistent)
- Zidovudine-Lamivudine 300/150 mg orally twice daily (prescribed, currently non-adherent)
- Mesalamine delayed-release 800 mg orally three times daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with Crohn's disease
- Maternal cousin with type 1 diabetes insipidus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (despite occupational exposure). Occupation: Bartender.

VITAL SIGNS:
Temperature of 37.2°C (99°F), pulse 110/min, blood pressure 90/60 mm Hg

PHYSICAL EXAMINATION:
Dry mucous membranes. On examination, his abdomen is soft but slightly tender, with hyperactive bowel sounds.

LABORATORY DATA:
Laboratory stool studies are pending.

## Encounter 7: source 2023-01-15, practice 2025-10-26, ed, Emergency Medicine

Chief complaint: Worsening crampy abdominal pain and progressive bloating with nonbloody vomiting and no flatus or stool for 24 hours.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Maria Garcia, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening crampy abdominal pain and progressive bloating with nonbloody vomiting and no flatus or stool for 24 hours.

HISTORY OF PRESENT ILLNESS:
The patient is a 32-year-old male with central diabetes insipidus previously exacerbated by traumatic injuries, HIV infection complicated by prior opportunistic gastrointestinal infections and oral candidiasis, and Crohn's disease, who presents to the emergency department with acute abdominal complaints. He reports that abdominal pain began insidiously several days ago and has become more constant today. He reports repeated nonbloody, nonbilious vomiting and has not passed flatus or stool since yesterday afternoon. Over the past few years he has had intermittent loose stools and vague right-lower-quadrant discomfort that waxes and wanes with periods of fatigue and decreased appetite. His weight has fluctuated by several kilograms during recent months, and he occasionally notes small perianal skin breaks that sting but heal without drainage.

PAST MEDICAL HISTORY:
Active Problem List:
- Central diabetes insipidus (inadequately controlled) (diagnosed 2020-01-15)
- HIV-related opportunistic skin infection (diagnosed 2020-04-15)
- Cryptosporidiosis (diagnosed 2021-07-15)
- Cholera (diagnosed 2022-07-15)
- HIV infection
- Central diabetes insipidus
- Crohn's disease

PAST SURGICAL HISTORY:
He denies prior abdominal operations

MEDICATIONS:
Home Medications:
- Desmopressin acetate 0.2 mg orally twice daily (prescribed, adherence inconsistent)
- Zidovudine-Lamivudine 300/150 mg orally twice daily (prescribed, currently non-adherent)
- Mesalamine delayed-release 800 mg orally three times daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father with Crohn's disease
- Maternal cousin with type 1 diabetes insipidus

SOCIAL HISTORY:
He denies opioid use, recent travel, or ingestion of unusual foods.

PHYSICAL EXAMINATION:
On examination he appears volume depleted with a distended, tympanic abdomen that is tender diffusely without rebound or guarding. Bowel sounds are high-pitched and come in bursts separated by quiet intervals. Rectal examination shows an empty vault without masses.

LABORATORY DATA:
Basic laboratory studies reveal mild leukocytosis and iron-deficiency anemia; serum chemistries are otherwise unremarkable.

IMAGING:
Abdominal imaging reveals multiple dilated loops of small bowel with air–fluid levels and a discrete transition point in the right lower quadrant.
