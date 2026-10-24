#!/usr/bin/env node
// Compatible wrapper for the original command line; statechart.mjs is the full CLI.
// Verus backend adapted from pstack's chart-to-tla.mjs at eb4db2b; see verification/PSTACK-LICENSE.
import fs from "node:fs";
import process from "node:process";
import { compile, writeModel } from "./chart-verus.mjs";
import { printSearch, search } from "./chart-search.mjs";

const HELP = `Usage: node chart-to-verus.mjs CHART.mmd --module NAME [--dir OUT] [--check | --trace] [--max-states N]

Compile pstack's supported Mermaid stateDiagram-v2 subset to Verus.
Writes NAME.rs (generated behavior), NAME_steps.rs (one generated preservation
lemma per event), NAME.map.json (diagram correspondence), and NAME_proof.rs
(editable proof starter, created only when absent).
Existing generated files may be refreshed; hand-edited generated files are refused.
--check writes nothing and fails if generated files differ from the chart.
--trace searches this same finite chart without writing files; prints the shortest
violating path and a numbered Mermaid diagram. --max-states defaults to 100000.
An exhausted search covers only chart rules, not data proofs or progress.

Same as node statechart.mjs verus | check | trace. statechart.mjs also has flat,
query, lint, rules and compare; run it with --help.

Exit: 0 generated/current or completed chart search; 1 violation, invalid chart,
drift or I/O error; 2 invalid arguments; 3 incomplete search.
`;

function argumentsOf(argv) {
  const args = {};
  for (let i = 0; i < argv.length; i++) {
    const arg = argv[i];
    if (arg === "--help" || arg === "-h") return { help: true };
    if (arg === "--check") { args.check = true; continue; }
    if (arg === "--trace") { args.trace = true; continue; }
    if (arg === "--module" || arg === "--dir" || arg === "--max-states") {
      if (!argv[i + 1] || argv[i + 1].startsWith("-")) throw new Error(`${arg} needs a value`);
      const key = arg.slice(2);
      if (args[key]) throw new Error(`${arg} supplied twice`);
      args[key] = argv[++i];
    } else if (!arg.startsWith("-") && !args.chart) args.chart = arg;
    else throw new Error(`unknown argument ${arg}`);
  }
  if (!args.chart || !/^[A-Za-z][A-Za-z0-9_]*$/.test(args.module ?? ""))
    throw new Error("provide CHART.mmd and --module NAME (letters, digits, underscores)");
  if (args.check && args.trace) throw new Error("choose --check or --trace");
  if (args["max-states"] !== undefined && (!args.trace || !/^[1-9][0-9]*$/.test(args["max-states"]) || !Number.isSafeInteger(Number(args["max-states"]))))
    throw new Error("--max-states needs --trace and a positive safe integer");
  return args;
}

let args;
try { args = argumentsOf(process.argv.slice(2)); }
catch (error) { console.error(`${error.message}\n${HELP.split("\n")[0]}`); process.exit(2); }
if (args.help) { console.log(HELP); process.exit(0); }
try {
  if (args.trace) {
    const { ir } = compile(fs.readFileSync(args.chart, "utf8"), args.module);
    process.exitCode = printSearch(search(ir, Number(args["max-states"] ?? 100000)), ir);
  } else writeModel({ chart: args.chart, module: args.module, dir: args.dir, check: args.check });
} catch (error) { console.error(error.message); process.exitCode = 1; }
