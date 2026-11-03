// Starts the popup built from crates/extension-popup; an extension page can't run inline script.
import init from './popup/extension-popup.js';
await init();
