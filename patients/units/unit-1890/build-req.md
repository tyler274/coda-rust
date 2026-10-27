# Build requirements: unit-1890 (Kayla Andrade, MRN00001890)

Source: synthetic_hospital patient 1890, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its repeated chief-complaint wording.

## Who she is (S)

Born 1999-06-26, female, White, not Hispanic or Latino, English-speaking, lives in
Williamsburg, on a parent's private commercial plan. Graduate student. Never smoker;
occasional social alcohol, 1 to 2 drinks a week. No surgical history. Single.

Family history: mother with allergic rhinitis and eczema; father with asthma; a
younger brother with peanut allergy.

Lifelong atopic history (S): atopic dermatitis, allergic rhinitis, mild persistent
asthma, and peanut allergy known since childhood (R1).

Allergies at joining: peanut (anaphylaxis, April 2025), tree nuts (almond, walnut;
known since childhood, reaction not recorded). Kiwi, banana and avocado are added
after the November 2025 reaction (R2).

Joined the practice on 2025-06-11 (A1), six weeks after her first ED visit, on:
albuterol HFA 90 mcg/actuation, 2 puffs every 4 to 6 hours as needed for wheeze or
shortness of breath; epinephrine auto-injector 0.3 mg IM into the thigh at onset of
anaphylaxis, carry two pens; cetirizine 10 mg daily as needed for rhinitis;
fluticasone propionate nasal spray 2 sprays each nostril daily in allergy season;
triamcinolone acetonide 0.1% cream thin layer twice daily to flares. All prescribed
before she joined; the practice refills them.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2025-04-30 | outside (ED) | About 30 minutes after a slice of cake at a friend's birthday party, sudden trouble breathing, facial swelling and a diffuse itchy rash; worried the cake had nuts; known peanut allergy; on albuterol as needed; no GI or neurologic symptoms. BP 90/60, HR 120, RR 28, SpO2 92% on room air. Anxious, diffuse urticaria, lip angioedema, wheezing. Serum tryptase elevated; CBC normal; electrolytes normal. **Anaphylaxis due to peanut allergy.** Treated and discharged home the same day (R3). The record reaches the practice on 2025-06-24 after a records request; she brings her discharge sheet to the June visit |
| 2025-06-11 | practice | New-patient visit (A1): history as above, ED visit reviewed from her discharge sheet; lists built (problems, allergies, home drugs); exam normal; refills sent, including two epinephrine pens |
| 2025-11-16 | outside (ED) | Within minutes of a mixed fruit salad at a friend's birthday party, throat tightness and lip swelling, then trouble breathing, facial swelling and a diffuse itchy rash; the ED note frames her as having chronic atopic disease and an April 2025 peanut reaction, with mild atopic dermatitis. Allergy list on this record: peanuts, tree nuts (almond, walnut), kiwi, banana, avocado. BP 85/50, HR 120, RR 28, SpO2 92% on room air. Anxious, diffuse urticaria, hoarse voice, bilateral wheezes. Tryptase elevated; CBC normal; electrolytes normal. IV access and IV fluids (S), with treatment completed as R3. **Anaphylaxis due to food.** Discharged home the same day. The practice hears of it from her portal message the next day; the ED record arrives late, on 2026-01-13 (P, records gap) |
| 2025-12-02 | practice | Post-ED follow-up (P; she missed the 2025-11-24 slot). **Kiwi, banana and avocado added to the allergy list** (R2); anaphylaxis due to food added to the problem list; pens replaced |
| 2026-06-05 | practice | Annual physical (P). Last practice visit |
| 2026-06-30 | outside (ED) | Third presentation with suspected anaphylaxis after two in 2025: about 30 minutes after a slice of cake at a friend's birthday party, worried it held peanut oil; trouble breathing, facial swelling, diffuse itchy rash. Background eczema and allergic rhinitis. BP 90/60, HR 110, RR 25, SpO2 92% on room air. Bilateral wheezes, lip angioedema, urticaria over trunk and arms. Tryptase elevated; total IgE elevated; CBC normal; BMP normal. **IM epinephrine given immediately on arrival**; rest of treatment as R3; discharged home the same day. After last seen: the practice learns of it from a pharmacy fill of epinephrine pens the ED prescribed (2026-07-02), and the ED record is filed when it arrives on 2026-08-14. The practice's lists are not changed |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source lists peanut allergy only as an ED comorbidity and puts
  "anaphylaxis due to peanut allergy" on its list from the first encounter, while the
  first ED note already calls the peanut allergy known. Resolution: peanut allergy is a
  childhood diagnosis she brings on joining (with the brother's peanut allergy in the
  family history); the anaphylaxis is the April 2025 episode. Both sit on the practice
  problem list from 2025-06-11 and stay active.
