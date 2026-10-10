# EvidenceLibraryPage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**catalog_revision** | Option<**String**> |  | 
**counts** | **std::collections::HashMap<String, i32>** |  | 
**entries** | [**Vec<models::EvidenceLibraryEntry>**](EvidenceLibraryEntry.md) |  | 
**group** | Option<[**models::EvidenceLibraryEntry**](EvidenceLibraryEntry.md)> |  | [optional]
**health_records_generation** | **i32** |  | 
**is_refreshing** | Option<**bool**> |  | [optional][default to false]
**latest_dates** | Option<**std::collections::HashMap<String, chrono::DateTime<chrono::FixedOffset>>**> |  | [optional]
**latest_days** | Option<**std::collections::HashMap<String, chrono::NaiveDate>**> |  | [optional]
**next_cursor** | Option<**String**> |  | 
**revision** | **i32** |  | 
**total_count** | **i32** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


