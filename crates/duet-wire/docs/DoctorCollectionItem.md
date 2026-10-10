# DoctorCollectionItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**day** | Option<**chrono::NaiveDate**> |  | [optional]
**detail** | **String** |  | 
**evidence** | [**Vec<models::EvidenceReferenceOutput>**](EvidenceReferenceOutput.md) |  | 
**fact_snapshot** | Option<[**models::HealthFactOutput**](HealthFactOutput.md)> |  | [optional]
**fact_snapshot_id** | Option<**uuid::Uuid**> |  | [optional]
**id** | **uuid::Uuid** |  | 
**kind** | [**models::DoctorCollectionItemKind**](DoctorCollectionItemKind.md) |  | 
**medication_confirmed** | Option<**bool**> |  | [optional][default to false]
**page_entry** | Option<[**models::DoctorPageEntry**](DoctorPageEntry.md)> |  | [optional]
**position** | **i32** |  | 
**range_end** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**range_start** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**record** | Option<[**models::AddressedRecordRevision**](AddressedRecordRevision.md)> |  | [optional]
**review_required** | Option<**bool**> |  | [optional][default to false]
**saved_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**source_id** | **String** |  | 
**title** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


