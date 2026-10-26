# synthetic_hospital patient 2731

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 54 at first encounter, sex F, Non-Hispanic White, Commercial PPO (employer-sponsored).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-01-25 (Wed) | outpatient | Obstetrics & Gynecology |
| 1 | 2020-05-15 | 2023-05-15 (Mon) | outpatient | General Surgery |
| 2 | 2020-11-15 | 2023-11-27 (Mon) | outpatient | Obstetrics & Gynecology |
| 3 | 2021-05-15 | 2024-05-21 (Tue) | outpatient | Obstetrics & Gynecology |
| 4 | 2021-11-15 | 2024-11-18 (Mon) | outpatient | Cardiology |
| 5 | 2022-05-15 | 2025-05-27 (Tue) | outpatient | Obstetrics & Gynecology |
| 6 | 2022-11-15 | 2025-11-24 (Mon) | outpatient | Cardiology |
| 7 | 2023-05-15 | 2026-05-13 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Sulfa drugs (sulfamethoxazole-trimethoprim) - rash",
  "Iodinated contrast - urticaria (premedication required)"
 ],
 "insurance": "Commercial PPO (employer-sponsored)",
 "occupation": "Administrative Assistant (sedentary desk work)",
 "alcohol_use": "Social (1-2 glasses of wine per week)",
 "family_history": [
  "Mother: Postmenopausal breast cancer (diagnosed age 62, ER/PR positive)",
  "Father: Colon polyps, hypertension",
  "Sister: Hypothyroidism",
  "Paternal grandmother: Ovarian cancer (diagnosed age 70)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "1 mg PO daily (for breast cancer endocrine therapy)",
   "name": "Anastrozole"
  },
  {
   "dose": "5 mg PO twice daily (for pulmonary embolism treatment)",
   "name": "Apixaban"
  },
  {
   "dose": "40 mg PO daily (for constrictive pericarditis/heart failure symptoms)",
   "name": "Furosemide"
  },
  {
   "dose": "25 mg PO daily (for fluid retention/ascites)",
   "name": "Spironolactone"
  },
  {
   "dose": "300 mg PO three times daily (for vasomotor symptoms/hot flashes)",
   "name": "Gabapentin"
  },
  {
   "dose": "500 mg PO three times daily",
   "name": "Calcium carbonate"
  },
  {
   "dose": "2000 units PO daily",
   "name": "Vitamin D3 (cholecalciferol)"
  },
  {
   "dose": "17 g PO daily PRN constipation (secondary to opioids/narcotics for chronic pelvic pain)",
   "name": "Polyethylene glycol 3350"
  }
 ],
 "surgical_history": [
  "Age 44: Right breast lumpectomy with sentinel lymph node biopsy and port placement",
  "Age 44: Port removal following chemotherapy completion",
  "Age 49: Supracervical (subtotal) hysterectomy for symptomatic uterine fibroids (cervix preserved)",
  "Age 52: Radical hysterectomy with bilateral salpingo-oophorectomy and bilateral pelvic lymph node dissection for cervical cancer",
  "Age 53: Cystoscopy with catheterization for urinary retention (intervention for VVF)"
 ],
 "chronic_conditions": [
  "Obesity (Class I, BMI 32 kg/m\u00b2)",
  "Secondary lymphedema of left lower extremity (post-lymphadenectomy)",
  "Surgical menopause (status post bilateral salpingo-oophorectomy and pelvic radiation)",
  "History of Stage IIIB cervical cancer (NED, treated with radical hysterectomy, lymph node dissection, and chemoradiation)",
  "History of invasive breast cancer, Stage II (treated with lumpectomy, adjuvant chemotherapy, and chest wall radiation)",
  "Radiation-induced pericarditis with constrictive physiology",
  "Pelvic organ prolapse (cystocele)",
  "Vesicovaginal fistula (radiation-induced)",
  "History of pulmonary embolism (provoked, malignancy-associated)"
 ],
 "age_at_first_encounter": 54
}
```

Primary diagnoses: ["Pulmonary embolism", "Secondary lymphedema due to cancer treatment", "Vesicovaginal fistula", "Cystocele", "Radiation pericarditis", "Systemic menopausal syndrome", "Constrictive pericarditis"]
Comorbidities: ["History of breast cancer", "Iatrogenic immunosuppression", "Status post hysterectomy", "Obesity", "Status post total abdominal hysterectomy", "History of cervical cancer", "History of pelvic radiation therapy"]

## Encounter 0: source 2020-01-15, practice 2023-01-25, outpatient, Obstetrics & Gynecology

Chief complaint: Recurrent episodes of intense heat and profuse sweating disrupting sleep and work

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Recurrent episodes of intense heat and profuse sweating disrupting sleep and work

HISTORY OF PRESENT ILLNESS:
She reports that these episodes are disrupting her sleep and impairing her ability to work.

PAST MEDICAL HISTORY:
Active Problem List:
- Obesity (Class I, BMI 32 kg/m²)
- Secondary lymphedema of left lower extremity (post-lymphadenectomy)
- Surgical menopause (status post bilateral salpingo-oophorectomy and pelvic radiation)
- History of Stage IIIB cervical cancer (NED, treated with radical hysterectomy, lymph node dissection, and chemoradiation)
- History of invasive breast cancer, Stage II (treated with lumpectomy, adjuvant chemotherapy, and chest wall radiation)
- Radiation-induced pericarditis with constrictive physiology
- Pelvic organ prolapse (cystocele)
- Vesicovaginal fistula (radiation-induced)
- History of pulmonary embolism (provoked, malignancy-associated)

Her past medical history is significant for a total hysterectomy performed 2 years ago for uterine fibroids.

PAST SURGICAL HISTORY:
- Age 44: Right breast lumpectomy with sentinel lymph node biopsy and port placement
- Age 44: Port removal following chemotherapy completion
- Age 49: Supracervical (subtotal) hysterectomy for symptomatic uterine fibroids (cervix preserved)
- Age 52: Radical hysterectomy with bilateral salpingo-oophorectomy and bilateral pelvic lymph node dissection for cervical cancer
- Age 53: Cystoscopy with catheterization for urinary retention (intervention for VVF)

MEDICATIONS:
Her current medications include calcium and vitamin D supplements.

Home Medications:
- Anastrozole 1 mg PO daily (for breast cancer endocrine therapy)
- Apixaban 5 mg PO twice daily (for pulmonary embolism treatment)
- Furosemide 40 mg PO daily (for constrictive pericarditis/heart failure symptoms)
- Spironolactone 25 mg PO daily (for fluid retention/ascites)
- Gabapentin 300 mg PO three times daily (for vasomotor symptoms/hot flashes)
- Calcium carbonate 500 mg PO three times daily
- Vitamin D3 (cholecalciferol) 2000 units PO daily
- Polyethylene glycol 3350 17 g PO daily PRN constipation (secondary to opioids/narcotics for chronic pelvic pain)

ALLERGIES:
Sulfa drugs (sulfamethoxazole-trimethoprim) - rash, Iodinated contrast - urticaria (premedication required)

FAMILY HISTORY:
Her mother had osteoporosis.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per week). Occupation: Administrative Assistant (sedentary desk work).

VITAL SIGNS:
On physical examination, her blood pressure is 130/85 mmHg, heart rate is 78 beats per minute, and BMI is 26 kg/m^2.

PHYSICAL EXAMINATION:
The remainder of the physical examination is unremarkable.

LABORATORY DATA:
Laboratory tests reveal:
- FSH: 78 IU/L (elevated)
- LH: 55 IU/L (elevated)
- Estradiol: <20 pg/mL (low)
- TSH: 2.5 mIU/L (normal)
- Complete blood count: within normal limits
- Lipid profile: within normal limits

## Encounter 1: source 2020-05-15, practice 2023-05-15, outpatient, General Surgery

Chief complaint: Progressive swelling and heaviness in left leg over 6 months

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. Michael Patel, MD
Department: General Surgery
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive swelling and heaviness in left leg over 6 months

HISTORY OF PRESENT ILLNESS:
Patient returns for left leg swelling with lymphedema after prior encounter. She reports that the swelling initially improved with elevation but has become persistent. She denies any history of trauma or recent travel.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic menopausal syndrome (diagnosed 2020-01-15)
- Obesity (Class I, BMI 32 kg/m²)
- Secondary lymphedema of left lower extremity (post-lymphadenectomy)
- Surgical menopause (status post bilateral salpingo-oophorectomy and pelvic radiation)
- History of Stage IIIB cervical cancer (NED, treated with radical hysterectomy, lymph node dissection, and chemoradiation)
- History of invasive breast cancer, Stage II (treated with lumpectomy, adjuvant chemotherapy, and chest wall radiation)
- Radiation-induced pericarditis with constrictive physiology
- Pelvic organ prolapse (cystocele)
- Vesicovaginal fistula (radiation-induced)
- History of pulmonary embolism (provoked, malignancy-associated)

Her medical history is significant for stage IIIB cervical cancer treated with radical hysterectomy and pelvic lymph node dissection followed by chemoradiation therapy, completed 18 months ago.

PAST SURGICAL HISTORY:
- Age 44: Right breast lumpectomy with sentinel lymph node biopsy and port placement
- Age 44: Port removal following chemotherapy completion
- Age 49: Supracervical (subtotal) hysterectomy for symptomatic uterine fibroids (cervix preserved)
- Age 52: Radical hysterectomy with bilateral salpingo-oophorectomy and bilateral pelvic lymph node dissection for cervical cancer
- Age 53: Cystoscopy with catheterization for urinary retention (intervention for VVF)

MEDICATIONS:
Home Medications:
- Anastrozole 1 mg PO daily (for breast cancer endocrine therapy)
- Apixaban 5 mg PO twice daily (for pulmonary embolism treatment)
- Furosemide 40 mg PO daily (for constrictive pericarditis/heart failure symptoms)
- Spironolactone 25 mg PO daily (for fluid retention/ascites)
- Gabapentin 300 mg PO three times daily (for vasomotor symptoms/hot flashes)
- Calcium carbonate 500 mg PO three times daily
- Vitamin D3 (cholecalciferol) 2000 units PO daily
- Polyethylene glycol 3350 17 g PO daily PRN constipation (secondary to opioids/narcotics for chronic pelvic pain)

ALLERGIES:
Sulfa drugs (sulfamethoxazole-trimethoprim) - rash, Iodinated contrast - urticaria (premedication required)

FAMILY HISTORY:
- Mother: Postmenopausal breast cancer (diagnosed age 62, ER/PR positive)
- Father: Colon polyps, hypertension
- Sister: Hypothyroidism
- Paternal grandmother: Ovarian cancer (diagnosed age 70)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per week). Occupation: Administrative Assistant (sedentary desk work).

PHYSICAL EXAMINATION:
On physical examination, her left leg is swollen from the thigh to the ankle, with overlying skin that appears thickened and non-pitting on palpation. There is no erythema, warmth, or tenderness. The right leg appears normal.

IMAGING:
A Doppler ultrasound of the left leg shows no evidence of deep vein thrombosis.

## Encounter 2: source 2020-11-15, practice 2023-11-27, outpatient, Obstetrics & Gynecology

Chief complaint: Sensation of something 'falling out' of vagina with worsening urinary incontinence

ENCOUNTER NOTE
Date: 2020-11-15
Provider: Dr. Emily Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Sensation of something 'falling out' of vagina with worsening urinary incontinence

HISTORY OF PRESENT ILLNESS:
Returning to Obstetrics & Gynecology, she reports a sensation of something 'falling out' of the vagina, which has been progressively worsening over the past year. She reports associated urinary incontinence, which occurs when she coughs or sneezes.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic menopausal syndrome (diagnosed 2020-01-15)
- Secondary lymphedema due to cancer treatment (diagnosed 2020-05-15)
- Obesity (Class I, BMI 32 kg/m²)
- Secondary lymphedema of left lower extremity (post-lymphadenectomy)
- Surgical menopause (status post bilateral salpingo-oophorectomy and pelvic radiation)
- History of Stage IIIB cervical cancer (NED, treated with radical hysterectomy, lymph node dissection, and chemoradiation)
- History of invasive breast cancer, Stage II (treated with lumpectomy, adjuvant chemotherapy, and chest wall radiation)
- Radiation-induced pericarditis with constrictive physiology
- Pelvic organ prolapse (cystocele)
- Vesicovaginal fistula (radiation-induced)
- History of pulmonary embolism (provoked, malignancy-associated)

Her past medical history is significant for two vaginal deliveries, a hysterectomy performed for uterine fibroids 5 years ago, and a BMI of 32 kg/m^2.

PAST SURGICAL HISTORY:
- Age 44: Right breast lumpectomy with sentinel lymph node biopsy and port placement
- Age 44: Port removal following chemotherapy completion
- Age 49: Supracervical (subtotal) hysterectomy for symptomatic uterine fibroids (cervix preserved)
- Age 52: Radical hysterectomy with bilateral salpingo-oophorectomy and bilateral pelvic lymph node dissection for cervical cancer
- Age 53: Cystoscopy with catheterization for urinary retention (intervention for VVF)

MEDICATIONS:
Home Medications:
- Anastrozole 1 mg PO daily (for breast cancer endocrine therapy)
- Apixaban 5 mg PO twice daily (for pulmonary embolism treatment)
- Furosemide 40 mg PO daily (for constrictive pericarditis/heart failure symptoms)
- Spironolactone 25 mg PO daily (for fluid retention/ascites)
- Gabapentin 300 mg PO three times daily (for vasomotor symptoms/hot flashes)
- Calcium carbonate 500 mg PO three times daily
- Vitamin D3 (cholecalciferol) 2000 units PO daily
- Polyethylene glycol 3350 17 g PO daily PRN constipation (secondary to opioids/narcotics for chronic pelvic pain)

ALLERGIES:
Sulfa drugs (sulfamethoxazole-trimethoprim) - rash, Iodinated contrast - urticaria (premedication required)

FAMILY HISTORY:
- Mother: Postmenopausal breast cancer (diagnosed age 62, ER/PR positive)
- Father: Colon polyps, hypertension
- Sister: Hypothyroidism
- Paternal grandmother: Ovarian cancer (diagnosed age 70)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per week). Occupation: Administrative Assistant (sedentary desk work).

PHYSICAL EXAMINATION:
On physical examination, you note a bulging mass protruding through the vaginal introitus when she performs a Valsalva maneuver. A pelvic exam reveals a weakened pelvic floor and anterior vaginal wall prolapse.

LABORATORY DATA:
Urinalysis is unremarkable.

## Encounter 3: source 2021-05-15, practice 2024-05-21, outpatient, Obstetrics & Gynecology

Chief complaint: Continuous, painless leakage of urine for 6 months requiring pads

ENCOUNTER NOTE
Date: 2021-05-15
Provider: Dr. Emily Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Continuous, painless leakage of urine for 6 months requiring pads

HISTORY OF PRESENT ILLNESS:
The patient returns to the obstetrics and gynecology clinic with history of pelvic radiation. Following her previous evaluation six months ago, she presents with 6-month history of continuous, painless leakage of urine that worsens when upright. She reports wearing pads all the time and is embarrassed by odor. She has no dysuria, urgency, or frequency.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic menopausal syndrome (diagnosed 2020-01-15)
- Secondary lymphedema due to cancer treatment (diagnosed 2020-05-15)
- Cystocele (diagnosed 2020-11-15)
- Obesity (Class I, BMI 32 kg/m²)
- Secondary lymphedema of left lower extremity (post-lymphadenectomy)
- Surgical menopause (status post bilateral salpingo-oophorectomy and pelvic radiation)
- History of Stage IIIB cervical cancer (NED, treated with radical hysterectomy, lymph node dissection, and chemoradiation)
- History of invasive breast cancer, Stage II (treated with lumpectomy, adjuvant chemotherapy, and chest wall radiation)
- Radiation-induced pericarditis with constrictive physiology
- Pelvic organ prolapse (cystocele)
- Vesicovaginal fistula (radiation-induced)
- History of pulmonary embolism (provoked, malignancy-associated)

PAST SURGICAL HISTORY:
No previous surgeries other than the treatment for cervical cancer

MEDICATIONS:
Home Medications:
- Anastrozole 1 mg PO daily (for breast cancer endocrine therapy)
- Apixaban 5 mg PO twice daily (for pulmonary embolism treatment)
- Furosemide 40 mg PO daily (for constrictive pericarditis/heart failure symptoms)
- Spironolactone 25 mg PO daily (for fluid retention/ascites)
- Gabapentin 300 mg PO three times daily (for vasomotor symptoms/hot flashes)
- Calcium carbonate 500 mg PO three times daily
- Vitamin D3 (cholecalciferol) 2000 units PO daily
- Polyethylene glycol 3350 17 g PO daily PRN constipation (secondary to opioids/narcotics for chronic pelvic pain)

ALLERGIES:
Sulfa drugs (sulfamethoxazole-trimethoprim) - rash, Iodinated contrast - urticaria (premedication required)

FAMILY HISTORY:
- Mother: Postmenopausal breast cancer (diagnosed age 62, ER/PR positive)
- Father: Colon polyps, hypertension
- Sister: Hypothyroidism
- Paternal grandmother: Ovarian cancer (diagnosed age 70)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per week). Occupation: Administrative Assistant (sedentary desk work).

PHYSICAL EXAMINATION:
On physical examination, there is no suprapubic tenderness, and neurological examination is normal. A speculum examination reveals a small opening in the anterior vaginal wall with continuous leakage of clear fluid.

LABORATORY DATA:
Urinalysis shows no signs of infection or hematuria.

## Encounter 4: source 2021-11-15, practice 2024-11-18, outpatient, Cardiology

Chief complaint: Progressive dyspnea on exertion and fatigue over 2 months

ENCOUNTER NOTE
Date: 2021-11-15
Provider: Dr. James Washington, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive dyspnea on exertion and fatigue over 2 months

HISTORY OF PRESENT ILLNESS:
Patient with radiation-induced pericarditis and recent pelvic complications returns with progressive dyspnea on exertion and fatigue over the past two months. She denies chest pain, fever, or cough.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic menopausal syndrome (diagnosed 2020-01-15)
- Secondary lymphedema due to cancer treatment (diagnosed 2020-05-15)
- Cystocele (diagnosed 2020-11-15)
- Vesicovaginal fistula (diagnosed 2021-05-15)
- Obesity (Class I, BMI 32 kg/m²)
- Secondary lymphedema of left lower extremity (post-lymphadenectomy)
- Surgical menopause (status post bilateral salpingo-oophorectomy and pelvic radiation)
- History of Stage IIIB cervical cancer (NED, treated with radical hysterectomy, lymph node dissection, and chemoradiation)
- History of invasive breast cancer, Stage II (treated with lumpectomy, adjuvant chemotherapy, and chest wall radiation)
- Radiation-induced pericarditis with constrictive physiology
- Pelvic organ prolapse (cystocele)
- Vesicovaginal fistula (radiation-induced)
- History of pulmonary embolism (provoked, malignancy-associated)

PAST SURGICAL HISTORY:
- Age 44: Right breast lumpectomy with sentinel lymph node biopsy and port placement
- Age 44: Port removal following chemotherapy completion
- Age 49: Supracervical (subtotal) hysterectomy for symptomatic uterine fibroids (cervix preserved)
- Age 52: Radical hysterectomy with bilateral salpingo-oophorectomy and bilateral pelvic lymph node dissection for cervical cancer
- Age 53: Cystoscopy with catheterization for urinary retention (intervention for VVF)

MEDICATIONS:
Home Medications:
- Anastrozole 1 mg PO daily (for breast cancer endocrine therapy)
- Apixaban 5 mg PO twice daily (for pulmonary embolism treatment)
- Furosemide 40 mg PO daily (for constrictive pericarditis/heart failure symptoms)
- Spironolactone 25 mg PO daily (for fluid retention/ascites)
- Gabapentin 300 mg PO three times daily (for vasomotor symptoms/hot flashes)
- Calcium carbonate 500 mg PO three times daily
- Vitamin D3 (cholecalciferol) 2000 units PO daily
- Polyethylene glycol 3350 17 g PO daily PRN constipation (secondary to opioids/narcotics for chronic pelvic pain)

ALLERGIES:
Sulfa drugs (sulfamethoxazole-trimethoprim) - rash, Iodinated contrast - urticaria (premedication required)

FAMILY HISTORY:
- Mother: Postmenopausal breast cancer (diagnosed age 62, ER/PR positive)
- Father: Colon polyps, hypertension
- Sister: Hypothyroidism
- Paternal grandmother: Ovarian cancer (diagnosed age 70)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per week). Occupation: Administrative Assistant (sedentary desk work).

VITAL SIGNS:
Blood pressure is 110/70 mmHg, heart rate is 95 bpm, respiratory rate is 18 breaths/min, and oxygen saturation is 97% on room air

PHYSICAL EXAMINATION:
Cardiac auscultation reveals muffled heart sounds, and the apical impulse is not palpable. Jugular venous pressure is elevated.

IMAGING:
A chest x-ray shows an enlarged, 'water bottle'-shaped cardiac silhouette.

OTHER STUDIES:
An echocardiogram confirms the presence of a pericardial effusion.

## Encounter 5: source 2022-05-15, practice 2025-05-27, outpatient, Obstetrics & Gynecology

Chief complaint: 2-week history of continuous, painless watery vaginal discharge

ENCOUNTER NOTE
Date: 2022-05-15
Provider: Dr. Emily Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
2-week history of continuous, painless watery vaginal discharge

HISTORY OF PRESENT ILLNESS:
Patient returns with recurrent vesicovaginal fistula symptoms. She reports that the discharge is watery and she has to wear pads throughout the day.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic menopausal syndrome (diagnosed 2020-01-15)
- Secondary lymphedema due to cancer treatment (diagnosed 2020-05-15)
- Cystocele (diagnosed 2020-11-15)
- Vesicovaginal fistula (diagnosed 2021-05-15)
- Radiation pericarditis (diagnosed 2021-11-15)
- Obesity (Class I, BMI 32 kg/m²)
- Secondary lymphedema of left lower extremity (post-lymphadenectomy)
- Surgical menopause (status post bilateral salpingo-oophorectomy and pelvic radiation)
- History of Stage IIIB cervical cancer (NED, treated with radical hysterectomy, lymph node dissection, and chemoradiation)
- History of invasive breast cancer, Stage II (treated with lumpectomy, adjuvant chemotherapy, and chest wall radiation)
- Radiation-induced pericarditis with constrictive physiology
- Pelvic organ prolapse (cystocele)
- Vesicovaginal fistula (radiation-induced)
- History of pulmonary embolism (provoked, malignancy-associated)

PAST SURGICAL HISTORY:
Radical hysterectomy 6 weeks ago

MEDICATIONS:
Home Medications:
- Anastrozole 1 mg PO daily (for breast cancer endocrine therapy)
- Apixaban 5 mg PO twice daily (for pulmonary embolism treatment)
- Furosemide 40 mg PO daily (for constrictive pericarditis/heart failure symptoms)
- Spironolactone 25 mg PO daily (for fluid retention/ascites)
- Gabapentin 300 mg PO three times daily (for vasomotor symptoms/hot flashes)
- Calcium carbonate 500 mg PO three times daily
- Vitamin D3 (cholecalciferol) 2000 units PO daily
- Polyethylene glycol 3350 17 g PO daily PRN constipation (secondary to opioids/narcotics for chronic pelvic pain)

ALLERGIES:
Sulfa drugs (sulfamethoxazole-trimethoprim) - rash, Iodinated contrast - urticaria (premedication required)

FAMILY HISTORY:
- Mother: Postmenopausal breast cancer (diagnosed age 62, ER/PR positive)
- Father: Colon polyps, hypertension
- Sister: Hypothyroidism
- Paternal grandmother: Ovarian cancer (diagnosed age 70)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per week). Occupation: Administrative Assistant (sedentary desk work).

REVIEW OF SYSTEMS:
She denies any fever, urinary urgency, frequency, or dysuria.

VITAL SIGNS:
Afebrile

PHYSICAL EXAMINATION:
Abdomen is soft and non-tender. Pelvic examination reveals a small amount of clear fluid pooling in the vaginal vault.

LABORATORY DATA:
Urinalysis shows:
- pH: 6.0
- Specific gravity: 1.010
- Leukocyte esterase: Negative
- Nitrites: Negative
- Protein: Negative
- Glucose: Negative
- Ketones: Negative
- Blood: Negative
- Bacteria: Not observed
- Epithelial cells: Few

## Encounter 6: source 2022-11-15, practice 2025-11-24, outpatient, Cardiology

Chief complaint: Progressive shortness of breath, peripheral edema, and abdominal swelling with 10-pound weight gain

ENCOUNTER NOTE
Date: 2022-11-15
Provider: Dr. James Washington, MD
Department: Cardiology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive shortness of breath, peripheral edema, and abdominal swelling with 10-pound weight gain

HISTORY OF PRESENT ILLNESS:
The patient returns to the cardiology clinic for follow-up of her radiation-induced pericarditis diagnosed one year prior in the context of breast and cervical malignancy. She presents with a 6-month history of progressive shortness of breath, peripheral edema, and abdominal swelling. She reports that her symptoms are worse at the end of the day and that she has gained 10 pounds over the last two months despite no significant change in her diet.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic menopausal syndrome (diagnosed 2020-01-15)
- Secondary lymphedema due to cancer treatment (diagnosed 2020-05-15)
- Cystocele (diagnosed 2020-11-15)
- Vesicovaginal fistula (diagnosed 2021-05-15)
- Radiation pericarditis (diagnosed 2021-11-15)
- Obesity (Class I, BMI 32 kg/m²)
- Secondary lymphedema of left lower extremity (post-lymphadenectomy)
- Surgical menopause (status post bilateral salpingo-oophorectomy and pelvic radiation)
- History of Stage IIIB cervical cancer (NED, treated with radical hysterectomy, lymph node dissection, and chemoradiation)
- History of invasive breast cancer, Stage II (treated with lumpectomy, adjuvant chemotherapy, and chest wall radiation)
- Radiation-induced pericarditis with constrictive physiology
- Pelvic organ prolapse (cystocele)
- Vesicovaginal fistula (radiation-induced)
- History of pulmonary embolism (provoked, malignancy-associated)

Breast cancer treated with adjuvant chemotherapy and radiation therapy 10 years ago.

PAST SURGICAL HISTORY:
Mastectomy 10 years ago

MEDICATIONS:
Home Medications:
- Anastrozole 1 mg PO daily (for breast cancer endocrine therapy)
- Apixaban 5 mg PO twice daily (for pulmonary embolism treatment)
- Furosemide 40 mg PO daily (for constrictive pericarditis/heart failure symptoms)
- Spironolactone 25 mg PO daily (for fluid retention/ascites)
- Gabapentin 300 mg PO three times daily (for vasomotor symptoms/hot flashes)
- Calcium carbonate 500 mg PO three times daily
- Vitamin D3 (cholecalciferol) 2000 units PO daily
- Polyethylene glycol 3350 17 g PO daily PRN constipation (secondary to opioids/narcotics for chronic pelvic pain)

ALLERGIES:
Sulfa drugs (sulfamethoxazole-trimethoprim) - rash, Iodinated contrast - urticaria (premedication required)

FAMILY HISTORY:
- Mother: Postmenopausal breast cancer (diagnosed age 62, ER/PR positive)
- Father: Colon polyps, hypertension
- Sister: Hypothyroidism
- Paternal grandmother: Ovarian cancer (diagnosed age 70)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per week). Occupation: Administrative Assistant (sedentary desk work).

VITAL SIGNS:
On physical examination, her blood pressure is 130/80 mmHg, heart rate is 88 beats per minute, and respiratory rate is 18 breaths per minute.

PHYSICAL EXAMINATION:
She has elevated jugular venous pressure, positive hepatojugular reflux, and a palpable liver edge 3 cm below the right costal margin. Auscultation of the heart reveals a pericardial knock.

LABORATORY DATA:
Laboratory Values:
- Hemoglobin: 13.2 g/dL
- White blood cell count: 7,800 /μL
- Platelet count: 250,000 /μL
- Serum creatinine: 1.0 mg/dL
- Serum albumin: 3.5 g/dL
- Alanine aminotransferase (ALT): 45 U/L
- Aspartate aminotransferase (AST): 50 U/L
- Total bilirubin: 1.2 mg/dL
- Brain natriuretic peptide (BNP): 550 pg/mL

IMAGING:
Chest X-ray shows pericardial calcification without pulmonary congestion. Echocardiography demonstrates normal left ventricular function, thickened pericardium, and septal bounce.

## Encounter 7: source 2023-05-15, practice 2026-05-13, ed, Emergency Medicine

Chief complaint: Sudden onset shortness of breath and left-sided pleuritic chest pain

ENCOUNTER NOTE
Date: 2023-05-15
Provider: Dr. Aisha Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset shortness of breath and left-sided pleuritic chest pain

HISTORY OF PRESENT ILLNESS:
The patient with prior PE presents with dyspnea and pleuritic chest pain. She also reports a cough with occasional wheezing over the past two days. She denies recent travel, surgery, or prolonged immobilization.

PAST MEDICAL HISTORY:
Active Problem List:
- Systemic menopausal syndrome (diagnosed 2020-01-15)
- Secondary lymphedema due to cancer treatment (diagnosed 2020-05-15)
- Cystocele (diagnosed 2020-11-15)
- Vesicovaginal fistula (diagnosed 2021-05-15)
- Radiation pericarditis (diagnosed 2021-11-15)
- Constrictive pericarditis (diagnosed 2022-11-15)
- Obesity (Class I, BMI 32 kg/m²)
- Secondary lymphedema of left lower extremity (post-lymphadenectomy)
- Surgical menopause (status post bilateral salpingo-oophorectomy and pelvic radiation)
- History of Stage IIIB cervical cancer (NED, treated with radical hysterectomy, lymph node dissection, and chemoradiation)
- History of invasive breast cancer, Stage II (treated with lumpectomy, adjuvant chemotherapy, and chest wall radiation)
- Radiation-induced pericarditis with constrictive physiology
- Pelvic organ prolapse (cystocele)
- Vesicovaginal fistula (radiation-induced)
- History of pulmonary embolism (provoked, malignancy-associated)

PAST SURGICAL HISTORY:
- Age 44: Right breast lumpectomy with sentinel lymph node biopsy and port placement
- Age 44: Port removal following chemotherapy completion
- Age 49: Supracervical (subtotal) hysterectomy for symptomatic uterine fibroids (cervix preserved)
- Age 52: Radical hysterectomy with bilateral salpingo-oophorectomy and bilateral pelvic lymph node dissection for cervical cancer
- Age 53: Cystoscopy with catheterization for urinary retention (intervention for VVF)

MEDICATIONS:
Home Medications:
- Anastrozole 1 mg PO daily (for breast cancer endocrine therapy)
- Apixaban 5 mg PO twice daily (for pulmonary embolism treatment)
- Furosemide 40 mg PO daily (for constrictive pericarditis/heart failure symptoms)
- Spironolactone 25 mg PO daily (for fluid retention/ascites)
- Gabapentin 300 mg PO three times daily (for vasomotor symptoms/hot flashes)
- Calcium carbonate 500 mg PO three times daily
- Vitamin D3 (cholecalciferol) 2000 units PO daily
- Polyethylene glycol 3350 17 g PO daily PRN constipation (secondary to opioids/narcotics for chronic pelvic pain)

ALLERGIES:
Sulfa drugs (sulfamethoxazole-trimethoprim) - rash, Iodinated contrast - urticaria (premedication required)

FAMILY HISTORY:
- Mother: Postmenopausal breast cancer (diagnosed age 62, ER/PR positive)
- Father: Colon polyps, hypertension
- Sister: Hypothyroidism
- Paternal grandmother: Ovarian cancer (diagnosed age 70)

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 glasses of wine per week). Occupation: Administrative Assistant (sedentary desk work).

VITAL SIGNS:
Her temperature is 37.8°C (100°F), blood pressure is 135/85 mmHg, pulse is 110/min, and respiratory rate is 22/min. Oxygen saturation is 89% on room air.

PHYSICAL EXAMINATION:
Physical examination reveals decreased breath sounds and expiratory wheezing over the left lower lung field. Lower extremity examination shows mild left calf swelling with tenderness to palpation.

LABORATORY DATA:
Arterial blood gas on room air shows:

- pH: 7.48
- PaCO2: 32 mmHg
- PaO2: 70 mmHg
- HCO3-: 24 mEq/L

IMAGING:
Chest X-ray is unremarkable.
