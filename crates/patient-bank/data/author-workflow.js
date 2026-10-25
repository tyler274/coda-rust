export const meta = {
  name: 'author-deep-patients',
  description: 'Author deep longitudinal primary-care charts per unit: story first, ledger in parts, check until ACCEPTED, preview and reread',
  phases: [
    { title: 'Author', detail: 'story.md, then ledger parts, check until ACCEPTED, preview and reread' },
  ],
}

// args: { repo: '<absolute path of the coda-rust checkout>', units: ['unit-0001', ...] }
const REPO = args.repo
const DRAFTS = REPO + '/target/patient-bank/drafts'
const RUN = `cd ${REPO} && cargo run -q -p patient-bank --`

const RULES = `Ground rules:
- Work only inside ${REPO}. Run every command from there: ${RUN} brief|check|preview <unit>
- Write only inside ${DRAFTS}/<unit>/. Never edit code, the guide, the catalog or seeds; never run git.
- Read only the authoring guide, the vocabulary file, and your unit's draft folder. Do not open other repositories, other units' drafts, or any eval folder.
- All people, places and records are fictional; refer to clinicians and staff by role only, and never name real practices, hospitals, labs, pharmacies, stores or EHR/portal products (drug, vaccine and assay product names are fine).`

const AUTHOR_SCHEMA = {
  type: 'object',
  properties: {
    unit: { type: 'string' },
    accepted: { type: 'boolean', description: 'true only if the last check printed ACCEPTED' },
    patients: { type: 'array', items: { type: 'object', properties: {
      mrn: { type: 'string' }, documents: { type: 'integer' }, observations: { type: 'integer' } },
      required: ['mrn', 'documents', 'observations'] } },
    parts: { type: 'integer' },
    open_warnings: { type: 'array', items: { type: 'string' }, description: 'check warnings you judged false positives, with why' },
  },
  required: ['unit', 'accepted', 'patients', 'parts', 'open_warnings'],
}

const authorPrompt = (u) => `You are authoring the chart for authoring unit ${u} of a primary-care patient bank: one fictional adult patient (or a family) over many years of care, as dense and textured as a real EHR.

Start by running:
  ${RUN} brief ${u}
Then read the authoring guide it names, end to end, and the vocabulary file. The guide's "How you work" section is your process: story.md first (with a contact calendar counted against the targets), then the ledger in chronological part files of two to three years each, then check until it prints ACCEPTED, then preview and read chart.txt start to end as a physician would, fixing what you find and checking again.

What matters most:
- Chronology: every record knows only what was known on its date.
- Density with substance: reach the document and observation targets through the contacts, short notes, vitals and monitoring this person would really have. No filler, no near-duplicate notes.
- Many voices: physicians, NPs, nurses, MAs, front desk, pharmacists, specialists, the patient's own portal messages.
- Use your clinical world knowledge: guideline-concordant but human care, real drug doses, plausible lab trajectories, realistic gaps and imperfections.

${RULES}

Return the structured result. Report accepted=true only if your final check printed ACCEPTED.`

const units = args.units
log(`${units.length} units`)

const results = await pipeline(
  units,
  (u) => agent(authorPrompt(u), { label: `author:${u}`, phase: 'Author', schema: AUTHOR_SCHEMA }),
)

// compact summary; full author results stay in the run journal
const summary = results.map((r, i) => r
  ? { unit: units[i], accepted: r.accepted, patients: r.patients, open_warnings: r.open_warnings.length }
  : { unit: units[i], failed: true })
log(`${summary.filter(s => s.accepted).length}/${units.length} units accepted`)
return summary
