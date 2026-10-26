# synthetic_hospital patient 1730

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex F, Non-Hispanic White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2023-07-01 (Sat) | inpatient | Obstetrics & Gynecology |
| 1 | 2020-01-15 | 2023-07-01 (Sat) | ed | Emergency Medicine |
| 2 | 2020-01-15 | 2023-07-04 (Tue) | ed | Emergency Medicine |
| 3 | 2020-01-15 | 2023-07-04 (Tue) | ed | Emergency Medicine |
| 4 | 2020-01-15 | 2023-07-04 (Tue) | inpatient | Internal Medicine |
| 5 | 2020-03-15 | 2023-08-30 (Wed) | outpatient | Internal Medicine |
| 6 | 2022-07-15 | 2025-12-22 (Mon) | outpatient | Obstetrics & Gynecology |
| 7 | 2023-01-15 | 2026-06-23 (Tue) | outpatient | Endocrinology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA",
  "Latex (contact dermatitis)"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Librarian",
 "alcohol_use": "None",
 "family_history": [
  "Mother: Type 1 Von Willebrand disease, menorrhagia requiring hysterectomy at age 42",
  "Father: Delayed puberty (age 16), poor sense of smell (presumed variant Kallmann syndrome)",
  "Sister: Primary infertility, secondary amenorrhea"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "325 mg PO daily",
   "name": "Ferrous sulfate"
  },
  {
   "dose": "150 mcg intranasal spray as needed for bleeding episodes",
   "name": "Desmopressin acetate"
  },
  {
   "dose": "1 tablet PO daily (continuous cycling for hormone replacement)",
   "name": "Ethinyl estradiol/norethindrone acetate"
  },
  {
   "dose": "2000 IU PO daily",
   "name": "Vitamin D3"
  }
 ],
 "surgical_history": [
  "Emergency low transverse cesarean section (age 32)",
  "Repeat cesarean section (age 34)",
  "Dilation and curettage for retained products of conception",
  "Manual reduction of uterine inversion",
  "Bilateral tubal ligation (post-second delivery)"
 ],
 "chronic_conditions": [
  "Von Willebrand disease (Type 1)",
  "Kallmann syndrome (isolated hypogonadotropic hypogonadism with anosmia)",
  "Chronic iron deficiency anemia",
  "History of infertility secondary to hypogonadism"
 ],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Kallmann syndrome", "Von Willebrand disease", "Secondary postpartum hemorrhage due to retained products of conception", "Acute uterine inversion", "Acute postpartum hemorrhage"]
Comorbidities: ["Hyponatremia", "Postpartum hemorrhage", "Acute posthemorrhagic anemia", "History of previous cesarean delivery", "Hypovolemic shock", "Secondary adrenal insufficiency", "Hypotension"]

## Encounter 0: source 2020-01-15, practice 2023-07-01, inpatient, Obstetrics & Gynecology

Chief complaint: Acute hypotension and tachycardia immediately following vaginal delivery of first child

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Johnson, MD
Department: Obstetrics & Gynecology
Visit Type: inpatient

CHIEF COMPLAINT:
Acute hypotension and tachycardia immediately following vaginal delivery of first child

HISTORY OF PRESENT ILLNESS:
After an uncomplicated labor, she delivers a healthy male infant vaginally with the assistance of a small dose of oxytocin. Immediately postpartum, she becomes hypotensive and tachycardic,

PAST MEDICAL HISTORY:
Active Problem List:
- Von Willebrand disease (Type 1)
- Kallmann syndrome (isolated hypogonadotropic hypogonadism with anosmia)
- Chronic iron deficiency anemia
- History of infertility secondary to hypogonadism

PAST SURGICAL HISTORY:
She has a history of one previous cesarean delivery.

MEDICATIONS:
Home Medications:
- Ferrous sulfate 325 mg PO daily
- Desmopressin acetate 150 mcg intranasal spray as needed for bleeding episodes
- Ethinyl estradiol/norethindrone acetate 1 tablet PO daily (continuous cycling for hormone replacement)
- Vitamin D3 2000 IU PO daily

ALLERGIES:
NKDA, Latex (contact dermatitis)

