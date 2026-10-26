# unit-2753 build requirements (synthetic_hospital patient 2753)

Every clinical fact in this chart traces to **S** (the source as carried here), **R**
(a numbered resolution), **A** (a numbered addition) or **P** (the primary-care
layer below), and nothing else. Texture (voice, scheduling, refills, portal
traffic, admin, missed visits) is free and carries no new clinical fact.
Re-voice everything: no source sentence, clinician name, problem-list dump,
"(diagnosed ...)" stamp or quiz text is copied. Clinicians are named by role only.
The practice's today is 2026-08-27T12:00 Eastern.

## Who she is (S)

- **Identity (seed, unchanged):** Tracy Bianchi, MRN00002753, female, born
  1975-01-14, Florence MA 01062, White, not Hispanic or Latino, English, Medicaid.
  Phone +1-860-555-0199, tracy.bianchi@example.test. Single; lives alone in an
  apartment in Florence. Her sister is her primary support person. In 2023 she has
  a partner who does not live with her (S: brought her to the ED).
- **Occupation:** unemployed on disability leave; formerly an administrative
  assistant.
- **Tobacco:** light cigarette smoker, about 1 pack per week (see R7 for the
  timeline of quitting and resuming).
- **Alcohol:** social, 1-2 cocktails on weekends. No drug use.
- **Allergies/intolerances (known at joining):** NSAIDs (ineffective, poor GI
  tolerance; intolerance); gabapentin (ineffective; intolerance). No other drug
  allergies.
- **Surgical history at joining:** multiple small bowel resections for Crohn's
  disease (before joining, dates not known); tunneled central venous catheter for
  TPN (before joining). Laparoscopic cholecystectomy and right knee arthroscopy
  happen during the chart (R3, R4).
- **Family history:** mother with major depressive disorder and type 2 diabetes;
  father with Crohn's disease; sister with generalized anxiety disorder (primary
  support person); maternal aunt with deep vein thrombosis.
- **Psychiatric history (S):** two prior major depressive episodes, one in her late
  20s and one in her mid-30s, both treated with medication. No history of mania,
  hypomania or psychosis before 2024.

### Joining the practice

She **establishes care 2020-07-14** (A1), about six weeks before the first source
encounter, which is a psychiatry follow-up (she was already an outside
psychiatry patient). Prior primary care records arrive 2020-07-10.

Brought on joining (problems): Crohn's disease; short bowel syndrome; dependence
on total parenteral nutrition through a tunneled central venous catheter; major
depressive disorder, recurrent, severe (current episode); generalized anxiety
disorder; chronic low back pain (R2).

Brought on joining (drugs, all already taking; psychiatric drugs prescribed by her
outside psychiatrist, TPN by her outside GI/intestinal rehabilitation team):
- sertraline 100 mg by mouth daily
- bupropion XL 150 mg by mouth daily
- alprazolam 0.25 mg by mouth as needed for anxiety; she sometimes takes 2 tablets
  at night
- diphenhydramine 25 mg by mouth at night as needed for sleep (over the counter)
- ethinyl estradiol/norethindrone combined oral contraceptive, 1 tablet daily
  (R10: norethindrone acetate 1 mg / ethinyl estradiol 0.02 mg)
- multivitamin 1 tablet daily
- TPN infused over 12 hours nightly through the central line
- vedolizumab 300 mg IV every 8 weeks at the outside GI infusion center (A10)

Not known at joining (start at their encounters): temporal lobe epilepsy, the
superior gluteal nerve palsy (R5), acute cholecystitis, pulmonary embolism, major
depressive disorder with psychotic features, hyponatremia, central pontine
myelinolysis.

## Timeline

