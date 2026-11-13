// The message in the thread whose button says `button`, before and after its status and actions
// join it: where its steps line stands, whether the steps are open and listed, whether the line and
// its parent are still the nodes the first call marked, and whether the status words `status` and
// the row of actions have joined. The first call finds the thread, keeps it, marks the line and
// scrolls it into view; later calls read the kept thread, since the button changes its words once
// the answer arrives. Called through `Runtime.callFunctionOn`, so both words arrive as values,
// never as script text.
function (button, status) {
  if (!globalThis.arrivingAnswer) {
    const thread = Array.from(document.querySelectorAll('button'))
      .find((node) => node.textContent.trim() === button)
      .closest('.gallery-thread');
    const line = thread.querySelector('button[aria-expanded]');
    line.seenBefore = true;
    line.parentElement.seenBefore = true;
    line.scrollIntoView({ block: 'center', behavior: 'instant' });
    globalThis.arrivingAnswer = thread;
  }
  const thread = globalThis.arrivingAnswer;
  const line = thread.querySelector('button[aria-expanded]');
  const box = line.getBoundingClientRect();
  const steps = document.getElementById(line.getAttribute('aria-controls'));
  return {
    at: [box.left + box.width / 2, box.top + box.height / 2],
    expanded: line.getAttribute('aria-expanded'),
    stepsHidden: steps.getAttribute('aria-hidden'),
    steps: Array.from(steps.querySelectorAll('li'), (step) => step.textContent),
    sameTrace: line.seenBefore === true && line.parentElement.seenBefore === true,
    joined: thread.textContent.includes(status) && !!thread.querySelector('[role=group][aria-label=Response]'),
  };
}
