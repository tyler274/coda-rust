// Compatibility entry: read a chart and return its flat model.
// Adapted from cursor/plugins pstack at eb4db2b55b2b7dec3d9282754cddf0a56bf4ff10.
// MIT license: ../../../../verification/PSTACK-LICENSE (relative to this directory).
import { readChart } from "./chart-parse.mjs";
import { expand } from "./chart-expand.mjs";
import { flatten } from "./chart-flat.mjs";

export { LEGEND_ID } from "./chart-parse.mjs";

// parse → expand (templates, events) → flatten (hierarchy, parallel parts, guards).
// options.lenient collects strict-event problems instead of throwing (used by lint).
export function parseMermaid(text, options = {}) {
  return flatten(expand(readChart(text), options));
}
