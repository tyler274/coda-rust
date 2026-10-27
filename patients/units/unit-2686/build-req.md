# Build requirements: unit-2686 (Glenn Upton, MRN00002686)

Source: synthetic_hospital patient 2686, in `source.md` beside this file. This file and
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
"(diagnosed 2020-03-15)" stamps, or its quiz text.

## Who he is (S)

Glenn Upton, born 1991-01-01, male, White, not Hispanic or Latino, English, lives in
Amherst (01002), commercial PPO through his employer. Construction foreman. Never
smoker. Alcohol socially, 3 to 4 drinks a week. No known drug allergies. Surgical
history: tonsillectomy at age 8. Family history: father with type 2 diabetes, mother
with hypothyroidism, one sibling healthy.

Joined the practice in August 2024 (new-patient visit 2024-08-20, A1). He brought no
chronic problems and no medications.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2024-08-20 | practice | new-patient visit and physical (A1): healthy, history as above, normal exam; screening labs (P) |
| 2025-07-02 | outside | regional hospital ED: about 5 days of muscle aches, fever and swelling around the eyes. Back about 2 weeks from an out-of-state hunting trip (R1) where he ate grilled game meat cooked over a campfire; felt fine at first, then low-grade fevers and vague abdominal discomfort starting about 3 days after getting home, then worsening muscle aches, fatigue and puffy eyes. No cough, sore throat or sick contacts. T 38.3 °C (101 °F), BP 118/72, HR 92. Bilateral periorbital edema; thighs and upper arms tender to palpation. Labs: leukocytosis with eosinophilia and a mildly elevated CK (no numbers reach the practice, R2). **Trichinellosis diagnosed** (presumptive in the ED). **Albendazole 400 mg twice daily for 10 days started** (A2). Discharged home |
| 2025-07-03 | practice | portal message from patient: was in the ED, told it is a parasite from the meat, on pills; asks to be seen (P) |
| 2025-07-08 | practice | post-ED visit (P): on albendazole, fever gone, aches and eye puffiness easing; periorbital edema mild, mild thigh tenderness. Labs: CBC with differential, CK, Trichinella antibody (A3). ED record requested |
| 2025-07-10 | practice | results (A3): Trichinella antibody positive; WBC 10.8, eosinophils 13%, eosinophil count 1.4 ×10³/µL; CK 310 U/L. Expected course, recheck in a month |
| 2025-07-24 | practice | ED record received (R2) |
| 2025-08-12 | practice | follow-up (A3): symptom-free, normal exam; WBC 7.2, eosinophils 4%, eosinophil count 0.3 ×10³/µL, CK 140 U/L. **Trichinellosis resolved.** Counseled to cook game and pork thoroughly |
| 2026-06-22 | practice | patient texts the physician (P): eyes puffy again, achy all over, a rash; asked to come in the next morning |
| 2026-06-23 | practice | same-day office visit, the last practice visit (P): about 3 weeks ago abdominal pain, nausea and vomiting that settled on its own; about a week of steadily worsening muscle soreness, swelling around the eyes and a diffuse rash. No travel; ate undercooked pork at a local fair about 2 weeks before the stomach symptoms began (R3). T 38.3 °C (101 °F). Periorbital edema, muscle tenderness on palpation, subungual splinter hemorrhages, diffuse rash. **Trichinellosis diagnosed again** (new infection, R3). The physician arranges direct admission to the regional hospital's medicine service |
| 2026-06-23 to 2026-06-26 | outside | regional hospital admission, internal medicine (S): T 38.3 °C; periorbital edema, muscle tenderness, subungual splinter hemorrhages, diffuse rash. Labs: WBC 12,000/mm³ (12.0 ×10³/µL), hemoglobin 14 g/dL, eosinophil count 2,000/mm³ (2.0 ×10³/µL, 16% of WBCs), CK 600 U/L, ALT 45 U/L, AST 50 U/L. Blood cultures no growth (A4). Trichinellosis, new infection. **Albendazole 400 mg twice daily for 14 days and prednisone 40 mg daily for 7 days started** (A5). Discharged 2026-06-26 |
| 2026-06-27 | practice | pharmacy fill notice: albendazole and prednisone dispensed 2026-06-26 (R2); lists not updated |
| 2026-07-01 | practice | portal message from patient: home, feeling better, taking the pills; physician asks him to book a recheck after the course; no visit booked (P) |
| 2026-08-11 | practice | discharge summary received after a records request (R2), filed without changing lists |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source names the state of the hunting trip. The chart says only "an
  out-of-state hunting trip" and "game meat"; nothing clinical depends on the state.
