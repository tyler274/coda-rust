# synthetic_hospital patient 2541

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 29 at first encounter, sex M, Caucasian, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-01-16 (Tue) | outpatient | Psychiatry |
| 1 | 2020-02-15 | 2024-02-06 (Tue) | ed | Emergency Medicine |
| 2 | 2020-05-15 | 2024-05-13 (Mon) | outpatient | Psychiatry |
| 3 | 2020-09-15 | 2024-09-09 (Mon) | outpatient | Psychiatry |
| 4 | 2020-11-15 | 2024-11-06 (Wed) | ed | Emergency Medicine |
| 5 | 2021-01-15 | 2025-01-07 (Tue) | ed | Emergency Medicine |
| 6 | 2021-05-15 | 2025-05-09 (Fri) | ed | Emergency Medicine |
| 7 | 2022-01-15 | 2026-01-05 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicaid",
 "occupation": "Unemployed; receiving disability benefits; previously enrolled in college",
 "alcohol_use": "None; denies illicit substance use",
 "family_history": [
  "First-degree relative with schizophrenia",
  "Mother with major depressive disorder",
  "Father with hypertension"
 ],
 "race_ethnicity": "Caucasian",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "4 mg oral tablet twice daily",
   "name": "Risperidone"
  },
  {
   "dose": "10 mg oral tablet once daily",
   "name": "Aripiprazole"
  },
  {
   "dose": "300 mg oral tablet once daily (for LTBI; adherence variable)",
   "name": "Isoniazid"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Schizophrenia, paranoid type, diagnosed at age 21 with history of medication nonadherence and multiple psychiatric hospitalizations",
  "Latent tuberculosis infection (prescribed treatment but historically noncompliant)",
  "History of antipsychotic-induced acute dystonic reaction (oculogyric crisis)"
 ],
 "age_at_first_encounter": 29
}
```

Primary diagnoses: ["Inadequate sedation or agitation", "Schizophrenia", "Chloroquine-sensitive malaria prophylaxis", "Tuberculous meningitis", "Oculogyric crisis due to antipsychotic medication"]
Comorbidities: ["Medication noncompliance", "Schizophrenia"]

## Encounter 0: source 2020-01-15, practice 2024-01-16, outpatient, Psychiatry

Chief complaint: Increasingly bizarre behavior over the past 7 months, including social withdrawal and paranoid delusions about neighbors.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Increasingly bizarre behavior over the past 7 months, including social withdrawal and paranoid delusions about neighbors.

HISTORY OF PRESENT ILLNESS:
She reports that he has been living alone since dropping out of college and has become socially withdrawn. He often talks to himself and claims to hear voices that comment on his actions. He believes that his neighbors are plotting to harm him and has boarded up his windows to prevent them from spying.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizophrenia, paranoid type, diagnosed at age 21 with history of medication nonadherence and multiple psychiatric hospitalizations
- Latent tuberculosis infection (prescribed treatment but historically noncompliant)
- History of antipsychotic-induced acute dystonic reaction (oculogyric crisis)

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Risperidone 4 mg oral tablet twice daily
- Aripiprazole 10 mg oral tablet once daily
- Isoniazid 300 mg oral tablet once daily (for LTBI; adherence variable)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- First-degree relative with schizophrenia
- Mother with major depressive disorder
- Father with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None; denies illicit substance use. Occupation: Unemployed; receiving disability benefits; previously enrolled in college.

PHYSICAL EXAMINATION:
On examination, the patient is disheveled, avoids eye contact, and appears to be responding to internal stimuli. His speech is coherent but tangential at times. No abnormalities are noted on neurological examination.

LABORATORY DATA:
Laboratory studies, including a complete blood count, comprehensive metabolic panel, and urine toxicology screen, are within normal limits.

CBC:
- Hemoglobin: 14.2 g/dL
- WBC: 6.8 x 10^3/uL
- Platelets: 250 x 10^3/uL

Comprehensive Metabolic Panel:
- Sodium: 140 mEq/L
- Potassium: 4.1 mEq/L
- Chloride: 102 mEq/L
- Bicarbonate: 24 mEq/L
- BUN: 18 mg/dL
- Creatinine: 1.0 mg/dL
- Glucose: 90 mg/dL
- Calcium: 9.6 mg/dL
- AST: 20 U/L
- ALT: 18 U/L
- Alkaline Phosphatase: 70 U/L
- Total Protein: 7.0 g/dL
- Albumin: 4.5 g/dL
- Bilirubin: 0.8 mg/dL

Urine Toxicology Screen: Negative

## Encounter 1: source 2020-02-15, practice 2024-02-06, ed, Emergency Medicine

Chief complaint: Episodes of sudden upward eye rolling lasting several minutes, occurring multiple times daily for the past two days after starting new antipsychotic medication.

ENCOUNTER NOTE
Date: 2020-02-15
Provider: Dr. Marcus Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Episodes of sudden upward eye rolling lasting several minutes, occurring multiple times daily for the past two days after starting new antipsychotic medication.

HISTORY OF PRESENT ILLNESS:
The patient is a 29-year-old with schizophrenia and prior dystonic reactions returning after recent psychiatry evaluation. These episodes have been occurring multiple times a day for the past two days. Patient was started on a new medication for schizophrenia two weeks ago.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizophrenia (diagnosed 2020-01-15)
- Schizophrenia, paranoid type, diagnosed at age 21 with history of medication nonadherence and multiple psychiatric hospitalizations
- Latent tuberculosis infection (prescribed treatment but historically noncompliant)
- History of antipsychotic-induced acute dystonic reaction (oculogyric crisis)

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Risperidone 4 mg oral tablet twice daily
- Aripiprazole 10 mg oral tablet once daily
- Isoniazid 300 mg oral tablet once daily (for LTBI; adherence variable)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- First-degree relative with schizophrenia
- Mother with major depressive disorder
- Father with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None; denies illicit substance use. Occupation: Unemployed; receiving disability benefits; previously enrolled in college.

VITAL SIGNS:
His vital signs are within normal limits.

PHYSICAL EXAMINATION:
On examination, the patient is alert but appears anxious. Neurological examination reveals intermittent, involuntary upward deviation of both eyes, which the patient finds distressing. There is no neck stiffness, and the rest of the cranial nerve examination is normal.

LABORATORY DATA:
Complete blood count: within normal limits
- Comprehensive metabolic panel: within normal limits
- Serum prolactin: within normal limits

## Encounter 2: source 2020-05-15, practice 2024-05-13, outpatient, Psychiatry

Chief complaint: Follow-up evaluation for persistent auditory hallucinations, paranoid delusions, and social withdrawal despite medication adherence.

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. Sarah Chen, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Follow-up evaluation for persistent auditory hallucinations, paranoid delusions, and social withdrawal despite medication adherence.

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of schizophrenia, three months following an oculogyric crisis. Despite adherence to treatment, he reports persistent auditory hallucinations, paranoid delusions, and social withdrawal. He has been hospitalized twice in the past year due to acute psychotic episodes.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizophrenia (diagnosed 2020-01-15)
- Oculogyric crisis due to antipsychotic medication (diagnosed 2020-02-15)
- Schizophrenia, paranoid type, diagnosed at age 21 with history of medication nonadherence and multiple psychiatric hospitalizations
- Latent tuberculosis infection (prescribed treatment but historically noncompliant)
- History of antipsychotic-induced acute dystonic reaction (oculogyric crisis)

He was diagnosed with schizophrenia at the age of 21 and has been on multiple antipsychotic medications since then.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
His current medication regimen includes risperidone 4 mg twice daily and aripiprazole 10 mg daily.

Home Medications:
- Isoniazid 300 mg oral tablet once daily (for LTBI; adherence variable)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- First-degree relative with schizophrenia
- Mother with major depressive disorder
- Father with hypertension

SOCIAL HISTORY:
He denies any substance use, and his family confirms his medication compliance.

PHYSICAL EXAMINATION:
On examination, he appears disheveled and is noted to have a blunted affect and decreased speech output.

LABORATORY DATA:
His laboratory tests are as follows:

- CBC: within normal limits
- CMP: within normal limits
- Lipid panel: within normal limits
- Fasting glucose: within normal limits
- Urine toxicology screen: negative

ASSESSMENT AND PLAN:
Given the patient's history and treatment course,

## Encounter 3: source 2020-09-15, practice 2024-09-09, outpatient, Psychiatry

Chief complaint: Worsening auditory hallucinations and progressive social withdrawal requiring reassessment of current treatment regimen.

ENCOUNTER NOTE
Date: 2020-09-15
Provider: Dr. Emily Rodriguez, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening auditory hallucinations and progressive social withdrawal requiring reassessment of current treatment regimen.

HISTORY OF PRESENT ILLNESS:
The patient returns for psychiatric follow-up four months after his prior evaluation for persistent auditory hallucinations and social withdrawal, with interim history significant for a prior antipsychotic-induced oculogyric crisis. A 29-year-old man with a 6-year history of schizophrenia is brought to the clinic by his father due to worsening auditory hallucinations and social withdrawal. The patient has had multiple hospitalizations due to poor treatment response and medication nonadherence. He has previously been treated with risperidone and aripiprazole, both at therapeutic doses for at least 6 weeks, with minimal improvement.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizophrenia (diagnosed 2020-01-15)
- Oculogyric crisis due to antipsychotic medication (diagnosed 2020-02-15)
- Schizophrenia, paranoid type, diagnosed at age 21 with history of medication nonadherence and multiple psychiatric hospitalizations
- Latent tuberculosis infection (prescribed treatment but historically noncompliant)
- History of antipsychotic-induced acute dystonic reaction (oculogyric crisis)

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Risperidone 4 mg oral tablet twice daily
- Aripiprazole 10 mg oral tablet once daily
- Isoniazid 300 mg oral tablet once daily (for LTBI; adherence variable)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- First-degree relative with schizophrenia
- Mother with major depressive disorder
- Father with hypertension

SOCIAL HISTORY:
He does not smoke or use substances and lives at home with his parents.

VITAL SIGNS:
Vital signs are temperature: 36.7°C (98.1°F), blood pressure: 122/ 78 mm Hg, heart rate: 76/min, respiratory rate: 14/min, SpO ₂: 98% on room air .

PHYSICAL EXAMINATION:
Physical exam is unremarkable.

LABORATORY DATA:
Basic labs, including CBC, CMP, and liver enzymes, are within normal limits.

PLAN:
The psychiatrist initiates treatment with clozapine.

## Encounter 4: source 2020-11-15, practice 2024-11-06, ed, Emergency Medicine

Chief complaint: Agitated, violent behavior in public park with shouting and throwing objects; brought by police after failed verbal de-escalation attempts.

ENCOUNTER NOTE
Date: 2020-11-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Agitated, violent behavior in public park with shouting and throwing objects; brought by police after failed verbal de-escalation attempts.

HISTORY OF PRESENT ILLNESS:
This 29-year-old male with schizophrenia, paranoid type, and medication nonadherence, seen in outpatient psychiatry two months prior for worsening symptoms, returns to the emergency department accompanied by police after being found shouting and throwing objects in a public park. He has no history of violence. Initial attempts at verbal de-escalation, including offering food and water, have been unsuccessful. The patient suddenly starts throwing chairs and threatens to harm the staff.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizophrenia (diagnosed 2020-01-15)
- Oculogyric crisis due to antipsychotic medication (diagnosed 2020-02-15)
- Schizophrenia, paranoid type, diagnosed at age 21 with history of medication nonadherence and multiple psychiatric hospitalizations
- Latent tuberculosis infection (prescribed treatment but historically noncompliant)
- History of antipsychotic-induced acute dystonic reaction (oculogyric crisis)

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
The patient has not taken his prescribed medications for several weeks according to the police report.

Home Medications:
- Risperidone 4 mg oral tablet twice daily
- Aripiprazole 10 mg oral tablet once daily
- Isoniazid 300 mg oral tablet once daily (for LTBI; adherence variable)

ALLERGIES:
He has no known allergies

FAMILY HISTORY:
- First-degree relative with schizophrenia
- Mother with major depressive disorder
- Father with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None; denies illicit substance use. Occupation: Unemployed; receiving disability benefits; previously enrolled in college.

VITAL SIGNS:
His vital signs are stable,

PHYSICAL EXAMINATION:
On examination, the patient is agitated, pacing back and forth, and appears to be responding to internal stimuli. and physical examination is unremarkable.

LABORATORY DATA:
Laboratory values are as follows:

- Sodium: 140 mEq/L
- Potassium: 4.1 mEq/L
- Chloride: 102 mEq/L
- Bicarbonate: 24 mEq/L
- Blood urea nitrogen: 18 mg/dL
- Creatinine: 1.0 mg/dL
- Glucose: 98 mg/dL
- Complete blood count: within normal limits

## Encounter 5: source 2021-01-15, practice 2025-01-07, ed, Emergency Medicine

Chief complaint: Brought by police after being found shouting and throwing objects in public park with profuse sweating, flushed face, and verbal aggression.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Aisha Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Brought by police after being found shouting and throwing objects in public park with profuse sweating, flushed face, and verbal aggression.

HISTORY OF PRESENT ILLNESS:
The patient with schizophrenia returns two months after a prior police encounter, sweating profusely, with flushed face, and pacing the room. He is verbally aggressive towards staff and appears to be responding to internal stimuli.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizophrenia (diagnosed 2020-01-15)
- Oculogyric crisis due to antipsychotic medication (diagnosed 2020-02-15)
- Schizophrenia, paranoid type, diagnosed at age 21 with history of medication nonadherence and multiple psychiatric hospitalizations
- Latent tuberculosis infection (prescribed treatment but historically noncompliant)
- History of antipsychotic-induced acute dystonic reaction (oculogyric crisis)

His medical history is significant for schizophrenia, for which he has been non-compliant with his medication regimen.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Risperidone 4 mg oral tablet twice daily
- Aripiprazole 10 mg oral tablet once daily
- Isoniazid 300 mg oral tablet once daily (for LTBI; adherence variable)

ALLERGIES:
The patient has no known drug allergies

FAMILY HISTORY:
- First-degree relative with schizophrenia
- Mother with major depressive disorder
- Father with hypertension

SOCIAL HISTORY:
He denies substance use.

VITAL SIGNS:
Vital signs reveal a temperature of 37.8°C (100°F), blood pressure of 150/95 mmHg, pulse of 110 beats per minute, and respiratory rate of 22 breaths per minute.

PHYSICAL EXAMINATION:
Physical examination shows dilated pupils and increased muscle tone.

LABORATORY DATA:
Laboratory tests are as follows:

- Complete blood count: within normal limits
- Serum electrolytes: within normal limits
- Urine toxicology screen: negative
- Blood alcohol level: <10 mg/dL

ASSESSMENT AND PLAN:
Given the patient's acute agitation and history of schizophrenia,

## Encounter 6: source 2021-05-15, practice 2025-05-09, ed, Emergency Medicine

Chief complaint: Five-day history of intermittent high fever, shaking chills, and drenching sweats following recent return from camping trip in rural Uganda.

ENCOUNTER NOTE
Date: 2021-05-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Five-day history of intermittent high fever, shaking chills, and drenching sweats following recent return from camping trip in rural Uganda.

HISTORY OF PRESENT ILLNESS:
The patient is a 30-year-old male with schizophrenia, paranoid type, and history of antipsychotic-induced oculogyric crisis, who presents to the Emergency Department. He recently returned from a 3-week trip to rural Uganda, where he participated in outdoor camping. Two days after returning, he began experiencing intermittent high fevers accompanied by shaking chills and followed by profuse sweating, leaving him exhausted. He also reports headaches, myalgias, and generalized malaise. He did not take any malaria prophylaxis during his trip.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizophrenia (diagnosed 2020-01-15)
- Oculogyric crisis due to antipsychotic medication (diagnosed 2020-02-15)
- Inadequate sedation or agitation (diagnosed 2021-01-15)
- Schizophrenia, paranoid type, diagnosed at age 21 with history of medication nonadherence and multiple psychiatric hospitalizations
- Latent tuberculosis infection (prescribed treatment but historically noncompliant)
- History of antipsychotic-induced acute dystonic reaction (oculogyric crisis)

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Risperidone 4 mg oral tablet twice daily
- Aripiprazole 10 mg oral tablet once daily
- Isoniazid 300 mg oral tablet once daily (for LTBI; adherence variable)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- First-degree relative with schizophrenia
- Mother with major depressive disorder
- Father with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None; denies illicit substance use. Occupation: Unemployed; receiving disability benefits; previously enrolled in college.

PHYSICAL EXAMINATION:
Physical examination shows a tired-appearing man with a temperature of 39.4°C (103°F), mild scleral icterus, and palpable splenomegaly.

LABORATORY DATA:
Laboratory evaluation reveals mild anemia and thrombocytopenia. A peripheral blood smear demonstrates intraerythrocytic ring forms.

## Encounter 7: source 2022-01-15, practice 2026-01-05, ed, Emergency Medicine

Chief complaint: Two-week history of severe headache, fever, neck stiffness, and new onset confusion with drowsiness.

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Lisa Wong, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Two-week history of severe headache, fever, neck stiffness, and new onset confusion with drowsiness.

HISTORY OF PRESENT ILLNESS:
This 31-year-old male with schizophrenia, medication nonadherence, and latent tuberculosis infection with variable prophylaxis adherence presents to the emergency department with a 2-week history of headache, fever, and neck stiffness. He also reports a recent onset of confusion and drowsiness.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizophrenia (diagnosed 2020-01-15)
- Oculogyric crisis due to antipsychotic medication (diagnosed 2020-02-15)
- Inadequate sedation or agitation (diagnosed 2021-01-15)
- Chloroquine-sensitive malaria prophylaxis (diagnosed 2021-05-15)
- Schizophrenia, paranoid type, diagnosed at age 21 with history of medication nonadherence and multiple psychiatric hospitalizations
- Latent tuberculosis infection (prescribed treatment but historically noncompliant)
- History of antipsychotic-induced acute dystonic reaction (oculogyric crisis)

His medical history is significant for untreated latent tuberculosis due to noncompliance with medications.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Risperidone 4 mg oral tablet twice daily
- Aripiprazole 10 mg oral tablet once daily
- Isoniazid 300 mg oral tablet once daily (for LTBI; adherence variable)

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- First-degree relative with schizophrenia
- Mother with major depressive disorder
- Father with hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None; denies illicit substance use. Occupation: Unemployed; receiving disability benefits; previously enrolled in college.

PHYSICAL EXAMINATION:
On physical examination, the patient is febrile with a temperature of 38.5°C, has nuchal rigidity, and a positive Brudzinski's sign. Cranial nerve examination reveals a right abducens nerve palsy.

LABORATORY DATA:
A lumbar puncture is performed, and the cerebrospinal fluid (CSF) analysis shows:

- White blood cell count: 150 cells/µL (80% lymphocytes)
- Red blood cell count: 10 cells/µL
- Protein: 100 mg/dL
- Glucose: 30 mg/dL (serum glucose: 90 mg/dL)
- Adenosine deaminase: 12 U/L (elevated)

ASSESSMENT AND PLAN:
Based on the clinical presentation and CSF analysis,
