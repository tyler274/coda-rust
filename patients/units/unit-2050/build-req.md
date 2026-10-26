# unit-2050 build requirements

Every clinical fact in this chart traces to **S** (the source as carried here), **R** (a
numbered resolution), **A** (a numbered addition) or **P** (the primary-care layer), and
nothing else. Texture (voice, scheduling, refills, portal traffic, admin, missed visits)
is free and carries no clinical fact. Re-voice everything: no source sentence, clinician
name, problem-list dump, "(diagnosed ...)" stamp or quiz text is copied. Clinicians are
named by role; outside places are "a regional hospital", "an outside infectious disease
clinic" and the like.

## Who he is (S)

- Rodney Ferraro, MRN00002050, male, born 1979-10-20, Amherst MA 01002, English,
  White, not Hispanic or Latino, Medicaid. Phone +1-646-555-0131,
  rodney.ferraro@example.test. Single, lives alone in an Amherst apartment (texture).
- Occupation: construction laborer, currently unemployed (unemployed throughout the
  chart; no reason recorded).
- Tobacco: current smoker, 1 pack per day for 20 years (20 pack-years at joining; the
  count is carried as 20 throughout, not recalculated).
- Alcohol: occasional, 1-2 drinks per month.
- Drugs: opioid use disorder with intravenous use (R3).
- Allergies: no known drug allergies.
- Surgical history: open reduction internal fixation of the right femur (2020, R1);
  incision and drainage of a cutaneous abscess of the left forearm (2021, R1).
- Family history: father had a myocardial infarction at 58 and hypertension; mother has
  type 2 diabetes; brother has opioid use disorder.
- **Joins the practice 2022-09-13** (establish-care visit, P; the seed gives 4 years with
  the practice). He brings:
  - Problems: opioid use disorder, severe; chronic hepatitis C; essential hypertension;
    major depressive disorder, recurrent, moderate; nicotine dependence (cigarettes).
  - Drugs (all historical on joining): buprenorphine/naloxone 8 mg/2 mg sublingual film
    daily, prescribed by an outside office-based addiction treatment program (R4);
    naloxone 4 mg nasal spray as needed for overdose reversal; amlodipine 5 mg daily;
    sertraline 50 mg daily. The practice takes over amlodipine, sertraline and naloxone
    refills; buprenorphine stays with the program for the whole chart.

## Timeline

