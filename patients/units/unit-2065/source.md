# synthetic_hospital patient 2065

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 52 at first encounter, sex M, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2026-01-01 (Thu) | ed | Emergency Medicine |
| 1 | 2020-03-15 | 2026-03-08 (Sun) | icu | Critical Care Medicine |
| 2 | 2020-07-15 | 2026-07-12 (Sun) | icu | Internal Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Construction supervisor",
 "alcohol_use": "Heavy daily use (approximately 1 pint of vodka daily)",
 "family_history": [
  "Father: Alcohol use disorder, hypertension",
  "Mother: Osteoarthritis",
  "Brother: Chronic low back pain"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 20 pack-years, quit 2018",
 "home_medications": [
  {
   "dose": "5-10 mg by mouth every 6 hours as needed for pain",
   "name": "Oxycodone"
  },
  {
   "dose": "20 mg by mouth daily for gastritis prophylaxis",
   "name": "Omeprazole"
  }
 ],
 "surgical_history": [
  "Appendectomy at age 11"
 ],
 "chronic_conditions": [
  "Chronic lower back pain",
  "Alcohol use disorder",
  "Alcoholic liver disease with suspected cirrhosis"
 ],
 "age_at_first_encounter": 52
}
```

Primary diagnoses: ["Opioid overdose", "Acetaminophen toxicity", "Delirium tremens"]
Comorbidities: ["Alcohol use disorder"]

## Encounter 0: source 2020-01-15, practice 2026-01-01, ed, Emergency Medicine

Chief complaint: nausea, vomiting, and right upper quadrant pain

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Nausea, vomiting, and right upper quadrant pain

HISTORY OF PRESENT ILLNESS:
2-day history of nausea, vomiting, and right upper quadrant pain. He reports consuming a "few extra" over-the-counter pain relievers for a headache over the past week.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic lower back pain
- Alcohol use disorder
- Alcoholic liver disease with suspected cirrhosis

PAST SURGICAL HISTORY:
- Appendectomy at age 11

MEDICATIONS:
Home Medications:
- Oxycodone 5-10 mg by mouth every 6 hours as needed for pain
- Omeprazole 20 mg by mouth daily for gastritis prophylaxis

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Alcohol use disorder, hypertension
- Mother: Osteoarthritis
- Brother: Chronic low back pain

SOCIAL HISTORY:
History of chronic alcohol use

PHYSICAL EXAMINATION:
On examination, he is lethargic but oriented to person and place. His skin and sclerae are icteric. His abdomen is tender to palpation in the right upper quadrant without rebound or guarding.

LABORATORY DATA:
Laboratory studies are as follows:

- AST: 2500 U/L
- ALT: 2300 U/L
- Total bilirubin: 5.2 mg/dL
- INR: 2.1
- Serum acetaminophen level: 30 mcg/mL (therapeutic range 10-20 mcg/mL)

## Encounter 1: source 2020-03-15, practice 2026-03-08, icu, Critical Care Medicine

Chief complaint: brought to the emergency department by his wife due to lethargy and shallow breathing

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Michael Rodriguez, MD
Department: Critical Care Medicine
Visit Type: icu

CHIEF COMPLAINT:
Brought to the emergency department by his wife due to lethargy and shallow breathing

HISTORY OF PRESENT ILLNESS:
The patient with alcohol use disorder returns two months after acetaminophen toxicity. His wife reports that he has been increasingly drowsy over the past few hours.

PAST MEDICAL HISTORY:
Active Problem List:
- Acetaminophen toxicity (diagnosed 2020-01-15)
- Chronic lower back pain
- Alcohol use disorder
- Alcoholic liver disease with suspected cirrhosis

PAST SURGICAL HISTORY:
- Appendectomy at age 11

MEDICATIONS:
He takes for which he takes prescribed oxycodone.

Home Medications:
- Omeprazole 20 mg by mouth daily for gastritis prophylaxis

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Alcohol use disorder, hypertension
- Mother: Osteoarthritis
- Brother: Chronic low back pain

SOCIAL HISTORY:
He also consumes alcohol regularly but denies using any other drugs

VITAL SIGNS:
Respiratory rate of 8 breaths per minute. His blood pressure is 110/70 mmHg, pulse is 55 beats per minute, and oxygen saturation is 89% on room air

PHYSICAL EXAMINATION:
On examination, the patient is somnolent but arousable

LABORATORY DATA:
An arterial blood gas (ABG) reveals the following:

- pH: 7.28
- PaCO2: 65 mmHg
- PaO2: 58 mmHg
- HCO3-: 24 mEq/L

## Encounter 2: source 2020-07-15, practice 2026-07-12, icu, Internal Medicine

Chief complaint: hematemesis with subsequent development of severe agitation and visual hallucinations on hospital day 3

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Priya Patel, MD
Department: Internal Medicine
Visit Type: icu

CHIEF COMPLAINT:
Hematemesis with subsequent development of severe agitation and visual hallucinations on hospital day 3

HISTORY OF PRESENT ILLNESS:
This patient with alcohol use disorder and recent critical care admissions presents with hematemesis. On hospital day 3, he becomes progressively agitated and confused. He is attempting to climb out of bed, insisting he needs to "get away from the spiders crawling on the walls."

PAST MEDICAL HISTORY:
Active Problem List:
- Acetaminophen toxicity (diagnosed 2020-01-15)
- Opioid overdose (diagnosed 2020-03-15)
- Chronic lower back pain
- Alcohol use disorder
- Alcoholic liver disease with suspected cirrhosis

PAST SURGICAL HISTORY:
- Appendectomy at age 11

MEDICATIONS:
Home Medications:
- Oxycodone 5-10 mg by mouth every 6 hours as needed for pain
- Omeprazole 20 mg by mouth daily for gastritis prophylaxis

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Alcohol use disorder, hypertension
- Mother: Osteoarthritis
- Brother: Chronic low back pain

SOCIAL HISTORY:
His wife confirms that he drinks a pint of vodka daily and had his last drink 3 days ago, on the day of admission.

VITAL SIGNS:
Vitals are: temperature 38.6°C (101.5°F), blood pressure 168/96 mm Hg, pulse 122/min, and respiratory rate 22/min.

PHYSICAL EXAMINATION:
Physical exam reveals diaphoresis, fine tremor, and generalized disorientation.

LABORATORY DATA:
Laboratory studies show no electrolyte derangements.

IMAGING:
A CT scan of the head is normal.
