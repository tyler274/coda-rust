# SelectedMeasurementContextOutput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fact** | [**models::HealthFactOutput**](HealthFactOutput.md) |  | 
**health_context_revision** | **i32** |  | 
**kind** | Option<**Kind**> |  (enum: measurement) | [optional][default to Measurement]
**measurement_id** | **String** |  | 
**period_end** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**period_start** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**source** | [**models::MeasurementSourceIdentity**](MeasurementSourceIdentity.md) |  | 
**source_boundary_id** | **uuid::Uuid** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