| Date | Where | What happens |
|---|---|---|
| 2022-09-13 | Practice, establish care (P) | History as above. BP about 136/86 on amlodipine; weight about 80.3 kg, height 178 cm. PHQ-9 11. Labs: CMP, CBC, lipid panel, A1c, HIV screen nonreactive, **HCV RNA 1,840,000 IU/mL (A1)**, hepatitis B surface antigen and surface antibody both negative (A2). |
| 2022-09-21 | Outside records (P) | Prior primary care office's records: the same problems and drugs, surgical history, no allergies. |
| 2022-10 to 2022-11 | Practice nurse visits (P) | Influenza and COVID vaccines; **hepatitis B vaccine (Heplisav-B) two doses (A2)**. |
| 2023-09 | Practice, annual physical (P) | PCV20 and Tdap. Routine labs normal. |
| 2024-10 | Practice, annual physical (P) | Age 45: FIT kit given; FIT negative in 2024-11. |
| 2025-03-21 (Fri) | Practice, same-day visit (A3) | Two weeks of fever and fatigue and new low back pain; no trauma or heavy lifting. Temp 38.6 C, HR 104, BP 112/72. Sent to the emergency department. |
| **2025-03-22 to 2025-04-04** | Outside: regional hospital, internal medicine admission (S, encounter 0) | Fever, fatigue, new low back pain for two weeks; denies trauma or heavy lifting; history of intravenous drug use. Temp 38.9 C, BP 110/70, HR 105, RR 20. Holosystolic murmur loudest at the left lower sternal border radiating to the right sternal border; tender raised lesions on the finger and toe pads. Hemoglobin 10.2 g/dL (normocytic), WBC 12.0, ESR 85 mm/hr; blood cultures drawn. **Blood cultures grow methicillin-susceptible Staphylococcus aureus in 2 of 2 sets; repeat cultures 2025-03-24 no growth (A4).** **Transthoracic echo: 0.9 cm mobile vegetation on the aortic valve, mild aortic regurgitation, no abscess, LVEF 60% (A4).** **Diagnosed: infective endocarditis of the aortic valve.** **Start IV cefazolin 2 g every 8 hours for 6 weeks from the first negative culture, through 2025-05-05, via PICC with home infusion after discharge (A4).** Home drugs continued. Back pain settles on treatment (A4). |
| 2025-04-08 | Practice, post-discharge visit (P) | CBC: **hemoglobin 11.4 (A5)**; BMP normal. Weight about 77.0 kg. |
| 2025-05-06 | Outside ID clinic (A4) | Cefazolin completed 2025-05-05; PICC removed. No further antibiotics. **Stop cefazolin.** |
| **2025-06-15 to 2025-06-24** | Outside: regional hospital ED, then admission (S, encounter 1; admission A6) | Sudden severe left flank pain with nausea and vomiting for 6 hours; intermittent fevers and night sweats for two weeks. Temp 38.7 C (101.7 F), BP 145/90, HR 102. New grade 3/6 systolic murmur at the left lower sternal border; left costovertebral angle tenderness. Urinalysis: red blood cells and protein, no casts. WBC 15.0, hemoglobin 13.2, platelets 250, creatinine 1.2, BUN 21. **CT abdomen/pelvis with contrast: wedge-shaped area of decreased enhancement in the left renal cortex. Diagnosed: renal infarction.** **Admission (A6): blood cultures grow methicillin-susceptible S. aureus in 2 of 2 sets (relapse); TTE: 0.7 cm aortic valve vegetation, mild aortic regurgitation, no abscess, LVEF 60%; cardiac surgery consulted, no surgery recommended; repeat cultures 2025-06-17 no growth; restart IV cefazolin 2 g every 8 hours for 6 weeks through 2025-07-28 via PICC with home infusion.** He reports injecting in the weeks before (R3). |
| 2025-06-30 | Practice, post-discharge visit (P) | BMP: **creatinine 1.1 (A7)**, which stays the level thereafter. |
| 2025-07-29 | Outside ID clinic (A6) | Course completed 2025-07-28, PICC removed. **Stop cefazolin.** |
| 2025-11 | Practice, annual physical (P) | Age 46. Routine labs. |
| **2025-12-13 to 2025-12-30** | Outside: regional hospital ICU, then floor (S, encounter 2) | Age 46 (R5). Two weeks of fever, chills and new headache; 10-pound weight loss over the same two weeks; sudden left-sided weakness and slurred speech. Temp 38.9 C, HR 110, BP 125/70. Murmur consistent with mitral regurgitation; Roth spots on fundoscopy. Blood cultures grow Staphylococcus aureus (**methicillin-susceptible, A8**). Echo (TTE): vegetations on the native mitral valve with moderate mitral regurgitation (**aortic valve without vegetation, LVEF 60%, A8**). Non-contrast CT head: no hemorrhage. **A8: MRI brain: several small acute infarcts in the right frontal and parietal cortex, consistent with septic emboli; no hemorrhage or abscess. Diagnosed: embolic cerebral infarction (R6) and mitral regurgitation. Weakness and speech back to baseline by 2025-12-20. Cardiac surgery consulted: surgery not recommended at this time. IV cefazolin 2 g every 8 hours for 6 weeks through 2026-01-24. Discharged 2025-12-30 to a skilled nursing facility to finish antibiotics.** |
| 2026-01-24 | Outside: skilled nursing facility (A8) | Cefazolin completed, PICC removed, discharged home. **Stop cefazolin.** |
| 2026-01-28 | Practice, post-discharge visit (P) | Weight about 73.3 kg (the 10-pound loss from about 77.5 kg in November). Neuro exam nonfocal. Murmur of mitral regurgitation. CBC: **hemoglobin 12.1 (A9)**; BMP with creatinine 1.1. Embolic infarction moved to history of stroke without residual deficit. |
| 2026-02-24 | Outside neurology clinic (A8) | No residual deficit; no further neurology follow-up needed. |
| 2026-03-11 | Outside cardiology clinic (A10) | Follow-up TTE: no vegetation, moderate mitral regurgitation, LVEF 60%. Repeat echo in about a year. |
| 2026-06 | Practice visit (P) | Weight back to about 78 kg. |
| 2026-09-23 | Practice, booked (P) | Follow-up visit. |

## Resolutions (R)

1. **Surgical-history years.** The source lists the femur ORIF in 2015 and the forearm
   abscess drainage in 2021, already in the history at its first encounter. On the re-dated
   timeline the ORIF moves to 2020. The 2021 drainage would move to 2026, after the
   encounters that list it, so it stays 2021, a year before he joins. Both are carried as
   history from the prior practice, with no further detail.
2. **Endocarditis as a primary diagnosis from the start.** The profile lists endocarditis and
   renal infarction as if known throughout. Endocarditis is first diagnosed 2025-03-22 and
   renal infarction 2025-06-15; neither is on any list before. The source's "(diagnosed
   ...)" stamps are not carried.
3. **"Opioid use disorder, severe, active" while on buprenorphine, and "intravenous drug
   use".** He takes buprenorphine/naloxone from an outside program and still injects
   intermittently: at joining he reports occasional injecting; the 2025 admissions record
   injecting in the weeks before each one. The intravenous use is carried as the route of his
   opioid use disorder, not as a separate problem. After the December 2025 admission he
   reports no injecting to today (A11).
4. **Who prescribes buprenorphine.** The source does not say. An outside office-based
   addiction treatment program prescribes it for the whole chart; practice lists carry it as
   historical. The dose stays 8 mg/2 mg daily throughout, as every source encounter has it.
5. **Age at the ICU encounter.** The source says 45; on the seed birth date he is 46 on
   2025-12-13. The ICU record says 46. The other two encounters (45) stand.
