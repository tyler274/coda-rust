# synthetic_hospital patient 2562

From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), benchmark_v1.3.db, MIT, fully synthetic.
Source: age 30 at first encounter, sex M, White, non-Hispanic, Medicaid.

## Practice dates

Each source encounter moved to end before the practice's today (2026-08-27) and off the 15th.

| # | Source date | Practice date | Type | Department |
|---|---|---|---|---|
| 0 | 2020-01-15 | 2021-04-22 (Thu) | outpatient | Internal Medicine |
| 1 | 2020-07-15 | 2021-10-11 (Mon) | outpatient | Orthopedics |
| 2 | 2021-03-15 | 2022-06-18 (Sat) | ed | Emergency Medicine |
| 3 | 2021-09-15 | 2022-12-15 (Thu) | ed | Emergency Medicine |
| 4 | 2022-05-15 | 2023-08-15 (Tue) | ed | Emergency Medicine |
| 5 | 2024-01-15 | 2025-04-21 (Mon) | ed | Emergency Medicine |
| 6 | 2024-07-15 | 2025-10-25 (Sat) | icu | Critical Care |
| 7 | 2025-01-15 | 2026-04-22 (Wed) | ed | Emergency Medicine |

## Profile

```json
{
 "sex": "M",
 "allergies": [
  "NKDA"
 ],
 "insurance": "Medicaid",
 "occupation": "Unemployed; formerly construction laborer",
 "alcohol_use": "Occasional, 1-2 drinks per week",
 "family_history": [
  "Opioid use disorder in older brother",
  "Major depressive disorder in mother",
  "Hypertension in father"
 ],
 "race_ethnicity": "White, non-Hispanic",
 "smoking_status": "Current daily smoker, 1 pack per day",
 "home_medications": [],
 "surgical_history": [
  "Splenectomy (2 years ago following motor vehicle accident)",
  "Left below-knee amputation (6 months ago following traumatic injury)"
 ],
 "chronic_conditions": [
  "Opioid use disorder, severe, recurrent",
  "Asplenia",
  "Chronic pain syndrome (post-amputation and mechanical low back)",
  "History of motor vehicle accident sequelae"
 ],
 "age_at_first_encounter": 30
}
```

Primary diagnoses: ["Opioid Withdrawal", "Phrenic nerve injury", "Heroin overdose", "Septic shock", "Intravenous Drug Use (Substance Use Disorder)"]
Comorbidities: ["History of Blunt Abdominal Trauma", "Intravenous Drug Use (Substance Use Disorder)"]

## Encounter 0: source 2020-01-15, practice 2021-04-22, outpatient, Internal Medicine

Chief complaint: Patient with opioid use disorder seeks help to quit heroin use and requests treatment

ENCOUNTER NOTE
Date: 2020-01-15
Provider: Dr. Sarah Chen, MD
Department: Internal Medicine
Visit Type: outpatient

CHIEF COMPLAINT:
Patient with opioid use disorder seeks help to quit heroin use and requests treatment

HISTORY OF PRESENT ILLNESS:
He has been using heroin daily for the past year and has tried to quit multiple times but has been unsuccessful. He now wants help to quit. His physician recommends starting buprenorphine. During the visit, the patient's girlfriend says to the doctor, "How can you give him another drug to stop it? Please don't prescribe it to him!"

PAST MEDICAL HISTORY:
Active Problem List:
- Opioid use disorder, severe, recurrent
- Asplenia
- Chronic pain syndrome (post-amputation and mechanical low back)
- History of motor vehicle accident sequelae

PAST SURGICAL HISTORY:
- Splenectomy (2 years ago following motor vehicle accident)
- Left below-knee amputation (6 months ago following traumatic injury)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Opioid use disorder in older brother
- Major depressive disorder in mother
- Hypertension in father

SOCIAL HISTORY:
Smoking: Current daily smoker, 1 pack per day. Alcohol: Occasional, 1-2 drinks per week. Occupation: Unemployed; formerly construction laborer.

## Encounter 1: source 2020-07-15, practice 2021-10-11, outpatient, Orthopedics

Chief complaint: Sharp, shooting pain in left lower limb affecting prosthetic limb use

ENCOUNTER NOTE
Date: 2020-07-15
Provider: Dr. Michael Patel, MD
Department: Orthopedics
Visit Type: outpatient

CHIEF COMPLAINT:
Sharp, shooting pain in left lower limb affecting prosthetic limb use

