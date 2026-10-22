// One-time, reproducible source port. Run from the worktree root with node.
// Reads frozen snapshots; all generated output remains separate in this crate.
// Updating snapshots is a distinct, manually reviewed tests/import-snapshots.mjs operation.
// The original TypeScript modules are imported as modules (Node strips their types), so
// their values are exactly what Next evaluated; no source text is rewritten.
import { readFileSync, writeFileSync, mkdirSync } from 'node:fs';
import { registerHooks } from 'node:module';
import { dirname } from 'node:path';
import vm from 'node:vm';
const source = new URL('../tests/fixtures/original/', import.meta.url);
const served = new URL('../tests/fixtures/served/', import.meta.url);
const dest = new URL('./', new URL('../', import.meta.url));
const read = p => readFileSync(new URL(p, source), 'utf8');
const output = (p, bytes) => {
  const destination = new URL(p, dest);
  mkdirSync(dirname(destination.pathname), { recursive: true });
  writeFileSync(destination, bytes);
};
// The original's "@/" path alias and extensionless imports, as its tsconfig resolves them.
registerHooks({
  resolve(specifier, context, next) {
    if (specifier.startsWith('@/')) specifier = new URL(specifier.slice(2) + '.ts', source).href;
    const resolved = next(specifier, context);
    return resolved.url.endsWith('.json') ? { ...resolved, importAttributes: { type: 'json' } } : resolved;
  },
});
const original = path => import(new URL(path, source).href);
const notices = JSON.parse(read('app/duet/clinical-terminology-notices.json'));
const { duetTermsVersion, duetTermsEffective, duetPrivacyEffective, duetPrivacySummary, duetPrivacySections, duetTermsSummary, duetTermsSections, duetConsumerHealthDataSections } = await original('app/duet/legal-content.ts');
const duet = { duetTermsVersion, duetTermsEffective, duetPrivacyEffective, duetPrivacySummary, duetPrivacySections, duetTermsSummary, duetTermsSections, duetConsumerHealthDataSections };
const { lastUpdated, termsSections, privacySections } = await original('app/legal-content.ts');
const coda = { lastUpdated, termsSections, privacySections };
// The clinical page is TSX; only its plain data declarations are evaluated.
const clinicalSource = read('app/clinical-transparency/page.tsx');
const clinical = vm.runInNewContext(clinicalSource.slice(clinicalSource.indexOf('const lastUpdated'), clinicalSource.indexOf('export const metadata')) + ';({lastUpdated, sections})');
for (const name of ['privacy-2026-09-09', 'privacy-2026-09-23', 'privacy-2026-09-24', 'terms-2026-09-08']) {
  output('data/' + name + '.json', read('app/duet/archive/' + name + '.json'));
}
const { duetTermsArchive, duetPrivacyArchive } = await original('app/duet/legal-archive.ts');
output('data/current.json', JSON.stringify({duet, coda, clinical}, null, 2) + '\n');
output('data/archives.json', JSON.stringify({duetTermsArchive, duetPrivacyArchive}, null, 2) + '\n');
output('data/clinical-terminology-notices.json', JSON.stringify(notices, null, 2) + '\n');
// Served byte-for-byte: original styles and motion, and what production compiled or rendered.
output('assets/css/chrome.css', read('app/chrome.css'));
output('assets/css/site.css', read('app/_homepage/site.css'));
output('assets/homepage/site.js', read('public/homepage/site.js'));
output('assets/css/utility.css', readFileSync(new URL('utility.css', served)));
output('assets/scheduling/dev/opengraph-image.png', readFileSync(new URL('scheduling/dev/opengraph-image.png', served)));
