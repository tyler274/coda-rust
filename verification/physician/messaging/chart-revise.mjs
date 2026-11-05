// Asking Coda to revise, and restoring your version, in your words: the view of chart-revise.cfg on
// ReplyCharts.tla, for PStack's statechart skill. It leaves out: copying and No reply needed.
import { colors, groupOf, label, reply } from "./words.mjs";

export const title = "Asking Coda to revise, and restoring your version";

export const state = (v) => reply(v);

export const step = (action, from, to) => label(from, to);

export const group = (v) => groupOf(state(v));
export { colors };
