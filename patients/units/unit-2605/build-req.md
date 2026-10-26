# unit-2605 build requirements

Every clinical fact in this chart traces to **S** (the source as carried here), **R**
(a numbered resolution), **A** (a numbered addition) or **P** (the primary-care layer
below), and nothing else. Texture (voice, scheduling, refills, portal traffic, admin,
missed visits) is free and carries no new clinical fact. Re-voice everything: no source
sentence, clinician name, problem-list dump, "(diagnosed ...)" stamp or quiz text.

## Who he is (S)

- Min-jun Burgos, male, born 1996-07-05, MRN00002605. South Hadley, MA 01075. White,
  not Hispanic or Latino; family of Mediterranean ancestry. English. Blue Cross PPO.
  Phone +1-978-555-0190, min-jun.burgos@example.test. Single.
- Software engineer. Never smoker. No alcohol. No recreational drugs. Balanced diet with
  meat, grains and vegetables (2023). Exercised four times a week in 2023; by January
  2026 describes himself as mostly sedentary (R6).
- Allergies: no known drug allergies.
- Surgical history: none.
- Family history (R4): father with ankylosing spondylitis, HLA-B27 positive, and back
  surgery in his 40s; father and paternal uncle each once told they had small red blood
  cells, never treated or transfused; father with psoriasis; paternal uncle with Crohn's
  disease.
- **Joins the practice 2019-09-10 (A1)**, age 23, from a prior primary care office.
  Brings no chronic problems and no medications. Family history of the father's
  ankylosing spondylitis and the uncle's Crohn's disease is recorded at joining; the
  small-red-cell history is first taken 2023-01-23 and the father's psoriasis
  2024-01-29, the father's back surgery 2026-01-27.

## Timeline

| Date | Where | What happens |
|---|---|---|
| 2019-09-10 | Practice, establish care (A1) | New patient. Healthy, no problems, no medications, NKDA. Tdap, HIV and hepatitis C screening (P). |
| about 2022-12-07 | Outside, occupational health (S, R2) | Workplace screening CBC shows anemia with microcytosis. **Oral iron (ferrous sulfate 325 mg daily) started empirically** by the occupational health clinic about 2022-12-12. Reaches the chart through the patient's message and a faxed occupational health letter in January 2023. |
| 2023-01-09 | Practice, portal (R2) | Patient reports the workplace result and the iron. PCP orders repeat CBC, iron studies and hemoglobin electrophoresis. |
| 2023-01-12 | Practice lab (R2) | CBC again shows microcytic anemia with no improvement on iron (Hgb level 12.4 g/dL, MCV 65 fL, RBC 5.96, A3). Serum iron and ferritin normal (iron 104, ferritin 92, A3). Hemoglobin electrophoresis (resulted 2023-01-19): **hemoglobin A2 elevated at 5.4 %, hemoglobin F mildly increased at 1.8 %** (A3). |
| 2023-01-23 (Mon) | Practice, office visit — source enc 0 | Evaluation of anemia found on workplace screening. Feels well: no fatigue, weakness, lightheadedness, palpitations or dyspnea; exercises four times a week, no endurance limits; regular bowel movements, no change in stool color or consistency. Six weeks of iron with no change in how he feels. Father and paternal uncle told of small red cells, never treated. Diet balanced; no alcohol, tobacco or drugs. Vitals normal. Well-appearing, no pallor, jaundice or scleral icterus; heart and lungs normal; no hepatosplenomegaly. **Beta thalassemia minor diagnosed. Ferrous sulfate stopped.** |
| about 2023-07 | — | Low back pain begins (onset six months before 2024-01-29). Episodes of mild anterior uveitis during 2023, seen at an outside eye clinic (R3); no records in the chart. |
| 2024-01-17 | Practice, portal (R5) | Patient reports the back pain; PCP orders CRP, ESR, HLA-B27 and sacroiliac joint radiographs before the visit. |
| 2024-01-22 | Practice lab (R5) | CRP elevated (24 mg/L), ESR elevated (32 mm/h) (A3). HLA-B27 positive (resulted 2024-01-25). |
| 2024-01-23 | Outside radiology, report received (R5) | Sacroiliac joint radiographs: bilateral sacroiliitis. |
| 2024-01-29 (Mon) | Practice, office visit — source enc 1 | Six months of low back pain, worse in the morning, better with exercise; several courses of over-the-counter ibuprofen (200 mg tablets, as needed) with minimal relief (historical). Occasional mild anterior uveitis episodes over the past year. Family history of psoriasis (father, R4); no personal skin problems. Never smoker, no alcohol. Decreased lumbar flexion, tenderness over the sacroiliac joints. Reviews CRP, ESR, HLA-B27 and the radiographs. **Ankylosing spondylitis diagnosed. Recurrent anterior uveitis entered on the problem list** from his report (R3). Beta thalassemia minor carried. **Rheumatology referral placed.** |
| 2024-03-07 (Thu) | Outside rheumatology, consult letter — source enc 2 (R1) | Initial consult for AS. Same six-month inflammatory back pain history and occasional mild anterior uveitis; denies recent infections or GI symptoms. Father with psoriasis. SI joint tenderness, decreased lumbar range of motion, abnormal Schober's test. Reviews the January pelvis/SI radiographs: bilateral sacroiliitis (R5). **Sulfasalazine started, building to 1000 mg orally twice daily.** Adalimumab 40 mg subcutaneously every 2 weeks planned once TB and hepatitis B screening is back (A2). Referral closed. |
| 2024-03-11 | Practice lab, at rheumatology's request (A2) | QuantiFERON-TB Gold negative; hepatitis B surface antigen nonreactive, core antibody nonreactive, surface antibody reactive. Baseline CBC with differential and CMP (P). |
| 2024-03-19 | Specialty pharmacy / rheumatology notice (A2) | Prior authorization approved. **Adalimumab 40 mg subcutaneously every 2 weeks started**, first dose with rheumatology nurse teaching 2024-03-21. |
| 2024-04-08 (Mon) | Outside rheumatology, follow-up letter — source enc 3 | Age 27, AS and beta thalassemia minor, on adalimumab and sulfasalazine. Pain still worse in the morning, better with exercise; occasional mild anterior uveitis over the past year; denies infections and GI symptoms. Mother's psoriasis not carried (R4). Vitals normal. Decreased lumbar flexion, SI joint tenderness. CRP and ESR remain elevated (no values in the letter). Pelvis radiographs as before (R5). HLA-B27 "pending" not carried (R5). Continue both drugs. |
| 2026-01-27 (Tue) | Outside rheumatology, follow-up letter — source enc 4 | Age 29, AS diagnosed January 2024, beta thalassemia minor. Chronic dull, deep low back and hip pain, progressively worse over two years, worse in the morning or after rest, better with activity; morning stiffness now about an hour, recently started. No trauma or strain. Occasional eye redness and discomfort over the past few months that resolved on its own. Denies fevers, chills, weight loss. Works as a software engineer; sedentary (R6). Father had back surgery in his 40s. Decreased lumbar flexion, reduced chest expansion, SI joint tenderness, normal gait, positive Schober's test. Inflammatory markers elevated (no values); rheumatoid factor negative, ANA negative. **Spine radiographs: changes suggestive of sacroiliac joint sclerosis and fusion.** Adalimumab and sulfasalazine continue unchanged. |

