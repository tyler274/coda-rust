// Native/source parity check; does not invoke a browser or modify the reference.
// nix develop --command node crates/public-site/island/check-paint.mjs
import { readFileSync } from 'node:fs';
import { execFileSync } from 'node:child_process';
import { runInNewContext } from 'node:vm';
import { fileURLToPath } from 'node:url';
const manifest = fileURLToPath(new URL('./Cargo.toml', import.meta.url));
// Exact original painter slice, not SVGs emitted by the Rust implementation.
const reference = fileURLToPath(new URL('../tests/fixtures/original/public/homepage/site.js', import.meta.url));
const source = readFileSync(reference, 'utf8');
const paint = runInNewContext(source + '\npaint;');
const actual = execFileSync('cargo', ['run', '--quiet', '--locked', '--manifest-path', manifest, '--example', 'paint'], { encoding: 'utf8' }).trimEnd().split('\n');
const frames = [['mist', 7], ['sage', 21], ['sage', 31], ['sage', 32], ['sage', 33], ['dawn', 5], ['deep', 18], ['deep', 3]];
frames.forEach(([palette, seed], index) => {
  const css = paint(palette, seed);
  const expected = decodeURIComponent(css.slice(css.indexOf(',') + 1, -2));
  if (actual[index] !== expected) {
    const at = [...expected].findIndex((char, offset) => actual[index]?.[offset] !== char);
    throw new Error(`${palette}-${seed} differs at ${at}: source ${expected.slice(at, at + 80)}; Rust ${actual[index]?.slice(at, at + 80)}`);
  }
  console.log(`${palette}-${seed}: exact SVG parity (${expected.length} bytes)`);
});
