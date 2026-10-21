# DurableRestrictionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_id** | **uuid::Uuid** |  |
**operation** | **Operation** |  (enum: luna_withdraw, sessions_revoke, evidence_delete, doctor_page_delete, doctor_note_delete, doctor_item_delete, publication_revoke, device_withdraw, records_withdraw, follow_up_pause, follow_up_cancel, account_delete) |
**scope_id** | Option<**uuid::Uuid**> |  | [optional]
**scope_version** | **i64** |  |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


