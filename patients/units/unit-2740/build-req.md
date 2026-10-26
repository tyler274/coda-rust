# Build requirements: unit-2740 (Jennifer Michaud, MRN00002740)

Source: synthetic_hospital patient 2740, in `source.md` beside this file. This file and
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
"(diagnosed 2020-07-15)" stamps, its unfinished "Given the patient's age..." plan, or
its second-person exam text ("you note").

## Who she is (S)

Jennifer Michaud, born 1969-11-26, female, White, not Hispanic or Latino, English,
private insurance through her employer, lives in the seed's town (01060). Works as an
administrative assistant. Former smoker, quit about 10 years ago, 10 pack-years. No
alcohol. Allergies: no known drug allergies.

Surgical history: right carpal tunnel release (2018); tonsillectomy in childhood.

Family history: mother with Hashimoto thyroiditis; father with colorectal cancer
diagnosed at 65; a sister with rheumatoid arthritis.

**Joined the practice 2021-09-14** (A1), age 51, with **hypothyroidism on levothyroxine
100 mcg daily** and nothing else on her lists. Rheumatoid arthritis, restless legs,
lichen sclerosus, colorectal neoplasia and lymphoma are not known at joining; each
starts at its encounter below.

Chart texture (seed): **sparse**. Most outside care reaches the practice as the
patient's own account, a short note or a pharmacy fill, with few numbers. Per contact
below.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2021-09-14 | practice | new-patient visit (A1): history as above; height 165 cm, weight 68 kg, BP normal; baseline labs normal; prior records requested |
| 2021-10-05 | outside records | prior practice records (P): colonoscopy 2016 normal (done early for her father's history), Pap with HPV co-test 2019 negative, screening mammogram 2020 normal, Tdap 2017, COVID primary series spring 2021 |
| 2022-09-08 | practice | annual physical; reports about 5 months of stiffness and pain in both hands and wrists, worst in the morning; swelling of wrists and PIP joints on exam; **rheumatology referral** (A2) |
| 2022-10-20 | outside: rheumatology | consult; a short consult letter reaches the practice 2022-11-01. Six months of symmetric joint pain, morning stiffness over an hour, easing with activity; swelling of wrists and PIPs; no recent infection. Exam: warm, red, tender affected joints, slightly reduced range of motion. RF positive, anti-CCP positive, ESR 50 mm/hr, CRP 3.0 mg/dL; CBC, renal and liver tests normal. **Rheumatoid arthritis diagnosed. Methotrexate 15 mg weekly, folic acid 1 mg daily and prednisone 5 mg daily started** (R1) |
| 2022-12-27 | outside: rheumatology | follow-up (R3, moved off the holiday). No letter reaches the practice. Facts carried by the outside record only as the patient reports them (portal 2023-01-10): wrists still painful with long morning stiffness; numbness and tingling in thumb, index and middle fingers, worse at night; a wrist-flexion test reproduced the tingling; hand x-rays showed bone thinning near the joints and some erosions; same medicines. Full source values (ESR 50 mm/hr, CRP 25 mg/L, RF positive, anti-CCP positive, TSH normal; x-rays with periarticular osteopenia and marginal erosions; tender swollen PIPs and MCPs) are what rheumatology recorded; the practice chart never receives them |
| 2023-03-14 | practice | follow-up; describes a creeping, uncomfortable feeling in both legs at night with an urge to move, relieved by walking, for about 4 months; **neurology referral** (A3) |
| 2023-05-01 | outside: neurology | consult; short letter received 2023-05-12. About six months of a deep urge to move the legs with a crawling feeling, worse at night, relieved by walking or stretching; no back pain, weakness or sensory change; neurological exam normal, pulses intact. Hgb 12.5 g/dL, ferritin 15 ng/mL, creatinine 0.9 mg/dL, TSH 2.5 mIU/L. **Restless legs syndrome diagnosed. Ferrous sulfate 325 mg every other day started** (A4) |
| 2023-06-06 | practice (portal) | about six months of vulvar itching, worse at night, no relief from OTC antifungal creams, no discharge or bleeding; asks to see gynecology; **gynecology referral** (A5) |
| 2023-06-22 | outside: gynecology | visit; no letter. Patient's account (portal 2023-06-23): thin, white patchy skin of the vulva and around the anus; gynecologist thinks lichen sclerosus, plans a biopsy next visit; use OTC hydrocortisone meanwhile (R5). **Lichen sclerosus recorded** (clinical, R6). No lymphadenopathy |
| 2023-08-21 | outside: gynecology | follow-up; worse despite OTC creams; white patchy atrophic areas, no vesicles or ulcers; vulvar punch biopsy. A copy of the pathology report reaches the practice 2023-08-30: hyperkeratosis, atrophic epidermis, band-like lymphocytic infiltrate in the dermis; consistent with lichen sclerosus |
| 2023-09-12 | practice | annual physical; she **stopped ferrous sulfate in August for constipation** (A4); legs still bother her some nights |
| 2023-10-23 | outside: gynecology | follow-up; no letter. Known from a pharmacy fill 2023-10-24 and her portal message 2023-10-25: itching still worse at night and not helped by OTC hydrocortisone; gynecologist saw white plaques with scarring and loss of normal architecture including the labia minora; no groin nodes; **clobetasol 0.05% ointment started** (A6) nightly for a month, then twice weekly. No second biopsy (R5) |
| 2025-03-11 | practice | about two months of crampy lower abdominal pain relieved by a bowel movement, alternating constipation and diarrhea, recently worse at night; appetite down with work stress; weight down; no travel or antibiotics; **gastroenterology referral** (A7) |
| 2025-04-14 | outside: gastroenterology | consult; letter received 2025-04-22. Ten-pound weight loss over two months (68 to about 63.5 kg, R12); abdomen soft with mild diffuse tenderness, no rebound or guarding; rectal exam unremarkable. Hgb 10.8 g/dL, MCV 74 fL, ferritin 15 ng/mL, CRP 8 mg/L, tissue transglutaminase IgA negative. **Iron deficiency anemia**; colonoscopy planned; **ferrous sulfate 325 mg daily restarted** (A7, R10) |
| 2025-05-08 | outside: endoscopy | colonoscopy (A8); report and pathology received 2025-05-20: 22 mm sessile polyp in the ascending colon removed by endoscopic mucosal resection (piecemeal), site clipped; rest of the colon normal to the cecum. Pathology: tubulovillous adenoma with high-grade dysplasia, no invasive carcinoma. **Colorectal neoplasia (advanced adenoma of the ascending colon)**; surveillance colonoscopy in about 6 months |
| 2025-07-15 | practice | follow-up: bowel symptoms eased since the colonoscopy; weight 66 kg; Hgb 11.6 g/dL, ferritin 32 ng/mL (A9); continue iron |
| 2025-08-12 | practice | annual physical; well; no fevers or sweats yet |
| 2025-10-21 | outside: ED | short ED note received 2025-10-23. Two months of night sweats, low-grade fevers and 10-lb unintended weight loss (to about 61.5 kg, R12); a lump on the left side of the neck slowly enlarging. Firm, rubbery, non-tender 3 cm node in the left cervical chain; no hepatosplenomegaly. Hgb 11.2 g/dL, WBC 7,000/uL, platelets 250,000/uL, LDH 245 U/L, ESR 50 mm/hr, ANA positive, RF elevated. Discharged with urgent referral for excisional biopsy (R9) |
| 2025-10-30 | outside: general surgery | excisional biopsy of the left cervical node (R9); pathology received 2025-11-04: diffuse proliferation of large B cells, **diffuse large B-cell lymphoma** |
| 2025-11-04 | practice | phone call: diagnosis discussed; **hematology/oncology referral** (A10); rheumatology contacted |
| 2025-11-10 | outside: oncology | consult; letter received 2025-11-20 (A10): PET-CT 2025-11-07 shows FDG-avid nodes in the left neck only, largest 3.1 cm, no disease elsewhere, marrow and spleen not involved: stage I with B symptoms. Hepatitis B surface antigen and core antibody negative, HIV negative, echocardiogram LVEF 60%. **Methotrexate stopped** (with rheumatology's agreement); folic acid stopped with it. Plan: R-CHOP every 21 days for 6 cycles from 2025-11-13. Prednisone 5 mg continues on non-chemotherapy days |
| 2025-11-13 to 2026-02-26 | outside: oncology | six cycles of R-CHOP (A10); only her portal messages and a telehealth check reach the chart (fatigue, nausea managed by oncology) |
| 2026-04-09 | outside: oncology | end-of-treatment PET-CT (A11); letter received 2026-04-20: complete metabolic response (Deauville 2); surveillance every 3 months |
| 2026-04-28 | outside: rheumatology | follow-up (A12), patient's account at the 2026-05-12 visit: joints quiet since chemotherapy; stays on prednisone 5 mg daily, no DMARD for now, rheumatology to revisit if her arthritis flares |
| 2026-05-12 | practice | post-treatment follow-up; weight 64 kg; Hgb 12.4 g/dL, ferritin 58 ng/mL (A9) |
| 2026-06-11 | outside: endoscopy | surveillance colonoscopy (A8, delayed for chemotherapy); report received 2026-06-19: clipped resection scar in the ascending colon with no residual polyp, scar biopsies benign colonic mucosa; rest of the colon normal; next colonoscopy in 1 year |
| 2026-07-16 | outside: oncology | surveillance visit, patient's account (portal 2026-07-21): no evidence of lymphoma on exam and labs; next in 3 months |
| 2026-09-15 | practice | booked: annual physical (P) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The profile lists rheumatoid arthritis, methotrexate, folic acid and prednisone
  from the start, yet the first rheumatology visit is a diagnostic workup of a 6-month
  illness and the later stamps date the diagnosis to it. Resolution: RA is diagnosed on
  2022-10-20 and all three drugs start there. The ED's "methotrexate and low-dose
  prednisone for the past 8 years" is not carried: about three years by then.
- **R2.** The first rheumatology note says she denies autoimmune family history, and the
  GI note says no GI family history, against a sister with RA and a father with
  colorectal cancer. Resolution: the profile's family history holds throughout; outside
  letters carry it.
- **R3.** Encounter 1's practice date (2022-12-26) is the observed Christmas holiday.
  Resolution: the visit is 2022-12-27.
- **R4.** Ages follow the birth date: 52 in October and December 2022, 53 at neurology
  and gynecology in 2023, 55 at GI and the ED in 2025. The source's "52" at neurology and
  "54" at GI are not carried. Years the source states ("September and November 2020")
  follow the re-dated timeline (June and August 2023).
- **R5.** The source performs a vulvar biopsy at both the second and third gynecology
  visits with the same histology, and at the third says OTC hydrocortisone failed.
  Resolution: one biopsy, on 2023-08-21; the third visit's pathology is that report
  re-cited. She uses OTC antifungal creams before June 2023 and OTC hydrocortisone from
  June to October 2023; clobetasol starts at the third visit.
- **R6.** Lichen sclerosus is stamped to the first gynecology visit, before the biopsy.
  Resolution: recorded as a clinical diagnosis at that visit (via her account) and
  confirmed by the August biopsy.
- **R7.** Several home lists drop levothyroxine. Resolution: levothyroxine 100 mcg daily
  continues unchanged throughout.
- **R8.** The ED home list omits methotrexate and prednisone while its history says she
  takes both. Resolution: both continue up to the lymphoma diagnosis; the ED list is
  incomplete, as outside lists are.
- **R9.** The ED "performs" an excisional lymph node biopsy. Resolution: the ED refers
  her, and general surgery excises the node as an outpatient on 2025-10-30.
- **R10.** The GI plan stops mid-sentence. Resolution: colonoscopy and oral iron (A7, A8).
- **R11.** CRP is 3.0 mg/dL in October 2022 and 25 mg/L in December 2022. Both are
  carried in their own units (the December value never reaches the practice).
- **R12.** Two separate 10-lb losses. Resolution: 68 kg baseline, about 63.5 kg in April
  2025, back to about 66 kg by July and August 2025 after the polyp was removed and iron
  started, then about 61.5 kg by the October 2025 ED visit; about 64 kg by May 2026.
- **R13.** The ED's "prior colorectal neoplasia" is the advanced adenoma resected in May
  2025 (A8).
- **R14.** The December 2022 median-nerve symptoms and wrist-flexion finding are carried
  as findings only, as the source gives them; no carpal tunnel diagnosis, splint, study
  or surgery is added. Neurology's "no sensory changes" in May 2023 stays as written.
- **Left as the source has it:** erosive RA two months into methotrexate gets no
  escalation; nobody orders bone density or adds calcium and vitamin D for years of
  prednisone; restless legs get iron only, no other drug; after chemotherapy no DMARD is
  restarted. The chart keeps these without comment.

## Additions (A): physician to confirm

- **A1.** New-patient visit 2021-09-14 (years with the practice from the seed); she
  brings hypothyroidism on levothyroxine 100 mcg.
- **A2.** The 2022-09-08 practice visit that refers her to rheumatology.
- **A3.** The 2023-03-14 practice visit that refers her to neurology.
- **A4.** Ferrous sulfate 325 mg every other day from neurology for restless legs with a
  ferritin of 15 (standard care for a diagnosis the source leaves untreated); she stops
  it in August 2023 for constipation, which leaves her ferritin low for the GI workup.
- **A5.** The 2023-06-06 portal exchange that refers her to gynecology.
- **A6.** Clobetasol propionate 0.05% ointment for biopsy-proven lichen sclerosus, nightly
  for 4 weeks then twice weekly maintenance; gynecology follows yearly (2024-10 visit,
  patient's account: controlled on twice-weekly use).
- **A7.** The 2025-03-11 practice visit that refers her to GI; ferrous sulfate 325 mg
  daily from GI for iron deficiency anemia, continued to today.
- **A8.** Colonoscopy with resection 2025-05-08 and surveillance colonoscopy 2026-06-11,
  findings exactly as in the timeline. The "colorectal neoplasia" is this advanced
  adenoma; no colorectal cancer.
- **A9.** Anemia recovery levels: Hgb 11.6 g/dL and ferritin 32 ng/mL in July 2025;
  Hgb 12.4 g/dL and ferritin 58 ng/mL in May 2026.
- **A10.** DLBCL course: hematology/oncology referral, staging PET-CT, pre-treatment
  hepatitis B, HIV and echocardiogram results as given, methotrexate and folic acid
  stopped, six cycles of R-CHOP from 2025-11-13 to 2026-02-26 run by oncology.
- **A11.** End-of-treatment PET-CT 2026-04-09: complete metabolic response (Deauville 2);
  oncology surveillance every 3 months (visit 2026-07-16, no evidence of disease).
- **A12.** Rheumatology follow-up 2026-04-28: prednisone 5 mg alone, no DMARD for now.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may
  carry the descriptors its kind always carries, but nothing that adds a territory, a
  severity grade or a symptom, or changes the diagnosis or plan.
- Records from a prior practice may mention routine screening done there, with normal results.
- Visits for hypothyroidism and her outside conditions every 4 to 8 months; yearly
  physicals in late summer or September; TSH yearly and normal on levothyroxine 100 mcg;
  yearly CBC and CMP, with Hgb levels about 13 (2021, 2022), about 12.3 (2023) and about
  12.0 g/dL with an MCV about 80 (2024, low-normal or just below), none commented on; lipids normal; BP normal; weight 68 kg until
  2025, then as in R12.
- Prevention: yearly screening mammograms normal (none during chemotherapy); Pap with HPV
  co-test 2024 negative; hepatitis C and HIV screening at joining, nonreactive;
  influenza each fall; COVID vaccines; Shingrix series 2022; PCV20 in 2023 for
  immunosuppression; PHQ-2 screens negative. Not a lung-screening candidate (10
  pack-years).
- Refills (levothyroxine from the practice; RA, iron and skin drugs from their
  prescribers), pharmacy fills, portal messages about results and scheduling, one
  missed visit, records requests.
- Booked visit after 2026-08-27: annual physical 2026-09-15.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.
