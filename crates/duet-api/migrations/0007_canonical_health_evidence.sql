BEGIN;

DROP FUNCTION IF EXISTS public.lookup_percentile(text, double precision, integer, text);
DROP TABLE IF EXISTS public.population_reference;

-- One-time breaking cutover of retained JSONB. Runtime readers accept only
-- the canonical contract after this migration; no legacy decoder or dual
-- representation remains in application code.
DO $$
DECLARE
    row_record record;
    payload_value jsonb;
    source_value jsonb;
    context_value jsonb;
    component_values jsonb;
    concept_codings jsonb;
    category_codings jsonb;
    coding_line text;
    coding_parts text[];
    record_kind text;
    provenance text;
BEGIN
    FOR row_record IN
        SELECT id, payload
        FROM public.health_evidence
    LOOP
        payload_value := row_record.payload;

        IF payload_value #>> '{value,type}' = 'scalar' THEN
            payload_value := jsonb_set(
                payload_value,
                '{value,type}',
                to_jsonb('quantity'::text)
            );
        ELSIF payload_value #>> '{value,type}' = 'qualitative' THEN
            payload_value := jsonb_set(
                payload_value,
                '{value,type}',
                to_jsonb('text'::text)
            );
        ELSIF payload_value #>> '{value,type}' = 'components' THEN
            SELECT coalesce(jsonb_agg(
                CASE
                    WHEN component ? 'quantity' AND NOT component ? 'value'
                    THEN (component - 'quantity') || jsonb_build_object(
                        'value', jsonb_build_object(
                            'type', 'quantity',
                            'quantity', component -> 'quantity'
                        )
                    )
                    ELSE component
                END
                ORDER BY ordinal
            ), '[]'::jsonb)
            INTO component_values
            FROM jsonb_array_elements(
                coalesce(payload_value #> '{value,components}', '[]'::jsonb)
            ) WITH ORDINALITY AS elements(component, ordinal);
            payload_value := jsonb_set(
                payload_value,
                '{value,components}',
                component_values
            );
        END IF;

        context_value := coalesce(payload_value -> 'context', '{}'::jsonb);
        source_value := coalesce(payload_value -> 'source', '{}'::jsonb);
        provenance := CASE source_value ->> 'kind'
            WHEN 'appleHealth' THEN 'native'
            WHEN 'googleHealth' THEN 'native'
            WHEN 'epic' THEN 'native'
            WHEN 'healthRecords' THEN 'copied'
            ELSE 'unknown'
        END;
        source_value := source_value || jsonb_build_object(
            'provenance', provenance
        );
        IF nullif(context_value ->> 'issuer', '') IS NOT NULL THEN
            source_value := source_value || jsonb_build_object(
                'issuer', context_value ->> 'issuer'
            );
        END IF;
        IF nullif(context_value ->> 'resource_type', '') IS NOT NULL THEN
            source_value := source_value || jsonb_build_object(
                'resource_type', context_value ->> 'resource_type'
            );
        END IF;
        IF nullif(source_value ->> 'issuer', '') IS NOT NULL
           AND nullif(source_value ->> 'resource_type', '') IS NOT NULL
           AND nullif(source_value ->> 'record_id', '') IS NOT NULL THEN
            source_value := source_value || jsonb_build_object(
                'upstream_record_id', source_value ->> 'record_id'
            );
        END IF;
        payload_value := jsonb_set(payload_value, '{source}', source_value);

        concept_codings := '[]'::jsonb;
        FOR coding_line IN
            SELECT unnest(string_to_array(
                coalesce(context_value ->> 'concept_codings', ''), E'\n'
            ))
        LOOP
            coding_parts := string_to_array(coding_line, '|');
            IF array_length(coding_parts, 1) = 3
               AND nullif(btrim(coding_parts[1]), '') IS NOT NULL
               AND nullif(btrim(coding_parts[3]), '') IS NOT NULL THEN
                concept_codings := concept_codings || jsonb_build_array(
                    jsonb_build_object(
                        'system', btrim(coding_parts[1]),
                        'version', nullif(btrim(coding_parts[2]), ''),
                        'code', btrim(coding_parts[3]),
                        'display', null
                    )
                );
            END IF;
        END LOOP;
        payload_value := jsonb_set(
            payload_value,
            '{concept_codings}',
            concept_codings
        );
        payload_value := jsonb_set(
            payload_value,
            '{method_codings}',
            '[]'::jsonb
        );

        IF context_value ->> 'specimen' IS NOT NULL THEN
            payload_value := jsonb_set(
                payload_value,
                '{specimen}',
                to_jsonb(context_value ->> 'specimen')
            );
        END IF;
        IF payload_value ->> 'method' = 'source_confirmed_fhir' THEN
            payload_value := jsonb_set(
                payload_value,
                '{method}',
                'null'::jsonb
            );
        END IF;
        IF context_value ->> 'date_unavailable' = 'true' THEN
            payload_value := jsonb_set(
                payload_value,
                '{measured_interval}',
                'null'::jsonb
            );
        END IF;

        record_kind := CASE payload_value ->> 'concept_id'
            WHEN 'medication:record' THEN 'medication'
            WHEN 'clinical:condition' THEN 'condition'
            WHEN 'clinical:allergy' THEN 'allergy'
            WHEN 'clinical:immunization' THEN 'immunization'
            WHEN 'clinical:procedure' THEN 'procedure'
            WHEN 'clinical:diagnostic_report' THEN 'diagnosticReport'
            WHEN 'clinical:encounter' THEN 'encounter'
            WHEN 'clinical:note' THEN 'clinicalNote'
            WHEN 'clinical:document' THEN 'document'
            WHEN 'clinical:coverage' THEN 'coverage'
            WHEN 'clinical:vision_prescription' THEN 'visionPrescription'
            WHEN 'clinical:imaging_study' THEN 'imagingStudy'
            ELSE CASE
                WHEN payload_value ->> 'concept_id' LIKE 'clinical:%'
                THEN 'other'
                ELSE null
            END
        END;
        IF record_kind IS NOT NULL THEN
            category_codings := '[]'::jsonb;
            FOR coding_line IN
                SELECT unnest(string_to_array(
                    coalesce(context_value ->> 'category_codings', ''), E'\n'
                ))
            LOOP
                coding_parts := string_to_array(coding_line, '|');
                IF array_length(coding_parts, 1) = 2
                   AND nullif(btrim(coding_parts[1]), '') IS NOT NULL
                   AND nullif(btrim(coding_parts[2]), '') IS NOT NULL THEN
                    category_codings := category_codings || jsonb_build_array(
                        jsonb_build_object(
                            'system', btrim(coding_parts[1]),
                            'version', null,
                            'code', btrim(coding_parts[2]),
                            'display', null
                        )
                    );
                END IF;
            END LOOP;
            payload_value := jsonb_set(
                payload_value,
                '{clinical_record}',
                jsonb_build_object(
                    'kind', record_kind,
                    'record_status', context_value ->> 'status',
                    'clinical_status', context_value ->> 'clinical_status',
                    'verification_status', context_value ->> 'verification_status',
                    'category_codings', category_codings
                )
            );
        END IF;

        context_value := context_value
            - 'native_origin'
            - 'issuer'
            - 'resource_type'
            - 'record_kind'
            - 'status'
            - 'clinical_status'
            - 'verification_status'
            - 'category_codings'
            - 'concept_codings'
            - 'date_unavailable'
            - 'specimen';
        IF context_value = '{}'::jsonb THEN
            payload_value := payload_value - 'context';
        ELSE
            payload_value := jsonb_set(
                payload_value,
                '{context}',
                context_value
            );
        END IF;

        UPDATE public.health_evidence
        SET payload = payload_value,
            updated_at = now()
        WHERE id = row_record.id;
    END LOOP;
END
$$;

INSERT INTO public.schema_migrations(version)
VALUES ('0007_canonical_health_evidence')
ON CONFLICT (version) DO NOTHING;

COMMIT;
