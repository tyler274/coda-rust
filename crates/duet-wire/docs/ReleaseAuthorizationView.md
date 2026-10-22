# ReleaseAuthorizationView

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  |
**discloser_member_id** | **uuid::Uuid** |  |
**patient** | [**models::ReleasePatient**](ReleasePatient.md) |  |
**discloser** | [**models::ReleaseParty**](ReleaseParty.md) |  |
**recipient** | [**models::ReleaseRecipient**](ReleaseRecipient.md) |  |
**scope** | [**models::ReleaseScope**](ReleaseScope.md) |  |
**purpose** | **String** |  |
**expires_on** | Option<**chrono::NaiveDate**> |  | [optional]
**expiration_event** | Option<**String**> |  | [optional]
**signed_name** | Option<**String**> |  | [optional]
**signed_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**created_at** | **chrono::DateTime<chrono::FixedOffset>** |  |
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** |  |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


