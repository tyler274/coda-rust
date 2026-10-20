// One-time, reproducible source port. Run from the worktree root with node.
// Reads kytos as reference only; all output remains in this crate.
import { readFileSync, writeFileSync, mkdirSync } from 'node:fs';
import vm from 'node:vm';
import { stripTypeScriptTypes } from 'node:module';
const source = 'submodules/kytos/';
const dest = 'crates/public-site/';
const read = p => readFileSync(source + p, 'utf8');
const output = (p, text) => writeFileSync(dest + p, text);
function evaluate(path, names, context = {}) {
  let code = read(path).replace(/^import[\s\S]*?;\n/gm, '').replace(/export /g, '');
  code = stripTypeScriptTypes(code);
  return vm.runInNewContext(code + '\n;({' + names.join(',') + '})', { contactEmail: 'hello@codahealth.ai', ...context });
}
const notices = JSON.parse(read('app/duet/clinical-terminology-notices.json'));
const duet = evaluate('app/duet/legal-content.ts', ['duetTermsVersion', 'duetTermsEffective', 'duetPrivacyEffective', 'duetPrivacySummary', 'duetPrivacySections', 'duetTermsSummary', 'duetTermsSections', 'duetConsumerHealthDataSections'], {clinicalTerminologyNotices: notices});
const coda = evaluate('app/legal-content.ts', ['lastUpdated', 'termsSections', 'privacySections']);
const clinicalSource = read('app/clinical-transparency/page.tsx');
const clinical = vm.runInNewContext(clinicalSource.slice(clinicalSource.indexOf('const lastUpdated'), clinicalSource.indexOf('export const metadata')) + ';({lastUpdated, sections})');
const archiveNames = ['privacy-2026-09-09', 'privacy-2026-09-23', 'privacy-2026-09-24', 'terms-2026-09-08'];
const archiveContext = {};
for (const name of archiveNames) {
  const text = read('app/duet/archive/' + name + '.json');
  output('data/' + name + '.json', text);
  archiveContext[name.replaceAll('-', '')] = JSON.parse(text);
}
const archives = evaluate('app/duet/legal-archive.ts', ['duetTermsArchive', 'duetPrivacyArchive'], archiveContext);
output('data/current.json', JSON.stringify({duet, coda, clinical}, null, 2) + '\n');
output('data/archives.json', JSON.stringify(archives, null, 2) + '\n');
output('data/clinical-terminology-notices.json', JSON.stringify(notices, null, 2) + '\n');
mkdirSync(dest + 'assets', { recursive: true });
output('assets/css/chrome.css', read('app/chrome.css'));
output('assets/css/site.css', read('app/_homepage/site.css'));
