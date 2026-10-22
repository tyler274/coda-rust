# ReleaseAuthorizationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**discloser_member_id** | **uuid::Uuid** |  |
**patient** | Option<[**models::ReleasePatient**](ReleasePatient.md)> |  | [optional]
**recipient** | [**models::ReleaseRecipient**](ReleaseRecipient.md) |  |
**scope** | [**models::ReleaseScope**](ReleaseScope.md) |  |
**purpose** | Option<**String**> |  | [optional]
**expires_on** | Option<**chrono::NaiveDate**> |  | [optional]
**expiration_event** | Option<**String**> |  | [optional]
**signed_name** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


