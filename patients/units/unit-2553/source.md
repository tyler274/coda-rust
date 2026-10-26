# synthetic_hospital patient 2553

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 22 at first encounter, sex M, Non-Hispanic White, Private insurance (parental plan).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-10-28 (Fri) | outpatient | Psychiatry |
| 1 | 2020-03-15 | 2022-12-27 (Tue) | outpatient | Internal Medicine |
| 2 | 2021-03-15 | 2023-12-16 (Sat) | ed | Emergency Medicine |
| 3 | 2022-01-15 | 2024-10-22 (Tue) | ed | Emergency Medicine |
| 4 | 2023-01-15 | 2025-10-28 (Tue) | icu | Medical ICU |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private insurance (parental plan)",
 "occupation": "Unemployed; withdrew from university 2 months prior",
 "alcohol_use": "Denies current use",
 "family_history": [
  "Mother with schizophrenia",
  "Paternal grandfather with alcohol use disorder",
  "Parents are obligate carriers of glycogen storage disease type IV (autosomal recessive inheritance)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never",
 "home_medications": [],
 "surgical_history": [
  "Tonsillectomy (age 8)"
 ],
 "chronic_conditions": [
  "Schizophrenia",
  "Glycogen storage disease type IV (Andersen disease)"
 ],
 "age_at_first_encounter": 22
}
```

Primary diagnoses: ["Neuroleptic malignant syndrome", "Glycogen storage disease type IV (Andersen disease)", "MDMA (ecstasy) intoxication", "Schizophrenia"]
Comorbidities: ["Rhabdomyolysis requiring fluid resuscitation", "Schizophrenia"]

## Encounter 0: source 2020-01-15, practice 2022-10-28, outpatient, Psychiatry

Chief complaint: Increasingly bizarre behavior over the past several months with paranoia, social withdrawal, and dropout from university

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Increasingly bizarre behavior over the past several months with paranoia, social withdrawal, and dropout from university

HISTORY OF PRESENT ILLNESS:
He recently dropped out of university and has spent most of his time alone in his room, whispering to himself and refusing to attend family meals. He has become increasingly suspicious, accusing his family of planting cameras in the walls and saying that his phone is being tracked by government agents. He has also stopped bathing and changing clothes, and his speech has become disorganized. He eats little, has lost 10 pounds, and rarely leaves the house.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizophrenia
- Glycogen storage disease type IV (Andersen disease)

PAST SURGICAL HISTORY:
- Tonsillectomy (age 8)

MEDICATIONS:
He takes no medications.

ALLERGIES:
NKDA

FAMILY HISTORY:
There is a maternal uncle with a history of chronic psychiatric illness, but the patient has never seen a psychiatrist.

SOCIAL HISTORY:
Smoking: Never. Alcohol: Denies current use. Occupation: Unemployed; withdrew from university 2 months prior.

PHYSICAL EXAMINATION:
On mental status exam, he is inattentive, muttering under his breath, and frequently looks toward the ceiling when asked questions.

## Encounter 1: source 2020-03-15, practice 2022-12-27, outpatient, Internal Medicine

Chief complaint: Recurrent episodes of muscle pain, severe cramps, and occasional dark urine following intense physical activity since high school

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Marcus Rodriguez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Recurrent episodes of muscle pain, severe cramps, and occasional dark urine following intense physical activity since high school

HISTORY OF PRESENT ILLNESS:
The patient is a 22-year-old male with a history of Schizophrenia established during prior psychiatric evaluation on January 15, 2020, who presents to Internal Medicine for evaluation of musculoskeletal symptoms. He reports that since high school, he has often had to stop during short bursts of exercise such as sprinting or weightlifting because of "severe cramps" in his thighs and arms. The discomfort usually resolves within minutes of rest but occasionally has been followed by dark-colored urine the next morning. He reports that he can complete prolonged low-intensity activities such as walking without difficulty. There is no history of trauma, joint pain, rash, or fever.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizophrenia (diagnosed 2020-01-15)
- Glycogen storage disease type IV (Andersen disease)

PAST SURGICAL HISTORY:
- Tonsillectomy (age 8)

MEDICATIONS:
He takes no medications

ALLERGIES:
NKDA

FAMILY HISTORY:
His father had similar exercise intolerance and "brown urine" episodes during military training.

SOCIAL HISTORY:
He denies alcohol or drug use

PHYSICAL EXAMINATION:
Physical examination between episodes is normal, with no muscle atrophy or weakness.

LABORATORY DATA:
Serum creatine kinase level is elevated at baseline. Urinalysis after a recent episode shows positive blood on dipstick with no red blood cells on microscopy.

## Encounter 2: source 2021-03-15, practice 2023-12-16, ed, Emergency Medicine

Chief complaint: Brought by friends from music event after ingesting MDMA, presenting with euphoria, hyperactivity, polydipsia, hyperthermia, and subsequent confusion

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Emily Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Brought by friends from music event after ingesting MDMA, presenting with euphoria, hyperactivity, polydipsia, hyperthermia, and subsequent confusion

HISTORY OF PRESENT ILLNESS:
The patient is a 23-year-old male with schizophrenia and glycogen storage disease type IV (Andersen disease) established during prior Psychiatry and Internal Medicine encounters, who presents to the emergency department. They report he took "a small pill" earlier in the evening and since then has become unusually euphoric, talkative, and energetic. He has been dancing continuously, drinking large amounts of water, and complaining of feeling very warm. After several hours, he becomes anxious and confused.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizophrenia (diagnosed 2020-01-15)
- Glycogen storage disease type IV (Andersen disease) (diagnosed 2020-03-15)

PAST SURGICAL HISTORY:
- Tonsillectomy (age 8)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with schizophrenia
- Paternal grandfather with alcohol use disorder
- Parents are obligate carriers of glycogen storage disease type IV (autosomal recessive inheritance)

SOCIAL HISTORY:
Smoking: Never. Alcohol: Denies current use. Occupation: Unemployed; withdrew from university 2 months prior.

VITAL SIGNS:
His temperature is 40°C (104°F), blood pressure is 150/90 mm Hg, pulse is 128/min, and respirations are 26/min.

PHYSICAL EXAMINATION:
Physical examination reveals flushed, diaphoretic skin, dilated pupils, and mild jaw clenching. He is alert but restless, with tremors and occasional muscle rigidity.

LABORATORY DATA:
Laboratory testing shows mild hyponatremia, elevated creatine kinase, and metabolic acidosis. A urine toxicology screen is negative for amphetamines, cocaine, and opioids.

## Encounter 3: source 2022-01-15, practice 2024-10-22, ed, Emergency Medicine

Chief complaint: Altered mental status, high fever, and generalized muscle rigidity developing two weeks after initiation of new antipsychotic medication

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. James Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Altered mental status, high fever, and generalized muscle rigidity developing two weeks after initiation of new antipsychotic medication

HISTORY OF PRESENT ILLNESS:
The patient is a 24-year-old male with a history of schizophrenia and glycogen storage disease type IV who returns to the emergency department, having presented in 2021 following MDMA ingestion. He is brought by his family due to altered mental status, fever, and muscle rigidity. His family reports that he was started on a new medication 2 weeks ago, but they are unsure of its name.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizophrenia (diagnosed 2020-01-15)
- Glycogen storage disease type IV (Andersen disease) (diagnosed 2020-03-15)
- MDMA (ecstasy) intoxication (diagnosed 2021-03-15)

PAST SURGICAL HISTORY:
- Tonsillectomy (age 8)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with schizophrenia
- Paternal grandfather with alcohol use disorder
- Parents are obligate carriers of glycogen storage disease type IV (autosomal recessive inheritance)

SOCIAL HISTORY:
Smoking: Never. Alcohol: Denies current use. Occupation: Unemployed; withdrew from university 2 months prior.

VITAL SIGNS:
Temperature of 39.5°C (103.1°F), heart rate is 110 bpm, blood pressure is 160/90 mmHg

PHYSICAL EXAMINATION:
On examination, the patient is diaphoretic, febrile with a temperature of 39.5°C (103.1°F), and exhibits generalized lead-pipe rigidity. His heart rate is 110 bpm, blood pressure is 160/90 mmHg, and he appears confused and agitated.

LABORATORY DATA:
Laboratory studies are as follows:

- White blood cell count: 15,000/mm³
- Creatine kinase: 5,000 U/L
- Serum electrolytes: within normal limits
- Liver function tests: mildly elevated AST and ALT
- Urinalysis: dark-colored urine, no evidence of infection

## Encounter 4: source 2023-01-15, practice 2025-10-28, icu, Medical ICU

Chief complaint: Severe high fever, profound muscle rigidity, and altered mental status two days after starting alternative antipsychotic therapy

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Aisha Patel, MD
Department: Medical ICU
Visit Type: icu

CHIEF COMPLAINT:
Severe high fever, profound muscle rigidity, and altered mental status two days after starting alternative antipsychotic therapy

HISTORY OF PRESENT ILLNESS:
This 25-year-old male with schizophrenia returns to the emergency department with a two-day history of high fever, severe muscle rigidity, and altered mental status, similar to his 2022 presentation. His mother reports that he was recently started on a new medication for his psychiatric condition.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizophrenia (diagnosed 2020-01-15)
- Glycogen storage disease type IV (Andersen disease) (diagnosed 2020-03-15)
- MDMA (ecstasy) intoxication (diagnosed 2021-03-15)
- Neuroleptic malignant syndrome (diagnosed 2022-01-15)

PAST SURGICAL HISTORY:
- Tonsillectomy (age 8)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with schizophrenia
- Paternal grandfather with alcohol use disorder
- Parents are obligate carriers of glycogen storage disease type IV (autosomal recessive inheritance)

SOCIAL HISTORY:
Smoking: Never. Alcohol: Denies current use. Occupation: Unemployed; withdrew from university 2 months prior.

VITAL SIGNS:
Temperature of 39.5°C (103.1°F), heart rate of 110 bpm, blood pressure of 160/90 mmHg

PHYSICAL EXAMINATION:
On examination, the patient is diaphoretic, has a temperature of 39.5°C (103.1°F), heart rate of 110 bpm, blood pressure of 160/90 mmHg, and is markedly rigid in all extremities. He is confused and unable to provide further history.

LABORATORY DATA:
White blood cell count: 15,000/mm³
Serum creatine kinase: 5,000 U/L
Serum potassium: 5.2 mEq/L
Serum sodium: 140 mEq/L
Serum creatinine: 1.2 mg/dL
