# SharingDeviceResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active_at** | **chrono::DateTime<chrono::FixedOffset>** |  |
**algorithm** | Option<**Algorithm**> |  (enum: curve25519) | [optional][default to Curve25519]
**installation_id** | Option<**uuid::Uuid**> |  | [optional]
**key_version** | **i32** |  |
**public_key** | **String** |  |
**replaced_existing_key** | Option<**bool**> |  | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


