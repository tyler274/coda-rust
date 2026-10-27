# Build requirements: unit-2642 (Mason Dean, MRN00002642)

Source: synthetic_hospital patient 2642, in `source.md` beside this file. This file and
the source are the only authority for clinical facts. Every diagnosis, medication,
result, procedure and specialist finding in the chart must trace to one of:

- **S** a fact in source.md, carried as listed below;
- **R** a resolution below, where the source contradicts itself or leaves a plan unfinished;
- **A** an addition below, listed one by one;
- **P** the primary-care layer described at the end.

Nothing else. No diagnosis, medication, test result, procedure, family member or
life event that is not on these lists. Texture (wording, scheduling, refills, portal
traffic, who signed what) is free, as long as it carries no new clinical fact.

Re-voice everything. The source notes are exam vignettes: never copy a sentence, and
never carry a source clinician's name, its "Active Problem List" dumps, its
"(diagnosed 2020-01-15)" stamps, or its quiz phrasing ("you note").

## Who he is (S)

Mason Dean, born 1989-10-18, male, White, not Hispanic or Latino, English, lives in
Deerfield (01342), private insurance through his employer. Construction worker (boots
all day, sweaty feet). Never smoker. Alcohol socially, 1 to 2 drinks a week. No known
drug allergies. No surgical history. Family history: father with type 2 diabetes,
mother with hypertension. No chronic conditions and no home medications. He jogs
daily (source encounter 2; stated at the 2026-07-31 visit, may appear earlier as
exercise history).

Joined the practice on 2024-09-10 (A1), a healthy man of 34 with no problems and no
medications.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2024-09-10 | practice | new-patient preventive visit (A1): history as above, normal exam, height 180 cm, weight 86 kg, BP normal; PHQ-2 negative; Tdap and influenza vaccine given; screening labs (P) normal |
| 2025-08-05 | practice | rash on both feet for several weeks, itchy and burning, worse after a day in work boots, feet sweaty. Exam: erythema, scaling and fissuring between the toes of both feet; thickened scaly skin over the soles and lateral feet in a moccasin distribution; no vesicles or pustules. KOH prep of scrapings done in the office: branched, segmented hyphae. **Tinea pedis diagnosed.** **Terbinafine 1% cream twice daily for 4 weeks** (R1); dry socks, air out boots |
| 2026-01-20 | practice (portal) | groin rash since early October 2025 that clears with over-the-counter antifungal cream and returns a few weeks after he stops; the foot rash keeps coming back too. PCP places a **dermatology referral** (A2) |
| 2026-02-09 | outside | dermatology consult (letter received): referred for the groin rash; recalls the foot rash worked up in August 2025; 4 months of recurrent itchy red groin rash that improves on OTC antifungal cream and returns a few weeks after stopping; athlete's foot treated several times over the past year; no new sexual partners, no change in hygiene products. Exam: well-demarcated, red, scaly rash in both inguinal folds; scaling and maceration between the toes of both feet; toenails thickened and discolored; no lymphadenopathy. CBC within normal limits; HIV negative; fungal culture of the groin sent. **Tinea cruris diagnosed**; tinea pedis confirmed. **Terbinafine 250 mg by mouth daily for 14 days** (R2) |
| 2026-03-05 | outside | dermatology culture addendum received: fungal culture of the groin rash grew Trichophyton rubrum |
| 2026-03-30 | practice (portal) | he reports both rashes are gone since finishing the terbinafine; PCP marks **tinea cruris and tinea pedis resolved** (R3) |
| 2026-07-31 | practice | last seen. Itchy rash on both feet for two weeks, worse after his daily jogs; the earlier rash had cleared; no recent travel, no sick contacts; takes no medications. Exam: erythema, scaling and maceration in the web spaces of both feet, worst between the fourth and fifth toes; no vesicles or pustules. KOH prep in the office: branching hyphae. **Tinea pedis diagnosed again** (recurrence). **Terbinafine 1% cream twice daily for 2 weeks** (R4) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Encounter 0 ends at the KOH result with no plan. Resolution: the PCP diagnoses
  tinea pedis and prescribes topical terbinafine 1% cream twice daily for 4 weeks with
  foot-hygiene advice. (Moccasin-type disease often needs an oral agent; the PCP starts
  topical, an ordinary choice, and no note revisits it.)
- **R2.** Encounter 1 gives exam and labs but no plan. Resolution: the dermatologist
  treats the recurrent groin and foot infection with oral terbinafine 250 mg daily for
  14 days, with drying and hygiene advice, and sends a fungal culture whose result
  (Trichophyton rubrum) reaches the practice as a later addendum. The CBC and HIV test are
  the dermatologist's workup for recurrent dermatophyte infection and come back with the
  letter.
- **R3.** Encounter 2 says symptoms had previously resolved, and lists tinea pedis and
  tinea cruris as active. Resolution: both resolve after the oral course (portal report,
  2026-03-30); tinea pedis is diagnosed again on 2026-07-31 as a recurrence. Tinea cruris
  stays resolved (the groin is not involved in July 2026).
- **R4.** Encounter 2 ends at the KOH result with no plan; "he takes no medications".
  Resolution: topical terbinafine 1% cream twice daily for 2 weeks. The source's "January
  2020 and July 2020" become August 2025 and February 2026.
- **Source's "athlete's foot treated several times in the past year":** the August 2025
  course, a refill for a flare in November 2025 (P), and OTC antifungal cream he bought
  himself. No other drug.
- **Left as the source has it:** the dermatologist records thickened, discolored toenails
  and nobody names, cultures or treats them. No glucose screen is repeated for recurrent
  fungal infection beyond the routine 2024 A1c. Nobody comments.

## Additions (A): physician to confirm

- **A1.** New-patient preventive visit on 2024-09-10 (seed: about two years with the
  practice), with records from his prior primary care office.
- **A2.** The January 2026 portal exchange in which the PCP places the dermatology
  referral that encounter 1 implies.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Records from his prior primary care office: healthy, Tdap in 2014, routine labs normal.
- Screening labs at joining (2024-09-10): CMP, CBC, lipid panel, hemoglobin A1c (father
  with diabetes) and hepatitis C antibody, all normal or negative. HIV screening is the
  dermatologist's 2026 test.
- Yearly preventive visit (2025-10-07; an earlier 2025-09-23 booking cancelled by him).
  PHQ-2 at preventive visits, negative. Influenza vaccine each fall (2024, 2025).
- Portal messages about results, refills and scheduling; one cream refill; referral
  scheduling call; a portal message after 2026-07-31 about the cream.
- No booked visit after today.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002642 on 2026-07-31. Practice contacts after that day are
messages and refills only; nothing is booked.
