# synthetic_hospital patient 2765

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex F, Non-Hispanic White, Private commercial insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-12-31 (Tue) | outpatient | Internal Medicine |
| 1 | 2021-07-15 | 2026-06-28 (Sun) | ed | Emergency Medicine |
| 2 | 2021-08-15 | 2026-08-01 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private commercial insurance",
 "occupation": "Administrative assistant",
 "alcohol_use": "Chronic daily alcohol use, approximately 3-4 standard drinks per day",
 "family_history": [
  "Mother with cholelithiasis",
  "Father with type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "100 mg PO daily",
   "name": "Thiamine"
  },
  {
   "dose": "1 mg PO daily",
   "name": "Folic acid"
  },
  {
   "dose": "400 mg PO every 6 hours PRN pain",
   "name": "Ibuprofen"
  }
 ],
 "surgical_history": [
  "Endoscopic retrograde cholangiopancreatography (ERCP)"
 ],
 "chronic_conditions": [
  "Cholelithiasis",
  "Alcohol use disorder"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Acute pancreatitis", "Acute calculous cholecystitis", "Cholelithiasis"]
Comorbidities: ["Cholelithiasis"]

## Encounter 0: source 2020-01-15, practice 2024-12-31, outpatient, Internal Medicine

Chief complaint: Health maintenance examination with 4-year history of episodic postprandial right upper quadrant pain

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Health maintenance examination with 4-year history of episodic postprandial right upper quadrant pain

HISTORY OF PRESENT ILLNESS:
She has a 4-year history of episodic colicky right upper quadrant pain that is worst after meals.

PAST MEDICAL HISTORY:
Active Problem List:
- Cholelithiasis
- Alcohol use disorder

PAST SURGICAL HISTORY:
- Endoscopic retrograde cholangiopancreatography (ERCP)

MEDICATIONS:
Home Medications:
- Thiamine 100 mg PO daily
- Folic acid 1 mg PO daily
- Ibuprofen 400 mg PO every 6 hours PRN pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with cholelithiasis
- Father with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Chronic daily alcohol use, approximately 3-4 standard drinks per day. Occupation: Administrative assistant.

ASSESSMENT AND PLAN:
The physician discusses with her the possibility of an elective cholecystectomy, which she is immediately opposed to. She remarks that she had a close friend who had an appendectomy and got a "big infection" from the hospital, so she is scared of a hospital-acquired infection. She wants to know the chance that she would have a hospital-acquired infection.

The physician researches this information and finds that 95% of those admitted to hospital for cholecystectomy undergo laparoscopic removal, in which 1% get an infection. 5% undergo open removal, in which 4% get an infection.

## Encounter 1: source 2021-07-15, practice 2026-06-28, ed, Emergency Medicine

Chief complaint: Severe right upper quadrant abdominal pain radiating to right shoulder after eating a fatty meal

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe right upper quadrant abdominal pain radiating to right shoulder after eating a fatty meal

HISTORY OF PRESENT ILLNESS:
The patient presents to the emergency department with progression of her known cholelithiasis, previously evaluated in January 2020 for a four-year history of episodic postprandial right upper quadrant pain. She reports a 6-hour history of severe right upper quadrant (RUQ) abdominal pain that radiates to her right shoulder. She reports that the pain started after eating a fatty meal and has been constant since onset. She also complains of nausea and has vomited twice. She has a history of multiple episodes of similar, but less severe, pain over the past year.

PAST MEDICAL HISTORY:
Active Problem List:
- Cholelithiasis (diagnosed 2020-01-15)
- Alcohol use disorder

PAST SURGICAL HISTORY:
- Endoscopic retrograde cholangiopancreatography (ERCP)

MEDICATIONS:
Home Medications:
- Thiamine 100 mg PO daily
- Folic acid 1 mg PO daily
- Ibuprofen 400 mg PO every 6 hours PRN pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with cholelithiasis
- Father with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Chronic daily alcohol use, approximately 3-4 standard drinks per day. Occupation: Administrative assistant.

VITAL SIGNS:
Her temperature is 38.3°C (101°F), heart rate is 102 beats per minute, and blood pressure is 135/85 mmHg.

PHYSICAL EXAMINATION:
On physical examination, she has marked tenderness in the RUQ, and Murphy's sign is positive.

LABORATORY DATA:
Laboratory results are as follows:

- White blood cell count: 14,000/mm³ (normal: 4,500-11,000/mm³)
- Hemoglobin: 13.2 g/dL (normal: 12-16 g/dL)
- Alanine aminotransferase (ALT): 45 U/L (normal: 7-56 U/L)
- Aspartate aminotransferase (AST): 50 U/L (normal: 8-48 U/L)
- Alkaline phosphatase (ALP): 120 U/L (normal: 40-129 U/L)
- Total bilirubin: 1.2 mg/dL (normal: 0.3-1.9 mg/dL)

IMAGING:
An abdominal ultrasound shows gallstones and a thickened gallbladder wall with pericholecystic fluid.

## Encounter 2: source 2021-08-15, practice 2026-08-01, ed, Emergency Medicine

Chief complaint: Severe epigastric pain radiating to the back with nausea and vomiting following recent ERCP

ENCOUNTER NOTE
Date: 2021-08-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe epigastric pain radiating to the back with nausea and vomiting following recent ERCP

HISTORY OF PRESENT ILLNESS:
The patient returns to the emergency department with cholelithiasis initially diagnosed in January 2020 and recent acute calculous cholecystitis in July 2021. She reports that the pain started 6 hours ago, shortly after undergoing an endoscopic retrograde cholangiopancreatography (ERCP) for suspected choledocholithiasis. She describes the pain as constant and sharp, and it is exacerbated by lying flat. She also complains of nausea and has vomited twice since the onset of pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Cholelithiasis (diagnosed 2020-01-15)
- Acute calculous cholecystitis (diagnosed 2021-07-15)
- Alcohol use disorder

PAST SURGICAL HISTORY:
- Endoscopic retrograde cholangiopancreatography (ERCP)

MEDICATIONS:
Home Medications:
- Thiamine 100 mg PO daily
- Folic acid 1 mg PO daily
- Ibuprofen 400 mg PO every 6 hours PRN pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with cholelithiasis
- Father with type 2 diabetes mellitus

SOCIAL HISTORY:
Chronic alcohol use

VITAL SIGNS:
Blood pressure is 130/85 mmHg, heart rate is 102 beats per minute, and temperature is 37.8°C (100°F)

PHYSICAL EXAMINATION:
Abdominal examination reveals epigastric tenderness without rebound or guarding. Bowel sounds are normal.

LABORATORY DATA:
- White blood cell count: 12,000 /µL
- Hemoglobin: 13.5 g/dL
- Lipase: 900 U/L (Normal: 0-160 U/L)
- Amylase: 450 U/L (Normal: 30-110 U/L)
- Liver function tests: Within normal limits
