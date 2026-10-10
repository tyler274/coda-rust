# \AuthApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**accept_terms_api_auth_account_terms_put**](AuthApi.md#accept_terms_api_auth_account_terms_put) | **PUT** /api/auth/account/terms | Accept Terms
[**delete_account_api_auth_account_delete**](AuthApi.md#delete_account_api_auth_account_delete) | **DELETE** /api/auth/account | Delete Account
[**ensure_account_api_auth_account_put**](AuthApi.md#ensure_account_api_auth_account_put) | **PUT** /api/auth/account | Ensure Account
[**record_foreground_api_account_foreground_post**](AuthApi.md#record_foreground_api_account_foreground_post) | **POST** /api/account/foreground | Record Foreground
[**revoke_sessions_api_auth_session_revoke_post**](AuthApi.md#revoke_sessions_api_auth_session_revoke_post) | **POST** /api/auth/session/revoke | Revoke Sessions
[**update_today_stat_selection_api_auth_account_today_stats_put**](AuthApi.md#update_today_stat_selection_api_auth_account_today_stats_put) | **PUT** /api/auth/account/today-stats | Update Today Stat Selection



## accept_terms_api_auth_account_terms_put

> models::AccountState accept_terms_api_auth_account_terms_put(terms_acceptance_request)
Accept Terms

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**terms_acceptance_request** | [**TermsAcceptanceRequest**](TermsAcceptanceRequest.md) |  | [required] |

### Return type

[**models::AccountState**](AccountState.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_account_api_auth_account_delete

> std::collections::HashMap<String, String> delete_account_api_auth_account_delete()
Delete Account

Delete the authenticated identity and its cascaded Duet account.

### Parameters

This endpoint does not need any parameter.

### Return type

**std::collections::HashMap<String, String>**

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ensure_account_api_auth_account_put

> models::AccountState ensure_account_api_auth_account_put(account_identity_request)
Ensure Account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_identity_request** | [**AccountIdentityRequest**](AccountIdentityRequest.md) |  | [required] |

### Return type

[**models::AccountState**](AccountState.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## record_foreground_api_account_foreground_post

> record_foreground_api_account_foreground_post()
Record Foreground

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_sessions_api_auth_session_revoke_post

> models::SessionRevocationResponse revoke_sessions_api_auth_session_revoke_post()
Revoke Sessions

End every session that signed in during or before this second: first durably, on the server, where every iPhone's push registration goes and such a session can no longer register one or take the sharing key; then at Identity Platform, from the account's stored fence (`account_sessions`). `confirmed`: Identity Platform was read back holding it. `pending`: the backend holds the request and finishes it itself. With no account there is nothing to hold, so anything short of confirmed is the retryable 503. Sharing itself continues (North Star).

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SessionRevocationResponse**](SessionRevocationResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_today_stat_selection_api_auth_account_today_stats_put

> models::TodayStatSelectionState update_today_stat_selection_api_auth_account_today_stats_put(today_stat_selection_request)
Update Today Stat Selection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**today_stat_selection_request** | [**TodayStatSelectionRequest**](TodayStatSelectionRequest.md) |  | [required] |

### Return type

[**models::TodayStatSelectionState**](TodayStatSelectionState.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

