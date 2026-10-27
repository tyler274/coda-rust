# Build requirements: unit-2644 (Asher Brennan, MRN00002644)

Source: synthetic_hospital patient 2644, in `source.md` beside this file. This file and
the source are the only authority for clinical facts. Every diagnosis, medication,
result, procedure and specialist finding in the chart must trace to one of:

- **S** a fact in source.md, carried as listed below;
- **R** a resolution below, where the source contradicts itself or stops short;
- **A** an addition below, listed one by one;
- **P** the primary-care layer described at the end.

Nothing else. No diagnosis, medication, test result, procedure, family member or
life event that is not on these lists. Texture (wording, scheduling, portal traffic,
who signed what) is free, as long as it carries no new clinical fact.

Re-voice everything. The source notes are exam vignettes: never copy a sentence, and
never carry a source clinician's name, its "Active Problem List" dump, its
"(diagnosed ...)" stamp, or its unfinished ED note as written.

## Who he is (S)

Asher Brennan, born 2006-02-01, male, White, not Hispanic or Latino, English. Home
address with his parents in Easthampton (01027); a university student health
insurance plan (PPO). A high school senior when he joined; a college freshman living in a
university dormitory from the summer of 2025 (R1). Never smoker. Alcohol: occasional social use, 1 to 2 drinks on
a weekend. No surgical history. No regular medications at any point.
Family history: father hypertension, diagnosed at 42; mother hypothyroidism,
diagnosed at 40; no family history of primary immunodeficiency or autoimmune disease.
Perinatal history from pediatric records: neonatal herpes simplex virus exposure
(R5), carried as a history item only.

Joined the practice on 2024-02-20, shortly after his 18th birthday, moving from a
pediatric practice (A1). He brought no active problems, no medications and no known
drug allergies. Immunizations complete per pediatric records (R4).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2024-02-20 | practice | new-patient preventive visit (A1): healthy, history as above, NKDA; height and weight recorded, normal exam; CBC, CMP, lipid panel, HIV-1/2 Ag/Ab and hepatitis C antibody screening (P), all normal; PHQ-2 negative. Pediatric records requested |
| 2024-03-05 | outside | pediatric records received (A1): routine childhood care; vaccines complete including Tdap, MenACWY (two doses) and HPV series; neonatal herpes simplex virus exposure listed (R5) |
| 2024-10-08 | practice | nurse visit: influenza and updated COVID-19 vaccines (P) |
| 2025-06-10 | practice | preventive visit before starting college (P); university health form completed; normal exam; PHQ-9 low |
| 2025-07-25 | outside | student health services visit (summer session, R1): 1 week of fever, sore throat and fatigue, worse despite over-the-counter pain relievers; roommate recently recovered from a similar illness; took a friend's amoxicillin for the sore throat and 3 days later developed a widespread maculopapular rash. Temperature 38.5 °C (101.3 °F). Exudative pharyngitis, bilateral tender cervical lymphadenopathy, mild hepatosplenomegaly. WBC 15.0 K/uL, atypical lymphocytes 20%, ALT 70 U/L, AST 68 U/L, monospot positive. **Infectious mononucleosis diagnosed**; supportive care. The letter reaches the practice only on 2025-09-30, after a records request (Records) |
| 2025-07-28 | practice | portal message from the patient: diagnosed with mono at student health, rash after the friend's amoxicillin. **Amoxicillin allergy recorded** (rash, mild, low criticality; recorded as the source labels it, R2). PCP reply: supportive care, avoid contact sports and heavy lifting for at least 4 weeks while the spleen is enlarged (A3), follow-up visit |
| 2025-08-19 | practice | post-mono follow-up (A2): fatigue improving, afebrile, throat clear, no lymphadenopathy, spleen not palpable, no rash; hepatic panel ALT 41 U/L, AST 29 U/L. Student health records requested. Mono stays on the problem list as active (list lag, Records) |
| 2026-01-12 | outside | ED visit: 2 days of headache, photophobia, neck stiffness and low-grade fever, with increasing fatigue, nausea, poor appetite and diffuse myalgias; several dormitory roommates with sore throat and malaise over the past week; no travel, animal exposure or drug use. Prior mono; fully vaccinated; no regular medications; amoxicillin allergy. T 38.1 °C (100.6 °F), BP 112/74, pulse 92, RR 18. Alert, uncomfortable, photophobia and neck rigidity, no focal neurologic deficits. Head CT unremarkable. LP: clear CSF, opening pressure 170 mm H2O, WBC 130/mm3 with 92% lymphocytes, RBC 5/mm3, protein 76 mg/dL, glucose 64 mg/dL; serum glucose 98 mg/dL; Gram stain no organisms. CSF enterovirus PCR detected, CSF culture no growth (A4). **Enteroviral meningitis diagnosed**; discharged home from the ED with supportive care (R3). The ED record reaches the practice on 2026-02-09 (Records) |
| 2026-01-13 | practice | portal message from the patient about the ED visit ("they said it was a virus"); PCP reply and a follow-up visit offered (P) |
| 2026-01-23 | practice | post-ED visit (A5): headache gone, appetite back, some tiredness; afebrile, no meningismus, neurologic exam normal. **Enteroviral meningitis resolved.** ED records requested; reports a flu shot at the campus clinic in October 2025 (P) |
| 2026-02-09 | outside | ED record received and reviewed (includes the enterovirus PCR and the final culture) |
| 2026-06-12 | practice | preventive visit cancelled by the patient (P), rebooked |
| 2026-06-26 | practice | **last seen**: preventive visit, home for the summer; well; normal exam; PHQ-9 low. **Infectious mononucleosis resolved** on the problem list. No labs due |
| 2026-08-11 | practice | portal request for a copy of his immunization record for school; front desk sends it (P) |
| 2026-12-22 | practice | booked: routine visit over winter break (P) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source calls him a college freshman at both encounters, which fall in
  July 2025 and January 2026 on the practice dates. Resolution: he lives in a
  university dormitory for a summer session in July 2025 and is a freshman for the
  2025-26 academic year. The ED's dormitory roommates are his freshman-year roommates.
  Ages (19 at both) follow from the birth date.
