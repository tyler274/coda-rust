# synthetic_hospital patient 1831

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 28 at first encounter, sex F, Caucasian, non-Hispanic, Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-09-16 (Mon) | outpatient | Medical Genetics |
| 1 | 2020-04-15 | 2024-12-17 (Tue) | outpatient | Medical Genetics |
| 2 | 2021-04-15 | 2025-12-22 (Mon) | outpatient | Obstetrics & Gynecology |
| 3 | 2021-10-15 | 2026-06-15 (Mon) | outpatient | Obstetrics & Gynecology |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Blue Cross Blue Shield PPO",
 "occupation": "Elementary school teacher",
 "alcohol_use": "None (abstinent during pregnancy)",
 "family_history": [
  "Father with Marfan syndrome (autosomal dominant)",
  "Maternal first cousin with Gaucher disease type 1 (autosomal recessive)",
  "Mother with type 2 diabetes mellitus"
 ],
 "race_ethnicity": "Caucasian, non-Hispanic",
 "smoking_status": "Never smoker",
 "home_medications": [],
 "surgical_history": [
  "Tonsillectomy at age 8"
 ],
 "chronic_conditions": [],
 "age_at_first_encounter": 28
}
```

Primary diagnoses: ["Down syndrome", "Ventricular septal defect", "Preconception Genetic Counseling", "Autosomal recessive genetic predisposition"]
Comorbidities: ["Preconception Genetic Counseling", "Family History of Hereditary Hemorrhagic Telangiectasia"]

## Encounter 0: source 2020-01-15, practice 2024-09-16, outpatient, Medical Genetics

Chief complaint: presents for genetic counseling following the identification of an autosomal dominant condition in her family

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Medical Genetics
Visit Type: outpatient

CHIEF COMPLAINT:
She presents for genetic counseling following the identification of an autosomal dominant condition in her family

PAST SURGICAL HISTORY:
- Tonsillectomy at age 8

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
A pedigree is shown below indicating the inheritance pattern and the presence of a single nucleotide polymorphism (SNP) marker (alleles A and B) used to track the disease allele.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (abstinent during pregnancy). Occupation: Elementary school teacher.

LABORATORY DATA:
The disorder has a penetrance of 60%. Molecular testing reveals a recombination fraction of 0.20 between the SNP marker and the disease mutation. The mother carries the B marker, which is usually associated with the disease allele, but recombination can separate them. The father is unaffected and does not carry the disease allele. The fetus is found to have inherited the B marker.

## Encounter 1: source 2020-04-15, practice 2024-12-17, outpatient, Medical Genetics

Chief complaint: presents for preconception genetic counseling

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Sarah Chen, MD
Department: Medical Genetics
Visit Type: outpatient

CHIEF COMPLAINT:
She presents for preconception genetic counseling

HISTORY OF PRESENT ILLNESS:
The patient returns to the Medical Genetics clinic for continued preconception genetic counseling following her initial consultation on January 15, 2020, regarding an autosomal dominant condition identified in her family. The genetic counselor explains that the disease results from mutations in a single gene, with a population disease frequency of 1 in 40,000. Further molecular analysis reveals that 30% of pathogenic alleles are due to frameshift mutations, while the remaining 70% are due to point mutations. The patient is concerned about her carrier status and requests information on the likelihood of carrying a frameshift mutation.

PAST MEDICAL HISTORY:
Active Problem List:
- Preconception Genetic Counseling (diagnosed 2020-01-15)

PAST SURGICAL HISTORY:
- Tonsillectomy at age 8

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
She reports a family history of a rare autosomal recessive disorder characterized by progressive neurologic decline and hepatosplenomegaly.

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (abstinent during pregnancy). Occupation: Elementary school teacher.

## Encounter 2: source 2021-04-15, practice 2025-12-22, outpatient, Obstetrics & Gynecology

Chief complaint: routine prenatal visit, concerned about risk of chromosomal abnormalities in her baby due to her age

ENCOUNTER NOTE
Date: 2021-04-15
Provider: Dr. Michael Torres, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
Routine prenatal visit, concerned about risk of chromosomal abnormalities in her baby due to her age

HISTORY OF PRESENT ILLNESS:
Patient returns for routine prenatal care following 2020 preconception counseling. Her pregnancy has been uneventful thus far. She is concerned about the risk of chromosomal abnormalities in her baby due to her age.

PAST MEDICAL HISTORY:
Active Problem List:
- Preconception Genetic Counseling (diagnosed 2020-01-15)
- Autosomal recessive genetic predisposition (diagnosed 2020-04-15)

PAST SURGICAL HISTORY:
- Tonsillectomy at age 8

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with Marfan syndrome (autosomal dominant)
- Maternal first cousin with Gaucher disease type 1 (autosomal recessive)
- Mother with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (abstinent during pregnancy). Occupation: Elementary school teacher.

VITAL SIGNS:
On examination, her blood pressure is 120/78 mmHg, heart rate is 72 beats per minute, and fetal heart rate is 150 beats per minute

PHYSICAL EXAMINATION:
Her abdomen is soft, non-tender, and consistent with a 16-week gestational size. She has no edema, and the rest of the physical examination is unremarkable.

LABORATORY DATA:
The patient opts for maternal serum screening, and the results are as follows:

- Maternal serum α-fetoprotein (MSAFP): 0.5 MoM (decreased)
- β-hCG: 2.5 MoM (increased)
- Inhibin A: 2.0 MoM (increased)
- Estriol: 0.5 MoM (decreased)

## Encounter 3: source 2021-10-15, practice 2026-06-15, outpatient, Obstetrics & Gynecology

Chief complaint: presents for a routine prenatal visit at 24 weeks of gestation

ENCOUNTER NOTE
Date: 2021-10-15
Provider: Dr. Michael Torres, MD
Department: Obstetrics & Gynecology
Visit Type: outpatient

CHIEF COMPLAINT:
She presents for a routine prenatal visit at 24 weeks of gestation

HISTORY OF PRESENT ILLNESS:
She returns for prenatal care following genetic counseling. First child is healthy and she has had no complications during her current pregnancy.

PAST MEDICAL HISTORY:
Active Problem List:
- Preconception Genetic Counseling (diagnosed 2020-01-15)
- Autosomal recessive genetic predisposition (diagnosed 2020-04-15)
- Down syndrome (diagnosed 2021-04-15)

PAST SURGICAL HISTORY:
- Tonsillectomy at age 8

MEDICATIONS:
She takes no medications

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father with Marfan syndrome (autosomal dominant)
- Maternal first cousin with Gaucher disease type 1 (autosomal recessive)
- Mother with type 2 diabetes mellitus

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None (abstinent during pregnancy). Occupation: Elementary school teacher.

LABORATORY DATA:
Her prenatal labs have been unremarkable.

IMAGING:
An ultrasound performed during the visit reveals a fluid-filled stomach and a 'double bubble' sign, suggestive of duodenal atresia. There is also evidence of polyhydramnios.
