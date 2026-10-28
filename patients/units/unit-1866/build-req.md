# Build requirements: unit-1866 (Isabelle Beaulieu, MRN00001866)

Source: synthetic_hospital patient 1866, in `source.md` beside this file. This file and
the source are the only authority for clinical facts. Every diagnosis, medication,
result, procedure and specialist finding in the chart must trace to one of:

- **S** a fact in source.md, carried as listed below;
- **R** a resolution below, where the source contradicts itself or stops short;
- **A** an addition below, listed one by one;
- **P** the primary-care layer described at the end.

Nothing else. No diagnosis, medication, test result, procedure, family member or
life event that is not on these lists. Texture (wording, scheduling, refills, portal
traffic, who signed what) is free, as long as it carries no new clinical fact.

Re-voice everything. The source notes are exam vignettes: never copy a sentence, and
never carry a source clinician's name, its "Active Problem List" dumps or its
"(diagnosed ...)" stamps.

## Who she is (S)

Isabelle Beaulieu, born 1997-03-24, female, White, not Hispanic or Latino, English,
Private PPO. Elementary school teacher. Never smoker. Alcohol socially, 1 to 2 drinks a
month. No known drug allergies. Surgical history: tonsillectomy at age 8. Family
history: mother with recurrent urinary tract infections; father with hypertension.
Sexually active with one partner, uses condoms inconsistently; no new partners. No
regular medications.

Joined the practice on 2024-09-10 (A1), a new-patient visit. She brings recurrent
urinary tract infections (a chronic problem on the source profile from the start) and
no medications.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2024-09-10 | practice | new-patient visit (A1): history as above; reports UTIs on and off for several years, the most recent in April 2024 treated at an urgent care with a short antibiotic course she cannot name (A2). **Recurrent urinary tract infections** on the problem list. Normal exam; height 165 cm, weight 61 kg; screening labs (P) |
| 2024-11-18 | practice | same-day visit for dysuria and frequency (A3): urinalysis leukocyte esterase 1+, nitrite positive, WBC 20 /hpf, RBC 0 /hpf, bacteria many; urine culture sent. **Acute cystitis**, **nitrofurantoin 100 mg twice daily for 5 days**. Culture (resulted 2024-11-21): E. coli greater than 100,000 CFU/mL, susceptible to nitrofurantoin. Symptoms gone by 2024-11-25, cystitis resolved |
| 2025-02-03 | practice | encounter 0: dysuria, frequency and suprapubic pain for 2 days; her third episode in the past year, the last one 3 months ago (the November visit); LMP 2 weeks ago, normal; one partner, condoms inconsistently; no new partners, vaginal discharge or genital lesions. Vitals normal; suprapubic tenderness. Urinalysis with pyuria and bacteriuria, carried as leukocyte esterase 2+, nitrite positive, WBC 35 /hpf, RBC 2 /hpf, bacteria many (R2); the November culture grew E. coli. **Acute cystitis**, **nitrofurantoin 100 mg twice daily for 5 days** (R1). Culture (resulted 2025-02-05): E. coli 50,000 to 100,000 CFU/mL, susceptible to nitrofurantoin (A4). Symptoms gone by 2025-02-10, cystitis resolved |
| 2025-09-08 | practice | annual physical (P): well; no UTI since February; Tdap (P) |
| 2025-10-13 | outside | encounter 1, outside gynecology office (she booked it herself, R4): age 28; a week of thin white discharge with a strong fishy odor, worse after intercourse; no itching or burning; no medications. No vulvar erythema or edema; homogeneous white discharge coating the vaginal walls on speculum exam. Vaginal pH 5.0, whiff test positive, clue cells on wet mount. **Bacterial vaginosis**, **metronidazole 500 mg twice daily for 7 days** (A5). The office sends no letter at the time (R4) |
| 2025-10-14 | practice | outside fill history shows metronidazole filled 2025-10-13, prescriber an outside gynecologist (R4) |
| 2025-10-21 | practice | nurse visit for vaccines (P): she says the gynecologist treated a vaginal infection last week; metronidazole recorded as an outside short course; records requested from the gynecology office. The problem list is not updated (R4) |
| 2026-06-01 | outside | encounter 2, same gynecology office: age 29; a week of grayish-white discharge with fishy odor, worse after intercourse; no itching, burning or dysuria; LMP 2 weeks ago. No vulvar erythema or edema; thin gray discharge in the vault; no cervical motion tenderness. Clue cells (epithelial cells with stippled borders) on microscopy; vaginal pH 5.5. Recurrent bacterial vaginosis; **metronidazole 500 mg twice daily for 7 days** (A5) |
| 2026-06-12 | practice | records received from the gynecology office: both visit notes (2025-10-13, sent late, and 2026-06-01) (R4) |
| 2026-06-26 | practice | last practice visit (follow-up, P): reviews the gynecology notes; **bacterial vaginosis added to the problem list** (onset 2025-10-13); metronidazole course from June recorded as outside, completed; no UTI since February 2025; cervical cytology collected (P). Vitals normal |
| 2026-07-02 | practice | cervical cytology result: negative for intraepithelial lesion or malignancy (P) |
| 2026-09-14 | practice | booked annual physical (after today) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Encounter 0's plan stops mid-sentence. Resolution: acute cystitis is diagnosed
  and treated with nitrofurantoin 100 mg twice daily for 5 days, and a culture is sent.
  Encounter 1 says she takes no medications in the autumn, so no continuing drug follows.