## Resolutions (R)

1. **Drugs and diagnoses listed from the start.** The profile and every note list AS,
   recurrent anterior uveitis, adalimumab and sulfasalazine from 2023-01-23. They are
   not known then: in 2023 he has no back pain and the anemia visit lists only the
   iron. AS and recurrent anterior uveitis enter the chart 2024-01-29. Source enc 2
   ("returns ... on adalimumab and sulfasalazine") is the first rheumatology consult
   and starts sulfasalazine; adalimumab starts 2024-03-19 after screening (A2). The
   "Thalassemia minor" and "Beta Thalassemia Minor" entries are one condition. The
   comorbidity "Family history of hereditary bleeding disorder" has no support anywhere
   in the source and is not carried.
2. **Anemia work-up sequence.** The workplace CBC (occupational health, early December
   2022) and the "local clinic" that started iron are the same occupational health
   clinic. Oral iron is carried as ferrous sulfate 325 mg daily, started about
   2022-12-12 so that 2023-01-23 is six weeks on it. The PCP's repeat CBC, iron studies
   and electrophoresis are drawn 2023-01-12 and resulted before the visit, so the
   visit reviews them all.
3. **Uveitis episodes.** The source reports episodes of mild anterior uveitis over the
   past year at the 2024 visits and eye redness at the 2026 visit, with no encounter
   that diagnoses them. The chart carries them as he reports them: the 2023 episodes
   were diagnosed at an outside eye clinic (no records obtained, treatment not
   recorded), entered as recurrent anterior uveitis (recurrent acute iridocyclitis,
   ICD-10 H20.029) on 2024-01-29 with onset 2023.
