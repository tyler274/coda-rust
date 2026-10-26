# synthetic_hospital patient 2546

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 28 at first encounter, sex M, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2020-06-16 (Tue) | outpatient | Otolaryngology |
| 1 | 2021-01-15 | 2021-06-08 (Tue) | outpatient | Neurology |
| 2 | 2023-01-15 | 2023-06-13 (Tue) | outpatient | Pulmonology |
| 3 | 2026-01-15 | 2026-06-09 (Tue) | inpatient | Psychiatry |
| 4 | 2026-02-15 | 2026-07-03 (Fri) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Benztropine (anticholinergic toxicity)",
  "Tree pollen",
  "Grass pollen"
 ],
 "insurance": "Medicaid",
 "occupation": "Freelance graphic designer (previously construction laborer)",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Mother with mitochondrial myopathy",
  "Father with alpha-1 antitrypsin deficiency and early-onset emphysema",
  "Maternal uncle with schizophrenia",
  "Paternal grandmother with liver cirrhosis of unknown etiology"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "2 mg PO daily",
   "name": "Risperidone"
  },
  {
   "dose": "1 mg PO BID",
   "name": "Benztropine"
  },
  {
   "dose": "10 mg PO daily PRN",
   "name": "Cetirizine"
  },
  {
   "dose": "2 sprays each nostril daily",
   "name": "Fluticasone propionate nasal spray"
  }
 ],
 "surgical_history": [
  "Tonsillectomy (age 12)"
 ],
 "chronic_conditions": [
  "Schizophrenia",
  "Alpha-1 antitrypsin deficiency",
  "Mitochondrial myopathy",
  "Allergic rhinitis (seasonal)",
  "Exercise intolerance"
 ],
 "age_at_first_encounter": 28
}
```

Primary diagnoses: ["Schizophrenia", "Adverse effect of anticholinergic medication", "Alpha-1 antitrypsin deficiency", "Mitochondrial myopathy", "Allergic rhinitis"]
Comorbidities: ["Family History of Hereditary Hemorrhagic Telangiectasia"]

## Encounter 0: source 2020-01-15, practice 2020-06-16, outpatient, Otolaryngology

Chief complaint: Evaluation and management of long-standing seasonal allergies with recurrent sneezing, nasal congestion, and itchy eyes during spring and fall.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Otolaryngology
Visit Type: outpatient

CHIEF COMPLAINT:
Evaluation and management of long-standing seasonal allergies with recurrent sneezing, nasal congestion, and itchy eyes during spring and fall.

HISTORY OF PRESENT ILLNESS:
Since childhood, he has experienced recurrent episodes of sneezing, nasal congestion, itchy eyes, and postnasal drip, particularly during the spring and early fall. He also notes occasional wheezing and shortness of breath triggered by high pollen counts. Over-the-counter antihistamines and intranasal corticosteroids provide partial relief, but his symptoms continue to affect his work productivity and quality of life.

PAST MEDICAL HISTORY:
Active Problem List:
- Schizophrenia
- Alpha-1 antitrypsin deficiency
- Mitochondrial myopathy
- Allergic rhinitis (seasonal)
- Exercise intolerance

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)

MEDICATIONS:
Home Medications:
- Risperidone 2 mg PO daily
- Benztropine 1 mg PO BID
- Cetirizine 10 mg PO daily PRN
- Fluticasone propionate nasal spray 2 sprays each nostril daily

ALLERGIES:
Benztropine (anticholinergic toxicity), Tree pollen, Grass pollen

FAMILY HISTORY:
- Mother with mitochondrial myopathy
- Father with alpha-1 antitrypsin deficiency and early-onset emphysema
- Maternal uncle with schizophrenia
- Paternal grandmother with liver cirrhosis of unknown etiology

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Freelance graphic designer (previously construction laborer).

OTHER STUDIES:
Skin testing confirms sensitivity to multiple aeroallergens, including grass, tree, and weed pollens.

PLAN:
After discussing treatment options, the patient elects to begin a long-term course of allergen immunotherapy involving regular subcutaneous injections of gradually increasing doses of the relevant allergens.

## Encounter 1: source 2021-01-15, practice 2021-06-08, outpatient, Neurology

Chief complaint: Progressive difficulty keeping up with coworkers during physical labor over the last several years with limb weakness and occasional double vision.

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. Michael Okafor, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive difficulty keeping up with coworkers during physical labor over the last several years with limb weakness and occasional double vision.

HISTORY OF PRESENT ILLNESS:
He returns for evaluation of progressive neuromuscular symptoms one year following otolaryngology care for allergic rhinitis, in the setting of established mitochondrial myopathy, schizophrenia, and alpha-1 antitrypsin deficiency. He states that as a child, he was able to participate in most sports but would tire sooner than his peers. Over the past year, he has noticed more frequent episodes of limb weakness, especially after climbing several flights of stairs, along with occasional double vision and difficulty focusing his eyes for prolonged periods. He has no history of head trauma, seizures, or syncope.

PAST MEDICAL HISTORY:
Active Problem List:
- Allergic rhinitis (diagnosed 2020-01-15)
- Schizophrenia
- Alpha-1 antitrypsin deficiency
- Mitochondrial myopathy
- Allergic rhinitis (seasonal)
- Exercise intolerance

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)

MEDICATIONS:
Home Medications:
- Risperidone 2 mg PO daily
- Benztropine 1 mg PO BID
- Cetirizine 10 mg PO daily PRN
- Fluticasone propionate nasal spray 2 sprays each nostril daily

ALLERGIES:
Benztropine (anticholinergic toxicity), Tree pollen, Grass pollen

FAMILY HISTORY:
Family history is notable for a mother who developed similar symptoms in her 30s and maternal relatives with early-onset hearing loss.

SOCIAL HISTORY:
The patient works in construction and has no toxic exposures.

PHYSICAL EXAMINATION:
On examination, he is thin, with bilateral ptosis and mild weakness of the proximal limb muscles. Extraocular movements are restricted in multiple directions. Cardiopulmonary examination is normal.

LABORATORY DATA:
Laboratory studies reveal elevated creatine kinase, lactate levels persistently high at rest and after minimal exertion, and normal thyroid function.

PATHOLOGY:
A muscle biopsy with modified Gomori trichrome stain reveals irregularly shaped muscle fibers with subsarcolemmal collections of abnormal mitochondria.

OTHER STUDIES:
Electromyography shows a myopathic pattern.

## Encounter 2: source 2023-01-15, practice 2023-06-13, outpatient, Pulmonology

Chief complaint: Progressive shortness of breath over the past 18 months with dyspnea on minimal exertion despite never smoking.

ENCOUNTER NOTE
Date: 2023-01-15
Provider: Dr. Priya Patel, MD
Department: Pulmonology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive shortness of breath over the past 18 months with dyspnea on minimal exertion despite never smoking.

HISTORY OF PRESENT ILLNESS:
The patient returns with progressive dyspnea in the context of known Alpha-1 antitrypsin deficiency and prior evaluation for mitochondrial myopathy. He reports previously jogging regularly but now becomes winded even after walking two blocks. He endorses occasional mild chest tightness but denies chest pain, wheezing, hemoptysis, or cough.

PAST MEDICAL HISTORY:
Active Problem List:
- Allergic rhinitis (diagnosed 2020-01-15)
- Mitochondrial myopathy (diagnosed 2021-01-15)
- Schizophrenia
- Alpha-1 antitrypsin deficiency
- Allergic rhinitis (seasonal)
- Exercise intolerance

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)

MEDICATIONS:
Home Medications:
- Risperidone 2 mg PO daily
- Benztropine 1 mg PO BID
- Cetirizine 10 mg PO daily PRN
- Fluticasone propionate nasal spray 2 sprays each nostril daily

ALLERGIES:
Benztropine (anticholinergic toxicity), Tree pollen, Grass pollen

FAMILY HISTORY:
His father died in his mid-40s from cirrhosis, which was attributed to alcohol use, though the patient remembers his father also had chronic breathing issues.

SOCIAL HISTORY:
He has never smoked. He works as a freelance graphic designer from home and has no pets. He drinks socially and does not use recreational drugs.

VITAL SIGNS:
Vital signs are within normal limits.
Oxygen saturation is 94% on room air.

PHYSICAL EXAMINATION:
On examination, the patient appears thin and mildly tachypneic. His lungs are clear to auscultation with mildly prolonged expiration and hyperresonance on percussion. Cardiovascular exam is normal, and there is no clubbing or cyanosis.

LABORATORY DATA:
Blood work is notable for mild elevations in liver transaminases.

IMAGING:
Chest x-ray shows increased lung lucency in the lower lobes bilaterally and flattened diaphragms.

OTHER STUDIES:
Pulmonary function testing reveals increased total lung capacity and residual volume, with a decreased FEV₁/FVC ratio.

## Encounter 3: source 2026-01-15, practice 2026-06-09, inpatient, Psychiatry

Chief complaint: Auditory hallucinations, disorganized speech, and social withdrawal for the past 8 months requiring stabilization.

ENCOUNTER NOTE
Date: 2026-01-15
Provider: Dr. James Wilson, MD
Department: Psychiatry
Visit Type: inpatient

CHIEF COMPLAINT:
Auditory hallucinations, disorganized speech, and social withdrawal for the past 8 months requiring stabilization.

PAST MEDICAL HISTORY:
Active Problem List:
- Allergic rhinitis (diagnosed 2020-01-15)
- Mitochondrial myopathy (diagnosed 2021-01-15)
- Alpha-1 antitrypsin deficiency (diagnosed 2023-01-15)
- Schizophrenia
- Allergic rhinitis (seasonal)
- Exercise intolerance

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)

MEDICATIONS:
Home Medications:
- Risperidone 2 mg PO daily
- Benztropine 1 mg PO BID
- Cetirizine 10 mg PO daily PRN
- Fluticasone propionate nasal spray 2 sprays each nostril daily

ALLERGIES:
Benztropine (anticholinergic toxicity), Tree pollen, Grass pollen

FAMILY HISTORY:
His family history is significant for a twin brother with whom he shares all genetic material. The brother is currently asymptomatic and asks about his personal risk of developing a similar psychiatric illness.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Freelance graphic designer (previously construction laborer).

ASSESSMENT AND PLAN:
He is diagnosed with schizophrenia after meeting DSM-5 criteria
Schizophrenia is a complex disorder with both genetic and environmental contributions. Concordance rates for relatives vary widely based on genetic proximity. Compare this to the risk in the general population, which is estimated at about 1%. Dizygotic twins have a lower risk than monozygotic twins despite shared environment.

## Encounter 4: source 2026-02-15, practice 2026-07-03, ed, Emergency Medicine

Chief complaint: Acute confusion, restlessness, and decreased oral intake for the past 2 days with blurry vision one week after psychiatric discharge.

ENCOUNTER NOTE
Date: 2026-02-15
Provider: Dr. Maria Gonzalez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute confusion, restlessness, and decreased oral intake for the past 2 days with blurry vision one week after psychiatric discharge.

HISTORY OF PRESENT ILLNESS:
The patient is a 34-year-old male with schizophrenia, alpha-1 antitrypsin deficiency, and mitochondrial myopathy. He presents to the emergency department one week following inpatient psychiatric discharge. He was recently started on risperidone and benztropine. His roommate reports that the patient has been "acting strange," talking to himself, and refusing to eat or drink. He also complains of blurry vision and feeling excessively hot.

PAST MEDICAL HISTORY:
Active Problem List:
- Allergic rhinitis (diagnosed 2020-01-15)
- Mitochondrial myopathy (diagnosed 2021-01-15)
- Alpha-1 antitrypsin deficiency (diagnosed 2023-01-15)
- Schizophrenia (diagnosed 2026-01-15)
- Allergic rhinitis (seasonal)
- Exercise intolerance

PAST SURGICAL HISTORY:
- Tonsillectomy (age 12)

MEDICATIONS:
Home Medications:
- Risperidone 2 mg PO daily
- Benztropine 1 mg PO BID
- Cetirizine 10 mg PO daily PRN
- Fluticasone propionate nasal spray 2 sprays each nostril daily

ALLERGIES:
Benztropine (anticholinergic toxicity), Tree pollen, Grass pollen

FAMILY HISTORY:
- Mother with mitochondrial myopathy
- Father with alpha-1 antitrypsin deficiency and early-onset emphysema
- Maternal uncle with schizophrenia
- Paternal grandmother with liver cirrhosis of unknown etiology

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Freelance graphic designer (previously construction laborer).

VITAL SIGNS:
His temperature is 38.1°C (100.6°F), heart rate is 104/min, blood pressure is 152/88 mm Hg, and respiratory rate is 18/min.

PHYSICAL EXAMINATION:
On exam, the patient is flushed, with dry skin and dry oral mucosa. Pupils are 5 mm and sluggishly reactive to light. Neurologic exam reveals no rigidity. The patient is inattentive and disoriented to time and place.
