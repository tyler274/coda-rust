# Build requirements: unit-1928 (Kelly Langevin, MRN00001928)

Source: synthetic_hospital patient 1928, in `source.md` beside this file. This file and
the source are the only authority for clinical facts. Every diagnosis, medication,
result, procedure and specialist finding in the chart must trace to one of:

- **S** a fact in source.md, carried as listed below;
- **R** a resolution below, where the source contradicts itself or leaves a value open;
- **A** an addition below, listed one by one;
- **P** the primary-care layer described at the end.

Nothing else. No diagnosis, medication, test result, procedure, family member or
life event that is not on these lists. Texture (wording, scheduling, refills, portal
traffic, who signed what) is free, as long as it carries no new clinical fact.

Re-voice everything. The source notes are exam vignettes: never copy a sentence, and
never carry a source clinician's name, its "Active Problem List" dumps, its
"(diagnosed 2020-01-15)" stamps, or its copied-forward HPI text.

## Who she is (S)

Born 1990-03-09, female, White, not Hispanic or Latino, English-speaking, Blue Cross
Blue Shield, lives in Easthampton (01027). Self-employed landscape gardener. Never
smoker. Alcohol socially, 1 to 2 glasses of wine a week. No known drug allergies. No
surgical history. No chronic conditions. Family history: mother with seasonal
allergic rhinitis; maternal uncle with atopic dermatitis.

Joined the practice on 2024-09-10 (A1, a new-patient visit), age 34. She brought no
active problems and one drug: combined oral contraceptive, **levonorgestrel 0.15 mg /
ethinyl estradiol 0.03 mg tablet, 1 tablet by mouth daily** (R1), prescribed before
she joined; the practice continues it (refills).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2024-09-10 | practice | new-patient visit (A1): history as above; normal exam; height 168 cm, weight 64 kg, BP normal; PHQ-2 negative; labs: CBC, CMP, lipid panel, HIV and hepatitis C screening, all normal/nonreactive (P); Tdap given (P); oral contraceptive continued; prior records requested |
| 2025-02-02 | practice (portal, Sunday) | P: she writes that a very itchy rash came up on both hands and wrists the day after she switched to a new brand of hand soap (onset 2025-02-01); physician tells her to stop the soap and arranges dermatology for Monday; **dermatology referral placed** |
| 2025-02-03 | outside | dermatology consult (S, enc 0): 2 days of itchy rash of hands and wrists beginning a day after a new hand soap; no new drugs, foods or travel; exam: erythema, edema and multiple small vesicles over the backs of both hands and wrists, no mucosal lesions, no systemic involvement, afebrile, rest of exam normal; CBC within normal limits; serum IgE not elevated, **43 IU/mL** (R2). **Allergic contact dermatitis** (hand soap). Plan: stop the soap, **triamcinolone 0.1% cream twice daily for 2 weeks** (A2), fragrance-free emollient, gloves for wet work. Letter reaches the practice 2025-02-11 |
| 2025-05-01 | outside | ED visit (S, from enc 1 history): slipped in her garden and cut her left forearm; superficial laceration cleaned and dressed (R3), no sutures. **No ED record ever reaches the practice** (records gap); the practice learns of it from her |
| 2025-05-05 | practice (portal) | P: she reports the cut and the ED visit; wound healing well; since 2025-05-03 a red, itchy patch where the dressing sat; no fever. Physician: no sign of infection by her account; advises she go back to her dermatologist; **laceration of left forearm recorded** |
| 2025-05-06 | outside | dermatology follow-up (S, enc 1): 3 days of red itchy rash on the left forearm starting 2 days after the ED visit; spreading a little but staying inside a rectangular area; laceration healed well; no fever, chills or systemic symptoms; exam: well-demarcated erythematous lichenified plaque with fine scaling and mild excoriations on the dorsal forearm in a rectangular distribution; no warmth, fluctuance or regional lymphadenopathy. Allergic contact dermatitis at the dressing site (R4). **Triamcinolone 0.1% cream twice daily for 2 weeks** again (A2); avoid that type of dressing (R4). **The letter does not arrive**: the practice requests it 2025-07-22 and receives it 2025-08-05 |
| 2025-05-13 | practice | P: short office visit to look at the arm: laceration healed (**laceration resolved**); forearm patch fading on the cream; vitals normal; Tdap current from 2024 |
| 2025-11-13 | practice | P: annual physical, **last seen**: well; no new symptoms; skin clear; BP normal, weight about 64 kg; PHQ-9 low; CBC and CMP normal; influenza and COVID vaccines; oral contraceptive renewed for a year. Allergic contact dermatitis stays on the problem list as a history item |
| 2026-05-06 | outside | ED visit (S, from enc 2 history): a second slip in the garden, another superficial laceration of the left forearm, cleaned and dressed (R3, R5). No ED record reaches the practice |
| 2026-05-09 | practice (portal) | P: she writes that she cut the same arm again, was seen in the ED on 05-06, and the same rectangular rash started at the dressing on 05-08; she has booked her dermatologist herself. Physician acknowledges and asks her to have the letter sent. No list changes (after last seen) |
| 2026-05-11 | outside | dermatology follow-up (S, enc 2): 3 days of red itchy rash on the left forearm starting 2 days after the ED visit, spreading slightly within a rectangular area; laceration healed well; no fever, chills or systemic symptoms; same exam as 2025-05-06 (well-demarcated erythematous lichenified plaque, fine scale, mild excoriations, dorsal forearm, rectangular; no warmth, fluctuance or nodes). Recurrent allergic contact dermatitis at the dressing site (R4). **Triamcinolone 0.1% cream twice daily for 2 weeks** (A2); advises asking for non-adhesive dressings in future (A3). Letter received 2026-06-03 and filed without changing the practice's lists |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source gives the contraceptive as "ethinyl estradiol/levonorgestrel, 1
  tablet daily" with no strength. Resolution: levonorgestrel 0.15 mg / ethinyl
  estradiol 0.03 mg, the common monophasic strength, the same throughout.
