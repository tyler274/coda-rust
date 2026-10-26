# Build requirements: unit-2685 (Cameron Mazur, MRN00002685)

Source: synthetic_hospital patient 2685, in `source.md` beside this file. This file and
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
"(diagnosed 2020-09-15)" stamps, or its quiz text.

## Who he is (S)

Cameron Mazur, born 1993-07-01, male, White, not Hispanic or Latino, English,
commercial PPO, lives in Sunderland (01375). Single (no spouse or partner appears;
his brother is the family contact). Construction foreman. Former smoker, 5
pack-years, quit about 2 years before joining (2023). Social alcohol, 2 to 3 drinks
a week. Allergy: penicillin (hives). Surgical history: arthroscopic knee surgery at
age 28 (about 2021-2022; side and findings not given). Family history: father
hypertension, mother hypothyroidism, brother healthy.

Joined the practice 2025-08-20 (new-patient visit, A1), bringing:
- Problems: intermittent mechanical low back pain (catalog "Low back pain"),
  seasonal allergic rhinitis ("Allergic rhinitis due to pollen").
- Drugs: ibuprofen 400 mg as needed for back pain; loratadine 10 mg daily as needed
  for allergies. Both continue throughout.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2025-08-20 | practice | new-patient visit (A1): history as above; exam normal; screening labs and Tdap (P) |
| 2025-10-08 | practice | nurse visit: influenza and COVID vaccines (P) |
| 2025-11-04 | outside | regional hospital ED after a motor vehicle collision with a direct blow to the back: severe back pain, cannot move his legs. BP 130/85, HR 88, RR 18. Flaccid paralysis of both legs, absent deep tendon reflexes; loss of pain and temperature sensation from T10 down; proprioception and vibration intact. MRI spine: anterior wedge compression fracture of the T8 vertebral body with swelling and signal change in the anterior spinal cord. **Anterior cord syndrome diagnosed**; **T8 wedge compression fracture recorded**. Admitted (A2) |
| 2025-11-04 to 2025-11-11 | outside | acute admission, trauma/spine service (A2): nonoperative management in a thoracolumbosacral orthosis (TLSO), mean arterial pressure kept above 85 for the first days, enoxaparin 40 mg daily prophylaxis in hospital. Trace hip flexion returning in both legs by transfer. Transfers to inpatient rehabilitation |
| 2025-11-11 to 2025-12-12 | outside | inpatient rehabilitation (A3): at discharge walks about 150 feet with a rolling walker, wheelchair for longer distances, TLSO when upright; enoxaparin stopped at discharge; home drugs unchanged; outpatient physical therapy and physical medicine and rehabilitation (PM&R) follow-up arranged |
| 2025-12-16 | practice | RN post-discharge call (P) |
| 2025-12-23 | practice | post-discharge (transitional care) visit (A3): arrives in wheelchair with TLSO; hip flexion and knee extension about 3+/5, ankle dorsiflexion 3/5 both sides; pin sensation reduced below T10; CBC and BMP normal; problem list now carries anterior cord syndrome and the T8 fracture |
| 2026-02-11 | outside | PM&R follow-up letter (A4): walking with a single-point cane; TLSO discontinued; standing thoracic spine x-ray shows the T8 anterior wedge unchanged without progression; hip flexion and ankle dorsiflexion 4/5 both sides; reduced pain and temperature sensation below T10 persists; continue outpatient PT |
| 2026-03-18 | practice | follow-up, **last seen**: walking with a cane, outpatient PT continuing, back pain flares on ibuprofen as before, spring allergies on loratadine; not back at work yet (P) |
| 2026-06-29 | outside | regional hospital ED after a motorcycle crash: thrown from the motorcycle and landed on his back (R2). T 36.9 °C (98.4 °F), BP 84/60, pulse 52. Awake and alert, cannot move either leg; complete paralysis of both legs, flaccid, absent deep tendon reflexes; pain, temperature, vibration and proprioception absent below the nipples; anal tone absent; no bulbocavernosus response; arms intact. Foley returns 1.1 L of retained urine. Admitted (A5) |
| 2026-06-29 to 2026-07-24 | outside | acute admission, trauma/spine (A5); discharge summary received; transferred to inpatient rehabilitation, where he remains at today |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source's second note calls him 33 on 2026-06-29; by the seed's birth date
  he is 32 that day (33 two days later). The ED record states 32. The source's
  "September 2020" prior presentation follows the re-dated timeline: November 2025.