- **R2.** The source gives only "pyuria and bacteriuria". Resolution: the urinalysis is
  carried with the values in the timeline; urine blood is negative, the rest normal.
- **R3.** Ages and years: the source's "27-year-old" at encounter 1 and "28-year-old" at
  encounter 2 follow the birth date: 28 on 2025-10-13 and 29 on 2026-06-01. Encounter 1's
  "seen in January for dysuria" becomes the February 2025 practice visit. The source's
  "Acute cystitis (diagnosed ...)" on the gynecology problem lists may stay on those
  letters as copied history; on the practice list the cystitis is resolved each time.
- **R4.** Records have gaps (seed). The October 2025 gynecology visit sends no letter; the
  practice learns of it from a fill history on 2025-10-14 and from the patient on
  2025-10-21, and the note arrives only in June 2026 with the June visit's note. The
  practice problem list lags: bacterial vaginosis is added only at the 2026-06-26 visit.
  She saw gynecology on her own; there is no practice referral.
- **R5.** The source's comorbidity "Sexual activity" is social history, not a problem.
- **Left as the source has it:** three UTIs in a year with no prophylaxis, no imaging and
  no urology referral; no sexually transmitted infection testing at either gynecology
  visit; no partner discussion. Nobody comments on these.

## Additions (A): physician to confirm

- **A1.** 2024-09-10 new-patient visit (encounter 0 lists recurrent UTI as an established
  problem and refers to earlier episodes, so she joined before it; seed: about 2 years
  with the practice).
- **A2.** The April 2024 urgent-care UTI by her report: the first of the "three in the past
  year". No urgent-care record reaches the chart.
- **A3.** The November 2024 practice visit, the episode "3 months ago" whose culture grew
  E. coli, with the values in the timeline and nitrofurantoin.
- **A4.** Encounter 0's culture result, as in the timeline.
- **A5.** Metronidazole 500 mg twice daily for 7 days at each gynecology visit (the
  standard treatment; the source names bacterial vaginosis twice and gives no plan).

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it.
- Records from a prior practice may mention routine screening done there with normal
  results (a normal Pap in 2023) and childhood and adolescent vaccines.
- At joining: CBC, comprehensive metabolic panel, lipid panel, HIV and hepatitis C
  screening once, all normal. Weight stays about 60 to 62 kg; blood pressure normal.
- Influenza and updated COVID vaccines each fall (2024 and 2025); Tdap in 2025.
  Depression screens (PHQ-2) negative at the physical and new-patient visit.
- Cervical cytology every 3 years (age 21 to 29): 2023 at the prior practice, 2026 here.
- Portal messages about results and scheduling, phone calls, a cancelled visit,
  reminders, records requests.
- Booked visit after 2026-08-27: annual physical on 2026-09-14.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00001866 on 2026-06-26. Practice contacts after that day do not
happen; outside care after it still does, and its records are filed as received without
changing the practice's lists. A visit is booked after today.
