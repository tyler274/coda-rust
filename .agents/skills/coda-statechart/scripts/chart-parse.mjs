// Read a Mermaid stateDiagram-v2 chart into a syntax tree.
// Adapted from cursor/plugins pstack at eb4db2b55b2b7dec3d9282754cddf0a56bf4ff10.
// MIT license: ../../../../verification/PSTACK-LICENSE (relative to this directory).
// Coda rejects guard text without a machine guard, malformed rule annotations,
// guarded initial arrows, and ambiguous region/state identities instead of dropping them.
export const unwrap = (s) =>
  s
    .replace(/<br\s*\/?>|#160;/g, " ")
    .replace(/#(quot|34);/g, '"')
    .replace(/\s+/g, " ")
    .trim();

export const LEGEND_ID = "legend";

// UML labels: "event [guard words] / effect words". The effect starts at the first " / "
// outside brackets; the event and guard words come before it.
export function splitEffect(text) {
  let depth = 0;
  for (let i = 0; i < text.length; i++) {
    if (text[i] === "[") depth++;
    else if (text[i] === "]") depth = Math.max(0, depth - 1);
    else if (depth === 0 && text.startsWith(" / ", i))
      return { said: text.slice(0, i).trim(), effect: text.slice(i + 3).trim() };
  }
  return { said: text, effect: "" };
}

// The event name, whether guard words are present, and the effect as intent.
// UML "/ may ...": the region may also keep its state when this edge is enabled.
export function labelParts(text) {
  const { said, effect } = splitEffect(text);
  const label = said.replace(/\s*\[[^\]]*\]/g, "");
  return {
    label,
    words: label !== said,
    ...(effect ? { intent: effect, ...(/^may\s/.test(effect) ? { optional: true } : {}) } : {}),
  };
}

// An optional reason tag on a limit: "%% when Ids | reason: TEXT", "%% never ... | reason: TEXT",
// and "| needs Ids | reason: TEXT |" in an event line. The reason is free text.
export function reasoned(text) {
  const m = /^(.*?)\s*\|\s*reason:\s*(.+)$/.exec(text);
  return m ? [m[1], m[2].trim()] : [text, null];
}

// Reserved words at the start of a %% line. A line that starts with one but does not
// have the construct's shape stays an ordinary comment; lint lists such lines.
export const KEYWORDS = /^%%\s*(when|never|template|instance|only|except|event|events|cause)\b/;

// Shapes: a composite whose first line is `%% shape NAME` is not part of the chart; it
// defines contents written once. A composite whose first line is
// `%% use NAME as KEY: name = value, ...` is filled with those contents as a one-instance
// template, so the same shape can sit inside different composites; the lines after it
// add what only that use has, such as the steps that start it.
const OPENS = /^\s*state\s+"[^"]*"\s+as\s+[A-Za-z_]\w*\s*\{\s*$/;
const CLOSES = /^\s*\}\s*$/;
function useShapes(lines) {
  const shapes = new Map();
  const rest = [];
  for (let i = 0; i < lines.length; i++) {
    const shape = OPENS.test(lines[i]) && /^\s*%%\s*shape\s+([A-Za-z_][\w-]*)\s*$/.exec(lines[i + 1] ?? "");
    if (!shape) { rest.push(lines[i]); continue; }
    if (shapes.has(shape[1])) throw new Error(`the shape ${shape[1]} is defined twice`);
    const contents = [];
    let depth = 1, j = i + 2;
    for (; j < lines.length; j++) {
      if (OPENS.test(lines[j])) depth++;
      else if (CLOSES.test(lines[j]) && !--depth) break;
      contents.push(lines[j]);
    }
    if (depth) throw new Error(`the shape ${shape[1]} has no closing }`);
    shapes.set(shape[1], contents);
    i = j;
  }
  const out = [];
  for (let i = 0; i < rest.length; i++) {
    const use = /^\s*%%\s*use\s+([A-Za-z_][\w-]*)\s+as\s+([A-Za-z0-9_]+)\s*(?::\s*(.*))?$/.exec(rest[i]);
    if (!use) { out.push(rest[i]); continue; }
    if (!OPENS.test(out.at(-1) ?? ""))
      throw new Error(`%% use must be the first line inside the composite it fills: ${rest[i].trim()}`);
    const contents = shapes.get(use[1]);
    if (!contents) throw new Error(`%% use names the shape ${use[1]}, which no %% shape composite defines`);
    out.push(`%% template from ${use[1]}`, `%% instance ${use[2]}: ${use[3] ?? ""}`, ...contents);
  }
  return out;
}

