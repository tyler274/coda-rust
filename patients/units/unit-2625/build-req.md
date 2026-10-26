# unit-2625 build requirements: Austin Crowley (MRN00002625)

Every clinical fact in the chart (diagnosis, medication, dose, result, finding,
procedure, referral outcome, family member, life event) traces to **S** (the
source as carried here), **R** (a numbered resolution), **A** (a numbered
addition) or **P** (the primary-care layer below), and nothing else. Texture
(voices, scheduling, refills, portal traffic, admin, a missed visit) is free and
carries no clinical fact. Re-voice everything: no source sentence, clinician name,
problem-list dump, "(diagnosed ...)" stamp or unfinished plan reaches the chart.

## Who he is (S)

- Austin Crowley, male, born 1992-04-05, Easthampton MA 01027. White, not Hispanic
  or Latino, English. Private PPO. Phone +1-646-555-0133,
  austin.crowley@example.test. Single.
- Financial analyst. Never smoker. No alcohol.
- Allergies: no known drug allergies.
- Surgical history: none.
- Family history: maternal grandmother with ulcerative colitis; father with primary
  sclerosing cholangitis; mother with autoimmune hypothyroidism (R3).
- **Joins the practice 2022-09-12** (A1), age 30, bringing:
  - Ulcerative colitis, diagnosed 2015 (age 23) by an outside gastroenterology
    group, which still follows him (R1).
  - Mesalamine 1.2 g (1200 mg delayed-release tablet) by mouth twice daily since
    diagnosis, prescribed by outside GI (historical).
- Primary sclerosing cholangitis is not known at joining; it is diagnosed in
  October 2025 (R4).

## Timeline

| Date | Where | What happens |
|---|---|---|
| 2022-09-12 | Practice | **A1 establish care.** UC on mesalamine 1.2 g BID, followed yearly by outside GI, no recent flare by his report. NKDA, family history as above. Normal exam. Weight 77.0 kg, height 180.3 cm, normal BP. P labs and screening. |
| 2024-10-14 | Outside GI (letter received 2024-10-21) | **S0** routine UC follow-up. Taking mesalamine as prescribed, no flare in the past year; denies recent bloody stools, abdominal pain, weight loss. Vitals stable. Abdomen without tenderness or masses. CBC and BMP normal (drawn at GI that day). Plan (R2): continue mesalamine 1.2 g BID, return in one year. |
| 2025-10-06 | Outside GI (visit, labs; letter received 2025-10-24) | **S1** fatigue and pruritus for about six months. Intermittent episodes of bloody diarrhea since late 2024, which he put down to hemorrhoids and had not reported (R5). Mesalamine 1.2 g BID. Denies travel, alcohol, risky sexual behavior. Exam: well-nourished, jaundiced; abdomen soft, non-distended, nontender, no hepatosplenomegaly, no ascites. Labs 2025-10-06: total bilirubin 2.1 mg/dL, direct bilirubin 1.5 mg/dL, alkaline phosphatase 420 U/L, GGT 210 U/L, AST 50 U/L, ALT 55 U/L. |
| 2025-10-17 | Outside imaging (in the same GI letter, plus its report) | **S1** MRCP: multifocal intrahepatic and extrahepatic biliary strictures with segmental dilations (R4). **Primary sclerosing cholangitis diagnosed** by GI on this study. Mesalamine continues. |
| 2026-04-20 | Outside GI (visit, ultrasound, labs; letter received 2026-04-24) | **S2** follow-up six months after the October evaluation (R6): continued fatigue and pruritus, intermittent jaundice, now pale stools and dark urine. UC controlled on mesalamine, no further bleeding episodes (R5). Afebrile, icteric sclerae, abdomen soft, non-distended, no masses, mild right upper quadrant tenderness. Labs 2026-04-20: total bilirubin 3.2 mg/dL, direct bilirubin 2.5 mg/dL, alkaline phosphatase 420 U/L, AST 80 U/L, ALT 75 U/L; CA 19-9 22 U/mL (A2). Abdominal ultrasound: beading and irregularity of the intrahepatic bile ducts. Plan: ERCP (A3). |
| 2026-05-12 | Outside hospital endoscopy (report received 2026-05-15; cytology received 2026-05-20) | **A3** ERCP: dominant stricture of the common bile duct, balloon dilation, brushings taken; no stent. Brush cytology negative for malignancy. |
| 2026-06-22 | Outside GI (letter received 2026-06-26) | **A4** post-ERCP follow-up: pruritus much improved, stools normal color, no dark urine. Labs 2026-06-22: total bilirubin 1.4 mg/dL, direct bilirubin 0.8 mg/dL, alkaline phosphatase 310 U/L, AST 48 U/L, ALT 52 U/L. Continue mesalamine. Repeat hepatic panel and CA 19-9 in late August; repeat MRCP in about a year. |
| 2026-08-26 | Practice lab draw for GI | **A5** hepatic panel and CA 19-9 drawn; pending at today. |

