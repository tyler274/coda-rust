# unit-1900 build requirements

Source: synthetic_hospital patient 1900 (`source.md`). Practice today: 2026-08-27T12:00 Eastern.

Every clinical fact in the chart traces to **S** (the source as carried here), **R** (a
resolution below), **A** (an addition below) or **P** (the primary-care layer below), and
nothing else. Texture (voice, scheduling, portal traffic, admin) is free and carries no
clinical fact. Re-voice everything: no source sentence, clinician name, problem-list dump,
"(diagnosed ...)" stamp or quiz text reaches the chart.

## Who she is (S)

- Angela Damon, MRN00001900, female, born 1991-05-18 (34 at the first source encounter,
  35 today). Northampton, MA 01060. White, not Hispanic or Latino. English. Private
  insurance (Blue Cross Blue Shield). Phone +1-617-555-0101, angela.damon@example.test.
  Marital status: single.
- Elementary school teacher. Never smoker. Alcohol: social, 1-2 drinks per week.
- Allergies: no known drug allergies.
- Surgical history: none. Family history: no significant family history.
- Home medications: none.
- **Joins the practice 2025-07-30** at source encounter 0, a new-patient visit for cough.
  She brings no chronic problems and no medications.

## Timeline

| Date | Where | What happens |
|---|---|---|
| 2025-07-30 (Wed) | Practice, new-patient visit (S, encounter 0) | Establishes care. Cough for 3 weeks following an upper respiratory tract infection (URI recorded as history, onset about 2025-07-09, resolved), worse at night, with a sensation of mucus dripping down the back of the throat. Over-the-counter cough syrups gave no relief. No history of pulmonary disease; otherwise unremarkable history. No medications, NKDA, no surgeries, no significant family history, denies smoking. Vitals within normal limits. Lungs: no wheezes, rales or rhonchi; rest of exam unremarkable. **Diagnosed: upper airway cough syndrome (postnasal drip syndrome).** No drug started (see Left as the source has it). Weight 63.5 kg, height 165 cm (P). |
| 2025-08-06 | Outside records received (P) | Prior primary-care practice records: no chronic problems, no medications, NKDA; cervical cytology with HPV co-test 2023-03, normal; Tdap 2021-08. |
| 2025-08-25 | Portal (P) | Patient reports the cough has gone. |
| 2025-10-08 (Wed) | Practice, annual physical (P) | **Upper airway cough syndrome resolved.** PHQ-2 0. Labs: CBC, CMP, lipid panel, HIV-1/2 Ag/Ab, hepatitis C antibody, all normal. Influenza and updated COVID-19 vaccines given. Cervical screening next due 2028. |
| 2026-04-21 (Tue) | Practice, portal message then same-day visit (P, with A1) | Neck pain and a feeling of fullness in the throat for about a week, low-grade fevers, malaise, palpitations, intermittent sweating; a cold about two weeks earlier that cleared on its own (S, R2). Exam: tender, enlarged thyroid without nodularity (S finding, as the endocrinologist also finds it). Temperature 37.9 °C (A1). Labs drawn: TSH, free T4, ESR, CRP. |
| 2026-04-22 | Practice, results and telephone (P) | TSH <0.01 mIU/L, free T4 2.5 ng/dL, ESR 70 mm/hr, CRP 8 mg/L (S, values carried exactly). PCP calls, places urgent endocrinology referral. |
| 2026-04-29 (Wed) | Outside endocrinology consult (S, encounter 1); letter received 2026-04-30 | Neck pain and throat fullness for 2 weeks with low-grade fever, malaise, palpitations, intermittent sweating. URI about 3 weeks earlier, resolved without intervention. Known upper airway cough syndrome from 2025, since resolved (R3). HR 88, BP 132/85. Tender, enlarged thyroid without nodularity. Reviews the 2026-04-22 labs above. **Diagnosed: subacute thyroiditis.** No medication started. Plan: repeat TSH and free T4 in about 6 weeks through primary care, call sooner for worsening (P). Referral closed. |
| 2026-06-10 (Wed) | Practice, lab only (P) | TSH 2.4 mIU/L, free T4 1.2 ng/dL (normal). Copy to endocrinology. |
| 2026-06-17 (Wed) | Practice, telehealth follow-up (P) | Neck pain, palpitations and sweating gone. **Subacute thyroiditis resolved.** |
| 2026-08 | Portal (P) | Schedules annual physical. |
| 2026-10-07 (Wed) | Practice, booked (P) | Annual physical, appointment booked. |

## Resolutions (R)

1. **Joining date.** Encounter 0 does not read as established care (no prior problems, no
   "returns"), so she joins at it as a new patient on 2025-07-30; the seed's one year with
   the practice fits.
2. **The URI in encounter 1.** Encounter 1 says the URI was "3 weeks ago" and resolved
   without intervention, nine months after the URI that preceded the cough. The chart reads
   it as a second, recent self-limited viral illness before the thyroiditis, reported by the
   patient and never seen at the practice; it is not recorded as a new condition. The
   encounter 0 URI is recorded once, as history, at the 2025-07-30 visit.
3. **"Her January visit" and the active problem list.** Encounter 1 says she returns after
   "her January visit for cough" and lists the cough syndrome as active. On the re-dated
   timeline that visit was 2025-07-30 and the cough resolved by 2025-10-08. The endocrinology
   letter mentions the 2025 cough visit as past history, not as an active problem.
4. **"Returns."** Encounter 1 is an outside endocrinology consult, not a return to the same
   clinic; the patient is new to endocrinology, referred by the PCP.
5. **Vital signs at encounter 0** are "within normal limits": the chart measures them
   without stating levels. HR 88 and BP 132/85 are carried only at the endocrinology visit.

**Left as the source has it:** no medication for the cough syndrome (no antihistamine,
decongestant or nasal steroid); no medication for the thyroiditis (no anti-inflammatory,
steroid or beta blocker); no thyroid uptake scan, ultrasound or thyroid antibodies. Nobody
raises or remarks on these.

## Additions (A)

1. **Temperature 37.9 °C at the 2026-04-21 PCP visit**, the low-grade fever the source
   reports, measured at the visit that sent her to endocrinology.

## Primary-care layer (P)

- The 2026-04-21 PCP visit and 2026-04-22 result call that make the endocrinology referral,
  and the labs drawn there (values in the timeline, carried exactly).
- Outside records from the prior practice (normal 2023 cervical co-test, Tdap 2021).
- One annual physical (2025-10-08) with CBC, CMP, lipid panel, HIV and hepatitis C
  screening, all normal; PHQ-2 0; influenza and updated COVID-19 vaccines. First visit
  weight 63.5 kg, height 165 cm; values otherwise her own normal.
- Thyroid recheck 2026-06-10 (TSH 2.4, free T4 1.2) and a telehealth follow-up on 2026-06-17.
- Portal messages about the cough, results and scheduling; one cancelled visit is fine.
- Annual physical booked 2026-10-07.
- No acute illnesses, injuries, new diagnoses or life events beyond the timeline.
