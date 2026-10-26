# Build requirements: unit-2743 (Dawn Cote, MRN00002743)

Source: synthetic_hospital patient 2743, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its unfinished plans.

## Who she is (S)

Born 1986-08-02, female, White, not Hispanic or Latino, English, commercial PPO (Blue
Cross Blue Shield). Data entry specialist. Married; her husband is the only household
member the chart names. Never smoker; occasional social alcohol, 1 to 2 drinks a
month; no drug use. No known drug allergies. No surgical history. Family history:
mother with major depressive disorder; maternal aunt with hypothyroidism and early
menopause at 34; paternal grandmother with rheumatoid arthritis.

Joined the practice in 2021 (A1: new-patient visit 2021-09-14, age 35), moving her care
from a prior primary-care office. She brought one problem, autoimmune hypothyroidism
(Hashimoto thyroiditis, diagnosed at the prior office in 2014, A2), on levothyroxine
88 mcg daily, TSH stable on that dose. Nothing else: no depression, no ovarian
diagnosis, no hand symptoms at joining. Height 160 cm.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2021-09-14 | practice | new-patient visit (A1): history as above; exam normal; PHQ-9 3; baseline labs (P) |
| 2023-03-21 | practice | annual physical (P); mentions she and her husband have been trying to conceive since about October 2022 |
| 2023-05-10 | practice | telehealth visit (P): not yet pregnant after about 7 months; cycles regular every 28 days; **gynecology referral placed** |
| 2023-06-14 | outside | gynecology consult (letter received): trying to conceive 8 months, regular 28-day cycles with no change in flow or length; husband's semen analysis normal; on levothyroxine; vitals normal; pelvic exam unremarkable; FSH 15 IU/L, estradiol 45 pg/mL, AMH 0.4 ng/mL, TSH 2.5 µIU/mL, prolactin 14 ng/mL. **Primary ovarian insufficiency diagnosed** (R1). Plan (R2): reduced ovarian reserve explained in light of age 36 and the aunt's early menopause; reproductive endocrinology referral offered; she wants to talk it over at home first |
| 2023-09-12 | practice | office visit (P): about 5 months of intermittent numbness, tingling and burning in the right hand, worse at night and waking her, eased by shaking the hand; no injury. Exam: positive Tinel sign at the right wrist, positive Phalen test. Night wrist splint advised; **orthopedics referral placed**; TSH (normal) |
| 2023-10-19 | outside | orthopedics consult (letter and nerve conduction report received): 6 months of the same right-hand symptoms; no trauma; thenar muscle atrophy, positive Tinel sign over the median nerve at the wrist, Phalen positive at 60 seconds; nerve conduction study shows prolonged right median nerve latency. **Carpal tunnel syndrome, right, diagnosed.** Plan (R3): carpal tunnel release recommended given the atrophy; she prefers to keep splinting for now and will call to schedule |
| 2024-08-13 | practice | office visit: 6 months of fatigue, headaches and diffuse muscle aches affecting work and home life; down most days, poor sleep, hopeless about the future, occasional thoughts that her family would be better off without her, no plan or intent. Vitals normal; exam unremarkable; TSH stable; CBC and CMP normal. PHQ-9 17 (A3). **Major depressive disorder diagnosed.** Plan (R4): safety plan and crisis line; therapy referral; sertraline offered and declined for now; recheck in 4 weeks |
| 2025-08-19 | practice | office visit (R4): low mood back after a partial spring improvement; PHQ-9 16; passive thoughts again, no plan or intent; **sertraline 50 mg daily started** |
| 2025-10-01 | practice | last practice visit (R4): small change only; PHQ-9 14; **sertraline increased to 100 mg daily**; **psychiatry referral placed**; CBC, CMP, TSH and vitamin B12 drawn, all normal (R5); weight 81.5 kg |
| 2025-12-22 | outside | psychiatry initial evaluation (letter received): six weeks of worse low mood, waking about 4 a.m. and unable to return to sleep, poor concentration; lost interest in hiking and cooking; poor appetite, about 10 lb lost in the last month; guilt about being unproductive; sometimes wonders if her family would be better off without her, denies intent or plan; no mania or psychosis; no prior psychiatric or therapy care (R6); open to therapy but too worn out to find a therapist now; denies recent stressors; asks for something to take the edge off. Alcohol socially, no drugs. Vitals normal; exam unremarkable; mental status: tired, poor eye contact, psychomotor slowing, soft coherent speech, constricted affect, hopeless about the future but willing to try treatment. Reviews the October CBC, TSH, CMP and B12, all normal. Plan (R7) |
| 2026-03-17 | outside | psychiatry follow-up (letter received): three months on sertraline 100 mg without real improvement; poor concentration at work, anhedonia, decreased appetite, 5 kg lost over 3 months; no thoughts of self-harm; marked drop in libido since starting sertraline, straining her marriage. No tobacco, alcohol or drugs at present (R8). Exam normal. Recent CBC, TSH and fasting glucose normal (drawn through psychiatry). Plan (R9): **sertraline increased to 150 mg daily** |
| 2026-06-12 | outside | psychiatry follow-up (letter received): six months of persistent sadness, anhedonia and fatigue; significant weight gain and lower libido, both worse since the dose increase; no improvement in mood. BMI 32; appears lethargic, exam otherwise unremarkable. TSH and CMP normal (drawn through psychiatry). Described as treatment-resistant depression (R10). Plan (R10) |
| 2026-09-15 | practice | booked annual physical (after today) |