## Resolutions (R)

1. **UC onset.** The 2024 note says diagnosed nine years earlier (2015) and on
   mesalamine since then; later notes stamp UC "diagnosed" on the first source
   date. The chart carries diagnosis in 2015 (onset 2015-06-01) and drops the stamps.
2. **Unfinished plan at S0.** The 2024 GI assessment stops mid-sentence. The GI
   letter says: UC in remission, continue mesalamine 1.2 g twice daily, return in a
   year. No other plan.
3. **Family history.** The 2024 note gives colorectal cancer in his father at 50;
   the 2025 note gives UC in his mother. The profile and 2026 note give maternal
   grandmother with UC, father with PSC, mother with autoimmune hypothyroidism. The
   chart carries the profile set only; father's colorectal cancer and mother's UC
   are not carried. GI letters list the profile set.
4. **PSC timing and evidence.** The profile lists PSC from the start; the source
   problem list dates it to the second encounter. PSC is diagnosed by GI on the
   MRCP that followed the 2025-10-06 visit; the MRCP is dated 2025-10-17 (its own
   report and the GI letter), so the visit and the diagnosis are eleven days apart.
   Onset 2025-10-17. The practice records it on receiving the letter (2025-10-24).
5. **Bloody stools.** The 2024 visit denies recent bloody stools and no flare in a
   year; the 2025 visit reports intermittent bloody diarrhea "for the past year".
   The chart starts the episodes after the October 2024 visit (late 2024), unreported
   until 2025-10-06 because he attributed them to hemorrhoids. The 2026 note calls UC
   well controlled: by 2026-04-20 the episodes had stopped. No hemorrhoid diagnosis is
   made and no flare treatment is given.
6. **"Six-month history" at S2.** Fatigue and pruritus began about April 2025, so by
   April 2026 they are about a year old. The chart reads the S2 complaint as six
   months of ongoing symptoms since the GI evaluation, with intermittent jaundice
   over that period.
7. **ALT 55 flagged at S1** sits inside the stated range; it is carried as 55,
   unflagged. AST 50 is flagged high.
8. **Labs and imaging at S1 and S2** are drawn or performed by the outside GI group
   (the source shows them at the visit); the practice receives them with the letters.

**Left as the source has it** (the chart keeps these without comment):
- No treatment for pruritus at any point.
- No workup or treatment of the bloody diarrhea episodes beyond noting them.
- No colonoscopy, no liver elastography, no bone density, no ursodiol.
- GGT is measured once (2025-10-06) and not repeated.

## Additions (A)

1. **Establish-care visit 2022-09-12.** The first source encounter is a routine
   follow-up of established UC; the seed gives four years with the practice. He
   brings UC and mesalamine as above.
2. **CA 19-9 22 U/mL on 2026-04-20.** Rising bilirubin with pale stools in PSC: any
   gastroenterologist checks it before ERCP.
3. **ERCP 2026-05-12** at a regional hospital: the usual next step for worsening
   cholestatic jaundice in PSC. Findings exactly: a dominant stricture of the common
   bile duct, balloon dilated, brushings obtained, no stent, no complication.
   Brush cytology negative for malignancy.
4. **GI follow-up 2026-06-22** with the labs given in the timeline (bilirubin falling
   after dilation).
5. **Repeat hepatic panel and CA 19-9 drawn 2026-08-26** at the practice for GI;
   pending today.

## Primary-care layer (P)

- Visits: establish care 2022-09-12; preventive physicals 2023-09 and 2024-09; the
  2025 physical (2025-08) cancelled by the patient and done 2025-12; an office visit
  after the PSC letter (2025-11); a telehealth check-in after the ERCP (2026-05); a
  booked physical 2026-09-21.
- Labs for a man of 30 to 34 on mesalamine: CBC, BMP and lipid panel at physicals
  (renal function for mesalamine). All normal. **No hepatic panel or CMP is drawn at
  the practice before 2025-10-06**, and after it hepatic labs are drawn by or for GI.
  HIV and hepatitis C screening once (2022), nonreactive.
- Vitals at practice visits: weight around 77 kg throughout, normal BP. PHQ-2 at
  physicals, 0 or 1.
- Immunizations: Tdap 2022-09-12; influenza each fall (2022 to 2025); COVID-19 each
  fall (2022 to 2025); hepatitis B (Heplisav-B) two doses in 2023; after the PSC
  diagnosis, hepatitis A two doses (2025-11 and 2026-05) and PCV20 (2025-11) for
  chronic liver disease.
- Prior primary-care records (received 2022-09) with routine normal care.
- Mesalamine stays with outside GI; the practice sees pharmacy refill requests and
  routes them, and one prior-authorization round for the delayed-release tablet.
- Portal messages about results, scheduling, the GI letters, and a records request.
- No acute illnesses, injuries, new diagnoses or life events beyond the timeline.
