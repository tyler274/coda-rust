# Build requirements: unit-2874 (Theresa Mayhew, MRN00002874)

Source: synthetic_hospital patient 2874, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its quiz text.

## Who she is (S)

Born 1959-12-16, female, White, not Hispanic or Latino, English, Medicare
(disability), retired factory worker. Lives in the seed town. Current cigarette
smoker, 42 pack-years, contemplation stage: knows smoking has hurt her but is
ambivalent about quitting. No alcohol.

Allergies: sulfa drugs (rash); latex (contact dermatitis).

Surgical history: tubal ligation at age 32 (about 1992); loop electrosurgical
excision procedure (LEEP) for cervical dysplasia at age 58 (2018).

Family history: mother cervical cancer, died at 67; father lung cancer, died at 71;
sister breast cancer at 55.

Joined the practice 2021-06-08 (new-patient visit, A1), age 61, bringing:
COPD; high-risk HPV infection (found about 2018, with the cervical dysplasia that led
to the LEEP); nicotine dependence (cigarettes). Medications on joining:
fluticasone/salmeterol 250/50 mcg one inhalation twice daily, tiotropium 18 mcg
inhaled once daily, albuterol 90 mcg inhaler as needed for shortness of breath.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2021-06-08 | practice | new-patient visit (A1): history as above; inhalers continued; smoking counselled, she is not ready to quit; low-dose CT for lung screening ordered (A2); screening labs (P) |
| 2021-06-29 | outside | low-dose CT chest (A2): Lung-RADS 1 |
| 2022-03 | practice | spirometry (A3): post-bronchodilator FEV1/FVC 0.58, FEV1 61% predicted; moderate obstruction consistent with COPD; inhalers unchanged |
| 2022-06, 2023-06 | outside | yearly low-dose CT chest (A2): Lung-RADS 1 |
| 2023-10-03 | practice | about 3 months of intermittent vaginal bleeding and foul-smelling discharge; urgent gynecology referral (P). Pelvic exam left to gynecology, seen within days |
| 2023-10-11 | outside | gynecology visit: 3 months of intermittent vaginal bleeding and foul-smelling discharge; no real pain, a sense of pelvic discomfort; irregular ulcerated lesion on the posterior vaginal wall; biopsy taken. Reaches the practice as a short gynecology note (sparse) |
| 2023-10-16 | outside | surgical pathology, vaginal biopsy: atypical squamous cells invading the subepithelial connective tissue; **invasive squamous cell carcinoma of the vagina diagnosed**. Report faxed to the practice |
| 2023-10 to 11 | outside | gynecologic oncology (A4): MRI pelvis and PET/CT show the tumor confined to the posterior vaginal wall with no enlarged pelvic or inguinal nodes and no distant disease; FIGO stage I. Known to the practice from one brief gynecologic oncology letter and the patient's account |
| 2023-11-13 to 2024-01-19 | outside | radiation oncology (A5): external-beam pelvic radiation 45 Gy in 25 fractions, then vaginal brachytherapy; completed. A one-page treatment summary reaches the practice |
| 2024 to 2026 | outside | gynecologic oncology surveillance (A6): exams every 3 months through early 2026, then every 6 months; no evidence of disease each time. Practice hears of it from the patient; one letter (2026-07) arrives after last seen and is filed |
| 2025-09-29 | outside | ED visit: COPD exacerbation with increased shortness of breath; age 65; history notes the vaginal cancer diagnosed in October 2023 (R4); SpO2 91% on room air on arrival, 95% after nebulized albuterol/ipratropium (A7); **prednisone 40 mg daily for 5 days** (A7); discharged home. Before leaving she says smoking has done her harm but it is probably too late to quit. Reaches the practice as a short ED note |
| 2025-10-06 | practice | post-ED follow-up (P): breathing back to her usual; prednisone finished; inhaler technique reviewed; cessation offered again (varenicline or patch discussed), she declines |
| 2026-05-12 | practice | last seen: Medicare wellness and COPD visit; stable; labs drawn that day; colonoscopy referral placed, still open (P) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The gynecology note says she has no family history of gynecologic cancer,
  while the profile and ED note list her mother's cervical cancer. Resolution: the
  mother's cervical cancer is carried everywhere; the "no family history" statement
  is dropped.
