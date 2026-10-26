# synthetic_hospital patient 1800

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 26 at first encounter, sex F, Caucasian, Private insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-05-27 (Fri) | outpatient | Psychiatry |
| 1 | 2020-04-15 | 2022-08-23 (Tue) | outpatient | Psychiatry |
| 2 | 2020-07-15 | 2022-11-11 (Fri) | outpatient | Psychiatry |
| 3 | 2023-01-15 | 2025-05-28 (Wed) | ed | Emergency Medicine |
| 4 | 2024-01-15 | 2026-05-21 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private insurance",
 "occupation": "Graduate student",
 "alcohol_use": "None",
 "family_history": [
  "Major depressive disorder in mother",
  "Bipolar disorder in maternal aunt",
  "Schizophrenia in maternal uncle"
 ],
 "race_ethnicity": "Caucasian",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "50 mg daily",
   "name": "Sertraline"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Schizoaffective disorder (depressive type)",
  "Body dysmorphic disorder",
  "Major depressive disorder"
 ],
 "age_at_first_encounter": 26
}
```

Primary diagnoses: ["Suicidal ideation", "Major depressive disorder with psychotic features", "Body dysmorphic disorder", "Schizoaffective disorder"]
Comorbidities: ["Suicidal ideation", "Major depressive disorder"]

## Encounter 0: source 2020-01-15, practice 2022-05-27, outpatient, Psychiatry

Chief complaint: Brought to outpatient psychiatry clinic by her parents due to ongoing behavioral changes including social isolation, muttering to self, and odd beliefs over the past two years.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Brought to outpatient psychiatry clinic by her parents due to ongoing behavioral changes including social isolation, muttering to self, and odd beliefs over the past two years.

HISTORY OF PRESENT ILLNESS:
They report that she frequently isolates herself in her room, mutters to herself, and has increasingly odd beliefs. She believes that news anchors are sending her hidden messages during broadcasts and that her professors are stealing her research ideas through "electromagnetic readings." During today's visit, she is cooperative but makes minimal eye contact. Her speech is disorganized at times, and she describes hearing voices commenting on her actions even when she is alone. When asked about her mood, she states that she sometimes feels "completely numb for weeks," with no energy or motivation to shower or eat. She denies ever feeling euphoric or impulsive, but says she often feels hopeless and occasionally thinks about death.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizoaffective disorder (depressive type)
- Body dysmorphic disorder
- Major depressive disorder

She had a similar depressive episode lasting about 3 months last year, during which she also experienced the same voices and paranoia. On review of records, the patient had a prior hospitalization 6 months ago for auditory hallucinations and delusional thoughts, but at that time denied any depressive or manic symptoms.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
She is.

Home Medications:
- Sertraline 50 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
Her family history is negative for mood or psychotic disorders.

SOCIAL HISTORY:
She denies any drug or alcohol use.

PHYSICAL EXAMINATION:
Physical exam and basic labs are unremarkable.

LABORATORY DATA:
Physical exam and basic labs are unremarkable.

## Encounter 1: source 2020-04-15, practice 2022-08-23, outpatient, Psychiatry

Chief complaint: Six-month history of preoccupation with her nose, which she describes as hideously large and deformed, spending several hours daily mirror-checking and using heavy makeup to conceal the perceived defect.

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of preoccupation with her nose, which she describes as hideously large and deformed, spending several hours daily mirror-checking and using heavy makeup to conceal the perceived defect.

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up care three months following her initial psychiatric evaluation for behavioral changes, social isolation, and odd beliefs, presenting with a six-month history of preoccupation with her nose, which she describes as "hideously large and deformed." She reports spending several hours a day looking at her nose in the mirror and has taken to wearing heavy makeup to "cover up" what she perceives as a defect. She denies any history of trauma to her nose and has not had any previous nasal surgeries.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizoaffective disorder (diagnosed 2020-01-15)
- Schizoaffective disorder (depressive type)
- Body dysmorphic disorder
- Major depressive disorder

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Currently taking sertraline

ALLERGIES:
NKDA

FAMILY HISTORY:
- Major depressive disorder in mother
- Bipolar disorder in maternal aunt
- Schizophrenia in maternal uncle

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Graduate student.

PHYSICAL EXAMINATION:
On physical examination, her nose appears normal in size and contour with no evidence of deformity. She has no respiratory distress, and the rest of the examination is unremarkable.

PLAN:
She requests a referral to a plastic surgeon for a rhinoplasty.

## Encounter 2: source 2020-07-15, practice 2022-11-11, outpatient, Psychiatry

Chief complaint: Three-week follow-up visit to assess response to newly initiated Sertraline therapy for depressive symptoms, reporting improved mood, normalized appetite, and better sleep.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Sarah Chen, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Three-week follow-up visit to assess response to newly initiated Sertraline therapy for depressive symptoms, reporting improved mood, normalized appetite, and better sleep.

HISTORY OF PRESENT ILLNESS:
Patient with history of schizoaffective disorder and body dysmorphic disorder returns for follow-up. She reports normalized appetite and better sleep. She denies any side effects.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizoaffective disorder (diagnosed 2020-01-15)
- Body dysmorphic disorder (diagnosed 2020-04-15)
- Schizoaffective disorder (depressive type)
- Major depressive disorder

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
She has recently been started on a new medication.

Home Medications:
- Sertraline 50 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Major depressive disorder in mother
- Bipolar disorder in maternal aunt
- Schizophrenia in maternal uncle

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Graduate student.

ASSESSMENT AND PLAN:
The medication she is taking is known to increase levels of a neurotransmitter synthesized primarily by neurons in the raphe nuclei of the brainstem.

## Encounter 3: source 2023-01-15, practice 2025-05-28, ed, Emergency Medicine

Chief complaint: Roommate reports the patient has been expressing thoughts of wanting to end it all, has stopped attending therapy, and has been stockpiling her prescribed medications.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. James Thompson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Roommate reports the patient has been expressing thoughts of wanting to end it all, has stopped attending therapy, and has been stockpiling her prescribed medications.

HISTORY OF PRESENT ILLNESS:
The patient is a 29-year-old female with schizoaffective disorder and body dysmorphic disorder, previously stabilized on sertraline, who presents with acute worsening. She has been increasingly isolating herself over the past month and has stopped attending her therapy sessions. She admits to feeling hopeless and confirms that she has been stockpiling her medications. She denies any current suicidal actions but insists that life is not worth living.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizoaffective disorder (diagnosed 2020-01-15)
- Body dysmorphic disorder (diagnosed 2020-04-15)
- Schizoaffective disorder (depressive type)
- Major depressive disorder

The patient has a history of major depressive disorder.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
She was recently started on sertraline.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Major depressive disorder in mother
- Bipolar disorder in maternal aunt
- Schizophrenia in maternal uncle

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Graduate student.

VITAL SIGNS:
Her vital signs are stable

PHYSICAL EXAMINATION:
On examination, the patient is disheveled, has poor eye contact, and speaks in a low, monotone voice., and physical examination is unremarkable.

LABORATORY DATA:
Laboratory values are as follows:

- Sodium: 140 mEq/L
- Potassium: 4.1 mEq/L
- Chloride: 102 mEq/L
- Bicarbonate: 24 mEq/L
- Blood urea nitrogen: 18 mg/dL
- Creatinine: 0.9 mg/dL
- Glucose: 98 mg/dL
- Thyroid-stimulating hormone: 2.5 μIU/mL
- Urine toxicology screen: Negative

## Encounter 4: source 2024-01-15, practice 2026-05-21, ed, Emergency Medicine

Chief complaint: Brought to the emergency department by her roommate who found her sitting on the edge of their apartment's rooftop expressing hopelessness and recurrent self-harm thoughts.

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Brought to the emergency department by her roommate who found her sitting on the edge of their apartment's rooftop expressing hopelessness and recurrent self-harm thoughts.

HISTORY OF PRESENT ILLNESS:
Patient with schizoaffective and body dysmorphic disorders returns one year after ED visit for suicidal ideation. She admits to having recurrent thoughts of self-harm. She reports feeling "empty" and has experienced significant weight loss over the past 2 months. She has been socially withdrawn and has had trouble sleeping.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizoaffective disorder (diagnosed 2020-01-15)
- Body dysmorphic disorder (diagnosed 2020-04-15)
- Suicidal ideation (diagnosed 2023-01-15)
- Schizoaffective disorder (depressive type)
- Major depressive disorder

The patient has a history of major depressive disorder.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Currently.

Home Medications:
- Sertraline 50 mg daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Major depressive disorder in mother
- Bipolar disorder in maternal aunt
- Schizophrenia in maternal uncle

SOCIAL HISTORY:
She denies substance abuse

VITAL SIGNS:
Her vital signs are within normal limits

PHYSICAL EXAMINATION:
On physical examination, she is noted to be agitated and has a blunted affect. A neurological examination is unremarkable

LABORATORY DATA:
Laboratory studies are ordered, including a complete blood count, comprehensive metabolic panel, thyroid function tests, and urine toxicology, all of which return within normal ranges

PLAN:
Given the patient's history and presentation, the decision is made to measure her cerebrospinal fluid (CSF) levels of 5-hydroxyindoleacetic acid (5-HIAA)
