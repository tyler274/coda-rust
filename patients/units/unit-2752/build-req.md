# Build requirements: unit-2752 (Morgan Hale, MRN00002752)

Source: synthetic_hospital patient 2752, in `source.md` beside this file. This file and
the source are the only authority for clinical facts. Every diagnosis, medication,
result, procedure and specialist finding in the chart must trace to one of:

- **S** a fact in source.md, carried as listed below;
- **R** a resolution below, where the source contradicts itself;
- **A** an addition below, listed one by one;
- **P** the primary-care layer described at the end.

Nothing else. No diagnosis, medication, test result, procedure, family member or
life event that is not on these lists. Texture (wording, scheduling, refills, portal
traffic, who signed what) is free, as long as it carries no new clinical fact.

Re-voice everything. The source notes are exam vignettes: never copy a sentence, and
never carry a source clinician's name, its "Active Problem List" dumps, its
"(diagnosed ...)" stamps, or its quiz-style labels.

## Who she is (S)

Morgan Hale, born 1984-09-07, female, White, not Hispanic or Latino, English, Blue
Cross PPO. International travel agent (much of the work at a desk, with frequent
long-haul flights). Current smoker, 1 pack per day for about 20 years (20 pack-years).
Alcohol occasional, 1 to 2 drinks a month. No known drug allergies. No prior surgeries.
Family history: mother had a deep vein thrombosis at 45; father has hypertension; a
paternal uncle had a pulmonary embolism.

Joined the practice 2023-08-22 (new-patient visit, A1), moving her care from a prior
primary-care office. She brought: migraine (without aura, R4), mild major depressive
disorder (no medication), menstrual irregularity, and tobacco use disorder (cigarettes).
Medication on joining: combined oral contraceptive pill, ethinyl estradiol 0.035 mg /
norgestimate 0.25 mg, 1 tablet daily (R3), taken for irregular menses for about 6 years
by then (R2).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2023-08-22 | practice | new-patient visit (A1): history as above; PHQ-9 in the mild range; smoking cessation advised, she declines medication for now; screening labs (P) |
| 2024-09-24 | practice | annual physical (P); screening mammogram ordered (age 40) |
| 2025-04-14 | outside | regional ED: sudden shortness of breath and lightheadedness walking from her car into the hospital, with palpitations and air hunger not eased by rest; no fever, cough or sputum; just back from a 12-hour flight from Australia. T 37.0 °C, BP 106/68, HR 114, RR 28, SpO2 91% on room air. Tachypneic, mildly anxious; lungs clear. ABG pH 7.48, PaCO2 31 mmHg (R5); D-dimer 2.4 mg/L FEU (R5). Chest x-ray normal. ECG sinus tachycardia only. CT pulmonary angiography: right segmental pulmonary embolism. **Pulmonary embolism diagnosed.** **Apixaban started** (A2: 10 mg twice daily for 7 days, then 5 mg twice daily, planned 3 months); discharged home from the ED. OCP continued (left as the source has it). The ED record reaches the practice late, 2025-05-06, after a records request (texture: gaps); the practice first hears of it from her portal message 2025-04-16 and the apixaban fill |
| 2025-04-23 | practice | post-ED visit (P): breathing back to baseline; on apixaban; PE on the problem list |
| 2025-07-22 | practice | follow-up (A2): 3 months of apixaban completed, no symptoms; **apixaban stopped**; PE resolved, history of PE recorded |
| 2025-10-01 | outside | regional ED: shortness of breath and palpitations starting an hour earlier climbing stairs, worse over the hour, now lightheaded and anxious; no chest pain, cough, fever or recent illness; back from a 10-hour flight from South America. On the OCP. RR 28, HR 112, SpO2 92% on room air. Anxious, mildly diaphoretic; no JVD; normal heart sounds; lungs clear without wheeze, crackles or rhonchi; calves soft and non-tender, no asymmetry. ABG pH 7.47 (R5); D-dimer 3.1 mg/L FEU (R5). Chest x-ray: no infiltrate, effusion or consolidation. ECG sinus tachycardia, otherwise unremarkable. CT pulmonary angiography (A3): acute filling defect in a left lower lobe segmental pulmonary artery. **Recurrent pulmonary embolism diagnosed**; **apixaban restarted** (A2: same regimen, planned 3 months); discharged home from the ED. OCP continued (left as the source has it). The ED record arrives 2025-10-21 |
| 2025-10-08 | practice | post-ED visit (P): on apixaban; influenza vaccine |
| 2026-01-13 | practice | follow-up (A2): 3 months of apixaban completed; **apixaban stopped** (an ordinary decision, not the best one; no note debates it); PE resolved |
| 2026-04-08 to 2026-04-11 | outside | regional hospital admission, internal medicine: sudden shortness of breath and chest tightness worse on inspiration, starting while shopping; feels like her prior episodes but harder to breathe; no fever, sputum or prior lung disease, no asthma or COPD; no leg swelling, mild right calf soreness the day before that went away; flew Los Angeles to Tokyo one week earlier. Only medication the combined OCP. T 36.8 °C, BP 108/72, HR 116, RR 30, SpO2 91% on room air. Anxious, mildly tachypneic, using accessory muscles; regular rhythm with a loud second heart sound; lungs clear. ABG pH 7.49, PaCO2 29 mmHg (R5). Chest x-ray unremarkable. ECG sinus tachycardia. CT pulmonary angiography: wedge-shaped perfusion defect in the right lower lobe. **Acute pulmonary embolism (third) diagnosed.** Supplemental oxygen by nasal cannula, off by discharge (A4). **Apixaban started, indefinite** (A4: 10 mg twice daily for 7 days, then 5 mg twice daily); **combined OCP stopped** at discharge, estrogen to be avoided (A4). Discharge summary received 2026-04-13 |
| 2026-04-16 | practice | post-discharge visit (P): breathing improving; lists updated (apixaban indefinite, OCP off, long-term anticoagulant use recorded); gynecology referral for non-estrogen management of irregular menses and contraception (A5); smoking cessation advised again |
| 2026-07-01 | practice | last visit (P): follow-up on indefinite apixaban, no bleeding, no recurrence; CBC and BMP drawn, normal; she has not yet scheduled with gynecology (referral left open) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Ages and years follow the seed and the re-dated timeline. The October 2025 ED
  note's "40-year-old" becomes 41. The source's "January 2020" and "July 2020"
  embolisms are the April 2025 and October 2025 events.
