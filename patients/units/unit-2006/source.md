# synthetic_hospital patient 2006

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex M, Non-Hispanic White, Commercial insurance through employer.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-06-21 (Mon) | outpatient | Orthopedics |
| 1 | 2020-04-15 | 2021-09-19 (Sun) | ed | Emergency Medicine |
| 2 | 2021-03-15 | 2022-08-17 (Wed) | ed | Emergency Medicine |
| 3 | 2022-03-15 | 2023-08-10 (Thu) | outpatient | Urology |
| 4 | 2023-03-15 | 2024-08-14 (Wed) | ed | Emergency Medicine |
| 5 | 2023-11-15 | 2025-04-07 (Mon) | ed | Emergency Medicine |
| 6 | 2024-09-15 | 2026-02-12 (Thu) | ed | Emergency Medicine |
| 7 | 2024-09-15 | 2026-02-12 (Thu) | icu | Pulmonary & Critical Care |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Allopurinol (hypersensitivity reaction)",
  "Sulfa drugs"
 ],
 "insurance": "Commercial insurance through employer",
 "occupation": "Construction worker",
 "alcohol_use": "Occasional social use (1-2 drinks per week)",
 "family_history": [
  "Father with recurrent kidney stones",
  "Mother with hypertension",
  "Paternal uncle with gout"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Former smoker, 15 pack-years, quit 3 years ago",
 "home_medications": [
  {
   "dose": "10 mg PO daily",
   "name": "Lisinopril"
  },
  {
   "dose": "12.5 mg PO daily",
   "name": "Hydrochlorothiazide"
  },
  {
   "dose": "500 mg PO PRN pain",
   "name": "Acetaminophen"
  }
 ],
 "surgical_history": [
  "Tonsillectomy"
 ],
 "chronic_conditions": [
  "Hypertension",
  "Gout",
  "Recurrent nephrolithiasis",
  "Chronic right shoulder rotator cuff pathology"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Rotator cuff tear of right shoulder", "Ureterolithiasis", "Drug-induced acute interstitial nephritis (Allopurinol-induced)", "Clear cell renal cell carcinoma", "Flail chest"]
Comorbidities: ["Essential hypertension", "Acute hypoxemic respiratory failure without hypercapnic encephalopathy"]

## Encounter 0: source 2020-01-15, practice 2021-06-21, outpatient, Orthopedics

Chief complaint: 6-month history of progressive right shoulder pain and weakness

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Orthopedics
Visit Type: outpatient

CHIEF COMPLAINT:
6-month history of progressive right shoulder pain and weakness

HISTORY OF PRESENT ILLNESS:
He reports that the pain is worse at night and often interferes with his sleep. He denies any recent trauma but notes that the pain is exacerbated by overhead activities.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypertension
- Gout
- Recurrent nephrolithiasis
- Chronic right shoulder rotator cuff pathology

PAST SURGICAL HISTORY:
- Tonsillectomy

MEDICATIONS:
He takes lisinopril.

Home Medications:
- Hydrochlorothiazide 12.5 mg PO daily
- Acetaminophen 500 mg PO PRN pain

ALLERGIES:
Allopurinol (hypersensitivity reaction), Sulfa drugs

FAMILY HISTORY:
- Father with recurrent kidney stones
- Mother with hypertension
- Paternal uncle with gout

SOCIAL HISTORY:
Smoking: Former smoker, 15 pack-years, quit 3 years ago. Alcohol: Occasional social use (1-2 drinks per week). Occupation: Construction worker.

PHYSICAL EXAMINATION:
On physical examination, there is tenderness to palpation over the lateral aspect of the shoulder. Active abduction of the right arm is limited by pain, and he is unable to lower the arm smoothly from an overhead position. Passive range of motion is full.

IMAGING:
An MRI of the shoulder reveals a high signal intensity area in the supraspinatus tendon consistent with a tear.

## Encounter 1: source 2020-04-15, practice 2021-09-19, ed, Emergency Medicine

Chief complaint: 2-week history of fatigue, low-grade fever, and generalized maculopapular rash

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
2-week history of fatigue, low-grade fever, and generalized maculopapular rash

HISTORY OF PRESENT ILLNESS:
Patient with hypertension, gout, chronic rotator cuff pathology reports recent history of severe sore throat treated with antibiotics.

PAST MEDICAL HISTORY:
Active Problem List:
- Rotator cuff tear of right shoulder (diagnosed 2020-01-15)
- Hypertension
- Gout
- Recurrent nephrolithiasis
- Chronic right shoulder rotator cuff pathology

His medical history is significant for hypertension, for which he takes hydrochlorothiazide, and gout, for which he started allopurinol 3 weeks ago.

PAST SURGICAL HISTORY:
- Tonsillectomy

MEDICATIONS:
His medical history is significant for hypertension, for which he takes hydrochlorothiazide, and gout, for which he started allopurinol 3 weeks ago.

Home Medications:
- Lisinopril 10 mg PO daily
- Acetaminophen 500 mg PO PRN pain

ALLERGIES:
Allopurinol (hypersensitivity reaction), Sulfa drugs

FAMILY HISTORY:
- Father with recurrent kidney stones
- Mother with hypertension
- Paternal uncle with gout

SOCIAL HISTORY:
Smoking: Former smoker, 15 pack-years, quit 3 years ago. Alcohol: Occasional social use (1-2 drinks per week). Occupation: Construction worker.

VITAL SIGNS:
Temperature of 37.8°C (100°F), blood pressure of 135/85 mmHg

PHYSICAL EXAMINATION:
On physical examination, the patient has a temperature of 37.8°C (100°F), blood pressure of 135/85 mmHg, and no signs of joint inflammation.

LABORATORY DATA:
Laboratory findings are as follows:

- Serum creatinine: 2.5 mg/dL (increased from a baseline of 1.0 mg/dL)
- Blood urea nitrogen (BUN): 40 mg/dL
- Urinalysis: White blood cell casts, no red blood cells
- Urine eosinophils: Positive
- Serum C-reactive protein (CRP): Elevated

## Encounter 2: source 2021-03-15, practice 2022-08-17, ed, Emergency Medicine

Chief complaint: Acute onset of severe left flank pain radiating to the groin with nausea and hematuria

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Emily Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute onset of severe left flank pain radiating to the groin with nausea and hematuria

HISTORY OF PRESENT ILLNESS:
The patient with recurrent nephrolithiasis presents with pain. Pain is colicky in nature. Denies recent trauma or infections.

PAST MEDICAL HISTORY:
Active Problem List:
- Rotator cuff tear of right shoulder (diagnosed 2020-01-15)
- Drug-induced acute interstitial nephritis (Allopurinol-induced) (diagnosed 2020-04-15)
- Hypertension
- Gout
- Recurrent nephrolithiasis
- Chronic right shoulder rotator cuff pathology

His past medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- Tonsillectomy

MEDICATIONS:
He takes hydrochlorothiazide.

Home Medications:
- Lisinopril 10 mg PO daily
- Acetaminophen 500 mg PO PRN pain

ALLERGIES:
Allopurinol (hypersensitivity reaction), Sulfa drugs

FAMILY HISTORY:
- Father with recurrent kidney stones
- Mother with hypertension
- Paternal uncle with gout

SOCIAL HISTORY:
Smoking: Former smoker, 15 pack-years, quit 3 years ago. Alcohol: Occasional social use (1-2 drinks per week). Occupation: Construction worker.

VITAL SIGNS:
Afebrile, blood pressure is 150/90 mmHg, and heart rate is 98 bpm

PHYSICAL EXAMINATION:
Abdominal examination reveals left costovertebral angle tenderness

LABORATORY DATA:
Urinalysis shows red blood cells but no signs of infection

IMAGING:
A non-contrast CT scan of the abdomen and pelvis reveals a 5 mm calculus in the left distal ureter without hydronephrosis

## Encounter 3: source 2022-03-15, practice 2023-08-10, outpatient, Urology

Chief complaint: 3-month history of dull, aching pain in left scrotum radiating to left flank

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. James Williams, MD
Department: Urology
Visit Type: outpatient

CHIEF COMPLAINT:
3-month history of dull, aching pain in left scrotum radiating to left flank

HISTORY OF PRESENT ILLNESS:
The patient is a 47-year-old male with recurrent nephrolithiasis, hypertension, gout, and rotator cuff pathology who presents with scrotal pain. He has prior histories of ureterolithiasis and rotator cuff tear. He reports a 3-month history of dull, aching pain in his left scrotum that sometimes radiates to his left flank.

PAST MEDICAL HISTORY:
Active Problem List:
- Rotator cuff tear of right shoulder (diagnosed 2020-01-15)
- Drug-induced acute interstitial nephritis (Allopurinol-induced) (diagnosed 2020-04-15)
- Ureterolithiasis (diagnosed 2021-03-15)
- Hypertension
- Gout
- Recurrent nephrolithiasis
- Chronic right shoulder rotator cuff pathology

The patient has a history of hypertension, which is well-controlled with lisinopril.

PAST SURGICAL HISTORY:
- Tonsillectomy

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Acetaminophen 500 mg PO PRN pain

ALLERGIES:
Allopurinol (hypersensitivity reaction), Sulfa drugs

FAMILY HISTORY:
- Father with recurrent kidney stones
- Mother with hypertension
- Paternal uncle with gout

SOCIAL HISTORY:
Smoking: Former smoker, 15 pack-years, quit 3 years ago. Alcohol: Occasional social use (1-2 drinks per week). Occupation: Construction worker.

REVIEW OF SYSTEMS:
He denies any fever, dysuria, or hematuria.

VITAL SIGNS:
His vital signs are within normal limits.

PHYSICAL EXAMINATION:
On physical examination, a mass is palpable above the left testicle, which feels like a "bag of worms." The mass does not reduce in size when the patient is in the supine position. The remainder of the genitourinary examination is unremarkable. The patient's abdominal examination is normal, and there are no palpable masses.

LABORATORY DATA:
Laboratory tests reveal:
- Hemoglobin: 14.5 g/dL
- White blood cell count: 7,000/uL
- Platelets: 250,000/uL
- Creatinine: 1.0 mg/dL
- Urinalysis: Normal

## Encounter 4: source 2023-03-15, practice 2024-08-14, ed, Emergency Medicine

Chief complaint: Right shoulder pain following fall from ladder onto outstretched arm

ENCOUNTER NOTE
Date: 2023-03-15
Provider: Dr. David Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Right shoulder pain following fall from ladder onto outstretched arm

HISTORY OF PRESENT ILLNESS:
The patient is a 48-year-old male with chronic right shoulder rotator cuff pathology, recurrent nephrolithiasis, and clear cell renal cell carcinoma who presents following trauma to his affected shoulder. He reports a 2-day history of right shoulder pain following a fall from a ladder, landing on his outstretched arm and immediately felt a sharp pain in his shoulder. Since the accident, he has had difficulty lifting objects above shoulder level.

PAST MEDICAL HISTORY:
Active Problem List:
- Rotator cuff tear of right shoulder (diagnosed 2020-01-15)
- Drug-induced acute interstitial nephritis (Allopurinol-induced) (diagnosed 2020-04-15)
- Ureterolithiasis (diagnosed 2021-03-15)
- Clear cell renal cell carcinoma (diagnosed 2022-03-15)
- Hypertension
- Gout
- Recurrent nephrolithiasis
- Chronic right shoulder rotator cuff pathology

His medical history is significant for hypertension, controlled with lisinopril.

PAST SURGICAL HISTORY:
- Tonsillectomy

MEDICATIONS:
Home Medications:
- Lisinopril 10 mg PO daily
- Hydrochlorothiazide 12.5 mg PO daily
- Acetaminophen 500 mg PO PRN pain

ALLERGIES:
Allopurinol (hypersensitivity reaction), Sulfa drugs

FAMILY HISTORY:
- Father with recurrent kidney stones
- Mother with hypertension
- Paternal uncle with gout

SOCIAL HISTORY:
Smoking: Former smoker, 15 pack-years, quit 3 years ago. Alcohol: Occasional social use (1-2 drinks per week). Occupation: Construction worker.

PHYSICAL EXAMINATION:
On physical examination, there is tenderness to palpation over the anterior aspect of the right shoulder. There is a noticeable decrease in strength during abduction and external rotation of the right arm. Passive range of motion is preserved.

IMAGING:
An MRI of the shoulder shows a high signal intensity area in the supraspinatus tendon consistent with a tear.

## Encounter 5: source 2023-11-15, practice 2025-04-07, ed, Emergency Medicine

Chief complaint: Severe left flank pain radiating to groin with nausea and vomiting

ENCOUNTER NOTE
Date: 2023-11-15
Provider: Dr. Lisa Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe left flank pain radiating to groin with nausea and vomiting

HISTORY OF PRESENT ILLNESS:
This patient with recurrent nephrolithiasis and prior left ureterolithiasis presents to the emergency department with severe left flank pain radiating to the groin, nausea, and vomiting for the past 6 hours.

PAST MEDICAL HISTORY:
Active Problem List:
- Rotator cuff tear of right shoulder (diagnosed 2020-01-15)
- Drug-induced acute interstitial nephritis (Allopurinol-induced) (diagnosed 2020-04-15)
- Ureterolithiasis (diagnosed 2021-03-15)
- Clear cell renal cell carcinoma (diagnosed 2022-03-15)
- Hypertension
- Gout
- Recurrent nephrolithiasis
- Chronic right shoulder rotator cuff pathology

He has a history of hypertension controlled with lisinopril.

PAST SURGICAL HISTORY:
- Tonsillectomy

MEDICATIONS:
He takes lisinopril.

Home Medications:
- Hydrochlorothiazide 12.5 mg PO daily
- Acetaminophen 500 mg PO PRN pain

ALLERGIES:
Allopurinol (hypersensitivity reaction), Sulfa drugs

FAMILY HISTORY:
- Father with recurrent kidney stones
- Mother with hypertension
- Paternal uncle with gout

SOCIAL HISTORY:
Smoking: Former smoker, 15 pack-years, quit 3 years ago. Alcohol: Occasional social use (1-2 drinks per week). Occupation: Construction worker.

REVIEW OF SYSTEMS:
He denies any fever or chills.

VITAL SIGNS:
On physical examination, he is afebrile, blood pressure is 150/90 mmHg, and heart rate is 98 bpm.

PHYSICAL EXAMINATION:
Abdominal examination reveals left costovertebral angle tenderness.

LABORATORY DATA:
Urinalysis shows hematuria. His serum creatinine is 1.2 mg/dL (baseline 1.0 mg/dL), and white blood cell count is 12,000/uL.

IMAGING:
Non-contrast CT of the abdomen and pelvis reveals a 7 mm stone at the ureterovesical junction and mild hydronephrosis without signs of infection.

## Encounter 6: source 2024-09-15, practice 2026-02-12, ed, Emergency Medicine

Chief complaint: Severe left-sided chest pain after fall from second-story scaffold

ENCOUNTER NOTE
Date: 2024-09-15
Provider: Dr. Michael Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe left-sided chest pain after fall from second-story scaffold

HISTORY OF PRESENT ILLNESS:
This 49-year-old male construction worker with hypertension, gout, recurrent nephrolithiasis, and chronic right shoulder rotator cuff pathology, including a prior fall from a ladder in 2023, presents to the emergency department following a fall from a second-story scaffold. He complains of severe pain on the left side of his chest, exacerbated by deep breathing and coughing. He denies loss of consciousness, headache, or vomiting.

PAST MEDICAL HISTORY:
Active Problem List:
- Rotator cuff tear of right shoulder (diagnosed 2020-01-15)
- Drug-induced acute interstitial nephritis (Allopurinol-induced) (diagnosed 2020-04-15)
- Ureterolithiasis (diagnosed 2021-03-15)
- Clear cell renal cell carcinoma (diagnosed 2022-03-15)
- Hypertension
- Gout
- Recurrent nephrolithiasis
- Chronic right shoulder rotator cuff pathology

His medical history is significant for hypertension, controlled with lisinopril.

PAST SURGICAL HISTORY:
- Tonsillectomy

MEDICATIONS:
He takes lisinopril.

Home Medications:
- Hydrochlorothiazide 12.5 mg PO daily
- Acetaminophen 500 mg PO PRN pain

ALLERGIES:
Allopurinol (hypersensitivity reaction), Sulfa drugs

FAMILY HISTORY:
- Father with recurrent kidney stones
- Mother with hypertension
- Paternal uncle with gout

SOCIAL HISTORY:
Smoking: Former smoker, 15 pack-years, quit 3 years ago. Alcohol: Occasional social use (1-2 drinks per week). Occupation: Construction worker.

VITAL SIGNS:
Respiratory rate of 28 breaths per minute

PHYSICAL EXAMINATION:
On examination, you note that the patient is tachypneic and appears to be in distress. Auscultation of the chest reveals decreased breath sounds over the left lower lung fields. Palpation of the chest wall elicits severe pain and you observe paradoxical movement of the left chest wall during inspiration.

LABORATORY DATA:
An arterial blood gas shows the following:

- pH: 7.45
- PaCO2: 32 mmHg
- PaO2: 60 mmHg
- HCO3-: 24 mEq/L
- O2 saturation: 90%

IMAGING:
A chest X-ray confirms the presence of multiple rib fractures involving the fourth through seventh ribs in more than two locations on the left side.

## Encounter 7: source 2024-09-15, practice 2026-02-12, icu, Pulmonary & Critical Care

Chief complaint: Continued respiratory distress and chest wall instability following scaffold fall

ENCOUNTER NOTE
Date: 2024-09-15
Provider: Dr. Priya Patel, MD
Department: Pulmonary & Critical Care
Visit Type: icu

CHIEF COMPLAINT:
Continued respiratory distress and chest wall instability following scaffold fall

HISTORY OF PRESENT ILLNESS:
Following ED evaluation for a scaffold fall, this 49-year-old with shoulder pathology and nephrolithiasis complains of severe pain on the left side of his chest, exacerbated by deep breathing and coughing.

PAST MEDICAL HISTORY:
Active Problem List:
- Rotator cuff tear of right shoulder (diagnosed 2020-01-15)
- Drug-induced acute interstitial nephritis (Allopurinol-induced) (diagnosed 2020-04-15)
- Ureterolithiasis (diagnosed 2021-03-15)
- Clear cell renal cell carcinoma (diagnosed 2022-03-15)
- Flail chest (diagnosed 2024-09-15)
- Hypertension
- Gout
- Recurrent nephrolithiasis
- Chronic right shoulder rotator cuff pathology

The patient&#x27;s past medical history is significant for hypertension.

PAST SURGICAL HISTORY:
- Tonsillectomy

MEDICATIONS:
He takes hydrochlorothiazide.

Home Medications:
- Lisinopril 10 mg PO daily
- Acetaminophen 500 mg PO PRN pain

ALLERGIES:
Allopurinol (hypersensitivity reaction), Sulfa drugs

FAMILY HISTORY:
- Father with recurrent kidney stones
- Mother with hypertension
- Paternal uncle with gout

SOCIAL HISTORY:
Smoking: Former smoker, 15 pack-years, quit 3 years ago. Alcohol: Occasional social use (1-2 drinks per week). Occupation: Construction worker.

VITAL SIGNS:
He is in moderate respiratory distress with a respiratory rate of 28 breaths per minute. His oxygen saturation is 92% on room air. His temperature is 37.2°C (99°F), blood pressure is 145/90 mmHg, and pulse is 110 beats per minute.

PHYSICAL EXAMINATION:
On examination, there is obvious deformity of the left chest wall with paradoxical movement noted during inspiration. Auscultation reveals decreased breath sounds over the left lung field.

LABORATORY DATA:
Arterial blood gas shows:

- pH: 7.35
- PaCO2: 45 mmHg
- PaO2: 60 mmHg
- HCO3-: 24 mEq/L

IMAGING:
Chest X-ray confirms multiple rib fractures on the left side with no evidence of pneumothorax.
