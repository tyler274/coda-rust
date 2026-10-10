# EvidenceReadPage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**canonical_revisions** | [**Vec<models::AddressedRecordRevision>**](AddressedRecordRevision.md) |  | 
**catalog_revision** | Option<**String**> |  | 
**clinical_notes** | Option<[**Vec<models::ClinicalNoteResponse>**](ClinicalNoteResponse.md)> |  | [optional]
**health_evidence** | Option<**Vec<std::collections::HashMap<String, serde_json::Value>>**> |  | [optional]
**health_records_generation** | **i32** |  | 
**next_cursor** | Option<**String**> |  | 
**revision** | **i32** |  | 
**total_count** | **i32** |  | 
**unavailable** | Option<**Vec<uuid::Uuid>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


