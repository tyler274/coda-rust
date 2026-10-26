# synthetic_hospital patient 2095

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 45 at first encounter, sex M, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-08-16 (Mon) | outpatient | Neurology |
| 1 | 2020-05-15 | 2021-12-14 (Tue) | outpatient | Gastroenterology |
| 2 | 2021-03-15 | 2022-10-10 (Mon) | outpatient | Internal Medicine |
| 3 | 2022-07-15 | 2024-02-19 (Mon) | outpatient | Dermatology |
| 4 | 2024-07-15 | 2026-02-02 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Sulfamethoxazole-trimethoprim (urticarial vasculitis and angioedema)",
  "Penicillin (rash)"
 ],
 "insurance": "Medicaid",
 "occupation": "Construction worker (currently on medical disability)",
 "alcohol_use": "Occasional social use (1-2 drinks monthly)",
 "family_history": [
  "Brother with progressive limb-girdle muscle weakness (genetic myopathy)",
  "Father with chronic obstructive pulmonary disease (COPD)",
  "Paternal uncle with lung cancer (diagnosed age 62)"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Current smoker, 35 pack-year history (1.5 packs/day since age 20)",
 "home_medications": [
  {
   "dose": "50mg/25mg/200mg PO daily",
   "name": "Bictegravir/tenofovir alafenamide/emtricitabine"
  },
  {
   "dose": "30mg PO daily",
   "name": "Nifedipine extended-release"
  },
  {
   "dose": "500mg PO every 6 hours PRN pain",
   "name": "Acetaminophen"
  },
  {
   "dose": "21mg/24hr topical daily",
   "name": "Nicotine transdermal patch"
  }
 ],
 "surgical_history": [
  "Diagnostic muscle biopsy (quadriceps)",
  "Esophageal manometry and upper endoscopy",
  "Diagnostic bronchoscopy with bronchoalveolar lavage"
 ],
 "chronic_conditions": [
  "HIV infection (AIDS)",
  "Achalasia",
  "Hereditary myopathy with chaperone protein abnormality",
  "Tobacco use disorder",
  "History of intravenous drug use (in remission)",
  "History of recurrent bacterial pneumonia"
 ],
 "age_at_first_encounter": 45
}
```

Primary diagnoses: ["Lung cancer screening candidate (high-risk smoker)", "Achalasia", "Hereditary myopathy with chaperone protein abnormality", "Urticarial vasculitis", "Kaposi sarcoma"]
Comorbidities: ["History of recent pneumonia", "Family history of breast cancer"]

## Encounter 0: source 2020-01-15, practice 2021-08-16, outpatient, Neurology

Chief complaint: progressive muscle weakness and difficulty swallowing

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Neurology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive muscle weakness and difficulty swallowing

PAST MEDICAL HISTORY:
Active Problem List:
- HIV infection (AIDS)
- Achalasia
- Hereditary myopathy with chaperone protein abnormality
- Tobacco use disorder
- History of intravenous drug use (in remission)
- History of recurrent bacterial pneumonia

PAST SURGICAL HISTORY:
- Diagnostic muscle biopsy (quadriceps)
- Esophageal manometry and upper endoscopy
- Diagnostic bronchoscopy with bronchoalveolar lavage

MEDICATIONS:
Home Medications:
- Bictegravir/tenofovir alafenamide/emtricitabine 50mg/25mg/200mg PO daily
- Nifedipine extended-release 30mg PO daily
- Acetaminophen 500mg PO every 6 hours PRN pain
- Nicotine transdermal patch 21mg/24hr topical daily

ALLERGIES:
Sulfamethoxazole-trimethoprim (urticarial vasculitis and angioedema), Penicillin (rash)

FAMILY HISTORY:
Family history reveals several relatives with similar symptoms leading to respiratory failure in midlife

SOCIAL HISTORY:
Smoking: Current smoker, 35 pack-year history (1.5 packs/day since age 20). Alcohol: Occasional social use (1-2 drinks monthly). Occupation: Construction worker (currently on medical disability).

PATHOLOGY:
Muscle biopsy demonstrates abnormal chaperone proteins

## Encounter 1: source 2020-05-15, practice 2021-12-14, outpatient, Gastroenterology

Chief complaint: progressive difficulty swallowing both solids and liquids for several months

ENCOUNTER NOTE
Date: 2020-05-15
Provider: Dr. Michael Rodriguez, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive difficulty swallowing both solids and liquids for several months

HISTORY OF PRESENT ILLNESS:
He returns for evaluation of progressive dysphagia with achalasia, hereditary myopathy, and prior Neurology assessment. He describes regurgitation of undigested food, occasional foul-smelling breath, and mild, non-specific chest discomfort. He denies weight loss, odynophagia, or heartburn.

PAST MEDICAL HISTORY:
Active Problem List:
- Hereditary myopathy with chaperone protein abnormality (diagnosed 2020-01-15)
- HIV infection (AIDS)
- Achalasia
- Tobacco use disorder
- History of intravenous drug use (in remission)
- History of recurrent bacterial pneumonia

PAST SURGICAL HISTORY:
- Diagnostic muscle biopsy (quadriceps)
- Esophageal manometry and upper endoscopy
- Diagnostic bronchoscopy with bronchoalveolar lavage

MEDICATIONS:
Home Medications:
- Bictegravir/tenofovir alafenamide/emtricitabine 50mg/25mg/200mg PO daily
- Nifedipine extended-release 30mg PO daily
- Acetaminophen 500mg PO every 6 hours PRN pain
- Nicotine transdermal patch 21mg/24hr topical daily

ALLERGIES:
Sulfamethoxazole-trimethoprim (urticarial vasculitis and angioedema), Penicillin (rash)

FAMILY HISTORY:
He has a family history of esophageal cancer in his father.

SOCIAL HISTORY:
Smoking: Current smoker, 35 pack-year history (1.5 packs/day since age 20). Alcohol: Occasional social use (1-2 drinks monthly). Occupation: Construction worker (currently on medical disability).

VITAL SIGNS:
His temperature is 36.9°C (98.4°F), blood pressure is 126/78 mmHg, heart rate is 74/min, and respiratory rate is 14/min.

PHYSICAL EXAMINATION:
Physical examination is unremarkable.

IMAGING:
A barium swallow study is shown below.

## Encounter 2: source 2021-03-15, practice 2022-10-10, outpatient, Internal Medicine

Chief complaint: presents to the clinic for a routine check-up

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. Jennifer Okafor, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
He presents to the clinic for a routine check-up

HISTORY OF PRESENT ILLNESS:
The patient returns for follow-up of HIV (AIDS), hereditary myopathy, and achalasia, seen in Neurology and Gastroenterology. He has no complaints and is currently asymptomatic. He is concerned about his risk of developing lung cancer and inquires about any available screening methods.

PAST MEDICAL HISTORY:
Active Problem List:
- Hereditary myopathy with chaperone protein abnormality (diagnosed 2020-01-15)
- Achalasia (diagnosed 2020-05-15)
- HIV infection (AIDS)
- Tobacco use disorder
- History of intravenous drug use (in remission)
- History of recurrent bacterial pneumonia

PAST SURGICAL HISTORY:
- Diagnostic muscle biopsy (quadriceps)
- Esophageal manometry and upper endoscopy
- Diagnostic bronchoscopy with bronchoalveolar lavage

MEDICATIONS:
Home Medications:
- Bictegravir/tenofovir alafenamide/emtricitabine 50mg/25mg/200mg PO daily
- Nifedipine extended-release 30mg PO daily
- Acetaminophen 500mg PO every 6 hours PRN pain
- Nicotine transdermal patch 21mg/24hr topical daily

ALLERGIES:
Sulfamethoxazole-trimethoprim (urticarial vasculitis and angioedema), Penicillin (rash)

FAMILY HISTORY:
His family history is significant for lung cancer in his father.

SOCIAL HISTORY:
With a history of smoking

VITAL SIGNS:
On physical examination, his vital signs are within normal limits, and

PHYSICAL EXAMINATION:
Lung auscultation reveals no abnormalities.

LABORATORY DATA:
- Not applicable for this scenario.

OTHER STUDIES:
The physician decides to conduct a study to determine if low-dose computed tomography (LDCT) screening reduces the incidence of lung cancer deaths among high-risk individuals. The study includes 1000 smokers aged 55 to 74 with a smoking history of at least 30 pack-years. Participants are randomized into two groups: one group receives annual LDCT screening for three years, and the other group receives a chest x-ray annually for the same period. After five years, the incidence of lung cancer deaths is compared between the two groups.

## Encounter 3: source 2022-07-15, practice 2024-02-19, outpatient, Dermatology

Chief complaint: multiple violaceous lesions on his hard palate and lower extremities

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. David Kim, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Multiple violaceous lesions on his hard palate and lower extremities

HISTORY OF PRESENT ILLNESS:
The patient with HIV and achalasia returns for violaceous lesions, reporting a chronic, nonproductive cough and several weeks of weight loss and night sweats.

PAST MEDICAL HISTORY:
Active Problem List:
- Hereditary myopathy with chaperone protein abnormality (diagnosed 2020-01-15)
- Achalasia (diagnosed 2020-05-15)
- Lung cancer screening candidate (high-risk smoker) (diagnosed 2021-03-15)
- HIV infection (AIDS)
- Tobacco use disorder
- History of intravenous drug use (in remission)
- History of recurrent bacterial pneumonia

PAST SURGICAL HISTORY:
- Diagnostic muscle biopsy (quadriceps)
- Esophageal manometry and upper endoscopy
- Diagnostic bronchoscopy with bronchoalveolar lavage

MEDICATIONS:
Home Medications:
- Bictegravir/tenofovir alafenamide/emtricitabine 50mg/25mg/200mg PO daily
- Nifedipine extended-release 30mg PO daily
- Acetaminophen 500mg PO every 6 hours PRN pain
- Nicotine transdermal patch 21mg/24hr topical daily

ALLERGIES:
Sulfamethoxazole-trimethoprim (urticarial vasculitis and angioedema), Penicillin (rash)

FAMILY HISTORY:
- Brother with progressive limb-girdle muscle weakness (genetic myopathy)
- Father with chronic obstructive pulmonary disease (COPD)
- Paternal uncle with lung cancer (diagnosed age 62)

SOCIAL HISTORY:
Smoking: Current smoker, 35 pack-year history (1.5 packs/day since age 20). Alcohol: Occasional social use (1-2 drinks monthly). Occupation: Construction worker (currently on medical disability).

PHYSICAL EXAMINATION:
On physical exam, there are multiple reddish-purple, non-blanching papules on the skin and oral mucosa.

IMAGING:
A chest CT reveals bilateral perihilar infiltrates and small pleural effusions.

PATHOLOGY:
A skin biopsy shows spindle-shaped cells, neovascularization, and extravasated red blood cells.

## Encounter 4: source 2024-07-15, practice 2026-02-02, ed, Emergency Medicine

Chief complaint: new rash on legs with joint stiffness following recent antibiotic course

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Amanda Washington, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
New rash on legs with joint stiffness following recent antibiotic course

HISTORY OF PRESENT ILLNESS:
The patient, with HIV infection and prior Kaposi sarcoma, presents with a new rash on his legs that has been gradually worsening over the past week. He was recently treated with antibiotics for a respiratory infection. He reports mild joint stiffness and fatigue but denies fever, weight loss, or abdominal pain.

PAST MEDICAL HISTORY:
Active Problem List:
- Hereditary myopathy with chaperone protein abnormality (diagnosed 2020-01-15)
- Achalasia (diagnosed 2020-05-15)
- Lung cancer screening candidate (high-risk smoker) (diagnosed 2021-03-15)
- Kaposi sarcoma (diagnosed 2022-07-15)
- HIV infection (AIDS)
- Tobacco use disorder
- History of intravenous drug use (in remission)
- History of recurrent bacterial pneumonia

PAST SURGICAL HISTORY:
- Diagnostic muscle biopsy (quadriceps)
- Esophageal manometry and upper endoscopy
- Diagnostic bronchoscopy with bronchoalveolar lavage

MEDICATIONS:
Home Medications:
- Bictegravir/tenofovir alafenamide/emtricitabine 50mg/25mg/200mg PO daily
- Nifedipine extended-release 30mg PO daily
- Acetaminophen 500mg PO every 6 hours PRN pain
- Nicotine transdermal patch 21mg/24hr topical daily

ALLERGIES:
Sulfamethoxazole-trimethoprim (urticarial vasculitis and angioedema), Penicillin (rash)

FAMILY HISTORY:
- Brother with progressive limb-girdle muscle weakness (genetic myopathy)
- Father with chronic obstructive pulmonary disease (COPD)
- Paternal uncle with lung cancer (diagnosed age 62)

SOCIAL HISTORY:
Smoking: Current smoker, 35 pack-year history (1.5 packs/day since age 20). Alcohol: Occasional social use (1-2 drinks monthly). Occupation: Construction worker (currently on medical disability).

PHYSICAL EXAMINATION:
On physical examination, there are multiple non-blanching, palpable purpuric lesions on both lower extremities. No ulcers are noted. Cardiac, pulmonary, and abdominal examinations are normal.

LABORATORY DATA:
Laboratory studies show normal platelet count and coagulation profile.

PATHOLOGY:
A skin biopsy reveals perivascular neutrophilic infiltration with nuclear debris and fibrinoid necrosis of small dermal vessels.