| Date | Where | What happens |
|---|---|---|
| 2020-07-10 | Practice (records in) | A1: prior primary care records received: Crohn's with multiple small bowel resections, short bowel syndrome on nightly TPN via tunneled line, recurrent MDD, GAD, chronic low back pain with prior NSAID and gabapentin trials (both ineffective, NSAIDs poorly tolerated) and a prior course of physical therapy she found unhelpful; weight 62.0 kg in 2019-11; normal cervical cytology 2019. |
| 2020-07-14 | Practice | **Establish care** (A1). Low mood since about April 2020 (R1), weight down (57.4 kg, height 165.0 cm), poor sleep. PHQ-9 21, GAD-7 12. Labs CBC, BMP, TSH all normal (these are the "most recent labs" the psychiatry letter cites). Hep C and HIV screens negative; lipid panel and A1c normal (P). PCP contacts her psychiatrist; earlier psychiatry follow-up arranged. |
| 2020-08-24 | Outside psychiatry (letter) | S enc 0: follow-up for persistent depressive symptoms: 4-month history of persistent sadness, loss of interest, difficulty concentrating (R1), significant weight loss, insomnia, feelings of worthlessness. Two prior episodes (late 20s, mid-30s) needing medication. Family history of depression in her mother. Denies current substance use. Exam unremarkable. Labs CBC, TSH, BMP within normal limits. Plan: continue current regimen, follow up in psychiatry. No medication change. |
| 2020-12-06 | Practice (phone) | P: sister calls about an episode that day (R6). |
| 2020-12-07 | Practice | P: visit for the episode: while sitting at the kitchen table she abruptly stopped talking and stared for about 45 seconds, unresponsive, no memory of it, confused for several minutes afterward; sister saw no shaking and no fall/loss of consciousness. Feels "slowed down", trouble concentrating. Neuro exam non-focal. Labs CBC, BMP, TSH, B12 drawn (all normal). Urgent neurology referral. No driving until seen. |
| 2020-12-08 | Outside neurology (letter) | S enc 1: evaluation of the staring spell (as above). Meds: bupropion, sertraline, alprazolam as needed, OTC diphenhydramine for sleep (R8). Told neurology she does not drink alcohol, and admits sometimes taking more than one alprazolam at night when she cannot sleep (R9). Vitals normal. Oriented, flat affect, pauses often; neuro exam non-focal. CBC, BMP, TSH, B12 normal. **Diagnosis: temporal lobe epilepsy** (focal seizure with impaired awareness) (R6). EEG ordered (A2). Seizure precautions and driving counseling. A9: **lamotrigine started** with the titration to 100 mg twice daily. |
| 2020-12-22 | Outside neurology (EEG report) | A2: routine EEG: normal background; intermittent left temporal sharp waves. Consistent with left temporal lobe epilepsy. |
| 2021-03-08 | Outside neurology (letter) | A9: EEG reviewed; no further events; at lamotrigine 100 mg twice daily since 2021-01-19, no rash; trough level 4.8 mcg/mL. Yearly follow-up. |
| 2021-04-20 | Practice | P: back pain worse; referral to pain management. Quit smoking about two months ago (R7). Labs drawn: CBC normal, ESR 20 mm/hr, TSH 2.5 mIU/L, 25-OH vitamin D 30 ng/mL (S enc 2 values, drawn here). Pain 8/10. |
| 2021-05-10 | Outside pain management (letter) | S enc 2: worsening low back pain she calls unbearable despite trying "everything"; prior NSAIDs and gabapentin without real relief; prior physical therapy she calls useless. Hopeless about improvement, tearful, frustrated that no one can help. Former light smoker, quit about 3 months ago (previously 1 pack/week); social alcohol. Exam: lumbar paraspinal tenderness, no neurologic deficit. Labs: CBC normal, ESR 20, TSH 2.5, vitamin D 30. Impression: chronic mechanical low back pain. Plan: no interventional target; home exercise and activity; continue mood care with her psychiatrist; return as needed (R11). No new drug. |
| 2021-09-21 | Practice | P: right leg instability; orthopedic referral. Mild limp. |
| 2021-10-25 | Outside orthopedics (letter) | S enc 3: 6-month history of difficulty walking and instability standing on the right leg; right hip "gives way" on stairs. Exam: slight limp; on standing on the right leg the left hemipelvis drops below horizontal (positive Trendelenburg sign on the right); weakness of right hip abduction; no sensory loss; reflexes intact. **Diagnosis: right superior gluteal nerve palsy** (R5). Plan: physical therapy for hip abductor strengthening (A3). |
| 2021-11 to 2022-01 | Outside PT | A3: PT evaluation 2021-11-03 and a course of hip abductor strengthening; discharged 2022-01-12 with a home program; instability less frequent, limp persists mildly. |
| 2022-03-14 | Outside neurology (letter) | A9: yearly review; no seizures; lamotrigine 100 mg twice daily unchanged. |
| 2022-11-04 | Outside ED, then admission | S enc 4: age 47. 24 hours of constant severe right upper quadrant pain radiating to the back, nausea, low-grade fever; no change in bowel habits, no urinary symptoms. T 38.2 C, HR 102, BP 135/85. RUQ tenderness, positive Murphy sign. WBC 14.0 K/uL, Hgb 13.2 g/dL, platelets 250 K/uL, total bilirubin 1.8 mg/dL, alkaline phosphatase 120 U/L, AST 50 U/L, ALT 45 U/L, amylase 90 U/L, lipase 85 U/L. RUQ ultrasound: gallbladder wall thickening and pericholecystic fluid, no gallstones. **Diagnosis: acute (acalculous) cholecystitis** (R3). A4: admitted; IV antibiotics; **laparoscopic cholecystectomy 2022-11-05**; pathology: acute cholecystitis, no stones; discharged 2022-11-07, no new drugs. |
| 2022-11-14 | Practice | P: post-discharge visit: incisions healing; hepatic panel and CBC normal. Acute cholecystitis resolved; status post cholecystectomy. |
| 2023-04-18 | Practice | R7: she has started smoking again, about 1 pack per week. Counseling. |
| 2023-10-24 | Outside orthopedic surgery | R4: right knee arthroscopy (outside surgeon; indication not in the records received). |
| 2023-10-29 | Outside ED, then admission | S enc 5: age 48. Brought by her partner with sudden shortness of breath 5 days after right knee arthroscopy, recovering at home with limited mobility. Smokes 1 pack per week; occasional weekend cocktail. BP 125/85, HR 110. A5: CT pulmonary angiogram: acute pulmonary emboli in segmental branches of the right lower lobe; no right heart strain; lungs otherwise clear. CBC and BMP normal. **Diagnosis: acute pulmonary embolism, provoked (post-operative)** (R4). **Start enoxaparin 60 mg subcutaneously every 12 hours** (A5); **combined oral contraceptive stopped** (A6). Observed overnight; discharged 2023-10-31. |
| 2023-11-06 | Practice | P: post-hospital visit; enoxaparin plan for 3 months; OCP stopped confirmed; smoking counseling. |
| 2023-12-04 | Outside neurology (letter) | A9: OCP stopped 2023-10-29; lamotrigine trough 8.1 mcg/mL, no dizziness, double vision or unsteadiness; dose unchanged. |
| 2024-01-29 | Practice | A5: 3 months of anticoagulation complete: **enoxaparin stopped**; PE resolved, now history of PE. |
| 2024-09-17 | Practice | P: no-show for a scheduled follow-up. |
| 2024-10-25 | Outside psychiatric ED, then admission | S enc 6: age 49. Two weeks of acute behavioral change: will not leave her apartment, believes neighbors are discussing her sins through the walls and that God is punishing her for being a terrible mother (R12); cries often, feels "unfixable", believes she deserves jail; poor appetite, early morning waking, low energy; passive thoughts of death, no active plan. Sister: several prior depressive episodes, none this severe; no known mania, hypomania or psychosis. **Not taking any psychiatric medications** (R8: she stopped sertraline, bupropion and alprazolam on her own in about 2024-08). No substance use; unemployed, lives alone. Vitals and physical exam unremarkable. Flat affect, avoids eye contact; no signs of dementia on cognitive testing; neuro exam intact. Urine drug screen negative. **Diagnosis: major depressive disorder, recurrent, severe, with psychotic features.** A7: admitted to inpatient psychiatry; discharged 2024-11-12 on **sertraline 100 mg daily (restarted), bupropion XL 150 mg daily (restarted), aripiprazole 5 mg daily (new), alprazolam 0.25 mg as needed (restarted)**. |
| 2024-11-18 | Practice | P: post-discharge visit with her sister; meds reconciled. |
| 2025-05-12 | Outside psychiatry (letter) | A7: psychotic symptoms resolved, mood improved: **aripiprazole stopped**; MDD now recurrent, in partial remission; continue sertraline and bupropion. |
| 2025-09-16 | Practice | R7: quit smoking about a month ago (mid-August 2025). BMP normal (sodium normal). |
| 2025-11-07 | Outside hospital admission | S enc 7: age 50. Found confused and lethargic (by her sister). **Serum sodium 112 mEq/L. Diagnosis: hyponatremia.** Treated with intravenous fluids; over the next 48 hours sodium rises to **129 mEq/L** (2025-11-09). Former light smoker, quit about 3 months ago; social alcohol. On hospital day 3 (2025-11-09) she becomes unresponsive and moves to the ICU: opens eyes to pain, blinks to command, otherwise motionless, no speech, no limb movement; pupillary reflexes intact. A8: hyponatremia workup consistent with SIADH (serum osmolality 238 mOsm/kg, urine osmolality 420 mOsm/kg, urine sodium 54 mEq/L); **sertraline stopped** as a possible contributor. **MRI brain 2025-11-12: central pontine T2/FLAIR hyperintensity with restricted diffusion, sparing the periphery; no extrapontine lesions — central pontine myelinolysis (diagnosis).** Supportive ICU care without intubation; TPN continued. Gradual return of eye, then limb movement and speech. To the medical floor 2025-11-21; discharged to inpatient rehabilitation 2025-12-05. Hyponatremia resolved by discharge. |
| 2025-12-05 to 2026-01-23 | Outside inpatient rehabilitation | A8: PT, OT and speech therapy. Discharged home 2026-01-23 walking with a cane, independent in self-care, mild dysarthria, mild gait unsteadiness; home PT/OT. Meds at discharge: bupropion XL 150 mg, alprazolam 0.25 mg as needed, diphenhydramine 25 mg at night as needed, multivitamin, TPN nightly. |
| 2026-01-28 | Practice | P: transitional care visit; BMP normal. |
| 2026-04-13 | Outside neurology (letter) | A8: post-myelinolysis follow-up: improving; mild dysarthria and mild gait unsteadiness persist; cane outdoors only; no seizures since 2020. Continue home exercises. |
| 2026-07-14 | Practice | P: preventive visit. |
| 2026-09-15 | Practice (booked) | P: routine follow-up booked. |

