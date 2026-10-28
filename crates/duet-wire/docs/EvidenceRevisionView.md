# EvidenceRevisionView

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address** | [**models::CanonicalSourceAddress**](CanonicalSourceAddress.md) |  |
**status** | **Status** |  (enum: current, superseded, removed_by_source, unavailable) |
**current** | **bool** |  |
**newer_revision_exists** | Option<**bool**> |  | [optional]
**current_revision** | Option<**i64**> |  | [optional]
**payload** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]
**reference** | Option<[**models::ConversationSource**](ConversationSource.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