The practice last saw her on 2025-10-01. After that day come only portal messages,
refills, the October draw's results and the psychiatry letters, filed as received. The
practice medication list stays at sertraline 100 mg; the 150 mg dose appears only in
the psychiatry letters.

Ages follow the seed birth date: 36 at gynecology, 37 at orthopedics, 38 in August
2024, 39 at all three psychiatry visits. Years stated in the source text follow the
re-dated timeline (depression first diagnosed in August 2024; psychiatry visits in
December 2025 and March 2026).

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source lists primary ovarian insufficiency and hypergonadotropic
  hypogonadism as separate entries and puts both on every list from the start. They are
  one condition, recorded once as primary ovarian insufficiency at the 2023-06-14
  gynecology visit that gives its evidence (high FSH, very low AMH, family history).
  Her cycles stay regular; this is the gynecologist's label for early ovarian failure
  with reduced reserve, and the chart keeps it without debate. No hormone therapy (she
  is still cycling).
- **R2.** The gynecology plan stops mid-sentence. The letter explains the reduced
  reserve and offers a reproductive endocrinology referral; she wants to think about it.
  At the 2023-09-12 practice visit she says she and her husband have decided not to
  pursue fertility treatment for now. No fertility treatment, no pregnancy.
- **R3.** The orthopedics plan is missing, and the source lists no surgery while
  carpal tunnel stays on every later list. Resolution: release is recommended for the
  thenar atrophy, she defers it and keeps the night splint. Later practice notes say the
  splint helps somewhat and she has not scheduled surgery. No surgery happens.
