# InsightCycle

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**concept_ids** | Option<**Vec<String>**> |  | [optional]
**context_snapshot_id** | Option<**uuid::Uuid**> |  |
**corrected_insight_ids** | Option<**Vec<uuid::Uuid>**> |  | [optional]
**first_weekday** | **i32** |  |
**generated_at** | **chrono::DateTime<chrono::FixedOffset>** |  |
**id** | **uuid::Uuid** |  |
**insights** | Option<[**Vec<models::HealthInsight>**](HealthInsight.md)> |  | [optional]
**kind** | [**models::InsightCycleKind**](InsightCycleKind.md) |  |
**observed_through** | **chrono::DateTime<chrono::FixedOffset>** |  |
**supporting_facts** | Option<[**Vec<models::HealthFactOutput>**](HealthFactOutput.md)> |  | [optional]
**timezone** | **String** |  |
**window_end** | **chrono::DateTime<chrono::FixedOffset>** |  |
**window_start** | **chrono::DateTime<chrono::FixedOffset>** |  |

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


