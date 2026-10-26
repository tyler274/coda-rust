# synthetic_hospital patient 1740

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 30 at first encounter, sex F, Caucasian (Non-Hispanic White), Blue Cross Blue Shield PPO (employer-sponsored).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-12-29 (Thu) | outpatient | Neurology |
| 1 | 2021-01-15 | 2024-01-01 (Mon) | outpatient | Neurology |
| 2 | 2021-07-15 | 2024-07-01 (Mon) | outpatient | Neurology |
| 3 | 2022-01-15 | 2025-01-06 (Mon) | outpatient | Urology |
| 4 | 2022-04-15 | 2025-04-03 (Thu) | outpatient | Internal Medicine |
| 5 | 2022-07-15 | 2025-07-02 (Wed) | ed | Emergency Medicine |
| 6 | 2023-01-15 | 2025-12-29 (Mon) | ed | Emergency Medicine |
| 7 | 2023-07-15 | 2026-06-16 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Blue Cross Blue Shield PPO (employer-sponsored)",
 "occupation": "Financial Analyst",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Mother: Autoimmune hypothyroidism (Hashimoto's thyroiditis)",
  "Father: Essential hypertension",
  "No known first-degree relatives with multiple sclerosis"
 ],
 "race_ethnicity": "Caucasian (Non-Hispanic White)",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "44 mcg subcutaneous injection three times weekly",
   "name": "Interferon beta-1a"
  },
  {
   "dose": "5 mg oral twice daily as needed for urinary urgency",
   "name": "Oxybutynin"
  },
  {
   "dose": "2000 IU oral daily",
   "name": "Vitamin D3"
  }
 ],
 "surgical_history": [
  "No prior surgical procedures"
 ],
 "chronic_conditions": [
  "Relapsing-remitting multiple sclerosis (diagnosed age 27)",
  "Neurogenic bladder with detrusor overactivity",
  "History of recurrent optic neuritis (bilateral)"
 ],
 "age_at_first_encounter": 30
}
```

Primary diagnoses: ["Right optic neuropathy", "Multiple sclerosis", "Urinary incontinence", "Uncomplicated cystitis"]
Comorbidities: ["Right optic neuropathy", "Multiple sclerosis", "Upper motor neuron syndrome", "History of Rheumatic Fever"]

## Encounter 0: source 2020-01-15, practice 2022-12-29, outpatient, Neurology

Chief complaint: Intermittent numbness and tingling in left arm and leg, and episodes of blurred vision in right eye that resolve spontaneously.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent numbness and tingling in left arm and leg, and episodes of blurred vision in right eye that resolve spontaneously.

HISTORY OF PRESENT ILLNESS:
A 32-year-old Caucasian woman presents to the clinic with a 3-month history of intermittent numbness and tingling in her left arm and leg. She also reports episodes of blurred vision in her right eye that seem to resolve spontaneously.

PAST MEDICAL HISTORY:
Active Problem List:
- Relapsing-remitting multiple sclerosis (diagnosed age 27)
- Neurogenic bladder with detrusor overactivity
- History of recurrent optic neuritis (bilateral)

PAST SURGICAL HISTORY:
- No prior surgical procedures

MEDICATIONS:
Home Medications:
- Interferon beta-1a 44 mcg subcutaneous injection three times weekly
- Oxybutynin 5 mg oral twice daily as needed for urinary urgency
- Vitamin D3 2000 IU oral daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Autoimmune hypothyroidism (Hashimoto's thyroiditis)
- Father: Essential hypertension
- No known first-degree relatives with multiple sclerosis

SOCIAL HISTORY:
She does not smoke. She moved to the northern United States from the south two years ago.

PHYSICAL EXAMINATION:
Afferent pupillary defect in the right eye and hyperreflexia in the left upper and lower extremities

LABORATORY DATA:
Vitamin D level: 15 ng/mL (Normal: 30-100 ng/mL)
- Complete Blood Count: Normal
- Erythrocyte Sedimentation Rate: Normal
- C-Reactive Protein: Normal
- Antinuclear Antibodies: Negative

IMAGING:
MRI of the brain with and without contrast shows multiple periventricular white matter lesions

## Encounter 1: source 2021-01-15, practice 2024-01-01, outpatient, Neurology

Chief complaint: Blurred vision in left eye and electric shock sensation radiating down spine when bending neck forward.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Blurred vision in left eye and electric shock sensation radiating down spine when bending neck forward.

HISTORY OF PRESENT ILLNESS:
The patient with RRMS and optic neuritis returns to clinic one year after initial presentation with left-sided numbness and right eye blurred vision and presents to clinic with a 2-day history of blurred vision in her left eye and a sensation of electric shock radiating down her spine when she bends her neck forward. She also reports intermittent numbness and tingling in her right hand over the past month.

PAST MEDICAL HISTORY:
Active Problem List:
- Multiple sclerosis (diagnosed 2020-01-15)
- Relapsing-remitting multiple sclerosis (diagnosed age 27)
- Neurogenic bladder with detrusor overactivity
- History of recurrent optic neuritis (bilateral)

PAST SURGICAL HISTORY:
- No prior surgical procedures

MEDICATIONS:
Home Medications:
- Interferon beta-1a 44 mcg subcutaneous injection three times weekly
- Oxybutynin 5 mg oral twice daily as needed for urinary urgency
- Vitamin D3 2000 IU oral daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Autoimmune hypothyroidism (Hashimoto's thyroiditis)
- Father: Essential hypertension
- No known first-degree relatives with multiple sclerosis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Financial Analyst.

PHYSICAL EXAMINATION:
On examination, her left eye has an afferent pupillary defect, and there is decreased sensation to light touch in her right hand. Visual acuity is reduced in the left eye. The remainder of the neurologic examination is normal.

LABORATORY DATA:
The patient's most recent laboratory results are as follows:

- Complete Blood Count: Normal
- Erythrocyte Sedimentation Rate: Normal
- Vitamin B12 Level: Normal
- Rapid Plasma Reagin: Non-reactive
- Antinuclear Antibody: Negative

IMAGING:
Brain MRI with and without contrast is performed and reveals multiple periventricular white matter lesions, some of which enhance with gadolinium.

ASSESSMENT AND PLAN:
Based on the clinical presentation and test results,

## Encounter 2: source 2021-07-15, practice 2024-07-01, outpatient, Neurology

Chief complaint: Several weeks of worsening right eye pain and blurry vision with color desaturation.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Robert Kim, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Several weeks of worsening right eye pain and blurry vision with color desaturation.

HISTORY OF PRESENT ILLNESS:
The patient returns for evaluation of recurrent visual symptoms following prior bilateral optic neuritis and previous blurred vision episodes in both eyes. She reports pain that worsens with eye movement and some color desaturation.

PAST MEDICAL HISTORY:
Active Problem List:
- Multiple sclerosis (diagnosed 2020-01-15)
- Relapsing-remitting multiple sclerosis (diagnosed age 27)
- Neurogenic bladder with detrusor overactivity
- History of recurrent optic neuritis (bilateral)

PAST SURGICAL HISTORY:
- No prior surgical procedures

MEDICATIONS:
Home Medications:
- Interferon beta-1a 44 mcg subcutaneous injection three times weekly
- Oxybutynin 5 mg oral twice daily as needed for urinary urgency
- Vitamin D3 2000 IU oral daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Autoimmune hypothyroidism (Hashimoto's thyroiditis)
- Father: Essential hypertension
- No known first-degree relatives with multiple sclerosis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Financial Analyst.

PHYSICAL EXAMINATION:
Neurologic exam reveals decreased visual acuity and a relative afferent pupillary defect in the right eye.

IMAGING:
MRI of the brain shows multiple periventricular white matter lesions oriented perpendicularly to the ventricles, along with T2 hyperintensities in the corpus callosum.

PATHOLOGY:
Biopsy of a lesion would show infiltration by lymphocytes and macrophages with reactive astrocytes.

## Encounter 3: source 2022-01-15, practice 2025-01-06, outpatient, Urology

Chief complaint: Episodes of sudden urinary leakage and urgency that have become more noticeable, causing hesitation to leave home.

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Maria Gonzalez, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
Episodes of sudden urinary leakage and urgency that have become more noticeable, causing hesitation to leave home.

HISTORY OF PRESENT ILLNESS:
The patient returns for evaluation of worsening urinary symptoms in the context of neurogenic bladder and relapsing-remitting multiple sclerosis previously managed by Neurology. She describes an unpredictable loss of urine that tends to occur suddenly and without much warning, sometimes even before she can reach the bathroom. She also notes an uncomfortable sensation that builds quickly and compels her to void immediately. These symptoms have made her hesitant to leave home for extended periods.

PAST MEDICAL HISTORY:
Active Problem List:
- Multiple sclerosis (diagnosed 2020-01-15)
- Relapsing-remitting multiple sclerosis (diagnosed age 27)
- Neurogenic bladder with detrusor overactivity
- History of recurrent optic neuritis (bilateral)

Her past medical history includes several episodes over the past year of visual disturbance and weakness affecting different limbs at different times, each resolving over days. She was evaluated by a neurologist and underwent imaging that showed scattered lesions in the brain and spinal cord.

PAST SURGICAL HISTORY:
- No prior surgical procedures

MEDICATIONS:
Home Medications:
- Interferon beta-1a 44 mcg subcutaneous injection three times weekly
- Oxybutynin 5 mg oral twice daily as needed for urinary urgency
- Vitamin D3 2000 IU oral daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Autoimmune hypothyroidism (Hashimoto's thyroiditis)
- Father: Essential hypertension
- No known first-degree relatives with multiple sclerosis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Financial Analyst.

PHYSICAL EXAMINATION:
On examination, she has increased tone in the lower extremities, brisk reflexes, and reduced proprioception in the feet. No abdominal or pelvic masses are noted.

## Encounter 4: source 2022-04-15, practice 2025-04-03, outpatient, Internal Medicine

Chief complaint: Fever, chills, and dysuria with increased urinary frequency over the past week.

ENCOUNTER NOTE
Date: 2022-04-15
Provider: Dr. Priya Sharma, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Fever, chills, and dysuria with increased urinary frequency over the past week.

HISTORY OF PRESENT ILLNESS:
The patient is a 32-year-old woman with relapsing-remitting multiple sclerosis since age 27 and neurogenic bladder with detrusor overactivity who returns for evaluation following her urology visit three months prior. She presents to the outpatient clinic with a 2-day history of fever, chills, and dysuria. She reports that she has been experiencing increased urinary frequency and urgency for the past week. She has a known neurogenic bladder due to her multiple sclerosis and has been managing her urinary symptoms with clean intermittent catheterization for the past year. Her last urinary tract infection was three months ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Multiple sclerosis (diagnosed 2020-01-15)
- Urinary incontinence (diagnosed 2022-01-15)
- Relapsing-remitting multiple sclerosis (diagnosed age 27)
- Neurogenic bladder with detrusor overactivity
- History of recurrent optic neuritis (bilateral)

PAST SURGICAL HISTORY:
- No prior surgical procedures

MEDICATIONS:
Home Medications:
- Interferon beta-1a 44 mcg subcutaneous injection three times weekly
- Oxybutynin 5 mg oral twice daily as needed for urinary urgency
- Vitamin D3 2000 IU oral daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Autoimmune hypothyroidism (Hashimoto's thyroiditis)
- Father: Essential hypertension
- No known first-degree relatives with multiple sclerosis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Financial Analyst.

VITAL SIGNS:
Temperature is 38.5°C (101.3°F), blood pressure is 120/78 mmHg, and heart rate is 88 beats per minute

PHYSICAL EXAMINATION:
On physical examination, her abdomen is soft and non-tender. There is no costovertebral angle tenderness.

LABORATORY DATA:
White blood cell count: 12,000/mm³
Hemoglobin: 13.2 g/dL
Platelets: 250,000/mm³
Urinalysis:
Leukocyte esterase: Positive
Nitrites: Positive
WBC: >50/high-power field
RBC: 2/high-power field
Bacteria: Many Gram-negative rods

## Encounter 5: source 2022-07-15, practice 2025-07-02, ed, Emergency Medicine

Chief complaint: Intermittent visual blurring in right eye, involuntary muscle spasms of left hand, and urinary urgency with frequency.

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Emily Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Intermittent visual blurring in right eye, involuntary muscle spasms of left hand, and urinary urgency with frequency.

HISTORY OF PRESENT ILLNESS:
The patient with relapsing-remitting multiple sclerosis, recurrent optic neuritis, and neurogenic bladder returns with new visual and urinary symptoms similar to prior relapses. These started 10 days ago with sudden, sharp pain behind her right eye followed by difficulty focusing, especially in low light. She also notes frequent, urgent restroom trips, totaling over 10 urinations daily, with occasional loss of urinary control.

PAST MEDICAL HISTORY:
Active Problem List:
- Multiple sclerosis (diagnosed 2020-01-15)
- Urinary incontinence (diagnosed 2022-01-15)
- Uncomplicated cystitis (diagnosed 2022-04-15)
- Relapsing-remitting multiple sclerosis (diagnosed age 27)
- Neurogenic bladder with detrusor overactivity
- History of recurrent optic neuritis (bilateral)

PAST SURGICAL HISTORY:
- No prior surgical procedures

MEDICATIONS:
Home Medications:
- Interferon beta-1a 44 mcg subcutaneous injection three times weekly
- Oxybutynin 5 mg oral twice daily as needed for urinary urgency
- Vitamin D3 2000 IU oral daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
Family history is negative for neurodegenerative disorders, but her mother has systemic lupus erythematosus.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Financial Analyst.

PHYSICAL EXAMINATION:
Neurologic examination demonstrates hyperreflexia and clonus in the left upper extremity, along with decreased sensation to vibration in the right lower limb. Cranial nerve assessment reveals a subtle afferent pupillary defect on the right.
Funduscopic exam is notable for mild optic disc swelling.

IMAGING:
MRI of the brain shows multiple hyperintense lesions in the periventricular white matter.

## Encounter 6: source 2023-01-15, practice 2025-12-29, ed, Emergency Medicine

Chief complaint: Sudden onset right leg weakness and numbness, with blurred vision in left eye that started one week ago.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. David Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset right leg weakness and numbness, with blurred vision in left eye that started one week ago.

HISTORY OF PRESENT ILLNESS:
This 33-year-old female with relapsing-remitting multiple sclerosis and recurrent bilateral optic neuritis returns to the emergency department with a 2-day history of sudden onset right leg weakness and numbness. She also reports blurred vision in her left eye that started a week ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Multiple sclerosis (diagnosed 2020-01-15)
- Urinary incontinence (diagnosed 2022-01-15)
- Uncomplicated cystitis (diagnosed 2022-04-15)
- Right optic neuropathy (diagnosed 2022-07-15)
- Relapsing-remitting multiple sclerosis (diagnosed age 27)
- Neurogenic bladder with detrusor overactivity
- History of recurrent optic neuritis (bilateral)

Her past medical history is significant for multiple sclerosis (MS), diagnosed 5 years ago, with two previous episodes of optic neuritis and one episode of sensory symptoms.

PAST SURGICAL HISTORY:
- No prior surgical procedures

MEDICATIONS:
She has been on interferon beta-1a since her diagnosis. She does not take any other medications.

Home Medications:
- Oxybutynin 5 mg oral twice daily as needed for urinary urgency
- Vitamin D3 2000 IU oral daily

ALLERGIES:
She has no known drug allergies

FAMILY HISTORY:
- Mother: Autoimmune hypothyroidism (Hashimoto's thyroiditis)
- Father: Essential hypertension
- No known first-degree relatives with multiple sclerosis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Financial Analyst.

PHYSICAL EXAMINATION:
On examination, her right lower extremity shows a strength of 3/5, increased tone, and hyperreflexia. Visual acuity is decreased in the left eye, with an afferent pupillary defect.

LABORATORY DATA:
Laboratory values:
- WBC: 7,000/uL
- Hemoglobin: 13.5 g/dL
- Platelets: 250,000/uL
- Sodium: 140 mEq/L
- Potassium: 4.2 mEq/L
- Creatinine: 0.9 mg/dL
- Glucose: 98 mg/dL

IMAGING:
MRI of the brain with gadolinium shows multiple periventricular white matter lesions, with one enhancing lesion.

## Encounter 7: source 2023-07-15, practice 2026-06-16, ed, Emergency Medicine

Chief complaint: Progressive weakness in both legs with difficulty walking, numbness below navel, and urinary retention since morning.

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. David Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive weakness in both legs with difficulty walking, numbness below navel, and urinary retention since morning.

HISTORY OF PRESENT ILLNESS:
The patient with relapsing-remitting multiple sclerosis and neurogenic bladder returns for evaluation following prior unilateral weakness. She reports difficulty walking with several falls at home, numbness below her navel, and has been unable to void since morning.

PAST MEDICAL HISTORY:
Active Problem List:
- Multiple sclerosis (diagnosed 2020-01-15)
- Urinary incontinence (diagnosed 2022-01-15)
- Uncomplicated cystitis (diagnosed 2022-04-15)
- Right optic neuropathy (diagnosed 2022-07-15)
- Relapsing-remitting multiple sclerosis (diagnosed age 27)
- Neurogenic bladder with detrusor overactivity
- History of recurrent optic neuritis (bilateral)

Her past medical history is significant for an episode of blurred vision in her right eye two years ago, which resolved spontaneously.

PAST SURGICAL HISTORY:
- No prior surgical procedures

MEDICATIONS:
Home Medications:
- Interferon beta-1a 44 mcg subcutaneous injection three times weekly
- Oxybutynin 5 mg oral twice daily as needed for urinary urgency
- Vitamin D3 2000 IU oral daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother: Autoimmune hypothyroidism (Hashimoto's thyroiditis)
- Father: Essential hypertension
- No known first-degree relatives with multiple sclerosis

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Financial Analyst.

PHYSICAL EXAMINATION:
On examination, her strength is 2/5 in both lower extremities, and she has decreased sensation to light touch and pinprick below the T6 dermatome. Deep tendon reflexes are brisk in the lower extremities, and Babinski sign is positive bilaterally.

LABORATORY DATA:
Laboratory Values:
- Complete Blood Count: Normal
- Erythrocyte Sedimentation Rate: Normal
- Vitamin B12 Level: Normal
- Aquaporin-4 Antibody: Negative
- Oligoclonal Bands in CSF: Positive

IMAGING:
MRI of the spinal cord reveals a hyperintense lesion on T2-weighted images at the T4 level.