## Resolutions (R)

1. **"Difficulty concentrating at work."** She is on disability leave and not
   working. The chart carries difficulty concentrating, without "at work". The
   4-month history dates the episode's onset to about 2020-04. "Significant weight
   loss" is carried as weight falling from 62.0 kg (prior records, 2019-11) to
   57.4 kg at joining.
2. **Chronic low back pain diagnosed 2020-09-15 (enc 2) yet on every earlier list,
   with NSAID/gabapentin intolerances listed from the first encounter.** The pain
   management note itself describes long-standing pain already treated with NSAIDs,
   gabapentin and physical therapy. The chart has the back pain, the NSAID and
   gabapentin trials, the intolerances and the prior PT all before joining (prior
   records); the pain management consult names it chronic mechanical low back pain.
   The "diagnosed" stamp is not carried.
3. **Acute cholecystitis in a patient listed as already status post cholecystectomy.**
   The ultrasound shows a gallbladder (wall thickening, pericholecystic fluid, no
   stones). The cholecystectomy is the treatment of this episode (2022-11-05); "status
   post laparoscopic cholecystectomy" applies only from then. The ED note's statement
   that she was already post-cholecystectomy is not carried. Acalculous cholecystitis
   in a long-term TPN patient is the form named.
4. **Pulmonary embolism, "history of post-operative PE" and the knee arthroscopy on
   every list from 2020.** There is one PE: the provoked post-operative PE of
   2023-10-29, five days after a right knee arthroscopy on 2023-10-24. "History of
   pulmonary embolism (provoked, post-operative)" is what it becomes after
   anticoagulation ends (2024-01-29). No earlier PE; the ED's "history of
   postoperative PE" is not carried. "Status post right knee arthroscopy" applies
   only from 2023-10-24. The arthroscopy's indication is not in the records the
   practice receives.
