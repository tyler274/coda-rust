# synthetic_hospital patient 2618

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 25 at first encounter, sex M, Non-Hispanic White, University Student Health Insurance Plan.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-10-12 (Mon) | outpatient | Psychiatry |
| 1 | 2021-07-15 | 2022-04-08 (Fri) | outpatient | Obstetrics & Gynecology |
| 2 | 2022-01-15 | 2022-10-12 (Wed) | ed | Emergency Medicine |
| 3 | 2025-01-15 | 2025-10-21 (Tue) | ed | Emergency Medicine |
| 4 | 2025-01-15 | 2025-10-21 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "University Student Health Insurance Plan",
 "occupation": "Graduate student in Electrical Engineering",
 "alcohol_use": "Occasional social use (1-2 drinks per month)",
 "family_history": [
  "Father: Bipolar I Disorder",
  "Maternal uncle: Schizotypal personality traits (undiagnosed)",
  "Mother: Major depressive disorder"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "300 mg PO BID",
   "name": "Lithium carbonate"
  },
  {
   "dose": "100 mg PO daily",
   "name": "Lamotrigine"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Bipolar I Disorder",
  "Schizotypal Personality Disorder"
 ],
 "age_at_first_encounter": 25
}
```

Primary diagnoses: ["Oculogyric crisis due to antipsychotic medication", "Homozygous Normal CFTR Genotype (Daughter)", "Bipolar I Disorder with Psychotic Features", "Schizotypal personality disorder"]
Comorbidities: ["Family history of osteoporosis", "Family History of Stroke"]

## Encounter 0: source 2020-01-15, practice 2020-10-12, outpatient, Psychiatry

Chief complaint: Presents to the clinic with complaints of difficulty in social situations and a long-standing feeling of discomfort with close relationships.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Elena Vostok, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Presents to the clinic with complaints of difficulty in social situations and a long-standing feeling of discomfort with close relationships.

HISTORY OF PRESENT ILLNESS:
He describes himself as a "loner" and prefers to spend time in his own company. He reports that he often perceives situations differently from others, claiming that he can "sense the energy of the universe" and occasionally hears his name being called when no one is around.

PAST MEDICAL HISTORY:
Active Problem List:
- Bipolar I Disorder
- Schizotypal Personality Disorder

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Lithium carbonate 300 mg PO BID
- Lamotrigine 100 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
His family history is notable for a first-degree relative with schizophrenia.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per month). Occupation: Graduate student in Electrical Engineering.

PHYSICAL EXAMINATION:
On examination, the patient appears anxious and has an odd manner of speaking. He avoids eye contact and has an unusual, elaborate style of dress. His thought process is coherent, but he expresses belief in clairvoyance and telepathy. He has no delusions or hallucinations and his insight into his condition is partially preserved.

LABORATORY DATA:
Laboratory values:
- Complete blood count: within normal limits
- Thyroid function tests: within normal limits
- Urine toxicology screen: negative

## Encounter 1: source 2021-07-15, practice 2022-04-08, outpatient, Obstetrics & Gynecology

Chief complaint: Presents for preconception counseling.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Amara Okafor, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Presents for preconception counseling.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizotypal personality disorder (diagnosed 2020-01-15)
- Bipolar I Disorder

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Lithium carbonate 300 mg PO BID
- Lamotrigine 100 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
His family history is notable for two siblings diagnosed with cystic fibrosis.

SOCIAL HISTORY:
He plans to have a child with a woman from the general population who has no known family history of cystic fibrosis.

OTHER STUDIES:
Genetic screening confirms he is healthy with no clinical evidence of cystic fibrosis, but his carrier status is unknown.

ASSESSMENT AND PLAN:
Cystic fibrosis is an autosomal recessive disorder caused by mutations in the CFTR gene, leading to thick, viscous secretions and recurrent pulmonary infections. The carrier frequency for CFTR mutations in the general U.S. population is 1/25.

## Encounter 2: source 2022-01-15, practice 2022-10-12, ed, Emergency Medicine

Chief complaint: Brought to the emergency department by his roommate due to erratic behavior.

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. David Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Brought to the emergency department by his roommate due to erratic behavior.

HISTORY OF PRESENT ILLNESS:
The patient is a 27-year-old male with Schizotypal Personality Disorder diagnosed in January 2020 and Bipolar I Disorder currently managed with lithium carbonate and lamotrigine, who presents with acute behavioral changes. Over the past 3 days, he has been barely sleeping and working nonstop on what he describes as a “world-changing invention.” He has been unusually talkative, jumping between topics, and spent thousands of dollars on gadgets online. He also insists that his professors are watching him through his devices to steal his work.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizotypal personality disorder (diagnosed 2020-01-15)
- Homozygous Normal CFTR Genotype (Daughter) (diagnosed 2021-07-15)
- Bipolar I Disorder

The roommate reports that the patient had a similar episode a year ago and had to be hospitalized.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Lithium carbonate 300 mg PO BID
- Lamotrigine 100 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Bipolar I Disorder
- Maternal uncle: Schizotypal personality traits (undiagnosed)
- Mother: Major depressive disorder

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per month). Occupation: Graduate student in Electrical Engineering.

LABORATORY DATA:
Urine drug screen is negative.

## Encounter 3: source 2025-01-15, practice 2025-10-21, ed, Emergency Medicine

Chief complaint: Brought to the emergency department by his roommate due to escalating behavioral changes over the past 10 days.

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Sarah Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Brought to the emergency department by his roommate due to escalating behavioral changes over the past 10 days.

HISTORY OF PRESENT ILLNESS:
The patient is a 30-year-old male with Bipolar I Disorder and Schizotypal Personality Disorder, on lithium and lamotrigine, returning three years after a prior ED admission for similar symptoms. The roommate says the patient has been sleeping only 1–2 hours per night, is constantly talking, often jumping between topics, and has been impulsively spending large amounts of money online. He recently claimed he has developed a device that allows him to “communicate with satellites,” despite no background in engineering. He believes government agents are now tracking him through his phone.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizotypal personality disorder (diagnosed 2020-01-15)
- Homozygous Normal CFTR Genotype (Daughter) (diagnosed 2021-07-15)
- Bipolar I Disorder with Psychotic Features (diagnosed 2022-01-15)
- Bipolar I Disorder

His medical and psychiatric history is unremarkable.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Lithium carbonate 300 mg PO BID
- Lamotrigine 100 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Bipolar I Disorder
- Maternal uncle: Schizotypal personality traits (undiagnosed)
- Mother: Major depressive disorder

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Occasional social use (1-2 drinks per month). Occupation: Graduate student in Electrical Engineering.

PHYSICAL EXAMINATION:
On examination, the patient is restless and speaking rapidly with tangential thoughts. He displays pressured speech and an irritable affect but is cooperative and oriented.

LABORATORY DATA:
Urine toxicology is negative.

ASSESSMENT AND PLAN:
After admission, his speech remains rapid and mood remains elevated, and his psychotic beliefs persist.

Within 5 days of starting inpatient treatment with mood stabilizers and antipsychotics, his mood and thought process begin to normalize. At 3-week follow-up, the patient is euthymic with no residual psychotic symptoms.

## Encounter 4: source 2025-01-15, practice 2025-10-21, ed, Emergency Medicine

Chief complaint: Brought to the emergency department by his roommate due to sudden onset of severe neck pain, facial tightness, and difficulty moving his eyes.

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. James Patterson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Brought to the emergency department by his roommate due to sudden onset of severe neck pain, facial tightness, and difficulty moving his eyes.

HISTORY OF PRESENT ILLNESS:
The patient is a 30-year-old male with established diagnoses of Bipolar I Disorder and Schizotypal Personality Disorder who returns to the emergency department on the same day as his discharge from a week-long inpatient psychiatric admission for acute mania with psychotic features. Earlier in the day, he had been discharged from the inpatient unit after being prescribed a new medication to help stabilize his mood and prevent recurrence of his symptoms. The patient states that approximately 5 hours after taking his first dose at home, he noticed a “weird pulling” sensation in his jaw and stiffness in the back of his neck. Over the next hour, the pain worsened and his eyes “rolled up” without his control. His roommate became concerned when the patient was unable to straighten his head and appeared increasingly distressed, prompting transport to the hospital. The roommate confirms that the new medication is a high-potency antipsychotic. The patient denies any history of similar symptoms, previous antipsychotic use, recent alcohol or illicit drug use, and head trauma.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizotypal personality disorder (diagnosed 2020-01-15)
- Homozygous Normal CFTR Genotype (Daughter) (diagnosed 2021-07-15)
- Bipolar I Disorder with Psychotic Features (diagnosed 2022-01-15)
- Bipolar I Disorder

During that admission, he had exhibited pressured speech, grandiosity, and disorganized behavior, including attempting to lecture strangers on “unlocking the secrets of the universe” in the middle of the night.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Lithium carbonate 300 mg PO BID
- Lamotrigine 100 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
His family history is significant for schizophrenia in a paternal uncle and depression in his mother.

SOCIAL HISTORY:
The patient is a healthy nonsmoker who occasionally drinks socially but has no history of substance abuse.

REVIEW OF SYSTEMS:
The patient denies fever, confusion, generalized stiffness, or tremors.

VITAL SIGNS:
His vital signs are: temperature 37°C (98.6°F), pulse 82/min, respirations 14/min, and blood pressure 124/78 mm Hg.

PHYSICAL EXAMINATION:
On arrival, he is alert and oriented to person, place, and time but appears anxious. He is diaphoretic and holding his head tilted to one side with his jaw partially clenched. His eyes are fixed in an upward position. Cardiopulmonary examination is normal. Neurologic exam reveals sustained contraction of the sternocleidomastoid and trapezius muscles, with no focal weakness or sensory deficits. There are no signs of muscle rigidity in the limbs, no tremor, and no cogwheel rigidity.

LABORATORY DATA:
Laboratory testing is pending.
