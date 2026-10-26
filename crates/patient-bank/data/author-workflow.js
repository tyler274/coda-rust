export const meta = {
  name: 'author-patients',
  description: 'Make primary-care charts source-first: build-req, its check, the chart, an independent trace, a fix',
  phases: [
    { title: 'Build-req', detail: 'write build-req.md from source.md, then check it against the source' },
    { title: 'Author', detail: 'write the chart from build-req.md until check prints ACCEPTED' },
    { title: 'Trace', detail: 'an independent reader traces every clinical fact to build-req.md' },
    { title: 'Fix', detail: 'the author fixes what the trace found' },
  ],
}

// args: { repo: '<absolute path of the coda-rust checkout>', units: ['unit-2013', ...] }
const REPO = args.repo
const UNITS = REPO + '/patients/units'
const PREVIEW = REPO + '/target/patient-bank/preview'
const RUN = `cd ${REPO} && cargo run -q -p patient-bank --`
const GUIDES = REPO + '/crates/patient-bank/data'

const DONE = {
  type: 'object',
  properties: {
    unit: { type: 'string' },
    unfit: { type: 'boolean', description: 'true only if build-req.md says UNFIT' },
    notes: { type: 'string', description: 'what you could not settle from the source, briefly' },
  },
  required: ['unit', 'unfit', 'notes'],
}

const ISSUES = {
  type: 'object',
  properties: {
    unit: { type: 'string' },
    issues: { type: 'array', items: { type: 'string' } },
  },
  required: ['unit', 'issues'],
}

const CHART = {
  type: 'object',
  properties: {
    unit: { type: 'string' },
    accepted: { type: 'boolean', description: 'true only if the last check printed ACCEPTED' },
    documents: { type: 'integer' },
    observations: { type: 'integer' },
    notes: { type: 'string', description: 'anything in build-req.md you could not follow, and why' },
  },
  required: ['unit', 'accepted', 'documents', 'observations', 'notes'],
}

const TRACE = {
  type: 'object',
  properties: {
    unit: { type: 'string' },
    untraced: { type: 'array', items: { type: 'object', properties: {
      where: { type: 'string' }, fact: { type: 'string' }, why: { type: 'string' } },
      required: ['where', 'fact', 'why'] } },
    mismatched: { type: 'array', items: { type: 'string' } },
    missing: { type: 'array', items: { type: 'string' } },
  },
  required: ['unit', 'untraced', 'mismatched', 'missing'],
}

const ground = (u, writes) => `Ground rules:
- Run commands only from ${REPO}. Write only ${writes}.
- Never read other units, the seeds file or any code, never edit the guides, the catalog or the seeds, and never run git.`

const writeReq = (u, issues) => `You are writing ${UNITS}/${u}/build-req.md: how one synthetic_hospital patient becomes a patient of our primary-care practice.

Read ${GUIDES}/build-req-guide.md end to end, then ${UNITS}/${u}/source.md end to end, and run ${RUN} brief ${u} for the seed. Read the pilots' build-reqs the guide names (${UNITS}/unit-2013/build-req.md, unit-2188, unit-2797) as worked examples, never as content to reuse. ${issues ? `\n\nA reader checked your build-req.md against the source and found:\n\n${issues}\n\nFix each in build-req.md.` : 'Write build-req.md following the guide.'}

${ground(u, `${UNITS}/${u}/build-req.md`)}

Return the structured result.`

const checkReq = (u) => `You are checking ${UNITS}/${u}/build-req.md against ${UNITS}/${u}/source.md, by the rules in ${GUIDES}/build-req-guide.md. Read all three end to end, and run ${RUN} brief ${u} for the seed.

List, exactly and tersely:
- every clinical fact or number in source.md that build-req.md neither carries nor resolves;
- every contradiction the source holds that build-req.md leaves unresolved, or resolves incoherently;
- every addition that is not justified by the guide's rule, and every new diagnosis, family member or life event;
- every date, interval or age that does not fit the source's own statements or the seed;
- every place build-req.md copies source wording or names a source clinician or a real institution.

Return no issues if there are none. Read only those files; change nothing.`

