# ConversationRun

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**assistant_message_id** | Option<**uuid::Uuid**> |  |
**created_at** | **chrono::DateTime<chrono::FixedOffset>** |  |
**failure** | Option<**Failure**> |  (enum: unavailable, interrupted, execution_failed) | [optional]
**follow_up_id** | Option<**uuid::Uuid**> |  | [optional]
**follow_up_occurrence_id** | Option<**uuid::Uuid**> |  | [optional]
**follow_up_revision** | Option<**i32**> |  | [optional]
**id** | **uuid::Uuid** |  |
**message_id** | Option<**uuid::Uuid**> |  |
**revision** | **i32** |  |
**selected_follow_up_id** | Option<**uuid::Uuid**> |  | [optional]
**selected_follow_up_revision** | Option<**i32**> |  | [optional]
**state** | [**models::ConversationRunState**](ConversationRunState.md) |  |
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** |  |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


