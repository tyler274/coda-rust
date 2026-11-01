# Build requirements: unit-2552 (Henry Dabrowski, MRN00002552)

Source: synthetic_hospital patient 2552, in `source.md` beside this file. This file and
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
"(diagnosed 2020-07-15)" stamps, or its quiz text ("CSF staining is shown").

## Who he is (S)

Henry Dabrowski, born 1989-12-30, male, White, not Hispanic or Latino, English,
Amherst MA 01002, Medicaid. International aid worker (field postings abroad). Former
smoker, quit 2019. No alcohol. No known drug allergies. Surgical history: appendectomy
(remote, no records). Family history: father hypertension, mother type 2 diabetes.
Single.

Joined the practice 2021-09-14 (A1, new-patient visit). He brought one problem: a
**history of P. vivax malaria** from an earlier field posting, treated abroad years
before joining, no records (S, R4). No medications. Past injection drug use is not
known at joining; he discloses it on 2023-04-21 (S, R5).

The practice last saw him on **2024-06-18** (last_seen). Everything after that reaches
the chart as messages, refills and outside records, filed as received; the practice's
problem and medication lists stay as they were on 2024-06-18. Nothing is booked.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2021-09-14 | practice | new-patient visit with physical (A1): history as above; exam normal; weight 76.0 kg, height 180 cm; routine labs normal; Tdap (P). HIV screening not done (P gap, R5) |
| 2022-09-20 | practice | physical cancelled by patient (P); not rescheduled that year |
| 2023-04-04 | practice | annual physical (P); feels well; one-time HIV and hepatitis C screening ordered with routine labs |
| 2023-04-06 | practice | HIV-1/2 antigen/antibody reactive, HIV-1 antibody confirmed on differentiation; hepatitis C antibody nonreactive (A2). PCP calls him. **HIV infection recorded.** Baseline HIV labs and a tuberculin skin test arranged |
| 2023-04-07 | practice lab | **CD4 350 cells/mm3, HIV-1 viral load 55,000 copies/mL** (S); genotype with no major resistance mutations, hepatitis B surface antigen negative and surface antibody positive (immune), RPR nonreactive (P baseline) |
| 2023-04-17 | practice nurse | TST placed (R1) |
| 2023-04-19 | practice nurse | TST read at 48 hours: **7 mm induration** (S). Chest x-ray, 2 views, same day: no evidence of active tuberculosis (S) |
| 2023-04-21 | practice | HIV evaluation visit (S enc 0): no cough, fever, night sweats or weight loss; exam unremarkable; discloses past injection drug use, none in years (R5); CD4 350, VL 55,000; CXR negative; TST 7 mm. **Latent tuberculosis infection diagnosed.** **Bictegravir/emtricitabine/tenofovir alafenamide 50/200/25 mg daily started; isoniazid 300 mg daily (9-month course) and pyridoxine 50 mg daily started** (R2). Referred to Infectious Disease. PCV20 and MenACWY dose 1 (P) |
| 2023-05-16 | outside | Infectious Disease consult letter (A3): agrees with ART, INH plan; HIV care co-managed; monthly LFTs while on INH |
| 2023-05-19 | practice lab | hepatic panel normal (P, INH monitoring) |
| 2023-06-23 | practice nurse | MenACWY dose 2 (P); draw: hepatic panel normal, **CD4 362, VL 140** (A3) |
| 2023-10-15 | outside | ED then admission (S enc 1), 2023-10-15 to 2023-10-30, regional hospital. 3 weeks of progressively worse headache, intermittent low-grade fevers, trouble concentrating, occasional blurred vision; began shortly after a backpacking trip in the Pacific Northwest (hostels, wooded hiking); 5 kg weight loss over the month (weight 70.5 kg). Afebrile; oriented but slow to respond; mild nuchal rigidity; bilateral papilledema. CT head: no mass lesion or hydrocephalus. LP: opening pressure elevated, lymphocytic predominance, protein elevated, glucose low (no CSF numbers carried, R3). India ink positive for encapsulated yeast, CSF cryptococcal antigen positive titer 1:1024, CSF fungal culture Cryptococcus neoformans (A4). **CD4 170, VL 3,400**; he had missed ART doses for several weeks during the trip (A5). **Cryptococcal meningitis diagnosed.** Induction liposomal amphotericin B with flucytosine for 2 weeks, repeat therapeutic LPs (inpatient only). ART, INH and pyridoxine continued. Discharged on **fluconazole 400 mg daily** and **trimethoprim-sulfamethoxazole 160/800 mg daily** (R2, A5) |
| 2023-11-03 | practice | post-discharge visit (P): headaches gone, concentrating better; weight 71.0 kg; meds reconciled; BMP and CBC normal; influenza and updated COVID vaccines (P) |
| 2023-11-14 | outside | ID follow-up letter (A3): CSF culture final as above; continue fluconazole 400 mg; continue TMP-SMX; adherence good since discharge |
| 2024-01-23 | practice | follow-up (P): **INH 9-month course complete; isoniazid and pyridoxine stopped** (R2); latent TB treated, kept on the list; weight 73.0 kg; CMP, CBC normal; **CD4 228, VL <20** (A3); lipids normal |
| 2024-03-19 | outside | ID letter (A3): CD4 241, VL <20; continue fluconazole and TMP-SMX, recheck in 3 months |
| 2024-05-28 | outside | ID letter received before the visit (A3): CD4 266, VL <20 |
| 2024-06-18 | practice | **last practice visit** (P, A6): pre-travel visit before a long field posting in rural East Africa starting early July; well; weight 74.5 kg; ID labs from May reviewed; CBC, BMP normal. Malaria prophylaxis discussed; **atovaquone-proguanil 250/100 mg daily started for the trip** (A6); extended supplies of ART, fluconazole and TMP-SMX arranged; country vaccines through his organization's travel clinic |
| 2024-07 to 2025-03 | messages | refill traffic and a portal message from abroad (P) |
| 2025-04-08 | outside | admission (S enc 2), 2025-04-08 to 2025-04-12, regional hospital. Returned 3 weeks before after about 9 months in rural East Africa (R6); took no malaria prophylaxis (R6). 4 days of fevers, chills, fatigue, vague abdominal discomfort; fever and sweats every other day with headache and muscle aches. T 39.5 C. Tired, mildly diaphoretic, scleral icterus, spleen palpable 2 cm below the left costal margin. **Hemoglobin 9.2 g/dL, platelets 90,000/mm3, reticulocytes 4.8%, LDH 340 U/L** (S). Thin and thick smears: P. vivax, parasitemia 0.6%; total bilirubin 2.3 mg/dL; G6PD normal (A7). **P. vivax malaria, chloroquine-sensitive** (R7): chloroquine 3-day course, then primaquine 30 mg base daily for 14 days (A7). ART, fluconazole, TMP-SMX continued. Discharge summary received 2025-04-17; practice lists unchanged |
| 2025-04-21 | message | portal message: home, was admitted for malaria, will follow with ID; declines a practice visit for now (P) |
| 2025-05-20 | outside | ID letter (A3): primaquine finished; hemoglobin 12.6, platelets 224; CD4 284, VL <20 |
| 2025-12-09 | outside | ID letter: missed two ID visits this fall; pharmacy shows no ART fills since October (S nonadherence, texture) |
| 2026-04-11 | outside | ICU admission (S enc 3), 2026-04-11 to 2026-04-29, regional hospital. 2 weeks of low-grade fever, headache and malaise, gradually worse; has not taken ART for the past 6 months. T 38.2 C, BP 120/75, pulse 88, RR 16. Nuchal rigidity, positive Brudzinski sign; fundi normal; no focal deficits. **CD4 45.** LP: **opening pressure 280 mmH2O, WBC 20/uL (80% lymphocytes), protein 50 mg/dL, glucose 25 mg/dL (serum glucose 90 mg/dL), India ink negative**, cryptococcal antigen sent. Next steps (A8): CSF cryptococcal antigen positive titer 1:256; CSF fungal culture Cryptococcus neoformans; VL 162,000. Relapsed cryptococcal meningitis; reinduction liposomal amphotericin B with flucytosine for 2 weeks, daily therapeutic LPs until pressure normalized; discharged on fluconazole 800 mg daily consolidation and TMP-SMX; ART to restart with ID at 4 to 6 weeks. Discharge summary received 2026-05-05; practice lists unchanged |
| 2026-05-27 | outside | ID letter (A8): **bictegravir/emtricitabine/tenofovir alafenamide restarted 2026-05-19**; fluconazole 800 mg until 8 weeks of consolidation, then 200 mg; TMP-SMX continued; headache resolved |
| 2026-07-28 | outside | ID letter (A8): VL 410, CD4 88; fluconazole stepped down to 200 mg daily maintenance; taking ART daily. Filed; no practice reply needed |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source puts the TST in encounter 0 and reads it at 48 hours; the visit is a
  Friday. Resolution: placed Monday 2023-04-17 at a nurse visit, read Wednesday
  2023-04-19, reviewed at the 2023-04-21 visit with the chest x-ray done on 04-19.
