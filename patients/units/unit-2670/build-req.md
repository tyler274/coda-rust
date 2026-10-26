# Build requirements: unit-2670 (Lucas Becker, MRN00002670)

Source: synthetic_hospital patient 2670, in `source.md` beside this file. This file and
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
"(diagnosed ...)" stamps, or its quiz framing.

## Who he is (S)

Lucas Becker, born 1989-10-25, male, White, not Hispanic or Latino, English, commercial
insurance, lives in Holyoke (01040) with a roommate (S: the roommate finds him in 2026).
Office administrator. Never smoker, no alcohol, no drug use. No known drug allergies.
No surgical history. Family history: father with a generalized seizure disorder (R2).

Joined the practice 2024-09-10 (A1), new-patient preventive visit: no known problems,
no medications. He does not mention his earlier spells at joining (S: he had never
sought care for them), and nobody asks about them.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2024-09-10 | practice | new-patient preventive visit (A1). Height 180 cm, weight 82 kg, normal BP. Healthy, no meds, NKDA, FH father seizure disorder. CBC, CMP, lipids, HIV and hepatitis C screens (P), normal. Tdap (P) |
| 2025-05-10 → 05-11 | outside | ED visit, arrived by ambulance from work: colleagues saw about 2 minutes of convulsions; afterward confused and sleepy. Has had similar spells before but never sought care. No recent illness, drugs or alcohol. Drowsy but rousable, bitten/bruised tongue, small lower-lip laceration (no repair needed, A3), stable vitals, nonfocal neuro exam. Glucose 98, sodium 140, calcium 9.2, CBC normal, postictal prolactin elevated at 44 ng/mL (R3); rest of CMP normal (A2). EEG during the ED observation stay: sharp-wave discharges over the temporal region. **Epilepsy diagnosed** (R1). ED with phone neurology input **starts valproic acid 500 mg by mouth twice daily** (R1), driving restriction counseling (A4), outpatient neurology referral. ED record and EEG report received 2025-05-12 |
| 2025-05-14 | practice | post-ED visit: no further events, tolerating valproic acid, lip healing. Neurology referral placed (A5). Driving restriction reinforced (A4) |
| 2025-06-18 | outside | neurology consult letter: new-onset epilepsy with temporal sharp waves; continue valproic acid 500 mg twice daily; valproate level 74 mcg/mL (A6); hepatic panel and CBC normal (A6); return in 6 months |
| 2025-09-23 | practice | annual preventive visit (P); seizure-free on valproic acid; CBC and CMP normal (P); flu and COVID vaccines (P) |
| 2026-01-06 → 01-09 | outside | admission to pulmonary and critical care: found unresponsive at home by his roommate after a generalized tonic-clonic seizure of about 2 minutes that stopped on its own; takes his valproic acid as prescribed. Arrival vitals: T 37.0 °C, BP 130/85, pulse 88, RR 8, SpO2 92% on room air. Drowsy but rousable, GCS 13. Arterial blood gas: pH 7.28, PaCO2 60, HCO3 29, PaO2 70 (postictal hypoventilation; breathing and alertness recover within hours without ventilation, A7). Given generous IV normal saline. On hospital day 2 a BMP shows **hyperchloremic metabolic acidosis due to excess normal saline infusion** (R4): sodium 142, chloride 116, bicarbonate 17, anion gap 9. Fluids changed to a balanced crystalloid; 2026-01-08 chloride 105, bicarbonate 23 (**resolved**, R4). Valproate level on admission 71 mcg/mL (A6). Inpatient neurology: continue valproic acid 500 mg twice daily unchanged. Discharge summary received 2026-01-12 |
| 2026-01-13 | practice | post-discharge visit: back to baseline, no further events; BMP normal (P); driving restriction restarted (A4) |
| 2026-02-11 | outside | neurology follow-up letter: no events since January; valproic acid unchanged at 500 mg twice daily; next visit in 6 months |
| 2026-09-29 | practice | booked annual preventive visit (P); nothing else open today |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source has him on valproic acid with an active epilepsy diagnosis at the
  May 2025 ED visit, yet says he had never sought care for his spells, and stamps the
  diagnosis to that same visit. Resolution: epilepsy is diagnosed at the 2025-05-10 ED
  visit (EEG temporal sharp waves, recurrent spells) and valproic acid 500 mg twice
  daily starts at ED discharge. The ED record's home medication list does not carry it;
  before that visit he takes nothing.
- **R2.** The ED note says his mother has epilepsy; the profile and the 2026 note say
  his father has a generalized seizure disorder. Resolution: father. No family history
  in the mother.
- **R3.** "Postictal prolactin elevated" has no number. Resolution: 44 ng/mL, flagged high.
- **R4.** The source lists hyperchloremic metabolic acidosis due to excess normal saline
  as a primary diagnosis, but its only gas (pH 7.28, PaCO2 60, HCO3 29) is a respiratory
  acidosis from postictal hypoventilation (RR 8). Resolution: the arrival gas is carried
  as written and read as postictal hypoventilation (no separate coded diagnosis); the
  saline acidosis develops on hospital day 2 after large-volume normal saline (numbers
  above), resolves by 2026-01-08 on a balanced crystalloid, and is recorded diagnosed and
  resolved.
- **R5.** The 2026 note calls him 35 and refers to a "January 2020" seizure. Resolution:
  he is 36 on 2026-01-06, and the earlier seizure is the May 2025 ED visit.
- **Left as the source has it:** no brain MRI or other neuroimaging is ever ordered for
  his new epilepsy, and valproic acid stays at 500 mg twice daily after a breakthrough
  seizure on adherent therapy. Nobody comments on either.

## Additions (A): physician to confirm

- **A1.** 2024-09-10 new-patient visit (years with the practice is 2; the source's first
  encounter is the ED).
- **A2.** ED draws a CMP (the rest normal) as a baseline before starting valproic acid.
- **A3.** Lip laceration small, cleaned, no sutures.
- **A4.** State driving restriction counseling (no driving until 6 months seizure-free)
  after each seizure: standard after any seizure.
- **A5.** Practice neurology referral at the post-ED visit (the ED's referral is carried
  through the practice).
- **A6.** Valproate monitoring: level 74 mcg/mL (June 2025), 71 mcg/mL (January 2026);
  hepatic panel and CBC normal at neurology in June 2025.
- **A7.** Hypoventilation recovers within hours on supplemental oxygen and observation;
  no intubation.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Yearly preventive visits; CBC and CMP yearly on valproic acid; lipids once at joining.
- HIV and hepatitis C screening once (2024), nonreactive. Tdap at joining; influenza each
  fall; COVID vaccine each fall. PHQ-2 at preventive visits, negative.
- Refills of valproic acid through the pharmacy, portal messages about results, refills,
  scheduling and work/driving letters, a records request, a cancelled visit.
- Booked visit after 2026-08-27: annual preventive visit 2026-09-29.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.


## Last seen

The practice last saw MRN00002670 on 2025-09-23. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. Nothing is booked after today.