- **R2.** The source gives serum IgE only as "not elevated". Resolution: one value,
  43 IU/mL, within the reference range. The CBC is "within normal limits" with no
  numbers: code fills normal values.
- **R3.** Encounter 2 says "laceration repair" where encounter 1 says the wound was
  cleaned and dressed. Resolution: both ED visits cleaned and dressed a superficial
  cut without sutures.
- **R4.** The source names allergic contact dermatitis as the diagnosis without saying
  what caused the forearm rash. Resolution: the dermatologist reads the rectangular
  plaque at the wound dressing as allergic contact dermatitis to the dressing or its
  adhesive. No specific allergen is named or tested.
- **R5.** Encounter 2 copies encounter 1's history almost word for word but says she was
  seen before "with a similar rash following emergency department laceration repair",
  so it is a second, separate episode a year later. Resolution: a second gardening
  fall, ED visit and dressing reaction in May 2026, with the same findings. The
  source's years ("April 2020", "January 2020") follow the re-dated timeline (May
  2025, February 2025).
- **R6.** "Lichenified" after 3 days is the dermatologist's wording at both forearm
  visits; the chart keeps it as each letter's description.
- **Left as the source has it:** no patch testing is ever done, no allergen is
  entered on the allergy list (it stays no known drug allergies), and no one
  remarks on either.

## Additions (A): physician to confirm

- **A1.** 2024-09-10 new-patient visit (about two years with the practice by today).
- **A2.** Triamcinolone acetonide 0.1% cream twice daily for 2 weeks at each
  dermatology visit, the standard treatment for the allergic contact dermatitis the
  source diagnoses but never treats. Prescribed by dermatology (short courses).
- **A3.** The 2026 dermatology advice to ask for non-adhesive dressings, the usual
  counsel after a second reaction at a dressing site.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not
  mention. An abnormal finding appears only as the source or this file gives it.
- Records from a prior practice may mention routine screening done there: a normal
  cervical cytology with negative HPV co-test in 2022 (next due 2027), childhood
  vaccines complete.
- Healthy young adult: a yearly physical; labs as above; influenza vaccine each fall
  (2024 at a nurse visit, 2025 at the physical); COVID vaccine each fall; Tdap once
  in 2024; HIV and hepatitis C screening once; depression screens.
- Oral contraceptive refills, pharmacy traffic, records requests, portal messages
  about results and scheduling, a physical reminder.
- Nothing booked after 2026-08-27.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00001928 on 2025-11-13 (annual physical). Practice visits
after that day do not happen; the May 2026 ED visit and dermatology visit still do,
and the practice hears of them by portal message and letter, filed as received
without changing its lists. No visit is booked.
