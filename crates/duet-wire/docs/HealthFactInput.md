# HealthFactInput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**absolute_delta** | Option<**f64**> |  | [optional]
**comparison_support_count** | Option<**i32**> |  | [optional]
**comparison_value** | Option<**f64**> |  | [optional]
**concept_ids** | **Vec<String>** |  | 
**current_support_count** | Option<**i32**> |  | [optional]
**evidence** | [**Vec<models::EvidenceReferenceInput>**](EvidenceReferenceInput.md) |  | 
**fit_strength** | Option<**f64**> |  | [optional]
**id** | **String** |  | 
**kind** | [**models::HealthFactKind**](HealthFactKind.md) |  | 
**percent_delta** | Option<**f64**> |  | [optional]
**range_end** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**range_start** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**sensitivity** | Option<[**models::HealthSensitivity**](HealthSensitivity.md)> |  | [optional][default to Standard]
**support_count** | Option<**i32**> |  | [optional]
**title** | **String** |  | 
**unit** | Option<**String**> |  | [optional]
**value** | Option<**f64**> |  | [optional]
**window** | [**models::HealthWindow**](HealthWindow.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


