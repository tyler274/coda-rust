import argparse,collections,gzip,hashlib,json
from pathlib import Path
parser = argparse.ArgumentParser(description="Prepare the fixed local Hint/MIMIC research sample; never copy connection or token metadata.")
parser.add_argument("--hint-checkpoint", type=Path, required=True, help="Saved sandbox checkpoint containing the selected patient and all twelve section responses")
parser.add_argument("--mimic-corpus", type=Path, required=True, help="MIMIC-IV demo on FHIR 2.1.0 directory containing original .ndjson.gz files")
parser.add_argument("--research-cases", type=Path, required=True, help="Original mapping research manifest containing the fixed 57 public MIMIC cases")
parser.add_argument("--output", type=Path, required=True, help="Fresh local directory for response bodies, selected records and provenance")
parser.add_argument("--hint-capture", type=Path, required=True, help="Reader-produced capture for the same archived Hint chart; acquisition is external")
args = parser.parse_args()
out=args.output
out.mkdir(parents=True, exist_ok=True)
if (out/'mimic').exists():
    raise SystemExit("Use a fresh output directory; fixture inputs must not be appended twice")
capture=args.hint_checkpoint
x=json.loads(capture.read_text());pid='sbx-pat-QItKCySDsdET'
# Only response bodies cross into replay: no connection, auth, or receipt metadata.
requests=[r for r in x['requests'] if r.get('patient_id')==pid and r.get('kind')=='chart']
assert len(requests)==1, 'Select one acquisition; never merge chart requests'
request_id=requests[0]['id']
bodies={j['section']:j['result']['body'] for j in x['jobs'] if j.get('request_id')==request_id and j.get('section') and j.get('result')}
(out/'hint-bodies.json').write_text(json.dumps(bodies))
filed_capture=json.loads(args.hint_capture.read_text())
assert filed_capture['request']['patient_id']==pid
(out/'hint-replayed.json').write_bytes(args.hint_capture.read_bytes())
corpus=args.mimic_corpus
cases=[json.loads(l) for l in args.research_cases.read_text().splitlines()]
cases=[c for c in cases if c['origin']['kind']=='public_mimic_demo_record']
assert len(bodies)==12, 'Expected all twelve saved sandbox sections'
assert len(cases)==57, 'Expected the frozen 57 unmodified MIMIC research cases'
# Final agreed policy, independent of production code and superseding experiment labels:
# local categories/exam/survey -> Other; standard lab/vitals -> Results/Vitals.
kind={'MedicationRequest':'medications','MedicationAdministration':'medications','MedicationDispense':'medications','MedicationStatement':'medications','Condition':'problems','Encounter':'visits','Procedure':'procedures','Specimen':'other'}
rows=[];wanted=collections.defaultdict(dict)
for c in cases:
 r=c['record'];bucket=kind.get(r['resourceType'],'other')
 if c['origin']['file'] in ['MimicObservationLabevents.ndjson.gz','MimicObservationMicroTest.ndjson.gz']:bucket='results'
 if c['origin']['file']=='MimicObservationVitalSignsED.ndjson.gz':bucket='vitals'
 wanted[c['origin']['file']][c['origin']['line']]=(c,bucket)
patients=set();references=collections.defaultdict(set)
def save(raw,bucket,origin,patient_id,case):
 r=json.loads(raw);p=out/'mimic'/'patients'/patient_id;p.mkdir(parents=True,exist_ok=True)
 with (p/(r['resourceType']+'.ndjson')).open('a') as f:f.write(raw.rstrip('\n')+'\n')
 rows.append(dict(patient=patient_id,native=r['resourceType']+'/'+r['id'],bucket=bucket,raw=raw.rstrip('\n'),origin=origin,case=case))
for filename,selected in wanted.items():
 with gzip.open(corpus/filename,'rt') as f:
  for n,raw in enumerate(f,1):
   if n not in selected:continue
   c,bucket=selected[n];r=json.loads(raw);assert r==c['record']
   patient_id=r['subject']['reference'].split('/')[-1];patients.add(patient_id)
   for key in ['medicationReference','specimen']:
    ref=r.get(key,{}).get('reference')
    if ref:references[ref].add(patient_id)
   save(raw,bucket,dict(file=filename,line=n,sha256=hashlib.sha256(raw.encode()).hexdigest()),patient_id,c['id'])
# A missing/truncated input file must not silently shrink the qualification set.
assert {row['case'] for row in rows} == {case['id'] for case in cases}, 'MIMIC source selection is incomplete'
# Acquire actual patient and directly referenced drug/specimen definitions as fixture inputs.
for filename in ['MimicPatient.ndjson.gz','MimicMedication.ndjson.gz','MimicMedicationMix.ndjson.gz','MimicSpecimen.ndjson.gz','MimicSpecimenLab.ndjson.gz']:
 with gzip.open(corpus/filename,'rt') as f:
  for n,raw in enumerate(f,1):
   r=json.loads(raw);native=r['resourceType']+'/'+r['id'];targets={r['id']} & patients if r['resourceType']=='Patient' else references.get(native,set())
   for patient_id in targets:
    if any(a['patient']==patient_id and a['native']==native for a in rows):continue
    save(raw,'patient' if r['resourceType']=='Patient' else 'other',dict(file=filename,line=n,sha256=hashlib.sha256(raw.encode()).hexdigest()),patient_id,'support')
missing=[r for r in references if not any(a['native']==r for a in rows)];assert not missing,missing
(out/'mimic-expected.json').write_text(json.dumps(rows))
(out/'input-provenance.json').write_text(json.dumps(dict(mimic_release='MIMIC-IV demo on FHIR 2.1.0',mimic_release_url='https://physionet.org/content/mimic-iv-fhir-demo/2.1.0/',corpus_files_sha256={name:hashlib.file_digest((corpus/name).open('rb'),'sha256').hexdigest() for name in sorted(set(wanted)|{'MimicPatient.ndjson.gz','MimicMedication.ndjson.gz','MimicMedicationMix.ndjson.gz','MimicSpecimen.ndjson.gz','MimicSpecimenLab.ndjson.gz'})},hint_request_id=request_id,research_cases_sha256=hashlib.sha256(args.research_cases.read_bytes()).hexdigest(),hint_checkpoint_sha256=hashlib.sha256(capture.read_bytes()).hexdigest(),hint_sections=len(bodies),mimic_selected=len(cases),mimic_total=len(rows),patients=len(patients),direct_references=sorted(references)),indent=2))
print('Prepared',len(bodies),'Hint bodies and',len(rows),'unmodified MIMIC records across',len(patients),'patients')
