// Each words-only button beside its busy twin in the part titled `waiting`, and the retry ready
// beside the one running in the part titled `retry`: whether each pair has one size, whether the
// busy one's spinner shows, and whether it stands centred in its button, within a pixel each way.
// Called through `Runtime.callFunctionOn`, so both titles arrive as values, never as script text.
function (waiting, retry) {
  const part = (title) => Array.from(document.querySelectorAll('.gallery-part'))
    .find((node) => node.querySelector('.gallery-part-title').textContent === title);
  const size = (button) => {
    const box = button.getBoundingClientRect();
    return [box.width, box.height];
  };
  const middle = (node) => {
    const box = node.getBoundingClientRect();
    return [box.left + box.width / 2, box.top + box.height / 2];
  };
  const spinnerOf = (button) => button.querySelector('.spinner');
  const shown = (button) => {
    const spinner = spinnerOf(button);
    return !!spinner && getComputedStyle(spinner).display !== 'none' && spinner.getBoundingClientRect().width > 0;
  };
  const centred = (button) => {
    const spinner = spinnerOf(button);
    if (!spinner) return false;
    const [x, y] = middle(spinner);
    const [bx, by] = middle(button);
    return Math.abs(x - bx) <= 1 && Math.abs(y - by) <= 1;
  };
  const pairs = Array.from(part(waiting).querySelectorAll('.gallery-row'))
    .map((row) => Array.from(row.querySelectorAll('button')))
    .filter((buttons) => buttons.length === 2 && !buttons[0].ariaBusy && buttons[1].ariaBusy === 'true');
  const retries = part(retry);
  pairs.push([retries.querySelector('button:not([disabled])'), retries.querySelector('button[aria-busy=true]')]);
  return pairs.map(([idle, busy]) => ({
    words: idle.textContent.trim(),
    same_size: JSON.stringify(size(idle)) === JSON.stringify(size(busy)),
    spinner: shown(busy),
    centred: centred(busy),
  }));
}
