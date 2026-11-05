// Your draft when the patient writes again, in your words: the view of chart-draft.cfg on
// ReplyCharts.tla, for PStack's statechart skill. It leaves out: revising, copying and No reply
// needed.
import { colors, groupOf, label, reply } from "./words.mjs";

export const title = "Your draft when the patient writes again";

export const state = (v) => reply(v);

export const step = (action, from, to) => label(from, to);

export const group = (v) => groupOf(state(v));
export { colors };
