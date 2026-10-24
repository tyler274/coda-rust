export const meta = {
  name: 'author-deep-patients',
  description: 'Author deep longitudinal primary-care charts per unit: story first, ledger in parts, self-check, independent review, fix',
  phases: [
    { title: 'Author', detail: 'story.md, then ledger parts, check until ACCEPTED, preview and reread' },
    { title: 'Review', detail: 'independent physician-eye read of the rendered chart against the story' },
    { title: 'Fix', detail: 'apply major review findings, re-check, re-preview' },
  ],
}

const REPO = '/Users/dpatale/Downloads/coda-rust-patient-bank'
const DRAFTS = REPO + '/target/patient-bank/drafts'

const RULES = `Ground rules:
- Work only inside ${REPO}. Run every command from there: cd ${REPO} && python3 -I scripts/patient-bank/unit.py ...
- Write only inside ${DRAFTS}/<unit>/. Never edit scripts, the guide, the catalog or seeds; never run git.
- Read only the authoring guide, the vocabulary file, and your unit's draft folder. Do not open other repositories, other units' drafts, or any eval folder.
- All people, places and records are fictional; never name real clinicians, practices, hospitals, labs, pharmacies, stores or EHR/portal products (drug, vaccine and assay product names are fine).`

const AUTHOR_SCHEMA = {
  type: 'object',
  properties: {
    unit: { type: 'string' },
    accepted: { type: 'boolean', description: 'true only if the last unit.py check printed ACCEPTED' },
    patients: { type: 'array', items: { type: 'object', properties: {
      mrn: { type: 'string' }, documents: { type: 'integer' }, observations: { type: 'integer' } },
      required: ['mrn', 'documents', 'observations'] } },
    parts: { type: 'integer' },
    open_warnings: { type: 'array', items: { type: 'string' }, description: 'future-mention or other warnings you judged false positives, with why' },
  },
  required: ['unit', 'accepted', 'patients', 'parts', 'open_warnings'],
}

const REVIEW_SCHEMA = {
  type: 'object',
  properties: {
    unit: { type: 'string' },
    findings: { type: 'array', items: { type: 'object', properties: {
      severity: { type: 'string', enum: ['major', 'minor'] },
      where: { type: 'string', description: 'date and document/record' },
      problem: { type: 'string' },
      fix: { type: 'string' } }, required: ['severity', 'where', 'problem', 'fix'] } },
    verdict: { type: 'string', description: 'one or two sentences: would a physician believe this is a real chart?' },
  },
  required: ['unit', 'findings', 'verdict'],
}

const FIX_SCHEMA = {
  type: 'object',
  properties: {
    unit: { type: 'string' },
    accepted: { type: 'boolean' },
    fixed: { type: 'array', items: { type: 'string' } },
    not_fixed: { type: 'array', items: { type: 'string' }, description: 'findings left alone, with why' },
  },
  required: ['unit', 'accepted', 'fixed', 'not_fixed'],
}

const authorPrompt = (u) => `You are authoring the chart for authoring unit ${u} of a primary-care patient bank: one fictional adult patient (or a family) over many years of care, as dense and textured as a real EHR.

Start by running:
  cd ${REPO} && python3 -I scripts/patient-bank/unit.py brief ${u}
Then read the authoring guide it names, end to end, and the vocabulary file. The guide's "How you work" section is your process: story.md first (with a contact calendar counted against the targets), then the ledger in chronological part files of two to three years each, then unit.py check until it prints ACCEPTED, then unit.py preview and read chart.txt start to end as a physician would, fixing what you find and checking again.

What matters most:
- Chronology: every record knows only what was known on its date.
- Density with substance: reach the document and observation targets through the contacts, short notes, vitals and monitoring this person would really have. No filler, no near-duplicate notes.
- Many voices: physicians, NPs, nurses, MAs, front desk, pharmacists, specialists, the patient's own portal messages.
- Use your clinical world knowledge: guideline-concordant but human care, real drug doses, plausible lab trajectories, realistic gaps and imperfections.

${RULES}

Return the structured result. Report accepted=true only if your final check printed ACCEPTED.`