- **R2.** The source lists all five drugs at every encounter, including the visit where
  HIV is first found. Resolution: ART, isoniazid and pyridoxine start 2023-04-21;
  fluconazole and TMP-SMX start at the October 2023 discharge (fluconazole after
  cryptococcal induction, TMP-SMX because CD4 fell to 170). Isoniazid and pyridoxine
  stop 2024-01-23 after the 9-month course. The 2025 and 2026 hospital lists still
  copy isoniazid and pyridoxine as outside lists do; the practice's list never does
  after January 2024.
- **R3.** Encounter 1 gives the CSF findings only qualitatively and "CSF staining is
  shown". Resolution: CSF numbers are not invented; the ED and hospital records say
  elevated, lymphocytic, high protein, low glucose; the stain becomes India ink positive (A4).
- **R4.** History of cryptococcal meningitis is on the source's list from encounter 0,
  but encounter 1 makes the diagnosis. Resolution: cryptococcal meningitis starts
  2023-10-15; before then no note mentions it. One problem-list entry
  ("Cryptococcal meningitis", B45.1) carries it after that, on maintenance fluconazole.
  History of P. vivax malaria is on the list from the start and stays a history entry
  (Z86.19), known at joining.
- **R5.** Encounter 0 carries a history of intravenous drug use; encounter 1 says he
  had no prior conditions and had not seen a physician in years. Resolution: the IV
  drug use is remote and first disclosed 2023-04-21; "no prior conditions" and "not
  seen a physician in years" are not carried (he was a practice patient since 2021).
  The 2021 and 2022 practice care never ordered an HIV test; nobody remarks on it.