4. **Family history.** The profile (mother with beta thalassemia trait, father with AS,
   paternal uncle with Crohn's) and the notes (father and paternal uncle with small red
   cells; psoriasis unspecified, in the father, then in the mother; "no family history
   of autoimmune disease"; father's "bad back" with surgery in his 40s) cannot all be
   true. Carried: father with AS (HLA-B27 positive), back surgery in his 40s, psoriasis,
   and small red cells; paternal uncle with small red cells and Crohn's disease. Not
   carried: the mother's thalassemia trait, the mother's psoriasis, and the 2026 denial
   of autoimmune family history (the 2026 letter records the father's AS and back
   surgery).
5. **HLA-B27 and imaging.** HLA-B27 is positive on 2024-01-22 (enc 1); the 2024-04-08
   "pending" is not carried. Enc 1's "radiographs of the sacroiliac joints" and enc 2/3's
   "plain x-rays of the pelvis" are the one study of 2024-01-23, which rheumatology
   reviews; no new pelvis films are taken in 2024. The PCP orders the labs and films
   from the 2024-01-17 message so they are back for the 2024-01-29 visit. The 2026
   spine radiographs are a new rheumatology study.
6. **Activity.** "Exercises four times per week" (2023) and "sedentary lifestyle" (2026)
   are both true at their dates; the chart does not explain the change.
7. **Plans.** The source gives no plans at any encounter. Enc 0's plan is stopping iron
   and explaining carrier status; enc 1's is the rheumatology referral; enc 2 starts the
   drugs; enc 3 and enc 4 continue them unchanged with rheumatology follow-up.

**Left as the source has it:** no change in AS therapy after the January 2026 letter
despite progressive symptoms and elevated markers; no ophthalmology referral or eye
records for the uveitis episodes, including the 2025-2026 eye redness; no NSAID
prescribed after ibuprofen failed; no CBC at the practice before 2023. Nobody comments
on these.

## Additions (A)

1. **Joining date 2019-09-10**, seven years with the practice; the 2023 anemia visit
   reads as an established patient.
2. **Pre-biologic screening and adalimumab start.** Any rheumatologist screens before a
   TNF inhibitor: QuantiFERON-TB Gold negative; HBsAg nonreactive, hepatitis B core
   antibody (total) nonreactive, hepatitis B surface antibody reactive (childhood
   series), drawn 2024-03-11. Prior authorization and first dose 2024-03-19/21. Gives
   the source's "on adalimumab" at enc 3 a start.
3. **Levels for the source's qualitative results.** Thalassemia trait indices from
   2023-01-12 onward: hemoglobin about 12.4 g/dL, MCV about 65 fL, RBC about 5.96
   million/µL (stable for the rest of the chart, including on sulfasalazine). Iron 104
   µg/dL and ferritin 92 ng/mL on 2023-01-12. Hemoglobin A2 5.4 %, hemoglobin F 1.8 %.
   CRP 24 mg/L and ESR 32 mm/h on 2024-01-22. Rheumatology's markers stay qualitative.

## Primary-care layer (P)

- Establish care 2019-09-10; yearly preventive visits (2020-09, 2021-09, 2022-10 after a
  cancelled September slot, 2023-06, 2024-06, 2025-06, 2026-06) with normal exams apart
  from what the timeline gives; PHQ-2 at each, negative. Height 178 cm, weight about 76
  kg at joining, about 78 kg by 2026. Blood pressure normal throughout.
- Labs: lipid panel and CMP at joining, normal (total cholesterol about 163 mg/dL, LDL
  about 92, HDL about 52, triglycerides about 95); HIV and hepatitis C screening once
  (2019), nonreactive. On sulfasalazine and adalimumab: CBC with differential and CMP
  every three months at the practice's lab on rheumatology's standing request, normal
  apart from the stable thalassemia indices. Result notes to the patient.
- Prior primary care records received 2019: childhood immunizations complete (hepatitis
  B, MMR, varicella, HPV series), nothing else of note.
- Immunizations: Tdap 2019-09-10; influenza every fall 2019-2025; COVID-19 primary
  series spring 2021, booster December 2021, bivalent fall 2022, updated formula fall
  2023, 2024 and 2025 (pharmacy, recorded from the patient or the state registry). On a
  TNF inhibitor: PCV20 2024-06 and Shingrix two doses 2024-06 and 2024-08. No live
  vaccines after 2024-03.
- Rheumatology follow-up letters between source encounters, about every six months
  (2024-10, 2025-06): back pain persists, morning-worse, no eye or GI symptoms reported
  at those visits, adalimumab and sulfasalazine unchanged, no new findings and no values.
- Specialty pharmacy and prior-authorization renewal notices for adalimumab; refill
  traffic routed to rheumatology; portal messages about results, scheduling and forms;
  one cancelled visit and one cancelled nurse visit.
- A nurse visit for the 2026 influenza vaccine is booked after today. Nothing else open.
- No acute illnesses, injuries, new diagnoses or life events beyond the timeline.


## Last seen

The practice last saw MRN00002605 on 2024-10-08. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. Nothing is booked after today.
