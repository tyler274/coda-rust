// Spruce disconnects while the reply is open, in your words: the view of chart-disconnect.cfg on
// ReplyCharts.tla, for PStack's statechart skill. It leaves out: editing and copying.
import { colors, groupOf, label, reply } from "./words.mjs";

export const title = "Spruce disconnects while the reply is open";

export const state = (v) => reply(v, { practice: true, cut: true });

export const step = (action, from, to) => label(from, to);

export const group = (v) => groupOf(state(v));
export { colors };
