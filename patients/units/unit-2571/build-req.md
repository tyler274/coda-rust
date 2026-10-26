# Build requirements: unit-2571 (August Rosen, MRN00002571)

Source: synthetic_hospital patient 2571, in `source.md` beside this file. This file and
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
"(diagnosed 2020-07-15)" stamps, or its quiz text ("is shown below").

## Who he is (S)

August Rosen, born 2001-10-11, male, White, not Hispanic or Latino, English, lives in
Deerfield (01342), single, employer-sponsored private insurance. Construction laborer.
Tobacco: cigarettes, half a pack a day; quits about 18 months before the February 2026
crash, so in early August 2024 (R4). Alcohol: 2 to 3 beers on weekends. Allergies: no
known drug allergies. Surgical history: tonsillectomy and adenoidectomy at age 8;
open reduction and internal fixation of a right distal radius fracture at age 16.
Family history: father with hypertension and hyperlipidemia; mother with major
depressive disorder; an older brother with ADHD.

Joined the practice 2023-09-12 (A1), age 21, bringing:
- ADHD, combined type, on methylphenidate extended-release 20 mg by mouth every morning;
- seasonal allergic rhinitis, on cetirizine 10 mg daily as needed;
- current cigarette smoking (nicotine dependence).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2023-09-12 | practice | new-patient visit (A1): history as above; methylphenidate ER 20 mg each morning and cetirizine 10 mg as needed continued; smoking counseled; prior-practice records requested |
| 2023-12 to 2025-11 | practice | ADHD follow-up about every 3 to 4 months, two physicals, refills (P) |
| early 2024-08 | patient report | **stops smoking** on his own (R4); nicotine dependence resolved, history of tobacco dependence recorded at the next visit |
| 2025-11-10 | practice | last practice visit (seed `last_seen`): routine ADHD follow-up; no change |
| 2026-02-07 | outside: regional trauma center ED | brought by ambulance after a high-speed crash; unrestrained driver, car struck a tree head-on. Intubated on arrival for airway protection; GCS 4 (E1 V1 M2). BP 122/78, HR 86, SpO2 98% on the ventilator. No external head trauma or skull fracture; pupils equal and reactive; no lateralizing deficit. Initial labs unremarkable; toxicology negative. Non-contrast CT head: no intracranial hemorrhage, no mass, no skull fracture (R2). MRI brain pending. EEG: preserved background, no seizure activity. Neurosurgery consulted; **admitted to the neurosurgical ICU**. Home medications listed: methylphenidate ER 20 mg each morning, cetirizine 10 mg as needed; NKDA |
| 2026-02-07 | outside: neurosurgical ICU (R1) | neurosurgery ICU admission note: intubated and sedated; BP 110/70, HR 92, RR 14 on the ventilator, SpO2 98% on FiO2 50%; GCS 6; pupils equal and reactive; no external head trauma. Na 138, K 4.1, Cl 100, HCO3 24, BUN 18, Cr 0.9, glucose 110. Repeat CT head: diffuse cerebral edema, no focal lesion. Intracranial pressure monitor placed, ICP elevated (A2). **Traumatic cerebral edema with elevated intracranial pressure** diagnosed; **coma** and **dependence on mechanical ventilation** recorded. **Propofol** for sedation and **fentanyl** for analgesia. Home methylphenidate and cetirizine held (R3) |
| 2026-02-07 to 02-09 | outside: ICU | remains comatose through the first 48 hours (R2) |
| 2026-02-09 | outside: ICU (R1) | neurosurgery note: new episodes of HR up to 130, BP spikes to 190/110, RR up to 30, temperature up to 38.9 C, profuse diaphoresis; during an episode pupils dilated, tone increased, extensor posturing; GCS 7; still intubated and sedated on propofol and fentanyl. MRI brain: diffuse axonal injury, no hemorrhage (R2). **Diffuse axonal injury** and **paroxysmal sympathetic hyperactivity** diagnosed. **Propranolol 20 mg every 6 hours and gabapentin 300 mg three times daily, both per tube, started** (A3). Same-day BMP: Na 147, Cl 113, HCO3 24 on hypertonic saline for ICP (A2); otherwise normal |
| 2026-02-19 | outside: ICU | tracheostomy and percutaneous gastrostomy tube placement for prolonged ventilation and coma (A4) |
| 2026-03-02 | outside: ICU | weaned from the ventilator to trach collar; propofol and fentanyl stopped by then (A4) |
| 2026-03-06 | outside: ICU | **transferred to an inpatient brain injury rehabilitation unit** (A5): eyes open spontaneously, not yet consistently following commands; on propranolol 20 mg every 6 hours and gabapentin 300 mg three times daily per tube; methylphenidate and cetirizine still held; labs that week normal (CBC, BMP) |
| 2026-08-12 | outside: rehabilitation | discharged home to his parents with outpatient therapy arranged, per the mother's call (A5); the rehabilitation unit sends no summary |