- **R2.** The source profile carries five food allergies from the start, but the first
  and third ED notes record only peanuts and the second lists all five. Resolution:
  peanut and tree nuts (almond, walnut) are known at joining. Kiwi, banana and avocado
  first appear on the November 2025 ED record, after a fruit-salad reaction whose
  culprit fruit no record names; the practice adds all three, with no reaction type,
  at the 2025-12-02 visit. The June 2026 ED record lists only peanuts, as the source
  does; ED allergy lists may be incomplete, as outside lists are.
- **R3.** The source gives no treatment at the first ED visit, only IV access and fluids
  at the second, and only IM epinephrine at the third, and no disposition for any.
  Resolution: each ED record documents standard anaphylaxis care: IM epinephrine 0.3 mg
  (at the third, immediately on arrival), an H1 antihistamine (diphenhydramine IV),
  famotidine IV, nebulized albuterol for the wheeze, IV normal saline for the low
  pressure, observation for several hours with symptoms resolving, and discharge home
  the same day with instructions to avoid the trigger, carry two pens and follow up
  with her PCP. The only vital signs with numbers are the source's arrival set; later
  improvement is stated in words. No steroid course, no admission. The ED records do
  not say whether she used her own auto-injector before arrival, and no note says.
- **R4.** The source dates the earlier reactions to "2020" and "January 2020". Those
  years follow the re-dated timeline: April 2025, and "two in 2025".
- **R5.** The source's diagnosis labels map to the chart as: "Anaphylaxis due to peanut
  allergy" (ICD-10 T78.01XA) recorded at the April 2025 ED visit; "Anaphylaxis due to
  food" (T78.00XA) at the November 2025 ED visit; "Allergy to peanuts" as Peanut
  allergy. The June 2026 episode adds no new problem.
- **Left as the source has it:** mild persistent asthma is treated with albuterol alone;
  no controller inhaler is started, at the practice or in any ED. No allergy referral
  is made after any of the three reactions. Nobody comments on either.

## Additions (A): physician to confirm

- **A1.** The 2025-06-11 new-patient visit: the source starts in an ED and has her on a
  full set of home drugs, so she needs a primary-care home; she joins after the first
  ED visit.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. Practice exams
  find clear lungs and no active eczema.
- Weight about 62 kg, height about 165 cm; blood pressure normal at practice visits.
- Records from a prior practice (received July 2025): childhood atopic history as above,
  childhood vaccines and the HPV series complete, Tdap in 2019, a normal Pap in 2023.
- Influenza and updated COVID vaccines in October 2025 at a nurse visit.
- The 2026-06-05 annual physical: PHQ-2 negative, cervical cytology (Pap, due three years
  after 2023) normal, one-time HIV and hepatitis C screening nonreactive, CBC, CMP and a
  lipid panel, all normal.
- A spring 2026 telehealth visit for allergy-season rhinitis, with refills.
- Refills (triamcinolone for a hand flare by portal, fluticasone, cetirizine, albuterol,
  epinephrine pens), pharmacy traffic, portal messages about results and scheduling,
  the missed 2025-11-24 visit, records requests.
- Nothing booked after today.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00001890 on 2026-06-05. Practice contacts after that day do not
happen; outside care after it still does, and its records are filed as received without
changing the practice's lists. Nothing is booked after today.