HISTORY OF PRESENT ILLNESS:
The patient is a 30-year-old male with opioid use disorder and chronic pain evaluated for substance use treatment. He returns with pain in left lower limb which has been worsening over the past 3 months. It is localized and intensifies with pressure. It affects his ability to wear his prosthetic limb.

PAST MEDICAL HISTORY:
Active Problem List:
- Intravenous Drug Use (Substance Use Disorder) (diagnosed 2020-01-15)
- Opioid use disorder, severe, recurrent
- Asplenia
- Chronic pain syndrome (post-amputation and mechanical low back)
- History of motor vehicle accident sequelae

The patient has a history of a traumatic below-the-knee amputation of the same limb following a motor vehicle accident 6 months ago.

PAST SURGICAL HISTORY:
- Splenectomy (2 years ago following motor vehicle accident)
- Left below-knee amputation (6 months ago following traumatic injury)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Opioid use disorder in older brother
- Major depressive disorder in mother
- Hypertension in father

SOCIAL HISTORY:
Smoking: Current daily smoker, 1 pack per day. Alcohol: Occasional, 1-2 drinks per week. Occupation: Unemployed; formerly construction laborer.

PHYSICAL EXAMINATION:
On physical examination, a tender nodule is palpated at the distal end of the amputation site. No signs of infection are noted. The skin over the nodule is intact, and there is no erythema or warmth.

OTHER STUDIES:
An injection of lidocaine into the area provides immediate pain relief.

## Encounter 2: source 2021-03-15, practice 2022-06-18, ed, Emergency Medicine

Chief complaint: Worsening restlessness, insomnia, muscle aches, and gastrointestinal symptoms

ENCOUNTER NOTE
Date: 2021-03-15
Provider: Dr. James Wilson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Worsening restlessness, insomnia, muscle aches, and gastrointestinal symptoms

HISTORY OF PRESENT ILLNESS:
This 31-year-old male with severe recurrent opioid use disorder, asplenia, and chronic pain syndrome following amputation, seen in Jan 2020 for heroin use and July 2020 for prosthetic limb pain, is brought to the emergency department by his partner because of several days of worsening restlessness and inability to sleep. He reports muscle aches, abdominal cramping, and episodes of nausea and diarrhea. Over the past 2 days he has also experienced rhinorrhea, watery eyes, and frequent yawning. He feels extremely anxious and describes "goosebumps" on his skin that come and go.

PAST MEDICAL HISTORY:
Active Problem List:
- Intravenous Drug Use (Substance Use Disorder) (diagnosed 2020-01-15)
- Phrenic nerve injury (diagnosed 2020-07-15)
- Opioid use disorder, severe, recurrent
- Asplenia
- Chronic pain syndrome (post-amputation and mechanical low back)
- History of motor vehicle accident sequelae

PAST SURGICAL HISTORY:
- Splenectomy (2 years ago following motor vehicle accident)
- Left below-knee amputation (6 months ago following traumatic injury)

MEDICATIONS:
He takes no prescribed medications

ALLERGIES:
NKDA

FAMILY HISTORY:
- Opioid use disorder in older brother
- Major depressive disorder in mother
- Hypertension in father

SOCIAL HISTORY:
He does admit to previous use of prescription pain medications for a back injury but says he has been unable to obtain them recently

PHYSICAL EXAMINATION:
On examination, his pupils are markedly dilated, blood pressure is mildly elevated, and he appears uncomfortable and sweaty. There are no focal neurologic deficits.

## Encounter 3: source 2021-09-15, practice 2022-12-15, ed, Emergency Medicine

Chief complaint: Found unresponsive at home by friends

ENCOUNTER NOTE
Date: 2021-09-15
Provider: Dr. Aisha Johnson, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Found unresponsive at home by friends

HISTORY OF PRESENT ILLNESS:
The patient with recurrent opioid use disorder returns and is brought to the emergency department by friends after they found him unresponsive at home.

PAST MEDICAL HISTORY:
Active Problem List:
- Intravenous Drug Use (Substance Use Disorder) (diagnosed 2020-01-15)
- Phrenic nerve injury (diagnosed 2020-07-15)
- Opioid Withdrawal (diagnosed 2021-03-15)
- Opioid use disorder, severe, recurrent
- Asplenia
- Chronic pain syndrome (post-amputation and mechanical low back)
- History of motor vehicle accident sequelae

PAST SURGICAL HISTORY:
- Splenectomy (2 years ago following motor vehicle accident)
- Left below-knee amputation (6 months ago following traumatic injury)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Opioid use disorder in older brother
- Major depressive disorder in mother
- Hypertension in father

SOCIAL HISTORY:
His friends report that he has a history of heroin use and was recently released from a rehabilitation center.

