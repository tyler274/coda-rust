// A message and its reply, in your words: the view of chart-reply.cfg on ReplyCharts.tla, for
// PStack's statechart skill. It leaves out: editing and revising, drawn in the draft and revise
// charts; a second physician; Undo of No reply needed.
import { colors, groupOf, label, reply } from "./words.mjs";

export const title = "A message and its reply";

export const state = (v) => reply(v, { draft: false, copies: true, practice: true });

export const step = (action, from, to) => label(from, to);

export const group = (v) => groupOf(state(v));
export { colors };
