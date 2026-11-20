const { readFileSync } = require('node:fs');
const { resolve } = require('node:path');
const { runInNewContext } = require('node:vm');
const { test } = require('node:test');
const assert = require('node:assert/strict');

const root = resolve(__dirname, '../../..');
const workflow = readFileSync(resolve(root, '.github/workflows/pstack-watch.yml'), 'utf8');
const script = workflow.split('          script: |\n')[1]
  .split('\n').map(line => line.replace(/^            /, '')).join('\n');
const defaultReadme = readFileSync(resolve(root, 'verification/README.md'), 'utf8');

async function runWatch({
  hasUpdates = true,
  openPrs = [],
  issues = [],
  createPrError = null,
  readme = defaultReadme,
} = {}) {
  const prsCreated = [], prsUpdated = [], issuesCreated = [], issuesUpdated = [], notices = [], summaries = [];
  const summary = {
    addHeading(text) { summaries.push(text); return this; },
    addRaw(text) { summaries.push(text); return this; },
    async write() {},
  };
  const github = {
    rest: {
      pulls: {
        async list() { return { data: openPrs }; },
        async create(pr) {
          if (createPrError) throw createPrError;
          const number = 200 + prsCreated.length;
          const created = { ...pr, number, html_url: `https://example.test/pull/${number}` };
          prsCreated.push(created);
          return { data: created };
        },
        async update(pr) { prsUpdated.push(pr); },
      },
      issues: {
        listForRepo: 'issues',
        async create(issue) {
          const number = 300 + issuesCreated.length;
          const created = { ...issue, number, html_url: `https://example.test/issues/${number}` };
          issuesCreated.push(created);
          return { data: created };
        },
        async update(issue) { issuesUpdated.push(issue); },
      },
    },
    async paginate(route) {
      assert.equal(route, 'issues');
      return issues;
    },
  };

  await runInNewContext(`(async () => {${script}\n})()`, {
    require: name => {
      assert.equal(name, 'node:fs');
      return { readFileSync: () => readme };
    },
    process: {
      env: {
        HAS_UPDATES: hasUpdates ? 'true' : 'false',
        BASE_BRANCH: 'main',
        UPDATE_BRANCH: 'codex/update-pstack-skills',
      },
    },
    github,
    context: { repo: { owner: 'tyler274', repo: 'coda-rust' } },
    core: { notice: msg => notices.push(msg), summary },
  });

  return { prsCreated, prsUpdated, issuesCreated, issuesUpdated, notices, summaries };
}

function forbiddenPrError() {
  const err = new Error('GitHub Actions is not permitted to create or approve pull requests.');
  err.status = 403;
  return err;
}

test('workflow grants issues: write beside contents: write and pull-requests: write', () => {
  assert.match(workflow, /permissions:\s*\n\s*contents:\s*write\s*\n\s*issues:\s*write\s*\n\s*pull-requests:\s*write/);
});

test('creates a review PR when Actions PR creation is permitted', async () => {
  const res = await runWatch({ hasUpdates: true });
  assert.equal(res.prsCreated.length, 1);
  assert.equal(res.prsCreated[0].head, 'codex/update-pstack-skills');
  assert.equal(res.prsCreated[0].base, 'main');
  assert.equal(res.issuesCreated.length, 0);
});

test('updates an open PR in place and closes stale tracking issue', async () => {
  const staleIssue = {
    number: 12,
    state: 'open',
    user: { login: 'github-actions[bot]' },
    body: '<!-- coda-pstack-watch -->\n<!-- upstream-revision:0000000000000000000000000000000000000000 -->',
    html_url: 'https://example.test/issues/12',
  };
  const openPr = { number: 88, html_url: 'https://example.test/pull/88' };
  const res = await runWatch({ hasUpdates: true, openPrs: [openPr], issues: [staleIssue] });
  assert.equal(res.prsCreated.length, 0);
  assert.equal(res.prsUpdated.length, 1);
  assert.equal(res.prsUpdated[0].pull_number, 88);
  assert.equal(res.issuesUpdated.length, 1);
  assert.equal(res.issuesUpdated[0].state, 'closed');
});

test('falls back to a tracking issue on 403 PR-creation refusal and avoids duplicates', async () => {
  const first = await runWatch({ hasUpdates: true, createPrError: forbiddenPrError() });
  assert.equal(first.prsCreated.length, 0);
  assert.equal(first.issuesCreated.length, 1);
  assert.match(first.issuesCreated[0].body, /<!-- coda-pstack-watch -->/);
  assert.match(first.issuesCreated[0].body, /https:\/\/github\.com\/tyler274\/coda-rust\/compare\/main\.\.\.codex\/update-pstack-skills\?expand=1/);

  const existing = {
    number: 300,
    state: 'open',
    user: { login: 'github-actions[bot]' },
    body: first.issuesCreated[0].body,
    html_url: 'https://example.test/issues/300',
  };
  const second = await runWatch({ hasUpdates: true, createPrError: forbiddenPrError(), issues: [existing] });
  assert.equal(second.issuesCreated.length, 0);
  assert.equal(second.issuesUpdated.length, 1);
  assert.equal(second.issuesUpdated[0].issue_number, 300);
  assert.equal(second.issuesUpdated[0].state, 'open');
});

test('closes open tracking issue when main matches upstream', async () => {
  const existing = {
    number: 301,
    state: 'open',
    user: { login: 'github-actions[bot]' },
    body: '<!-- coda-pstack-watch -->\n<!-- upstream-revision:0b46ec399d55f2829e88822998f53b84944837d3 -->',
    html_url: 'https://example.test/issues/301',
  };
  const res = await runWatch({ hasUpdates: false, issues: [existing] });
  assert.equal(res.prsCreated.length, 0);
  assert.equal(res.issuesCreated.length, 0);
  assert.equal(res.issuesUpdated.length, 1);
  assert.equal(res.issuesUpdated[0].issue_number, 301);
  assert.equal(res.issuesUpdated[0].state, 'closed');
});

test('re-throws non-permission pull request API failures', async () => {
  const err = new Error('GitHub API internal error');
  err.status = 500;
  await assert.rejects(
    runWatch({ hasUpdates: true, createPrError: err }),
    /GitHub API internal error/,
  );
});