FAMILY HISTORY:
- Mother: Type 1 Von Willebrand disease, menorrhagia requiring hysterectomy at age 42
- Father: Delayed puberty (age 16), poor sense of smell (presumed variant Kallmann syndrome)
- Sister: Primary infertility, secondary amenorrhea

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Librarian.

VITAL SIGNS:
Blood pressure of 80/50 mmHg and a heart rate of 120 bpm

PHYSICAL EXAMINATION:
Her uterus is not palpable abdominally, and there is a large, red, rounded mass protruding from her vagina. The patient is becoming increasingly agitated and confused. Her skin is cool and clammy.

LABORATORY DATA:
- Hemoglobin: 9 g/dL (decreased from 12 g/dL on admission)
- Platelets: 250 x 10^9/L
- International Normalized Ratio (INR): 1.1
- Blood type: A positive

PLAN:
The obstetric team is preparing for an intervention.

## Encounter 1: source 2020-01-15, practice 2023-07-01, ed, Emergency Medicine

Chief complaint: Heavy vaginal bleeding that began 30 minutes after vaginal delivery at term

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Michael Chen, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Heavy vaginal bleeding that began 30 minutes after vaginal delivery at term

HISTORY OF PRESENT ILLNESS:
This 32-year-old female with Von Willebrand disease Type 1 and chronic iron deficiency anemia presents with heavy vaginal bleeding 30 minutes after vaginal delivery, following admission for uterine inversion. Her pregnancy was largely uneventful, though she had a few episodes of light spotting during third trimester. She received no ultrasound beyond 20 weeks and had no prenatal complications. Labor progressed spontaneously without induction, augmentation, or instrumentation, and she delivered a healthy newborn without difficulty. Placenta was delivered a few minutes later but was noted to have an irregular margin and a vessel extending to the membranes. Bleeding continues despite administration of oxytocin.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute uterine inversion (diagnosed 2020-01-15)
- Von Willebrand disease (Type 1)
- Kallmann syndrome (isolated hypogonadotropic hypogonadism with anosmia)
- Chronic iron deficiency anemia
- History of infertility secondary to hypogonadism

PAST SURGICAL HISTORY:
- Emergency low transverse cesarean section (age 32)
- Repeat cesarean section (age 34)
- Dilation and curettage for retained products of conception
- Manual reduction of uterine inversion
- Bilateral tubal ligation (post-second delivery)

MEDICATIONS:
Home Medications:
- Ferrous sulfate 325 mg PO daily
- Desmopressin acetate 150 mcg intranasal spray as needed for bleeding episodes
- Ethinyl estradiol/norethindrone acetate 1 tablet PO daily (continuous cycling for hormone replacement)
- Vitamin D3 2000 IU PO daily

ALLERGIES:
NKDA, Latex (contact dermatitis)

FAMILY HISTORY:
- Mother: Type 1 Von Willebrand disease, menorrhagia requiring hysterectomy at age 42
- Father: Delayed puberty (age 16), poor sense of smell (presumed variant Kallmann syndrome)
- Sister: Primary infertility, secondary amenorrhea

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Librarian.

VITAL SIGNS:
Temperature is normal, blood pressure is 92/60 mmHg, and heart rate is 122/min

PHYSICAL EXAMINATION:
The uterus is firm and consistent in size with postpartum expectations. There are no vaginal or cervical lacerations, and fundal massage does not affect the bleeding.

LABORATORY DATA:
Hemoglobin is 7.4 g/dL

## Encounter 2: source 2020-01-15, practice 2023-07-04, ed, Emergency Medicine

Chief complaint: Severe abdominal pain and dizziness 30 minutes after spontaneous vaginal delivery at home

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Lisa Wong, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe abdominal pain and dizziness 30 minutes after spontaneous vaginal delivery at home

HISTORY OF PRESENT ILLNESS:
This patient with Von Willebrand disease and chronic iron deficiency, treated earlier today for postpartum complications, returns to the emergency department 30 minutes after a spontaneous vaginal delivery at home. Her husband reports that she has had heavy vaginal bleeding since the delivery.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute uterine inversion (diagnosed 2020-01-15)
- Acute postpartum hemorrhage (diagnosed 2020-01-15)
- Von Willebrand disease (Type 1)
- Kallmann syndrome (isolated hypogonadotropic hypogonadism with anosmia)
- Chronic iron deficiency anemia
- History of infertility secondary to hypogonadism

