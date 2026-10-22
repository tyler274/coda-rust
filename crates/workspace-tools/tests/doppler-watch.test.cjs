const { readFileSync } = require('node:fs');
const { resolve } = require('node:path');
const { runInNewContext } = require('node:vm');
const { test } = require('node:test');
const assert = require('node:assert/strict');

const root = resolve(__dirname, '../../..');
const workflow = readFileSync(resolve(root, '.github/workflows/doppler-watch.yml'), 'utf8');
const script = workflow.split('          script: |\n')[1]
  .split('\n').map(line => line.replace(/^            /, '')).join('\n');
const source = readFileSync(resolve(root, 'crates/workspace-tools/src/doppler.rs'), 'utf8');
const pin = name => source.match(new RegExp(`pub const ${name}: &str =\\s*"([^"]+)";`))[1];
const pinned = pin('VERSION').replace(/^v/, '');

async function watch({ latest = pinned, advisories = [], issues = [], badDigest = false, apiError = false } = {}) {
  const created = [], updated = [], failures = [];
  const summary = { addHeading() { return this; }, addRaw() { return this; }, async write() {} };
  const github = {
    rest: {
      repos: {
        async getLatestRelease() { return { data: { tag_name: latest, html_url: `https://github.com/DopplerHQ/cli/releases/tag/${latest}` } }; },
        async getReleaseByTag() { return { data: { assets: [{ name: `doppler_${pinned}_linux_amd64.tar.gz`, digest: badDigest ? 'wrong' : `sha256:${pin('SHA256')}`, browser_download_url: pin('URL') }] } }; },
      },
      issues: {
        listForRepo: 'issues',
        async create(issue) { created.push(issue); return { data: { html_url: 'https://example.test/issue' } }; },
        async update(issue) { updated.push(issue); },
      },
    },
    async paginate(route) {
      if (route === 'issues') return issues;
      if (apiError) throw new Error('advisory API unavailable');
      return advisories;
    },
  };
  await runInNewContext(`(async () => {${script}\n})()`, {
    require: name => { assert.equal(name, 'node:fs'); return { readFileSync: () => source }; },
    github, context: { repo: { owner: 'test', repo: 'test' } },
    core: { notice() {}, setFailed: message => failures.push(message), summary },
  });
  return { created, updated, failures };
}

const advisory = { ghsa_id: 'GHSA-test', severity: 'high', html_url: 'https://example.test/advisory', updated_at: '2026-10-22T00:00:00Z' };

test('current release and no advisories creates no noise', async () => {
  assert.deepEqual(await watch(), { created: [], updated: [], failures: [] });
});
test('new release creates a review, not an automatic upgrade', async () => {
  const { created } = await watch({ latest: '99.0.0' });
  assert.equal(created.length, 1);
  assert.match(created[0].body, /coda-doppler-watch:release-99.0.0/);
  assert.match(created[0].body, /never upgrades, merges, or deploys automatically/);
});
test('existing closed release issue is not duplicated', async () => {
  const issues = [{ user: { login: 'github-actions[bot]' }, body: '<!-- coda-doppler-watch:release-99.0.0 -->', state: 'closed' }];
  assert.equal((await watch({ latest: '99.0.0', issues })).created.length, 0);
});
test('advisory is reported even when the pinned release is current', async () => {
  const { created } = await watch({ advisories: [advisory] });
  assert.equal(created.length, 1);
  assert.match(created[0].title, /GHSA-test/);
});
test('changed advisory reopens its review; unchanged advisory is quiet', async () => {
  const issue = { number: 7, user: { login: 'github-actions[bot]' }, body: '<!-- coda-doppler-watch:advisory-GHSA-test -->', state: 'closed' };
  const { updated } = await watch({ advisories: [advisory], issues: [issue] });
  assert.equal(updated[0].state, 'open');
  issue.body = updated[0].body;
  assert.equal((await watch({ advisories: [advisory], issues: [issue] })).updated.length, 0);
});
test('integrity mismatch alerts and fails without accepting a new checksum', async () => {
  const { created, failures } = await watch({ badDigest: true });
  assert.equal(created.length, 1);
  assert.equal(failures.length, 1);
});
test('API errors are not interpreted as no security advisories', async () => {
  await assert.rejects(watch({ apiError: true }), /advisory API unavailable/);
});
test('malformed stable version is rejected', async () => {
  await assert.rejects(watch({ latest: 'not-a-version' }), /Unexpected stable release/);
});
