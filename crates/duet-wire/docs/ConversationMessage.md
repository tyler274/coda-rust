# ConversationMessage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**actions** | Option<[**Vec<models::ActionsInner>**](ActionsInner.md)> |  | [optional]
**created_at** | **chrono::DateTime<chrono::FixedOffset>** |  |
**feedback** | Option<[**models::ConversationReplyFeedback**](ConversationReplyFeedback.md)> |  | [optional]
**id** | **uuid::Uuid** |  |
**model_written** | Option<**bool**> |  | [optional][default to false]
**reply_to** | Option<[**models::ConversationReplyReference**](ConversationReplyReference.md)> |  | [optional]
**reply_to_message_id** | Option<**uuid::Uuid**> |  | [optional]
**revision** | **i32** |  |
**role** | **Role** |  (enum: user, assistant) |
**run_id** | Option<**uuid::Uuid**> |  | [optional]
**selected_context** | Option<[**models::ConversationMessageSelectedContext**](ConversationMessageSelectedContext.md)> |  | [optional]
**selected_insight** | Option<[**models::PublishedInsightSelection**](PublishedInsightSelection.md)> |  | [optional]
**sequence** | **i32** |  |
**sources** | Option<[**Vec<models::ConversationSource>**](ConversationSource.md)> |  | [optional]
**state** | [**models::ConversationMessageState**](ConversationMessageState.md) |  |
**text** | **String** |  |
**tutorial** | Option<**Tutorial**> |  (enum: wearables_apple_health) | [optional]
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** |  |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