Her past medical history is significant for two previous cesarean sections.

PAST SURGICAL HISTORY:
- Emergency low transverse cesarean section (age 32)
- Repeat cesarean section (age 34)
- Dilation and curettage for retained products of conception
- Manual reduction of uterine inversion
- Bilateral tubal ligation (post-second delivery)

MEDICATIONS:
Home Medications:
- Ferrous sulfate 325 mg PO daily
- Desmopressin acetate 150 mcg intranasal spray as needed for bleeding episodes
- Ethinyl estradiol/norethindrone acetate 1 tablet PO daily (continuous cycling for hormone replacement)
- Vitamin D3 2000 IU PO daily

ALLERGIES:
NKDA, Latex (contact dermatitis)

FAMILY HISTORY:
- Mother: Type 1 Von Willebrand disease, menorrhagia requiring hysterectomy at age 42
- Father: Delayed puberty (age 16), poor sense of smell (presumed variant Kallmann syndrome)
- Sister: Primary infertility, secondary amenorrhea

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Librarian.

VITAL SIGNS:
Her blood pressure is 90/60 mmHg, heart rate is 120 beats per minute

PHYSICAL EXAMINATION:
On examination, she appears pale and diaphoretic. A firm, rounded mass is noted protruding through the dilated cervix. The uterine fundus is not palpable on abdominal examination.

LABORATORY DATA:
Her laboratory findings are as follows:

- Hemoglobin: 8 g/dL (Normal: 12-16 g/dL)
- Hematocrit: 24% (Normal: 37-47%)
- Platelets: 250,000/mm³ (Normal: 150,000-400,000/mm³)
- Prothrombin time (PT): 12 seconds (Normal: 11-13.5 seconds)
- Partial thromboplastin time (PTT): 30 seconds (Normal: 25-35 seconds)

## Encounter 3: source 2020-01-15, practice 2023-07-04, ed, Emergency Medicine

Chief complaint: Significant continuous vaginal bleeding 12 hours after vaginal delivery of first child

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. James Wilson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Significant continuous vaginal bleeding 12 hours after vaginal delivery of first child

HISTORY OF PRESENT ILLNESS:
This 32-year-old with Von Willebrand disease Type 1 and Kallmann syndrome returns to the emergency department twelve hours following vaginal delivery after earlier presentations today for postpartum bleeding. She reports that the bleeding has been continuous and seems to be increasing in volume. Her pregnancy was uncomplicated, and she had a spontaneous vaginal delivery without any instrumentation.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute uterine inversion (diagnosed 2020-01-15)
- Acute postpartum hemorrhage (diagnosed 2020-01-15)
- Secondary postpartum hemorrhage due to retained products of conception (diagnosed 2020-01-15)
- Von Willebrand disease (Type 1)
- Kallmann syndrome (isolated hypogonadotropic hypogonadism with anosmia)
- Chronic iron deficiency anemia
- History of infertility secondary to hypogonadism

Her past medical history is notable for easy bruising and frequent nosebleeds since childhood,.

PAST SURGICAL HISTORY:
- Emergency low transverse cesarean section (age 32)
- Repeat cesarean section (age 34)
- Dilation and curettage for retained products of conception
- Manual reduction of uterine inversion
- Bilateral tubal ligation (post-second delivery)

MEDICATIONS:
Home Medications:
- Ferrous sulfate 325 mg PO daily
- Desmopressin acetate 150 mcg intranasal spray as needed for bleeding episodes
- Ethinyl estradiol/norethindrone acetate 1 tablet PO daily (continuous cycling for hormone replacement)
- Vitamin D3 2000 IU PO daily

ALLERGIES:
NKDA, Latex (contact dermatitis)

FAMILY HISTORY:
She mentions that her mother has a similar bleeding tendency.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Librarian.

VITAL SIGNS:
On examination, her blood pressure is 110/70 mmHg, heart rate is 98 bpm, and she appears pale.

