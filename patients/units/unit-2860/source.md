# synthetic_hospital patient 2860

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 67 at first encounter, sex F, Non-Hispanic White, Medicare with supplemental private insurance.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-05-24 (Mon) | outpatient | Obstetrics & Gynecology |
| 1 | 2020-02-15 | 2021-06-24 (Thu) | outpatient | Gynecologic Oncology |
| 2 | 2020-03-15 | 2021-07-18 (Sun) | inpatient | Hematology & Oncology |
| 3 | 2021-09-15 | 2023-02-01 (Wed) | outpatient | Hematology & Oncology |
| 4 | 2025-01-15 | 2026-05-27 (Wed) | inpatient | Palliative Care |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicare with supplemental private insurance",
 "occupation": "Retired elementary school librarian",
 "alcohol_use": "None",
 "family_history": [
  "Mother with breast cancer (diagnosed age 45)",
  "Maternal grandmother with ovarian cancer (diagnosed age 68)",
  "Father with hypertension and type 2 diabetes"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "400 mg PO PRN arthritic pain",
   "name": "Ibuprofen"
  },
  {
   "dose": "500 mg PO PRN pain",
   "name": "Acetaminophen"
  },
  {
   "dose": "0.05% cream topical BID to vulvar area",
   "name": "Clobetasol propionate"
  },
  {
   "dose": "600 mg/400 IU PO daily",
   "name": "Calcium carbonate with Vitamin D3"
  }
 ],
 "surgical_history": [
  "Total abdominal hysterectomy with bilateral salpingo-oophorectomy (age 62)",
  "Tumor debulking surgery with omentectomy (age 62)",
  "Peripherally inserted central catheter placement (right arm, age 62)",
  "Exploratory laparotomy (age 62)"
 ],
 "chronic_conditions": [
  "Osteoarthritis",
  "History of stage III ovarian cancer (diagnosed age 62, currently with suspected recurrence)",
  "History of catheter-related upper extremity deep vein thrombosis",
  "Lichen sclerosus",
  "Postmenopausal status"
 ],
 "age_at_first_encounter": 67
}
```

Primary diagnoses: ["Malignant neoplasm of ovary", "Catheter-related upper extremity deep vein thrombosis (Subclavian vein thrombosis)", "Lichen sclerosus"]
Comorbidities: ["Postmenopausal status", "Cachexia"]

## Encounter 0: source 2020-01-15, practice 2021-05-24, outpatient, Obstetrics & Gynecology

Chief complaint: vulvar itching and burning

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Emily Watson, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Vulvar itching and burning

HISTORY OF PRESENT ILLNESS:
6-month history of vulvar itching and burning. She denies vaginal discharge or pelvic pain but reports discomfort during sexual intercourse and some urinary urgency.

PAST MEDICAL HISTORY:
Active Problem List:
- Osteoarthritis
- History of stage III ovarian cancer (diagnosed age 62, currently with suspected recurrence)
- History of catheter-related upper extremity deep vein thrombosis
- Lichen sclerosus
- Postmenopausal status

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy (age 62)
- Tumor debulking surgery with omentectomy (age 62)
- Peripherally inserted central catheter placement (right arm, age 62)
- Exploratory laparotomy (age 62)

MEDICATIONS:
Home Medications:
- Ibuprofen 400 mg PO PRN arthritic pain
- Acetaminophen 500 mg PO PRN pain
- Clobetasol propionate 0.05% cream topical BID to vulvar area
- Calcium carbonate with Vitamin D3 600 mg/400 IU PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with breast cancer (diagnosed age 45)
- Maternal grandmother with ovarian cancer (diagnosed age 68)
- Father with hypertension and type 2 diabetes

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired elementary school librarian.

PHYSICAL EXAMINATION:
Physical examination reveals thinning and pallor of the vulvar skin with areas of white, shiny plaques without ulceration. The labia minora appear fused, and the vaginal introitus is narrowed. The remainder of the pelvic examination is unremarkable.

## Encounter 1: source 2020-02-15, practice 2021-06-24, outpatient, Gynecologic Oncology

Chief complaint: abdominal bloating, increased abdominal girth, and intermittent constipation

ENCOUNTER NOTE
Date: 2020-02-15
Provider: Dr. Sarah Chen, MD
Department: Gynecologic Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
Abdominal bloating, increased abdominal girth, and intermittent constipation

HISTORY OF PRESENT ILLNESS:
The patient returns to gynecologic oncology with stage III ovarian carcinoma history and recent lichen sclerosus care, reporting a 4-month history of abdominal bloating, increased abdominal girth, and intermittent constipation. She also reports feeling full after eating small amounts of food and has noticed mild lower back discomfort.

PAST MEDICAL HISTORY:
Active Problem List:
- Lichen sclerosus (diagnosed 2020-01-15)
- Osteoarthritis
- History of stage III ovarian cancer (diagnosed age 62, currently with suspected recurrence)
- History of catheter-related upper extremity deep vein thrombosis
- Postmenopausal status

She is gravida 4, para 4, and reached menopause at age 50.

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy (age 62)
- Tumor debulking surgery with omentectomy (age 62)
- Peripherally inserted central catheter placement (right arm, age 62)
- Exploratory laparotomy (age 62)

MEDICATIONS:
Home Medications:
- Ibuprofen 400 mg PO PRN arthritic pain
- Acetaminophen 500 mg PO PRN pain
- Clobetasol propionate 0.05% cream topical BID to vulvar area
- Calcium carbonate with Vitamin D3 600 mg/400 IU PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with breast cancer (diagnosed age 45)
- Maternal grandmother with ovarian cancer (diagnosed age 68)
- Father with hypertension and type 2 diabetes

SOCIAL HISTORY:
She does not smoke or drink alcohol.

PHYSICAL EXAMINATION:
On examination, her abdomen is distended, and fluid wave is present. There is mild diffuse tenderness without rebound. Pelvic examination reveals a fixed, irregular mass in the left adnexal region.

IMAGING:
CT scan of the abdomen and pelvis reveals bilateral complex adnexal masses, large-volume ascites, and nodular thickening of the omentum.

PATHOLOGY:
Cytology from ascitic fluid is positive for malignant epithelial cells. Two biopsy specimens are shown.

## Encounter 2: source 2020-03-15, practice 2021-07-18, inpatient, Hematology & Oncology

Chief complaint: progressive right arm swelling and discomfort

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Michael Rodriguez, MD
Department: Hematology & Oncology
Visit Type: inpatient

CHIEF COMPLAINT:
Progressive right arm swelling and discomfort

HISTORY OF PRESENT ILLNESS:
The patient is admitted to Hematology & Oncology for chemotherapy initiation following her recent Gynecologic Oncology encounter for suspected ovarian cancer recurrence. Given her history of catheter-related upper extremity deep vein thrombosis, she presents with new right arm symptoms. She had a peripherally inserted central catheter placed in her right arm two weeks ago for this purpose. She reports a 3-day history of progressive right arm swelling and discomfort.

PAST MEDICAL HISTORY:
Active Problem List:
- Lichen sclerosus (diagnosed 2020-01-15)
- Malignant neoplasm of ovary (diagnosed 2020-02-15)
- Osteoarthritis
- History of stage III ovarian cancer (diagnosed age 62, currently with suspected recurrence)
- History of catheter-related upper extremity deep vein thrombosis
- Postmenopausal status

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy (age 62)
- Tumor debulking surgery with omentectomy (age 62)
- Peripherally inserted central catheter placement (right arm, age 62)
- Exploratory laparotomy (age 62)

MEDICATIONS:
Home Medications:
- Ibuprofen 400 mg PO PRN arthritic pain
- Acetaminophen 500 mg PO PRN pain
- Clobetasol propionate 0.05% cream topical BID to vulvar area
- Calcium carbonate with Vitamin D3 600 mg/400 IU PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with breast cancer (diagnosed age 45)
- Maternal grandmother with ovarian cancer (diagnosed age 68)
- Father with hypertension and type 2 diabetes

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired elementary school librarian.

VITAL SIGNS:
Her vital signs are stable, and she is afebrile.

PHYSICAL EXAMINATION:
On examination, her right upper extremity is erythematous, edematous, and tender to palpation along the course of the PICC line. There is no palpable cord. Pulses are intact, and there is no evidence of compartment syndrome.

LABORATORY DATA:
Laboratory Values:
- Hemoglobin: 12.2 g/dL
- White blood cell count: 7.8 x 10^9/L
- Platelet count: 275 x 10^9/L
- Prothrombin time (PT): 12 seconds
- International normalized ratio (INR): 1.0
- Partial thromboplastin time (PTT): 30 seconds

IMAGING:
A duplex ultrasonography of the right upper extremity is performed and shows a non-compressible segment in the right subclavian vein with direct visualization of an intraluminal thrombus.

## Encounter 3: source 2021-09-15, practice 2023-02-01, outpatient, Hematology & Oncology

Chief complaint: presents to her oncologist for a routine follow-up

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. Michael Rodriguez, MD
Department: Hematology & Oncology
Visit Type: outpatient

CHIEF COMPLAINT:
She presents to her oncologist for a routine follow-up

HISTORY OF PRESENT ILLNESS:
This 68-year-old female with a history of stage III ovarian cancer diagnosed in 2020 and complicated by prior catheter-related upper extremity deep vein thrombosis returns for routine oncology follow-up. She has been undergoing chemotherapy for the past 18 months, but recent imaging studies show progressive disease with multiple liver metastases and ascites. She reports increasing fatigue, abdominal discomfort, and decreased appetite over the past month. She denies any nausea, vomiting, or changes in bowel habits. Her performance status has declined, and she is now mostly bedridden.

PAST MEDICAL HISTORY:
Active Problem List:
- Lichen sclerosus (diagnosed 2020-01-15)
- Malignant neoplasm of ovary (diagnosed 2020-02-15)
- Catheter-related upper extremity deep vein thrombosis (Subclavian vein thrombosis) (diagnosed 2020-03-15)
- Osteoarthritis
- History of stage III ovarian cancer (diagnosed age 62, currently with suspected recurrence)
- History of catheter-related upper extremity deep vein thrombosis
- Postmenopausal status

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy (age 62)
- Tumor debulking surgery with omentectomy (age 62)
- Peripherally inserted central catheter placement (right arm, age 62)
- Exploratory laparotomy (age 62)

MEDICATIONS:
Home Medications:
- Ibuprofen 400 mg PO PRN arthritic pain
- Acetaminophen 500 mg PO PRN pain
- Clobetasol propionate 0.05% cream topical BID to vulvar area
- Calcium carbonate with Vitamin D3 600 mg/400 IU PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with breast cancer (diagnosed age 45)
- Maternal grandmother with ovarian cancer (diagnosed age 68)
- Father with hypertension and type 2 diabetes

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired elementary school librarian.

PHYSICAL EXAMINATION:
Her physical examination reveals a cachectic woman with pallor. Her abdomen is distended with a fluid wave consistent with ascites. There is no rebound tenderness or guarding.

LABORATORY DATA:
Laboratory values are as follows:

- Hemoglobin: 9.2 g/dL
- Platelets: 110,000/uL
- Serum creatinine: 1.1 mg/dL
- Total bilirubin: 1.8 mg/dL
- CA-125: Elevated

ASSESSMENT AND PLAN:
She expresses a desire to focus on her quality of life rather than pursuing aggressive treatment.

## Encounter 4: source 2025-01-15, practice 2026-05-27, inpatient, Palliative Care

Chief complaint: management of pain and dyspnea

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Aisha Patel, MD
Department: Palliative Care
Visit Type: inpatient

CHIEF COMPLAINT:
Management of pain and dyspnea

HISTORY OF PRESENT ILLNESS:
The patient is a 72-year-old woman with a longitudinal history notable for vulvar pruritus and burning consistent with lichen sclerosus since 2020, stage III ovarian cancer initially presenting with abdominal bloating and increased girth, and complicated by catheter-related upper extremity deep vein thrombosis. She returns to the inpatient setting for management of pain and dyspnea. She has been on chemotherapy for the past 18 months, but her disease has progressed, and she has decided to focus on comfort measures. She reports a constant, dull ache in her abdomen, with intermittent sharp pains that she rates as 8 out of 10 in intensity. She also experiences shortness of breath with minimal exertion. The patient's family is at the bedside and expresses their wish for her to be as comfortable as possible in her remaining time.

PAST MEDICAL HISTORY:
Active Problem List:
- Lichen sclerosus (diagnosed 2020-01-15)
- Malignant neoplasm of ovary (diagnosed 2020-02-15)
- Catheter-related upper extremity deep vein thrombosis (Subclavian vein thrombosis) (diagnosed 2020-03-15)
- Osteoarthritis
- History of stage III ovarian cancer (diagnosed age 62, currently with suspected recurrence)
- History of catheter-related upper extremity deep vein thrombosis
- Postmenopausal status

PAST SURGICAL HISTORY:
- Total abdominal hysterectomy with bilateral salpingo-oophorectomy (age 62)
- Tumor debulking surgery with omentectomy (age 62)
- Peripherally inserted central catheter placement (right arm, age 62)
- Exploratory laparotomy (age 62)

MEDICATIONS:
Her current medications include ibuprofen and a short-acting opioid for breakthrough pain.

Home Medications:
- Acetaminophen 500 mg PO PRN pain
- Clobetasol propionate 0.05% cream topical BID to vulvar area
- Calcium carbonate with Vitamin D3 600 mg/400 IU PO daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with breast cancer (diagnosed age 45)
- Maternal grandmother with ovarian cancer (diagnosed age 68)
- Father with hypertension and type 2 diabetes

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Retired elementary school librarian.

VITAL SIGNS:
Her oxygen saturation is 92% on room air.

PHYSICAL EXAMINATION:
On examination, she appears cachectic and is noted to have decreased breath sounds at the lung bases bilaterally. Her abdomen is distended with a fluid wave consistent with ascites.

LABORATORY DATA:
Laboratory values:
- Hemoglobin: 9.2 g/dL
- White blood cell count: 7,000/uL
- Platelet count: 150,000/uL
- Serum creatinine: 1.1 mg/dL
- Blood urea nitrogen: 21 mg/dL
- Serum albumin: 2.8 g/dL
