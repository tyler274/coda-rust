---
name: patient-memory-maintenance/authoring-guide
description: Detailed operating-index organization, reconciliation guidance, and worked clinical examples.
---

# Operating index organization

## One-Liner

- Write a concise patient "one-liner" in the format: “[First Name] [Last Name] ([DOB]) is a [age]-year-old [woman/man/person] with [key medical problems].” Use age as of the current clinical date and demographic information supported by the record.
- Select problems from the reconciled “List of medical problems” that best characterize the patient’s current clinical situation. Include relevant complications, major interventions, and disease severity or trajectory when supported.
- Order problems by current clinical importance and relevance to the physician’s specialty. If the specialty is unknown, prioritize general clinical relevance.
- Use familiar, unambiguous medical abbreviations. Preserve meaningful relationships between conditions, complications, and procedures.
- Distinguish historical events from active conditions and last-documented treatment from confirmed current use. Include material uncertainty when omitting it would misrepresent the patient.
- Keep the one-liner selective; retain the complete problem inventory and supporting detail in the problem sections.
- Preventive care belongs in the index, but the heading Healthcare maintenance is not a diagnosis to add to the one-liner.

Example (Cardiology):
`67-year-old man with past medical history of CAD c/b anterior STEMI s/p PCI LAD (5/2024), ischemic cardiomyopathy/HFmrEF (LVEF 45%), paroxysmal AFib (on Eliquis), HTN, HLD, pre-diabetes, obesity, GERD, osteoarthritis.`


## List of Medical Problems

- Use the chart’s problem list as a starting point and reconcile it with relevant notes, reports, procedures, and other source evidence. Include clinically important conditions or unresolved symptoms documented elsewhere in the chart.
- Consolidate entries only when evidence establishes that they describe the same condition or episode. Preserve meaningful distinctions, complications, and diagnostic uncertainty.
- Clarify vague entries through targeted chart review. For example, investigate “Presence of coronary angioplasty implant and graft” to establish the documented procedure, location, and date; do not infer these details from the label alone.
- Use concise clinical problem statements that preserve supported subtype, severity, complications, and major related interventions. Include procedure dates at the precision available in the source.
- Distinguish active conditions, historical or resolved conditions, and suspected diagnoses. Missing recent documentation does not establish resolution.
- Order problems by current clinical importance and relevance to the physician’s specialty, while retaining significant problems outside that specialty.
- Keep a Healthcare maintenance section after the clinical problems for relevant preventive and monitoring care, including material gaps in the available evidence. Its detailed guidance is in `references/preventive-care.md` within this skill. It remains part of patient understanding even when no preventive action is currently due.
- For example, if the following are all in a patient's problem list:
    - Long term (current) use of antithrombotics/antiplatelets
    - Atherosclerotic heart disease of native coronary artery without angina pectoris
    - Presence of coronary angioplasty implant and graft
    - ST elevation (STEMI) myocardial infarction involving left anterior descending coronary artery
    - Chest pain, unspecified
    - Attention-deficit hyperactivity disorder, predominantly inattentative type
    - Family history of ischemic heart disease and other diseases of the circulatory system
    - Generalized anxiety disorder
    - Unspecified asthma, uncomplicated
- If source review establishes that the coronary labels and chest pain belong to the same MI episode, a consolidated inventory could be:
    - Coronary artery disease complicated by ST elevation myocardial infarction involving the left anterior descending coronary artery, status post percutaneous coronary intervention (MM/YYYY).
    - Attention-deficit hyperactivity disorder
    - Asthma
    - Generalized anxiety disorder

Retain the antiplatelet plan and relevant family history within the corresponding problem or cross-cutting context; consolidation must not discard their clinical meaning.

### Findings outside the problem list

Use relevant source reports and histories to challenge an incomplete problem inventory. For example,
prior imaging may document atherosclerotic calcification, aortic dilation, or another incidental
finding that the problem list omits. Read the report when available, preserve the finding's actual
description and date, and judge whether it belongs to an existing problem, warrants a distinct
problem, or needs incidental-finding follow-through. A mention alone does not establish a clinical
diagnosis, severity, or action. An unread or inaccessible report does not establish a negative finding.

Choose investigation breadth for the current task and source availability; this is not an instruction
to repeat all imaging reads on each update. Preserve clinically important detail across specialties.
The `references/specialty-review.md` resource supplies deeper prompts when those conditions apply.


## Data by Medical Problem

Organize each clinically significant problem around its timeline, current status, and outstanding issues. Use the following sections as a useful default; combine brief historical problems or omit empty subsections when that preserves a clearer patient understanding.

Preserve the information needed for the next clinical judgment without reproducing every encounter
or duplicating the same facts across problems. Physician specialty can change emphasis, not justify
discarding important context. Healthcare maintenance uses current status and outstanding issues;
its relevant completed dates need not be repeated in a second timeline.

### Timeline
- Summarize major events in chronological order using dated bullet points.
- Include the initial presentation or diagnosis, significant changes in severity, major investigations or procedures, treatment changes, complications, and meaningful responses to treatment.
- Use the date precision supported by the source. Omit repetitive encounters that do not materially change understanding.

### Current Status
- Describe the latest supported clinical state, including symptoms, severity, functional impact, and relevant diagnostic findings or trends. Include assessment dates and units where relevant.
- Document relevant medications and known doses as dated orientation, each dated and cited to its source (for example, "per the 2026-02 medication list"), distinguishing prescribed treatment, last-documented use, and confirmed current use. Preserve reasons for important treatment changes or discontinuations. The index is never the current medication list: work that depends on a current medication or latest value re-reads the chart.
- Summarize the documented care plan and physician direction. Clearly distinguish recommendations, agreed plans, and completed care.
- Include relevant family history, patient preferences, and treatment barriers where they affect interpretation or care.

### Outstanding Issues
- List unresolved clinical questions, pending investigations, treatment decisions, and follow-through requiring clarification.
- State what remains unresolved and why it matters. Distinguish documented noncompletion from completion that is simply unconfirmed in the available record.
- Preserve the rationale for existing Coda follow-ups without duplicating their scheduling or lifecycle mechanics.
- If no outstanding issue is identified, say so when useful (for example, “No outstanding issues identified in the reviewed evidence”), or omit an empty subsection. Do not invent work to populate the structure.

Throughout all three sections:
- Ground material claims in exact source evidence. Identify patient- or physician-reported information as such, and distinguish it from independently documented findings.
- Preserve meaningful conflicts, evidence gaps, and uncertainty. Missing documentation does not establish absence, resolution, or failure to follow through.
- Keep unexplained symptoms separate unless evidence supports linking them to an established condition.
- Place information affecting several problems in brief cross-cutting context rather than repeating it under each problem.
- Keep the content selective and avoid duplication: the timeline explains how the problem evolved, current status describes the latest supported state, and outstanding issues identify what remains unresolved.
