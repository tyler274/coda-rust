# synthetic_hospital patient 2810

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 36 at first encounter, sex F, Non-Hispanic White, Employer-sponsored Blue Cross Blue Shield PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2024-01-17 (Wed) | outpatient | Rheumatology |
| 1 | 2022-01-15 | 2026-01-19 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Employer-sponsored Blue Cross Blue Shield PPO",
 "occupation": "Elementary School Teacher",
 "alcohol_use": "None",
 "family_history": [
  "Mother with rheumatoid arthritis",
  "Father with type 2 diabetes mellitus",
  "Sister with hypothyroidism"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "15 mg PO weekly",
   "name": "Methotrexate"
  },
  {
   "dose": "1 mg PO daily",
   "name": "Folic acid"
  },
  {
   "dose": "5 mg PO daily",
   "name": "Prednisone"
  },
  {
   "dose": "325 mg PO daily",
   "name": "Ferrous sulfate"
  },
  {
   "dose": "400 mg PO every 6 hours PRN joint pain",
   "name": "Ibuprofen"
  }
 ],
 "surgical_history": [
  "Tonsillectomy",
  "Wisdom teeth extraction"
 ],
 "chronic_conditions": [
  "Seropositive rheumatoid arthritis",
  "Chronic anemia"
 ],
 "age_at_first_encounter": 36
}
```

Primary diagnoses: ["Chronic anemia", "Tuberculosis"]
Comorbidities: ["Rheumatoid arthritis"]

## Encounter 0: source 2020-01-15, practice 2024-01-17, outpatient, Rheumatology

Chief complaint: Joint pain, fatigue, and low energy

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Elena Vasquez, MD
Department: Rheumatology
Visit Type: outpatient

CHIEF COMPLAINT:
Joint pain, fatigue, and low energy

HISTORY OF PRESENT ILLNESS:
Over the past 9 months, she has had gradually worsening morning stiffness in her wrists, knees, and small joints of the hands, especially the second and third metacarpophalangeal (MCP) and proximal interphalangeal (PIP) joints. The stiffness lasts nearly two hours each morning before improving. She has difficulty opening jars, typing, and occasionally needs help with grooming. She also reports persistent fatigue and has noticed that her skin appears "pale" in photos.

PAST MEDICAL HISTORY:
Active Problem List:
- Seropositive rheumatoid arthritis
- Chronic anemia

PAST SURGICAL HISTORY:
- Tonsillectomy
- Wisdom teeth extraction

MEDICATIONS:
Home Medications:
- Methotrexate 15 mg PO weekly
- Folic acid 1 mg PO daily
- Prednisone 5 mg PO daily
- Ferrous sulfate 325 mg PO daily
- Ibuprofen 400 mg PO every 6 hours PRN joint pain

ALLERGIES:
NKDA

FAMILY HISTORY:
- Mother with rheumatoid arthritis
- Father with type 2 diabetes mellitus
- Sister with hypothyroidism

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: None. Occupation: Elementary School Teacher.

REVIEW OF SYSTEMS:
She denies gastrointestinal symptoms, blood loss, heavy menses, or dietary restrictions.

VITAL SIGNS:
Her vital signs are normal

PHYSICAL EXAMINATION:
There is symmetric swelling and tenderness in both wrists and the second and third MCPs bilaterally. The DIP joints are spared.

LABORATORY DATA:
Laboratory studies show hemoglobin of 10.3 g/dL, mean corpuscular volume of 84 fL, white blood cell count of 7,100/μL, and platelet count of 421,000/μL. Erythrocyte sedimentation rate is 68 mm/hr and C-reactive protein is 34 mg/L. Ferritin is normal, serum iron is low, and transferrin is low. Additional testing is positive for rheumatoid factor and anti–cyclic citrullinated peptide antibodies.

## Encounter 1: source 2022-01-15, practice 2026-01-19, ed, Emergency Medicine

Chief complaint: Progressive cough, fever, weight loss, and night sweats over the past 6 weeks

ENCOUNTER NOTE
Date: 2022-01-15
Provider: Dr. Kwame Asante, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Progressive cough, fever, weight loss, and night sweats over the past 6 weeks

HISTORY OF PRESENT ILLNESS:
The patient is a 38-year-old female with seropositive rheumatoid arthritis and chronic anemia initially documented during a rheumatology encounter in 2020 who presents to the emergency department with progressive respiratory and constitutional symptoms. She states that she first noticed mild fatigue and reduced appetite but attributed it to work-related stress. Over time, she developed an intermittent cough productive of yellow sputum and occasional streaks of blood. She has also lost 12 pounds unintentionally in the past 2 months. The night sweats have worsened, and she now wakes up drenched in sweat several times per week.

PAST MEDICAL HISTORY:
Active Problem List:
- Chronic anemia (diagnosed 2020-01-15)
- Seropositive rheumatoid arthritis

Her medical history is significant for seropositive rheumatoid arthritis, diagnosed 4 years ago. Initially she was treated with nonsteroidal anti-inflammatory drugs and low-dose prednisone for symptom control. She was then started on methotrexate, which helped temporarily, but her disease flared with persistent joint pain, swelling, and morning stiffness. Five months ago, she was started on adalimumab because her arthritis was inadequately controlled. Her joint pain has since improved dramatically, and she was able to taper off prednisone completely. However, she recalls a positive tuberculin skin test during college, when she participated in a volunteer program in India, but she never received treatment at that time.

PAST SURGICAL HISTORY:
- Tonsillectomy
- Wisdom teeth extraction

MEDICATIONS:
Home Medications:
- Methotrexate 15 mg PO weekly
- Folic acid 1 mg PO daily
- Prednisone 5 mg PO daily
- Ferrous sulfate 325 mg PO daily
- Ibuprofen 400 mg PO every 6 hours PRN joint pain

ALLERGIES:
NKDA

FAMILY HISTORY:
Family history is notable only for hypertension in her parents.

SOCIAL HISTORY:
She works as a schoolteacher and lives with her husband and two young children. She does not drink alcohol, use recreational drugs, or smoke.

VITAL SIGNS:
Her temperature is 38.3°C (100.9°F), blood pressure 110/70 mm Hg, pulse 96/min, and respirations 20/min. Oxygen saturation is 95% on room air.

PHYSICAL EXAMINATION:
On physical examination, the patient is tired-appearing and underweight. Auscultation of the lungs reveals crackles over the right upper lobe. Mild cervical lymphadenopathy is present. Examination of her hands shows decreased swelling and tenderness of the metacarpophalangeal joints, consistent with good control of her arthritis on adalimumab.

LABORATORY DATA:
Laboratory studies reveal normocytic anemia, an elevated erythrocyte sedimentation rate, and a C-reactive protein level mildly above normal. HIV test is negative. Three sputum samples are positive for acid-fast bacilli on Ziehl-Neelsen staining.

IMAGING:
Chest x-ray demonstrates cavitary lesions in the upper lobes of both lungs.
