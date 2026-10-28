# Build requirements: unit-2679 (Silas Nolan, MRN00002679)

Source: synthetic_hospital patient 2679, in `source.md` beside this file. This file and
the source are the only authority for clinical facts. Every diagnosis, medication,
result, procedure and outside finding in the chart must trace to one of:

- **S** a fact in source.md, carried as listed below;
- **R** a resolution below, where the source contradicts itself or stops short;
- **A** an addition below, listed one by one;
- **P** the primary-care layer described at the end.

Nothing else. No diagnosis, medication, test result, procedure, family member or
life event that is not on these lists. Texture (wording, scheduling, refills, portal
traffic, who signed what) is free, as long as it carries no new clinical fact.

Re-voice everything. The source notes are exam vignettes: never copy a sentence, and
never carry a source clinician's name, its "Active Problem List" dumps, its
"(diagnosed ...)" stamps, or its "x-ray is shown" quiz text.

## Who he is (S)

Born 2001-02-20, male, White, not Hispanic or Latino, English-speaking, single,
graduate student, on a university student health insurance plan. Tall, thin build
(height 188 cm, weight about 68 kg, steady throughout; P). Current smoker, half a pack
a day, started at about 18 (about 5 years by joining, 6 years by 2025; R6). Alcohol
1 to 2 drinks a month. No known drug allergies.

Surgical history: right needle decompression and right chest tube, both for a prior
right spontaneous pneumothorax (R1). Family history: father with recurrent spontaneous
pneumothorax; paternal grandmother with Birt-Hogg-Dubé syndrome. No other family
history is recorded.

Joined the practice 2024-09-10 (A1, a new-patient visit). Brought: history of
spontaneous pneumothorax (one prior right-sided episode, R1), nicotine dependence on
cigarettes (the source's "tobacco use disorder", R7), and nicotine polacrilex gum 2 mg
chewed as needed for cravings.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2024-09-10 | practice | new-patient visit (A1): history as above; exam normal, lungs clear and equal; nicotine gum 2 mg as needed continued and prescribed; cessation counseled, not ready to set a date; records from the hospital that treated the prior episode requested |
| 2024-10-22 | outside records | prior hospital's discharge summary arrives (R1): right spontaneous pneumothorax March 2022, needle decompression then right chest tube, lung re-expanded, home after a few days |
| 2025-01-14 | practice | telehealth tobacco check-in (P): still half a pack a day; uses the gum a few times a week |
| 2025-04-24 | outside | ED: sudden sharp right chest pain and breathlessness that began while walking briskly to class, worse on breathing in, not eased by rest; tall, thin, anxious, short of breath; T 98.7 °F, HR 112, RR 26, BP 102/66, SpO2 91% on room air; absent right breath sounds, right hyperresonance, trachea deviated to the left. Chest x-ray: large right pneumothorax with the mediastinum shifted to the left (R3). **Tension pneumothorax.** Needle decompression on the right, then a right chest tube, admitted; tube out and lung re-expanded on film; home 2025-04-27 (A2). CBC and basic metabolic panel normal (P) |
| 2025-04-28 | practice | patient's portal message: he was in the hospital for a collapsed lung again |
| 2025-05-06 | practice | post-hospital office visit (A5): breathing back to normal; lungs clear and equal; SpO2 normal; **tension pneumothorax recorded on the problem list (onset 2025-04-24)**; cessation counseled again; still no records from the ED |
| 2025-06-19 | outside records | the April ED record, chest x-ray reports and discharge summary arrive late (records texture) |
| 2025-08-29 | practice | preventive visit, last seen (P): no breathing complaints; still half a pack a day; exam normal; CBC and CMP drawn |
| 2025-11-01 | outside | ED, about six months after the April visit: sudden sharp right chest pain and severe breathlessness that began while walking; no trauma, no recent infection (R2); BP 92/58, HR 122, RR 28, SpO2 86% on room air; jugular venous distension, right hyperresonance, markedly reduced right breath sounds and reduced right tactile fremitus, left lung clear. Chest x-ray: large right pneumothorax with leftward mediastinal shift (R3). Tension pneumothorax again: needle decompression, right chest tube, admitted, home 2025-11-05 (A3). CBC and basic metabolic panel normal (P) |
| 2025-11-10 | practice | patient's portal message: another collapse, hospital stay; will come in after the semester |
| 2025-12-16 | outside records | the November ED record and its chest x-ray reports arrive; the discharge summary never does (records texture) |
| 2026-01-23 | outside | third ED visit: during placement of a right subclavian central venous catheter in the ED he developed mild shortness of breath; anxious; BP 92/58, HR 118, RR 24, SpO2 91% on room air; reduced right breath sounds, no jugular venous distension. Chest x-ray after the complaint: right pneumothorax after the line (A4). Small-bore right chest tube, admitted, home 2026-01-26 (A4) |
| 2026-01-28 | practice | patient's portal message: third hospital stay, lung went down after a line |
| 2026-03-10 | outside records | the ED procedure note for the line and the chest x-ray report arrive; the ED provider note and discharge summary never do (R4) |
| 2026-09-10 | practice | booked follow-up visit (after today) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source lists the needle thoracostomy and chest tube as from a "previous
  episode" with no date. Resolution: one prior right spontaneous pneumothorax in March
  2022 (age 21), treated at an outside hospital with needle decompression then a chest
  tube; the chart learns of it from the patient at joining and from the discharge
  summary that arrives in October 2024. History of spontaneous pneumothorax is on the
  list from joining (onset 2022-03).
- **R2.** The November 2025 history says he has "no history of ... smoking". He is a
  current smoker on every other record. Resolution: the denial is dropped; the
  November ED record calls him a current smoker. The absence of trauma and recent
  infection stays.
- **R3.** The source says chest x-rays were "obtained and shown" in April and November
  2025. Resolution: each ED record carries a radiology read of a large right
  pneumothorax with the mediastinum shifted to the left, and a later film with the
  tube in place and the lung re-expanded. No other findings.
- **R4.** The January 2026 source gives no reason for the central venous catheter and
  stops before the chest x-ray. Resolution: the practice never receives the ED provider
  note or a discharge summary (records texture), so the chart does not know why the
  line was placed; the procedure note it does receive gives the indication only as
  central venous access. The chest x-ray and its course are A4.
- **R5.** Only the April 2025 episode reaches the practice's lists (tension
  pneumothorax, diagnosed at the 2025-05-06 visit). The November 2025 and January 2026
  episodes come after the last visit; their records are filed as received and the
  lists are not updated.
