// Expand authoring constructs in the syntax tree before flattening, so every backend
// sees ordinary regions, states and labeled transitions.
//
// Templates: a composite marked `%% template` is one region written once. Each
// `%% instance KEY: name = value, ...` line makes one copy, with {name} (and {key})
// substituted in captions, labels, guard words, effects, %% when ids and %% never lines.
// Template-local ids become `Id@KEY`, the form other %% lines use to name an instance.
//
// Declared events (see expandEvents) attach each event's needs to every edge carrying it.
import { labelParts, validate } from "./chart-parse.mjs";

const PLACEHOLDER = /\{([A-Za-z_]\w*)\}/g;
export const placeholders = (text) => [...new Set([...text.matchAll(PLACEHOLDER)].map((m) => m[1]))];
export const substitute = (text, values) => text.replace(PLACEHOLDER, (whole, name) => Object.hasOwn(values, name) ? values[name] : whole);

// One step bound to one set of values: its words, effect, %% when ids and reason. Template
// copies, event families and event groups all bind steps here, so none of them skips a field.
function bindStep(x, values, rename = (id) => id) {
  const text = substitute(x.text, values);
  return {
    from: rename(x.from), to: rename(x.to), text, ...labelParts(text),
    ...(x.when ? { when: { ...x.when, ids: x.when.ids.map((id) => rename(substitute(id, values))),
      ...(x.when.reason ? { reason: substitute(x.when.reason, values) } : {}) } } : {}),
  };
}

