// Moving the conversation, and Undo, in your words: the view of chart-correction.cfg on
// ArrivalCharts.tla, for PStack's statechart skill. It leaves out: drafts and who loses the reply,
// drawn in the withheld chart; Filtered and 15 minutes.
import { arrival, colors, groupOf, label } from "./words.mjs";

export const title = "Moving the conversation, and Undo";

export const state = (v) => arrival(v);

export const step = (action, from, to) => label(from, to);

export const group = (v) => groupOf(state(v));
export { colors };
