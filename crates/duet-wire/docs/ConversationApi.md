# \ConversationApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**accepted_message_api_conversation_messages_message_id_get**](ConversationApi.md#accepted_message_api_conversation_messages_message_id_get) | **GET** /api/conversation/messages/{message_id} | Accepted Message
[**clear_feedback_api_conversation_messages_message_id_feedback_delete**](ConversationApi.md#clear_feedback_api_conversation_messages_message_id_feedback_delete) | **DELETE** /api/conversation/messages/{message_id}/feedback | Clear Feedback
[**events_api_conversation_events_get**](ConversationApi.md#events_api_conversation_events_get) | **GET** /api/conversation/events | Events
[**export_conversation_api_conversation_export_get**](ConversationApi.md#export_conversation_api_conversation_export_get) | **GET** /api/conversation/export | Export Conversation
[**message_source_api_conversation_messages_message_id_sources_source_id_get**](ConversationApi.md#message_source_api_conversation_messages_message_id_sources_source_id_get) | **GET** /api/conversation/messages/{message_id}/sources/{source_id} | Message Source
[**messages_api_conversation_messages_get**](ConversationApi.md#messages_api_conversation_messages_get) | **GET** /api/conversation/messages | Messages
[**send_message_api_conversation_messages_post**](ConversationApi.md#send_message_api_conversation_messages_post) | **POST** /api/conversation/messages | Send Message
[**set_feedback_api_conversation_messages_message_id_feedback_put**](ConversationApi.md#set_feedback_api_conversation_messages_message_id_feedback_put) | **PUT** /api/conversation/messages/{message_id}/feedback | Set Feedback



## accepted_message_api_conversation_messages_message_id_get

> models::ConversationAcceptance accepted_message_api_conversation_messages_message_id_get(message_id)
Accepted Message

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::ConversationAcceptance**](ConversationAcceptance.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clear_feedback_api_conversation_messages_message_id_feedback_delete

> models::ConversationMessage clear_feedback_api_conversation_messages_message_id_feedback_delete(message_id)
Clear Feedback

Remove the person's feedback on a reply; clearing none changes nothing.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::ConversationMessage**](ConversationMessage.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_api_conversation_events_get

> serde_json::Value events_api_conversation_events_get(after)
Events

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**after** | Option<**i32**> |  |  |[default to 0]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## export_conversation_api_conversation_export_get

> models::ConversationExport export_conversation_api_conversation_export_get()
Export Conversation

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ConversationExport**](ConversationExport.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## message_source_api_conversation_messages_message_id_sources_source_id_get

> models::ConversationSourceDetail message_source_api_conversation_messages_message_id_sources_source_id_get(message_id, source_id)
Message Source

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **uuid::Uuid** |  | [required] |
**source_id** | **String** |  | [required] |

### Return type

[**models::ConversationSourceDetail**](ConversationSourceDetail.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## messages_api_conversation_messages_get

> models::ConversationPage messages_api_conversation_messages_get(before, limit)
Messages

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**before** | Option<**i32**> |  |  |
**limit** | Option<**i32**> |  |  |[default to 50]

### Return type

[**models::ConversationPage**](ConversationPage.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_message_api_conversation_messages_post

> models::ConversationAcceptance send_message_api_conversation_messages_post(conversation_send_request)
Send Message

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_send_request** | [**ConversationSendRequest**](ConversationSendRequest.md) |  | [required] |

### Return type

[**models::ConversationAcceptance**](ConversationAcceptance.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_feedback_api_conversation_messages_message_id_feedback_put

> models::ConversationMessage set_feedback_api_conversation_messages_message_id_feedback_put(message_id, conversation_reply_feedback_request)
Set Feedback

Replace the person's feedback on one of Duet's finished replies.  Repeating the same feedback changes nothing. The reply comes back with its current feedback at the revision the change took.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message_id** | **uuid::Uuid** |  | [required] |
**conversation_reply_feedback_request** | [**ConversationReplyFeedbackRequest**](ConversationReplyFeedbackRequest.md) |  | [required] |

### Return type

[**models::ConversationMessage**](ConversationMessage.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

