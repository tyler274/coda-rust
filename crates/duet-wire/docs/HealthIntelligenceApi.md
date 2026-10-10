# \HealthIntelligenceApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**complete_setup_api_account_setup_complete_post**](HealthIntelligenceApi.md#complete_setup_api_account_setup_complete_post) | **POST** /api/account/setup/complete | Complete Setup
[**correct_insight_api_insight_cycles_cycle_id_insights_insight_id_correction_put**](HealthIntelligenceApi.md#correct_insight_api_insight_cycles_cycle_id_insights_insight_id_correction_put) | **PUT** /api/insight-cycles/{cycle_id}/insights/{insight_id}/correction | Correct Insight
[**create_insight_cycle_api_insight_cycles_post**](HealthIntelligenceApi.md#create_insight_cycle_api_insight_cycles_post) | **POST** /api/insight-cycles | Create Insight Cycle
[**export_intelligence_understanding_api_intelligence_understanding_export_get**](HealthIntelligenceApi.md#export_intelligence_understanding_api_intelligence_understanding_export_get) | **GET** /api/intelligence/understanding/export | Export Intelligence Understanding
[**get_intelligence_settings_api_intelligence_settings_get**](HealthIntelligenceApi.md#get_intelligence_settings_api_intelligence_settings_get) | **GET** /api/intelligence/settings | Get Intelligence Settings
[**insight_source_api_insight_cycles_cycle_id_insights_insight_id_sources_source_id_get**](HealthIntelligenceApi.md#insight_source_api_insight_cycles_cycle_id_insights_insight_id_sources_source_id_get) | **GET** /api/insight-cycles/{cycle_id}/insights/{insight_id}/sources/{source_id} | Insight Source
[**list_insight_cycles_api_insight_cycles_get**](HealthIntelligenceApi.md#list_insight_cycles_api_insight_cycles_get) | **GET** /api/insight-cycles | List Insight Cycles
[**read_insight_cycle_slot_api_insight_cycles_slot_get**](HealthIntelligenceApi.md#read_insight_cycle_slot_api_insight_cycles_slot_get) | **GET** /api/insight-cycles/slot | Read Insight Cycle Slot
[**refresh_health_context_api_health_context_refresh_post**](HealthIntelligenceApi.md#refresh_health_context_api_health_context_refresh_post) | **POST** /api/health-context/refresh | Refresh Health Context
[**update_intelligence_settings_api_intelligence_settings_put**](HealthIntelligenceApi.md#update_intelligence_settings_api_intelligence_settings_put) | **PUT** /api/intelligence/settings | Update Intelligence Settings
[**update_luna_consent_api_intelligence_consent_put**](HealthIntelligenceApi.md#update_luna_consent_api_intelligence_consent_put) | **PUT** /api/intelligence/consent | Update Luna Consent
[**upload_health_context_api_health_context_post**](HealthIntelligenceApi.md#upload_health_context_api_health_context_post) | **POST** /api/health-context | Upload Health Context



## complete_setup_api_account_setup_complete_post

> models::AccountState complete_setup_api_account_setup_complete_post()
Complete Setup

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AccountState**](AccountState.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## correct_insight_api_insight_cycles_cycle_id_insights_insight_id_correction_put

> models::InsightCycle correct_insight_api_insight_cycles_cycle_id_insights_insight_id_correction_put(cycle_id, insight_id)
Correct Insight

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cycle_id** | **uuid::Uuid** |  | [required] |
**insight_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::InsightCycle**](InsightCycle.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_insight_cycle_api_insight_cycles_post

> models::InsightCycleGenerationResponse create_insight_cycle_api_insight_cycles_post(insight_cycle_request)
Create Insight Cycle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**insight_cycle_request** | [**InsightCycleRequest**](InsightCycleRequest.md) |  | [required] |

### Return type

[**models::InsightCycleGenerationResponse**](InsightCycleGenerationResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_intelligence_understanding_api_intelligence_understanding_export_get

> models::OperatingIndexExport export_intelligence_understanding_api_intelligence_understanding_export_get()
Export Intelligence Understanding

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::OperatingIndexExport**](OperatingIndexExport.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_intelligence_settings_api_intelligence_settings_get

> models::IntelligenceSettings get_intelligence_settings_api_intelligence_settings_get()
Get Intelligence Settings

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::IntelligenceSettings**](IntelligenceSettings.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insight_source_api_insight_cycles_cycle_id_insights_insight_id_sources_source_id_get

> models::ConversationSourceDetail insight_source_api_insight_cycles_cycle_id_insights_insight_id_sources_source_id_get(cycle_id, insight_id, source_id)
Insight Source

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cycle_id** | **uuid::Uuid** |  | [required] |
**insight_id** | **uuid::Uuid** |  | [required] |
**source_id** | **String** |  | [required] |

### Return type

[**models::ConversationSourceDetail**](ConversationSourceDetail.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_insight_cycles_api_insight_cycles_get

> models::InsightCyclePage list_insight_cycles_api_insight_cycles_get(cursor, limit)
List Insight Cycles

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**uuid::Uuid**> |  |  |
**limit** | Option<**i32**> |  |  |[default to 50]

### Return type

[**models::InsightCyclePage**](InsightCyclePage.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_insight_cycle_slot_api_insight_cycles_slot_get

> models::InsightCycleSlotStatus read_insight_cycle_slot_api_insight_cycles_slot_get(kind, window_start, window_end, timezone, first_weekday)
Read Insight Cycle Slot

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**kind** | [**InsightCycleKind**](InsightCycleKind.md) |  | [required] |
**window_start** | **chrono::DateTime<chrono::FixedOffset>** |  | [required] |
**window_end** | **chrono::DateTime<chrono::FixedOffset>** |  | [required] |
**timezone** | **String** |  | [required] |
**first_weekday** | **i32** |  | [required] |

### Return type

[**models::InsightCycleSlotStatus**](InsightCycleSlotStatus.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_health_context_api_health_context_refresh_post

> models::HealthContextReadResponse refresh_health_context_api_health_context_refresh_post()
Refresh Health Context

Refresh bounded server-held facts and return the canonical cache body.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HealthContextReadResponse**](HealthContextReadResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_intelligence_settings_api_intelligence_settings_put

> models::IntelligenceSettings update_intelligence_settings_api_intelligence_settings_put(intelligence_control)
Update Intelligence Settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**intelligence_control** | [**IntelligenceControl**](IntelligenceControl.md) |  | [required] |

### Return type

[**models::IntelligenceSettings**](IntelligenceSettings.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_luna_consent_api_intelligence_consent_put

> models::AccountState update_luna_consent_api_intelligence_consent_put(luna_consent_request)
Update Luna Consent

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**luna_consent_request** | [**LunaConsentRequest**](LunaConsentRequest.md) |  | [required] |

### Return type

[**models::AccountState**](AccountState.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_health_context_api_health_context_post

> models::HealthContextUploadResponse upload_health_context_api_health_context_post(health_context_upload_request)
Upload Health Context

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**health_context_upload_request** | [**HealthContextUploadRequest**](HealthContextUploadRequest.md) |  | [required] |

### Return type

[**models::HealthContextUploadResponse**](HealthContextUploadResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

