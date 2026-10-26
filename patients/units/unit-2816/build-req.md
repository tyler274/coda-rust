# Build requirements: unit-2816 (Jada Washington, MRN00002816)

Source: synthetic_hospital patient 2816, in `source.md` beside this file. This file and
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
"(diagnosed 2020-07-15)" stamps, or its "CT findings are shown below" placeholder.

## Who she is (S)

Born 1981-03-25, female, Black, not Hispanic, English-speaking, Medicaid, lives in
Florence with her partner (the partner who brings her to the ED in 2026). Part-time
customer service representative. Former smoker, 20 pack-years, quit in March 2024
(R4). Alcohol socially, 1 to 2 drinks a month. No drug allergies (NKDA). No surgeries.
Family history: mother type 2 diabetes; father hypertension and a stroke at 67; an
older brother with opioid use disorder.

Joined the practice on **2024-08-20** (A1, new-patient visit). She brought:

- **HIV infection with AIDS (CD4 under 200)**, followed at an outside infectious
  disease clinic, on **bictegravir/emtricitabine/tenofovir alafenamide 1 tablet daily**
  and **trimethoprim-sulfamethoxazole 800/160 mg (DS) three times weekly, Mon/Wed/Fri**
  (Pneumocystis prophylaxis), both prescribed by ID (historical on the practice list).
- **Opioid use disorder in sustained remission on maintenance buprenorphine-naloxone
  8 mg/2 mg sublingual daily**. From joining, the PCP prescribes it (A2).
- History of an HIV-related opportunistic skin infection, treated and resolved before
  she joined (R5).
- History of tobacco dependence (former smoker, above).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2024-08-20 | practice | new-patient visit (A1): history as above; NKDA; exam unremarkable, no skin lesions; buprenorphine-naloxone prescribing taken over (A2); records requested from the ID clinic and prior practice |
| 2025-09-14 (Sun) | outside | ED, an area hospital: 6 days of progressively blurred vision and left eye discomfort, mild photophobia, floaters; no trauma, no recent travel. T 36.9 °C, pulse 74, BP 122/68. Fundus: patches of retinal whitening with hemorrhage in the left eye. No skin lesions, no oral thrush. CD4 86 cells/µL. ED impression: viral (CMV) retinal infection in advanced immunosuppression; urgent ophthalmology. Practice receives a short ED note (R1) |
| 2025-09-16 | outside | ophthalmology (retina) sees her (A3): **CMV retinitis, left eye**, right eye clear. ID starts **valganciclovir 900 mg twice daily for 21 days (induction)**. Reaches the practice as the patient's call and a pharmacy prior-authorization request; no letter |
| 2025-09-24 | practice | post-ED visit (P): vision in the left eye still blurred, improving; taking valganciclovir; ED note reviewed; **CMV retinitis on the problem list**; list updated |
| 2025-10-07 | outside | ID moves valganciclovir to **900 mg once daily (maintenance)** after induction (A3); the practice learns it from the fill history and the patient at the October nurse visit |
| 2026-03-20 | practice | last visit: buprenorphine follow-up, stable; still on valganciclovir maintenance per ID; turned 45, FIT kit given (P) |
| 2026-05-12 | outside | ED, an area hospital: brought by her partner for a week of worsening confusion and headache, nonverbal and lethargic today; disoriented, mild right-sided weakness; CD4 at her last ID clinic visit 70 cells/mm³ (drawn at ID in April 2026, never sent to the practice). CT head: a left cerebral mass (R2). Admitted |
| 2026-05-12 to 2026-05-29 | outside | admission (A4): MRI brain with and without contrast, a single enhancing mass in the left cerebral hemisphere with surrounding edema; stereotactic biopsy: diffuse large B-cell lymphoma, **primary CNS lymphoma**; **dexamethasone** for edema, tapering; oncology begins **high-dose methotrexate with rituximab** (leucovorin rescue), given inpatient in cycles; ART, TMP-SMX, valganciclovir and buprenorphine continued. Discharged 2026-05-29: confusion cleared, speaking normally, right-sided weakness improving |
| 2026-06-03 | outside | discharge summary received by the practice and filed (A4) |
| 2026-08-11 | outside | oncology clinic note received (A5): induction ongoing with planned inpatient cycles; interim MRI smaller lesion with less edema; dexamethasone finished; plan to continue induction |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source's primary diagnosis for the first encounter is "Cytomegalovirus
  esophagitis", but every finding is in the left eye (blurred vision, floaters,
  photophobia, retinal whitening and hemorrhage, CD4 86) and no swallowing symptom is
  given. Resolution: the encounter is CMV retinitis of the left eye, confirmed by
  ophthalmology (A3). The esophagitis label is not carried; no swallowing symptom,
  endoscopy or esophagitis appears anywhere. The coded problem is CMV disease (B25.8)
  named as retinitis, left eye.
