// The screen stack in the part titled `part`: does `action` there with `value`, waits until its
// layers have settled, and answers what stands. Actions: `open` presses the row titled `value` on
// the top layer, `back` presses its Back, `watched-back` does the same and also answers the closed
// layer's opacity and left edge on every frame until it settles (null once it has left the page),
// `history-back` steps back through the page's history, `tab` presses the tab named `value`,
// `form` chooses the form named `value`, `scroll` sets the first layer's scroll place to `value`,
// `quickly` makes each press in `value`, a list of [action, value] pairs, two frames after the one
// before, holding every step through history the page asks for until two frames after the last
// press, then lets them go, waits until each step has landed and also answers `history.length`,
// and `read` does nothing. It answers history's depth, what has focus (a layer by its title, or a
// tab as `tab` and its name), each layer's title, where it stands and whether it takes input, and
// the first layer's scroll place. Called through `Runtime.callFunctionOn`, so every value arrives
// as an argument, never as script text.
async function (part, action, value) {
  const root = Array.from(document.querySelectorAll('.gallery-part'))
    .find((node) => node.querySelector('.gallery-part-title').textContent === part);
  const layers = () => Array.from(root.querySelectorAll('.screen-frame'));
  const top = () => layers().find((layer) => layer.classList.contains('is-top'));
  const closing = action === 'watched-back' ? top() : null;
  const seen = [];
  // The closed layer's opacity and how far its left edge stands from its resting place, the left
  // edge of the box that holds the layers.
  const frame = () => new Promise((done) => requestAnimationFrame(() => done())).then(() => {
    if (!closing) return;
    seen.push(closing.isConnected
      ? [+parseFloat(getComputedStyle(closing).opacity).toFixed(3),
        Math.round(closing.getBoundingClientRect().left - closing.parentElement.getBoundingClientRect().left)]
      : null);
  });
  const depth = () => (Number.isInteger(history.state?.screenDepth) ? history.state.screenDepth : 0);
  const standings = ['is-top', 'is-under', 'is-away', 'is-exiting'];
  const standing = (layer) => standings.find((name) => layer.classList.contains(name)).slice(3);
  const shape = () => `${depth()} ${layers().map((layer) => standing(layer)).join(' ')}`;
  const press = (action, value) => {
    if (action === 'open') {
      Array.from(top().querySelectorAll('.list-row'))
        .find((row) => row.querySelector('strong').textContent === value).click();
    } else if (action === 'back' || action === 'watched-back') {
      top().querySelector('.top-bar-back').click();
    } else if (action === 'history-back') {
      history.back();
    } else if (action === 'tab') {
      // Pressed as from the keys: focused, then activated.
      const button = Array.from(root.querySelectorAll('.tabs button')).find((each) => each.textContent.trim() === value);
      button.focus();
      button.click();
    } else if (action === 'form') {
      Array.from(root.querySelectorAll('.segmented-option')).find((option) => option.textContent.trim() === value).click();
    } else if (action === 'scroll') {
      layers()[0].querySelector('.screen-frame-body').scrollTop = value;
    }
  };
  const before = shape();
  if (action === 'quickly') {
    // Each press is made while the steps asked for before it are still held, so every press
    // overlaps them however slow the page is. Once let go, a step is travelling until it lands as a
    // `popstate`, which this listener hears after the page's own.
    const go = History.prototype.go;
    const held = [];
    let travelling = 0;
    const landed = () => { travelling -= 1; };
    history.go = (delta) => { held.push(delta); };
    for (const [index, [each, its]] of value.entries()) {
      if (index > 0) { await frame(); await frame(); }
      press(each, its);
    }
    await frame(); await frame();
    addEventListener('popstate', landed);
    history.go = (delta) => { travelling += 1; go.call(history, delta); };
    for (const delta of held) history.go(delta);
    for (let wait = 0; wait < 300 && travelling > 0; wait += 1) await frame();
    removeEventListener('popstate', landed);
    delete history.go;
  } else {
    press(action, value);
  }
  // A step that changes the stack is waited for, then its motion: settled is three quiet frames
  // with nothing leaving and nothing moving.
  if (!['scroll', 'read', 'form', 'quickly'].includes(action)) {
    for (let wait = 0; wait < 120 && shape() === before; wait += 1) await frame();
  }
  for (let quiet = 0, wait = 0; quiet < 3 && wait < 300; wait += 1) {
    await frame();
    const moving = root.getAnimations({ subtree: true }).length > 0;
    quiet = moving || layers().some((layer) => layer.classList.contains('is-exiting')) ? 0 : quiet + 1;
  }
  const focused = layers().find((layer) => layer === document.activeElement);
  const tab = Array.from(root.querySelectorAll('.tabs button')).find((button) => button === document.activeElement);
  return {
    depth: depth(),
    focus: focused ? focused.querySelector('.top-bar-title').textContent : tab ? `tab ${tab.textContent.trim()}` : null,
    layers: layers().map((layer) => [layer.querySelector('.top-bar-title').textContent, standing(layer), !layer.inert]),
    scroll: layers()[0].querySelector('.screen-frame-body').scrollTop,
    ...(closing ? { closing: seen } : {}),
    ...(action === 'quickly' ? { length: history.length } : {}),
  };
}
