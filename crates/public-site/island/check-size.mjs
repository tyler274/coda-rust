// Node's built-in Brotli avoids a separate CLI dependency. Counts bootstrap,
// generated glue and wasm together, not merely the wasm payload.
import { readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { brotliCompressSync, constants } from 'node:zlib';
const dir = process.argv[2];
let total = 0;
for (const name of ['bootstrap.js', 'public_site_motion.js', 'public_site_motion_bg.wasm']) {
  const data = readFileSync(join(dir, name));
  const compressed = brotliCompressSync(data, { params: { [constants.BROTLI_PARAM_QUALITY]: 11 } });
  writeFileSync(join(dir, name + '.br'), compressed);
  total += compressed.length;
  console.log(`${name}: ${data.length} raw bytes, ${compressed.length} brotli bytes`);
}
console.log(`Motion transfer total: ${total} brotli bytes (budget 60000)`);
if (total > 60000) process.exitCode = 1;