- **R6.** Encounter 2 says 18 months in rural western Uganda, which cannot fit a
  practice visit on 2024-06-18. Resolution: he left in early July 2024 and was away
  about 9 months; the chart names the region as rural East Africa. "Did not take any
  malaria prophylaxis" stays: atovaquone-proguanil was prescribed in June 2024 and he
  did not take it (the source's medication noncompliance).
- **R7.** "Chloroquine-sensitive malaria prophylaxis" is the source's encounter-2
  diagnosis label. Resolution: the admission diagnoses chloroquine-sensitive P. vivax
  malaria (every-other-day fevers, hemolysis, splenomegaly) and treats it with
  chloroquine; no chloroquine prophylaxis is prescribed. The 2026 list carries
  "malaria, treated 2025" only in the hospital's history.
- **R8.** Encounter 3 dates the cryptococcal meningitis to "2020"; that is 2023 on the
  re-dated timeline. Its HPI says "returns" to the ED; carried as a second admission.
- **Left as the source has it:** fluconazole stays 400 mg after the October 2023
  consolidation through mid-2024 rather than stepping down to 200 mg maintenance; no
  note comments on it. TMP-SMX continues with CD4 above 200 through 2024. No practice
  visit follows the 2025 malaria or the 2026 relapse (he was not seen after 2024-06-18).
  Recombinant zoster and hepatitis A vaccines are never given.

## Additions (A): physician to confirm

- **A1.** 2021-09-14 new-patient visit (five years with the practice; the source's first
  encounter reads as a known patient presenting with a new result).
- **A2.** The screening HIV test that produced the "positive test" (the source names
  none), and its one-time hepatitis C companion, negative.
- **A3.** Infectious Disease co-management with letters and CD4/viral load trend:
  2023-06 CD4 362, VL 140; 2023-11 continue; 2024-01 CD4 228, VL <20; 2024-03 CD4 241,
  VL <20; 2024-05 CD4 266, VL <20; 2025-05 CD4 284, VL <20, Hb 12.6, platelets 224.
  Hemoglobin back to his usual (about 14.5) by the 2026 admission.
- **A4.** CSF India ink positive, CSF cryptococcal antigen titer 1:1024, culture
  Cryptococcus neoformans in October 2023, to support the source's diagnosis.
- **A5.** October 2023 CD4 170 and VL 3,400 with missed doses during the trip, which
  gives TMP-SMX its indication and fits the source's medication noncompliance.
- **A6.** The 2024-06-18 pre-travel visit and atovaquone-proguanil 250/100 mg daily
  (the source's "did not take any prophylaxis" for an aid worker going to a malarious
  area).
- **A7.** Malaria smear (P. vivax, 0.6%), total bilirubin 2.3 for the icterus, G6PD
  normal before primaquine, and chloroquine then primaquine as standard treatment.
- **A8.** The relapse's next steps after the source ends: CSF antigen titer 1:256,
  culture C. neoformans, VL 162,000, reinduction, fluconazole 800 mg consolidation then
  200 mg, ART restarted 2026-05-19, July 2026 VL 410 and CD4 88. All by the hospital and
  ID; the practice files the records.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Physicals 2021 and 2023 with CBC, CMP, lipids, normal; PHQ-2 negative; BP normal.
- HIV baseline labs as listed; hepatic panel monitoring on INH, normal; CBC and BMP on
  TMP-SMX and fluconazole, normal.
- Vaccines: Tdap 2021; influenza fall 2021, 2022 (nurse visits) and 2023; updated COVID
  2023; PCV20 2023; MenACWY two doses 2023. COVID primary series 2021 elsewhere by report.
- Refills, pharmacy traffic, portal messages about results, refills and scheduling, a
  cancelled physical, reminders, records received and scanned.
- Weight 76 kg at joining, 75.5 kg in 2023, 70.5 kg at the October 2023 admission,
  recovering to 74.5 kg by June 2024. Height 180 cm.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002552 on 2024-06-18. Practice contacts after that day are
only messages and refills; outside care after it still happens, and its records are
filed as received without changing the practice's lists. Nothing is booked.
