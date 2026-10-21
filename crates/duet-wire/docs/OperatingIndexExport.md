# OperatingIndexExport

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**context_snapshot_id** | Option<**uuid::Uuid**> |  |
**control_revision** | **i32** |  |
**conversation_reads** | [**Vec<models::OperatingIndexConversationRead>**](OperatingIndexConversationRead.md) |  |
**created_at** | **chrono::DateTime<chrono::FixedOffset>** |  |
**evidence_revision** | **i32** |  |
**fact_receipts** | **Vec<std::collections::HashMap<String, serde_json::Value>>** |  |
**history_cursor** | **i32** |  |
**id** | **uuid::Uuid** |  |
**instruction_version** | **String** |  |
**markdown** | **String** |  |
**model** | **String** |  |
**read_receipts** | **Vec<std::collections::HashMap<String, serde_json::Value>>** |  |
**revision** | **i32** |  |
**saved_routine_ids** | **Vec<uuid::Uuid>** |  |
**source_connection_ids** | **Vec<String>** |  |
**source_kinds** | **Vec<String>** |  |
**source_record_ids** | **Vec<uuid::Uuid>** |  |
**superseded** | **bool** |  |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