- **R2.** The source labels both encounters "anterior cord syndrome", yet the second
  exam is a complete injury at a higher level (all modalities lost below the nipples,
  about T4) with hypotension and bradycardia and absent bulbocavernosus reflex. The
  ED record calls it what the exam shows: a new acute complete thoracic spinal cord
  injury, sensory level T4, with neurogenic shock and spinal shock, and urinary
  retention. Because it falls after last seen, it stays in the outside records and the
  practice's problem list is not changed. The source does not say whether he was
  driving; for someone walking with a cane four months earlier, the chart has him as
  a motorcycle passenger.
- **R3.** Anterior cord syndrome usually leaves heavy motor loss, yet eight months later
  the source puts him on a motorcycle. Resolution: an incomplete injury with
  substantial motor recovery (A3, A4), walking with a cane by February 2026.
- **R4.** The source's ED encounters list home drugs and no others. Hospital drugs
  (enoxaparin, pressors) appear only in hospital records (A2, A5); the practice list
  stays ibuprofen and loratadine.
- **Left as the source has it:** the source records no imaging for the second ED visit
  in its own note; imaging appears only in the discharge summary (A5). No bone density
  or other workup is added after the first fracture.

## Additions (A): physician to confirm

- **A1.** 2025-08-20 new-patient visit (seed: about one year with the practice; the
  first source encounter lists established problems and drugs).
- **A2.** First admission 2025-11-04 to 2025-11-11: nonoperative TLSO management of the
  T8 fracture, MAP goal above 85, in-hospital enoxaparin prophylaxis, early trace hip
  flexion; transfer to rehab. Voiding spontaneously before transfer.
- **A3.** Inpatient rehab 2025-11-11 to 2025-12-12: discharge function as in the
  timeline; enoxaparin stopped at discharge; outpatient PT and PM&R follow-up.
- **A4.** PM&R letter 2026-02-11 with the findings in the timeline, including the
  standing thoracic spine x-ray (T8 wedge unchanged).
- **A5.** Second admission 2026-06-29 to 2026-07-24: CT and MRI of the thoracic spine
  show a T3-T4 fracture-dislocation with cord compression; posterior instrumented
  fusion T1-T6 on 2026-06-30; norepinephrine for neurogenic shock in the ICU, weaned
  by 2026-07-03; enoxaparin prophylaxis; intermittent catheterization for neurogenic
  bladder and a bowel program for neurogenic bowel; at transfer complete (ASIA A) paraplegia with sensory
  level T4. Transfer to inpatient rehab 2026-07-24.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not
  mention. An abnormal finding appears only as the source or this file gives it.
- Records from a prior practice may mention routine care there, with normal results.
- New-patient labs: CBC, CMP, lipid panel, hepatitis C antibody and HIV screen, all
  normal. Tdap at joining. Influenza and COVID vaccines in October 2025.
- PHQ-9 at practice visits, low scores (under 5).
- Refills of loratadine and ibuprofen, portal messages about results and scheduling,
  work and disability paperwork, the brother's calls to the office after each injury.
- Weight about 86 kg, height 180 cm; normal blood pressure at practice visits.
- No booked visit (seed `booked` false).

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002685 on 2026-03-18. Practice contacts after that day do
not happen; outside care after it still does, and its records are filed as received
without changing the practice's lists. No visit is booked.
