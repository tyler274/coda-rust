# synthetic_hospital patient 2124

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 42 at first encounter, sex M, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-12-07 (Mon) | outpatient | Neurology |
| 1 | 2021-01-15 | 2021-11-30 (Tue) | outpatient | Internal Medicine |
| 2 | 2022-01-15 | 2022-12-05 (Mon) | outpatient | Psychiatry |
| 3 | 2023-01-15 | 2023-12-07 (Thu) | outpatient | Neurology |
| 4 | 2025-01-15 | 2025-12-09 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Financial Analyst (sedentary desk job)",
 "alcohol_use": "Rare (1-2 drinks per year)",
 "family_history": [
  "Father: Huntington disease (deceased age 53)",
  "Mother: Hypertension, hyperlipidemia",
  "Paternal uncle: Huntington disease (living, age 48)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "50 mg PO daily",
   "name": "Sertraline"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Escitalopram"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Huntington disease (diagnosed age 41)",
  "Major depressive disorder",
  "Generalized anxiety disorder",
  "History of venous thromboembolism (DVT at age 40)",
  "Obesity (BMI 32, 25 lb weight gain over past year)"
 ],
 "age_at_first_encounter": 42
}
```

Primary diagnoses: ["Huntington disease", "Iliofemoral Deep Vein Thrombosis"]
Comorbidities: ["Risk of Alzheimer's disease", "Major depressive disorder"]

## Encounter 0: source 2020-01-15, practice 2020-12-07, outpatient, Neurology

Chief complaint: Involuntary jerking movements of arms and legs, difficulty maintaining balance, and increasing forgetfulness

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Involuntary jerking movements of arms and legs, difficulty maintaining balance, and increasing forgetfulness

HISTORY OF PRESENT ILLNESS:
A 41-year-old man presents to the clinic with a one-year history of involuntary jerking movements of his arms and legs, difficulty maintaining balance, and increasing forgetfulness. His wife reports that over the past six months he has also developed mood swings and progressive cognitive decline that interfere with his work.

PAST MEDICAL HISTORY:
Active Problem List:
- Huntington disease (diagnosed age 41)
- Major depressive disorder
- Generalized anxiety disorder
- History of venous thromboembolism (DVT at age 40)
- Obesity (BMI 32, 25 lb weight gain over past year)

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Sertraline 50 mg PO daily
- Escitalopram 10 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
The patient's father had similar symptoms and died at age 56 after years of progressive neurologic deterioration.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (1-2 drinks per year). Occupation: Financial Analyst (sedentary desk job).

PHYSICAL EXAMINATION:
Neurologic examination reveals choreiform movements and mild dystonia, with preserved strength and sensation.

LABORATORY DATA:
Genetic testing confirms a CAG trinucleotide repeat expansion.

IMAGING:
Imaging findings are shown below.

OTHER STUDIES:
Cognitive testing demonstrates impaired attention, poor executive function, and short-term memory loss.

## Encounter 1: source 2021-01-15, practice 2021-11-30, outpatient, Internal Medicine

Chief complaint: Wife reports unusual behavior including irritability, mood swings, and occasional aggressive outbursts; difficulty concentrating at work

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Marcus Johnson, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Wife reports unusual behavior including irritability, mood swings, and occasional aggressive outbursts; difficulty concentrating at work

HISTORY OF PRESENT ILLNESS:
The patient returns for Huntington disease follow-up. He admits to feeling depressed and has difficulty concentrating at work.

PAST MEDICAL HISTORY:
Active Problem List:
- Huntington disease (diagnosed 2020-01-15)
- Major depressive disorder
- Generalized anxiety disorder
- History of venous thromboembolism (DVT at age 40)
- Obesity (BMI 32, 25 lb weight gain over past year)

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Sertraline 50 mg PO daily
- Escitalopram 10 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
His medical history is significant for a father who died at the age of 50 with similar psychiatric symptoms and involuntary movements.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (1-2 drinks per year). Occupation: Financial Analyst (sedentary desk job).

PHYSICAL EXAMINATION:
On examination, the patient displays brief, irregular, and unpredictable jerky movements of his limbs, consistent with chorea. He also has difficulty with rapid, alternating movements during the finger-to-nose test.

LABORATORY DATA:
Genetic testing shows an increased number of CAG repeats in the HTT gene.

IMAGING:
Magnetic resonance imaging (MRI) of the brain reveals atrophy of the caudate nucleus and putamen.

## Encounter 2: source 2022-01-15, practice 2022-12-05, outpatient, Psychiatry

Chief complaint: Increasingly irritable with episodes of uncharacteristic aggression; depression and difficulty concentrating

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Emily Rodriguez, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Increasingly irritable with episodes of uncharacteristic aggression; depression and difficulty concentrating

HISTORY OF PRESENT ILLNESS:
The patient is a 44-year-old male with Huntington disease who returns for psychiatric follow-up, having presented to Neurology in 2020 with motor symptoms and to Internal Medicine in 2021 with behavioral changes. He reports that over the past year, he has become increasingly irritable and has had several episodes of uncharacteristic aggression. The patient admits to feeling depressed and has difficulty concentrating at work. He also mentions involuntary jerking movements of his arms and legs, which have been progressively worsening.

PAST MEDICAL HISTORY:
Active Problem List:
- Huntington disease (diagnosed 2020-01-15)
- Major depressive disorder
- Generalized anxiety disorder
- History of venous thromboembolism (DVT at age 40)
- Obesity (BMI 32, 25 lb weight gain over past year)

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Sertraline 50 mg PO daily
- Escitalopram 10 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
His father had similar symptoms and died at the age of 60.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (1-2 drinks per year). Occupation: Financial Analyst (sedentary desk job).

PHYSICAL EXAMINATION:
On examination, the patient exhibits brief, irregular, and unpredictable jerks in his upper extremities. His gait is unsteady, and he has difficulty with tandem walking. His cognitive examination reveals mild difficulty with executive function tasks.

OTHER STUDIES:
Genetic testing shows an increased number of CAG repeats in the HTT gene.

## Encounter 3: source 2023-01-15, practice 2023-12-07, outpatient, Neurology

Chief complaint: Unusual behavior with irritability, mood swings, and occasional aggressive outbursts; involuntary jerking movements

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Unusual behavior with irritability, mood swings, and occasional aggressive outbursts; involuntary jerking movements

HISTORY OF PRESENT ILLNESS:
The patient is a 45-year-old male with Huntington disease followed in Neurology and Psychiatry since 2020 who returns for continued assessment and reports that over the past year, he has been exhibiting unusual behavior, including irritability, mood swings, and occasional aggressive outbursts. The patient admits to feeling depressed and anxious but cannot pinpoint a reason for these feelings. He also mentions involuntary jerking movements of his arms and legs, which have been progressively worsening over the past six months.

PAST MEDICAL HISTORY:
Active Problem List:
- Huntington disease (diagnosed 2020-01-15)
- Major depressive disorder
- Generalized anxiety disorder
- History of venous thromboembolism (DVT at age 40)
- Obesity (BMI 32, 25 lb weight gain over past year)

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Sertraline 50 mg PO daily
- Escitalopram 10 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
His medical history is significant for a father who passed away at the age of 50 with similar symptoms.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (1-2 drinks per year). Occupation: Financial Analyst (sedentary desk job).

PHYSICAL EXAMINATION:
On physical examination, the patient demonstrates brief, irregular, and unpredictable movements in his upper extremities. Neurological examination reveals difficulty with rapid, alternating movements and an unstable gait.

LABORATORY DATA:
Laboratory Values:
- CAG repeats in HTT gene: 44 (normal <36)
- CBC: Within normal limits
- CMP: Within normal limits
- TSH: Within normal limits
- Vitamin B12: Within normal limits

OTHER STUDIES:
Genetic testing shows an increased number of CAG repeats in the HTT gene.

ASSESSMENT AND PLAN:
Given the patient's history, symptoms, and genetic testing results,

## Encounter 4: source 2025-01-15, practice 2025-12-09, ed, Emergency Medicine

Chief complaint: Swollen, painful left leg

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. James Patterson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Swollen, painful left leg

HISTORY OF PRESENT ILLNESS:
This 47-year-old male with Huntington disease, major depressive disorder, generalized anxiety disorder, and venous thromboembolism presents with a swollen, painful left leg. He states this happened once before two years ago and was told he had a 'blood clot.' He recently returned from a 9-hour international flight and has gained about 25 lbs over the past year. He works a desk job and spends most of his day sitting.

PAST MEDICAL HISTORY:
Active Problem List:
- Huntington disease (diagnosed 2020-01-15)
- Major depressive disorder
- Generalized anxiety disorder
- History of venous thromboembolism (DVT at age 40)
- Obesity (BMI 32, 25 lb weight gain over past year)

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He also takes a daily selective serotonin reuptake inhibitor for depression

Home Medications:
- Sertraline 50 mg PO daily
- Escitalopram 10 mg PO daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Huntington disease (deceased age 53)
- Mother: Hypertension, hyperlipidemia
- Paternal uncle: Huntington disease (living, age 48)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Rare (1-2 drinks per year). Occupation: Financial Analyst (sedentary desk job).

VITAL SIGNS:
He has a BMI of 33 kg/m². Vital signs are: temperature 98.7°F (37.1°C), blood pressure 128/84 mmHg, heart rate 92/min, respiratory rate 16/min, and oxygen saturation 98% on room air.

PHYSICAL EXAMINATION:
On examination, his left leg is warm, erythematous, and tender to palpation.

IMAGING:
Duplex ultrasonography confirms the presence of a deep vein thrombosis.
