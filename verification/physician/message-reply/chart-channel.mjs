// The words of the channel chart, for PStack's statechart skill. Run it with Charts.tla and
// chart-channel.cfg: one physician, and Coda's reply to one message is ready. The channel is
// disconnected, and you mark the reply Done or Coda reconnects. It leaves out drafts, moves, the
// patient writing, and the practice, which can't reach Coda while the channel is disconnected.

export const title = "The channel is disconnected";

export const state = (v) => {
  const r = v.reply.state === "Ready" ? "Reply ready" : "Marked Done";
  return v.channel === "Disconnected" ? `${r}; disconnected${v.reply.state === "Ready" ? ", the reply says so" : ""}` : r;
};

const steps = {
  ChannelDisconnects: "the channel is disconnected",
  CodaReconnects: "Coda reconnects and sees the reply",
  YouMarkDone: "you mark it Done",
};

export const step = (action) => steps[action] ?? null;

export const group = (v) => (v.reply.state !== "Ready" ? "Handled" : v.channel === "Disconnected" ? "Disconnected" : "Connected");

export const colors = { Connected: "teal", Disconnected: "orange", Handled: "tan" };
