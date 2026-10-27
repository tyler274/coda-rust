# Build requirements: unit-2162 (Sean McDermott, MRN00002162)

Source: synthetic_hospital patient 2162, in `source.md` beside this file. This file and
the source are the only authority for clinical facts. Every diagnosis, medication,
result, procedure and specialist finding in the chart must trace to one of:

- **S** a fact in source.md, carried as listed below;
- **R** a resolution below, where the source contradicts itself or leaves a gap it needs filled;
- **A** an addition below, listed one by one;
- **P** the primary-care layer described at the end.

Nothing else. No diagnosis, medication, test result, procedure, family member or
life event that is not on these lists. Texture (wording, scheduling, refills, portal
traffic, who signed what) is free, as long as it carries no new clinical fact.

Re-voice everything. The source notes are exam vignettes: never copy a sentence, and
never carry a source clinician's name, its "Active Problem List" dumps, its
"(diagnosed 2020-01-15)" stamps, or its "An ECG is shown" quiz text.

## Who he is (S)

Sean McDermott, born 1978-11-26, male, White, not Hispanic or Latino, English,
Aetna PPO, lives in Williamsburg (01096). Software implementation consultant. Never
smoker. Alcohol: occasional social use, 1 to 2 drinks per month. No drug use stated.
Marital status single (texture; no partner or household member enters the chart).

- **Allergies:** penicillin (hives/urticaria), sulfa drugs (sulfonamide antibiotics; rash).
- **Surgical history:** appendectomy at age 12 (about 1990-1991); right knee
  meniscectomy at age 28 (about 2006-2007). Text history only.
- **Family history:** father with hypertension and type 2 diabetes; mother with
  hypothyroidism; paternal uncle with hyperlipidemia. No one else.
- **Long-standing problems (from the source profile, present before the first
  encounter):** gastroesophageal reflux disease (without esophagitis), seasonal
  allergic rhinitis (pollen), obesity with BMI 32.
- **Home medications throughout:** omeprazole 20 mg by mouth daily; fluticasone
  propionate nasal spray 2 sprays each nostril daily; cetirizine 10 mg by mouth daily
  as needed. Not on anticoagulants.
- **Joining the practice:** he had no current primary-care physician when the first
  crash happened. He establishes care on **2025-06-17** (A1, a new-patient visit) and
  brings the three problems above, the three drugs above, both allergies, and the
  history of the April 2025 traumatic cardiac tamponade (resolved).

Body size: height 180 cm, weight about 103.7 kg (BMI 32) at joining, and about the
same (within 1 to 2 kg) at every later practice visit.

## Timeline (dates fixed; S unless marked)

| Date | Where | What happens |
|---|---|---|
| 2025-04-26 (Sat) | outside, regional hospital ED and trauma service | High-speed motor vehicle collision, restrained driver, airbags deployed. Alert, anxious, diaphoretic. BP 85/50, HR 120, RR 22, SpO2 92% on room air. Jugular venous distention, lungs clear, no external signs of significant bleeding, abdomen soft and non-tender. Chest radiograph: no rib fractures, no pneumothorax. FAST: no intra-abdominal fluid; the pericardial view shows a pericardial effusion with right ventricular diastolic collapse (R1). ECG: sinus tachycardia without ischemic changes. Central venous catheter placed; CVP 18 mmHg. **Cardiac tamponade (traumatic) diagnosed.** Course after the source ends (A2): ultrasound-guided pericardiocentesis in the ED, 90 mL of non-clotting blood withdrawn, BP 116/72 and HR 98 afterward; pericardial drain left in; admitted to the trauma ICU. |
| 2025-04-27 | outside, same admission | A2: inpatient transthoracic echocardiogram: no residual pericardial effusion, normal LV size and function, LVEF 60%, no wall motion abnormality. Drain output minimal; drain removed. |
| 2025-04-29 | outside, same admission | A2: discharged home. Home medications unchanged. Cardiology clinic echo in about 6 weeks. |
| 2025-06-05 | outside, hospital cardiology clinic | A3: outpatient transthoracic echocardiogram: no pericardial effusion, LVEF 60%, normal valves. Letter: tamponade resolved, no further cardiology follow-up needed. |
| 2025-06-12 | practice receives | The ED record, the discharge summary and the 2025-06-05 echo letter arrive by fax after the new-patient records request (P). |
| 2025-06-17 (Tue) | practice | **New-patient visit (A1).** History as above, including the April crash and tamponade (resolved). Obesity (BMI 32), GERD controlled on omeprazole, allergic rhinitis on fluticasone with cetirizine as needed. Baseline labs and one-time hepatitis C and HIV screening (P). |
| 2025-10-07 (Tue) | practice | Nurse visit: influenza and COVID vaccines; FIT kit handed out (P). |
| 2025-10-21 | practice | FIT negative (P). |
| 2026-02-27 (Fri) | practice | **Last seen.** Office follow-up for obesity, reflux and allergy season planning; weight unchanged; no medication changes (P). |
| 2026-04-21 (Tue) | outside, regional hospital ED | Second high-speed motor vehicle collision, restrained driver, extricated from the vehicle. Chest pain and shortness of breath; hypotensive despite IV fluids. BP 84/60, pulse 124. Alert but pale, tachypneic, diaphoretic. Distended neck veins, muffled heart sounds, no murmurs, lungs clear. Cool extremities with delayed capillary refill. Systolic drop of 14 mmHg with inspiration (pulsus paradoxus). Troponin mildly elevated: high-sensitivity troponin I 46 ng/L (R3). Chest x-ray: widened mediastinum. Point-of-care ultrasound: large pericardial effusion with right atrial diastolic collapse; no pneumothorax, no intra-abdominal fluid. Repeat ultrasound: effusion persists, ejection fraction preserved. ECG: sinus tachycardia (R4). Not on anticoagulants. Surgical consultation requested. **Cardiac tamponade diagnosed again** (outside record only; see Records). |
| 2026-04-21 to 2026-04-25 | outside, same hospital, trauma surgery | A4: subxiphoid pericardial window under general anesthesia the same day: 240 mL of blood and clot evacuated from the pericardium, no ongoing bleeding, no cardiac injury seen; pericardial drain placed. Post-operative day 1 CT angiography of the chest: normal thoracic aorta, no aortic injury, no mediastinal hematoma, small post-operative pericardial fluid around the drain (the x-ray widening is attributed to the pericardial collection). Drain removed on day 3. Discharged 2026-04-25, home medications unchanged. |
| 2026-05-12 | outside, trauma surgery clinic | A5: post-operative visit: subxiphoid incision healed; limited echocardiogram with no recurrent pericardial effusion and preserved LV function; may return to work and driving; no further surgical follow-up. |

