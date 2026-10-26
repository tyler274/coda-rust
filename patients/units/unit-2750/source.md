# synthetic_hospital patient 2750

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 43 at first encounter, sex F, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-12-07 (Thu) | outpatient | Internal Medicine |
| 1 | 2020-04-15 | 2024-03-11 (Mon) | outpatient | Obstetrics & Gynecology |
| 2 | 2020-10-15 | 2024-09-02 (Mon) | outpatient | Gastroenterology |
| 3 | 2022-09-15 | 2026-07-28 (Tue) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Senior Marketing Manager",
 "alcohol_use": "Social (wine with dinner most nights)",
 "family_history": [
  "Breast cancer (mother, age 54)",
  "Type 2 diabetes mellitus (father)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [
  "Laparoscopic detorsion of right ovary (prior)",
  "Vaginal delivery x 2"
 ],
 "chronic_conditions": [
  "Perimenopause (systemic menopausal syndrome)",
  "Functional dyspepsia",
  "History of ovarian torsion"
 ],
 "age_at_first_encounter": 43
}
```

Primary diagnoses: ["Systemic menopausal syndrome", "Functional dyspepsia", "Ovarian Torsion", "Family history of breast cancer"]
Comorbidities: ["Obesity"]

## Encounter 0: source 2020-01-15, practice 2023-12-07, outpatient, Internal Medicine

Chief complaint: Routine health maintenance examination with concern regarding breast cancer risk assessment due to family history

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Maria Rodriguez, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine health maintenance examination with concern regarding breast cancer risk assessment due to family history

HISTORY OF PRESENT ILLNESS:
She reports no acute complaints but expresses concern about her risk of developing breast cancer. She has two children, both of whom she breastfed for six months. Her menstrual history is notable for menarche at age 12 and no history of oral contraceptive use. She inquires about lifestyle modifications that could reduce her risk of breast cancer.

PAST MEDICAL HISTORY:
Active Problem List:
- Perimenopause (systemic menopausal syndrome)
- Functional dyspepsia
- History of ovarian torsion

PAST SURGICAL HISTORY:
- Laparoscopic detorsion of right ovary (prior)
- Vaginal delivery x 2

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
Her mother was diagnosed with breast cancer at the age of 50

SOCIAL HISTORY:
She admits to consuming wine with dinner most nights of the week. She does not exercise regularly due to her busy schedule.

VITAL SIGNS:
The patient has a body mass index (BMI) of 26 kg/m^2

PHYSICAL EXAMINATION:
On physical examination, there are no palpable breast masses, and the remainder of the exam is unremarkable.

## Encounter 1: source 2020-04-15, practice 2024-03-11, outpatient, Obstetrics & Gynecology

Chief complaint: Hot flashes, night sweats, difficulty sleeping, and irregular menstrual periods for the past 6 months

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Sarah Chen, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Hot flashes, night sweats, difficulty sleeping, and irregular menstrual periods for the past 6 months

HISTORY OF PRESENT ILLNESS:
The patient with perimenopause and prior ovarian torsion returns following internal medicine visit. She reports that her menstrual cycle has been varying from 21 to 45 days and the flow has been lighter than usual. She denies any palpitations, tremors, or unintentional weight loss.

PAST MEDICAL HISTORY:
Active Problem List:
- Family history of breast cancer (diagnosed 2020-01-15)
- Perimenopause (systemic menopausal syndrome)
- Functional dyspepsia
- History of ovarian torsion

PAST SURGICAL HISTORY:
- Laparoscopic detorsion of right ovary (prior)
- Vaginal delivery x 2

MEDICATIONS:
She takes no medications

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
Her mother had a history of early menopause at the age of 45

SOCIAL HISTORY:
She has never smoked and drinks alcohol socially

VITAL SIGNS:
Her blood pressure is 130/85 mmHg, heart rate is 78 beats per minute, and BMI is 26 kg/m^2

PHYSICAL EXAMINATION:
The thyroid gland is not enlarged or tender, and there is no exophthalmos. The remainder of the physical examination is within normal limits.

LABORATORY DATA:
- Serum Thyroid-Stimulating Hormone (TSH): 2.5 µIU/mL (normal range: 0.4-4.0 µIU/mL)
- Serum Follicle-Stimulating Hormone (FSH): 45 mIU/mL (normal range for follicular phase: 3.5-12.5 mIU/mL)

## Encounter 2: source 2020-10-15, practice 2024-09-02, outpatient, Gastroenterology

Chief complaint: Intermittent upper abdominal discomfort and postprandial bloating present for 4 months

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. James O'Brien, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent upper abdominal discomfort and postprandial bloating present for 4 months

HISTORY OF PRESENT ILLNESS:
Patient presents with upper gastrointestinal complaints in the setting of systemic menopausal syndrome previously assessed by Obstetrics and Gynecology. She describes the sensation as a dull, gnawing pain accompanied by occasional bloating that worsens after meals. The pain is not sharp or colicky. The symptoms are not related to physical activity or bowel movements. She denies weight loss, vomiting, melena, or hematemesis.

PAST MEDICAL HISTORY:
Active Problem List:
- Family history of breast cancer (diagnosed 2020-01-15)
- Systemic menopausal syndrome (diagnosed 2020-04-15)
- Perimenopause (systemic menopausal syndrome)
- Functional dyspepsia
- History of ovarian torsion

PAST SURGICAL HISTORY:
- Laparoscopic detorsion of right ovary (prior)
- Vaginal delivery x 2

MEDICATIONS:
She takes no medications other than a daily multivitamin and combined oral contraceptive pill

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Breast cancer (mother, age 54)
- Type 2 diabetes mellitus (father)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (wine with dinner most nights). Occupation: Senior Marketing Manager.

VITAL SIGNS:
BMI is 28. Vitals are normal.

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

LABORATORY DATA:
Labs are normal.

OTHER STUDIES:
A 2-week trial of omeprazole yielded uncertain results.

## Encounter 3: source 2022-09-15, practice 2026-07-28, ed, Emergency Medicine

Chief complaint: Severe sudden lower abdominal pain that started 3 hours ago with associated nausea

ENCOUNTER NOTE
Date: 2022-09-15
Provider: Dr. Priya Patel, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe sudden lower abdominal pain that started 3 hours ago with associated nausea

HISTORY OF PRESENT ILLNESS:
The patient is a 45-year-old female with a history of ovarian torsion and systemic menopausal syndrome who presents to the Emergency Department with acute lower abdominal pain. She reports that the pain has been intermittent for the past week but has become constant and more intense since this morning. She describes nausea but denies vomiting, fever, or urinary symptoms. Her last menstrual period was 2 weeks ago, and her cycles are regular.

PAST MEDICAL HISTORY:
Active Problem List:
- Family history of breast cancer (diagnosed 2020-01-15)
- Systemic menopausal syndrome (diagnosed 2020-04-15)
- Functional dyspepsia (diagnosed 2020-10-15)
- Perimenopause (systemic menopausal syndrome)
- History of ovarian torsion

She has had two vaginal deliveries.

PAST SURGICAL HISTORY:
No history of gynecologic surgery.

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Breast cancer (mother, age 54)
- Type 2 diabetes mellitus (father)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (wine with dinner most nights). Occupation: Senior Marketing Manager.

VITAL SIGNS:
Her temperature is 37.0°C (98.6°F), blood pressure is 122/78 mmHg, heart rate is 104/min, respiratory rate is 18/min, weight is 82 kg, height is 167 cm, and BMI is 29 kg/m².

PHYSICAL EXAMINATION:
On physical examination, she has localized tenderness in the left lower quadrant without rebound or guarding.

ASSESSMENT AND PLAN:
This patient's condition most likely involves twisting of
