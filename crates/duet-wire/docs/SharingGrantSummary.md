# SharingGrantSummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**connection_id** | **uuid::Uuid** |  | 
**created_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**grant_id** | **uuid::Uuid** |  | 
**last_payload_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**peer** | [**models::SharingPeer**](SharingPeer.md) |  | 
**recipient_key_version** | **i32** |  | 
**recipient_user_id** | **uuid::Uuid** |  | 
**revision** | **i32** |  | 
**scopes** | Option<[**Vec<models::SharingGrantScope>**](SharingGrantScope.md)> |  | [optional]
**sender_key_version** | **i32** |  | 
**sender_user_id** | **uuid::Uuid** |  | 
**state** | **State** |  (enum: offered, active, revoked, reconnect_required) | 
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


