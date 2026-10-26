# synthetic_hospital patient 2115

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex M, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-11-18 (Tue) | ed | Emergency Medicine |
| 1 | 2020-03-15 | 2026-01-18 (Sun) | ed | Emergency Medicine |
| 2 | 2020-09-15 | 2026-07-18 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Penicillin - anaphylaxis"
 ],
 "insurance": "Medicaid",
 "occupation": "Construction laborer",
 "alcohol_use": "Abstinent for 2 years; prior heavy use",
 "family_history": [
  "Father: Deceased age 54, hepatocellular carcinoma secondary to alcohol-induced cirrhosis",
  "Mother: Living, hypertension, type 2 diabetes mellitus",
  "Brother: Chronic Hepatitis C infection"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 1 pack per day for 20 years",
 "home_medications": [
  {
   "dose": "300 mg PO daily",
   "name": "Tenofovir disoproxil fumarate"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "4 mg intranasal PRN overdose",
   "name": "Naloxone nasal spray"
  },
  {
   "dose": "500 mg PO PRN pain",
   "name": "Acetaminophen"
  }
 ],
 "surgical_history": [
  "Incision and drainage of left forearm abscess, 2019",
  "Appendectomy, 2005"
 ],
 "chronic_conditions": [
  "Chronic Hepatitis B infection",
  "Opioid use disorder, severe, with recent relapse",
  "Hypertension"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Acute bacterial vertebral osteomyelitis (thoracic)", "Hepatitis D Virus Infection", "Septic arthritis"]
Comorbidities: ["Intravenous Drug Use (Substance Use Disorder)"]

## Encounter 0: source 2020-01-15, practice 2025-11-18, ed, Emergency Medicine

Chief complaint: New-onset jaundice, malaise, and right upper quadrant abdominal pain

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
New-onset jaundice, malaise, and right upper quadrant abdominal pain

HISTORY OF PRESENT ILLNESS:
He reports dark-colored urine and pale stools over the past week, along with low-grade fevers and nausea.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic Hepatitis B infection
- Opioid use disorder, severe, with recent relapse
- Hypertension

PAST SURGICAL HISTORY:
- Incision and drainage of left forearm abscess, 2019
- Appendectomy, 2005

MEDICATIONS:
Home Medications:
- Tenofovir disoproxil fumarate 300 mg PO daily
- Lisinopril 10 mg PO daily
- Naloxone nasal spray 4 mg intranasal PRN overdose
- Acetaminophen 500 mg PO PRN pain

ALLERGIES:
Penicillin - anaphylaxis

FAMILY HISTORY:
- Father: Deceased age 54, hepatocellular carcinoma secondary to alcohol-induced cirrhosis
- Mother: Living, hypertension, type 2 diabetes mellitus
- Brother: Chronic Hepatitis C infection

SOCIAL HISTORY:
He recently returned from a trip to Eastern Europe and admits to sharing injection drug equipment during the trip.

PHYSICAL EXAMINATION:
Physical exam reveals scleral icterus and hepatomegaly.

LABORATORY DATA:
Laboratory evaluation shows markedly elevated transaminases and total bilirubin. Serologic testing is positive for hepatitis D.

## Encounter 1: source 2020-03-15, practice 2026-01-18, ed, Emergency Medicine

Chief complaint: Fever, chills, and severe pain in right knee with inability to bear weight

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Fever, chills, and severe pain in right knee with inability to bear weight

HISTORY OF PRESENT ILLNESS:
This 45-year-old male with chronic Hepatitis B, recent Hepatitis D superinfection, and severe opioid use disorder with recent relapse returns to the emergency department with a 2-day history of fever, chills, and severe pain in his right knee. He reports that the pain has progressively worsened and he is now unable to bear weight on the affected leg.

PAST MEDICAL HISTORY:
Active Problem List:
- Hepatitis D Virus Infection (diagnosed 2020-01-15)
- Chronic Hepatitis B infection
- Opioid use disorder, severe, with recent relapse
- Hypertension

PAST SURGICAL HISTORY:
- Incision and drainage of left forearm abscess, 2019
- Appendectomy, 2005

MEDICATIONS:
Home Medications:
- Tenofovir disoproxil fumarate 300 mg PO daily
- Lisinopril 10 mg PO daily
- Naloxone nasal spray 4 mg intranasal PRN overdose
- Acetaminophen 500 mg PO PRN pain

ALLERGIES:
Penicillin - anaphylaxis

FAMILY HISTORY:
- Father: Deceased age 54, hepatocellular carcinoma secondary to alcohol-induced cirrhosis
- Mother: Living, hypertension, type 2 diabetes mellitus
- Brother: Chronic Hepatitis C infection

SOCIAL HISTORY:
History of intravenous drug use

VITAL SIGNS:
Temperature is 38.9°C (102°F), heart rate is 110 bpm, and blood pressure is 130/85 mmHg

PHYSICAL EXAMINATION:
The right knee is swollen, erythematous, and warm to the touch with a markedly decreased range of motion due to pain.

LABORATORY DATA:
White blood cell count: 18,000/uL
- Erythrocyte sedimentation rate (ESR): 85 mm/hr
- C-reactive protein (CRP): 25 mg/dL
- Synovial fluid analysis: WBC 50,000/uL with 90% neutrophils, negative Gram stain

## Encounter 2: source 2020-09-15, practice 2026-07-18, ed, Emergency Medicine

Chief complaint: Severe constant thoracic back pain for two weeks with fever and night sweats

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe constant thoracic back pain for two weeks with fever and night sweats

HISTORY OF PRESENT ILLNESS:
This 45-year-old male with chronic Hepatitis B, prior Hepatitis D superinfection, and septic arthritis earlier this year returns to the emergency department complaining of severe back pain for the past two weeks. He describes the pain as constant and unrelieved by rest or over-the-counter analgesics. He denies any recent trauma but reports a low-grade fever and night sweats.

PAST MEDICAL HISTORY:
Active Problem List:
- Hepatitis D Virus Infection (diagnosed 2020-01-15)
- Septic arthritis (diagnosed 2020-03-15)
- Chronic Hepatitis B infection
- Opioid use disorder, severe, with recent relapse
- Hypertension

PAST SURGICAL HISTORY:
- Incision and drainage of left forearm abscess, 2019
- Appendectomy, 2005

MEDICATIONS:
Home Medications:
- Tenofovir disoproxil fumarate 300 mg PO daily
- Lisinopril 10 mg PO daily
- Naloxone nasal spray 4 mg intranasal PRN overdose
- Acetaminophen 500 mg PO PRN pain

ALLERGIES:
Penicillin - anaphylaxis

FAMILY HISTORY:
- Father: Deceased age 54, hepatocellular carcinoma secondary to alcohol-induced cirrhosis
- Mother: Living, hypertension, type 2 diabetes mellitus
- Brother: Chronic Hepatitis C infection

SOCIAL HISTORY:
Intravenous drug use

PHYSICAL EXAMINATION:
On physical examination, there is marked tenderness to gentle percussion over the T10 spinous process. Neurological examination is unremarkable.

LABORATORY DATA:
White blood cell count: 12,000/uL (normal: 4,500-11,000/uL)
- Hemoglobin: 13.5 g/dL (normal: 13.8-17.2 g/dL)
- Erythrocyte sedimentation rate (ESR): 85 mm/hr (normal: 0-22 mm/hr)
- C-reactive protein (CRP): 25 mg/L (normal: <10 mg/L)

IMAGING:
An MRI of the thoracic spine is ordered.
