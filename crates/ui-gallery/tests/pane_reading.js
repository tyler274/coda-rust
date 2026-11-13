// What the open source pane shows of its record, or null while it shows none: the labels of its text
// parts in order; each marked place in reading order, a passage by its words and a field by its name;
// the notice's words, or null; whether
// the record's body has words; whether the first marked place stands inside the pane's scrolling
// body, or null with nothing marked; and whether that body holds more than it shows. Called through
// `Runtime.callFunctionOn`.
function () {
  const pane = document.querySelector('.source-reading-pane');
  const record = pane?.querySelector('.source-record');
  if (!record) return null;
  const body = pane.querySelector('.source-pane-body');
  const marks = Array.from(record.querySelectorAll('mark'));
  const words = (mark) => mark.closest('.record-field')?.querySelector('dt').textContent ?? mark.textContent;
  const shown = body.getBoundingClientRect();
  const first = marks[0]?.getBoundingClientRect();
  return {
    labels: Array.from(record.querySelectorAll('.source-text-label'), (label) => label.textContent),
    marks: marks.map(words),
    notice: pane.querySelector('.source-notice')?.textContent ?? null,
    has_words: record.querySelector('.source-record-body').textContent.trim().length > 0,
    first_in_view: first ? first.top >= shown.top && first.bottom <= shown.bottom : null,
    overflows: body.scrollHeight > body.clientHeight,
  };
}
