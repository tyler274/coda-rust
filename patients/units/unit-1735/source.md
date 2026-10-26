# synthetic_hospital patient 1735

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 21 at first encounter, sex F, African American, University student health insurance plan.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-11-30 (Wed) | outpatient | Endocrinology |
| 1 | 2020-04-15 | 2023-02-24 (Fri) | outpatient | Psychiatry |
| 2 | 2020-10-15 | 2023-08-21 (Mon) | outpatient | Psychiatry |
| 3 | 2021-04-15 | 2024-03-04 (Mon) | outpatient | Pulmonology |
| 4 | 2021-10-15 | 2024-08-26 (Mon) | outpatient | Gastroenterology |
| 5 | 2022-04-15 | 2025-02-28 (Fri) | outpatient | Orthopedics |
| 6 | 2022-10-15 | 2025-08-15 (Fri) | outpatient | Orthopedics |
| 7 | 2023-04-15 | 2026-02-20 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "University student health insurance plan",
 "occupation": "University student, Division I cross-country athlete",
 "alcohol_use": "None",
 "family_history": [
  "Mother: Sickle cell trait, hypothyroidism",
  "Father: Sickle cell trait",
  "Maternal grandmother: Systemic sclerosis",
  "Paternal uncle: Crohn's disease",
  "Older sister: Anorexia nervosa (in remission)"
 ],
 "race_ethnicity": "African American",
 "smoking_status": "Never",
 "home_medications": [
  {
   "dose": "1000 mg PO daily",
   "name": "Hydroxyurea"
  },
  {
   "dose": "1 mg PO daily",
   "name": "Folic acid"
  },
  {
   "dose": "1.2 g PO twice daily",
   "name": "Mesalamine delayed-release"
  },
  {
   "dose": "20 mg PO three times daily",
   "name": "Sildenafil"
  },
  {
   "dose": "600 mg/400 IU PO twice daily",
   "name": "Calcium carbonate with Vitamin D3"
  }
 ],
 "surgical_history": [
  "Laparoscopic cholecystectomy (age 19) for symptomatic cholelithiasis secondary to chronic hemolysis"
 ],
 "chronic_conditions": [
  "Sickle cell disease (HbSS)",
  "Systemic sclerosis (limited cutaneous)",
  "Crohn's disease (ileocolonic)",
  "Anorexia nervosa, restricting type",
  "Osteoporosis (multifactorial: disuse, hypogonadism, malnutrition)",
  "Pulmonary arterial hypertension (WHO Group 1, associated with systemic sclerosis)",
  "Secondary amenorrhea"
 ],
 "age_at_first_encounter": 21
}
```

Primary diagnoses: ["Disuse osteoporosis (Immobilization)", "Anorexia nervosa, restricting type", "Crohn's disease flare with inflammatory stricture (without complete obstruction)", "Female Athlete Triad", "Toxic megacolon secondary to Clostridioides difficile infection", "Pulmonary arterial hypertension associated with systemic sclerosis", "Tibial stress fracture"]
Comorbidities: ["Cancer-related constitutional symptoms", "Anemia", "Hypokalemia managed with oral replacement (tolerating PO intake)", "Secondary Amenorrhea", "Anorexia nervosa, restricting type"]

## Encounter 0: source 2020-01-15, practice 2022-11-30, outpatient, Endocrinology

Chief complaint: Fatigue, dizziness, and a recent episode of syncope

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue, dizziness, and a recent episode of syncope

HISTORY OF PRESENT ILLNESS:
She is an active member of the university's cross-country team. Despite her high level of physical activity, she expresses intense fear of gaining weight and admits to restricting her caloric intake significantly. She reports that her menstrual periods have become irregular over the past 6 months.

PAST MEDICAL HISTORY:
Active Problem List:
- Sickle cell disease (HbSS)
- Systemic sclerosis (limited cutaneous)
- Crohn's disease (ileocolonic)
- Anorexia nervosa, restricting type
- Osteoporosis (multifactorial: disuse, hypogonadism, malnutrition)
- Pulmonary arterial hypertension (WHO Group 1, associated with systemic sclerosis)
- Secondary amenorrhea

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (age 19) for symptomatic cholelithiasis secondary to chronic hemolysis

MEDICATIONS:
Home Medications:
- Hydroxyurea 1000 mg PO daily
- Folic acid 1 mg PO daily
- Mesalamine delayed-release 1.2 g PO twice daily
- Sildenafil 20 mg PO three times daily
- Calcium carbonate with Vitamin D3 600 mg/400 IU PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Sickle cell trait, hypothyroidism
- Father: Sickle cell trait
- Maternal grandmother: Systemic sclerosis
- Paternal uncle: Crohn's disease
- Older sister: Anorexia nervosa (in remission)

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: University student, Division I cross-country athlete.

VITAL SIGNS:
Her vital signs reveal a blood pressure of 90/60 mmHg and a heart rate of 48 bpm.

PHYSICAL EXAMINATION:
On examination, her BMI is calculated to be 16.5 kg/m^2. Physical examination is notable for mild bilateral ankle edema and a soft systolic murmur.

LABORATORY DATA:
Hemoglobin: 12.1 g/dL
Serum potassium: 3.2 mEq/L
Serum chloride: 98 mEq/L
Serum calcium: 8.9 mg/dL
Thyroid-stimulating hormone (TSH): 2.5 μIU/mL

OTHER STUDIES:
ECG shows sinus bradycardia with no other abnormalities.

## Encounter 1: source 2020-04-15, practice 2023-02-24, outpatient, Psychiatry

Chief complaint: Fatigue, cold intolerance, and hair loss over the past six months

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Marcus Johnson, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Fatigue, cold intolerance, and hair loss over the past six months

HISTORY OF PRESENT ILLNESS:
This 21-year-old with sickle cell disease, systemic sclerosis, Crohn's disease, and anorexia nervosa presents to psychiatry three months after endocrinology evaluation. She reports 15 kg loss during this period from strict dieting and excessive exercise, fearing weight gain despite underweight, with irregular menses.

PAST MEDICAL HISTORY:
Active Problem List:
- Anorexia nervosa, restricting type (diagnosed 2020-01-15)
- Sickle cell disease (HbSS)
- Systemic sclerosis (limited cutaneous)
- Crohn's disease (ileocolonic)
- Osteoporosis (multifactorial: disuse, hypogonadism, malnutrition)
- Pulmonary arterial hypertension (WHO Group 1, associated with systemic sclerosis)
- Secondary amenorrhea

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (age 19) for symptomatic cholelithiasis secondary to chronic hemolysis

MEDICATIONS:
Home Medications:
- Hydroxyurea 1000 mg PO daily
- Folic acid 1 mg PO daily
- Mesalamine delayed-release 1.2 g PO twice daily
- Sildenafil 20 mg PO three times daily
- Calcium carbonate with Vitamin D3 600 mg/400 IU PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Sickle cell trait, hypothyroidism
- Father: Sickle cell trait
- Maternal grandmother: Systemic sclerosis
- Paternal uncle: Crohn's disease
- Older sister: Anorexia nervosa (in remission)

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: University student, Division I cross-country athlete.

VITAL SIGNS:
Her vital signs are within normal limits.

PHYSICAL EXAMINATION:
On examination, her BMI is 16.5 kg/m^2, and she has dry, scaly skin with fine downy hair (lanugo) on her face and back.

LABORATORY DATA:
Laboratory tests reveal the following:

- Hemoglobin: 12.1 g/dL (normal: 12-16 g/dL)
- Serum albumin: 3.2 g/dL (normal: 3.5-5.0 g/dL)
- Thyroid-stimulating hormone (TSH): 4.5 mIU/L (normal: 0.4-4.0 mIU/L)
- Total cholesterol: 210 mg/dL (normal: <200 mg/dL)
- Electrolytes: within normal limits

## Encounter 2: source 2020-10-15, practice 2023-08-21, outpatient, Psychiatry

Chief complaint: Persistent fatigue and difficulty concentrating

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Marcus Johnson, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent fatigue and difficulty concentrating

HISTORY OF PRESENT ILLNESS:
The patient is a 21-year-old female with sickle cell disease, systemic sclerosis, and Crohn's disease who returns for psychiatric follow-up approximately six months after her last visit, again presenting with concerns regarding food intake and exercise patterns initially addressed in Endocrinology earlier this year. She reports a recent change in lifestyle to "feel healthier" and has been strictly limiting her food intake and exercising intensely every day. She avoids carbohydrates and fats, frequently skips meals, and becomes anxious when she cannot complete her daily workout. She denies purging or using laxatives but admits she feels guilty if she eats more than planned. Her menstrual periods stopped approximately one year ago, which she attributes to work stress and weight changes. She weighs herself daily and says she's comfortable maintaining a low body weight because she "feels in control."

PAST MEDICAL HISTORY:
Active Problem List:
- Anorexia nervosa, restricting type (diagnosed 2020-01-15)
- Sickle cell disease (HbSS)
- Systemic sclerosis (limited cutaneous)
- Crohn's disease (ileocolonic)
- Osteoporosis (multifactorial: disuse, hypogonadism, malnutrition)
- Pulmonary arterial hypertension (WHO Group 1, associated with systemic sclerosis)
- Secondary amenorrhea

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (age 19) for symptomatic cholelithiasis secondary to chronic hemolysis

MEDICATIONS:
She takes any medications.

Home Medications:
- Hydroxyurea 1000 mg PO daily
- Folic acid 1 mg PO daily
- Mesalamine delayed-release 1.2 g PO twice daily
- Sildenafil 20 mg PO three times daily
- Calcium carbonate with Vitamin D3 600 mg/400 IU PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Sickle cell trait, hypothyroidism
- Father: Sickle cell trait
- Maternal grandmother: Systemic sclerosis
- Paternal uncle: Crohn's disease
- Older sister: Anorexia nervosa (in remission)

SOCIAL HISTORY:
She denies alcohol, drug use,

PHYSICAL EXAMINATION:
On physical examination, she appears thin, with a BMI of 17.3. Her skin is dry and fine hair is visible over her back and arms. Cardiopulmonary and neurologic exams are normal.

LABORATORY DATA:
Labs show mild hypokalemia and normocytic anemia.

## Encounter 3: source 2021-04-15, practice 2024-03-04, outpatient, Pulmonology

Chief complaint: Progressive shortness of breath on exertion and a dry cough

ENCOUNTER NOTE
Date: 2021-04-15
Provider: Dr. David Rodriguez, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive shortness of breath on exertion and a dry cough

HISTORY OF PRESENT ILLNESS:
The patient with systemic sclerosis and history of anorexia nervosa presents to the clinic. She reports a 6-month history of progressive shortness of breath on exertion and a dry cough.

PAST MEDICAL HISTORY:
Active Problem List:
- Anorexia nervosa, restricting type (diagnosed 2020-01-15)
- Disuse osteoporosis (Immobilization) (diagnosed 2020-10-15)
- Sickle cell disease (HbSS)
- Systemic sclerosis (limited cutaneous)
- Crohn's disease (ileocolonic)
- Osteoporosis (multifactorial: disuse, hypogonadism, malnutrition)
- Pulmonary arterial hypertension (WHO Group 1, associated with systemic sclerosis)
- Secondary amenorrhea

She has a past medical history of sickle cell disease and has been hospitalized twice in the past year for vaso-occlusive crises.

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (age 19) for symptomatic cholelithiasis secondary to chronic hemolysis

MEDICATIONS:
Home Medications:
- Hydroxyurea 1000 mg PO daily
- Folic acid 1 mg PO daily
- Mesalamine delayed-release 1.2 g PO twice daily
- Sildenafil 20 mg PO three times daily
- Calcium carbonate with Vitamin D3 600 mg/400 IU PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Sickle cell trait, hypothyroidism
- Father: Sickle cell trait
- Maternal grandmother: Systemic sclerosis
- Paternal uncle: Crohn's disease
- Older sister: Anorexia nervosa (in remission)

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: University student, Division I cross-country athlete.

VITAL SIGNS:
Respiratory rate of 22 breaths per minute. Her oxygen saturation is 94% on room air.

PHYSICAL EXAMINATION:
On examination, she appears tachypneic. Cardiac examination reveals a loud S2 and a 3/6 systolic ejection murmur along the left sternal border. Abdominal examination shows mild hepatomegaly. Lower extremity examination is notable for 1+ pitting edema.

LABORATORY DATA:
Laboratory values are as follows:

- Hemoglobin: 8.5 g/dL
- Hematocrit: 26%
- Lactate dehydrogenase (LDH): Elevated
- Total bilirubin: Elevated
- Haptoglobin: Decreased

IMAGING:
Chest X-ray demonstrates enlarged pulmonary arteries and an echocardiogram reveals right ventricular hypertrophy with a tricuspid regurgitant jet velocity of 3.4 m/s.

## Encounter 4: source 2021-10-15, practice 2024-08-26, outpatient, Gastroenterology

Chief complaint: Intermittent abdominal pain, non-bloody diarrhea, and a 10-pound weight loss

ENCOUNTER NOTE
Date: 2021-10-15
Provider: Dr. Emily Watson, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent abdominal pain, non-bloody diarrhea, and a 10-pound weight loss

HISTORY OF PRESENT ILLNESS:
The patient, with known ileocolonic Crohn's disease, systemic sclerosis, and anorexia nervosa, returns six months after her last Pulmonology visit and presents with a 6-month history of intermittent abdominal pain, non-bloody diarrhea, and a 10-pound weight loss. She notes symptoms are worse after meals and she occasionally notices mouth ulcers.

PAST MEDICAL HISTORY:
Active Problem List:
- Anorexia nervosa, restricting type (diagnosed 2020-01-15)
- Disuse osteoporosis (Immobilization) (diagnosed 2020-10-15)
- Pulmonary arterial hypertension associated with systemic sclerosis (diagnosed 2021-04-15)
- Sickle cell disease (HbSS)
- Systemic sclerosis (limited cutaneous)
- Crohn's disease (ileocolonic)
- Osteoporosis (multifactorial: disuse, hypogonadism, malnutrition)
- Pulmonary arterial hypertension (WHO Group 1, associated with systemic sclerosis)
- Secondary amenorrhea

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (age 19) for symptomatic cholelithiasis secondary to chronic hemolysis

MEDICATIONS:
Home Medications:
- Hydroxyurea 1000 mg PO daily
- Folic acid 1 mg PO daily
- Mesalamine delayed-release 1.2 g PO twice daily
- Sildenafil 20 mg PO three times daily
- Calcium carbonate with Vitamin D3 600 mg/400 IU PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
She denies any family history of gastrointestinal diseases.

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: University student, Division I cross-country athlete.

PHYSICAL EXAMINATION:
On physical examination, she is afebrile, and her abdominal exam reveals tenderness in the right lower quadrant without rebound or guarding. Bowel sounds are normal.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 11.2 g/dL (Normal: 12-16 g/dL)
- Mean corpuscular volume (MCV): 84 fL (Normal: 80-100 fL)
- C-reactive protein (CRP): 24 mg/L (Normal: <5 mg/L)
- Erythrocyte sedimentation rate (ESR): 40 mm/hr (Normal: 0-20 mm/hr)
- Fecal calprotectin: 250 µg/g (Normal: <50 µg/g)

## Encounter 5: source 2022-04-15, practice 2025-02-28, outpatient, Orthopedics

Chief complaint: Persistent pain in her left shin, which worsens with running

ENCOUNTER NOTE
Date: 2022-04-15
Provider: Dr. James Park, MD
Department: Orthopedics
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent pain in her left shin, which worsens with running

HISTORY OF PRESENT ILLNESS:
The patient is a 23-year-old female with sickle cell disease, systemic sclerosis, Crohn's disease, anorexia nervosa, and osteoporosis who returns for evaluation of persistent left shin pain. She presents to orthopedics following her last gastroenterology encounter six months ago for a Crohn's disease flare. She reports that she has been training for a marathon over the past 6 months and has recently increased her running distance. She denies any specific injury but mentions that the pain has been gradually increasing. She also expresses frustration about having to reduce her training due to the pain. On further questioning, she reveals that she has been following a strict diet to maintain her weight for competitive running and admits to being fearful of gaining weight. She has not had a menstrual period for the last 4 months.

PAST MEDICAL HISTORY:
Active Problem List:
- Anorexia nervosa, restricting type (diagnosed 2020-01-15)
- Disuse osteoporosis (Immobilization) (diagnosed 2020-10-15)
- Pulmonary arterial hypertension associated with systemic sclerosis (diagnosed 2021-04-15)
- Crohn's disease flare with inflammatory stricture (without complete obstruction) (diagnosed 2021-10-15)
- Sickle cell disease (HbSS)
- Systemic sclerosis (limited cutaneous)
- Crohn's disease (ileocolonic)
- Osteoporosis (multifactorial: disuse, hypogonadism, malnutrition)
- Pulmonary arterial hypertension (WHO Group 1, associated with systemic sclerosis)
- Secondary amenorrhea

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (age 19) for symptomatic cholelithiasis secondary to chronic hemolysis

MEDICATIONS:
Home Medications:
- Hydroxyurea 1000 mg PO daily
- Folic acid 1 mg PO daily
- Mesalamine delayed-release 1.2 g PO twice daily
- Sildenafil 20 mg PO three times daily
- Calcium carbonate with Vitamin D3 600 mg/400 IU PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Sickle cell trait, hypothyroidism
- Father: Sickle cell trait
- Maternal grandmother: Systemic sclerosis
- Paternal uncle: Crohn's disease
- Older sister: Anorexia nervosa (in remission)

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: University student, Division I cross-country athlete.

PHYSICAL EXAMINATION:
On examination, her BMI is 17.5 kg/m^2, and there is tenderness over the middle third of the tibia. No swelling or redness is noted.

LABORATORY DATA:
Laboratory tests reveal the following:

- Serum calcium: 9.0 mg/dL (Normal: 8.6-10.2 mg/dL)
- Serum phosphorus: 3.5 mg/dL (Normal: 2.5-4.5 mg/dL)
- Alkaline phosphatase: 70 U/L (Normal: 40-150 U/L)
- TSH: 2.5 mIU/L (Normal: 0.4-4.0 mIU/L)
- Estradiol: 15 pg/mL (Normal for follicular phase: 30-120 pg/mL)

## Encounter 6: source 2022-10-15, practice 2025-08-15, outpatient, Orthopedics

Chief complaint: Progressive pain in her right lower leg

ENCOUNTER NOTE
Date: 2022-10-15
Provider: Dr. James Park, MD
Department: Orthopedics
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive pain in her right lower leg

HISTORY OF PRESENT ILLNESS:
The patient returns with progressive contralateral lower leg pain six months after prior evaluation for left shin pain in the setting of multifactorial osteoporosis. She is a cross-country runner who recently increased training intensity for an upcoming competition. She denies any trauma but mentions that the pain worsens with activity and improves with rest.

PAST MEDICAL HISTORY:
Active Problem List:
- Anorexia nervosa, restricting type (diagnosed 2020-01-15)
- Disuse osteoporosis (Immobilization) (diagnosed 2020-10-15)
- Pulmonary arterial hypertension associated with systemic sclerosis (diagnosed 2021-04-15)
- Crohn's disease flare with inflammatory stricture (without complete obstruction) (diagnosed 2021-10-15)
- Female Athlete Triad (diagnosed 2022-04-15)
- Sickle cell disease (HbSS)
- Systemic sclerosis (limited cutaneous)
- Crohn's disease (ileocolonic)
- Osteoporosis (multifactorial: disuse, hypogonadism, malnutrition)
- Pulmonary arterial hypertension (WHO Group 1, associated with systemic sclerosis)
- Secondary amenorrhea

She has a history of irregular menstrual cycles, occurring every 2-3 months.

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (age 19) for symptomatic cholelithiasis secondary to chronic hemolysis

MEDICATIONS:
Home Medications:
- Hydroxyurea 1000 mg PO daily
- Folic acid 1 mg PO daily
- Mesalamine delayed-release 1.2 g PO twice daily
- Sildenafil 20 mg PO three times daily
- Calcium carbonate with Vitamin D3 600 mg/400 IU PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Sickle cell trait, hypothyroidism
- Father: Sickle cell trait
- Maternal grandmother: Systemic sclerosis
- Paternal uncle: Crohn's disease
- Older sister: Anorexia nervosa (in remission)

SOCIAL HISTORY:
She appears thin, and upon dietary questioning, admits to restricting her caloric intake to maintain her current weight for performance purposes.

PHYSICAL EXAMINATION:
On physical examination, there is localized tenderness over the mid-distal tibia, and mild swelling is noted. No erythema or warmth is present.

IMAGING:
Radiographs of the right tibia are unremarkable. A bone scan is ordered, which shows an area of increased uptake in the mid-distal tibia.

## Encounter 7: source 2023-04-15, practice 2026-02-20, ed, Emergency Medicine

Chief complaint: Severe abdominal pain, bloody diarrhea, and fever

ENCOUNTER NOTE
Date: 2023-04-15
Provider: Dr. Aisha Washington, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Severe abdominal pain, bloody diarrhea, and fever

HISTORY OF PRESENT ILLNESS:
The patient is a 24-year-old female with a history of Crohn's disease, most recently evaluated in 2021 for inflammatory stricture, who presents to the emergency department with a 2-day history of severe abdominal pain, bloody diarrhea, and fever. She reports that she has experienced intermittent bloody diarrhea and abdominal cramping for the past 6 months but has not sought medical attention until now.

PAST MEDICAL HISTORY:
Active Problem List:
- Anorexia nervosa, restricting type (diagnosed 2020-01-15)
- Disuse osteoporosis (Immobilization) (diagnosed 2020-10-15)
- Pulmonary arterial hypertension associated with systemic sclerosis (diagnosed 2021-04-15)
- Crohn's disease flare with inflammatory stricture (without complete obstruction) (diagnosed 2021-10-15)
- Female Athlete Triad (diagnosed 2022-04-15)
- Tibial stress fracture (diagnosed 2022-10-15)
- Sickle cell disease (HbSS)
- Systemic sclerosis (limited cutaneous)
- Crohn's disease (ileocolonic)
- Osteoporosis (multifactorial: disuse, hypogonadism, malnutrition)
- Pulmonary arterial hypertension (WHO Group 1, associated with systemic sclerosis)
- Secondary amenorrhea

PAST SURGICAL HISTORY:
- Laparoscopic cholecystectomy (age 19) for symptomatic cholelithiasis secondary to chronic hemolysis

MEDICATIONS:
Home Medications:
- Hydroxyurea 1000 mg PO daily
- Folic acid 1 mg PO daily
- Mesalamine delayed-release 1.2 g PO twice daily
- Sildenafil 20 mg PO three times daily
- Calcium carbonate with Vitamin D3 600 mg/400 IU PO twice daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother: Sickle cell trait, hypothyroidism
- Father: Sickle cell trait
- Maternal grandmother: Systemic sclerosis
- Paternal uncle: Crohn's disease
- Older sister: Anorexia nervosa (in remission)

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: University student, Division I cross-country athlete.

VITAL SIGNS:
Her vital signs are: temperature 38.9°C, blood pressure 100/60 mmHg, heart rate 110 beats/min, and respiratory rate 22 breaths/min.

PHYSICAL EXAMINATION:
Physical examination reveals a distended abdomen with tenderness to palpation diffusely, but no rebound or guarding. Bowel sounds are hypoactive.

LABORATORY DATA:
Laboratory findings are as follows:

- Hemoglobin: 10.2 g/dL (Normal: 12-16 g/dL)
- White blood cell count: 18,000/mm³ (Normal: 4,500-11,000/mm³)
- Platelet count: 450,000/mm³ (Normal: 150,000-450,000/mm³)
- C-reactive protein: 25 mg/L (Normal: <5 mg/L)
- Albumin: 3.0 g/dL (Normal: 3.5-5.0 g/dL)

IMAGING:
An abdominal x-ray shows colonic dilation with no evidence of perforation.