6. **Valve and the neurological event.** The first admission is aortic valve endocarditis by
   the source's diagnosis; its examiner's holosystolic left lower sternal border murmur and the
   ED's new 3/6 systolic murmur are kept as each examiner wrote them. The December echo finds
   vegetations on the native mitral valve with moderate regurgitation; that episode is a
   third S. aureus endocarditis now involving the mitral valve, with the aortic valve clear.
   The sudden left-sided weakness and slurred speech are recorded as an embolic cerebral
   infarction complicating the endocarditis (the source's "with complications"); at the
   January visit it becomes history of stroke without residual deficits. A physician should
   confirm this reading.
7. **Profile medication list copied to every encounter.** All four home drugs are on
   before joining and continue through the chart; nothing in the source stops one.
8. **"Blood cultures: pending"** at encounter 0 resolves as A4. The source's statement that
   "the patient suddenly develops" weakness in the ICU is carried as the event that brought
   him in, not as an in-ICU event.
9. **Hemoglobin 10.2 then 13.2 three months apart** is carried as recovery from the
   inflammatory anemia of the first episode (A5 between them).

**Left as the source has it** (kept, without comment): no hepatitis C treatment is started
or discussed; no smoking-cessation drug; no statin; no endocarditis antibiotic prophylaxis
for dental work; no spine imaging for the March back pain; buprenorphine dose never
changed; sertraline dose never changed.

## Additions (A)

1. **HCV RNA 1,840,000 IU/mL** at establish care: supports the listed chronic hepatitis C.
   Liver enzymes normal.
2. **Hepatitis B surface antigen and antibody negative; Heplisav-B two doses** (2022-10,
   2022-11): routine for a person with HCV who injects drugs.
3. **Same-day practice visit 2025-03-21** that sent him to the ED: temp 38.6 C, HR 104,
   BP 112/72, murmur heard.
4. **First admission's results and course:** MSSA in 2 of 2 sets, clearance 2025-03-24, TTE
   (0.9 cm aortic vegetation, mild AR, no abscess, LVEF 60%), IV cefazolin 2 g q8h through
   2025-05-05 via PICC and home infusion, discharge 2025-04-04, ID visit 2025-05-06 with PICC
   removal, back pain settling on treatment.
5. **Hemoglobin 11.4** at the 2025-04-08 post-discharge visit.
6. **Admission after the June ED visit** (the ICU note calls the renal infarction a
   hospitalization): MSSA 2 of 2 sets, repeat cultures 2025-06-17 no growth, TTE (0.7 cm aortic vegetation, mild AR, no abscess,
   LVEF 60%), cardiac surgery: no surgery, IV cefazolin through 2025-07-28, discharge
   2025-06-24, ID visit 2025-07-29 with PICC removal.
7. **Creatinine 1.1** from 2025-06-30 on (after the renal infarction; earlier levels normal).
8. **December admission's course:** MSSA susceptibility; TTE aortic valve clear, LVEF 60%;
   MRI brain (several small acute right frontal and parietal cortical infarcts, septic
   emboli, no hemorrhage or abscess); repeat cultures 2025-12-14 no growth; deficits resolved by 2025-12-20; cardiac surgery: not
   at this time; IV cefazolin through 2026-01-24; skilled nursing facility 2025-12-30 to
   2026-01-24; neurology clinic 2026-02-24, no residual deficit.
9. **Hemoglobin 12.1** at the 2026-01-28 post-discharge visit.
10. **Cardiology follow-up 2026-03-11:** TTE with no vegetation, moderate MR, LVEF 60%; repeat
    echo in about a year.
11. **No injecting since December 2025**, by his report and the program's letters (the usual
    course of the treated disorder after the source ends).

## Primary-care layer (P)

- Visits every 2-4 months for hypertension, depression and his other problems; yearly
  physicals (2023-09, 2024-10, 2025-11); a missed visit in 2023; a telehealth visit.
- Blood pressure on amlodipine 5 mg usually about 132-138 / 82-86. Weight about 80 kg
  before 2025, about 77 after the spring 2025 admission, about 73 after December, about 75 in
  March 2026 and back to about 78 by June 2026. PHQ-9 between 7 and 13 over the years on sertraline 50 mg; AUDIT-C 1.
- Labs: CMP, CBC, lipid panel and A1c yearly, all normal except as set above (total cholesterol about 190, A1c about 5.4); HIV screen
  once (nonreactive).
- Prevention: influenza and COVID vaccines each fall; PCV20 and Tdap 2023-09; colorectal
  screening by FIT at 45 (negative); no lung cancer screening (under 50). Tobacco counseling
  at visits; he is not ready to quit.
- Refills for amlodipine, sertraline and naloxone; pharmacy requests; portal messages about
  results and scheduling; letters from the addiction program confirming he is engaged in
  treatment; records requests around the admissions.
- A visit booked for 2026-09-23. Nothing else open today.
- No acute illnesses, injuries, new diagnoses or life events beyond the timeline.


## Last seen

The practice last saw MRN00002050 on 2026-06-10. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. Nothing is booked after today.
