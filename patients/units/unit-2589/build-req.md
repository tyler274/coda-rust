# Build requirements: unit-2589 (Noah Foley, MRN00002589)

Source: synthetic_hospital patient 2589, in `source.md` beside this file. This file and
the source are the only authority for clinical facts. Every diagnosis, medication,
result, procedure and specialist finding in the chart must trace to one of:

- **S** a fact in source.md, carried as listed below;
- **R** a resolution below, where the source contradicts itself or leaves a plan unfinished;
- **A** an addition below, listed one by one;
- **P** the primary-care layer described at the end.

Nothing else. No diagnosis, medication, test result, procedure, family member or
life event that is not on these lists. Texture (wording, scheduling, refills, portal
traffic, who signed what) is free, as long as it carries no new clinical fact.

Re-voice everything. The source notes are exam vignettes: never copy a sentence, and
never carry a source clinician's name, its "Active Problem List" dumps, its
"(diagnosed 2020-...)" stamps, or its quiz phrasing ("you note...").

## Who he is (S)

Noah Foley, born 2000-10-27, male, White, not Hispanic or Latino, English-speaking,
Belchertown MA 01007, university student health insurance plan. Undergraduate
student; lives in a shared dormitory during the school year (ED social history).
Never smoker. Alcohol socially, 2 to 3 drinks a week. No known drug allergies. No
surgical history. Family history: father type 2 diabetes, mother essential
hypertension. No other family members recorded. Sexually active with multiple
partners, inconsistent condom use (S, from April 2025 on; partners' sex not recorded).

Joined the practice 2024-09-12 (A1, new-patient preventive visit) with no problems,
no medications, no allergies. Height 178 cm, weight about 74 kg, steady throughout.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2024-09-12 | practice | new-patient preventive visit (A1): healthy, no meds, NKDA, family and social history as above; HIV-1/2 Ag/Ab nonreactive and hepatitis C antibody nonreactive (P, once); lipid panel normal (P); PHQ-2 0 |
| 2024-10-08 | practice | nurse visit: influenza and COVID-19 vaccines (P) |
| 2025-04-24 | practice | painless penile ulcer for 2 weeks, unchanged in size, no discharge; condomless sex with multiple partners in the past month; no PMH, no meds, NKDA; exam: round, sharply demarcated ulcer with clean base and indurated edges, non-tender inguinal lymphadenopathy; **RPR negative**. **Primary syphilis diagnosed** (clinical), **high risk sexual behavior** recorded; **penicillin G benzathine 2.4 million units IM once, given in clinic** (R1) |
| 2025-06-10 | outside (regional hospital ED) | fatigue and sore throat for more than 2 weeks, about two months after the ulcer visit; intermittent low-grade fevers, night sweats, malaise, poor appetite, dry cough; no travel, no recent antibiotics, no sick contacts; shared dorm, multiple partners, inconsistent condoms. Vitals within normal limits. Enlarged tonsils with white exudate, scattered white plaques on the buccal mucosa, mild cervical and occipital nodes, axillary and inguinal nodes enlarged, spleen tip palpable. Rapid strep negative, heterophile (Monospot) negative; **Hgb 10.2 g/dL, platelets 95,000/mm3, WBC 2,400/mm3**. HIV-1/2 Ag/Ab **reactive**, HIV-1 RNA **1,870,000 copies/mL** (resulted 2025-06-12, A2). **Acute HIV infection** diagnosed; discharged to follow up with primary care and infectious disease. Record reaches the practice late (2025-07-02); the practice first hears from the patient |
| 2025-06-13 | practice | post-ED visit (P): patient brings ED paperwork; acute HIV infection added to the problem list; infectious disease referral placed. No new labs |
| 2025-07-16 | outside | infectious disease intake missed (no-show, texture); ID office notifies the practice; RN outreach |
| 2025-10-15 | outside (dermatology) | 2-week non-itchy rash starting on the trunk and spreading to the limbs; feels unwell with low-grade fever and malaise, sore throat, mild headaches; history of primary syphilis and acute HIV infection; takes no medications; NKDA; multiple partners, inconsistent condoms. Exam: diffuse symmetric maculopapular rash along skin-cleavage lines of the back ("Christmas tree" distribution), involving palms and soles; no lymphadenopathy, no hepatosplenomegaly; rest of exam unremarkable. **RPR positive**; CBC within normal limits; hepatitis panel negative (HBsAg nonreactive, hepatitis C antibody nonreactive). **Secondary syphilis**. Letter asks the practice to treat (R2, R3). Letter received 2025-10-16 |
| 2025-10-17 | practice | **secondary syphilis** on the problem list, **primary syphilis resolved**; RPR titer drawn before treatment: **reactive 1:64** (A3); **penicillin G benzathine 2.4 million units IM once** (A3); ID referral re-sent; influenza vaccine (P) |
| 2025-11-06 | outside (infectious disease) | ID intake (A4): baseline **CD4 380 cells/uL, HIV-1 RNA 64,000 copies/mL**, genotype without major resistance mutations; **bictegravir 50 mg / emtricitabine 200 mg / tenofovir alafenamide 25 mg one tablet daily started**. No letter ever reaches the practice; the practice learns of the drug from a pharmacy fill notice on 2025-11-07 and the list lags until April |
| 2026-01-14 | outside (infectious disease) | ID follow-up (A4): adherent; **HIV-1 RNA 45 copies/mL, CD4 430 cells/uL**. Letter received late, 2026-03-20 |
| 2026-04-10 | practice (last seen) | follow-up: feels well, rash gone; **bictegravir/emtricitabine/tenofovir alafenamide added to the list** as prescribed by ID; RPR titer **reactive 1:8** (A5); **acute HIV infection resolved and HIV disease recorded** (A6); **PCV20** (A7); PHQ-9 2 |
| 2026-05-13 | outside (infectious disease) | ID follow-up (A4): **HIV-1 RNA not detected (<20 copies/mL), CD4 505 cells/uL**; continue same regimen. Letter received 2026-06-04 and filed; practice lists unchanged |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The April 2025 note ends at the negative RPR with no plan, yet the later
  notes carry "primary syphilis" as diagnosed that day. Resolution: the PCP makes a
  clinical diagnosis from the classic chancre and exposure, notes that an early RPR
  can be negative, and gives penicillin G benzathine 2.4 million units IM in clinic
  the same day. The ED's "no recent antibiotics" stays (the shot was seven weeks
  earlier and is not an ongoing antibiotic). The source's "no medications" at the
  October dermatology visit stays true: the single shot is not a home medication.
