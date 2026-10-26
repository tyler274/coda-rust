# Build requirements: unit-2541 (Noah Nadeau, MRN00002541)

Source: synthetic_hospital patient 2541, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, its "Chloroquine-sensitive malaria prophylaxis" label,
or its cut-off plans ("Given the patient's history and treatment course,", "Given the
patient's acute agitation and history of schizophrenia,", "Based on the clinical
presentation and CSF analysis,").

Where the source gives a result only as "within normal limits", the chart may show
ordinary normal values for it and nothing else.

## Who he is (S)

Noah Nadeau, born 1994-08-03, male, White, not Hispanic or Latino, English-speaking,
Medicaid, lives in Northampton (seed).
Unemployed on disability benefits; once enrolled in college, he dropped out at a time the
chart does not give, and lived alone after that until 2024 (R2).
Never smoker, no alcohol, denies illicit drugs. NKDA throughout (R6). No surgery.
Family history: a first-degree relative with schizophrenia (the chart never says which),
mother with major depressive disorder, father with hypertension.

Joined the practice in 2023 (first visit 2023-09-12, a new-patient visit, age 29; A1).
He brought:

- **Paranoid schizophrenia**, diagnosed at 21 (about 2015 to 2016), treated with several
  antipsychotics since then (which ones before aripiprazole is not recorded), with medication
  nonadherence and multiple psychiatric hospitalizations, the last two in May and June
  2023 at an outside psychiatric unit (R3). Records of those admissions are not in the chart.
- **Latent tuberculosis infection**, treatment prescribed before joining and historically
  taken poorly (A2 for its evidence).
- **History of antipsychotic-induced acute dystonic reaction (oculogyric crisis)**, before
  joining; the drug responsible is not recorded (R5).
- Medications: **aripiprazole 10 mg daily** (R1) and **isoniazid 300 mg daily** for LTBI,
  adherence variable.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2023-09-12 | practice | new-patient visit (A1): history and medications as above, including several antipsychotics since diagnosis; guarded, sparse speech, normal vitals; LFTs normal; isoniazid 300 mg daily re-prescribed with adherence counselling; chest x-ray clear (A2); referred to an outpatient psychiatry group |
| 2024-01-16 | outside | psychiatry intake letter, accompanied by his mother (R2): behavior growing stranger over seven months: he has pulled away from people, mutters to himself, and hears voices remarking on what he does; he is convinced the neighbors mean him harm and has boarded over his windows so they cannot watch him; he has lived on his own since dropping out of college. Unkempt, avoids eye contact, appears to attend to internal stimuli; speech makes sense but drifts off the point now and then; neurological exam normal. Hb 14.2, WBC 6.8, platelets 250; Na 140, K 4.1, Cl 102, bicarbonate 24, BUN 18, Cr 1.0, glucose 90, Ca 9.6, AST 20, ALT 18, alkaline phosphatase 70, total protein 7.0, albumin 4.5, bilirubin 0.8; urine toxicology negative. Relapse of paranoid schizophrenia (R4): **risperidone 4 mg twice daily started**, aripiprazole continued (R1) |
| 2024-01-30 | outside | ED record (R1 for the date): age 29; for two days his eyes have suddenly rolled upward for minutes at a time, several times a day, starting two weeks into the new antipsychotic; alert, anxious, and upset by the episodes; vitals normal; on exam both eyes are pulled upward involuntarily in bursts; neck supple; rest of cranial nerves normal; CBC, CMP and prolactin normal. **Oculogyric crisis due to risperidone** (R5); IV diphenhydramine and a short benztropine course (A3); back to psychiatry |
| 2024-02-08 | practice | post-ED visit (P): episodes gone; reaction added to the problem list as an adverse drug reaction, not an allergy (R6); psychiatry keeps risperidone |
| 2024-05-13 | outside | psychiatry follow-up letter, three months after the oculogyric crisis: voices, paranoid delusions and withdrawal persist despite adherence, which his family confirms; he has been through several antipsychotics since his diagnosis at 21; he now lives with his parents (R2); two psychiatric admissions in the past year (R3); no substance use. Disheveled, blunted affect, reduced speech. CBC, CMP, lipid panel, fasting glucose normal; urine toxicology negative. Plan: continue risperidone and aripiprazole unchanged, follow up in a few months (R7) |
| 2024-09-09 | outside | psychiatry letter (a second psychiatrist in the group), four months later: brought by his father for worse voices and withdrawal; lives with his parents; no tobacco or substances; age 30 and about nine years of illness (R8); repeated admissions for poor response and nonadherence; adequate trials of both risperidone and aripiprazole (therapeutic dose, six weeks or longer each) have helped little. T 36.7 C, BP 122/78, HR 76, RR 14, SpO2 98% on room air; exam unremarkable; CBC, CMP, liver enzymes normal. **Clozapine started** with weekly ANC (A4); risperidone and aripiprazole continue pending a cross-taper |
| 2024-10 | — | he stops all his medications, including clozapine, in mid-October (S: not taken "for several weeks" by November); weekly ANC draws stop (A4) |
| 2024-11-06 | outside | ED record: age 30 (R8); police bring him in from a park where he was yelling and hurling things, two months after the psychiatry visit; never violent before; talking him down and offering food and water do not work; he then flings chairs and says he will hurt staff; off his medications for weeks; pacing, responding to internal stimuli, vitals stable, exam otherwise unremarkable. Na 140, K 4.1, Cl 102, bicarbonate 24, BUN 18, Cr 1.0, glucose 98; CBC normal. IM sedation, then **admission to inpatient psychiatry**; **clozapine stopped** and not resumed; risperidone and aripiprazole resumed (A5) |
| 2024-11-18 | outside | psychiatric discharge summary received (A5): discharged to his parents on risperidone 4 mg twice daily, aripiprazole 10 mg daily, isoniazid 300 mg daily |
| 2024-11-21 | practice | post-hospital visit (P) |
| 2025-01-07 | outside | ED record: police bring him in again two months after the last time, once more yelling and hurling things in a park; drenched in sweat, flushed, pacing, verbally aggressive, responding to internal stimuli; not taking his medications; no substances. T 37.8 C (100 F), BP 150/95, pulse 110, RR 22; dilated pupils, increased muscle tone. CBC and electrolytes normal; urine toxicology negative; blood alcohol <10 mg/dL. **Acute agitation with an inadequate response to the first sedating dose** (R9; A6 for treatment, CK and course); discharged with his parents to psychiatry follow-up |
| 2025-01-13 | practice | post-ED visit (P): back on risperidone and aripiprazole; psychiatry follow-up booked |
| 2025-04-11 to 2025-05-02 | — | three-week camping trip in rural Uganda; no malaria prophylaxis taken (S) |
| 2025-05-09 | outside | ED record and admission at a regional hospital: age 30; fevers since two days after his return (five days), with shaking chills, drenching sweats, exhaustion, headache, myalgias and malaise; no prophylaxis. Tired-looking; T 39.4 C (103 F); mild scleral icterus; palpable spleen. Mild anemia and thrombocytopenia, ring forms inside red cells on smear (R10 for values). **Malaria** (R10); admitted and treated (A7) |
| 2025-05-12 | outside | discharge summary received (A7) |
| 2025-05-19 | practice | post-discharge visit (P): well; CBC and bilirubin normal again; advised to book a pre-travel visit before any future trip |
| 2026-01-05 | outside | ED record and admission at a regional hospital: age 31; for two weeks a bad headache, fevers and a stiff neck, and in the last few days he has become muddled and sleepy; LTBI left untreated through poor isoniazid adherence. T 38.5 C, nuchal rigidity, positive Brudzinski sign, right abducens palsy. CSF: WBC 150/uL (80% lymphocytes), RBC 10/uL, protein 100 mg/dL, glucose 30 mg/dL with serum glucose 90, adenosine deaminase 12 U/L (elevated). **Tuberculous meningitis** (R11); work-up and treatment per A8: **isoniazid continues within four-drug therapy; rifampin, pyrazinamide, ethambutol, pyridoxine and dexamethasone started** |
| 2026-01-23 | outside | discharge summary received (A8): CSF AFB culture still pending; public health directly observed therapy begins |
| 2026-01-26 | outside | infectious disease clinic letter (A8): the 2026-01-05 CSF AFB culture grows Mycobacterium tuberculosis; susceptibilities pending; regimen unchanged |
| 2026-01-29 | practice | post-discharge visit (P): headache and confusion gone; abducens palsy improving; culture result reviewed |
| 2026-02-09 | outside | infectious disease clinic letter (A8): the isolate is susceptible to all first-line drugs; regimen unchanged |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Every source list shows risperidone 4 mg twice daily and aripiprazole 10 mg daily from
  the first visit, yet the February ED note says a new schizophrenia drug began two weeks
  earlier, after the psychiatry evaluation. Resolution: he joins on aripiprazole alone;
  risperidone is the new drug, started at the 2024-01-16 intake. The ED visit is moved from
  2024-02-06 to 2024-01-30 so it falls two weeks after the start. Risperidone is the more
  likely cause of an oculogyric crisis at 8 mg a day.
- **R2.** The January 2024 note has an unnamed woman giving the history and says he lives
  alone; by September he lives with his parents and his father brings him. Resolution: his
  mother accompanies him in January; by the May 2024 visit he lives with his parents (the
  chart gives no date or reason for the move).
- **R3.** "Hospitalized twice in the past year" in May 2024, yet the source shows no admission
  in that year. Resolution: both admissions were in May and June 2023, before he joined; the
  seven months of worsening before the January intake began after the second discharge.
- **R4.** The source lists "Schizophrenia (diagnosed 2020-01-15)" beside paranoid schizophrenia
  diagnosed at 21. Resolution: one diagnosis, paranoid schizophrenia since 21; the January
  2024 intake evaluates a relapse. Medication nonadherence is carried as its own problem.
- **R5.** The source lists a past antipsychotic dystonic reaction from the start and then
  "diagnoses" an oculogyric crisis in 2024. Resolution: both are true. He had a dystonic
  reaction before joining (the ED note's "prior dystonic reactions"; the drug is not recorded),
  and the January 2024 episode is a new one caused by risperidone. The problem list keeps one
  entry with both episodes.
- **R6.** Allergies stay NKDA, as every source note says. The dystonic reactions live on the
  problem list as adverse drug reactions, not on the allergy list, because psychiatry keeps
  prescribing antipsychotics.
- **R7.** The May 2024 plan is cut off. Resolution: no change; continue both drugs and follow
  up. The source changes nothing until September.
- **R8.** Ages and duration: the September and November 2024 notes say 29, but he turned 30 on
  2024-08-03, so the chart says 30. The September "6-year history" is not carried: diagnosed
  at 21, he has about nine years of illness in 2024.
- **R9.** "Inadequate sedation or agitation" sits on later problem lists as if chronic.
  Resolution: it is the diagnosis of the 2025-01-07 ED visit only, recorded there as acute
  agitation in schizophrenia that needed a repeat sedating dose; the practice problem list
  does not carry it as active. The cut-off plan becomes A6.
- **R10.** The "Chloroquine-sensitive malaria prophylaxis" diagnosis is a quiz answer, and
  chloroquine would not suit East Africa. Resolution: the diagnosis is malaria in a traveller
  who took no prophylaxis, on the problem list as resolved after May 2025. The source's
  "mild" values are carried as Hb 12.4 g/dL, platelets 112 x10^3/uL, total bilirubin
  2.1 mg/dL.
- **R11.** The January 2026 plan is cut off. Resolution: tuberculous meningitis, admitted and
  treated per A8. The LTBI problem is replaced by tuberculous meningitis, and isoniazid
  continues as part of treatment rather than as LTBI therapy.
- **R12.** Clozapine is started in September 2024 but never appears on a later source list,
  while risperidone and aripiprazole appear on every one. Resolution: risperidone and
  aripiprazole continue during the clozapine titration, he stops everything in October,
  and the November admission resumes the old pair and does not restart clozapine (A5).
  Outside ED lists from 2025 and 2026 match the practice list.
- **Left as the source has it:** risperidone continues at full dose beside aripiprazole after
  the oculogyric crisis it caused; isoniazid alone stays prescribed for over two years with
  variable adherence, and no one moves him to a shorter or observed LTBI regimen; no
  long-acting injectable antipsychotic is offered despite repeated nonadherence; clozapine is
  not tried again; the low-grade fever at the January 2025 ED gets no infectious work-up (no
  cultures, urinalysis, chest x-ray or lumbar puncture): the ED checks only the CBC, the CK and
  repeat vitals in A6, and puts it down to agitation.
  No note comments on any of these. The physician who inherits the chart can find those gaps.

## Additions (A): physician to confirm

- **A1.** The 2023-09-12 new-patient visit (he is an established schizophrenia and LTBI patient
  at the first source encounter, and the practice made the psychiatry referral).
- **A2.** Evidence for the source's LTBI: records from prior care show a positive
  interferon-gamma release assay (2022), and the practice's chest x-ray at joining is clear,
  with no sign of active tuberculosis.
- **A3.** ED treatment of the oculogyric crisis: diphenhydramine 50 mg IV, episodes stop within
  minutes; discharged with benztropine 1 mg twice daily for three days, then stopped.
- **A4.** Clozapine dosing and monitoring: 12.5 mg at bedtime from 2024-09-09, titrated by
  psychiatry to 150 mg at bedtime by 2024-10-04. Weekly ANC from the September baseline, all
  normal (3.4 to 4.8 x10^3/uL), until he stops in mid-October and the draws stop.
- **A5.** The November 2024 ED gives IM haloperidol 5 mg, lorazepam 2 mg and diphenhydramine
  50 mg (diphenhydramine because of his dystonia history); he settles and is admitted to an
  inpatient psychiatric unit at a regional hospital from 2024-11-06 to 2024-11-18. There
  risperidone 4 mg twice daily and aripiprazole 10 mg daily are resumed, clozapine is stopped
  after the lapse, and isoniazid continues.
- **A6.** The January 2025 ED gives IM lorazepam 2 mg, with little effect, then a second 2 mg
  dose 30 minutes later, after which he is calm. CK 160 U/L (normal), checked because of the
  fever, rigidity and tachycardia on antipsychotics. Over several hours of observation:
  T 37.0 C, HR 84, BP 128/80, muscle tone normal. An ED psychiatric evaluation finds him
  safe for discharge with his parents; oral risperidone and aripiprazole restart, with
  psychiatry follow-up within a week.
- **A7.** Malaria: smear species Plasmodium falciparum, parasitemia 1.2%; admitted from
  2025-05-09 to 2025-05-12 and treated with atovaquone-proguanil, four adult tablets daily
  for three days (chosen over artemether-lumefantrine to avoid QT effects with his
  antipsychotics); fever gone and smear negative before discharge.
- **A8.** Tuberculous meningitis work-up and treatment, admitted 2026-01-05 to 2026-01-23:
  CT head before the lumbar puncture without mass or hydrocephalus; MRI brain with basal
  meningeal enhancement, no hydrocephalus or infarct; chest x-ray clear; HIV, hepatitis B and
  hepatitis C negative; the CSF AFB culture is sent and still pending at discharge. Weight 70 kg. Isoniazid 300 mg, rifampin 600 mg,
  pyrazinamide 1500 mg and ethambutol 1200 mg daily with pyridoxine 50 mg daily, and
  dexamethasone IV then an oral taper stopped after eight weeks (early March 2026). After two
  months (early March 2026) pyrazinamide and ethambutol stop and isoniazid, rifampin and
  pyridoxine continue for ten more months, to a planned end in early January 2027. Reported to
  public health, which gives the drugs by directly observed therapy; outpatient infectious
  disease follows him monthly. The discharge summary notes that rifampin lowers antipsychotic
  levels and asks psychiatry to watch for relapse; no dose change is recorded. After discharge
  the infectious disease clinic reports the culture: Mycobacterium tuberculosis grows on
  2026-01-26, three weeks after the lumbar puncture, and on 2026-02-09 the isolate is
  susceptible to isoniazid, rifampin, pyrazinamide and ethambutol; the regimen does not change.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention
  (heart and lung sounds, abdominal exam outside the malaria admission, neurological exam
  outside the dystonia and meningitis). An abnormal finding appears only as the source or this
  file gives it. A report may carry the descriptors its kind always carries (how a size was
  measured, a procedure's access site), but nothing that adds a territory, a severity grade or
  a symptom, or changes the diagnosis or plan.
- Records from a prior practice may mention routine screening done there, with normal results.
- Practice visits about every 3 to 4 months: isoniazid adherence and LFTs (normal) before
  2026; antipsychotic metabolic monitoring yearly (weight about 70 kg, height 175 cm, BMI about
  23, stable; BP normal; fasting glucose or A1c and lipids normal); coordination with
  psychiatry. From 2026, visits after the infectious disease letters, with monthly LFTs
  (normal), ethambutol vision checks (normal) through March, and the abducens palsy resolved by
  April 2026. His psychiatric symptoms are stable through 2026 on the same drugs.
- Yearly physicals in 2024 and 2025, with depression screens.
- Prevention for his age and sex: influenza each fall, COVID vaccines, Tdap if due; HIV and
  hepatitis C screening once at joining, negative.
- Refills and pharmacy traffic (isoniazid fills are irregular before 2026), phone calls from
  his parents about appointments and refills, portal messages about results and scheduling,
  reminders, and a missed visit in 2024.
- Booked visit after 2026-08-27: a routine follow-up on 2026-09-22.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.
