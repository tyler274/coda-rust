# Build requirements: unit-2671 (Nathan Ryan, MRN00002671)

Source: synthetic_hospital patient 2671, in `source.md` beside this file. This file and
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
never carry a source clinician's name, its "Active Problem List" dump, its
"(diagnosed 2020-01-15)" stamp, or any quiz phrasing ("you note").

## Who he is (S)

Nathan Ryan, born 1990-02-22, male, White, not Hispanic or Latino, English, Blue Cross
Blue Shield PPO, lives in Springfield (01109). Marital status recorded as single; no
partner or children appear anywhere in the chart. Electrical contractor (runs job sites,
climbs ladders). Never smoker. Alcohol: social, 1 to 2 beers on weekends. No drugs.
Allergies: no known drug allergies.

Surgical history: tonsillectomy at age 8; knee arthroscopy at age 28 (side and
indication not recorded).

Family history: father with hypertension (at 62); mother with hypothyroidism (at 60);
no known family history of sudden cardiac death, cardiomyopathy or premature coronary
disease (R1).

Joined the practice in June 2023 (A1), healthy, with no problems and no medications.
Height 180 cm, weight about 84 kg throughout.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2023-05-30 | practice | phone call to join the practice (P) |
| 2023-06-13 | practice | new-patient preventive visit (A1): history as above; normal exam (heart sounds described as regular, no murmur comment needed); BP normal; CBC, CMP, lipid panel, A1c normal; hepatitis C and HIV screens negative; Tdap given; PHQ-2 0 (P) |
| 2023-07 | outside | prior practice records received: routine care, no problems (P) |
| 2023-10, 2024-10, 2025-10 | practice | nurse visits or physical: influenza and COVID vaccines each fall (P) |
| 2024-06-25 | practice | annual preventive visit, well, normal exam (P) |
| 2025-06-09 / 06-10 | practice | portal message, then office visit (A2): exertional shortness of breath and chest pain since about January 2025, slowly worse (on ladders, carrying wire, stairs); no syncope, no palpitations; exam: harsh systolic murmur at the left sternal border, louder with Valsalva; CBC, BMP, TSH normal (P). **Cardiology referral placed** |
| 2025-07-18 | outside | cardiology consult: exertional dyspnea and chest pain worsening over six months; denies syncope, palpitations and family sudden cardiac death; BP 130/85, HR 78; harsh crescendo-decrescendo systolic murmur at the left sternal border, louder with Valsalva, softer on squatting. Transthoracic echocardiogram: asymmetric septal hypertrophy, systolic anterior motion of the mitral leaflet, resting LVOT gradient 55 mmHg (S); septum 19 mm, posterior wall 10 mm, LVEF 65% (A3). **Hypertrophic obstructive cardiomyopathy diagnosed.** **Metoprolol succinate 50 mg daily started** (A4); Holter ordered (A5); advice on hydration and avoiding very heavy exertion; first-degree relatives advised to have screening echocardiograms (A4). Letter and echo report received 2025-07-23 |
| 2025-07-29 | outside | 48-hour Holter (A5): sinus rhythm throughout, no nonsustained ventricular tachycardia, no atrial fibrillation; report received 2025-08-05 |
| 2025-08-12 | practice | follow-up after cardiology (P): letter and Holter reviewed; tolerating metoprolol; breathing somewhat better on it (R2); HR lower on beta-blocker |
| 2025-10-14 | practice | annual preventive visit (P): lipid panel, CMP normal; flu and COVID vaccines |
| 2026-03-17 | practice | video visit (P, R2): breathlessness and chest pain on exertion creeping back since about January 2026; no syncope; cardiology follow-up already booked for July; metoprolol unchanged; hydration and pacing advice |
| 2026-05-19 | practice | follow-up visit cancelled by patient (work), not rebooked (P) |
| 2026-07-22 | outside | cardiology follow-up, one year after diagnosis: six months of progressive exertional dyspnea and chest pain, worse lying flat; harsh systolic murmur louder with Valsalva. Echocardiogram: asymmetric septal hypertrophy and systolic anterior motion of the mitral leaflet (S); resting LVOT gradient 60 mmHg, septum 19 mm, LVEF 65% (A3). **Metoprolol succinate 50 mg stopped, 100 mg daily started** (A6); **referral to a hypertrophic cardiomyopathy center** to discuss a cardiac myosin inhibitor or septal reduction (A6), still open today. Letter and echo report received 2026-07-27 |
| 2026-08-04 | practice | portal exchange: PCP has reviewed the letter, list updated; patient waiting to hear from the HCM center (P) |
| 2026-10-13 | practice | booked: annual preventive visit (P) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Family history: the profile and encounter 0 say no known family history of
  sudden cardiac death, and encounter 1 says a family member died suddenly young.
  Resolution: the profile is carried everywhere (no known sudden cardiac death,
  cardiomyopathy or premature coronary disease). The encounter-1 statement is not
  carried; no relative is added.
- **R2.** Encounter 1 gives a "6-month history" of progressive symptoms as if new,
  though encounter 0 already described six months of worsening symptoms. Resolution:
  symptoms eased somewhat on metoprolol after July 2025, then progressed again from
  about January 2026; the March 2026 video visit records the return, and the July 2026
  cardiology letter describes six months of progression.
- **R3.** Encounter 1's problem-list dump and "(diagnosed 2020-01-15)" stamp: not
  carried. The diagnosis dates to the cardiology consult of 2025-07-18.
- **Left as the source has it:** neither cardiology visit reports an ECG, so the chart
  carries no ECG. The orthopnea in July 2026 gets no separate workup and no heart
  failure label. The BP of 130/85 at cardiology is not acted on. No one comments on
  any of this.

## Additions (A): physician to confirm

- **A1.** June 2023 new-patient visit (the seed gives about three years with the
  practice; the source starts at cardiology).
- **A2.** The June 2025 practice visit that heard the murmur and made the cardiology
  referral, with its exam as the source's.
- **A3.** Echo measurements any HCM echo report states: septum 19 mm, posterior wall
  10 mm, LVEF 65% on both studies; resting LVOT gradient 60 mmHg on the 2026 study
  (the source gives no gradient for it).
- **A4.** The source diagnoses symptomatic obstructive HCM and names no treatment.
  Standard first-line care: metoprolol succinate 50 mg daily, started and managed by
  cardiology; hydration and exertion advice; screening echocardiograms suggested for
  first-degree relatives (advice only, no relative's result).
- **A5.** One 48-hour Holter for sudden-death risk assessment: sinus rhythm, no
  nonsustained VT, no atrial fibrillation.
- **A6.** Next steps for symptoms progressing on a beta-blocker: metoprolol succinate
  increased to 100 mg daily in July 2026 and a referral to an HCM center for advanced
  therapy discussion. No visit there before today.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Records from a prior practice may mention routine care there, with normal results.
- Yearly preventive visits (age 33 to 36): vitals, depression screens (PHQ-2/PHQ-9
  normal), lipid panel and metabolic panel now and then; hepatitis C and HIV screening
  once (2023), negative. Tdap 2023. Influenza and COVID vaccines each fall.
- After the diagnosis, follow-up of HCM and the beta-blocker at the practice, with
  vitals; refills of metoprolol (cardiology prescribes; the practice may send 90-day
  refills).
- Portal messages about results, refills and scheduling; the cancelled May 2026 visit.
- Booked visit after 2026-08-27: annual preventive visit 2026-10-13.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.


## Last seen

The practice last saw MRN00002671 on 2026-03-17. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. Nothing is booked after today.
