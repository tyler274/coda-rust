// Explicit developer-only import from reviewed original source, never application output.
// Normal tests use --check, which needs neither Git nor the optional submodule.
import {readFileSync, writeFileSync, mkdirSync} from 'node:fs';
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {dirname} from 'node:path';
import {fileURLToPath} from 'node:url';

const commit = '5012bb749e29104e00ccb208a14a810f93fa420f';
const root = new URL('./fixtures/original/', import.meta.url);
const files = [
  'app/_homepage/main.html', 'app/_homepage/duet-phones.html',
  'app/_homepage/site.css', 'app/chrome.css',
  'app/duet/page.tsx', 'app/scheduling/dev/page.tsx',
  'app/scheduling/dev/opengraph-image.tsx',
  'app/duet/legal-content.ts', 'app/legal-content.ts',
  'app/clinical-transparency/page.tsx', 'app/duet/legal-archive.ts',
  'app/duet/clinical-terminology-notices.json',
  ...['privacy-2026-09-09', 'privacy-2026-09-23', 'privacy-2026-09-24', 'terms-2026-09-08']
    .map(name => `app/duet/archive/${name}.json`),
  'public/homepage/site.js',
];
const digest = bytes => createHash('sha256').update(bytes).digest('hex');
const manifestPath = new URL('provenance.json', root);
if (process.argv[2] === '--import') {
  const repository = fileURLToPath(new URL('../../../submodules/kytos/', import.meta.url));
  const git = args => execFileSync('git', ['-C', repository, ...args]);
  if (git(['rev-parse', 'HEAD']).toString().trim() !== commit || git(['status', '--porcelain']).length) {
    throw new Error('Import requires the clean, explicitly reviewed source commit');
  }
  const entries = files.map(source => {
    const original = git(['show', `${commit}:${source}`]);
    let bytes = original;
    let extraction = 'verbatim';
    if (source === 'public/homepage/site.js') {
      const text = original.toString();
      const start = text.indexOf('  function rng(');
      const end = text.indexOf('  var artCache');
      if (start < 0 || end <= start) throw new Error('Original painter delimiters changed');
      bytes = Buffer.from(text.slice(start, end));
      extraction = 'Exact byte slice from "  function rng(" up to (excluding) "  var artCache"';
    }
    const destination = new URL(source, root);
    mkdirSync(dirname(fileURLToPath(destination)), {recursive: true});
    writeFileSync(destination, bytes);
    return {path: source, extraction, originalSha256: digest(original), sha256: digest(bytes)};
  });
  writeFileSync(manifestPath, JSON.stringify({repository: 'Coda-Health/kytos', commit, files: entries}, null, 2) + '\n');
} else if (process.argv[2] !== '--check') {
  throw new Error('Use --check for offline integrity verification, or --import for a deliberate source update');
}
const manifest = JSON.parse(readFileSync(manifestPath));
if (manifest.commit !== commit || manifest.repository !== 'Coda-Health/kytos' ||
    JSON.stringify(manifest.files.map(entry => entry.path)) !== JSON.stringify(files)) {
  throw new Error('Snapshot provenance or inventory differs from the reviewed source');
}
for (const entry of manifest.files) {
  if (digest(readFileSync(new URL(entry.path, root))) !== entry.sha256) {
    throw new Error(`Frozen source snapshot changed: ${entry.path}; use explicit import and manual review`);
  }
}
console.log(`Verified ${manifest.files.length} original-source snapshots at ${commit}`);
