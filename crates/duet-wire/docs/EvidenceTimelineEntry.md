# EvidenceTimelineEntry

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**clinical_note** | Option<[**models::ClinicalNoteResponse**](ClinicalNoteResponse.md)> |  | [optional]
**date** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  |
**entity** | **Entity** |  (enum: clinical_note, health_evidence) |
**health_evidence** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]
**id** | **String** |  |
**lab_history_id** | Option<**String**> |  | [optional]
**record_id** | **uuid::Uuid** |  |
**summary** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


