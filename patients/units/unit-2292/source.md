# synthetic_hospital patient 2292

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 64 at first encounter, sex M, Non-Hispanic White, Private insurance through union retiree benefits.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-04-01 (Fri) | outpatient | Pulmonology |
| 1 | 2021-07-15 | 2023-10-05 (Thu) | outpatient | Pulmonology |
| 2 | 2022-07-15 | 2024-10-10 (Thu) | outpatient | Pulmonology |
| 3 | 2023-07-15 | 2025-09-22 (Mon) | ed | Emergency Medicine |
| 4 | 2024-01-15 | 2026-03-28 (Sat) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Private insurance through union retiree benefits",
 "occupation": "Retired construction worker; previously employed in commercial building renovation for 20 years",
 "alcohol_use": "Occasional social use, 1-2 drinks per week",
 "family_history": [
  "Father: Lung cancer, deceased age 72",
  "Mother: Emphysema, age 89",
  "Brother: Hypertension"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 1 pack per day (45 pack-year history)",
 "home_medications": [
  {
   "dose": "250/50 mcg, inhaled, twice daily",
   "name": "Fluticasone propionate/salmeterol (Advair Diskus)"
  },
  {
   "dose": "90 mcg, inhaled, every 4-6 hours as needed for wheezing or shortness of breath",
   "name": "Albuterol (ProAir HFA)"
  },
  {
   "dose": "18 mcg, inhaled, once daily",
   "name": "Tiotropium (Spiriva HandiHaler)"
  }
 ],
 "surgical_history": [
  "No prior surgeries",
  "Tonsillectomy at age 8"
 ],
 "chronic_conditions": [
  "Chronic obstructive pulmonary disease (COPD), severe",
  "Chronic bronchitis with recurrent exacerbations",
  "History of recurrent lower respiratory tract infections"
 ],
 "age_at_first_encounter": 64
}
```

Primary diagnoses: ["Chronic obstructive pulmonary disease", "Primary spontaneous pneumothorax, small", "Community-acquired pneumonia"]
Comorbidities: ["Chronic obstructive pulmonary disease", "Tobacco use disorder", "History of recent pneumonia"]

## Encounter 0: source 2020-01-15, practice 2022-04-01, outpatient, Pulmonology

Chief complaint: chronic cough, intermittent wheezing, and progressive exertional dyspnea

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Chronic cough, intermittent wheezing, and progressive exertional dyspnea

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (COPD), severe
- Chronic bronchitis with recurrent exacerbations
- History of recurrent lower respiratory tract infections

PAST SURGICAL HISTORY:
- No prior surgeries
- Tonsillectomy at age 8

MEDICATIONS:
He takes inhaled corticosteroid and a long-acting β2-agonist.

Home Medications:
- Fluticasone propionate/salmeterol (Advair Diskus) 250/50 mcg, inhaled, twice daily
- Albuterol (ProAir HFA) 90 mcg, inhaled, every 4-6 hours as needed for wheezing or shortness of breath
- Tiotropium (Spiriva HandiHaler) 18 mcg, inhaled, once daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer, deceased age 72
- Mother: Emphysema, age 89
- Brother: Hypertension

SOCIAL HISTORY:
45-pack-year smoking history

PATHOLOGY:
Histologic samples obtained as part of a research study show goblet cell hyperplasia, mucous gland enlargement, and destruction of alveolar septa

OTHER STUDIES:
Pulmonary function testing reveals an FEV1/FVC ratio of 0.56 and an FEV1 that is 45% of predicted, with minimal improvement following bronchodilator administration

## Encounter 1: source 2021-07-15, practice 2023-10-05, outpatient, Pulmonology

Chief complaint: worsening shortness of breath over the past 6 months

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Sarah Chen, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Worsening shortness of breath over the past 6 months

HISTORY OF PRESENT ILLNESS:
Patient with severe COPD returns for follow-up. He reports that he now becomes dyspneic with minimal activities such as dressing or walking across the room.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (diagnosed 2020-01-15)
- Chronic obstructive pulmonary disease (COPD), severe
- Chronic bronchitis with recurrent exacerbations
- History of recurrent lower respiratory tract infections

He has a history of two hospitalizations in the past year for acute exacerbations of his respiratory condition.

PAST SURGICAL HISTORY:
- No prior surgeries
- Tonsillectomy at age 8

MEDICATIONS:
His medications include albuterol as needed.

Home Medications:
- Fluticasone propionate/salmeterol (Advair Diskus) 250/50 mcg, inhaled, twice daily
- Albuterol (ProAir HFA) 90 mcg, inhaled, every 4-6 hours as needed for wheezing or shortness of breath
- Tiotropium (Spiriva HandiHaler) 18 mcg, inhaled, once daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer, deceased age 72
- Mother: Emphysema, age 89
- Brother: Hypertension

SOCIAL HISTORY:
40-pack-year smoking history

VITAL SIGNS:
His oxygen saturation is 92% on room air.

PHYSICAL EXAMINATION:
On physical examination, you note a prolonged expiratory phase with wheezing and decreased breath sounds bilaterally.

OTHER STUDIES:
Spirometry shows an FEV1/FVC ratio of 0.60 after bronchodilator administration.

## Encounter 2: source 2022-07-15, practice 2024-10-10, outpatient, Pulmonology

Chief complaint: persistent cough and mild weight loss

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Michael Rodriguez, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Persistent cough and mild weight loss

HISTORY OF PRESENT ILLNESS:
The patient with severe COPD and recurrent exacerbations returns to the pulmonology clinic for evaluation of a persistent cough and mild weight loss over the past 3 months.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (diagnosed 2020-01-15)
- Chronic obstructive pulmonary disease (COPD), severe
- Chronic bronchitis with recurrent exacerbations
- History of recurrent lower respiratory tract infections

He has a 30-year history of chronic cough productive of sputum and has been hospitalized twice in the past for lung infections.

PAST SURGICAL HISTORY:
- No prior surgeries
- Tonsillectomy at age 8

MEDICATIONS:
Home Medications:
- Fluticasone propionate/salmeterol (Advair Diskus) 250/50 mcg, inhaled, twice daily
- Albuterol (ProAir HFA) 90 mcg, inhaled, every 4-6 hours as needed for wheezing or shortness of breath
- Tiotropium (Spiriva HandiHaler) 18 mcg, inhaled, once daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer, deceased age 72
- Mother: Emphysema, age 89
- Brother: Hypertension

SOCIAL HISTORY:
He smokes one pack of cigarettes daily and worked in construction for 20 years.

REVIEW OF SYSTEMS:
He reports occasional night sweats but denies hemoptysis or chest pain.

PHYSICAL EXAMINATION:
Physical examination shows mild digital clubbing and scattered rhonchi on auscultation.

LABORATORY DATA:
HIV testing is negative, and complete blood count is within normal limits.

IMAGING:
Chest radiograph reveals a 1.5-cm nodular opacity located peripherally in the upper lobe of the right lung.

PATHOLOGY:
A needle biopsy is performed, and histologic examination shows granulomas, some of which exhibit central areas of caseous necrosis. Special staining of the necrotic material reveals yeast forms within the lesion.

PLAN:
The patient is started on antifungal therapy and counseled on smoking cessation.

## Encounter 3: source 2023-07-15, practice 2025-09-22, ed, Emergency Medicine

Chief complaint: three days of worsening shortness of breath, productive cough, and low-grade fever

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Aisha Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Three days of worsening shortness of breath, productive cough, and low-grade fever

HISTORY OF PRESENT ILLNESS:
The patient with severe COPD and recurrent lower respiratory tract infections, followed in Pulmonology since 2020, presents with acute worsening. He reports that his sputum has become yellow and occasionally blood-streaked. He has had progressive fatigue and dyspnea on exertion for the past week and now feels winded even while sitting upright.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (diagnosed 2020-01-15)
- Chronic obstructive pulmonary disease (COPD), severe
- Chronic bronchitis with recurrent exacerbations
- History of recurrent lower respiratory tract infections

PAST SURGICAL HISTORY:
- No prior surgeries
- Tonsillectomy at age 8

MEDICATIONS:
Home Medications:
- Fluticasone propionate/salmeterol (Advair Diskus) 250/50 mcg, inhaled, twice daily
- Albuterol (ProAir HFA) 90 mcg, inhaled, every 4-6 hours as needed for wheezing or shortness of breath
- Tiotropium (Spiriva HandiHaler) 18 mcg, inhaled, once daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer, deceased age 72
- Mother: Emphysema, age 89
- Brother: Hypertension

SOCIAL HISTORY:
He has a 50-pack-year smoking history

REVIEW OF SYSTEMS:
He denies chest pain, leg swelling, or recent travel.

VITAL SIGNS:
His temperature is 38.2°C (100.8°F), blood pressure is 124/76 mmHg, pulse is 102/min, and respiratory rate is 26/min. Oxygen saturation is 87% on room air and improves to 93% with 4 L/min oxygen via nasal cannula.

PHYSICAL EXAMINATION:
On exam, he is in mild respiratory distress and using accessory muscles. Breath sounds are decreased at the right base, with crackles and dullness to percussion. Egophony is present over the same area.

LABORATORY DATA:
Arterial blood gas reveals mild hypoxemia and an elevated alveolar-arterial (A–a) oxygen gradient.

IMAGING:
Chest x-ray reveals consolidation of the right lower lobe.

## Encounter 4: source 2024-01-15, practice 2026-03-28, ed, Emergency Medicine

Chief complaint: sudden onset of shortness of breath and left-sided chest pain

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. James Park, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden onset of shortness of breath and left-sided chest pain

HISTORY OF PRESENT ILLNESS:
The patient with severe COPD returns with sudden dyspnea and sharp left chest pain worsening with deep breaths, denying recent trauma or injury.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic obstructive pulmonary disease (diagnosed 2020-01-15)
- Community-acquired pneumonia (diagnosed 2023-07-15)
- Chronic obstructive pulmonary disease (COPD), severe
- Chronic bronchitis with recurrent exacerbations
- History of recurrent lower respiratory tract infections

PAST SURGICAL HISTORY:
- No prior surgeries
- Tonsillectomy at age 8

MEDICATIONS:
He takes uses inhaled bronchodilators regularly.

Home Medications:
- Fluticasone propionate/salmeterol (Advair Diskus) 250/50 mcg, inhaled, twice daily
- Albuterol (ProAir HFA) 90 mcg, inhaled, every 4-6 hours as needed for wheezing or shortness of breath
- Tiotropium (Spiriva HandiHaler) 18 mcg, inhaled, once daily

ALLERGIES:
NKDA

FAMILY HISTORY:
- Father: Lung cancer, deceased age 72
- Mother: Emphysema, age 89
- Brother: Hypertension

SOCIAL HISTORY:
He has a 40-pack-year smoking history

VITAL SIGNS:
On examination, his respiratory rate is 28 breaths per minute, heart rate is 110 beats per minute, blood pressure is 135/85 mmHg, and oxygen saturation is 89% on room air

PHYSICAL EXAMINATION:
Chest auscultation reveals decreased breath sounds on the left side without wheezing. Percussion of the left chest is hyperresonant. The remainder of the examination is unremarkable.

IMAGING:
A chest X-ray is performed and shows a left-sided pneumothorax with 30% lung collapse.
