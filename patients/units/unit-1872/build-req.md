# Build requirements: unit-1872 (Jada Owusu, MRN00001872)

Source: synthetic_hospital patient 1872, in `source.md` beside this file. This file and
the source are the only authority for clinical facts. Every diagnosis, medication,
result, procedure and specialist finding in the chart must trace to one of:

- **S** a fact in source.md, carried as listed below;
- **R** a resolution below, where the source contradicts itself or leaves a gap open;
- **A** an addition below, listed one by one;
- **P** the primary-care layer described at the end.

Nothing else. No diagnosis, medication, test result, procedure, family member or
life event that is not on these lists. Texture (wording, scheduling, refills, portal
traffic, who signed what) is free, as long as it carries no new clinical fact.

Re-voice everything. The source notes are exam vignettes: never copy a sentence, and
never carry a source clinician's name, its "Active Problem List" dumps, its
"(diagnosed 2020-01-15)" stamps, or its "diagnosed 12 months prior" phrasing.

## Who she is (S)

Born 2000-11-09, female, Black or African American, not Hispanic or Latino, English,
Medicaid, lives in Sunderland (01375), single. Administrative assistant, never smoker,
no alcohol. No surgical history.

- **Allergy:** sulfamethoxazole, recorded as an anaphylaxis risk (high criticality;
  no reaction description is carried beyond "anaphylaxis risk").
- **Family history:** younger brother with sickle cell disease; mother and father each
  with sickle cell trait; paternal aunt with systemic lupus erythematosus.
- **Joined the practice** 2025-08-12 (A1), a new-patient visit. She brought:
  sickle cell disease, HbSS genotype, since birth (baseline hemoglobin about 9.5);
  HIV infection diagnosed in April 2025 at her previous clinic, never on antiretroviral
  therapy; folic acid 1 mg daily. No prior hospitalizations are reported at joining.

## Timeline (dates fixed; S unless marked)

Pregnancy dating (R1): last menstrual period 2026-01-13, so 14 weeks 2 days on
2026-04-23 and 28 weeks 0 days on 2026-07-28; due date 2026-10-20.

| Date | Where | What happens |
|---|---|---|
| 2025-08-12 | practice | new-patient visit (A1): history as above; exam normal; Pap (P); CBC with Hb 9.5 and CMP normal apart from the chronic indirect hyperbilirubinemia of HbSS (total bilirubin about 1.9); PCV20 (P). HIV noted as untreated: **Infectious Disease referral placed** (R2) |
| 2025-10-07 | practice | nurse visit: influenza and COVID vaccines (P) |
| 2025-12-09 | practice | follow-up visit, no-show (P) |
| 2026-02-19 | practice | office visit (A2): positive home pregnancy test, LMP 2026-01-13; urine pregnancy test positive in office; **high-risk pregnancy recorded**; **prenatal vitamin 1 tablet daily started**, folic acid 1 mg continued; **Obstetrics referral placed** (high-risk prenatal care); ID referral still open, she is reminded of it (R2) |
| 2026-04-23 | outside, OB prenatal clinic | routine prenatal follow-up at 14 weeks 2 days; she asks about the chance of passing HIV to the baby. Diagnosed with HIV a year ago, never on ART; otherwise well. Meds folic acid 1 mg daily and prenatal vitamin daily. Vitals normal; exam unremarkable. HIV-1 RNA 15,000 copies/mL; CD4 450 cells/mm3; CBC at her baseline (R3); hepatitis B surface antigen negative; hepatitis C antibody negative. **HIV infection complicating pregnancy** diagnosed. Letter's plan: counseling that treatment in pregnancy greatly lowers transmission risk, coordination with ID for ART (R4). Letter received 2026-04-28 |
| 2026-05-08 | practice | follow-up, last held visit: reviews the OB letter; 16 weeks 3 days, feeling well; vitals normal; CBC drawn (Hb about 9.4); ID referral re-sent as urgent with the OB clinic in copy (R2). No ART prescribed by the practice (R4) |
| 2026-05-27 to 2026-05-31 | outside, regional hospital | admission for **acute chest syndrome** #1 (R5): fever and chest pain, new infiltrate on chest X-ray, low oxygen saturation; ceftriaxone and azithromycin, IV analgesia, incentive spirometry, oxygen; **simple transfusion of 2 units packed red cells** for admission Hb 7.4; discharge Hb 9.2. Discharge summary received 2026-06-03 |
| 2026-06-29 to 2026-07-03 | outside, regional hospital | admission for **acute chest syndrome** #2 (R5): same picture; **simple transfusion of 1 unit packed red cells** for admission Hb 7.8; discharge Hb 9.0. Discharge summary received 2026-07-07 |
| 2026-07-28 | outside, regional hospital ED | 6 hours of severe, sharp, constant abdominal pain not relieved by fluids and rest at home; no vaginal bleeding or discharge; 28 weeks pregnant; has kept up with prenatal care; pregnancy complicated by the two acute chest syndrome admissions with transfusions. Meds folic acid and prenatal vitamin (no ART). T 37.8 C, BP 130/85, HR 105, RR 22. Abdomen diffusely tender without rebound or guarding; fetal heart tones normal. Hb 8.2 (baseline 9.5), WBC 15.0, platelets 350, LDH elevated, indirect bilirubin elevated, haptoglobin decreased (values R6); urinalysis normal. **Sickle cell vaso-occlusive crisis** (abdominal). Admitted (R7). ED record received 2026-08-03 |
| 2026-07-28 to 2026-07-31 | outside, regional hospital | admission (R7): IV fluids, IV opioid analgesia, continuous fetal monitoring reassuring; hematology and maternal-fetal medicine involved. Inpatient ID consult: HIV-1 RNA and resistance genotype sent; **dolutegravir 50 mg daily plus emtricitabine/tenofovir alafenamide 200/25 mg daily started 2026-07-29** (A3). Pain resolved; discharge Hb 8.6. Discharge summary received 2026-08-04 |
| 2026-08-19 | outside, ID clinic | follow-up (A3): taking ART daily without side effects; HIV-1 RNA 290 copies/mL; genotype showed no resistance; continue regimen, recheck in a month. Letter received 2026-08-24 |
| 2026-09-10 | practice | booked visit after today: follow-up after the hospital stays (P) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source lists "Pregnancy (28 weeks gestation)" without saying for which
  encounter, and places the ED visit three months after the prenatal visit.
  Resolution: 28 weeks is at the ED visit; the prenatal visit is at 14 weeks 2 days.
  LMP 2026-01-13, due 2026-10-20.