PHYSICAL EXAMINATION:
On examination, the patient is somnolent with pinpoint pupils and a respiratory rate of 6 breaths per minute. His blood pressure is 90/50 mmHg, and pulse is 50 beats per minute. Oxygen saturation on room air is 85%.

LABORATORY DATA:
Arterial blood gas analysis reveals the following:

- pH: 7.25
- PaCO2: 60 mmHg
- PaO2: 50 mmHg
- HCO3-: 24 mEq/L

## Encounter 4: source 2022-05-15, practice 2023-08-15, ed, Emergency Medicine

Chief complaint: Found unresponsive in apartment by friends

ENCOUNTER NOTE
Date: 2022-05-15
Provider: Dr. Robert Kim, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Found unresponsive in apartment by friends

PAST MEDICAL HISTORY:
Active Problem List:
- Intravenous Drug Use (Substance Use Disorder) (diagnosed 2020-01-15)
- Phrenic nerve injury (diagnosed 2020-07-15)
- Opioid Withdrawal (diagnosed 2021-03-15)
- Heroin overdose (diagnosed 2021-09-15)
- Opioid use disorder, severe, recurrent
- Asplenia
- Chronic pain syndrome (post-amputation and mechanical low back)
- History of motor vehicle accident sequelae

PAST SURGICAL HISTORY:
- Splenectomy (2 years ago following motor vehicle accident)
- Left below-knee amputation (6 months ago following traumatic injury)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Opioid use disorder in older brother
- Major depressive disorder in mother
- Hypertension in father

SOCIAL HISTORY:
His friends report that he has a history of heroin use but had been in a rehabilitation program.

VITAL SIGNS:
His vital signs are: blood pressure 90/50 mmHg, heart rate 55 bpm, respiratory rate 6 breaths per minute, and temperature 35.6°C (96.1°F).

PHYSICAL EXAMINATION:
On examination, the patient is somnolent but responsive to painful stimuli. Pupils are pinpoint. The skin is cool and clammy. Auscultation of the lungs reveals decreased breath sounds bilaterally without crackles. The abdomen is soft with markedly decreased bowel sounds.

LABORATORY DATA:
A fingerstick glucose level is 110 mg/dL.

PLAN:
Naloxone is administered, and the patient's respiratory rate improves to 14 breaths per minute.

## Encounter 5: source 2024-01-15, practice 2025-04-21, ed, Emergency Medicine

Chief complaint: Agitation and sweating immediately following release from incarceration

ENCOUNTER NOTE
Date: 2024-01-15
Provider: Dr. Emily Rodriguez, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Agitation and sweating immediately following release from incarceration

HISTORY OF PRESENT ILLNESS:
The patient is a 34-year-old male with severe recurrent opioid use disorder, asplenia, and chronic pain syndrome status post amputation who returns following recent incarceration, having previously presented with opioid withdrawal in March 2021 and heroin overdoses in September 2021 and May 2022. He is brought to the emergency department by his friends. He was released from prison 6 hours ago, where he served a 2-year sentence for drug possession. His friends mention that he has a history of heroin use but claim he has been clean for the past year. His friends are concerned he might have used heroin shortly before his symptoms started. He complains of abdominal cramps, nausea, and an intense craving for opioids.

PAST MEDICAL HISTORY:
Active Problem List:
- Intravenous Drug Use (Substance Use Disorder) (diagnosed 2020-01-15)
- Phrenic nerve injury (diagnosed 2020-07-15)
- Opioid Withdrawal (diagnosed 2021-03-15)
- Heroin overdose (diagnosed 2021-09-15)
- Opioid use disorder, severe, recurrent
- Asplenia
- Chronic pain syndrome (post-amputation and mechanical low back)
- History of motor vehicle accident sequelae

PAST SURGICAL HISTORY:
- Splenectomy (2 years ago following motor vehicle accident)
- Left below-knee amputation (6 months ago following traumatic injury)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Opioid use disorder in older brother
- Major depressive disorder in mother
- Hypertension in father

SOCIAL HISTORY:
Smoking: Current daily smoker, 1 pack per day. Alcohol: Occasional, 1-2 drinks per week. Occupation: Unemployed; formerly construction laborer.

VITAL SIGNS:
His vital signs are: blood pressure 160/90 mmHg, heart rate 110 beats/min, respiratory rate 22 breaths/min, and temperature 37.8°C (100°F).

PHYSICAL EXAMINATION:
On examination, the patient is anxious, diaphoretic, and has dilated pupils.

LABORATORY DATA:
Urine toxicology is positive for opioids.

