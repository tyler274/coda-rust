# FollowUpActionResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**confirmed_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**current_revision** | **i32** |  | 
**follow_up_id** | **uuid::Uuid** |  | 
**id** | **uuid::Uuid** |  | 
**kind** | **Kind** |  (enum: create_follow_up, update_follow_up, pause_follow_up, resume_follow_up, cancel_follow_up) | 
**message_id** | **uuid::Uuid** |  | 
**result_revision** | **i32** |  | 
**run_id** | **uuid::Uuid** |  | 
**status** | **Status** |  (enum: confirmed, changed, cancelled) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


