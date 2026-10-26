# synthetic_hospital patient 2604

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 24 at first encounter, sex M, Non-Hispanic White, University student health insurance (PPO).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2025-04-28 (Mon) | outpatient | Dermatology |
| 1 | 2020-07-15 | 2025-10-27 (Mon) | outpatient | Gastroenterology |
| 2 | 2021-01-15 | 2026-04-22 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Seasonal pollens (tree, grass, ragweed)",
  "Dust mites"
 ],
 "insurance": "University student health insurance (PPO)",
 "occupation": "Graduate student",
 "alcohol_use": "None",
 "family_history": [
  "Mother: Seasonal allergies, eczema",
  "Father: Asthma",
  "Sister: Atopic dermatitis"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never",
 "home_medications": [
  {
   "dose": "90 mcg (2 puffs) inhaled PRN for wheezing/shortness of breath",
   "name": "Albuterol inhaler"
  },
  {
   "dose": "10 mg PO daily",
   "name": "Cetirizine"
  },
  {
   "dose": "Apply thin layer to affected skin areas BID PRN for eczema flares",
   "name": "Triamcinolone acetonide cream 0.1%"
  }
 ],
 "surgical_history": [
  "None"
 ],
 "chronic_conditions": [
  "Allergic rhinitis (seasonal)",
  "Mild persistent asthma",
  "Atopic dermatitis"
 ],
 "age_at_first_encounter": 24
}
```

Primary diagnoses: ["Eosinophilic esophagitis", "Atopic dermatitis"]
Comorbidities: ["Allergic rhinitis"]

## Encounter 0: source 2020-01-15, practice 2025-04-28, outpatient, Dermatology

Chief complaint: Chronic itchy skin rash worse during winter and after hot showers

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Chronic itchy skin rash worse during winter and after hot showers

HISTORY OF PRESENT ILLNESS:
He reports that the symptoms are worse during the winter and after hot showers. He has been using over-the-counter creams with partial relief but notes that the rash tends to recur in the same locations.

PAST MEDICAL HISTORY:
Active Problem List:
- Allergic rhinitis (seasonal)
- Mild persistent asthma
- Atopic dermatitis

His past medical history is significant for seasonal allergies during the spring.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Albuterol inhaler 90 mcg (2 puffs) inhaled PRN for wheezing/shortness of breath
- Cetirizine 10 mg PO daily
- Triamcinolone acetonide cream 0.1% Apply thin layer to affected skin areas BID PRN for eczema flares

ALLERGIES:
Seasonal pollens (tree, grass, ragweed), Dust mites

FAMILY HISTORY:
Family history reveals that his mother has a history of asthma.

SOCIAL HISTORY:
He denies recent travel, new medications, or contact with sick individuals.

PHYSICAL EXAMINATION:
On examination, there are multiple erythematous patches with excoriations over the flexural surfaces of the elbows. An image of the rash is provided. No pustules, vesicles, or drainage are seen. The remainder of the skin and systemic examination is normal.

LABORATORY DATA:
Laboratory studies, including complete blood count and metabolic panel, are unremarkable.

## Encounter 1: source 2020-07-15, practice 2025-10-27, outpatient, Gastroenterology

Chief complaint: Intermittent difficulty swallowing solid foods and sensation of food sticking in chest

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Priya Patel, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent difficulty swallowing solid foods and sensation of food sticking in chest

HISTORY OF PRESENT ILLNESS:
The patient is a 24-year-old male with allergic rhinitis, mild persistent asthma, and atopic dermatitis (last evaluated in Dermatology on 2020-01-15) who presents to the clinic with a 6-month history of intermittent difficulty swallowing solid foods and a sensation of food getting stuck in his chest. He has tried over-the-counter antacids and a proton pump inhibitor for what he thought was heartburn, but his symptoms have not improved.

PAST MEDICAL HISTORY:
Active Problem List:
- Atopic dermatitis (diagnosed 2020-01-15)
- Allergic rhinitis (seasonal)
- Mild persistent asthma

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
He takes uses an albuterol inhaler as needed.

Home Medications:
- Cetirizine 10 mg PO daily
- Triamcinolone acetonide cream 0.1% Apply thin layer to affected skin areas BID PRN for eczema flares

ALLERGIES:
Seasonal pollens (tree, grass, ragweed), Dust mites

FAMILY HISTORY:
His mother has a history of atopic dermatitis.

SOCIAL HISTORY:
Smoking: Never. Alcohol: None. Occupation: Graduate student.

REVIEW OF SYSTEMS:
He denies any weight loss, vomiting, or changes in bowel habits.

PHYSICAL EXAMINATION:
His physical examination is unremarkable.

PATHOLOGY:
The pathology report shows:

- Squamous epithelium with >15 eosinophils per high power field
- No evidence of viral inclusions or fungal organisms
- Normal gastric and duodenal biopsies

OTHER STUDIES:
An upper endoscopy is performed, and biopsies are taken.

## Encounter 2: source 2021-01-15, practice 2026-04-22, ed, Emergency Medicine

Chief complaint: Acute vomiting after meals with progressive dysphagia to solids

ENCOUNTER NOTE
Date: 2021-01-15
Provider: Dr. James Wilson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Acute vomiting after meals with progressive dysphagia to solids

HISTORY OF PRESENT ILLNESS:
This 25-year-old male with a history of allergic rhinitis, mild persistent asthma, and atopic dermatitis returns to the emergency department six months after his prior Gastroenterology evaluation with progression of his esophageal symptoms. He presents with a 6-month history of intermittent dysphagia, particularly with solid foods, and a sensation of food getting stuck in his chest. He also reports occasional epigastric pain and has had two episodes of vomiting after meals in the past month.

PAST MEDICAL HISTORY:
Active Problem List:
- Atopic dermatitis (diagnosed 2020-01-15)
- Eosinophilic esophagitis (diagnosed 2020-07-15)
- Allergic rhinitis (seasonal)
- Mild persistent asthma

He has a history of atopic dermatitis and seasonal allergies.

PAST SURGICAL HISTORY:
- None

MEDICATIONS:
Home Medications:
- Albuterol inhaler 90 mcg (2 puffs) inhaled PRN for wheezing/shortness of breath
- Cetirizine 10 mg PO daily
- Triamcinolone acetonide cream 0.1% Apply thin layer to affected skin areas BID PRN for eczema flares

ALLERGIES:
Seasonal pollens (tree, grass, ragweed), Dust mites

FAMILY HISTORY:
His family history is non-contributory.

SOCIAL HISTORY:
He denies smoking, alcohol, or drug use.

PHYSICAL EXAMINATION:
His physical examination is unremarkable, and there is no evidence of anemia or weight loss.

PATHOLOGY:
Biopsy of the esophageal mucosa shows >20 eosinophils per high-power field.

OTHER STUDIES:
An upper endoscopy reveals multiple concentric rings and linear furrows in the esophagus.
