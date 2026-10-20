// Extract expected data from frozen original source, never generated application data.
// Provenance and update policy: book/src/public-site.md; no submodule required.
import {readFileSync} from 'node:fs';
import {stripTypeScriptTypes} from 'node:module';
import vm from 'node:vm';
const root = new URL('./fixtures/original/', import.meta.url);
const read = path => readFileSync(new URL(path, root), 'utf8');
function evaluate(path, names, context = {}) {
  const code = stripTypeScriptTypes(read(path).replace(/^import[\s\S]*?;\n/gm, '').replace(/export /g, ''));
  return vm.runInNewContext(code + '\n;({' + names.join(',') + '})', {contactEmail: 'hello@codahealth.ai', ...context});
}
const notices = JSON.parse(read('app/duet/clinical-terminology-notices.json'));
const duet = evaluate('app/duet/legal-content.ts', ['duetTermsVersion', 'duetTermsEffective', 'duetPrivacyEffective', 'duetPrivacySummary', 'duetPrivacySections', 'duetTermsSummary', 'duetTermsSections', 'duetConsumerHealthDataSections'], {clinicalTerminologyNotices: notices});
const coda = evaluate('app/legal-content.ts', ['lastUpdated', 'termsSections', 'privacySections']);
const clinicalSource = read('app/clinical-transparency/page.tsx');
const clinical = vm.runInNewContext(clinicalSource.slice(clinicalSource.indexOf('const lastUpdated'), clinicalSource.indexOf('export const metadata')) + ';({lastUpdated, sections})');
const archiveNames = ['privacy-2026-09-09', 'privacy-2026-09-23', 'privacy-2026-09-24', 'terms-2026-09-08'];
const archiveContext = {};
for (const name of archiveNames) archiveContext[name.replaceAll('-', '')] = JSON.parse(read('app/duet/archive/' + name + '.json'));
const archives = evaluate('app/duet/legal-archive.ts', ['duetTermsArchive', 'duetPrivacyArchive'], archiveContext);
console.log(JSON.stringify({current: {duet, coda, clinical}, archives, notices}));