- **R2.** The profile lists invasive vaginal carcinoma from the start. Resolution:
  it is not known at joining; it is diagnosed by the 2023-10-16 pathology. Notes before
  October 2023 never mention it, bleeding or discharge.
- **R3.** HPV "diagnosed 5 years ago" (from 2023) and the LEEP at age 58 both fall in
  2018. Resolution: high-risk HPV was found in 2018 with the cervical dysplasia treated
  by LEEP; the practice carries it as a history-level problem. No HPV or cytology
  results are given.
- **R4.** The ED note dates the cancer to "January 2020" (the source's own calendar).
  Resolution: the re-dated timeline holds; the ED note says October 2023.
- **R5.** The source's ED note gives no treatment, vitals or disposition.
  Resolution: A7 supplies the minimum an ED note carries; she is discharged from the ED.
- **Left as the source has it:** she keeps smoking; no cessation drug is ever started
  (she declines each offer). Inhalers stay the same doses throughout, including after
  the exacerbation. Post-LEEP cervical follow-up is with her outside gynecology and no
  results reach the practice.

## Additions (A): physician to confirm

- **A1.** 2021-06-08 new-patient visit (she has five years with the practice and the
  source begins with a specialist visit).
- **A2.** Yearly low-dose CT lung screening (61 to 66, 42 pack-years, current smoker):
  2021-06, 2022-06, 2023-06, then 2024-11 and 2025-11 (the 2023 staging PET/CT covers
  late 2023). Each Lung-RADS 1; the reports may note mild centrilobular emphysema,
  the imaging form of her COPD. No nodules.
- **A3.** One spirometry for the COPD the source asserts without testing:
  post-bronchodilator FEV1/FVC 0.58, FEV1 61% predicted.
- **A4.** Staging after the biopsy: MRI pelvis and PET/CT, tumor confined to the
  vaginal wall, no nodal or distant disease, FIGO stage I.
- **A5.** Definitive radiation for stage I vaginal squamous cell carcinoma: pelvic
  external-beam 45 Gy in 25 fractions plus vaginal brachytherapy, 2023-11-13 to
  2024-01-19, completed.
- **A6.** Gynecologic oncology surveillance, no evidence of disease through 2026.
- **A7.** At the 2025-09-29 ED visit: SpO2 91% on arrival, 95% after nebulized
  albuterol/ipratropium; prednisone 40 mg daily for 5 days; discharged.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may carry
  the descriptors its kind always carries, but nothing that adds a territory, a severity
  grade or a symptom, or changes the diagnosis or plan.
- Records from a prior practice may mention routine screening done there, with normal
  results (a normal screening colonoscopy in 2016, repeat due 2026).
- COPD visits about every 6 months; Medicare annual wellness visits yearly; CBC, CMP and
  lipids about yearly; hepatitis C and HIV screening once; an A1c once; PHQ-2 screens.
- Screening mammograms yearly (sister's breast cancer), normal; one DXA at 65, normal.
- Influenza each fall, COVID vaccines, PCV20, Shingrix series, Tdap, RSV (2024).
- Smoking counselling at most visits; she stays in contemplation.
- A post-ED visit; a referral visit before the gynecology visit; a check-in after
  radiation.
- Refills (inhalers through mail order or the local pharmacy), Medicare Part D and
  inhaler prior-authorization traffic, portal messages about results and scheduling,
  a missed visit.
- Records are sparse: outside care arrives as short notes or the patient's account,
  with few numbers.
- Booked visit after 2026-08-27: a routine COPD follow-up.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002874 on 2026-05-12. Practice contacts after that day do not
happen; outside care after it still does, and its records are filed as received without
changing the practice's lists. A visit is booked after today.
