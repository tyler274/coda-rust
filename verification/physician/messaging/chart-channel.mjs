// Coda can't read Spruce, or the practice disconnects it, in your words: the view of
// chart-channel.cfg on ArrivalCharts.tla, for PStack's statechart skill. It leaves out: the review
// and the reply; Filtered and 15 minutes.
import { channelLine, label, messages } from "./words.mjs";

export const title = "Coda can't read Spruce, or the practice disconnects it";

export const state = (v) => ({ Message: messages(v), Spruce: channelLine(v) || "Coda reads Spruce" });

export const step = (action, from, to) => label(from, to);