// The tree: root.parts[i] = { children, transitions }; a child is a leaf
// { name, leaf: true } or a composite { name, parts }. Ids are global, as in Mermaid.
// A transition keeps its label text so templates can substitute placeholders in it.
export function readChart(text) {
  const names = new Map();
  const nevers = [];
  const ignored = [];
  const events = [];
  const causes = new Map(); // cause -> what every event it causes needs
  const groups = new Map();
  let strict = false;
  let last = null; // the transition a %% when line guards
  let item = null; // the state or transition a %% only / %% except line limits
  const newPart = () => ({ children: [], transitions: [] });
  const root = { name: "", parts: [newPart()] };
  const stack = [root];
  const part = () => stack[stack.length - 1].parts.at(-1);
  const template = () => [...stack].reverse().find((n) => n.template) ?? null;
  const implicit = new Map();
  const declare = (id) => {
    const prev = implicit.get(id);
    if (prev)
      prev.children.splice(
        prev.children.findIndex((c) => c.name === id),
        1,
      );
    implicit.set(id, null);
  };
  const mention = (id) => {
    if (id === "[*]" || implicit.has(id)) return;
    part().children.push({ name: id, leaf: true });
    implicit.set(id, part());
  };
  const body = text.replace(/^\s*---[ \t]*\r?\n[^]*?\r?\n---[ \t]*(\r?\n|$)/, "");
  if (!/^\s*stateDiagram-v2\s*$/m.test(body))
    throw new Error("expected a stateDiagram-v2 chart (not a flowchart or Markdown fence)");
  for (const raw of useShapes(body.split("\n"))) {
    const line = raw.trim().replace(/(\[\*\]|[A-Za-z_]\w*):::[\w-]+/g, "$1");
    const when = /^%%\s*when\s+(not\s+)?(.+)$/.exec(line);
    if (when) {
      if (!last) throw new Error(`a %% when line must come right after a transition: ${line}`);
      if (!last.words)
        throw new Error(
          `the guarded step "${last.label}" must end its label with the guard's words in [ ]`,
        );
      const [ids, reason] = reasoned(when[2]);
      last.when = { not: Boolean(when[1]), ids: ids.split(",").map((s) => s.trim()), ...(reason ? { reason } : {}) };
      last = null;
      continue;
    }
    last = null;
    const [ruleLine, ruleReason] = /^%%\s*never\b/.test(line) ? reasoned(line) : [line, null];
    const never = /^%%\s*never\s+([A-Za-z_][A-Za-z0-9_]*)\s*:\s*(.+?)(?:\s+--\s+(.*))?$/.exec(ruleLine);
    if (never) {
      nevers.push({
        name: never[1],
        states: never[2].split("&").map((s) => s.trim()),
        sentence: never[3] ?? "",
        ...(ruleReason ? { reason: ruleReason } : {}),
        ...(template() ? { template: template() } : {}),
      });
      item = null;
      continue;
    }
    if (/^%%\s*(when|never)\b/.test(line))
      throw new Error(`malformed formal annotation: ${line}`);
    // Templates: one region written once, expanded per %% instance line.
    const marks = /^%%\s*template(?:\s+from\s+(\S+))?\s*$/.exec(line);
    if (marks) {
      const node = stack.at(-1);
      if (template()) throw new Error(`templates cannot be nested: ${node.name}`);
      if (node === root || node.parts.length > 1 || node.parts[0].children.length || node.parts[0].transitions.length)
        throw new Error(`%% template must be the first line inside the composite it marks: ${line}`);
      node.template = { instances: [], ...(marks[1] ? { shape: marks[1] } : {}) };
      item = null;
      continue;
    }
    const instance = /^%%\s*instance\s+([A-Za-z0-9_]+)\s*:\s*(.*)$/.exec(line);
    if (instance) {
      const node = stack.at(-1);
      if (!node.template) throw new Error(`%% instance must be directly inside a %% template composite: ${line}`);
      if (node.template.instances.some((i) => i.key === instance[1]))
        throw new Error(`template ${node.name} has two instances named ${instance[1]}`);
      const values = { key: instance[1] };
      for (const pair of instance[2].trim() ? instance[2].split(/,\s*(?=[A-Za-z_]\w*\s*=)/) : []) {
        const kv = /^\s*([A-Za-z_]\w*)\s*=\s*(.*?)\s*$/.exec(pair);
        if (!kv) throw new Error(`cannot read the substitution "${pair}" in: ${line}`);
        if (kv[1] === "key") throw new Error(`{key} is the instance key and cannot be substituted: ${line}`);
        values[kv[1]] = kv[2];
      }
      node.template.instances.push({ key: instance[1], values });
      item = null;
      continue;
    }
    // A receipt: the first line of an empty composite, which the tool fills with the receipt
    // for every command declared on ITEM.
    // Elsewhere the line stays a comment; a command on ITEM then finds no receipt and is refused.
    const receipt = /^%%\s*receipt\s+(.+)$/.exec(line);
    const holder = stack.at(-1);
    if (receipt && holder !== root && !holder.receipt && holder.parts.length === 1
      && !holder.parts[0].children.length && !holder.parts[0].transitions.length) {
      holder.receipt = { item: receipt[1].trim() };
      item = null;
      continue;
    }
    // Declared events: "%% event LABEL | by CAUSE | command on ITEM: VERB | needs [not] Id, Id | INTENT"
    // (command and needs optional).
    const event = /^%%\s*event\s+(.*)$/.exec(line);
    if (event && /\|\s*by\s/.test(event[1])) {
      const fields = event[1].split("|").map((s) => s.trim());
      const [label] = fields;
      const cause = /^by\s+(.+)$/.exec(fields[1] ?? "");
      if (!label || /[[\]]/.test(label) || !cause)
        throw new Error(`an event line reads "%% event LABEL | by CAUSE | needs [not] Id, Id | INTENT": ${line}`);
      let next = 2;
      const command = /^command\s+on\s+(.+?)\s*:\s*(.+?)(?:\s*,\s*clicked\s+while\s+(.+))?$/.exec(fields[next] ?? "");
      if (command) next++;
      else if (/^command\b/.test(fields[next] ?? ""))
        throw new Error(`a command reads "| command on ITEM: VERB |" or "| command on ITEM: VERB, clicked while Ids |": ${line}`);
      const needs = /^needs\s+(not\s+)?(.+)$/.exec(fields[next] ?? "");
      if (needs) next++;
      const why = needs && /^reason:\s*(.+)$/.exec(fields[next] ?? "");
      if (why) next++;
      if (/^reason:/.test(fields[next] ?? "")) throw new Error(`a reason follows the needs it explains: ${line}`);
      const intent = fields.slice(next).join(" | ").trim();
      if (!intent) throw new Error(`the event "${label}" needs a one-sentence intent after its cause: ${line}`);
      if (events.some((e) => e.label === label)) throw new Error(`the event "${label}" is declared twice`);
      events.push({ label, cause: cause[1].trim(), intent,
        ...(command ? { command: { item: command[1].trim(), verb: command[2].trim(),
          ...(command[3] ? { client: command[3].split(",").map((s) => s.trim()) } : {}) } } : {}),
        ...(needs ? { needs: { not: Boolean(needs[1]), ids: needs[2].split(",").map((s) => s.trim()),
          ...(why ? { reason: why[1].trim() } : {}) } } : {}) });
      item = null;
      continue;
    }
    // A cause's standing limit: "%% cause CAUSE | needs [not] Id, Id | reason: TEXT" holds for
    // every event CAUSE causes, as its own condition beside each event's needs.
    const standing = /^%%\s*cause\s+(.+?)\s*\|\s*needs\s+(not\s+)?(.+?)\s*$/.exec(line);
    if (standing) {
      const [ids, why] = reasoned(standing[3]);
      if (causes.has(standing[1])) throw new Error(`the cause "${standing[1]}" is declared twice`);
      causes.set(standing[1], { not: Boolean(standing[2]), ids: ids.split(",").map((s) => s.trim()), ...(why ? { reason: why } : {}) });
      item = null;
      continue;
    }
    if (/^%%\s*events\s+declared\s*$/.test(line)) { strict = true; item = null; continue; }
    const group = /^%%\s*events\s+(.+?)\s*=\s*(.+)$/.exec(line);
    if (group) {
      if (groups.has(group[1])) throw new Error(`the event group "${group[1]}" is declared twice`);
      groups.set(group[1], group[2].split("|").map((s) => s.trim()).filter(Boolean));
      item = null;
      continue;
    }
    const limit = /^%%\s*(only|except)\s+([A-Za-z0-9_]+(?:\s*,\s*[A-Za-z0-9_]+)*)\s*$/.exec(line);
    if (limit) {
      if (!item) throw new Error(`a %% ${limit[1]} line must come right after a state or transition: ${line}`);
      if (!template()) throw new Error(`%% ${limit[1]} limits a state or step to template instances, but it is outside any template: ${line}`);
      item.limit = { only: limit[1] === "only", keys: limit[2].split(",").map((s) => s.trim()) };
      item = null;
      continue;
    }
    if (KEYWORDS.test(line)) ignored.push(line);
    item = null;
    if (
      !line ||
      line.startsWith("%%") ||
      /^(stateDiagram|direction|classDef|class |note)/.test(line)
    )
      continue;
    let m = /^state\s+"([^"]*)"\s+as\s+([A-Za-z_]\w*)\s*(\{)?\s*$/.exec(line);
    if (!m) {
      const plain = /^state\s+([A-Za-z_]\w*)\s*(\{)?\s*$/.exec(line);
      if (plain) m = [line, null, plain[1], plain[2]];
    }
    if (m) {
      if (m[1] !== null && !names.has(m[2])) names.set(m[2], unwrap(m[1]));
      declare(m[2]);
      if (m[3]) {
        const node = { name: m[2], parts: [newPart()] };
        part().children.push(node);
        stack.push(node);
        item = node;
      } else part().children.push((item = { name: m[2], leaf: true }));
      continue;
    }
    if (line === "}") {
      if (stack.length === 1) throw new Error("unmatched } in the chart");
      stack.pop();
      continue;
    }
    if (line === "--") {
      stack[stack.length - 1].parts.push(newPart());
      continue;
    }
    m = /^(\[\*\]|[A-Za-z_]\w*)\s*-->\s*(\[\*\]|[A-Za-z_]\w*)(?:\s*:\s*(.*))?$/.exec(line);
    if (m) {
      if (m[2] === "[*]" && m[1] !== "[*]")
        throw new Error(
          `cannot read final state in "${line}"; draw the step to the state the reader sees next`,
        );
      mention(m[1]);
      mention(m[2]);
      const said = m[3] ? unwrap(m[3]) : "";
      last = item = { from: m[1], to: m[2], text: said, ...labelParts(said) };
      part().transitions.push(last);
      continue;
    }
    m = /^([A-Za-z_]\w*)\s*:\s*(.*)$/.exec(line);
    if (m) {
      if (!names.has(m[1])) names.set(m[1], unwrap(m[2]));
      if (implicit.get(m[1]) === null) continue;
      declare(m[1]);
      part().children.push((item = { name: m[1], leaf: true }));
      continue;
    }
    throw new Error(`cannot read chart line: ${line}`);
  }
  if (stack.length !== 1) throw new Error("the chart has an unclosed {");
  if (root.parts.length > 1)
    throw new Error(
      "the chart has -- outside a composite; to make parallel regions, split one top-level composite with --",
    );
  return { root, names, nevers, ignored, events, causes, groups, strict };
}

// Checks that need the final (expanded) tree.
export function validate(root) {
  (function check(q) {
    for (const p of q.parts) {
      for (const t of p.transitions) {
        if (t.words && !t.when)
          throw new Error(`guard words on "${t.label}" need a %% when annotation; prose is not an executable guard`);
        // A labeled start ([*] --> X : event) is checked where it is allowed: in a parallel part.
        if (t.from === "[*]" && t.when)
          throw new Error("an initial arrow cannot have a guard or event label");
        if (t.from === "[*]" && t.intent) throw new Error(`an initial arrow cannot have an effect: [*] --> ${t.to} : ${t.text}`);
      }
      for (const c of p.children) if (!c.leaf) check(c);
    }
  })(root);
}