- **R2.** The OCP is "8 years" at the second ED visit and "started 6 months ago" at the
  admission. Resolution: about 8 years by October 2025 (since about 2017), continuous;
  "6 months ago" is not carried.
- **R3.** The source names the pill but no strength. Resolution: ethinyl estradiol
  0.035 mg / norgestimate 0.25 mg, 1 tablet daily.
- **R4.** "Migraine disorder" is coded as migraine without aura (no aura described
  anywhere), episodic, managed without a prescription; no migraine drug in the chart.
- **R5.** The source gives the ABG and D-dimer as "elevated pH", "decreased PaCO2",
  "D-dimer elevated" without numbers. Resolution: the numbers in the timeline above.
  No PaO2 or other gas values are carried.
- **R6.** Pack-years are 20 in the profile and "15" at the admission. Resolution:
  20 pack-years (about 1 pack per day for 20 years); 15 is not carried.
- **R7.** "Desk job with prolonged sitting" at the admission and "international travel
  agent" are one job: a travel agent who works at a desk and flies often.
- **R8.** The source's comorbidity labels "Delayed management of life-threatening
  respiratory failure" and "Long-term oral contraceptive use" are not diagnoses on the
  chart; the first is dropped, the second is carried only as the medication history.
  "Pulmonary embolism" and "Acute Pulmonary Embolism" are the three PE events above.
- **R9.** The source's lists show pulmonary embolism as a standing problem from the
  first ED visit on. Resolution: each PE is diagnosed at its event and resolved at the
  practice visit that stops the 3-month course; history of PE is recorded after the
  first; the third stays active through today.
- **Left as the source has it:** the estrogen-containing pill continues through the
  first two embolisms; no one at the ED or the practice stops it or comments on it
  until the April 2026 discharge. No thrombophilia testing, no hematology referral, no
  lower-extremity duplex, no echocardiogram despite the loud second heart sound, no
  antidepressant and no migraine medication. Nobody remarks on any of these.

## Additions (A): physician to confirm

- **A1.** 2023-08-22 new-patient visit (the seed gives her 3 years with the practice;
  the source starts at an ED).
- **A2.** Anticoagulation for the first two embolisms, which the source never mentions:
  apixaban 10 mg twice daily for 7 days, then 5 mg twice daily, a 3-month course each
  time, stopped at the practice follow-ups 2025-07-22 and 2026-01-13. This keeps the
  admission's "only medication is the OCP" true.
- **A3.** CT pulmonary angiography at the October 2025 ED visit, so the recurrent PE the
  admission note asserts has evidence: acute filling defect in a left lower lobe
  segmental pulmonary artery.
- **A4.** Standard inpatient care after the third embolism: supplemental oxygen by
  nasal cannula, weaned off before discharge; apixaban started as above with indefinite
  duration; combined oral contraceptive stopped with advice to avoid estrogen.
- **A5.** Gynecology referral at the 2026-04-16 visit for non-estrogen options for
  irregular menses and contraception; still open today.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may carry
  the descriptors its kind always carries, but nothing that adds a territory, a severity
  grade or a symptom, or changes the diagnosis or plan.
- Records from her prior primary-care office (received 2023-09) may mention routine
  screening done there, with normal results: a normal Pap with negative HPV in 2021.
- Height 165 cm, weight about 63 to 65 kg, BP normal (about 110 to 120 / 70 to 78).
- Visits every few months around the events; a yearly physical; CBC, CMP, lipids, TSH at
  joining or physicals, normal; CBC and BMP while on apixaban, normal; urine pregnancy
  test negative where an ED or admission would run one.
- PHQ-9 in the mild range (5 to 9) at visits that screen; mood discussed, no medication,
  she declines counseling referral.
- Smoking cessation advice at most visits; she declines medication; status stays current
  smoker, 1 pack per day.
- Screening mammogram 2024-10 normal (BI-RADS 1). HIV and hepatitis C screening once,
  negative. Influenza vaccine each fall, COVID vaccines, Tdap once.
- Refills (OCP until April 2026, apixaban), pharmacy and prior-authorization traffic,
  portal messages about results, refills, travel and scheduling, a missed visit,
  records requests.
- No visit booked after today.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002752 on 2026-07-01. Practice contacts after that day do not
happen; outside care after it still does, and its records are filed as received without
changing the practice's lists. No visit is booked after today.
