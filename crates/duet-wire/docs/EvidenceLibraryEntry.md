# EvidenceLibraryEntry

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**category** | **String** |  | 
**coding_identities** | [**Vec<models::ClinicalCodingIdentity>**](ClinicalCodingIdentity.md) |  | 
**condition_status_conflict** | **bool** | Build 134's required wire name for the same conflict state. | [readonly]
**date** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | 
**day** | Option<**chrono::NaiveDate**> |  | [optional]
**entity** | **Entity** |  (enum: clinical_note, health_evidence) | 
**facets** | Option<**Vec<std::collections::HashMap<String, serde_json::Value>>**> |  | [optional]
**group_kind** | Option<**String**> |  | 
**id** | **String** |  | 
**lab_history_id** | Option<**String**> |  | [optional]
**lab_result_id** | Option<**uuid::Uuid**> |  | [optional]
**member_count** | **i32** |  | 
**preview** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | 
**record_id** | **uuid::Uuid** |  | 
**source_label** | Option<**String**> |  | [optional]
**status_conflict** | **bool** |  | 
**summary** | Option<**String**> |  | [optional]
**title** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


