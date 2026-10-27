# Build requirements: unit-2612 (Eli Hale, MRN00002612)

Source: synthetic_hospital patient 2612, in `source.md` beside this file. This file and
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

## Who he is (S)

Eli Hale, born 2000-10-10, male, White, not Hispanic or Latino, English-speaking, lives
in Amherst (01002), private PPO through his employer. Sales representative. Never
smoker. Alcohol: social use, 3 to 4 drinks a week. Allergies: no known drug allergies.
Surgical history: wisdom teeth extraction at age 18 (2018 to 2019). Family history:
father with plaque psoriasis, paternal uncle with ankylosing spondylitis, mother with
hypothyroidism.

Joined the practice in January 2024 (A1: new-patient visit 2024-01-30, age 23). He
brought plaque psoriasis (on clobetasol propionate 0.05% ointment to plaques twice
daily as needed and ketoconazole 2% shampoo twice weekly for the scalp, both started at
his former practice) and seasonal allergic rhinitis (no drug for it).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2024-01-30 | practice | new-patient preventive visit (A1): history as above; psoriasis plaques on elbows and knees, scalp flaking, controlled on the two topicals; rhinitis in spring and fall; normal exam; screening labs normal (P), HIV and hepatitis C screening negative (P) |
| 2024-02-20 | outside records | former primary care office records received: psoriasis care there, Tdap 2019 (P) |
| 2025-02-04 | practice | psoriasis follow-up (P): a few small plaques, scalp quiet; both topicals continued |
| 2025-12-26 | practice | **last seen**: annual physical (P), normal exam, routine labs normal; lists unchanged |
| 2026-01-09 | outside: urology | 3 days of burning with urination and clear penile discharge; unprotected intercourse with a new partner two weeks before; no travel or sick contacts, never had this before. Afebrile, vitals normal. Clear mucoid urethral discharge; no lesions, redness or lymphadenopathy. Urinalysis: no bacteria, no white cells. NAAT sent on the urethral specimen. Addendum 2026-01-12: **Chlamydia trachomatis NAAT positive, gonorrhea NAAT negative (A2)**; **chlamydial urethritis diagnosed (R1)**; **doxycycline 100 mg twice daily for 7 days (A2)**; partner notification and 7 days of abstinence advised. Letter received 2026-01-14 |
| 2026-02-12 | outside: rheumatology | age 25; chlamydia treated about a month ago; two weeks of painful urination, a red painful right eye, and sudden pain and swelling of the left knee and right ankle; no trauma. Sexually active with multiple partners, condoms not consistent. Vitals normal. Right eye injected; purulent urethral discharge; left knee and right ankle swollen and warm; no skin lesions (R3). ESR 50 mm/hr, CRP 25 mg/L, urinalysis with pyuria, urethral swab negative for gonorrhea, HLA-B27 positive. **Reactive arthritis diagnosed.** **Naproxen 500 mg twice daily with food started (A3)**; repeat chlamydia NAAT negative (A3); same-week ophthalmology (A4). Family history in the letter includes the uncle's ankylosing spondylitis (R2). **Chlamydial urethritis resolved** (treated, NAAT negative). Letter received 2026-02-19 |
| 2026-02-16 | outside: ophthalmology | (A4) right eye: conjunctival injection, visual acuity 20/20 both eyes, cornea clear, anterior chamber quiet with no cells or flare, normal pressures and fundi. Conjunctivitis of reactive arthritis, no uveitis; preservative-free artificial tears as needed. Letter received 2026-02-24 |
| 2026-04-16 | outside: rheumatology | follow-up (A5): eye and urinary symptoms gone; knee and ankle much better, mild ankle swelling only; ESR 26 mm/hr, CRP 7.8 mg/L; CMP and CBC normal on naproxen; naproxen continued. Letter received 2026-04-21 |
| 2026-07-09 | outside: rheumatology | follow-up (A5): no joint pain or swelling for several weeks; normal joint exam; ESR 11 mm/hr, CRP 2.1 mg/L; **naproxen stopped**; reactive arthritis in remission; return as needed if symptoms recur. Letter received 2026-07-14 |
| 2026-09-10 | practice | booked: follow-up visit after the rheumatology care (P) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source labels the infection "Chlamydia trachomatis infection (Cervicitis)",
  which cannot apply to a man. The chart records chlamydial urethritis (ICD-10 A56.01),
  diagnosed at the urology addendum on 2026-01-12, and never uses "cervicitis".
- **R2.** The rheumatology note says he denies any family history of arthritis, while the
  profile lists a paternal uncle with ankylosing spondylitis. The chart carries the
  uncle's ankylosing spondylitis everywhere, including the rheumatology letter; the
  denial is not carried.
- **R3.** Plaque psoriasis is a standing diagnosis, yet the rheumatology exam finds no skin
  lesions. The chart reads it as psoriasis quiet in February 2026 (and no keratoderma or
  penile lesions); psoriasis stays on the list.
- **R4.** The source's problem list carries the chlamydia infection into the rheumatology
  visit; the chart marks it treated and resolved there, after the negative repeat NAAT.
- **Left as the source has it:** no drug for allergic rhinitis is ever prescribed; no HIV
  or syphilis testing is done with the STI care at urology or rheumatology (the practice
  screened HIV once in 2024). Nobody comments on either.

## Additions (A): physician to confirm

- **A1.** The 2024-01-30 new-patient visit: he has two years with the practice, and the
  source starts him at specialists.
- **A2.** The NAAT result (chlamydia positive, gonorrhea negative) and doxycycline 100 mg
  twice daily for 7 days at urology: the rheumatology note says he was treated for
  chlamydia a month earlier, and its gonorrhea-negative swab fits.
- **A3.** Naproxen 500 mg twice daily with food (standard first-line treatment of reactive
  arthritis) and a repeat chlamydia NAAT, negative, at the rheumatology consult.
- **A4.** Ophthalmology on 2026-02-16 for the red painful eye, findings as in the timeline:
  the usual next step for a red eye in reactive arthritis.
- **A5.** Two rheumatology follow-ups (April and July 2026) with ESR 26 / CRP 7.8 and
  ESR 11 / CRP 2.1, naproxen continued then stopped, remission: the usual course of
  reactive arthritis to today.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Records from his former primary care office may mention routine care there with normal
  results (Tdap 2019).
- Psoriasis visits about yearly; topical refills; a yearly physical with CBC, CMP and lipids
  (normal), height about 180 cm, weight about 79 kg, normal blood pressure.
- HIV and hepatitis C screening once (2024), negative. Depression screens (PHQ-2) negative.
- Influenza vaccine each fall; COVID vaccine in the fall.
- Refills, pharmacy traffic, portal messages about results and scheduling, a cancelled
  visit; after 2025-12-26 only messages, refills and outside records filed as received.
- Booked visit after 2026-08-27: 2026-09-10 follow-up after rheumatology care.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002612 on 2025-12-26. Practice contacts after that day do not
happen; outside care after it still does, and its records are filed as received without
changing the practice's lists. A visit is booked after today. So the chlamydial urethritis,
the reactive arthritis, doxycycline and naproxen live in the outside letters' text and
results only; the practice's problem and medication lists stay as of 2025-12-26
(psoriasis, allergic rhinitis, clobetasol, ketoconazole shampoo).
