# Build requirements: unit-2212 (Stanley Kaminski, MRN00002212)

Source: synthetic_hospital patient 2212, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its unfinished "Based on the CHA2DS2-VASc score,"
line.

## Who he is (S)

Stanley Kaminski, born 1957-01-15, male, White, not Hispanic or Latino, English,
Medicare, lives in Williamsburg (01096). Retired postal worker. Former smoker, 30
pack-years, quit in August 2023 (R2). Alcohol: occasional, 1 to 2 drinks a week. No
known drug allergies. Surgical history: laparoscopic cholecystectomy 2005, left
inguinal hernia repair 2012. Family history: father MI at 61, mother type 2 diabetes,
brother ischemic stroke at 65.

Joined the practice 2023-11-07 (A1, new patient visit). He brought: hypertension
(since about 2011), type 2 diabetes (since about 2016), hyperlipidemia (since about
2013), nicotine dependence in remission (quit August 2023); metformin 1000 mg twice
daily, lisinopril 20 mg daily, atorvastatin 40 mg daily. No atrial fibrillation and
no anticoagulant at joining.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2023-11-07 | practice | new patient visit (A1): history as above; BP above goal; labs, vaccines, lung screening ordered (A3) |
| 2024-01 to 2025-07 | practice | routine diabetes/BP care (P); LDCT lung screening yearly (A3); AAA ultrasound once (P) |
| 2025-08-14 (Thu) | practice | intermittent palpitations and mild exertional dyspnea for two months; no chest pain, dizziness or syncope. BP 142/88, HR 98 irregularly irregular, SpO2 97% RA. Lungs clear, no leg edema. Hgb 14.2, platelets 250, INR 1.0, Cr 1.1 (R4). ECG: atrial fibrillation, ventricular rate 100. **Atrial fibrillation diagnosed.** CHA2DS2-VASc 3 (hypertension, diabetes, age 65-74); apixaban recommended, **he declines anticoagulation** (R3). TSH ordered (P, normal), echocardiogram and cardiology referral (A2). Rate under 110, no rate-control drug |
| 2025-08-27 | practice-ordered | transthoracic echo (A2): normal LV size and function, LVEF 60%, no significant valve disease, normal left atrial size |
| 2025-09-17 | outside | cardiology consult letter (A2): back in sinus rhythm on ECG; paroxysmal atrial fibrillation; recommends apixaban 5 mg twice daily; he declines again (R3); follow-up offered |
| 2025-10-12 (Sun) | outside | ED (R1 date): sudden palpitations and shortness of breath while gardening, began 2 hours before arrival; no chest pain, dizziness or syncope. BP 90/60, HR 170, RR 22, SpO2 94% RA. JVP not raised, no crackles, no leg edema. Hgb 14, WBC 7.5, platelets 250, Na 140, K 4.2, Cr 1.0, glucose 180. ECG: AF with rapid ventricular response, no ST or T-wave changes. **Synchronized electrical cardioversion** to sinus rhythm (A4); after it BP 124/76, HR 82 sinus. Discharged home the same day; ED advises anticoagulation, he declines (R3). Record received by the practice 2025-10-14 |
| 2025-10-16 (Thu) | practice | post-ED visit (P): regular rhythm, no symptoms; anticoagulation revisited, still declines (R3); flu and COVID vaccines |
| 2026-01-21 | outside | cardiology follow-up letter (A2): sinus rhythm, no further episodes reported; apixaban recommended again and declined (R3); follow-up in 6 months |
| 2026-07-04 (Sat) | outside | ED: sudden left-sided weakness; no headache, visual change or speech difficulty. BP 180/95, HR 78, afebrile. Left facial droop, left arm and leg power 3/5, sensation intact, no cerebellar signs. ECG sinus rhythm (A5). Non-contrast head CT: no hemorrhage, no mass effect. Fasting lipids (next morning, inpatient): TC 220, HDL 35, LDL 160, TG 250. Not given thrombolysis: arrived beyond the window (R5). Admitted |
| 2026-07-04 to 2026-07-08 | outside | stroke admission (A5): MRI brain acute lacunar infarct, right posterior limb of internal capsule, 12 mm, no hemorrhage; carotid duplex without hemodynamically significant stenosis either side; telemetry sinus rhythm. **Lacunar stroke diagnosed.** He now accepts anticoagulation: **apixaban 5 mg twice daily started 2026-07-07**; **atorvastatin 40 stopped, atorvastatin 80 mg daily started**; **amlodipine 5 mg daily added**; lisinopril and metformin continue. Outpatient PT/OT. Discharge left arm and leg power 4-/5. Summary received 2026-07-09 |
| 2026-07-14 (Tue) | practice | post-discharge visit (A5): left arm and leg 4/5, mild left facial asymmetry, walks without device; BP 138/82; taking apixaban |
| 2026-08-18 | outside | neurology stroke clinic letter (A5): left arm and leg 4+/5, improving; continue apixaban and atorvastatin 80; LDL goal under 70; repeat lipids |
| 2026-08-21 | practice | lipid panel and hepatic panel drawn, **pending** at today (P) |
| 2026-09-22 | practice | booked: annual wellness visit and stroke follow-up (P) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source's dates put the gardening onset in December in western
  Massachusetts. The source also says the ED visit came two months after the AF
  diagnosis. Resolution: encounter 0 moves from 2025-10-16 to 2025-08-14 and
  encounter 1 from 2025-12-07 to 2025-10-12 (an autumn Sunday in the garden). The
  stroke stays 2026-07-04.