- **R2.** The source has HIV diagnosed a year before the prenatal visit and never
  treated. Resolution: the diagnosis came at her previous clinic in April 2025, before
  she joined. The practice notes the gap and refers her to Infectious Disease at
  joining; she is reminded in February and the referral is re-sent in May. No ID
  visit happens before the July admission, and no note says why. The referral order
  from 2025-08-12 stays open (the inpatient consult is not the referral visit).
- **R3.** The source calls the prenatal CBC "within normal limits" in a woman with
  HbSS whose baseline hemoglobin is 9.5. Resolution: the CBC sits at her baseline
  (Hb 9.5, WBC and platelets normal), and the OB letter calls it stable.
- **R4.** The source gives no plan at the prenatal visit and still lists no ART three
  months later. Resolution: the letter counsels and defers ART to ID; nobody starts it
  before 2026-07-29, and no note in the practice or the outside records explains the
  delay. The practice does not prescribe ART.
- **R5.** The source says the pregnancy was complicated by two acute chest syndrome
  admissions with transfusions before the ED visit, while the prenatal visit calls
  her history unremarkable. Resolution: both admissions fall between the last
  practice visit and the ED visit. Their summaries carry only the findings above;
  the chest X-ray gives a new infiltrate without naming a lobe in the summary.
  Hemoglobin values 7.4/9.2 and 7.8/9.0 are this file's.
- **R6.** The source gives LDH, indirect bilirubin and haptoglobin only as elevated,
  elevated and decreased. Resolution: LDH 486 U/L, total bilirubin 3.4 mg/dL with
  indirect 3.0 mg/dL, haptoglobin 8 mg/dL.
- **R7.** The source gives no disposition for a 28-week HbSS crisis with tachycardia,
  low-grade temperature and leukocytosis. Resolution: admitted 2026-07-28 to 2026-07-31
  for pain control and fetal monitoring; no transfusion this stay.
- **Left as the source has it:** no hydroxyurea before pregnancy, no low-dose aspirin
  in pregnancy, no local hematology follow-up arranged by the practice. Nobody raises
  them.

## Additions (A): physician to confirm

- **A1.** 2025-08-12 new-patient visit (one year with the practice; the prenatal visit
  is a follow-up, so she was already a patient).
- **A2.** 2026-02-19 visit confirming pregnancy and sending her to obstetrics; the
  prenatal vitamin starts here (the source lists it by April).
- **A3.** After the source ends, the untreated HIV in pregnancy gets standard therapy:
  dolutegravir 50 mg plus emtricitabine/tenofovir alafenamide 200/25 mg daily, started
  by inpatient ID on 2026-07-29; ID clinic 2026-08-19 with HIV-1 RNA 290 copies/mL and
  no resistance on genotype. These reach the practice only as outside records after
  the last visit, so the practice medication list does not carry ART.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Height 165 cm; weight 58 kg at joining, about 59 kg in February 2026, about 62 kg in
  May 2026. Blood pressure normal (about 108/66) at practice visits.
- HbSS baseline levels: hemoglobin about 9.5, red cell distribution width about 19%,
  total bilirubin about 1.9 mg/dL (mostly indirect); WBC and platelets normal.
- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Cervical cytology at joining (age 24, HIV): negative for intraepithelial lesion.
- Vaccines: PCV20 at joining; influenza and COVID in fall 2025.
- PHQ-2/PHQ-9 screens negative.
- Refills (folic acid, prenatal vitamin), pharmacy traffic, portal messages about
  results, scheduling and her hospital stays, a missed visit.
- Booked visit after 2026-08-27: follow-up after the hospital stays (2026-09-10).

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00001872 on 2026-05-08. Practice contacts after that day are
only messages, refills and outside records; records are filed as received without
changing the practice's lists. A visit is booked after today.
