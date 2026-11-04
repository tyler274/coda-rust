// The words of the scenes, for PStack's statechart skill. Run it with Flows.tla and flows.cfg: each
// flow's path is told step by step, with what stands after each step.

export const title = "Patient message scenes";

const items = (v) => Object.values(v.inbox ?? {});

// Everything that stands after a step: the conversation, each item, the review, the card, who has
// it open, and what history says was sent.
export const state = (v) => {
  const parts = [v.link === "Nobody" ? "conversation not linked" : `conversation linked to ${v.link}`];
  let n = 0;
  items(v).forEach((item, k) => {
    const i = k + 1;
    if (item.from === "Practice") {
      parts.push(i > v.filed ? `reply in the channel, not yet seen (${item.words})` : "reply seen");
      return;
    }
    n += 1;
    const match = v.match[i];
    const status = i > v.filed ? "in the channel"
      : match === "Waiting" ? "waiting in Messages to match"
      : match === "Unlinked" ? "left unlinked"
      : (v.handled ?? []).includes(i) ? "settled"
      : !item.readable ? "unreadable, on Today with a link to the channel"
      : "open";
    parts.push(`message ${n} ${status}`);
  });
  const reviewing = Object.entries(v.review).filter(([, r]) => r.on).map(([p]) => p);
  if (reviewing.length) parts.push(`Coda reviewing ${reviewing.join(", ")}`);
  const c = v.card;
  if (c.open) {
    const shows = c.state !== "Ready" ? c.state.toLowerCase()
      : c.text === "Yours" ? "your edit" : "Coda's draft";
    const notes = [c.told && "says the patient wrote again", c.offer && "Coda's update offered"].filter(Boolean);
    const open = Object.entries(v.viewing).filter(([, on]) => on).map(([d]) => d);
    parts.push(`reply card for ${c.patient}: ${shows}${notes.length ? `, ${notes.join(", ")}` : ""}`
      + (open.length ? `, open for ${open.join(" and ")}` : ""));
  }
  if (v.channel === "Disconnected") parts.push("channel disconnected");
  for (const h of v.history ?? []) parts.push(`${h.patient}'s history: the practice sent ${h.words}`);
  return parts.join("; ");
};

export const step = (action, from, to) => (action === "Rest" ? null : to.event.says);
