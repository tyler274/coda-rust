export const meta = {
  name: 'author-patients',
  description: 'Author one primary-care chart per unit: story, ledger parts, check until ACCEPTED, preview and reread',
  phases: [
    { title: 'Author', detail: 'one agent per unit, following the authoring guide' },
  ],
}

// args: { repo: '<absolute path of the coda-rust checkout>', units: ['unit-0001', ...] }
const REPO = args.repo
const DRAFTS = REPO + '/target/patient-bank/drafts'
const RUN = `cd ${REPO} && cargo run -q -p patient-bank --`

const SCHEMA = {
  type: 'object',
  properties: {
    unit: { type: 'string' },
    accepted: { type: 'boolean', description: 'true only if the last check printed ACCEPTED' },
    patients: { type: 'array', items: { type: 'object', properties: {
      mrn: { type: 'string' }, documents: { type: 'integer' }, observations: { type: 'integer' } },
      required: ['mrn', 'documents', 'observations'] } },
  },
  required: ['unit', 'accepted', 'patients'],
}

const prompt = (u) => `You are writing the chart for unit ${u} of a primary-care patient bank: one fictional adult patient (or a family) over years of care.

Run ${RUN} brief ${u}, then read the authoring guide it names end to end, and the vocabulary file. Follow the guide's "How you work": story.md, then the ledger part files, then check until it prints ACCEPTED, then preview and read chart.txt start to end as the physician inheriting this patient, fixing what you find and checking again.

Ground rules:
- Run commands only from ${REPO}. Write only inside ${DRAFTS}/${u}/.
- Read only the guide, the vocabulary file and your own draft folder. Never edit code, the guide, the catalog or the seeds, and never run git.

Return the structured result. Report accepted=true only if your final check printed ACCEPTED.`

const results = await pipeline(args.units, (u) =>
  agent(prompt(u), { label: `author:${u}`, phase: 'Author', schema: SCHEMA }))
const summary = results.map((r, i) => r || { unit: args.units[i], failed: true })
log(`${summary.filter((s) => s.accepted).length}/${args.units.length} units accepted`)
return summary