- **R2.** The second encounter's CT is "shown below" and never given. Resolution: the
  ED note says the head CT showed a left cerebral mass (left, from the right-sided
  weakness), with no size or further descriptors; the detail belongs to the MRI (A4).
- **R3.** The source's dates are "seen July 2020" and the July problem-list stamp.
  Resolution: the eye visit is September 2025 on the re-dated timeline, and the 2026
  ED note says so.
- **R4.** Both encounters say she quit smoking "18 months ago". Resolution: she quit
  in March 2024 (18 months before the September 2025 ED visit); the May 2026 ED note
  says about two years. 20 pack-years throughout.
- **R5.** "HIV-related opportunistic skin infection" is listed as a comorbidity with no
  detail, and both encounters find no skin lesions. Resolution: a past episode,
  treated and resolved before she joined, carried as history only (no organism, no
  drug named); every skin exam is clear.
- **R6.** The May 2026 ED medication list omits TMP-SMX while its prose says she takes
  it. Resolution: TMP-SMX continues throughout.
- **Left as the source has it:** her CD4 stays under 100 on ART with no explanation of
  why, and no viral load ever reaches the practice. No note speculates about adherence
  or resistance. No naloxone rescue kit is prescribed. Nobody comments on either.

## Additions (A): physician to confirm

- **A1.** The 2024-08-20 new-patient visit (two years with the practice; the source
  has no primary-care contact).
- **A2.** The PCP prescribes her buprenorphine-naloxone 8 mg/2 mg daily from joining
  (her previous prescriber was at her prior practice), with a prescription-monitoring
  check at each prescribing visit. Dose never changes.
- **A3.** CMV retinitis needs ophthalmology confirmation and treatment: retina exam
  2025-09-16 confirming left-eye CMV retinitis with the right eye clear;
  valganciclovir 900 mg twice daily for 21 days started by ID 2025-09-16, then 900 mg
  daily from 2025-10-07, continued to today. Ophthalmology follow-up every few months,
  reported by the patient as stable; blurred vision in the left eye improved but not
  fully back.
- **A4.** Primary CNS lymphoma needs tissue and treatment: the 2026-05-12 to 05-29
  admission as in the timeline (MRI, stereotactic biopsy showing diffuse large B-cell
  lymphoma, dexamethasone taper, high-dose methotrexate with rituximab and leucovorin
  rescue, discharge with confusion cleared and right-sided weakness improving), and
  the discharge summary received 2026-06-03.
- **A5.** The 2026-08-11 oncology note (induction continuing; interim MRI: the lesion
  smaller with less edema; dexamethasone finished).

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may
  carry the descriptors its kind always carries, but nothing that adds a territory, a
  severity grade or a symptom, or changes the diagnosis or plan.
- Records from the prior practice may mention routine screening done there (a normal
  Pap and mammogram in 2023).
- Buprenorphine visits about every 2 to 4 months with vitals, PHQ-9, a monitoring-
  database check, and 28-day prescriptions with refills; refills by message between.
- Yearly labs at the practice: CMP, lipid panel, A1c, CBC, normal; hepatitis C
  antibody once, negative. HIV labs (CD4, viral load) are drawn at ID and do not
  reach the practice, except the CD4 values above.
- Prevention: influenza vaccine each fall, updated COVID vaccine, PCV20 once, Shingrix
  two doses, a screening mammogram (normal), a Pap with HPV co-test (normal, HPV
  negative), PHQ-9 screens (low scores), FIT kit given at 45 (not returned by today).
- Refills, pharmacy and prior-authorization traffic, portal messages, a missed visit.
- After 2026-03-20: only messages and calls (the partner calling about the admission,
  buprenorphine refills sent by message), outside records filed as received. The
  practice's problem and medication lists stay as they were on 2026-03-20.
- Nothing booked after today.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002816 on 2026-03-20. Practice contacts after that day do
not happen; outside care after it still does, and its records are filed as received
without changing the practice's lists. Nothing is booked after today.
