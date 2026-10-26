# synthetic_hospital patient 2544

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 29 at first encounter, sex M, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-10-29 (Tue) | ed | Emergency Medicine |
| 1 | 2020-07-15 | 2025-04-23 (Wed) | ed | Emergency Medicine |
| 2 | 2021-01-15 | 2025-10-30 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Construction laborer",
 "alcohol_use": "Occasional, 2-3 beers on weekends",
 "family_history": [
  "Father: Alcohol use disorder, hypertension",
  "Mother: Major depressive disorder, fibromyalgia",
  "Older brother: Chronic low back pain"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 1/2 pack per day",
 "home_medications": [
  {
   "dose": "50 mg PO daily",
   "name": "Sertraline"
  },
  {
   "dose": "5-10 mg PO every 6 hours as needed for back pain",
   "name": "Oxycodone"
  },
  {
   "dose": "600 mg PO every 8 hours as needed for pain",
   "name": "Ibuprofen"
  }
 ],
 "surgical_history": [
  "Lumbar microdiscectomy L5-S1 (2 years ago)"
 ],
 "chronic_conditions": [
  "Chronic lower back pain",
  "Major depressive disorder",
  "Opioid use disorder"
 ],
 "age_at_first_encounter": 29
}
```

Primary diagnoses: ["Opioid Withdrawal"]
Comorbidities: ["Intravenous Drug Use (Substance Use Disorder)", "Chronic back pain"]

## Encounter 0: source 2020-01-15, practice 2024-10-29, ed, Emergency Medicine

Chief complaint: Nausea, abdominal cramps, and diarrhea

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Nausea, abdominal cramps, and diarrhea

HISTORY OF PRESENT ILLNESS:
He presents to the emergency department with a 2-day history of nausea, abdominal cramps, and diarrhea. He reports feeling extremely anxious and has not been able to sleep due to restlessness. He also mentions a runny nose and tearing eyes.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic lower back pain
- Major depressive disorder
- Opioid use disorder

His past medical history is significant for chronic back pain for which he was prescribed oxycodone, but he stopped taking it 3 days ago.

PAST SURGICAL HISTORY:
- Lumbar microdiscectomy L5-S1 (2 years ago)

MEDICATIONS:
Home Medications:
- Sertraline 50 mg PO daily
- Oxycodone 5-10 mg PO every 6 hours as needed for back pain
- Ibuprofen 600 mg PO every 8 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder, hypertension
- Mother: Major depressive disorder, fibromyalgia
- Older brother: Chronic low back pain

SOCIAL HISTORY:
Smoking: Current smoker, 1/2 pack per day. Alcohol: Occasional, 2-3 beers on weekends. Occupation: Construction laborer.

VITAL SIGNS:
Blood pressure is 160/90 mmHg, heart rate is 110 beats per minute, temperature is 37.8°C (100°F)

PHYSICAL EXAMINATION:
On examination, his pupils are dilated, and skin is noted to have piloerection. Bowel sounds are hyperactive. The remainder of the physical examination is unremarkable.

LABORATORY DATA:
Laboratory values:
- CBC: Within normal limits
- Serum electrolytes: Within normal limits
- Urine toxicology screen: Positive for opioids

## Encounter 1: source 2020-07-15, practice 2025-04-23, ed, Emergency Medicine

Chief complaint: Increasing anxiety, abdominal cramps, nausea, and diarrhea

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Increasing anxiety, abdominal cramps, nausea, and diarrhea

HISTORY OF PRESENT ILLNESS:
The patient with opioid use disorder returns with symptoms similar to his January 2020 presentation. He reports muscle aches, runny nose, inability to stop yawning, and feeling unusually hot and cold.

PAST MEDICAL HISTORY:
Active Problem List:
- Opioid Withdrawal (diagnosed 2020-01-15)
- Chronic lower back pain
- Major depressive disorder
- Opioid use disorder

He has a past medical history of chronic back pain for which he was prescribed oxycodone, but he stopped taking it 3 days ago because he ran out of his prescription.

PAST SURGICAL HISTORY:
- Lumbar microdiscectomy L5-S1 (2 years ago)

MEDICATIONS:
Home Medications:
- Sertraline 50 mg PO daily
- Oxycodone 5-10 mg PO every 6 hours as needed for back pain
- Ibuprofen 600 mg PO every 8 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder, hypertension
- Mother: Major depressive disorder, fibromyalgia
- Older brother: Chronic low back pain

SOCIAL HISTORY:
He denies any other drug use

VITAL SIGNS:
His vital signs are as follows: temperature 37.2°C, blood pressure 150/90 mmHg, heart rate 102 beats per minute, and respiratory rate 16 breaths per minute.

PHYSICAL EXAMINATION:
On examination, he is alert and oriented, but appears restless. Pupils are dilated and reactive to light. Skin is warm and moist with piloerection noted on the arms. The remainder of the physical examination is unremarkable.

LABORATORY DATA:
His urine toxicology screen is negative except for opioids

## Encounter 2: source 2021-01-15, practice 2025-10-30, ed, Emergency Medicine

Chief complaint: Increasing muscle and joint aches, nausea, diarrhea, and abdominal cramping

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Increasing muscle and joint aches, nausea, diarrhea, and abdominal cramping

HISTORY OF PRESENT ILLNESS:
The patient with chronic lower back pain on oxycodone returns with recurrent symptoms similar to prior presentations in January and July 2020. He reports a runny nose and excessive tearing. He appears anxious and agitated. The patient reluctantly admits to using heroin "a few times a week" but states he has not used any in the past 72 hours.

PAST MEDICAL HISTORY:
Active Problem List:
- Opioid Withdrawal (diagnosed 2020-01-15)
- Chronic lower back pain
- Major depressive disorder
- Opioid use disorder

His past medical history is significant for depression and chronic back pain.

PAST SURGICAL HISTORY:
- Lumbar microdiscectomy L5-S1 (2 years ago)

MEDICATIONS:
Home Medications:
- Sertraline 50 mg PO daily
- Oxycodone 5-10 mg PO every 6 hours as needed for back pain
- Ibuprofen 600 mg PO every 8 hours as needed for pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Alcohol use disorder, hypertension
- Mother: Major depressive disorder, fibromyalgia
- Older brother: Chronic low back pain

SOCIAL HISTORY:
Smoking: Current smoker, 1/2 pack per day. Alcohol: Occasional, 2-3 beers on weekends. Occupation: Construction laborer.

VITAL SIGNS:
On physical examination, his vital signs are: blood pressure 150/90 mmHg, heart rate 110 beats per minute, respiratory rate 22 breaths per minute, and temperature 37.8°C (100°F).

PHYSICAL EXAMINATION:
Pupils are noted to be bilaterally dilated and reactive to light. The remainder of the exam is unremarkable. No skin lesions or track marks are noted.
