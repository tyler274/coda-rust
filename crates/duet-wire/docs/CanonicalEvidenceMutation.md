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
**payload** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


