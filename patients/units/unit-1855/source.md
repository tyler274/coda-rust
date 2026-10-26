# synthetic_hospital patient 1855

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 28 at first encounter, sex F, Non-Hispanic White, Private.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-07-12 (Fri) | outpatient | Obstetrics & Gynecology |
| 1 | 2022-01-15 | 2026-07-16 (Thu) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private",
 "occupation": "Graduate student",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Mother with endometriosis",
  "Maternal aunt with infertility"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never",
 "home_medications": [
  {
   "dose": "400 mg PO every 6 hours as needed for pelvic pain",
   "name": "Ibuprofen"
  },
  {
   "dose": "220 mg PO every 8 hours as needed for menstrual cramps",
   "name": "Naproxen sodium"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [],
 "age_at_first_encounter": 28
}
```

Primary diagnoses: ["Endometriosis", "Hypotension"]
Comorbidities: ["Strict vegetarian diet"]

## Encounter 0: source 2020-01-15, practice 2024-07-12, outpatient, Obstetrics & Gynecology

Chief complaint: Progressive dysmenorrhea and deep pelvic pain during intercourse over 6 months affecting daily activities

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive dysmenorrhea and deep pelvic pain during intercourse over 6 months affecting daily activities

HISTORY OF PRESENT ILLNESS:
She presents to the clinic with a 6-month history of dysmenorrhea and deep pelvic pain during intercourse. Her menstrual cycles are regular, but she reports that the pain has progressively worsened and is now affecting her daily activities.

PAST MEDICAL HISTORY:
Her past medical history is unremarkable, and she has no children.

MEDICATIONS:
She has used ibuprofen and naproxen with minimal relief.

Home Medications:
- Naproxen sodium 220 mg PO every 8 hours as needed for menstrual cramps

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with endometriosis
- Maternal aunt with infertility

SOCIAL HISTORY:
She is currently using condoms for contraception.

PHYSICAL EXAMINATION:
On physical examination, there is tenderness noted in the posterior cul-de-sac, and the uterus is retroverted.

LABORATORY DATA:
Her laboratory tests are as follows:

- Hemoglobin: 13.2 g/dL
- White blood cell count: 7,500/mm^3
- Platelets: 250,000/mm^3
- CA-125: 35 U/mL (normal range: 0-35 U/mL)

IMAGING:
A transvaginal ultrasound shows no abnormalities.

ASSESSMENT AND PLAN:
Given the patient's symptoms and the failure of NSAIDs to provide adequate relief,

## Encounter 1: source 2022-01-15, practice 2026-07-16, ed, Emergency Medicine

Chief complaint: Acute hypotension episode during participation in a research study investigating physiologic differences in vascular regulation between men and women

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Marcus Williams, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute hypotension episode during participation in a research study investigating physiologic differences in vascular regulation between men and women

PAST MEDICAL HISTORY:
Active Problem List:
- Endometriosis (diagnosed 2020-01-15)

MEDICATIONS:
Home Medications:
- Ibuprofen 400 mg PO every 6 hours as needed for pelvic pain
- Naproxen sodium 220 mg PO every 8 hours as needed for menstrual cramps

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with endometriosis
- Maternal aunt with infertility

SOCIAL HISTORY:
Her diet and activity levels are typical for her age

REVIEW OF SYSTEMS:
She reports no dizziness, palpitations, or fatigue

VITAL SIGNS:
Her BMI is 21 kg/m². During multiple measurements, her mean arterial pressure consistently remains at the lower end of the normal range compared with male participants of similar age and fitness. Her heart rate is 72/min

LABORATORY DATA:
Laboratory studies show normal electrolytes, hemoglobin, and renal function. Endocrine evaluation reveals normal renin, aldosterone, and catecholamine levels.

OTHER STUDIES:
Further vascular testing demonstrates preserved baroreceptor reflexes, normal sympathetic nerve activity, and enhanced endothelium-dependent vasodilation in response to acetylcholine infusion. This vasodilatory response is markedly blunted when the same study is repeated in the presence of a specific inhibitor of a soluble guanylyl cyclase–mediated pathway.
