"""Assemble app proof inputs from acquired Hint captures and the prepared FHIR selection."""
import argparse
import hashlib
import json
from pathlib import Path
import shutil

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--hint-captures', type=Path, required=True)
parser.add_argument('--selection', type=Path, required=True)
parser.add_argument('--output', type=Path, required=True)
args = parser.parse_args()
args.output.mkdir(parents=True, exist_ok=True)
if any(args.output.iterdir()):
    raise SystemExit('App input output must be empty')
(args.output / 'hint').mkdir()
for i in range(4):
    source = args.hint_captures / f'worker-0-{i}.json'
    shutil.copyfile(source, args.output / 'hint' / source.name)
shutil.copytree(args.selection / 'mimic', args.output / 'mimic')
shutil.copyfile(args.selection / 'mimic-expected.json', args.output / 'mimic-expected.json')
files = {str(p.relative_to(args.output)): hashlib.sha256(p.read_bytes()).hexdigest()
         for p in sorted(args.output.rglob('*')) if p.is_file()}
(args.output / 'input-sha256.json').write_text(json.dumps(files, indent=2) + '\n')
print(f'Assembled {len(files)} input files; SHA256 receipts in {args.output}/input-sha256.json')
