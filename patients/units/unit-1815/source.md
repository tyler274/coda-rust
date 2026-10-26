# synthetic_hospital patient 1815

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 22 at first encounter, sex F, Non-Hispanic White, University student health insurance plan.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-08-07 (Thu) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2026-01-31 (Sat) | ed | Emergency Medicine |
| 2 | 2020-08-15 | 2026-03-06 (Fri) | icu | Critical Care Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "University student health insurance plan",
 "occupation": "Undergraduate student",
 "alcohol_use": "None; patient denies alcohol use",
 "family_history": [
  "Mother with major depressive disorder",
  "Paternal uncle with cirrhosis of unknown etiology"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [],
 "chronic_conditions": [],
 "age_at_first_encounter": 22
}
```

Primary diagnoses: ["Acute liver failure", "Fanconi syndrome", "Acetaminophen toxicity"]
Comorbidities: ["Traumatic coagulopathy", "Dehydration"]

## Encounter 0: source 2020-01-15, practice 2025-08-07, outpatient, Internal Medicine

Chief complaint: Increased thirst and frequent urination over the past month with associated muscle weakness and difficulty concentrating during lectures.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Sharma, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Increased thirst and frequent urination over the past month with associated muscle weakness and difficulty concentrating during lectures.

HISTORY OF PRESENT ILLNESS:
She reports waking multiple times at night to urinate and has been carrying a water bottle with her everywhere due to persistent dry mouth. She has also noticed muscle weakness and recent difficulty concentrating during lectures. Her appetite has decreased slightly, and she has unintentionally lost 3 kg over the past few weeks.

PAST MEDICAL HISTORY:
She has no history of diabetes or renal disease. Past medical history is unremarkable

MEDICATIONS:
She takes no regular medications. However, she admits to recently using a leftover antibiotic found in her family’s medicine cabinet for what she thought was a “sinus infection.” She says the medication was “a few years old” and yellowed in color.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with major depressive disorder
- Paternal uncle with cirrhosis of unknown etiology

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None; patient denies alcohol use. Occupation: Undergraduate student.

PHYSICAL EXAMINATION:
Physical examination is notable for dry mucous membranes and mild orthostatic hypotension. Neurologic exam shows diminished vibratory sense in the lower extremities.

## Encounter 1: source 2020-07-15, practice 2026-01-31, ed, Emergency Medicine

Chief complaint: Nausea, vomiting, and right upper quadrant pain 16 hours after intentional overdose of acetaminophen.

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. James O'Brien, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Nausea, vomiting, and right upper quadrant pain 16 hours after intentional overdose of acetaminophen.

HISTORY OF PRESENT ILLNESS:
Previously evaluated for polyuria and polydipsia, the patient presents 16 hours after an intentional overdose of acetaminophen. She reports taking "a whole bottle" but cannot recall how many pills.

PAST MEDICAL HISTORY:
Active Problem List:
- Fanconi syndrome (diagnosed 2020-01-15)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with major depressive disorder
- Paternal uncle with cirrhosis of unknown etiology

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None; patient denies alcohol use. Occupation: Undergraduate student.

VITAL SIGNS:
Her temperature is 99 F, blood pressure is 110/70 mm Hg, and pulse is 90/min.

PHYSICAL EXAMINATION:
On examination, she is alert but uncomfortable, with tenderness to palpation over the right upper quadrant.

LABORATORY DATA:
Laboratory studies show AST of 3,200 U/L and ALT of 3,600 U/L; total bilirubin and alkaline phosphatase are normal. INR is mildly elevated at 1.7.

PLAN:
The patient is admitted and started on N -acetylcysteine (NAC) therapy.

## Encounter 2: source 2020-08-15, practice 2026-03-06, icu, Critical Care Medicine

Chief complaint: Found drowsy and nauseated several hours after ingesting an unknown number of over-the-counter pain tablets, with episodes of vomiting.

ENCOUNTER NOTE
Date: 2020-08-15
Provider: Dr. Elena Vasquez, MD
Department: Critical Care Medicine
Visit Type: icu

CHIEF COMPLAINT:
Found drowsy and nauseated several hours after ingesting an unknown number of over-the-counter pain tablets, with episodes of vomiting.

HISTORY OF PRESENT ILLNESS:
This 22-year-old female with a history of increased thirst, frequent urination, and muscle weakness in January 2020 and acetaminophen overdose with nausea in July 2020 presents following ingestion of over-the-counter pain tablets. She has had several episodes of vomiting but no hematemesis or melena.

PAST MEDICAL HISTORY:
Active Problem List:
- Fanconi syndrome (diagnosed 2020-01-15)
- Acetaminophen toxicity (diagnosed 2020-07-15)

MEDICATIONS:
She takes no chronic medications

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with major depressive disorder
- Paternal uncle with cirrhosis of unknown etiology

SOCIAL HISTORY:
The roommate notes the patient has been under significant stress due to exams but denies knowledge of alcohol or recreational drug use.

VITAL SIGNS:
Temperature is 37.0 °C (98.6 °F), blood pressure is 92/58 mmHg, pulse is 118/min, and respirations are 22/min

PHYSICAL EXAMINATION:
On examination, she appears lethargic but arousable, with dry mucous membranes. Skin is warm without track marks or rashes. The abdomen is mildly tender in the right upper quadrant, with no rebound or guarding.

LABORATORY DATA:
Laboratory studies show markedly elevated aminotransferases, prolonged prothrombin time, and low blood glucose.