export function expand(parsed, options = {}) {
  const { root, names } = parsed;
  const instances = []; // { template, key, values }
  const local = new Map(); // template-local id -> template name
  let nevers = parsed.nevers;
  // Each %% use of a shape is a one-instance template; %% only and %% except inside the
  // shape may name any use of it.
  const shapeKeys = new Map();
  (function scan(q) {
    for (const p of q.parts) for (const c of p.children) {
      if (c.leaf) continue;
      if (c.template?.shape) {
        const keys = shapeKeys.get(c.template.shape) ?? new Set();
        for (const { key } of c.template.instances) {
          if (keys.has(key)) throw new Error(`two %% use lines of the shape ${c.template.shape} are named ${key}`);
          keys.add(key);
        }
        shapeKeys.set(c.template.shape, keys);
      }
      scan(c);
    }
  })(root);

  function expandIn(node) {
    for (const p of node.parts) for (const c of p.children) if (!c.leaf && !c.template) expandIn(c);
    const parts = [];
    for (const p of node.parts) {
      const t = p.children.find((c) => c.template);
      if (!t) { parts.push(p); continue; }
      if (node === root)
        throw new Error(`template ${t.name} must be a region: put it inside a composite, alone in its part`);
      if (p.children.length !== 1 || !p.transitions.every((x) => x.from === "[*]" && x.to === t.name))
        throw new Error(`template ${t.name} must be the only state in its part, entered only by [*]`);
      if (!t.template.instances.length) throw new Error(`template ${t.name} has no %% instance lines`);
      const ids = new Set();
      (function collect(q) {
        ids.add(q.name);
        if (!q.leaf) for (const sub of q.parts) for (const c of sub.children) collect(c);
      })(t);
      for (const id of ids) local.set(id, t.name);
      const keys = shapeKeys.get(t.template.shape) ?? new Set(t.template.instances.map((i) => i.key));
      const check = (limit, what) => {
        for (const k of limit?.keys ?? [])
          if (!keys.has(k)) throw new Error(`%% ${limit.only ? "only" : "except"} on ${what} names ${k}, which is not ${t.template.shape ? `a use of the shape ${t.template.shape}` : `an instance of template ${t.name}`}`);
      };
      for (const { key, values } of t.template.instances) {
        instances.push({ template: t.name, key, values });
        const rename = (id) => (ids.has(id) ? `${id}@${key}` : id);
        const keeps = (limit) => !limit || limit.keys.includes(key) === limit.only;
        const dropped = new Set();
        const copy = (q) => {
          check(q.limit, q.name);
          if (!keeps(q.limit)) {
            (function drop(x) { dropped.add(x.name); if (!x.leaf) for (const sub of x.parts) for (const c of sub.children) drop(c); })(q);
            return null;
          }
          const caption = substitute(names.get(q.name) ?? q.name, values);
          names.set(rename(q.name), caption);
          if (q.leaf) return { name: rename(q.name), leaf: true };
          // A parallel region whose states all drop for this instance is left out of it.
          const parts = q.parts.map((sub) => ({
            children: sub.children.map(copy).filter(Boolean),
            transitions: sub.transitions.filter((x) => { check(x.limit, `${x.from} --> ${x.to}`); return keeps(x.limit); })
              .map((x) => bindStep(x, values, rename)),
          })).filter((sub, i) => sub.children.length || !q.parts[i].children.length);
          if (q.parts.length > 1 && parts.length < 2)
            throw new Error(`${q.name} keeps fewer than two parallel regions for instance ${key}; limit the composite itself instead`);
          return { name: rename(q.name), ...(q.receipt ? { receipt: { item: substitute(q.receipt.item, values) } } : {}), parts };
        };
        const clone = copy(t);
        if (!clone) throw new Error(`template ${t.name} drops its own region for instance ${key}`);
        // Steps that touch a state this instance leaves out drop out too.
        (function prune(q) {
          for (const sub of q.parts) {
            sub.transitions = sub.transitions.filter((x) => ![x.from, x.to].some((id) => dropped.has(id.replace(/@[^@]*$/, ""))));
            for (const c of sub.children) if (!c.leaf) prune(c);
          }
        })(clone);
        clone.instance = { template: t.name, key };
        parts.push({ children: [clone], transitions: p.transitions.map((x) => ({ ...x, to: clone.name })) });
      }
      // %% never lines written inside the template hold for each instance.
      nevers = nevers.flatMap((n) => n.template !== t ? [n] : t.template.instances.map(({ key, values }) => ({
        name: `${n.name}_${key}`,
        states: n.states.map((s) => substitute(s, values)),
        sentence: substitute(n.sentence, values),
        ...(n.reason ? { reason: substitute(n.reason, values) } : {}),
        home: { template: t.name, key },
      })));
    }
    node.parts = parts;
  }
  expandIn(root);
  const { catalog, problems } = expandEvents(parsed, instances, options);
  expandCommands(root, names, catalog);

  // Every id a step reads, after templates and events are bound: outside its template, a
  // template-local id must name its instance.
  (function refs(q) {
    for (const p of q.parts) {
      for (const t of p.transitions) {
        for (const id of [t.from, t.to])
          if (local.has(id)) throw new Error(`${id} is inside template ${local.get(id)}; outside it, name one instance as ${id}@KEY in a %% line`);
        for (const id of t.when?.ids ?? [])
          if (local.has(id)) throw new Error(`a %% when line names ${id}, which is inside template ${local.get(id)}; name one instance as ${id}@KEY`);
        for (const id of t.needsIds?.ids ?? [])
          if (local.has(id)) throw new Error(`the event "${t.label}" needs ${id}, which is inside template ${local.get(id)}; name one instance as ${id}@KEY, or ${id}@{key} in a family`);
      }
      for (const c of p.children) if (!c.leaf) refs(c);
    }
  })(root);
  for (const [id, caption] of names)
    if (id.includes("@") && placeholders(caption).length)
      throw new Error(`the caption "${caption}" has a placeholder its instance does not bind`);

  validate(root);
  return { ...parsed, nevers, instances, catalog, problems };
}

