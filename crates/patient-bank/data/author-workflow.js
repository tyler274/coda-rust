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

Run ${RUN} brief ${u}, then read the authoring guide it names end to end, the vocabulary file and the taken list it names. Follow the guide's "How you work": story.md, then the ledger part files, then check until it prints ACCEPTED, then preview and read chart.txt start to end as the physician inheriting this patient, fixing what you find and checking again.

Ground rules:
- Run commands only from ${REPO}. Write only inside ${DRAFTS}/${u}/.
- Read only the guide, the vocabulary file and your own draft folder. Never edit code, the guide, the catalog or the seeds, and never run git.

Return the structured result. Report accepted=true only if your final check printed ACCEPTED.`

// args.revise: { '<unit>': 'what a reader found' }. The unit already has an
// accepted chart; the agent revises its part files instead of starting over.
const revisePrompt = (u, found) => `You are revising the accepted chart for unit ${u} of a primary-care patient bank: one fictional adult patient (or a family) over years of care.

Run ${RUN} brief ${u}, then read the authoring guide it names end to end (its "A life, not a plot" section is new), the vocabulary file, the taken list it names, ${DRAFTS}/${u}/story.md and the part files. A physician reading the chart blind found:

${found}

Revise the part files (and story.md if the plan changes) so the chart meets the guide: flatten the plotting, cut the props, let mistakes go unexplained, and make the values vary the way real ones do. Replace any name, phrase or lab value from the taken list with this patient's own. Keep what already reads like a real record, and keep everything else as it is. Then check until it prints ACCEPTED, preview, and reread the passages you changed in chart.txt.

Ground rules:
- Run commands only from ${REPO}. Write only inside ${DRAFTS}/${u}/.
- Read only the guide, the vocabulary file and your own draft folder. Never edit code, the guide, the catalog or the seeds, and never run git.

Return the structured result. Report accepted=true only if your final check printed ACCEPTED.`

const results = await pipeline(args.units, (u) => args.revise
  ? agent(revisePrompt(u, args.revise[u]), { label: `revise:${u}`, phase: 'Author', schema: SCHEMA })
  : agent(prompt(u), { label: `author:${u}`, phase: 'Author', schema: SCHEMA }))
const summary = results.map((r, i) => r || { unit: args.units[i], failed: true })
log(`${summary.filter((s) => s.accepted).length}/${args.units.length} units accepted`)
return summary