- **R2.** The profile lists the amoxicillin reaction from the start. Resolution: it is
  recorded first on 2025-07-28 from the patient's report and is absent before (NKDA in
  2024 and June 2025). It is recorded as an amoxicillin allergy with a rash reaction,
  as the source labels it; no one de-labels or re-challenges it (Left as the source has it).
- **R3.** The ED note stops after the head CT, with no assessment or disposition.
  Resolution: the ED record received by the practice gives the diagnosis of enteroviral
  meningitis supported by the CSF PCR (A4), no antibiotics, discharge home the same
  night with supportive care, return precautions and primary-care follow-up.
- **R4.** "He is fully vaccinated" maps to the pediatric records (A1): childhood series,
  Tdap, two doses of MenACWY and the HPV series, all complete.
- **R5.** The comorbidity "neonatal herpes simplex virus exposure" carries no detail.
  Resolution: a history item from the pediatric records only, with no further detail;
  no note discusses it.
- **R6.** The source gives a WBC and an atypical-lymphocyte percentage but no full
  differential. Resolution: the student health CBC carries WBC 15.0 and atypical
  lymphocytes 20% as written; no other differential values are reported.
- **Left as the source has it:** no EBV-specific serology at student health; the
  amoxicillin label is never questioned or removed; no neuroimaging beyond the head CT
  and no further meningitis workup. Nobody comments on any of it.

## Additions (A): physician to confirm

- **A1.** The 2024-02-20 new-patient visit and the pediatric records (he joined about
  2.5 years before his last visit; the source starts with no practice care).
- **A2.** The 2025-08-19 post-mono visit with a repeat hepatic panel, ALT 41, AST 29,
  to follow the source's elevated transaminases and hepatosplenomegaly.
- **A3.** Standard advice to avoid contact sports and heavy lifting while the spleen
  is enlarged, given in the PCP's reply.
- **A4.** CSF enterovirus PCR detected and CSF bacterial culture no growth (final
  2026-01-15), to support the source's enteroviral meningitis.
- **A5.** The 2026-01-23 post-ED practice visit.

## Records

The seed's chart texture is gaps. The practice hears of both outside encounters first
from the patient. The student health letter arrives on 2025-09-30 after a request
placed at the 2025-08-19 visit; the ED record arrives on 2026-02-09 after a request
placed at the 2026-01-23 visit. The problem list lags: mononucleosis stays active
until the June 2026 physical. The October 2025 campus flu shot is known only from the
patient's report.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Records from the pediatric practice may mention routine childhood care with normal results.
- Preventive visits roughly yearly (2024-02, 2025-06, 2026-06), with vitals, depression
  screens (PHQ-2/PHQ-9 negative), alcohol and tobacco history, HIV and hepatitis C
  screening once (2024), one lipid panel (2024).
- Influenza each fall (practice 2024, campus clinic 2025 by report); updated COVID-19
  vaccine in fall 2024.
- Portal messages about results, scheduling, forms and records; a cancelled visit;
  records requests.
- Booked visit after 2026-08-27: a routine winter-break visit.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002644 on 2026-06-26. Practice contacts after that day are
messages only; outside care after it would be filed as received without changing the
practice's lists. A visit is booked after today.
