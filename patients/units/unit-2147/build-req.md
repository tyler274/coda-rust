# Build requirements: unit-2147 (Henry Weiss, MRN00002147)

Source: synthetic_hospital patient 2147, in `source.md` beside this file. This file and
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
"(diagnosed 2020-10-15)" stamps, or its quiz framing.

## Who he is (S)

Henry Weiss, born 1984-12-06, male, White, not Hispanic or Latino, English, private
employer-sponsored PPO. Construction site supervisor. Tobacco: cigarettes, 8 pack-years
in all; still smoking when he joined, quit in June 2024 (R3), so a former smoker who quit
about 2 years before the June 2026 crash. Alcohol: social, 2 to 3 drinks a week. No known
drug allergies. Surgical history: tonsillectomy at age 7; open reduction and internal
fixation of the left ankle at age 32 (about 2017). Family history: father myocardial
infarction at 58; mother type 2 diabetes; a brother with hypertension.

Joined the practice in September 2023 (A1), aged 38, from a prior primary-care practice.
He brought: type 2 diabetes (diagnosed about 2019), essential hypertension (about 2018),
obesity (BMI 34; height 180 cm, weight about 110 kg), obstructive sleep apnea (diagnosed
by a home sleep test at the prior practice about 2021, on nightly CPAP, A2), and current
cigarette smoking. Medications on joining: metformin 1000 mg twice daily, lisinopril
20 mg daily, atorvastatin 40 mg nightly, aspirin 81 mg daily. All four continue unchanged
through today.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2023-09-12 | practice | new-patient visit (A1): history as above, BP on lisinopril near goal, baseline labs, Tdap, records request to prior practice (P) |
| 2023-09 to 2026-05 | practice | routine primary care (P): diabetes and BP visits, yearly physicals, vaccines, eye letters; quits smoking June 2024 (R3) |
| 2026-06-20 (Sat) | outside: ED of a regional trauma hospital | brought in after a high-speed motor vehicle collision, **multiple traumatic injuries**; hypotensive and tachycardic on arrival (numbers A3); difficult peripheral access, so a right internal jugular central venous catheter is placed urgently with him sitting upright; shortly after, acute dyspnea, vague chest tightness, dizziness, further fall in BP, gasping. Exam: distended neck veins, mild confusion, new systolic murmur loudest at the left sternal border, lungs clear. Chest x-ray unremarkable apart from a properly positioned catheter. Bedside cardiac ultrasound inconclusive for poor acoustic windows. ECG sinus tachycardia with nonspecific ST-T changes. **Venous air embolism** diagnosed. Treatment and course A4. Admitted to orthopedic surgery |
| 2026-06-20 to 2026-07-06 | outside: same hospital, orthopedic surgery | operative fixation (ORIF) of multiple orthopedic injuries (R1); expected to be immobilized for several weeks. Metformin held while inpatient (P texture of the letter only); **enoxaparin 40 mg subcutaneous daily** prophylaxis started (A5) |
| 2026-06-30 | outside: same admission, hospital day 10 | orthopedic progress note: **new-onset flaccid paralysis**; significant decrease in lean body mass, mostly in the lower limbs, despite adequate nutritional support (R2); exam: muscle wasting and reduced strength in the lower extremities; alert and oriented; no signs of systemic infection. Plan: physical therapy, nutrition, physiatry for rehabilitation placement (A6) |
| 2026-07-06 to 2026-07-24 | outside: inpatient rehabilitation facility | A6: transferred from the hospital for rehabilitation; home 2026-07-24 walking with a rolling walker; discharged on enoxaparin to finish 2026-08-07 and a short course of **acetaminophen/oxycodone 325/5 mg** as needed (A5) |
| 2026-08-19 (Wed) | practice | last seen. Post-hospital visit (A7): findings as in A7; venous air embolism resolved; flaccid paralysis improving, still present; enoxaparin course finished, acetaminophen/oxycodone no longer taken; outpatient physical therapy referral placed (A7) and left open; BMP, CBC, A1c drawn |

## Records (seed texture: gaps)

- The practice learns of the crash from the patient's own portal message on 2026-06-23.
- ED record (physician note, chest x-ray report, ECG report): requested 2026-06-29, received
  2026-07-09.
- Orthopedic hospital day 10 progress note: arrives alone, late, on 2026-08-14.
- The operative notes, hospital discharge summary and rehabilitation discharge summary never
  arrive by today. The practice learns of the rehabilitation stay and of enoxaparin and
  acetaminophen/oxycodone from a pharmacy prior-authorization request on 2026-07-27 and
  from the patient. A second request goes out after the 2026-08-19 visit.
- The practice's problem list lags: the hospital diagnoses are reconciled only at the
  2026-08-19 visit.

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source never names the orthopedic injuries, only "multiple orthopedic injuries
  requiring surgical fixation" and wasting mostly in the lower limbs. Resolution: every
  record calls them multiple orthopedic injuries fixed operatively, with no bone named; the
  operative notes that would name them never reach the practice (Records). The prior left
  ankle ORIF at 32 is a separate, old operation.
