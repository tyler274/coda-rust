# \MomentsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_moments_api_moments_get**](MomentsApi.md#list_moments_api_moments_get) | **GET** /api/moments | List Moments
[**remove_moment_api_moments_moment_id_delete**](MomentsApi.md#remove_moment_api_moments_moment_id_delete) | **DELETE** /api/moments/{moment_id} | Remove Moment



## list_moments_api_moments_get

> Vec<models::AccountMoment> list_moments_api_moments_get()
List Moments

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::AccountMoment>**](AccountMoment.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_moment_api_moments_moment_id_delete

> remove_moment_api_moments_moment_id_delete(moment_id)
Remove Moment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**moment_id** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

