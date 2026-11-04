(() => {
  const coda = window.__coda;
  if (!coda || location.href === 'about:blank') return { shows: 'Closed', held: [], news: 0, signingOut: 0, patient: false };
  const text = document.body ? document.body.innerText : '';
  // A pending read's words are for screen readers and its line fades in after a delay, so read
  // what the page says, not what it has painted yet.
  const said = document.body ? document.body.textContent : '';
  const has = (selector) => !!document.querySelector(selector);
  const heading = (document.querySelector('.sign-in-content h1') || {}).innerText || '';
  const shows = has('.session-prompt') ? 'Sign-in prompt over your work'
    : has('.glass-app') ? (said.includes('Loading Today…') || said.includes('Opening the patient…') ? 'Opening your workspace' : 'Your workspace')
    : heading.startsWith('Sign in to Coda') ? 'Enter your work email'
    : heading.startsWith('Check your email') ? 'Check your email'
    : heading.startsWith('Signing you in') ? 'Signing you in'
    : heading.startsWith('No access') ? 'No access to this practice'
    : heading.startsWith('Let') ? (text.includes('no longer valid') ? 'This link no longer works' : "Coda couldn't reach sign-in")
    : 'Something else: ' + heading;
  return { shows, held: coda.held.map((h) => [h.kind, h.bearer]), news: coda.news.length,
    signingOut: coda.signingOut, patient: text.includes('Records as of') };
})()
