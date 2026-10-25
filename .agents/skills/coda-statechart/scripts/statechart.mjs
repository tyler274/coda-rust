#!/usr/bin/env node
// One entry for the chart tools: node statechart.mjs <command> ... (see --help).
import fs from "node:fs";
import process from "node:process";
import { pathToFileURL } from "node:url";
import { HELP, USAGE, more, usage } from "./statechart-help.mjs";
import { compile, writeModel } from "./chart-verus.mjs";
import { printSearch, search } from "./chart-search.mjs";
import { printFlat } from "./chart-flat.mjs";
import { lint, queryEvent, queryRegion, queryState, summary } from "./chart-query.mjs";
import { checkRules, printRules } from "./chart-rules.mjs";
import { compareCharts, printComparison, readRenames } from "./compare-charts.mjs";
import { against, printAgainst, readMap } from "./chart-against.mjs";
import { coverage, printCoverage } from "./chart-coverage.mjs";

class Usage extends Error {}

const positive = (name, value) => {
  if (!/^[1-9][0-9]*$/.test(value) || !Number.isSafeInteger(Number(value)))
    throw new Usage(`${name} needs a positive safe integer`);
  return Number(value);
};
const moduleName = (flags, required) => {
  if (flags.module === undefined && !required) return "Chart";
  if (!/^[A-Za-z][A-Za-z0-9_]*$/.test(flags.module ?? ""))
    throw new Usage("provide --module NAME (letters, digits, underscores)");
  return flags.module;
};
const read = (file) => fs.readFileSync(file, "utf8");

export const COMMANDS = {
  verus: {
    inputs: 1, values: ["--module", "--dir"],
    run: ([chart], flags) => { writeModel({ chart, module: moduleName(flags, true), dir: flags.dir }); return 0; },
  },
  check: {
    inputs: 1, values: ["--module", "--dir"],
    run: ([chart], flags) => { writeModel({ chart, module: moduleName(flags, true), dir: flags.dir, check: true }); return 0; },
  },
  trace: {
    inputs: 1, values: ["--module", "--max-states"],
    run: ([chart], flags) => {
      const limit = flags["max-states"] === undefined ? 100000 : positive("--max-states", flags["max-states"]);
      const { ir } = compile(read(chart), moduleName(flags, false));
      return printSearch(search(ir, limit), ir);
    },
  },
  rules: {
    inputs: 1, values: ["--rule", "--max-cone", "--max-states"],
    run: ([chart], flags) => {
      const maxCone = flags["max-cone"] === undefined ? 12 : positive("--max-cone", flags["max-cone"]);
      const maxStates = flags["max-states"] === undefined ? 100000 : positive("--max-states", flags["max-states"]);
      const { ir } = compile(read(chart), "Chart");
      return printRules(checkRules(ir, { rule: flags.rule, maxCone, maxStates }), ir);
    },
  },
  compare: {
    inputs: 2, values: ["--rename"],
    run: ([a, b], flags) => {
      const renames = flags.rename === undefined ? undefined : readRenames(read(flags.rename));
      return printComparison(compareCharts(read(a), read(b), renames));
    },
  },
  against: {
    inputs: 2, values: ["--map", "--out"],
    run: ([detail, agreed], flags) => {
      if (flags.map === undefined) throw new Usage("provide --map MAP.json");
      const result = against(read(detail), read(agreed), readMap(read(flags.map)));
      if (flags.out !== undefined) fs.writeFileSync(flags.out, result.layered);
      return printAgainst(result);
    },
  },
  coverage: {
    inputs: 2,
    run: ([chart, table]) => printCoverage(coverage(read(chart), read(table))),
  },
  flat: {
    inputs: 1,
    run: ([chart]) => {
      const { parsed, ir } = compile(read(chart), "Chart");
      process.stdout.write(printFlat(parsed, ir.rules));
      return 0;
    },
  },
  query: {
    inputs: 1, values: ["--state", "--event", "--region"],
    run: ([chart], flags) => {
      const asked = ["state", "event", "region"].filter((k) => flags[k] !== undefined);
      if (asked.length > 1) throw new Usage("ask about one --state, --event or --region at a time");
      const compiled = compile(read(chart), "Chart", { lenient: true });
      const [kind] = asked;
      const answer = kind === "state" ? queryState(compiled, flags.state)
        : kind === "event" ? queryEvent(compiled, flags.event)
          : kind === "region" ? queryRegion(compiled, flags.region) : summary(compiled);
      console.log(answer);
      return 0;
    },
  },
  lint: {
    inputs: 1, values: ["--cause"], switches: ["--repeats"],
    run: ([chart], flags) => {
      const findings = lint(compile(read(chart), "Chart", { lenient: true }), { cause: flags.cause, repeats: flags.repeats });
      for (const f of findings) console.log(f);
      console.log(findings.length ? `${findings.length} finding${findings.length === 1 ? "" : "s"}.` : "No findings.");
      return findings.length ? 1 : 0;
    },
  },
};

export function parseArgs([command, ...rest]) {
  if (command === undefined || command === "-h" || command === "--help") return { help: HELP[""] };
  if (command === "help") return { help: HELP[rest[0] ?? ""] ?? HELP[""] };
  if (!Object.hasOwn(COMMANDS, command))
    throw new Usage(`unknown command ${command}\n${usage(Object.values(USAGE))}\n${more()}`);
  const { values = [], switches = [], inputs } = COMMANDS[command];
  const flags = {};
  const files = [];
  for (let i = 0; i < rest.length; i++) {
    const a = rest[i];
    if (a === "-h" || a === "--help") return { help: HELP[command] };
    if (switches.includes(a)) flags[a.slice(2)] = true;
    else if (values.includes(a)) {
      if (i + 1 === rest.length || rest[i + 1].startsWith("--")) throw new Usage(`${a} needs a value. ${more(command)}`);
      if (flags[a.slice(2)] !== undefined) throw new Usage(`${a} supplied twice`);
      flags[a.slice(2)] = rest[++i];
    } else if (!a.startsWith("-") && files.length < inputs) files.push(a);
    else throw new Usage(`unknown argument ${a}. ${more(command)}`);
  }
  if (files.length !== inputs) throw new Usage(`${usage([USAGE[command]])}\n${more(command)}`);
  return { command, files, flags };
}

export function main(argv) {
  let parsed;
  try { parsed = parseArgs(argv); }
  catch (error) {
    if (!(error instanceof Usage)) throw error;
    console.error(error.message);
    return 2;
  }
  if (parsed.help) { console.log(parsed.help); return 0; }
  try { return COMMANDS[parsed.command].run(parsed.files, parsed.flags); }
  catch (error) {
    if (error instanceof Usage) { console.error(`${error.message}\n${more(parsed.command)}`); return 2; }
    console.error(error.message);
    return 1;
  }
}

if (process.argv[1] && import.meta.url === pathToFileURL(fs.realpathSync(process.argv[1])).href)
  process.exitCode = main(process.argv.slice(2));
