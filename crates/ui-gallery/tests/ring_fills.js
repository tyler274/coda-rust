// How the page's rings and boxes are filled: whether a group stands with none chosen, how many
// fills the rings not chosen have between them, whether those differ from a chosen ring's, and
// whether a mixed box fills as a checked one does. Called through `Runtime.callFunctionOn`.
function () {
  const fill = (input) => getComputedStyle(input).backgroundColor;
  const groups = Array.from(document.querySelectorAll('.radio-group'))
    .map((group) => Array.from(group.querySelectorAll('input')));
  const rings = groups.flat();
  const boxes = Array.from(document.querySelectorAll('.selection-row input[type=checkbox]'));
  const mixed = boxes.find((box) => box.indeterminate);
  const checked = boxes.find((box) => box.checked);
  return {
    none_chosen: groups.some((group) => group.every((ring) => !ring.checked)),
    empty_rings: new Set(rings.filter((ring) => !ring.checked).map(fill)).size,
    empty_unlike_chosen: fill(rings.find((ring) => !ring.checked)) !== fill(rings.find((ring) => ring.checked)),
    mixed_filled: !!mixed && !!checked && fill(mixed) === fill(checked),
  };
}
