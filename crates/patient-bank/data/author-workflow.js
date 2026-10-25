export const meta = {
  name: 'author-deep-patients',
  description: 'Author deep longitudinal primary-care charts per unit, then an independent physician-eye review and a fix pass',
  phases: [
    { title: 'Author', detail: 'story.md, then ledger parts, check until ACCEPTED, preview and reread' },
    { title: 'Review', detail: 'an independent reader of chart.txt against the guide' },
    { title: 'Fix', detail: 'fix every major and most minor findings, check until ACCEPTED, preview' },
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

const REVIEW_SCHEMA = {
  type: 'object',
  properties: {
    findings: { type: 'array', items: { type: 'object', properties: {
      severity: { type: 'string', enum: ['major', 'minor'] },
      where: { type: 'string', description: 'date and document or record, as chart.txt shows it' },
      problem: { type: 'string' },
      fix: { type: 'string' } },
      required: ['severity', 'where', 'problem', 'fix'] } },
    verdict: { type: 'string', description: 'two sentences: would a physician believe this chart, and what stands out' },
  },
  required: ['findings', 'verdict'],
}

const authorPrompt = (u) => `You are authoring the chart for authoring unit ${u} of a primary-care patient bank: one fictional adult patient (or a family) over many years of care, as dense and textured as a real EHR.

Start by running:
  ${RUN} brief ${u}
Then read the authoring guide it names, end to end, and the vocabulary file. The guide's "How you work" section is your process: story.md first (with a contact calendar counted against the targets), then the ledger in chronological part files of two to three years each, then check until it prints ACCEPTED, then preview and read chart.txt start to end against the guide's "Before you finish", fixing what you find and checking again.

What matters most:
- Chronology: every record knows only what was known on its date.
- The chart ends mid-life at today: nothing after it is written except pending results and booked visits.
- Density with substance: reach the document and observation targets through the contacts, short notes, vitals and monitoring this person would really have. No filler, no near-duplicate notes.
- Many voices, each document with its author's role: physicians, NPs, nurses, MAs, front desk, pharmacists, specialists, the patient's own messages.
- Use your clinical world knowledge: human care by busy clinicians, real drug doses, plausible lab trajectories, realistic gaps, friction and imperfections.

${RULES}

Return the structured result. Report accepted=true only if your final check printed ACCEPTED.`

const reviewPrompt = (u) => `You are an experienced primary-care physician reviewing a chart someone else wrote for authoring unit ${u} of a fictional patient bank. You did not write it; find what a physician inheriting this patient would not believe.

Run ${RUN} brief ${u} for the seed and the guide's path. Read the authoring guide end to end (its "Before you finish" section is your checklist), then ${DRAFTS}/${u}/story.md, then ${DRAFTS}/${u}/chart.txt start to end. chart.txt ends with the visits already booked; nothing after today is written.

Report each problem with where it is and the fix. Major: a chronology leak, a note and data that disagree, clinically wrong care, a device-impossible number, teaching voice, a generic or repeated story element, a chart with nothing realistic going on at the bank time, rewrite damage such as a role used as a name. Minor: voice, length, texture. Do not report taste. Do not edit any file.

${RULES}`

const fixPrompt = (u, review) => `You are fixing the chart for authoring unit ${u} of a fictional primary-care patient bank after an independent physician review.

Run ${RUN} brief ${u}, read the authoring guide it names, then ${DRAFTS}/${u}/story.md and the part files. The review:

${JSON.stringify(review, null, 2)}

Fix every major finding and every minor one you agree with, by editing the part files (and story.md if the plan changes). Keep everything else as it is. Then check until it prints ACCEPTED, preview, and reread the passages you changed in chart.txt.

${RULES}

Return the structured result. Report accepted=true only if your final check printed ACCEPTED, and list in open_warnings any finding you chose not to fix, with why.`

const units = args.units
log(`${units.length} units`)

const results = await pipeline(
  units,
  (u) => agent(authorPrompt(u), { label: `author:${u}`, phase: 'Author', schema: AUTHOR_SCHEMA }),
  (authored, u) => authored && authored.accepted
    ? agent(reviewPrompt(u), { label: `review:${u}`, phase: 'Review', schema: REVIEW_SCHEMA })
      .then((review) => ({ authored, review }))
    : { authored },
  (r, u) => r.review
    ? agent(fixPrompt(u, r.review), { label: `fix:${u}`, phase: 'Fix', schema: AUTHOR_SCHEMA })
      .then((fixed) => ({ ...r, fixed }))
    : r,
)

// compact summary; full results stay in the run journal
const summary = results.map((r, i) => {
  if (!r || !r.authored) return { unit: units[i], failed: true }
  const findings = r.review ? r.review.findings : []
  return {
    unit: units[i],
    accepted: Boolean(r.fixed ? r.fixed.accepted : r.authored.accepted),
    reviewed: Boolean(r.review),
    major: findings.filter((f) => f.severity === 'major').length,
    minor: findings.filter((f) => f.severity === 'minor').length,
    unfixed: r.fixed ? r.fixed.open_warnings : null,
    verdict: r.review ? r.review.verdict : null,
  }
})
log(`${summary.filter((s) => s.accepted && s.reviewed).length}/${units.length} units reviewed and accepted`)
return summary
