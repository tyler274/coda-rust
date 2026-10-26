# Build requirements: unit-1912 (Taylor Coolidge, MRN00001912)

Source: synthetic_hospital patient 1912, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz text.

## Who she is (S)

Taylor Coolidge, born 1999-09-10, female, White, not Hispanic or Latino, English,
private PPO, lives in Northampton MA 01060, single. Swim instructor (teaches lessons
and is in the pool most working days). Never smoker; social alcohol, 1 to 2 drinks a
week. Surgical history: wisdom teeth extraction at age 18 (2017 to 2018). Family
history: mother with asthma and allergic rhinitis; father with atopic dermatitis;
maternal uncle with shellfish allergy. Environmental allergies: dust mites, seasonal
pollen (no reaction detail in the source; the chart records them as text allergens
without a named reaction and does not turn them into a diagnosis). No drug allergies.

Joined the practice on **2022-09-13** (A1), a new-patient visit at age 23. She brought:
atopic dermatitis, mild and intermittent, since childhood; triamcinolone acetonide
0.1% cream, thin layer twice daily as needed for flares; petroleum jelly to dry skin
daily for maintenance. No shellfish allergy is known at joining (R1).

Records: complete. Every outside letter, ED record and consult reaches the practice
within a few days, and the practice lists are kept current.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2022-09-13 | practice | new-patient visit and preventive exam (A1, P): history as above; normal exam; height 168 cm, weight 62 kg; baseline labs and screens (P); Tdap given (P) |
| 2022-09-20 | outside records | prior primary-care records received (P): childhood series and HPV series complete, Tdap 2011, Pap 2021 normal |
| 2025-06-05 | practice | same-day visit (P, the referral visit): left ear pain since 2025-06-03 (three days by the ENT visit), worse with pulling on the earlobe, fullness and decreased hearing on the left; no fever, no upper respiratory symptoms, no recent illness; swims and dives most days for work. Vitals normal. Left canal red and swollen with greenish discharge, TM not visible through the swelling, tragal tenderness, otoscopy painful. **Acute diffuse otitis externa, left, diagnosed.** Canal too swollen for drops to reach; urgent ENT appointment for the next morning for cleaning and a wick (no drug started today) |
| 2025-06-06 | outside (ENT) | otolaryngology consult, letter received 2025-06-09: same history and findings as above (canal erythematous and edematous, greenish discharge, TM not clearly seen, tragal tenderness, painful otoscopy); vitals within normal limits. Acute diffuse otitis externa of the left ear. Plan (A2): microscopic suction cleaning of the canal and a wick placed; **ciprofloxacin-dexamethasone otic suspension 4 drops left ear twice daily for 7 days started**; keep the ear dry, out of the pool for 10 days; return if not improving in 72 hours |
| 2025-06-12 | practice (portal) | she reports the wick fell out on day 3, pain nearly gone, hearing back (A2) |
| 2025-08-17 | outside (ED) | ED record, received 2025-08-18. Age 25. About 30 minutes after eating shrimp at dinner at a friend's house: sudden difficulty breathing, facial swelling, diffuse itchy rash; has eaten shrimp before without trouble; no new medication, no insect bite. Mentions the June ear infection, now better (R3). BP 90/50, HR 110, RR 25, SpO2 92% on room air. Anxious, audible wheezing, face visibly swollen, urticaria on trunk and arms. Serum tryptase elevated, 21.8 ng/mL (R5); CBC and electrolytes normal. **Anaphylaxis due to shellfish (shrimp) diagnosed; shellfish allergy, anaphylaxis, recorded** (R1). Treatment and discharge as A3. **Epinephrine 0.3 mg autoinjector two-pack started** (A3) |
| 2025-08-19 | practice | post-ED visit (P): feels back to normal; filled the autoinjectors; avoidance of shrimp and all crustacean shellfish reviewed; when and how to use the autoinjector and to call 911 after; **allergy/immunology referral placed** (A4). Left ear canal clear, TM intact and normal: **otitis externa resolved** |
| 2025-11-18 | outside (allergy) | the first new-patient allergy appointment is cancelled by the allergy office and rebooked for April 2026 (texture: front-desk note) |
| 2026-02-02 | outside (ED) | ED record, received 2026-02-03. Age 26 (R4). Recurrent episode like her August 2025 one (R2): at a seafood restaurant with friends, about 15 minutes after eating, sudden facial swelling, shortness of breath and diffuse itchy rash; shrimp was in the dish (R6); no insect sting, no new medication, no new personal-care product. Used her own autoinjector at the table (A5); brought in by ambulance. T 37.0 C, BP 92/60, HR 118, RR 24, SpO2 93% on room air. Alert, anxious, dyspneic; periorbital edema, urticaria across chest and arms, inspiratory stridor, bilateral wheezing. IV placed, oxygen by face mask (S); further treatment, overnight observation and discharge 2026-02-03 as A5 |
| 2026-02-05 | practice | post-ED visit (P): recovered; autoinjector replaced; practice asks the allergy office to move her appointment up (A4) |
| 2026-03-10 | outside (allergy) | allergy/immunology consult letter, received 2026-03-12 (A4): two reactions to shrimp, the second needing two doses of epinephrine; skin prick testing to shrimp positive, wheal 9 mm (histamine control 6 mm, saline control 0 mm). Shellfish (shrimp) allergy with anaphylaxis. Strict avoidance of all crustacean shellfish; carry two autoinjectors at all times; written anaphylaxis action plan; allergy follow-up in a year |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The profile lists shellfish anaphylaxis from the first encounter, yet the
  August 2025 ED visit says she had eaten shrimp before without a problem.
  Resolution: the shellfish allergy and the anaphylaxis diagnosis are first recorded at
  the 2025-08-17 ED visit. The June 2025 practice visit and ENT letter list only dust
  mites and seasonal pollen. The maternal uncle's shellfish allergy is family history
  known from joining.
