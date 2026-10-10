# EpicAcquisitionUnit

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**current_authorization** | **bool** |  | 
**failed_pages** | Option<[**Vec<models::EpicAcquisitionPageFailure>**](EpicAcquisitionPageFailure.md)> |  | [optional]
**failed_records** | Option<[**Vec<models::EpicAcquisitionRecordFailure>**](EpicAcquisitionRecordFailure.md)> |  | [optional]
**last_completed_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | 
**last_failure_code** | Option<**String**> |  | 
**last_failure_kind** | Option<**LastFailureKind**> |  (enum: temporary, access_denied, reauthorization_required, account_not_linked, provider_contract_failure) | [optional]
**pages** | **i32** |  | 
**policy_version** | **String** |  | 
**published_count** | **i32** |  | 
**query_parameters** | **std::collections::HashMap<String, String>** |  | 
**resource_type** | **String** |  | 
**retry_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**search_outcome** | Option<**SearchOutcome**> |  (enum: warning) | [optional]
**state** | **State** |  (enum: pending, running, completed, failed, unavailable) | 
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


