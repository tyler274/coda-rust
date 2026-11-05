// Linking a message: on its own, or by you, in your words: the view of chart-linking.cfg on
// ArrivalCharts.tla, for PStack's statechart skill. It leaves out: the review and the reply after
// linking; the channel stopping; moves.
import { arrival, colors, groupOf, label } from "./words.mjs";

export const title = "Linking a message: on its own, or by you";

export const state = (v) => arrival(v);

export const step = (action, from, to) => label(from, to);

export const group = (v) => groupOf(state(v));
export { colors };
