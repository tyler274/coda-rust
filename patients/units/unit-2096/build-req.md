# Build requirements: unit-2096 (Todd McKenna, MRN00002096)

Source: synthetic_hospital patient 2096, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz-like wording.

## Who he is (S)

Todd McKenna, born 1976-01-19, male, White, not Hispanic or Latino, English, Medicaid,
lives in Amherst (01002). Construction laborer. Former smoker, quit about 10 years
before his 2023 visit (about 2013). Alcohol: occasional, 1 to 2 drinks a month. No known
drug allergies.

Surgical and injury history: motor vehicle collision at age 35 (about 2011) with bilateral
femoral fractures treated by open reduction and internal fixation, and a splenectomy in the
same operation; complicated postoperative course with multiple blood transfusions. Post-
traumatic arthritis of the right knee since.

Family history: father with hypertension and coronary artery disease (alive, 74 in the
source's frame); mother with type 2 diabetes and hypothyroidism (71). No family history of
immunodeficiency, autoimmune disease or bleeding disorders.

Hepatitis A and B vaccination documented in prior records.

**Joining (A1).** New patient visit 2021-09-20 (Mon). He brings: HIV/AIDS, advanced, not on
antiretroviral therapy (A2: diagnosed 2016 at an outside clinic, took ART for a short time
and stopped, has declined restarting since); anemia of chronic disease; chronic kidney
disease stage 2; post-traumatic arthritis right knee; asplenia after splenectomy. Drugs on
joining: trimethoprim-sulfamethoxazole 160/800 mg (one double-strength tablet) three times
weekly for Pneumocystis prophylaxis, ibuprofen 400 mg every 6 hours as needed for joint pain,
acetaminophen 500 mg every 6 hours as needed for fever or headache. The practice continues and
refills all three.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2021-09-20 | practice | new patient visit (A1). History as above. Labs drawn (P). ART discussed; declines; agrees to keep taking TMP-SMX |
| 2023-11-13 | practice | about 3 months of worsening fatigue, joint pain, tea-colored urine and a recurring lower-leg rash. Mild bilateral swelling and pain of hands, wrists and knees limiting work; reddish lower-leg rash that flares in the cold and does not blanch; several weeks of tingling and weakness in the feet, hard to walk far. T 37.8 C, BP 115/70, HR 90. Multiple non-blanching palpable purpuric lesions below the knees; diminished vibration sense in the distal lower extremities. Labs: creatinine elevated (1.35, R3), C4 low (5 mg/dL, R3), proteinuria and microscopic hematuria on urinalysis (urine protein 2+, urine blood 2+, urine RBC 15/hpf, R3); hepatitis B surface antigen and hepatitis C antibody negative. **Peripheral neuropathy diagnosed** at the visit. Rheumatology referral (A4) |
| 2023-11-17 | practice (result) | serum cryoglobulins positive (A3). **Cryoglobulinemia diagnosed** by phone with the patient |
| 2024-01-10 | outside (letter received 2024-01-18) | rheumatology consult (A4) |
| 2024-11-08 | practice | follow-up about a year after the first presentation: same picture persists (fatigue, hand/wrist/knee swelling and pain limiting work, dark urine, cold-triggered non-blanching lower-leg rash, several weeks of foot tingling and weakness). T 37.8 C, BP 115/70, HR 90. Palpable purpura below the knees; diminished distal vibration sense. Creatinine elevated (1.38), C4 low (6 mg/dL), urine protein 2+, urine blood 2+, urine RBC 12/hpf; hepatitis B surface antigen and hepatitis C antibody again negative; hepatitis A and B vaccination noted as documented. ART offered again, declined |
| 2026-03-09 | practice | last practice visit (P): routine follow-up; CD4 and viral load drawn; next visit booked |
| 2026-05-06 | outside (ED record received 2026-05-11; discharge summary received 2026-05-14) | ED: three weeks of cough, fever and weight loss, and a painful swelling of the right forearm that began as a small pimple and enlarged over the past week. T 38.2 C (100.8 F), BP 125/80, RR 22. Single fluctuant erythematous nodule right forearm; scattered crackles right lung base. WBC 9.0 K/uL, hemoglobin 12.5 g/dL, CD4 120 cells/uL. Sputum Gram stain: filamentous gram-positive, weakly acid-fast rods. Chest X-ray: patchy right lower lobe infiltrates. **Nocardiosis (pulmonary) diagnosed; HIV-related opportunistic skin infection (the forearm nodule, cutaneous nocardiosis) diagnosed** (R1). Admitted 2026-05-06 to 2026-05-09 (A5) |
| 2026-07-13 | outside (discharge summary received 2026-08-14) | ICU admission via ED: two weeks of cough, fever, progressive shortness of breath, severe headache and intermittent confusion; age 50 (R4). T 38.9 C, BP 110/70, HR 102, RR 22. Crackles right lung base; mildly disoriented to time and place. WBC 12.0 K/uL, hemoglobin 13.2 g/dL, platelets 250 K/uL, CD4 50 cells/mm3, HIV viral load 120,000 copies/mL. Chest X-ray: right lower lobe infiltrate. Sputum Gram stain: branching filamentous organisms. **Nocardiosis, pulmonary with CNS involvement** (A6 gives the course and discharge 2026-08-11) |
| 2026-09-08 | practice (booked) | post-hospital follow-up, booked (P) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source stamps "HIV-related opportunistic skin infection" as diagnosed at the
  first visit (2023-11-13), but no skin infection is described there; the lower-leg lesions
  are purpura of cryoglobulinemic vasculitis. The only skin infection in the source is the
  fluctuant right-forearm nodule at the 2026-05-06 ED visit, in a patient whose sputum shows
  Nocardia. Resolution: the opportunistic skin infection is that forearm nodule (cutaneous
  nocardiosis), diagnosed 2026-05-06. The ED's "prior opportunistic infections" is not
  carried; the ED note says advanced HIV not on ART.
- **R2.** The profile lists TMP-SMX prophylaxis from the start and the comorbidities list
  medication noncompliance. Resolution: he is prescribed TMP-SMX three times weekly
  throughout and admits to missing doses; he declines ART at every offer (A2). "Medication
  noncompliance" is added to the problem list at the 2024-02-26 practice visit, after the
  rheumatology letter and his further ART refusal. The source's later lists still show the
  prophylaxis dose; on 2026-05-06 the hospital changes TMP-SMX to treatment dosing (A5), so the
  outside records from May on carry treatment dosing. The practice list stays on the
  prophylaxis dose (last practice visit before the change).
- **R3.** The source gives "elevated creatinine", "low C4", "proteinuria" and "microscopic
  hematuria" with no numbers, and lists CKD stage 2. Resolution: baseline creatinine about
  1.1 mg/dL (eGFR in the high 70s to low 80s) from joining; 1.35 on 2023-11-13 and 1.38 on
  2024-11-08 (eGFR in the low 60s, still stage 2 by filtration); between and after,
  1.2 to 1.3. C4 5 mg/dL (2023) and 6 mg/dL (2024). Urine protein 2+, blood 2+, RBC 15 and
  12/hpf at those visits, and urine protein 1+ and blood 1+ on the checks between flares; urine albumin/creatinine 45 to 60 mg/g on yearly checks from joining
  (supports stage 2), higher (about 300 mg/g) from late 2023.
- **R4.** The ICU note calls him 49; by the birth date he is 50 on 2026-07-13. The chart says
  50. "Nocardiosis two months prior" fits the 2026-05-06 ED visit.
- **R5.** Encounter 1 (2024-11-08) repeats encounter 0's history almost word for word.
  Resolution: it is a follow-up where the same findings persist; the note says so in its own
  words rather than as a new presentation.
- **R6.** The 2023 note says his current viral serologies are negative and the 2024 note says
  it again. Resolution: hepatitis B surface antigen and hepatitis C antibody are drawn and
  negative at both visits.
- **Left as the source has it:** he is never on ART during the source span (2021 to the
  2026-07-13 admission), by his choice. Ibuprofen as needed continues on every list despite
  the kidney disease and nephritis picture; no one stops it and no note comments on it. No
  immunosuppression is given for the cryoglobulinemia. No brain imaging is done at the May
  2026 admission, and no note comments on that. No nephrology referral or kidney biopsy is
  made.

## Additions (A): physician to confirm

- **A1.** 2021-09-20 new-patient visit (years with the practice; the source's first visit
  presents him with an established problem list and prophylaxis).
- **A2.** HIV diagnosed 2016 at an outside clinic; a brief course of ART then, stopped on his
  own; declines restarting at the practice (supports "not on ART" and medication
  noncompliance). His CD4 and viral load at the practice (P levels): CD4 about 170 cells/uL and
  viral load about 85,000 copies/mL in 2021 to 2022; CD4 about 150 and viral load about
  95,000 in 2023 to 2024; CD4 about 135 and viral load about 110,000 on 2026-03-09.
- **A3.** Serum cryoglobulins positive (2023-11-13 draw, resulted 2023-11-17), to support the
  source's cryoglobulinemia.
- **A4.** Rheumatology consult 2024-01-10 (referred 2023-11-13): mixed cryoglobulinemia with
  cutaneous vasculitis, arthralgia, neuropathy and renal involvement, attributed to untreated
  HIV once hepatitis B and C are negative. Recommends ART through an HIV clinic as the
  treatment of the cause; no immunosuppression given his immune status; he declines the HIV
  clinic referral at the visit. Return as needed.
- **A5.** May 2026 admission (2026-05-06 to 2026-05-09) after the ED visit: sputum culture
  grows Nocardia species; started on trimethoprim-sulfamethoxazole two double-strength tablets
  (sulfamethoxazole 800 mg / trimethoprim 160 mg each) by mouth twice daily as treatment, in
  place of the prophylaxis dose; planned course of at least 6 months; infectious disease clinic
  follow-up arranged; ART to be started at that clinic. Discharged home 2026-05-09.
- **A6.** July to August 2026 admission (2026-07-13 to 2026-08-11), received as a discharge
  summary: sputum culture again grows Nocardia species; MRI brain with and without contrast
  shows a single 1.6 cm ring-enhancing lesion in the right frontal lobe with mild surrounding
  edema and no midline shift, consistent with nocardial abscess (the source's CNS involvement).
  Treated with IV trimethoprim-sulfamethoxazole and IV imipenem; ICU for the first days, then
  the medical floor; confusion and headache resolve. Infectious disease starts
  bictegravir/emtricitabine/tenofovir alafenamide 50/200/25 mg daily on 2026-07-28. He tells the
  team he had stopped the May antibiotic after a few weeks (R2). Discharged 2026-08-11 on
  trimethoprim-sulfamethoxazole two double-strength tablets three times daily for at least 12
  months, plus the ART; infectious disease follow-up and repeat brain MRI arranged by them.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention. An
  abnormal finding appears only as the source or this file gives it. A report may carry the
  descriptors its kind always carries, but nothing that adds a territory, a severity grade or a
  symptom, or changes the diagnosis or plan.
- Records from a prior practice may mention routine screening done there, with normal results.
- Follow-up visits every 3 to 6 months 2021 to 2026, with vitals, CBC (hemoglobin about 12.4,
  mild anemia; WBC and platelets normal), CMP, urinalysis, CD4 and HIV viral load about every 6
  months at the levels in A2, urine albumin/creatinine yearly (R3), lipids every couple of years
  (normal). Weight about 79 kg at joining (height 180 cm), about 77 kg by late 2023 and 76 kg in
  March 2026; about 72 kg at the May 2026 ED visit and 70 kg in July 2026 (the source's weight
  loss). BP otherwise normal (about 115 to 125 / 70 to 78).
- Prevention: yearly physicals; influenza vaccine each fall; COVID vaccines; asplenia and HIV
  vaccines: PCV20 once, MenACWY two-dose series and boosters as due, MenB series, Hib once;
  Shingrix two doses; Tdap if due. No live vaccines. Colorectal screening by yearly FIT from
  2022, negative. Hepatitis C screening at joining, negative. PHQ-2/PHQ-9 screens, negative.
  Smoking: former smoker; no pack-year history is recorded, so no lung cancer screening is
  ordered.
- Refills, pharmacy and prior-authorization traffic, portal messages about results,
  refills and scheduling, a missed visit, reminders.
- After 2026-03-09: only messages, refills and outside records filed as received; the practice
  lists stay as of that visit. A portal message from him in August 2026 asking for a visit
  after his discharge leads to the booked visit.
- Booked visit after 2026-08-27: post-hospital follow-up, 2026-09-08.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002096 on 2026-03-09. Practice contacts after that day are
messages, refills and filed records only; outside care after it still happens, and its records
are filed as received without changing the practice's lists. A visit is booked after today.
