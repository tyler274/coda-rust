# synthetic_hospital patient 2098

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex M, Vietnamese, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-06-28 (Mon) | outpatient | Gastroenterology |
| 1 | 2020-05-15 | 2021-10-21 (Thu) | outpatient | Endocrinology |
| 2 | 2021-01-15 | 2022-06-20 (Mon) | outpatient | Internal Medicine |
| 3 | 2021-09-15 | 2023-02-15 (Wed) | outpatient | Gastroenterology |
| 4 | 2022-09-15 | 2024-02-17 (Sat) | ed | Emergency Medicine |
| 5 | 2023-07-15 | 2024-12-23 (Mon) | ed | Emergency Medicine |
| 6 | 2024-09-15 | 2026-03-05 (Thu) | icu | Critical Care Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicaid",
 "occupation": "Unemployed; formerly employed as software engineer",
 "alcohol_use": "Chronic alcohol use disorder with periods of sobriety and intermittent relapses",
 "family_history": [
  "Unknown family history (limited information due to immigration)",
  "No known family history of hereditary pancreatitis or MEN1"
 ],
 "race_ethnicity": "Vietnamese",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "36,000 units lipase with meals",
   "name": "Pancrelipase"
  },
  {
   "dose": "20 mg twice daily",
   "name": "Omeprazole"
  },
  {
   "dose": "325 mg as needed for headache",
   "name": "Aspirin"
  },
  {
   "dose": "One tablet daily",
   "name": "Multivitamin"
  }
 ],
 "surgical_history": [
  "Partial colectomy for ischemic colitis (age 44)"
 ],
 "chronic_conditions": [
  "Chronic pancreatitis with exocrine insufficiency",
  "Alcohol use disorder",
  "Status post partial colectomy",
  "Malnutrition"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Chronic pancreatitis (dietary management)", "Zollinger-Ellison syndrome", "Portal vein thrombosis", "Tuberculosis", "Scurvy", "Bile acid malabsorption", "NSAID-induced gastropathy or upper gastrointestinal bleeding"]
Comorbidities: ["Alcohol consumption", "Protein-energy malnutrition", "History of recurrent diverticulitis"]

## Encounter 0: source 2020-01-15, practice 2021-06-28, outpatient, Gastroenterology

Chief complaint: Recurrent episodes of severe upper abdominal pain radiating to the back, worse after high-fat meals, with oily stools and weight loss

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Recurrent episodes of severe upper abdominal pain radiating to the back, worse after high-fat meals, with oily stools and weight loss

HISTORY OF PRESENT ILLNESS:
That radiates to his back and worsens after eating, especially high-fat meals. He reports a 10-pound weight loss over the past two months and frequent, foul-smelling, oily stools.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic pancreatitis with exocrine insufficiency
- Alcohol use disorder
- Status post partial colectomy
- Malnutrition

His past medical history is significant for multiple episodes of acute pancreatitis.

PAST SURGICAL HISTORY:
- Partial colectomy for ischemic colitis (age 44)

MEDICATIONS:
Home Medications:
- Pancrelipase 36,000 units lipase with meals
- Omeprazole 20 mg twice daily
- Aspirin 325 mg as needed for headache
- Multivitamin One tablet daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Unknown family history (limited information due to immigration)
- No known family history of hereditary pancreatitis or MEN1

SOCIAL HISTORY:
With a history of chronic alcohol abuse

PHYSICAL EXAMINATION:
On physical examination, the patient appears cachectic and is tender to palpation in the epigastric region.

LABORATORY DATA:
Laboratory tests reveal:

- Serum amylase: Elevated
- Serum lipase: Elevated
- Fecal elastase: Low
- Hemoglobin A1c: 5.8%
- Triglycerides: 150 mg/dL

IMAGING:
An abdominal CT scan shows calcifications within the pancreas.

ASSESSMENT AND PLAN:
Based on the clinical presentation and imaging findings,

## Encounter 1: source 2020-05-15, practice 2021-10-21, outpatient, Endocrinology

Chief complaint: Persistent burning epigastric pain for 9 months, worsening after meals and waking him at night, refractory to standard PPI therapy

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. James Rodriguez, MD
Department: Endocrinology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent burning epigastric pain for 9 months, worsening after meals and waking him at night, refractory to standard PPI therapy

HISTORY OF PRESENT ILLNESS:
The patient returns for evaluation approximately four months following his January 2020 gastroenterology encounter for chronic pancreatitis, now presenting with progressive upper gastrointestinal symptoms despite ongoing pancreatic enzyme replacement therapy. He reports that for the past 9 months he has had burning epigastric pain that worsens after meals and often wakes him at night. He has been treated by his primary care physician with standard doses of proton pump inhibitors (PPIs), which initially gave mild relief, but his symptoms quickly returned. He has also been prescribed multiple courses of antibiotics for presumed Helicobacter pylori infection, but repeat testing shows no eradication despite appropriate therapy. The patient describes frequent episodes of nausea and occasional non-bloody vomiting. He has noticed black, tarry stools on several occasions and attributes this to "ulcers bleeding again." He has lost 15 pounds unintentionally in the past 6 months because eating often triggers pain. Over-the-counter antacids provide only transient relief. He is concerned that his symptoms are becoming debilitating and interfering with his job performance. Despite increasing his PPI dose to twice daily, his ulcers persist, and his pain continues.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic pancreatitis (dietary management) (diagnosed 2020-01-15)
- Chronic pancreatitis with exocrine insufficiency
- Alcohol use disorder
- Status post partial colectomy
- Malnutrition

PAST SURGICAL HISTORY:
- Partial colectomy for ischemic colitis (age 44)

MEDICATIONS:
Home Medications:
- Pancrelipase 36,000 units lipase with meals
- Omeprazole 20 mg twice daily
- Aspirin 325 mg as needed for headache
- Multivitamin One tablet daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
His family history is notable for a brother who had hypercalcemia and kidney stones in his 30s.

SOCIAL HISTORY:
He has no history of alcohol use disorder or smoking. He denies recent NSAID use. The patient works as a software engineer and lives with his partner.

REVIEW OF SYSTEMS:
On review of systems, the patient also reports intermittent diarrhea over the past several months. He notes that the stools are foul-smelling and greasy at times. He denies hematochezia, but the diarrhea seems to worsen when his ulcer pain flares.

VITAL SIGNS:
Temperature is 98.4°F, pulse 92/min, blood pressure 126/74 mmHg, and respirations 14/min.

PHYSICAL EXAMINATION:
On physical examination, the patient appears thin but not acutely ill. Abdominal exam shows mild tenderness in the epigastrium without rebound or guarding. No hepatosplenomegaly or palpable masses are noted. There is no scleral icterus, lymphadenopathy, or skin hyperpigmentation.

LABORATORY DATA:
Hemoglobin 10.9 g/dL
MCV 82 fL
Ferritin 12 ng/mL
Calcium 11.3 mg/dL
Stool studies demonstrate increased fat content, consistent with steatorrhea.

IMAGING:
Abdominal CT scan identifies a 2.5-cm enhancing mass in the head of the pancreas.

PATHOLOGY:
Biopsies are negative for H. pylori and malignancy.

OTHER STUDIES:
At his first gastroenterology visit, an upper endoscopy reveals multiple duodenal ulcers, including one in the distal duodenum. The gastric folds appear thickened, and the stomach contains copious amounts of acid.

PLAN:
The patient is admitted for further management.

## Encounter 2: source 2021-01-15, practice 2022-06-20, outpatient, Internal Medicine

Chief complaint: Persistent pain in legs and arms with easy bruising, petechiae, and bleeding gums

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Priya Patel, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent pain in legs and arms with easy bruising, petechiae, and bleeding gums

HISTORY OF PRESENT ILLNESS:
The patient with chronic pancreatitis, exocrine insufficiency, and malnutrition previously evaluated by Gastroenterology and Endocrinology presents with symptoms that began after he slipped on ice three months ago. He reports that bruises from the incident took an unusually long time to resolve and notes new petechiae over his lower legs. He also describes recent bleeding from his gums and general fatigue.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic pancreatitis (dietary management) (diagnosed 2020-01-15)
- Zollinger-Ellison syndrome (diagnosed 2020-05-15)
- Chronic pancreatitis with exocrine insufficiency
- Alcohol use disorder
- Status post partial colectomy
- Malnutrition

PAST SURGICAL HISTORY:
- Partial colectomy for ischemic colitis (age 44)

MEDICATIONS:
Home Medications:
- Pancrelipase 36,000 units lipase with meals
- Omeprazole 20 mg twice daily
- Aspirin 325 mg as needed for headache
- Multivitamin One tablet daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Unknown family history (limited information due to immigration)
- No known family history of hereditary pancreatitis or MEN1

SOCIAL HISTORY:
The patient is currently unemployed and admits to a diet primarily consisting of canned soup and white bread.

PHYSICAL EXAMINATION:
Physical examination reveals perifollicular hemorrhages, ecchymoses on the lower extremities, occasional bleeding from the gums and poor dentition. Joint range of motion is limited due to pain, but there is no warmth or swelling.

LABORATORY DATA:
Laboratory tests reveal a normal platelet count and coagulation profile.

IMAGING:
An X-ray of the left leg is shown below, demonstrating non-union fracture and aberrant callus formation.

## Encounter 3: source 2021-09-15, practice 2023-02-15, outpatient, Gastroenterology

Chief complaint: 2-week history of profuse, watery diarrhea occurring even during fasting and at night

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. Michael Park, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
2-week history of profuse, watery diarrhea occurring even during fasting and at night

HISTORY OF PRESENT ILLNESS:
The patient with chronic pancreatitis, refractory epigastric pain, and nutritional deficiency presents with diarrhea. He reports that the diarrhea persists even when he does not eat and occurs multiple times during the night. He denies recent travel or sick contacts.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic pancreatitis (dietary management) (diagnosed 2020-01-15)
- Zollinger-Ellison syndrome (diagnosed 2020-05-15)
- Scurvy (diagnosed 2021-01-15)
- Chronic pancreatitis with exocrine insufficiency
- Alcohol use disorder
- Status post partial colectomy
- Malnutrition

His past medical history is significant for a partial colectomy due to ischemic colitis one year ago.

PAST SURGICAL HISTORY:
- Partial colectomy for ischemic colitis (age 44)

MEDICATIONS:
Home Medications:
- Pancrelipase 36,000 units lipase with meals
- Omeprazole 20 mg twice daily
- Aspirin 325 mg as needed for headache
- Multivitamin One tablet daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Unknown family history (limited information due to immigration)
- No known family history of hereditary pancreatitis or MEN1

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Chronic alcohol use disorder with periods of sobriety and intermittent relapses. Occupation: Unemployed; formerly employed as software engineer.

PHYSICAL EXAMINATION:
On physical examination, his abdomen is non-distended, without tenderness or palpable masses. Bowel sounds are normal.

LABORATORY DATA:
Laboratory results are as follows:

- Stool osmotic gap: 40 mOsm/kg
- Stool sodium: 30 mEq/L
- Stool potassium: 20 mEq/L
- Stool pH: 7.5

## Encounter 4: source 2022-09-15, practice 2024-02-17, ed, Emergency Medicine

Chief complaint: 3-week history of fatigue, fever, night sweats, and productive cough with occasional hemoptysis

ENCOUNTER NOTE
Date: 2022-09-15
Provider: Dr. Aisha Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
3-week history of fatigue, fever, night sweats, and productive cough with occasional hemoptysis

HISTORY OF PRESENT ILLNESS:
The patient is a 46-year-old man with chronic pancreatitis, malnutrition, and a history of scurvy and bile acid malabsorption who presents to the emergency department with a 3-week history of fatigue, fever, night sweats, and a productive cough with occasional blood. He has unintentionally lost 10 kg (22 lb) over the past 2 months.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic pancreatitis (dietary management) (diagnosed 2020-01-15)
- Zollinger-Ellison syndrome (diagnosed 2020-05-15)
- Scurvy (diagnosed 2021-01-15)
- Bile acid malabsorption (diagnosed 2021-09-15)
- Chronic pancreatitis with exocrine insufficiency
- Alcohol use disorder
- Status post partial colectomy
- Malnutrition

PAST SURGICAL HISTORY:
- Partial colectomy for ischemic colitis (age 44)

MEDICATIONS:
He does not take any medications

Home Medications:
- Pancrelipase 36,000 units lipase with meals
- Omeprazole 20 mg twice daily
- Aspirin 325 mg as needed for headache
- Multivitamin One tablet daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Unknown family history (limited information due to immigration)
- No known family history of hereditary pancreatitis or MEN1

SOCIAL HISTORY:
He emigrated from Vietnam 2 years ago and lives in a crowded shelter.

VITAL SIGNS:
Temperature is 38.2°C (100.8°F), blood pressure 108/72 mm Hg, heart rate 96/min, respiratory rate 22/min, and oxygen saturation 91% on room air.

PHYSICAL EXAMINATION:
On examination, he is cachectic, with cervical lymphadenopathy and crackles over the left upper lung field.

IMAGING:
Chest x-ray is shown in the diagram below.

OTHER STUDIES:
A tuberculin skin test is negative.

## Encounter 5: source 2023-07-15, practice 2024-12-23, ed, Emergency Medicine

Chief complaint: 2-day history of vomiting and severe epigastric pain radiating to the back

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
2-day history of vomiting and severe epigastric pain radiating to the back

HISTORY OF PRESENT ILLNESS:
The patient with chronic pancreatitis and prior Zollinger-Ellison syndrome presents with pain. He reports it is sharp, constant, and radiates to his back. He denies any recent trauma.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic pancreatitis (dietary management) (diagnosed 2020-01-15)
- Zollinger-Ellison syndrome (diagnosed 2020-05-15)
- Scurvy (diagnosed 2021-01-15)
- Bile acid malabsorption (diagnosed 2021-09-15)
- Tuberculosis (diagnosed 2022-09-15)
- Chronic pancreatitis with exocrine insufficiency
- Alcohol use disorder
- Status post partial colectomy
- Malnutrition

PAST SURGICAL HISTORY:
- Partial colectomy for ischemic colitis (age 44)

MEDICATIONS:
He takes taking aspirin for a headache the day before symptoms began. or use of other medications.

Home Medications:
- Pancrelipase 36,000 units lipase with meals
- Omeprazole 20 mg twice daily
- Multivitamin One tablet daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Unknown family history (limited information due to immigration)
- No known family history of hereditary pancreatitis or MEN1

SOCIAL HISTORY:
He admits to consuming a significant amount of alcohol over the past week

VITAL SIGNS:
On physical examination, his vital signs are stable

PHYSICAL EXAMINATION:
He appears dehydrated and has epigastric tenderness on palpation.

LABORATORY DATA:
Laboratory tests reveal:

- Hemoglobin: 13.2 g/dL
- Hematocrit: 39%
- Platelets: 250,000/mm^3
- Serum amylase: 90 U/L (normal range: 30-110 U/L)
- Serum lipase: 45 U/L (normal range: 0-160 U/L)
- Liver function tests: within normal limits

OTHER STUDIES:
An upper endoscopy is performed, which shows multiple superficial hemorrhagic lesions in the gastric mucosa.

ASSESSMENT AND PLAN:
Based on the patient's history and endoscopic findings,

## Encounter 6: source 2024-09-15, practice 2026-03-05, icu, Critical Care Medicine

Chief complaint: Vomiting blood and dark stools with abdominal fullness and nausea

ENCOUNTER NOTE
Date: 2024-09-15
Provider: Dr. Lisa Thompson, MD
Department: Critical Care Medicine
Visit Type: icu

CHIEF COMPLAINT:
Vomiting blood and dark stools with abdominal fullness and nausea

HISTORY OF PRESENT ILLNESS:
The patient with chronic pancreatitis, prior upper GI bleeding, and malnutrition presents to the ICU. The patient reports that over the past 2 days he has had increasing episodes of nausea, a feeling of abdominal fullness, and dark stools. Since that time, he has noticed intermittent upper abdominal discomfort, early satiety, and occasional bloating after meals.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic pancreatitis (dietary management) (diagnosed 2020-01-15)
- Zollinger-Ellison syndrome (diagnosed 2020-05-15)
- Scurvy (diagnosed 2021-01-15)
- Bile acid malabsorption (diagnosed 2021-09-15)
- Tuberculosis (diagnosed 2022-09-15)
- NSAID-induced gastropathy or upper gastrointestinal bleeding (diagnosed 2023-07-15)
- Chronic pancreatitis with exocrine insufficiency
- Alcohol use disorder
- Status post partial colectomy
- Malnutrition

Six months ago, he was hospitalized for 3 months due to severe alcohol-induced pancreatitis.

PAST SURGICAL HISTORY:
His recovery was complicated by the development of a large pancreatic pseudocyst in the tail of the pancreas that required surgical drainage.

MEDICATIONS:
Home Medications:
- Pancrelipase 36,000 units lipase with meals
- Omeprazole 20 mg twice daily
- Aspirin 325 mg as needed for headache
- Multivitamin One tablet daily

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Unknown family history (limited information due to immigration)
- No known family history of hereditary pancreatitis or MEN1

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Chronic alcohol use disorder with periods of sobriety and intermittent relapses. Occupation: Unemployed; formerly employed as software engineer.

REVIEW OF SYSTEMS:
He has not had fevers, chills, or unintentional weight loss.

VITAL SIGNS:
His vital signs are stable.

PHYSICAL EXAMINATION:
On examination, the patient appears pale but is in no acute distress. Abdominal examination reveals mild tenderness to palpation in the left upper quadrant and a firm mass palpable below the left costal margin. There is no fluid wave or shifting dullness.

LABORATORY DATA:
Laboratory studies show hemoglobin concentration of 9.4 g/dL.

OTHER STUDIES:
Upper endoscopy reveals isolated gastric varices without evidence of esophageal varices.
