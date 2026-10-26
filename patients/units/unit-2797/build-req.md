# Build requirements: unit-2797 (Kristen Marchetti, MRN00001103)

Source: synthetic_hospital patient 2797, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or "A CT angiogram of the abdomen is shown."

## Who she is (S)

Born 1983-09-20, female, White, commercial PPO, high school mathematics teacher, never
smoker, one or two glasses of wine a month. NKDA. No surgery. Family history: mother
hypertension diagnosed at 45, a maternal aunt with an ischemic stroke at 52, father with
hyperlipidemia.

Joined the practice in 2018 (first visit 2018-04-17, a new-patient visit), when her
hypertension was found and **hydrochlorothiazide 25 mg daily** started (S: "diagnosed 2
years ago, poorly controlled on hydrochlorothiazide", R1).

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2018-04-17 | practice | new-patient visit; BP elevated on repeat; hypertension; HCTZ started (A1) |
| 2020-01-21 | practice | BP still high on HCTZ (166/102 both arms), HR 78; muscle cramps; no edema, no bruits. Labs: Na 147, K 2.9, bicarbonate 32 (R2); plasma renin activity and aldosterone both elevated; 24-hour urine metanephrines normal. Secondary hyperaldosteronism raises renovascular hypertension: **potassium chloride 20 mEq daily** (A2), renal artery Doppler ordered, cardiology referral |
| 2020-03-11 | outside | cardiology consult: six months of episodic right-sided pulsatile headaches, sometimes with a whooshing sound in her ears; BP 155/95 both arms; high-pitched bruit over the right neck and the right upper abdomen; creatinine and potassium normal; renal artery Doppler with turbulence and a narrowed right renal artery. **Fibromuscular dysplasia** diagnosed (R3) |
| 2020-07-18 | outside | emergency department (a Saturday): two episodes of transient vision loss in the right eye, each gone within minutes, with the ongoing headaches and pulsatile tinnitus; BP 162/98 right arm, 154/92 left; high-pitched epigastric bruit; no carotid bruit heard (R4); normal neurological exam; normal renal function; renal artery duplex with beading and irregularity consistent with FMD stenosis. Discharged to outpatient follow-up (A3) |
| 2020-07-28 | practice | post-ED visit (P); symptoms settled; BP still above goal; plan to continue HCTZ and follow cardiology |
| 2021-03-16 | practice | routine follow-up of severe hypertension with FMD and the earlier vision loss, headaches and tinnitus; no headache, vision change or chest pain now; BP 180/120, normal exam; creatinine and electrolytes normal. **CT angiogram of the abdomen** ordered; result per A4. **Losartan 25 mg daily** started (A5) |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The January 2020 chief complaint says "new-onset hypertension", while the ED note
  says it was diagnosed two years earlier and she is already on HCTZ. Resolution: hypertension
  since 2018; January 2020 is when it is recognized as resistant, with a metabolic clue.
- **R2.** The January 2020 note reports an arterial pH 7.48 and PaCO2 48, which an office
  visit would not draw. Resolution: the ABG is dropped and the metabolic alkalosis is
  carried by the serum bicarbonate of 32.
- **R3.** The source lists FMD as "diagnosed 2020-01-15", before any imaging. Resolution:
  FMD is suspected in January and diagnosed at cardiology in March, from the Doppler.
- **R4.** Cardiology heard a right neck bruit and the ED heard none. Both stay as each
  examiner wrote them. Nobody reconciles them, and neither the ED nor cardiology images
  the neck vessels after the eye episodes, because the source shows no such imaging. The
  physician who inherits the chart can find that gap.

## Additions (A): physician to confirm

- **A1.** The 2018 new-patient visit and the start of HCTZ.
- **A2.** Potassium chloride after K 2.9. That makes the normal March potassium coherent.
- **A3.** The ED discharge to outpatient follow-up.
- **A4.** CT angiogram (April 2021): beaded, irregular right renal artery with focal
  narrowing, consistent with multifocal FMD; left renal artery normal; kidneys normal size.
- **A5.** Losartan added, then 50 mg from mid-2021; basic metabolic panel 1 to 2 weeks after
  each change. As a woman of childbearing age on an ARB she is counselled about pregnancy;
  she reports reliable contraception (no method named). After mid-2021 her BP settles to
  the 130s to 140s over 80s to 90s. Potassium chloride stops once K stays normal on losartan.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention
  (kidney size, no hydronephrosis, aorta caliber, heart sounds). An abnormal finding appears
  only as the source or this file gives it. A report may carry the descriptors its kind
  always carries (how a size was measured, an aneurysm's shape, a benign nodule's size and
  lobe within its Lung-RADS category, a procedure's access site), but nothing that adds a
  territory, a severity grade or a symptom, or changes the diagnosis or plan.
- Records from a prior practice may mention routine screening done there, with normal results.
- Blood pressure visits about every 3 to 6 months while uncontrolled, then every 6 months;
  yearly physicals; basic metabolic panel with each BP visit or medication change; lipids once
  or twice.
- Age-appropriate prevention: cervical screening (co-test 2019 and 2024, normal), influenza
  each fall, COVID vaccines, Tdap if due; HIV and hepatitis C screening once; depression
  screens at physicals.
- Home BP log readings reported through the portal; refills; pharmacy traffic; school forms
  are fine (she is a teacher); a missed visit.
- Booked visit after 2026-08-27: a routine blood pressure follow-up.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.
