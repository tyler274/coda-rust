// A concern stays when the reply closes, in your words: the view of chart-concern.cfg on
// ReplyCharts.tla, for PStack's statechart skill. It leaves out: editing, copying and revising.
import { colors, groupOf, label, reply } from "./words.mjs";

export const title = "A concern stays when the reply closes";

export const state = (v) => reply(v);

export const step = (action, from, to) => label(from, to);

export const group = (v) => groupOf(state(v));
export { colors };
