# synthetic_hospital patient 2108

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 42 at first encounter, sex M, Non-Hispanic White, Medicaid, secondary to Workers' Compensation.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2019-11-05 (Tue) | outpatient | Internal Medicine |
| 1 | 2021-01-15 | 2020-11-03 (Tue) | outpatient | Endocrinology |
| 2 | 2023-01-15 | 2022-11-02 (Wed) | inpatient | Internal Medicine |
| 3 | 2026-01-15 | 2025-10-31 (Fri) | ed | Emergency Medicine |
| 4 | 2026-02-15 | 2025-11-28 (Fri) | icu | Critical Care Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicaid, secondary to Workers' Compensation",
 "occupation": "Warehouse forklift operator",
 "alcohol_use": "None",
 "family_history": [
  "Father: Alcohol use disorder",
  "Mother: Major depressive disorder",
  "Older brother: Chronic low back pain"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "10 mg PO every 4-6 hours as needed for back pain",
   "name": "Oxycodone"
  },
  {
   "dose": "0.5 mg PO twice daily as needed for anxiety",
   "name": "Alprazolam"
  }
 ],
 "surgical_history": [
  "No prior surgeries"
 ],
 "chronic_conditions": [
  "Chronic lower back pain, post-traumatic, work-related",
  "Major depressive disorder",
  "History of opioid use disorder, in remission",
  "History of hepatitis C infection, resolved",
  "Secondary hypogonadism"
 ],
 "age_at_first_encounter": 42
}
```

Primary diagnoses: ["Opioid-induced secondary hypogonadism", "Patient-ventilator asynchrony", "Naltrexone-induced opioid receptor blockade causing therapeutic failure of morphine", "Helicobacter pylori-associated gastric ulcer with hemorrhage", "Resolved Hepatitis C Infection"]
Comorbidities: ["Long-term opioid use (extended-release morphine)", "Chronic back pain"]

## Encounter 0: source 2020-01-15, practice 2019-11-05, outpatient, Internal Medicine

Chief complaint: Routine health examination and follow-up for resolved hepatitis C infection with occasional fatigue.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine health examination and follow-up for resolved hepatitis C infection with occasional fatigue.

HISTORY OF PRESENT ILLNESS:
He reports feeling generally well but mentions occasional fatigue.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic lower back pain, post-traumatic, work-related
- Major depressive disorder
- History of opioid use disorder, in remission
- History of hepatitis C infection, resolved
- Secondary hypogonadism

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Oxycodone 10 mg PO every 4-6 hours as needed for back pain
- Alprazolam 0.5 mg PO twice daily as needed for anxiety

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Alcohol use disorder
- Mother: Major depressive disorder
- Older brother: Chronic low back pain

SOCIAL HISTORY:
He has been in a methadone program for the past year and denies any recent drug use.

REVIEW OF SYSTEMS:
He has no jaundice, abdominal pain, or pruritus.

PHYSICAL EXAMINATION:
His physical examination is unremarkable.

LABORATORY DATA:
His liver function tests are within normal limits.
The HCV antibody test returns positive.
The HCV RNA PCR result is now available:

- HCV RNA PCR: Not detected

ASSESSMENT AND PLAN:
Given his risk factors, you decide to screen for hepatitis C virus (HCV).
Based on these findings,

You proceed with HCV RNA PCR testing to confirm chronic infection.

## Encounter 1: source 2021-01-15, practice 2020-11-03, outpatient, Endocrinology

Chief complaint: Decreased libido, erectile dysfunction, and fatigue with reduced facial hair growth over the past six months.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Marcus Johnson, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Decreased libido, erectile dysfunction, and fatigue with reduced facial hair growth over the past six months.

HISTORY OF PRESENT ILLNESS:
The patient is a 43-year-old male with chronic lower back pain on long-term opioid therapy and resolved hepatitis C infection who returns to the endocrinology clinic for evaluation. He was previously seen in internal medicine one year ago for routine health maintenance. He reports feeling fatigued and has noticed a decrease in facial hair growth.

PAST MEDICAL HISTORY:
Active Problem List:
- Resolved Hepatitis C Infection (diagnosed 2020-01-15)
- Chronic lower back pain, post-traumatic, work-related
- Major depressive disorder
- History of opioid use disorder, in remission
- Secondary hypogonadism

His past medical history is significant for chronic back pain due to a work-related injury.

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
He takes he has been taking prescribed oxycodone for the past two years.

Home Medications:
- Alprazolam 0.5 mg PO twice daily as needed for anxiety

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Alcohol use disorder
- Mother: Major depressive disorder
- Older brother: Chronic low back pain

SOCIAL HISTORY:
He denies use of tobacco, alcohol, or illicit drugs.

VITAL SIGNS:
His vital signs are within normal limits.

PHYSICAL EXAMINATION:
Physical examination reveals decreased body hair, soft testes, and a body mass index of 30 kg/m^2.

LABORATORY DATA:
Laboratory tests reveal the following:

- Serum testosterone: 150 ng/dL (Normal: 300-1000 ng/dL)
- Luteinizing hormone (LH): 2 mIU/mL (Normal: 1.7-8.6 mIU/mL)
- Follicle-stimulating hormone (FSH): 3 mIU/mL (Normal: 1.5-12.4 mIU/mL)
- Prolactin: 14 ng/mL (Normal: 4-15 ng/mL)

## Encounter 2: source 2023-01-15, practice 2022-11-02, inpatient, Internal Medicine

Chief complaint: Severe lower back pain rated 9/10 after lifting heavy object at work, unresponsive to morphine therapy.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Priya Patel, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Severe lower back pain rated 9/10 after lifting heavy object at work, unresponsive to morphine therapy.

HISTORY OF PRESENT ILLNESS:
The patient, a 45-year-old male with chronic post-traumatic work-related lower back pain and opioid-induced secondary hypogonadism followed in endocrinology and a history of resolved hepatitis C and opioid use disorder in remission, is brought to the emergency department with severe lower back pain after lifting a heavy object. He denies any recent trauma, fever, or weight loss. On the second day of admission, despite receiving his scheduled doses of morphine, he reports persistent, severe pain rated 9 out of 10 that is unresponsive to treatment.

PAST MEDICAL HISTORY:
Active Problem List:
- Resolved Hepatitis C Infection (diagnosed 2020-01-15)
- Opioid-induced secondary hypogonadism (diagnosed 2021-01-15)
- Chronic lower back pain, post-traumatic, work-related
- Major depressive disorder
- History of opioid use disorder, in remission
- Secondary hypogonadism

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
He has been on a stable dose of extended-release morphine for his chronic back pain and has been participating in a substance abuse program, where he receives monthly intramuscular injections of naltrexone. His current medications include morphine sulfate extended-release 60 mg twice daily and ibuprofen as needed.

Home Medications:
- Oxycodone 10 mg PO every 4-6 hours as needed for back pain
- Alprazolam 0.5 mg PO twice daily as needed for anxiety

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Alcohol use disorder
- Mother: Major depressive disorder
- Older brother: Chronic low back pain

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Warehouse forklift operator.

VITAL SIGNS:
His vital signs are within normal limits.

PHYSICAL EXAMINATION:
On physical examination, he has tenderness over the lumbar paraspinal muscles without neurological deficits.

LABORATORY DATA:
Urine toxicology screen is negative for illicit substances.

PLAN:
The patient is admitted for pain control and physical therapy.

## Encounter 3: source 2026-01-15, practice 2025-10-31, ed, Emergency Medicine

Chief complaint: Repeated episodes of dark-colored vomiting over the past day with generalized fatigue and upper abdominal discomfort.

ENCOUNTER NOTE
Date: 2026-01-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Repeated episodes of dark-colored vomiting over the past day with generalized fatigue and upper abdominal discomfort.

HISTORY OF PRESENT ILLNESS:
This 48-year-old male with chronic post-traumatic lower back pain, history of opioid use disorder in remission, and prior naltrexone-induced opioid receptor blockade is brought to the emergency department by his partner after experiencing repeated episodes of dark-colored vomiting over the past day. He reports generalized fatigue and a vague discomfort in his upper abdomen over the past several weeks, which he attributed to work-related stress. He does not recall any recent fever or dietary changes.

PAST MEDICAL HISTORY:
Active Problem List:
- Resolved Hepatitis C Infection (diagnosed 2020-01-15)
- Opioid-induced secondary hypogonadism (diagnosed 2021-01-15)
- Naltrexone-induced opioid receptor blockade causing therapeutic failure of morphine (diagnosed 2023-01-15)
- Chronic lower back pain, post-traumatic, work-related
- Major depressive disorder
- History of opioid use disorder, in remission
- Secondary hypogonadism

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
The patient mentions using some over-the-counter medications intermittently for chronic back discomfort.

Home Medications:
- Oxycodone 10 mg PO every 4-6 hours as needed for back pain
- Alprazolam 0.5 mg PO twice daily as needed for anxiety

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Alcohol use disorder
- Mother: Major depressive disorder
- Older brother: Chronic low back pain

SOCIAL HISTORY:
He denies any use of alcohol or tobacco

REVIEW OF SYSTEMS:
Review of systems is unremarkable except for mild dizziness.

PHYSICAL EXAMINATION:
On examination, he appears pale and slightly diaphoretic. Abdominal exam reveals no rebound or guarding, but there is mild epigastric tenderness to deep palpation. Rectal examination shows dark, tarry stool.

LABORATORY DATA:
Laboratory studies reveal a hemoglobin of 9.8 g/dL, and serum creatinine is within normal range.

OTHER STUDIES:
Upper endoscopy shows a lesion with overlying clot in the gastric antrum.

## Encounter 4: source 2026-02-15, practice 2025-11-28, icu, Critical Care Medicine

Chief complaint: Progressive drowsiness and headache over two days with recent increased alprazolam use for work-related stress.

ENCOUNTER NOTE
Date: 2026-02-15
Provider: Dr. Maria Rodriguez, MD
Department: Critical Care Medicine
Visit Type: icu

CHIEF COMPLAINT:
Progressive drowsiness and headache over two days with recent increased alprazolam use for work-related stress.

HISTORY OF PRESENT ILLNESS:
This patient with chronic lower back pain returns to the ICU one month following upper gastrointestinal hemorrhage. He reports that his symptoms have been progressively worsening over the past two days. His wife mentions that he has been using more of his prescribed alprazolam recently due to increased stress at work.

PAST MEDICAL HISTORY:
Active Problem List:
- Resolved Hepatitis C Infection (diagnosed 2020-01-15)
- Opioid-induced secondary hypogonadism (diagnosed 2021-01-15)
- Naltrexone-induced opioid receptor blockade causing therapeutic failure of morphine (diagnosed 2023-01-15)
- Helicobacter pylori-associated gastric ulcer with hemorrhage (diagnosed 2026-01-15)
- Chronic lower back pain, post-traumatic, work-related
- Major depressive disorder
- History of opioid use disorder, in remission
- Secondary hypogonadism

PAST SURGICAL HISTORY:
- No prior surgeries

MEDICATIONS:
Home Medications:
- Oxycodone 10 mg PO every 4-6 hours as needed for back pain
- Alprazolam 0.5 mg PO twice daily as needed for anxiety

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Alcohol use disorder
- Mother: Major depressive disorder
- Older brother: Chronic low back pain

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Warehouse forklift operator.

REVIEW OF SYSTEMS:
He denies fever, cough, or chest pain.

VITAL SIGNS:
His vitals are: BP 130/85 mmHg, HR 88 bpm, RR 8/min, T 37.0°C, and SpO2 92% on room air.

PHYSICAL EXAMINATION:
On examination, the patient is somnolent but arousable, with shallow breathing.

LABORATORY DATA:
Arterial blood gas analysis reveals the following:
- pH: 7.28
- PaCO2: 60 mmHg
- PaO2: 70 mmHg
- HCO3-: 24 mEq/L

IMAGING:
The patient&#x27;s chest X-ray is unremarkable.
