// Two physicians share one reply, in your words: the view of chart-two.cfg on ReplyCharts.tla, for
// PStack's statechart skill. It leaves out: another physician's own draft, which you never see;
// revising; Undo.
import { colors, groupOf, label, reply } from "./words.mjs";

export const title = "Two physicians share one reply";

export const state = (v) => reply(v, { copies: true });

export const step = (action, from, to) => label(from, to);

export const group = (v) => groupOf(state(v));
export { colors };
