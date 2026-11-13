// What the placed citation preview labelled `label` shows of its source, or null while there is none:
// the words of what the record says about itself, or null, and how many blocks of a cited place it
// shows: quoted passages, quoted fields and pages. Called through `Runtime.callFunctionOn`, so the label
// arrives as a value, never as script text.
function (label) {
  const preview = Array.from(document.querySelectorAll('.citation-preview[data-side]'))
    .find((node) => node.getAttribute('aria-label') === label);
  if (!preview) return null;
  const body = preview.querySelector('.citation-preview-body');
  return {
    about: body.querySelector('.source-record-about')?.textContent ?? null,
    places: body.querySelectorAll('.source-quote, .source-page').length,
  };
}
