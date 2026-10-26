# synthetic_hospital patient 2007

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 56 at first encounter, sex M, Non-Hispanic White, Preferred Provider Organization (PPO) through employer.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-12-09 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-09-15 | 2025-08-01 (Fri) | inpatient | General Surgery |
| 2 | 2021-04-15 | 2026-03-14 (Sat) | icu | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Preferred Provider Organization (PPO) through employer",
 "occupation": "High school science teacher",
 "alcohol_use": "Social, approximately 2-3 drinks per month",
 "family_history": [
  "Father: Sudden cardiac death at age 64, hypertension",
  "Mother: Type 2 diabetes mellitus, hyperlipidemia",
  "Sibling: Hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 20 pack-year history, quit 5 years ago",
 "home_medications": [
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "20 mg PO daily",
   "name": "Atorvastatin"
  },
  {
   "dose": "500 mg PO PRN for musculoskeletal pain",
   "name": "Acetaminophen"
  }
 ],
 "surgical_history": [
  "Abdominal surgery (recent, complicated by postoperative paralytic ileus)"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Hypercholesterolemia",
  "Chronic musculoskeletal lower back pain"
 ],
 "age_at_first_encounter": 56
}
```

Primary diagnoses: ["Ventricular fibrillation (refractory)", "Sinus tachycardia secondary to burns", "Paralytic ileus"]
Comorbidities: ["Essential hypertension"]

## Encounter 0: source 2020-01-15, practice 2024-12-09, outpatient, Internal Medicine

Chief complaint: Follow-up visit for recently diagnosed hypertension with mild lightheadedness when standing quickly after starting new medication 10 days ago.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up visit for recently diagnosed hypertension with mild lightheadedness when standing quickly after starting new medication 10 days ago.

HISTORY OF PRESENT ILLNESS:
At his last visit, his blood pressure was 158/96 mm Hg, and his physician started him on enalapril. Today, he reports mild lightheadedness when standing up quickly but no other symptoms. He has been taking the medication consistently for the past 10 days.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Hypercholesterolemia
- Chronic musculoskeletal lower back pain

He has no other known medical conditions.

PAST SURGICAL HISTORY:
- Abdominal surgery (recent, complicated by postoperative paralytic ileus)

MEDICATIONS:
He does not take any medications.

Home Medications:
- Lisinopril 10 mg PO daily
- Atorvastatin 20 mg PO daily
- Acetaminophen 500 mg PO PRN for musculoskeletal pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Sudden cardiac death at age 64, hypertension
- Mother: Type 2 diabetes mellitus, hyperlipidemia
- Sibling: Hypertension

SOCIAL HISTORY:
He has a 20-pack-year smoking history but quit 5 years ago.

REVIEW OF SYSTEMS:
He has no chest pain, shortness of breath, dizziness, or palpitations. He denies orthopnea, paroxysmal nocturnal dyspnea, or leg swelling.

VITAL SIGNS:
Vital signs today are: blood pressure 120/78 mm Hg and heart rate 84/min.

PHYSICAL EXAMINATION:
Physical exam is normal except for trace postural lightheadedness without orthostatic hypotension.

LABORATORY DATA:
Basic metabolic panel shows normal creatinine and electrolytes.

OTHER STUDIES:
ECG shows normal sinus rhythm.

## Encounter 1: source 2020-09-15, practice 2025-08-01, inpatient, General Surgery

Chief complaint: Increasing abdominal distention, poor appetite, nausea with vomiting, and absence of stool or flatus for 3 days following abdominal surgery.

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. James Okonkwo, MD
Department: General Surgery
Visit Type: inpatient

CHIEF COMPLAINT:
Increasing abdominal distention, poor appetite, nausea with vomiting, and absence of stool or flatus for 3 days following abdominal surgery.

HISTORY OF PRESENT ILLNESS:
A 56-year-old male with hypertension and hypercholesterolemia, evaluated for orthostatic symptoms, presents postoperatively. Since the operation, he has experienced increasing abdominal distention, poor appetite, and repeated episodes of nausea with vomiting. He has not passed stool or flatus during this period.

PAST MEDICAL HISTORY:
Active Problem List:
- Sinus tachycardia secondary to burns (diagnosed 2020-01-15)
- Hypertension
- Hypercholesterolemia
- Chronic musculoskeletal lower back pain

His past medical history includes hypertension and chronic musculoskeletal pain.

PAST SURGICAL HISTORY:
- Abdominal surgery (recent, complicated by postoperative paralytic ileus)

MEDICATIONS:
He takes for which he takes intermittent analgesics.

Home Medications:
- Lisinopril 10 mg PO daily
- Atorvastatin 20 mg PO daily
- Acetaminophen 500 mg PO PRN for musculoskeletal pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Sudden cardiac death at age 64, hypertension
- Mother: Type 2 diabetes mellitus, hyperlipidemia
- Sibling: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-year history, quit 5 years ago. Alcohol: Social, approximately 2-3 drinks per month. Occupation: High school science teacher.

REVIEW OF SYSTEMS:
He denies fevers, chills, chest pain, or urinary complaints.

PHYSICAL EXAMINATION:
On examination, the abdomen is distended but soft, with diffuse mild tenderness on palpation. No guarding, rebound, or focal masses are appreciated. Bowel sounds are markedly reduced, and the surgical incision appears clean without erythema or drainage.

LABORATORY DATA:
Laboratory studies show mild electrolyte abnormalities.

## Encounter 2: source 2021-04-15, practice 2026-03-14, icu, Emergency Medicine

Chief complaint: Sudden collapse and cardiac arrest while playing basketball at local gym; unresponsive with bystander CPR in progress upon EMS arrival.

ENCOUNTER NOTE
Date: 2021-04-15
Provider: Dr. Maria Rodriguez, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Sudden collapse and cardiac arrest while playing basketball at local gym; unresponsive with bystander CPR in progress upon EMS arrival.

HISTORY OF PRESENT ILLNESS:
The 57-year-old with hypertension, hypercholesterolemia, and chronic musculoskeletal back pain has history of orthostatic symptoms on antihypertensives and postoperative ileus. Bystanders note that he is unresponsive and not breathing. One bystander, trained in basic life support, initiates cardiopulmonary resuscitation (CPR) while another calls emergency services. On arrival, emergency medical services (EMS) find the bystander performing high-quality CPR. After two minutes of CPR,

PAST MEDICAL HISTORY:
Active Problem List:
- Sinus tachycardia secondary to burns (diagnosed 2020-01-15)
- Paralytic ileus (diagnosed 2020-09-15)
- Hypertension
- Hypercholesterolemia
- Chronic musculoskeletal lower back pain

The patient has a history of hypertension and hypercholesterolemia.

PAST SURGICAL HISTORY:
- Abdominal surgery (recent, complicated by postoperative paralytic ileus)

MEDICATIONS:
He takes for which he takes lisinopril and atorvastatin.

Home Medications:
- Acetaminophen 500 mg PO PRN for musculoskeletal pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Sudden cardiac death at age 64, hypertension
- Mother: Type 2 diabetes mellitus, hyperlipidemia
- Sibling: Hypertension

SOCIAL HISTORY:
Smoking: Former smoker, 20 pack-year history, quit 5 years ago. Alcohol: Social, approximately 2-3 drinks per month. Occupation: High school science teacher.

PHYSICAL EXAMINATION:
The patient has no pulse

OTHER STUDIES:
The cardiac monitor shows ventricular fibrillation