- **R2.** The 2026 ED note's "similar to her March 2020 presentation" follows the
  re-dated timeline: her August 2025 episode.
- **R3.** The August 2025 ED note keeps acute otitis externa on the active list two
  months on. Resolution: the otitis externa was treated in June and had resolved; the
  ED record mentions it only as a recent ear infection, now better, and the practice
  records it resolved on 2025-08-19.
- **R4.** The source calls her 25 at the February 2026 ED visit. Resolution: she is 26
  then (born 1999-09-10); 25 at the June and August 2025 contacts.
- **R5.** The source gives the tryptase only as "elevated". Resolution: the ED report
  carries a value, 21.8 ng/mL (reference up to 11.4). CBC and electrolytes are normal
  as the source says.
- **R6.** The February 2026 note has her eating shrimp again six months after a known
  shrimp anaphylaxis. Resolution: she ordered a dish she was told contained no
  shellfish, and shrimp turned out to be in it. The ED record states this; nobody
  comments further.
- **R7.** The ENT note says she came "in July"; the visit is 2025-06-06. The month is
  dropped. The ENT note ends without a plan: the plan is A2.
- **R8.** The February 2026 ED plan stops after the IV and oxygen. The rest of the visit
  is A5.
- **Left as the source has it:** no hearing test is done for the decreased hearing in
  the swollen ear; nobody orders one. The source says nothing of drying drops or ear
  protection beyond the ENT plan; the practice gives ordinary advice about keeping
  ears dry and nothing else.

## Additions (A): physician to confirm

- **A1.** New-patient visit 2022-09-13 (the practice's 4 years with her; she arrives
  with a chronic skin condition and a home regimen already in place).
- **A2.** ENT treatment of the otitis externa (the source gives no plan): suction
  cleaning, a wick, ciprofloxacin 0.3% / dexamethasone 0.1% otic suspension 4 drops
  twice daily for 7 days, dry-ear and no-swimming precautions for 10 days. It
  resolves: wick out on day 3 and symptoms gone within a week (2025-06-12 portal);
  normal canal and TM on 2025-08-19.
- **A3.** First ED visit treatment (the source gives none): epinephrine 0.5 mg IM to
  the thigh on arrival; IV normal saline 1 L; diphenhydramine 50 mg IV; famotidine
  20 mg IV; methylprednisolone 125 mg IV; one albuterol nebulizer for the wheeze.
  Tryptase drawn about an hour after arrival. Reassessed 4 hours in: BP 108/68, HR 92,
  RR 18, SpO2 98%, swelling and hives fading, no wheeze. Discharged after about 6
  hours of observation: BP 114/72, HR 84, RR 16, SpO2 99%, T 36.8 C. Discharge
  prescription: epinephrine 0.3 mg autoinjector two-pack, use for any recurrence and
  call 911; follow up with primary care and an allergist.
- **A4.** Allergy/immunology referral at the 2025-08-19 visit (standard after food
  anaphylaxis). The consult happens on 2026-03-10 after the allergy office cancels
  and rebooks the first slot and the practice asks for it to be moved up after the
  second reaction. Skin prick to shrimp positive, wheal 9 mm, histamine 6 mm, saline
  0 mm. Plan as in the timeline. No other allergens are tested and no new diagnosis is
  made.
- **A5.** Second ED visit treatment beyond the source's IV and oxygen: her own
  epinephrine autoinjector given at the restaurant at onset; in the ED a second
  epinephrine 0.5 mg IM, nebulized racemic epinephrine for the stridor, one albuterol
  nebulizer, IV normal saline 2 L, methylprednisolone 125 mg IV, diphenhydramine 50 mg
  IV, famotidine 20 mg IV. Stridor gone within the hour. Reassessed at about 2 hours:
  BP 106/66, HR 100, RR 20, SpO2 97% on room air. Observed overnight without
  recurrence; discharged 2026-02-03 in the morning: T 36.7 C, BP 112/70, HR 78, RR 16,
  SpO2 99% on room air. Discharge: a new autoinjector two-pack prescription to replace
  the one used; primary care in a few days; allergy follow-up.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not
  mention. An abnormal finding appears only as the source or this file gives it.
- Records from the prior practice may mention routine screening done there, with
  normal results (Pap 2021 normal; HPV series complete; Tdap 2011).
- Yearly preventive visits (fall 2023, 2024 and 2025) with vitals, PHQ-2 or PHQ-9
  (normal), alcohol and tobacco history. Baseline labs once at joining: CBC, CMP,
  lipid panel, all normal (lipid levels: total cholesterol about 168, HDL about 63, triglycerides about 74, LDL about 90 mg/dL); HIV and hepatitis C screening once, nonreactive.
  Chlamydia and gonorrhea NAAT yearly while under 25 (2022, 2023, 2024), negative.
  Cervical cytology at the 2024 physical, normal (NILM); next due 2027.
- Immunizations: Tdap at joining; influenza each fall (2022 to 2025); COVID vaccines
  in fall 2022, 2023 and 2024 (she declines in 2025).
- Atopic dermatitis care: refills of triamcinolone 0.1% cream as needed; one winter
  flare visit (hands and inner elbows, worse in winter and with pool time) managed
  with the same cream and emollients, no new drug.
- Weight steady around 62 kg; blood pressure normal throughout (outside the ED
  readings above).
- Refills, pharmacy traffic (autoinjector fills and an expiry reminder), portal
  messages about results and scheduling, a work form for her employer's aquatic
  staff emergency action plan, a missed or rescheduled visit, reminders.
- Booked visit after 2026-08-27: annual physical on 2026-10-20.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.
