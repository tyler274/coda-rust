// A journey model for verification/statechart.mjs: regions that each hold one state at a time, and
// events whose ordered cases read like the code's own steps. On an event, the first case whose `when`
// holds applies, and `set` gives each region it changes either a state or a map from the region's
// current state to its next; regions a case does not set stay. The event happens when that case
// changes or keeps some region. Guards are is, not, all and any: what Verus can state.

export const is = (region, ...states) => states.length === 1
  ? { in: [region, states[0]] }
  : { or: states.map((state) => ({ in: [region, state] })) };
export const not = (guard) => ({ not: guard });
export const all = (...guards) => {
  guards = guards.filter(Boolean);
  return guards.length === 0 ? undefined : guards.length === 1 ? guards[0] : { and: guards };
};
export const any = (guards) => ({ or: guards });

// Each region lists its states, the first one initial. `never` names states that must never hold
// together; `invariant` names facts, as guards, that hold in every state and explain why the rules
// do; `reach` names, in order, states the model must pass through. `controls` gives each rule
// sentence a change that breaks it, the way a plausible mistake would: a check never seen to fail
// proves nothing, so `check` makes each change and requires the sentence's rule to break.
export function model({ id, regions, events, never = [], invariant = [], reach = [], controls = [] }) {
  const has = (region, state, where) => {
    if (!regions[region]) throw new Error(`${where}: no region ${region}`);
    if (!regions[region].includes(state)) throw new Error(`${where}: ${region} has no state ${state}`);
  };
  const guard = (g, where) => {
    if (g === undefined) return;
    if (g.in) return has(g.in[0], g.in[1], where);
    if (g.not) return guard(g.not, where);
    const list = g.and ?? g.or;
    if (!Array.isArray(list)) throw new Error(`${where}: a guard is not is, not, all or any`);
    list.forEach((x) => guard(x, where));
  };
  const states = (map, where) => {
    for (const [region, list] of Object.entries(map)) for (const state of [list].flat()) has(region, state, where);
  };
  for (const [event, { cases }] of Object.entries(events))
    cases.forEach((c, i) => {
      const where = `${event} case ${i + 1}`;
      if (cases.slice(0, i).some((e) => !e.when)) throw new Error(`${where} follows a case that always applies`);
      guard(c.when, where);
      for (const [region, effect] of Object.entries(c.set))
        for (const [from, to] of typeof effect === 'string' ? [[undefined, effect]] : Object.entries(effect)) {
          if (from !== undefined) has(region, from, where);
          has(region, to, where);
        }
    });
  never.forEach((rule) => states(rule.states, `rule ${rule.name}`));
  invariant.forEach((fact) => guard(fact.holds, `fact ${fact.name}`));
  reach.forEach((outcome) => outcome.through.forEach((map) => states(map, `outcome ${outcome.name}`)));
  const sentences = new Set(never.map((rule) => rule.sentence));
  for (const control of controls) {
    if (!sentences.has(control.sentence)) throw new Error(`control "${control.breaks}": no rule says "${control.sentence}"`);
    if (typeof control.change !== 'function') throw new Error(`control "${control.breaks}" has no change`);
  }
  for (const sentence of sentences)
    if (!controls.some((control) => control.sentence === sentence)) throw new Error(`no control breaks "${sentence}"`);
  return { id, regions, events, never, invariant, reach, controls };
}
