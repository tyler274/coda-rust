# \DoctorWorkspaceApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**check_doctor_page_api_doctor_collection_page_share_post**](DoctorWorkspaceApi.md#check_doctor_page_api_doctor_collection_page_share_post) | **POST** /api/doctor-collection/page/share | Check Doctor Page
[**clear_doctor_workspace_api_doctor_collection_delete**](DoctorWorkspaceApi.md#clear_doctor_workspace_api_doctor_collection_delete) | **DELETE** /api/doctor-collection | Clear Doctor Workspace
[**doctor_preparation_events_api_doctor_collection_preparation_events_get**](DoctorWorkspaceApi.md#doctor_preparation_events_api_doctor_collection_preparation_events_get) | **GET** /api/doctor-collection/preparation/events | Doctor Preparation Events
[**edit_doctor_preparation_api_doctor_collection_preparation_put**](DoctorWorkspaceApi.md#edit_doctor_preparation_api_doctor_collection_preparation_put) | **PUT** /api/doctor-collection/preparation | Edit Doctor Preparation
[**get_doctor_collection_api_doctor_collection_get**](DoctorWorkspaceApi.md#get_doctor_collection_api_doctor_collection_get) | **GET** /api/doctor-collection | Get Doctor Collection
[**get_doctor_preparation_api_doctor_collection_preparation_get**](DoctorWorkspaceApi.md#get_doctor_preparation_api_doctor_collection_preparation_get) | **GET** /api/doctor-collection/preparation | Get Doctor Preparation
[**prepare_doctor_collection_api_doctor_collection_prepare_post**](DoctorWorkspaceApi.md#prepare_doctor_collection_api_doctor_collection_prepare_post) | **POST** /api/doctor-collection/prepare | Prepare Doctor Collection
[**remove_doctor_collection_item_api_doctor_collection_items_item_id_delete**](DoctorWorkspaceApi.md#remove_doctor_collection_item_api_doctor_collection_items_item_id_delete) | **DELETE** /api/doctor-collection/items/{item_id} | Remove Doctor Collection Item
[**reorder_doctor_collection_api_doctor_collection_order_put**](DoctorWorkspaceApi.md#reorder_doctor_collection_api_doctor_collection_order_put) | **PUT** /api/doctor-collection/order | Reorder Doctor Collection
[**save_doctor_page_api_doctor_collection_page_put**](DoctorWorkspaceApi.md#save_doctor_page_api_doctor_collection_page_put) | **PUT** /api/doctor-collection/page | Save Doctor Page
[**save_fact_for_doctor_api_doctor_collection_facts_fact_id_put**](DoctorWorkspaceApi.md#save_fact_for_doctor_api_doctor_collection_facts_fact_id_put) | **PUT** /api/doctor-collection/facts/{fact_id} | Save Fact For Doctor
[**save_insight_for_doctor_api_doctor_collection_insights_cycle_id_insight_id_put**](DoctorWorkspaceApi.md#save_insight_for_doctor_api_doctor_collection_insights_cycle_id_insight_id_put) | **PUT** /api/doctor-collection/insights/{cycle_id}/{insight_id} | Save Insight For Doctor
[**save_record_for_doctor_api_doctor_collection_record_put**](DoctorWorkspaceApi.md#save_record_for_doctor_api_doctor_collection_record_put) | **PUT** /api/doctor-collection/record | Save Record For Doctor
[**update_doctor_collection_note_api_doctor_collection_note_put**](DoctorWorkspaceApi.md#update_doctor_collection_note_api_doctor_collection_note_put) | **PUT** /api/doctor-collection/note | Update Doctor Collection Note



## check_doctor_page_api_doctor_collection_page_share_post

> models::DoctorCollection check_doctor_page_api_doctor_collection_page_share_post(doctor_page_check)
Check Doctor Page

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**doctor_page_check** | [**DoctorPageCheck**](DoctorPageCheck.md) |  | [required] |

### Return type

[**models::DoctorCollection**](DoctorCollection.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## clear_doctor_workspace_api_doctor_collection_delete

> clear_doctor_workspace_api_doctor_collection_delete()
Clear Doctor Workspace

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


## doctor_preparation_events_api_doctor_collection_preparation_events_get

> serde_json::Value doctor_preparation_events_api_doctor_collection_preparation_events_get()
Doctor Preparation Events

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_doctor_preparation_api_doctor_collection_preparation_put

> models::DoctorPreparationEnvelope edit_doctor_preparation_api_doctor_collection_preparation_put(doctor_preparation_edit_request)
Edit Doctor Preparation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**doctor_preparation_edit_request** | [**DoctorPreparationEditRequest**](DoctorPreparationEditRequest.md) |  | [required] |

### Return type

[**models::DoctorPreparationEnvelope**](DoctorPreparationEnvelope.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_doctor_collection_api_doctor_collection_get

> models::DoctorCollection get_doctor_collection_api_doctor_collection_get()
Get Doctor Collection

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::DoctorCollection**](DoctorCollection.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_doctor_preparation_api_doctor_collection_preparation_get

> models::DoctorPreparationEnvelope get_doctor_preparation_api_doctor_collection_preparation_get()
Get Doctor Preparation

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::DoctorPreparationEnvelope**](DoctorPreparationEnvelope.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prepare_doctor_collection_api_doctor_collection_prepare_post

> models::DoctorPreparationEnvelope prepare_doctor_collection_api_doctor_collection_prepare_post(doctor_preparation_prepare_request)
Prepare Doctor Collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**doctor_preparation_prepare_request** | [**DoctorPreparationPrepareRequest**](DoctorPreparationPrepareRequest.md) |  | [required] |

### Return type

[**models::DoctorPreparationEnvelope**](DoctorPreparationEnvelope.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_doctor_collection_item_api_doctor_collection_items_item_id_delete

> models::DoctorCollection remove_doctor_collection_item_api_doctor_collection_items_item_id_delete(item_id)
Remove Doctor Collection Item

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::DoctorCollection**](DoctorCollection.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reorder_doctor_collection_api_doctor_collection_order_put

> models::DoctorCollection reorder_doctor_collection_api_doctor_collection_order_put(doctor_collection_order_update)
Reorder Doctor Collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**doctor_collection_order_update** | [**DoctorCollectionOrderUpdate**](DoctorCollectionOrderUpdate.md) |  | [required] |

### Return type

[**models::DoctorCollection**](DoctorCollection.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## save_doctor_page_api_doctor_collection_page_put

> models::DoctorCollection save_doctor_page_api_doctor_collection_page_put(doctor_page_save)
Save Doctor Page

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**doctor_page_save** | [**DoctorPageSave**](DoctorPageSave.md) |  | [required] |

### Return type

[**models::DoctorCollection**](DoctorCollection.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## save_fact_for_doctor_api_doctor_collection_facts_fact_id_put

> models::DoctorCollection save_fact_for_doctor_api_doctor_collection_facts_fact_id_put(fact_id)
Save Fact For Doctor

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fact_id** | **String** |  | [required] |

### Return type

[**models::DoctorCollection**](DoctorCollection.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## save_insight_for_doctor_api_doctor_collection_insights_cycle_id_insight_id_put

> models::DoctorCollection save_insight_for_doctor_api_doctor_collection_insights_cycle_id_insight_id_put(cycle_id, insight_id)
Save Insight For Doctor

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cycle_id** | **uuid::Uuid** |  | [required] |
**insight_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::DoctorCollection**](DoctorCollection.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## save_record_for_doctor_api_doctor_collection_record_put

> models::DoctorCollection save_record_for_doctor_api_doctor_collection_record_put(canonical_record_selection)
Save Record For Doctor

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**canonical_record_selection** | [**CanonicalRecordSelection**](CanonicalRecordSelection.md) |  | [required] |

### Return type

[**models::DoctorCollection**](DoctorCollection.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_doctor_collection_note_api_doctor_collection_note_put

> models::DoctorCollection update_doctor_collection_note_api_doctor_collection_note_put(doctor_collection_note_update)
Update Doctor Collection Note

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**doctor_collection_note_update** | [**DoctorCollectionNoteUpdate**](DoctorCollectionNoteUpdate.md) |  | [required] |

### Return type

[**models::DoctorCollection**](DoctorCollection.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

