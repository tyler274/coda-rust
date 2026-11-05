// The conversation moves: the reply is withheld, and your draft stays yours, in your words: the view
// of chart-withheld.cfg on ReplyCharts.tla, for PStack's statechart skill. It leaves out: copying,
// revising and No reply needed; a second physician, drawn in the two chart.
import { colors, groupOf, label, reply } from "./words.mjs";

export const title = "The conversation moves: the reply is withheld, and your draft stays yours";

export const state = (v) => reply(v);

export const step = (action, from, to) => label(from, to);

export const group = (v) => groupOf(state(v));
export { colors };
