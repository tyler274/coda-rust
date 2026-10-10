# AccountState

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**display_name** | Option<**String**> |  | [optional]
**intelligence_available** | Option<**bool**> |  | [optional][default to true]
**intelligence_enabled** | Option<**bool**> |  | [optional][default to true]
**luna_consent** | Option<[**models::LunaConsentDecision**](LunaConsentDecision.md)> |  | [optional][default to Undecided]
**luna_consent_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**luna_consent_version** | Option<**String**> |  | [optional][default to 2026-09-24]
**setup_complete** | Option<**bool**> |  | [optional][default to false]
**terms_accepted** | Option<**bool**> |  | [optional][default to false]
**terms_accepted_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**terms_version** | Option<**String**> |  | [optional][default to 2026-09-23]
**today_stat_ids** | Option<**Vec<String>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


