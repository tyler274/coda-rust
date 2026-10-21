-- duet-release: maintenance
begin;
set local lock_timeout = '5s';
set local statement_timeout = '60s';

-- Earlier native adapters retained the original FHIR body but omitted its
-- compact display metadata. Recover only literal fields in that retained body;
-- the body and its digest remain intact. This repairs the current compact
-- projection, not a source version. Retained versions and read receipts keep
-- exactly the bodies that earlier runs received.
alter table public.health_evidence
    disable trigger health_evidence_retain_revision,
    disable trigger health_evidence_update_revision;

with originals as (
    select id, case when (payload#>>'{source_record,content}') is json object
                   then (payload#>>'{source_record,content}')::jsonb end as resource
    from public.health_evidence
    where payload#>>'{source_record,format}'='fhir-json'
), metadata as (
    select id, jsonb_build_object('statuses',coalesce((select jsonb_object_agg(key,value)
                        from jsonb_each(resource)
                        where key in ('status','clinicalStatus','verificationStatus','docStatus')),
                       '{}'::jsonb)) ||
               case when resource->>'resourceType'='Immunization'
                    then jsonb_build_object('vaccine_code',resource->'vaccineCode')
                    else '{}'::jsonb end as value
    from originals where resource is not null
)
update public.health_evidence r
set payload=jsonb_set(r.payload,'{source}',(r.payload->'source') || metadata.value),updated_at=now()
from metadata where r.id=metadata.id
  and (r.payload->'source') is distinct from ((r.payload->'source') || metadata.value);

alter table public.health_evidence
    enable trigger health_evidence_retain_revision,
    enable trigger health_evidence_update_revision;

-- Every library list built before this release hides medications by the
-- status their source gave, retitles vaccines with the catalog's label and
-- names its disagreement flag `condition_status_conflict`, yet reads as
-- current. Rebuild every account's list through the existing browse
-- generation; reads say refreshing until the recovery sweep has. Maintenance:
-- a serving release would rebuild the lists again in the old shape.
update public.evidence_browse_generations
set generation = generation + 1,
    minimum_readable_generation = generation + 1;

insert into public.schema_migrations(version) values ('0114_rebuild_library_status')
on conflict do nothing;
commit;
