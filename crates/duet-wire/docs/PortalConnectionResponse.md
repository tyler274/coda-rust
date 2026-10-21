# PortalConnectionResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**acquisition** | [**models::EpicAcquisitionCoverage**](EpicAcquisitionCoverage.md) |  |
**available** | Option<**bool**> |  | [optional][default to true]
**brand** | **String** |  |
**evidence_revision** | **i32** |  |
**failure** | Option<[**models::RemoteConnectorFailure**](RemoteConnectorFailure.md)> |  | [optional]
**granted_scopes** | Option<**Vec<String>**> |  | [optional]
**id** | **String** |  |
**last_synced_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**organization_id** | **String** |  |
**organization_name** | **String** |  |
**status** | Option<**Status**> |  (enum: not_connected, pending, connected, reauthorization_required) | [optional][default to NotConnected]
**vendor** | **Vendor** |  (enum: epic, oracle, healow) |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


