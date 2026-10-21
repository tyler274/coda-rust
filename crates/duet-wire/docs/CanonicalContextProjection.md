# CanonicalContextProjection

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | **Kind** |  (enum: daily, weekly) |
**window_start** | **chrono::DateTime<chrono::FixedOffset>** |  |
**window_end** | **chrono::DateTime<chrono::FixedOffset>** |  |
**timezone** | **String** |  |
**first_weekday** | **i32** |  |
**facts** | Option<[**Vec<models::HealthFactInput>**](HealthFactInput.md)> | Only supported native supplemental namespaces; measurement facts are server-derived, never trusted uploads | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


