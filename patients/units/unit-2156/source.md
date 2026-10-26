# synthetic_hospital patient 2156

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 41 at first encounter, sex M, Non-Hispanic White (Northern European ancestry), Private (Aetna).

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-05-15 (Wed) | outpatient | Dermatology |
| 1 | 2021-07-15 | 2025-11-04 (Tue) | outpatient | Rheumatology |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "Penicillin (anaphylaxis)",
  "Sulfa drugs (rash)"
 ],
 "insurance": "Private (Aetna)",
 "occupation": "Landscaping contractor",
 "alcohol_use": "Social (1 drink per week), reduced following liver enzyme elevation",
 "family_history": [
  "Father: Hereditary hemochromatosis with cirrhosis, type 2 diabetes mellitus",
  "Paternal grandfather: Liver disease (suspected hemochromatosis)",
  "Mother: Hypothyroidism",
  "Sister: Elevated serum ferritin"
 ],
 "race_ethnicity": "Non-Hispanic White (Northern European ancestry)",
 "smoking_status": "Former, quit 10 years ago (8 pack-year history)",
 "home_medications": [
  {
   "dose": "500 mg PO PRN pain (max 2g/day)",
   "name": "Acetaminophen"
  },
  {
   "dose": "200 mg PO twice weekly",
   "name": "Hydroxychloroquine"
  },
  {
   "dose": "2000 IU PO daily",
   "name": "Vitamin D3"
  },
  {
   "dose": "Apply liberally to exposed skin BID",
   "name": "Zinc oxide sunscreen SPF 50+"
  }
 ],
 "surgical_history": [
  "Cholecystectomy (age 32)",
  "Left knee meniscectomy (age 38)"
 ],
 "chronic_conditions": [
  "Hereditary hemochromatosis (HFE-related)",
  "Porphyria cutanea tarda",
  "Chronic hepatitis C infection",
  "History of intravenous substance use disorder (in remission, 10 years)",
  "Iron deposition arthropathy"
 ],
 "age_at_first_encounter": 41
}
```

Primary diagnoses: ["Hereditary Hemochromatosis with arthropathy", "Porphyria cutanea tarda"]
Comorbidities: ["Hepatic dysfunction due to iron overload"]

## Encounter 0: source 2020-01-15, practice 2024-05-15, outpatient, Dermatology

Chief complaint: Skin lesions and joint pain for 6 months with fragile skin on dorsal hands and blistering after minor trauma or sun exposure

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Elena Rodriguez, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Skin lesions and joint pain for 6 months with fragile skin on dorsal hands and blistering after minor trauma or sun exposure

HISTORY OF PRESENT ILLNESS:
He reports that his skin has become very fragile, especially on the back of his hands, and he develops blisters following minor trauma or after spending time in the sun. He also mentions experiencing arthralgias in his hands and wrists.

PAST MEDICAL HISTORY:
Active Problem List:
- Hereditary hemochromatosis (HFE-related)
- Porphyria cutanea tarda
- Chronic hepatitis C infection
- History of intravenous substance use disorder (in remission, 10 years)
- Iron deposition arthropathy

His past medical history is significant for intravenous drug use, which he quit 10 years ago.

PAST SURGICAL HISTORY:
- Cholecystectomy (age 32)
- Left knee meniscectomy (age 38)

MEDICATIONS:
Home Medications:
- Acetaminophen 500 mg PO PRN pain (max 2g/day)
- Hydroxychloroquine 200 mg PO twice weekly
- Vitamin D3 2000 IU PO daily
- Zinc oxide sunscreen SPF 50+ Apply liberally to exposed skin BID

ALLERGIES:
Penicillin (anaphylaxis), Sulfa drugs (rash)

FAMILY HISTORY:
- Father: Hereditary hemochromatosis with cirrhosis, type 2 diabetes mellitus
- Paternal grandfather: Liver disease (suspected hemochromatosis)
- Mother: Hypothyroidism
- Sister: Elevated serum ferritin

SOCIAL HISTORY:
Smoking: Former, quit 10 years ago (8 pack-year history). Alcohol: Social (1 drink per week), reduced following liver enzyme elevation. Occupation: Landscaping contractor.

PHYSICAL EXAMINATION:
On physical examination, you note several vesicles and bullae on the dorsal aspects of his hands and forearms, as well as mild synovitis in the wrists. There is no scleral icterus or hepatomegaly.

LABORATORY DATA:
Laboratory studies are ordered, and the results are as follows:

- Complete Blood Count (CBC):
- Hemoglobin: 14.2 g/dL
- White Blood Cell Count: 6,800 cells/mm^3
- Platelet Count: 250,000 cells/mm^3
- Liver Function Tests (LFTs):
- Aspartate Aminotransferase (AST): 68 U/L (Normal: 10-40 U/L)
- Alanine Aminotransferase (ALT): 74 U/L (Normal: 7-56 U/L)
- Alkaline Phosphatase (ALP): 90 U/L (Normal: 40-130 U/L)
- Total Bilirubin: 1.2 mg/dL (Normal: 0.3-1.2 mg/dL)
- Urine Uroporphyrin: Elevated
- Serum Iron Studies:
- Serum Iron: 180 mcg/dL (Normal: 60-170 mcg/dL)
- Total Iron Binding Capacity (TIBC): 300 mcg/dL (Normal: 250-450 mcg/dL)
- Ferritin: 310 ng/mL (Normal: 30-400 ng/mL)

## Encounter 1: source 2021-07-15, practice 2025-11-04, outpatient, Rheumatology

Chief complaint: Progressive bilateral hand joint pain and stiffness for 6 months with swelling in second and third MCP joints, accompanied by wrist and knee involvement

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Samuel Okonkwo, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive bilateral hand joint pain and stiffness for 6 months with swelling in second and third MCP joints, accompanied by wrist and knee involvement

HISTORY OF PRESENT ILLNESS:
The patient with established hereditary hemochromatosis-related arthropathy, previously evaluated for porphyria cutanea tarda in 2020, returns with progressive joint symptoms. He reports that the pain is most severe in his hands, particularly in the second and third metacarpophalangeal joints, and is accompanied by swelling. He also experiences pain in his wrists and knees. He denies any morning stiffness or skin rashes.

PAST MEDICAL HISTORY:
Active Problem List:
- Porphyria cutanea tarda (diagnosed 2020-01-15)
- Hereditary hemochromatosis (HFE-related)
- Chronic hepatitis C infection
- History of intravenous substance use disorder (in remission, 10 years)
- Iron deposition arthropathy

His past medical history is significant for elevated liver enzymes discovered incidentally during a routine check-up one year ago.

PAST SURGICAL HISTORY:
- Cholecystectomy (age 32)
- Left knee meniscectomy (age 38)

MEDICATIONS:
Home Medications:
- Acetaminophen 500 mg PO PRN pain (max 2g/day)
- Hydroxychloroquine 200 mg PO twice weekly
- Vitamin D3 2000 IU PO daily
- Zinc oxide sunscreen SPF 50+ Apply liberally to exposed skin BID

ALLERGIES:
Penicillin (anaphylaxis), Sulfa drugs (rash)

FAMILY HISTORY:
- Father: Hereditary hemochromatosis with cirrhosis, type 2 diabetes mellitus
- Paternal grandfather: Liver disease (suspected hemochromatosis)
- Mother: Hypothyroidism
- Sister: Elevated serum ferritin

SOCIAL HISTORY:
Smoking: Former, quit 10 years ago (8 pack-year history). Alcohol: Social (1 drink per week), reduced following liver enzyme elevation. Occupation: Landscaping contractor.

PHYSICAL EXAMINATION:
On physical examination, there is noticeable swelling and tenderness over the second and third metacarpophalangeal joints and both wrists. There is no synovitis or deformity noted.

LABORATORY DATA:
Laboratory tests reveal the following:

- Serum iron: 180 mcg/dL (normal: 60-170 mcg/dL)
- Total iron-binding capacity: 250 mcg/dL (normal: 250-450 mcg/dL)
- Transferrin saturation: 72% (normal: 20-50%)
- Ferritin: 600 ng/mL (normal: 30-400 ng/mL)

IMAGING:
Radiographs of the hands show chondrocalcinosis and joint space narrowing at the affected metacarpophalangeal joints.
