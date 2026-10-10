# \SavedRoutinesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_saved_routine_api_saved_routines_routine_id_delete**](SavedRoutinesApi.md#delete_saved_routine_api_saved_routines_routine_id_delete) | **DELETE** /api/saved-routines/{routine_id} | Delete Saved Routine
[**get_saved_routine_api_saved_routines_routine_id_get**](SavedRoutinesApi.md#get_saved_routine_api_saved_routines_routine_id_get) | **GET** /api/saved-routines/{routine_id} | Get Saved Routine
[**list_saved_routines_api_saved_routines_get**](SavedRoutinesApi.md#list_saved_routines_api_saved_routines_get) | **GET** /api/saved-routines | List Saved Routines
[**update_saved_routine_api_saved_routines_routine_id_put**](SavedRoutinesApi.md#update_saved_routine_api_saved_routines_routine_id_put) | **PUT** /api/saved-routines/{routine_id} | Update Saved Routine



## delete_saved_routine_api_saved_routines_routine_id_delete

> delete_saved_routine_api_saved_routines_routine_id_delete(routine_id, expected_revision)
Delete Saved Routine

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**routine_id** | **uuid::Uuid** |  | [required] |
**expected_revision** | **i32** |  | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_saved_routine_api_saved_routines_routine_id_get

> models::SavedRoutine get_saved_routine_api_saved_routines_routine_id_get(routine_id)
Get Saved Routine

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**routine_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::SavedRoutine**](SavedRoutine.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_saved_routines_api_saved_routines_get

> models::SavedRoutineList list_saved_routines_api_saved_routines_get()
List Saved Routines

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SavedRoutineList**](SavedRoutineList.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_saved_routine_api_saved_routines_routine_id_put

> models::SavedRoutine update_saved_routine_api_saved_routines_routine_id_put(routine_id, saved_routine_update_request)
Update Saved Routine

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**routine_id** | **uuid::Uuid** |  | [required] |
**saved_routine_update_request** | [**SavedRoutineUpdateRequest**](SavedRoutineUpdateRequest.md) |  | [required] |

### Return type

[**models::SavedRoutine**](SavedRoutine.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

