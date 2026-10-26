# Build requirements: unit-2622 (Austin Morin, MRN00002622)

Source: synthetic_hospital patient 2622, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz-style framing.

## Who he is (S)

Austin Morin, born 1990-08-10, male, White, not Hispanic or Latino, English, lives in
Greenfield MA 01301, employer-sponsored PPO. Single, lives alone (chosen; the source
gives no household). International logistics consultant whose work takes him abroad.
Never smoker. Alcohol: social, 1 to 2 drinks a week. No known drug allergies. No
surgical history. No blood transfusions ever. Family history: father with alcohol use
disorder; mother with hypertension.

Joined the practice on 2021-09-21 (A1), age 31, from a prior primary care office. He
brought:

- **History of intravenous drug use, in remission** (no current use; no medication for
  it; the substance and the years of use are not documented beyond "remote").
- **History of acute viral hepatitis at age 24, resolved** (about 2014 to 2015): he was
  told he had "hepatitis" after travel abroad, received no antiviral treatment and
  recovered fully. No records from that illness exist in the chart.
- No medications.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2021-09-21 | practice | **new-patient visit** (A1); history as above; normal exam; establishes care. Prevention per P (one-time HIV and hepatitis C screens nonreactive) |
| 2021-10-27 | outside | prior primary care office records arrive (partial, P): routine care, nothing beyond the history above |
| 2024-12-30 | practice | routine health evaluation (annual physical); no current symptoms; no IV drug use now, no transfusions ever; history and family history as above; normal exam. Hepatitis B serologies: **HBsAg negative, anti-HBs positive, anti-HBc positive, HBeAg negative, anti-HBe positive** (R2) |
| 2026-06-19 | outside | ED visit, age 35: a week of malaise, fatigue and dark urine; mild nausea, no vomiting, no abdominal pain; back recently from a work trip to Southeast Asia (exposure, R4); still no medications. Mild scleral icterus; right upper quadrant tenderness without hepatomegaly. **AST 980 U/L, ALT 1150 U/L, total bilirubin 3.2 mg/dL, alkaline phosphatase 110 U/L, albumin 4.2 g/dL, INR 1.0.** Right upper quadrant ultrasound normal (A3). Acute hepatitis, likely viral; acute hepatitis panel sent; discharged home to follow up with primary care (R3). The ED record reaches the practice late (records, below) |
| 2026-06-23 | practice | post-ED visit (A2) |
| 2026-06-24 | practice | hepatitis A IgM reactive on the practice's draw: **acute hepatitis A diagnosed** (A4) |
| 2026-07-08 to 2026-08-17 | practice | recovery and serial liver tests (A5); **hepatitis A resolved** at the 2026-08-17 result |

## Records (chart texture: gaps)

- The ED sends nothing at discharge. The practice learns of the visit from the patient's
  portal message the next day (2026-06-20). The PCP requests the ED record on
  2026-06-23 and again on 2026-07-06; it arrives by fax on 2026-07-10 with the ED's
  labs, ultrasound report and physician note. Until then the practice's notes know the
  ED visit only as the patient tells it ("they said my liver numbers were way up").
- The ED's own acute hepatitis panel, pending at discharge, never reaches the practice.
  The practice repeats the panel on 2026-06-23.
- The prior primary care office sends only its last two years of notes, about five weeks
  after the request. No records of the age-24 hepatitis exist.

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source's ED problem list stamps "History of intravenous drug use
  (diagnosed 2020-01-15)" and lists it twice. Resolution: one history entry, carried
  from the 2021 new-patient visit; no stamp; no "diagnosed" date.
- **R2.** The source calls the age-24 illness "acute viral hepatitis" without a type,
  while the 2024 panel shows past hepatitis B infection that has cleared, with
  immunity. Resolution: the 2024 result note reads the panel as resolved past
  hepatitis B infection, immune, no chronic infection, no vaccine needed, and says it
  probably explains the hepatitis he had at 24. The problem list keeps its one entry,
  "history of resolved acute viral hepatitis (age 24)", unchanged (the list lags);
  the result note and later visit notes carry the hepatitis B reading. No hepatitis B DNA, no specialist referral, no new
  diagnosis. That earlier hepatitis cannot have been hepatitis A, since he gets
  hepatitis A in 2026.
- **R3.** The source's ED pathology describes a liver biopsy (apoptotic hepatocytes with
  mononuclear infiltrates). An ED does not biopsy the liver, and acute viral hepatitis
  with a normal INR is not biopsied. Resolution: no biopsy in the chart. The
  hepatocellular injury it describes is carried by the ED's enzymes and, at the
  practice, the hepatitis A IgM (A4).
- **R4.** The source lists "occupational exposure to hepatitis A virus" as a
  comorbidity. Resolution: the exposure is his work travel to Southeast Asia in the
  weeks before illness (he returned in early June 2026). It appears in the history of
  the ED and practice notes, not as a separate problem-list entry.