- **R2.** "Quit 2 years ago" at encounter 0 (August 2025) puts the quit in August
  2023, three months before he joins. At joining the problem is recorded as nicotine
  dependence, cigarettes, in remission (ICD-10 F17.211); the profile's "sustained
  remission" is true by the 2025 encounters. No cessation drug.
- **R3.** The source's encounter-0 plan stops at the CHA2DS2-VASc score, and every
  later source list (including the stroke admission's home medications) has no
  anticoagulant or rate-control drug. Resolution: the score is 3; each clinician
  (PCP August and October 2025, cardiology September 2025 and January 2026, ED
  October 2025) recommends apixaban and he declines it each time, without a reason
  beyond not wanting a blood thinner. No beta blocker or calcium-channel blocker for
  rate. He accepts apixaban only after the stroke.
- **R4.** The source's eGFR of 60 cannot go with a creatinine of 1.1 in a 68-year-old
  man (the lab's CKD-EPI value is about 72). Creatinine 1.1 is carried; the eGFR is
  the lab's derived value and the 60 is not carried. No kidney disease diagnosis.
- **R5.** The source gives no acute stroke treatment. Resolution: symptoms began
  late morning and he arrived mid-afternoon, beyond the thrombolysis window; no
  thrombolysis or thrombectomy.
- The source's encounter-1 problem list stamps AF "with rapid ventricular response"
  as the diagnosis. The problem list carries one entry, atrial fibrillation (I48.91),
  from 2025-08-14; RVR describes the October episode.
- The source's stroke-encounter social history (30 pack-years only) is consistent
  with the profile; nothing changes.
- **Left as the source has it:** blood pressure stays in the 140s/80s on lisinopril
  20 mg alone and LDL runs about 155 to 160 on atorvastatin 40 mg from 2023 to the
  stroke. Nobody adds a blood pressure drug or raises the statin before July 2026,
  and no note remarks on it.

## Additions (A): physician to confirm

- **A1.** 2023-11-07 new-patient visit (encounter 0 reads as established care).
- **A2.** For new AF: transthoracic echo 2025-08-27 (normal, LVEF 60%, normal left
  atrium) and an outside cardiology consult 2025-09-17 with a follow-up 2026-01-21,
  findings as in the timeline.
- **A3.** Low-dose CT lung cancer screening (30 pack-years, quit 2023, age 66-69):
  2024-01-16, 2025-01-21, 2026-02-10, each Lung-RADS 1, no nodules.
- **A4.** Synchronized cardioversion in the ED for AF with RVR and hypotension (onset
  2 hours), with the post-cardioversion vitals above.
- **A5.** The usual course after the source's lacunar stroke: the admission, MRI,
  carotid duplex, telemetry, apixaban, atorvastatin 80, amlodipine 5, PT/OT, the
  practice post-discharge visit and the neurology clinic letter, with findings
  exactly as in the timeline. CHA2DS2-VASc is 5 after the stroke.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not
  mention. An abnormal finding appears only as the source or this file gives it. A
  report may carry the descriptors its kind always carries, but nothing that adds a
  territory, a severity grade or a symptom, or changes the diagnosis or plan.
- Records from the prior practice: medication and problem history as above, a
  screening colonoscopy in 2019 with no polyps.
- Levels: height 178 cm, weight about 92 kg throughout; office BP about 142/88 before
  the stroke and about 136/82 after amlodipine; A1c about 7.3 to 7.6 on metformin;
  creatinine about 1.0 to 1.1; lipids about HDL 35, LDL 158, TG 245 (total cholesterol derived) before the
  stroke; urine albumin/creatinine normal; TSH normal.
- Diabetes and blood pressure visits about every 3 to 6 months with A1c and BMP,
  lipids and urine albumin yearly, foot exams normal. Yearly diabetic eye exam
  letters from an optometrist: no retinopathy.
- One abdominal aortic ultrasound (male ever-smoker 65-75): normal aortic caliber.
- Prevention: influenza each fall, COVID updated vaccines, PCV20, Shingrix series,
  RSV once, Tdap; hepatitis C and HIV screening once, nonreactive; PHQ-2 negative.
- Refills, pharmacy traffic, portal messages about results, refills and scheduling, a
  missed visit, the hospital's transition call.
- Booked visit after 2026-08-27: annual wellness and stroke follow-up 2026-09-22.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.


## Last seen

The practice last saw MRN00002212 on 2026-07-14. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. A visit is booked after today.