PHYSICAL EXAMINATION:
Her uterus is firm on palpation,

LABORATORY DATA:
Laboratory studies reveal:

- Hemoglobin: 9.2 g/dL (12.0-15.5 g/dL)
- Platelet count: 250,000/µL (150,000-400,000/µL)
- Prothrombin time (PT): 12 seconds (11-13.5 seconds)
- Activated partial thromboplastin time (aPTT): 38 seconds (25-35 seconds)
- Fibrinogen: Normal

IMAGING:
There is no evidence of retained placental tissue on ultrasound.

## Encounter 4: source 2020-01-15, practice 2023-07-04, inpatient, Internal Medicine

Chief complaint: Worsening fatigue, difficulty caring for newborn, and low breastmilk production one week after emergency cesarean section

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Priya Patel, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Worsening fatigue, difficulty caring for newborn, and low breastmilk production one week after emergency cesarean section

HISTORY OF PRESENT ILLNESS:
The patient is a 32-year-old female with Von Willebrand disease Type 1, Kallmann syndrome, and chronic iron deficiency anemia who presents for inpatient evaluation one week after undergoing an emergency cesarean section complicated by acute uterine inversion and postpartum hemorrhage. The delivery was complicated by significant intraoperative bleeding, during which her blood pressure dropped to 80/40 mm Hg. She required transfusion of 4 units of packed red blood cells. Since the surgery, she has experienced worsening fatigue and difficulty caring for her newborn. She also reports that her breastmilk production has been unusually low despite frequent attempts at breastfeeding.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute uterine inversion (diagnosed 2020-01-15)
- Acute postpartum hemorrhage (diagnosed 2020-01-15)
- Secondary postpartum hemorrhage due to retained products of conception (diagnosed 2020-01-15)
- Von Willebrand disease (diagnosed 2020-01-15)
- Von Willebrand disease (Type 1)
- Kallmann syndrome (isolated hypogonadotropic hypogonadism with anosmia)
- Chronic iron deficiency anemia
- History of infertility secondary to hypogonadism

PAST SURGICAL HISTORY:
- Emergency low transverse cesarean section (age 32)
- Repeat cesarean section (age 34)
- Dilation and curettage for retained products of conception
- Manual reduction of uterine inversion
- Bilateral tubal ligation (post-second delivery)

MEDICATIONS:
Home Medications:
- Ferrous sulfate 325 mg PO daily
- Desmopressin acetate 150 mcg intranasal spray as needed for bleeding episodes
- Ethinyl estradiol/norethindrone acetate 1 tablet PO daily (continuous cycling for hormone replacement)
- Vitamin D3 2000 IU PO daily

ALLERGIES:
NKDA, Latex (contact dermatitis)

FAMILY HISTORY:
- Mother: Type 1 Von Willebrand disease, menorrhagia requiring hysterectomy at age 42
- Father: Delayed puberty (age 16), poor sense of smell (presumed variant Kallmann syndrome)
- Sister: Primary infertility, secondary amenorrhea

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Librarian.

PHYSICAL EXAMINATION:
On physical exam, she appears pale and mildly hypotensive. Her surgical wound is healing appropriately, and there are no signs of infection.

## Encounter 5: source 2020-03-15, practice 2023-08-30, outpatient, Internal Medicine

Chief complaint: Persistent fatigue and inability to breastfeed six weeks after delivery of first child

ENCOUNTER NOTE
Date: 2020-03-15
Provider: Dr. Robert Thompson, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent fatigue and inability to breastfeed six weeks after delivery of first child

HISTORY OF PRESENT ILLNESS:
The patient is a 32-year-old female with Von Willebrand disease Type 1, Kallmann syndrome, and chronic iron deficiency anemia who returns six weeks after a complicated delivery with uterine inversion and hemorrhage. She presents with fatigue, inability to breastfeed, and a recent history of significant postpartum hemorrhage following the delivery of her first child 6 weeks ago, with reports of insufficient breast milk and formula supplementation. Additionally, she complains of persistent dizziness, especially upon standing, and has lost her appetite.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute uterine inversion (diagnosed 2020-01-15)
- Acute postpartum hemorrhage (diagnosed 2020-01-15)
- Secondary postpartum hemorrhage due to retained products of conception (diagnosed 2020-01-15)
- Von Willebrand disease (diagnosed 2020-01-15)
- Kallmann syndrome (diagnosed 2020-01-15)
- Von Willebrand disease (Type 1)
- Kallmann syndrome (isolated hypogonadotropic hypogonadism with anosmia)
- Chronic iron deficiency anemia
- History of infertility secondary to hypogonadism

