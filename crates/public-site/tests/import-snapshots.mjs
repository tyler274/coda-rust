// Explicit developer-only import from reviewed original source, never application output.
// Normal tests use --check, which needs neither Git, the optional submodule, nor network.
//   --import         original source files, from Git blobs at the reviewed commit
//   --import-served  what production serves at that same commit: every page, the sitemap,
//                    the compiled utility stylesheet, the rendered social image and
//                    redirect behavior.
//                    These are the outputs of Next, Tailwind and ImageResponse, which the
//                    port consumes or is checked against instead of re-implementing.
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
// Every document production serves, plus the page it serves for an unknown path.
const pages = [
  '/', '/duet', '/duet/privacy', '/duet/privacy/2026-09-09', '/duet/privacy/2026-09-23',
  '/duet/privacy/2026-09-24', '/duet/terms', '/duet/terms/2026-09-08',
  '/duet/consumer-health-data', '/privacy', '/terms', '/clinical-transparency', '/scheduling/dev',
];
const notFound = '/does-not-exist';
const pageFile = path => `pages${path === '/' ? '/index' : path}.html`;
// Requests whose status and redirect target production defines (Next's trailing-slash rule).
const probes = [
  ...pages, ...pages.filter(path => path !== '/').map(path => path + '/'),
  '/duet//', '/duet/?a=1', notFound, notFound + '/', '/brand/duet/mark.svg/', '/api/health/',
];
const served = [
  ...pages.map(pageFile), 'pages/not-found.html', 'sitemap.xml',
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
} else if (process.argv[2] === '--import-served') {
  const get = async path => {
    const response = await fetch(origin + path, {redirect: 'manual', headers: {'user-agent': 'coda-rust snapshot import'}});
    return {response, bytes: Buffer.from(await response.arrayBuffer())};
  };
  const health = JSON.parse((await get('/api/health')).bytes);
  if (health.release_sha !== commit) {
    throw new Error(`Production serves ${health.release_sha}, not the reviewed commit ${commit}`);
  }
  const entries = [];
  const keep = (path, url, bytes) => { write(servedRoot, path, bytes); entries.push({path, url, sha256: digest(bytes)}); };
  for (const path of pages) {
    const {response, bytes} = await get(path);
    if (response.status !== 200) throw new Error(`${path} returned ${response.status}`);
    keep(pageFile(path), origin + path, bytes);
  }
  const missing = await get(notFound);
  if (missing.response.status !== 404) throw new Error(`${notFound} returned ${missing.response.status}`);
  keep('pages/not-found.html', origin + notFound, missing.bytes);
  keep('sitemap.xml', origin + '/sitemap.xml', (await get('/sitemap.xml')).bytes);
  // Every page outside the homepage system links this one compiled stylesheet.
  const stylesheet = html => [...html.matchAll(/<link rel="stylesheet" href="([^"]+)"/g)].map(match => match[1]);
  const [utility] = stylesheet(readFileSync(new URL(pageFile('/privacy'), servedRoot), 'utf8'));
  const css = await get(utility);
  keep('utility.css', origin + utility, css.bytes);
  const image = await get('/scheduling/dev/opengraph-image');
  if (image.response.headers.get('content-type') !== 'image/png') throw new Error('Social image is not a PNG');
  keep('scheduling/dev/opengraph-image.png', origin + '/scheduling/dev/opengraph-image', image.bytes);
  const responses = {};
  for (const path of probes) {
    const {response} = await get(path);
    responses[path] = {status: response.status, location: response.headers.get('location')};
  }
  keep('responses.json', origin, Buffer.from(JSON.stringify(responses, null, 2) + '\n'));
  const manifest = JSON.parse(readFileSync(manifestPath));
  manifest.served = {origin, releaseSha: health.release_sha, files: entries};
  writeFileSync(manifestPath, JSON.stringify(manifest, null, 2) + '\n');
} else if (process.argv[2] !== '--check') {
  throw new Error('Use --check for offline integrity verification, or --import / --import-served for a deliberate update');
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