5. **"Phrenic nerve injury" (stamped at the orthopedic encounter) with findings of
   hip abductor weakness and a positive right Trendelenburg sign.** Nothing in the
   source involves the diaphragm. The orthopedic findings are those of right superior
   gluteal nerve dysfunction; the chart's diagnosis is right superior gluteal nerve
   palsy (ICD-10 G57.81). Phrenic nerve injury is not carried anywhere.
6. **Temporal lobe epilepsy as a "history" in enc 0-1 but stamped diagnosed at
   enc 1.** TLE is not known at joining and is diagnosed by neurology on 2020-12-08
   from the staring spell. The episode is placed on Sunday 2020-12-06 ("earlier in the
   week" of the Tuesday neurology visit). The psychiatry letter of 2020-08-24 does not
   mention epilepsy.
7. **Smoking: "former, quit 3 months ago" in 2021, 2021-10, 2022-11 and 2025-11, but
   smoking 1 pack per week in 2023-10.** Timeline: current light smoker (1 pack/week)
   at joining; quits about 2021-02; former smoker through 2022-11; resumed about 1
   pack per week by 2023-04 (recorded then) and still smoking at the PE; quits again
   about 2025-08; former smoker since. The repeated "3 months ago" in 2021-10 and
   2022-11 is not carried literally.
8. **Psychiatric medications.** At enc 1 the home medication list omits sertraline,
   alprazolam and diphenhydramine while the narrative names them: she is taking all
   of them (list dropout, not carried). At enc 6 she "takes no psychiatric
   medications" while the list shows them: she stopped sertraline, bupropion and
   alprazolam on her own in about 2024-08 and did not tell the practice; the
   psychiatry ED's list is stale. They restart at the 2024-11-12 psychiatric
   discharge. The enc 7 list (sertraline, bupropion, alprazolam, diphenhydramine,
   OCP, multivitamin, TPN) is what she takes at admission except the OCP, which an
   outside list copies stale (stopped 2023-10-29); sertraline stops at that
   admission (A8). The enc 5 list omits the multivitamin (stale omission; she takes
   it throughout).
9. **Alcohol.** She tells neurology she does not drink; every other record says
   social, 1-2 cocktails on weekends. Kept as what she told each clinician. The
   psychiatry ED's "no substance use" stands as written.
10. **OCP strength not given.** Carried as norethindrone acetate 1 mg / ethinyl
    estradiol 0.02 mg tablet, 1 daily.
11. **Pain management plan is unstated.** The letter closes with no procedure, no
    new drug, home exercise, coordination with psychiatry and follow-up as needed.
12. **Delusional guilt "for being a terrible mother."** The source never says whether
    she has children. The chart carries the delusion as her own words of guilt about
    her mothering, and no record adds or denies children.
13. **Ages and source text years.** Ages follow the birth date (45 at 2020-08, 47,
    48, 49, 50 at the ED/ICU encounters). "Psychiatry January 2020 / Neurology April
    2020" read as 2020-08 and 2020-12; "pain management September 2020" reads as
    2021-05; "previously evaluated in 2020" stands.
14. **Encounter 7 setting.** The ICU encounter is outside hospital care (a regional
    hospital), received as a discharge summary, not a practice visit.

**Left as the source has it (no comment in the chart):**
- Bupropion continues despite epilepsy.
- No analgesic is prescribed for the back pain after pain management.
- No change to the antidepressant regimen at the 2020-08 psychiatry visit despite
  persistent symptoms.
- No VTE prophylaxis is documented around the knee arthroscopy; no thrombophilia
  workup after the PE.
- The source's sodium correction (112 to 129 mEq/L in 48 hours) stands as given.

## Additions (A)

1. **Establish care 2020-07-14 and prior records 2020-07-10.** The first source
   encounter is a follow-up; she joined about six years before today (seed). Prior
   records carry the history listed above (weight 62.0 kg in 2019-11; normal cervical
   cytology 2019).
2. **Routine EEG 2020-12-22** (left temporal sharp waves, normal background), the
   one test supporting the TLE diagnosis the source asserts.
3. **Physical therapy for the superior gluteal nerve palsy** (2021-11-03 to
   2022-01-12), the usual treatment; right hip abduction 3+/5 at evaluation, 4/5 at
   discharge; outcome: instability less frequent, mild limp persists.
4. **Admission and laparoscopic cholecystectomy 2022-11-04 to 2022-11-07**; IV
   piperacillin-tazobactam, blood cultures no growth; pathology acute cholecystitis
   without stones; discharged on her home medications with acetaminophen as needed
   for incision pain, no antibiotics. Needed for R3.
5. **PE confirmation and treatment:** CT pulmonary angiogram (segmental right lower
   lobe emboli, no right heart strain), CBC and BMP normal, overnight observation to
   2023-10-31, enoxaparin 60 mg SC every 12 hours for 3 months (2023-10-29 to
   2024-01-29). LMWH because oral absorption is unreliable with short bowel.
6. **Combined oral contraceptive stopped 2023-10-29** (estrogen after a PE). No other
   contraceptive drug is started.
7. **Treatment of psychotic depression (enc 7 says "recent treatment"):** inpatient
   psychiatry 2024-10-25 to 2024-11-12; sertraline 100 mg, bupropion XL 150 mg and
   alprazolam 0.25 mg as needed restarted; aripiprazole 5 mg daily added; stopped by
   outside psychiatry 2025-05-12 with psychotic symptoms resolved and depression in
   partial remission.
8. **Course of hyponatremia and myelinolysis to today:** SIADH workup values above;
   sertraline stopped and not restarted; MRI brain 2025-11-12 as above; ICU without
   intubation; bupropion held while she could not swallow, resumed after a passed
   swallow evaluation (lamotrigine and vedolizumab: A9, A10); floor 2025-11-21; inpatient rehabilitation 2025-12-05 to 2026-01-23;
   at rehab discharge walking with a cane, independent in self-care, mild dysarthria,
   mild gait unsteadiness; neurology 2026-04-13 improving with mild dysarthria and
   mild gait unsteadiness, cane outdoors only. Depression afterward is treated with
   bupropion alone; outside psychiatry (2026-03-23) adds weekly supportive therapy
   and does not restart an SSRI. Home health PT/OT after rehab (start of care
   2026-01-27); her sister stays with her the first week home.
9. **Antiseizure treatment for the temporal lobe epilepsy the source asserts**, run by
   outside neurology. Lamotrigine (chosen for her depression; no hyponatremia risk,
   unlike oxcarbazepine or carbamazepine) starts at the 2020-12-08 diagnosis: 25 mg
   daily for 2 weeks, 50 mg daily for 2 weeks, 50 mg twice daily for 1 week, 75 mg
   twice daily for 1 week, then 100 mg twice daily from 2021-01-19, the maintenance
   dose to today. Rash counseling at start; no rash. Trough level 4.8 mcg/mL
   (reference 2.5-15.0) at neurology 2021-03-08. Neurology reviews her yearly (letters
   2021-03-08, 2022-03-14, 2023-12-04 and 2026-04-13 reach the chart); no seizures
   after 2020-12-06. After the OCP stops (2023-10-29, the hospital notifies
   neurology), trough 8.1 mcg/mL on 2023-12-04 with no dizziness, double vision or
   unsteadiness; dose unchanged. She keeps taking it when she stops her psychiatric
   drugs in 2024-08. In the 2025-11 ICU stay it is given crushed by nasogastric tube
   while she cannot swallow, by mouth after the passed swallow evaluation; no
   interruption. Refills via neurology.
10. **Crohn's maintenance therapy**, run by the outside GI/intestinal rehabilitation
   team: vedolizumab 300 mg IV every 8 weeks at the GI infusion center, started by
   that team in 2018 (prior records) and already taking at joining; gut-selective,
   chosen for her permanent central line. No drug-specific monitoring beyond liver
   tests with the routine TPN labs (normal). Continued through every admission; the
   dose due during the 2025-11 to 2026-01 hospital and rehabilitation stay is
   deferred and given 2026-02-10, then every 8 weeks again. Crohn's stays quiet
   (no flares; colonoscopy 2022-03 normal).

## Primary-care layer (P)

- **Visits:** about every 2-4 months for depression/anxiety, back pain, gait, TPN
  and line coordination; post-ED/post-discharge visits after each hospital stay;
  a preventive visit yearly in July (2021-2026). In-person; occasional telehealth.
- **Weight levels (kg, set as state; the scale reads around them):** 57.4 (2020-07), 59.0 (2021-01), 60.2 (2021-07), 60.8 (2022-07),
  59.1 (2022-11), 60.5 (2023-07), 60.0 (2024-07), 56.2 (2024-11), 58.9 (2025-07),
  52.6 (2026-01), 55.8 (2026-07). Height 165.0 cm. Blood pressure normal throughout.
- **Screens:** PHQ-9 21 (2020-07), 18 (2020-09), 15 (2021-01), 17 (2021-04),
  13 (2021-07), 10 (2022-07), 9 (2023-07), 7 (2024-07), 16 (2024-11), 12 (2025-01),
  8 (2025-07), 11 (2026-01), 9 (2026-07). GAD-7 12 (2020-07), 10 (2021-07),
  8 (2022-07), 8 (2023-07), 7 (2024-07), 13 (2024-11), 9 (2025-07), 9 (2026-07).
  Pain severity 8/10 (2021-04), 6-7/10 at later back-pain mentions. AUDIT-C 2 at
  physicals. No suicidal ideation on PCP screens except as the 2024 psychiatric ED
  records.
- **Lab levels set for code (all normal):** TSH 2.5 (source value), total
  cholesterol 182, LDL 104, HDL 58, triglycerides 96, A1c 5.3, B12 540, vitamin D 30
  and ESR 20 (source values), ferritin 64, iron 82, TIBC 318, magnesium 1.9.
- **Labs (normal unless the timeline says otherwise):** CBC, CMP and magnesium every
  6-12 months for TPN/short bowel (copies of the home infusion labs go to GI);
  25-OH vitamin D and B12 yearly; iron studies yearly; lipid panel and A1c 2020,
  2023, 2026 (mother with type 2 diabetes); hepatitis C and HIV screens once
  (2020, negative); hepatic panel and CBC after cholecystectomy; BMP after each
  sodium-relevant event (2025-09 normal; 2026-01, 2026-03 and 2026-07 normal).
  CMP and magnesium also at some routine visits (2021-01, 2022-01, 2024-04) and a
  lab-only draw 2023-02 (CMP, Mg, CBC), all normal.
- **Prevention:** screening mammogram yearly-to-biennially, all BI-RADS 1
  (2020-10, 2021-11, 2022-12, 2024-02, 2025-03, 2026-08); cervical cytology with HPV
  2022-07 normal, HPV negative; colonoscopy by outside GI 2022-03 normal (high-risk
  screening for Crohn's; repeat in 5 years). Influenza each fall (2024 dose given
  late, 2024-11-18); COVID-19 primary series 2021 (pharmacy), booster 2021-11,
  bivalent 2022, updated 2023, 2024, 2025; Tdap 2021-07; PCV20 2022-07 (smoker);
  Shingrix 2025-01 and 2025-03.
- **Outside follow-up letters (no new facts):** GI/intestinal rehabilitation yearly
  (TPN regimen unchanged, line without infection, vedolizumab continued (A10), no flares);
  psychiatry follow-ups (2021-06, 2024-12, 2025-05, 2026-03) continuing the resolved
  regimen; neurology letters as A9.
- **Psychiatry cadence:** she sees outside psychiatry about every 3 months; only
  some letters reach the chart.
- **Admin texture:** Medicaid prior authorization for enoxaparin and TPN
  supplies, home health certification after rehab, records requests, refill
  requests (OCP until 2023-10; other refills via psychiatry/GI/neurology), portal messages
  about results and scheduling, one no-show (2024-09-17).
- **Today:** nothing open except a booked routine follow-up on 2026-09-15 and the
  2026-08-04 mammogram result already communicated.
- No acute illnesses, injuries, new diagnoses or life events beyond the timeline.