PAST SURGICAL HISTORY:
- Emergency low transverse cesarean section (age 32)
- Repeat cesarean section (age 34)
- Dilation and curettage for retained products of conception
- Manual reduction of uterine inversion
- Bilateral tubal ligation (post-second delivery)

MEDICATIONS:
Home Medications:
- Ferrous sulfate 325 mg PO daily
- Desmopressin acetate 150 mcg intranasal spray as needed for bleeding episodes
- Ethinyl estradiol/norethindrone acetate 1 tablet PO daily (continuous cycling for hormone replacement)
- Vitamin D3 2000 IU PO daily

ALLERGIES:
NKDA, Latex (contact dermatitis)

FAMILY HISTORY:
- Mother: Type 1 Von Willebrand disease, menorrhagia requiring hysterectomy at age 42
- Father: Delayed puberty (age 16), poor sense of smell (presumed variant Kallmann syndrome)
- Sister: Primary infertility, secondary amenorrhea

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Librarian.

VITAL SIGNS:
Her blood pressure is 90/60 mmHg, heart rate is 55 bpm,

PHYSICAL EXAMINATION:
Skin appears pale and slightly dry.

LABORATORY DATA:
Laboratory tests reveal the following:

- Sodium: 130 mEq/L (Normal: 135-145 mEq/L)
- Potassium: 5.2 mEq/L (Normal: 3.5-5.1 mEq/L)
- Serum cortisol (8 AM): 3 µg/dL (Normal: 10-20 µg/dL)
- Thyroid-stimulating hormone (TSH): 0.5 µIU/mL (Normal: 0.4-4.0 µIU/mL)
- Free thyroxine (T4): 0.7 ng/dL (Normal: 0.9-1.7 ng/dL)
- Prolactin: 5 ng/mL (Normal: 5-20 ng/mL in non-pregnant women)

## Encounter 6: source 2022-07-15, practice 2025-12-22, outpatient, Obstetrics & Gynecology

Chief complaint: Postoperative follow-up one week after repeat cesarean section delivery of second child

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Maria Rodriguez, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Postoperative follow-up one week after repeat cesarean section delivery of second child

HISTORY OF PRESENT ILLNESS:
The patient is a 34-year-old female with Kallmann syndrome and Von Willebrand disease type 1 who returns for postoperative follow-up one week after repeat cesarean section delivery of her second child, following a prior pregnancy complicated by uterine inversion and severe postpartum hemorrhage. Her delivery was complicated by significant hemorrhage requiring transfusion. She reports profound fatigue and an inability to produce milk despite frequent attempts at breastfeeding. She also mentions feeling lightheaded when standing and has had difficulty concentrating. She notes that she has not had any vaginal bleeding or lochia since delivery.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute uterine inversion (diagnosed 2020-01-15)
- Acute postpartum hemorrhage (diagnosed 2020-01-15)
- Secondary postpartum hemorrhage due to retained products of conception (diagnosed 2020-01-15)
- Von Willebrand disease (diagnosed 2020-01-15)
- Kallmann syndrome (diagnosed 2020-01-15)
- Von Willebrand disease (Type 1)
- Kallmann syndrome (isolated hypogonadotropic hypogonadism with anosmia)
- Chronic iron deficiency anemia
- History of infertility secondary to hypogonadism

PAST SURGICAL HISTORY:
- Emergency low transverse cesarean section (age 32)
- Repeat cesarean section (age 34)
- Dilation and curettage for retained products of conception
- Manual reduction of uterine inversion
- Bilateral tubal ligation (post-second delivery)

