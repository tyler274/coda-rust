# synthetic_hospital patient 1828

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex F, White, Commercial PPO.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-01-15 (Fri) | outpatient | Otolaryngology |
| 1 | 2020-04-15 | 2021-04-20 (Tue) | outpatient | Psychiatry |
| 2 | 2021-04-15 | 2022-04-18 (Mon) | outpatient | Psychiatry |
| 3 | 2022-07-15 | 2023-07-10 (Mon) | outpatient | Dermatology |
| 4 | 2023-07-15 | 2024-07-22 (Mon) | outpatient | Otolaryngology |
| 5 | 2025-01-15 | 2026-01-19 (Mon) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "F",
 "allergies": [
  "Aspirin and NSAIDs (anaphylaxis - facial flushing, bronchospasm, hypotension)",
  "Tree and grass pollen"
 ],
 "insurance": "Commercial PPO",
 "occupation": "Early childhood educator at a full-time daycare center",
 "alcohol_use": "Social (1-2 drinks weekly)",
 "family_history": [
  "Father deceased (nontraumatic death, recent)",
  "Mother with seasonal allergies and asthma"
 ],
 "race_ethnicity": "White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "10 mg PO daily",
   "name": "Montelukast"
  },
  {
   "dose": "35 mcg/0.25 mg PO daily",
   "name": "Ethinyl estradiol/norgestimate"
  },
  {
   "dose": "10 mg PO PRN for allergic symptoms",
   "name": "Cetirizine"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "Seasonal and perennial allergic rhinitis",
  "Chronic rhinosinusitis with nasal polyps",
  "Aspirin-exacerbated respiratory disease (Samter's triad)"
 ],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["Aspirin-exacerbated respiratory disease", "Nasal septal perforation", "Adjustment disorder", "Allergic rhinitis", "Primary dermatologic neoplasm or infection"]
Comorbidities: ["Allergic rhinitis", "Adjustment disorder"]

## Encounter 0: source 2020-01-15, practice 2021-01-15, outpatient, Otolaryngology

Chief complaint: Three-month history of nasal congestion, intermittent dry cough, and eye irritation

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Otolaryngology
Visit Type: outpatient

CHIEF COMPLAINT:
Three-month history of nasal congestion, intermittent dry cough, and eye irritation

HISTORY OF PRESENT ILLNESS:
She reports frequent sneezing, clear rhinorrhea, nasal itching, and watery eyes. The cough is dry and worse at night but not associated with meals, exertion, or wheezing. She often notices her symptoms worsening during the workweek but improving slightly on weekends. She asks what is causing her symptoms and what she can do to reduce them.