## Records (chart texture: gaps)

- 2026-02-09: the practice first hears of the crash from his mother by phone.
- The trauma center ED record (CT, labs, EEG, vitals) arrives 2026-03-18, after a
  records request.
- The neurosurgery ICU notes (2026-02-07 and 2026-02-09) and the ICU transfer summary
  arrive late, 2026-05-27, after a second request.
- The rehabilitation unit never sends a letter; by today the practice knows of the
  rehab stay and the August discharge only from the mother. A records request to the
  rehabilitation unit is open.
- After 2025-11-10 nothing is reconciled: the practice problem and medication lists
  still show ADHD and allergic rhinitis with methylphenidate and cetirizine active;
  the injury diagnoses, propranolol and gabapentin appear only in the outside records.

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Encounters 2 and 3 carry the practice date 2026-02-15, but encounter 2 says
  he came from the ED "earlier today" (ED 2026-02-07) and encounter 3 is "two days
  after admission". Resolution: the ICU admission note is 2026-02-07 and the
  sympathetic-storm note is 2026-02-09.
- **R2.** The ED note says intracranial pressure was normal and he was off sedation
  through the first 48 hours, while neurosurgery records elevated ICP and keeps him
  sedated. Resolution: neurosurgery's account stands (elevated ICP, sedated); the ED's
  "normal ICP" and "absence of sedation" are not carried; only "remains comatose over
  the first 48 hours" is kept. The ED CT "shown below" becomes a CT report without
  hemorrhage, mass or fracture. Encounter 3 attributes diffuse axonal injury to CT;
  the chart carries it as the MRI that was pending in the ED, without hemorrhage.
- **R3.** Encounter 3 calls him "on methylphenidate extended-release and cetirizine"
  while intubated in the ICU. Resolution: both were held at ICU admission; the ICU
  notes list them as home medications only.
- **R4.** "Quit 18 months ago" at a February 2026 crash puts the quit date in early
  August 2024. He smokes half a pack a day at joining and through mid-2024.
- **Left as the source has it:** the fever to 38.9 C is attributed to sympathetic
  storming with no infection workup documented. No note comments on it.

## Additions (A): physician to confirm

- **A1.** 2023-09-12 new-patient visit (three years with the practice by today).
- **A2.** An intracranial pressure monitor (elevated ICP needs a measurement) and
  hypertonic saline for ICP, with sodium 147, chloride 113 and bicarbonate 24 on 2026-02-09. No ICP numbers.
- **A3.** Propranolol 20 mg every 6 hours and gabapentin 300 mg three times daily per
  tube for paroxysmal sympathetic hyperactivity, which the source diagnoses without
  treatment.
- **A4.** Tracheostomy and percutaneous gastrostomy on 2026-02-19; ventilator weaned
  to trach collar by 2026-03-02; propofol and fentanyl stopped. Usual course for a
  comatose, ventilator-dependent patient after diffuse axonal injury.
- **A5.** Transfer to inpatient brain injury rehabilitation on 2026-03-06 (status as
  above) and discharge home to his parents on 2026-08-12, known only from the mother.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not
  mention. An abnormal finding appears only as the source or this file gives it.
- Records from the prior practice: routine childhood vaccines and HPV series complete,
  Tdap in early adolescence, ADHD long established on methylphenidate.
- ADHD visits every 3 to 4 months with blood pressure and pulse, PDMP checks, a
  controlled-substance agreement, monthly Schedule II e-prescriptions, a yearly prior
  authorization; cetirizine refills in season.
- Physicals in 2024 and 2025: PHQ-2/PHQ-9 and AUDIT-C, smoking counseling until he
  quits; 2024 labs CBC, CMP, lipid panel, HIV and hepatitis C screening, all normal;
  Tdap in 2024; influenza vaccine each fall 2023 to 2025; an updated COVID vaccine in
  fall 2024.
- Portal messages about refills, results and scheduling; a missed visit in January 2025.
- After 2025-11-10: the mother's calls, a declined methylphenidate auto-refill while he
  is hospitalized, records requests and records filed as received, a cancelled
  February 2026 follow-up.
- Booked visit after 2026-08-27: a post-hospital primary-care visit on 2026-09-08.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002571 on 2025-11-10. Practice contacts after that day do
not happen; outside care after it still does, and its records are filed as received
without changing the practice's lists. A visit is booked after today.
