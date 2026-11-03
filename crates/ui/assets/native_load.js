// Makes behaviour.js ready on a native webview's page, from the source the script channel sends.
// `window.codaUi` is set before anything is awaited, so a behaviour run after this script finds it,
// and a module that won't load rejects this script, which Rust hears and says. A later `Stylesheet`
// keeps the module, and still takes its source, since Rust can't send to a script that has returned.
const source = dioxus.recv();
window.codaUi ??= source.then((module) => import(URL.createObjectURL(new Blob([module], { type: 'text/javascript' }))));
await Promise.all([source, window.codaUi]);
