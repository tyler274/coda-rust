# synthetic_hospital patient 2523

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 32 at first encounter, sex M, Non-Hispanic White, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2022-01-09 (Sun) | inpatient | Internal Medicine |
| 1 | 2020-04-15 | 2022-04-26 (Tue) | outpatient | Infectious Diseases |
| 2 | 2020-10-15 | 2022-10-12 (Wed) | outpatient | Infectious Diseases |
| 3 | 2021-07-15 | 2023-07-24 (Mon) | outpatient | Gastroenterology |
| 4 | 2022-03-15 | 2024-03-18 (Mon) | follow_up | Internal Medicine |
| 5 | 2023-03-15 | 2025-03-20 (Thu) | ed | Emergency Medicine |
| 6 | 2023-11-15 | 2025-11-23 (Sun) | icu | Emergency Medicine |
| 7 | 2024-07-15 | 2026-07-22 (Wed) | inpatient | Infectious Diseases |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "No known drug allergies"
 ],
 "insurance": "Medicaid",
 "occupation": "Freelance travel photographer",
 "alcohol_use": "Social (1-2 drinks per week)",
 "family_history": [
  "Father: Chronic obstructive pulmonary disease",
  "Mother: Major depressive disorder",
  "Brother: Asthma"
 ],
 "race_ethnicity": "Non-Hispanic White",
 "smoking_status": "Never smoker",
 "home_medications": [
  {
   "dose": "90 mcg/actuation, 2 inhalations every 4-6 hours as needed for wheezing/shortness of breath",
   "name": "Albuterol HFA inhaler"
  },
  {
   "dose": "50/25/200 mg tablet daily - prescribed but patient non-adherent for past 6 months",
   "name": "Bictegravir/Tenofovir alafenamide/Emtricitabine (Biktarvy)"
  }
 ],
 "surgical_history": [],
 "chronic_conditions": [
  "HIV infection (AIDS)",
  "Asthma",
  "Opioid use disorder (in recovery with history of intravenous use)"
 ],
 "age_at_first_encounter": 32
}
```

Primary diagnoses: ["History of Pneumocystis jirovecii pneumonia", "Exposure to high altitude", "Heroin overdose", "Severe Acute Respiratory Syndrome (SARS)", "Cryptosporidiosis", "Bacillary angiomatosis"]
Comorbidities: ["Hypoxemia", "Hypochloremic metabolic alkalosis secondary to vomiting with hypovolemia", "Intravenous Drug Use (Substance Use Disorder)", "HIV-related opportunistic skin infection"]

## Encounter 0: source 2020-01-15, practice 2022-01-09, inpatient, Internal Medicine

Chief complaint: Progressive shortness of breath, nonproductive cough, and low-grade fevers for 3 weeks.

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: inpatient

CHIEF COMPLAINT:
Progressive shortness of breath, nonproductive cough, and low-grade fevers for 3 weeks.

HISTORY OF PRESENT ILLNESS:
He presents to the clinic with a 3-week history of progressive shortness of breath, nonproductive cough, and low-grade fevers. He has not been on antiretroviral therapy and does not know his most recent CD4 count.

PAST MEDICAL HISTORY:
Active Problem List:
- HIV infection (AIDS)
- Asthma
- Opioid use disorder (in recovery with history of intravenous use)

MEDICATIONS:
Home Medications:
- Albuterol HFA inhaler 90 mcg/actuation, 2 inhalations every 4-6 hours as needed for wheezing/shortness of breath
- Bictegravir/Tenofovir alafenamide/Emtricitabine (Biktarvy) 50/25/200 mg tablet daily - prescribed but patient non-adherent for past 6 months

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Chronic obstructive pulmonary disease
- Mother: Major depressive disorder
- Brother: Asthma

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Freelance travel photographer.

VITAL SIGNS:
On examination, he is afebrile, tachypneic with a respiratory rate of 24 breaths per minute, and his oxygen saturation is 89% on room air.

PHYSICAL EXAMINATION:
Auscultation of the lungs reveals bilateral diffuse crackles.

LABORATORY DATA:
Laboratory findings are as follows:

- White blood cell count: 6,000/uL
- Hemoglobin: 13.2 g/dL
- Platelets: 250,000/uL
- Lactate dehydrogenase (LDH): 450 U/L (Normal: 140-280 U/L)
- Arterial blood gas on room air:
- pH: 7.42
- PaCO2: 32 mm Hg
- PaO2: 65 mm Hg

IMAGING:
Chest X-ray shows bilateral interstitial infiltrates.

ASSESSMENT AND PLAN:
Based on the clinical presentation and test results,

## Encounter 1: source 2020-04-15, practice 2022-04-26, outpatient, Infectious Diseases

Chief complaint: Progressive dyspnea, nonproductive cough, fever, night sweats, and 10-pound weight loss over one month.

ENCOUNTER NOTE
Date: 2020-04-15
Provider: Dr. Michael Rodriguez, MD
Department: Infectious Diseases
Visit Type: outpatient

CHIEF COMPLAINT:
Progressive dyspnea, nonproductive cough, fever, night sweats, and 10-pound weight loss over one month.

HISTORY OF PRESENT ILLNESS:
Patient returns following prior Pneumocystis pneumonia with ART non-adherence, presenting with 3-week history of progressive dyspnea, nonproductive cough, and fever. He has lost 10 pounds over the last month and complains of night sweats.

PAST MEDICAL HISTORY:
Active Problem List:
- History of Pneumocystis jirovecii pneumonia (diagnosed 2020-01-15)
- HIV infection (AIDS)
- Asthma
- Opioid use disorder (in recovery with history of intravenous use)

MEDICATIONS:
Home Medications:
- Albuterol HFA inhaler 90 mcg/actuation, 2 inhalations every 4-6 hours as needed for wheezing/shortness of breath
- Bictegravir/Tenofovir alafenamide/Emtricitabine (Biktarvy) 50/25/200 mg tablet daily - prescribed but patient non-adherent for past 6 months

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Chronic obstructive pulmonary disease
- Mother: Major depressive disorder
- Brother: Asthma

SOCIAL HISTORY:
History of IV drug use

VITAL SIGNS:
Temperature is 38.6°C, blood pressure is 110/70 mm Hg, and pulse is 110 beats per minute. Oxygen saturation on room air is 88%.

PHYSICAL EXAMINATION:
On examination, he is thin, appears ill, and is tachypneic with a respiratory rate of 28 breaths per minute. Lung auscultation reveals bilateral diffuse crackles.

LABORATORY DATA:
CD4 count: 50/mm3; Hemoglobin: 12 g/dL; White blood cell count: 6,000/mm3; Platelet count: 250,000/mm3; Arterial blood gas on room air: pH: 7.45; PaCO2: 30 mm Hg; PaO2: 65 mm Hg

IMAGING:
Chest X-ray shows bilateral interstitial infiltrates

## Encounter 2: source 2020-10-15, practice 2022-10-12, outpatient, Infectious Diseases

Chief complaint: Multiple bright red nodules on skin for 3 weeks that bleed easily when traumatized.

ENCOUNTER NOTE
Date: 2020-10-15
Provider: Dr. Michael Rodriguez, MD
Department: Infectious Diseases
Visit Type: outpatient

CHIEF COMPLAINT:
Multiple bright red nodules on skin for 3 weeks that bleed easily when traumatized.

HISTORY OF PRESENT ILLNESS:
The patient with HIV/AIDS and prior Pneumocystis pneumonia returns with new skin complaints. He reports the onset of multiple bright red nodules on his skin that are painless but bleed easily when traumatized. He has not been adherent to his antiretroviral therapy for the past 6 months.

PAST MEDICAL HISTORY:
Active Problem List:
- History of Pneumocystis jirovecii pneumonia (diagnosed 2020-01-15)
- HIV infection (AIDS)
- Asthma
- Opioid use disorder (in recovery with history of intravenous use)

MEDICATIONS:
Home Medications:
- Albuterol HFA inhaler 90 mcg/actuation, 2 inhalations every 4-6 hours as needed for wheezing/shortness of breath
- Bictegravir/Tenofovir alafenamide/Emtricitabine (Biktarvy) 50/25/200 mg tablet daily - prescribed but patient non-adherent for past 6 months

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Chronic obstructive pulmonary disease
- Mother: Major depressive disorder
- Brother: Asthma

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Freelance travel photographer.

VITAL SIGNS:
Temperature is 37.2°C, blood pressure is 125/78 mmHg, and heart rate is 88 beats per minute

PHYSICAL EXAMINATION:
On examination, you note several firm, friable, exophytic nodules on his arms and back. There is no lymphadenopathy or hepatosplenomegaly.

LABORATORY DATA:
CD4+ count: 180 cells/µL
HIV viral load: 120,000 copies/mL
Hemoglobin: 13.2 g/dL
White blood cell count: 7,800 cells/µL
Platelet count: 250,000 cells/µL

PATHOLOGY:
Proliferation of small blood vessels lined by epithelioid endothelial cells, with a neutrophilic infiltrate

## Encounter 3: source 2021-07-15, practice 2023-07-24, outpatient, Gastroenterology

Chief complaint: Profuse watery diarrhea 5-6 times daily with abdominal cramping for 10 days after returning from Peru.

ENCOUNTER NOTE
Date: 2021-07-15
Provider: Dr. Jennifer Park, MD
Department: Gastroenterology
Visit Type: outpatient

CHIEF COMPLAINT:
Profuse watery diarrhea 5-6 times daily with abdominal cramping for 10 days after returning from Peru.

HISTORY OF PRESENT ILLNESS:
This 33-year-old male with AIDS, history of Pneumocystis jirovecii pneumonia and bacillary angiomatosis, and six months of antiretroviral non-adherence, reports recently returning from a 2-week hiking trip in Peru. The diarrhea is profuse, occurring 5-6 times per day, and is not bloody. He denies fever or vomiting.

PAST MEDICAL HISTORY:
Active Problem List:
- History of Pneumocystis jirovecii pneumonia (diagnosed 2020-01-15)
- Bacillary angiomatosis (diagnosed 2020-10-15)
- HIV infection (AIDS)
- Asthma
- Opioid use disorder (in recovery with history of intravenous use)

His past medical history is significant for HIV infection, for which he has not been taking antiretroviral therapy.

MEDICATIONS:
Home Medications:
- Albuterol HFA inhaler 90 mcg/actuation, 2 inhalations every 4-6 hours as needed for wheezing/shortness of breath
- Bictegravir/Tenofovir alafenamide/Emtricitabine (Biktarvy) 50/25/200 mg tablet daily - prescribed but patient non-adherent for past 6 months

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Chronic obstructive pulmonary disease
- Mother: Major depressive disorder
- Brother: Asthma

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Freelance travel photographer.

VITAL SIGNS:
His vital signs are stable

PHYSICAL EXAMINATION:
On physical examination, he appears dehydrated. Abdominal examination reveals mild diffuse tenderness without rebound or guarding.

LABORATORY DATA:
Stool studies are negative for bacterial pathogens, ova, and parasites on microscopy. A stool antigen test for Cryptosporidium is positive.

Laboratory Values:
- WBC: 5,000/uL
- Hemoglobin: 13.5 g/dL
- Platelets: 250,000/uL
- Sodium: 140 mEq/L
- Potassium: 3.5 mEq/L
- Chloride: 100 mEq/L
- Bicarbonate: 24 mEq/L
- BUN: 18 mg/dL
- Creatinine: 1.0 mg/dL
- HIV viral load: 120,000 copies/mL
- CD4 count: 180 cells/mm³

## Encounter 4: source 2022-03-15, practice 2024-03-18, follow_up, Internal Medicine

Chief complaint: Follow-up evaluation two days after returning from hiking trip in the Rocky Mountains where he experienced altitude-related symptoms.

ENCOUNTER NOTE
Date: 2022-03-15
Provider: Dr. David Okonkwo, MD
Department: Internal Medicine
Visit Type: follow_up

CHIEF COMPLAINT:
Follow-up evaluation two days after returning from hiking trip in the Rocky Mountains where he experienced altitude-related symptoms.

HISTORY OF PRESENT ILLNESS:
The patient is a 34-year-old male with HIV/AIDS (non-adherent to antiretroviral therapy), asthma, and a history of opportunistic infections including Pneumocystis jirovecii pneumonia, bacillary angiomatosis, and cryptosporidiosis, returning for follow-up two days after a hiking trip in the Rocky Mountains where he spent a week at elevations above 10,000 feet. He reports that he experienced mild headaches, dizziness, and shortness of breath during his first day at altitude but was advised to begin a medication by a local physician to help him acclimate more quickly. After taking the medication, he noticed he began urinating more frequently and his headaches improved. On return home, he feels completely back to baseline. The medication he took is known to decrease renal bicarbonate reabsorption by acting in the proximal tubule.

PAST MEDICAL HISTORY:
Active Problem List:
- History of Pneumocystis jirovecii pneumonia (diagnosed 2020-01-15)
- Bacillary angiomatosis (diagnosed 2020-10-15)
- Cryptosporidiosis (diagnosed 2021-07-15)
- HIV infection (AIDS)
- Asthma
- Opioid use disorder (in recovery with history of intravenous use)

MEDICATIONS:
Home Medications:
- Albuterol HFA inhaler 90 mcg/actuation, 2 inhalations every 4-6 hours as needed for wheezing/shortness of breath
- Bictegravir/Tenofovir alafenamide/Emtricitabine (Biktarvy) 50/25/200 mg tablet daily - prescribed but patient non-adherent for past 6 months

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Chronic obstructive pulmonary disease
- Mother: Major depressive disorder
- Brother: Asthma

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Freelance travel photographer.

REVIEW OF SYSTEMS:
He denies any chest pain, palpitations, or cough.

PHYSICAL EXAMINATION:
Physical examination is normal.

## Encounter 5: source 2023-03-15, practice 2025-03-20, ed, Emergency Medicine

Chief complaint: Persistent shortness of breath, lightheadedness, difficulty sleeping, and occasional headache after returning from Andes mountains.

ENCOUNTER NOTE
Date: 2023-03-15
Provider: Dr. Emily Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Persistent shortness of breath, lightheadedness, difficulty sleeping, and occasional headache after returning from Andes mountains.

HISTORY OF PRESENT ILLNESS:
This 35-year-old with HIV/AIDS and prior altitude illness presents following Andes travel. During the climb he developed persistent shortness of breath, lightheadedness, difficulty sleeping, and occasional headache, but denied chest pain, cough, or fever.

PAST MEDICAL HISTORY:
Active Problem List:
- History of Pneumocystis jirovecii pneumonia (diagnosed 2020-01-15)
- Bacillary angiomatosis (diagnosed 2020-10-15)
- Cryptosporidiosis (diagnosed 2021-07-15)
- Exposure to high altitude (diagnosed 2022-03-15)
- HIV infection (AIDS)
- Asthma
- Opioid use disorder (in recovery with history of intravenous use)

MEDICATIONS:
Home Medications:
- Albuterol HFA inhaler 90 mcg/actuation, 2 inhalations every 4-6 hours as needed for wheezing/shortness of breath
- Bictegravir/Tenofovir alafenamide/Emtricitabine (Biktarvy) 50/25/200 mg tablet daily - prescribed but patient non-adherent for past 6 months

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
His family history is noncontributory.

SOCIAL HISTORY:
He does not smoke,

VITAL SIGNS:
On examination in the clinic, his pulse is 96/min, respirations are 20/min, blood pressure is 118/74 mmHg, and oxygen saturation on room air is 90%.

PHYSICAL EXAMINATION:
He appears comfortable at rest and lung auscultation is clear without wheezes or rales. Neurologic and cardiovascular examinations are normal.

LABORATORY DATA:
Laboratory studies reveal an arterial blood gas with elevated pH and decreased PaCO₂.

## Encounter 6: source 2023-11-15, practice 2025-11-23, icu, Emergency Medicine

Chief complaint: Found unresponsive at home by friends after suspected relapse of heroin use.

ENCOUNTER NOTE
Date: 2023-11-15
Provider: Dr. James Smith, MD
Department: Emergency Medicine
Visit Type: icu

CHIEF COMPLAINT:
Found unresponsive at home by friends after suspected relapse of heroin use.

HISTORY OF PRESENT ILLNESS:
The patient is a 35-year-old male with HIV infection, history of opportunistic infections, and non-adherence with antiretroviral therapy, who has opioid use disorder in recovery. He is brought to the emergency department by his friends after they found him unresponsive at home. His friends report that he has a history of heroin use but had been in a rehabilitation program and was believed to be clean for the past three months. The patient's friends deny any knowledge of what substances he might have taken.

PAST MEDICAL HISTORY:
Active Problem List:
- History of Pneumocystis jirovecii pneumonia (diagnosed 2020-01-15)
- Bacillary angiomatosis (diagnosed 2020-10-15)
- Cryptosporidiosis (diagnosed 2021-07-15)
- Exposure to high altitude (diagnosed 2022-03-15)
- HIV infection (AIDS)
- Asthma
- Opioid use disorder (in recovery with history of intravenous use)

MEDICATIONS:
Home Medications:
- Albuterol HFA inhaler 90 mcg/actuation, 2 inhalations every 4-6 hours as needed for wheezing/shortness of breath
- Bictegravir/Tenofovir alafenamide/Emtricitabine (Biktarvy) 50/25/200 mg tablet daily - prescribed but patient non-adherent for past 6 months

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Chronic obstructive pulmonary disease
- Mother: Major depressive disorder
- Brother: Asthma

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Freelance travel photographer.

VITAL SIGNS:
Vital signs are as follows: BP 110/70 mmHg, HR 55 bpm, Temperature 36.6°C (97.9°F), and SpO2 88% on room air.

PHYSICAL EXAMINATION:
On examination, the patient is somnolent but arousable to painful stimuli. His respiratory rate is 6 breaths per minute, and breath sounds are shallow. Pupils are 4 mm and reactive to light. His skin is warm and dry. Bowel sounds are decreased.

LABORATORY DATA:
Blood glucose is 98 mg/dL. A urine drug screen is pending.

## Encounter 7: source 2024-07-15, practice 2026-07-22, inpatient, Infectious Diseases

Chief complaint: High-grade fever, cough, and difficulty breathing after returning from country with ongoing outbreak of highly contagious respiratory virus.

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. Maria Garcia, MD
Department: Infectious Diseases
Visit Type: inpatient

CHIEF COMPLAINT:
High-grade fever, cough, and difficulty breathing after returning from country with ongoing outbreak of highly contagious respiratory virus.

HISTORY OF PRESENT ILLNESS:
The patient with HIV/AIDS (non-adherent), asthma, and prior Pneumocystis jirovecii pneumonia recently returned from a country with an ongoing outbreak of a highly contagious respiratory virus known to cause severe respiratory distress syndrome.

PAST MEDICAL HISTORY:
Active Problem List:
- History of Pneumocystis jirovecii pneumonia (diagnosed 2020-01-15)
- Bacillary angiomatosis (diagnosed 2020-10-15)
- Cryptosporidiosis (diagnosed 2021-07-15)
- Exposure to high altitude (diagnosed 2022-03-15)
- Heroin overdose (diagnosed 2023-11-15)
- HIV infection (AIDS)
- Asthma
- Opioid use disorder (in recovery with history of intravenous use)

His medical history is significant for asthma.

MEDICATIONS:
He takes uses an albuterol inhaler as needed.

Home Medications:
- Albuterol HFA inhaler 90 mcg/actuation, 2 inhalations every 4-6 hours as needed for wheezing/shortness of breath
- Bictegravir/Tenofovir alafenamide/Emtricitabine (Biktarvy) 50/25/200 mg tablet daily - prescribed but patient non-adherent for past 6 months

ALLERGIES:
No known drug allergies

FAMILY HISTORY:
- Father: Chronic obstructive pulmonary disease
- Mother: Major depressive disorder
- Brother: Asthma

SOCIAL HISTORY:
Smoking: Never smoker. Alcohol: Social (1-2 drinks per week). Occupation: Freelance travel photographer.

VITAL SIGNS:
Respiratory rate of 28 breaths per minute, oxygen saturation is 89% on room air

PHYSICAL EXAMINATION:
Tachypneic and diffuse bilateral crackles are heard on auscultation

LABORATORY DATA:
Laboratory tests reveal:

- White blood cell count: 12,000 /µL
- Lymphocyte percentage: 15%
- Platelet count: 150,000 /µL
- C-reactive protein: 80 mg/L (normal <10 mg/L)
- Arterial blood gas:
- pH: 7.45
- PaCO2: 32 mmHg
- PaO2: 55 mmHg
- HCO3-: 24 mEq/L

IMAGING:
Chest X-ray shows bilateral interstitial infiltrates

ASSESSMENT AND PLAN:
Public health officials have identified the virus as highly transmissible with significant morbidity and mortality rates, and have recommended isolation and antiviral treatment for all cases.

The patient refuses any treatment, stating he believes in natural healing and fears potential side effects of medications.