// Commands: a declared event with "command on ITEM: VERB" is a request the system accepts
// exactly while its needs hold; "clicked while Ids" adds what only the click needs (the
// client's own state). The %% receipt composite for ITEM becomes one receipt for all of ITEM's commands:
// idle; per command, sending (a lost answer is resent with the same request, so it stays
// sending) and couldn't confirm. The click moves only the receipt, so the item's other
// commands wait. Every other step drawn on the click reacts to "your VERB on ITEM goes
// through" instead, so the item changes only on acceptance; "is refused" fires exactly when
// the needs fail and changes nothing else. Every outcome needs its command in flight, so no
// region reacts to an answer nobody asked for.
function expandCommands(root, names, catalog) {
  const receipts = new Map(); // item -> composite
  (function find(q) {
    for (const p of q.parts) for (const c of p.children) {
      if (c.leaf) continue;
      if (c.receipt) {
        if (receipts.has(c.receipt.item)) throw new Error(`two %% receipt composites fill "${c.receipt.item}"`);
        receipts.set(c.receipt.item, c);
      } else find(c);
    }
  })(root);
  const commands = new Map(); // item -> [{ click, verb, needs }]
  for (const [label, { decl, values }] of catalog) {
    if (!decl.command) continue;
    const item = substitute(decl.command.item, values), verb = substitute(decl.command.verb, values);
    if (!receipts.has(item)) throw new Error(`the command "${label}" is on ${item}, which no %% receipt composite fills`);
    const needs = decl.needs && { not: decl.needs.not, ids: decl.needs.ids.map((id) => substitute(id, values)),
      ...(decl.needs.reason ? { reason: substitute(decl.needs.reason, values) } : {}) };
    const client = decl.command.client && { not: false, ids: decl.command.client.map((id) => substitute(id, values)),
      reason: "the control shows only while the client's own state allows it" };
    if (needs?.not) throw new Error(`the command "${label}" names what it needs, not what it must avoid`);
    if (!commands.has(item)) commands.set(item, []);
    commands.get(item).push({ click: label, verb, needs, client });
  }
  const accepted = new Map(); // click label -> { through, sending }
  const generated = new Set();
  for (const [item, node] of receipts) {
    if (node.parts.length > 1 || node.parts[0].children.length || node.parts[0].transitions.length)
      throw new Error(`the %% receipt composite for ${item} holds nothing else; the tool fills it`);
    const list = commands.get(item);
    if (!list) throw new Error(`the %% receipt for ${item} has no command declared on it`);
    const [base, key] = node.name.split("@");
    const id = (slug) => `${base}_${slug}${key ? `@${key}` : ""}`;
    const state = (slug, caption) => { names.set(id(slug), caption); return { name: id(slug), leaf: true }; };
    const declare = (label, decl) => {
      if (catalog.has(label)) throw new Error(`the event "${label}" is declared twice`);
      catalog.set(label, { decl: { label, ...decl }, values: {} });
    };
    const idle = id("idle");
    const children = [state("idle", `No command on ${item} in flight`)];
    const transitions = [{ from: "[*]", to: idle, text: "", label: "" }];
    const slugs = new Set();
    for (const { click, verb, needs, client } of list) {
      let slug = verb.toLowerCase().replace(/[^a-z0-9]+/g, "_").replace(/^_|_$/g, "") || "command";
      while (slugs.has(slug)) slug += "_";
      slugs.add(slug);
      const sending = id(`${slug}_sending`), stuck = id(`${slug}_stuck`);
      children.push(state(`${slug}_sending`, `${verb} on ${item}: sending; other commands on it wait`),
        state(`${slug}_stuck`, `Couldn't confirm ${verb} on ${item}, with Check again`));
      const through = `your ${verb} on ${item} goes through`, refused = `your ${verb} on ${item} is refused`;
      const lost = `your ${verb} on ${item} can't be confirmed`, again = `check your ${verb} on ${item} again`;
      const inFlight = (at, also, why = "an answer comes only for a command in flight") =>
        ({ not: false, ids: [at, ...(also?.ids ?? [])], reason: also?.reason ? `${also.reason}; and ${why}` : why });
      declare(through, { cause: "the system", accepts: click, needs: inFlight(sending, needs),
        intent: `Accepted while the command is still allowed, directly or after the same request is resent; only now does ${item} change.` });
      declare(refused, { cause: "the system", needs: inFlight(sending),
        intent: `${item[0].toUpperCase()}${item.slice(1)} no longer allows it when it arrives; nothing changes, and the view refreshes and says why.` });
      declare(lost, { cause: "the client", needs: inFlight(sending), intent: "The same request was resent and still has no answer; you are asked, with Check again." });
      declare(again, { cause: "you", needs: inFlight(stuck, null, "Check again shows only when an answer couldn't be confirmed"), intent: "The same request is resent; it can never be applied twice." });
      const step = (from, to, label, extra = {}) => ({ from, to, text: label, label, ...extra });
      transitions.push(
        step(idle, sending, click, { ...(needs ? { needsIds: needs } : {}), ...(client ? { when: client } : {}) }),
        step(sending, idle, through, needs ? { needsIds: needs } : {}),
        step(sending, idle, refused, needs ? { refuseIds: needs } : {}),
        step(sending, stuck, lost),
        step(stuck, sending, again));
      accepted.set(click, { through, sending, inFlight });
    }
    node.parts = [{ children, transitions }];
    for (const t of transitions) generated.add(t);
  }
  (function relabel(q) {
    for (const p of q.parts) {
      for (const t of p.transitions) {
        const command = accepted.get(t.label);
        if (!command || generated.has(t)) continue;
        const { through, sending, inFlight } = command;
        Object.assign(t, { label: through, text: t.text.replace(t.label, through), needsIds: inFlight(sending, t.needsIds) });
      }
      for (const c of p.children) if (!c.leaf) relabel(c);
    }
  })(root);
}

