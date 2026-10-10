# HealthContextSnapshotOutput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**generated_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**id** | Option<**uuid::Uuid**> |  | [optional]
**projections** | [**Vec<models::HealthContextProjectionOutput>**](HealthContextProjectionOutput.md) |  | 
**schema_version** | Option<**i32**> |  | [optional][default to 1]
**source_boundary_id** | **uuid::Uuid** |  | 
**source_generations** | Option<**std::collections::HashMap<String, i32>**> |  | [optional]
**timezone** | Option<**String**> |  | [optional][default to UTC]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


