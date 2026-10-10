# DoctorActionResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**base_generation** | **i32** |  | 
**confirmed_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**current_generation** | **i32** |  | 
**id** | **uuid::Uuid** |  | 
**kind** | Option<**Kind**> |  (enum: save_doctor_question, prepare_doctor_summary) | [optional][default to SaveDoctorQuestion]
**message_id** | **uuid::Uuid** |  | 
**result_generation** | **i32** |  | 
**run_id** | **uuid::Uuid** |  | 
**status** | **Status** |  (enum: confirmed, changed) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


