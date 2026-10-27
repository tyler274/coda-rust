# Build requirements: unit-1855 (Alyssa Rezendes, MRN00001855)

Source: synthetic_hospital patient 1855, in `source.md` beside this file. This file and
the source are the only authority for clinical facts. Every diagnosis, medication,
result, procedure and specialist finding in the chart must trace to one of:

- **S** a fact in source.md, carried as listed below;
- **R** a resolution below, where the source contradicts itself or stops short;
- **A** an addition below, listed one by one;
- **P** the primary-care layer described at the end.

Nothing else. No diagnosis, medication, test result, procedure, family member or
life event that is not on these lists. Texture (wording, scheduling, portal
traffic, who signed what) is free, as long as it carries no new clinical fact.

Re-voice everything. The source notes are exam vignettes: never copy a sentence, and
never carry a source clinician's name, its "Active Problem List" dump, its
"(diagnosed ...)" stamp, or its quiz-style mechanism text.

## Who she is (S)

Alyssa Rezendes, born 1995-08-10, female, White, not Hispanic or Latino, English,
private insurance, lives in Belchertown (01007), single. Graduate student. Never
smoker; social alcohol, 1 to 2 drinks a week. Strict vegetarian diet. No known drug
allergies. No surgical history. Nulliparous. Uses condoms for contraception.
Family history: mother with endometriosis; maternal aunt with infertility.

Joined the practice in fall 2023 (A1). On joining she has no active problems and
takes **naproxen sodium 220 mg by mouth every 8 hours as needed for menstrual
cramps** (over the counter, historical). Height 165 cm, weight about 57 kg (BMI
about 21, as the 2026 ED record gives).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2023-09-18 | practice | phone call asking to join (P) |
| 2023-10-03 | practice | new-patient preventive visit (A1): history as above; ordinary periods with cramps eased by naproxen; normal exam; screening labs (P); flu and COVID vaccines (P) |
| 2023-10-11 | outside | records from her former primary care office arrive (P): routine care, a normal Pap in 2022 |
| 2024-05-21 | practice | about 4 months of worsening period pain and deep pain with intercourse; cycles regular; now also taking **ibuprofen 400 mg every 6 hours as needed for pelvic pain** (over the counter, historical) with naproxen, little relief; condoms; urine pregnancy test negative (P); abdomen soft and nontender; referred to gynecology (A2) |
| 2024-07-12 | outside | gynecology consult (letter received 2024-07-19): 6 months of progressive dysmenorrhea and deep dyspareunia now limiting daily activities; regular cycles; no children; ibuprofen and naproxen with minimal relief; condoms; family history as above; exam with posterior cul-de-sac tenderness and a retroverted uterus; hemoglobin 13.2 g/dL, WBC 7.5 x10^3/uL, platelets 250 x10^3/uL, CA-125 35 U/mL (reference 0 to 35); transvaginal ultrasound without abnormality. **Endometriosis diagnosed clinically** (R1). Plan in the letter (R1) |
| 2024-10-22 | practice | annual physical (P): pain about the same, NSAIDs on the worst days; PHQ-9 low; flu and COVID vaccines; CBC normal |
| 2024-12-25 | practice | **last seen.** Video visit she asked for over winter break (P): a painful period this cycle, NSAIDs took the edge off; no fever, no black stools; NSAID counseling (with food, within label doses); reminded the gynecology hormonal option stands. No medication change |
| 2026-07-16 | outside | emergency department visit (record received 2026-07-24): sent from a university research study of vascular regulation in men and women after a low blood pressure episode during a study session. No dizziness, palpitations or fatigue. BMI 21; heart rate 72; repeated pressures at the low end of normal (R2). Electrolytes, hemoglobin and renal function normal. The study team's evaluation, as relayed in the record: normal renin, aldosterone and catecholamines; preserved baroreceptor reflexes; normal sympathetic nerve activity; enhanced endothelium-dependent vasodilation (R3). Home meds ibuprofen and naproxen as above; NKDA; endometriosis on the ED's list. ED impression: **hypotension**, transient, resolved; discharged home (R2) |
| 2026-07-27 | practice | portal message from her about the ED visit; the physician's reply: record read, nothing to change, come in for an overdue physical when she can (P). Not booked |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The gynecology plan stops mid-sentence after naming NSAID failure, and the
  2026 ED list still shows only ibuprofen and naproxen. Resolution: the gynecologist
  makes a clinical diagnosis of endometriosis, offers hormonal suppression with a
  continuous combined oral contraceptive and discusses diagnostic laparoscopy if
  symptoms persist; she prefers to stay on NSAIDs for now and will call back. No
  hormonal drug is started, and none appears anywhere in the chart. The CA-125 of 35
  sits at the upper limit and is carried as written.
- **R2.** The source calls it an "acute hypotension episode" yet gives the mean
  arterial pressure only as the low end of normal and no blood pressure. Resolution:
  the study staff's low reading (not given a number) prompts the transfer; ED triage
  BP 88/56, then 96/60, then 102/66 at discharge, heart rate 72 at each, SpO2 99%,
  temperature 36.8 C, respirations 14. The ED records hypotension, resolved. Because
  this comes after `last_seen`, hypotension is filed in the ED record only and never
  reaches the practice's problem list.
- **R3.** Renin, aldosterone, catecholamines and the vascular testing (baroreflex,
  sympathetic nerve activity, acetylcholine-induced vasodilation) are research
  measurements an ED does not run. Resolution: the ED note reports them as the study
  team's results, without values. The source's last sentence (the response blunted
  by a guanylyl cyclase pathway inhibitor) is quiz text and is not carried.
- **R4.** The ED note says her diet is "typical for her age", while the profile
  lists a strict vegetarian diet. Resolution: the practice records her as strict
  vegetarian; the ED note says only that activity is ordinary and says nothing about
  diet.
- **R5.** Encounter 0 lists only naproxen as a home medication but says she used both
  ibuprofen and naproxen. Resolution: naproxen from before joining; ibuprofen added by
  her for pelvic pain once the pain worsened in early 2024, first recorded on
  2024-05-21. Both are over-the-counter and historical on the practice list.
- **Left as the source has it:** no hormonal therapy and no laparoscopy through
  today; no further evaluation of the low blood pressure. No one comments.

## Additions (A): physician to confirm

- **A1.** The 2023 new-patient visit (the source starts at a specialist; the seed
  gives about 3 years with the practice).
- **A2.** The 2024-05-21 practice visit that refers her to gynecology.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention
  (ovaries, endometrium, adnexa, free fluid). An abnormal finding appears only as the
  source or this file gives it, with nothing that adds a territory, a severity grade or
  a symptom, or changes the diagnosis or plan.
- Records from her prior practice may mention routine screening done there, with normal
  results (Pap 2022 normal, so next due 2025, which passes undone since she is not seen).
- Establish-care labs: CBC, CMP, lipid panel, HIV and hepatitis C screening once, all
  normal. PHQ-2 or PHQ-9 at preventive visits, low.
- Influenza and updated COVID vaccines in fall 2023 and fall 2024.
- Portal messages about results, scheduling, reminders; a recall message in 2025 that
  she answers without booking.
- No booked visit after 2026-08-27.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00001855 on 2024-12-25 (a video visit). Practice contacts after
that day are only messages; outside care after it still happens, and its records are filed
as received without changing the practice's lists. No visit is booked after today.
