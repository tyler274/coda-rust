// A native webview's way into behaviour.js, which native_load.js has made ready. It runs through the
// document's script channel, which sends it the behaviour to run. One that keeps listening waits for
// word to stop. A module that didn't load reaches a caller waiting to hear as why it has nothing.
const [kind, name, args] = await dioxus.recv();
const report = (heard) => dioxus.send(heard);
try {
  const ui = await window.codaUi;
  if (!ui) throw new Error('the module is not loaded: `Stylesheet` must come first');
  if (kind === 'fire') {
    ui.fire(name, args);
  } else if (kind === 'ask') {
    ui.ask(name, args, report);
  } else {
    const handle = ui.listen(name, args, report);
    await dioxus.recv();
    ui.stop(handle);
  }
} catch (error) {
  if (kind === 'fire') console.error(`coda ui: ${name}`, error);
  else report(JSON.stringify({ err: `${error?.name ?? 'Error'}: ${error?.message ?? error}` }));
}
