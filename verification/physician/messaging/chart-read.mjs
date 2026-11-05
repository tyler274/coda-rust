// A message Coda can't read, in your words: the view of chart-read.cfg on ArrivalCharts.tla, for
// PStack's statechart skill. It leaves out: Filtered and Messages to match; the reply's drafts.
import { arrival, colors, groupOf, label } from "./words.mjs";

export const title = "A message Coda can't read";

export const state = (v) => arrival(v);

export const step = (action, from, to) => label(from, to);

export const group = (v) => groupOf(state(v));
export { colors };
