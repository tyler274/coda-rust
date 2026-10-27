# Build requirements: unit-2065 (Glenn Ryan, MRN00002065)

Source: synthetic_hospital patient 2065, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quoted patient speech word for word.

## Who he is (S)

Glenn Ryan, born 1973-06-18, male, White, not Hispanic or Latino, English-speaking,
Commercial PPO, lives in Amherst (01002). Married; his wife is the one who brings him
in and reports his drinking. Construction supervisor. Former smoker, 20 pack-years,
quit 2018. Heavy daily alcohol use, about a pint of vodka a day. No known drug
allergies. Surgical history: appendectomy at age 11. Family history: father with
alcohol use disorder and hypertension; mother with osteoarthritis; brother with
chronic low back pain.

**Joined the practice 2023-09-12** (A1), a new-patient visit, three years before
today. He brought:

- Problems: chronic low back pain; alcohol use disorder (severe pattern by history,
  ICD-10 F10.20); alcoholic liver disease with suspected cirrhosis (K70.9; "suspected",
  never confirmed in this chart).
- Drugs: oxycodone 5 mg tablets, 5 to 10 mg by mouth every 6 hours as needed for
  pain, prescribed by an outside pain management clinician (R1; historical on the
  practice list); omeprazole 20 mg by mouth daily for gastritis prophylaxis (the
  practice takes over refills; R8).
- Former smoker (history of tobacco dependence).

Height 180 cm, weight about 84 kg at joining, steady after (P). Blood pressure is his
own normal at practice visits (no level set).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2023-09-12 | practice | **new-patient visit** (A1): history as above; AUDIT-C 12; counseled on alcohol, declines treatment referral for now; baseline labs (A2); RUQ ultrasound ordered (A3) |
| 2023-09-26 | outside imaging | RUQ ultrasound (A3), report received |
| 2026-01-01 | outside ED | 2 days of nausea, vomiting and right upper quadrant pain; took "a few extra" over-the-counter pain relievers for a headache over the past week. Lethargic, oriented to person and place; icteric skin and sclerae; RUQ tender without rebound or guarding. AST 2500 U/L, ALT 2300 U/L, total bilirubin 5.2 mg/dL, INR 2.1, serum acetaminophen 30 mcg/mL. **Acetaminophen toxicity.** N-acetylcysteine started and admitted (A4). The practice receives a short ED note (2026-01-06) |
| 2026-01-13 | practice | post-hospital visit: he says he was in about four days and finished the IV antidote (A4); **acetaminophen toxicity recorded** (onset 2026-01-01); told to take no acetaminophen-containing products; repeat hepatic panel and INR (A5) |
| 2026-03-08 | outside ED / ICU | wife brings him in for a few hours of increasing drowsiness, lethargy and shallow breathing, two months after the acetaminophen toxicity. He takes prescribed oxycodone; drinks regularly, denies other drugs. RR 8, BP 110/70, pulse 55, SpO2 89% on room air; somnolent but arousable. ABG pH 7.28, PaCO2 65 mmHg, PaO2 58 mmHg, HCO3 24 mEq/L. **Opioid overdose** with hypoventilation; naloxone in the ED, ICU observation, discharged 2026-03-10 with a **naloxone nasal spray** (A6). The practice hears from his wife by portal on 2026-03-09; a brief ICU discharge summary arrives on 2026-03-24 |
| 2026-03-17 | practice | post-discharge visit: **opioid overdose recorded** (onset 2026-03-08); naloxone spray noted (A6); alcohol counseling again |
| 2026-07-12 (Sun) | practice | **last seen**: same-day video visit after his wife texts the physician that he has vomited blood; sent to the ED by ambulance (A7) |
| 2026-07-12 to 2026-07-21 | outside hospital | admitted for hematemesis (Internal Medicine, ICU). Wife: a pint of vodka daily, last drink on the day of admission. Hospital day 3 (2026-07-14): increasingly agitated and confused, trying to climb out of bed, seeing spiders on the walls. T 38.6 °C (101.5 °F), BP 168/96, pulse 122, RR 22; diaphoresis, fine tremor, generalized disorientation. Labs without electrolyte derangements; CT head normal. **Delirium tremens**, treated with benzodiazepines and thiamine (A8). Discharged home 2026-07-21. Discharge summary received 2026-07-24 and filed; practice lists not updated |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Oxycodone is on the encounter 0 and 2 home lists but missing from the
  encounter 1 list, whose prose says he takes prescribed oxycodone. Resolution: he
  takes it throughout, prescribed monthly by an outside pain management clinician from
  before he joined; the practice carries it as historical and sees it in fill history.
  The dose never changes.