const reviewPrompt = (u) => `You are an experienced primary-care physician auditing a chart for realism and internal consistency. Be skeptical: your job is to find what would make a clinician say "this chart is fake" or "this is wrong".

Read ${DRAFTS}/${u}/chart.txt (the rendered chart, oldest first) and ${DRAFTS}/${u}/story.md (the author's plan). If chart.txt is missing or older than the part files, first run: cd ${REPO} && python3 -I scripts/patient-bank/unit.py preview ${u}

Look for:
- Chronology leaks: a note mentioning a diagnosis, drug, result, event or family history before it existed or was known; a stopped drug still listed as current; a result discussed before it was resulted.
- Contradictions between notes and structured data (problem list, medications, labs, vitals, allergies), or with the story.
- Clinical errors: wrong doses, implausible lab values or trajectories, care a physician would not give, missing monitoring a physician would do.
- Fake texture: repeated phrasing, near-duplicate notes, boilerplate, notes that exist only to pad counts, every visit looking the same, implausibly perfect adherence or follow-up.
- Wrong ages, dates, units, or vital values that do not fit the illness.

Mark a finding major if a clinician would distrust the chart because of it; minor otherwise. Give each a concrete fix. Do not edit any files.

${RULES}`

const fixPrompt = (u, findings) => `You authored, or are taking over, the ledger for unit ${u}. An independent physician review found these problems:

${findings.map((f, i) => `${i + 1}. [${f.severity}] ${f.where}: ${f.problem}\n   Suggested fix: ${f.fix}`).join('\n')}

Read ${REPO}/scripts/patient-bank/authoring-guide.md (the rules), ${DRAFTS}/${u}/story.md, and the part files. Fix every major finding and the minor ones that are cheap, by editing the part files (and story.md if the plan itself was wrong). Then run:
  cd ${REPO} && python3 -I scripts/patient-bank/unit.py check ${u}
until it prints ACCEPTED, then unit.py preview ${u}, and reread the affected stretches of chart.txt.

${RULES}

Report accepted=true only if your final check printed ACCEPTED.`

const units = args
log(`${units.length} units`)

const results = await pipeline(
  units,
  (u) => agent(authorPrompt(u), { label: `author:${u}`, phase: 'Author', schema: AUTHOR_SCHEMA }),
  (authored, u) => authored
    ? agent(reviewPrompt(u), { label: `review:${u}`, phase: 'Review', schema: REVIEW_SCHEMA }).then(r => ({ authored, review: r }))
    : { authored: null, review: null },
  (prev, u) => {
    const major = (prev.review?.findings || []).filter(f => f.severity === 'major')
    if (!prev.authored || !prev.review || major.length === 0) return { unit: u, ...prev, fix: null }
    return agent(fixPrompt(u, prev.review.findings), { label: `fix:${u}`, phase: 'Fix', schema: FIX_SCHEMA })
      .then(fix => ({ unit: u, ...prev, fix }))
  },
)

// compact summary; full author, review and fix results stay in the run journal
const summary = results.map((r, i) => {
  if (!r || !r.authored) return { unit: units[i], failed: true }
  const f = r.review?.findings || []
  return {
    unit: units[i],
    accepted: r.fix ? r.fix.accepted : r.authored.accepted,
    patients: r.authored.patients,
    major: f.filter(x => x.severity === 'major').length,
    minor: f.filter(x => x.severity === 'minor').length,
    fixed: r.fix ? r.fix.fixed.length : 0,
    not_fixed: r.fix ? r.fix.not_fixed : [],
  }
})
log(`${summary.filter(s => s.accepted).length}/${units.length} units accepted`)
return summary
