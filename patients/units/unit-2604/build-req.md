# Build requirements: unit-2604 (Wyatt Galvin, MRN00002604)

Source: synthetic_hospital patient 2604, in `source.md` beside this file. This file and
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
"(diagnosed 2020-01-15)" stamps, or its "an image of the rash is provided" text.

## Who he is (S)

Wyatt Galvin, born 2000-12-22, male, White, not Hispanic or Latino, English,
university student health insurance (PPO). Graduate student. Never smoker, no
alcohol, no drug use. No surgical history. Allergies: seasonal pollens (tree,
grass, ragweed) and dust mites (environmental; rhinitis). No drug allergy is
recorded either way.

Family history (R1): mother with seasonal allergies and eczema; father with asthma;
a sister with atopic dermatitis.

Joined the practice on 2023-09-12 (A1, a new-patient visit at the start of his
graduate program, about three years before today). He brought, all from childhood
or adolescence: seasonal allergic rhinitis, mild persistent asthma and atopic
dermatitis; on albuterol HFA 90 mcg, 2 puffs inhaled as needed for wheeze or
shortness of breath; cetirizine 10 mg by mouth daily; triamcinolone acetonide 0.1%
cream, thin layer twice daily as needed for eczema flares. Eosinophilic esophagitis
is not known at joining; it starts at its encounter.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2023-09-12 | practice | new-patient visit (A1): history and drugs as above; normal exam; HIV and hepatitis C screening negative (P); Tdap (P); prior records requested |
| 2023-09-27 | outside (records) | prior primary-care records received (P): childhood asthma, eczema and seasonal allergies; childhood vaccines and HPV series complete |
| 2023-10-10 | practice | nurse visit, influenza and COVID vaccines (P) |
| 2024-09-17 | practice | annual physical (P): stable; lipid panel normal; PHQ-9 low |
| 2024-10-08 | practice | nurse visit, influenza and COVID vaccines (P) |
| 2025-04-07 | practice | visit for an eczema flare over the winter (P, referral visit): itchy rash in the same spots, worse in winter and after hot showers; partial relief from over-the-counter creams alongside triamcinolone (R2); elbow flexures involved; **CBC and comprehensive metabolic panel drawn, normal** (S, R3); referred to dermatology |
| 2025-04-28 | outside | dermatology consult (age 24): chronic itchy rash worse in winter and after hot showers, partial relief with over-the-counter creams, recurs in the same places; no recent travel, new drugs or sick contacts. Exam: multiple erythematous patches with excoriations over the flexural elbows; no pustules, vesicles or drainage; rest of skin and exam normal; clinical photos taken (R4). CBC and metabolic panel unremarkable (the practice's 2025-04-07 draw, R3). Atopic dermatitis flare; continue triamcinolone 0.1% for flares; gentle skin care (A2). Letter received 2025-05-05 |
| 2025-09-16 | practice | annual physical (P, referral visit): about five months of intermittent trouble swallowing solids with food sticking in the chest, starting in spring 2025; tried over-the-counter antacids and an over-the-counter proton pump inhibitor (R5) for presumed heartburn without improvement; no weight loss, vomiting or bowel change; referred to gastroenterology |
| 2025-10-07 | practice | nurse visit, influenza and COVID vaccines (P) |
| 2025-10-27 | outside | gastroenterology consult (age 24): six months of intermittent solid-food dysphagia with food sticking in the chest; antacids and a PPI did not help; no weight loss, vomiting or bowel change; exam unremarkable. **Upper endoscopy with biopsies the same day** (R6); visible esophageal mucosa without stated abnormality. Pathology: squamous epithelium with more than 15 eosinophils per high-power field; no viral inclusions or fungal organisms; gastric and duodenal biopsies normal. **Eosinophilic esophagitis diagnosed.** Letter and pathology received 2025-11-04 |
| 2026-01-20 | practice | follow-up visit cancelled by the patient (P) |
| 2026-04-22 | outside | emergency department (age 25): progressive dysphagia to solids with food sticking in the chest, symptomatic for about a year (R7), occasional epigastric pain, two episodes of vomiting after meals in the past month. Exam unremarkable; no anemia (normal hemoglobin) or weight loss. Discharged home with urgent gastroenterology follow-up (R6). Record received 2026-04-24 |
| 2026-04-29 | practice | post-ED visit (P): symptoms as above, no vomiting since the ED; exam normal; GI appointment confirmed |
| 2026-05-06 | outside | gastroenterology repeat upper endoscopy (R6): multiple concentric rings and linear furrows in the esophagus; biopsy of the esophageal mucosa more than 20 eosinophils per high-power field. **Budesonide oral suspension 2 mg twice daily started by GI** (A3); repeat endoscopy planned after the course. Letter and pathology received 2026-05-13; the practice adds budesonide to its list |
| 2026-07-07 | practice | last seen; follow-up visit (P): asthma and allergies stable on current drugs; on budesonide, swallowing better and no vomiting (A4) |
| 2026-07-22 | outside | gastroenterology follow-up (A4): fewer food-sticking episodes, no vomiting; continue budesonide 2 mg twice daily until repeat endoscopy in September 2026. Letter received 2026-07-29, filed |
| 2026-09-22 | practice | booked follow-up visit |

## Resolutions of source contradictions (R): physician to confirm

- **R1.** Family history differs by note: the dermatology note gives the mother
  asthma, the GI note gives the mother atopic dermatitis, the ED calls it
  non-contributory. Resolution: the profile carries (mother seasonal allergies and
  eczema, father asthma, sister atopic dermatitis); the GI wording agrees with it.
  The dermatology letter's "mother with asthma" is not carried; the ED record may
  say non-contributory, as ED notes do.
- **R2.** Triamcinolone is a home drug at the dermatology visit, yet he reports
  "over-the-counter creams" with partial relief. Resolution: he used OTC moisturizers
  and creams alongside his prescribed triamcinolone; no OTC drug is charted.
- **R3.** The dermatology note reports a CBC and metabolic panel as unremarkable, an
  unusual draw for an eczema consult. Resolution: the practice drew a CBC and CMP at
  the 2025-04-07 referral visit, all normal, and the dermatology letter cites them.
- **R4.** "An image of the rash is provided": the dermatology letter says clinical
  photographs were taken for the record. No image is in the chart.
- **R5.** The PPI is unnamed and over the counter. Resolution: over-the-counter
  omeprazole 20 mg, taken by the patient for a few weeks in summer 2025 and stopped
  for lack of effect, recorded as patient-reported history at the 2025-09-16 visit
  and not on the active list. The antacids are not charted as a drug.
- **R6.** The source puts endoscopy and pathology inside an outpatient consult and an
  ED visit. Resolution: the 2025-10-27 GI consult and its endoscopy happen the same
  day (pathology follows in the letter). The ED does not perform endoscopy: it
  discharges him to urgent GI follow-up, and the rings, furrows and more than 20
  eosinophils per high-power field come from the GI repeat endoscopy on 2026-05-06.
- **R7.** The ED note repeats "6-month history" of dysphagia, but symptoms began in
  spring 2025. Resolution: the ED record says about a year of intermittent
  dysphagia, worse over the past six months.
- **R8.** The source profile lists eosinophilic esophagitis among diagnoses from the
  start. Resolution: it is recorded only when the GI pathology letter arrives
  (2025-11-04), onset spring 2025.
- **Left as the source has it:** no treatment for eosinophilic esophagitis is
  started between the October 2025 diagnosis and the April 2026 ED visit, and no
  note remarks on it. Mild persistent asthma stays on as-needed albuterol alone with
  no controller inhaler, and no note remarks on it.

## Additions (A): physician to confirm

- **A1.** 2023-09-12 new-patient visit (the first source encounter is a specialist
  consult for a long-standing condition, so he was already a patient).
- **A2.** The dermatology plan, which the source leaves unstated: continue
  triamcinolone 0.1% cream for flares; fragrance-free emollient after short,
  lukewarm showers.
- **A3.** Standard therapy for eosinophilic esophagitis after the source ends,
  chosen by GI after the repeat endoscopy: budesonide oral suspension 2 mg twice
  daily (0.2 mg/mL, 10 mL), swallowed, nothing by mouth for 30 minutes afterward;
  repeat endoscopy planned after the course.
- **A4.** Response on budesonide: by July 2026 fewer food-sticking episodes and no
  vomiting (practice visit 2026-07-07 by report; GI follow-up 2026-07-22). The
  repeat endoscopy is in September 2026, after today; nothing about it is in the
  chart beyond the plan.

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may
  carry the descriptors its kind always carries (biopsy sites, scope reached the second
  portion of the duodenum), but nothing that adds a territory, a severity grade or a
  symptom, or changes the diagnosis or plan.
- Records from a prior practice may mention routine childhood care and vaccines, with
  normal results.
- A physical each year (2024, 2025) and a follow-up visit about the allergic conditions;
  vitals, normal weight (height about 180 cm, weight about 74 kg, steady); PHQ-9 low;
  one lipid panel normal; normal blood pressure.
- Prevention for a man in his twenties: HIV and hepatitis C screening once (negative),
  Tdap at joining, influenza each fall, updated COVID each fall.
- Refills (albuterol, triamcinolone; cetirizine is over the counter), pharmacy traffic,
  portal messages about results, refills and scheduling, the cancelled January 2026
  visit, reminders.
- Booked visit after 2026-08-27: a routine follow-up on 2026-09-22.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002604 on 2026-07-07. Practice contacts after that day are
only messages, refills and records; outside care after it still happens, and its
records are filed as received without changing the practice's lists. A visit is booked
after today.