- **R2.** The source's problem lists carry acetaminophen toxicity and opioid overdose
  with stamps, and its profile lists all three primary diagnoses at once. Resolution:
  each starts at its own encounter. Acetaminophen toxicity is recorded on 2026-01-13
  and resolved on 2026-05-12 when the liver tests are back at his baseline; opioid
  overdose is recorded on 2026-03-17 and resolved on 2026-05-12. Delirium tremens
  happens after the last visit and appears only in the hospital's discharge summary.
- **R3.** "Over-the-counter pain relievers" in encounter 0 are unnamed. Resolution:
  the ED note names acetaminophen from the level; no product name.
- **R4.** The hematemesis in encounter 2 has no stated cause or endoscopy. Resolution:
  the discharge summary the practice receives says the bleeding stopped during the
  stay and names no source and no endoscopic finding; the chart adds none.
- **R5.** Encounter 2's date (2026-07-12) is a Sunday and is the seed's last-seen day.
  Resolution: the practice's held contact that day is the same-day video visit in A7,
  before the admission. No date is moved.
- **R6.** Encounter 2's prose ties the last drink to "3 days ago" and to the day of
  admission. Resolution: last drink the day of admission (2026-07-12); delirium on
  hospital day 3 (2026-07-14).
- **R7.** Only the ED and ICU vitals and labs the source gives are carried for those
  stays; the ED note and the two summaries are short and carry nothing else numeric.
- **R8.** Omeprazole's indication "gastritis prophylaxis" is carried as written; no
  gastritis diagnosis is added.
- **Left as the source has it:** oxycodone continues unchanged after the overdose and
  is on the July hospital list; no one changes it and no note questions it. No drug or
  specialty treatment for the alcohol use disorder is ever started (he is counseled and
  declines). No hepatology referral, elastography or liver cancer surveillance for the
  suspected cirrhosis. No cause is found for the hematemesis. The physician who
  inherits the chart can find these gaps.

## Additions (A): physician to confirm

- **A1.** 2023-09-12 new-patient visit (the source's first encounter already lists his
  chronic problems and drugs, so he was a patient before it).
- **A2.** Liver tests consistent with alcoholic liver disease, at joining and as his
  baseline on practice draws (2023 to 2025, and again by 2026-05-12): AST about 68 U/L,
  ALT about 34 U/L, GGT about 212 U/L, total bilirubin about 1.4 mg/dL, albumin about
  3.7 g/dL, INR about 1.2, platelets about 142 K/uL, MCV about 102 fL. Other CMP and CBC
  members normal.
- **A3.** RUQ ultrasound 2023-09-26 to support the suspected cirrhosis: liver
  coarsened and echogenic with a subtly nodular surface, suggestive of cirrhosis; no
  focal liver lesion; no ascites; portal vein patent with forward flow; spleen normal in
  size; gallbladder and bile ducts normal.
- **A4.** N-acetylcysteine and admission after the January ED visit; about four days
  in hospital, by his account (no discharge summary reaches the practice).
- **A5.** 2026-01-13 repeat: AST 74 U/L, ALT 168 U/L, total bilirubin 2.1 mg/dL, INR 1.3.
- **A6.** Naloxone in the March ED and a naloxone 4 mg nasal spray at discharge.
- **A7.** 2026-07-12 same-day video visit that sends him to the ED.
- **A8.** Benzodiazepines (lorazepam) and thiamine for the delirium tremens, IV acid
  suppression for the hematemesis, in the hospital only; none of these is on the
  practice list.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may
  carry the descriptors its kind always carries, but nothing that adds a territory, a
  severity grade or a symptom, or changes the diagnosis or plan.
- Chronic-care visits every 3 to 6 months (back pain, alcohol, liver), with vitals, pain
  scores, alcohol counseling, a CMP, CBC, PT/INR and GGT about twice a year at the A2
  levels; AUDIT-C 12 each time it is asked; PHQ-2 or PHQ-9 negative.
- Yearly physicals (fall 2024 and fall 2025) with a lipid panel and A1c, normal.
- Prevention: yearly low-dose CT lung screening (fall 2023, 2024, 2025), Lung-RADS 1;
  yearly FIT, negative; hepatitis C antibody, HIV, hepatitis B surface antigen and
  antibody once at joining, all negative (not immune to B); hepatitis B series and
  hepatitis A series for liver disease; PCV20 once; influenza each fall; COVID each
  fall; Shingrix series; Tdap if due.
- Refills of omeprazole; oxycodone fill-history review; portal messages from him and his
  wife about results, refills and scheduling; a missed visit; reminders.
- After 2026-07-12: messages, refill traffic and the discharge summary only. A post-
  hospital visit is offered by message and voicemail and is not booked by today.
- Nothing is booked after 2026-08-27.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002065 on 2026-07-12. Practice contacts after that day do
not happen; outside care after it still does, and its records are filed as received
without changing the practice's lists. Nothing is booked after today.