// Declared events. A declaration whose label has {placeholders} is a family: its members
// are the labels that template instances binding every placeholder produce. An edge whose
// label is a group name, or a family pattern no enclosing template bound, becomes one edge
// per member (substituting that member's values into its effect and %% when ids). Each
// edge then carries its event's needs, written as %% when ids and resolved after flattening.
function expandEvents({ root, events, groups, strict }, instances, { lenient = false } = {}) {
  const problems = [];
  const problem = (message) => { if (!lenient) throw new Error(message); problems.push(message); };
  const bindings = instances.map((i) => i.values);
  const members = (pattern) => {
    const wanted = placeholders(pattern);
    if (!wanted.length) return [{ label: pattern, values: {} }];
    const found = new Map();
    for (const values of bindings)
      if (wanted.every((p) => Object.hasOwn(values, p))) {
        const label = substitute(pattern, values);
        if (!found.has(label)) found.set(label, { label, values });
      }
    return [...found.values()];
  };
  const catalog = new Map(); // concrete event label -> { decl, values }
  for (const decl of events) {
    if (groups.has(decl.label)) throw new Error(`"${decl.label}" is both an event and an event group`);
    for (const m of members(decl.label)) {
      if (catalog.has(m.label)) throw new Error(`the event "${m.label}" is declared twice`);
      catalog.set(m.label, { decl, values: m.values });
    }
  }
  const edge = (t) => {
    if (!t.label) {
      if (strict && t.from !== "[*]") problem(`strict events: the step ${t.from} --> ${t.to} has no event label`);
      return [t];
    }
    // A group stands for its members; a family pattern for the instances that bind it.
    const bound = groups.has(t.label) ? groups.get(t.label).flatMap(members) : members(t.label);
    // A pattern nothing binds stays one step, so the check below names it.
    const copies = bound.length ? bound.map(({ label, values }) => ({ ...t, ...bindStep(t, values), label })) : [t];
    // Nothing a step says may keep a placeholder once templates and events are bound.
    for (const c of copies) {
      const left = [c.label, c.intent ?? "", ...(c.when?.ids ?? [])].flatMap(placeholders);
      if (left.length) problem(`the step ${c.from} --> ${c.to} on "${c.label}" leaves {${[...new Set(left)].join("}, {")}} unbound: no template instance binds it`);
    }
    for (const c of copies) {
      const known = catalog.get(c.label);
      if (!known) { if (strict) problem(`strict events: "${c.label}" is not declared with %% event`); continue; }
      if (known.decl.needs && c.from !== "[*]")
        c.needsIds = { not: known.decl.needs.not, ids: known.decl.needs.ids.map((id) => substitute(id, known.values)),
          ...(known.decl.needs.reason ? { reason: substitute(known.decl.needs.reason, known.values) } : {}) };
    }
    return copies;
  };
  (function walk(q) {
    for (const p of q.parts) {
      p.transitions = p.transitions.flatMap(edge);
      for (const c of p.children) if (!c.leaf) walk(c);
    }
  })(root);
  return { catalog, problems };
}
