# \DevicesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bind_push_withdrawal_key_api_devices_push_withdrawal_key_put**](DevicesApi.md#bind_push_withdrawal_key_api_devices_push_withdrawal_key_put) | **PUT** /api/devices/push/withdrawal-key | Bind Push Withdrawal Key
[**push_registration_api_devices_push_get**](DevicesApi.md#push_registration_api_devices_push_get) | **GET** /api/devices/push | Push Registration
[**update_push_registration_api_devices_push_put**](DevicesApi.md#update_push_registration_api_devices_push_put) | **PUT** /api/devices/push | Update Push Registration
[**withdraw_push_registration_api_devices_push_withdraw_post**](DevicesApi.md#withdraw_push_registration_api_devices_push_withdraw_post) | **POST** /api/devices/push/withdraw | Withdraw Push Registration



## bind_push_withdrawal_key_api_devices_push_withdrawal_key_put

> models::PushRegistrationState bind_push_withdrawal_key_api_devices_push_withdrawal_key_put(push_withdrawal_key_binding)
Bind Push Withdrawal Key

Bind this iPhone's withdrawal key to its installation's row before it registers a token. `409` when the row changed since the revision the iPhone read; `401` for a session that signed in before the account's last sign out everywhere.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**push_withdrawal_key_binding** | [**PushWithdrawalKeyBinding**](PushWithdrawalKeyBinding.md) |  | [required] |

### Return type

[**models::PushRegistrationState**](PushRegistrationState.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## push_registration_api_devices_push_get

> models::PushRegistrationState push_registration_api_devices_push_get(installation_id)
Push Registration

This iPhone's registration. When another account signed in on this iPhone before owns its row, only the row's revision, for the bind.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**installation_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::PushRegistrationState**](PushRegistrationState.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_push_registration_api_devices_push_put

> models::PushRegistrationState update_push_registration_api_devices_push_put(push_registration_request)
Update Push Registration

Register a token on the row holding this iPhone's key, or, with no token, clear this iPhone's registration: the row holding its key, the same rule as the session-free withdrawal. Another iPhone's registration is never touched. `409` when the row changed or holds another key: the iPhone reads its row again and binds its key before registering. Every request from a session that signed in before the account's last sign out everywhere is refused (`401`); a registration that got past that check before the fence committed is refused here, a clear is not. A signed-out iPhone withdraws with its key alone (`/push/withdraw`).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**push_registration_request** | [**PushRegistrationRequest**](PushRegistrationRequest.md) |  | [required] |

### Return type

[**models::PushRegistrationState**](PushRegistrationState.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## withdraw_push_registration_api_devices_push_withdraw_post

> withdraw_push_registration_api_devices_push_withdraw_post(push_withdrawal_request)
Withdraw Push Registration

Clear this iPhone's registration without an account session, so a signed-out iPhone can finish withdrawing its notifications.  The proof is the withdrawal key bound to the registration's row; it authorizes nothing else. Every well-formed request receives the same answer, whether it withdrew a registration or no row holds the key, and a key no row holds writes nothing.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**push_withdrawal_request** | [**PushWithdrawalRequest**](PushWithdrawalRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