- **R2.** Secondary syphilis in October 2025 after treated primary syphilis in April:
  read as reinfection, since both later encounters record continued condomless sex
  with multiple partners. No note debates relapse versus reinfection.
- **R3.** The dermatology note reports an HIV antigen/antibody test as negative four
  months after acute HIV infection was diagnosed. An untreated infection cannot test
  negative then. That result is not carried: the dermatology letter lists HIV as a
  known diagnosis and reports RPR, CBC and hepatitis panel only.
- **R4.** The source profile lists primary syphilis, acute HIV infection and secondary
  syphilis at once. Each starts at its own encounter: primary syphilis 2025-04-24,
  acute HIV infection at the ED 2025-06-10 (on the practice list from 2025-06-13),
  secondary syphilis 2025-10-15 (on the practice list 2025-10-17). High risk sexual
  behavior is recorded 2025-04-24.
- **R5.** The source gives the ED cytopenias but no white cell differential and no
  follow-up count. The ED values are carried as given; the dermatology CBC is
  "within normal limits" with no numbers. No other blood count appears in the chart.
- **Left as the source has it:** no HIV test, gonorrhea or chlamydia test, or HIV
  pre-exposure prophylaxis at the April 2025 ulcer visit; nothing is given for the
  white buccal plaques at the ED; no antiretroviral therapy between the June 2025
  diagnosis and the November 2025 ID intake (he missed the July intake). No note
  remarks on any of these.

## Additions (A): physician to confirm

- **A1.** The 2024-09-12 new-patient preventive visit (seed: about 2 years with the
  practice); the April 2025 ulcer visit is then an established-patient visit.
- **A2.** ED HIV testing that supports the source's acute HIV diagnosis: HIV-1/2
  Ag/Ab reactive; HIV-1 RNA 1,870,000 copies/mL.
- **A3.** Treatment of the source's secondary syphilis at the practice two days after
  the dermatology visit: pretreatment RPR titer reactive 1:64, penicillin G
  benzathine 2.4 million units IM once. No lumbar puncture (no neurologic or ocular
  findings beyond mild headache).
- **A4.** Standard HIV care after the source ends, run by an outside infectious
  disease clinic: intake 2025-11-06 (CD4 380 cells/uL, HIV-1 RNA 64,000 copies/mL,
  genotype without major resistance mutations), bictegravir/emtricitabine/tenofovir
  alafenamide one tablet daily; follow-up 2026-01-14 (HIV-1 RNA 45 copies/mL, CD4 430)
  and 2026-05-13 (HIV-1 RNA <20 copies/mL, not detected; CD4 505).
- **A5.** Six-month RPR after treatment of secondary syphilis, 2026-04-10: reactive
  1:8 (an eightfold fall from 1:64, an adequate response).
- **A6.** The acute HIV label outlasts its phase: on 2026-04-10 acute HIV infection is
  resolved and HIV disease (B20) recorded.
- **A7.** Pneumococcal conjugate vaccine (PCV20), indicated by HIV infection, given
  2026-04-10.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not
  mention. An abnormal finding appears only as the source or this file gives it.
- Records from a prior pediatric or student health office may mention routine
  childhood and college-entry vaccines as complete.
- One-time HIV and hepatitis C screening at joining (both nonreactive); a lipid panel
  at joining, normal; PHQ-2 or PHQ-9 screens, normal.
- Influenza vaccine each fall (2024, 2025); a COVID-19 vaccine in fall 2024.
- Counseling on condoms, abstaining until treated, and partner notification; the
  health department's partner services may call (texture).
- Portal messages about results, scheduling and records; an outreach call after the
  missed ID intake; forms; a pharmacy fill notice.
- No visit is booked after 2026-08-27; nothing is pending today.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002589 on 2026-04-10. Practice contacts after that day are
only messages and records; outside care after it still happens, and its records are
filed as received without changing the practice's lists. No visit is booked after today.
