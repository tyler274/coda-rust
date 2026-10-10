# EvidenceStateResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**canonical_revisions** | [**Vec<models::CanonicalRecordRevision>**](CanonicalRecordRevision.md) |  | 
**catalog_revision** | Option<**String**> |  | 
**clinical_notes** | Option<[**Vec<models::ClinicalNoteResponse>**](ClinicalNoteResponse.md)> |  | [optional]
**deleted_records** | Option<[**Vec<models::EvidenceDeletion>**](EvidenceDeletion.md)> |  | [optional]
**health_evidence** | Option<**Vec<std::collections::HashMap<String, serde_json::Value>>**> |  | [optional]
**health_records_generation** | **i32** |  | 
**mode** | Option<**Mode**> |  (enum: snapshot, changes) | [optional][default to Snapshot]
**next_cursor** | Option<**String**> |  | 
**revision** | **i32** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