## Records

The seed calls this chart complete. Every outside contact sends its records:

- 2025 ED record, discharge summary and June echo letter: received 2025-06-12.
- 2026 ED record: received 2026-04-24. Discharge summary with the operative findings and
  the CT angiography: received 2026-04-29. Trauma clinic letter: received 2026-05-14.
- Records after 2026-02-27 are filed as received. The practice's problem list keeps
  cardiac tamponade as a resolved history item from 2025 and is not updated for 2026;
  the medication list is unchanged. A patient portal message after discharge and the
  PCP's reply (glad he is home; call when he wants to come in) are allowed; no visit
  is booked.

## Resolutions of source contradictions (R): physician to confirm

- **R1.** The source's first encounter names cardiac tamponade as its diagnosis but
  gives no image of the pericardium (FAST "negative for intra-abdominal fluid").
  Resolution: the FAST abdominal windows are negative as written, and its pericardial
  window shows the effusion with right ventricular diastolic collapse. That, the
  hypotension, the distended neck veins and the CVP of 18 carry the diagnosis.
- **R2.** The second encounter lists cardiac tamponade as an active problem "diagnosed
  2020-01-15". Resolution: the first tamponade is dated 2025-04-26, it resolved with
  drainage, and between the crashes it is a resolved history item, not an active
  problem. "One year after prior MVC" fits the re-dated timeline.
- **R3.** "Troponins are mildly elevated" gives no number. Resolution: one
  high-sensitivity troponin I of 46 ng/L in the 2026 ED record; no other troponin value.
- **R4.** "An ECG is shown" is quiz text. Resolution: the 2026 ED ECG reads sinus
  tachycardia, with nothing else claimed.
- **R5.** The profile gives no onset for the long-standing problems. Resolution: the
  chart records approximate onsets: allergic rhinitis since young adulthood (about
  1998), reflux and obesity for about a decade (about 2015). Recorded at the
  2025-06-17 new-patient visit as history.
- **Left as the source has it:** no one in the ED comments on the pulsus value beyond
  recording it; the source's profile carries obesity with no treatment beyond advice,
  and the practice adds no weight-loss drug or referral and no note remarks on it.

## Additions (A): physician to confirm

- **A1.** The 2025-06-17 new-patient visit (he needs a primary-care home; the source has
  only ED care).
- **A2.** The 2025 ED drainage and admission (pericardiocentesis, 90 mL, BP 116/72 and HR
  98 after; inpatient echo with LVEF 60% and no residual effusion; discharge 2025-04-29):
  the usual course of the source's tamponade.
- **A3.** The 2025-06-05 cardiology clinic echo (no effusion, LVEF 60%): standard
  follow-up after traumatic tamponade.
- **A4.** The 2026 operation the source's "surgical consultation" leads to (subxiphoid
  pericardial window, 240 mL evacuated, no cardiac injury), the CT angiography of the
  chest that answers the widened mediastinum (normal aorta), and discharge 2026-04-25.
- **A5.** The 2026-05-12 trauma clinic visit (incision healed, no recurrent effusion).

## Primary-care layer (P)

Allowed without listing, with ordinary normal results unless a level above says otherwise:

- Reports and exams state ordinary normal findings for what the source does not mention.
  An abnormal finding appears only as the source or this file gives it. A report may
  carry the descriptors its kind always carries, but nothing that adds a territory, a
  severity grade or a symptom, or changes the diagnosis or plan.
- A prior practice may send a short summary: last physical in 2024 with normal routine
  labs, the same medications and allergies.
- Practice visits as in the timeline: vitals, baseline CMP, CBC, lipid panel, A1c
  (all normal), hepatitis C antibody and HIV screening once (negative), PHQ-2 negative.
- Prevention: average-risk colorectal screening by FIT (negative 2025); influenza and
  COVID vaccines in fall 2025. No shingles or pneumococcal vaccine (under 50, no
  indication).
- Refills of omeprazole, fluticasone and cetirizine, pharmacy traffic, portal messages
  about results and scheduling, the records request, one rescheduled visit.
- No booked visit after 2026-08-27.

No acute illnesses, injuries, new diagnoses or life events beyond the timeline.

## Last seen

The practice last saw MRN00002162 on 2026-02-27. Practice contacts after that day are
messages, refills and filed records only; outside care after it still happens, and its
records are filed as received without changing the practice's lists. Nothing is booked.
