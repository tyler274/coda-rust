# synthetic_hospital patient 2780

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 43 at first encounter, sex F, Non-Hispanic White, Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-08-25 (Mon) | outpatient | Internal Medicine |
| 1 | 2020-03-15 | 2025-10-19 (Sun) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Penicillin - anaphylaxis",
  "Iodinated contrast - urticaria",
  "Latex - contact dermatitis"
 ],
 "insurance": "Blue Cross Blue Shield PPO",
 "occupation": "High school mathematics teacher",
 "alcohol_use": "Social, 1-2 glasses of wine per week",
 "family_history": [
  "Mother: Infiltrating ductal carcinoma of the breast diagnosed at age 47, BRCA status unknown, status post bilateral mastectomy",
  "Maternal aunt: Epithelial ovarian cancer diagnosed at age 52",
  "Father: Type 2 diabetes mellitus diagnosed age 65"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "3.75 mg intramuscular injection every 28 days",
   "name": "Leuprolide acetate"
  },
  {
   "dose": "325 mg by mouth daily with vitamin C",
   "name": "Ferrous sulfate"
  },
  {
   "dose": "500 mg by mouth twice daily as needed for pelvic pain",
   "name": "Naproxen"
  },
  {
   "dose": "600 mg/400 units by mouth twice daily",
   "name": "Calcium carbonate with vitamin D3"
  }
 ],
 "surgical_history": [
  "Total abdominal hysterectomy with bilateral salpingo-oophorectomy (performed age 43 for refractory endometriosis)",
  "Laparoscopy with lysis of pelvic adhesions and endometrial ablation (age 39)"
 ],
 "chronic_conditions": [
  "Severe refractory endometriosis with chronic pelvic pain",
  "Iron deficiency anemia secondary to menorrhagia",
  "Osteopenia secondary to long-term GnRH agonist therapy"
 ],
 "age_at_first_encounter": 43
}
```

Primary diagnoses: ["Suspicious breast lesion requiring diagnostic imaging", "Pituitary apoplexy with secondary adrenal insufficiency (adrenal crisis)"]
Comorbidities: ["Family history of breast cancer"]

## Encounter 0: source 2020-01-15, practice 2025-08-25, outpatient, Internal Medicine

Chief complaint: Routine wellness visit with concern regarding breast cancer screening given maternal history of breast cancer diagnosed at age 47

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Routine wellness visit with concern regarding breast cancer screening given maternal history of breast cancer diagnosed at age 47

HISTORY OF PRESENT ILLNESS:
She reports no current symptoms but expresses concern about her risk of breast cancer given her mother was diagnosed at age 47. The patient has not had a mammogram in the past and inquires about starting screening.

PAST MEDICAL HISTORY:
Active Problem List:
- Severe refractory endometriosis with chronic pelvic pain
- Iron deficiency anemia secondary to menorrhagia
- Osteopenia secondary to long-term GnRH agonist therapy

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy (performed age 43 for refractory endometriosis)
- Laparoscopy with lysis of pelvic adhesions and endometrial ablation (age 39)

MEDICATIONS:
Home Medications:
- Leuprolide acetate 3.75 mg intramuscular injection every 28 days
- Ferrous sulfate 325 mg by mouth daily with vitamin C
- Naproxen 500 mg by mouth twice daily as needed for pelvic pain
- Calcium carbonate with vitamin D3 600 mg/400 units by mouth twice daily

ALLERGIES:
Penicillin - anaphylaxis, Iodinated contrast - urticaria, Latex - contact dermatitis

FAMILY HISTORY:
With a family history of breast cancer

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 glasses of wine per week. Occupation: High school mathematics teacher.

PHYSICAL EXAMINATION:
On examination, there are no palpable masses in either breast, and no axillary lymphadenopathy is noted.

IMAGING:
The patient opts to proceed with the mammogram, which reveals a small, 0.5 cm area of increased density in the right breast without calcifications. The radiologist suggests that the finding is likely benign but recommends a biopsy to rule out malignancy.

ASSESSMENT AND PLAN:
Her physician discusses the benefits and limitations of mammography and mentions that the sensitivity and specificity of the test can be affected by the chosen cutoff points for abnormal findings.

The patient is hesitant and asks about the implications of adjusting the cutoff point for an abnormal mammogram result.

## Encounter 1: source 2020-03-15, practice 2025-10-19, ed, Emergency Medicine

Chief complaint: Severe headache, visual disturbances, and profound weakness with hypotension occurring several weeks following total abdominal hysterectomy with bilateral salpingo-oophorectomy

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. James Okafor, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe headache, visual disturbances, and profound weakness with hypotension occurring several weeks following total abdominal hysterectomy with bilateral salpingo-oophorectomy

HISTORY OF PRESENT ILLNESS:
Patient is a 43-year-old female with severe refractory endometriosis and chronic pelvic pain recently evaluated for a suspicious breast lesion requiring diagnostic imaging. She presents to the emergency department several weeks following total abdominal hysterectomy and bilateral oophorectomy for refractory endometriosis. She carries a long-standing history of severe pelvic pain and reports moderate hot flashes, night sweats, and mood swings within several weeks of surgery. Upon follow-up, she declines estrogen or progesterone replacement therapy due to family history of breast cancer.

PAST MEDICAL HISTORY:
Active Problem List:
- Suspicious breast lesion requiring diagnostic imaging (diagnosed 2020-01-15)
- Severe refractory endometriosis with chronic pelvic pain
- Iron deficiency anemia secondary to menorrhagia
- Osteopenia secondary to long-term GnRH agonist therapy

She is otherwise healthy, with no significant medical comorbidities.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy (performed age 43 for refractory endometriosis)
- Laparoscopy with lysis of pelvic adhesions and endometrial ablation (age 39)

MEDICATIONS:
Home Medications:
- Leuprolide acetate 3.75 mg intramuscular injection every 28 days
- Ferrous sulfate 325 mg by mouth daily with vitamin C
- Naproxen 500 mg by mouth twice daily as needed for pelvic pain
- Calcium carbonate with vitamin D3 600 mg/400 units by mouth twice daily

ALLERGIES:
Penicillin - anaphylaxis, Iodinated contrast - urticaria, Latex - contact dermatitis

FAMILY HISTORY:
Family history of breast cancer

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social, 1-2 glasses of wine per week. Occupation: High school mathematics teacher.

PHYSICAL EXAMINATION:
Mild atrophic changes of the vulva and dryness
