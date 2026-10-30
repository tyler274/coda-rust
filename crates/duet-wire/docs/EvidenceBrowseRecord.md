# EvidenceBrowseRecord

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address** | [**models::CanonicalSourceAddress**](CanonicalSourceAddress.md) |  |
**status** | **Status** |  (enum: current, removed_by_source) |
**title** | **String** |  |
**provenance** | [**models::EvidenceRecordProvenance**](EvidenceRecordProvenance.md) |  |
**date** | Option<**String**> | Literal declared date string; absent means the date is unknown | [optional]
**date_end** | Option<**String**> | Literal interval end when it differs from date. Absent for a single instant. | [optional]
**date_role** | Option<**String**> |  | [optional]
**time_zone** | Option<**String**> |  | [optional]
**value** | Option<**String**> | Literal value exactly as declared | [optional]
**unit** | Option<**String**> |  | [optional]
**source_status** | Option<**String**> |  | [optional]
**payload** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