- **R4.** The source lists sertraline 100 mg "started 3 months ago" at every encounter
  from 2023, depression on every list from the start, and also says depression was
  first diagnosed at the internal medicine visit and sertraline started about three
  months before the first psychiatry visit. Resolution: depression starts at
  2024-08-13. She declines medication then and never connects with the therapy
  referral (the source's own year between diagnosis and sertraline). Follow-up PHQ-9s:
  15 on 2024-09-10, 11 on 2024-11-19, 8 at the 2025-03-25 physical. Sertraline 50 mg
  starts 2025-08-19 and goes to 100 mg on 2025-10-01, about twelve weeks before the
  psychiatry evaluation. The 2023 gynecology and orthopedics letters list
  levothyroxine only.
- **R5.** The psychiatry evaluation's "CBC, TSH, CMP and B12 all normal" are the
  practice's 2025-10-01 draw, reviewed by psychiatry. Values from code, normal.
- **R6.** "Never received psychiatric treatment" while on sertraline means no prior
  psychiatrist or therapist; the sertraline came from the practice.
- **R7.** The December psychiatry plan is missing. Resolution: sertraline 100 mg
  continued for a fuller trial (the March visit finds her still on 100 mg); the
  request for something to take the edge off is answered without a sedative, and no
  new drug starts; safety plan reviewed; a list of therapists and a therapy referral
  through psychiatry (she has had one therapy intake by March and continues therapy);
  return in about three months.
- **R8.** Social alcohol (profile and December) against "no alcohol" in March. Both
  stand: in March she says she has not been drinking at all lately.
- **R9.** The March plan is missing and the June visit refers to a dose increase.
  Resolution: psychiatry raises sertraline to 150 mg daily in March after discussing
  the libido effect. The June source line "sertraline 100 mg daily for the past 3
  months" is not carried; the June letter lists 150 mg.
- **R10.** The June plan is missing. Resolution: psychiatry discusses options (staying
  the course, switching to another antidepressant, adding to sertraline); she wants
  to talk it over with her husband first; sertraline 150 mg continues for now; follow-up
  in about three months, after today. No new drug in the chart. "Treatment-resistant"
  is psychiatry's own wording in the June letter; the practice problem list keeps
  major depressive disorder.
- **Weight.** BMI 32 in June 2026 at 160 cm is about 82 kg. Weights: 78.5 kg at
  joining, about 79.5 kg in 2024, 81.5 kg on 2025-10-01; the psychiatry letters carry
  the reported loss (about 10 lb in the month before December, 5 kg over the three
  months to March) and the June BMI. The practice never weighs her after October 2025.
  No weight diagnosis is added.
- **Left as the source has it:** the reproductive endocrinology referral is never
  made. Carpal tunnel release is never scheduled despite atrophy. Depression goes a
  year without treatment after diagnosis. Nobody checks bone density or discusses
  estrogen for the ovarian insufficiency. The chart does not comment on any of these.

## Additions (A): physician to confirm

- **A1.** The 2021-09-14 new-patient visit (the source's first internal medicine
  encounter is a return visit, and the seed gives five years with the practice).
- **A2.** Prior-office records received 2021-09-28: autoimmune thyroiditis diagnosed in
  2014 with positive thyroid peroxidase antibodies (no value), levothyroxine titrated
  to 88 mcg; routine screening there (Pap with HPV co-test 2019, normal; Tdap 2017;
  COVID primary series spring 2021).
- **A3.** PHQ-9 17 at diagnosis, with item 9 positive for passive thoughts and no plan
  or intent; 16 on 2025-08-19 and 14 on 2025-10-01 (and the R4 follow-up scores).

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not
  mention. An abnormal finding appears only as the source or this file gives it.
- Yearly physicals (2022-04-12, 2023-03-21, 2024-03-19, 2025-03-25) with vitals,
  PHQ-2 or PHQ-9, TSH yearly on levothyroxine 88 mcg (normal), CBC and CMP at intervals,
  lipids once or twice (normal).
- Prevention for a woman in her late 30s: Pap with HPV co-test 2024 (normal; next due
  2029); HIV and hepatitis C screening once at joining (negative); influenza each fall;
  COVID boosters as offered; Tdap current from 2017. Mammography is not yet due before
  the chart ends.
- Depression follow-ups after diagnosis at the R4 dates (2024-09-10 office,
  2024-11-19 telehealth), one cancelled or missed visit, and portal check-ins.
- Refills of levothyroxine and sertraline, pharmacy and scheduling traffic, portal
  messages about results, a records request, appointment reminders.
- Booked visit after 2026-08-27: annual physical on 2026-09-15.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002743 on 2025-10-01. Practice contacts after that day do not happen; outside care after it still does, and its records are filed as received without changing the practice's lists. A visit is booked after today.
