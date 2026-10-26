# synthetic_hospital patient 1842

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 27 at first encounter, sex F, Non-Hispanic White, University student health insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-09-19 (Tue) | outpatient | Gastroenterology |
| 1 | 2020-10-15 | 2024-06-10 (Mon) | ed | Emergency Medicine |
| 2 | 2021-10-15 | 2025-06-11 (Wed) | inpatient | Obstetrics & Gynecology |
| 3 | 2022-10-15 | 2026-06-08 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "University student health insurance",
 "occupation": "Graduate student in public health",
 "alcohol_use": "None",
 "family_history": [
  "Mother with hypothyroidism",
  "Father with hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "500 mg orally daily",
   "name": "Valacyclovir"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Genital herpes simplex virus type 2 infection"
 ],
 "age_at_first_encounter": 27
}
```

Primary diagnoses: ["Acute Hepatitis E", "Giardiasis", "Neonatal herpes simplex virus infection (disseminated or CNS)", "Carbon monoxide poisoning without airway compromise"]
Comorbidities: ["Term pregnancy", "Smoke inhalation exposure"]

## Encounter 0: source 2020-01-15, practice 2023-09-19, outpatient, Gastroenterology

Chief complaint: Loose, greasy stools, increased flatulence, and cramping abdominal pain for 10 days after returning from a backpacking trip through rural northern India.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Loose, greasy stools, increased flatulence, and cramping abdominal pain for 10 days after returning from a backpacking trip through rural northern India.

HISTORY OF PRESENT ILLNESS:
10-day history of loose, greasy stools, increased flatulence, and cramping abdominal pain. She recently returned from a 2-week backpacking trip through rural parts of northern India, where she drank untreated well water. She has not had fever, nausea, or vomiting. Her last menstrual period was 2 weeks ago, and she is sexually active with her male partner, using condoms inconsistently.

PAST MEDICAL HISTORY:
Active Problem List:
- Genital herpes simplex virus type 2 infection

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Valacyclovir 500 mg orally daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with hypothyroidism
- Father with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Graduate student in public health.

VITAL SIGNS:
Temperature is 36.9°C (98.4°F), blood pressure is 112/70 mm Hg, pulse is 78/min, and respirations are 16/min.

PHYSICAL EXAMINATION:
Abdominal examination shows mild distention with hyperactive bowel sounds and diffuse tenderness to deep palpation without rebound or guarding.

LABORATORY DATA:
Stool examination reveals fatty droplets and occasional cysts. Pregnancy test is negative.

## Encounter 1: source 2020-10-15, practice 2024-06-10, ed, Emergency Medicine

Chief complaint: Worsening fatigue, jaundice, and abdominal discomfort with nausea and dark-colored urine for the past several days.

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Michael Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening fatigue, jaundice, and abdominal discomfort with nausea and dark-colored urine for the past several days.

HISTORY OF PRESENT ILLNESS:
The patient with genital HSV-2 on valacyclovir and prior giardiasis in January 2020 presents with worsening fatigue, jaundice, and abdominal discomfort for the past several days. She has also experienced nausea and dark-colored urine.

PAST MEDICAL HISTORY:
Active Problem List:
- Giardiasis (diagnosed 2020-01-15)
- Genital herpes simplex virus type 2 infection

She has not received any blood transfusions.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
There is no history of acetaminophen use.

Home Medications:
- Valacyclovir 500 mg orally daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with hypothyroidism
- Father with hypertension

SOCIAL HISTORY:
There is no history of alcohol intake. She does not have any tattoos or intravenous drug use. She lives in a rural area with limited access to clean water and recalls a recent community outbreak of a diarrheal illness.

VITAL SIGNS:
Her vital signs show mild hypotension and tachycardia.

PHYSICAL EXAMINATION:
On examination, she is visibly jaundiced, with scleral icterus, hepatomegaly, and mild ascites. She appears lethargic but arousable.

LABORATORY DATA:
Laboratory studies reveal severe hepatic dysfunction with markedly elevated transaminases and coagulopathy. Serologic testing confirms the diagnosis.

## Encounter 2: source 2021-10-15, practice 2025-06-11, inpatient, Obstetrics & Gynecology

Chief complaint: Active labor at term with regular contractions and cervical dilation.

ENCOUNTER NOTE
Date: 2021-10-15
Provider: Dr. Priya Patel, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
Active labor at term with regular contractions and cervical dilation.

HISTORY OF PRESENT ILLNESS:
The patient is a 28-year-old female with a history of genital herpes simplex virus type 2 infection on daily valacyclovir suppression, previously diagnosed with giardiasis and acute hepatitis E, who presents to the hospital in active labor.

PAST MEDICAL HISTORY:
Active Problem List:
- Giardiasis (diagnosed 2020-01-15)
- Acute Hepatitis E (diagnosed 2020-10-15)
- Genital herpes simplex virus type 2 infection

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Valacyclovir 500 mg orally daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with hypothyroidism
- Father with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Graduate student in public health.

REVIEW OF SYSTEMS:
She denies fever, dysuria, or vaginal discharge.

PHYSICAL EXAMINATION:
On physical examination, multiple clustered vesicular lesions on an erythematous base are noted on the labia and perineum.

ASSESSMENT AND PLAN:
Given the appearance of the lesions, the obstetric team proceeds with an emergent cesarean delivery. A male infant is delivered with Apgar scores of 8 and 9 at 1 and 5 minutes, respectively. The newborn appears well, with no signs of rash, respiratory distress, or organomegaly. The primary rationale for performing a cesarean delivery in this patient is to prevent

## Encounter 3: source 2022-10-15, practice 2026-06-08, ed, Emergency Medicine

Chief complaint: Found drowsy and confused in her apartment next to an unvented gas space heater after complaining of headache, dizziness, and nausea.

ENCOUNTER NOTE
Date: 2022-10-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Found drowsy and confused in her apartment next to an unvented gas space heater after complaining of headache, dizziness, and nausea.

HISTORY OF PRESENT ILLNESS:
The 29-year-old female with genital HSV-2 and history of neonatal HSV presents to the emergency department. Her roommate states she had been complaining of headache, dizziness, and nausea since the morning and was planning to “sleep it off.” She was discovered lying on the couch next to an unvented gas space heater.

PAST MEDICAL HISTORY:
Active Problem List:
- Giardiasis (diagnosed 2020-01-15)
- Acute Hepatitis E (diagnosed 2020-10-15)
- Neonatal herpes simplex virus infection (disseminated or CNS) (diagnosed 2021-10-15)
- Genital herpes simplex virus type 2 infection

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Valacyclovir 500 mg orally daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Mother with hypothyroidism
- Father with hypertension

SOCIAL HISTORY:
The roommate denies drug or alcohol use, and there is no history of trauma.

VITAL SIGNS:
Her pulse is 110/min, respiratory rate 20/min, blood pressure 122/70 mm Hg, and temperature 37.1°C (98.8°F).

PHYSICAL EXAMINATION:
On arrival, she is confused, with mild slurring of speech, and has a bright red appearance to her skin.
Lungs are clear to auscultation, and cardiac examination is unremarkable. Neurologic examination reveals difficulty with rapid alternating movements but no focal weakness.

LABORATORY DATA:
Laboratory studies, including arterial blood gas and co-oximetry, are sent urgently.

OTHER STUDIES:
While supplemental oxygen by nonrebreather mask is started, a pulse oximeter reading is obtained and shows 96% oxygen saturation.
Cardiac monitoring shows sinus tachycardia without ischemic changes.

ASSESSMENT AND PLAN:
The treating resident is puzzled because the patient’s appearance and symptoms suggest hypoxia.
