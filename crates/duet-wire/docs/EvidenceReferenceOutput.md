# EvidenceReferenceOutput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**concept_id** | Option<**String**> |  | [optional]
**deep_link** | Option<**String**> |  | [optional]
**id** | **String** |  |
**kind** | [**models::EvidenceKind**](EvidenceKind.md) |  |
**range_end** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**range_start** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**source_connection_ids** | Option<**Vec<String>**> |  | [optional]
**source_fields** | Option<[**Vec<models::EvidenceSourceField>**](EvidenceSourceField.md)> |  | [optional]
**source_kind** | Option<**String**> |  | [optional]
**source_portal** | Option<**String**> | The patient portal a record came through, named from its source on every read, so evidence saved at any time is named the same way. | [readonly]
**subtitle** | Option<**String**> |  | [optional]
**title** | **String** |  |
**window** | Option<[**models::HealthWindow**](HealthWindow.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


