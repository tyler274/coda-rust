# \DefaultApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**apple_health_source_api_evidence_apple_health_get**](DefaultApi.md#apple_health_source_api_evidence_apple_health_get) | **GET** /api/evidence/apple-health | Apple Health Source
[**complete_epic_api_connectors_epic_complete_post**](DefaultApi.md#complete_epic_api_connectors_epic_complete_post) | **POST** /api/connectors/epic/complete | Complete Epic
[**complete_healow_api_connectors_healow_complete_post**](DefaultApi.md#complete_healow_api_connectors_healow_complete_post) | **POST** /api/connectors/healow/complete | Complete Healow
[**complete_oracle_api_connectors_oracle_complete_post**](DefaultApi.md#complete_oracle_api_connectors_oracle_complete_post) | **POST** /api/connectors/oracle/complete | Complete Oracle
[**connect_apple_health_api_evidence_apple_health_connect_post**](DefaultApi.md#connect_apple_health_api_evidence_apple_health_connect_post) | **POST** /api/evidence/apple-health/connect | Connect Apple Health
[**connect_health_records_api_evidence_health_records_connect_post**](DefaultApi.md#connect_health_records_api_evidence_health_records_connect_post) | **POST** /api/evidence/health-records/connect | Connect Health Records
[**disconnect_apple_health_api_evidence_apple_health_delete**](DefaultApi.md#disconnect_apple_health_api_evidence_apple_health_delete) | **DELETE** /api/evidence/apple-health | Disconnect Apple Health
[**disconnect_epic_api_connectors_epic_organization_id_delete**](DefaultApi.md#disconnect_epic_api_connectors_epic_organization_id_delete) | **DELETE** /api/connectors/epic/{organization_id} | Disconnect Epic
[**disconnect_healow_api_connectors_healow_organization_id_delete**](DefaultApi.md#disconnect_healow_api_connectors_healow_organization_id_delete) | **DELETE** /api/connectors/healow/{organization_id} | Disconnect Healow
[**disconnect_health_records_api_evidence_health_records_delete**](DefaultApi.md#disconnect_health_records_api_evidence_health_records_delete) | **DELETE** /api/evidence/health-records | Disconnect Health Records
[**disconnect_oracle_api_connectors_oracle_organization_id_delete**](DefaultApi.md#disconnect_oracle_api_connectors_oracle_organization_id_delete) | **DELETE** /api/connectors/oracle/{organization_id} | Disconnect Oracle
[**evidence_lab_api_evidence_labs_post**](DefaultApi.md#evidence_lab_api_evidence_labs_post) | **POST** /api/evidence/labs | Evidence Lab
[**evidence_library_api_evidence_library_post**](DefaultApi.md#evidence_library_api_evidence_library_post) | **POST** /api/evidence/library | Evidence Library
[**evidence_records_api_evidence_records_get**](DefaultApi.md#evidence_records_api_evidence_records_get) | **GET** /api/evidence/records | Evidence Records
[**evidence_series_api_evidence_series_post**](DefaultApi.md#evidence_series_api_evidence_series_post) | **POST** /api/evidence/series | Evidence Series
[**evidence_state_api_evidence_state_get**](DefaultApi.md#evidence_state_api_evidence_state_get) | **GET** /api/evidence/state | Evidence State
[**evidence_timeline_api_evidence_timeline_post**](DefaultApi.md#evidence_timeline_api_evidence_timeline_post) | **POST** /api/evidence/timeline | Evidence Timeline
[**health_health_get**](DefaultApi.md#health_health_get) | **GET** /health | Health
[**health_records_source_api_evidence_health_records_get**](DefaultApi.md#health_records_source_api_evidence_health_records_get) | **GET** /api/evidence/health-records | Health Records Source
[**ingest_evidence_api_evidence_post**](DefaultApi.md#ingest_evidence_api_evidence_post) | **POST** /api/evidence | Ingest Evidence
[**portal_organizations_api_connectors_portals_organizations_get**](DefaultApi.md#portal_organizations_api_connectors_portals_organizations_get) | **GET** /api/connectors/portals/organizations | Portal Organizations
[**portal_status_api_connectors_portals_get**](DefaultApi.md#portal_status_api_connectors_portals_get) | **GET** /api/connectors/portals | Portal Status
[**start_epic_api_connectors_epic_start_post**](DefaultApi.md#start_epic_api_connectors_epic_start_post) | **POST** /api/connectors/epic/start | Start Epic
[**start_healow_api_connectors_healow_start_post**](DefaultApi.md#start_healow_api_connectors_healow_start_post) | **POST** /api/connectors/healow/start | Start Healow
[**start_oracle_api_connectors_oracle_start_post**](DefaultApi.md#start_oracle_api_connectors_oracle_start_post) | **POST** /api/connectors/oracle/start | Start Oracle
[**sync_epic_api_connectors_epic_organization_id_sync_post**](DefaultApi.md#sync_epic_api_connectors_epic_organization_id_sync_post) | **POST** /api/connectors/epic/{organization_id}/sync | Sync Epic
[**sync_healow_api_connectors_healow_organization_id_sync_post**](DefaultApi.md#sync_healow_api_connectors_healow_organization_id_sync_post) | **POST** /api/connectors/healow/{organization_id}/sync | Sync Healow
[**sync_oracle_api_connectors_oracle_organization_id_sync_post**](DefaultApi.md#sync_oracle_api_connectors_oracle_organization_id_sync_post) | **POST** /api/connectors/oracle/{organization_id}/sync | Sync Oracle



## apple_health_source_api_evidence_apple_health_get

> models::PhoneHealthSourceResponse apple_health_source_api_evidence_apple_health_get(x_duet_source_key)
Apple Health Source

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_duet_source_key** | **String** |  | [required] |

### Return type

[**models::PhoneHealthSourceResponse**](PhoneHealthSourceResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## complete_epic_api_connectors_epic_complete_post

> complete_epic_api_connectors_epic_complete_post(epic_completion_request)
Complete Epic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**epic_completion_request** | [**EpicCompletionRequest**](EpicCompletionRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## complete_healow_api_connectors_healow_complete_post

> complete_healow_api_connectors_healow_complete_post(epic_completion_request)
Complete Healow

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**epic_completion_request** | [**EpicCompletionRequest**](EpicCompletionRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## complete_oracle_api_connectors_oracle_complete_post

> complete_oracle_api_connectors_oracle_complete_post(epic_completion_request)
Complete Oracle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**epic_completion_request** | [**EpicCompletionRequest**](EpicCompletionRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## connect_apple_health_api_evidence_apple_health_connect_post

> models::AppleHealthConnectResponse connect_apple_health_api_evidence_apple_health_connect_post(sharing_device_request)
Connect Apple Health

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sharing_device_request** | [**SharingDeviceRequest**](SharingDeviceRequest.md) |  | [required] |

### Return type

[**models::AppleHealthConnectResponse**](AppleHealthConnectResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## connect_health_records_api_evidence_health_records_connect_post

> models::HealthRecordsConnectResponse connect_health_records_api_evidence_health_records_connect_post(sharing_device_request)
Connect Health Records

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sharing_device_request** | [**SharingDeviceRequest**](SharingDeviceRequest.md) |  | [required] |

### Return type

[**models::HealthRecordsConnectResponse**](HealthRecordsConnectResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disconnect_apple_health_api_evidence_apple_health_delete

> models::AppleHealthDisconnectResponse disconnect_apple_health_api_evidence_apple_health_delete()
Disconnect Apple Health

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AppleHealthDisconnectResponse**](AppleHealthDisconnectResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disconnect_epic_api_connectors_epic_organization_id_delete

> models::EpicDisconnectResponse disconnect_epic_api_connectors_epic_organization_id_delete(organization_id)
Disconnect Epic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** |  | [required] |

### Return type

[**models::EpicDisconnectResponse**](EpicDisconnectResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disconnect_healow_api_connectors_healow_organization_id_delete

> models::EpicDisconnectResponse disconnect_healow_api_connectors_healow_organization_id_delete(organization_id)
Disconnect Healow

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** |  | [required] |

### Return type

[**models::EpicDisconnectResponse**](EpicDisconnectResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disconnect_health_records_api_evidence_health_records_delete

> models::HealthRecordsDisconnectResponse disconnect_health_records_api_evidence_health_records_delete()
Disconnect Health Records

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::HealthRecordsDisconnectResponse**](HealthRecordsDisconnectResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disconnect_oracle_api_connectors_oracle_organization_id_delete

> models::EpicDisconnectResponse disconnect_oracle_api_connectors_oracle_organization_id_delete(organization_id)
Disconnect Oracle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** |  | [required] |

### Return type

[**models::EpicDisconnectResponse**](EpicDisconnectResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## evidence_lab_api_evidence_labs_post

> models::EvidenceLabPage evidence_lab_api_evidence_labs_post(evidence_lab_query)
Evidence Lab

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**evidence_lab_query** | [**EvidenceLabQuery**](EvidenceLabQuery.md) |  | [required] |

### Return type

[**models::EvidenceLabPage**](EvidenceLabPage.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## evidence_library_api_evidence_library_post

> models::EvidenceLibraryPage evidence_library_api_evidence_library_post(evidence_library_query)
Evidence Library

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**evidence_library_query** | [**EvidenceLibraryQuery**](EvidenceLibraryQuery.md) |  | [required] |

### Return type

[**models::EvidenceLibraryPage**](EvidenceLibraryPage.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## evidence_records_api_evidence_records_get

> models::EvidenceReadPage evidence_records_api_evidence_records_get(entity, record_ids, concept_id, source_kind, clinical_kind, start, end, cursor)
Evidence Records

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**entity** | **String** |  | [required] |
**record_ids** | Option<[**Vec<uuid::Uuid>**](Uuid__Uuid.md)> |  |  |
**concept_id** | Option<**String**> |  |  |
**source_kind** | Option<**String**> |  |  |
**clinical_kind** | Option<**String**> |  |  |
**start** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  |  |
**end** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  |  |
**cursor** | Option<**String**> |  |  |

### Return type

[**models::EvidenceReadPage**](EvidenceReadPage.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## evidence_series_api_evidence_series_post

> models::EvidenceSeriesPage evidence_series_api_evidence_series_post(evidence_series_query, x_duet_app_build)
Evidence Series

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**evidence_series_query** | [**EvidenceSeriesQuery**](EvidenceSeriesQuery.md) |  | [required] |
**x_duet_app_build** | Option<**i32**> |  |  |

### Return type

[**models::EvidenceSeriesPage**](EvidenceSeriesPage.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## evidence_state_api_evidence_state_get

> models::EvidenceStateResponse evidence_state_api_evidence_state_get(cursor, since_revision, catalog_revision, source_generation)
Evidence State

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> |  |  |
**since_revision** | Option<**i32**> |  |  |
**catalog_revision** | Option<**String**> |  |  |
**source_generation** | Option<**i32**> |  |  |

### Return type

[**models::EvidenceStateResponse**](EvidenceStateResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## evidence_timeline_api_evidence_timeline_post

> models::EvidenceTimelinePage evidence_timeline_api_evidence_timeline_post(evidence_timeline_query)
Evidence Timeline

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**evidence_timeline_query** | [**EvidenceTimelineQuery**](EvidenceTimelineQuery.md) |  | [required] |

### Return type

[**models::EvidenceTimelinePage**](EvidenceTimelinePage.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## health_health_get

> std::collections::HashMap<String, String> health_health_get()
Health

### Parameters

This endpoint does not need any parameter.

### Return type

**std::collections::HashMap<String, String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## health_records_source_api_evidence_health_records_get

> models::PhoneHealthSourceResponse health_records_source_api_evidence_health_records_get(x_duet_source_key)
Health Records Source

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_duet_source_key** | Option<**String**> |  |  |

### Return type

[**models::PhoneHealthSourceResponse**](PhoneHealthSourceResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ingest_evidence_api_evidence_post

> models::EvidenceBatchResponse ingest_evidence_api_evidence_post(evidence_batch_request)
Ingest Evidence

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**evidence_batch_request** | [**EvidenceBatchRequest**](EvidenceBatchRequest.md) |  | [required] |

### Return type

[**models::EvidenceBatchResponse**](EvidenceBatchResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## portal_organizations_api_connectors_portals_organizations_get

> models::PortalOrganizationsResponse portal_organizations_api_connectors_portals_organizations_get(query)
Portal Organizations

One search across every available patient-portal vendor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | Option<**String**> |  |  |[default to ]

### Return type

[**models::PortalOrganizationsResponse**](PortalOrganizationsResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## portal_status_api_connectors_portals_get

> models::PortalsResponse portal_status_api_connectors_portals_get()
Portal Status

The vendors offered for new connections, and every patient-portal connection the person has, whether or not its vendor is still offered.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PortalsResponse**](PortalsResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_epic_api_connectors_epic_start_post

> models::EpicAuthorizationResponse start_epic_api_connectors_epic_start_post(epic_authorization_request)
Start Epic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**epic_authorization_request** | [**EpicAuthorizationRequest**](EpicAuthorizationRequest.md) |  | [required] |

### Return type

[**models::EpicAuthorizationResponse**](EpicAuthorizationResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_healow_api_connectors_healow_start_post

> models::EpicAuthorizationResponse start_healow_api_connectors_healow_start_post(epic_authorization_request)
Start Healow

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**epic_authorization_request** | [**EpicAuthorizationRequest**](EpicAuthorizationRequest.md) |  | [required] |

### Return type

[**models::EpicAuthorizationResponse**](EpicAuthorizationResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_oracle_api_connectors_oracle_start_post

> models::EpicAuthorizationResponse start_oracle_api_connectors_oracle_start_post(epic_authorization_request)
Start Oracle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**epic_authorization_request** | [**EpicAuthorizationRequest**](EpicAuthorizationRequest.md) |  | [required] |

### Return type

[**models::EpicAuthorizationResponse**](EpicAuthorizationResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sync_epic_api_connectors_epic_organization_id_sync_post

> models::PortalConnectionResponse sync_epic_api_connectors_epic_organization_id_sync_post(organization_id)
Sync Epic

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** |  | [required] |

### Return type

[**models::PortalConnectionResponse**](PortalConnectionResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sync_healow_api_connectors_healow_organization_id_sync_post

> models::PortalConnectionResponse sync_healow_api_connectors_healow_organization_id_sync_post(organization_id)
Sync Healow

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** |  | [required] |

### Return type

[**models::PortalConnectionResponse**](PortalConnectionResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sync_oracle_api_connectors_oracle_organization_id_sync_post

> models::PortalConnectionResponse sync_oracle_api_connectors_oracle_organization_id_sync_post(organization_id)
Sync Oracle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **String** |  | [required] |

### Return type

[**models::PortalConnectionResponse**](PortalConnectionResponse.md)

### Authorization

[HTTPBearer](../README.md#HTTPBearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

