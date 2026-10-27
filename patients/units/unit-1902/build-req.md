# Build requirements: unit-1902 (Emily Ouellette, MRN00001902)

Source: synthetic_hospital patient 1902, in `source.md` beside this file. This file and
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
"(diagnosed ...)" stamps, or its quiz text.

## Who she is (S)

Emily Ouellette, born 1999-07-19, female, White, not Hispanic or Latino, English,
Amherst 01002, university student health plan. Graduate student. Never smoker.
Alcohol: occasional social use, less than one drink a month. Surgical history:
tonsillectomy at age 8. Family history: a maternal cousin with epilepsy, father with
hypertension, mother with hypothyroidism.

Joined the practice 2023-09-12 (A1), age 24, at the start of her graduate program.
She brought: epilepsy (generalized tonic-clonic seizures, diagnosed at 16 by a
pediatric neurologist; seizure-free since about age 19), followed yearly by an
outside neurology group, on levetiracetam 500 mg twice daily (prescribed by
neurology). No known drug allergies on joining. Height 165 cm, weight about 61 kg,
normal blood pressure throughout.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2023-09-12 | practice | new-patient preventive visit (A1): history as above; exam normal; CBC, CMP, lipid panel, HIV and hepatitis C screening, all normal (P) |
| 2023-10-03 | outside | prior primary-care records received (P): HPV series and Tdap (2021) done; cervical cytology 2021 normal; COVID primary series |
| 2024-01-23 | outside | neurology yearly letter (P): seizure-free, continue levetiracetam 500 mg twice daily |
| 2024-09-17 | practice | annual physical (P): cervical cytology, negative for intraepithelial lesion; CBC, CMP normal |
| 2025-04-14 | outside | neurology visit (A2): at her request, a planned transition from levetiracetam to lamotrigine. **Lamotrigine started**: 25 mg daily for two weeks, then 50 mg daily; levetiracetam 500 mg twice daily continued until lamotrigine reaches its target; rash warning given. Letter received 2025-04-17 |
| 2025-05-06 | outside | neurology visit (S, encounter 0): two days of an itchy rash that began on the trunk and spread to the arms and legs, three weeks after starting lamotrigine (now 50 mg daily); no fever, no mucosal involvement, no systemic symptoms; vitals normal; multiple red macules and papules on trunk and extremities, no vesicles, bullae or target lesions, no lymphadenopathy; CBC, liver tests and creatinine normal (CBC and CMP). **Lamotrigine drug eruption diagnosed (R1); lamotrigine stopped permanently; lamotrigine allergy recorded (R2)**, high criticality because of the risk of a severe cutaneous reaction/DRESS; levetiracetam 500 mg twice daily continued (A2). Letter received 2025-05-08 |
| 2025-05-20 | practice | portal message (A3): rash fading after stopping lamotrigine |
| 2025-09-09 | practice | annual physical (P): rash gone (A3), **drug eruption resolved**; CBC, CMP normal |
| 2025-10-24 | outside | ED (S, encounter 1): brought by her roommate after a convulsive episode: sudden fall with loss of consciousness, limb jerking about 2 minutes, then unresponsive; regained consciousness after about 5 minutes, confused and drowsy. Temp 37.2 °C, BP 125/78, pulse 88, RR 16. Drowsy but oriented to time, place and person; bitten tongue; bruise on the left cheek; nonfocal neurological exam. Na 138, K 4.2, Cl 100, HCO3 24, BUN 14, Cr 0.9, glucose 98, Ca 9.2; CBC normal; antiepileptic drug level sent; its result never reaches the practice (R6). Allergy: lamotrigine. Medication: levetiracetam 500 mg twice daily (R3). **Breakthrough generalized tonic-clonic seizure diagnosed.** Discharged home the same day after return to baseline, neurology follow-up advised (A4). ED record received 2025-10-27 |
| 2025-10-28 | practice | post-ED visit (P): back to baseline, cheek bruise fading, tongue healing; no driving for six months after a seizure, per state rules; neurology visit arranged |
| 2025-11-12 | outside | neurology follow-up (A6): no further events; **levetiracetam increased to 750 mg twice daily**. Letter received 2025-11-14; practice list updated |
| 2025-12-24 | practice | last seen: epilepsy follow-up (P); no further seizures since October; tolerating 750 mg twice daily; **breakthrough seizure resolved** on the problem list (epilepsy stays) |
| 2026-05-12 | outside | neurology follow-up (A6): seizure-free since October 2025; levetiracetam 750 mg twice daily unchanged. Letter received 2026-05-14 and filed |
| 2026-09-16 | practice | booked: annual physical |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source's primary diagnosis is "Allopurinol hypersensitivity syndrome
  (DRESS)", but she never took allopurinol: the drug started three weeks before the
  rash is lamotrigine. And the findings (no fever, no mucosal or systemic involvement,
  normal CBC, liver tests and creatinine) do not meet DRESS. Resolution: the chart
  diagnoses a lamotrigine drug eruption (maculopapular, ICD-10 L27.0); because
  lamotrigine rashes can progress to SJS/TEN or DRESS, neurology stops it permanently
  and lists it as a high-criticality allergy ("severe cutaneous adverse reaction/DRESS
  risk; do not rechallenge"). Allopurinol appears nowhere in the chart.
- **R2.** The source lists the lamotrigine allergy before the reaction that caused it.
  Resolution: the allergy is recorded at the 2025-05-06 neurology visit; before that
  she has no known drug allergies.
- **R3.** The ED encounter still lists lamotrigine 50 mg daily as a home medication,
  months after it was stopped for the rash and while it sits on the allergy list.
  Resolution: lamotrigine is not carried after 2025-05-06; the ED record lists
  levetiracetam 500 mg twice daily (the source's "medication for seizure control").
- **R4.** The source's ED text calls the January 2020 rash visit by its source year.
  Resolution: it is the 2025-05-06 rash visit ("in May").
- **R6.** The ED's antiseizure level is pending in the source; the ED copy the
  practice receives says it was sent and gives no result, and no later record gives one.
- **R5.** "Liver function tests" at the rash visit are the hepatic part of a CMP drawn
  with the CBC; all normal.
- **Left as the source has it:** no treatment is given for the itchy rash beyond
  stopping lamotrigine; the ED does no head imaging or EEG after a first seizure in
  years; no one records a trigger for the breakthrough seizure. Nobody comments on
  these.

## Additions (A): physician to confirm

- **A1.** The 2023-09-12 new-patient visit (three years with the practice), with the
  epilepsy history and levetiracetam 500 mg twice daily she brought. The source names
  no antiseizure drug but lamotrigine, yet calls the October seizure a breakthrough on
  "medication for seizure control" after lamotrigine had to stop; levetiracetam is the
  standard choice for generalized epilepsy in a young woman.
- **A2.** The 2025-04-14 neurology visit that starts lamotrigine (the source says it
  began three weeks before the rash), as a cross-titration with levetiracetam
  continued; when lamotrigine stops, levetiracetam simply continues.
- **A3.** The rash fades within about two weeks of stopping lamotrigine and is gone by
  September 2025.
- **A4.** ED disposition: discharged home the same day, neurology follow-up.
- **A6.** Neurology follow-ups after the ED: 2025-11-12 levetiracetam raised to 750 mg
  twice daily, no driving for six months; 2026-05-12 seizure-free, unchanged, may
  drive again.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Records from a prior practice may mention routine screening done there, with normal results.
- Yearly physicals (2023 new-patient, 2024, 2025) with vitals, CBC and CMP each year on
  an antiseizure drug, lipid panel once, HIV and hepatitis C screening once, PHQ-2/PHQ-9
  normal, cervical cytology 2024 normal (2021 normal at the prior practice).
- Influenza vaccine each fall and updated COVID vaccines; Tdap (2021) and HPV series
  (adolescence) recorded from prior records.
- Post-ED visit and epilepsy follow-up as in the timeline; refills (neurology prescribes
  levetiracetam; the practice may bridge one), pharmacy traffic, portal messages about
  results and scheduling, an immunization-record request for her program, one cancelled
  visit.
- Booked visit after 2026-08-27: the 2026-09-16 annual physical.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00001902 on 2025-12-24. No practice visit comes after that day,
only portal messages, refills and scheduling; outside care after it still does, and its records are filed as received
without changing the practice's lists. A visit is booked after today.