const author = (u) => `You are writing the chart for unit ${u} of a primary-care patient bank: one fictional adult patient over years of care in a western Massachusetts primary-care practice.

The chart is source-first: ${UNITS}/${u}/build-req.md is the authority for every clinical fact, and ${UNITS}/${u}/source.md is the patient it comes from. Follow the "How you work" section of ${GUIDES}/authoring-guide.md. Write story.md and the part files in ${UNITS}/${u}/. Run ${RUN} check ${u} until it prints ACCEPTED, then ${RUN} preview ${u}, which writes ${PREVIEW}/${u}/chart.txt, and read that file start to end as the inheriting physician. Fix what you find and check again.

${ground(u, `inside ${UNITS}/${u}/ (never source.md or build-req.md)`)}

Return the structured result. Report accepted=true only if your final check printed ACCEPTED.`

const trace = (u) => `You are checking one synthetic patient chart against its sources. Read ${UNITS}/${u}/build-req.md and ${UNITS}/${u}/source.md end to end, then ${PREVIEW}/${u}/chart.txt start to end.

build-req.md allows a clinical fact only if it is (S) in the source as build-req carries it, (R) a listed resolution, (A) a listed addition, or (P) the primary-care layer it describes, including its rules for reports, exams and prior records. List every clinical fact in chart.txt that fails that: a diagnosis, medication, dose, result, finding, procedure, referral outcome, family member or life event with no basis, or one that contradicts build-req.md or another part of the chart. Also list any sentence copied from source.md word for word (more than about eight words in a row) and any source clinician name. List every number in note or message text that differs from the structured value on the chart for the same draw or visit, and any flagged H/L value that a note calls normal. Then list each timeline row of build-req.md that the chart does not carry.

Be exact and terse. Don't list texture (wording, scheduling, portal traffic) that adds no clinical fact. Read only those three files; change nothing.`

const fix = (u, found) => `You are revising the accepted, source-first chart for unit ${u}. Read ${UNITS}/${u}/build-req.md, ${UNITS}/${u}/source.md, ${UNITS}/${u}/story.md and the part files. An independent reader traced the chart against build-req.md and found:

${found}

Fix each in the part files (and story.md if the plan changes), keeping everything else that already reads like a real record. Then run ${RUN} check ${u} until it prints ACCEPTED, run ${RUN} preview ${u}, and reread in ${PREVIEW}/${u}/chart.txt the passages you touched and every note that quotes a number you changed.

${ground(u, `inside ${UNITS}/${u}/ (never source.md or build-req.md)`)}

Return the structured result. Report accepted=true only if your final check printed ACCEPTED.`

const listed = (t) => [
  ...t.untraced.map((x) => `- ${x.where}: ${x.fact} (${x.why})`),
  ...t.mismatched.map((x) => `- ${x}`),
  ...t.missing.map((x) => `- missing from the chart: ${x}`),
].join('\n')

const results = await pipeline(args.units,
  (u) => agent(writeReq(u), { label: `req:${u}`, phase: 'Build-req', schema: DONE }),
  async (r, u) => {
    if (!r || r.unfit) return r || { unit: u, failed: 'build-req' }
    const c = await agent(checkReq(u), { label: `req-check:${u}`, phase: 'Build-req', schema: ISSUES })
    if (c && c.issues.length) {
      const again = await agent(writeReq(u, c.issues.map((x) => `- ${x}`).join('\n')),
        { label: `req-fix:${u}`, phase: 'Build-req', schema: DONE })
      return { ...(again || r), reqIssues: c.issues.length }
    }
    return { ...r, reqIssues: 0 }
  },
  async (r, u) => {
    if (r.unfit || r.failed) return r
    const chart = await agent(author(u), { label: `author:${u}`, phase: 'Author', schema: CHART })
    return { ...r, chart }
  },
  async (r, u) => {
    if (r.unfit || r.failed || !r.chart || !r.chart.accepted) return r
    const t = await agent(trace(u), { label: `trace:${u}`, phase: 'Trace', schema: TRACE })
    const found = t ? listed(t) : ''
    if (!found) return { ...r, traced: 0 }
    const fixed = await agent(fix(u, found), { label: `fix:${u}`, phase: 'Fix', schema: CHART })
    return { ...r, traced: found.split('\n').length, found, fixed }
  })

const summary = results.map((r, i) => r || { unit: args.units[i], failed: true })
log(`${summary.filter((s) => (s.fixed || s.chart || {}).accepted).length}/${args.units.length} units accepted, ${summary.filter((s) => s.unfit).length} unfit`)
return summary
