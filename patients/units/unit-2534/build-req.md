# Build requirements: unit-2534 (Brandon Slattery, MRN00002534)

Source: synthetic_hospital patient 2534, in `source.md` beside this file. This file and
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
"(diagnosed 2020-07-15)" stamps, or its unfinished lines ("An abdominal CT scan with
intravenous contrast is performed").

## Who he is (S)

Born 1995-10-29, male, White, not Hispanic or Latino, English, Blue Cross Blue Shield
(private), lives in Amherst. Regional sales representative (he drives a sales
territory). Current smoker, 10 cigarettes a day since about 2018 (R1). Social alcohol,
2 to 3 drinks a week on weekends. Allergy: penicillin (maculopapular rash). Surgical
history: tonsillectomy at age 10 (about 2006), arthroscopic meniscal repair of the left
knee at age 24 (2020). Family history: father hypertension and myocardial infarction at
55; mother hypothyroidism; one sister, healthy. Marital status single.

Joined the practice on 2023-09-12 (A1), age 27, from a prior primary-care practice,
bringing essential hypertension (diagnosed 2021) on lisinopril 10 mg daily and
generalized anxiety disorder (diagnosed 2020) on sertraline 50 mg daily (A2). Both drugs
continue unchanged through today.

## Records (seed: gaps)

Some outside care never sends its letter or sends it late, and the practice learns of it
from the patient first; the problem list lags. Per contact:

- Prior practice records: requested at joining, arrive 2023-10-03.
- October 2024 admission: the patient calls on 2024-10-30; a brief trauma-center
  discharge summary arrives only on 2024-12-03 after a records request. No operative,
  imaging or ICU notes ever arrive.
- June 2025 admission: the patient messages on 2025-06-19; the ED record and discharge
  summary arrive together on 2025-07-08. The vascular surgery 1-month letter arrives
  2025-07-29.
- December 2025 admission: the patient calls on 2025-12-26; the ED record with the CT
  report arrives 2026-01-14. The discharge summary, operative note and the urology
  letter about catheter removal never arrive; the practice knows of the repair and the
  catheter removal from the patient.
- June 2026 surveillance CT: the report does not reach the practice by today. The
  patient reports he was told it was stable; a records request is outstanding (the one
  open item today).
- The hemorrhagic shock stays on the practice problem list until the 2025-09-16
  physical (the list lags).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2023-09-12 | practice | new-patient visit (A1): history as above; on lisinopril 10 mg and sertraline 50 mg (A2); BP at goal on lisinopril; smoking counseling; screening labs (P) |
| 2023-10-03 | outside | prior practice records arrive: HTN since 2021 on lisinopril 10 mg; GAD since 2020 on sertraline 50 mg; tonsillectomy, left knee meniscal repair 2020; penicillin rash; Tdap 2019 (P) |
| 2024-03-19 | practice | HTN and GAD follow-up (P) |
| 2024-09-10 | practice | annual physical (P) |
| 2024-10-17 to 2024-10-28 | outside | **trauma center admission (ICU), encounter 1.** Unrestrained driver, high-speed collision, major driver-side intrusion. On arrival alert, pale, anxious. BP 85/50, HR 128, RR 22, SpO2 96% on room air. Pelvis unstable on exam; large left flank ecchymosis; abdomen distended and tender. Hemoglobin 8.5 g/dL, platelets 95,000, PT 18 s, aPTT 40 s, INR 1.6. FAST positive for free abdominal fluid. **Traumatic hemorrhagic shock with acute coagulopathy** and **contusion of the left flank** diagnosed. Course (A3): pelvic binder, massive transfusion protocol with blood products, coagulopathy corrected, surgical ICU, discharged home 2024-10-28 on his home medications, no new drugs |
| 2024-10-30 | practice | patient phones to report the admission (P) |
| 2024-11-05 | practice | post-hospital visit (P): flank bruise fading; hemoglobin 11.4 (A4); weight 80 kg (P) |
| 2024-12-03 | outside | trauma-center discharge summary arrives (content as encounter 1 row) |
| 2025-02-11 | practice | follow-up: CBC normal (A4); flank contusion resolved on the list (P) |
| 2025-06-12 to 2025-06-17 | outside | **trauma center ED and admission, encounter 2.** Restrained driver, high-speed collision, airbags deployed; acute chest pain and shortness of breath. BP 145/90, HR 110, RR 22. Decreased breath sounds on the left; sternal contusion (bruise over the sternum). Hemoglobin 13.2 g/dL, hematocrit 39%, WBC 12,000, platelets 250,000. Chest x-ray: widened mediastinum, left pleural effusion, no rib fractures. FAST negative. ECG: sinus tachycardia, no ischemia (R2). **Blunt thoracic aortic injury** diagnosed (A5: CT angiography and TEVAR); discharged 2025-06-17 on home medications |
| 2025-06-19 | practice | portal message from patient about the admission (P) |
| 2025-06-24 | practice | post-hospital visit (P) |
| 2025-07-08 | outside | ED record and discharge summary arrive (content as encounter 2 row and A5) |
| 2025-07-17 | outside | vascular surgery 1-month visit with CT angiography (A5); letter arrives 2025-07-29 |
| 2025-09-16 | practice | annual physical (P); hemorrhagic shock resolved on the list |
| 2025-12-19 to 2025-12-23 | outside | **trauma center ED and admission, encounter 3.** Restrained driver, head-on collision; severe lower abdominal pain and unable to void for 3 hours since the crash. Alert, in distress. BP 110/70, HR 102, RR 22, temperature 37.1 C. Suprapubic tenderness and bruising; urethral (Foley) catheter placed with no urine returned. BUN 36 mg/dL, creatinine 1.8 mg/dL, hemoglobin 13.2 g/dL, WBC 12,000, potassium 4.5. CT abdomen and pelvis with IV contrast (A6). **Intraperitoneal bladder rupture** diagnosed; repaired (A6); discharged 2025-12-23 with the catheter in place on home medications |
| 2025-12-26 | practice | patient phones (P) |
| 2025-12-30 | practice | post-hospital visit with catheter in place: creatinine 0.98, BUN normal (A7) |
| 2026-01-06 | outside | urology: cystogram without leak, catheter removed (A6); no letter ever arrives; patient reports it by portal 2026-01-07 |
| 2026-01-14 | outside | ED record with CT report arrives |
| 2026-03-17 | practice | HTN and GAD follow-up (P); voiding normally; bladder rupture resolved on the list |
| 2026-06-18 | outside | vascular surgery 1-year CT angiography (A5); report not received by today |
| 2026-06-23 | practice | portal message: patient says he was told the scan was stable; records request sent 2026-06-24, re-sent 2026-07-21 (P) |
| 2026-09-22 | practice | booked annual physical (P) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Every source note says "10 cigarettes per day for 6 years", unchanged across
  14 months. Resolution: he started about 2018, so the October 2024 trauma record says
  6 years and later notes count from 2018 (about 7 years by late 2025). 10 cigarettes a
  day throughout; he never quits.
- **R2.** The source lists "sinus tachycardia secondary to burns" as a comorbidity, but
  no burn appears anywhere. Resolution: the June 2025 ECG shows sinus tachycardia after
  the crash, read as the ED read it; the burn attribution is dropped. Sinus tachycardia
  and leukocytosis (WBC 12,000 in June and December 2025) stay findings in the ED
  records and are not added to the practice problem list. The left flank contusion is a
  diagnosis of the October 2024 admission.
- **R3.** The source profile lists all three injuries from the first encounter.
  Resolution: each is diagnosed at its own encounter (shock October 2024, aortic injury
  June 2025, bladder rupture December 2025); earlier notes never mention a later one.
- **R4.** The source ends encounter 3 with a CT "performed" and no result. Resolution: the
  CT result is A6.
- **R5.** Hemoglobin 13.2 and WBC 12,000 appear at both 2025 encounters; carried as given
  at each.
- **Left as the source has it:** the October 2024 records name no bleeding source and no
  pelvic fracture for the unstable pelvis and positive FAST; nobody at the practice asks
  and no note comments. Lisinopril stays 10 mg after the aortic injury with office
  pressures in the high 120s to low 130s over low 80s; no one changes it. Smoking
  continues; counseling is offered and no cessation drug is started. Three crashes as
  the driver in 14 months draw only seatbelt and driving counseling, no further workup.

## Additions (A): physician to confirm

- **A1.** The 2023-09-12 new-patient visit (the seed gives about 3 years with the
  practice; he is already a patient at encounter 1).
- **A2.** Onset years for his two chronic conditions and their drugs (HTN 2021 with
  lisinopril 10 mg, GAD 2020 with sertraline 50 mg), from the prior practice, so the
  lists have a start.
- **A3.** The October 2024 course the source's shock needs: pelvic binder for the unstable
  pelvis, massive transfusion protocol, correction of the coagulopathy, surgical ICU
  stay, discharge 2024-10-28 home on his usual medications.
- **A4.** Recovery of the post-hemorrhage anemia: hemoglobin 11.4 g/dL on 2024-11-05;
  CBC normal by 2025-02-11 and after.
- **A5.** Standard care for the blunt thoracic aortic injury: CT angiography of the chest
  on 2025-06-12 confirms injury of the proximal descending thoracic aorta with a
  surrounding mediastinal hematoma and a left pleural effusion, no rib fractures;
  thoracic endovascular aortic repair (TEVAR, stent graft) the same day through right
  femoral access; one night in the ICU; the effusion is not drained. 1-month CT angiography (2025-07-17): stent
  graft in good position, no endoleak, mediastinal hematoma smaller, left effusion
  resolved. Vascular surgery advises blood pressure control, stopping smoking, and a
  1-year CT angiogram, done 2026-06-18 (result not received).
- **A6.** The encounter-3 CT and standard repair: CT abdomen and pelvis with IV contrast
  and a CT cystogram through the catheter shows contrast leaking from the bladder dome
  into the peritoneal cavity with free fluid in the pelvis, aortic stent graft unchanged,
  no solid-organ injury, no acute fracture. Same-day laparotomy with two-layer repair of
  the bladder; discharged with the urethral catheter; cystogram at urology on 2026-01-06
  shows no leak and the catheter is removed.
- **A7.** The source's BUN 36 and creatinine 1.8 come from the leaking urine: creatinine
  0.98 and BUN normal on 2025-12-30, normal thereafter.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may carry
  the descriptors its kind always carries, but nothing that adds a territory, a severity
  grade or a symptom, or changes the diagnosis or plan.
- Records from the prior practice may mention routine screening done there, with normal
  results (Tdap 2019).
- Height 180 cm; weight about 84 kg at joining, 80 kg at the 2024-11-05 visit after the
  ICU stay, back to about 84 kg by mid-2025 and after.
- Blood pressure on lisinopril about 128/82 at joining and through 2026, with ordinary
  wobble. GAD-7 scores 5 to 8 most visits, 11 at the 2024-11-05 visit and 9 at the
  2025-06-24 visit; PHQ-9 2 to 6. Sertraline unchanged. AUDIT-C 2 to 3.
- HTN and GAD visits about every 6 months, a physical each fall, BMP or CMP yearly,
  CBC, lipids at joining and in 2025, HIV and hepatitis C screening once (2023),
  nonreactive.
- Influenza vaccine each fall (2023-10-10, 2024-09-10, 2025-09-16); he declines COVID
  vaccine. Smoking and seatbelt counseling.
- Refills of lisinopril and sertraline, pharmacy traffic, portal messages about results
  and scheduling, records requests, one cancelled visit (2026-05-12, work travel).
- Booked visit after 2026-08-27: annual physical 2026-09-22.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.


## Last seen

The practice last saw MRN00002534 on 2024-09-10. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. A visit is booked after today.