- **R6.** "6 years" of smoking in 2025: started about 2019 at 18. At joining in 2024
  it is about 5 years, about 2.5 pack-years.
- **R7.** The source's "tobacco use disorder" is recorded as nicotine dependence,
  cigarettes, uncomplicated.
- **Left as the source has it:** after the April 2025 episode (his second) and the
  November episode (his third spontaneous one), nobody refers him to thoracic surgery,
  orders a CT chest, discusses surgical prevention, or evaluates him for
  Birt-Hogg-Dubé syndrome despite the family history. Cessation stays counseling plus
  nicotine gum; no other cessation drug. No note comments on any of this.

## Additions (A): physician to confirm

- **A1.** 2024-09-10 new-patient visit (the first source encounter already carries a
  problem list, surgical history and home medication, so he had a primary-care home).
- **A2.** April 2025 treatment course: needle decompression on the right, right chest
  tube, admission 2025-04-24 to 2025-04-27, tube removed with the lung re-expanded on
  film. The source gives no treatment; a patient with these findings is decompressed
  and drained.
- **A3.** November 2025 treatment course: the same, admission 2025-11-01 to 2025-11-05.
- **A4.** January 2026 course: chest x-ray shows a right pneumothorax after the right
  subclavian line (postprocedural pneumothorax, in the outside record only); small-bore
  right chest tube; admission 2026-01-23 to 2026-01-26.
- **A5.** 2025-05-06 post-hospital office visit.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Records from a prior practice or student health may mention routine care with normal
  results.
- Labs at joining and at the preventive visit: CBC, CMP, lipids once, HIV and hepatitis C
  screening once, all normal. PHQ-2 or PHQ-9 negative; AUDIT-C low.
- Influenza vaccine in the fall of 2024 (at the practice). Tdap if due.
- Tobacco counseling at each visit; nicotine gum refills, pharmacy traffic, portal
  messages about results and scheduling, a cancelled visit.
- Booked visit after 2026-08-27: a follow-up after the hospital stays.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Records

The chart has gaps: the practice learns of each ED visit first from the patient's
message; the April 2025 records arrive late (after the follow-up visit), the November
2025 discharge summary never arrives, and of the January 2026 visit only the procedure
note and chest x-ray report arrive.

## Last seen

The practice last saw MRN00002679 on 2025-08-29. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. A visit is booked after today.
