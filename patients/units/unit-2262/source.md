# synthetic_hospital patient 2262

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 70 at first encounter, sex M, Non-Hispanic White, Medicare.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-12-14 (Mon) | outpatient | Neurology |
| 1 | 2020-07-15 | 2021-05-27 (Thu) | outpatient | Gastroenterology |
| 2 | 2021-01-15 | 2021-12-02 (Thu) | outpatient | Neurology |
| 3 | 2021-07-15 | 2022-06-02 (Thu) | outpatient | Urology |
| 4 | 2022-07-15 | 2023-06-06 (Tue) | outpatient | Neurology |
| 5 | 2024-01-15 | 2024-12-07 (Sat) | icu | Emergency Medicine |
| 6 | 2024-07-15 | 2025-06-09 (Mon) | ed | Emergency Medicine |
| 7 | 2025-01-15 | 2025-11-26 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare",
 "occupation": "Retired construction foreman",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Father: Parkinson's disease",
  "Mother: Abdominal aortic aneurysm (repair at age 72)",
  "Brother: Type 2 Diabetes Mellitus",
  "Paternal uncle: Ischemic stroke at age 65"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, quit 15 years ago (20 pack-year history)",
 "home_medications": [
  {
   "dose": "25/100 mg PO TID",
   "name": "Carbidopa/Levodopa"
  },
  {
   "dose": "0.5 mg PO TID",
   "name": "Pramipexole"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Selegiline"
  },
  {
   "dose": "5 mg PO BID",
   "name": "Oxybutynin"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "12.5 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "5 mg PO BID",
   "name": "Apixaban"
  },
  {
   "dose": "81 mg PO daily",
   "name": "Aspirin"
  },
  {
   "dose": "1000 mg PO BID",
   "name": "Metformin"
  }
 ],
 "surgical_history": [
  "Emergency repair of Type A aortic dissection",
  "Sigmoid colectomy with primary anastomosis for volvulus",
  "Exploratory laparotomy with adhesiolysis for paralytic ileus"
 ],
 "chronic_conditions": [
  "Parkinson's disease (7-year duration)",
  "Type 2 Diabetes Mellitus (long-standing, poorly controlled)",
  "Hypertension",
  "Paroxysmal atrial fibrillation",
  "Chronic migraine",
  "Orthostatic hypotension (autonomic dysfunction)",
  "Neurogenic bladder secondary to sacral nerve root injury",
  "Chronic constipation"
 ],
 "age_at_first_encounter": 70
}
```

Primary diagnoses: ["Parkinson disease with psychosis", "Orthostatic hypotension", "Sigmoid volvulus", "Elevated synaptic norepinephrine", "Paralytic ileus", "Type A Aortic Dissection", "Organic constipation (secondary to underlying pathology)", "Neurogenic bladder due to sacral nerve root injury"]
Comorbidities: ["Parkinson disease with psychosis", "Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2020-12-14, outpatient, Neurology

Chief complaint: Progressive difficulty in walking and recent fall at home; wife reports decreased facial expression and softer voice over 6 months.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive difficulty in walking and recent fall at home; wife reports decreased facial expression and softer voice over 6 months.

HISTORY OF PRESENT ILLNESS:
A 68-year-old male presents to the clinic with a 6-month history of progressive difficulty in walking and a recent fall at home. His wife reports that he has been less expressive and his voice has become softer.

PAST MEDICAL HISTORY:
Active Problem List:
- Parkinson's disease (7-year duration)
- Type 2 Diabetes Mellitus (long-standing, poorly controlled)
- Hypertension
- Paroxysmal atrial fibrillation
- Chronic migraine
- Orthostatic hypotension (autonomic dysfunction)
- Neurogenic bladder secondary to sacral nerve root injury
- Chronic constipation

His past medical history is significant for hypertension, controlled with hydrochlorothiazide.

PAST SURGICAL HISTORY:
- Emergency repair of Type A aortic dissection
- Sigmoid colectomy with primary anastomosis for volvulus
- Exploratory laparotomy with adhesiolysis for paralytic ileus

MEDICATIONS:
His current medications include only hydrochlorothiazide and a daily multivitamin.

Home Medications:
- Carbidopa/Levodopa 25/100 mg PO TID
- Pramipexole 0.5 mg PO TID
- Selegiline 5 mg PO daily
- Oxybutynin 5 mg PO BID
- Lisinopril 20 mg PO daily
- Apixaban 5 mg PO BID
- Aspirin 81 mg PO daily
- Metformin 1000 mg PO BID

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Parkinson's disease
- Mother: Abdominal aortic aneurysm (repair at age 72)
- Brother: Type 2 Diabetes Mellitus
- Paternal uncle: Ischemic stroke at age 65

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (20 pack-year history). Alcohol: Social (1-2 drinks per week). Occupation: Retired construction foreman.

PHYSICAL EXAMINATION:
On examination, the patient has a stooped posture and a shuffling gait, and appears to be chasing his center of gravity. He also exhibits a pill-rolling tremor at rest and increased muscle rigidity in his upper limbs. His neurological examination reveals bradykinesia and a lack of arm swing while walking. There are no signs of cerebellar dysfunction, and his sensation is intact.

LABORATORY DATA:
His most recent laboratory tests are as follows:

- Complete Blood Count (CBC): Within normal limits
- Comprehensive Metabolic Panel (CMP): Within normal limits
- Thyroid Function Tests: Within normal limits
- Vitamin B12 Levels: Within normal limits

## Encounter 1: source 2020-07-15, practice 2021-05-27, outpatient, Gastroenterology

Chief complaint: Abdominal discomfort and infrequent bowel movements with bloating over the past month.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Abdominal discomfort and infrequent bowel movements with bloating over the past month.

HISTORY OF PRESENT ILLNESS:
The patient is a 70-year-old male with Parkinson's disease, autonomic dysfunction, and chronic constipation who presents with worsening gastrointestinal symptoms. He was last evaluated in Neurology six months ago for disease progression with psychosis. He reports that for the past month, he has had difficulty passing stool with only one bowel movement every 3 to 4 days. He often feels bloated and notes a sensation of incomplete evacuation after defecating. He denies blood in the stool, nausea, vomiting, or weight loss. His appetite is normal, and he has not made any recent dietary or travel changes. He drinks adequate fluids and eats vegetables daily. He denies overuse of laxatives or opioids.

PAST MEDICAL HISTORY:
Active Problem List:
- Parkinson disease with psychosis (diagnosed 2020-01-15)
- Parkinson's disease (7-year duration)
- Type 2 Diabetes Mellitus (long-standing, poorly controlled)
- Hypertension
- Paroxysmal atrial fibrillation
- Chronic migraine
- Orthostatic hypotension (autonomic dysfunction)
- Neurogenic bladder secondary to sacral nerve root injury
- Chronic constipation

His medical history includes hypertension, paroxysmal atrial fibrillation, and chronic migraine.

PAST SURGICAL HISTORY:
- Emergency repair of Type A aortic dissection
- Sigmoid colectomy with primary anastomosis for volvulus
- Exploratory laparotomy with adhesiolysis for paralytic ileus

MEDICATIONS:
Medications include lisinopril, low-dose aspirin, and another cardiovascular-related drug he recently started but can't remember the name of.

Home Medications:
- Carbidopa/Levodopa 25/100 mg PO TID
- Pramipexole 0.5 mg PO TID
- Selegiline 5 mg PO daily
- Oxybutynin 5 mg PO BID
- Hydrochlorothiazide 12.5 mg PO daily
- Apixaban 5 mg PO BID
- Metformin 1000 mg PO BID

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Parkinson's disease
- Mother: Abdominal aortic aneurysm (repair at age 72)
- Brother: Type 2 Diabetes Mellitus
- Paternal uncle: Ischemic stroke at age 65

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (20 pack-year history). Alcohol: Social (1-2 drinks per week). Occupation: Retired construction foreman.

PHYSICAL EXAMINATION:
His abdomen is soft and mildly distended, with no tenderness or masses. Rectal examination reveals normal tone and no fissures or blood.

LABORATORY DATA:
A basic metabolic panel and thyroid studies are unremarkable.

OTHER STUDIES:
Stool guaiac is negative.
Colonoscopy performed 2 years ago was normal.

PLAN:
He is advised to try increasing dietary fiber, but symptoms persist.

## Encounter 2: source 2021-01-15, practice 2021-12-02, outpatient, Neurology

Chief complaint: Recurrent episodes of lightheadedness, particularly when standing from a sitting position, increasing over past months.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Recurrent episodes of lightheadedness, particularly when standing from a sitting position, increasing over past months.

HISTORY OF PRESENT ILLNESS:
The patient with Parkinson's disease returns to neurology clinic one year after his prior evaluation. He reports these episodes are increasing in frequency over the past few months. He denies chest pain, palpitations, or loss of consciousness.

PAST MEDICAL HISTORY:
Active Problem List:
- Parkinson disease with psychosis (diagnosed 2020-01-15)
- Organic constipation (secondary to underlying pathology) (diagnosed 2020-07-15)
- Parkinson's disease (7-year duration)
- Type 2 Diabetes Mellitus (long-standing, poorly controlled)
- Hypertension
- Paroxysmal atrial fibrillation
- Chronic migraine
- Orthostatic hypotension (autonomic dysfunction)
- Neurogenic bladder secondary to sacral nerve root injury
- Chronic constipation

PAST SURGICAL HISTORY:
- Emergency repair of Type A aortic dissection
- Sigmoid colectomy with primary anastomosis for volvulus
- Exploratory laparotomy with adhesiolysis for paralytic ileus

MEDICATIONS:
His medications include carbidopa/levodopa and selegiline.

Home Medications:
- Pramipexole 0.5 mg PO TID
- Oxybutynin 5 mg PO BID
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Apixaban 5 mg PO BID
- Aspirin 81 mg PO daily
- Metformin 1000 mg PO BID

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Parkinson's disease
- Mother: Abdominal aortic aneurysm (repair at age 72)
- Brother: Type 2 Diabetes Mellitus
- Paternal uncle: Ischemic stroke at age 65

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (20 pack-year history). Alcohol: Social (1-2 drinks per week). Occupation: Retired construction foreman.

VITAL SIGNS:
On physical examination, his blood pressure is 145/85 mm Hg while seated and 120/70 mm Hg upon standing. His heart rate remains at 78 beats per minute in both positions.

PHYSICAL EXAMINATION:
Neurological examination is consistent with mild Parkinsonian features without acute changes.

## Encounter 3: source 2021-07-15, practice 2022-06-02, outpatient, Urology

Chief complaint: Persistent urinary issues with difficulty initiating stream, slow interrupted flow, and sensation of incomplete emptying worsening over the past year.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Priya Patel, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent urinary issues with difficulty initiating stream, slow interrupted flow, and sensation of incomplete emptying worsening over the past year.

HISTORY OF PRESENT ILLNESS:
This 71-year-old male with Parkinson's disease and associated autonomic dysfunction, including orthostatic hypotension and chronic constipation, returns for follow-up of persistent urinary dysfunction. He reports that over the past year, he often feels the need to urinate but has difficulty initiating the stream. When able to void, the urine flow is slow and intermittently interrupted, and he frequently feels he has not fully emptied his bladder. Over the last several months, he has begun noticing dampness in his underwear and occasional episodes of urine leakage, especially at night, though he is often unaware of when the leakage occurs. He denies burning with urination, blood in the urine, or urgency. He has also had episodes of numbness and tingling in his feet and recently began using a cane due to unsteadiness while walking.

PAST MEDICAL HISTORY:
Active Problem List:
- Parkinson disease with psychosis (diagnosed 2020-01-15)
- Organic constipation (secondary to underlying pathology) (diagnosed 2020-07-15)
- Orthostatic hypotension (diagnosed 2021-01-15)
- Parkinson's disease (7-year duration)
- Type 2 Diabetes Mellitus (long-standing, poorly controlled)
- Hypertension
- Paroxysmal atrial fibrillation
- Chronic migraine
- Orthostatic hypotension (autonomic dysfunction)
- Neurogenic bladder secondary to sacral nerve root injury
- Chronic constipation

His past medical history is significant for hypertension and long-standing poorly controlled type 2 diabetes mellitus.

PAST SURGICAL HISTORY:
- Emergency repair of Type A aortic dissection
- Sigmoid colectomy with primary anastomosis for volvulus
- Exploratory laparotomy with adhesiolysis for paralytic ileus

MEDICATIONS:
Home Medications:
- Carbidopa/Levodopa 25/100 mg PO TID
- Pramipexole 0.5 mg PO TID
- Selegiline 5 mg PO daily
- Oxybutynin 5 mg PO BID
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Apixaban 5 mg PO BID
- Aspirin 81 mg PO daily
- Metformin 1000 mg PO BID

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Parkinson's disease
- Mother: Abdominal aortic aneurysm (repair at age 72)
- Brother: Type 2 Diabetes Mellitus
- Paternal uncle: Ischemic stroke at age 65

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (20 pack-year history). Alcohol: Social (1-2 drinks per week). Occupation: Retired construction foreman.

PHYSICAL EXAMINATION:
On physical examination, he has reduced sensation to light touch and vibration in both feet and absent ankle reflexes. Palpation of the lower abdomen reveals a firm, distended, non-tender bladder.

OTHER STUDIES:
After voiding in the clinic, a post-void residual scan shows 480 mL of retained urine.

## Encounter 4: source 2022-07-15, practice 2023-06-06, outpatient, Neurology

Chief complaint: Persistent bradykinesia, mild tremor at rest, and rigidity despite optimized levodopa therapy; interested in clinical trial enrollment.

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. David Park, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent bradykinesia, mild tremor at rest, and rigidity despite optimized levodopa therapy; interested in clinical trial enrollment.

HISTORY OF PRESENT ILLNESS:
The patient is a 72-year-old male with Parkinson's disease and autonomic dysfunction who returns for follow-up of motor symptoms. Previously evaluated for orthostatic hypotension and neurogenic bladder, he is now enrolled in a clinical trial evaluating cognitive and motor effects of novel adjunct therapy. Despite optimized levodopa/carbidopa and dopamine agonist treatment, he reports persistent bradykinesia, mild tremor at rest, and rigidity. He also describes orthostatic lightheadedness and daytime fatigue. After 4 weeks of therapy, he reports increased energy, improved concentration, and better mobility.

PAST MEDICAL HISTORY:
Active Problem List:
- Parkinson disease with psychosis (diagnosed 2020-01-15)
- Organic constipation (secondary to underlying pathology) (diagnosed 2020-07-15)
- Orthostatic hypotension (diagnosed 2021-01-15)
- Neurogenic bladder due to sacral nerve root injury (diagnosed 2021-07-15)
- Parkinson's disease (7-year duration)
- Type 2 Diabetes Mellitus (long-standing, poorly controlled)
- Hypertension
- Paroxysmal atrial fibrillation
- Chronic migraine
- Orthostatic hypotension (autonomic dysfunction)
- Neurogenic bladder secondary to sacral nerve root injury
- Chronic constipation

PAST SURGICAL HISTORY:
- Emergency repair of Type A aortic dissection
- Sigmoid colectomy with primary anastomosis for volvulus
- Exploratory laparotomy with adhesiolysis for paralytic ileus

MEDICATIONS:
He takes levodopa/carbidopa and a dopamine agonist.

Home Medications:
- Carbidopa/Levodopa 25/100 mg PO TID
- Pramipexole 0.5 mg PO TID
- Selegiline 5 mg PO daily
- Oxybutynin 5 mg PO BID
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Apixaban 5 mg PO BID
- Aspirin 81 mg PO daily
- Metformin 1000 mg PO BID

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Parkinson's disease
- Mother: Abdominal aortic aneurysm (repair at age 72)
- Brother: Type 2 Diabetes Mellitus
- Paternal uncle: Ischemic stroke at age 65

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (20 pack-year history). Alcohol: Social (1-2 drinks per week). Occupation: Retired construction foreman.

LABORATORY DATA:
Cerebrospinal fluid analysis reveals reduced levels of 3-methoxy-4-hydroxyphenylglycol.

OTHER STUDIES:
The study drug, originally derived from a hydrazine-based compound, was shown in preliminary trials to reduce enzymatic breakdown of monoamines within presynaptic nerve terminals of the locus coeruleus and dorsal raphe nuclei.

## Encounter 5: source 2024-01-15, practice 2024-12-07, icu, Emergency Medicine

Chief complaint: Sudden onset of severe tearing chest pain radiating to the back after lifting heavy furniture with brief episode of syncope.

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Sudden onset of severe tearing chest pain radiating to the back after lifting heavy furniture with brief episode of syncope.

HISTORY OF PRESENT ILLNESS:
74-year-old male with seven-year history of Parkinson disease and autonomic dysfunction presents with acute chest pain. Pain is described as tearing in nature and began while the patient was lifting heavy furniture. He also reports a brief episode of syncope immediately after the pain started.

PAST MEDICAL HISTORY:
Active Problem List:
- Parkinson disease with psychosis (diagnosed 2020-01-15)
- Organic constipation (secondary to underlying pathology) (diagnosed 2020-07-15)
- Orthostatic hypotension (diagnosed 2021-01-15)
- Neurogenic bladder due to sacral nerve root injury (diagnosed 2021-07-15)
- Elevated synaptic norepinephrine (diagnosed 2022-07-15)
- Parkinson's disease (7-year duration)
- Type 2 Diabetes Mellitus (long-standing, poorly controlled)
- Hypertension
- Paroxysmal atrial fibrillation
- Chronic migraine
- Orthostatic hypotension (autonomic dysfunction)
- Neurogenic bladder secondary to sacral nerve root injury
- Chronic constipation

PAST SURGICAL HISTORY:
- Emergency repair of Type A aortic dissection
- Sigmoid colectomy with primary anastomosis for volvulus
- Exploratory laparotomy with adhesiolysis for paralytic ileus

MEDICATIONS:
Home Medications:
- Carbidopa/Levodopa 25/100 mg PO TID
- Pramipexole 0.5 mg PO TID
- Selegiline 5 mg PO daily
- Oxybutynin 5 mg PO BID
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Apixaban 5 mg PO BID
- Aspirin 81 mg PO daily
- Metformin 1000 mg PO BID

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Parkinson's disease
- Mother: Abdominal aortic aneurysm (repair at age 72)
- Brother: Type 2 Diabetes Mellitus
- Paternal uncle: Ischemic stroke at age 65

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (20 pack-year history). Alcohol: Social (1-2 drinks per week). Occupation: Retired construction foreman.

VITAL SIGNS:
His blood pressure is 180/95 mmHg in the right arm and 165/90 mmHg in the left arm.

PHYSICAL EXAMINATION:
Physical examination reveals a diastolic murmur best heard over the right second intercostal space and a weak femoral pulse compared to the radial pulse.

LABORATORY DATA:
Laboratory values are as follows:

- Hemoglobin: 14.2 g/dL
- White blood cell count: 11,000/uL
- Platelets: 250,000/uL
- Serum creatinine: 1.1 mg/dL
- Troponin I: <0.04 ng/mL (normal)

IMAGING:
A chest X-ray reveals a widened mediastinum.

OTHER STUDIES:
His electrocardiogram shows no evidence of ischemia or infarction.

## Encounter 6: source 2024-07-15, practice 2025-06-09, ed, Emergency Medicine

Chief complaint: Worsening abdominal pain and distension over 2 days with history of similar episodes.

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Elena Vasquez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening abdominal pain and distension over 2 days with history of similar episodes.

HISTORY OF PRESENT ILLNESS:
The patient, with Parkinson disease, autonomic dysfunction, chronic constipation, and recent Type A Aortic Dissection, presents to the emergency department with a 2-day history of worsening abdominal pain and distension. He reports having had similar, though less severe, episodes in the past which resolved spontaneously.

PAST MEDICAL HISTORY:
Active Problem List:
- Parkinson disease with psychosis (diagnosed 2020-01-15)
- Organic constipation (secondary to underlying pathology) (diagnosed 2020-07-15)
- Orthostatic hypotension (diagnosed 2021-01-15)
- Neurogenic bladder due to sacral nerve root injury (diagnosed 2021-07-15)
- Elevated synaptic norepinephrine (diagnosed 2022-07-15)
- Type A Aortic Dissection (diagnosed 2024-01-15)
- Parkinson's disease (7-year duration)
- Type 2 Diabetes Mellitus (long-standing, poorly controlled)
- Hypertension
- Paroxysmal atrial fibrillation
- Chronic migraine
- Orthostatic hypotension (autonomic dysfunction)
- Neurogenic bladder secondary to sacral nerve root injury
- Chronic constipation

His medical history is significant for chronic constipation and Parkinson&#x27;s disease.

PAST SURGICAL HISTORY:
- Emergency repair of Type A aortic dissection
- Sigmoid colectomy with primary anastomosis for volvulus
- Exploratory laparotomy with adhesiolysis for paralytic ileus

MEDICATIONS:
Home Medications:
- Carbidopa/Levodopa 25/100 mg PO TID
- Pramipexole 0.5 mg PO TID
- Selegiline 5 mg PO daily
- Oxybutynin 5 mg PO BID
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Apixaban 5 mg PO BID
- Aspirin 81 mg PO daily
- Metformin 1000 mg PO BID

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Parkinson's disease
- Mother: Abdominal aortic aneurysm (repair at age 72)
- Brother: Type 2 Diabetes Mellitus
- Paternal uncle: Ischemic stroke at age 65

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (20 pack-year history). Alcohol: Social (1-2 drinks per week). Occupation: Retired construction foreman.

PHYSICAL EXAMINATION:
On examination, his abdomen is distended and tympanic to percussion, with tenderness in the lower quadrants but no rebound or guarding. Bowel sounds are high-pitched and hyperactive. Rectal examination reveals an empty rectum.

IMAGING:
An abdominal x-ray shows a &#x27;coffee bean&#x27;-shaped dilated loop of colon in the left upper quadrant.

## Encounter 7: source 2025-01-15, practice 2025-11-26, ed, Emergency Medicine

Chief complaint: Abdominal distension and pain with no bowel movements or passage of gas for 48 hours.

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Marcus Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Abdominal distension and pain with no bowel movements or passage of gas for 48 hours.

HISTORY OF PRESENT ILLNESS:
The patient with Parkinson's disease and prior sigmoid volvulus returns to the emergency department with a 3-day history of abdominal distension and pain. He reports no bowel movements or passage of gas for the past 48 hours.

PAST MEDICAL HISTORY:
Active Problem List:
- Parkinson disease with psychosis (diagnosed 2020-01-15)
- Organic constipation (secondary to underlying pathology) (diagnosed 2020-07-15)
- Orthostatic hypotension (diagnosed 2021-01-15)
- Neurogenic bladder due to sacral nerve root injury (diagnosed 2021-07-15)
- Elevated synaptic norepinephrine (diagnosed 2022-07-15)
- Type A Aortic Dissection (diagnosed 2024-01-15)
- Sigmoid volvulus (diagnosed 2024-07-15)
- Parkinson's disease (7-year duration)
- Type 2 Diabetes Mellitus (long-standing, poorly controlled)
- Hypertension
- Paroxysmal atrial fibrillation
- Chronic migraine
- Orthostatic hypotension (autonomic dysfunction)
- Neurogenic bladder secondary to sacral nerve root injury
- Chronic constipation

PAST SURGICAL HISTORY:
Recent hip replacement surgery

MEDICATIONS:
His medications include carbidopa/levodopa and oxybutynin.

Home Medications:
- Pramipexole 0.5 mg PO TID
- Selegiline 5 mg PO daily
- Lisinopril 20 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Apixaban 5 mg PO BID
- Aspirin 81 mg PO daily
- Metformin 1000 mg PO BID

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Parkinson's disease
- Mother: Abdominal aortic aneurysm (repair at age 72)
- Brother: Type 2 Diabetes Mellitus
- Paternal uncle: Ischemic stroke at age 65

SOCIAL HISTORY:
Smoking: Former smoker, quit 15 years ago (20 pack-year history). Alcohol: Social (1-2 drinks per week). Occupation: Retired construction foreman.

PHYSICAL EXAMINATION:
On examination, his abdomen is markedly distended, tympanic to percussion, and diffusely tender without rebound or guarding. Bowel sounds are hypoactive. Rectal examination reveals an empty rectal vault.

LABORATORY DATA:
Laboratory results are as follows:

- Sodium: 140 mEq/L
- Potassium: 3.2 mEq/L
- Chloride: 100 mEq/L
- Bicarbonate: 28 mEq/L
- Blood urea nitrogen: 18 mg/dL
- Creatinine: 1.0 mg/dL
- Magnesium: 1.6 mg/dL
- Calcium: 9.2 mg/dL

IMAGING:
A CT scan of the abdomen and pelvis shows marked colonic dilation up to 10 cm in the ascending and transverse colon with no evidence of mechanical obstruction.