PAST MEDICAL HISTORY:
Active Problem List:
- Seasonal and perennial allergic rhinitis
- Chronic rhinosinusitis with nasal polyps
- Aspirin-exacerbated respiratory disease (Samter's triad)

MEDICATIONS:
Home Medications:
- Montelukast 10 mg PO daily
- Ethinyl estradiol/norgestimate 35 mcg/0.25 mg PO daily
- Cetirizine 10 mg PO PRN for allergic symptoms

ALLERGIES:
Aspirin and NSAIDs (anaphylaxis - facial flushing, bronchospasm, hypotension), Tree and grass pollen

FAMILY HISTORY:
- Father deceased (nontraumatic death, recent)
- Mother with seasonal allergies and asthma

SOCIAL HISTORY:
She does not smoke. She works full-time at a daycare center, where she spends much of the day supervising toddlers during indoor play.

REVIEW OF SYSTEMS:
She denies fever, sore throat, purulent discharge, facial pain, or pressure.

PHYSICAL EXAMINATION:
On examination, her nasal mucosa is pale and swollen, and her posterior pharynx shows cobblestoning. There is no cervical lymphadenopathy or facial tenderness. Lungs are clear.

## Encounter 1: source 2020-04-15, practice 2021-04-20, outpatient, Psychiatry

Chief complaint: Difficulty sleeping, persistent low mood, and tearfulness for 6 weeks following job loss

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Difficulty sleeping, persistent low mood, and tearfulness for 6 weeks following job loss

HISTORY OF PRESENT ILLNESS:
The patient is a 32-year-old female with established seasonal and perennial allergic rhinitis, chronic rhinosinusitis with nasal polyps, and aspirin-exacerbated respiratory disease who was evaluated in Otolaryngology three months prior for nasal congestion and allergic symptoms. She presents to the Psychiatry clinic with difficulty sleeping, persistent low mood, and tearfulness for the past six weeks following an unexpected layoff from her marketing position, which she had held for the past five years. She reports that she has been applying for new positions but has received few responses and feels discouraged. She describes feeling irritable around friends and family and states that she "snaps at people more than usual." She denies suicidal ideation or feelings of hopelessness but admits that she "cries at random times." Her appetite has remained normal. She continues to take care of her personal hygiene and responsibilities at home, but reports that she often lacks motivation and finds herself watching television most of the day. She describes herself as normally "pretty upbeat" and states that she usually handles stress well, but notes that this job loss "really shook" her confidence. She tried journaling and exercising but reports that her symptoms have not improved.

PAST MEDICAL HISTORY:
Active Problem List:
- Allergic rhinitis (diagnosed 2020-01-15)
- Seasonal and perennial allergic rhinitis
- Chronic rhinosinusitis with nasal polyps
- Aspirin-exacerbated respiratory disease (Samter's triad)

She has no past psychiatric history.

MEDICATIONS:
She has never taken psychiatric medications.

Home Medications:
- Montelukast 10 mg PO daily
- Ethinyl estradiol/norgestimate 35 mcg/0.25 mg PO daily
- Cetirizine 10 mg PO PRN for allergic symptoms

ALLERGIES:
Aspirin and NSAIDs (anaphylaxis - facial flushing, bronchospasm, hypotension), Tree and grass pollen

FAMILY HISTORY:
- Father deceased (nontraumatic death, recent)
- Mother with seasonal allergies and asthma

SOCIAL HISTORY:
She does not drink alcohol and denies illicit drug use.
She has a supportive partner and a close friend whom she speaks with regularly.

PHYSICAL EXAMINATION:
Her physical exam is normal. There are no signs of psychosis or hallucinations. She maintains good eye contact, answers all questions appropriately, and demonstrates logical thought processes.

## Encounter 2: source 2021-04-15, practice 2022-04-18, outpatient, Psychiatry

Chief complaint: Feeling overwhelmed, sad, and anxious for 3 weeks after unexpected job transfer

ENCOUNTER NOTE
Date: 2021-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Psychiatry
Visit Type: outpatient

CHIEF COMPLAINT:
Feeling overwhelmed, sad, and anxious for 3 weeks after unexpected job transfer

HISTORY OF PRESENT ILLNESS:
The patient is a 33-year-old female with seasonal and perennial allergic rhinitis, chronic rhinosinusitis with nasal polyps, and aspirin-exacerbated respiratory disease who returns to psychiatry, having been seen here on April 15, 2020 for similar symptoms after job loss. She presents to the clinic with a 3-week history of feeling overwhelmed, sad, and anxious. She reports difficulty sleeping and a lack of concentration that has affected her performance at work. Her symptoms began shortly after she was informed of an unexpected job transfer to a different city, which she perceives as less prestigious and isolating. She feels hopeless about her future career prospects and has started to socially withdraw, declining invitations from friends and family. She has no thoughts of self-harm or suicide.

PAST MEDICAL HISTORY:
Active Problem List:
- Allergic rhinitis (diagnosed 2020-01-15)
- Adjustment disorder (diagnosed 2020-04-15)
- Seasonal and perennial allergic rhinitis
- Chronic rhinosinusitis with nasal polyps
- Aspirin-exacerbated respiratory disease (Samter's triad)

She denies any previous psychiatric history.

MEDICATIONS:
Home Medications:
- Montelukast 10 mg PO daily
- Ethinyl estradiol/norgestimate 35 mcg/0.25 mg PO daily
- Cetirizine 10 mg PO PRN for allergic symptoms

ALLERGIES:
Aspirin and NSAIDs (anaphylaxis - facial flushing, bronchospasm, hypotension), Tree and grass pollen

FAMILY HISTORY:
- Father deceased (nontraumatic death, recent)
- Mother with seasonal allergies and asthma

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks weekly). Occupation: Early childhood educator at a full-time daycare center.

PHYSICAL EXAMINATION:
Her physical examination is unremarkable

LABORATORY DATA:
Her laboratory tests including a complete blood count, thyroid function tests, and basic metabolic panel are within normal limits.

## Encounter 3: source 2022-07-15, practice 2023-07-10, outpatient, Dermatology

Chief complaint: Intermittent outbreaks of painful, slow-healing lesions on face and upper chest

ENCOUNTER NOTE
Date: 2022-07-15
Provider: Dr. Aisha Patel, MD
Department: Dermatology
Visit Type: outpatient

CHIEF COMPLAINT:
Intermittent outbreaks of painful, slow-healing lesions on face and upper chest

HISTORY OF PRESENT ILLNESS:
The patient is a 34-year-old female with a known history of aspirin-exacerbated respiratory disease, chronic rhinosinusitis with nasal polyps, and prior outpatient treatment for adjustment disorder with anxiety, who presents to dermatology for evaluation of persistent skin lesions. She reports that over the past several months she has developed intermittent outbreaks of painful, slow-healing lesions on her face and upper chest, sometimes accompanied by redness and swelling. She was initially seen at two separate urgent care centers where she received topical antibiotics and corticosteroids with minimal improvement. Concerned by the lack of response and the worsening appearance of her skin, she sought evaluation by a dermatologist who performed a biopsy and confirmed the diagnosis of an uncommon chronic inflammatory skin condition. The dermatologist started her on topical therapy and referred her to primary care for continued management and counseling. During the current visit, the patient appears frustrated and overwhelmed, explaining that she has spent hours reading about the disease online but is finding conflicting information that has made her more anxious. She expresses concern about how this will affect her appearance long term, whether she'll have flares forever, and whether any of her internal organs might eventually be involved. She asks the physician directly, "Have you seen this disease before? I just need someone who knows what I'm going through."

PAST MEDICAL HISTORY:
Active Problem List:
- Allergic rhinitis (diagnosed 2020-01-15)
- Adjustment disorder (diagnosed 2020-04-15)
- Seasonal and perennial allergic rhinitis
- Chronic rhinosinusitis with nasal polyps
- Aspirin-exacerbated respiratory disease (Samter's triad)

MEDICATIONS:
Home Medications:
- Montelukast 10 mg PO daily
- Ethinyl estradiol/norgestimate 35 mcg/0.25 mg PO daily
- Cetirizine 10 mg PO PRN for allergic symptoms

ALLERGIES:
Aspirin and NSAIDs (anaphylaxis - facial flushing, bronchospasm, hypotension), Tree and grass pollen

FAMILY HISTORY:
- Father deceased (nontraumatic death, recent)
- Mother with seasonal allergies and asthma

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks weekly). Occupation: Early childhood educator at a full-time daycare center.

ASSESSMENT AND PLAN:
The physician reviews the specialist's documentation and pathology report and realizes he has no personal experience managing this specific disease and has not previously encountered a patient with this diagnosis.

## Encounter 4: source 2023-07-15, practice 2024-07-22, outpatient, Otolaryngology

Chief complaint: Two-week history of worsening nasal congestion with OTC nasal spray overuse

ENCOUNTER NOTE
Date: 2023-07-15
Provider: Dr. Sarah Chen, MD
Department: Otolaryngology
Visit Type: outpatient

CHIEF COMPLAINT:
Two-week history of worsening nasal congestion with OTC nasal spray overuse

HISTORY OF PRESENT ILLNESS:
The patient returns to Otolaryngology with worsening nasal congestion in the context of chronic rhinosinusitis with nasal polyps, aspirin-exacerbated respiratory disease, and allergic rhinitis, previously evaluated here in 2020. Her symptoms initially improved with over-the-counter nasal spray use. She reports worsening over the past few days despite continued use every 4 hours. She denies fever, sinus pain, or purulent nasal discharge.

PAST MEDICAL HISTORY:
Active Problem List:
- Allergic rhinitis (diagnosed 2020-01-15)
- Adjustment disorder (diagnosed 2020-04-15)
- Primary dermatologic neoplasm or infection (diagnosed 2022-07-15)
- Seasonal and perennial allergic rhinitis
- Chronic rhinosinusitis with nasal polyps
- Aspirin-exacerbated respiratory disease (Samter's triad)

Her medical history is significant for seasonal allergic rhinitis, which she typically manages with an oral antihistamine.

MEDICATIONS:
Home Medications:
- Montelukast 10 mg PO daily
- Ethinyl estradiol/norgestimate 35 mcg/0.25 mg PO daily
- Cetirizine 10 mg PO PRN for allergic symptoms

ALLERGIES:
Aspirin and NSAIDs (anaphylaxis - facial flushing, bronchospasm, hypotension), Tree and grass pollen

FAMILY HISTORY:
- Father deceased (nontraumatic death, recent)
- Mother with seasonal allergies and asthma

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks weekly). Occupation: Early childhood educator at a full-time daycare center.

VITAL SIGNS:
Her blood pressure is 125/78 mmHg, heart rate is 78 beats per minute, and temperature is 98.6°F (37°C).

PHYSICAL EXAMINATION:
On physical examination, her nasal mucosa appears erythematous and swollen. There is no evidence of polyps or purulent discharge. Anterior rhinoscopy reveals engorged nasal turbinates.

## Encounter 5: source 2025-01-15, practice 2026-01-19, ed, Emergency Medicine

Chief complaint: Sudden-onset shortness of breath, facial flushing, and chest tightness after taking naproxen

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. David Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Sudden-onset shortness of breath, facial flushing, and chest tightness after taking naproxen

HISTORY OF PRESENT ILLNESS:
The patient is a 37-year-old female with aspirin-exacerbated respiratory disease (Samter's triad) and chronic rhinosinusitis with nasal polyps, previously treated in Otolaryngology for allergic rhinitis and nasal septal perforation, who presents with acute respiratory symptoms. She reports that she had been feeling well until this morning, when she woke up with generalized muscle soreness following an intense spin class the previous day. For the soreness, she took one over-the-counter tablet of naproxen after breakfast. Within 30–60 minutes, she began experiencing progressive fatigue, lightheadedness, and a sense of chest tightness, followed by wheezing and difficulty breathing. She also felt flushed and noted puffiness around her eyes in the mirror. She worried that her current symptoms might be panic-related but became alarmed when the wheezing and facial swelling developed.

PAST MEDICAL HISTORY:
Active Problem List:
- Allergic rhinitis (diagnosed 2020-01-15)
- Adjustment disorder (diagnosed 2020-04-15)
- Primary dermatologic neoplasm or infection (diagnosed 2022-07-15)
- Nasal septal perforation (diagnosed 2023-07-15)
- Seasonal and perennial allergic rhinitis
- Chronic rhinosinusitis with nasal polyps
- Aspirin-exacerbated respiratory disease (Samter's triad)

Her past medical history is notable for allergic rhinitis, nasal polyps, and seasonal allergies. She denies any prior asthma diagnosis but reports having had episodes of “tight breathing” when sick in the past, which improved with over-the-counter medications. She recalls a similar episode six months ago after taking an over-the-counter medication for a headache, though she did not seek medical care at that time.

MEDICATIONS:
She takes montelukast daily, which has helped with her nasal congestion and postnasal drip. She also takes an oral contraceptive.

Home Medications:
- Ethinyl estradiol/norgestimate 35 mcg/0.25 mg PO daily
- Cetirizine 10 mg PO PRN for allergic symptoms

ALLERGIES:
She has no history of food or drug allergies

FAMILY HISTORY:
- Father deceased (nontraumatic death, recent)
- Mother with seasonal allergies and asthma

SOCIAL HISTORY:
She mentions that the past few weeks have been especially stressful because her father recently passed away. She flew home for the funeral last week and describes feeling emotionally overwhelmed, exhausted, and “on edge” ever since. She has been having trouble sleeping, skipped several meals, and admits she hasn’t been taking as much care of herself. She’s also noticed more frequent headaches and episodes of crying that come on suddenly. She does not smoke, drink, or use illicit drugs.

VITAL SIGNS:
Her vital signs are: temperature 37.1°C (98.8°F), blood pressure 102/68 mm Hg, pulse 96/min, respiratory rate 22/min, and oxygen saturation 94% on room air.

PHYSICAL EXAMINATION:
On examination, she appears mildly anxious and is using accessory muscles to breathe. She has periorbital edema, facial flushing, and diffuse bilateral expiratory wheezes on auscultation. The remainder of the physical exam is unremarkable.

LABORATORY DATA:
CBC, BMP, and troponins are within normal limits.

IMAGING:
Chest X-ray is normal.

PLAN:
Avoidance of
