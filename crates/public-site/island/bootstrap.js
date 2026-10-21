// Serve as type="module" (deferred by definition), after motion.css. Keep this,
// public_site_motion.js and public_site_motion_bg.wasm in one hashed directory,
// or rewrite the two sibling module URLs together when embedding server assets.
const root = document.documentElement;
const nav = document.querySelector('nav');
const scroll = () => nav?.classList.toggle('scrolled', window.scrollY > 12);
window.addEventListener('scroll', scroll, { passive: true });
scroll();
const load = () => {
  if (!document.querySelector('#heroStage, #memStage, .stage-wrap')) return;
  import('./public_site_motion.js')
    .then(async ({ default: init, start }) => { await init(); start(); })
    .catch(() => { root.classList.remove('motion-ready'); });
};
const defer = () => {
  if ('requestIdleCallback' in window) window.requestIdleCallback(load, { timeout: 1500 });
  else window.setTimeout(load, 100);
};
if (document.readyState === 'loading') document.addEventListener('DOMContentLoaded', defer, { once: true });
else defer();
