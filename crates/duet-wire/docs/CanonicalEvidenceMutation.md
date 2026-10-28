# CanonicalEvidenceMutation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**operation_id** | Option<**uuid::Uuid**> |  |
**canonical_record_id** | Option<**uuid::Uuid**> |  |
**record_id** | **String** |  |
**entity** | **Entity** |  (enum: clinical_note, health_evidence) |
**expected_revision** | Option<**i64**> |  | [optional]
**source_generation** | Option<**i32**> |  | [optional]
**operation** | Option<**Operation**> | delete is an explicit source removal: payload is exactly {\"source\":{\"kind\":..}} and expected_revision is the exact current revision. Retained revisions stay citable as non-current; it is not privacy erasure. (enum: upsert, delete) | [optional][default to Upsert]
**payload** | **std::collections::HashMap<String, serde_json::Value>** | Literal source object. Health evidence measured_at or measured_interval.start is projected only when an RFC 3339 instant; otherwise the stored date is unknown (null), never receipt time. |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


