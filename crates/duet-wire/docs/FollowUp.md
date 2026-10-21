# FollowUp

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accepted_instruction** | **String** |  |
**created_at** | **chrono::DateTime<chrono::FixedOffset>** |  |
**current_occurrence** | Option<[**models::FollowUpOccurrence**](FollowUpOccurrence.md)> |  | [optional]
**display_title** | Option<**String**> |  | [optional]
**id** | **uuid::Uuid** |  |
**last_outcome** | Option<[**models::FollowUpOutcome**](FollowUpOutcome.md)> |  | [optional]
**next_occurrence_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**operation** | [**models::FollowUpOperation**](FollowUpOperation.md) |  |
**pause_reason** | Option<**PauseReason**> |  (enum: user, confirmation_required, proactive_disabled, ai_disabled, permission_required) | [optional]
**revision** | **i32** |  |
**routine_id** | Option<**uuid::Uuid**> |  | [optional]
**schedule** | [**models::Schedule**](Schedule.md) |  |
**schedule_summary** | **String** |  |
**source_basis** | [**Vec<models::FollowUpSourceBasis>**](FollowUpSourceBasis.md) |  |
**state** | [**models::FollowUpState**](FollowUpState.md) |  |
**updated_at** | **chrono::DateTime<chrono::FixedOffset>** |  |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