- **R2.** The source dates the orthopedic inpatient note the same day as the ED visit yet
  calls it hospital day 10, and says a "laboratory evaluation" shows lost lean body mass.
  Resolution: the note is dated 2026-06-30 (day 10 of an admission from 2026-06-20); the
  loss of lean mass is the orthopedic and nutrition teams' assessment on exam, not a lab
  value. No inpatient lab values reach the practice. The flaccid paralysis is carried as affecting both lower limbs, where the source puts the wasting and the weakness.
- **R3.** The source calls him a former smoker who quit "2 years ago" at the 2026 crash, and
  he joined in 2023. Resolution: he is a current smoker at joining (nicotine dependence,
  cigarettes) and quits on his own in June 2024, no cessation drug; from then the chart
  records a history of tobacco dependence. 8 pack-years throughout.
- **R4.** The source's profile lists venous air embolism and flaccid paralysis among his
  diagnoses and its inpatient list stamps the embolism with the crash date. Resolution:
  neither exists before 2026-06-20; the embolism is diagnosed in the ED on 2026-06-20 and
  the flaccid paralysis on 2026-06-30. The murmur and confusion are not heard or seen after
  the ED.
- **Left as the source has it:** no formal echocardiogram follows the inconclusive bedside
  study; aspirin 81 mg for primary prevention at his age continues without comment; no
  hospital record names a cause for the flaccid paralysis and no one at the practice
  investigates one beyond requesting records.

## Additions (A): physician to confirm

- **A1.** New-patient visit 2023-09-12 (the source starts with an established patient).
- **A2.** Nightly CPAP for the source's obstructive sleep apnea, set up at the prior
  practice; he uses it most nights. No sleep study values are carried.
- **A3.** ED vital signs (the source says only hypotensive and tachycardic, then a further
  fall): arrival BP 94/58, HR 118, RR 22, SpO2 95% on room air; after line placement BP
  78/46, HR 134, RR 30, SpO2 86%; after treatment BP 104/64, HR 108, RR 20, SpO2 97% on
  oxygen.
- **A4.** ED treatment of the air embolism: catheter hub occluded, left side down with
  head lowered, 100% oxygen by non-rebreather mask; BP and breathing improve within the
  hour; then admitted.
- **A5.** Enoxaparin 40 mg subcutaneous daily as clot prophylaxis for immobilized orthopedic
  trauma, from the admission until 2026-08-07; acetaminophen/oxycodone 325/5 mg, 1 tablet
  every 6 hours as needed, a short course at rehab discharge. Both prescribed outside; the
  practice records them as outside drugs.
- **A6.** Physiatry consult and inpatient rehabilitation 2026-07-06 to 2026-07-24 (the usual
  next step after flaccid lower-limb weakness and lost lean mass), home with a rolling
  walker.
- **A7.** 2026-08-19 practice visit findings: walks short distances with a rolling walker;
  visible wasting of thighs and calves on both sides; hip flexion and knee extension 4-/5 on
  both sides, ankle dorsiflexion 4/5 on both sides; light-touch sensation intact; surgical
  incisions healed; alert and oriented; heart regular without murmur; lungs clear; no calf
  tenderness or swelling. Weight 101.5 kg (about 110 kg before the crash). BP 124/78.
  Outpatient physical therapy referral placed; out of work, a short-term disability form
  follows by portal.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level here says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention. An
  abnormal finding appears only as the source or this file gives it. A report may carry the
  descriptors its kind always carries (a catheter tip position, a procedure's access site),
  but nothing that adds a territory, a severity grade or a symptom, or changes the diagnosis
  or plan.
- Records from the prior practice may mention routine care there with normal results.
- Diabetes and BP visits about every 4 to 6 months, yearly physicals, with A1c, metabolic
  panel, lipids and urine microalbumin/creatinine yearly (normal), foot exams (normal).
  Levels: A1c 7.4 (2023-09), 7.2 (2024-01), 7.0 (2024-07), 7.6 (2025-02), 7.3 (2025-06),
  7.2 (2025-09), 7.1 (2026-01), 6.9 (2026-08), and about 7.0 at the 2024-09 physical; BP about 132/84 to 138/88 on lisinopril, 124/78
  after the hospital stay; LDL about 80 on atorvastatin; fasting glucose about 130 to 150; urine microalbumin/creatinine about 12 mg/g; weight 109 to 112 kg before the crash.
- Yearly diabetic eye exam letters from an optometrist: no retinopathy.
- Prevention: influenza vaccine each fall, COVID vaccines, Tdap 2023, PCV20 once (diabetes),
  hepatitis B two-dose series (diabetes, under 60); hepatitis C and HIV screening once,
  negative; PHQ-2 0 at physicals and 1 on 2026-08-19; AUDIT-C 2. Colorectal screening not
  yet due.
- Refills, pharmacy and CPAP-supply paperwork, portal messages about results and scheduling,
  a missed visit, and a cancelled visit on 2026-08-05 (no ride; he cannot drive yet).
- No booked visit after today. Open today: the physical therapy referral, the second records
  request, and the disability form.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002147 on 2026-08-19. Practice contacts after that day do not
happen; outside care after it still does, and its records are filed as received without
changing the practice's lists. No visit is booked after today.