- **R5.** The source labels the ED's primary diagnosis only "acute viral hepatitis".
  Resolution: the ED record says acute hepatitis, likely viral; the practice names it
  hepatitis A once the IgM returns (the source's hepatitis A exposure).
- **Left as the source has it:** no hepatitis A vaccine is ever offered before 2026,
  despite his injection drug use history and international work travel, and he has no
  pre-travel visit. No naloxone is offered. No note comments on either.

## Additions (A): physician to confirm

- **A1.** 2021-09-21 new-patient visit (the source's first encounter is a routine
  evaluation, and the seed has him with the practice about five years). Height 180 cm,
  weight 79 kg at joining; weight steady within about 2 kg through 2025.
- **A2.** 2026-06-23 post-ED office visit: still tired, appetite poor, mild nausea, urine
  still dark; mild scleral icterus; mild right upper quadrant tenderness, no
  hepatomegaly; alert, no asterixis, no bruising. Weight 76 kg (down about 3 kg). CMP,
  INR, CBC and an acute hepatitis panel drawn: **AST 412 U/L, ALT 688 U/L, total
  bilirubin 2.7 mg/dL, alkaline phosphatase 121 U/L, albumin 4.0 g/dL, INR 1.0**; CBC and
  the rest of the CMP normal. Advice: rest, fluids, no alcohol, acetaminophen no more
  than 2 g a day, no travel; out of work through 2026-07-06 (work note).
- **A3.** ED right upper quadrant ultrasound, 2026-06-19: normal liver size and
  echotexture, no focal lesion, no biliary dilatation, normal gallbladder without stones
  or wall thickening, patent portal vein with normal flow, no ascites. Any ED would image
  jaundice with right upper quadrant tenderness.
- **A4.** Acute hepatitis panel, collected 2026-06-23, resulted 2026-06-24: **hepatitis A
  IgM reactive**, hepatitis B surface antigen nonreactive, hepatitis B core IgM
  nonreactive, hepatitis C antibody nonreactive. Acute hepatitis A diagnosed (onset
  about 2026-06-12). A public health nurse from the local health department calls the practice to confirm the
  case (hepatitis A is reportable); he is not a food handler.
- **A5.** Recovery, the usual course of acute hepatitis A:
  - 2026-07-08 office visit: energy better, appetite back, urine normal, no icterus,
    abdomen nontender; weight 77 kg. Hepatic panel: **AST 98 U/L, ALT 214 U/L, total
    bilirubin 1.3 mg/dL, direct bilirubin 0.6 mg/dL, alkaline phosphatase 104 U/L,
    albumin 4.2 g/dL.** Back to work remotely; no international travel until liver tests
    are normal.
  - 2026-07-29 lab only: **AST 36 U/L, ALT 61 U/L, total bilirubin 0.8 mg/dL, direct
    bilirubin 0.2 mg/dL, alkaline phosphatase 86 U/L, albumin 4.3 g/dL.**
  - 2026-08-17 lab only: **AST 23 U/L, ALT 27 U/L, total bilirubin 0.6 mg/dL, direct
    bilirubin 0.1 mg/dL, alkaline phosphatase 74 U/L, albumin 4.4 g/dL.** Normal;
    hepatitis A resolved; cleared for travel and usual alcohol; weight back to about
    78 kg by report. Lifelong immunity to hepatitis A noted, so no vaccine needed.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may
  carry the descriptors its kind always carries, but nothing that adds a territory, a
  severity grade or a symptom, or changes the diagnosis or plan.
- Records from a prior practice may mention routine care done there, with normal results.
- Physicals at the practice: 2021-09-21 (new patient), 2022-10-20, 2023-11-09 (missed,
  no-show; not rebooked until a reminder in late 2024), 2024-12-30 (source), 2025-12-11.
  Vitals each time; PHQ-2 negative; AUDIT-C low; never smoker. Blood pressure normal
  throughout.
- Labs at physicals: CMP, CBC and lipids at joining; CMP and lipids in 2022 and 2025;
  CMP, CBC and lipids with the 2024 hepatitis B panel. All normal.
- One-time HIV and hepatitis C screens at joining, nonreactive.
- Immunizations: COVID mRNA primary series in April 2021 elsewhere (recorded as
  historical from the state registry), Tdap at joining, influenza each fall 2021 through
  2025, COVID bivalent fall 2022 and updated fall 2023 (declines later COVID boosters).
  No hepatitis A vaccine (gap above); no hepatitis B vaccine (immune).
- Portal traffic: scheduling, result notes, a request for his immunization record for
  work, reminders, the missed physical, the post-hepatitis questions about travel and
  alcohol. A work note.
- Booked visit after 2026-08-27: annual physical on 2026-12-08.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.
