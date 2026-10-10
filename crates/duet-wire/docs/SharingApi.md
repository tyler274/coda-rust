# \SharingApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**accept_invitation_api_sharing_invitations_accept_post**](SharingApi.md#accept_invitation_api_sharing_invitations_accept_post) | **POST** /api/sharing/invitations/accept | Accept Invitation
[**accept_offer_api_sharing_grants_accept_post**](SharingApi.md#accept_offer_api_sharing_grants_accept_post) | **POST** /api/sharing/grants/accept | Accept Offer
[**block_api_sharing_connections_block_post**](SharingApi.md#block_api_sharing_connections_block_post) | **POST** /api/sharing/connections/block | Block
[**cancel_invitation_api_sharing_invitations_cancel_post**](SharingApi.md#cancel_invitation_api_sharing_invitations_cancel_post) | **POST** /api/sharing/invitations/cancel | Cancel Invitation
[**create_invitation_api_sharing_invitations_post**](SharingApi.md#create_invitation_api_sharing_invitations_post) | **POST** /api/sharing/invitations | Create Invitation
[**create_offer_api_sharing_grants_offer_post**](SharingApi.md#create_offer_api_sharing_grants_offer_post) | **POST** /api/sharing/grants/offer | Create Offer
[**decline_invitation_api_sharing_invitations_decline_post**](SharingApi.md#decline_invitation_api_sharing_invitations_decline_post) | **POST** /api/sharing/invitations/decline | Decline Invitation
[**decline_offer_api_sharing_grants_decline_post**](SharingApi.md#decline_offer_api_sharing_grants_decline_post) | **POST** /api/sharing/grants/decline | Decline Offer
[**disconnect_api_sharing_connections_disconnect_post**](SharingApi.md#disconnect_api_sharing_connections_disconnect_post) | **POST** /api/sharing/connections/disconnect | Disconnect
[**download_chunks_api_sharing_chunks_download_post**](SharingApi.md#download_chunks_api_sharing_chunks_download_post) | **POST** /api/sharing/chunks/download | Download Chunks
[**get_device_api_sharing_device_get**](SharingApi.md#get_device_api_sharing_device_get) | **GET** /api/sharing/device | Get Device
[**overview_api_sharing_overview_get**](SharingApi.md#overview_api_sharing_overview_get) | **GET** /api/sharing/overview | Overview
[**preview_invitation_api_sharing_invitations_preview_post**](SharingApi.md#preview_invitation_api_sharing_invitations_preview_post) | **POST** /api/sharing/invitations/preview | Preview Invitation
[**register_device_api_sharing_device_register_post**](SharingApi.md#register_device_api_sharing_device_register_post) | **POST** /api/sharing/device/register | Register Device
[**replace_device_api_sharing_device_replace_post**](SharingApi.md#replace_device_api_sharing_device_replace_post) | **POST** /api/sharing/device/replace | Replace Device
[**stop_grant_api_sharing_grants_stop_post**](SharingApi.md#stop_grant_api_sharing_grants_stop_post) | **POST** /api/sharing/grants/stop | Stop Grant
[**update_grant_api_sharing_grants_put**](SharingApi.md#update_grant_api_sharing_grants_put) | **PUT** /api/sharing/grants | Update Grant
[**upsert_chunks_api_sharing_chunks_batch_post**](SharingApi.md#upsert_chunks_api_sharing_chunks_batch_post) | **POST** /api/sharing/chunks/batch | Upsert Chunks



## accept_invitation_api_sharing_invitations_accept_post

> models::SharingGrantSummary accept_invitation_api_sharing_invitations_accept_post(sharing_invitation_secret_request)
Accept Invitation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sharing_invitation_secret_request** | [**SharingInvitationSecretRequest**](SharingInvitationSecretRequest.md) |  | [required] |

### Return type

[**models::SharingGrantSummary**](SharingGrantSummary.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accept_offer_api_sharing_grants_accept_post

> models::SharingGrantSummary accept_offer_api_sharing_grants_accept_post(sharing_grant_accept_request)
Accept Offer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sharing_grant_accept_request** | [**SharingGrantAcceptRequest**](SharingGrantAcceptRequest.md) |  | [required] |

### Return type

[**models::SharingGrantSummary**](SharingGrantSummary.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## block_api_sharing_connections_block_post

> models::SharingStatusResponse block_api_sharing_connections_block_post(sharing_connection_action_request)
Block

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sharing_connection_action_request** | [**SharingConnectionActionRequest**](SharingConnectionActionRequest.md) |  | [required] |

### Return type

[**models::SharingStatusResponse**](SharingStatusResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_invitation_api_sharing_invitations_cancel_post

> models::SharingStatusResponse cancel_invitation_api_sharing_invitations_cancel_post(sharing_invitation_cancel_request)
Cancel Invitation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sharing_invitation_cancel_request** | [**SharingInvitationCancelRequest**](SharingInvitationCancelRequest.md) |  | [required] |

### Return type

[**models::SharingStatusResponse**](SharingStatusResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_invitation_api_sharing_invitations_post

> models::SharingInvitationCreatedResponse create_invitation_api_sharing_invitations_post(sharing_invitation_create_request)
Create Invitation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sharing_invitation_create_request** | [**SharingInvitationCreateRequest**](SharingInvitationCreateRequest.md) |  | [required] |

### Return type

[**models::SharingInvitationCreatedResponse**](SharingInvitationCreatedResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_offer_api_sharing_grants_offer_post

> models::SharingGrantSummary create_offer_api_sharing_grants_offer_post(sharing_offer_request)
Create Offer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sharing_offer_request** | [**SharingOfferRequest**](SharingOfferRequest.md) |  | [required] |

### Return type

[**models::SharingGrantSummary**](SharingGrantSummary.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## decline_invitation_api_sharing_invitations_decline_post

> models::SharingStatusResponse decline_invitation_api_sharing_invitations_decline_post(sharing_invitation_secret_request)
Decline Invitation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sharing_invitation_secret_request** | [**SharingInvitationSecretRequest**](SharingInvitationSecretRequest.md) |  | [required] |

### Return type

[**models::SharingStatusResponse**](SharingStatusResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## decline_offer_api_sharing_grants_decline_post

> models::SharingStatusResponse decline_offer_api_sharing_grants_decline_post(sharing_grant_action_request)
Decline Offer

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sharing_grant_action_request** | [**SharingGrantActionRequest**](SharingGrantActionRequest.md) |  | [required] |

### Return type

[**models::SharingStatusResponse**](SharingStatusResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disconnect_api_sharing_connections_disconnect_post

> models::SharingStatusResponse disconnect_api_sharing_connections_disconnect_post(sharing_connection_action_request)
Disconnect

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sharing_connection_action_request** | [**SharingConnectionActionRequest**](SharingConnectionActionRequest.md) |  | [required] |

### Return type

[**models::SharingStatusResponse**](SharingStatusResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_chunks_api_sharing_chunks_download_post

> models::SharingChunkDownloadResponse download_chunks_api_sharing_chunks_download_post(sharing_chunk_download_request)
Download Chunks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sharing_chunk_download_request** | [**SharingChunkDownloadRequest**](SharingChunkDownloadRequest.md) |  | [required] |

### Return type

[**models::SharingChunkDownloadResponse**](SharingChunkDownloadResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_device_api_sharing_device_get

> models::SharingDeviceResponse get_device_api_sharing_device_get()
Get Device

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SharingDeviceResponse**](SharingDeviceResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## overview_api_sharing_overview_get

> models::SharingOverviewResponse overview_api_sharing_overview_get()
Overview

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SharingOverviewResponse**](SharingOverviewResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## preview_invitation_api_sharing_invitations_preview_post

> models::SharingInvitationPreviewResponse preview_invitation_api_sharing_invitations_preview_post(sharing_invitation_secret_request)
Preview Invitation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sharing_invitation_secret_request** | [**SharingInvitationSecretRequest**](SharingInvitationSecretRequest.md) |  | [required] |

### Return type

[**models::SharingInvitationPreviewResponse**](SharingInvitationPreviewResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register_device_api_sharing_device_register_post

> models::SharingDeviceResponse register_device_api_sharing_device_register_post(sharing_device_request)
Register Device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sharing_device_request** | [**SharingDeviceRequest**](SharingDeviceRequest.md) |  | [required] |

### Return type

[**models::SharingDeviceResponse**](SharingDeviceResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_device_api_sharing_device_replace_post

> models::SharingDeviceResponse replace_device_api_sharing_device_replace_post(sharing_device_replacement_request)
Replace Device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sharing_device_replacement_request** | [**SharingDeviceReplacementRequest**](SharingDeviceReplacementRequest.md) |  | [required] |

### Return type

[**models::SharingDeviceResponse**](SharingDeviceResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_grant_api_sharing_grants_stop_post

> models::SharingStatusResponse stop_grant_api_sharing_grants_stop_post(sharing_grant_action_request)
Stop Grant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sharing_grant_action_request** | [**SharingGrantActionRequest**](SharingGrantActionRequest.md) |  | [required] |

### Return type

[**models::SharingStatusResponse**](SharingStatusResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_grant_api_sharing_grants_put

> models::SharingGrantSummary update_grant_api_sharing_grants_put(sharing_grant_update_request)
Update Grant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sharing_grant_update_request** | [**SharingGrantUpdateRequest**](SharingGrantUpdateRequest.md) |  | [required] |

### Return type

[**models::SharingGrantSummary**](SharingGrantSummary.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upsert_chunks_api_sharing_chunks_batch_post

> models::SharingChunkBatchResponse upsert_chunks_api_sharing_chunks_batch_post(sharing_chunk_batch_request)
Upsert Chunks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sharing_chunk_batch_request** | [**SharingChunkBatchRequest**](SharingChunkBatchRequest.md) |  | [required] |

### Return type

[**models::SharingChunkBatchResponse**](SharingChunkBatchResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

