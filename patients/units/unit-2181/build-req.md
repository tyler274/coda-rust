# Build requirements: unit-2181 (Derek Pereira, MRN00002181)

Source: synthetic_hospital patient 2181, in `source.md` beside this file. This file and
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

## Who he is (S)

Derek Pereira, born 1982-01-19, male, White, not Hispanic or Latino, English, lives in
Westfield (01085), employer-based PPO. Corporate sales manager. Never smoker. Alcohol
socially, 2 to 3 drinks on weekends. No known drug allergies. Surgical history:
tonsillectomy at age 7. Family history: father with hypertension and type 2 diabetes,
mother with hypothyroidism, sister with migraine. No chronic conditions and no
medications.

Joined the practice in late 2023 (A1): new-patient visit 2023-11-07, bringing no
problems and no drugs, only the tonsillectomy history.

Records are sparse: outside care reaches the practice as a brief ENT letter, a short
ED note and the patient's own account, with few numbers.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2023-11-07 | practice | new-patient visit (A1): history as above; normal exam; weight 84 kg, height 180 cm, BP normal; baseline labs and once-in-life screens (P) |
| 2024-10-22 | practice | annual physical (P): well, normal exam, routine labs normal, vaccines |
| 2025-06-24 morning | practice | phone call (A2): woke with the right side of his face feeling heavy; cannot brush his teeth properly, spills water when drinking; cannot raise the right eyebrow; no arm or leg weakness, no speech or vision change. Physician arranges a same-day otolaryngology visit (referral to Otolaryngology), with ED instructions for any limb weakness or speech change |
| 2025-06-24 | outside | otolaryngology visit; brief letter received 2025-06-26: right face heavy since waking, trouble brushing teeth and drinking without spilling, food tastes bland, no arm or leg weakness; unremarkable past history; external ear, canal and tympanic membrane normal; complete right-sided facial weakness involving forehead and lower face; normal limb strength and coordination. **Bell palsy, right, diagnosed.** **Prednisone 60 mg daily for 5 days then tapering by 10 mg daily over 5 days started; carboxymethylcellulose eye drops by day and mineral oil/petrolatum ointment at night started** (A3); eye protection; ENT follow-up as needed |
| 2025-07-08 | practice | Bell palsy follow-up (A4): steroid course finished; right face improving: forehead moves a little, right eye closes fully with effort, mouth droop less, taste coming back; using drops and ointment; no eye pain or redness. Steroid course closed; drops and ointment continued as needed |
| 2025-10-10 | practice | annual physical, **last seen** (A5): facial movement fully back, eye closes normally, taste normal; drops and ointment stopped; **Bell palsy resolved**; normal exam; routine labs normal; influenza and COVID vaccines |
| 2026-02-09 | outside | ED visit; short ED note received 2026-02-12: 2 days of sudden right facial asymmetry about eight months after the earlier episode seen by ENT; food collects between teeth and right cheek, biting the inside of the right lip, liquid dribbling from the right corner of the mouth; no head trauma, rash, ear pain or recent upper respiratory infection; no medications. Exam: flattened right nasolabial fold, drooping right mouth corner, incomplete right eye closure on blinking, absent right forehead wrinkling on brow raise; facial sensation to light touch and pinprick intact; hearing grossly normal; other cranial nerves normal. Recurrent right Bell palsy. Prednisone 60 mg daily for 5 days then tapering over 5 days, eye drops and nightly ointment (A6); follow up with ENT and primary care. The practice lists are not updated (after last seen) |
| 2026-02-10 | practice | portal message (P): he tells the physician it happened again and he was seen in the ED and given steroids; the physician replies (eye care, ED precautions, follow ENT advice) |
| 2026-04-28 | practice | portal message (A7): his face is nearly back to normal; physician acknowledges |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The ED note lists Bell palsy as an active problem with a source diagnosis
  stamp, though it calls the visit a recurrence. Resolution: the first episode is
  diagnosed on 2025-06-24 at ENT and resolves by 2025-10-10 (A5); the February 2026
  episode is a recurrence on the same side. The stamp is not carried.
- **R2.** The ENT note gives no assessment or plan. Resolution: the letter's
  assessment is right Bell palsy (the source's primary diagnosis), with the plan in A3.
- **R3.** The source makes the patient walk into an ENT clinic on the morning his
  symptoms began. Resolution: he calls the practice first and the physician gets him a
  same-day ENT slot (A2).
- **Left as the source has it:** no brain imaging, no Lyme or other serology and no
  antiviral, for either episode, including the same-side recurrence. No one orders
  them and no note comments on it.

## Additions (A): physician to confirm

- **A1.** The 2023-11-07 new-patient visit (the seed gives about two years with the
  practice; the source has no primary-care encounter).
- **A2.** The 2025-06-24 phone triage and the Otolaryngology referral that send him to
  the ENT visit; the referral closes when the ENT letter arrives.
- **A3.** Standard Bell palsy care at ENT: prednisone 60 mg daily for 5 days then a
  5-day taper (10 mg less each day), carboxymethylcellulose 0.5% drops by day,
  mineral oil/petrolatum ophthalmic ointment at night, taping or shielding the eye
  for sleep.
- **A4.** The 2025-07-08 practice follow-up with partial recovery as listed.
- **A5.** Full recovery by 2025-10-10; Bell palsy marked resolved; eye drops and
  ointment stopped.
- **A6.** The ED's treatment of the recurrence: the same prednisone course, eye drops
  and nightly ointment, advice to follow up with ENT. These appear only in the ED note
  and the patient's message, never on the practice lists.
- **A7.** The April 2026 message that his face is nearly back to normal.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results:

- Reports and exams state ordinary normal findings for what the source does not
  mention. An abnormal finding appears only as the source or this file gives it.
- Records from a prior practice may mention routine care done there, with normal
  results (vaccinations, a normal physical).
- Yearly physicals with vitals; CBC, metabolic panel, lipids and A1c (father with
  diabetes), all normal; hepatitis C and HIV screening once, negative; PHQ-2 negative.
- Influenza vaccine each fall, COVID vaccine in the fall, Tdap if due (given at the
  new-patient visit). Colorectal screening is not yet due (he turns 45 in January
  2027).
- Portal messages about results, scheduling and an employer wellness form; reminders.
- Nothing booked after today.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002181 on 2025-10-10. Practice contacts after that day are
only messages; outside care after it still happens, and its records are filed as
received without changing the practice's lists. No visit is booked after today.