MEDICATIONS:
Home Medications:
- Ferrous sulfate 325 mg PO daily
- Desmopressin acetate 150 mcg intranasal spray as needed for bleeding episodes
- Ethinyl estradiol/norethindrone acetate 1 tablet PO daily (continuous cycling for hormone replacement)
- Vitamin D3 2000 IU PO daily

ALLERGIES:
NKDA, Latex (contact dermatitis)

FAMILY HISTORY:
- Mother: Type 1 Von Willebrand disease, menorrhagia requiring hysterectomy at age 42
- Father: Delayed puberty (age 16), poor sense of smell (presumed variant Kallmann syndrome)
- Sister: Primary infertility, secondary amenorrhea

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Librarian.

VITAL SIGNS:
Vital signs show a blood pressure of 92/56 mm Hg and pulse of 92/min.

PHYSICAL EXAMINATION:
Physical examination is unremarkable. Her skin appears pale,

LABORATORY DATA:
Labs show sodium of 132 mmol/L and glucose of 64 mg/dL. A pregnancy test is negative.

## Encounter 7: source 2023-01-15, practice 2026-06-23, outpatient, Endocrinology

Chief complaint: Six-month history of amenorrhea and inability to breastfeed since delivering second child

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Priya Patel, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Six-month history of amenorrhea and inability to breastfeed since delivering second child

HISTORY OF PRESENT ILLNESS:
The patient is a 35-year-old female with known Kallmann syndrome and Von Willebrand disease who returns to Endocrinology six months following the delivery of her second child. She presents with a 6-month history of amenorrhea and inability to breastfeed since that delivery, complicated by a significant postpartum hemorrhage requiring blood transfusion. She reports feeling fatigued with a notable decrease in pubic and axillary hair.

PAST MEDICAL HISTORY:
Active Problem List:
- Acute uterine inversion (diagnosed 2020-01-15)
- Acute postpartum hemorrhage (diagnosed 2020-01-15)
- Secondary postpartum hemorrhage due to retained products of conception (diagnosed 2020-01-15)
- Von Willebrand disease (diagnosed 2020-01-15)
- Kallmann syndrome (diagnosed 2020-01-15)
- Von Willebrand disease (Type 1)
- Kallmann syndrome (isolated hypogonadotropic hypogonadism with anosmia)
- Chronic iron deficiency anemia
- History of infertility secondary to hypogonadism

PAST SURGICAL HISTORY:
- Emergency low transverse cesarean section (age 32)
- Repeat cesarean section (age 34)
- Dilation and curettage for retained products of conception
- Manual reduction of uterine inversion
- Bilateral tubal ligation (post-second delivery)

MEDICATIONS:
Home Medications:
- Ferrous sulfate 325 mg PO daily
- Desmopressin acetate 150 mcg intranasal spray as needed for bleeding episodes
- Ethinyl estradiol/norethindrone acetate 1 tablet PO daily (continuous cycling for hormone replacement)
- Vitamin D3 2000 IU PO daily

ALLERGIES:
NKDA, Latex (contact dermatitis)

FAMILY HISTORY:
- Mother: Type 1 Von Willebrand disease, menorrhagia requiring hysterectomy at age 42
- Father: Delayed puberty (age 16), poor sense of smell (presumed variant Kallmann syndrome)
- Sister: Primary infertility, secondary amenorrhea

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Librarian.

VITAL SIGNS:
Her blood pressure is 90/60 mmHg.

PHYSICAL EXAMINATION:
Physical examination reveals a BMI of 19, pallor, and an absence of galactorrhea.

LABORATORY DATA:
Laboratory tests show the following:

- Serum sodium: 130 mEq/L (Normal: 135-145 mEq/L)
- Serum potassium: 5.2 mEq/L (Normal: 3.5-5.1 mEq/L)
- Serum calcium: 9.0 mg/dL (Normal: 8.5-10.2 mg/dL)
- Serum cortisol at 8 AM: 3 µg/dL (Normal: 7-25 µg/dL)
- TSH: 0.5 µIU/mL (Normal: 0.5-5.0 µIU/mL)
- Free T4: 0.8 ng/dL (Normal: 0.9-1.7 ng/dL)
- Prolactin: 5 ng/mL (Normal: 5-20 ng/mL)
