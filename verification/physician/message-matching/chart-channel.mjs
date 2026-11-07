// The words of the channel chart, for PStack's statechart skill. Run it with Charts.tla and
// chart-channel.cfg: one physician who may access both patients, and no message. The channel works,
// stops for a while or is revoked, comes back, or is disconnected and connected again, and each
// state says what Today shows. chart-cant-read draws a message; chart draws linking.

export const title = "Coda reads the practice's channel";

// Each state shows the model's own channel and Today values, so a wrong Today shows on the chart.
const channels = {
  "Coda reads it": "Coda reads the channel",
  "Coda can't read it for a while": "Coda can't read the channel for a while",
  "The grant is revoked or blocked": "The grant is revoked or blocked",
  Disconnected: "Disconnected",
};

const todays = {
  nothing: "Today says nothing about it",
  "Coda can't read the channel": "Today: Coda can't read the channel",
  "Coda can't read the channel: Reconnect": "Today: Coda can't read the channel, Reconnect",
};

export const state = (v) => `${channels[v.channel]}; ${todays[v.today]}`;

const steps = {
  ChannelComesBack: "Coda can read the channel again",
  YouReconnect: "you press Reconnect",
  YouDisconnect: "you disconnect the channel",
  YouConnect: "you connect the channel",
};

export const step = (action, from, to) =>
  action === "ChannelStops"
    ? to.channel === "The grant is revoked or blocked" ? "the grant is revoked or blocked" : "Coda can't read the channel for a while"
    : steps[action] ?? null;

export const group = (v) => (v.channel === "Coda reads it" ? "Reading" : v.channel === "Disconnected" ? "Disconnected" : "Stopped");

export const colors = { Reading: "teal", Stopped: "orange", Disconnected: "tan" };
