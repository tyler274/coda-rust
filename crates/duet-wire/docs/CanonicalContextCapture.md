# CanonicalContextCapture

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **uuid::Uuid** |  |
**source_boundary_id** | **uuid::Uuid** |  |
**schema_version** | **i32** |  |
**generated_at** | **chrono::DateTime<chrono::FixedOffset>** |  |
**timezone** | **String** |  |
**source_generations** | Option<**std::collections::HashMap<String, i64>**> |  | [optional]
**projections** | [**Vec<models::CanonicalContextProjection>**](CanonicalContextProjection.md) |  |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