- **R2.** Chart texture is "records: gaps". The ED's labs are qualitative in the
  source, so the ED record the practice receives on 2025-07-24 (three weeks late, after a
  request) reports leukocytosis with eosinophilia and a mildly elevated CK with no
  numbers. The practice first hears of the ED visit from the patient. After the June
  2026 admission the practice learns of the drugs from a pharmacy fill notice, and the
  discharge summary arrives only on 2026-08-11. The practice never prescribes albendazole
  or prednisone: both courses are the ED's and the hospital's prescriptions (short
  courses, completed), known to the practice from patient report, the fill notice and
  the outside records.
- **R3.** The source keeps "Trichinellosis" on the active problem list from the first
  episode. Resolution: the 2025 infection resolves on 2025-08-12 (A3), and the 2026
  illness is a new infection from a different exposure (undercooked pork at a fair, no
  travel), diagnosed again on 2026-06-23. The source's "one year after" fits the
  re-dated span (July 2025 to June 2026). The source's encounter is inpatient internal
  medicine; it becomes the practice's same-day visit (the last practice visit, on
  `last_seen`) followed the same day by an outside admission. The exam and temperature
  hold on both records; the admission labs are the hospital's.
- **Left as the source has it:** no echocardiogram, ECG or cardiac enzymes are done for
  the splinter hemorrhages; no repeat CBC or CK after the 2026 admission (he is not
  seen again); no stool or muscle biopsy testing.

## Additions (A): physician to confirm

- **A1.** 2024-08-20 new-patient visit (years with practice: 2; the source starts with
  an ED visit, so the practice relationship begins before it).
- **A2.** The source gives no treatment for the 2025 episode. Albendazole 400 mg twice
  daily for 10 days, started by the ED, is the standard therapy.
- **A3.** Follow-up of the 2025 episode: Trichinella antibody positive on 2025-07-08
  (supports the diagnosis); CBC and CK 2025-07-08 (WBC 10.8, eosinophils 13%, count
  1.4 ×10³/µL, CK 310 U/L) and 2025-08-12 (WBC 7.2, eosinophils 4%, count 0.3
  ×10³/µL, CK 140 U/L); resolution on 2025-08-12.
- **A4.** Blood cultures ×2, no growth, at admission (fever with splinter hemorrhages).
- **A5.** The source gives no treatment for the 2026 episode. Albendazole 400 mg twice
  daily for 14 days with prednisone 40 mg daily for 7 days (more severe illness with
  rash and splinter hemorrhages), started by the hospital.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Records from a prior practice may mention routine screening done there, with normal results.
- New-patient physical 2024-08-20 and a yearly physical in fall 2025 (2025-11-04), with
  vitals (weight about 86 kg, height 180 cm, BP normal), PHQ-2/PHQ-9 normal. Labs at
  joining: CBC, CMP, lipid panel, A1c (father with diabetes), hepatitis C and HIV
  screening once, all normal; CBC and CMP at the 2025 physical, normal.
- Influenza vaccine each fall (2024, 2025) and updated COVID vaccine; Tdap at joining.
- Portal messages about results and scheduling, a work-form or immunization-record
  request, a cancelled visit, records requests.
- No visit is booked after 2026-08-27.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002686 on 2026-06-23. Practice contacts after that day are
messages, pharmacy notices and records only; outside care after it still happens, and
its records are filed as received without changing the practice's lists. No visit is
booked.
