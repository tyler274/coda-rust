export const meta = {
  name: 'author-patients',
  description: 'One agent per unit: build-req from the source, then the chart, until check prints ACCEPTED',
  phases: [{ title: 'Author', detail: 'one agent per unit, source to accepted chart' }],
}

// args: { repo: '<absolute path of the coda-rust checkout>', units: ['unit-2013', ...] }
const REPO = args.repo
const SCHEMA = {
  type: 'object',
  properties: {
    unit: { type: 'string' },
    accepted: { type: 'boolean', description: 'true only if the last check printed ACCEPTED' },
    unfit: { type: 'boolean', description: 'true only if build-req.md says UNFIT' },
    notes: { type: 'string', description: 'what a physician should confirm, and anything you could not do, briefly' },
  },
  required: ['unit', 'accepted', 'unfit', 'notes'],
}

const prompt = (u) => `You are making unit ${u} of a primary-care patient bank: one synthetic_hospital patient, placed in our practice and given a real chart. Work from ${REPO}. Run \`cargo run -q -p patient-bank -- brief ${u}\`, then follow "How you work" in crates/patient-bank/data/authoring-guide.md.

Write only inside patients/units/${u}/, never source.md. Don't open other units (the guide's worked example, patients/units/unit-2013/build-req.md, is fine), the seeds file or code, don't edit the guides, and don't run git. Return the structured result; accepted=true only if your last check printed ACCEPTED.`

const results = await pipeline(args.units, (u) => agent(prompt(u), { label: u, phase: 'Author', schema: SCHEMA, effort: 'medium' }))
const summary = results.map((r, i) => r || { unit: args.units[i], failed: true })
log(`${summary.filter((s) => s.accepted).length}/${args.units.length} accepted, ${summary.filter((s) => s.unfit).length} unfit`)
return summary
