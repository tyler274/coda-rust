# Build requirements: unit-1908 (Lisa O'Brien, MRN00001908)

Source: synthetic_hospital patient 1908, in `source.md` beside this file. This file and
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
"(diagnosed ...)" stamps, or its unfinished plan text.

## Who she is (S)

Lisa O'Brien, born 1990-03-15, female, White, not Hispanic or Latino, English,
Blue Cross Blue Shield PPO, lives in Holyoke (01040). Single. Elementary school
teacher. Never smoker. Alcohol occasional, two drinks a month or fewer.

- **Allergies at joining:** dust mites (rhinitis), tree pollen (seasonal rhinitis).
  Latex is recorded only after the 2025 reaction (R1).
- **Surgical history at joining:** tonsillectomy and adenoidectomy at age 8,
  complicated by intraoperative anaphylaxis (cause never established; she has no
  records of it). The nevus excision is a 2025 event (R2).
- **Family history:** mother with asthma and allergic rhinitis; father with atopic
  dermatitis; a younger sister with recurrent cold sores.
- **Joined the practice** 2022-09-08 (A1), from a prior primary-care office.
  Brought: atopic dermatitis (moderate, well controlled on topical steroid), allergic
  rhinitis (seasonal and perennial), recurrent herpes simplex labialis (two to three
  episodes a year), history of intraoperative anaphylaxis (the childhood episode).
  Medications on joining: mometasone furoate 0.1% ointment, thin layer to affected
  areas twice daily as needed for eczema; cetirizine 10 mg by mouth once daily as
  needed; fluticasone propionate nasal spray 1 spray each nostril once daily;
  valacyclovir 1 g by mouth at the first sign of prodrome, then 1 g 12 hours later
  (episodic, cold sores).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2022-09-08 | practice | new patient visit (A1): history as above; exam normal; skin with mild dry patches only; screening labs ordered (P) |
| 2024-08-29 | practice | annual physical (P); she asks to have a mole on her upper back looked at; **dermatology referral** (A2) |
| 2024-10-21 | practice | painful sores on the lips and inside the mouth for 3 days; similar episodes before, usually 2 to 3 a year, lately less often; denies recent sexual activity or new partners. Vitals normal. Several small vesicles on an erythematous base on the vermilion border and a few similar lesions on the buccal mucosa; no cervical lymphadenopathy; rest of exam unremarkable. CBC normal; HSV type-specific serology HSV-1 IgG positive, HSV-2 IgG negative. **Herpes simplex virus infection** (orolabial, HSV-1) diagnosed; **valacyclovir 1 g twice daily for 7 days** (A3) |
| 2025-01-16 | outside | excisional biopsy of the upper-back nevus at an outside dermatologic procedure suite; allergy & immunology record of the event. Back for the procedure after the cold sore was treated. Near the end of the procedure, sudden facial swelling, throat closing, difficulty breathing and widespread itchy rash; she says a similar but milder reaction happened at a past surgery. BP 85/50, HR 120, RR 28, SpO2 88% on room air. Bilateral periorbital edema, diffuse urticaria, wheezing; no JVD, no peripheral edema. IV access, IV fluids; **IM epinephrine 0.5 mg** (R3); transferred to the hospital emergency department, observed, discharged home that evening with an **epinephrine auto-injector 0.3 mg** (R3). **Anaphylaxis** |
| 2025-01-23 | practice | post-event visit (P): her account of the reaction; well now; exam normal; **referral to allergy and immunology** (A4); the auto-injector fill is confirmed with the pharmacy |
| 2025-03-04 | outside | allergy and immunology consult (A4): latex-specific IgE positive, 5.2 kU/L; **latex allergy (anaphylaxis)**; both intraoperative reactions attributed to latex; latex-free precautions for any procedure, dental care included; carry the auto-injector |
| 2025-04-17 | practice | spring rhinitis visit (P); latex (anaphylaxis) added to the allergy list here (list lag); anaphylaxis episode closed on the problem list, history of anaphylaxis stays |
| 2025-10-14 | outside | emergency department, ophthalmology: 3 days of severe pain, redness and blurred vision in the right eye, starting as a foreign-body feeling, then tearing and photophobia. Age 35. Visual acuity reduced in the right eye, 20/60 (R4), left 20/20. Slit lamp: multiple small corneal vesicles and a branching dendritic ulcer that stains with fluorescein; no purulent discharge. Left eye normal. **Herpes simplex keratitis**, right eye; **ganciclovir 0.15% ophthalmic gel** 1 drop right eye 5 times daily until the ulcer heals, then 3 times daily for 7 days (R5) |
| 2025-10-24 | outside | ophthalmology follow-up (A5): epithelium healed, no stromal involvement, right visual acuity 20/20; finish the gel taper |
| 2025-11-06 | practice | post-ED visit (P): eye comfortable, vision back to normal by her report; herpes simplex keratitis added to the problem list |
| 2026-07-30 | practice | annual physical (P); last seen |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source lists latex (anaphylaxis) as an allergy from the first visit, yet
  she has an anaphylactic reaction during a skin procedure. Resolution: latex was
  never identified before 2025. The childhood reaction's cause was never
  established; after the January 2025 reaction the allergy consult finds latex IgE
  positive (A4), and latex is recorded from then on (on the practice list from
  2025-04-17). No earlier note mentions latex.