## Encounter 6: source 2024-07-15, practice 2025-10-25, icu, Critical Care

Chief complaint: High fever, chills, and purpuric rash

ENCOUNTER NOTE
Date: 2024-07-15
Provider: Dr. David Thompson, MD
Department: Critical Care
Visit Type: icu

CHIEF COMPLAINT:
High fever, chills, and purpuric rash

HISTORY OF PRESENT ILLNESS:
The patient is a 34-year-old male with severe recurrent opioid use disorder and asplenia who presents to the emergency department six months after his last visit for opioid withdrawal with a 12-hour history of high fever, chills, and a purpuric rash. He has not received any vaccinations since his splenectomy.

PAST MEDICAL HISTORY:
Active Problem List:
- Intravenous Drug Use (Substance Use Disorder) (diagnosed 2020-01-15)
- Phrenic nerve injury (diagnosed 2020-07-15)
- Opioid Withdrawal (diagnosed 2021-03-15)
- Heroin overdose (diagnosed 2021-09-15)
- Opioid use disorder, severe, recurrent
- Asplenia
- Chronic pain syndrome (post-amputation and mechanical low back)
- History of motor vehicle accident sequelae

PAST SURGICAL HISTORY:
- Splenectomy (2 years ago following motor vehicle accident)
- Left below-knee amputation (6 months ago following traumatic injury)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Opioid use disorder in older brother
- Major depressive disorder in mother
- Hypertension in father

SOCIAL HISTORY:
Smoking: Current daily smoker, 1 pack per day. Alcohol: Occasional, 1-2 drinks per week. Occupation: Unemployed; formerly construction laborer.

VITAL SIGNS:
His temperature is 39.5°C, blood pressure is 90/60 mmHg, heart rate is 120 beats per minute, and respiratory rate is 22 breaths per minute.

PHYSICAL EXAMINATION:
Physical examination reveals a lethargic patient with petechiae and purpura scattered over his limbs and trunk.

LABORATORY DATA:
Laboratory results are as follows:
- White blood cell count: 18,000/mm³
- Hemoglobin: 13.2 g/dL
- Platelet count: 80,000/mm³
- C-reactive protein: 24 mg/L (normal <10 mg/L)
- Prothrombin time: 12 seconds (control 11 seconds)
- Partial thromboplastin time: 30 seconds (control 28 seconds)

OTHER STUDIES:
Blood cultures are drawn,

PLAN:
He is started on broad-spectrum antibiotics.

## Encounter 7: source 2025-01-15, practice 2026-04-22, ed, Emergency Medicine

Chief complaint: Found unresponsive at home with concern for overdose

ENCOUNTER NOTE
Date: 2025-01-15
Provider: Dr. Maria Garcia, MD
Department: Emergency Medicine
Visit Type: ed

CHIEF COMPLAINT:
Found unresponsive at home with concern for overdose

HISTORY OF PRESENT ILLNESS:
The patient with severe recurrent opioid use disorder and multiple prior overdoses is brought to the emergency department by his friends after they found him unresponsive at home. His friends report concern for overdose.

PAST MEDICAL HISTORY:
Active Problem List:
- Intravenous Drug Use (Substance Use Disorder) (diagnosed 2020-01-15)
- Phrenic nerve injury (diagnosed 2020-07-15)
- Opioid Withdrawal (diagnosed 2021-03-15)
- Heroin overdose (diagnosed 2021-09-15)
- Septic shock (diagnosed 2024-07-15)
- Opioid use disorder, severe, recurrent
- Asplenia
- Chronic pain syndrome (post-amputation and mechanical low back)
- History of motor vehicle accident sequelae

PAST SURGICAL HISTORY:
- Splenectomy (2 years ago following motor vehicle accident)
- Left below-knee amputation (6 months ago following traumatic injury)

ALLERGIES:
NKDA

FAMILY HISTORY:
- Opioid use disorder in older brother
- Major depressive disorder in mother
- Hypertension in father

SOCIAL HISTORY:
He has a history of heroin use

VITAL SIGNS:
His respiratory rate is 6 breaths per minute, blood pressure is 110/70 mmHg, pulse is 60 beats per minute, and temperature is 35.6°C (96.1°F). Oxygen saturation on room air is 85%.

PHYSICAL EXAMINATION:
On examination, the patient is somnolent but arousable to painful stimuli. Pupils are pinpoint but react to light. The patient's respiratory efforts are shallow, and auscultation of the lungs reveals decreased breath sounds bilaterally without wheezes or crackles.

LABORATORY DATA:
A fingerstick glucose level is 98 mg/dL.
