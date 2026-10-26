# synthetic_hospital patient 1882

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex F, Black/African American, Private insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-08-27 (Tue) | outpatient | Obstetrics & Gynecology |
| 1 | 2020-04-15 | 2024-11-20 (Wed) | outpatient | Dermatology |
| 2 | 2021-07-15 | 2026-02-16 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private insurance",
 "occupation": "Elementary school teacher",
 "alcohol_use": "Social (1-2 drinks per month)",
 "family_history": [
  "Mother: Breast cancer (diagnosed age 45)",
  "Maternal aunt: Systemic lupus erythematosus",
  "Father: Hypertension"
 ],
 "race_ethnicity": "Black/African American",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "200 mg PO BID",
   "name": "Hydroxychloroquine"
  },
  {
   "dose": "100 mg PO daily",
   "name": "Azathioprine"
  },
  {
   "dose": "DS 1 tablet PO three times weekly (MWF)",
   "name": "Trimethoprim-sulfamethoxazole"
  }
 ],
 "surgical_history": [
  "Right chest port-a-cath placement",
  "Left modified radical mastectomy with axillary lymph node dissection",
  "Excisional lymph node biopsy (cervical)"
 ],
 "chronic_conditions": [
  "Systemic lupus erythematosus",
  "Hodgkin lymphoma (in remission)",
  "Breast cancer (stage II, treated)"
 ],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Hypergonadotropic hypogonadism", "Anagen effluvium", "Agranulocytosis"]
Comorbidities: ["Iatrogenic immunosuppression"]

## Encounter 0: source 2020-01-15, practice 2024-08-27, outpatient, Obstetrics & Gynecology

Chief complaint: 6-month history of irregular menstrual cycles and 3-month history of amenorrhea with hot flashes and vaginal dryness

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
6-month history of irregular menstrual cycles and 3-month history of amenorrhea with hot flashes and vaginal dryness

HISTORY OF PRESENT ILLNESS:
She reports experiencing hot flashes, night sweats, and vaginal dryness. Her last menstrual period was 3 months ago, and she has never been pregnant.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic lupus erythematosus
- Hodgkin lymphoma (in remission)
- Breast cancer (stage II, treated)

She has a past medical history of Hodgkin lymphoma, for which she completed chemotherapy with doxorubicin, bleomycin, vinblastine, and dacarbazine (ABVD) 18 months ago.

PAST SURGICAL HISTORY:
- Right chest port-a-cath placement
- Left modified radical mastectomy with axillary lymph node dissection
- Excisional lymph node biopsy (cervical)

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg PO BID
- Azathioprine 100 mg PO daily
- Trimethoprim-sulfamethoxazole DS 1 tablet PO three times weekly (MWF)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer (diagnosed age 45)
- Maternal aunt: Systemic lupus erythematosus
- Father: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

VITAL SIGNS:
On physical examination, her vital signs are within normal limits.

PHYSICAL EXAMINATION:
Pelvic examination reveals atrophic vaginal mucosa.

LABORATORY DATA:
Her laboratory tests show the following:

- Follicle-stimulating hormone (FSH): 108 mIU/mL (normal range: 3.5-12.5 mIU/mL)
- Luteinizing hormone (LH): 95 mIU/mL (normal range: 2.4-12.6 mIU/mL)
- Estradiol: 15 pg/mL (normal range: 30-400 pg/mL)
- Thyroid-stimulating hormone (TSH): 2.5 µIU/mL (normal range: 0.4-4.0 µIU/mL)
- Prolactin: 14 ng/mL (normal range: 4-23 ng/mL)

## Encounter 1: source 2020-04-15, practice 2024-11-20, outpatient, Dermatology

Chief complaint: Hair falling out in clumps over the past two weeks following recent chemotherapy completion

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Priya Patel, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Hair falling out in clumps over the past two weeks following recent chemotherapy completion

HISTORY OF PRESENT ILLNESS:
The patient with systemic lupus erythematosus, Hodgkin lymphoma in remission, and breast cancer presents after her OB/GYN evaluation. She reports that her hair has been falling out in clumps over the past two weeks. She completed her final cycle of chemotherapy for breast cancer one month ago. She denies any scalp pain or itching.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypergonadotropic hypogonadism (diagnosed 2020-01-15)
- Systemic lupus erythematosus
- Hodgkin lymphoma (in remission)
- Breast cancer (stage II, treated)

Her medical history is significant for stage II breast cancer diagnosed six months ago, for which she received a combination of doxorubicin and cyclophosphamide.

PAST SURGICAL HISTORY:
- Right chest port-a-cath placement
- Left modified radical mastectomy with axillary lymph node dissection
- Excisional lymph node biopsy (cervical)

MEDICATIONS:
Home Medications:
- Hydroxychloroquine 200 mg PO BID
- Azathioprine 100 mg PO daily
- Trimethoprim-sulfamethoxazole DS 1 tablet PO three times weekly (MWF)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer (diagnosed age 45)
- Maternal aunt: Systemic lupus erythematosus
- Father: Hypertension

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per month). Occupation: Elementary school teacher.

PHYSICAL EXAMINATION:
On physical examination, diffuse thinning of the scalp hair is noted without any evidence of scarring or skin changes. No lymphadenopathy is palpable. The remainder of the examination is unremarkable.

LABORATORY DATA:
Laboratory values:
- Complete blood count: within normal limits
- Thyroid-stimulating hormone (TSH): within normal limits
- Serum iron: within normal limits
- Ferritin: within normal limits

## Encounter 2: source 2021-07-15, practice 2026-02-16, ed, Emergency Medicine

Chief complaint: High fever, fatigue, and painful oral ulcers that began 1 day ago

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. James Okafor, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
High fever, fatigue, and painful oral ulcers that began 1 day ago

HISTORY OF PRESENT ILLNESS:
This 33-year-old female with SLE, Hodgkin lymphoma, breast cancer, hypogonadism, and anagen effluvium describes feeling generally unwell for several days, with chills, decreased appetite, and difficulty eating due to mouth pain. She denies cough, shortness of breath, abdominal pain, or urinary symptoms.

PAST MEDICAL HISTORY:
Active Problem List:
- Hypergonadotropic hypogonadism (diagnosed 2020-01-15)
- Anagen effluvium (diagnosed 2020-04-15)
- Systemic lupus erythematosus
- Hodgkin lymphoma (in remission)
- Breast cancer (stage II, treated)

Her past medical history includes systemic lupus erythematosus,.

PAST SURGICAL HISTORY:
- Right chest port-a-cath placement
- Left modified radical mastectomy with axillary lymph node dissection
- Excisional lymph node biopsy (cervical)

MEDICATIONS:
She was recently prescribed daily trimethoprim-sulfamethoxazole (TMP-SMX) for prophylaxis against opportunistic infections due to worsening immunosuppression.

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Breast cancer (diagnosed age 45)
- Maternal aunt: Systemic lupus erythematosus
- Father: Hypertension

SOCIAL HISTORY:
She has no recent travel or sick contacts.

VITAL SIGNS:
On physical exam, her temperature is 39.5°C (103.1°F), blood pressure is 104/66 mm Hg, pulse is 114/min, and respiratory rate is 18/min.

PHYSICAL EXAMINATION:
She appears ill and fatigued. Oral exam reveals multiple erythematous ulcers with overlying white exudate on the lips, tongue, and buccal mucosa. There is no pharyngeal erythema, lymphadenopathy, rash, or joint swelling. Lungs and heart are normal. Abdomen is soft and nontender.

LABORATORY DATA:
Blood cultures are drawn

PLAN:
She is given intravenous fluids.