- **R2.** The source's surgical history lists "excisional biopsy of skin nevus at age 34
  complicated by intraoperative anaphylaxis" from the first encounter, but that is the
  January 2025 event (she is 34 then). Resolution: the nevus excision exists only from
  2025-01-16. The October 2024 visit knows only the childhood tonsillectomy and
  adenoidectomy reaction, and the anaphylaxis episode of 2025 is not on any earlier list.
- **R3.** The source's plan stops after IV access and fluids. Resolution: the record goes
  on with what any team gives for anaphylaxis: IM epinephrine 0.5 mg, transfer to the
  emergency department for observation, discharge the same evening with an
  epinephrine 0.3 mg auto-injector. No other drug, no tryptase, no other numbers.
- **R4.** The source says only that acuity is decreased in the right eye. Resolution:
  right 20/60, left 20/20.
- **R5.** The source gives no keratitis treatment. Resolution: ganciclovir 0.15% gel
  as above (a short course). Valacyclovir stays episodic for the lips.
- **R6.** The source's "Herpes Simplex Virus Infection" is listed with a diagnosis date
  at the cold-sore visit while recurrent herpes labialis is already listed. Resolution:
  recurrent herpes simplex labialis is on the list from joining; the October 2024
  visit records the herpes simplex virus infection (HSV-1 orolabial, serology
  confirmed), resolved at the next visit when the lesions are healed.
- **R7.** The source's "January 2020" and "April 2020" in the ophthalmology history
  become October 2024 and January 2025.
- **R8.** The source's "Anaphylaxis" with a diagnosis date sits beside "History of
  anaphylaxis". Resolution: history of anaphylaxis (the childhood episode) from
  joining; the 2025 episode is recorded as anaphylaxis and closed on 2025-04-17.
- **Left as the source has it:** no long-term oral antiviral suppression is started after
  the keratitis, and no one comments on it. Cetirizine, fluticasone, mometasone and
  episodic valacyclovir continue unchanged throughout.

## Additions (A): physician to confirm

- **A1.** The 2022-09-08 new-patient visit (the cold-sore visit reads as an established
  patient, and the seed puts her about four years with the practice).
- **A2.** The 2024 dermatology referral for the nevus that is excised in January 2025.
  The dermatology consult letter never reaches the practice.
- **A3.** Valacyclovir 1 g twice daily for 7 days at the October 2024 visit (the source
  says she came for the procedure "following treatment for herpes labialis").
- **A4.** The allergy and immunology referral and consult with latex-specific IgE 5.2 kU/L
  (supports the source's latex anaphylaxis allergy).
- **A5.** The ophthalmology follow-up of 2025-10-24 (the usual next step for a dendritic
  ulcer): healed, 20/20.
- **A6.** Surgical pathology of the nevus: benign compound melanocytic nevus, margins
  clear.

## Records (chart texture: gaps)

- Prior primary-care office records arrive in October 2022: cervical cytology with HPV
  co-test in 2021, negative; Tdap in 2019.
- Dermatology consult (late 2024): never received; the practice knows the excision
  plan only from the patient.
- January 2025 reaction: the practice hears first from the patient by portal the next
  day; the pharmacy confirms the auto-injector fill; the allergy & immunology event
  record arrives 2025-02-20; no emergency department note ever arrives.
- Pathology report arrives late, 2025-03-26.
- Allergy consult letter arrives 2025-03-18; latex goes on the practice list only at
  the 2025-04-17 visit.
- Keratitis: patient portal message the next day; the emergency department
  ophthalmology note arrives 2025-10-30; the follow-up letter of 2025-10-24 arrives
  late, 2026-01-08.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Height 165 cm, weight about 62 to 64 kg, normal blood pressure throughout.
- Visits: yearly physicals (2023-09-21, 2024-08-29, 2025-09-18, 2026-07-30) with
  CBC, CMP and lipids; winter eczema flares managed with the same mometasone and
  emollients (visits 2023-01-26 and 2026-02-12); spring rhinitis visits (2024-04-25,
  2025-04-17); a missed visit.
- HIV and hepatitis C screening once (2022), negative. Cervical co-test due again in
  2026: done at the 2026-07-30 physical, negative. PHQ-2 or PHQ-9 at physicals,
  negative. Influenza each fall and the season's COVID vaccine; Tdap from 2019 is current.
- Refills of mometasone, cetirizine, fluticasone and valacyclovir; yearly auto-injector
  renewal after 2025; portal messages about results, refills, scheduling and a school
  form for her epinephrine auto-injector.
- Booked visit after 2026-08-27: 2026-11-05, routine follow-up of eczema and allergies.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00001908 on 2026-07-30. Practice contacts after that day do
not happen; outside care after it still does, and its records are filed as received
without changing the practice's lists. A visit is booked after today.
