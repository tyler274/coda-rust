// Explicit developer-only import from reviewed original source, never application output.
// Normal tests use --check, which needs neither Git, the optional submodule, nor network.
//   --import         original source files, from Git blobs at the reviewed commit
// fixtures/served/ is the frozen record of what the kytos deployment served at that commit:
// every page, the sitemap, robots.txt, the compiled utility stylesheet, the rendered social
// image and redirect and HSTS behavior. These are the outputs of Next, Tailwind and
// ImageResponse, which the port consumes or is checked against instead of re-implementing.
import {readFileSync, writeFileSync, mkdirSync} from 'node:fs';
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {dirname} from 'node:path';
import {fileURLToPath} from 'node:url';

const commit = '5012bb749e29104e00ccb208a14a810f93fa420f';
const origin = 'https://codahealth.ai';
const root = new URL('./fixtures/original/', import.meta.url);
const servedRoot = new URL('./fixtures/served/', import.meta.url);
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
  'lib/site.ts',
];
// Every document the kytos deployment served; the frozen served record holds each.
const pages = [
  '/', '/duet', '/duet/privacy', '/duet/privacy/2026-09-09', '/duet/privacy/2026-09-23',
  '/duet/privacy/2026-09-24', '/duet/terms', '/duet/terms/2026-09-08',
  '/duet/consumer-health-data', '/privacy', '/terms', '/clinical-transparency', '/scheduling/dev',
];
const pageFile = path => `pages${path === '/' ? '/index' : path}.html`;
const served = [
  ...pages.map(pageFile), 'pages/not-found.html', 'sitemap.xml', 'robots.txt',
  'utility.css', 'scheduling/dev/opengraph-image.png', 'responses.json',
];
const digest = bytes => createHash('sha256').update(bytes).digest('hex');
const manifestPath = new URL('provenance.json', root);
const write = (base, path, bytes) => {
  const destination = new URL(path, base);
  mkdirSync(dirname(fileURLToPath(destination)), {recursive: true});
  writeFileSync(destination, bytes);
};

if (process.argv[2] === '--import') {
  const repository = fileURLToPath(new URL('../../../submodules/kytos/', import.meta.url));
  const git = args => execFileSync('git', ['-C', repository, ...args]);
  if (git(['rev-parse', 'HEAD']).toString().trim() !== commit || git(['status', '--porcelain']).length) {
    throw new Error('Import requires the clean, explicitly reviewed source commit');
  }
  const entries = files.map(source => {
    const original = git(['show', `${commit}:${source}`]);
    write(root, source, original);
    return {path: source, extraction: 'verbatim', originalSha256: digest(original), sha256: digest(original)};
  });
  const previous = JSON.parse(readFileSync(manifestPath));
  writeFileSync(manifestPath, JSON.stringify({repository: 'Coda-Health/kytos', commit, files: entries, served: previous.served}, null, 2) + '\n');
} else if (process.argv[2] !== '--check') {
  throw new Error('Use --check for offline integrity verification, or --import for a deliberate update');
}

const manifest = JSON.parse(readFileSync(manifestPath));
if (manifest.commit !== commit || manifest.repository !== 'Coda-Health/kytos' ||
    JSON.stringify(manifest.files.map(entry => entry.path)) !== JSON.stringify(files)) {
  throw new Error('Snapshot provenance or inventory differs from the reviewed source');
}
if (manifest.served?.releaseSha !== commit || manifest.served.origin !== origin ||
    JSON.stringify(manifest.served.files.map(entry => entry.path)) !== JSON.stringify(served)) {
  throw new Error('Served snapshot provenance or inventory differs from the reviewed commit');
}
for (const [base, entries] of [[root, manifest.files], [servedRoot, manifest.served.files]]) {
  for (const entry of entries) {
    if (digest(readFileSync(new URL(entry.path, base))) !== entry.sha256) {
      throw new Error(`Frozen snapshot changed: ${entry.path}; use explicit import and manual review`);
    }
  }
}
console.log(`Verified ${manifest.files.length} original-source and ${manifest.served.files.length} served snapshots at ${commit}`);
