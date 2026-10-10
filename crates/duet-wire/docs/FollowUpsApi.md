# \FollowUpsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_follow_up_api_follow_ups_follow_up_id_cancel_post**](FollowUpsApi.md#cancel_follow_up_api_follow_ups_follow_up_id_cancel_post) | **POST** /api/follow-ups/{follow_up_id}/cancel | Cancel Follow Up
[**export_follow_ups_api_follow_ups_export_get**](FollowUpsApi.md#export_follow_ups_api_follow_ups_export_get) | **GET** /api/follow-ups/export | Export Follow Ups
[**get_follow_up_api_follow_ups_follow_up_id_get**](FollowUpsApi.md#get_follow_up_api_follow_ups_follow_up_id_get) | **GET** /api/follow-ups/{follow_up_id} | Get Follow Up
[**list_follow_ups_api_follow_ups_get**](FollowUpsApi.md#list_follow_ups_api_follow_ups_get) | **GET** /api/follow-ups | List Follow Ups
[**pause_follow_up_api_follow_ups_follow_up_id_pause_post**](FollowUpsApi.md#pause_follow_up_api_follow_ups_follow_up_id_pause_post) | **POST** /api/follow-ups/{follow_up_id}/pause | Pause Follow Up
[**resume_follow_up_api_follow_ups_follow_up_id_resume_post**](FollowUpsApi.md#resume_follow_up_api_follow_ups_follow_up_id_resume_post) | **POST** /api/follow-ups/{follow_up_id}/resume | Resume Follow Up
[**settings_api_follow_ups_settings_get**](FollowUpsApi.md#settings_api_follow_ups_settings_get) | **GET** /api/follow-ups/settings | Settings
[**update_settings_api_follow_ups_settings_put**](FollowUpsApi.md#update_settings_api_follow_ups_settings_put) | **PUT** /api/follow-ups/settings | Update Settings



## cancel_follow_up_api_follow_ups_follow_up_id_cancel_post

> models::FollowUp cancel_follow_up_api_follow_ups_follow_up_id_cancel_post(follow_up_id, follow_up_version_request)
Cancel Follow Up

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**follow_up_id** | **uuid::Uuid** |  | [required] |
**follow_up_version_request** | [**FollowUpVersionRequest**](FollowUpVersionRequest.md) |  | [required] |

### Return type

[**models::FollowUp**](FollowUp.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_follow_ups_api_follow_ups_export_get

> models::FollowUpExport export_follow_ups_api_follow_ups_export_get()
Export Follow Ups

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::FollowUpExport**](FollowUpExport.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_follow_up_api_follow_ups_follow_up_id_get

> models::FollowUp get_follow_up_api_follow_ups_follow_up_id_get(follow_up_id)
Get Follow Up

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**follow_up_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::FollowUp**](FollowUp.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_follow_ups_api_follow_ups_get

> models::FollowUpPage list_follow_ups_api_follow_ups_get(cursor, limit)
List Follow Ups

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**uuid::Uuid**> |  |  |
**limit** | Option<**i32**> |  |  |[default to 50]

### Return type

[**models::FollowUpPage**](FollowUpPage.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pause_follow_up_api_follow_ups_follow_up_id_pause_post

> models::FollowUp pause_follow_up_api_follow_ups_follow_up_id_pause_post(follow_up_id, follow_up_version_request)
Pause Follow Up

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**follow_up_id** | **uuid::Uuid** |  | [required] |
**follow_up_version_request** | [**FollowUpVersionRequest**](FollowUpVersionRequest.md) |  | [required] |

### Return type

[**models::FollowUp**](FollowUp.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resume_follow_up_api_follow_ups_follow_up_id_resume_post

> models::FollowUp resume_follow_up_api_follow_ups_follow_up_id_resume_post(follow_up_id, follow_up_version_request)
Resume Follow Up

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**follow_up_id** | **uuid::Uuid** |  | [required] |
**follow_up_version_request** | [**FollowUpVersionRequest**](FollowUpVersionRequest.md) |  | [required] |

### Return type

[**models::FollowUp**](FollowUp.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## settings_api_follow_ups_settings_get

> models::ProactiveSettings settings_api_follow_ups_settings_get()
Settings

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ProactiveSettings**](ProactiveSettings.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_settings_api_follow_ups_settings_put

> models::ProactiveSettings update_settings_api_follow_ups_settings_put(proactive_settings_update)
Update Settings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**proactive_settings_update** | [**ProactiveSettingsUpdate**](ProactiveSettingsUpdate.md) |  | [required] |

### Return type

[**models::ProactiveSettings**](ProactiveSettings.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

