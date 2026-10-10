# FollowUpOccurrence

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**completed_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**conversation_message_id** | Option<**uuid::Uuid**> |  | [optional]
**conversation_run_id** | Option<**uuid::Uuid**> |  | [optional]
**follow_up_id** | **uuid::Uuid** |  | 
**follow_up_revision** | **i32** |  | 
**id** | **uuid::Uuid** |  | 
**scheduled_for** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**status** | **Status** |  (enum: queued, running, completed, unavailable, failed, interrupted) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


