# RecurringSchedule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**day_of_month** | Option<**i32**> |  | [optional]
**ends_on** | Option<**chrono::NaiveDate**> |  | [optional]
**frequency** | **Frequency** |  (enum: daily, weekly, monthly) |
**interval** | Option<**i32**> |  | [optional][default to 1]
**kind** | Option<**Kind**> |  (enum: recurring) | [optional][default to Recurring]
**local_time** | **String** |  |
**start_date** | **chrono::NaiveDate** |  |
**timezone** | **String** |  |
**weekdays** | Option<[**Vec<models::Weekday>**](Weekday.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


